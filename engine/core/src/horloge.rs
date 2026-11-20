//! La Tour de l'Horloge : une haute tour de brique sous un toit de cuivre, au milieu d'une cour pavée. Sur la façade du nord,
//! une grande horloge arrêtée depuis la mort de l'horloger. On y monte par quatre étages, trois énigmes et un boss :
//! - **la salle des rouages** (rez-de-chaussée) : trois engrenages géants barrent la salle, chacun percé d'une encoche. Trois
//!   leviers les font tourner d'un quart de tour, mais les rouages sont couplés : le premier levier entraîne les deux premiers,
//!   le deuxième les deux derniers, le troisième le dernier seul. Les trois encoches en bas, on passe.
//! - **la salle du cadran** (premier étage) : l'envers du grand cadran, ses deux aiguilles et leurs leviers. Il faut les
//!   remettre à l'heure où s'est arrêtée l'horloge de la façade (on l'a vue en arrivant) : la grille de l'échelle se lève.
//! - **la salle du balancier** (deuxième étage) : un pont étroit au-dessus d'une fosse ; le balancier le fauche à chaque
//!   passage, et projette qui s'y trouve. On l'observe, puis on court.
//! - **le beffroi** (au sommet) : sous la grande cloche, l'Automate de l'horloger.
//! L'heure de la façade et la position des rouages changent d'une tour à l'autre. Tout est vérifié par `tests/horloge.rs`.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Dial, Guard, Hint, Mech, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// La tour : demi-largeur (u, murs compris), murs du nord et du sud (w).
pub const HALF: i32 = 7; pub const NORTH: i32 = -9; pub const SOUTH: i32 = 9;
/// Les planchers (le bloc du sol de chaque étage) : rouages, cadran, balancier (la fosse), beffroi ; le haut des murs.
pub const FLOORS: [i32; 4] = [0, 8, 21, 36]; pub const TOP: i32 = 44;
/// Les trois engrenages (leur plan w) et leur centre (u, v) ; les trois leviers ; ce que chaque levier fait tourner.
pub const GEARS: [i32; 3] = [-3, 0, 3]; pub const GEAR_C: (i32, i32) = (0, 4);
pub const GEAR_LEVERS: [[i32; 3]; 3] = [[-3, 2, -6], [0, 2, -6], [3, 2, -6]];
pub const COUPLING: [&[usize]; 3] = [&[0, 1], &[1, 2], &[2]];
/// Le cadran : son centre (u, v) ; le plan de la face, de l'aiguille des heures, de celle des minutes (dedans) ; les leviers.
pub const CLOCK_C: (i32, i32) = (0, 15); pub const HOUR_LEVER: [i32; 3] = [-2, 10, -3]; pub const MINUTE_LEVER: [i32; 3] = [2, 10, -3];
/// L'échelle vers le balancier (u, w, du niveau, au niveau) et sa grille.
pub const LADDER_B: (i32, i32, i32, i32) = (-6, 8, 9, 25); pub const GRILLE: [[i32; 3]; 2] = [[-6, 9, 7], [-6, 10, 7]];
/// Le pont du balancier (w, son bloc v, de u, à u) ; le pivot du balancier (v, w), sa longueur, ses angles ; l'échelle de la fosse.
pub const BRIDGE: (i32, i32, i32, i32) = (2, 25, -3, 3); pub const PIVOT: (i32, i32) = (33, 2); pub const ARM: f64 = 6.5;
pub const SWING: [f64; 5] = [-30.0, -15.0, 0.0, 15.0, 30.0]; pub const PIT_LADDER: (i32, i32) = (-3, 6);
/// L'échelle du beffroi (u, w, du niveau, au niveau).
pub const LADDER_D: (i32, i32, i32, i32) = (6, -4, 26, 36);
/// L'Automate, le coffre.
pub const BOSS: &str = "automate"; pub const BOSS_AT: [f64; 3] = [0.5, 37.0, 3.5]; pub const CHEST: [i32; 3] = [0, 38, 7];

/// L'heure où s'est arrêtée l'horloge de cette tour (heure 0 à 11 — 0 pour midi —, quart d'heure 0 à 3) ; où sont les aiguilles
/// et les rouages au départ.
pub fn stopped(o: [i32; 3]) -> (u8, u8) { ((hash_float(o[0], o[2], 4711, 1) * 12.0) as u8 % 12, (hash_float(o[0], o[2], 4711, 2) * 4.0) as u8 % 4) }
pub fn hands_start(o: [i32; 3]) -> (u8, u8) { let (h, m) = stopped(o); ((h + 3 + (hash_float(o[0], o[2], 4711, 3) * 7.0) as u8) % 12, (m + 1 + (hash_float(o[0], o[2], 4711, 4) * 3.0) as u8 % 3) % 4) }
pub fn gears_start(o: [i32; 3]) -> [u8; 3] { let g = |k| (hash_float(o[0], o[2], 4711, 10 + k) * 4.0) as u8 % 4; let s = [g(0), g(1), g(2)]; if s == [0, 0, 0] { [1, 3, 2] } else { s } }

/// Le dessin d'un engrenage sur le cran `s` (0 : l'encoche en bas) : un disque de laiton, ses dents qui tournent avec lui, son
/// encoche (deux cases, du bord vers l'axe).
pub fn gear(w: i32, s: u8) -> Vec<([i32; 3], &'static str)> {
    let (cu, cv) = GEAR_C; let d = [(0, -1), (-1, 0), (0, 1), (1, 0)][s as usize % 4]; let notch = [(cu + d.0 * 2, cv + d.1 * 2), (cu + d.0 * 3, cv + d.1 * 3)];
    let mut o = Vec::new();
    for du in -3..=3i32 { for dv in -3..=3i32 { let r2 = du * du + dv * dv; if r2 > 9 || (du, dv) == (0, 0) || notch.contains(&(cu + du, cv + dv)) { continue; }
        let a = (dv as f64).atan2(du as f64).to_degrees() + 90.0 * s as f64; let tooth = (a.rem_euclid(360.0) / 45.0).floor() as i32 % 2 == 0;
        o.push(([cu + du, cv + dv, w], if r2 >= 7 && tooth { "brass_dark" } else { "brass" })); } }
    o
}
/// Une aiguille, de l'axe vers le cran `k` sur `n` (dans le sens des aiguilles d'une montre pour qui regarde vers `-w` si `inside`,
/// vers `+w` sinon), longue de `len`.
pub fn hand(w: i32, k: u8, n: u8, len: f64, inside: bool, b: &'static str) -> Vec<([i32; 3], &'static str)> {
    let a = (k as f64 / n as f64) * std::f64::consts::TAU; let (du, dv) = (if inside { a.sin() } else { -a.sin() }, a.cos()); let mut o: Vec<([i32; 3], &'static str)> = Vec::new();
    let mut t = 1.0; while t <= len { let c = [CLOCK_C.0 + (du * t).round() as i32, CLOCK_C.1 + (dv * t).round() as i32, w]; if [c[0], c[1]] != [CLOCK_C.0, CLOCK_C.1] && !o.iter().any(|(q, _)| *q == c) { o.push((c, b)); } t += 0.25; }
    o
}
/// Le balancier sur le cran `k` : sa tige de chaîne, sa lentille de laiton (trois de large, deux de haut, trois d'épaisseur).
pub fn pendulum(k: u8) -> Vec<([i32; 3], &'static str)> {
    let a = SWING[k as usize % SWING.len()].to_radians(); let (pv, pw) = PIVOT; let (cw, cv) = (pw as f64 + ARM * a.sin(), pv as f64 - ARM * a.cos()); let (cw, cv) = (cw.round() as i32, cv.round() as i32);
    let mut o: Vec<([i32; 3], &'static str)> = Vec::new(); for u in -1..=1 { for v in cv - 1..=cv { for w in cw - 1..=cw + 1 { o.push(([u, v, w], if u == 0 && v == cv && w == cw { "brass_dark" } else { "brass" })); } } }
    let mut t = 1.0; while t < ARM - 1.0 { let c = [0, (pv as f64 - t * a.cos()).round() as i32, (pw as f64 + t * a.sin()).round() as i32]; if !o.iter().any(|(q, _)| *q == c) { o.push((c, "chain")); } t += 0.5; }
    o
}

struct G<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl G<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn h(&self, u: i32, v: i32, w: i32, k: i32) -> f64 { hash_float(self.x + u + 337 * v, self.z + w, 8231, k) }
    fn draw(&mut self, l: &[([i32; 3], &'static str)]) { for (c, b) in l { self.n(c[0], c[1], c[2], b); } }
    fn column(&mut self, u: i32, w: i32, v0: i32, v1: i32) { for v in v0..=v1 { self.n(u, v, w, if v == v0 { "column_base" } else if v == v1 { "column_capital" } else { "column_shaft" }); } }
    /// La brique des murs : quelques briques sombres, de la pierre aux angles et aux bandeaux des étages.
    fn brick(&self, u: i32, v: i32, w: i32) -> &'static str {
        let corner = u.abs() == HALF && (w == NORTH || w == SOUTH); if corner || FLOORS.contains(&v) || v == TOP { return "stone_bricks"; }
        if self.h(u, v, w, 1) < 0.12 { "bricks_dark" } else { "clay_bricks" }
    }
}

/// Pose la tour dans le plan : `o` est le sol au centre (le dallage du rez-de-chaussée est un bloc plus haut).
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut g = G { p, x: o[0], f: o[1] + 1, z: o[2] };
    let (gs, (hs, ms), (hg, mg)) = (gears_start(o), hands_start(o), stopped(o));
    // la cour pavée, ses bancs et ses lanternes ; le perron du nord
    for u in -12..=12i32 { for w in -17..=13i32 { g.n(u, -1, w, if (u.abs() + w.abs()) % 7 == 0 { "stone_bricks_dark" } else { "stone_bricks" }); for v in 0..=3 { g.air(u, v, w); } } }
    for u in -12..=12i32 { for w in -17..=13i32 { if (u.abs() == 12 || w == 13 || w == -17) && !(u.abs() <= 2 && w == -17) { g.n(u, 0, w, "stone_bricks"); if (u + w) % 4 == 0 { g.n(u, 1, w, "lantern"); } } } }
    for (u, w) in [(-9, -14), (9, -14), (-9, 10), (9, 10)] { g.n(u, 0, w, "stone_bricks_dark"); g.n(u, 1, w, "stone_bricks_dark"); g.n(u, 2, w, "lantern"); }
    // la tour : murs de brique, planchers, l'intérieur vide
    for u in -HALF..=HALF { for w in NORTH..=SOUTH { let wall = u.abs() == HALF || w == NORTH || w == SOUTH;
        for v in 0..=TOP { if wall || FLOORS.contains(&v) { let b = if wall { g.brick(u, v, w) } else if v == 0 { if (u + w).rem_euclid(2) == 0 { "marble" } else { "marble_black" } } else { "planks" }; g.n(u, v, w, b); } else { g.air(u, v, w); } } } }
    // les quatre tourelles d'angle, coiffées de cuivre ; les créneaux ; le grand toit de cuivre et sa flèche
    for (su, sw) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] { let (cu, cw) = (su * HALF, if sw < 0 { NORTH } else { SOUTH });
        for v in TOP + 1..=TOP + 4 { g.n(cu, v, cw, "stone_bricks"); } for k in 0..3 { g.n(cu, TOP + 5 + k, cw, "copper_roof"); } g.n(cu, TOP + 8, cw, "lantern"); }
    for u in -HALF..=HALF { for w in NORTH..=SOUTH { if (u.abs() == HALF || w == NORTH || w == SOUTH) && (u + w).rem_euclid(2) == 0 { g.n(u, TOP + 1, w, "stone_bricks"); } } }
    for k in 0..6 { let (hu, hw) = (HALF - 1 - k, SOUTH - 1 - k); if hu < 0 { break; } for u in -hu..=hu { for w in -hw..=hw { if u.abs() == hu || w.abs() == hw { g.n(u, TOP + 2 + k, w, "copper_roof"); } } } }
    for u in -1..=1 { for w in -3..=3 { g.n(u, TOP + 8, w, "copper_roof"); } } for v in TOP + 9..=TOP + 12 { g.n(0, v, 0, "iron_block"); } g.n(0, TOP + 13, 0, "lantern");
    // les fenêtres hautes (vitraux) aux étages du cadran et du balancier, à l'est et à l'ouest
    for s in [-1, 1] { for w in [-5, 0, 5] { for v in [11, 12, 13, 14, 24, 25, 26, 27, 28] { g.n(s * HALF, v, w, "stained_glass"); } } }
    // la porte du nord : une arche de pierre, deux colonnes, des marches
    for u in -1..=1 { for v in 1..=3 { g.air(u, v, NORTH); } } for u in -2..=2 { g.n(u, 4, NORTH, "stone_bricks_dark"); } for u in [-2, 2] { g.column(u, NORTH - 1, 1, 4); }
    for u in -3..=3 { g.n(u, 0, NORTH - 1, "stone_bricks"); g.t(u, 0, NORTH - 2, "stone_bricks_stairs", 0, 1); }
    // la grande horloge de la façade, arrêtée : un disque d'émail, son anneau de laiton, les douze heures, les aiguilles
    let (cu, cv) = CLOCK_C; for du in -7..=7i32 { for dv in -7..=7i32 { let r = ((du * du + dv * dv) as f64).sqrt(); if r <= 6.4 { g.n(cu + du, cv + dv, NORTH - 1, if r > 5.6 { "brass" } else { "clock_face" }); } } }
    for k in 0..12 { let a = k as f64 * std::f64::consts::TAU / 12.0; g.n(cu + (-a.sin() * 5.0).round() as i32, cv + (a.cos() * 5.0).round() as i32, NORTH - 2, if k % 3 == 0 { "brass_dark" } else { "iron_block" }); }
    g.n(cu, cv, NORTH - 2, "iron_block"); g.draw(&hand(NORTH - 2, hg, 12, 3.4, false, "brass_dark")); g.draw(&hand(NORTH - 3, mg, 4, 4.6, false, "brass")); g.n(cu, cv, NORTH - 3, "iron_block");
    // — LA SALLE DES ROUAGES — trois murs d'engrenages, des rouages au repos aux murs, les leviers sur leurs socles
    for (k, w) in GEARS.iter().enumerate() { for u in -HALF + 1..HALF { for v in 1..FLOORS[1] { g.n(u, v, *w, if (u + v).rem_euclid(5) == 0 { "stone_bricks_dark" } else { "stone_bricks" }); } }
        let (cu, cv) = GEAR_C; for du in -3..=3i32 { for dv in -3..=3i32 { if du * du + dv * dv <= 9 { g.air(cu + du, cv + dv, *w); } } } g.n(cu, cv, *w, "iron_block"); g.draw(&gear(*w, gs[k])); }
    for (k, l) in GEAR_LEVERS.iter().enumerate() { g.n(l[0], l[1] - 1, l[2], "brass_dark"); g.t(l[0], l[1], l[2], "lever", 0, -1); let _ = k; }
    for w in [-7, 6] { for s in [-1, 1] { g.n(s * 6, 1, w, "lantern"); } }
    // l'escalier vers le cadran, le long du mur du sud ; la trémie au-dessus
    for k in 0..7 { let u = 5 - k; for w in [7, 8] { for v in 1..=k { g.n(u, v, w, "stone_bricks"); } g.t(u, k + 1, w, "stone_bricks_stairs", -1, 0); } }
    for u in -1..=5 { for w in [7, 8] { g.air(u, FLOORS[1], w); } } for u in -1..=5 { g.n(u, FLOORS[1] + 1, 6, "iron_bars"); }
    // — LA SALLE DU CADRAN — l'envers du cadran : la face, les heures, l'axe, les aiguilles ; les leviers ; la grille de l'échelle
    for du in -7..=7i32 { for dv in -7..=7i32 { let r = ((du * du + dv * dv) as f64).sqrt(); if r <= 6.4 { g.n(cu + du, cv + dv, NORTH + 1, if r > 5.6 { "brass" } else { "clock_face" }); } } }
    for k in 0..12 { let a = k as f64 * std::f64::consts::TAU / 12.0; g.n(cu + (a.sin() * 5.0).round() as i32, cv + (a.cos() * 5.0).round() as i32, NORTH + 2, if k % 3 == 0 { "brass_dark" } else { "iron_block" }); }
    g.n(cu, cv, NORTH + 2, "iron_block"); g.n(cu, cv, NORTH + 3, "iron_block"); g.draw(&hand(NORTH + 2, hs, 12, 3.4, true, "brass_dark")); g.draw(&hand(NORTH + 3, ms, 4, 4.6, true, "brass"));
    for l in [HOUR_LEVER, MINUTE_LEVER] { g.n(l[0], l[1] - 1, l[2], "brass_dark"); g.t(l[0], l[1], l[2], "lever", 0, -1); }
    for s in [-1, 1] { g.n(s * 6, FLOORS[1] + 1, 4, "lantern"); g.column(s * 4, 4, FLOORS[1] + 1, FLOORS[2] - 1); }
    let (lu, lw, l0, l1) = LADDER_B; for v in l0..=l1 { g.t(lu, v, lw, "ladder", 0, 1); } for v in FLOORS[1] + 1..=FLOORS[1] + 3 { g.n(lu + 1, v, lw, "stone_bricks"); } for c in GRILLE { g.n(c[0], c[1], c[2], "iron_bars"); }
    // — LA SALLE DU BALANCIER — la fosse ; les paliers de l'ouest et de l'est ; le pont ; l'axe et le balancier
    let (bw, bv, b0, b1) = BRIDGE; let c0 = FLOORS[2] + 1;
    for u in -6..=-4 { for w in 0..=8 { for v in c0..=bv { g.n(u, v, w, "stone_bricks"); } } } for u in 4..=6 { for w in -4..=4 { for v in c0..=bv { g.n(u, v, w, "stone_bricks"); } } }
    for v in l0..=l1 { g.t(lu, v, lw, "ladder", 0, 1); } for u in b0..=b1 { g.n(u, bv, bw, "planks"); }
    for v in c0..=bv { g.t(PIT_LADDER.0, v, PIT_LADDER.1, "ladder", -1, 0); }
    for u in -6..=6 { g.n(u, PIVOT.0 + 1, PIVOT.1, "brass_dark"); } g.n(0, PIVOT.0, PIVOT.1, "iron_block"); g.draw(&pendulum(0));
    for (u, w) in [(-3, -6), (3, -6), (-3, 8), (3, 8)] { g.n(u, c0, w, "lantern"); }
    for s in [-1, 1] { for (du, dv) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] { g.n(s * 6, 30 + dv, -6 + du, "brass_dark"); } }
    let (du, dw, d0, d1) = LADDER_D; for v in d0..=d1 { g.t(du, v, dw, "ladder", 1, 0); }
    // — LE BEFFROI — de grandes arches sur les quatre côtés, la grande cloche, l'autel et son coffre
    let b = FLOORS[3]; for k in -4..=4 { for v in b + 3..=b + 7 { g.air(k, v, NORTH); g.air(k, v, SOUTH); } for v in b + 3..=b + 7 { g.air(-HALF, v, k * 2); g.air(HALF, v, k * 2); } }
    for k in [-5, 5] { g.column(k, NORTH, b + 3, b + 7); g.column(k, SOUTH, b + 3, b + 7); }
    for v in b + 4..=TOP - 1 { let r = 0.6 + (TOP - 1 - v) as f64 * 0.55; for du in -3..=3i32 { for dw in -3..=3i32 { let d = ((du * du + dw * dw) as f64).sqrt(); if d <= r && d > r - 1.2 { g.n(du, v, dw, if v == b + 4 { "brass_dark" } else { "brass" }); } } } }
    for u in -6..=6 { g.n(u, TOP, 0, "brass_dark"); } g.n(0, TOP - 1, 0, "iron_block"); for v in b + 5..=TOP - 2 { g.n(0, v, 0, "chain"); }
    g.t(CHEST[0], CHEST[1] - 1, CHEST[2], "altar", 0, -1); g.n(CHEST[0], CHEST[1], CHEST[2], "chest"); for u in [-2, 2] { g.n(u, b + 1, CHEST[2], "lantern"); }
}

/// Les mécanismes de la tour bâtie en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] + 1; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let ids = |l: Vec<([i32; 3], &'static str)>| l.into_iter().map(|(c, b)| (at(c), by_name(b).unwrap_or(0))).collect::<Vec<_>>();
    let mut m = Mech::default(); let gs = gears_start(o); let (hs, ms) = hands_start(o); let (hg, mg) = stopped(o);
    for (k, w) in GEARS.iter().enumerate() { let levers = (0..3).filter(|l| COUPLING[*l].contains(&k)).map(|l| (at(GEAR_LEVERS[l]), 1)).collect();
        m.dials.push(Dial { name: name(&format!("rouage{k}")), n: 4, state: gs[k], goal: 0, draw: (0..4).map(|s| ids(gear(*w, s))).collect(), levers, ..Default::default() }); }
    m.dials.push(Dial { name: name("heures"), n: 12, state: hs, goal: hg, draw: (0..12).map(|s| ids(hand(NORTH + 2, s, 12, 3.4, true, "brass_dark"))).collect(), levers: vec![(at(HOUR_LEVER), 1)], ..Default::default() });
    m.dials.push(Dial { name: name("minutes"), n: 4, state: ms, goal: mg, draw: (0..4).map(|s| ids(hand(NORTH + 3, s, 4, 4.6, true, "brass"))).collect(), levers: vec![(at(MINUTE_LEVER), 1)], ..Default::default() });
    let iron = by_name("iron_bars").unwrap_or(0);
    m.wires.push(Wire { name: name("cadran"), needs: vec![name("heures"), name("minutes")], latch: true, doors: GRILLE.iter().map(|c| (at(*c), iron)).collect(), ..Default::default() });
    m.dials.push(Dial { name: name("balancier"), n: SWING.len() as u8, state: 0, goal: 255, draw: (0..SWING.len() as u8).map(|s| ids(pendulum(s))).collect(), auto: 0.35, swing: true, hit: 6.0, ..Default::default() });
    let b = FLOORS[3]; m.guards.push(Guard { min: at([-HALF + 1, b + 1, NORTH + 1]), max: at([HALF - 1, TOP - 1, SOUTH - 1]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: Vec::new(), state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-HALF, -1, NORTH - 4]), max: at([HALF, TOP + 12, SOUTH]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-6, 1, -8], [6, 3, -5], "La Tour de l'Horloge. Trois rouages barrent la salle, chacun percé d'une encoche. Trois leviers les font tourner d'un quart de tour, mais ils sont couplés : le premier entraîne aussi le deuxième, le deuxième aussi le troisième. Les trois encoches en bas, on passera.");
    hint(&mut m, [-6, FLOORS[1] + 1, -8], [6, FLOORS[1] + 3, 8], "L'envers du grand cadran. Dehors, l'horloge de la façade s'est arrêtée à l'heure où mourut l'horloger. Remettez les aiguilles à cette heure-là : la grille s'ouvrira.");
    hint(&mut m, [-6, bv() + 1, 0], [-4, bv() + 3, 8], "La salle du balancier. Il fauche le pont à chaque passage : observez-le, puis courez.");
    m
}
fn bv() -> i32 { BRIDGE.1 }

impl Session {
    /// Bâtit ici la tour (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol au centre de la tour.
    pub fn build_clock_tower(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("h{}:{}:", o[0], o[2]); if !self.mech.dials.iter().any(|d| d.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Une tour de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn clock_tower_wake(&mut self, pl: &Place) { let tag = format!("h{}:{}:", pl.x, pl.z); if self.mech.dials.iter().any(|d| d.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}
