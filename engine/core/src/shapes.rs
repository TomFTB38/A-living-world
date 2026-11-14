//! Blocs de construction de forme : dalles, escaliers, porte, trappe, échelle, fenêtre, piliers, murs fins.
//! Une seule source pour la géométrie : les boîtes servent au dessin, aux collisions et à l'aperçu de pose,
//! de sorte que ce que l'on voit est exactement ce qui arrête le joueur.
//! Orientation : le nom porte un quart de tour (« _1 » à « _3 »), la forme de base regarde vers +z.
use crate::blocks::{Id, by_name, BLOCKS};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum Kind { /** poutre droite (sens : 0 debout, 1 selon x, 2 selon z) */ Post, /** poutre en diagonale, qui monte vers son sens */ Brace, /** poutre en angle : un coude, bras vers le bas si `top` */ Elbow, Slab, Stairs, Door, Trapdoor, Ladder, Window, Pillar, Wall, VSlab, Shutter, Turn, /** pente à 45° */ Slope, /** arêtier */ Corner, /** noue */ Valley, /** faîtage */ Ridge, /** épi */ Peak, /** pointe de stalactite (haut : pend du plafond) ou de stalagmite ; sens : 0 la base, 2 la pointe */ Drip }
/// Ce qu'un bloc de forme est : sa famille, son sens (0 : +z, 1 : +x, 2 : -z, 3 : -x), moitié haute, ouvert, haut de porte,
/// et la variante de base (l'objet posé), dont on dérive toutes les autres.
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Info { pub kind: Kind, pub facing: u8, pub top: bool, pub open: bool, pub upper: bool, pub base: Id }
pub type Bx = [f64; 6];

fn parse(name: &str) -> Option<Info> {
    let digit = |s: &str| s.parse::<u8>().ok();
    let base = |n: &str| by_name(n).unwrap_or(0);
    // Meubles orientables : « chaise », « chaise@1 » à « chaise@3 ».
    if let Some((b, d)) = name.split_once('@') { return Some(Info { kind: Kind::Turn, facing: d.parse().unwrap_or(0), top: false, open: false, upper: false, base: base(b) }); }
    if by_name(&format!("{name}@1")).is_some() { return Some(Info { kind: Kind::Turn, facing: 0, top: false, open: false, upper: false, base: base(name) }); }
    if let Some(m) = name.strip_suffix("_slab_top") { return Some(Info { kind: Kind::Slab, facing: 0, top: true, open: false, upper: false, base: base(&format!("{m}_slab")) }); }
    if name.ends_with("_slab") { return Some(Info { kind: Kind::Slab, facing: 0, top: false, open: false, upper: false, base: base(name) }); }
    if let Some(i) = name.find("_stairs") { let rest = &name[i + 7..]; let top = rest.starts_with("_top"); let rest = rest.strip_prefix("_top").unwrap_or(rest); let f = rest.strip_prefix('_').and_then(digit).unwrap_or(0); return Some(Info { kind: Kind::Stairs, facing: f, top, open: false, upper: false, base: base(&name[..i + 7]) }); }
    // Couverture : « tuiles_slope[_top][_1…3] », « _corner_k », « _valley_k », « _ridge[_1] », « _peak ».
    for (sfx, kind) in [("_slope", Kind::Slope), ("_corner", Kind::Corner), ("_valley", Kind::Valley), ("_ridge", Kind::Ridge), ("_peak", Kind::Peak)] {
        if let Some(i) = name.find(sfx) { let rest = &name[i + sfx.len()..]; let top = rest.starts_with("_top"); let rest = rest.strip_prefix("_top").unwrap_or(rest);
            let f = rest.strip_prefix('_').and_then(digit).unwrap_or(0); return Some(Info { kind, facing: f, top, open: false, upper: false, base: base(&name[..i + sfx.len()]) }); }
    }
    if let Some(i) = name.find("_vslab") { let f = name[i + 6..].strip_prefix('_').and_then(digit).unwrap_or(0); return Some(Info { kind: Kind::VSlab, facing: f, top: false, open: false, upper: false, base: base(&name[..i + 6]) }); }
    if let Some(r) = name.strip_prefix("shutter") { let open = r.starts_with("_open"); let r = r.strip_prefix("_open").unwrap_or(r); let f = r.strip_prefix('_').and_then(digit).unwrap_or(0); return Some(Info { kind: Kind::Shutter, facing: f, top: false, open, upper: false, base: base("shutter") }); }
    if name == "door" || name.starts_with("door_") {
        let c: Vec<u8> = name.strip_prefix("door_").map_or(vec![0, 0, 0], |d| d.bytes().map(|b| b - b'0').collect());
        return Some(Info { kind: Kind::Door, facing: c[0], open: c[1] == 1, upper: c[2] == 1, top: false, base: base("door") });
    }
    if name == "trapdoor" { return Some(Info { kind: Kind::Trapdoor, facing: 0, top: false, open: false, upper: false, base: base("trapdoor") }); }
    if let Some(r) = name.strip_prefix("trapdoor_open") { let f = r.strip_prefix('_').and_then(digit).unwrap_or(0); return Some(Info { kind: Kind::Trapdoor, facing: f, top: false, open: true, upper: false, base: base("trapdoor") }); }
    if let Some(r) = name.strip_prefix("ladder") { let f = r.strip_prefix('_').and_then(digit).unwrap_or(0); return Some(Info { kind: Kind::Ladder, facing: f, top: false, open: false, upper: false, base: base("ladder") }); }
    if name == "window" || name == "window_x" { return Some(Info { kind: Kind::Window, facing: if name == "window" { 0 } else { 1 }, top: false, open: false, upper: false, base: base("window") }); }
    if let Some(i) = name.find("_point_") { let (d, k) = name[i + 7..].split_once('_')?; return Some(Info { kind: Kind::Drip, facing: digit(k)?.min(2), top: d == "down", open: false, upper: false, base: base(name) }); }
    if let Some(r) = name.strip_prefix("dripstone_") { let (d, k) = r.split_once('_')?; return Some(Info { kind: Kind::Drip, facing: digit(k)?.min(2), top: d == "down", open: false, upper: false, base: base(name) }); }
    // Poutres fines : « chêne_post[_x|_z] », « chêne_post_diag[_k] », « chêne_post_angle[_top][_k] ».
    if let Some(i) = name.find("_post") { if WOODS.contains(&&name[..i]) { let (sp, r) = (&name[..i], &name[i + 5..]);
        if let Some(r) = r.strip_prefix("_diag_flat") { return Some(Info { kind: Kind::Brace, facing: r.strip_prefix('_').and_then(digit).unwrap_or(0) % 2, top: true, open: false, upper: false, base: base(&format!("{sp}_post_diag")) }); }
        if let Some(r) = r.strip_prefix("_diag") { return Some(Info { kind: Kind::Brace, facing: r.strip_prefix('_').and_then(digit).unwrap_or(0), top: false, open: false, upper: false, base: base(&format!("{sp}_post_diag")) }); }
        if let Some(r) = r.strip_prefix("_angle") { let top = r.starts_with("_top"); let r = r.strip_prefix("_top").unwrap_or(r);
            return Some(Info { kind: Kind::Elbow, facing: r.strip_prefix('_').and_then(digit).unwrap_or(0), top, open: false, upper: false, base: base(&format!("{sp}_post_angle")) }); }
        return Some(Info { kind: Kind::Post, facing: match r { "_x" => 1, "_z" => 2, _ => 0 }, top: false, open: false, upper: false, base: base(&format!("{sp}_post")) });
    } }
    if name.ends_with("_pillar") { return Some(Info { kind: Kind::Pillar, facing: 0, top: false, open: false, upper: false, base: base(name) }); }
    if name == "stone_wall" || name == "stone_wall_x" { return Some(Info { kind: Kind::Wall, facing: if name == "stone_wall" { 0 } else { 1 }, top: false, open: false, upper: false, base: base("stone_wall") }); }
    None
}
fn table() -> &'static [Option<Info>; BLOCKS.len()] {
    static T: OnceLock<[Option<Info>; BLOCKS.len()]> = OnceLock::new();
    T.get_or_init(|| { let mut t = [None; BLOCKS.len()]; for b in BLOCKS.iter() { t[b.id as usize] = parse(b.name); } t })
}
pub fn info(id: Id) -> Option<Info> { table()[id as usize] }

/// Un quart de tour vers la droite, vu d'en haut : +z devient +x.
pub fn turn(b: Bx, f: u8) -> Bx { let mut b = b; for _ in 0..f % 4 { b = [b[2], b[1], 1.0 - b[3], b[5], b[4], 1.0 - b[0]]; } b }
pub const PANEL: f64 = 0.1875;
/// Marches des pentes de toit (collisions et visée approchée).
const RS: usize = 8;
/// Une forme renversée : la même, retournée de haut en bas dans sa case.
fn flip(top: bool, v: Vec<Bx>) -> Vec<Bx> { if top { v.into_iter().map(|b| [b[0], 1.0 - b[4], b[2], b[3], 1.0 - b[1], b[5]]).collect() } else { v } }
/// Les essences dont on taille des poutres fines.
pub const WOODS: [&str; 5] = ["oak", "birch", "beech", "spruce", "acacia"];
/// Demi-section d'une poutre fine : 0,4 bloc de côté, centrée dans sa case.
pub const POST: f64 = 0.2;
/// La diagonale déborde de chaque bout de sa case pour venir mordre dans la poutre voisine (montant ou traverse).
pub const BRACE_EXT: f64 = 0.42;
/// La géométrie, dans le repère du bloc [0, 1]³. Ce sont aussi les boîtes de collision (l'échelle n'en a pas : on y grimpe).
pub fn geometry(i: Info) -> Vec<Bx> {
    let f = i.facing; let r = |v: &[Bx]| v.iter().map(|b| turn(*b, f)).collect::<Vec<_>>();
    match i.kind {
        Kind::Slab => vec![if i.top { [0.0, 0.5, 0.0, 1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0, 0.5, 1.0] }],
        // Marche basse sur toute la surface, marche haute du côté où l'on monte (+z dans le sens de base).
        Kind::Stairs => if i.top { r(&[[0.0, 0.5, 0.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.5, 1.0, 0.5, 1.0]]) } else { r(&[[0.0, 0.0, 0.0, 1.0, 0.5, 1.0], [0.0, 0.5, 0.5, 1.0, 1.0, 1.0]]) },
        // Dalle verticale : la moitié du bloc du côté opposé à soi. Volet : un battant contre le mur du fond, qui s'ouvre contre le côté.
        Kind::VSlab => r(&[[0.0, 0.0, 0.5, 1.0, 1.0, 1.0]]),
        Kind::Shutter => r(&[if i.open { [1.0 - PANEL, 0.0, 0.0, 1.0, 1.0, 1.0] } else { [0.0, 0.0, 1.0 - PANEL, 1.0, 1.0, 1.0] }]),
        // Porte fermée : un battant contre le bord proche ; ouverte, il pivote contre le côté.
        Kind::Door => r(&[if i.open { [0.0, 0.0, 0.0, PANEL, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0, 1.0, PANEL] }]),
        Kind::Trapdoor => if i.open { r(&[[0.0, 0.0, 1.0 - PANEL, 1.0, 1.0, 1.0]]) } else { vec![[0.0, 0.0, 0.0, 1.0, PANEL, 1.0]] },
        Kind::Ladder => Vec::new(),
        Kind::Window => r(&[[0.0, 0.0, 0.4375, 1.0, 1.0, 0.5625]]),
        Kind::Pillar => vec![[0.25, 0.0, 0.25, 0.75, 1.0, 0.75]],
        Kind::Post => { let (a, b) = (0.5 - POST, 0.5 + POST); vec![match f { 1 => [0.0, a, a, 1.0, b, b], 2 => [a, a, 0.0, b, b, 1.0], _ => [a, 0.0, a, b, 1.0, b] }] }
        // La diagonale, pour les collisions : quatre marches qui suivent la pente (on ne passe pas au travers, on n'y bute pas plus).
        // À plat (`top`) : la diagonale court dans le plan du sol, de -x -z vers +x +z (sens 0) ou de -x +z vers +x -z (sens 1).
        Kind::Brace if i.top => { let (a, b, t) = (0.5 - POST, 0.5 + POST, POST * std::f64::consts::SQRT_2);
            (0..4).map(|k| { let (x0, x1) = (k as f64 * 0.25, (k + 1) as f64 * 0.25); let zc = (x0 + x1) * 0.5; let (z0, z1) = ((zc - t).max(0.0), (zc + t).min(1.0)); if f % 2 == 0 { [x0, a, z0, x1, b, z1] } else { [x0, a, 1.0 - z1, x1, b, 1.0 - z0] } }).collect() }
        Kind::Brace => { let (a, b, t) = (0.5 - POST, 0.5 + POST, POST * std::f64::consts::SQRT_2);
            r(&(0..4).map(|k| { let (z0, z1) = (k as f64 * 0.25, (k + 1) as f64 * 0.25); let yc = (z0 + z1) * 0.5; [a, (yc - t).max(0.0), z0, b, (yc + t).min(1.0), z1] }).collect::<Vec<_>>()) }
        Kind::Elbow => { let (a, b) = (0.5 - POST, 0.5 + POST); r(&[if i.top { [a, 0.0, a, b, b, b] } else { [a, a, a, b, 1.0, b] }, [a, a, b, b, b, 1.0]]) }
        // Les pointes s'effilent de leur attache vers le vide : des tranches de plus en plus fines.
        Kind::Drip => { let steps: &[(f64, f64, f64)] = match i.facing { 0 => &[(0.0, 0.35, 0.36), (0.35, 1.0, 0.42)], 1 => &[(0.0, 0.5, 0.2), (0.5, 1.0, 0.28)], _ => &[(0.0, 0.28, 0.06), (0.28, 0.62, 0.12), (0.62, 1.0, 0.18)] };
            steps.iter().map(|&(a, b, w)| { let (y0, y1) = if i.top { (a, b) } else { (1.0 - b, 1.0 - a) }; [0.5 - w, y0, 0.5 - w, 0.5 + w, y1, 0.5 + w] }).collect() }
        Kind::Wall => r(&[[0.0, 0.0, 0.3125, 1.0, 1.0, 0.6875]]),
        Kind::Turn => r(&turn_boxes(BLOCKS[i.base as usize].name)),
        // Les couvertures se touchent en petites marches d'un huitième : on y marche comme sur une pente, et elles collent à la
        // surface dessinée (une marche ne dépasse jamais de plus d'un huitième de bloc). Renversées, elles pendent du haut.
        Kind::Slope => { let q = |k: usize| k as f64 / RS as f64; if i.top { r(&(0..RS).map(|k| [0.0, q(k), 1.0 - q(k + 1), 1.0, q(k + 1), 1.0]).collect::<Vec<_>>()) } else { r(&(0..RS).map(|k| [0.0, q(k), q(k), 1.0, q(k + 1), 1.0]).collect::<Vec<_>>()) } }
        Kind::Corner => flip(i.top, r(&(0..RS).map(|k| { let q = k as f64 / RS as f64; [q, q, q, 1.0, q + 1.0 / RS as f64, 1.0] }).collect::<Vec<_>>())),
        Kind::Valley => flip(i.top, r(&(0..RS).flat_map(|k| { let (q, s) = (k as f64 / RS as f64, 1.0 / RS as f64); [[q, q, 0.0, 1.0, q + s, 1.0], [0.0, q, q, 1.0, q + s, 1.0]] }).collect::<Vec<_>>())),
        // Faîtage (un Λ de demi-hauteur le long de x) et épi (une pyramide) : quatre marches qui suivent leurs pans.
        Kind::Ridge => flip(i.top, r(&(0..4).map(|k| { let q = k as f64 * 0.125; [0.0, q, q, 1.0, q + 0.125, 1.0 - q] }).collect::<Vec<_>>())),
        Kind::Peak => flip(i.top, (0..4).map(|k| { let q = k as f64 * 0.125; [q, q, q, 1.0 - q, q + 0.125, 1.0 - q] }).collect()),
    }
}
/// Le volume d'un meuble orientable, pour le sens 0 (dos vers +z).
fn turn_boxes(n: &str) -> Vec<Bx> {
    match n {
        "chair" => vec![[0.14, 0.0, 0.14, 0.86, 0.52, 0.86], [0.18, 0.52, 0.76, 0.82, 1.0, 0.82]], "bench" => vec![[0.02, 0.0, 0.22, 0.98, 0.5, 0.78]],
        "armchair" | "throne" => vec![[0.1, 0.0, 0.1, 0.9, 0.56, 0.9], [0.1, 0.56, 0.8, 0.9, 1.0, 0.92]], "cupboard" => vec![[0.04, 0.0, 0.16, 0.96, 1.0, 0.98]],
        "spinning_wheel" => vec![[0.1, 0.0, 0.25, 0.9, 0.6, 0.75]], "grindstone" => vec![[0.15, 0.0, 0.2, 0.85, 0.9, 0.8]], "signpost" => vec![[0.42, 0.0, 0.42, 0.58, 1.0, 0.58]],
        "fireplace" | "loom" => vec![[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]], "bed" | "bed_foot" => vec![[0.0, 0.0, 0.0, 1.0, 0.56, 1.0]], "torch_wall" => Vec::new(),
        _ => vec![[0.0, 0.0, 0.84, 1.0, 1.0, 1.0]],
    }
}
/// Une moitié de lit (tête ou pied, dans n'importe quel sens).
pub fn is_bed(id: Id) -> bool { let n = BLOCKS[id as usize].name; n == "bed" || n.starts_with("bed@") || n.starts_with("bed_foot") }
/// L'autre moitié d'un lit : le pied est devant la tête, du côté opposé au mur.
pub fn bed_partner(p: [i32; 3], id: Id) -> Option<[i32; 3]> { if !is_bed(id) { return None; } let i = info(id)?; let (dx, dz) = [(0, 1), (1, 0), (0, -1), (-1, 0)][i.facing as usize % 4]; let k = if BLOCKS[id as usize].name.starts_with("bed_foot") { 1 } else { -1 }; Some([p[0] + dx * k, p[1], p[2] + dz * k]) }
/// Boîtes de collision d'un bloc : `None` pour un bloc ordinaire (cube plein ou rien, selon `is_solid`).
pub fn boxes(id: Id) -> Option<Vec<Bx>> { info(id).map(geometry) }
/// Les montants de l'échelle, contre le mur (+z dans le sens de base).
pub fn ladder_box(f: u8) -> Bx { turn([0.0, 0.0, 1.0 - 0.125, 1.0, 1.0, 1.0], f) }
pub fn is_ladder(id: Id) -> bool { info(id).is_some_and(|i| i.kind == Kind::Ladder) }

/// La variante d'une famille pour un sens, une moitié, un état.
pub fn variant(base: Id, facing: u8, top: bool, open: bool, upper: bool) -> Id {
    let n = BLOCKS[base as usize].name; let f = facing % 4; let sfx = |f: u8| if f == 0 { String::new() } else { format!("_{f}") };
    let name = match info(base).map(|i| i.kind) {
        Some(Kind::Slab) => if top { format!("{n}_top") } else { n.to_string() },
        Some(Kind::Stairs) => format!("{n}{}{}", if top { "_top" } else { "" }, sfx(f)),
        Some(Kind::VSlab) => format!("{n}{}", sfx(f)),
        Some(Kind::Shutter) => format!("shutter{}{}", if open { "_open" } else { "" }, sfx(f)),
        Some(Kind::Door) => if (f, open, upper) == (0, false, false) { "door".into() } else { format!("door_{f}{}{}", open as u8, upper as u8) },
        Some(Kind::Trapdoor) => if open { format!("trapdoor_open{}", sfx(f)) } else { "trapdoor".into() },
        Some(Kind::Ladder) => format!("ladder{}", sfx(f)),
        Some(Kind::Window) => if f % 2 == 0 { "window".into() } else { "window_x".into() },
        Some(Kind::Drip) => match n.find("_point") { Some(i) => format!("{}_point_{}_{}", &n[..i], if top { "down" } else { "up" }, f.min(2)), None => format!("dripstone_{}_{}", if top { "down" } else { "up" }, f.min(2)) },
        Some(Kind::Post) => format!("{n}{}", ["", "_x", "_z"][f.min(2) as usize]),
        Some(Kind::Brace) => if top { format!("{n}_flat{}", if f % 2 == 0 { "" } else { "_1" }) } else { format!("{n}{}", sfx(f)) },
        Some(Kind::Elbow) => format!("{n}{}{}", if top { "_top" } else { "" }, sfx(f)),
        Some(Kind::Wall) => if f % 2 == 0 { "stone_wall".into() } else { "stone_wall_x".into() },
        Some(Kind::Slope) => format!("{n}{}{}", if top { "_top" } else { "" }, sfx(f)),
        Some(Kind::Corner) | Some(Kind::Valley) => format!("{n}{}{}", if top { "_top" } else { "" }, sfx(f)),
        Some(Kind::Ridge) => format!("{n}{}{}", if top { "_top" } else { "" }, if f % 2 == 0 { "" } else { "_1" }),
        Some(Kind::Peak) => format!("{n}{}", if top { "_top" } else { "" }),
        Some(Kind::Turn) => if f == 0 { n.to_string() } else { format!("{n}@{f}") },
        _ => n.to_string(),
    };
    by_name(&name).unwrap_or(base)
}
/// Le sens du regard, au quart de tour près (0 : +z, 1 : +x, 2 : -z, 3 : -x).
pub fn facing_of(dx: f64, dz: f64) -> u8 { if dx.abs() > dz.abs() { if dx > 0.0 { 1 } else { 3 } } else if dz > 0.0 { 0 } else { 2 } }
/// La matière d'un bloc de forme : le bloc plein dont il prend la texture.
pub fn material(id: Id) -> Id {
    let n = BLOCKS[id as usize].name;
    if let Some(i) = info(id).filter(|i| matches!(i.kind, Kind::Post | Kind::Brace | Kind::Elbow)) { let _ = i; return by_name(&format!("{}_log", &n[..n.find("_post").unwrap_or(0)])).unwrap_or(0); }
    for sfx in ["_stairs", "_vslab", "_slab", "_slope", "_corner", "_valley", "_ridge", "_peak"] { if let Some(i) = n.find(sfx) { if let Some(b) = by_name(&n[..i]) { if BLOCKS[b as usize].faces[0] != 0 || b == crate::blocks::B::PLANKS { return b; } } } }
    let m = if let Some(i) = n.find("_point_") { &n[..i] } else if n.starts_with("dripstone") { "flowstone" } else if n.starts_with("stone_bricks") || n == "stone_pillar" || n.starts_with("stone_wall") { "stone_bricks" } else if n.starts_with("cobblestone") { "cobblestone" } else if n.starts_with("thatch") { "thatch" } else { "planks" };
    by_name(m).unwrap_or(0)
}
/// Deux dalles de même matière l'une sur l'autre font un bloc plein.
pub fn full_of_slab(base: Id) -> Option<Id> { BLOCKS[base as usize].name.strip_suffix("_slab").and_then(by_name) }

/// Les faces d'une couverture (pente, arêtier, noue, faîtage, épi), dans le repère du bloc : des triangles et des
/// quadrilatères (un triangle répète son dernier sommet). La pente monte vers le sens du bloc (+z dans le sens de base) ;
/// l'arêtier monte vers le coin +x +z, la noue n'y descend que de ce coin ; le faîtage court le long de x.
pub fn roof_faces(i: Info) -> (Vec<[[f64; 3]; 4]>, [f64; 3]) {
    let t = |p: [f64; 3]| -> [f64; 3] { let mut p = p; for _ in 0..i.facing % 4 { p = [p[2], p[1], 1.0 - p[0]]; } p };
    let tri = |a: [f64; 3], b: [f64; 3], c: [f64; 3]| [a, b, c, c];
    let mut f: Vec<[[f64; 3]; 4]> = match i.kind {
        Kind::Slope if !i.top => vec![[[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]], [[0., 0., 1.], [1., 0., 1.], [1., 1., 1.], [0., 1., 1.]], [[0., 0., 0.], [1., 0., 0.], [1., 1., 1.], [0., 1., 1.]],
            tri([0., 0., 0.], [0., 0., 1.], [0., 1., 1.]), tri([1., 0., 0.], [1., 0., 1.], [1., 1., 1.])],
        Kind::Slope => vec![[[0., 1., 0.], [1., 1., 0.], [1., 1., 1.], [0., 1., 1.]], [[0., 0., 1.], [1., 0., 1.], [1., 1., 1.], [0., 1., 1.]], [[0., 1., 0.], [1., 1., 0.], [1., 0., 1.], [0., 0., 1.]],
            tri([0., 1., 0.], [0., 1., 1.], [0., 0., 1.]), tri([1., 1., 0.], [1., 1., 1.], [1., 0., 1.])],
        Kind::Corner => vec![[[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]], tri([0., 0., 0.], [1., 1., 1.], [1., 0., 0.]), tri([0., 0., 0.], [0., 0., 1.], [1., 1., 1.]),
            tri([1., 0., 0.], [1., 0., 1.], [1., 1., 1.]), tri([0., 0., 1.], [1., 0., 1.], [1., 1., 1.])],
        Kind::Valley => vec![[[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]], [[1., 0., 0.], [1., 0., 1.], [1., 1., 1.], [1., 1., 0.]], [[0., 0., 1.], [1., 0., 1.], [1., 1., 1.], [0., 1., 1.]],
            tri([0., 0., 0.], [1., 1., 0.], [1., 1., 1.]), tri([0., 0., 0.], [1., 1., 1.], [0., 1., 1.]), tri([0., 0., 0.], [1., 0., 0.], [1., 1., 0.]), tri([0., 0., 0.], [0., 0., 1.], [0., 1., 1.])],
        Kind::Ridge => vec![[[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]], [[0., 0., 0.], [1., 0., 0.], [1., 0.5, 0.5], [0., 0.5, 0.5]], [[0., 0., 1.], [1., 0., 1.], [1., 0.5, 0.5], [0., 0.5, 0.5]],
            tri([0., 0., 0.], [0., 0., 1.], [0., 0.5, 0.5]), tri([1., 0., 0.], [1., 0., 1.], [1., 0.5, 0.5])],
        Kind::Peak => { let a = [0.5, 0.5, 0.5]; vec![[[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]], tri([0., 0., 0.], [1., 0., 0.], a), tri([1., 0., 0.], [1., 0., 1.], a), tri([1., 0., 1.], [0., 0., 1.], a), tri([0., 0., 1.], [0., 0., 0.], a)] }
        _ => Vec::new(),
    };
    for q in f.iter_mut() { for p in q.iter_mut() { *p = t(*p); } }
    // arêtier, noue, faîtage, épi renversés : retournés de haut en bas (la pente a ses propres faces renversées)
    let mirror = i.top && i.kind != Kind::Slope; if mirror { for q in f.iter_mut() { for p in q.iter_mut() { p[1] = 1.0 - p[1]; } } }
    // un point dans le plein de la forme : il dit de quel côté chaque face regarde vers dehors
    let inner = match i.kind { Kind::Slope if i.top => [0.5, 0.8, 0.8], Kind::Slope => [0.5, 0.2, 0.8], Kind::Corner => [0.8, 0.15, 0.8], Kind::Valley => [0.6, 0.3, 0.6], _ => [0.5, 0.12, 0.5] };
    let mut c = t(inner); if mirror { c[1] = 1.0 - c[1]; } (f, c)
}
/// Un bloc de couverture de toit ?
pub fn is_roof(id: Id) -> bool { info(id).is_some_and(|i| matches!(i.kind, Kind::Slope | Kind::Corner | Kind::Valley | Kind::Ridge | Kind::Peak)) }

// ------------------------------------------------------------------ ce qui arrête et ce qui se vise
// Un seul principe : la boîte d'un bloc est le bord de son dessin. Les formes de construction ont leur géométrie (ci-dessus) ;
// les blocs de décor prennent la boîte englobante de leur modèle ; la clôture et la branche suivent leurs bras vers les voisins.
use crate::player::Voxels;
use crate::blocks::{is_solid, is_log, is_branch, is_leaf, Shape, B};

/// Une clôture se lie à une autre clôture, à un bloc plein, à un rondin, à une poutre, à un pilier, à un mur.
pub fn fence_joins(b: Id) -> bool { b == B::FENCE || BLOCKS[b as usize].opaque || is_log(b) || info(b).is_some_and(|i| matches!(i.kind, Kind::Post | Kind::Elbow | Kind::Pillar | Kind::Wall)) }

/// La boîte englobante du dessin d'un bloc de décor sans forme, dans sa case (plusieurs tirages : certains modèles varient
/// d'une case à l'autre). Un dessin trop mince pour être visé garde au moins un huitième de bloc d'épaisseur.
pub fn model_box(b: Id) -> Option<Bx> {
    static T: OnceLock<Vec<Option<Bx>>> = OnceLock::new();
    T.get_or_init(|| { let m = crate::mesher::Mesher::new(); BLOCKS.iter().map(|k| {
        if k.shape != Shape::Deco || info(k.id).is_some() { return None; }
        let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
        for s in 0..6 { let mut o = crate::mesher::VBuf::new(256); m.deco_model(&mut o, k.id, 5, 5, 5, 37 * s + 3, 11 + 53 * s, 255.0, 0.0, 0);
            for v in o.data.chunks(crate::mesher::VSIZE) { for a in 0..3 { let p = i16::from_le_bytes([v[2 * a], v[2 * a + 1]]) as f64 / o.scale - 5.0; lo[a] = lo[a].min(p); hi[a] = hi[a].max(p); } } }
        if lo[0] > hi[0] { return None; }
        let mut b = [lo[0], lo[1], lo[2], hi[0], hi[1], hi[2]].map(|v| v.clamp(0.0, 1.0));
        for a in 0..3 { if b[a + 3] - b[a] < 0.125 { let c = ((b[a] + b[a + 3]) * 0.5).clamp(0.0625, 0.9375); b[a] = c - 0.0625; b[a + 3] = c + 0.0625; } }
        Some(b) }).collect() })[b as usize]
}
/// Les boîtes propres d'un bloc, sans regarder ses voisins. `solid` : ce qui arrête le corps (rien pour un bloc qu'on traverse).
/// `None` : la case entière.
pub fn own_boxes(b: Id, solid: bool) -> Option<Vec<Bx>> {
    if !solid { if let Some(i) = info(b).filter(|i| i.kind == Kind::Ladder) { return Some(vec![ladder_box(i.facing)]); } } // on la traverse, mais on la vise
    if let Some(v) = boxes(b) { return Some(v); }
    if solid && !is_solid(b) { return Some(Vec::new()); }
    if BLOCKS[b as usize].shape != Shape::Deco { return None; }
    if BLOCKS[b as usize].model == Some("fence") { return Some(vec![[0.39, 0.0, 0.39, 0.61, 1.0, 0.61]]); }
    model_box(b).map(|bb| vec![bb])
}
/// Les boîtes d'un bloc posé dans le monde : la clôture a ses lisses vers les voisins qui la tiennent, la branche ses bras.
pub fn cell_boxes(w: &impl Voxels, x: i32, y: i32, z: i32, b: Id, solid: bool) -> Option<Vec<Bx>> {
    let model = BLOCKS[b as usize].model; let nb = |dx: i32, dy: i32, dz: i32| w.get(x + dx, y + dy, z + dz).unwrap_or(0);
    if solid && !is_solid(b) { return own_boxes(b, solid); }
    match model {
        Some("fence") => {
            let mut v = vec![[0.39, 0.0, 0.39, 0.61, 1.0, 0.61]];
            for (dx, dz, bb) in [(1, 0, [0.61, 0.34, 0.43, 1.0, 0.85, 0.57]), (-1, 0, [0.0, 0.34, 0.43, 0.39, 0.85, 0.57]), (0, 1, [0.43, 0.34, 0.61, 0.57, 0.85, 1.0]), (0, -1, [0.43, 0.34, 0.0, 0.57, 0.85, 0.39])] {
                if fence_joins(nb(dx, 0, dz)) { v.push(bb); } }
            Some(v)
        }
        Some("branch") => {
            let (a, c) = (0.34, 0.66); let joins = |b2: Id| is_log(b2) || is_branch(b2) || is_leaf(b2); let mut v = vec![[a, a, a, c, c, c]]; let n0 = v.len();
            for (d, bb) in [((1, 0, 0), [c, a, a, 1.0, c, c]), ((-1, 0, 0), [0.0, a, a, a, c, c]), ((0, 0, 1), [a, a, c, c, c, 1.0]), ((0, 0, -1), [a, a, 0.0, c, c, a]), ((0, 1, 0), [a, c, a, c, 1.0, c])] {
                if joins(nb(d.0, d.1, d.2)) { v.push(bb); } }
            let below = nb(0, -1, 0); if joins(below) || BLOCKS[below as usize].opaque { v.push([a, 0.0, a, c, a, c]); }
            if v.len() == n0 { v = vec![[a, 0.0, a, c, 1.0, c]]; }
            Some(v)
        }
        _ => own_boxes(b, solid),
    }
}
/// Rayon contre boîte (méthode des tranches) : le paramètre d'entrée (0 si l'origine est dedans).
pub fn ray_box(o: [f64; 3], d: [f64; 3], b: Bx) -> Option<f64> {
    let (mut t0, mut t1) = (0.0f64, f64::MAX);
    for a in 0..3 {
        if d[a].abs() < 1e-12 { if o[a] < b[a] || o[a] > b[a + 3] { return None; } continue; }
        let (mut u, mut v) = ((b[a] - o[a]) / d[a], (b[a + 3] - o[a]) / d[a]); if u > v { std::mem::swap(&mut u, &mut v); }
        t0 = t0.max(u); t1 = t1.min(v); if t0 > t1 { return None; }
    }
    Some(t0)
}
fn ray_tri(o: [f64; 3], d: [f64; 3], a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Option<f64> {
    let sub = |p: [f64; 3], q: [f64; 3]| [p[0] - q[0], p[1] - q[1], p[2] - q[2]]; let cross = |p: [f64; 3], q: [f64; 3]| [p[1] * q[2] - p[2] * q[1], p[2] * q[0] - p[0] * q[2], p[0] * q[1] - p[1] * q[0]];
    let dot = |p: [f64; 3], q: [f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    let (e1, e2) = (sub(b, a), sub(c, a)); let h = cross(d, e2); let det = dot(e1, h); if det.abs() < 1e-12 { return None; }
    let f = 1.0 / det; let s = sub(o, a); let u = f * dot(s, h); if !(-1e-9..=1.0 + 1e-9).contains(&u) { return None; }
    let q = cross(s, e1); let v = f * dot(d, q); if v < -1e-9 || u + v > 1.0 + 1e-9 { return None; }
    let t = f * dot(e2, q); (t >= 0.0).then_some(t)
}
/// La boîte qui englobe une liste de boîtes (le contour qu'on dessine autour du bloc visé).
pub fn union(v: &[Bx]) -> Bx { v.iter().fold([1.0, 1.0, 1.0, 0.0, 0.0, 0.0], |u, b| [u[0].min(b[0]), u[1].min(b[1]), u[2].min(b[2]), u[3].max(b[3]), u[4].max(b[4]), u[5].max(b[5])]) }
/// Le rayon (origine relative à la case) touche-t-il ce bloc là où il est dessiné ? Le paramètre et le contour du bloc.
/// Les toits se visent sur leurs vraies faces inclinées ; le reste sur ses boîtes.
pub fn ray_hit(w: &impl Voxels, x: i32, y: i32, z: i32, b: Id, o: [f64; 3], d: [f64; 3]) -> Option<(f64, Bx)> {
    if let Some(i) = info(b).filter(|_| is_roof(b)) {
        let (faces, _) = roof_faces(i); let mut best: Option<f64> = None;
        for q in faces { for (p, r) in [(q[1], q[2]), (q[2], q[3])] { if let Some(t) = ray_tri(o, d, q[0], p, r) { best = Some(best.map_or(t, |u| u.min(t))); } } }
        return best.map(|t| (t, union(&geometry(i))));
    }
    match cell_boxes(w, x, y, z, b, false) {
        None => ray_box(o, d, [0.0, 0.0, 0.0, 1.0, 1.0, 1.0]).map(|t| (t, [0.0, 0.0, 0.0, 1.0, 1.0, 1.0])),
        Some(v) if v.is_empty() => None,
        Some(v) => { let t = v.iter().filter_map(|bb| ray_box(o, d, *bb)).fold(f64::MAX, f64::min); (t < f64::MAX).then(|| (t, union(&v))) }
    }
}

// ------------------------------------------------------------------ toits qui se lient
/// Les sens (dx, dz) des quatre directions : 0 +z, 1 +x, 2 -z, 3 -x.
pub const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
/// Une pente de toit prend la forme qu'appellent ses voisines, comme des escaliers qui tournent : si la pente d'au-dessus
/// (côté où elle monte) est une pente perpendiculaire de même matière, c'est un arêtier (angle sortant) ; si celle d'en dessous
/// l'est, c'est une noue (angle rentrant). `nb(dx, dz)` : le bloc voisin à la même hauteur. Les autres blocs ne changent pas.
pub fn roof_join(b: Id, nb: impl Fn(i32, i32) -> Id) -> Id {
    let Some(i) = info(b).filter(|i| i.kind == Kind::Slope) else { return b };
    let f = i.facing % 4; let slope_g = |d: u8| { let (dx, dz) = DIRS[d as usize % 4]; info(nb(dx, dz)).filter(|j| j.kind == Kind::Slope && j.base == i.base && j.top == i.top).map(|j| j.facing % 4) };
    let perp = |g: u8| (g + 1) % 4 == f || (f + 1) % 4 == g; let cover = |g: u8| if (f + 1) % 4 == g { f } else { g };
    let n = BLOCKS[i.base as usize].name; let m = n.strip_suffix("_slope").unwrap_or(n);
    let shape = |k: &str, g: u8| by_name(&format!("{m}_{k}")).map(|base| variant(base, cover(g), i.top, false, false)).unwrap_or(b);
    if let Some(g) = slope_g(f).filter(|g| perp(*g)) { return shape("corner", g); }
    if let Some(g) = slope_g((f + 2) % 4).filter(|g| perp(*g)) { return shape("valley", g); }
    b
}
/// Lie les toits d'une liste de blocs posés (un village, un bourg) : la dernière écriture de chaque case compte.
pub fn join_roofs(blocks: &mut [(i32, i32, i32, Id)]) {
    let last: std::collections::HashMap<(i32, i32, i32), Id> = blocks.iter().map(|b| ((b.0, b.1, b.2), b.3)).collect();
    for b in blocks.iter_mut() { if last.get(&(b.0, b.1, b.2)) != Some(&b.3) || !is_roof(b.3) { continue; }
        let (x, y, z) = (b.0, b.1, b.2); b.3 = roof_join(b.3, |dx, dz| last.get(&(x + dx, y, z + dz)).copied().unwrap_or(0)); }
}

/// Un arêtier ou une noue que ses voisines ne justifient plus (le pan perpendiculaire a été retiré) redevient une pente.
/// `nb` : les voisins d'aujourd'hui ; `was` : les mêmes avec le bloc qu'on vient de retirer. On ne touche qu'aux angles
/// que ce bloc justifiait : un angle posé à la main, sans raison d'être pour la règle, reste comme on l'a posé.
pub fn roof_unjoin(b: Id, nb: impl Fn(i32, i32) -> Id, was: impl Fn(i32, i32) -> Id) -> Id {
    let Some(i) = info(b).filter(|i| matches!(i.kind, Kind::Corner | Kind::Valley)) else { return b };
    let n = BLOCKS[i.base as usize].name; let m = n.strip_suffix("_corner").or_else(|| n.strip_suffix("_valley")).unwrap_or(n);
    let Some(slope) = by_name(&format!("{m}_slope")) else { return b }; let cands = [i.facing % 4, (i.facing + 1) % 4].map(|g| variant(slope, g, i.top, false, false));
    if !cands.iter().any(|c| roof_join(*c, &was) == b) || cands.iter().any(|c| roof_join(*c, &nb) == b) { return b; }
    // la pente qui reste : celle du pan encore là (une voisine de même sens), ou qui se lie autrement à ce qui l'entoure
    let same = |c: Id| DIRS.iter().any(|(dx, dz)| nb(*dx, *dz) == c);
    let keep = cands.iter().copied().find(|c| same(*c)).unwrap_or(cands[0]); roof_join(keep, &nb)
}
