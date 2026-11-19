//! La Grotte du Roi noyé : une immense caverne sous une doline, un lac noir, un temple sur son île, et trois énigmes dans
//! la même salle, à résoudre dans l'ordre qu'on veut.
//!
//! En surface, une esplanade dallée entoure un puits carré de treize blocs : une colonnade au nord, la statue du roi au sud,
//! des colonnes brisées sur les flancs. Un escalier taillé dans la paroi du puits descend en spirale jusqu'au fond de la
//! caverne, à vingt-huit blocs sous terre, dans la lumière du jour qui tombe du puits.
//!
//! Le temple, sur l'île au milieu du lac, est fermé par une grille et trois chaînes. Chacune tombe quand son énigme est résolue :
//! - **l'eau** : à l'ouest, une citerne à sec. On s'y laisse tomber ; au fond, un levier fait tomber la chaîne, une vanne
//!   remplit la citerne, et l'on remonte à la nage ;
//! - **la paroi** : à l'est, des corniches montent le long d'une falaise (deux trous à sauter) jusqu'à une niche et son levier ;
//! - **le regard** : trois rois de pierre se sont détournés du temple ; on les fait pivoter (clic droit) pour qu'ils le regardent.
//!
//! Les trois chaînes tombées, la grille se lève ; dans le temple, le Roi noyé se lève de son trône d'eau. Vaincu, il laisse
//! le trésor de l'autel. Tout est vérifié par `tests/grotte.rs` avec le solveur de chemins (`sanctuaire::reach`).
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Guard, Hint, Mech, Pool, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::shapes::{facing_of, variant};
use crate::sites::{Place, Plan};
use std::collections::BTreeSet;

/// Le sol de la caverne est à `DEPTH` blocs sous le sol de l'esplanade.
pub const DEPTH: i32 = 28;
/// L'étendue (u, w) de la caverne, autour du centre du puits.
pub const SPAN: ([i32; 2], [i32; 2]) = ([-31, 31], [-11, 35]);
/// Le temple : son centre sur l'île, l'intérieur de la cella (u0, u1, w0, w1, sol, voûte).
pub const TEMPLE: (i32, i32) = (0, 18); pub const CELLA: (i32, i32, i32, i32, i32, i32) = (-3, 3, 14, 22, 2, 7);
/// La porte (trois de large, trois de haut, dans le mur nord de la cella) et les trois chaînes devant elle.
pub const DOOR_W: i32 = 13; pub const CHAINS_W: i32 = 12;
/// La citerne (intérieur u0, u1, w0, w1, fond) ; ses deux leviers : la chaîne de l'eau, la vanne.
pub const CISTERN: (i32, i32, i32, i32, i32) = (-27, -24, 2, 5, -7);
pub const LEVER_WATER: [i32; 3] = [-27, -6, 3]; pub const VALVE: [i32; 3] = [-24, -6, 4];
/// La paroi : les corniches (u, w, dessus), la niche, son levier.
pub const LEDGES: [(i32, i32, i32); 18] = [(26, 3, 1), (26, 4, 2), (26, 5, 3), (26, 7, 3), (26, 8, 4), (26, 9, 5), (26, 10, 6), (26, 12, 6), (26, 13, 7), (26, 14, 8), (26, 15, 9), (26, 16, 10),
    (25, 16, 11), (25, 15, 12), (25, 14, 13), (25, 13, 14), (25, 11, 14), (25, 10, 15)];
pub const LEVER_WALL: [i32; 3] = [28, 16, 10];
/// Les trois rois de pierre : où ils se dressent (le bloc du bas), où ils regardent au départ, où ils doivent regarder.
pub const KINGS: [([i32; 3], (i32, i32), (i32, i32)); 3] = [([-20, 2, 18], (-1, 0), (1, 0)), ([20, 2, 18], (0, 1), (-1, 0)), ([0, 2, 32], (1, 0), (0, -1))];
/// Le Roi noyé, et où il se lève ; le coffre de l'autel.
pub const BOSS: &str = "roi_noye"; pub const BOSS_AT: [f64; 3] = [0.5, 2.0, 18.5]; pub const CHEST: [i32; 3] = [0, 3, 21];

/// La forme de la caverne : dedans si < 1 (une ellipse cabossée).
fn cave_r2(u: i32, w: i32) -> f64 { let (u, w) = (u as f64, w as f64); (u / 30.0).powi(2) + ((w - 12.0) / 22.0).powi(2) + 0.05 * (0.31 * u + 1.7).sin() * (0.27 * w + 0.4).cos() + 0.035 * (0.17 * u - 0.23 * w).sin() }
/// La hauteur de la voûte en (u, w) : une coupole, plus haute au milieu.
fn ceil(u: i32, w: i32) -> Option<i32> { let r2 = cave_r2(u, w); if r2 >= 1.0 { return None; } let h = 6.0 + 18.0 * (1.0 - r2).powf(0.3) + 1.5 * (0.4 * u as f64).sin() * (0.35 * w as f64).sin(); Some((h as i32).min(DEPTH - 5)) }
fn lake(u: i32, w: i32) -> bool { let (u, w) = (u as f64, w as f64); (u / 17.0).powi(2) + ((w - 18.0) / 11.0).powi(2) + 0.04 * (0.5 * u).sin() * (0.4 * w).cos() < 1.0 }
fn island(u: i32, w: i32) -> bool { let (u, w) = (u as f64, (w - 18) as f64); (u / 7.0).powi(2) + (w / 7.0).powi(2) < 1.0 }
/// Le sol (le dessus du bloc) : le lac est creusé à -4, l'eau monte jusqu'à 0.
fn floor(u: i32, w: i32) -> i32 { if lake(u, w) && !island(u, w) { -4 } else { 0 } }

/// Le chantier, en coordonnées de la caverne : u, w autour du puits, v depuis le sol de la caverne.
struct G<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl G<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn set(&self, u: i32, v: i32, w: i32) -> bool { let (x, y, z) = self.at(u, v, w); self.p.m.contains_key(&(x, y, z)) }
    fn rock(&self, u: i32, v: i32, w: i32) -> &'static str { let h = hash_float(self.x + u + 977 * v, self.z + w, 5113, 4); if h < 0.2 { "mossy_stone" } else if h < 0.27 { "granite" } else if h < 0.275 { "glow_moss" } else { "stone" } }
    fn column(&mut self, u: i32, w: i32, v0: i32, v1: i32) { for v in v0..=v1 { self.n(u, v, w, if v == v0 { "column_base" } else if v == v1 { "column_capital" } else { "column_shaft" }); } }
    fn statue(&mut self, u: i32, v: i32, w: i32, back: (i32, i32)) { self.t(u, v, w, "statue_base", back.0, back.1); self.t(u, v + 1, w, "statue_top", back.0, back.1); }
}

/// Le cheminement de l'escalier en spirale, le long de la paroi du puits : (u, w, dessus, coin ?, direction de la marche).
pub fn spiral() -> Vec<(i32, i32, i32, bool, (i32, i32))> {
    let mut ring: Vec<(i32, i32, (i32, i32))> = Vec::new(); for u in -5..5 { ring.push((u, -5, (1, 0))); } for w in -5..5 { ring.push((5, w, (0, 1))); } for u in (-4..=5).rev() { ring.push((u, 5, (-1, 0))); } for w in (-4..=5).rev() { ring.push((-5, w, (0, -1))); }
    let mut out = Vec::new(); let mut top = DEPTH - 1;
    for (i, &(u, w, d)) in ring.iter().enumerate() { let corner = u.abs() == 5 && w.abs() == 5; if i > 0 && !corner { top -= 1; } out.push((u, w, top, corner, d)); if top <= 1 { break; } }
    out
}

/// Pose la grotte dans le plan : `o` est le sol de l'esplanade, au centre du puits.
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut g = G { p, x: o[0], f: o[1] - DEPTH, z: o[2] };
    // la caverne : l'air, l'eau du lac, puis une coque de roche tout autour (aucune fuite vers les grottes voisines)
    let mut inside: BTreeSet<(i32, i32, i32)> = BTreeSet::new();
    for u in SPAN.0[0]..=SPAN.0[1] { for w in SPAN.1[0]..=SPAN.1[1] { let Some(c) = ceil(u, w) else { continue }; let fl = floor(u, w); for v in fl + 1..=c { inside.insert((u, v, w)); } } }
    // la falaise de l'est (sa niche), la citerne, le puits, la place des corniches
    for u in 21..=26 { for w in 1..=19 { for v in 1..=19 { inside.insert((u, v, w)); } } }
    for u in 26..=28 { for w in 9..=11 { for v in 16..=18 { inside.insert((u, v, w)); } } }
    for u in CISTERN.0..=CISTERN.1 { for w in CISTERN.2..=CISTERN.3 { for v in CISTERN.4..=0 { inside.insert((u, v, w)); } } }
    for u in -6..=6 { for w in -6..=6 { for v in 1..=DEPTH + 1 { inside.insert((u, v, w)); } } }
    let cliff = |u: i32, w: i32, v: i32| (27..=29).contains(&u) && (1..=19).contains(&w) && (0..=20).contains(&v) && !((26..=28).contains(&u) && (9..=11).contains(&w) && (16..=18).contains(&v));
    for &(u, v, w) in &inside { if cliff(u, w, v) { continue; } if lake(u, w) && !island(u, w) && v <= 0 { g.n(u, v, w, "water"); } else { g.air(u, v, w); } }
    for &(u, v, w) in &inside { for (du, dv, dw) in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)] { let q = (u + du, v + dv, w + dw);
        if !inside.contains(&q) && !g.set(q.0, q.1, q.2) && q.1 <= DEPTH { let r = if dv == -1 && hash_float(q.0, q.2, 77, 1) < 0.3 { "gravel" } else { g.rock(q.0, q.1, q.2) }; g.n(q.0, q.1, q.2, r); } } }
    for u in 27..=29 { for w in 1..=19 { for v in 0..=20 { if cliff(u, w, v) { let r = g.rock(u, v, w); g.n(u, v, w, r); } } } }
    // la niche de la paroi, son levier ; les corniches, des stalactites sous quelques-unes
    for u in 26..=28 { for w in 9..=11 { g.n(u, 15, w, "mossy_stone"); } } g.t(LEVER_WALL[0], LEVER_WALL[1], LEVER_WALL[2], "lever", 1, 0); g.n(27, 16, 9, "funeral_urn"); g.t(26, 17, 11, "wall_sconce", 0, 1);
    for (k, &(u, w, top)) in LEDGES.iter().enumerate() { g.n(u, top, w, if k % 3 == 0 { "mossy_stone" } else { "stone" }); if k % 4 == 1 && top > 2 { g.n(u, top - 1, w, "stone_point_down_1"); } }
    // la citerne : un fond de pierre, la chaîne de l'eau et la vanne aux murs, une applique
    for u in CISTERN.0..=CISTERN.1 { for w in CISTERN.2..=CISTERN.3 { g.n(u, CISTERN.4 - 1, w, "stone_bricks"); } }
    for v in CISTERN.4 - 1..=0 { for u in CISTERN.0 - 1..=CISTERN.1 + 1 { for w in CISTERN.2 - 1..=CISTERN.3 + 1 { if u < CISTERN.0 || u > CISTERN.1 || w < CISTERN.2 || w > CISTERN.3 { g.n(u, v, w, if (u + w + v) % 4 == 0 { "mossy_stone" } else { "stone_bricks" }); } } } }
    g.t(LEVER_WATER[0], LEVER_WATER[1], LEVER_WATER[2], "lever", -1, 0); g.t(VALVE[0], VALVE[1], VALVE[2], "lever", 1, 0); g.t(-25, -4, 5, "wall_sconce", 0, 1);
    for (u, w) in [(-29, 1), (-22, 1), (-29, 6), (-22, 6)] { g.column(u, w, 1, 4); }
    // l'escalier en spirale : deux de large, une assise sous chaque marche
    for &(u, w, top, corner, d) in &spiral() { let out = (if u.abs() == 5 { u.signum() } else { 0 }, if w.abs() == 5 { w.signum() } else { 0 });
        let cells: Vec<(i32, i32)> = if corner { vec![(u, w), (u + out.0, w), (u, w + out.1), (u + out.0, w + out.1)] } else { vec![(u, w), (u + out.0, w + out.1)] };
        for (cu, cw) in cells { if corner { g.n(cu, top, cw, "stone_bricks"); } else { g.t(cu, top, cw, "stone_bricks_stairs", -d.0, -d.1); } g.n(cu, top - 1, cw, "stone_bricks"); } }
    // le revêtement du puits, de la voûte à la surface
    for u in -7..=7i32 { for w in -7..=7i32 { if u.abs() == 7 || w.abs() == 7 { let c = ceil(u, w).unwrap_or(10); for v in c..=DEPTH - 1 { g.n(u, v, w, if hash_float(u, w + v, 3, 3) < 0.25 { "mossy_stone" } else { "stone_bricks" }); } } } }
    // au fond du puits, un bassin où tombe la lumière
    for u in -2..=2 { for w in -2..=2 { g.n(u, 0, w, "water"); g.n(u, -1, w, "stone_bricks"); } }
    // le lac : un fond de gravier et d'argile, des stalagmites sur les rives
    // le temple : un soubassement sur l'île, une cella, un portique, un toit et son fronton
    for u in -5..=5 { for w in 12..=24 { g.n(u, 1, w, if (u + w) % 5 == 0 { "mossy_stone" } else { "stone_bricks" }); } }
    for u in -4..=4i32 { for w in 13..=23 { for v in 2..=7 { let edge = u.abs() == 4 || w == 13 || w == 23; if edge { g.n(u, v, w, if v == 7 { "sandstone" } else { "ashlar" }); } else { g.air(u, v, w); } } } }
    for u in -5..=5 { for w in 12..=24 { g.n(u, 8, w, "stone_bricks"); } }
    for u in [-5, -3, 3, 5] { g.column(u, 12, 2, 7); } for w in [15, 18, 21] { for u in [-5, 5] { g.column(u, w, 2, 7); } } g.column(-5, 24, 2, 7); g.column(5, 24, 2, 7);
    for u in -4..=4 { g.n(u, 9, 12, "sandstone"); } for u in -2..=2 { g.n(u, 10, 12, "sandstone"); } g.n(0, 11, 12, "sandstone"); for u in -1..=1 { g.t(u, 9, 12, "wall_relief", 0, 1); } g.t(0, 10, 12, "wall_relief", 0, 1);
    for u in -1..=1 { for v in 2..=4 { g.n(u, v, DOOR_W, "iron_bars"); g.n(u, v, CHAINS_W, "great_chain"); } } for u in [-4, 4] { g.n(u, 2, 12, "brazier_stand"); }
    // dans la cella : l'autel et le coffre, deux rois de pierre, des brasiers, l'eau noire du trône
    g.t(CHEST[0], 2, CHEST[2], "altar", 0, 1); g.n(CHEST[0], CHEST[1], CHEST[2], "chest"); for u in [-3, 3] { g.statue(u, 2, 22, (0, 1)); g.n(u, 2, 15, "brazier_stand"); }
    for u in -1..=1 { g.n(u, 1, 18, "water"); } g.n(0, 7, 18, "chandelier");
    // les rois de pierre sur leurs socles, détournés du temple ; des brasiers sur les rives
    for (q, start, _) in KINGS { g.n(q[0], 1, q[2], "chiseled_stone"); g.statue(q[0], q[1], q[2], (-start.0, -start.1)); }
    // des lanternes pendues à la voûte, au bout de leurs chaînes, au-dessus du lac et des rives
    for (u, w) in [(-10, 20), (10, 20), (0, 8), (-14, 28), (14, 28), (-18, 8), (18, 10), (0, 30), (-22, 18)] { if let Some(c) = ceil(u, w) { if c - 4 > floor(u, w) + 3 { for v in c - 3..=c { g.n(u, v, w, "chain"); } g.n(u, c - 4, w, "lantern"); } } }
    for (u, w) in [(-10, 3), (10, 3), (-22, 12), (22, 22), (-12, 31), (12, 31), (-6, 8), (6, 8), (-7, 6), (7, 6), (-17, 26), (17, 4)] { if !lake(u, w) && ceil(u, w).is_some() { g.n(u, 1, w, "brazier_stand"); } }
    // des ruines sur les rives : colonnes brisées, urnes, un sarcophage
    for (u, w, hgt) in [(-14, 6, 3), (-8, 32, 2), (8, 33, 4), (16, 7, 2), (-23, 22, 3)] { if ceil(u, w).is_some_and(|c| c > hgt + 2) && !lake(u, w) { g.column(u, w, 1, hgt); if hgt < 4 { g.n(u, hgt, w, "column_shaft"); } } }
    for (u, w) in [(-13, 7), (9, 32), (-22, 23), (17, 8)] { if !lake(u, w) && ceil(u, w).is_some() { g.n(u, 1, w, "funeral_urn"); } }
    g.t(-6, 1, 31, "tomb_head", 0, 1); g.t(-5, 1, 31, "tomb_foot", 0, 1);
    // stalactites et stalagmites, loin des chemins
    for u in (SPAN.0[0]..=SPAN.0[1]).step_by(3) { for w in (SPAN.1[0]..=SPAN.1[1]).step_by(3) { let h = hash_float(u, w, 919, 2); let Some(c) = ceil(u, w) else { continue }; if u.abs() <= 8 && w <= 8 { continue; }
        let k = ((h * 31.0) as i32) % 3; let busy = (20..=29).contains(&u) || (-30..=-21).contains(&u) && w < 8 || island(u, w) || KINGS.iter().any(|(q, _, _)| (q[0] - u).abs() <= 2 && (q[2] - w).abs() <= 2);
        if h < 0.45 && c > 8 && !(u >= 19 && (0..=20).contains(&w)) { g.n(u, c, w, ["stone_point_down_0", "stone_point_down_1", "stone_point_down_2"][k as usize]); }
        if h > 0.8 && !lake(u, w) && !busy { g.n(u, 1, w, ["stone_point_up_0", "stone_point_up_1", "crystal_blue_point_up_1"][k as usize]); } } }
    // L'esplanade en surface : un dallage autour du puits, une balustrade (ouverte où l'escalier commence), une colonnade
    // au nord sous son linteau, la statue du roi au sud entre deux brasiers, des colonnes brisées sur les flancs.
    let s = DEPTH;
    for u in -11..=11i32 { for w in -11..=11i32 { if u.abs() <= 6 && w.abs() <= 6 { continue; } g.n(u, s, w, if hash_float(u, w, 55, 5) < 0.3 { "mossy_stone" } else { "stone_bricks" }); for v in s - 3..s { if !g.set(u, v, w) { g.n(u, v, w, "stone"); } } for v in s + 1..=s + 12 { g.air(u, v, w); } } }
    for u in -7..=7i32 { for w in -7..=7i32 { if (u.abs() == 7 || w.abs() == 7) && !(u == -7 && (-6..=-4).contains(&w)) && !(w == -7 && (-6..=-4).contains(&u)) { g.n(u, s + 1, w, "stone_bricks_slab"); } } }
    for u in [-8, -4, 0, 4, 8] { g.column(u, -10, s + 1, s + 5); } for u in -9..=9 { g.t(u, s + 6, -10, "lintel", 0, 1); }
    g.n(0, s + 1, 10, "chiseled_stone"); g.n(0, s + 2, 10, "chiseled_stone"); g.statue(0, s + 3, 10, (0, 1)); for u in [-2, 2] { g.n(u, s + 1, 10, "brazier_stand"); }
    for (u, w, hgt) in [(-10, -4, 3), (-10, 1, 2), (-10, 5, 4), (10, -3, 2), (10, 2, 4), (10, 6, 1)] { for v in 0..hgt { g.n(u, s + 1 + v, w, if v == 0 { "column_base" } else { "column_shaft" }); } }
    for (u, w) in [(-9, 8), (9, -8), (-8, -2)] { g.n(u, s + 1, w, "column_shaft"); }
}

/// Les mécanismes de la grotte bâtie en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] - DEPTH; let at = |u: i32, v: i32, w: i32| [o[0] + u, f + v, o[2] + w]; let name = |n: &str| format!("{tag}{n}");
    let chain = by_name("great_chain").unwrap_or(0); let iron = by_name("iron_bars").unwrap_or(0);
    let chain_of = |u: i32| (2..=4).map(|v| (at(u, v, CHAINS_W), chain)).collect::<Vec<_>>();
    let mut m = Mech::default();
    // l'eau : la chaîne au fond de la citerne ; la vanne qui la remplit
    m.wires.push(Wire { name: name("eau"), inputs: vec![at(LEVER_WATER[0], LEVER_WATER[1], LEVER_WATER[2])], latch: true, doors: chain_of(-1), ..Default::default() });
    m.wires.push(Wire { name: name("vanne"), inputs: vec![at(VALVE[0], VALVE[1], VALVE[2])], ..Default::default() });
    let cols: Vec<[i32; 3]> = (CISTERN.0..=CISTERN.1).flat_map(|u| (CISTERN.2..=CISTERN.3).map(move |w| [o[0] + u, o[2] + w, f + CISTERN.4])).collect();
    m.pools.push(Pool { cols, top: f, by: name("vanne"), level: f + CISTERN.4 - 1, t: 0.0, ..Default::default() });
    // la paroi : le levier de la niche
    m.wires.push(Wire { name: name("paroi"), inputs: vec![at(LEVER_WALL[0], LEVER_WALL[1], LEVER_WALL[2])], latch: true, doors: chain_of(0), ..Default::default() });
    // le regard : les trois rois tournés vers le temple
    let want = |d: (i32, i32)| { let b = variant(by_name("statue_base").unwrap_or(0), facing_of(-d.0 as f64, -d.1 as f64), false, false, false); crate::blocks::BLOCKS[b as usize].name.split_once('@').map_or(0, |(_, k)| k.parse().unwrap_or(0)) };
    m.wires.push(Wire { name: name("regard"), faces: KINGS.iter().map(|(q, _, to)| (at(q[0], q[1], q[2]), want(*to))).collect(), latch: true, doors: chain_of(1), ..Default::default() });
    // le temple : les trois chaînes tombées, la grille se lève
    m.wires.push(Wire { name: name("temple"), needs: vec![name("eau"), name("paroi"), name("regard")], latch: true, doors: (-1..=1).flat_map(|u| (2..=4).map(move |v| (u, v))).map(|(u, v)| (at(u, v, DOOR_W), iron)).collect(), ..Default::default() });
    // le Roi noyé
    let doors: Vec<([i32; 3], Id)> = (-1..=1).flat_map(|u| (2..=4).map(move |v| (u, v))).map(|(u, v)| (at(u, v, DOOR_W), iron)).collect();
    m.guards.push(Guard { min: at(CELLA.0, CELLA.4, CELLA.2), max: at(CELLA.1, CELLA.5, CELLA.3), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors, state: 0 });
    m.vaults.push(at(CHEST[0], CHEST[1], CHEST[2]));
    m.seals.push(Seal { min: at(SPAN.0[0] - 1, CISTERN.4 - 2, SPAN.1[0] - 1), max: at(SPAN.0[1] + 1, DEPTH - 1, SPAN.1[1] + 1), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: a, max: b, text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, at(-12, 1, -6), at(12, 4, 8), "La Grotte du Roi noyé. Sur son île, le temple attend derrière trois chaînes : celle de l'eau (la citerne, à l'ouest), celle de la paroi (la falaise, à l'est), celle du regard (trois rois de pierre se sont détournés de leur temple).");
    hint(&mut m, at(CISTERN.0 - 2, -7, CISTERN.2 - 2), at(CISTERN.1 + 2, 1, CISTERN.3 + 2), "Une citerne à sec, profonde de huit blocs (la chute fait mal). Au fond, deux leviers de bronze : l'un retient une chaîne, l'autre ouvre l'eau.");
    hint(&mut m, at(20, 1, 1), at(26, 4, 8), "Des corniches montent le long de la falaise, jusqu'à une niche. Il faudra sauter par-dessus deux brèches.");
    for (q, _, _) in KINGS { hint(&mut m, at(q[0] - 3, 1, q[2] - 3), at(q[0] + 3, 4, q[2] + 3), "Un roi de pierre, le regard perdu. Clic droit : le faire pivoter vers son temple."); }
    m
}

impl Session {
    /// Bâtit ici la grotte (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol de l'esplanade.
    pub fn build_grotto(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("g{}:{}:", o[0], o[2]); if !self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Une grotte de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn grotto_wake(&mut self, pl: &Place) { let tag = format!("g{}:{}:", pl.x, pl.z); if self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}
