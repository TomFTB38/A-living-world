//! Les Catacombes des Rois : un mausolée au bout d'une allée de rois de pierre ; dessous, un ossuaire en labyrinthe, la
//! Crypte des Rois et son charnier, la Porte des Morts, le Couloir des Lames, et le tombeau du Premier Roi.
//!
//! En surface, une allée dallée monte entre huit rois de pierre et des tombes jusqu'au mausolée : un portique à colonnes, un
//! fronton sculpté, un toit à degrés. Dedans, un escalier descend à seize blocs sous terre, dans le vestibule.
//!
//! Du vestibule, deux galeries d'ossuaire — des murs de crânes, des couloirs de deux de large, des impasses — mènent, à
//! l'est comme à l'ouest, à la Crypte des Rois. Plusieurs énigmes, dans la même salle et autour :
//! - **le charnier** coupe la crypte : on le franchit sur des passerelles de pierre friable, qui ne portent qu'un passage ; qui
//!   tombe, tombe sur des pics, et remonte par une échelle ;
//! - **l'ordre des règnes** : quatre souverains dorment dans la crypte, chacun sous sa statue, un levier devant son tombeau.
//!   On les abaisse dans l'ordre où ils ont régné (un ordre propre à chaque catacombe) ; une erreur relève tous les leviers et
//!   fait tirer les fléchettes. Cet ordre, on le lit sur leurs épitaphes, gravées au fond des impasses de l'ossuaire ;
//! - **le passage secret** : un levier rouillé, dans une impasse de la galerie de l'est, fait pivoter un pan du mur de crânes
//!   au bout de la galerie de l'ouest : une chambre au trésor.
//!
//! L'ordre trouvé, les chaînes de la Porte des Morts tombent ; derrière, le Couloir des Lames (des pics qui jaillissent en
//! vague, des fléchettes) mène au tombeau du Premier Roi, qui se lève de son sarcophage. Vaincu, il laisse son sceptre, et le
//! trésor de l'autel. Tout est vérifié par `tests/catacombes.rs` avec le solveur de chemins (`sanctuaire::reach`).
use crate::blocks::{by_name, Id, BLOCKS};
use crate::mecanismes::{Guard, Hint, Mech, Pulse, Seal, Wire};
use crate::noise::{hash_float, hash_int, Rng};
use crate::session::Session;
use crate::shapes::{facing_of, variant};
use crate::sites::{Place, Plan};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Le sol des catacombes est à `DEPTH` blocs sous le sol de l'allée.
pub const DEPTH: i32 = 16;
/// Les galeries de l'ossuaire : `NX` × `NZ` cases de deux sur deux, au pas de quatre (deux de couloir, deux de mur).
pub const NX: i32 = 6; pub const NZ: i32 = 9;
/// Le coin d'une case (i, j) de la galerie de l'est (celle de l'ouest en est le reflet : u → -u).
pub fn cell(i: i32, j: i32) -> (i32, i32) { (7 + 4 * i, -19 + 4 * j) }
/// La Crypte des Rois (intérieur u0, u1, w0, w1, voûte), le charnier qui la coupe (w0, w1, fond).
pub const CRYPT: (i32, i32, i32, i32, i32) = (-14, 14, -33, -24, 8); pub const TRENCH: (i32, i32, i32) = (-28, -26, -3);
/// Les passerelles friables (u) ; les échelles du charnier (u).
pub const BRIDGES: [i32; 3] = [-8, 0, 8]; pub const LADDERS: [i32; 2] = [-12, 12];
/// Les quatre souverains : leur nom, ce qu'en dit leur épitaphe, la tête de leur tombeau (u ; le pied est en u + 1).
pub const KINGS: [(&str, &str, i32); 4] = [
    ("Ervald aux Mains d'Or", "Il fit creuser ces catacombes pour que nul roi ne dorme seul.", -10),
    ("Mathilde la Sage", "Elle fit la paix avec les peuples des collines.", -4),
    ("Hugon le Hardi", "Il tomba l'épée à la main, aux portes de la cité.", 2),
    ("Aubrée la Pieuse", "Elle fit graver dans la pierre le nom de ses pères.", 8),
];
/// Le levier devant le tombeau du souverain `k`.
pub fn lever(k: usize) -> [i32; 3] { [KINGS[k].2, 1, -29] }
/// Les lance-fléchettes de la crypte (une erreur dans l'ordre les fait tirer).
pub const CRYPT_DARTS: [[i32; 3]; 4] = [[-1, 2, -34], [0, 2, -34], [-15, 2, -31], [15, 2, -31]];
/// Le vestibule (intérieur u0, u1, w0, w1, voûte) ; la Porte des Morts (ses chaînes, en w).
pub const HALL: (i32, i32, i32, i32, i32) = (-4, 4, 13, 18, 5); pub const GATE_W: i32 = 19;
/// Le Couloir des Lames (w0, w1), ses rangs de pics, ses lance-fléchettes.
pub const BLADES: (i32, i32) = (20, 26); pub const SPIKE_ROWS: [i32; 3] = [21, 23, 25]; pub const BLADE_DARTS: [[i32; 3]; 2] = [[-2, 2, 22], [2, 2, 24]];
/// Le tombeau du Premier Roi (intérieur u0, u1, w0, w1, voûte) ; la grille qui retombe derrière soi (en w).
pub const TOMB: (i32, i32, i32, i32, i32) = (-9, 9, 28, 36, 7); pub const TOMB_DOOR_W: i32 = 27;
pub const BOSS: &str = "premier_roi"; pub const BOSS_AT: [f64; 3] = [0.5, 1.0, 30.5]; pub const CHEST: [i32; 3] = [0, 2, 36];
/// La chambre secrète (intérieur u0, u1, w0, w1), son pan de mur (au bout de la galerie de l'ouest), son coffre.
pub const SECRET: (i32, i32, i32, i32) = (-35, -31, 0, 3); pub const SECRET_CHEST: [i32; 3] = [-35, 1, 2];
fn pane() -> Vec<[i32; 3]> { let mut v = Vec::new(); for u in [-30, -29] { for w in 1..=2 { for h in 1..=2 { v.push([u, h, w]); } } } v }

/// L'ordre des règnes de ces catacombes : le rang (0 : le premier) de chaque souverain.
pub fn reign(o: [i32; 3]) -> [usize; 4] {
    let mut r = Rng::new(hash_int(o[0], o[2], 4241, 9)); let mut p = [0usize, 1, 2, 3];
    for i in (1..4).rev() { let j = (r.next() * (i + 1) as f64) as usize % (i + 1); p.swap(i, j); }
    let mut rank = [0usize; 4]; for (r, k) in p.iter().enumerate() { rank[*k] = r; } rank
}
const ORD: [&str; 4] = ["en premier", "en deuxième", "en troisième", "en dernier"];

/// Une galerie de l'ossuaire : ses passages (entre cases voisines), ses chapelles (une case, et le côté fermé où s'ouvre la
/// niche du tombeau), et — à l'est — l'impasse du levier secret (une case, le mur où il est fixé). En coordonnées de l'est.
#[derive(Clone, Debug)]
pub struct Wing { pub open: BTreeSet<((i32, i32), (i32, i32))>, pub chapels: Vec<((i32, i32), (i32, i32))>, pub lever: Option<((i32, i32), (i32, i32))> }
const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
/// L'entrée (du vestibule) et la sortie (vers la crypte) d'une galerie.
pub const ENTRY: (i32, i32) = (0, NZ - 1); pub const EXIT: (i32, i32) = (0, 0);
fn key(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) { if a < b { (a, b) } else { (b, a) } }
impl Wing {
    pub fn linked(&self, a: (i32, i32), b: (i32, i32)) -> bool { self.open.contains(&key(a, b)) }
    /// Les côtés fermés d'une case (sans l'entrée ni la sortie).
    fn closed(&self, c: (i32, i32)) -> Vec<(i32, i32)> { DIRS.iter().copied().filter(|d| !self.linked(c, (c.0 + d.0, c.1 + d.1)) && !(c == ENTRY && *d == (-1, 0)) && !(c == EXIT && *d == (0, -1))).collect() }
}
/// La galerie `side` (1 : l'est, -1 : l'ouest) des catacombes bâties en `o` : un labyrinthe parfait, tiré au hasard.
pub fn wing(o: [i32; 3], side: i32) -> Wing {
    let mut r = Rng::new(hash_int(o[0], o[2], 7717, side + 2)); let mut open = BTreeSet::new(); let mut seen = BTreeSet::new();
    let mut stack = vec![ENTRY]; seen.insert(ENTRY);
    while let Some(&c) = stack.last() {
        let nb: Vec<(i32, i32)> = DIRS.iter().map(|d| (c.0 + d.0, c.1 + d.1)).filter(|n| n.0 >= 0 && n.0 < NX && n.1 >= 0 && n.1 < NZ && !seen.contains(n)).collect();
        if nb.is_empty() { stack.pop(); continue; } let n = nb[(r.next() * nb.len() as f64) as usize % nb.len()]; open.insert(key(c, n)); seen.insert(n); stack.push(n);
    }
    let mut w = Wing { open, chapels: Vec::new(), lever: None };
    // les impasses, de la plus lointaine (à pied, depuis l'entrée) à la plus proche
    let mut dist: BTreeMap<(i32, i32), i32> = BTreeMap::new(); let mut q = VecDeque::from([ENTRY]); dist.insert(ENTRY, 0);
    while let Some(c) = q.pop_front() { for d in DIRS { let n = (c.0 + d.0, c.1 + d.1); if w.linked(c, n) && !dist.contains_key(&n) { dist.insert(n, dist[&c] + 1); q.push_back(n); } } }
    let secret_cell = (NX - 1, 5);
    let mut cand: Vec<((i32, i32), usize)> = dist.keys().copied().filter(|c| *c != ENTRY && *c != EXIT && !(side < 0 && *c == secret_cell) && !w.closed(*c).is_empty()).map(|c| (c, 4 - w.closed(c).len())).collect();
    cand.sort_by_key(|(c, links)| (*links.min(&3), -dist[c], *c));
    let mut pick: Vec<(i32, i32)> = Vec::new(); for (c, _) in &cand { if pick.len() >= 3 { break; } if pick.iter().all(|p| (p.0 - c.0).abs() + (p.1 - c.1).abs() > 1) { pick.push(*c); } }
    let side_of = |w: &Wing, c: (i32, i32)| { let cl = w.closed(c); *cl.iter().find(|d| !(side < 0 && c == secret_cell && **d == (1, 0))).unwrap_or(&cl[0]) };
    for c in pick.iter().take(2) { let d = side_of(&w, *c); w.chapels.push((*c, d)); }
    if side > 0 { if let Some(c) = pick.get(2) { let d = side_of(&w, *c); w.lever = Some((*c, d)); } }
    w
}
/// Le levier secret (dans la galerie de l'est), en coordonnées des catacombes.
pub fn secret_lever(o: [i32; 3]) -> [i32; 3] { let w = wing(o, 1); let (c, d) = w.lever.expect("une impasse pour le levier"); let (u, ww) = cell(c.0, c.1); let (du, dw) = (if d.0 > 0 { 1 } else { 0 }, if d.1 > 0 { 1 } else { 0 }); [u + du, 2, ww + dw] }
/// La chapelle du souverain `k` : sa case (coin, en coordonnées des catacombes) ; les rois 0 et 1 dorment à l'ouest, 2 et 3 à l'est.
pub fn chapel(o: [i32; 3], k: usize) -> ((i32, i32), (i32, i32)) { let side = if k < 2 { -1 } else { 1 }; let w = wing(o, side); let (c, d) = w.chapels[k % 2]; let (u, ww) = cell(c.0, c.1); ((if side > 0 { u } else { -u - 1 }, ww), (d.0 * side, d.1)) }

/// Le chantier, en coordonnées des catacombes : u, w autour du centre du mausolée, v depuis le sol des catacombes.
struct C<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl C<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn is_air(&self, u: i32, v: i32, w: i32) -> bool { let (x, y, z) = self.at(u, v, w); self.p.m.get(&(x, y, z)) == Some(&-1) }
    fn h(&self, u: i32, v: i32, w: i32, k: i32) -> f64 { hash_float(self.x + u + 613 * v, self.z + w, 8111, k) }
    /// Une pierre de mur : brique sombre, parfois moussue ou fendue d'ossements.
    fn wall(&self, u: i32, v: i32, w: i32) -> &'static str { let h = self.h(u, v, w, 1); if h < 0.1 { "mossy_stone" } else if h < 0.16 { "bone" } else if h < 0.3 { "stone_bricks" } else { "stone_bricks_dark" } }
    fn floor(&self, u: i32, w: i32) -> &'static str { let h = self.h(u, 0, w, 2); if h < 0.15 { "gravel" } else if h < 0.35 { "cobblestone" } else { "stone_bricks" } }
    /// Une boîte pleine (murs, sol, voûte), puis l'air de son intérieur.
    fn room(&mut self, u0: i32, u1: i32, w0: i32, w1: i32, v1: i32) {
        for u in u0 - 1..=u1 + 1 { for w in w0 - 1..=w1 + 1 { for v in -1..=v1 + 1 { let inside = u >= u0 && u <= u1 && w >= w0 && w <= w1 && v >= 1 && v <= v1;
            if inside { self.air(u, v, w); } else if v <= 0 { let f = if v == 0 { self.floor(u, w) } else { "stone" }; self.n(u, v, w, f); } else if v == v1 + 1 { self.n(u, v, w, "stone_bricks_dark"); } else { let s = self.wall(u, v, w); self.n(u, v, w, s); } } } }
    }
    fn column(&mut self, u: i32, w: i32, v0: i32, v1: i32) { for v in v0..=v1 { self.n(u, v, w, if v == v0 { "column_base" } else if v == v1 { "column_capital" } else { "column_shaft" }); } }
    fn statue(&mut self, u: i32, v: i32, w: i32, back: (i32, i32)) { self.t(u, v, w, "statue_base", back.0, back.1); self.t(u, v + 1, w, "statue_top", back.0, back.1); }
    /// Un tombeau sur deux blocs, dos `back` : la tête en `a`, le pied à côté.
    fn tomb(&mut self, a: (i32, i32), v: i32, back: (i32, i32)) { self.t(a.0, v, a.1, "tomb_head", back.0, back.1); self.t(a.0 + back.1, v, a.1 - back.0, "tomb_foot", back.0, back.1); }
    /// Habille de crânes les murs d'une salle (de v0 à v1), là où ils regardent l'air.
    fn ossuary(&mut self, u0: i32, u1: i32, w0: i32, w1: i32, v0: i32, v1: i32) {
        for u in u0 - 1..=u1 + 1 { for w in w0 - 1..=w1 + 1 { let edge = u < u0 || u > u1 || w < w0 || w > w1; if !edge { continue; } for v in v0..=v1 {
            if self.is_air(u, v, w) { continue; } let (x, y, z) = self.at(u, v, w); let cur = self.p.m.get(&(x, y, z)).copied().unwrap_or(0); if cur > 0 && !matches!(BLOCKS[cur as usize].name, "stone_bricks_dark" | "stone_bricks" | "mossy_stone" | "bone") { continue; }
            if let Some(d) = DIRS.iter().find(|d| self.is_air(u + d.0, v, w + d.1)) { if self.h(u, v, w, 3) < 0.8 { self.t(u, v, w, "ossuary_wall", -d.0, -d.1); } } } } }
    }
}

/// Pose les catacombes dans le plan : `o` est le sol de l'allée, au centre du mausolée.
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut c = C { p, x: o[0], f: o[1] - DEPTH, z: o[2] }; let s = DEPTH;
    // le vestibule, sous le pied de l'escalier ; sa porte des morts au sud ; ses deux portes vers l'ossuaire
    let (a0, a1, b0, b1, hv) = HALL; c.room(a0, a1, b0, b1, hv);
    // les deux galeries de l'ossuaire : un bloc de murs, puis les couloirs creusés
    for side in [-1, 1] { let wg = wing(o, side); let mut air: BTreeSet<(i32, i32)> = BTreeSet::new();
        for i in 0..NX { for j in 0..NZ { let (u, w) = cell(i, j); for du in 0..2 { for dw in 0..2 { air.insert((u + du, w + dw)); } }
            if i + 1 < NX && wg.linked((i, j), (i + 1, j)) { for du in 2..4 { for dw in 0..2 { air.insert((u + du, w + dw)); } } }
            if j + 1 < NZ && wg.linked((i, j), (i, j + 1)) { for du in 0..2 { for dw in 2..4 { air.insert((u + du, w + dw)); } } } } }
        let (eu, ew) = cell(ENTRY.0, ENTRY.1); for du in -2..0 { for dw in 0..2 { air.insert((eu + du, ew + dw)); } }
        let (xu, xw) = cell(EXIT.0, EXIT.1); for du in 0..2 { for dw in -3..0 { air.insert((xu + du, xw + dw)); } }
        for ue in 5..=30 { for w in -22..=16 { let u = ue * side; if air.contains(&(ue, w)) { for v in 1..=3 { c.air(u, v, w); } } else { for v in 1..=3 { let m = c.wall(u, v, w); c.n(u, v, w, m); } }
            let fl = c.floor(u, w); c.n(u, 0, w, fl); c.n(u, -1, w, "stone"); c.n(u, 4, w, "stone_bricks_dark"); } }
        // les murs de crânes, tournés vers les couloirs
        for ue in 5..=30 { for w in -22..=16 { if air.contains(&(ue, w)) { continue; } for v in 1..=3 { let u = ue * side;
            if let Some(d) = DIRS.iter().find(|d| air.contains(&(ue + d.0, w + d.1))) { let h = c.h(u, v, w, 3); if (v == 2 && h < 0.85) || h < 0.4 { c.t(u, v, w, "ossuary_wall", -d.0 * side, -d.1); } } } } }
        // les chapelles : une niche dans le mur, le gisant, une épitaphe au-dessus, une chandelle
        for (cc, d) in &wg.chapels { let (u, w) = cell(cc.0, cc.1); let d = (d.0 * side, d.1);
            let cells: Vec<(i32, i32)> = if d.0 != 0 { let ue = if d.0 * side > 0 { u + 2 } else { u - 1 }; vec![(ue * side, w), (ue * side, w + 1)] } else { let ww = if d.1 > 0 { w + 2 } else { w - 1 }; vec![(u * side, ww), ((u + 1) * side, ww)] };
            for (nu, nw) in &cells { for v in 1..=2 { c.air(*nu, v, *nw); } c.n(*nu, 3, *nw, "stone_bricks_dark"); }
            let head = if (cells[0].0 + d.1, cells[0].1 - d.0) == cells[1] { cells[0] } else { cells[1] }; c.tomb(head, 1, d);
            let foot = if head == cells[0] { cells[1] } else { cells[0] }; c.n(foot.0, 2, foot.1, "candle"); c.t(head.0, 3, head.1, "wall_relief", d.0, d.1);
            let (iu, iw) = (if side > 0 { u } else { -u - 1 }, w); c.n(iu + if d.0 > 0 { 0 } else { 1 }, 1, iw + if d.1 > 0 { 0 } else { 1 }, "bone_pile"); }
        if let Some((cc, d)) = wg.lever { let (u, w) = cell(cc.0, cc.1); let (du, dw) = (if d.0 > 0 { 1 } else { 0 }, if d.1 > 0 { 1 } else { 0 }); c.t((u + du) * side, 2, w + dw, "lever", d.0 * side, d.1); c.n(u * side + if d.0 * side > 0 { -1 } else { 1 } * 0, 1, w + 1 - dw, "bone_pile"); }
        // des appliques çà et là, des toiles d'araignée, des ossements
        for i in 0..NX { for j in 0..NZ { let (u, w) = cell(i, j); let h = c.h(u * side, 7, w, 4); let (iu, iw) = (if side > 0 { u } else { -u - 1 }, w);
            if wg.chapels.iter().any(|(cc, _)| *cc == (i, j)) || wg.lever.is_some_and(|(cc, _)| cc == (i, j)) || (i, j) == ENTRY || (side < 0 && (i, j) == (NX - 1, 5)) { continue; }
            if (i + j) % 3 == 0 { if let Some(d) = DIRS.iter().find(|d| !wg.linked((i, j), (i + d.0, j + d.1)) && !((i, j) == EXIT && **d == (0, -1))) { let (du, dw) = (if d.0 > 0 { 1 } else { 0 }, if d.1 > 0 { 1 } else { 0 }); c.t((u + du) * side, 2, w + dw, "wall_sconce", d.0 * side, d.1); continue; } }
            if h < 0.25 { c.n(iu + (h * 8.0) as i32 % 2, 3, iw + 1, "cobweb"); } else if h > 0.8 { c.n(iu + 1, 1, iw, "bone_pile"); } } }
    }
    // la chambre secrète, derrière le pan de mur, au bout de la galerie de l'ouest
    let (s0, s1, t0, t1) = SECRET; c.room(s0, s1, t0, t1, 3); c.ossuary(s0, s1, t0, t1, 2, 3); for q in pane() { let m = if q[1] == 2 && q[0] == -29 { "" } else { "stone_bricks_dark" }; if m.is_empty() { c.t(q[0], q[1], q[2], "ossuary_wall", -1, 0); } else { c.n(q[0], q[1], q[2], m); } }
    c.n(SECRET_CHEST[0], SECRET_CHEST[1], SECRET_CHEST[2], "chest"); for (u, w) in [(-35, 0), (-35, 3), (-31, 0)] { c.n(u, 1, w, "funeral_urn"); } c.n(-33, 1, 3, "bone_pile"); c.t(-35, 2, 1, "wall_sconce", -1, 0);
    // la Crypte des Rois : ses murs, son charnier, ses passerelles et ses échelles
    let (u0, u1, w0, w1, hv) = CRYPT; c.room(u0, u1, w0, w1, hv); for u in u0..=u1 { for v in -3..=0 { for w in TRENCH.0..=TRENCH.1 { if v == TRENCH.2 { c.n(u, v, w, "stone_bricks_dark"); } else { c.air(u, v, w); } } } }
    for u in u0 - 1..=u1 + 1 { for v in TRENCH.2..=0 { for w in [TRENCH.0 - 1, TRENCH.1 + 1] { if v < 0 { c.n(u, v, w, "stone_bricks_dark"); } } if u == u0 - 1 || u == u1 + 1 { for w in TRENCH.0..=TRENCH.1 { c.n(u, v, w, "stone_bricks_dark"); } } } }
    for u in u0..=u1 { for w in TRENCH.0..=TRENCH.1 { if LADDERS.iter().any(|l| (*l - u).abs() <= 0) { continue; } let h = c.h(u, -2, w, 5); c.n(u, -2, w, if h < 0.55 { "spikes" } else if h < 0.75 { "bone_pile" } else { "bone" }); } }
    for u in BRIDGES { for w in TRENCH.0..=TRENCH.1 { c.n(u, 0, w, "crumbling_floor"); c.air(u, -2, w); c.air(u, -1, w); } }
    for u in LADDERS { for v in -2..=0 { c.t(u, v, TRENCH.0, "ladder", 0, -1); c.t(u, v, TRENCH.1, "ladder", 0, 1); } c.air(u, -2, TRENCH.0 + 1); }
    for side in [-1, 1] { for du in 0..2 { for v in 1..=3 { c.air((7 + du) * side, v, w1 + 1); } } }
    // les quatre souverains : un socle, le gisant, la statue du roi derrière, le levier devant, une chandelle
    for (k, (_, _, u)) in KINGS.iter().enumerate() { for du in 0..2 { for w in -32..=-30 { c.n(u + du, 1, w, "chiseled_stone"); } } c.tomb((*u, -31), 2, (0, 1)); c.statue(*u, 1, -33, (0, -1)); c.n(u + 1, 1, -33, "candelabra");
        let l = lever(k); c.t(l[0], l[1], l[2], "lever", 0, -1); c.n(u + 1, 2, -30, "candle"); }
    for u in [-13, -6, 5, 12] { c.column(u, -33, 1, hv); } for d in CRYPT_DARTS { c.n(d[0], d[1], d[2], "dart_trap"); }
    c.ossuary(u0, u1, w0, w1, 1, 4); for u in [-12, -4, 3, 11] { c.t(u, 5, -34, "wall_relief", 0, -1); }
    for (u, w) in [(-7, -30), (6, -30), (-5, -25), (4, -25)] { c.n(u, hv, w, "chandelier"); } for (u, w) in [(-14, -33), (14, -33), (-14, -24), (14, -24)] { c.n(u, hv, w, "cobweb"); c.n(u, hv - 1, w, "cobweb"); }
    for (u, w, dx) in [(-14, -30, -1), (14, -30, 1), (-14, -25, -1), (14, -25, 1)] { c.t(u, 3, w, "wall_sconce", dx, 0); } for (u, w) in [(-12, -24), (11, -32), (-2, -24), (13, -29)] { c.n(u, 1, w, "bone_pile"); }
    // l'escalier, du mausolée au vestibule : seize marches, trois de large
    for k in 0..DEPTH { let w = -3 + k; let top = DEPTH - 1 - k; let open_top = if w <= 5 { s } else { top + 4 };
        for u in -2..=2i32 { if u.abs() == 2 { for v in top - 1..=open_top.max(top + 5) { if v < s { c.n(u, v, w, if v == top - 1 { "stone_bricks_dark" } else { "stone_bricks" }); } } continue; }
            c.n(u, top - 1, w, "stone_bricks_dark"); c.t(u, top, w, "stone_bricks_stairs", 0, -1); for v in top + 1..=open_top { c.air(u, v, w); } if w > 5 { c.n(u, top + 5, w, "stone_bricks_dark"); } }
        if k % 4 == 2 && w > 5 { c.t(-1, top + 2, w, "wall_sconce", -1, 0); } }
    for u in -1..=1 { for v in 1..=4 { c.air(u, v, b0 - 1); } }
    // le vestibule : ses portes, ses gardiens de pierre, la porte des morts sous son bas-relief
    for side in [-1, 1] { for u in [5, 6] { for w in b0..=b0 + 1 { for v in 1..=3 { c.air(u * side, v, w); } } } }
    for u in [-4, 4] { c.statue(u, 1, b1, (0, 1)); c.n(u.signum() * 3, 1, b1, "brazier_stand"); } c.n(0, hv, 16, "chandelier"); c.ossuary(a0, a1, b0, b1, 1, 3);
    // le Couloir des Lames : des pics en vague, deux lance-fléchettes
    let (l0, l1) = BLADES; c.room(-1, 1, l0, l1, 3); for w in SPIKE_ROWS { for u in -1..=1 { c.n(u, 1, w, "spikes_down"); } } for d in BLADE_DARTS { c.n(d[0], d[1], d[2], "dart_trap"); } for u in -1..=1 { for v in 1..=3 { c.air(u, v, l0 - 1); c.air(u, v, l1 + 1); } }
    c.t(-1, 3, l0, "wall_sconce", -1, 0); c.t(1, 3, l1, "wall_sconce", 1, 0);
    // le tombeau du Premier Roi : son sarcophage ouvert, l'autel et le trésor, deux rois de pierre, des colonnes, des brasiers
    let (m0, m1, n0, n1, mv) = TOMB; c.room(m0, m1, n0, n1, mv); for u in -1..=1 { for v in 1..=3 { c.air(u, v, TOMB_DOOR_W); } }
    for u in -1..=0 { c.n(u, 1, 33, "chiseled_stone"); } c.tomb((-1, 33), 2, (0, 1)); c.t(CHEST[0], 1, CHEST[2], "altar", 0, 1); c.n(CHEST[0], CHEST[1], CHEST[2], "chest");
    for u in [-6, 6] { c.statue(u, 1, n1, (0, 1)); c.n(u / 2 + u.signum(), 1, n1, "brazier_stand"); } for (u, w) in [(-8, 29), (8, 29), (-8, 34), (8, 34)] { c.column(u, w, 1, mv); }
    c.n(0, mv, 31, "chandelier"); c.ossuary(m0, m1, n0, n1, 1, 3); for (u, w) in [(-9, 36), (9, 36), (-9, 28), (9, 28)] { c.n(u, mv, w, "cobweb"); } c.n(-7, 1, 31, "bone_pile"); c.n(6, 1, 33, "bone_pile");
    // la Porte des Morts : trois chaînes, un bas-relief au-dessus
    for u in -1..=1 { for v in 1..=3 { c.n(u, v, GATE_W, "great_chain"); } } c.t(0, 4, GATE_W, "wall_relief", 0, 1); for u in [-2, 2] { c.n(u, 4, GATE_W, "stone_bricks_dark"); }

    // En surface. Le mausolée : murs de brique sombre, un toit à degrés coiffé d'une statue, le portique au nord
    for u in -6..=6i32 { for w in -6..=6i32 { c.n(u, s, w, if u.abs() <= 2 && (-3..=5).contains(&w) { "stone_bricks_dark" } else if (u + w).rem_euclid(2) == 0 { "polished_stone" } else { "stone_bricks" });
        let edge = u.abs() == 6 || w.abs() == 6; for v in s + 1..=s + 6 { if edge { c.n(u, v, w, if u.abs() == 6 && w.abs() == 6 { "chiseled_stone" } else { "stone_bricks_dark" }); } else { c.air(u, v, w); } } c.n(u, s + 7, w, "stone_bricks_dark"); } }
    for (r, v) in [(4, s + 8), (2, s + 9)] { for u in -r..=r { for w in -r..=r { c.n(u, v, w, "stone_bricks_dark"); } } } c.statue(0, s + 10, 0, (0, 1));
    for u in -1..=1 { for k in 0..9 { c.air(u, s, -3 + k); } } for u in [-2, 2] { for w in -3..=5 { c.n(u, s + 1, w, "stone_bricks_slab"); } }
    for u in -1..=1 { for v in s + 1..=s + 3 { c.air(u, v, -6); } } c.t(0, s + 4, -6, "wall_relief", 0, 1);
    for u in [-4, 4] { c.statue(u, s + 1, 4, (0, 1)); } c.tomb((-5, -2), s + 1, (-1, 0)); c.tomb((5, -1), s + 1, (1, 0)); c.n(-5, s + 1, 2, "candelabra"); c.n(5, s + 1, 2, "candelabra"); c.n(0, s + 6, 0, "chandelier");
    for (u, w, dx, dz) in [(-5, 0, -1, 0), (5, 0, 1, 0), (-3, 5, 0, 1), (3, 5, 0, 1)] { c.t(u, s + 3, w, "wall_sconce", dx, dz); }
    for u in -6..=6 { for w in -9..=-7 { c.n(u, s, w, "stone_bricks"); } } for u in [-5, -2, 2, 5] { c.column(u, -8, s + 1, s + 5); } for u in -6..=6 { c.t(u, s + 6, -8, "lintel", 0, 1); }
    for (r, v) in [(5, s + 7), (3, s + 8), (1, s + 9)] { for u in -r..=r { c.n(u, v, -8, "stone_bricks_dark"); } } c.t(0, s + 7, -8, "wall_relief", 0, 1);
    // l'allée des rois : un dallage, huit rois de pierre sur leurs socles, des brasiers ; des tombes de part et d'autre
    for u in -2..=2i32 { for w in -31..=-10 { c.n(u, s, w, if u.abs() == 2 { "stone_bricks_dark" } else if c.h(u, s, w, 6) < 0.25 { "cobblestone" } else { "stone_bricks" }); } }
    for w in [-29, -24, -19, -14] { for side in [-1, 1] { c.n(4 * side, s + 1, w, "chiseled_stone"); c.statue(4 * side, s + 2, w, (side, 0)); } }
    for (u, w) in [(-4, -31), (4, -31), (-4, -10), (4, -10)] { c.n(u, s + 1, w, "brazier_stand"); }
    for w in (-30..=-12).step_by(3) { for side in [-1, 1] { for k in 0..2 { let u = side * (7 + 3 * k); let h = c.h(u, s, w, 7); if h < 0.25 { continue; }
        if h < 0.45 { c.tomb((u, w), s + 1, (0, 1)); } else if h < 0.85 { c.t(u, s + 1, w, if h < 0.7 { "gravestone" } else { "grave_cross" }, 0, 1); } else { c.n(u, s + 1, w, "column_base"); c.n(u, s + 2, w, "column_shaft"); } } } }
}

/// Les mécanismes des catacombes bâties en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] - DEPTH; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let chain = by_name("great_chain").unwrap_or(0); let iron = by_name("iron_bars").unwrap_or(0); let id = |n: &str| by_name(n).unwrap_or(0);
    let mut m = Mech::default();
    // l'ordre des règnes : la porte des morts
    let rank = reign(o); let mut order: Vec<usize> = (0..4).collect(); order.sort_by_key(|k| rank[*k]);
    m.wires.push(Wire { name: name("rois"), inputs: order.iter().map(|k| at(lever(*k))).collect(), seq: true, latch: true, fail_darts: true, darts: CRYPT_DARTS.iter().map(|d| at(*d)).collect(),
        doors: (-1..=1).flat_map(|u| (1..=3).map(move |v| [u, v, GATE_W])).map(|q| (at(q), chain)).collect(), ..Default::default() });
    // le passage secret
    let pane_ids: Vec<([i32; 3], Id)> = pane().into_iter().map(|q| (at(q), if q[1] == 2 && q[0] == -29 { variant(id("ossuary_wall"), facing_of(-1.0, 0.0), false, false, false) } else { id("stone_bricks_dark") })).collect();
    m.wires.push(Wire { name: name("secret"), inputs: vec![at(secret_lever(o))], latch: true, doors: pane_ids, ..Default::default() });
    // le Couloir des Lames
    for (k, w) in SPIKE_ROWS.iter().enumerate() { m.pulses.push(Pulse { spikes: (-1..=1).map(|u| at([u, 1, *w])).collect(), period: 2.4, up: 0.4, phase: k as f64 * 0.8, ..Default::default() }); }
    m.pulses.push(Pulse { darts: BLADE_DARTS.iter().map(|d| at(*d)).collect(), period: 2.4, up: 0.0, phase: 1.2, ..Default::default() });
    // le Premier Roi
    let doors: Vec<([i32; 3], Id)> = (-1..=1).flat_map(|u| (1..=3).map(move |v| [u, v, TOMB_DOOR_W])).map(|q| (at(q), iron)).collect();
    m.guards.push(Guard { min: at([TOMB.0, 1, TOMB.2]), max: at([TOMB.1, TOMB.4, TOMB.3]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors, state: 0 });
    m.vaults.push(at(CHEST)); m.secrets.push(at(SECRET_CHEST));
    m.seals.push(Seal { min: at([-37, -4, -35]), max: at([37, DEPTH - 1, 38]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str, xp: f64| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp });
    hint(&mut m, [HALL.0, 1, HALL.2], [HALL.1, HALL.4, HALL.3], "Les Catacombes des Rois. Au sud, la Porte des Morts, fermée de lourdes chaînes. À l'est et à l'ouest, les galeries de l'ossuaire mènent à la Crypte des Rois. Gardez une torche à la main : on s'y perd.", 0.0);
    hint(&mut m, [CRYPT.0, 1, TRENCH.1 + 1], [CRYPT.1, CRYPT.4, CRYPT.3], "La Crypte des Rois. Quatre souverains dorment ici, chacun sous sa statue. Abaissez leurs leviers dans l'ordre de leurs règnes, et la Porte des Morts s'ouvrira ; une erreur réveille les pièges. Leurs épitaphes sont gravées au fond de l'ossuaire. Devant vous, un charnier : ses passerelles de pierre friable ne portent qu'un passage.", 0.0);
    for (k, (n, _, u)) in KINGS.iter().enumerate() { hint(&mut m, [*u, 1, -29], [u + 1, 2, -29], &format!("Le tombeau de {n}."), 0.0); let _ = k; }
    for (k, (n, line, _)) in KINGS.iter().enumerate() { let ((u, w), _) = chapel(o, k); hint(&mut m, [u.min(u + 1) , 1, w], [u + 1, 3, w + 1], &format!("Une épitaphe, gravée au-dessus d'un gisant : « Ici repose {n}, qui régna {}. {line} »", ORD[rank[k]]), 0.0); }
    hint(&mut m, [-1, 1, BLADES.0], [1, 3, BLADES.0], "Le Couloir des Lames : des pics jaillissent du sol, en vague. Observez le rythme avant de passer.", 0.0);
    hint(&mut m, [SECRET.0, 1, SECRET.2], [SECRET.1, 3, SECRET.3], "Une chambre secrète, oubliée derrière le mur de crânes !", 30.0);
    m
}

impl Session {
    /// Bâtit ici les catacombes (monde déjà chargé : essais, captures) et branche leurs mécanismes. `o` : le sol de l'allée.
    pub fn build_catacombs(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("c{}:{}:", o[0], o[2]); if !self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Des catacombes de la carte, qu'on approche : leurs mécanismes se branchent, une fois.
    pub(crate) fn catacombs_wake(&mut self, pl: &Place) { let tag = format!("c{}:{}:", pl.x, pl.z); if self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}
