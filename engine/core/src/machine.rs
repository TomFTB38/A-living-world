//! La machine volante, à la manière de Léonard de Vinci : un berceau de bois sur patins, un mât, deux grandes ailes de toile
//! tendue sur des nervures, qu'on fait battre en pédalant, et une queue en éventail.
//! On la pose au sol (clic droit, la machine en main), on monte dedans (clic droit dessus). Au sol, avancer la fait rouler ;
//! lancée, sauter la décolle. En l'air : sauter fait battre les ailes et monter (au prix du souffle), sans battre on plane en
//! descendant doucement, reculer fait piquer ; trop lente, elle décroche. Gauche et droite virent. Elle se pose seule en
//! touchant le sol ; accroupi, on descend (une fois posé). Vérifié par `tests/machine.rs`.
use crate::actors::{b, Draw, Models};
use crate::blocks::is_solid;
use crate::mesher::VBuf;
use crate::player::{Input, Voxels};
use crate::session::Session;

/// Vitesses (m/s) : au sol, de décollage, de croisière, en plané lent, en dessous de laquelle elle décroche.
pub const ROLL: f64 = 7.0; pub const TAKEOFF: f64 = 4.5; pub const CRUISE: f64 = 11.0; pub const SLOW: f64 = 6.0; pub const STALL: f64 = 4.0;
/// Vitesses verticales (m/s) : en battant des ailes, en planant, en piquant, en décrochage.
pub const CLIMB: f64 = 2.4; pub const GLIDE: f64 = -1.2; pub const DIVE: f64 = -3.8; pub const FALL: f64 = -6.0;
/// Le souffle dépensé par seconde de battement, la vitesse de virage, le plafond.
pub const FLAP_COST: f64 = 16.0; pub const TURN: f64 = 1.2; pub const CEILING: f64 = 230.0;
/// Envergure (demi), demi-longueur.
pub const HALF_SPAN: f64 = 2.9; pub const HALF_L: f64 = 1.5;
/// L'articulation des ailes au sommet du berceau (côté droit ; le gauche en miroir).
pub const HINGE: [f64; 3] = [0.28, 1.3, 0.2];

#[derive(Clone, Copy, Debug)]
pub struct Flyer { /** le dessous des patins, au milieu */ pub p: [f64; 3], pub yaw: f64, /** vitesse vers l'avant */ pub v: f64, pub vy: f64, pub ground: bool,
    /** le battement des ailes (radians) et sa vigueur (0 : ailes immobiles, 1 : on pédale fort) */ pub flap: f64, pub beat: f64 }
impl Flyer { pub fn new(p: [f64; 3], yaw: f64) -> Self { Flyer { p, yaw, v: 0.0, vy: 0.0, ground: true, flap: 0.0, beat: 0.0 } } }

/// L'angle des ailes (relevées si positif) pour un battement donné : au repos, un peu tombantes.
pub fn wing_angle(f: &Flyer) -> f64 { -0.08 + f.beat * 0.45 * f.flap.sin() }

/// Quelque chose de solide là où passerait la machine (le berceau et le bout des ailes) ?
fn blocked(w: &impl Voxels, p: [f64; 3], yaw: f64) -> bool {
    let (f, r) = ([yaw.sin(), yaw.cos()], [yaw.cos(), -yaw.sin()]);
    [(0.0, 0.0, 0.3), (0.0, 0.0, 1.2), (HALF_L - 0.2, 0.0, 0.5), (-(HALF_L - 0.2), 0.0, 0.8), (0.0, HALF_SPAN - 0.2, 1.3), (0.0, -(HALF_SPAN - 0.2), 1.3)].iter().any(|(a, c, h)| {
        let q = [p[0] + f[0] * a + r[0] * c, p[1] + h, p[2] + f[1] * a + r[1] * c]; w.get(q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32).is_some_and(is_solid) })
}
/// Le sol sous les patins : la hauteur du dessus du bloc solide le plus haut juste sous `p` (à moins d'un demi-bloc).
fn ground_under(w: &impl Voxels, p: [f64; 3]) -> Option<f64> {
    let y = (p[1] - 0.05).floor() as i32; w.get(p[0].floor() as i32, y, p[2].floor() as i32).is_some_and(is_solid).then_some(y as f64 + 1.0)
}

impl Session {
    /// Le clic droit : poser la machine au sol, y monter, ou (accroupi) la ranger. Rend `Some` si elle a pris le clic.
    pub fn flyer_click(&mut self) -> Option<bool> {
        if self.flyer.is_some() || self.boat.is_some() || self.use_t > 0.0 { return None; }
        let (o, d) = (self.eye, self.look);
        let block = self.target.map(|t| ((t.x as f64 + 0.5 - o[0]).powi(2) + (t.y as f64 + 0.5 - o[1]).powi(2) + (t.z as f64 + 0.5 - o[2]).powi(2)).sqrt()).unwrap_or(f64::INFINITY);
        let hit = self.flyers.iter().enumerate().filter_map(|(k, f)| flyer_ray(f, o, d).map(|t| (k, t))).filter(|(_, t)| *t < 5.0 && *t < block + 0.3).min_by(|a, b| a.1.total_cmp(&b.1));
        if let Some((k, _)) = hit {
            self.use_t = 0.4;
            if self.player.crouch { let f = self.flyers.remove(k); if self.add_item("flying_machine", 1) > 0 { self.spill("flying_machine", 1, f.p); } self.notes.push("La machine volante est démontée et rangée".into()); self.dirty = true; return Some(true); }
            self.flyer = Some(k); self.ride = None; self.seated = None; let q = self.flyers[k].p; self.player.p = [q[0], q[1] + 0.42, q[2]]; self.player.v = [0.0; 3];
            self.notes.push("Aux commandes : avancez pour prendre de l'élan, sautez pour décoller puis battre des ailes ; reculez pour descendre".into()); return Some(true);
        }
        if self.inv[self.sel].map(|s| s.item) != Some("flying_machine") { return None; }
        self.use_t = 0.4; let Some(t) = self.target else { return Some(true) };
        let p = [t.px as f64 + 0.5, t.py as f64, t.pz as f64 + 0.5]; let yaw = self.player.yaw;
        let floor = self.world.get(t.px, t.py - 1, t.pz).is_some_and(is_solid);
        if !floor || blocked(&self.world, p, yaw) || self.flyers.iter().any(|f| (f.p[0] - p[0]).hypot(f.p[2] - p[2]) < 5.0) { self.notes.push("La machine volante se pose sur un sol dégagé, ailes comprises".into()); return Some(true); }
        self.flyers.push(Flyer::new(p, yaw)); self.consume_held(); self.dirty = true; self.sound.place("planks"); self.notes.push("La machine volante est prête : clic droit pour monter".into()); Some(true)
    }
    /// Aux commandes : rouler, décoller, battre des ailes, planer, virer, se poser. Rend vrai tant qu'on est dedans.
    pub(crate) fn flyer_step(&mut self, dt: f64, i: Input) -> bool {
        let Some(k) = self.flyer else { return false }; if k >= self.flyers.len() { self.flyer = None; return false; }
        if (self.player.p[0] - self.flyers[k].p[0]).hypot(self.player.p[2] - self.flyers[k].p[2]) > 4.0 || self.dead.is_some() { self.flyer = None; return false; } // emporté ailleurs
        if i.crouch && self.flyers[k].ground { self.leave_flyer(); return false; }
        if i.crouch && !self.flyers[k].ground && self.use_t <= 0.0 { self.notes.push("Posez-vous d'abord : reculez pour descendre".into()); self.use_t = 1.5; }
        let can_flap = (self.creative || (self.stamina > 1.0 && !self.winded)) && i.jump;
        let w = &self.world; let f = &mut self.flyers[k]; let mut spent = 0.0;
        f.yaw -= i.x * TURN * dt;
        if f.ground {
            f.v += (ROLL * i.z.max(0.0) - f.v).clamp(-4.0 * dt, 3.0 * dt); f.vy = 0.0;
            if i.jump && f.v >= TAKEOFF { f.ground = false; f.vy = CLIMB; }
        } else {
            let cruise = if i.z > 0.0 { CRUISE } else if i.z < 0.0 { SLOW } else { 8.5 }; f.v += (cruise - f.v).clamp(-2.0 * dt, 2.0 * dt);
            let want = if f.v < STALL { FALL } else if can_flap { CLIMB } else if i.z < 0.0 { DIVE } else { GLIDE }; f.vy += (want - f.vy).clamp(-4.0 * dt, 4.0 * dt);
            if can_flap && !self.creative { spent = FLAP_COST * dt; }
        }
        // les ailes : elles battent quand on pédale (au sol pour l'élan, en l'air pour monter), à peine en planant
        let beat = if can_flap { 1.0 } else if f.ground && i.z > 0.0 { 0.5 } else if !f.ground { 0.12 } else { 0.0 }; f.beat += (beat - f.beat).clamp(-2.0 * dt, 4.0 * dt);
        f.flap = (f.flap + dt * (2.0 + 7.0 * f.beat)).rem_euclid(std::f64::consts::TAU);
        // le déplacement : arrêté net par un mur, posé par le sol
        let (fx, fz) = (f.yaw.sin(), f.yaw.cos()); let next = [f.p[0] + fx * f.v * dt, (f.p[1] + f.vy * dt).min(CEILING), f.p[2] + fz * f.v * dt];
        if !blocked(w, [next[0], f.p[1], next[2]], f.yaw) { f.p[0] = next[0]; f.p[2] = next[2]; } else { f.v = 0.0; }
        if f.vy > 0.0 { if !blocked(w, [f.p[0], next[1], f.p[2]], f.yaw) { f.p[1] = next[1]; } else { f.vy = 0.0; } }
        else if let Some(g) = ground_under(w, [f.p[0], next[1], f.p[2]]) { if !f.ground { f.ground = true; f.vy = 0.0; } f.p[1] = g; }
        else { f.p[1] = next[1]; if f.ground { f.ground = false; f.vy = 0.0; } } // le sol s'est dérobé : elle part en plané
        let p = f.p; let yaw = f.yaw;
        if spent > 0.0 { self.spend(spent); }
        self.player.p = [p[0], p[1] + 0.42, p[2]]; self.player.v = [0.0; 3]; self.player.ground = true; self.player.speed = 0.0; self.player.in_water = false; let _ = yaw; true
    }
    /// Descendre de la machine posée : à côté, sur le sol.
    pub fn leave_flyer(&mut self) {
        let Some(k) = self.flyer.take() else { return }; let Some(f) = self.flyers.get(k).copied() else { return };
        let side = f.yaw + std::f64::consts::FRAC_PI_2; let (x, z) = (f.p[0] + side.sin() * 1.0 - f.yaw.sin() * 2.0, f.p[2] + side.cos() * 1.0 - f.yaw.cos() * 2.0);
        self.player.p = [x, f.p[1] + 0.05, z]; self.player.v = [0.0; 3]; self.flyers[k].v = 0.0; self.notes.push("Vous descendez de la machine volante".into());
    }
    /// Les machines volantes, à dessiner : le berceau, puis chaque aile qui bat autour de son articulation.
    pub fn flyer_draws(&self, md: &Models) -> Vec<Draw> {
        use crate::view::{mul, roll_z, trs}; let mut o = Vec::new();
        for f in &self.flyers { let tilt = if f.ground { 0.0 } else { (-f.vy * 0.06).clamp(-0.2, 0.25) }; let body = trs(f.p[0], f.p[1], f.p[2], f.yaw, tilt, 1.0);
            let d = |m, r| Draw { m, r, view: false, local: false, hero: false, dynamic: false, hurt: 0.0 }; o.push(d(body, md.part("flyer"))); let a = wing_angle(f);
            for (sd, part) in [(-1.0f64, "wing_l"), (1.0, "wing_r")] { o.push(d(mul(&mul(&body, &trs(sd * HINGE[0], HINGE[1], HINGE[2], 0.0, 0.0, 1.0)), &roll_z(sd * a)), md.part(part))); } }
        o
    }
    pub(crate) fn flyers_save(&self) -> String { self.flyers.iter().map(|f| format!("flyer {} {} {} {}\n", f.p[0], f.p[1], f.p[2], f.yaw)).collect() }
}

/// Le rayon touche-t-il la machine (le berceau, les ailes) ? À quelle distance.
pub fn flyer_ray(f: &Flyer, o: [f64; 3], d: [f64; 3]) -> Option<f64> {
    let (s, c) = (f.yaw.sin(), f.yaw.cos()); let rel = [o[0] - f.p[0], o[1] - f.p[1], o[2] - f.p[2]];
    let lo = [rel[0] * c - rel[2] * s, rel[1], rel[0] * s + rel[2] * c]; let ld = [d[0] * c - d[2] * s, d[1], d[0] * s + d[2] * c];
    let body = crate::shapes::ray_box(lo, ld, [-0.6, 0.0, -HALF_L, 0.6, 1.8, HALF_L]); let wings = crate::shapes::ray_box(lo, ld, [-HALF_SPAN, 1.0, -0.6, HALF_SPAN, 1.6, 1.0]);
    match (body, wings) { (Some(a), Some(b2)) => Some(a.min(b2)), (a, b2) => a.or(b2) }
}

/// Le modèle : patins, berceau de planches, siège, pédalier, mât et haubans, queue en éventail ; les ailes à part (nervures
/// de bois en éventail depuis le longeron, toile tendue entre elles), pour battre.
pub fn parts() -> Vec<(&'static str, VBuf)> {
    let (wood, wood2, dark, rope, cloth, cloth2) = ([0.56, 0.4, 0.23], [0.47, 0.33, 0.18], [0.3, 0.2, 0.11], [0.72, 0.64, 0.46], [0.88, 0.82, 0.67], [0.8, 0.73, 0.58]);
    let mut o = VBuf::new(1500);
    for sd in [-1.0, 1.0] { b(&mut o, [sd * 0.55 - 0.05, 0.0, -1.2, sd * 0.55 + 0.05, 0.09, 1.25], dark); b(&mut o, [sd * 0.55 - 0.05, 0.03, 1.2, sd * 0.55 + 0.05, 0.2, 1.38], dark); // les patins, relevés devant
        for z in [-0.9, 0.0, 0.9] { b(&mut o, [sd * 0.5 - 0.035, 0.05, z - 0.035, sd * 0.5 + 0.035, 0.5, z + 0.035], wood2); } // les montants
        b(&mut o, [sd * 0.42 - 0.03, 0.42, -1.05, sd * 0.42 + 0.03, 0.62, 1.05], wood); } // les flancs du berceau
    b(&mut o, [-0.45, 0.4, -1.05, 0.45, 0.46, 1.05], wood2); // le plancher
    b(&mut o, [-0.25, 0.46, -0.35, 0.25, 0.56, 0.1], dark); b(&mut o, [-0.25, 0.56, -0.38, 0.25, 0.95, -0.32], dark); // le siège et son dossier
    b(&mut o, [-0.18, 0.52, 0.62, 0.18, 0.56, 0.66], dark); for sd in [-1.0, 1.0] { b(&mut o, [sd * 0.14 - 0.04, 0.5, 0.55, sd * 0.14 + 0.04, 0.62, 0.72], wood); } // le pédalier
    b(&mut o, [-0.05, 0.46, 0.15, 0.05, 1.42, 0.25], wood); b(&mut o, [-0.4, 1.28, 0.12, 0.4, 1.36, 0.28], dark); // le mât et la traverse des ailes
    for (x, z) in [(-0.42, -1.0), (0.42, -1.0), (-0.42, 1.0), (0.42, 1.0)] { let n = 8; for k in 0..n { let t = k as f64 / n as f64; let q = [x + (0.0 - x) * t, 0.6 + 0.8 * t, z + (0.2 - z) * t]; b(&mut o, [q[0] - 0.012, q[1] - 0.05, q[2] - 0.012, q[0] + 0.012, q[1] + 0.05, q[2] + 0.012], rope); } } // les haubans
    b(&mut o, [-0.04, 0.85, -2.6, 0.04, 0.93, -1.0], wood); // la poutre de queue
    for k in 0..6 { let a = -0.75 + 0.3 * k as f64; let (dx, dz) = (a.sin() * 0.8, -a.cos() * 0.8); let n = 5;
        for s in 0..n { let t = (s as f64 + 0.5) / n as f64; let q = [dx * t, 0.89, -2.55 + dz * t]; b(&mut o, [q[0] - 0.13 * t - 0.02, q[1] - 0.012, q[2] - 0.1, q[0] + 0.13 * t + 0.02, q[1] + 0.012, q[2] + 0.1], if k % 2 == 0 { cloth } else { cloth2 }); }
        b(&mut o, [dx * 0.5 - 0.015, 0.9, -2.55 + dz * 0.5 - 0.015, dx * 0.5 + 0.015, 0.92, -2.55 + dz * 0.5 + 0.015], dark); } // l'éventail de queue
    // Une aile : le long de +x depuis l'articulation ; le longeron en arc, six nervures vers l'arrière, la toile entre elles.
    let wing = |sd: f64| { let mut q = VBuf::new(1200); let span = HALF_SPAN - HINGE[0]; let m = |x0: f64, x1: f64| if sd > 0.0 { (x0, x1) } else { (-x1, -x0) };
        let chord = |x: f64| 1.45 * (1.0 - (x / span).powi(2) * 0.75); let lead = |x: f64| 0.25 + 0.15 * (x / span); let lift = |x: f64| 0.12 * (x / span).powi(2);
        let n = 14; for k in 0..n { let (x0, x1) = (span * k as f64 / n as f64, span * (k + 1) as f64 / n as f64); let xm = 0.5 * (x0 + x1); let (a, c) = m(x0, x1);
            b(&mut q, [a, lift(xm) - 0.03, lead(xm) - 0.04, c, lift(xm) + 0.03, lead(xm) + 0.04], wood); // le longeron
            let ch = chord(xm); let rows = 6; for r in 0..rows { let z0 = lead(xm) - ch * (r as f64) / rows as f64; let z1 = lead(xm) - ch * (r + 1) as f64 / rows as f64; let sag = -0.03 * ((r as f64 + 0.5) / rows as f64 * std::f64::consts::PI).sin();
                b(&mut q, [a, lift(xm) + sag - 0.008, z1, c, lift(xm) + sag + 0.008, z0], if (k + r) % 2 == 0 { cloth } else { cloth2 }); } }
        for k in 0..6 { let x = span * (0.08 + 0.17 * k as f64); let (a, c) = m(x - 0.025, x + 0.025); let ch = chord(x); b(&mut q, [a, lift(x) - 0.02, lead(x) - ch - 0.02, c, lift(x) + 0.03, lead(x)], dark); } // les nervures
        q };
    vec![("flyer", o), ("wing_r", wing(1.0)), ("wing_l", wing(-1.0))]
}
