//! La machine volante, à la manière de Léonard de Vinci : un berceau de bois sur patins, un mât, deux grandes ailes de toile
//! tendue sur des nervures, qu'on fait battre en pédalant, et une queue en éventail.
//! On la pose au sol (clic droit, la machine en main), on monte dedans (clic droit dessus). Au sol, avancer la fait rouler ;
//! lancée, sauter la décolle. En l'air : sauter fait battre les ailes et monter (au prix du souffle), sans battre on plane en
//! descendant doucement, reculer fait piquer ; trop lente, elle décroche. Gauche et droite virent. Elle se pose seule en
//! touchant le sol ; accroupi, on descend (une fois posé). Vérifié par `tests/machine.rs`.
use crate::actors::{Draw, Models};
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
pub fn wing_angle(f: &Flyer) -> f64 { -0.05 + f.beat * 0.2 * f.flap.sin() }
/// La partie extérieure de l'aile plie un peu plus, avec un léger retard : le battement ondule jusqu'au bout.
pub fn tip_angle(f: &Flyer) -> f64 { 0.03 + f.beat * 0.16 * (f.flap - 0.6).sin() }

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
        let beat = if can_flap { 1.0 } else if f.ground && i.z > 0.0 { 0.4 } else if !f.ground { 0.35 } else { 0.0 }; f.beat += (beat - f.beat).clamp(-2.0 * dt, 4.0 * dt);
        f.flap = (f.flap + dt * (2.2 + 3.6 * f.beat)).rem_euclid(std::f64::consts::TAU); // un battement ample et lent, pas un frétillement
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
            let a2 = tip_angle(f);
            for (sd, inner, outer) in [(-1.0f64, "wing_l", "wing_l2"), (1.0, "wing_r", "wing_r2")] { let root = mul(&mul(&body, &trs(sd * HINGE[0], HINGE[1], HINGE[2], 0.0, 0.0, 1.0)), &roll_z(sd * a)); o.push(d(root, md.part(inner)));
                o.push(d(mul(&mul(&root, &trs(sd * JOINT, rise(JOINT), 0.0, 0.0, 0.0, 1.0)), &roll_z(sd * a2)), md.part(outer))); } }
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

/// La jointure des ailes, à mi-envergure : la partie extérieure plie un peu plus que l'intérieure, comme une aile de chauve-souris.
pub const JOINT: f64 = 1.25;
fn span() -> f64 { HALF_SPAN - HINGE[0] }
/// Le profil d'une aile le long de l'envergure x (depuis l'articulation) : bord d'attaque (z), corde, flèche (y, relevée vers le bout).
fn lead(x: f64) -> f64 { 0.28 + 0.2 * (x / span()) - 0.18 * (x / span()).powi(2) }
fn chord(x: f64) -> f64 { 1.5 * (1.0 - 0.8 * (x / span()).powf(1.6)).max(0.12) }
fn rise(x: f64) -> f64 { 0.16 * (x / span()).powi(2) }

/// Le modèle, en volumes galbés de couleur unie : un berceau en forme de nacelle sur deux patins recourbés, un siège, un
/// pédalier, un mât et ses haubans, une poutre de queue et son éventail ; les ailes à part, en deux pièces chacune (le
/// longeron en arc, des nervures effilées en éventail, une toile bombée qui se creuse en festons entre elles), pour battre.
pub fn parts() -> Vec<(&'static str, VBuf)> {
    use crate::hostile::Mat; use crate::sculpt::Pen;
    let (wood, wood_l, dark, rope, iron) = ([0.5, 0.34, 0.19], [0.62, 0.45, 0.27], [0.28, 0.18, 0.1], [0.74, 0.66, 0.48], [0.36, 0.36, 0.38]);
    let mut o = VBuf::new(6000);
    { let mut p = Pen::new(&mut o, Mat::Plain, 0.05);
        // la nacelle : un fuseau de bois, plus large au milieu, quille relevée aux deux bouts
        p.body(&[(-1.15, 0.12, 0.66, 0.5), (-0.8, 0.34, 0.64, 0.36), (-0.1, 0.43, 0.62, 0.3), (0.6, 0.38, 0.63, 0.33), (1.05, 0.18, 0.66, 0.45), (1.25, 0.05, 0.7, 0.58)], wood_l, wood, dark);
        p.tube([0.0, 0.67, -1.12], [0.0, 0.68, 1.22], [0.03, 0.03], [0.03, 0.03], dark, dark); // le plat-bord central
        p.tube([-0.24, 0.68, -0.4], [0.24, 0.68, -0.4], [0.12, 0.05], [0.12, 0.05], dark, dark); // le siège
        p.tube([0.0, 0.7, -0.52], [0.0, 1.05, -0.58], [0.2, 0.03], [0.17, 0.03], dark, wood); // le dossier
        for sd in [-1.0, 1.0] { p.tube([sd * 0.16, 0.66, 0.7], [sd * 0.16, 0.74, 0.82], [0.03, 0.03], [0.03, 0.03], iron, iron); p.tube([sd * 0.22, 0.72, 0.76], [sd * 0.1, 0.72, 0.76], [0.06, 0.025], [0.06, 0.025], dark, dark); } // le pédalier
        // les patins recourbés devant, et leurs jambes de force
        for sd in [-1.0f64, 1.0] { let x = sd * 0.58;
            p.tube([x, 0.05, -1.25], [x, 0.05, 1.05], [0.045, 0.045], [0.045, 0.045], dark, dark); p.tube([x, 0.05, 1.05], [x, 0.14, 1.32], [0.045, 0.045], [0.04, 0.04], dark, dark); p.tube([x, 0.14, 1.32], [x, 0.3, 1.42], [0.04, 0.04], [0.035, 0.035], dark, dark);
            for z in [-0.85f64, 0.0, 0.85] { p.tube([x, 0.08, z], [sd * 0.36, 0.46, z * 0.9], [0.035, 0.035], [0.03, 0.03], wood, wood); } }
        // le mât, la traverse qui porte les ailes, les haubans
        p.tube([0.0, 0.62, 0.2], [0.0, 1.5, 0.2], [0.06, 0.06], [0.04, 0.04], wood, wood_l); p.blob([0.0, 1.52, 0.2], [0.06, 0.06, 0.06], iron, iron);
        p.tube([-HINGE[0] - 0.08, HINGE[1], HINGE[2]], [HINGE[0] + 0.08, HINGE[1], HINGE[2]], [0.05, 0.05], [0.05, 0.05], dark, dark);
        for (x, z) in [(-0.4, -1.0), (0.4, -1.0), (-0.4, 1.0), (0.4, 1.0)] { p.tube([x, 0.64, z], [0.0, 1.46, 0.2], [0.012, 0.012], [0.012, 0.012], rope, rope); }
        // la queue : une poutre effilée, l'éventail de toile tendu sur ses baguettes
        p.tube([0.0, 0.62, -1.05], [0.0, 0.95, -2.5], [0.045, 0.045], [0.03, 0.03], wood, wood);
        let cloth = [0.9, 0.84, 0.7]; for k in 0..7 { let a = -0.8 + 0.8 * k as f64 / 3.0; let tip = [a.sin() * 0.85, 0.97, -2.5 - a.cos() * 0.85];
            p.tube([0.0, 0.95, -2.5], tip, [0.018, 0.018], [0.012, 0.012], dark, dark);
            if k < 6 { let a2 = -0.8 + 0.8 * (k + 1) as f64 / 3.0; let n = 6; for s2 in 0..n { let t = (s2 as f64 + 0.5) / n as f64; let r = 0.85 * t; let (ca, cb) = ((a + a2) * 0.5, (a2 - a).abs() * 0.5);
                let c = [ca.sin() * r, 0.955, -2.5 - ca.cos() * r]; let w = r * cb.sin() + 0.02; p.tube([c[0] - w * ca.cos(), c[1], c[2] - w * ca.sin()], [c[0] + w * ca.cos(), c[1], c[2] + w * ca.sin()], [0.008, 0.075], [0.008, 0.075], cloth, cloth); } } }
    }
    // Une moitié d'aile, de x0 à x1 le long de l'envergure, ramenée à sa propre articulation (en x0), du côté `sd`.
    let wing = |sd: f64, x0: f64, x1: f64| { let mut q = VBuf::new(6000); { let mut p = Pen::new(&mut q, Mat::Plain, 0.04);
        let (cloth_in, cloth_out, spar) = ([0.92, 0.86, 0.72], [0.86, 0.78, 0.62], [0.42, 0.28, 0.15]);
        let at = |x: f64, z: f64, y: f64| [sd * (x - x0), rise(x) - rise(x0) + y, z];
        // le longeron : un arc de bois qui s'affine vers le bout
        let n = 10; for k in 0..n { let (xa, xb) = (x0 + (x1 - x0) * k as f64 / n as f64, x0 + (x1 - x0) * (k + 1) as f64 / n as f64); let r = |x: f64| 0.05 - 0.028 * (x / span());
            p.tube(at(xa, lead(xa), 0.0), at(xb, lead(xb), 0.0), [r(xa), r(xa)], [r(xb), r(xb)], spar, spar); }
        // les nervures, effilées, en éventail vers l'arrière ; la toile tendue entre elles, bombée, festonnée au bord de fuite
        let ribs: Vec<f64> = (0..=8).map(|k| span() * (0.02 + 0.125 * k as f64)).filter(|x| *x >= x0 - 1e-9 && *x <= x1 + 1e-9).collect();
        for &x in &ribs { let (z0, z1) = (lead(x), lead(x) - chord(x)); p.tube(at(x, z0, 0.0), at(x, z1, -0.02), [0.026, 0.03], [0.012, 0.014], spar, spar); }
        let cols = 18; let rows = 9;
        for c in 0..cols { let (xa, xb) = (x0 + (x1 - x0) * c as f64 / cols as f64, x0 + (x1 - x0) * (c + 1) as f64 / cols as f64); let xm = 0.5 * (xa + xb);
            // le feston : entre deux nervures, la toile recule un peu au bord de fuite
            let pitch = span() * 0.125; let ph = ((xm - span() * 0.02) / pitch).fract(); let scallop = 0.16 * (ph * std::f64::consts::PI).sin() * (0.4 + 0.6 * xm / span());
            let ch = (chord(xm) - scallop).max(0.08); let col = crate::sculpt::mix(cloth_in, cloth_out, xm / span());
            for r in 0..rows { let (ua, ub) = (r as f64 / rows as f64, (r + 1) as f64 / rows as f64); let um = 0.5 * (ua + ub); let camber = 0.07 * (um * std::f64::consts::PI).sin() * (1.0 - 0.5 * xm / span());
                let (za, zb) = (lead(xm) - ch * ua, lead(xm) - ch * ub); let a = at(xa, 0.0, camber); let b2 = at(xb, 0.0, camber);
                p.bx([a[0].min(b2[0]), a[1].min(b2[1]) - 0.01, zb, a[0].max(b2[0]), a[1].max(b2[1]) + 0.01, za], col); } }
    } q };
    vec![("flyer", o), ("wing_r", wing(1.0, 0.0, JOINT)), ("wing_l", wing(-1.0, 0.0, JOINT)), ("wing_r2", wing(1.0, JOINT, span())), ("wing_l2", wing(-1.0, JOINT, span()))]
}
