//! Générateur procédural de bâtiments médiévaux (phase 2 de la refonte des villages). Cinq archétypes entièrement
//! bâtis, chacun en plusieurs variantes de silhouette et de plan (pas seulement de couleur) : chaumière, maison à
//! colombages, forge, auberge, grange. Chacun a son intérieur fonctionnel (lit, foyer, table, coffre, postes de travail,
//! comptoir, chambres, réserves) et un état (neuf, entretenu, ancien, réparé, endommagé, abandonné, en construction).
//! La chapelle, le moulin et les autres ateliers reprennent encore les plans de `batisse` en attendant leur tour.
//! Tout se déduit d'une graine stable : un bâtiment garde exactement son identité d'un chargement à l'autre.
use crate::batisse::{cross_dir, id, layer, turned, Lot, Plan, Style, Work, B, C3};
use crate::blocks::Id;

/// Les archétypes de bâtiments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arch { Cottage, TimberHouse, Forge, Inn, Barn, Workshop, Chapel, Mill }
/// L'état d'un bâtiment. Il est déduit de l'âge, de la richesse et de l'orientation du village ; il pourra évoluer en jeu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cond { New, Kept, Old, Repaired, Damaged, Abandoned, Building }
impl Cond { pub fn label(self) -> &'static str { match self { Cond::New => "neuf", Cond::Kept => "entretenu", Cond::Old => "ancien", Cond::Repaired => "réparé", Cond::Damaged => "endommagé", Cond::Abandoned => "abandonné", Cond::Building => "en construction" } } }
impl Arch {
    pub fn label(self) -> &'static str { match self { Arch::Cottage => "chaumière", Arch::TimberHouse => "maison à colombages", Arch::Forge => "forge", Arch::Inn => "auberge", Arch::Barn => "grange", Arch::Workshop => "atelier", Arch::Chapel => "chapelle", Arch::Mill => "moulin" } }
    /// Nombre de variantes géométriques.
    pub fn variants(self) -> u8 { match self { Arch::Cottage | Arch::TimberHouse => 6, Arch::Forge | Arch::Inn => 3, Arch::Barn => 4, _ => 1 } }
}

/// La fiche d'un bâtiment : archétype, variante, dimensions du corps principal, étages, état, graine, richesse, métier.
#[derive(Clone, Copy, Debug)]
pub struct Spec { pub arch: Arch, pub var: u8, pub w: i32, pub d: i32, pub floors: i32, pub cond: Cond, pub h: f64, pub rich: f64, pub craft: Option<&'static str>, pub door_side: u8 }
impl Spec {
    pub fn new(arch: Arch, var: u8, h: f64, rich: f64, craft: Option<&'static str>, cond: Cond) -> Spec {
        let k = |n: i32| (h * 7919.0).floor() as i32 % n; let var = var % arch.variants();
        let (w, d, floors, door_side) = match arch {
            Arch::Cottage => (5 + k(3), 6 + k(2) + if var == 1 { 3 } else { 0 }, 1, 0),
            Arch::TimberHouse => match var { 1 => (5, 6 + k(2), 3, 0), 2 => (6 + k(2), 8, 2, 1), _ => (5 + k(3), 6 + k(3), 2, 0) },
            Arch::Forge => (if craft == Some("maitre_forgeron") { 9 } else { 7 }, 6, 1, 0),
            Arch::Inn => if var == 2 { (7, 9, 3, 0) } else { (9, 8, 2, 0) },
            Arch::Barn => if var == 3 { (7, 8, 1, 0) } else { (7 + 2 * k(2), 9 + 2 * k(2), 1, 0) },
            Arch::Workshop => { let (w, d) = Plan::Shop("").size(); (w, d, 2, 0) }
            Arch::Chapel => { let (w, d) = Plan::Church.size(); (w, d, 1, 0) }
            Arch::Mill => (3, 3, 1, 0),
        };
        Spec { arch, var, w, d, floors, cond, h, rich, craft, door_side }
    }
    /// Habité (un lit, une porte, des habitants) ?
    pub fn dwelling(&self) -> bool { !matches!(self.arch, Arch::Barn | Arch::Chapel | Arch::Mill) && !matches!(self.cond, Cond::Abandoned | Cond::Building) }
    /// L'emprise au sol, débords de toit, annexes et abords compris, en coordonnées du terrain : [u0, w0, u1, w1] inclus.
    pub fn extent(&self) -> [i32; 4] {
        let (w, d) = (self.w, self.d); let mut e = [-1, -1, w, d];
        match (self.arch, self.var) {
            (Arch::Cottage, 2) => e[2] = w + 3, (Arch::Cottage, 4) => e[0] = -6, (Arch::Cottage, 5) => e[2] = w + 2, (Arch::Cottage, _) => e[2] = w + 1,
            (Arch::TimberHouse, 3) => e[0] = -4, (Arch::TimberHouse, _) => {}
            (Arch::Forge, 1) => { e[2] = w + 5; e[0] = -2; } (Arch::Forge, 2) => e[0] = -4, (Arch::Forge, _) => e[0] = -2,
            (Arch::Inn, 1) => { e[0] = -5; e[2] = w + 1; } (Arch::Inn, _) => e[2] = w + 5,
            (Arch::Barn, 2) => { e[2] = w + 3; e[1] = -4; } (Arch::Barn, _) => { e[1] = -4; e[2] = w + 1; }
            (Arch::Workshop, _) => { e = [-3, -2, w + 1, d]; } (Arch::Chapel, _) => { e = [-1, -4, w, d]; } (Arch::Mill, _) => { e = [-3, -6, 5, 4]; }
        }
        e
    }
}

/// Ce que le village doit savoir d'un bâtiment bâti : la porte (côté, case le long du mur), le poste de travail (case où
/// l'on se tient, case du poste), le lit (case où l'on se couche), en coordonnées du terrain.
#[derive(Clone, Copy, Debug, Default)]
pub struct Made { pub door: Option<(u8, i32)>, pub spot: Option<((i32, i32), (i32, i32))>, pub bed: Option<(i32, i32)> }

/// Les matières d'un bâtiment médiéval selon sa richesse et son âge.
fn mats(sp: &Spec) -> Style {
    let mut s = crate::batisse::style("prairie");
    let wall = if sp.rich > 0.62 { "limestone_masonry" } else if sp.h < 0.5 { "rubble_wall" } else { "fieldstone" };
    s.found = id("rubble_wall"); s.ground = id(wall); s.upper = id(if sp.rich > 0.75 && sp.h > 0.7 { "plaster_ochre" } else { "plaster" }); s.post = id("timber"); s.frame = true;
    s.floor = id("planks"); s.core = id("planks"); s.thatch = id("thatch"); s.chimney = id(if sp.rich > 0.5 { "stone_bricks" } else { "rubble_wall" }); s.pave = id("cobblestone");
    let aged = matches!(sp.cond, Cond::Old | Cond::Damaged | Cond::Abandoned);
    s.tiles = id(if aged { "mossy_roof_tiles" } else if sp.rich > 0.72 { "slate" } else { "roof_tiles" }); s.noble = id("slate");
    s
}
/// Prépare le terrain : volume dégagé, fondations sous le corps principal, plancher.
fn prep(b: &mut B, ext: [i32; 4], y0: i32, clear: i32) {
    for u in ext[0]..=ext[2] { for ww in ext[1]..=ext[3] { for y in y0 + 1..y0 + clear { b.put(u, y, ww, 0); } } }
    let (w, d, s) = (b.w, b.d, b.s); for u in 0..w { for ww in 0..d { for y in y0 - 3..y0 { b.put(u, y, ww, s.found); } b.put(u, y0, ww, s.floor); } }
}
fn f_of(lot: Lot, du: i32, dw: i32) -> u8 { crate::shapes::facing_of((du * lot.u.0 + dw * lot.w.0) as f64, (du * lot.u.1 + dw * lot.w.1) as f64) }
/// Un appentis contre le flanc droit (côté 1) ou gauche (côté 3), de `ww0` à `ww1`, sur `depth` cases : fermé de `wall`, ou ouvert sur poteaux.
/// Un passage est percé dans le mur de la maison s'il est fermé. Rend les cases intérieures.
#[allow(clippy::too_many_arguments)]
fn lean_to(b: &mut B, side: u8, ww0: i32, ww1: i32, depth: i32, y0: i32, wall: Option<Id>, cover: Id) -> Vec<(i32, i32)> {
    let s = b.s; let w = b.w; let us: Vec<i32> = if side == 1 { (w..w + depth).collect() } else { (-depth..0).collect() }; let outer = if side == 1 { w + depth - 1 } else { -depth };
    let mut cells = Vec::new();
    for &u in &us { for ww in ww0..=ww1 { for y in y0 - 2..y0 { b.put(u, y, ww, s.found); } b.put(u, y0, ww, s.floor); for y in y0 + 1..y0 + 4 { b.put(u, y, ww, 0); }
        let edge = u == outer || ww == ww0 || ww == ww1;
        match wall { Some(m) if edge => { for y in y0 + 1..=y0 + 2 { b.put(u, y, ww, m); } } _ => cells.push((u, ww)) } } }
    if wall.is_none() { for ww in [ww0, ww1] { for y in y0 + 1..=y0 + 2 { b.put(outer, y, ww, id("oak_beam")); } } cells.retain(|c| !(c.0 == outer && (c.1 == ww0 || c.1 == ww1))); }
    else { let hu = if side == 1 { w - 1 } else { 0 }; let mw = (ww0 + ww1) / 2; for y in y0 + 1..=y0 + 2 { b.put(hu, y, mw, 0); } }
    let slab = id("planks_slab"); for &u in &us { for ww in ww0..=ww1 { b.put(u, y0 + 3, ww, slab); } }
    let lot = b.lot; let (uin, uout) = if side == 1 { (w as f64, (w + depth) as f64 + 0.35) } else { (0.0, -depth as f64 - 0.35) }; let y = y0 as f64;
    let at = lot.cell(us[0], y0 + 3, ww0);
    b.k.quad(at, [lot.p(uin, y + 4.3, ww0 as f64 - 0.3), lot.p(uin, y + 4.3, ww1 as f64 + 1.3), lot.p(uout, y + 3.6, ww1 as f64 + 1.3), lot.p(uout, y + 3.6, ww0 as f64 - 0.3)], [1.0, 1.0, 1.0], layer(cover), lot.grain());
    let beam = s.beam; b.k.bar(at, lot.p(uout - if side == 1 { 0.15 } else { -0.15 }, y + 3.55, ww0 as f64 - 0.3), lot.p(uout - if side == 1 { 0.15 } else { -0.15 }, y + 3.55, ww1 as f64 + 1.3), [0.0, 1.0, 0.0], 0.06, 0.07, beam, -1.0);
    cells
}
/// Un petit auvent sur consoles au-dessus d'une porte (façade avant).
fn porch(b: &mut B, du: i32, y: f64, cover: Id) {
    let lot = b.lot; let at = lot.cell(du, y as i32 - 1, 0); let n = b.normal(0); let beam = b.s.beam; let (l, r, out) = (du as f64 - 0.45, du as f64 + 1.45, 0.95);
    b.k.quad(at, [lot.p(l, y + 0.32, 0.0), lot.p(r, y + 0.32, 0.0), lot.p(r, y, -out), lot.p(l, y, -out)], [1.0, 1.0, 1.0], layer(cover), lot.grain() % 2 + 1);
    for t in [l + 0.08, r - 0.08] { b.k.bar(at, lot.p(t, y - 0.55, 0.0), lot.p(t, y + 0.05, -out + 0.12), cross_dir(n), 0.045, 0.045, beam, -1.0); }
    b.k.cube(at, lot.p(du as f64 - 0.25, y - 2.16, -0.75), lot.p(du as f64 + 1.25, y - 1.94, 0.0), [1.0, 1.0, 1.0], layer(b.s.found)); // le perron
}
/// Fenêtres d'un mur sur une rangée, en évitant les cases données.
fn windows(b: &mut B, side: u8, y: i32, ts: &[i32], avoid: &[i32]) { let len = b.len(side); for &t in ts { if t > 0 && t < len - 1 && !avoid.contains(&t) { b.window(side, t, y); } } }
/// Un escalier droit le long d'un mur, de l'étage `y` à l'étage `y + 4`, en montant vers le fond ; la trémie est percée.
fn stair(b: &mut B, u: i32, ww0: i32, y: i32) {
    let st = crate::shapes::variant(id("planks_stairs"), b.lot_face(), false, false, false);
    for k in 0..4 { b.put(u, y + 1 + k, ww0 + k, st); if k < 3 { b.put(u, y + 4, ww0 + k, 0); } }
}

/// Bâtit la fiche `sp` sur son terrain, le sol à `y0`. Rend la porte, le poste et le lit.
pub fn raise(k: &mut Work, lot: Lot, sp: &Spec, y0: i32) -> Made {
    const PAINT: [C3; 5] = [[0.24, 0.36, 0.26], [0.22, 0.3, 0.42], [0.5, 0.18, 0.14], [0.38, 0.3, 0.2], [0.3, 0.4, 0.42]];
    // Les plans encore anciens : chapelle, moulin, ateliers.
    match sp.arch {
        Arch::Chapel | Arch::Mill | Arch::Workshop => {
            let plan = match sp.arch { Arch::Chapel => Plan::Church, Arch::Mill => Plan::Mill, _ => Plan::Shop(sp.craft.unwrap_or("menuisier")) };
            let (du, back) = crate::batisse::raise(k, lot, plan, y0, crate::batisse::style("prairie"), sp.h);
            if sp.arch != Arch::Workshop { return Made::default(); }
            let fb = f_of(lot, 0, 1); k.put(lot.cell(1, y0 + 1, back), crate::shapes::variant(id("bed"), fb, false, false, false)); k.put(lot.cell(1, y0 + 1, back - 1), crate::shapes::variant(id("bed_foot"), fb, false, false, false));
            let station = match sp.craft { Some("tailleur_pierre") => "mason_bench", _ => "workbench" }; k.put(lot.cell(du, y0 + 1, back), id(station)); k.put(lot.cell(du + 1, y0 + 1, back), id(if station == "mason_bench" { "cobblestone" } else { "planks" }));
            return Made { door: Some((0, du)), spot: Some(((du, back - 1), (du, back))), bed: Some((2, back)) };
        }
        _ => {}
    }
    let s = mats(sp); let mut b = B { k, lot, w: sp.w, d: sp.d, s, gaps: Vec::new(), paint: PAINT[(sp.h * 5.0) as usize % 5], h: sp.h };
    if sp.cond == Cond::Building { return building_site(&mut b, sp, y0); }
    let made = match sp.arch { Arch::Cottage => cottage(&mut b, sp, y0), Arch::TimberHouse => timber_house(&mut b, sp, y0), Arch::Forge => forge(&mut b, sp, y0), Arch::Inn => inn(&mut b, sp, y0), _ => barn(&mut b, sp, y0) };
    wear(&mut b, sp, y0);
    made
}

// ------------------------------------------------------------------------------------------------ chaumière
/// La chaumière : murs de moellons (ou pans de bois), un épais toit de chaume qui descend bas, une cheminée de pierre.
/// Variantes : simple ; longère avec son étable au fond ; en L avec un cellier ; à colombages ; avec auvent et potager
/// clos ; avec un bûcher ouvert.
fn cottage(b: &mut B, sp: &Spec, y0: i32) -> Made {
    let (w, d) = (b.w, b.d); let du = w / 2; let s = b.s; let timber = sp.var == 3; let byre = sp.var == 1;
    prep(b, sp.extent(), y0, 4 + w / 2 + 4);
    if timber { b.walls(y0 + 1, y0 + 3, s.found, s.upper, s.post); } else { b.walls(y0 + 1, y0 + 3, s.found, s.ground, s.ground); }
    let room = if byre { d - 4 } else { d - 1 }; // le mur du fond de la pièce à vivre
    if byre { let boards = id("barn_boards"); for y in y0 + 1..=y0 + 3 { for ww in d - 3..d { b.put(0, y, ww, boards); b.put(w - 1, y, ww, boards); } for u in 1..w - 1 { b.put(u, y, d - 1, boards); b.put(u, y, room, s.core); } } b.put(1, y0 + 1, room, 0); b.put(1, y0 + 2, room, 0);
        b.door(1, d - 2, y0 + 1); }
    b.door(0, du, y0 + 1);
    windows(b, 0, y0 + 2, &[du - 2, du + 2], &[du]); let mid = room / 2 + 1; windows(b, 3, y0 + 2, &[d - 1 - mid], &[]); if !byre { windows(b, 1, y0 + 2, &[mid], &[]); }
    if timber { for side in 0..4u8 { b.frame(side, (y0 + 2) as f64, (y0 + 4) as f64); } } else { for side in 0..4u8 { b.trims(side, (y0 + 1) as f64, (y0 + 5) as f64); } }
    let yr = y0 + 4; b.roof(yr, s.thatch, if timber { s.upper } else { s.ground }, 0.42);
    let cu = w - 2; b.chimney(cu, y0 + 1, yr + w / 2 + 1);
    let mut store = Vec::new();
    match sp.var {
        2 => store = lean_to(b, 1, 1, (d - 2).min(4), 3, y0, Some(s.ground), s.thatch),
        4 => { porch(b, du, (y0 + 3) as f64 + 0.25, s.thatch); garden_plot(b, y0, sp.h); }
        5 => { let c = lean_to(b, 1, 1, d - 2, 2, y0, None, s.thatch); for (i, (u, ww)) in c.iter().enumerate() { b.put(*u, y0 + 1, *ww, id("log_pile")); if i % 2 == 0 { b.put(*u, y0 + 2, *ww, id("log_pile")); } } }
        _ => { if !byre { crate::batisse::hay(b.k, b.lot, w, y0, d); } }
    }
    if sp.var != 4 { crate::batisse::garden(b.k, b.lot, w, d, du, y0, sp.h); }
    // L'intérieur : le foyer au fond, le lit dans le coin, la table et sa chaise, le coffre près de la porte, une étagère.
    let f = b.lot_face(); let y = y0 + 1;
    b.put(cu, y, room - 1, turned("fireplace", f)); b.put(1, y, room - 1, turned("bed", f)); b.put(1, y, room - 2, turned("bed_foot", f));
    b.put(1, y, 1, id("chest")); if w >= 6 { b.put(w - 2, y, 2, id("table")); b.put(w - 2, y + 1, 2, id(if sp.h > 0.5 { "candle" } else { "bread_basket" })); b.put(w - 3, y, 2, turned("chair", f + 1)); }
    else { b.put(w - 2, y, 1, id("sack")); }
    b.put(cu, y + 2, room - 1, id("lantern")); b.put(1, y + 2, 1, turned("wall_shelf", f_of(b.lot, -1, 0)));
    if byre { b.put(w - 2, y, d - 2, turned("trough", f)); b.put(1, y, d - 2, id("hay_bale")); b.put(1, y + 1, d - 2, id("hay_bale")); b.put(w - 2, y, d - 3, id("bucket")); }
    for (u, ww) in store.iter().take(3) { b.put(*u, y, *ww, id(["barrel", "sack", "crate"][(*u + *ww).rem_euclid(3) as usize])); }
    Made { door: Some((0, du)), spot: None, bed: Some((2, room - 1)) }
}
/// Potager clos contre le flanc gauche : une clôture, des rangs de choux, de carottes et de pommes de terre, une rigole.
fn garden_plot(b: &mut B, y0: i32, h: f64) {
    let d = b.d; let (u0, u1) = (-6, -2); let crops = ["carrot_3", "potato_3", "wheat_3"];
    for u in u0..=u1 { for ww in 0..d { let edge = u == u0 || u == u1 || ww == 0 || ww == d - 1;
        b.put(u, y0, ww, id("dirt")); for y in y0 + 1..y0 + 3 { b.put(u, y, ww, 0); }
        if edge { if !(ww == 0 && u == u0 + 2) { b.put(u, y0 + 1, ww, id("fence")); } }
        else if u == u0 + 2 { b.put(u, y0, ww, id("water")); } else { b.put(u, y0, ww, id("farmland")); b.put(u, y0 + 1, ww, id(crops[((u + ww) as usize + (h * 3.0) as usize) % 3])); } } }
}

// ------------------------------------------------------------------------------------------------ maison à colombages
/// La maison à colombages : un rez-de-chaussée de pierre, un ou deux étages de torchis entre les poutres qui avancent
/// en encorbellement, un toit de tuiles (ardoise chez les riches), une cheminée. Variantes : pignon sur rue ; haute
/// et étroite sur trois niveaux ; mur gouttereau sur rue (la porte sur le long côté) ; avec une aile basse ; avec balcon
/// fleuri ; maison de marchand à auvent et étal.
fn timber_house(b: &mut B, sp: &Spec, y0: i32) -> Made {
    let (w, d) = (b.w, b.d); let s = b.s; let floors = sp.floors; let du = w / 2; let side_door = sp.door_side == 1;
    prep(b, sp.extent(), y0, 4 * floors + w / 2 + 5);
    b.walls(y0 + 1, y0 + 3, s.found, s.ground, s.ground);
    for f in 1..floors { let y = y0 + 4 * f; b.walls(y, y, s.post, s.post, s.post); b.walls(y + 1, y + 3, s.upper, s.upper, s.post); for u in 1..w - 1 { for ww in 1..d - 1 { b.put(u, y, ww, s.floor); } } }
    let (dside, dt) = if side_door { (1u8, d / 2) } else { (0u8, du) };
    b.door(dside, dt, y0 + 1);
    windows(b, 0, y0 + 2, &[1, w - 2], &if side_door { vec![] } else { vec![du] }); windows(b, 1, y0 + 2, &[2, d - 3], &if side_door { vec![dt, dt - 1, dt + 1] } else { vec![] }); windows(b, 3, y0 + 2, &[d / 2], &[]);
    for f in 1..floors { let y = y0 + 4 * f + 2;
        let front: Vec<i32> = if w >= 7 { vec![1, du, w - 2] } else { vec![1, w - 2] }; windows(b, 0, y, &front, &[]); if !side_door { for t in &front { if *t > 0 && *t < w - 1 { b.flowers(0, *t, y); } } }
        windows(b, 1, y, &[1, d / 2, d - 2], &[]); windows(b, 3, y, &[1, d - 2], &[]); windows(b, 2, y, &[w / 2], &[]);
        if side_door { for t in [1, d / 2, d - 2] { if t > 0 && t < d - 1 { b.flowers(1, t, y); } } } }
    for side in 0..4u8 { b.trims(side, (y0 + 1) as f64, (y0 + 4) as f64); for f in 1..floors { b.frame(side, (y0 + 4 * f + 1) as f64, (y0 + 4 * f + 4) as f64); } }
    // Encorbellement : la poutre de plancher saille à chaque étage, portée par des consoles.
    for f in 1..floors { let yy = (y0 + 4 * f) as f64; for side in 0..4u8 { let len = b.len(side) as f64; let n = b.normal(side); let at = b.anchor(side, len * 0.5, yy);
        let (a, c) = (b.on(side, -0.1, yy + 0.5, 0.1), b.on(side, len + 0.1, yy + 0.5, 0.1)); b.k.bar(at, a, c, n, 0.5, 0.1, s.beam, -1.0);
        let mut t = 0.5; while t < len { let at = b.anchor(side, t, yy); let p = b.on(side, t, yy - 0.12, 0.08); b.k.solid(at, p, [cross_dir(n).map(|v| v * 0.08), [0.0, 0.12, 0.0], n.map(|v| v * 0.08)], s.beam, -1.0, 0); t += 2.0; } } }
    let yr = y0 + 4 * floors; b.roof(yr, s.tiles, s.upper, 0.16);
    let cu = if side_door { w - 2 } else { 1 }; b.chimney(cu, y0 + 1, yr + w / 2 + 1);
    match sp.var {
        3 => { let c = lean_to(b, 3, 1, (d - 2).min(4), 3, y0, Some(s.ground), s.tiles); for (u, ww) in c.iter().take(4) { b.put(*u, y0 + 1, *ww, id(["barrel", "crate", "sack", "loom"][(*u + *ww).rem_euclid(4) as usize])); } }
        4 => balcony(b, du, y0 + 5),
        5 => awning(b, y0, sp.h),
        _ => if !side_door { porch(b, du, (y0 + 3) as f64 + 0.25, s.tiles); },
    }
    if !side_door { crate::batisse::garden(b.k, b.lot, w, d, du, y0, sp.h); }
    // Le rez-de-chaussée : foyer au fond à gauche, lit, table et chaises, coffre ; l'escalier le long du mur.
    let f = b.lot_face(); let y = y0 + 1; let su = if side_door { 1 } else { w - 2 };
    stair(b, su, 1, y0); if floors == 3 { stair(b, su, 1, y0 + 4); }
    let (fu, bu, tu) = if side_door { (w - 2, 2, 3) } else { (1, 1, 2) };
    b.put(fu, y, d - 2, turned("fireplace", f)); b.put(bu, y, d - 3, turned("bed", f)); b.put(bu, y, d - 4, turned("bed_foot", f));
    let bed = if side_door { (bu + 1, d - 3) } else { (2, d - 3) };
    if w >= 6 && !side_door { b.put(tu, y, d - 2, id("table")); b.put(tu, y + 1, d - 2, id("candle")); }
    if side_door { b.put(tu, y, 1, id("table")); b.put(tu, y + 1, 1, id("bread_basket")); b.put(tu + 1, y, 1, turned("chair", f_of(b.lot, -1, 0))); }
    b.put(if side_door { w - 2 } else { 1 }, y, 1, id("chest")); b.put(fu, y + 2, d - 3, id("lantern"));
    // À l'étage : les chambres.
    for fl in 1..floors { let yy = y0 + 4 * fl + 1; let ou = if side_door { w - 2 } else { 1 };
        b.put(ou, yy, d - 2, turned("bed", f)); b.put(ou, yy, d - 3, turned("bed_foot", f)); b.put(if side_door { w - 3 } else { 2 }, yy, d - 2, id("chest"));
        if w >= 6 { b.put(du, yy, d - 2, turned("cupboard", f)); } b.put(ou, yy + 2, 1, id("lantern")); b.put(if side_door { w - 3 } else { 2 }, yy, 2, id(if sp.h > 0.5 { "carpet_red" } else { "carpet_green" })); }
    Made { door: Some((dside, dt)), spot: None, bed: Some(bed) }
}
/// Balcon de bois en façade : plancher sur consoles, garde-corps à balustres, pots de fleurs.
fn balcony(b: &mut B, du: i32, yf: i32) {
    let lot = b.lot; let beam = b.s.beam; let at = lot.cell(du, yf, 0); let (l, rr, out) = (du as f64 - 1.0, du as f64 + 2.0, 0.85); let fy = yf as f64;
    b.k.cube(at, lot.p(l, fy - 0.1, -out), lot.p(rr, fy + 0.02, 0.0), beam, -1.0);
    let mut t = l + 0.1; while t < rr { b.k.cube(at, lot.p(t - 0.035, fy, -out + 0.02), lot.p(t + 0.035, fy + 0.85, -out + 0.09), beam, -1.0); t += 0.27; }
    b.k.cube(at, lot.p(l, fy + 0.85, -out), lot.p(rr, fy + 0.95, -out + 0.12), beam, -1.0);
    for k in [0.5, 1.5, 2.5] { b.k.bar(at, lot.p(l + k, fy - 0.1, 0.0), lot.p(l + k, fy - 0.1, -out), [0.0, -1.0, 0.0], 0.06, 0.12, beam, -1.0); }
    for t in [l + 0.3, rr - 0.3] { let p = lot.p(t, fy + 0.14, -out * 0.5); b.k.solid(at, p, [lot.v(0.12, 0.0, 0.0), [0.0, 0.1, 0.0], lot.v(0.0, 0.0, 0.12)], [0.62, 0.32, 0.2], -1.0, 0); b.k.solid(at, [p[0], p[1] + 0.22, p[2]], [lot.v(0.16, 0.0, 0.0), [0.0, 0.13, 0.0], lot.v(0.0, 0.0, 0.16)], [0.28, 0.48, 0.2], -1.0, 0); }
    // la porte-fenêtre du balcon
    b.window(0, du, yf + 1);
}
/// Maison de marchand : un auvent rayé sur toute la façade, un étal et des caisses dessous.
fn awning(b: &mut B, y0: i32, h: f64) {
    let lot = b.lot; let w = b.w as f64; let at = lot.cell(0, y0 + 3, 0); let y = y0 as f64; let cols = [[[0.62, 0.14, 0.12], [0.9, 0.86, 0.78]], [[0.18, 0.28, 0.5], [0.9, 0.86, 0.78]], [[0.28, 0.42, 0.22], [0.88, 0.82, 0.64]]][(h * 3.0) as usize % 3];
    let n = (w * 2.0) as i32; for i in 0..n { let u0 = i as f64 * 0.5; b.k.quad(at, [lot.p(u0, y + 3.7, 0.0), lot.p(u0 + 0.5, y + 3.7, 0.0), lot.p(u0 + 0.5, y + 3.15, -1.3), lot.p(u0, y + 3.15, -1.3)], cols[(i % 2) as usize], -1.0, 0);
        b.k.quad(at, [lot.p(u0, y + 3.15, -1.3), lot.p(u0 + 0.5, y + 3.15, -1.3), lot.p(u0 + 0.5, y + 2.95, -1.31), lot.p(u0, y + 2.95, -1.31)], cols[((i + 1) % 2) as usize], -1.0, 0); }
    for t in [0.15, w - 0.15] { b.k.bar(at, lot.p(t, y + 1.0, -1.2), lot.p(t, y + 3.15, -1.2), lot.v(0.0, 0.0, 1.0), 0.05, 0.05, b.s.beam, -1.0); }
    b.put(1, y0 + 1, -1, id("table")); b.put(1, y0 + 2, -1, id("bread_basket")); b.put(b.w - 2, y0 + 1, -1, id("crate")); b.put(b.w - 2, y0 + 2, -1, id("sack"));
}

// ------------------------------------------------------------------------------------------------ forge
/// La forge : une halle ouverte sur la rue, poteaux de chêne et sablière ; murs de moellons gris au fond et sur les
/// côtés ; le foyer et sa grande cheminée de pierre ; l'enclume au milieu, la cuve de trempe, la meule, les outils,
/// le charbon ; un réduit fermé pour le forgeron. Variantes : seule ; avec la maison du forgeron accolée ; avec un
/// appentis à charbon. Le maître forgeron a une halle plus large et deux foyers.
fn forge(b: &mut B, sp: &Spec, y0: i32) -> Made {
    let (w, d) = (b.w, b.d); let s = b.s; let beam = id("oak_beam"); let rub = id("rubble_wall"); let master = w >= 9;
    prep(b, sp.extent(), y0, 4 + w / 2 + 6);
    b.walls(y0 + 1, y0 + 3, s.found, rub, beam);
    let bay = if master { 5 } else { 3 }; // la halle ouverte : de u = 1 à bay - 1, poteau en bay
    for u in 1..bay { for y in y0 + 1..=y0 + 3 { b.put(u, y, 0, 0); } } for y in y0 + 1..=y0 + 3 { b.put(bay, y, 0, beam); }
    for u in 0..w { b.put(u, y0, 0, id("cobblestone")); } for u in 1..bay { for ww in 1..d - 1 { b.put(u, y0, ww, id("cobblestone")); } }
    let dt = w - 2; b.door(0, dt, y0 + 1); windows(b, 1, y0 + 2, &[d / 2], &[]); windows(b, 0, y0 + 2, &[bay + 1], &[dt]);
    for side in 0..4u8 { b.trims(side, (y0 + 1) as f64, (y0 + 4) as f64); }
    let yr = y0 + 4; b.roof(yr, if sp.rich > 0.5 { s.noble } else { s.tiles }, rub, 0.16);
    b.chimney(1, y0 + 1, yr + w / 2 + 2); if master { b.chimney(bay - 1, y0 + 1, yr + w / 2 + 2); }
    b.sign(dt as f64 + 1.9, (y0 + 3) as f64 + 0.6, "enclume");
    // La halle : foyer au fond, tas de charbon, enclume au milieu (le forgeron se tient devant), cuve de trempe, meule.
    let f = b.lot_face(); let y = y0 + 1;
    b.put(1, y, d - 2, id("furnace")); b.put(1, y, d - 3, id("coal_heap")); if master { b.put(bay - 1, y, d - 2, id("furnace")); b.put(bay - 2, y, 2, id("anvil")); }
    b.put(2, y, 2, id("anvil")); b.put(bay - 1, y, d - 3, id("cauldron")); b.put(1, y, 1, turned("grindstone", f)); b.put(2, y, d - 2, id("bucket"));
    b.put(bay - 1, y + 1, d - 2, turned("weapon_rack", f)); b.put(bay, y + 2, 1, id("lantern"));
    // Le réduit du forgeron : lit, coffre, établi.
    b.put(w - 2, y, d - 2, turned("bed", f)); b.put(w - 2, y, d - 3, turned("bed_foot", f)); b.put(bay + 1, y, 1, id("chest")); b.put(bay + 1, y + 2, d - 2, turned("tool_rack", f));
    // Dehors : un tonneau d'eau, le bois, des barres de fer.
    b.put(-2, y, 1, id("barrel")); crate::batisse::woodpile(b.k, b.lot, w, y0);
    match sp.var {
        1 => { // la maison du forgeron, accolée à droite
            let lot2 = Lot { o: (b.lot.o.0 + b.lot.u.0 * w, b.lot.o.1 + b.lot.u.1 * w), ..b.lot }; let sp2 = Spec { arch: Arch::TimberHouse, var: 0, w: 5, d, floors: 2, door_side: 0, ..*sp };
            let mut b2 = B { k: &mut *b.k, lot: lot2, w: 5, d, s: b.s, gaps: Vec::new(), paint: b.paint, h: sp.h };
            for ww in 0..d { for y in y0 + 1..y0 + 4 { b2.put(0, y, ww, 0); } } let _ = timber_house(&mut b2, &sp2, y0);
        }
        2 => { let c = lean_to(b, 3, 1, d - 2, 3, y0, None, s.tiles); for (i, (u, ww)) in c.iter().enumerate() { b.put(*u, y, *ww, id(if i % 3 == 0 { "coal_heap" } else { "log_pile" })); } }
        _ => {}
    }
    Made { door: Some((0, dt)), spot: Some(((2, 1), (2, 2))), bed: Some((w - 3, d - 2)) }
}

// ------------------------------------------------------------------------------------------------ auberge
/// L'auberge : grande maison à colombages de deux ou trois niveaux, balcon courant sur la façade, enseigne à la chope,
/// terrasse couverte sur le côté ; dedans, la salle commune (tables rondes, tabourets, cheminée, lustre), le comptoir
/// et ses tonneaux, la cuisine ; à l'étage, les chambres. Variantes : à terrasse et balcon ; avec une écurie ; haute
/// et étroite sur trois niveaux.
fn inn(b: &mut B, sp: &Spec, y0: i32) -> Made {
    let (w, d) = (b.w, b.d); let s = b.s; let floors = sp.floors; let du = w / 2;
    prep(b, sp.extent(), y0, 4 * floors + w / 2 + 5);
    b.walls(y0 + 1, y0 + 3, s.found, s.ground, s.ground);
    for f in 1..floors { let y = y0 + 4 * f; b.walls(y, y, s.post, s.post, s.post); b.walls(y + 1, y + 3, s.upper, s.upper, s.post); for u in 1..w - 1 { for ww in 1..d - 1 { b.put(u, y, ww, s.floor); } } }
    b.door(0, du, y0 + 1); windows(b, 0, y0 + 2, &[1, 2, w - 3, w - 2], &[du]); windows(b, 1, y0 + 2, &[2, d - 3], &[]); windows(b, 3, y0 + 2, &[2, d - 3], &[]);
    for f in 1..floors { let y = y0 + 4 * f + 2; let front: Vec<i32> = (1..w - 1).step_by(2).collect(); windows(b, 0, y, &front, &[]); for t in &front { b.flowers(0, *t, y); }
        windows(b, 1, y, &[1, d / 2, d - 2], &[]); windows(b, 3, y, &[1, d / 2, d - 2], &[]); windows(b, 2, y, &[2, w - 3], &[]); }
    for side in 0..4u8 { b.trims(side, (y0 + 1) as f64, (y0 + 4) as f64); for f in 1..floors { b.frame(side, (y0 + 4 * f + 1) as f64, (y0 + 4 * f + 4) as f64); } }
    let yr = y0 + 4 * floors; b.roof(yr, s.tiles, s.upper, 0.16); b.chimney(w - 2, y0 + 1, yr + w / 2 + 1); b.chimney(1, y0 + 1, yr + w / 2 + 1);
    b.sign(du as f64 + 1.9, (y0 + 3) as f64 + 0.6, "chope"); for u in [du - 2, du + 2] { b.put(u, y0 + 1, -1, id("barrel")); }
    if sp.var != 1 { porch(b, du, (y0 + 3) as f64 + 0.25, s.tiles); }
    if sp.var == 0 { balcony(b, du, y0 + 5); }
    match sp.var { 1 => stable(b, y0), _ => terrace(b, y0, sp.h) }
    // La salle : l'escalier à gauche, la cheminée, les tables ; le comptoir au fond à droite, les tonneaux derrière.
    let f = b.lot_face(); let y = y0 + 1; let left = f_of(b.lot, -1, 0);
    stair(b, 1, 1, y0); if floors == 3 { stair(b, 1, 1, y0 + 4); }
    b.put(1, y, d - 2, turned("fireplace", left)); b.put(du, y + 2, d / 2, id("chandelier"));
    for u in (w - 4).max(du + 1)..w - 1 { b.put(u, y, d - 3, id("table")); b.put(u, y, d - 2, id("barrel")); } b.put(w - 2, y + 1, d - 3, id("bottles")); b.put(w - 3, y + 1, d - 3, id("cheese"));
    let tables: &[(i32, i32)] = if w >= 9 { &[(3, 3), (5, 2)] } else { &[(3, 3)] };
    for &(u, ww) in tables { if u < w - 1 && ww < d - 3 { b.put(u, y, ww, id("round_table")); b.put(u, y + 1, ww, id("candle")); b.put(u - 1, y, ww, id("stool")); b.put(u, y, ww + 1, id("stool")); } }
    b.put(w - 2, y, 1, turned("bed_foot", f)); b.put(w - 2, y, 2, turned("bed", f)); // le lit de l'aubergiste, derrière la salle
    b.put(2, y, d - 2, id("cauldron"));
    // Les chambres de l'étage.
    for fl in 1..floors { let yy = y0 + 4 * fl + 1; for u in [3, w - 2] { if u < w - 1 { b.put(u, yy, d - 2, turned("bed", f)); b.put(u, yy, d - 3, turned("bed_foot", f)); b.put(u - 1, yy, d - 2, id("chest")); } }
        b.put(w - 2, yy, 1, turned("bed_foot", f)); b.put(w - 2, yy, 2, turned("bed", f)); b.put(du, yy + 2, 2, id("lantern")); }
    Made { door: Some((0, du)), spot: Some(((w - 3, d - 4), (w - 3, d - 3))), bed: Some((w - 3, 2)) }
}
/// La terrasse couverte, sur le flanc droit : un plancher, des tables rondes et des tabourets, un auvent de toile sur
/// quatre perches, des tonneaux, une lanterne.
fn terrace(b: &mut B, y0: i32, h: f64) {
    let (w, d) = (b.w, b.d); let lot = b.lot; let beam = b.s.beam; let (u0, u1, w0, w1) = (w + 1, w + 4, 1, (d - 2).min(6));
    for u in u0..=u1 { for ww in w0..=w1 { for y in y0 - 2..y0 { b.put(u, y, ww, b.s.found); } b.put(u, y0, ww, id("planks")); for y in y0 + 1..y0 + 4 { b.put(u, y, ww, 0); } } }
    for (u, ww) in [(u0 + 1, w0 + 1), (u1 - 1, w1 - 1)] { b.put(u, y0 + 1, ww, id("round_table")); b.put(u, y0 + 2, ww, id(if ww == w0 + 1 { "bottles" } else { "candle" })); b.put(u + 1, y0 + 1, ww, id("stool")); b.put(u, y0 + 1, ww + 1, id("stool")); b.put(u - 1, y0 + 1, ww, id("stool")); }
    b.put(u1, y0 + 1, w0, id("barrel")); b.put(u0, y0 + 1, w1, id("barrel"));
    let at = lot.cell(u0, y0, w0); let y = y0 as f64; let cols = [[[0.78, 0.68, 0.48], [0.62, 0.18, 0.14]], [[0.9, 0.86, 0.76], [0.3, 0.42, 0.62]]][(h * 2.0) as usize % 2];
    for (pu, pw) in [(u0 as f64 + 0.1, w0 as f64 + 0.1), (u1 as f64 + 0.9, w0 as f64 + 0.1), (u0 as f64 + 0.1, w1 as f64 + 0.9), (u1 as f64 + 0.9, w1 as f64 + 0.9)] { b.k.cube(at, lot.p(pu - 0.05, y + 1.0, pw - 0.05), lot.p(pu + 0.05, y + 3.0, pw + 0.05), beam, -1.0); }
    let n = ((w1 - w0 + 1) * 2) as usize; for i in 0..n { let a = w0 as f64 + i as f64 * 0.5; b.k.quad(at, [lot.p(u0 as f64, y + 3.25, a), lot.p(u0 as f64, y + 3.25, a + 0.5), lot.p(u1 as f64 + 1.0, y + 2.85, a + 0.5), lot.p(u1 as f64 + 1.0, y + 2.85, a)], cols[i % 2], -1.0, 0); }
    b.put(u1, y0 + 1, w1, id("fence")); b.put(u1, y0 + 2, w1, id("lantern"));
}
/// L'écurie de l'auberge, à gauche : un appentis ouvert de bardage, du foin, un abreuvoir, une charrette.
fn stable(b: &mut B, y0: i32) {
    let d = b.d; let c = lean_to(b, 3, 1, d - 2, 4, y0, None, b.s.tiles); let boards = id("barn_boards"); let f = b.lot_face();
    for y in y0 + 1..=y0 + 2 { for ww in 1..d - 1 { b.put(-4, y, ww, boards); } }
    for (i, (u, ww)) in c.iter().enumerate() { if *u == -4 { continue; } match i % 5 { 0 => { b.put(*u, y0 + 1, *ww, id("hay_bale")); } 2 => { b.put(*u, y0 + 1, *ww, turned("trough", f + 1)); } _ => {} } }
    b.put(-2, y0 + 1, 1, turned("cart", f)); let _ = c;
}

// ------------------------------------------------------------------------------------------------ grange
/// La grange : un haut volume de bardage sur un solin de moellons, poteaux de chêne aux angles, un grand portail à deux
/// vantaux ouvert sur la cour, un fenil sur la moitié du fond (échelle), du foin partout, la charrette devant.
/// Variantes : bardée ; à pignons de pierre ; avec un appentis ; petite, avec la poutre du monte-foin.
fn barn(b: &mut B, sp: &Spec, y0: i32) -> Made {
    let (w, d) = (b.w, b.d); let s = b.s; let boards = id("barn_boards"); let beam = id("oak_beam"); let rub = id("rubble_wall"); let du = w / 2;
    prep(b, sp.extent(), y0, 5 + w / 2 + 5);
    b.walls(y0 + 1, y0 + 4, rub, boards, beam);
    if sp.var == 1 { for y in y0 + 1..=y0 + 4 { for u in 0..w { b.put(u, y, 0, rub); b.put(u, y, d - 1, rub); } } }
    for u in du - 1..=du + 1 { for y in y0 + 1..=y0 + 3 { b.put(u, y, 0, 0); } } for u in du - 1..=du + 1 { b.put(u, y0 + 4, 0, if b.lot.u.0 != 0 { id("oak_beam_x") } else { id("oak_beam_z") }); }
    windows(b, 1, y0 + 3, &[2, d - 3], &[]); windows(b, 3, y0 + 3, &[d / 2], &[]);
    for u in 1..w - 1 { for ww in 1..d - 1 { b.put(u, y0, ww, id("dirt")); } }
    // Le fenil : un plancher sur la moitié du fond, l'échelle pour y monter.
    let loft = d / 2; for u in 1..w - 1 { for ww in loft..d - 1 { b.put(u, y0 + 4, ww, s.floor); } }
    for y in y0 + 1..=y0 + 4 { b.put(1, y, loft - 1, crate::shapes::variant(id("ladder"), b.lot_face(), false, false, false)); }
    let thatched = sp.h < 0.45; let cover = if thatched { s.thatch } else { id("shingles") };
    let yr = y0 + 5; b.roof(yr, cover, if sp.var == 1 { rub } else { boards }, if thatched { 0.42 } else { 0.16 });
    // Les vantaux grands ouverts, rabattus contre la façade.
    let lot = b.lot; let n = b.normal(0); let at = lot.cell(0, y0 + 1, 0); let y = y0 as f64 + 1.0; let wood = layer(boards);
    for (a0, a1) in [(du as f64 - 2.6, du as f64 - 1.05), (du as f64 + 2.05, du as f64 + 3.6)] { b.k.solid(at, b.on(0, (a0 + a1) * 0.5, y + 1.5, 0.08), [lot.v((a1 - a0) * 0.5, 0.0, 0.0), [0.0, 1.5, 0.0], n.map(|v| v * 0.05)], [1.0, 1.0, 1.0], wood, 1);
        for yy in [y + 0.4, y + 2.6] { b.k.bar(at, b.on(0, a0 + 0.1, yy, 0.14), b.on(0, a1 - 0.1, yy, 0.14), n, 0.07, 0.03, s.beam, -1.0); } }
    // Dedans : des bottes de foin le long des murs (sur deux rangs), au fenil aussi ; les outils, les sacs.
    let f = b.lot_face(); let hb = id("hay_bale");
    for ww in loft..d - 1 { for u in [1, w - 2] { b.put(u, y0 + 1, ww, hb); if (u + ww) % 2 == 0 { b.put(u, y0 + 2, ww, hb); } } }
    for u in 2..w - 2 { for ww in loft..d - 1 { if (u * 3 + ww) % 4 != 0 { b.put(u, y0 + 5, ww, hb); } } }
    b.put(w - 2, y0 + 2, 2, turned("tool_rack", f_of(lot, 1, 0))); b.put(w - 2, y0 + 1, 1, id("sack")); b.put(w - 2, y0 + 1, 2, id("sack")); b.put(1, y0 + 2, 1, id("lantern"));
    // Dehors : la charrette dans la cour, du foin, un abreuvoir.
    b.put(du + 2, y0 + 1, -3, turned("cart", f + 1)); b.put(du - 3, y0 + 1, -2, hb); b.put(du - 3, y0 + 1, -3, hb); b.put(w, y0 + 1, 2, turned("trough", f));
    match sp.var {
        2 => { let c = lean_to(b, 1, 1, d - 2, 3, y0, None, cover); for (i, (u, ww)) in c.iter().enumerate() { b.put(*u, y0 + 1, *ww, if i % 2 == 0 { hb } else { id("log_pile") }); } }
        3 => { let at = lot.cell(du, yr + w / 2, 0); let top = (yr + w / 2) as f64 + 1.2; b.k.bar(at, lot.p(du as f64 + 0.5, top, 0.5), lot.p(du as f64 + 0.5, top, -1.6), [0.0, 1.0, 0.0], 0.13, 0.13, s.beam, -1.0);
            b.k.bar(at, lot.p(du as f64 + 0.5, top - 0.1, -1.45), lot.p(du as f64 + 0.5, top - 1.9, -1.45), [1.0, 0.0, 0.0], 0.015, 0.015, [0.62, 0.55, 0.4], -1.0);
            b.k.solid(at, lot.p(du as f64 + 0.5, top - 2.2, -1.45), [lot.v(0.32, 0.0, 0.0), [0.0, 0.3, 0.0], lot.v(0.0, 0.0, 0.3)], [1.0, 1.0, 1.0], layer(hb), 0); }
        _ => {}
    }
    Made::default()
}

// ------------------------------------------------------------------------------------------------ états
/// Le chantier : les fondations, les murs à mi-hauteur, des échafaudages, les matériaux qui attendent, une échelle.
fn building_site(b: &mut B, sp: &Spec, y0: i32) -> Made {
    let (w, d) = (b.w, b.d); let s = b.s; prep(b, sp.extent(), y0, 9);
    let wall = if sp.arch == Arch::Barn { id("barn_boards") } else { s.ground };
    b.walls(y0 + 1, y0 + 2, s.found, wall, id("oak_beam")); for y in y0 + 3..=y0 + 5 { for (u, ww) in [(0, 0), (w - 1, 0), (0, d - 1), (w - 1, d - 1)] { b.put(u, y, ww, id("oak_beam")); } }
    b.put(w / 2, y0 + 1, 0, 0); b.put(w / 2, y0 + 2, 0, 0);
    // Échafaudage le long de la façade : perches, platelage, échelle.
    for u in (-1..=w).step_by(3) { for y in y0 + 1..=y0 + 4 { b.put(u, y, -1, id("fence")); } } for u in -1..=w { b.put(u, y0 + 3, -1, id("planks_slab")); }
    for y in y0 + 1..=y0 + 3 { b.put(w, y, 0, crate::shapes::variant(id("ladder"), b.lot_face(), false, false, false)); }
    // Les matériaux : pierres, planches, poutres, une charrette.
    for (u, ww, n) in [(-1, 1, "cobblestone"), (-1, 2, "cobblestone"), (w, 1, "planks"), (w, 2, "log_pile"), (w, 3, "log_pile")] { b.put(u, y0 + 1, ww, id(n)); }
    b.put(-1, y0 + 2, 1, id("cobblestone")); b.put(w, y0 + 2, 2, id("log_pile")); b.put(w / 2, y0 + 1, d / 2, id("crate"));
    Made::default()
}
/// L'usure : mousse et lierre sur les vieux murs, pierres de rapiéçage, trous et vitres brisées, herbes dans les ruines.
fn wear(b: &mut B, sp: &Spec, y0: i32) {
    let (w, d) = (b.w, b.d); let mut r = crate::noise::Rng::new((sp.h * 9.1e6) as u32 + 3); let top = y0 + 3;
    let wall_cell = |r: &mut crate::noise::Rng| -> (i32, i32) { match r.int(0, 3) { 0 => (r.int(1, w - 2), 0), 1 => (w - 1, r.int(1, d - 2)), 2 => (r.int(1, w - 2), d - 1), _ => (0, r.int(1, d - 2)) } };
    match sp.cond {
        Cond::Old | Cond::Abandoned => { for _ in 0..(w + d) / 2 { let (u, ww) = wall_cell(&mut r); let (ou, ow) = if u == 0 { (-1, ww) } else if u == w - 1 { (w, ww) } else if ww == 0 { (u, -1) } else { (u, d) }; for y in y0 + 1..=y0 + r.int(1, 3) { b.put(ou, y, ow, id("vine")); } } }
        Cond::Repaired => { for _ in 0..4 { let (u, ww) = wall_cell(&mut r); let y = y0 + 1 + r.int(0, 2); b.put(u, y, ww, id(if r.next() < 0.5 { "fieldstone" } else { "cobblestone" })); } }
        _ => {}
    }
    if matches!(sp.cond, Cond::Damaged | Cond::Abandoned) {
        for _ in 0..3 + (sp.cond == Cond::Abandoned) as i32 * 3 { let (u, ww) = wall_cell(&mut r); let y = y0 + 2 + r.int(0, 1); if y <= top { b.put(u, y, ww, 0); }
            let (ou, ow) = if u == 0 { (-1, ww) } else if u == w - 1 { (w, ww) } else if ww == 0 { (u, -1) } else { (u, d) }; b.put(ou, y0 + 1, ow, id("cobblestone")); }
        for g in b.gaps.clone() { if !g.door && r.next() < 0.6 { let (u, ww) = b.cell_of(g.side, g.t); b.put(u, g.y0, ww, 0); } }
    }
    if sp.cond == Cond::Abandoned { for _ in 0..8 { let (u, ww) = (r.int(1, w - 2), r.int(1, d - 2)); b.put(u, y0 + 1, ww, id("tall_grass")); } }
}
