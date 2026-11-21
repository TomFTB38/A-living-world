//! Le Colisée oublié : un amphithéâtre de grès à demi ruiné, ses gradins, ses arcades sur deux étages, la loge de l'empereur.
//! L'arène est creusée trois blocs sous les gradins : on y descend par le tunnel du nord, et l'on n'en remonte pas. Trois
//! épreuves, un boss :
//! - **les vagues** (le nouveau mécanisme : les arènes) : qu'on pose le pied sur le sable, la grille du nord retombe et les
//!   cages s'ouvrent ; trois vagues d'adversaires, chacune quand la précédente est tombée. Qu'on en meure, tout est à refaire.
//! - **la porte du Champion** : l'arène conquise, la herse du sud se lève sur l'escalier de l'hypogée.
//! - **le Champion de l'arène** attend dans l'hypogée, sous les gradins, devant le coffre des jeux.
//! Tout est vérifié par `tests/arene.rs`.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Arena, Guard, Hint, Mech, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// Les demi-axes (u, w) : du sable de l'arène, du mur du podium, du dernier gradin, du mur extérieur.
pub const SAND: (f64, f64) = (12.0, 15.0); pub const PODIUM: (f64, f64) = (13.0, 16.0); pub const ROWS: i32 = 6; pub const WALL_H: i32 = 11;
/// Le sable est à `-DROP` sous le sol (on s'y tient à `1 - DROP`) ; le podium est à hauteur du sol.
pub const DROP: i32 = 3;
/// La grille du nord (l'entrée, qui retombe) et la herse du sud (la porte du Champion) : leur rang w.
pub const NORTH_GATE: i32 = -16; pub const SOUTH_GATE: i32 = 16;
/// L'hypogée : (u0, u1, w0, w1, sol) ; le Champion, le coffre.
pub const CRYPT: (i32, i32, i32, i32, i32) = (-8, 8, 24, 33, -10); pub const BOSS: &str = "champion"; pub const BOSS_AT: [f64; 3] = [0.5, -9.0, 29.5]; pub const CHEST: [i32; 3] = [0, -9, 33];
/// D'où sortent les adversaires : devant les quatre cages.
pub const CAGES: [(i32, i32); 4] = [(-8, -10), (8, -10), (-8, 10), (8, 10)];

/// Une case est-elle dans l'ellipse de demi-axes `r` ?
pub fn within(u: i32, w: i32, r: (f64, f64)) -> bool { (u as f64 / r.0).powi(2) + (w as f64 / r.1).powi(2) <= 1.0 }
/// Le rang du gradin (0 au bord du podium, ROWS - 1 en haut) ; `ROWS` : le mur extérieur ; `None` : dedans ou dehors.
pub fn row(u: i32, w: i32) -> Option<i32> { if within(u, w, PODIUM) { return None; } (0..=ROWS).find(|j| within(u, w, (PODIUM.0 + 1.0 + *j as f64, PODIUM.1 + 1.0 + *j as f64))) }
/// Les trois vagues : (espèce, place u, w), à tenir dans l'ordre.
pub fn waves() -> Vec<Vec<(&'static str, (i32, i32))>> {
    vec![vec![("rodeur", CAGES[0]), ("rodeur", CAGES[1]), ("rodeur", CAGES[3])],
         vec![("brigand", CAGES[0]), ("brigand", CAGES[1]), ("archer", CAGES[2]), ("archer", CAGES[3])],
         vec![("brigand_epeiste", CAGES[2]), ("araignee", CAGES[0]), ("araignee", CAGES[1]), ("rodeur", CAGES[3])]]
}
/// Les marches de l'escalier de l'hypogée (w, dessus).
pub fn crypt_steps() -> Vec<(i32, i32)> { (SOUTH_GATE + 1..CRYPT.2).map(|w| (w, -DROP - (w - SOUTH_GATE - 1))).collect() }

struct G<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl G<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn h(&self, u: i32, v: i32, w: i32, k: i32) -> f64 { hash_float(self.x + u + 211 * v, self.z + w, 8111, k) }
    /// Le grès des nouvelles pierres et des vieilles : quelques blocs rongés, rougis.
    fn stone(&self, u: i32, v: i32, w: i32) -> &'static str { let h = self.h(u, v, w, 1); if h < 0.12 { "red_sandstone" } else if h < 0.2 { "cobblestone" } else { "sandstone" } }
}

/// Pose le Colisée dans le plan : `o` est le sol au centre de l'arène.
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut g = G { p, x: o[0], f: o[1] + 1, z: o[2] }; let lo = 1 - DROP; // où l'on se tient sur le sable
    let (ou, ow) = (PODIUM.0 + 1.0 + ROWS as f64, PODIUM.1 + 1.0 + ROWS as f64);
    // le sous-sol : un bloc de grès, pour que l'arène et l'hypogée soient clos quoi qu'il y ait dessous
    for u in -21..=21 { for w in -24..=34 { if !within(u, w, (ou + 1.0, ow + 1.0)) && !(u.abs() <= CRYPT.1 + 1 && w > 0) { continue; } for v in CRYPT.4 - 1..=-2 { let s = g.stone(u, v, w); g.n(u, v, w, s); } } }
    // — L'ARÈNE — le sable, ses taches sombres ; le mur du podium, ses cages ; les braseros du podium
    for u in -13..=13 { for w in -16..=16 { if within(u, w, SAND) { g.n(u, -DROP, w, if g.h(u, 0, w, 2) < 0.06 { "dirt" } else { "sand" }); for v in lo..=8 { g.air(u, v, w); } }
        else if within(u, w, PODIUM) { for v in -DROP..=0 { g.n(u, v, w, if v == 0 { "sandstone_slab_top" } else { "sandstone" }); } } } }
    for u in -13..=13 { for w in -16..=16 { if within(u, w, PODIUM) && !within(u, w, SAND) && CAGES.iter().any(|(cu, cw)| (u - cu.signum() * 9).abs() + (w - cw.signum() * 11).abs() <= 2) { for v in lo..=lo + 1 { g.n(u, v, w, "iron_bars"); } } } }
    for (u, w) in [(-13, 0), (13, 0), (-9, -11), (9, -11), (-9, 11), (9, 11)] { if row(u, w).is_some() || within(u, w, PODIUM) && !within(u, w, SAND) { g.n(u, 1, w, "brazier"); } }
    // — LES GRADINS — six rangs qui montent d'une marche ; le mur extérieur et ses arcades sur deux étages, rongé par le temps
    for u in -21..=21 { for w in -24..=24 { let Some(j) = row(u, w) else { continue };
        if j < ROWS { for v in -1..=j { let s = g.stone(u, v, w); g.n(u, v, w, if v == j { if (j + u.abs()) % 7 == 0 { "sandstone_slab_top" } else { "sandstone" } } else { s }); } for v in j + 1..=j + 4 { g.air(u, v, w); } continue; }
        let a = (u as f64 / ou).atan2(w as f64 / ow); let bay = ((a + std::f64::consts::PI) / std::f64::consts::TAU * 48.0).floor() as i32; let top = |b: i32| { let b = b.rem_euclid(48); if g.h(b, 0, 0, 7) < 0.4 { 4 + (g.h(b, 0, 0, 8) * 6.0) as i32 } else { WALL_H } };
        // une arcade ne tient que si ses deux piliers tiennent : au-dessus du plus bas des deux, elle est tombée
        let top = if bay % 2 == 0 { top(bay).min(top(bay - 1)).min(top(bay + 1)) } else { top(bay) };
        for v in -1..=top { let arch = bay % 2 == 0 && ((0..=2).contains(&v) || (5..=7).contains(&v));
            if arch { g.air(u, v, w); } else { let s = g.stone(u, v, w); g.n(u, v, w, if v == 3 || v == 8 { "sandstone_slab_top" } else if v == WALL_H { "sandstone_slab" } else { s }); } } } }
    // — LE TUNNEL DU NORD — il traverse les gradins, puis descend sur le sable sous la grille de l'entrée
    for w in -24..NORTH_GATE { for u in -1..=1 { g.n(u, -1, w, "sandstone"); for v in 0..=2 { g.air(u, v, w); } } for u in [-2, 2] { if row(u, w).is_some_and(|j| j > 2) { for v in 0..=2 { g.n(u, v, w, "sandstone"); } g.n(u, 3, w, "sandstone_slab_top"); } } }
    for w in -22..NORTH_GATE { if row(0, w).is_some_and(|j| j > 2) { for u in -1..=1 { g.n(u, 3, w, "sandstone"); } } }
    for u in -1..=1 { g.t(u, lo, NORTH_GATE, "sandstone_stairs", 0, -1); for v in lo + 1..=1 { g.air(u, v, NORTH_GATE); } }
    for u in [-3, 3] { g.n(u, 0, -25, "statue_base"); g.n(u, 1, -25, "statue_top"); }
    // — LA HERSE DU SUD ET L'HYPOGÉE — l'escalier sous les gradins ; la salle voûtée, ses colonnes, ses armes, le coffre des jeux
    for u in -1..=1 { g.t(u, lo, SOUTH_GATE, "sandstone_stairs", 0, 1); for v in lo + 1..=1 { g.n(u, v, SOUTH_GATE, "iron_bars"); } }
    for (w, top) in crypt_steps() { for u in -1..=1 { g.t(u, top, w, "sandstone_stairs", 0, 1); for v in top + 1..=top + 3 { g.air(u, v, w); } } }
    let (c0, c1, cw0, cw1, cf) = CRYPT; for u in c0..=c1 { for w in cw0..=cw1 { g.n(u, cf, w, if (u + w).rem_euclid(2) == 0 { "sandstone" } else { "red_sandstone" }); for v in cf + 1..=cf + 6 { g.air(u, v, w); } } }
    for u in [-5, 5] { for w in [26, 31] { g.n(u, cf + 1, w, "column_base"); for v in cf + 2..cf + 6 { g.n(u, v, w, "column_shaft"); } g.n(u, cf + 6, w, "column_capital"); } }
    for (u, w) in [(c0, cw0), (c1, cw0), (c0, cw1), (c1, cw1)] { g.n(u, cf + 1, w, "brazier"); } for w in [27, 29, 31] { g.n(c0, cf + 1, w, "amphora"); g.n(c1, cf + 1, w, "crate"); }
    for u in -2..=2 { g.n(u, cf + 1, cw1, "sandstone"); } g.n(CHEST[0], CHEST[1], CHEST[2], "chest"); g.n(-1, cf + 2, cw1, "statue_base"); g.n(1, cf + 2, cw1, "statue_base");
    // — LA LOGE DE L'EMPEREUR — à l'est, sur les gradins : un tapis rouge, quatre colonnes, un dais
    for u in 15..=18 { for w in -2..=2 { if let Some(j) = row(u, w) { if j < ROWS { g.n(u, j, w, "wool_red"); } } } }
    for (u, w) in [(15, -2), (15, 2), (18, -2), (18, 2)] { if let Some(j) = row(u, w) { for v in j + 1..=8 { g.n(u, v, w, if v == j + 1 { "column_base" } else if v == 8 { "column_capital" } else { "column_shaft" }); } } }
    for u in 15..=18 { for w in -2..=2 { g.n(u, 9, w, "sandstone_slab"); } }
}

/// Les mécanismes du Colisée bâti en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] + 1; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}"); let lo = 1 - DROP;
    let bars = by_name("iron_bars").unwrap_or(0); let gate = |w: i32| -> Vec<([i32; 3], Id)> { (-1..=1).flat_map(|u| (lo + 1..=1).map(move |v| [u, v, w])).map(|c| (at(c), bars)).collect() };
    let mut m = Mech::default();
    let wv = waves().into_iter().map(|w| w.into_iter().map(|(id, (u, ww))| (id.to_string(), [o[0] as f64 + u as f64 + 0.5, (f + lo) as f64, o[2] as f64 + ww as f64 + 0.5])).collect()).collect();
    m.arenas.push(Arena { name: name("arene"), min: at([-11, lo, -14]), max: at([11, lo + 1, 14]), waves: wv, doors: gate(NORTH_GATE), ..Default::default() });
    m.wires.push(Wire { name: name("herse"), needs: vec![name("arene")], latch: true, doors: gate(SOUTH_GATE), ..Default::default() });
    let (c0, c1, cw0, cw1, cf) = CRYPT;
    m.guards.push(Guard { min: at([c0 + 1, cf + 1, cw0 + 1]), max: at([c1 - 1, cf + 5, cw1]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: gate(SOUTH_GATE), state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-21, CRYPT.4 - 1, -25]), max: at([21, WALL_H + 1, 34]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-1, 0, -24], [1, 2, -21], "Le Colisée oublié. Le tunnel descend sur le sable de l'arène : qui y entre n'en ressort que vainqueur, ou mort.");
    hint(&mut m, [-1, lo, SOUTH_GATE + 1], [1, lo + 2, SOUTH_GATE + 3], "L'escalier de l'hypogée. En bas, sous les gradins, le Champion attend son dernier adversaire.");
    m
}

impl Session {
    /// Bâtit ici le Colisée (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol au centre.
    pub fn build_colosseum(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("a{}:{}:", o[0], o[2]); if !self.mech.arenas.iter().any(|a| a.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Un Colisée de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn colosseum_wake(&mut self, pl: &Place) { let tag = format!("a{}:{}:", pl.x, pl.z); if self.mech.arenas.iter().any(|a| a.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}

/// L'étendue du Colisée, de part et d'autre de son centre (x, z) : elle doit tenir dans la portée des empreintes.
pub fn footprint() -> ([i32; 2], [i32; 2]) { let mut p = Plan::new(); carve(&mut p, [0, 80, 0]); p.m.keys().fold(([i32::MAX; 2], [i32::MIN; 2]), |(lo, hi), &(x, _, z)| ([lo[0].min(x), lo[1].min(z)], [hi[0].max(x), hi[1].max(z)])) }
