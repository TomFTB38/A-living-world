//! Les villages côtiers : leurs bâtiments, bâtis bloc par bloc comme les patrons médiévaux.
//!
//! Des maisons de pêcheurs basses et longues, en granite ou blanchies à la chaux sur un soubassement de granite, sous un
//! toit d'ardoise pentu dont le faîte suit la rue, une souche de cheminée à chaque pignon, des volets ; devant, des
//! casiers, une ancre, des cordages ; parfois un appentis pour le matériel de pêche. Les ateliers sont des hangars de
//! planches sur un muret de granite, ouverts sur la rue. L'auberge du port a un étage blanchi, une enseigne, une terrasse.
//! La capitainerie, en granite, porte une tourelle de guet avec sa lanterne et son fanion. Les fermes, l'église et le
//! moulin restent ceux de la campagne. Le port (ponton, voilier, halle aux poissons) et le phare sont dans `village`.
//! Vérifié par `tests/cote.rs`.
use crate::batisse::{Lot, Work};
use crate::edifice::Arch;
use crate::patrons::{Built, Plan, Rec, P};

/// Les bâtiments de la côte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKind { Maison, Atelier, Auberge, Capitainerie, GrandeCapitainerie }
/// Le bâtiment côtier qui tient la place d'un archétype ; aucun pour les bâtisses de la campagne (fermes, église, moulin).
pub fn kind_of(a: Arch) -> Option<CKind> {
    match a { Arch::House | Arch::Cottage => Some(CKind::Maison), Arch::Inn => Some(CKind::Auberge), Arch::Hall => Some(CKind::Capitainerie), Arch::TownHall => Some(CKind::GrandeCapitainerie),
        Arch::Forge | Arch::Workshop | Arch::Tannery | Arch::Woodshop => Some(CKind::Atelier), _ => None }
}
impl CKind {
    pub fn label(self) -> &'static str { match self { CKind::Maison => "maison de pêcheur", CKind::Atelier => "hangar d'artisan", CKind::Auberge => "auberge du port", CKind::Capitainerie => "capitainerie", CKind::GrandeCapitainerie => "grande capitainerie" } }
}
/// Corps, porte sur la façade (z = 0) et emprise. Maison : bit 0, plus longue ; bit 1, blanchie à la chaux ; bit 2, en
/// miroir ; bit 3, un appentis au pignon.
pub fn plan(k: CKind, var: u8) -> Plan {
    let (w, d, door, ext) = match k {
        CKind::Maison => { let w = if var & 1 != 0 { 8 } else { 7 }; (w, 5, 2, [-1, -2, w + if var & 8 != 0 { 2 } else { 0 }, 6]) }
        CKind::Atelier => (7, 6, 5, [-1, -2, 7, 7]), CKind::Auberge => (11, 7, 5, [-1, -3, 11, 8]),
        CKind::Capitainerie => (9, 7, 3, [-1, -2, 9, 8]), CKind::GrandeCapitainerie => (11, 8, 4, [-1, -2, 11, 9]),
    };
    let mirror = k == CKind::Maison && var & 4 != 0;
    if !mirror { return Plan { w, d, door, ext, mirror }; }
    Plan { w, d, door: w - 1 - door, ext: [w - 1 - ext[2], ext[1], w - 1 - ext[0], ext[3]], mirror }
}

/// Bâtit un bâtiment côtier, le sol à y0 ; rend porte, poste et lit en coordonnées du terrain.
#[allow(clippy::too_many_arguments)]
pub fn build(k: &mut Work, lot: Lot, y0: i32, kind: CKind, var: u8, h: f64, rich: f64, craft: Option<&'static str>) -> Built {
    let pl = plan(kind, var); let mut p = P { k, lot, y0, mirror: pl.mirror, w: pl.w, d: pl.d, h, rec: Rec::default() };
    let b = match kind {
        CKind::Maison => maison(&mut p, pl.w, var), CKind::Atelier => atelier(&mut p, craft), CKind::Auberge => auberge(&mut p, rich),
        CKind::Capitainerie => capitainerie(&mut p, 9, 7), CKind::GrandeCapitainerie => capitainerie(&mut p, 11, 8),
    };
    p.dress(); // les volets aux fenêtres, les chaînes des lanternes
    let fx = |x: i32| if pl.mirror { pl.w - 1 - x } else { x };
    Built { door: b.door.map(fx), spot: b.spot.map(|(a, s)| ((fx(a.0), a.1), (fx(s.0), s.1))), bed: b.bed.map(|(x, z)| (fx(x), z)) }
}

fn is_edge(x: i32, z: i32, xr: (i32, i32), zr: (i32, i32)) -> bool { x == xr.0 || x == xr.1 || z == zr.0 || z == zr.1 }
/// Des murs sur le pourtour : la chaîne d'angle et le soubassement en granite, le reste dans la matière donnée.
fn walls(p: &mut P, x: (i32, i32), z: (i32, i32), y: (i32, i32), mat: &str) {
    for xx in x.0..=x.1 { for zz in z.0..=z.1 { if !is_edge(xx, zz, x, z) { continue; } let corner = (xx == x.0 || xx == x.1) && (zz == z.0 || zz == z.1);
        for yy in y.0..=y.1 { p.n(xx, yy, zz, if corner || yy == 1 { "granite" } else { mat }); } } }
}
/// Une souche de cheminée en granite qui monte du sol du pignon jusqu'au-dessus du faîte, coiffée d'une dalle.
fn chimney(p: &mut P, x: i32, z: i32, top: i32) { for y in 1..=top { p.n(x, y, z, "granite"); } p.n(x, top + 1, z, "granite_slab"); }

// ------------------------------------------------------------------ la maison de pêcheur
fn maison(p: &mut P, w: i32, var: u8) -> Built {
    let d = 5; let wall = if var & 2 != 0 { "plaster" } else { "granite" }; let door = 2;
    p.ground([-1, -2, w + if var & 8 != 0 { 2 } else { 0 }, d + 1], 10, [0, 0, w - 1, d - 1], "spruce_planks", "granite");
    walls(p, (0, w - 1), (0, d - 1), (1, 3), wall);
    p.roof((0, w - 1), (0, d - 1), 4, 1, false, "slate", Some(wall)); chimney(p, 0, 2, 7); chimney(p, w - 1, 2, 7);
    p.door(door, 1, 0, 0, -1); p.win(4, 2, 0, true); p.win(w - 2, 2, 0, true); p.win(3, 2, d - 1, true); p.win(w - 3, 2, d - 1, true);
    // dedans : l'âtre au pignon, la table et le tabouret, le lit, le coffre, une lanterne
    p.turn(1, 1, 2, "fireplace", -1, 0); p.n(3, 1, 2, "table"); p.n(3, 1, 1, "stool"); p.n(1, 1, 3, "chest"); p.n(3, 3, 2, "lantern");
    p.turn(w - 2, 1, 3, "bed", 1, 0); p.turn(w - 3, 1, 3, "bed_foot", 1, 0); p.n(w - 2, 1, 1, "barrel");
    // devant : casiers empilés, une ancre, des cordages ; une bouée accrochée au mur
    p.n(0, 1, -1, "creel"); p.n(0, 2, -1, "creel"); p.turn(w - 1, 1, -1, "anchor", 0, 1); p.n(1, 1, -2, "rope_coil"); p.turn(w - 3, 1, -1, "bench", 0, 1);
    if var & 8 != 0 {
        // l'appentis au pignon : des poteaux, des planches, un toit d'ardoise qui descend ; dedans, les filets et les avirons
        let x0 = w; for z in 0..d { for x in [x0, x0 + 1] { p.n(x, 0, z, "granite"); } } for z in [0, d - 1] { for y in 1..=2 { p.n(x0 + 1, y, z, "spruce_log"); } }
        for z in 1..d - 1 { for y in 1..=2 { p.n(x0 + 1, y, z, "spruce_planks"); } } for z in 0..d { p.stair(x0, 3, z, "slate_stairs", -1, 0, false); p.stair(x0 + 1, 2, z, "slate_stairs", -1, 0, false); }
        p.n(x0 + 1, 3, 0, "spruce_planks"); p.n(x0 + 1, 3, d - 1, "spruce_planks"); p.turn(x0, 1, 2, "net_pile", 0, 1); p.turn(x0, 1, 3, "oars", 1, 0); p.n(x0, 1, 1, "buoy");
    }
    Built { door: Some(door), spot: None, bed: Some((w - 4, 3)) }
}

// ------------------------------------------------------------------ le hangar d'artisan
fn atelier(p: &mut P, craft: Option<&'static str>) -> Built {
    let (w, d) = (7, 6); p.ground([-1, -2, w, d + 1], 10, [0, 0, w - 1, d - 1], "spruce_planks", "granite");
    walls(p, (0, w - 1), (0, d - 1), (1, 3), "spruce_planks"); for x in 0..w { for z in 0..d { if is_edge(x, z, (0, w - 1), (0, d - 1)) { p.n(x, 1, z, "granite"); } } }
    for x in 1..=3 { for y in 1..=2 { p.put(x, y, 0, 0); } } p.n(0, 1, 0, "spruce_log"); p.n(0, 2, 0, "spruce_log"); p.n(4, 1, 0, "spruce_log"); p.n(4, 2, 0, "spruce_log"); // l'atelier ouvert sur la rue
    p.roof((0, w - 1), (0, d - 1), 4, 1, false, "slate", Some("spruce_planks")); p.door(5, 1, 0, 0, -1); p.win(w - 1, 2, 3, false); p.win(3, 2, d - 1, true);
    let station = match craft { Some("forgeron" | "maitre_forgeron" | "armurier") => "anvil", Some("tanneur") => "cauldron", Some("tisserand" | "tailleur") => "loom", _ => "workbench" };
    p.n(2, 1, 3, station); if station == "anvil" { p.n(1, 1, 4, "furnace"); chimney(p, 0, 4, 7); } else { p.n(1, 1, 4, "barrel"); p.n(1, 1, 1, "crate"); }
    p.n(3, 3, 2, "lantern"); p.turn(5, 1, 4, "bed", 0, 1); p.turn(5, 1, 3, "bed_foot", 0, 1); p.n(-1, 1, -1, "rope_coil"); p.n(1, 1, -2, "fish_crate");
    Built { door: Some(5), spot: Some(((2, 2), (2, 3))), bed: Some((4, 4)) }
}

// ------------------------------------------------------------------ l'auberge du port
fn auberge(p: &mut P, rich: f64) -> Built {
    let (w, d) = (11, 7); let up = if rich > 0.5 { "plaster" } else { "plaster_blue" }; p.ground([-1, -3, w, d + 1], 12, [0, 0, w - 1, d - 1], "spruce_planks", "granite");
    walls(p, (0, w - 1), (0, d - 1), (1, 3), "granite"); walls(p, (0, w - 1), (0, d - 1), (5, 6), up); for x in 0..w { for z in 0..d { if is_edge(x, z, (0, w - 1), (0, d - 1)) { p.n(x, 4, z, "spruce_planks"); } } }
    p.fill((1, w - 2), (4, 4), (1, d - 2), "spruce_planks"); p.roof((0, w - 1), (0, d - 1), 7, 1, false, "slate", Some(up)); chimney(p, 0, 3, 11); chimney(p, w - 1, 3, 11);
    p.door(5, 1, 0, 0, -1); for x in [2, 8] { p.win(x, 2, 0, true); p.win(x, 2, d - 1, true); } for x in [2, 5, 8] { p.win(x, 5, 0, true); p.win(x, 5, d - 1, true); } for z in [2, 4] { p.win(0, 5, z, false); p.win(w - 1, 5, z, false); }
    p.turn(6, 3, -1, "hanging_sign", 0, 1); p.turn(4, 3, -1, "wall_sconce", 0, 1);
    // la salle : le comptoir, les tonneaux, les tables ; l'échelle vers les chambres
    for x in 6..=9 { p.n(x, 1, 4, "table"); } p.n(9, 1, 5, "barrel"); p.n(8, 1, 5, "barrel"); p.n(7, 2, 4, "bottles");
    for (x, z) in [(2, 2), (3, 4)] { p.n(x, 1, z, "table"); p.n(x + 1, 1, z, "stool"); p.n(x - 1, 1, z, "stool"); } p.n(5, 3, 3, "lantern"); p.turn(1, 1, 3, "fireplace", -1, 0);
    for y in 1..=4 { p.put(1, y, 5, crate::shapes::variant(crate::batisse::id("ladder"), p.face(-1, 0), false, false, false)); }
    for x in [3, 6, 9] { p.turn(x, 5, 5, "bed", 0, 1); p.turn(x, 5, 4, "bed_foot", 0, 1); } p.n(5, 6, 3, "lantern"); p.n(2, 5, 1, "chest");
    // la terrasse devant : deux tables, des tabourets, un tonneau
    for x in [2, 8] { p.n(x, 1, -2, "table"); p.n(x - 1, 1, -2, "stool"); p.n(x + 1, 1, -2, "stool"); } p.n(10, 1, -1, "barrel");
    Built { door: Some(5), spot: None, bed: Some((4, 5)) }
}

// ------------------------------------------------------------------ la capitainerie
fn capitainerie(p: &mut P, w: i32, d: i32) -> Built {
    let door = w / 2 - 1; p.ground([-1, -2, w, d + 1], 14, [0, 0, w - 1, d - 1], "spruce_planks", "granite");
    walls(p, (0, w - 1), (0, d - 1), (1, 4), "granite"); p.roof((0, w - 1), (0, d - 1), 5, 1, false, "slate", Some("granite"));
    p.door(door, 1, 0, 0, -1); for x in (1..w - 1).step_by(2) { if x != door && x < w - 3 { p.win(x, 2, 0, true); } p.win(x, 2, d - 1, true); } p.win(0, 2, d / 2, false);
    p.turn(door - 1, 1, -1, "anchor", 0, 1); p.turn(door + 1, 3, -1, "wall_sconce", 0, 1); p.n(door + 1, 1, -2, "rope_coil");
    // la tourelle de guet, à l'angle avant droit : quatre murs de granite, une plateforme, sa lanterne et son fanion
    let (x0, top) = (w - 3, 12); for y in 1..=top { for (x, z) in [(x0, 0), (x0 + 2, 0), (x0, 2), (x0 + 2, 2), (x0 + 1, 0), (x0 + 1, 2), (x0, 1), (x0 + 2, 1)] { p.n(x, y, z, "granite"); } }
    for y in 1..=top { p.put(x0 + 1, y, 1, 0); } for y in 1..top { p.put(x0 + 1, y, 1, crate::shapes::variant(crate::batisse::id("ladder"), p.face(0, 1), false, false, false)); }
    for y in [6, 9] { p.win(x0 + 1, y, 0, true); p.win(x0 + 2, y, 1, false); } p.n(x0 + 1, top, 1, "spruce_planks");
    for (x, z) in [(x0, 0), (x0 + 2, 0), (x0, 2), (x0 + 2, 2)] { p.n(x, top + 1, z, "granite"); p.n(x, top + 2, z, "granite"); } for (x, z) in [(x0 + 1, 0), (x0, 1), (x0 + 2, 1), (x0 + 1, 2)] { p.n(x, top + 1, z, "fence"); }
    p.pyramid(x0 + 1, 1, 1, top + 3, "slate"); p.n(x0 + 1, top + 1, 1, "lantern"); p.n(x0 + 1, top + 5, 1, "pennant");
    // dedans : la grande table des cartes, le coffre aux registres, les lanternes
    p.n(2, 1, d / 2, "table"); p.n(3, 1, d / 2, "table"); p.n(2, 1, d / 2 + 1, "chair"); p.n(1, 1, d - 2, "chest"); p.n(3, 3, d / 2, "lantern"); p.n(w - 5, 1, d - 2, "barrel");
    Built { door: Some(door), spot: None, bed: None }
}
