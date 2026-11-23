//! Les villages du désert : leurs bâtiments, bâtis bloc par bloc comme les patrons médiévaux.
//!
//! Des maisons de pisé cubiques aux toits plats bordés d'un muret, où l'on monte par une échelle : une terrasse sous une
//! treille de palmes, ou une chambre haute coiffée d'une petite coupole chez les plus aisés ; des poutres qui dépassent
//! des façades, des fenêtres en arc, un auvent de toile au-dessus de la porte, des tapis, des poufs et des jarres dedans.
//! Les ateliers s'ouvrent sur la rue sous une rangée d'auvents. Le caravansérail est un carré fermé autour d'une cour à
//! arcades, un bassin et un palmier au milieu, quatre tourelles crénelées aux angles. La maison du conseil est blanchie à
//! la chaux, crénelée, coiffée d'une coupole ; la grande a en plus une tour de guet. Les enclos à dromadaires ont un
//! muret de grès, un abri de paille, des abreuvoirs ; la ferme y adosse sa maison.
//! Le tracé du village (l'oasis au milieu, le marché, la palmeraie) est dans `bourg`. Vérifié par `tests/desert.rs`.
use crate::batisse::{id, Lot, Work};
use crate::edifice::Arch;
use crate::patrons::{Built, Plan, Rec, P};
use crate::shapes::variant;

/// Les bâtiments du désert.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DKind { Maison, Atelier, Caravanserail, Conseil, GrandConseil, Ferme, Enclos }
/// Le bâtiment du désert qui tient la place d'un archétype médiéval (même rôle dans la vie et l'économie du village).
pub fn kind_of(a: Arch) -> DKind {
    match a { Arch::Inn => DKind::Caravanserail, Arch::Hall => DKind::Conseil, Arch::TownHall => DKind::GrandConseil, Arch::Farm => DKind::Ferme, Arch::Fold | Arch::Stable | Arch::Barn => DKind::Enclos,
        Arch::Forge | Arch::Workshop | Arch::Tannery | Arch::Woodshop => DKind::Atelier, _ => DKind::Maison }
}
impl DKind {
    pub fn label(self) -> &'static str { match self { DKind::Maison => "maison de pisé", DKind::Atelier => "atelier sous auvents", DKind::Caravanserail => "caravansérail", DKind::Conseil => "maison du conseil", DKind::GrandConseil => "grande maison du conseil", DKind::Ferme => "ferme et enclos", DKind::Enclos => "enclos à dromadaires" } }
}
/// Corps, porte sur la façade (z = 0) et emprise. Maison : bit 0, un peu plus grande ; bit 1, une chambre haute au lieu
/// de la terrasse ; bit 2, en miroir.
pub fn plan(k: DKind, var: u8) -> Plan {
    let (w, d, door, ext) = match k {
        DKind::Maison => { let (w, d) = if var & 1 != 0 { (7, 7) } else { (6, 6) }; (w, d, w / 2, [-1, -2, w, d]) }
        DKind::Atelier => (7, 7, 5, [-1, -2, 7, 7]), DKind::Caravanserail => (13, 13, 6, [-1, -2, 13, 13]),
        DKind::Conseil => (9, 9, 4, [-1, -2, 9, 9]), DKind::GrandConseil => (11, 11, 5, [-1, -2, 11, 11]),
        DKind::Ferme => (13, 9, 6, [-1, -1, 13, 9]), DKind::Enclos => (11, 9, 5, [-1, -1, 11, 9]),
    };
    let mirror = k == DKind::Maison && var & 4 != 0;
    if !mirror { return Plan { w, d, door, ext, mirror }; }
    Plan { w, d, door: w - 1 - door, ext: [w - 1 - ext[2], ext[1], w - 1 - ext[0], ext[3]], mirror }
}

/// Bâtit un bâtiment du désert, le sol à y0 ; rend porte, poste et lit en coordonnées du terrain. `h` : la graine du
/// bâtiment, `rich` : sa richesse (les murs blanchis, la coupole), `craft` : le métier de l'atelier.
#[allow(clippy::too_many_arguments)]
pub fn build(k: &mut Work, lot: Lot, y0: i32, kind: DKind, var: u8, h: f64, rich: f64, craft: Option<&'static str>) -> Built {
    let pl = plan(kind, var); let mut p = P { k, lot, y0, mirror: pl.mirror, w: pl.w, d: pl.d, h, rec: Rec::default() };
    let wall = if rich > 0.72 { "plaster" } else if h < 0.4 { "sandstone" } else { "plaster_ochre" };
    let awn = if h * 7.0 % 1.0 < 0.5 { "awning_red" } else { "awning_blue" };
    let b = match kind {
        DKind::Maison => maison(&mut p, pl.w, pl.d, var & 2 != 0, rich > 0.55, wall, awn),
        DKind::Atelier => atelier(&mut p, craft, wall, awn), DKind::Caravanserail => caravanserail(&mut p),
        DKind::Conseil => conseil(&mut p, 9, false), DKind::GrandConseil => conseil(&mut p, 11, true),
        DKind::Ferme => enclos(&mut p, 13, true, wall), DKind::Enclos => enclos(&mut p, 11, false, wall),
    };
    let fx = |x: i32| if pl.mirror { pl.w - 1 - x } else { x };
    Built { door: b.door.map(fx), spot: b.spot.map(|(a, s)| ((fx(a.0), a.1), (fx(s.0), s.1))), bed: b.bed.map(|(x, z)| (fx(x), z)) }
}

// ------------------------------------------------------------------ les gestes du bâtisseur
fn is_edge(x: i32, z: i32, xr: (i32, i32), zr: (i32, i32)) -> bool { x == xr.0 || x == xr.1 || z == zr.0 || z == zr.1 }
/// Des murs sur le pourtour d'un rectangle, de la rangée y.0 à y.1.
fn walls(p: &mut P, x: (i32, i32), z: (i32, i32), y: (i32, i32), mat: &str) { for xx in x.0..=x.1 { for zz in z.0..=z.1 { if is_edge(xx, zz, x, z) { for yy in y.0..=y.1 { p.n(xx, yy, zz, mat); } } } } }
/// Le muret d'un toit plat : plein, ou crénelé (un merlon sur deux, les angles pleins).
fn parapet(p: &mut P, x: (i32, i32), z: (i32, i32), y: i32, mat: &str, crenel: bool) {
    for xx in x.0..=x.1 { for zz in z.0..=z.1 { if !is_edge(xx, zz, x, z) { continue; } let corner = (xx == x.0 || xx == x.1) && (zz == z.0 || zz == z.1);
        p.n(xx, y, zz, mat); if crenel && (corner || (xx + zz) % 2 == 0) { p.n(xx, y + 1, zz, mat); } } }
}
/// Une fenêtre en arc dans un mur parallèle à x (`along_x`) ou à z.
fn arch_win(p: &mut P, x: i32, y: i32, z: i32, along_x: bool) { let f = if along_x { p.face(0, 1) } else { p.face(1, 0) }; p.put(x, y, z, variant(id("window_arched"), f, false, false, false)); }
/// Les poutres du plafond qui dépassent de la façade (z = -1) et du dos (z = d), sous le toit.
fn vigas(p: &mut P, w: i32, d: i32, y: i32, skip: i32) { for x in (1..w - 1).step_by(2) { if x == skip { continue; } p.log(x, y, -1, 'z'); p.log(x, y, d, 'z'); } }
/// Une petite coupole sur un carré de 2r+1 de côté centré en (cx, cz), posée à la hauteur y : des gradins de marches
/// tournées vers le centre, un bouton au sommet.
fn dome(p: &mut P, cx: i32, cz: i32, r: i32, y: i32, mat: &str) {
    for k in 0..=r { let rr = r - k; if rr == 0 { p.n(cx, y + k, cz, mat); p.n(cx, y + k + 1, cz, &format!("{mat}_slab")); break; }
        for dx in -rr..=rr { for dz in -rr..=rr { if dx.abs().max(dz.abs()) != rr { p.n(cx + dx, y + k, cz + dz, mat); continue; }
            let (sx, sz) = if dx.abs() >= dz.abs() { (-dx.signum(), 0) } else { (0, -dz.signum()) }; p.stair(cx + dx, y + k, cz + dz, &format!("{mat}_stairs"), sx, sz, false); } } }
}
/// Une treille de palmes sur quatre poteaux, au-dessus d'un carré du toit (x0, z0)–(x0+2, z0+2) posé à y.
fn pergola(p: &mut P, x0: i32, z0: i32, y: i32) {
    for (x, z) in [(x0, z0), (x0 + 2, z0), (x0, z0 + 2), (x0 + 2, z0 + 2)] { p.n(x, y, z, "fence"); p.n(x, y + 1, z, "fence"); }
    for x in x0..=x0 + 2 { for z in z0..=z0 + 2 { p.n(x, y + 2, z, "thatch"); } }
}

// ------------------------------------------------------------------ la maison de pisé
fn maison(p: &mut P, w: i32, d: i32, upper: bool, rich: bool, wall: &str, awn: &str) -> Built {
    let door = w / 2; p.ground([-1, -2, w, d], 12, [0, 0, w - 1, d - 1], "terracotta_floor", "sandstone");
    walls(p, (0, w - 1), (0, d - 1), (1, 3), wall); p.fill((0, w - 1), (4, 4), (0, d - 1), "sandstone"); parapet(p, (0, w - 1), (0, d - 1), 5, wall, false);
    p.door(door, 1, 0, 0, -1); p.turn(door, 3, -1, awn, 0, 1);
    for z in [2, d - 3] { arch_win(p, 0, 2, z, false); arch_win(p, w - 1, 2, z, false); } arch_win(p, door, 2, d - 1, true); if w > 6 { arch_win(p, 1, 2, 0, true); }
    vigas(p, w, d, 4, door);
    // dedans : l'échelle vers le toit, le lit, le coffre, les jarres, le tapis berbère, un pouf, une lanterne ajourée
    for y in 1..=3 { p.put(1, y, d - 2, variant(id("ladder"), p.face(-1, 0), false, false, false)); } p.n(1, 4, d - 2, "trapdoor");
    p.turn(w - 2, 1, d - 2, "bed", 0, 1); p.turn(w - 2, 1, d - 3, "bed_foot", 0, 1); p.n(w - 2, 1, 1, "chest"); p.n(1, 1, 1, "amphora");
    p.n(door, 1, d / 2, "berber_rug"); p.n(2, 1, d - 3, "pouf"); p.n(door, 3, d / 2, "moroccan_lantern");
    if upper {
        // la chambre haute, au fond, côté échelle ; sa porte donne sur la terrasse ; une coupole chez les plus aisés
        walls(p, (0, 2), (d - 3, d - 1), (5, 7), wall); p.door(1, 5, d - 3, 0, -1); arch_win(p, 0, 6, d - 2, false); p.n(1, 4, d - 2, "terracotta_floor");
        for y in 5..=7 { p.put(1, y, d - 2, 0); } p.n(1, 5, d - 2, "berber_rug");
        if rich { p.fill((0, 2), (8, 8), (d - 3, d - 1), wall); dome(p, 1, d - 2, 1, 9, "plaster"); } else { p.fill((0, 2), (8, 8), (d - 3, d - 1), "sandstone"); parapet(p, (0, 2), (d - 3, d - 1), 9, wall, false); }
        p.n(w - 2, 5, 1, "potted_fern");
    } else {
        // la terrasse : une treille de palmes, un tapis qui sèche, des plantes en pot, une jarre
        pergola(p, w - 4, 1, 5); p.n(w - 3, 5, 2, "berber_rug"); p.n(1, 5, 1, "potted_fern"); p.n(w - 2, 5, d - 2, "amphora");
    }
    Built { door: Some(door), spot: None, bed: Some((w - 3, d - 2)) }
}

// ------------------------------------------------------------------ l'atelier sous auvents
fn atelier(p: &mut P, craft: Option<&'static str>, wall: &str, awn: &str) -> Built {
    let (w, d) = (7, 7); p.ground([-1, -2, w, d], 12, [0, 0, w - 1, d - 1], "sandstone", "sandstone");
    walls(p, (0, w - 1), (0, d - 1), (1, 3), wall); for x in 1..=3 { for y in 1..=2 { p.put(x, y, 0, 0); } } // la boutique ouverte sur la rue
    p.fill((0, w - 1), (4, 4), (0, d - 1), "sandstone"); parapet(p, (0, w - 1), (0, d - 1), 5, wall, false); vigas(p, w, d, 4, 5);
    for x in 0..=4 { p.turn(x, 3, -1, awn, 0, 1); } p.door(5, 1, 0, 0, -1); arch_win(p, w - 1, 2, 3, false); arch_win(p, 0, 2, 4, false);
    // le poste, selon le métier
    let station = match craft { Some("forgeron" | "maitre_forgeron" | "armurier") => "anvil", Some("tanneur") => "cauldron", Some("tisserand" | "tailleur") => "loom", _ => "workbench" };
    p.n(2, 1, 3, station);
    if station == "anvil" { p.n(1, 1, 5, "furnace"); for y in 4..=7 { p.n(1, y, 5, "sandstone"); } p.n(1, 8, 5, "sandstone_slab"); p.n(3, 1, 5, "cauldron"); }
    else { p.n(1, 1, 5, "barrel"); p.n(3, 1, 5, "crate"); p.n(3, 2, 5, "sack"); }
    p.n(1, 1, 1, "amphora"); p.n(3, 1, 1, "sack"); p.n(3, 3, 3, "moroccan_lantern"); p.n(4, 1, 3, "berber_rug");
    p.turn(5, 1, 5, "bed", 0, 1); p.turn(5, 1, 4, "bed_foot", 0, 1);
    Built { door: Some(5), spot: Some(((2, 2), (2, 3))), bed: Some((4, 5)) }
}

// ------------------------------------------------------------------ le caravansérail
fn caravanserail(p: &mut P) -> Built {
    let (w, d) = (13, 13); let s = "sandstone"; p.ground([-1, -2, w, d], 14, [0, 0, w - 1, d - 1], "terracotta_floor", s);
    let court = |x: i32, z: i32| (4..=8).contains(&x) && (4..=8).contains(&z);
    walls(p, (0, w - 1), (0, d - 1), (1, 4), s);
    // l'arcade autour de la cour : un pilier sur deux, des arcs entre eux
    for x in 3..=9 { for z in 3..=9 { if !is_edge(x, z, (3, 9), (3, 9)) { continue; } p.n(x, 4, z, s);
        if (x + z) % 2 == 0 { for y in 1..=3 { p.n(x, y, z, s); } } else { p.n(x, 3, z, "sandstone_slab_top"); } } }
    for x in 0..w { for z in 0..d { if !court(x, z) { p.n(x, 5, z, s); } else { p.n(x, 0, z, "sandstone"); } } }
    parapet(p, (0, w - 1), (0, d - 1), 6, s, true); for x in 3..=9 { for z in 3..=9 { if is_edge(x, z, (3, 9), (3, 9)) { p.n(x, 6, z, "sandstone_slab"); } } }
    // les tourelles d'angle
    for (cx, cz) in [(0, 0), (w - 2, 0), (0, d - 2), (w - 2, d - 2)] { for x in cx..=cx + 1 { for z in cz..=cz + 1 { for y in 6..=7 { p.n(x, y, z, s); } } } parapet(p, (cx, cx + 1), (cz, cz + 1), 8, s, true); }
    // le grand portail : trois blocs de large, l'arc au-dessus, des bannières
    for x in 5..=7 { for y in 1..=3 { p.put(x, y, 0, 0); } } p.n(5, 3, 0, "sandstone_slab_top"); p.n(7, 3, 0, "sandstone_slab_top"); p.turn(4, 3, -1, "banner_red", 0, 1); p.turn(8, 3, -1, "banner_red", 0, 1); p.n(6, 4, -1, "moroccan_lantern");
    // la cour : le bassin, le palmier, les abreuvoirs
    p.n(6, 0, 6, "water"); p.n(6, -1, 6, "sandstone"); for (x, z) in [(5, 6), (7, 6), (6, 5), (6, 7)] { p.n(x, 1, z, "sandstone_slab"); }
    let palm = crate::gen::palm_blocks(0.0, 1.0, 0.0, (p.h * 1e6) as u32 + 41, 0.35); for b in palm { let (x, y, z) = (8 + b[0], b[1], 8 + b[2]); if !p.filled(x, y, z) { p.put(x, y, z, b[3] as crate::blocks::Id); } }
    p.turn(4, 1, 8, "trough", 1, 0); p.turn(8, 1, 4, "trough", -1, 0);
    // les chambres des voyageurs à gauche, l'écurie à droite, le comptoir à l'entrée, le salon au fond
    for z in [5, 7, 9] { p.turn(1, 1, z, "bed", -1, 0); p.turn(2, 1, z, "bed_foot", -1, 0); } p.n(1, 1, 2, "chest");
    for z in 5..=9 { p.n(11, 1, z, "hay_bale"); } p.n(10, 1, 7, "trough"); p.n(11, 2, 6, "hay_bale");
    p.n(10, 1, 2, "table"); p.n(11, 1, 2, "table"); p.n(10, 2, 2, "bottles"); p.n(11, 1, 1, "barrel"); p.n(9, 1, 1, "amphora");
    for x in 4..=8 { p.n(x, 1, 11, "berber_rug"); } p.n(4, 1, 10, "pouf"); p.n(8, 1, 10, "pouf"); p.n(6, 1, 10, "round_table");
    for (x, z) in [(2, 6), (2, 10), (10, 10), (6, 11), (10, 4), (2, 2)] { p.n(x, 4, z, "moroccan_lantern"); }
    for y in 1..=4 { p.put(11, y, 11, variant(id("ladder"), p.face(1, 0), false, false, false)); } p.n(11, 5, 11, "trapdoor");
    Built { door: Some(6), spot: None, bed: Some((2, 6)) }
}

// ------------------------------------------------------------------ la maison du conseil
fn conseil(p: &mut P, w: i32, tower: bool) -> Built {
    let d = w; let door = w / 2; let c = w / 2; p.ground([-1, -2, w, d], 20, [0, 0, w - 1, d - 1], "terracotta_floor", "sandstone");
    walls(p, (0, w - 1), (0, d - 1), (1, 4), "plaster"); for x in 0..w { for z in 0..d { if is_edge(x, z, (0, w - 1), (0, d - 1)) { p.n(x, 1, z, "sandstone"); } } }
    p.fill((0, w - 1), (5, 5), (0, d - 1), "sandstone"); parapet(p, (0, w - 1), (0, d - 1), 6, "plaster", true);
    p.door(door, 1, 0, 0, -1); p.turn(door, 3, -1, "awning_blue", 0, 1); p.turn(door - 2, 3, -1, "banner_gold", 0, 1); p.turn(door + 2, 3, -1, "banner_gold", 0, 1);
    for t in (1..w - 1).step_by(2) { if t != door { arch_win(p, t, 2, 0, true); } arch_win(p, t, 2, d - 1, true); arch_win(p, 0, 2, t, false); arch_win(p, w - 1, 2, t, false); }
    // la salle haute et sa coupole blanche
    let (a, b) = (2, w - 3); walls(p, (a, b), (a, b), (6, 8), "plaster"); p.fill((a, b), (9, 9), (a, b), "plaster"); p.door(c, 6, a, 0, -1); arch_win(p, a, 7, c, false); arch_win(p, b, 7, c, false); arch_win(p, c, 7, b, true);
    dome(p, c, c, (b - a) / 2 - 1, 10, "plaster");
    // dedans : tapis et poufs autour d'une table basse, lanternes, coffre ; l'échelle vers les toits
    for x in 2..=w - 3 { for z in 2..=d - 3 { p.n(x, 1, z, "berber_rug"); } } p.n(c, 1, c, "round_table"); for (dx, dz) in [(-2, 0), (2, 0), (0, 2), (-2, 2), (2, 2)] { p.n(c + dx, 1, c + dz, "pouf"); }
    p.n(w - 2, 1, 1, "chest"); p.n(1, 1, 1, "amphora"); for (x, z) in [(c, 2), (c, d - 3), (2, c), (w - 3, c)] { p.n(x, 4, z, "moroccan_lantern"); }
    for y in 1..=4 { p.put(1, y, d - 2, variant(id("ladder"), p.face(-1, 0), false, false, false)); } p.n(1, 5, d - 2, "trapdoor");
    if tower {
        // la tour de guet, à l'angle avant droit : elle domine le village et l'oasis
        let (x0, z0) = (w - 3, 0); walls(p, (x0, x0 + 2), (z0, z0 + 2), (6, 12), "plaster"); p.n(x0 + 1, 12, z0 + 1, "sandstone"); parapet(p, (x0, x0 + 2), (z0, z0 + 2), 13, "plaster", true);
        p.n(x0 + 1, 13, z0 + 1, "moroccan_lantern"); for y in [8, 11] { arch_win(p, x0 + 1, y, z0, true); arch_win(p, x0 + 2, y, z0 + 1, false); } p.turn(x0 + 1, 12, z0 - 1, "banner_blue", 0, 1);
    }
    Built { door: Some(door), spot: None, bed: None }
}

// ------------------------------------------------------------------ l'enclos à dromadaires (et la ferme)
fn enclos(p: &mut P, w: i32, house: bool, wall: &str) -> Built {
    let d = 9; let gate = w / 2; p.ground([-1, -1, w, d], 8, [0, 0, w - 1, d - 1], "sand", "sandstone");
    for x in 0..w { for z in 0..d { if is_edge(x, z, (0, w - 1), (0, d - 1)) { p.n(x, 1, z, "sandstone"); if (x + z) % 3 == 0 { p.n(x, 2, z, "sandstone_slab"); } } } }
    for x in gate..=gate + 1 { p.put(x, 1, 0, 0); p.put(x, 2, 0, 0); } for x in [gate - 1, gate + 2] { p.n(x, 1, 0, "palm_log"); p.n(x, 2, 0, "palm_log"); }
    // l'abri de paille au fond, sur des poteaux de palmier
    let x0 = if house { 6 } else { 0 }; for x in (x0..w).step_by(3) { for y in 1..=2 { p.n(x, y, d - 3, "palm_log"); } } p.fill((x0, w - 1), (3, 3), (d - 3, d - 1), "thatch");
    for x in x0 + 1..w - 1 { if x % 3 != 0 { p.n(x, 1, d - 2, "hay_bale"); } } p.turn(x0 + 2, 1, 3, "trough", 0, 1); p.turn(x0 + 3, 1, 3, "trough", 0, 1); p.n(w - 2, 1, 1, "amphora");
    if !house { return Built { door: Some(gate), spot: None, bed: None }; }
    // la ferme : la maison de pisé adossée au coin du fond, sa porte sur la cour
    walls(p, (0, 4), (4, d - 1), (1, 3), wall); p.fill((0, 4), (4, 4), (4, d - 1), "sandstone"); parapet(p, (0, 4), (4, d - 1), 5, wall, false);
    for x in 1..=3 { for z in 5..=d - 2 { for y in 1..=3 { p.put(x, y, z, 0); } p.n(x, 0, z, "terracotta_floor"); } }
    p.door(2, 1, 4, 0, -1); arch_win(p, 4, 2, 6, false); p.turn(1, 1, 7, "bed", 0, 1); p.turn(1, 1, 6, "bed_foot", 0, 1); p.n(3, 1, 7, "chest"); p.n(3, 1, 5, "sack"); p.n(2, 3, 6, "moroccan_lantern");
    for x in 1..=3 { p.n(x, 1, 2, "sack"); } p.n(1, 2, 2, "sack"); // la récolte de dattes, en sacs
    Built { door: Some(gate), spot: None, bed: Some((2, 7)) }
}
