//! L'Observatoire céleste : un palais de marbre blanc sous une coupole de cuivre, au milieu d'une cour pavée. Trois épreuves,
//! un boss :
//! - **la rotonde des étoiles** (rez-de-chaussée) : neuf étoiles de pierre sur le dallage, un levier sur chacune. Un levier
//!   allume ou éteint son étoile et ses voisines (devant, derrière, à gauche, à droite). Il faut reproduire la constellation
//!   peinte en or au plafond : la grille du planétaire se lève.
//! - **le planétaire** : une fosse, et au-dessus, le soleil sur son pilier et une planète qui tourne autour, au bout de son bras.
//!   On monte sur la planète quand elle passe au bord du palier ; elle vous emporte jusqu'au palier d'en face (le nouveau
//!   mécanisme : les plates-formes qui emportent). De là, l'escalier, puis la galerie, mènent à la coupole.
//! - **la coupole** : l'Astronome y veille sur la grande lunette. Vaincu, il reste la lunette à tourner vers l'étoile d'or de la
//!   rose des vents dallée au sol : la niche du coffre s'ouvre.
//! La constellation et l'étoile d'or changent d'un observatoire à l'autre. Tout est vérifié par `tests/observatoire.rs`.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Dial, Guard, Hint, Mech, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// Le bâtiment : demi-largeur (u, murs compris), mur du nord, du sud ; le mur entre la rotonde et le planétaire.
pub const HALF: i32 = 8; pub const NORTH: i32 = -15; pub const SOUTH: i32 = 13; pub const MID: i32 = -3;
/// Les neuf étoiles : colonnes (u) et rangées (w) ; une étoile est un bloc posé sur le dallage (v = 1), son levier dessus.
pub const STAR_U: [i32; 3] = [-2, 0, 2]; pub const STAR_W: [i32; 3] = [-11, -9, -7]; pub const CEILING: i32 = 7;
/// Les constellations possibles (rangée du nord d'abord, bit 0 = ouest) et leurs noms.
pub const SKIES: [(&str, [u8; 3]); 6] = [("le Cygne", [0b010, 0b111, 0b101]), ("la Flèche", [0b100, 0b010, 0b001]), ("la Grande Ourse", [0b011, 0b110, 0b100]), ("la Couronne", [0b111, 0b101, 0b000]), ("le Dragon", [0b001, 0b111, 0b100]), ("la Lyre", [0b010, 0b101, 0b111])];
/// Le planétaire : le centre de l'orbite (w), son rayon, ses crans ; le fond de la fosse (le bloc) ; les paliers (w) du nord et du sud.
pub const ORBIT_W: i32 = 5; pub const ORBIT_R: f64 = 4.0; pub const ORBIT_N: u8 = 16; pub const PIT: i32 = -5;
pub const LEDGE_N: [i32; 2] = [-2, -1]; pub const LEDGE_S: [i32; 2] = [11, 12]; pub const PIT_LADDER: i32 = -6;
/// La galerie (le bloc v) qui mène à la porte de la coupole ; la coupole : le bloc du sol, le centre (w), le rayon.
pub const GALLERY: i32 = 8; pub const DOME_W: i32 = -9; pub const DOME_R: f64 = 5.5;
/// La grande lunette : son pivot, son levier ; la niche du coffre et sa grille.
pub const PIVOT: [i32; 3] = [0, 11, DOME_W]; pub const SCOPE_LEVER: [i32; 3] = [3, 10, -13];
pub const CHEST: [i32; 3] = [0, 9, -14]; pub const VAULT_BARS: [[i32; 3]; 2] = [[0, 9, -13], [0, 10, -13]]; pub const DOME_DOOR: [[i32; 3]; 2] = [[0, 9, MID], [0, 10, MID]];
pub const BOSS: &str = "astronome"; pub const BOSS_AT: [f64; 3] = [3.5, 9.0, -6.5];

/// La constellation de cet observatoire (l'indice dans `SKIES`) ; la direction de l'étoile d'or (0 à 7, 0 au nord, dans le sens des
/// aiguilles d'une montre vu d'en haut) ; où pointe la lunette au départ.
pub fn sky(o: [i32; 3]) -> usize { (hash_float(o[0], o[2], 3301, 1) * SKIES.len() as f64) as usize % SKIES.len() }
pub fn star(o: [i32; 3]) -> u8 { (hash_float(o[0], o[2], 3301, 2) * 8.0) as u8 % 8 }
pub fn scope_start(o: [i32; 3]) -> u8 { (star(o) + 2 + (hash_float(o[0], o[2], 3301, 3) * 5.0) as u8) % 8 }
/// L'étoile (colonne, rangée) est-elle dans la constellation ?
pub fn lit(sky: usize, c: usize, r: usize) -> bool { SKIES[sky].1[r] >> c & 1 == 1 }
/// Les étoiles qu'allume ou éteint le levier de l'étoile (c, r) : elle et ses voisines.
pub fn flips(c: usize, r: usize) -> Vec<(usize, usize)> { let mut o = vec![(c, r)]; if c > 0 { o.push((c - 1, r)); } if c < 2 { o.push((c + 1, r)); } if r > 0 { o.push((c, r - 1)); } if r < 2 { o.push((c, r + 1)); } o }
pub fn star_cell(c: usize, r: usize) -> [i32; 3] { [STAR_U[c], 1, STAR_W[r]] }

/// Un cap (0 à 7) : son pas (u, w) pour une longueur `t`.
fn heading(k: u8, t: f64) -> (f64, f64) { let a = k as f64 * std::f64::consts::FRAC_PI_4; (a.sin() * t, -a.cos() * t) }
/// La planète sur le cran `k` : d'abord les neuf cases de son dessus (dans le même ordre à chaque cran : c'est ce qui permet de
/// porter le joueur), son dessous, puis le bras qui la tient au soleil.
pub fn planet(k: u8) -> Vec<([i32; 3], &'static str)> {
    let a = k as f64 / ORBIT_N as f64 * std::f64::consts::TAU; let (cu, cw) = ((a.sin() * ORBIT_R).round() as i32, (ORBIT_W as f64 - a.cos() * ORBIT_R).round() as i32);
    let mut o: Vec<([i32; 3], &'static str)> = Vec::new();
    for du in -1..=1 { for dw in -1..=1 { o.push(([cu + du, 0, cw + dw], if du == 0 && dw == 0 { "brass" } else { "copper_roof" })); } }
    for du in -1..=1 { for dw in -1..=1 { o.push(([cu + du, -1, cw + dw], "brass_dark")); } }
    let mut t = 1.0; while t < ORBIT_R - 1.0 { let c = [(a.sin() * t).round() as i32, -2, (ORBIT_W as f64 - a.cos() * t).round() as i32]; if !o.iter().any(|(q, _)| *q == c) && !(c[0] == 0 && c[2] == ORBIT_W) { o.push((c, "brass_dark")); } t += 0.5; }
    o
}
/// La petite planète à anneau qui tourne haut au-dessus de la fosse (pour le décor), sur le cran `k` (sur 8).
pub fn ringed(k: u8) -> Vec<([i32; 3], &'static str)> {
    let a = -(k as f64) / 8.0 * std::f64::consts::TAU; let (cu, cw) = ((a.sin() * 2.6).round() as i32, (ORBIT_W as f64 - a.cos() * 2.6).round() as i32);
    let mut o = vec![([cu, 7, cw], "star_lit")]; for (du, dw) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] { o.push(([cu + du, 7, cw + dw], "brass")); } o
}
/// La grande lunette pointée au cap `k`, levée à 45 degrés : un tube de laiton, l'oculaire sombre, l'objectif de verre.
pub fn scope(k: u8) -> Vec<([i32; 3], &'static str)> {
    let mut o: Vec<([i32; 3], &'static str)> = Vec::new(); let s = std::f64::consts::FRAC_1_SQRT_2; let mut t = -1.5;
    while t <= 4.51 { let (du, dw) = heading(k, t * s); let c = [PIVOT[0] + du.round() as i32, PIVOT[1] + (t * s).round() as i32, PIVOT[2] + dw.round() as i32];
        if c != PIVOT && !(c[0] == PIVOT[0] && c[2] == PIVOT[2] && c[1] < PIVOT[1]) && !o.iter().any(|(q, _)| *q == c) { o.push((c, if t < -0.6 { "brass_dark" } else if t > 4.0 { "stained_glass" } else { "brass" })); } t += 0.25; }
    o
}

struct G<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl G<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn draw(&mut self, l: &[([i32; 3], &'static str)]) { for (c, b) in l { self.n(c[0], c[1], c[2], b); } }
    fn column(&mut self, u: i32, w: i32, v0: i32, v1: i32) { for v in v0..=v1 { self.n(u, v, w, if v == v0 { "column_base" } else if v == v1 { "column_capital" } else { "column_shaft" }); } }
}
/// Le rayon de la coupole à la hauteur `v` (un tambour droit, puis le dôme).
pub fn dome_r(v: i32) -> f64 { if v <= 12 { DOME_R } else { (DOME_R * DOME_R - ((v - 12) as f64).powi(2)).max(0.0).sqrt() } }
fn dome_d(u: i32, w: i32) -> f64 { ((u * u + (w - DOME_W) * (w - DOME_W)) as f64).sqrt() }

/// Pose l'observatoire dans le plan : `o` est le sol au centre (le dallage est un bloc plus haut).
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut g = G { p, x: o[0], f: o[1] + 1, z: o[2] }; let sk = sky(o); let (gold, k0) = (star(o), scope_start(o));
    // la cour pavée, son muret, ses lanternes ; les deux cadrans solaires de l'allée
    for u in -13..=13i32 { for w in -21..=16i32 { g.n(u, -1, w, if (u.abs() + w.abs()) % 6 == 0 { "night_stone" } else { "stone_bricks" }); for v in 0..=3 { g.air(u, v, w); } } }
    for u in -13..=13i32 { for w in -21..=16i32 { if (u.abs() == 13 || w == 16 || w == -21) && !(u.abs() <= 2 && w == -21) { g.n(u, 0, w, "marble"); if (u + w) % 4 == 0 { g.n(u, 1, w, "lantern"); } } } }
    for su in [-5, 5] { g.column(su, -18, 0, 1); g.n(su, 2, -18, "brass"); }
    // les murs de marbre, des bandeaux de pierre de nuit ; le dallage ; tout le dedans vide (la fosse en bas)
    for u in -HALF..=HALF { for w in NORTH..=SOUTH { let wall = u.abs() == HALF || w == NORTH || w == SOUTH || w == MID; let hall = w > MID;
        let top = if hall { 11 } else { CEILING }; let bottom = if hall && w > LEDGE_N[1] && w < LEDGE_S[0] && !wall { PIT } else { 0 };
        for v in PIT..=top { if wall || v == top { g.n(u, v, w, if v == 0 || v == top || v == 4 { "night_stone" } else { "marble" }); } else if v < bottom { g.n(u, v, w, "stone_bricks"); } else if v == bottom { g.n(u, v, w, if bottom == 0 { if (u + w).rem_euclid(2) == 0 { "marble" } else { "night_stone" } } else { "stone_bricks" }); } else { g.air(u, v, w); } } } }
    // le toit en terrasse du planétaire, son parapet
    for u in -HALF..=HALF { for w in MID..=SOUTH { if (u.abs() == HALF || w == SOUTH) && (u + w).rem_euclid(2) == 0 { g.n(u, 12, w, "marble"); } } }
    // la porte du nord : deux colonnes, un fronton, des marches
    for u in -1..=1 { for v in 1..=3 { g.air(u, v, NORTH); } } for u in -3..=3 { g.n(u, 5, NORTH - 1, "marble"); } for u in -2..=2 { g.n(u, 6, NORTH - 1, "night_stone"); } g.n(0, 7, NORTH - 1, "marble");
    for u in [-3, 3] { g.column(u, NORTH - 1, 1, 4); } for u in -3..=3 { g.n(u, 0, NORTH - 1, "marble"); g.t(u, 0, NORTH - 2, "stone_bricks_stairs", 0, 1); }
    // les hautes fenêtres de la rotonde et du planétaire
    for s in [-1, 1] { for w in [-12, -6, 1, 5, 9] { for v in 2..=5 { if w < MID || v > 1 { g.n(s * HALF, v + if w > MID { 3 } else { 0 }, w, "stained_glass"); } } } }
    // — LA ROTONDE DES ÉTOILES — le plafond de nuit et la constellation d'or ; les neuf étoiles éteintes et leurs leviers
    for u in -HALF + 1..HALF { for w in NORTH + 1..MID { g.n(u, CEILING, w, "night_stone"); } }
    for c in 0..3 { for r in 0..3 { let q = star_cell(c, r); if lit(sk, c, r) { g.n(q[0], CEILING, q[2], "star_lit"); } g.n(q[0], 1, q[2], "star_dark"); g.t(q[0], 2, q[2], "lever", 0, -1); } }
    for (u, w) in [(-6, -14), (6, -14), (-6, -4), (6, -4)] { g.n(u, 1, w, "lantern"); } for u in [-5, 5] { for w in [-12, -6] { g.column(u, w, 1, CEILING - 1); } }
    for u in -1..=1 { for v in 1..=3 { g.n(u, v, MID, "iron_bars"); } }
    // — LE PLANÉTAIRE — les paliers, la fosse et son échelle, le soleil sur son pilier, la planète, la petite planète à anneau
    for w in LEDGE_N[0]..=LEDGE_N[1] { for u in -HALF + 1..HALF { for v in PIT + 1..=0 { g.n(u, v, w, if v == 0 { "marble" } else { "stone_bricks" }); } } }
    for v in PIT + 1..=0 { g.t(PIT_LADDER, v, LEDGE_N[1] + 1, "ladder", 0, -1); }
    for v in PIT + 1..=0 { g.n(0, v, ORBIT_W, "brass"); } for du in -2..=2i32 { for dv in -2..=2i32 { for dw in -2..=2i32 { if du * du + dv * dv + dw * dw <= 3 { g.n(du, 2 + dv, ORBIT_W + dw, "star_lit"); } } } }
    g.draw(&planet(0)); g.draw(&ringed(0)); for (u, w) in [(-6, 0), (6, 0), (-6, 11), (6, 11)] { g.n(u, PIT + 1, w, "lantern"); }
    // l'escalier du palier sud, le long du mur de l'est, puis la galerie jusqu'à la porte de la coupole
    for k in 0..8 { let w = 10 - k; for u in [6, 7] { for v in PIT + 1..=k { g.n(u, v, w, "marble"); } g.t(u, k + 1, w, "stone_bricks_stairs", 0, -1); } }
    for u in 0..=7 { for w in [-2, -1] { g.n(u, GALLERY, w, "marble"); } } for w in 0..=2 { for u in [6, 7] { g.n(u, GALLERY, w, "marble"); } }
    for u in 0..=5 { g.n(u, GALLERY + 1, 0, "iron_bars"); }
    // — LA COUPOLE — le sol de marbre et sa rose des vents, le tambour, le dôme de cuivre ; la porte ; la lunette ; la niche du coffre
    for u in -HALF..=HALF { for w in NORTH..=MID { for v in GALLERY..=19 { let d = dome_d(u, w); let r = dome_r(v);
        if v == GALLERY { if d <= DOME_R + 1.3 { g.n(u, v, w, "marble"); } } else if d <= r && r > 0.0 { g.air(u, v, w); } else if d <= r + 1.3 && d <= DOME_R + 1.3 { g.n(u, v, w, if v <= 12 { "marble" } else { "copper_roof" }); } } } }
    g.n(0, 18, DOME_W, "copper_roof"); g.n(0, 19, DOME_W, "lantern");
    for c in DOME_DOOR { g.air(c[0], c[1], c[2]); }
    for k in 0..8u8 { for t in [2.0, 3.0, 4.0] { let (du, dw) = heading(k, t); g.n(PIVOT[0] + du.round() as i32, GALLERY, PIVOT[2] + dw.round() as i32, if k == gold { "star_lit" } else { "night_stone" }); } }
    for v in GALLERY + 1..PIVOT[1] { g.n(PIVOT[0], v, PIVOT[2], "brass_dark"); } g.n(PIVOT[0], PIVOT[1], PIVOT[2], "iron_block"); g.draw(&scope(k0));
    g.n(SCOPE_LEVER[0], SCOPE_LEVER[1] - 1, SCOPE_LEVER[2], "brass_dark"); g.t(SCOPE_LEVER[0], SCOPE_LEVER[1], SCOPE_LEVER[2], "lever", 0, -1);
    for v in 9..=10 { for u in [-1, 1] { g.n(u, v, CHEST[2], "night_stone"); } } g.n(CHEST[0], CHEST[1], CHEST[2], "chest"); for c in VAULT_BARS { g.n(c[0], c[1], c[2], "iron_bars"); }
    for (u, w) in [(-4, -6), (4, -12)] { g.n(u, GALLERY + 1, w, "lantern"); }
}

/// Les mécanismes de l'observatoire bâti en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] + 1; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let ids = |l: Vec<([i32; 3], &'static str)>| l.into_iter().map(|(c, b)| (at(c), by_name(b).unwrap_or(0))).collect::<Vec<_>>();
    let mut m = Mech::default(); let sk = sky(o); let (dark, light) = (by_name("star_dark").unwrap_or(0), by_name("star_lit").unwrap_or(0));
    let mut stars = Vec::new();
    for c in 0..3 { for r in 0..3 { let q = star_cell(c, r); let n = name(&format!("etoile{c}{r}")); stars.push(n.clone());
        let levers = (0..3).flat_map(|cc| (0..3).map(move |rr| (cc, rr))).filter(|(cc, rr)| flips(*cc, *rr).contains(&(c, r))).map(|(cc, rr)| { let l = star_cell(cc, rr); (at([l[0], 2, l[2]]), 1) }).collect();
        m.dials.push(Dial { name: n, n: 2, state: 0, goal: lit(sk, c, r) as u8, draw: vec![vec![(at(q), dark)], vec![(at(q), light)]], levers, ..Default::default() }); } }
    let iron = by_name("iron_bars").unwrap_or(0);
    m.wires.push(Wire { name: name("etoiles"), needs: stars, latch: true, doors: (-1..=1).flat_map(|u| (1..=3).map(move |v| [u, v, MID])).map(|c| (at(c), iron)).collect(), ..Default::default() });
    m.dials.push(Dial { name: name("planete"), n: ORBIT_N, state: 0, goal: 255, draw: (0..ORBIT_N).map(|k| ids(planet(k))).collect(), auto: 0.45, carry: true, ..Default::default() });
    m.dials.push(Dial { name: name("anneau"), n: 8, state: 0, goal: 255, draw: (0..8).map(|k| ids(ringed(k))).collect(), auto: 0.9, ..Default::default() });
    m.dials.push(Dial { name: name("lunette"), n: 8, state: scope_start(o), goal: star(o), draw: (0..8).map(|k| ids(scope(k))).collect(), levers: vec![(at(SCOPE_LEVER), 1)], ..Default::default() });
    m.wires.push(Wire { name: name("niche"), needs: vec![name("lunette")], latch: true, doors: VAULT_BARS.iter().map(|c| (at(*c), iron)).collect(), ..Default::default() });
    m.guards.push(Guard { min: at([-6, GALLERY + 1, NORTH + 1]), max: at([6, 17, MID - 1]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: DOME_DOOR.iter().map(|c| (at(*c), iron)).collect(), state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-HALF, PIT, NORTH - 2]), max: at([HALF, 19, SOUTH]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-7, 1, -14], [7, 3, -12], &format!("L'Observatoire céleste. Au plafond, en or, {}. Chaque levier allume ou éteint son étoile et ses voisines : rallumez le ciel du plafond.", SKIES[sk].0));
    hint(&mut m, [-7, 1, LEDGE_N[0]], [7, 3, LEDGE_N[1]], "Le planétaire. La planète passe au bord du palier : montez dessus, elle vous emportera.");
    hint(&mut m, [-1, GALLERY + 1, -2], [1, GALLERY + 3, -1], "La coupole et sa grande lunette. Une étoile d'or brille dans la rose des vents du dallage.");
    m
}

impl Session {
    /// Bâtit ici l'observatoire (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol au centre.
    pub fn build_observatory(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("o{}:{}:", o[0], o[2]); if !self.mech.dials.iter().any(|d| d.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Un observatoire de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn observatory_wake(&mut self, pl: &Place) { let tag = format!("o{}:{}:", pl.x, pl.z); if self.mech.dials.iter().any(|d| d.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}
