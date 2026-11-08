//! Blocs de construction de forme : dalles, escaliers, porte, trappe, échelle, fenêtre, piliers, murs fins.
//! Une seule source pour la géométrie : les boîtes servent au dessin, aux collisions et à l'aperçu de pose,
//! de sorte que ce que l'on voit est exactement ce qui arrête le joueur.
//! Orientation : le nom porte un quart de tour (« _1 » à « _3 »), la forme de base regarde vers +z.
use crate::blocks::{by_name, BLOCKS};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum Kind { Slab, Stairs, Door, Trapdoor, Ladder, Window, Pillar, Wall }
/// Ce qu'un bloc de forme est : sa famille, son sens (0 : +z, 1 : +x, 2 : -z, 3 : -x), moitié haute, ouvert, haut de porte,
/// et la variante de base (l'objet posé), dont on dérive toutes les autres.
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Info { pub kind: Kind, pub facing: u8, pub top: bool, pub open: bool, pub upper: bool, pub base: u8 }
pub type Bx = [f64; 6];

fn parse(name: &str) -> Option<Info> {
    let digit = |s: &str| s.parse::<u8>().ok();
    let base = |n: &str| by_name(n).unwrap_or(0);
    if let Some(m) = name.strip_suffix("_slab_top") { return Some(Info { kind: Kind::Slab, facing: 0, top: true, open: false, upper: false, base: base(&format!("{m}_slab")) }); }
    if name.ends_with("_slab") { return Some(Info { kind: Kind::Slab, facing: 0, top: false, open: false, upper: false, base: base(name) }); }
    if let Some(i) = name.find("_stairs") { let f = name[i + 7..].strip_prefix('_').and_then(digit).unwrap_or(0); return Some(Info { kind: Kind::Stairs, facing: f, top: false, open: false, upper: false, base: base(&name[..i + 7]) }); }
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
fn table() -> &'static [Option<Info>; 256] {
    static T: OnceLock<[Option<Info>; 256]> = OnceLock::new();
    T.get_or_init(|| { let mut t = [None; 256]; for b in BLOCKS.iter() { t[b.id as usize] = parse(b.name); } t })
}
pub fn info(id: u8) -> Option<Info> { table()[id as usize] }

/// Un quart de tour vers la droite, vu d'en haut : +z devient +x.
fn turn(b: Bx, f: u8) -> Bx { let mut b = b; for _ in 0..f % 4 { b = [b[2], b[1], 1.0 - b[3], b[5], b[4], 1.0 - b[0]]; } b }
const PANEL: f64 = 0.1875;
/// La géométrie, dans le repère du bloc [0, 1]³. Ce sont aussi les boîtes de collision (l'échelle n'en a pas : on y grimpe).
pub fn geometry(i: Info) -> Vec<Bx> {
    let f = i.facing; let r = |v: &[Bx]| v.iter().map(|b| turn(*b, f)).collect::<Vec<_>>();
    match i.kind {
        Kind::Slab => vec![if i.top { [0.0, 0.5, 0.0, 1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0, 0.5, 1.0] }],
        // Marche basse sur toute la surface, marche haute du côté où l'on monte (+z dans le sens de base).
        Kind::Stairs => r(&[[0.0, 0.0, 0.0, 1.0, 0.5, 1.0], [0.0, 0.5, 0.5, 1.0, 1.0, 1.0]]),
        // Porte fermée : un battant contre le bord proche ; ouverte, il pivote contre le côté.
        Kind::Door => r(&[if i.open { [0.0, 0.0, 0.0, PANEL, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0, 1.0, PANEL] }]),
        Kind::Trapdoor => if i.open { r(&[[0.0, 0.0, 1.0 - PANEL, 1.0, 1.0, 1.0]]) } else { vec![[0.0, 0.0, 0.0, 1.0, PANEL, 1.0]] },
        Kind::Ladder => Vec::new(),
        Kind::Window => r(&[[0.0, 0.0, 0.4375, 1.0, 1.0, 0.5625]]),
        Kind::Pillar => vec![[0.25, 0.0, 0.25, 0.75, 1.0, 0.75]],
        Kind::Wall => r(&[[0.0, 0.0, 0.3125, 1.0, 1.0, 0.6875]]),
    }
}
/// Boîtes de collision d'un bloc : `None` pour un bloc ordinaire (cube plein ou rien, selon `is_solid`).
pub fn boxes(id: u8) -> Option<Vec<Bx>> { info(id).map(geometry) }
/// Les montants de l'échelle, contre le mur (+z dans le sens de base).
pub fn ladder_box(f: u8) -> Bx { turn([0.0, 0.0, 1.0 - 0.125, 1.0, 1.0, 1.0], f) }
pub fn is_ladder(id: u8) -> bool { info(id).is_some_and(|i| i.kind == Kind::Ladder) }

/// La variante d'une famille pour un sens, une moitié, un état.
pub fn variant(base: u8, facing: u8, top: bool, open: bool, upper: bool) -> u8 {
    let n = BLOCKS[base as usize].name; let f = facing % 4; let sfx = |f: u8| if f == 0 { String::new() } else { format!("_{f}") };
    let name = match info(base).map(|i| i.kind) {
        Some(Kind::Slab) => if top { format!("{n}_top") } else { n.to_string() },
        Some(Kind::Stairs) => format!("{n}{}", sfx(f)),
        Some(Kind::Door) => if (f, open, upper) == (0, false, false) { "door".into() } else { format!("door_{f}{}{}", open as u8, upper as u8) },
        Some(Kind::Trapdoor) => if open { format!("trapdoor_open{}", sfx(f)) } else { "trapdoor".into() },
        Some(Kind::Ladder) => format!("ladder{}", sfx(f)),
        Some(Kind::Window) => if f % 2 == 0 { "window".into() } else { "window_x".into() },
        Some(Kind::Wall) => if f % 2 == 0 { "stone_wall".into() } else { "stone_wall_x".into() },
        _ => n.to_string(),
    };
    by_name(&name).unwrap_or(base)
}
/// Le sens du regard, au quart de tour près (0 : +z, 1 : +x, 2 : -z, 3 : -x).
pub fn facing_of(dx: f64, dz: f64) -> u8 { if dx.abs() > dz.abs() { if dx > 0.0 { 1 } else { 3 } } else if dz > 0.0 { 0 } else { 2 } }
/// La matière d'un bloc de forme : le bloc plein dont il prend la texture.
pub fn material(id: u8) -> u8 {
    let n = BLOCKS[id as usize].name;
    let m = if n.starts_with("stone_bricks") || n == "stone_pillar" || n.starts_with("stone_wall") { "stone_bricks" } else if n.starts_with("cobblestone") { "cobblestone" } else if n.starts_with("thatch") { "thatch" } else { "planks" };
    by_name(m).unwrap_or(0)
}
/// Deux dalles de même matière l'une sur l'autre font un bloc plein.
pub fn full_of_slab(base: u8) -> Option<u8> { BLOCKS[base as usize].name.strip_suffix("_slab").and_then(by_name) }
