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
        B::PLANKS => [B::PLANKS, B::PLANKS_Y, B::PLANKS_Z],
        _ => return id,
    };
    v[match axis { 'x' => 0, 'y' => 1, _ => 2 }]
}
