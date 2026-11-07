// Généré par engine/tools/gen_blocks.mjs. Les identifiants sont stables : ils vivent dans les sauvegardes.
use crate::blocks::{Block, Shape};

pub const VARIANTS: u32 = 4;
pub const MATERIALS: [&str; 29] = ["grass_top", "grass_side", "dirt", "path", "stone", "mossy_stone", "limestone", "granite", "gravel", "sand", "bark_oak", "bark_birch", "bark_beech", "leaf_oak", "leaf_birch", "leaf_beech", "planks", "forest_soil", "water_foam", "end_oak", "end_birch", "end_beech", "snow", "snow_side", "ice", "bark_spruce", "end_spruce", "leaf_spruce", "leaf_snow"];

pub const BLOCKS: [Block; 68] = [
    Block { id: 0, name: "air", shape: Shape::Air, hardness: 0.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "decor", drop: Some("air") },
    Block { id: 1, name: "grass", shape: Shape::Cube, hardness: 0.5, faces: [0,4,8], dir_faces: [4,0,8,4], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("dirt") },
    Block { id: 2, name: "dirt", shape: Shape::Cube, hardness: 0.45, faces: [8,8,8], dir_faces: [8,8,8,8], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("dirt") },
    Block { id: 3, name: "path", shape: Shape::Cube, hardness: 0.45, faces: [12,8,8], dir_faces: [8,12,8,8], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("dirt") },
    Block { id: 4, name: "forest_soil", shape: Shape::Cube, hardness: 0.5, faces: [68,8,8], dir_faces: [8,68,8,8], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("dirt") },
    Block { id: 5, name: "stone", shape: Shape::Cube, hardness: 1.3, faces: [16,16,16], dir_faces: [16,16,16,16], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("stone") },
    Block { id: 6, name: "mossy_stone", shape: Shape::Cube, hardness: 1.3, faces: [20,20,16], dir_faces: [20,20,16,20], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("mossy_stone") },
    Block { id: 7, name: "limestone", shape: Shape::Cube, hardness: 1.2, faces: [24,24,24], dir_faces: [24,24,24,24], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("limestone") },
    Block { id: 8, name: "granite", shape: Shape::Cube, hardness: 1.6, faces: [28,28,28], dir_faces: [28,28,28,28], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("granite") },
    Block { id: 9, name: "gravel", shape: Shape::Cube, hardness: 0.5, faces: [32,32,32], dir_faces: [32,32,32,32], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("gravel") },
    Block { id: 10, name: "sand", shape: Shape::Cube, hardness: 0.4, faces: [36,36,36], dir_faces: [36,36,36,36], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("sand") },
    Block { id: 11, name: "oak_log", shape: Shape::Cube, hardness: 1.0, faces: [76,40,76], dir_faces: [40,76,76,40], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("oak_log") },
    Block { id: 12, name: "birch_log", shape: Shape::Cube, hardness: 0.9, faces: [80,44,80], dir_faces: [44,80,80,44], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("birch_log") },
    Block { id: 13, name: "beech_log", shape: Shape::Cube, hardness: 1.0, faces: [84,48,84], dir_faces: [48,84,84,48], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("beech_log") },
    Block { id: 14, name: "oak_leaves", shape: Shape::Leaf, hardness: 0.12, faces: [52,52,52], dir_faces: [52,52,52,52], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,0.9], model: None, cat: "foliage", drop: None },
    Block { id: 15, name: "birch_leaves", shape: Shape::Leaf, hardness: 0.12, faces: [56,56,56], dir_faces: [56,56,56,56], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.08,1.08,0.82], model: None, cat: "foliage", drop: None },
    Block { id: 16, name: "beech_leaves", shape: Shape::Leaf, hardness: 0.12, faces: [60,60,60], dir_faces: [60,60,60,60], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [0.92,1.02,0.86], model: None, cat: "foliage", drop: None },
    Block { id: 17, name: "planks", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("planks") },
    Block { id: 18, name: "water", shape: Shape::Water, hardness: 0.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "fluid", drop: Some("water") },
    Block { id: 19, name: "tall_grass", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grass"), cat: "decor", drop: None },
    Block { id: 20, name: "fern", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fern"), cat: "decor", drop: None },
    Block { id: 21, name: "daisy", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("daisy"), cat: "decor", drop: None },
    Block { id: 22, name: "lupine", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lupine"), cat: "decor", drop: None },
    Block { id: 23, name: "pink_flower", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("pink"), cat: "decor", drop: None },
    Block { id: 24, name: "yellow_flower", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("yellow"), cat: "decor", drop: None },
    Block { id: 25, name: "mushroom", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("mushroom"), cat: "decor", drop: None },
    Block { id: 26, name: "lily_pad", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lily"), cat: "decor", drop: None },
    Block { id: 27, name: "flagstone", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("flagstone"), cat: "decor", drop: Some("stone") },
    Block { id: 28, name: "lantern", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 14, tint: [1.0,1.0,1.0], model: Some("lantern"), cat: "light", drop: Some("lantern") },
    Block { id: 29, name: "vine", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("vine"), cat: "decor", drop: None },
    Block { id: 30, name: "shelf_fungus", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fungus"), cat: "decor", drop: None },
    Block { id: 31, name: "fence", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fence"), cat: "wood", drop: Some("planks") },
    Block { id: 32, name: "oak_log_x", shape: Shape::Cube, hardness: 1.0, faces: [40,76,40], dir_faces: [76,40,40,40], grain: [0,1,1,1], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("oak_log") },
    Block { id: 33, name: "oak_log_z", shape: Shape::Cube, hardness: 1.0, faces: [40,40,76], dir_faces: [40,40,40,76], grain: [1,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("oak_log") },
    Block { id: 34, name: "birch_log_x", shape: Shape::Cube, hardness: 0.9, faces: [44,80,44], dir_faces: [80,44,44,44], grain: [0,1,1,1], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("birch_log") },
    Block { id: 35, name: "birch_log_z", shape: Shape::Cube, hardness: 0.9, faces: [44,44,80], dir_faces: [44,44,44,80], grain: [1,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("birch_log") },
    Block { id: 36, name: "beech_log_x", shape: Shape::Cube, hardness: 1.0, faces: [48,84,48], dir_faces: [84,48,48,48], grain: [0,1,1,1], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("beech_log") },
    Block { id: 37, name: "beech_log_z", shape: Shape::Cube, hardness: 1.0, faces: [48,48,84], dir_faces: [48,48,48,84], grain: [1,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("beech_log") },
    Block { id: 38, name: "planks_z", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,1,1,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("planks") },
    Block { id: 39, name: "planks_y", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [1,0,0,1], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("planks") },
    Block { id: 40, name: "water_1", shape: Shape::Water, hardness: 0.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "fluid", drop: None },
    Block { id: 41, name: "water_2", shape: Shape::Water, hardness: 0.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "fluid", drop: None },
    Block { id: 42, name: "water_3", shape: Shape::Water, hardness: 0.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "fluid", drop: None },
    Block { id: 43, name: "water_4", shape: Shape::Water, hardness: 0.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "fluid", drop: None },
    Block { id: 44, name: "water_5", shape: Shape::Water, hardness: 0.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "fluid", drop: None },
    Block { id: 45, name: "water_6", shape: Shape::Water, hardness: 0.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "fluid", drop: None },
    Block { id: 46, name: "water_7", shape: Shape::Water, hardness: 0.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "fluid", drop: None },
    Block { id: 47, name: "grass_deep", shape: Shape::Cube, hardness: 0.5, faces: [0,4,8], dir_faces: [4,0,8,4], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("dirt") },
    Block { id: 48, name: "oak_leaves_deep", shape: Shape::Leaf, hardness: 0.12, faces: [52,52,52], dir_faces: [52,52,52,52], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [0.56,0.68,0.56], model: None, cat: "foliage", drop: None },
    Block { id: 49, name: "beech_leaves_deep", shape: Shape::Leaf, hardness: 0.12, faces: [60,60,60], dir_faces: [60,60,60,60], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [0.5,0.66,0.54], model: None, cat: "foliage", drop: None },
    Block { id: 50, name: "glowcap", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("glowcap"), cat: "decor", drop: Some("glowcap") },
    Block { id: 51, name: "oak_log_c", shape: Shape::Cube, hardness: 1.0, faces: [76,40,76], dir_faces: [40,76,76,40], grain: [4,4,4,4], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("oak_log") },
    Block { id: 52, name: "oak_log_x_c", shape: Shape::Cube, hardness: 1.0, faces: [40,76,40], dir_faces: [76,40,40,40], grain: [4,5,5,5], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("oak_log") },
    Block { id: 53, name: "oak_log_z_c", shape: Shape::Cube, hardness: 1.0, faces: [40,40,76], dir_faces: [40,40,40,76], grain: [5,6,6,4], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("oak_log") },
    Block { id: 54, name: "beech_log_c", shape: Shape::Cube, hardness: 1.0, faces: [84,48,84], dir_faces: [48,84,84,48], grain: [4,4,4,4], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("beech_log") },
    Block { id: 55, name: "beech_log_x_c", shape: Shape::Cube, hardness: 1.0, faces: [48,84,48], dir_faces: [84,48,48,48], grain: [4,5,5,5], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("beech_log") },
    Block { id: 56, name: "beech_log_z_c", shape: Shape::Cube, hardness: 1.0, faces: [48,48,84], dir_faces: [48,48,48,84], grain: [5,6,6,4], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("beech_log") },
    Block { id: 57, name: "snow", shape: Shape::Cube, hardness: 0.4, faces: [88,92,8], dir_faces: [92,88,8,92], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("snow") },
    Block { id: 58, name: "ice", shape: Shape::Cube, hardness: 0.5, faces: [96,96,96], dir_faces: [96,96,96,96], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("ice") },
    Block { id: 59, name: "spruce_log", shape: Shape::Cube, hardness: 1.0, faces: [104,100,104], dir_faces: [100,104,104,100], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 60, name: "spruce_log_x", shape: Shape::Cube, hardness: 1.0, faces: [100,104,100], dir_faces: [104,100,100,100], grain: [0,1,1,1], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 61, name: "spruce_log_z", shape: Shape::Cube, hardness: 1.0, faces: [100,100,104], dir_faces: [100,100,100,104], grain: [1,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 62, name: "spruce_log_c", shape: Shape::Cube, hardness: 1.0, faces: [104,100,104], dir_faces: [100,104,104,100], grain: [4,4,4,4], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 63, name: "spruce_log_x_c", shape: Shape::Cube, hardness: 1.0, faces: [100,104,100], dir_faces: [104,100,100,100], grain: [4,5,5,5], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 64, name: "spruce_log_z_c", shape: Shape::Cube, hardness: 1.0, faces: [100,100,104], dir_faces: [100,100,100,104], grain: [5,6,6,4], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 65, name: "spruce_leaves", shape: Shape::Leaf, hardness: 0.12, faces: [108,108,108], dir_faces: [108,108,108,108], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "foliage", drop: None },
    Block { id: 66, name: "spruce_leaves_snow", shape: Shape::Leaf, hardness: 0.12, faces: [112,112,112], dir_faces: [112,112,112,112], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "foliage", drop: None },
    Block { id: 67, name: "icicle", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("icicle"), cat: "decor", drop: None },
];

#[allow(non_snake_case)]
pub mod B {
    pub const AIR: u8 = 0;
    pub const GRASS: u8 = 1;
    pub const DIRT: u8 = 2;
    pub const PATH: u8 = 3;
    pub const FOREST_SOIL: u8 = 4;
    pub const STONE: u8 = 5;
    pub const MOSSY_STONE: u8 = 6;
    pub const LIMESTONE: u8 = 7;
    pub const GRANITE: u8 = 8;
    pub const GRAVEL: u8 = 9;
    pub const SAND: u8 = 10;
    pub const OAK_LOG: u8 = 11;
    pub const BIRCH_LOG: u8 = 12;
    pub const BEECH_LOG: u8 = 13;
    pub const OAK_LEAVES: u8 = 14;
    pub const BIRCH_LEAVES: u8 = 15;
    pub const BEECH_LEAVES: u8 = 16;
    pub const PLANKS: u8 = 17;
    pub const WATER: u8 = 18;
    pub const TALL_GRASS: u8 = 19;
    pub const FERN: u8 = 20;
    pub const DAISY: u8 = 21;
    pub const LUPINE: u8 = 22;
    pub const PINK_FLOWER: u8 = 23;
    pub const YELLOW_FLOWER: u8 = 24;
    pub const MUSHROOM: u8 = 25;
    pub const LILY_PAD: u8 = 26;
    pub const FLAGSTONE: u8 = 27;
    pub const LANTERN: u8 = 28;
    pub const VINE: u8 = 29;
    pub const SHELF_FUNGUS: u8 = 30;
    pub const FENCE: u8 = 31;
    pub const OAK_LOG_X: u8 = 32;
    pub const OAK_LOG_Z: u8 = 33;
    pub const BIRCH_LOG_X: u8 = 34;
    pub const BIRCH_LOG_Z: u8 = 35;
    pub const BEECH_LOG_X: u8 = 36;
    pub const BEECH_LOG_Z: u8 = 37;
    pub const PLANKS_Z: u8 = 38;
    pub const PLANKS_Y: u8 = 39;
    pub const WATER_1: u8 = 40;
    pub const WATER_2: u8 = 41;
    pub const WATER_3: u8 = 42;
    pub const WATER_4: u8 = 43;
    pub const WATER_5: u8 = 44;
    pub const WATER_6: u8 = 45;
    pub const WATER_7: u8 = 46;
    pub const GRASS_DEEP: u8 = 47;
    pub const OAK_LEAVES_DEEP: u8 = 48;
    pub const BEECH_LEAVES_DEEP: u8 = 49;
    pub const GLOWCAP: u8 = 50;
    pub const OAK_LOG_C: u8 = 51;
    pub const OAK_LOG_X_C: u8 = 52;
    pub const OAK_LOG_Z_C: u8 = 53;
    pub const BEECH_LOG_C: u8 = 54;
    pub const BEECH_LOG_X_C: u8 = 55;
    pub const BEECH_LOG_Z_C: u8 = 56;
    pub const SNOW: u8 = 57;
    pub const ICE: u8 = 58;
    pub const SPRUCE_LOG: u8 = 59;
    pub const SPRUCE_LOG_X: u8 = 60;
    pub const SPRUCE_LOG_Z: u8 = 61;
    pub const SPRUCE_LOG_C: u8 = 62;
    pub const SPRUCE_LOG_X_C: u8 = 63;
    pub const SPRUCE_LOG_Z_C: u8 = 64;
    pub const SPRUCE_LEAVES: u8 = 65;
    pub const SPRUCE_LEAVES_SNOW: u8 = 66;
    pub const ICICLE: u8 = 67;
}
