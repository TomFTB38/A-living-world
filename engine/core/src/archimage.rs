//! La Tour de l'Archimage : en surface, une tour de mage en ruine, et en son cœur une dalle de cristal blanc. Pas d'escalier :
//! les salles, seize blocs sous terre, sont closes de toutes parts ; seuls les portails y mènent (le nouveau mécanisme). Trois
//! épreuves, un boss :
//! - **le hall des portails** : quatre dalles de cristal. Le bleu mène à la bibliothèque, le rose au cachot, le blanc à la
//!   surface ; celle du nord change de couleur selon le cadran du laboratoire : violet (le laboratoire), rose (le cachot), or (le
//!   Sanctuaire).
//! - **la bibliothèque** : sur le lutrin, le grimoire de l'Archimage ; qui en lit la formule (le levier du lutrin) peut passer le
//!   portail d'or. Sans elle, il vacille et s'éteint.
//! - **le laboratoire** et son cadran (un levier : un cran) ; **le cachot**, un piège : les barreaux tombent sur le portail du
//!   retour, et les prisonniers de l'Archimage sortent de l'ombre (une arène à vagues).
//! - **l'Archimage** attend dans son Sanctuaire, devant le coffre de ses trésors.
//! Tout est vérifié par `tests/archimage.rs`.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Arena, Dial, Guard, Hint, Mech, Portal, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// Le sol des salles est à `DEPTH` blocs sous la cour.
pub const DEPTH: i32 = 16;
/// Les salles (u0, u1, w0, w1, haut) : le hall, la bibliothèque, le laboratoire, le cachot, le Sanctuaire.
pub const HALL: (i32, i32, i32, i32, i32) = (-6, 6, -6, 6, 6); pub const LIBRARY: (i32, i32, i32, i32, i32) = (-30, -18, -7, 7, 8); pub const LAB: (i32, i32, i32, i32, i32) = (-6, 6, -30, -18, 6);
pub const CELL: (i32, i32, i32, i32, i32) = (18, 30, -6, 6, 5); pub const SANCTUM: (i32, i32, i32, i32, i32) = (-8, 8, 18, 32, 9);
/// Les dalles du hall (nord, est, sud, ouest), et celle de retour de chaque salle ; la dalle de la tour, en surface.
pub const HALL_N: [i32; 3] = [0, 0, -4]; pub const HALL_E: [i32; 3] = [4, 0, 0]; pub const HALL_S: [i32; 3] = [0, 0, 4]; pub const HALL_W: [i32; 3] = [-4, 0, 0];
pub const LIB_PAD: [i32; 3] = [-19, 0, 0]; pub const LAB_PAD: [i32; 3] = [0, 0, -19]; pub const CELL_PAD: [i32; 3] = [19, 0, 0]; pub const SANCTUM_PAD: [i32; 3] = [0, 0, 19]; pub const TOWER_PAD: [i32; 3] = [0, DEPTH, 0];
/// Le levier du grimoire (sur le lutrin), celui du cadran ; les couleurs du portail du nord, cran par cran.
pub const LECTERN: [i32; 3] = [-28, 1, 0]; pub const DIAL_LEVER: [i32; 3] = [0, 1, -29]; pub const NORTH_COLORS: [&str; 3] = ["crystal_violet", "crystal_pink", "gold_idol"];
pub const BOSS: &str = "archimage"; pub const BOSS_AT: [f64; 3] = [0.5, 1.0, 27.5]; pub const CHEST: [i32; 3] = [0, 1, 32];

/// Où l'on arrive, devant la dalle de retour d'une salle (deux pas vers son centre) ; en surface, à côté de la dalle de la tour.
pub fn arrive(pad: [i32; 3], toward: [i32; 2]) -> [i32; 3] { [pad[0] + 2 * toward[0], pad[1] + 1, pad[2] + 2 * toward[1]] }
/// Les arrivées : dans le hall (au centre), la bibliothèque, le laboratoire, le cachot, le Sanctuaire, la surface.
pub fn arrivals() -> [[i32; 3]; 6] { [[0, 1, 0], arrive(LIB_PAD, [-1, 0]), arrive(LAB_PAD, [0, -1]), arrive(CELL_PAD, [1, 0]), arrive(SANCTUM_PAD, [0, 1]), [0, DEPTH + 1, 2]] }

struct G<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl G<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn h(&self, u: i32, v: i32, w: i32, k: i32) -> f64 { hash_float(self.x + u + 401 * v, self.z + w, 9533, k) }
    /// Une salle close : le dallage, l'air, les murs, la voûte.
    fn room(&mut self, (u0, u1, w0, w1, top): (i32, i32, i32, i32, i32), floor: [&'static str; 2], wall: &'static str) {
        for u in u0 - 1..=u1 + 1 { for w in w0 - 1..=w1 + 1 { let edge = u < u0 || u > u1 || w < w0 || w > w1;
            for v in 0..=top + 1 { if v == 0 { self.n(u, 0, w, floor[((u + w).rem_euclid(2)) as usize]); } else if edge || v == top + 1 { self.n(u, v, w, wall); } else { self.air(u, v, w); } } } }
    }
}

/// Pose la tour dans le plan : `o` est le sol de la cour (au-dessus du hall).
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut g = G { p, x: o[0], f: o[1] - DEPTH, z: o[2] }; let d = DEPTH;
    // un bloc de roche autour de tout : les salles ne s'ouvrent sur rien
    for u in -32..=32 { for w in -32..=34 { for v in -1..=10 { let s = if g.h(u, v, w, 1) < 0.2 { "cobblestone" } else { "stone" }; g.n(u, v, w, s); } } }
    // — LE HALL DES PORTAILS — des piliers, des chaînes, les quatre dalles, un cercle de bougies
    g.room(HALL, ["polished_stone", "stone_bricks_dark"], "stone_bricks_dark");
    for (u, w) in [(-5, -5), (5, -5), (-5, 5), (5, 5)] { g.n(u, 1, w, "column_base"); for v in 2..HALL.4 { g.n(u, v, w, "column_shaft"); } g.n(u, HALL.4, w, "column_capital"); }
    for (q, b) in [(HALL_N, NORTH_COLORS[0]), (HALL_E, "crystal_pink"), (HALL_S, "crystal_white"), (HALL_W, "crystal_blue")] { g.n(q[0], q[1], q[2], b); for (du, dw) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { g.n(q[0] + du, 0, q[2] + dw, "polished_stone"); } }
    for (u, w) in [(-2, -2), (2, -2), (-2, 2), (2, 2)] { g.n(u, 1, w, "candle"); } for v in 4..=HALL.4 { g.n(0, v, 0, "chain"); } g.n(0, 3, 0, "lantern");
    // — LA BIBLIOTHÈQUE — des rayonnages sur tous les murs, des tables de lecture, le lutrin et son grimoire
    g.room(LIBRARY, ["planks", "spruce_planks"], "stone_bricks");
    let (l0, l1, lw0, lw1, lt) = LIBRARY; for u in l0..=l1 { for w in lw0..=lw1 { let wall = u == l0 || u == l1 || w == lw0 || w == lw1; if wall && !(u == l1 && w.abs() <= 1) && !(u == l0 && w.abs() <= 1) { for v in 1..lt { g.n(u, v, w, if v == 4 { "planks" } else { "bookshelf" }); } } } }
    for w in [-4, 4] { for u in [-26, -22] { g.n(u, 1, w, "table"); g.n(u, 2, w, if (u + w) % 3 == 0 { "candle" } else { "books_pile" }); } }
    g.n(LECTERN[0] - 1, 1, 0, "bookshelf"); g.n(LECTERN[0] - 1, 2, 0, "books_pile"); g.t(LECTERN[0], LECTERN[1], LECTERN[2], "lever", 1, 0); g.n(LECTERN[0] - 1, 3, 0, "candle");
    g.n(LIB_PAD[0], 0, LIB_PAD[2], "crystal_white"); for (u, w) in [(-24, -6), (-24, 6)] { g.n(u, 1, w, "lantern"); }
    // — LE LABORATOIRE — des chaudrons, des cristaux qui poussent, des fioles ; le cadran et son levier
    g.room(LAB, ["polished_stone", "cobblestone"], "stone_bricks_dark"); g.n(LAB_PAD[0], 0, LAB_PAD[2], "crystal_white");
    for (u, w) in [(-4, -27), (4, -27), (-4, -22)] { g.n(u, 1, w, "cauldron"); } for (u, w, c) in [(-5, -29, "crystal_violet"), (5, -29, "crystal_blue"), (5, -21, "crystal_pink"), (-5, -20, "crystal_white")] { g.n(u, 1, w, c); g.n(u, 2, w, if c == "crystal_violet" { "crystal_violet_point_up_0" } else if c == "crystal_blue" { "crystal_blue_point_up_0" } else if c == "crystal_pink" { "crystal_pink_point_up_0" } else { "crystal_white_point_up_0" }); }
    for u in -2..=2 { g.n(u, 1, -30, "table"); } g.t(DIAL_LEVER[0], DIAL_LEVER[1], DIAL_LEVER[2], "lever", 0, 1); for (u, c) in [(-2, NORTH_COLORS[0]), (0, NORTH_COLORS[1]), (2, NORTH_COLORS[2])] { g.n(u, 3, -31, c); } g.n(0, 4, -30, "lantern");
    // — LE CACHOT — des barreaux, des chaînes aux murs, de la paille ; la dalle du retour sous sa grille ouverte
    g.room(CELL, ["cobblestone", "gravel"], "cobblestone"); g.n(CELL_PAD[0], 0, CELL_PAD[2], "crystal_white");
    for w in [-6, -2, 2, 6] { for v in 1..=4 { g.n(CELL.1, v, w, "iron_bars"); } } for (u, w) in [(24, -6), (24, 6), (28, -6), (28, 6)] { for v in 2..=4 { g.n(u, v, w, "chain"); } } for (u, w) in [(26, 3), (27, -2)] { g.n(u, 1, w, "hay_bale"); }
    g.n(22, 3, -6, "lantern");
    // — LE SANCTUAIRE — un dallage d'étoiles, un cercle de colonnes, des bannières, le trône de l'Archimage et son coffre
    g.room(SANCTUM, ["stone_bricks_dark", "polished_stone"], "stone_bricks_dark"); g.n(SANCTUM_PAD[0], 0, SANCTUM_PAD[2], "crystal_white");
    for k in 0..8 { let a = k as f64 * std::f64::consts::TAU / 8.0; let (u, w) = ((a.sin() * 6.0).round() as i32, 25 + (a.cos() * 5.0).round() as i32); g.n(u, 1, w, "column_base"); for v in 2..SANCTUM.4 { g.n(u, v, w, "column_shaft"); } g.n(u, SANCTUM.4, w, "column_capital"); }
    for (u, w) in [(-1, 25), (1, 25), (0, 24), (0, 26), (0, 25)] { g.n(u, 0, w, "crystal_violet"); } for u in [-7, 7] { for w in [20, 30] { g.t(u, 5, w, "banner_blue", -u.signum(), 0); g.n(u, 1, w, "brazier"); } }
    for u in -1..=1 { g.n(u, 1, 31, "stone_bricks_dark"); } g.n(CHEST[0], CHEST[1], CHEST[2], "chest");
    // — EN SURFACE — la tour en ruine : un cercle de murs qui s'effondre, une porte au nord, la dalle de cristal au centre
    let s = d; for u in -6..=6i32 { for w in -6..=6i32 { let r = ((u * u + w * w) as f64).sqrt(); if r > 6.4 { continue; } g.n(u, s, w, if r < 1.0 { "crystal_white" } else { "polished_stone" });
        if r > 4.6 { let top = s + 3 + (g.h(u, 0, w, 3) * 7.0) as i32; let door = u.abs() <= 1 && w < 0; for v in s + 1..=top { if door && v <= s + 3 { continue; } g.n(u, v, w, if v == s + 4 || v == s + 8 { "stone_bricks_dark" } else { "stone_bricks" }); } }
        else { for v in s + 1..=s + 4 { g.air(u, v, w); } } } }
    for (u, w) in [(-3, 3), (3, 3)] { g.n(u, s + 1, w, "candle"); } g.n(-3, s + 1, -2, "books_pile");
}

/// Les mécanismes de la tour bâtie en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] - DEPTH; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let to = |q: [i32; 3]| { let c = at(q); [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5] }; let ar = arrivals();
    let mut m = Mech::default();
    m.wires.push(Wire { name: name("grimoire"), inputs: vec![at(LECTERN)], ..Default::default() });
    m.dials.push(Dial { name: name("cadran"), n: 3, state: 0, goal: 255, draw: NORTH_COLORS.iter().map(|c| vec![(at(HALL_N), by_name(c).unwrap_or(0))]).collect(), levers: vec![(at(DIAL_LEVER), 1)], ..Default::default() });
    let p = |m: &mut Mech, n: &str, pad: [i32; 3], dest: Vec<[i32; 3]>, dial: &str, locked: Vec<String>| m.portals.push(Portal { name: name(n), pad: at(pad), to: dest.into_iter().map(to).collect(), dial: dial.to_string(), locked, t: 0.0 });
    p(&mut m, "tour", TOWER_PAD, vec![ar[0]], "", vec![]);
    p(&mut m, "hall_ouest", HALL_W, vec![ar[1]], "", vec![]); p(&mut m, "hall_est", HALL_E, vec![ar[3]], "", vec![]); p(&mut m, "hall_sud", HALL_S, vec![ar[5]], "", vec![]);
    p(&mut m, "hall_nord", HALL_N, vec![ar[2], ar[3], ar[4]], &name("cadran"), vec![String::new(), String::new(), name("grimoire")]);
    for (n, pad) in [("biblio", LIB_PAD), ("labo", LAB_PAD), ("cachot", CELL_PAD), ("sanctuaire", SANCTUM_PAD)] { p(&mut m, n, pad, vec![ar[0]], "", vec![]); }
    // le cachot : les barreaux tombent sur la dalle du retour, deux vagues de prisonniers
    let bars = by_name("iron_bars").unwrap_or(0); let (c0, c1, cw0, cw1, _) = CELL; let spot = |u: i32, w: i32| [o[0] as f64 + u as f64 + 0.5, (f + 1) as f64, o[2] as f64 + w as f64 + 0.5];
    m.arenas.push(Arena { name: name("cachot"), min: at([c0 + 1, 1, cw0]), max: at([c1, 2, cw1]), doors: (1..=2).map(|v| (at([CELL_PAD[0], v, CELL_PAD[2]]), bars)).collect(),
        waves: vec![vec![("rodeur".into(), spot(27, -4)), ("rodeur".into(), spot(27, 4))], vec![("brigand".into(), spot(28, 0)), ("rodeur".into(), spot(26, -5))]], ..Default::default() });
    let (s0, s1, sw0, sw1, st) = SANCTUM;
    m.guards.push(Guard { min: at([s0, 1, sw0 + 3]), max: at([s1, st - 1, sw1]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: vec![], state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-32, -1, -32]), max: at([32, DEPTH + 11, 34]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-2, 1, -2], [2, 3, 2], "Le hall des portails. Tenez-vous sur une dalle de cristal : bleu, rose, blanc… Celle du nord change de couleur.");
    hint(&mut m, [l0(), 1, -7], [-20, 3, 7], "La bibliothèque de l'Archimage. Sur le lutrin, son grimoire : « Le cadran du laboratoire règle le portail du nord. Violet, rose, or : l'or mène à mon Sanctuaire — et seul qui a lu ma formule le passe. »");
    hint(&mut m, [-6, 1, -30], [6, 3, -22], "Le laboratoire. Le levier fait tourner le cadran : un cran à chaque fois.");
    m
}
fn l0() -> i32 { LIBRARY.0 }

impl Session {
    /// Bâtit ici la tour (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol de la cour.
    pub fn build_mage_tower(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("z{}:{}:", o[0], o[2]); if !self.mech.portals.iter().any(|p| p.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Une tour de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn mage_tower_wake(&mut self, pl: &Place) { let tag = format!("z{}:{}:", pl.x, pl.z); if self.mech.portals.iter().any(|p| p.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}

/// L'étendue de la tour, de part et d'autre de son centre (x, z) : elle doit tenir dans la portée des empreintes.
pub fn footprint() -> ([i32; 2], [i32; 2]) { let mut p = Plan::new(); carve(&mut p, [0, 80, 0]); p.m.keys().fold(([i32::MAX; 2], [i32::MIN; 2]), |(lo, hi), &(x, _, z)| ([lo[0].min(x), lo[1].min(z)], [hi[0].max(x), hi[1].max(z)])) }
