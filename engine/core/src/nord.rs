//! Les villages vikings des montagnes et de la taïga : leurs bâtiments, bâtis bloc par bloc comme les patrons médiévaux.
//!
//! Des maisons longues en rondins de sapin croisés aux angles, sur un soubassement de pierres sèches, sous un toit de
//! gazon pentu dont les rives se croisent au faîte en têtes de dragon ; des boucliers ronds peints de part et d'autre de la
//! porte, du bois empilé au pignon ; dedans, le foyer au milieu, des bancs couverts de fourrures, un coffre à runes, un
//! râtelier d'armes. Les ateliers s'ouvrent sur la rue ; la halle à hydromel a ses longues tables, ses tonneaux et ses lits
//! de voyageurs ; la grande halle du jarl, haute, bordée de boucliers, a son trône au bout du long foyer. Les étables sont
//! basses, en rondins sous le gazon ; la ferme y loge sa famille au bout, près d'un petit foyer.
//! Le tracé du village (le cercle de pierres et la pierre runique au milieu, la palissade) est dans `bourg`. Vérifié par
//! `tests/nord.rs`.
use crate::batisse::{Lot, Work};
use crate::edifice::Arch;
use crate::patrons::{Built, Plan, Rec, P};

/// Les bâtiments du Nord.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NKind { Longue, Atelier, Hydromel, Halle, GrandeHalle, Etable, Ferme }
/// Le bâtiment du Nord qui tient la place d'un archétype ; aucun pour l'église et le moulin (le Nord n'en a pas).
pub fn kind_of(a: Arch) -> Option<NKind> {
    match a { Arch::House | Arch::Cottage => Some(NKind::Longue), Arch::Inn => Some(NKind::Hydromel), Arch::Hall => Some(NKind::Halle), Arch::TownHall => Some(NKind::GrandeHalle),
        Arch::Forge | Arch::Workshop | Arch::Tannery | Arch::Woodshop => Some(NKind::Atelier), Arch::Farm => Some(NKind::Ferme), Arch::Fold | Arch::Stable | Arch::Barn => Some(NKind::Etable), _ => None }
}
impl NKind {
    pub fn label(self) -> &'static str { match self { NKind::Longue => "maison longue", NKind::Atelier => "atelier de rondins", NKind::Hydromel => "halle à hydromel", NKind::Halle => "halle du jarl", NKind::GrandeHalle => "grande halle du jarl", NKind::Etable => "étable de gazon", NKind::Ferme => "ferme-étable" } }
}
/// Corps, porte sur la façade (z = 0) et emprise. Maison : bit 0, plus longue.
pub fn plan(k: NKind, var: u8) -> Plan {
    let (w, d, door) = match k { NKind::Longue => (if var & 1 != 0 { 11 } else { 9 }, 6, 2), NKind::Atelier => (7, 6, 5), NKind::Hydromel => (13, 8, 6), NKind::Halle => (15, 9, 7), NKind::GrandeHalle => (17, 10, 8), NKind::Etable => (9, 6, 4), NKind::Ferme => (11, 6, 3) };
    Plan { w, d, door, ext: [-2, -2, w + 1, d + 1], mirror: false }
}

/// Bâtit un bâtiment du Nord, le sol à y0 ; rend porte, poste et lit en coordonnées du terrain.
#[allow(clippy::too_many_arguments)]
pub fn build(k: &mut Work, lot: Lot, y0: i32, kind: NKind, var: u8, h: f64, rich: f64, craft: Option<&'static str>) -> Built {
    let pl = plan(kind, var); let mut p = P { k, lot, y0, mirror: false, w: pl.w, d: pl.d, h, rec: Rec::default() };
    let b = match kind {
        NKind::Longue => longue(&mut p, pl.w, rich), NKind::Atelier => atelier(&mut p, craft), NKind::Hydromel => hydromel(&mut p),
        NKind::Halle => halle(&mut p, 15, 9), NKind::GrandeHalle => halle(&mut p, 17, 10), NKind::Etable => etable(&mut p, 9, false), NKind::Ferme => etable(&mut p, 11, true),
    };
    p.dress(); b
}

// ------------------------------------------------------------------ les gestes du charpentier
fn is_edge(x: i32, z: i32, xr: (i32, i32), zr: (i32, i32)) -> bool { x == xr.0 || x == xr.1 || z == zr.0 || z == zr.1 }
/// Une bûche de sapin couchée le long de l'axe local x (ou z), ou debout.
fn slog(p: &mut P, x: i32, y: i32, z: i32, axis: char) {
    let n = match axis { 'x' => if p.lot.u.0 != 0 { "spruce_log_x_c" } else { "spruce_log_z_c" }, 'z' => if p.lot.w.0 != 0 { "spruce_log_x_c" } else { "spruce_log_z_c" }, _ => "spruce_log_c" }; p.n(x, y, z, n);
}
/// Des murs de rondins : couchés le long des murs, debout aux angles ; la première rangée en pierres sèches.
fn logs(p: &mut P, x: (i32, i32), z: (i32, i32), top: i32) {
    for xx in x.0..=x.1 { for zz in z.0..=z.1 { if !is_edge(xx, zz, x, z) { continue; } let corner = (xx == x.0 || xx == x.1) && (zz == z.0 || zz == z.1);
        p.n(xx, 1, zz, "fieldstone"); for y in 2..=top { if corner { slog(p, xx, y, zz, 'y'); } else if zz == z.0 || zz == z.1 { slog(p, xx, y, zz, 'x'); } else { slog(p, xx, y, zz, 'z'); } } } }
}
/// Les rives croisées au faîte des deux pignons, terminées en têtes de dragon (la gueule, l'œil rouge, deux cornes). Le
/// faîte court le long de x ; les pignons sont à x = -1 et x = w ; le toit part de la rangée y, débord ov.
fn dragons(p: &mut P, w: i32, d: i32, y: i32, ov: i32) {
    let lot = p.lot; let beam = [0.26, 0.17, 0.1]; let dark = [0.2, 0.13, 0.08]; let (df, yf, o) = (d as f64, (p.y0 + y) as f64, ov as f64);
    let half = (df + 2.0 * o) * 0.5; let peak = yf + half; let mid = df * 0.5;
    for u in [-0.55 - o * 0.1, w as f64 - 0.45 + o * 0.1] { let at = lot.cell(if u < 0.0 { 0 } else { w - 1 }, p.y0 + y + d / 2 + ov, d / 2);
        for side in [-1.0f64, 1.0] { let z0 = if side < 0.0 { -o } else { df + o }; let dir = [mid - z0, peak - yf]; let l = (dir[0] * dir[0] + dir[1] * dir[1]).sqrt();
            let (z1, y1) = (mid + dir[0] / l * 1.2, peak + 0.2 + dir[1] / l * 1.2);
            p.k.bar(at, lot.p(u, yf + 0.1, z0), lot.p(u, y1, z1), lot.v(1.0, 0.0, 0.0), 0.13, 0.05, beam, -1.0);
            let hd = lot.p(u, y1 + 0.1, z1 + dir[0] / l * 0.15); p.k.solid(at, hd, [lot.v(0.0, 0.0, 0.16), [0.0, 0.11, 0.0], lot.v(0.07, 0.0, 0.0)], dark, -1.0, 0);
            let snout = lot.p(u, y1 + 0.08, z1 + dir[0] / l * 0.15 - side * 0.2); p.k.solid(at, snout, [lot.v(0.0, 0.0, 0.12), [0.0, 0.06, 0.0], lot.v(0.06, 0.0, 0.0)], dark, -1.0, 0);
            for s in [-1.0, 1.0] { let eye = lot.p(u + s * 0.075, y1 + 0.16, z1 + dir[0] / l * 0.15 - side * 0.05); p.k.solid(at, eye, [lot.v(0.0, 0.0, 0.025), [0.0, 0.025, 0.0], lot.v(0.01, 0.0, 0.0)], [0.9, 0.2, 0.1], -1.0, 0); }
            let horn = lot.p(u, y1 + 0.3, z1 + dir[0] / l * 0.15 + side * 0.08); p.k.solid(at, horn, [lot.v(0.0, 0.0, 0.03), [0.0, 0.1, 0.0], lot.v(0.03, 0.0, 0.0)], dark, -1.0, 0); } }
}
/// Un foyer long au milieu de la salle : un lit de pierres, des feux, une marmite.
fn hearth(p: &mut P, x0: i32, x1: i32, z: i32) { for x in x0..=x1 { p.n(x, 0, z, "cobblestone"); p.n(x, 1, z, if (x - x0) % 2 == 0 { "campfire" } else { "cobblestone_slab" }); } p.n(x1 + 1, 1, z, "cauldron"); }
/// Des boucliers ronds accrochés au mur de façade, à la hauteur y.
fn shields(p: &mut P, xs: &[i32], y: i32) { for &x in xs { p.turn(x, y, -1, "round_shield", 0, 1); } }

// ------------------------------------------------------------------ la maison longue
fn longue(p: &mut P, w: i32, rich: f64) -> Built {
    let d = 6; p.ground([-2, -2, w + 1, d + 1], 12, [0, 0, w - 1, d - 1], "spruce_planks", "fieldstone");
    logs(p, (0, w - 1), (0, d - 1), 3); p.roof((0, w - 1), (0, d - 1), 4, 1, false, "turf", Some("spruce_planks")); dragons(p, w, d, 4, 1);
    p.door(2, 1, 0, 0, -1); shields(p, &[1, 3], 2); if rich > 0.5 { shields(p, &[5], 2); } p.win(w - 3, 2, 0, true); p.win(w / 2, 2, d - 1, true);
    // dedans : le foyer au milieu, les bancs de fourrure, le lit, le coffre à runes, le râtelier, les bois de cerf
    hearth(p, w / 2 - 1, w / 2, 3); for x in 4..w - 3 { if x != w / 2 { p.turn(x, 1, d - 2, "fur_bench", 0, 1); } }
    p.turn(w - 2, 1, d - 2, "bed", 0, 1); p.turn(w - 2, 1, d - 3, "bed_foot", 0, 1); p.n(1, 1, 1, "rune_chest"); p.turn(1, 1, d - 2, "weapon_rack", -1, 0); p.turn(1, 2, 3, "antlers", -1, 0); p.n(3, 3, 2, "lantern");
    p.n(w - 2, 1, 1, "barrel"); p.n(w - 3, 1, 1, "hides");
    // dehors : le bois empilé au pignon
    for (z, y) in [(1, 1), (2, 1), (3, 1), (4, 1), (2, 2), (3, 2)] { p.turn(-1, y, z, "log_pile", 1, 0); }
    Built { door: Some(2), spot: None, bed: Some((w - 3, d - 2)) }
}

// ------------------------------------------------------------------ l'atelier de rondins
fn atelier(p: &mut P, craft: Option<&'static str>) -> Built {
    let (w, d) = (7, 6); p.ground([-2, -2, w + 1, d + 1], 12, [0, 0, w - 1, d - 1], "spruce_planks", "fieldstone");
    logs(p, (0, w - 1), (0, d - 1), 3); for x in 1..=3 { for y in 1..=3 { p.put(x, y, 0, 0); } } for y in 2..=3 { slog(p, 0, y, 0, 'y'); slog(p, 4, y, 0, 'y'); } p.n(4, 1, 0, "fieldstone"); // ouvert sur la rue
    p.roof((0, w - 1), (0, d - 1), 4, 1, false, "turf", Some("spruce_planks")); dragons(p, w, d, 4, 1); p.door(5, 1, 0, 0, -1); p.win(w - 1, 2, 3, false);
    let station = match craft { Some("forgeron" | "maitre_forgeron" | "armurier") => "anvil", Some("tanneur") => "cauldron", Some("tisserand" | "tailleur") => "loom", _ => "workbench" };
    p.n(2, 1, 3, station); if station == "anvil" { p.n(1, 1, 4, "furnace_lit"); for y in 1..=8 { p.n(0, y, 4, "fieldstone"); } p.n(1, 1, 1, "weapon_rack"); } else { p.n(1, 1, 4, "barrel"); p.n(1, 1, 1, "log_pile"); }
    p.turn(3, 2, d - 1, "tool_rack", 0, 1); p.turn(5, 1, 4, "bed", 0, 1); p.turn(5, 1, 3, "bed_foot", 0, 1); p.n(3, 3, 2, "lantern");
    Built { door: Some(5), spot: Some(((2, 2), (2, 3))), bed: Some((4, 4)) }
}

// ------------------------------------------------------------------ la halle à hydromel
fn hydromel(p: &mut P) -> Built {
    let (w, d) = (13, 8); p.ground([-2, -2, w + 1, d + 1], 14, [0, 0, w - 1, d - 1], "spruce_planks", "fieldstone");
    logs(p, (0, w - 1), (0, d - 1), 4); p.roof((0, w - 1), (0, d - 1), 5, 1, false, "turf", Some("spruce_planks")); dragons(p, w, d, 5, 1);
    p.door(6, 1, 0, 0, -1); shields(p, &[2, 4, 8, 10], 3); p.turn(5, 3, -1, "banner_red", 0, 1); p.turn(7, 3, -1, "banner_red", 0, 1); for x in [3, 9] { p.win(x, 2, 0, true); p.win(x, 2, d - 1, true); }
    // les longues tables et leurs bancs, le foyer, les tonneaux d'hydromel, les lits des voyageurs
    hearth(p, 5, 7, 4); for x in 2..=4 { p.n(x, 1, 2, "table"); p.turn(x, 1, 1, "bench", 0, -1); p.n(x + 6, 1, 2, "table"); p.turn(x + 6, 1, 1, "bench", 0, -1); }
    for z in 3..=5 { p.n(1, 1, z, "barrel"); } p.n(1, 2, 4, "barrel"); p.n(2, 2, 2, "bottles");
    for x in [3, 6, 9] { p.turn(x, 1, d - 2, "bed", 0, 1); p.turn(x, 1, d - 3, "bed_foot", 0, 1); } p.n(11, 1, d - 2, "rune_chest");
    for x in [3, 9] { p.n(x, 4, 3, "lantern"); }
    Built { door: Some(6), spot: None, bed: Some((4, d - 2)) }
}

// ------------------------------------------------------------------ la halle du jarl
fn halle(p: &mut P, w: i32, d: i32) -> Built {
    let door = w / 2; p.ground([-2, -2, w + 1, d + 1], 20, [0, 0, w - 1, d - 1], "spruce_planks", "fieldstone");
    logs(p, (0, w - 1), (0, d - 1), 5); for x in (2..w - 2).step_by(4) { for y in 2..=5 { slog(p, x, y, 0, 'y'); slog(p, x, y, d - 1, 'y'); } } // les poteaux du mur
    p.roof((0, w - 1), (0, d - 1), 6, 1, false, "turf", Some("spruce_planks")); dragons(p, w, d, 6, 1);
    p.door(door, 1, 0, 0, -1); let sx: Vec<i32> = (1..w - 1).filter(|x| x % 2 == 1 && (x - door).abs() > 1).collect(); shields(p, &sx, 3); p.turn(door - 1, 4, -1, "banner_red", 0, 1); p.turn(door + 1, 4, -1, "banner_red", 0, 1);
    for x in [3, w - 4] { p.win(x, 4, 0, true); p.win(x, 4, d - 1, true); }
    // le long foyer, le trône du jarl au bout sur son estrade, les tables, les bancs de fourrure, les armes, les bois de cerf
    hearth(p, 3, w - 6, d / 2); for x in 1..=w - 2 { if x % 3 == 0 { p.n(x, 1, 1, "table"); p.n(x, 1, d - 2, "table"); } else { p.turn(x, 1, 1, "fur_bench", 0, -1); p.turn(x, 1, d - 2, "fur_bench", 0, 1); } }
    for z in d / 2 - 1..=d / 2 + 1 { p.n(w - 2, 1, z, "spruce_planks"); } p.turn(w - 2, 2, d / 2, "throne", 1, 0); p.turn(w - 2, 3, d / 2 - 1, "banner_gold", 1, 0); p.turn(w - 2, 3, d / 2 + 1, "banner_gold", 1, 0);
    p.turn(1, 2, d / 2, "antlers", -1, 0); p.turn(1, 1, d / 2 - 1, "weapon_rack", -1, 0); p.turn(1, 1, d / 2 + 1, "weapon_rack", -1, 0); p.n(1, 1, 1, "rune_chest");
    for x in (3..w - 3).step_by(4) { p.n(x, 5, d / 2, "lantern"); } for x in [door - 2, door + 2] { p.n(x, 1, -2, "brazier"); }
    Built { door: Some(door), spot: None, bed: None }
}

// ------------------------------------------------------------------ l'étable de gazon (et la ferme-étable)
fn etable(p: &mut P, w: i32, house: bool) -> Built {
    let d = 6; let gate = if house { 3 } else { w / 2 }; p.ground([-2, -2, w + 1, d + 1], 10, [0, 0, w - 1, d - 1], "dirt", "fieldstone");
    logs(p, (0, w - 1), (0, d - 1), 3); p.roof((0, w - 1), (0, d - 1), 4, 1, false, "turf", Some("spruce_planks")); dragons(p, w, d, 4, 1);
    for x in gate..=gate + 1 { for y in 1..=2 { p.put(x, y, 0, 0); } } // la grande porte de l'étable
    for x in 1..w - 1 { if house && x >= w - 4 { continue; } if x != gate && x != gate + 1 { p.n(x, 1, d - 2, "hay_bale"); } } p.turn(1, 1, 2, "trough", -1, 0); p.turn(1, 1, 3, "trough", -1, 0); p.n(w - 2, 1, 1, "barrel");
    if !house { p.win(w - 1, 2, 3, false); return Built { door: Some(gate), spot: None, bed: None }; }
    // la ferme-étable : la famille vit au bout, derrière une cloison, près d'un petit foyer
    for z in 1..d - 1 { for y in 1..=3 { if !(z == 2 && y <= 2) { slog(p, w - 5, y, z, 'z'); } } } p.put(w - 5, 1, 2, 0); p.put(w - 5, 2, 2, 0);
    for x in w - 4..=w - 2 { for z in 1..=d - 2 { p.n(x, 0, z, "spruce_planks"); } } p.n(w - 3, 1, 3, "campfire"); p.turn(w - 2, 1, d - 2, "bed", 0, 1); p.turn(w - 2, 1, d - 3, "bed_foot", 0, 1); p.n(w - 4, 1, 1, "rune_chest"); p.turn(w - 4, 1, d - 2, "fur_bench", 0, 1);
    p.win(w - 1, 2, 3, false); p.n(w - 3, 3, 2, "lantern");
    Built { door: Some(gate), spot: None, bed: Some((w - 3, d - 2)) }
}
