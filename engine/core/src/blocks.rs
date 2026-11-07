//! Registre de blocs. Ajouter un bloc revient à ajouter une ligne de données.
pub use crate::blocks_data::{BLOCKS, B, MATERIALS, VARIANTS};

pub const CX: usize = 32;
pub const CH: usize = 128;
pub const CA: usize = CX * CX * CH;

#[inline(always)]
pub const fn idx(x: usize, y: usize, z: usize) -> usize { x + z * CX + y * CX * CX }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shape { Air, Cube, Leaf, Water, Deco }

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub id: u8, pub name: &'static str, pub shape: Shape, pub hardness: f64,
    pub faces: [u16; 3], pub dir_faces: [u16; 4], pub grain: [u8; 4],
    pub solid: bool, pub opaque: bool, pub light: u8, pub tint: [f64; 3],
    pub model: Option<&'static str>, pub cat: &'static str, pub drop: Option<&'static str>,
}

#[inline(always)] pub fn is_solid(b: u8) -> bool { BLOCKS[b as usize].solid }
#[inline(always)] pub fn is_opaque(b: u8) -> bool { BLOCKS[b as usize].opaque }
#[inline(always)] pub fn is_water(b: u8) -> bool { BLOCKS[b as usize].shape == Shape::Water }
/// Niveau d'eau : 8 pour une source, 1 à 7 pour l'eau qui coule, 0 sinon.
#[inline] pub fn water_level(b: u8) -> u8 { if b == B::WATER { 8 } else if is_water(b) { b - B::WATER_1 + 1 } else { 0 } }
#[inline] pub fn flow_block(level: u8) -> u8 { B::WATER_1 + level - 1 }
/// Vrai pour tous les troncs, quelle que soit leur orientation ; l'essence est le premier mot du nom.
pub fn is_log(b: u8) -> bool { BLOCKS[b as usize].name.contains("_log") }
/// Teinte de biome des faces pleines : numéro dans TONES, gardé dans le masque du maillage.
pub const TONES: [[f64; 3]; 2] = [[1.0, 1.0, 1.0], [0.6, 0.74, 0.62]];
#[inline] pub fn tone(b: u8) -> i32 { if b == B::GRASS_DEEP { 1 } else { 0 } }
#[inline] pub fn is_grass(b: u8) -> bool { b == B::GRASS || b == B::GRASS_DEEP }
/// Variante sans mousse d'un bloc posé par le joueur, quand elle existe.
pub fn placed_variant(b: u8) -> u8 { let n = BLOCKS[b as usize].name; if is_log(b) { by_name(&format!("{n}_c")).unwrap_or(b) } else { b } }
pub fn species(b: u8) -> &'static str { BLOCKS[b as usize].name.split('_').next().unwrap_or("") }
#[inline(always)] pub fn is_leaf(b: u8) -> bool { BLOCKS[b as usize].shape == Shape::Leaf }
#[inline(always)] pub fn is_deco(b: u8) -> bool { BLOCKS[b as usize].shape == Shape::Deco }
#[inline(always)] pub fn emit(b: u8) -> u8 { BLOCKS[b as usize].light }
pub fn mat_layer(name: &str) -> u16 { MATERIALS.iter().position(|m| *m == name).expect("matériau inconnu") as u16 * VARIANTS as u16 }

/// Bloc orientable : variante pour l'axe 'x', 'y' ou 'z'.
pub fn oriented_block(id: u8, axis: char) -> u8 {
    let v = match id {
        B::OAK_LOG => [B::OAK_LOG_X, B::OAK_LOG, B::OAK_LOG_Z],
        B::BIRCH_LOG => [B::BIRCH_LOG_X, B::BIRCH_LOG, B::BIRCH_LOG_Z],
        B::BEECH_LOG => [B::BEECH_LOG_X, B::BEECH_LOG, B::BEECH_LOG_Z],
        B::SPRUCE_LOG => [B::SPRUCE_LOG_X, B::SPRUCE_LOG, B::SPRUCE_LOG_Z],
        B::PLANKS => [B::PLANKS, B::PLANKS_Y, B::PLANKS_Z],
        _ => return id,
    };
    v[match axis { 'x' => 0, 'y' => 1, _ => 2 }]
}

/// Axe de pose selon le regard. `prev` évite de basculer sans arrêt quand on vise entre deux directions.
/// Modes : 0 selon le regard, 1 debout, 2 dans le sens du regard, 3 en travers.
pub fn gaze_axis(dx: f64, dy: f64, dz: f64, mode: u8, prev: Option<char>) -> char {
    let (ax, az) = (dx.abs(), dz.abs()); let mut h = if ax >= az { 'x' } else { 'z' };
    if prev == Some('x') && ax * 1.25 > az { h = 'x'; } else if prev == Some('z') && az * 1.25 > ax { h = 'z'; }
    match mode { 1 => 'y', 2 => h, 3 => if h == 'x' { 'z' } else { 'x' }, _ => if dy.abs() > if prev == Some('y') { 0.74 } else { 0.84 } { 'y' } else { h } }
}
pub const MODES: [&str; 4] = ["selon le regard", "debout", "dans le sens du regard", "en travers du regard"];
pub fn by_name(n: &str) -> Option<u8> { BLOCKS.iter().find(|b| b.name == n).map(|b| b.id) }
