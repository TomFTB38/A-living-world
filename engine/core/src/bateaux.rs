//! Les barques : on en pose une sur l'eau (clic droit, une barque en main), on monte dedans (clic droit dessus), on rame
//! (avancer, reculer ; gauche et droite font virer la barque), on descend en s'accroupissant (sur la rive si elle est là).
//! Une barque flotte, suit le niveau de l'eau, s'arrête contre la rive ; accroupi, un clic droit la range dans le sac.
//! Elle va bien plus vite qu'à la nage. Vérifié par `tests/bateaux.rs`.
use crate::actors::{b, Draw, Models};
use crate::blocks::{is_solid, is_water};
use crate::mesher::VBuf;
use crate::player::{Input, Voxels};
use crate::session::Session;

/// Vitesse de croisière à la rame (m/s), en arrière, et l'accélération.
pub const ROW_SPEED: f64 = 6.5; pub const BACK_SPEED: f64 = 2.2; const ACCEL: f64 = 3.0;
/// Vitesse de virage (radians par seconde).
pub const TURN: f64 = 1.7;
/// Les rames : dame de nage (dans le repère de la coque, côté droit ; le gauche en miroir), longueur du manche à
/// l'intérieur et de la rame entière au-delà, cadence (coups par seconde).
pub const OARLOCK: [f64; 3] = [0.6, 0.39, 0.45]; pub const OAR_IN: f64 = 0.55; pub const OAR_OUT: f64 = 1.75; pub const STROKES: f64 = 0.85;
/// La pose d'une rame : (balayage, la pale vers l'avant si positif ; plongée, la pale vers le bas si positive), radians.
/// En nageant : la pale entre dans l'eau devant, pousse vers l'arrière en plongeant, ressort et revient par les airs.
/// Au repos : rentrée le long de la coque, pale relevée, vers l'arrière.
pub fn oar_pose(phase: f64, pull: f64) -> (f64, f64) {
    let (sweep, dip) = (0.55 * phase.cos(), 0.3 + 0.14 * phase.sin()); let k = pull.clamp(0.0, 1.0);
    (-1.0 + (sweep + 1.0) * k, 0.12 + (dip - 0.12) * k)
}
/// Le bout de la pale d'une rame (0 : bâbord, à gauche ; 1 : tribord, à droite), dans le monde.
pub fn blade_tip(bt: &Boat, side: usize) -> [f64; 3] {
    let sd = if side == 1 { 1.0 } else { -1.0 }; let (a, d) = oar_pose(bt.oar[side], bt.pull[side]);
    // dans le repère de la coque : la rame part de la dame de nage, vers l'extérieur, tournée de `a`, plongée de `d`
    let (h, y) = (OAR_OUT * d.cos(), -OAR_OUT * d.sin()); let local = [sd * (OARLOCK[0] + h * a.cos()), OARLOCK[1] + y, OARLOCK[2] + h * a.sin()];
    let (s, c) = (bt.yaw.sin(), bt.yaw.cos()); [bt.p[0] + local[0] * c + local[2] * s, bt.p[1] + 0.03 + bt.bob + local[1], bt.p[2] - local[0] * s + local[2] * c]
}
/// Demi-longueur et demi-largeur de la coque (m).
pub const HALF_L: f64 = 1.45; pub const HALF_W: f64 = 0.66;

#[derive(Clone, Copy, Debug)]
pub struct Boat { /** le milieu de la coque, au ras de l'eau */ pub p: [f64; 3], pub yaw: f64, /** vitesse le long de la coque */ pub v: f64, /** tangage léger, pour le dessin */ pub bob: f64,
    /** le coup de rame de chaque bord (gauche, droite), en radians : 0 l'attaque, pale devant ; π le dégagé, pale derrière */ pub oar: [f64; 2], /** de 0 (rames rentrées, au repos) à 1 (on rame) */ pub pull: [f64; 2] }
impl Boat { pub fn new(p: [f64; 3], yaw: f64) -> Self { Boat { p, yaw, v: 0.0, bob: 0.0, oar: [0.0; 2], pull: [0.0; 2] } } }

/// La surface de l'eau sous (x, z), près de la hauteur `near` : le haut du dernier bloc d'eau qui a de l'air au-dessus.
pub fn surface(w: &impl Voxels, x: f64, z: f64, near: f64) -> Option<f64> {
    let (xi, zi, y0) = (x.floor() as i32, z.floor() as i32, near.floor() as i32);
    for y in (y0 - 4..=y0 + 2).rev() { let (here, up) = (w.get(xi, y, zi)?, w.get(xi, y + 1, zi)?); if is_water(here) && !is_water(up) && !is_solid(up) { return Some(y as f64 + 0.9); } }
    None
}
/// La coque passe-t-elle en (x, z) : de l'eau au même niveau sous la pointe, le milieu et les flancs.
fn afloat(w: &impl Voxels, x: f64, z: f64, yaw: f64, level: f64) -> bool {
    let (f, r) = ([yaw.sin(), yaw.cos()], [yaw.cos(), -yaw.sin()]);
    [(0.0, 0.0), (HALF_L - 0.15, 0.0), (-(HALF_L - 0.15), 0.0), (0.0, HALF_W - 0.1), (0.0, -(HALF_W - 0.1))].iter().all(|(a, c)| {
        let (px, pz) = (x + f[0] * a + r[0] * c, z + f[1] * a + r[1] * c); surface(w, px, pz, level).is_some_and(|s| (s - level).abs() < 0.6) })
}

impl Session {
    /// Le clic droit : poser une barque sur l'eau, monter dans celle qu'on vise, ou (accroupi) la ranger. Rend `Some` si
    /// la barque a pris le clic.
    pub fn boat_click(&mut self) -> Option<bool> {
        if self.boat.is_some() || self.flyer.is_some() || self.use_t > 0.0 { return None; }
        let (o, d) = (self.eye, self.look);
        // la barque visée, la plus proche, avant le bloc visé
        let block = self.target.map(|t| ((t.x as f64 + 0.5 - o[0]).powi(2) + (t.y as f64 + 0.5 - o[1]).powi(2) + (t.z as f64 + 0.5 - o[2]).powi(2)).sqrt()).unwrap_or(f64::INFINITY);
        let hit = self.boats.iter().enumerate().filter_map(|(k, bt)| boat_ray(bt, o, d).map(|t| (k, t))).filter(|(_, t)| *t < 4.5 && *t < block + 0.3).min_by(|a, b| a.1.total_cmp(&b.1));
        if let Some((k, _)) = hit {
            self.use_t = 0.4;
            if self.player.crouch { let bt = self.boats.remove(k); if self.add_item("boat", 1) > 0 { self.spill("boat", 1, bt.p); } self.notes.push("La barque est rangée dans votre sac".into()); self.dirty = true; return Some(true); }
            self.boat = Some(k); self.boats[k].v = 0.0; self.ride = None; self.seated = None; let q = self.boats[k].p; self.player.p = [q[0], q[1] - 0.2, q[2]]; self.player.v = [0.0; 3]; self.notes.push("Dans la barque : avancez pour ramer, gauche et droite pour virer, accroupi pour descendre".into()); return Some(true);
        }
        if self.inv[self.sel].map(|s| s.item) != Some("boat") { return None; }
        // poser : la première eau libre sous le regard, à six mètres au plus
        self.use_t = 0.4;
        let mut t = 0.3; while t < 6.0 { let q = [o[0] + d[0] * t, o[1] + d[1] * t, o[2] + d[2] * t]; let id = self.world.get(q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32).unwrap_or(0);
            if is_water(id) { if let Some(y) = surface(&self.world, q[0], q[2], q[1]) { let yaw = self.player.yaw;
                    if afloat(&self.world, q[0], q[2], yaw, y) && !self.boats.iter().any(|bt| (bt.p[0] - q[0]).hypot(bt.p[2] - q[2]) < 2.0) {
                        self.boats.push(Boat::new([q[0], y, q[2]], yaw)); self.consume_held(); self.dirty = true; self.sound.place("planks"); self.notes.push("La barque flotte : clic droit pour monter dedans".into()); return Some(true); } }
                break; }
            if is_solid(id) { break; } t += 0.1; }
        self.notes.push("Une barque se pose sur une eau assez large".into()); Some(true)
    }
    /// En barque : on rame, la barque vire, flotte et s'arrête contre la rive ; accroupi, on descend. Les barques vides
    /// suivent le niveau de l'eau. Rend vrai tant qu'on est dans une barque (le joueur est alors porté).
    pub(crate) fn boat_step(&mut self, dt: f64, i: Input) -> bool {
        // les barques vides : au fil de l'eau, elles ralentissent et suivent son niveau
        self.anim.row = None; for (k, bt) in self.boats.iter_mut().enumerate() { if Some(k) == self.boat { continue; } bt.v *= (1.0 - 2.0 * dt).max(0.0); for q in bt.pull.iter_mut() { *q = (*q - 3.0 * dt).max(0.0); } if let Some(y) = surface(&self.world, bt.p[0], bt.p[2], bt.p[1]) { bt.p[1] = y; } }
        let Some(k) = self.boat else { return false }; if k >= self.boats.len() { self.boat = None; return false; }
        // emporté ailleurs (une commande, la mort) : on n'est plus dans la barque
        if (self.player.p[0] - self.boats[k].p[0]).hypot(self.player.p[2] - self.boats[k].p[2]) > 4.0 { self.boat = None; return false; }
        if i.crouch || self.dead.is_some() { self.leave_boat(); return false; }
        let w = &self.world; let bt = &mut self.boats[k];
        bt.yaw -= i.x * TURN * dt * if bt.v < -0.1 { -1.0 } else { 1.0 };
        let want = if i.z > 0.0 { ROW_SPEED * i.z } else { BACK_SPEED * i.z }; let dv = (want - bt.v).clamp(-ACCEL * dt, ACCEL * dt); bt.v += dv;
        let (fx, fz) = (bt.yaw.sin(), bt.yaw.cos()); let (nx, nz) = (bt.p[0] + fx * bt.v * dt, bt.p[2] + fz * bt.v * dt);
        if afloat(w, nx, nz, bt.yaw, bt.p[1]) { bt.p[0] = nx; bt.p[2] = nz; } else { bt.v = 0.0; } // la rive arrête la barque
        if let Some(y) = surface(w, bt.p[0], bt.p[2], bt.p[1]) { bt.p[1] += (y - bt.p[1]).clamp(-2.0 * dt, 2.0 * dt); }
        // Les rames : chaque bord nage selon ce qu'on demande (en avant, les deux ; pour virer à droite, le bord gauche pousse
        // et le droit recule ; pour virer en reculant, l'inverse), au rythme d'un coup par seconde environ ; sans ordre, elles se
        // rentrent le long de la coque.
        for (k, act) in [(i.z + i.x).clamp(-1.0, 1.0), (i.z - i.x).clamp(-1.0, 1.0)].into_iter().enumerate() {
            if act.abs() > 0.05 { bt.oar[k] = (bt.oar[k] + act * std::f64::consts::TAU * STROKES * dt).rem_euclid(std::f64::consts::TAU); }
            let want = if act.abs() > 0.05 { 1.0 } else { 0.0 }; bt.pull[k] += (want - bt.pull[k]).clamp(-3.0 * dt, 4.0 * dt); }
        bt.bob = (self.elapsed * 1.7).sin() * 0.02 + bt.v.abs() * 0.004; let (p, oar, pull) = (bt.p, bt.oar, bt.pull);
        self.anim.row = Some([oar[0], oar[1], pull[0], pull[1]]);
        self.player.p = [p[0], p[1] - 0.2, p[2]]; self.player.v = [0.0; 3]; self.player.ground = true; self.player.speed = 0.0; self.player.in_water = false; true
    }
    /// Quitter la barque : sur la rive la plus proche si un pas suffit, sinon à l'eau, à côté.
    pub fn leave_boat(&mut self) {
        let Some(k) = self.boat.take() else { return }; let Some(bt) = self.boats.get(k).copied() else { return };
        let w = &self.world; let stand = |x: i32, y: i32, z: i32| w.get(x, y - 1, z).is_some_and(is_solid) && [y, y + 1].iter().all(|yy| w.get(x, *yy, z).is_some_and(|b| !is_solid(b) && !is_water(b)));
        let (cx, cy, cz) = (bt.p[0].floor() as i32, bt.p[1].floor() as i32, bt.p[2].floor() as i32); let mut best: Option<([i32; 3], f64)> = None;
        for dx in -3..=3 { for dz in -3..=3 { for dy in 0..=2 { let (x, y, z) = (cx + dx, cy + dy, cz + dz); if stand(x, y, z) { let d = ((x as f64 + 0.5 - bt.p[0]).hypot(z as f64 + 0.5 - bt.p[2])) + dy as f64 * 0.3; if best.is_none_or(|b| d < b.1) { best = Some(([x, y, z], d)); } } } } }
        self.boats[k].v = 0.0;
        self.player.p = match best { Some((q, _)) => [q[0] as f64 + 0.5, q[1] as f64, q[2] as f64 + 0.5], None => { let side = bt.yaw + std::f64::consts::FRAC_PI_2; [bt.p[0] + side.sin() * 1.3, bt.p[1] - 0.4, bt.p[2] + side.cos() * 1.3] } };
        self.player.v = [0.0; 3]; self.notes.push(if best.is_some() { "Vous mettez pied à terre".into() } else { "Vous vous laissez glisser à l'eau".into() });
    }
    /// Les barques, à dessiner.
    pub fn boat_draws(&self, md: &Models) -> Vec<Draw> {
        use crate::view::{mul, roll_z, trs}; let mut o = Vec::new();
        for bt in &self.boats { let hull = trs(bt.p[0], bt.p[1] + 0.03 + bt.bob, bt.p[2], bt.yaw, 0.0, 1.0); let d = |m, r| Draw { m, r, view: false, local: false, hero: false, dynamic: false, hurt: 0.0 };
            o.push(d(hull, md.part("boat")));
            // chaque rame tourne dans sa dame de nage : balayage autour de la verticale, puis plongée
            for (side, sd, part) in [(0usize, -1.0f64, "oar_l"), (1, 1.0, "oar_r")] { let (a, dip) = oar_pose(bt.oar[side], bt.pull[side]);
                let m = mul(&mul(&hull, &trs(sd * OARLOCK[0], OARLOCK[1], OARLOCK[2], -sd * a, 0.0, 1.0)), &roll_z(-sd * dip)); o.push(d(m, md.part(part))); } }
        o
    }
    pub(crate) fn boats_save(&self) -> String { self.boats.iter().map(|b| format!("boat {} {} {} {}\n", b.p[0], b.p[1], b.p[2], b.yaw)).collect() }
}

/// Le rayon touche-t-il la coque (dans le repère de la barque) ? À quelle distance.
pub fn boat_ray(bt: &Boat, o: [f64; 3], d: [f64; 3]) -> Option<f64> {
    let (s, c) = (bt.yaw.sin(), bt.yaw.cos()); let rel = [o[0] - bt.p[0], o[1] - bt.p[1], o[2] - bt.p[2]];
    // dans le repère de la coque : x en travers, z dans la longueur
    let lo = [rel[0] * c - rel[2] * s, rel[1], rel[0] * s + rel[2] * c]; let ld = [d[0] * c - d[2] * s, d[1], d[0] * s + d[2] * c];
    crate::shapes::ray_box(lo, ld, [-HALF_W, -0.45, -HALF_L, HALF_W, 0.55, HALF_L])
}

/// La barque : une coque de planches en fuseau (fond, bordages qui se relèvent aux deux bouts), une étrave et un tableau,
/// deux bancs, deux rames posées dans leurs dames de nage. Longue de 2,9 m, large de 1,3 m ; le haut du bord est à +0,32.
pub fn parts() -> Vec<(&'static str, VBuf)> {
    let mut o = VBuf::new(2000); let (wood, wood2, dark, oar) = ([0.5, 0.33, 0.18], [0.42, 0.27, 0.14], [0.27, 0.17, 0.09], [0.52, 0.4, 0.24]);
    let n = 16; let half = |z: f64| HALF_W * (1.0 - (z / HALF_L).abs().powf(2.2)).max(0.0).sqrt();
    for k in 0..n { let (z0, z1) = (-HALF_L + 2.0 * HALF_L * k as f64 / n as f64, -HALF_L + 2.0 * HALF_L * (k + 1) as f64 / n as f64); let zm = 0.5 * (z0 + z1);
        let w = half(zm).max(0.08); let sheer = 0.3 + 0.12 * (zm / HALF_L).powi(2); let col = if k % 2 == 0 { wood } else { wood2 };
        b(&mut o, [-w + 0.06, -0.06, z0, w - 0.06, 0.0, z1], dark); // le fond, au-dessus de l'eau
        b(&mut o, [-w * 0.75, -0.24, z0, w * 0.75, -0.04, z1], wood2); b(&mut o, [-0.05, -0.3, z0, 0.05, -0.2, z1], dark); // la carène et la quille, dans l'eau
        b(&mut o, [-w, -0.02, z0, -w + 0.07, sheer, z1], col); b(&mut o, [w - 0.07, -0.02, z0, w, sheer, z1], col); // les bordages
        b(&mut o, [-w - 0.01, sheer - 0.04, z0, -w + 0.09, sheer + 0.02, z1], dark); b(&mut o, [w - 0.09, sheer - 0.04, z0, w + 0.01, sheer + 0.02, z1], dark); } // le plat-bord
    b(&mut o, [-0.06, -0.04, HALF_L - 0.12, 0.06, 0.5, HALF_L + 0.04], dark); // l'étrave
    b(&mut o, [-0.18, -0.04, -HALF_L - 0.02, 0.18, 0.44, -HALF_L + 0.08], wood2); // le tableau
    for z in [-0.35, 0.45] { let w = half(z); b(&mut o, [-w + 0.05, 0.16, z - 0.13, w - 0.05, 0.21, z + 0.13], wood); }
    for sd in [-1.0, 1.0] { let w = half(0.45); b(&mut o, [sd * w - 0.04, 0.32, 0.4, sd * w + 0.04, 0.4, 0.5], dark); // dame de nage
 }
    // Une rame, le long de +x depuis la dame de nage : la poignée à l'intérieur, le manche, la pale plate au bout ; la
    // gauche est son miroir.
    let oar_mesh = |sd: f64| { let mut q = VBuf::new(200); let x = |a: f64, b2: f64| if sd > 0.0 { (a, b2) } else { (-b2, -a) };
        let (a0, a1) = x(-OAR_IN, -OAR_IN + 0.16); b(&mut q, [a0, -0.03, -0.03, a1, 0.03, 0.03], dark); // la poignée
        let (a0, a1) = x(-OAR_IN + 0.16, OAR_OUT - 0.45); b(&mut q, [a0, -0.022, -0.022, a1, 0.022, 0.022], oar); // le manche
        let (a0, a1) = x(OAR_OUT - 0.45, OAR_OUT - 0.3); b(&mut q, [a0, -0.02, -0.05, a1, 0.02, 0.05], oar);
        let (a0, a1) = x(OAR_OUT - 0.3, OAR_OUT); b(&mut q, [a0, -0.015, -0.09, a1, 0.015, 0.09], oar); // la pale
        let (a0, a1) = x(-0.04, 0.04); b(&mut q, [a0, -0.035, -0.035, a1, 0.035, 0.035], dark); q }; // le taquet contre la dame
    vec![("boat", o), ("oar_r", oar_mesh(1.0)), ("oar_l", oar_mesh(-1.0))]
}
