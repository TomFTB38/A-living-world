//! Blocs de construction de forme : dalles, escaliers, porte, trappe, échelle, fenêtre, piliers, murs fins.
//! Une seule source pour la géométrie : les boîtes servent au dessin, aux collisions et à l'aperçu de pose,
//! de sorte que ce que l'on voit est exactement ce qui arrête le joueur.
//! Orientation : le nom porte un quart de tour (« _1 » à « _3 »), la forme de base regarde vers +z.
use crate::blocks::{Id, by_name, BLOCKS};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum Kind { Slab, Stairs, Door, Trapdoor, Ladder, Window, Pillar, Wall, VSlab, Shutter, Turn }
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
fn turn(b: Bx, f: u8) -> Bx { let mut b = b; for _ in 0..f % 4 { b = [b[2], b[1], 1.0 - b[3], b[5], b[4], 1.0 - b[0]]; } b }
const PANEL: f64 = 0.1875;
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
        Kind::Wall => r(&[[0.0, 0.0, 0.3125, 1.0, 1.0, 0.6875]]),
        Kind::Turn => r(&turn_boxes(BLOCKS[i.base as usize].name)),
    }
}
/// Le volume d'un meuble orientable, pour le sens 0 (dos vers +z).
fn turn_boxes(n: &str) -> Vec<Bx> {
    match n {
        "chair" => vec![[0.14, 0.0, 0.14, 0.86, 0.52, 0.86], [0.18, 0.52, 0.76, 0.82, 1.0, 0.82]], "bench" => vec![[0.02, 0.0, 0.22, 0.98, 0.5, 0.78]],
        "armchair" | "throne" => vec![[0.1, 0.0, 0.1, 0.9, 0.56, 0.9], [0.1, 0.56, 0.8, 0.9, 1.0, 0.92]], "cupboard" => vec![[0.04, 0.0, 0.16, 0.96, 1.0, 0.98]],
        "spinning_wheel" => vec![[0.1, 0.0, 0.25, 0.9, 0.6, 0.75]], "grindstone" => vec![[0.15, 0.0, 0.2, 0.85, 0.9, 0.8]], "signpost" => vec![[0.42, 0.0, 0.42, 0.58, 1.0, 0.58]],
        "fireplace" | "loom" => vec![[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]],
        _ => vec![[0.0, 0.0, 0.84, 1.0, 1.0, 1.0]],
    }
}
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
        Some(Kind::Wall) => if f % 2 == 0 { "stone_wall".into() } else { "stone_wall_x".into() },
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
    for sfx in ["_stairs", "_vslab", "_slab"] { if let Some(i) = n.find(sfx) { if let Some(b) = by_name(&n[..i]) { if BLOCKS[b as usize].faces[0] != 0 || b == crate::blocks::B::PLANKS { return b; } } } }
    let m = if n.starts_with("stone_bricks") || n == "stone_pillar" || n.starts_with("stone_wall") { "stone_bricks" } else if n.starts_with("cobblestone") { "cobblestone" } else if n.starts_with("thatch") { "thatch" } else { "planks" };
    by_name(m).unwrap_or(0)
}
/// Deux dalles de même matière l'une sur l'autre font un bloc plein.
pub fn full_of_slab(base: Id) -> Option<Id> { BLOCKS[base as usize].name.strip_suffix("_slab").and_then(by_name) }
