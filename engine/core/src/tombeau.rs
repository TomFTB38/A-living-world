//! Le Tombeau du Pharaon : en surface, un pylône de grès entre deux obélisques ; un escalier descend, douze blocs plus bas, dans
//! la tombe. Trois épreuves, un boss :
//! - **la salle hypostyle** et ses colonnes, son plafond étoilé : la porte de pierre de l'est ne s'abaisse que si trois dalles
//!   sont pesées à la fois. On n'est qu'un : deux blocs de pierre à pousser, entre les colonnes, jusqu'aux deux dalles du fond.
//! - **l'idole et le rocher roulant** (le nouveau mécanisme) : au bout du couloir de l'est, le Scarabée d'or luit sur son socle.
//!   Qu'on le prenne, le plafond s'ouvre : un rocher tombe et roule dans le couloir. Courir ; ou se jeter dans une des niches.
//! - **le mur fragile** : le rocher traverse la salle hypostyle et se fracasse contre le mur de l'ouest, qui s'effondre sur la
//!   chambre funéraire, cachée jusque-là.
//! - **le Gardien chacal** se lève devant le sarcophage du Pharaon ; derrière, le coffre du trésor.
//! Tout est vérifié par `tests/tombeau.rs`.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Boulder, Guard, Hint, Home, Mech, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// Le sol des salles est à `DEPTH` blocs sous la cour.
pub const DEPTH: i32 = 12;
/// La salle hypostyle (u0, u1, w0, w1, haut) ; le couloir de l'idole (u0, u1) ; la chambre de l'idole (u0, u1, w0, w1, haut) ; la
/// chambre funéraire (u0, u1, w0, w1, haut).
pub const HALL: (i32, i32, i32, i32, i32) = (-10, 10, -16, -2, 7); pub const CORRIDOR: (i32, i32) = (11, 24); pub const SHRINE: (i32, i32, i32, i32, i32) = (25, 31, -13, -5, 6); pub const CRYPT: (i32, i32, i32, i32, i32) = (-22, -12, -14, -4, 6);
/// L'allée du rocher (w de son centre), les niches du couloir (u de la première case de chacune), le puits du plafond (u).
pub const LANE: i32 = -9; pub const NICHES: [i32; 2] = [15, 20]; pub const SHAFT_U: i32 = 30;
/// L'idole, son socle ; la porte de l'est (u) ; le mur fragile (u).
pub const IDOL: [i32; 3] = [28, 2, -9]; pub const DOOR_U: i32 = 11; pub const WEAK_U: i32 = -11;
/// Les trois dalles (deux pour les blocs, une pour soi) et d'où partent les deux blocs.
pub const PLATES: [[i32; 3]; 3] = [[8, 0, -13], [8, 0, -5], [9, 0, -9]]; pub const BLOCKS: [[i32; 3]; 2] = [[-4, 1, -12], [-4, 1, -6]];
/// Les colonnes de la salle hypostyle (u, w).
pub const COLUMNS: [(i32, i32); 8] = [(-7, -13), (-3, -13), (3, -13), (7, -13), (-7, -5), (-3, -5), (3, -5), (7, -5)];
pub const BOSS: &str = "gardien_chacal"; pub const BOSS_AT: [f64; 3] = [-15.5, 1.0, -8.5]; pub const CHEST: [i32; 3] = [-21, 1, -9];

/// La course du rocher : il tombe du puits, puis roule vers l'ouest le long de l'allée, jusqu'au mur fragile.
pub fn path() -> Vec<[i32; 3]> { (2..=9).rev().map(|v| [SHAFT_U, v, LANE]).chain((WEAK_U + 2..SHAFT_U).rev().map(|u| [u, 2, LANE])).collect() }
/// Le mur fragile : les cases qui s'effondrent.
pub fn weak_wall() -> Vec<[i32; 3]> { (LANE - 1..=LANE + 1).flat_map(|w| (1..=3).map(move |v| [WEAK_U, v, w])).collect() }

struct G<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl G<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn h(&self, u: i32, v: i32, w: i32, k: i32) -> f64 { hash_float(self.x + u + 307 * v, self.z + w, 9203, k) }
    /// Une salle : le dallage, l'air, les murs peints (une plinthe de grès, des bandes de couleur, une frise d'or), le plafond.
    fn room(&mut self, (u0, u1, w0, w1, top): (i32, i32, i32, i32, i32), sky: bool) {
        for u in u0 - 1..=u1 + 1 { for w in w0 - 1..=w1 + 1 { let wall = u < u0 || u > u1 || w < w0 || w > w1;
            for v in 0..=top + 1 { if v == 0 { self.n(u, 0, w, if (u + w).rem_euclid(2) == 0 { "sandstone" } else { "terracotta_floor" }); }
                else if v == top + 1 { self.n(u, v, w, if sky { if self.h(u, v, w, 2) < 0.07 { "star_lit" } else { "painted_blue" } } else { "sandstone" }); }
                else if wall { let k = (u + w).rem_euclid(9) / 3; self.n(u, v, w, if v == 1 { "sandstone" } else if v == top { "brass" } else { ["painted_blue", "painted_red", "painted_green"][((k + v) % 3) as usize] }); }
                else { self.air(u, v, w); } } } }
    }
}

/// Pose le tombeau dans le plan : `o` est le sol de la cour (au-dessus de la salle hypostyle).
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut g = G { p, x: o[0], f: o[1] - DEPTH, z: o[2] }; let d = DEPTH;
    // un bloc de grès autour de tout, pour que les salles soient closes quoi qu'il y ait sous terre
    for u in -24..=33 { for w in -18..=0 { for v in -1..=10 { let s = if g.h(u, v, w, 1) < 0.15 { "sandstone" } else { "sandstone" }; g.n(u, v, w, s); } } }
    // — LA SALLE HYPOSTYLE — ses colonnes, ses dalles, ses deux blocs, la porte de l'est ; le mur fragile de l'ouest
    g.room(HALL, true); for (u, w) in COLUMNS { g.n(u, 1, w, "column_base"); for v in 2..HALL.4 { g.n(u, v, w, "column_shaft"); } g.n(u, HALL.4, w, "column_capital"); }
    for q in PLATES { g.n(q[0], q[1], q[2], "pressure_plate"); } for q in BLOCKS { g.n(q[0], q[1], q[2], "push_block"); }
    for (u, w) in [(-10, -16), (10, -16), (-10, -2), (10, -2)] { g.n(u, 1, w, "brazier"); } for w in [-15, -3] { g.n(-10, 1, w, "amphora"); }
    // — LE COULOIR DE L'IDOLE — ses deux niches, sur le côté nord
    for u in CORRIDOR.0..=CORRIDOR.1 { for w in LANE - 1..=LANE + 1 { g.n(u, 0, w, "sandstone"); for v in 1..=3 { g.air(u, v, w); } } g.n(u, 4, LANE, "painted_blue"); }
    for u in CORRIDOR.0 + 1..CORRIDOR.1 { if u % 3 == 0 { g.n(u, 2, LANE + 2, "lantern"); } } for u0 in NICHES { for u in u0..=u0 + 1 { for w in LANE - 3..=LANE - 2 { g.n(u, 0, w, "sandstone"); for v in 1..=2 { g.air(u, v, w); } } } }
    for w in LANE - 1..=LANE + 1 { for v in 1..=3 { g.n(DOOR_U, v, w, "red_sandstone"); } }
    // — LA CHAMBRE DE L'IDOLE — le socle, le Scarabée d'or ; le puits du plafond, d'où tombe le rocher
    g.room(SHRINE, true); g.n(IDOL[0], 1, IDOL[2], "sandstone"); g.n(IDOL[0], IDOL[1], IDOL[2], "gold_idol"); for w in [SHRINE.2, SHRINE.3] { g.n(IDOL[0], 1, w, "statue_base"); g.n(IDOL[0], 2, w, "statue_top"); }
    for w in LANE - 1..=LANE + 1 { for v in 1..=3 { g.air(SHRINE.0 - 1, v, w); } }
    for u in SHAFT_U - 1..=SHAFT_U + 1 { for w in LANE - 1..=LANE + 1 { for v in SHRINE.4 + 1..=10 { g.air(u, v, w); } } }
    for (u, w) in [(SHRINE.0, SHRINE.2), (SHRINE.0, SHRINE.3)] { g.n(u, 1, w, "brazier"); }
    // — LA CHAMBRE FUNÉRAIRE — le sarcophage du Pharaon, ses vases, le coffre du trésor
    g.room(CRYPT, true); for u in -17..=-15 { g.n(u, 1, LANE, "sandstone"); } g.t(-17, 2, LANE, "tomb_head", 1, 0); g.t(-16, 2, LANE, "tomb_foot", 1, 0);
    for (u, w) in [(-21, -13), (-21, -5), (-13, -13), (-13, -5)] { g.n(u, 1, w, "brazier"); } for w in [-12, -11, -7, -6] { g.n(-22, 1, w, "amphora"); } g.n(CHEST[0], CHEST[1], CHEST[2], "chest");
    for (u, w) in [(-19, -14), (-19, -4)] { g.n(u, 1, w, "statue_base"); g.n(u, 2, w, "statue_top"); }
    for q in weak_wall() { g.n(q[0], q[1], q[2], "cracked_wall"); }
    // — L'ESCALIER — de la cour à la salle hypostyle, sous la cour
    for u in -3..=3 { for w in -30..=-17 { for v in -1..d { g.n(u, v, w, "sandstone"); } } }
    for w in -28..=-17 { let top = d - 1 - (w + 28); for u in -1..=1 { g.t(u, top, w, "sandstone_stairs", 0, -1); for v in top + 1..=(top + 4).min(d + 4) { g.air(u, v, w); } } if w % 3 == 0 { g.n(2, top + 2, w, "lantern"); g.n(-2, top + 2, w, "lantern"); } }
    for u in -1..=1 { for v in 1..=4 { g.air(u, v, -17); } }
    // — EN SURFACE — le pylône : deux tours de grès qui s'amincissent, une porte sous un linteau peint ; deux obélisques
    let s = d; for side in [-1i32, 1] { for k in 2..=8 { let u = side * k; for w in -31..=-27 { for h in 0..10 { let (out, deep) = (8 - h / 3, h / 5); if k > out || w < -31 + deep || w > -27 - deep { continue; }
        g.n(u, s + 1 + h, w, if h == 9 { "brass" } else if k == out && h % 3 == 2 { "sandstone" } else if w == -31 + deep && (3..=7).contains(&h) && (k == 4 || k == 6) { "painted_red" } else { "sandstone" }); } } } }
    for u in -1..=1 { for w in -31..=-27 { g.n(u, s, w, "sandstone"); for v in s + 1..=s + 4 { g.air(u, v, w); } g.n(u, s + 5, w, "painted_blue"); g.n(u, s + 6, w, "brass"); } }
    for u in -1..=1 { for w in -28..=-27 { g.air(u, s, w); } }
    for u in [-6, 6] { let w = -33; g.n(u, s + 1, w, "sandstone"); for v in s + 2..=s + 8 { g.n(u, v, w, "sandstone"); } g.n(u, s + 9, w, "brass"); }
}

/// Les mécanismes du tombeau bâti en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] - DEPTH; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let door_id = by_name("red_sandstone").unwrap_or(0); let door: Vec<([i32; 3], Id)> = (LANE - 1..=LANE + 1).flat_map(|w| (1..=3).map(move |v| [DOOR_U, v, w])).map(|c| (at(c), door_id)).collect();
    let mut m = Mech::default();
    // une dalle, un fil ; la porte demande les trois à la fois
    for (k, q) in PLATES.iter().enumerate() { m.wires.push(Wire { name: name(&format!("dalle{k}")), inputs: vec![at(*q)], ..Default::default() }); }
    m.wires.push(Wire { name: name("porte"), needs: (0..PLATES.len()).map(|k| name(&format!("dalle{k}"))).collect(), latch: true, doors: door.clone(), ..Default::default() });
    m.homes.push(Home { min: at([HALL.0, 1, HALL.2]), max: at([HALL.1, HALL.4, HALL.3]), homes: BLOCKS.iter().map(|q| at(*q)).collect(), keep: name("porte") });
    m.boulders.push(Boulder { name: name("rocher"), idol: at(IDOL), path: path().into_iter().map(at).collect(), breaks: weak_wall().into_iter().map(at).collect(), ..Default::default() });
    let (c0, c1, cw0, cw1, ct) = CRYPT;
    m.guards.push(Guard { min: at([c0 + 1, 1, cw0 + 1]), max: at([c1 - 2, ct - 1, cw1 - 1]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: vec![], state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-24, -1, -34]), max: at([33, DEPTH + 11, 0]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-2, 1, -16], [2, 3, -14], "La salle hypostyle du Pharaon. La porte de l'est ne s'abaisse que si trois dalles sont pesées à la fois : deux blocs à pousser, et vous.");
    hint(&mut m, [SHRINE.0, 1, SHRINE.2], [SHRINE.1, 3, SHRINE.3], "Le Scarabée d'or, sur son socle. Au-dessus, le plafond est percé… Qui le prendra devra courir — ou trouver où se mettre à l'abri.");
    hint(&mut m, [WEAK_U - 1, 1, LANE - 1], [WEAK_U - 1, 3, LANE + 1], "La chambre funéraire du Pharaon, murée depuis des siècles.");
    m
}

impl Session {
    /// Bâtit ici le tombeau (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol de la cour.
    pub fn build_tomb(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("p{}:{}:", o[0], o[2]); if !self.mech.boulders.iter().any(|b| b.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Un tombeau de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn tomb_wake(&mut self, pl: &Place) { let tag = format!("p{}:{}:", pl.x, pl.z); if self.mech.boulders.iter().any(|b| b.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}

/// L'étendue du tombeau, de part et d'autre de son centre (x, z) : elle doit tenir dans la portée des empreintes.
pub fn footprint() -> ([i32; 2], [i32; 2]) { let mut p = Plan::new(); carve(&mut p, [0, 80, 0]); p.m.keys().fold(([i32::MAX; 2], [i32::MIN; 2]), |(lo, hi), &(x, _, z)| ([lo[0].min(x), lo[1].min(z)], [hi[0].max(x), hi[1].max(z)])) }
