// Généré par engine/tools/gen_blocks.mjs. Les identifiants sont stables : ils vivent dans les sauvegardes.
use crate::blocks::{Block, Shape};

pub const VARIANTS: u32 = 4;
pub const MATERIALS: [&str; 48] = ["grass_top", "grass_side", "dirt", "path", "stone", "mossy_stone", "limestone", "granite", "gravel", "sand", "bark_oak", "bark_birch", "bark_beech", "leaf_oak", "leaf_birch", "leaf_beech", "planks", "forest_soil", "water_foam", "end_oak", "end_birch", "end_beech", "snow", "snow_side", "ice", "bark_spruce", "end_spruce", "leaf_spruce", "leaf_snow", "grass_dry_top", "grass_dry_side", "dry_earth", "bark_dead", "end_dead", "lichen", "bark_acacia", "end_acacia", "leaf_acacia", "bone", "wood_inner", "sandstone", "sandstone_top", "red_sandstone", "cactus_side", "cactus_top", "bark_palm", "end_palm", "leaf_palm"];

pub const BLOCKS: [Block; 124] = [
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
    Block { id: 57, name: "snow", shape: Shape::Cube, hardness: 0.4, faces: [88,88,88], dir_faces: [88,88,88,88], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("snow") },
    Block { id: 58, name: "ice", shape: Shape::Cube, hardness: 0.5, faces: [96,96,96], dir_faces: [96,96,96,96], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("ice") },
    Block { id: 59, name: "spruce_log", shape: Shape::Cube, hardness: 1.0, faces: [104,100,104], dir_faces: [100,104,104,100], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 60, name: "spruce_log_x", shape: Shape::Cube, hardness: 1.0, faces: [100,104,100], dir_faces: [104,100,100,100], grain: [0,1,1,1], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 61, name: "spruce_log_z", shape: Shape::Cube, hardness: 1.0, faces: [100,100,104], dir_faces: [100,100,100,104], grain: [1,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 62, name: "spruce_log_c", shape: Shape::Cube, hardness: 1.0, faces: [104,100,104], dir_faces: [100,104,104,100], grain: [4,4,4,4], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 63, name: "spruce_log_x_c", shape: Shape::Cube, hardness: 1.0, faces: [100,104,100], dir_faces: [104,100,100,100], grain: [4,5,5,5], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 64, name: "spruce_log_z_c", shape: Shape::Cube, hardness: 1.0, faces: [100,100,104], dir_faces: [100,100,100,104], grain: [5,6,6,4], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_log") },
    Block { id: 65, name: "spruce_leaves", shape: Shape::Leaf, hardness: 0.12, faces: [108,108,108], dir_faces: [108,108,108,108], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "foliage", drop: None },
    Block { id: 66, name: "spruce_leaves_snow", shape: Shape::Leaf, hardness: 0.12, faces: [88,112,108], dir_faces: [112,88,108,112], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "foliage", drop: None },
    Block { id: 67, name: "icicle", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("icicle"), cat: "decor", drop: None },
    Block { id: 68, name: "snow_layer", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("snowlayer"), cat: "decor", drop: Some("snow_layer") },
    Block { id: 69, name: "pinecone", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("pinecone"), cat: "decor", drop: Some("pinecone") },
    Block { id: 70, name: "frost_grass", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("frost"), cat: "decor", drop: None },
    Block { id: 71, name: "snow_layer_2", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("snowlayer"), cat: "decor", drop: Some("snow_layer") },
    Block { id: 72, name: "snow_layer_3", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("snowlayer"), cat: "decor", drop: Some("snow_layer") },
    Block { id: 73, name: "snow_layer_4", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("snowlayer"), cat: "decor", drop: Some("snow_layer") },
    Block { id: 74, name: "snow_layer_5", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("snowlayer"), cat: "decor", drop: Some("snow_layer") },
    Block { id: 75, name: "snow_layer_6", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("snowlayer"), cat: "decor", drop: Some("snow_layer") },
    Block { id: 76, name: "snow_layer_7", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("snowlayer"), cat: "decor", drop: Some("snow_layer") },
    Block { id: 77, name: "snow_layer_8", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("snowlayer"), cat: "decor", drop: Some("snow_layer") },
    Block { id: 78, name: "snow_layer_9", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("snowlayer"), cat: "decor", drop: Some("snow_layer") },
    Block { id: 79, name: "steppe_grass", shape: Shape::Cube, hardness: 0.5, faces: [116,120,8], dir_faces: [120,116,8,120], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("dirt") },
    Block { id: 80, name: "dry_earth", shape: Shape::Cube, hardness: 0.45, faces: [124,124,8], dir_faces: [124,124,8,124], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("dry_earth") },
    Block { id: 81, name: "feather_grass", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("feather"), cat: "decor", drop: Some("feather_grass") },
    Block { id: 82, name: "sagebrush", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sage"), cat: "decor", drop: Some("sagebrush") },
    Block { id: 83, name: "dead_bush", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("deadbush"), cat: "decor", drop: Some("dead_bush") },
    Block { id: 84, name: "dead_log", shape: Shape::Cube, hardness: 0.9, faces: [132,128,132], dir_faces: [128,132,132,128], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("dead_log") },
    Block { id: 85, name: "dead_log_x", shape: Shape::Cube, hardness: 0.9, faces: [128,132,128], dir_faces: [132,128,128,128], grain: [0,1,1,1], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("dead_log") },
    Block { id: 86, name: "dead_log_z", shape: Shape::Cube, hardness: 0.9, faces: [128,128,132], dir_faces: [128,128,128,132], grain: [1,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("dead_log") },
    Block { id: 87, name: "lichen_stone", shape: Shape::Cube, hardness: 1.3, faces: [136,136,16], dir_faces: [136,136,16,136], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("lichen_stone") },
    Block { id: 88, name: "blueberry", shape: Shape::Deco, hardness: 0.03, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("blueberry"), cat: "decor", drop: Some("blueberry") },
    Block { id: 89, name: "rowan", shape: Shape::Deco, hardness: 0.06, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("rowan"), cat: "decor", drop: Some("rowan") },
    Block { id: 90, name: "fern_dry", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fern_dry"), cat: "decor", drop: Some("fern_dry") },
    Block { id: 91, name: "acacia_log", shape: Shape::Cube, hardness: 1.0, faces: [144,140,144], dir_faces: [140,144,144,140], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("acacia_log") },
    Block { id: 92, name: "acacia_log_x", shape: Shape::Cube, hardness: 1.0, faces: [140,144,140], dir_faces: [144,140,140,140], grain: [0,1,1,1], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("acacia_log") },
    Block { id: 93, name: "acacia_log_z", shape: Shape::Cube, hardness: 1.0, faces: [140,140,144], dir_faces: [140,140,140,144], grain: [1,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("acacia_log") },
    Block { id: 94, name: "acacia_leaves", shape: Shape::Leaf, hardness: 0.12, faces: [148,148,148], dir_faces: [148,148,148,148], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [0.66,0.66,0.46], model: None, cat: "foliage", drop: None },
    Block { id: 95, name: "bone", shape: Shape::Cube, hardness: 0.9, faces: [152,152,152], dir_faces: [152,152,152,152], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("bone") },
    Block { id: 96, name: "birch_leaves_gold", shape: Shape::Leaf, hardness: 0.12, faces: [56,56,56], dir_faces: [56,56,56,56], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.45,1.05,0.35], model: None, cat: "foliage", drop: None },
    Block { id: 97, name: "oak_branch", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("branch"), cat: "wood", drop: Some("oak_log") },
    Block { id: 98, name: "birch_branch", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("branch"), cat: "wood", drop: Some("birch_log") },
    Block { id: 99, name: "beech_branch", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("branch"), cat: "wood", drop: Some("beech_log") },
    Block { id: 100, name: "spruce_branch", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("branch"), cat: "wood", drop: Some("spruce_log") },
    Block { id: 101, name: "dead_branch", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("branch"), cat: "wood", drop: Some("dead_log") },
    Block { id: 102, name: "acacia_branch", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("branch"), cat: "wood", drop: Some("acacia_log") },
    Block { id: 103, name: "oak_hollow", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hollow"), cat: "wood", drop: Some("oak_hollow") },
    Block { id: 104, name: "oak_hollow_x", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hollow"), cat: "wood", drop: Some("oak_hollow") },
    Block { id: 105, name: "oak_hollow_z", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hollow"), cat: "wood", drop: Some("oak_hollow") },
    Block { id: 106, name: "spruce_hollow", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hollow"), cat: "wood", drop: Some("spruce_hollow") },
    Block { id: 107, name: "spruce_hollow_x", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hollow"), cat: "wood", drop: Some("spruce_hollow") },
    Block { id: 108, name: "spruce_hollow_z", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hollow"), cat: "wood", drop: Some("spruce_hollow") },
    Block { id: 109, name: "dead_hollow", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hollow"), cat: "wood", drop: Some("dead_hollow") },
    Block { id: 110, name: "dead_hollow_x", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hollow"), cat: "wood", drop: Some("dead_hollow") },
    Block { id: 111, name: "dead_hollow_z", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hollow"), cat: "wood", drop: Some("dead_hollow") },
    Block { id: 112, name: "glowcap_blue", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("glowcap"), cat: "decor", drop: Some("glowcap_blue") },
    Block { id: 113, name: "glowcap_pink", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("glowcap"), cat: "decor", drop: Some("glowcap_pink") },
    Block { id: 114, name: "glowcap_violet", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("glowcap"), cat: "decor", drop: Some("glowcap_violet") },
    Block { id: 115, name: "glowcap_cyan", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("glowcap"), cat: "decor", drop: Some("glowcap_cyan") },
    Block { id: 116, name: "sandstone", shape: Shape::Cube, hardness: 1.0, faces: [164,160,164], dir_faces: [160,164,164,160], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("sandstone") },
    Block { id: 117, name: "red_sandstone", shape: Shape::Cube, hardness: 1.0, faces: [168,168,168], dir_faces: [168,168,168,168], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("red_sandstone") },
    Block { id: 118, name: "cactus", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cactus"), cat: "foliage", drop: Some("cactus") },
    Block { id: 119, name: "palm_log", shape: Shape::Cube, hardness: 0.9, faces: [184,180,184], dir_faces: [180,184,184,180], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("palm_log") },
    Block { id: 120, name: "palm_leaves", shape: Shape::Leaf, hardness: 0.12, faces: [188,188,188], dir_faces: [188,188,188,188], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [0.8,0.86,0.6], model: None, cat: "foliage", drop: None },
    Block { id: 121, name: "moor_grass", shape: Shape::Cube, hardness: 0.5, faces: [0,4,8], dir_faces: [4,0,8,4], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("dirt") },
    Block { id: 122, name: "heather", shape: Shape::Deco, hardness: 0.04, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("heather"), cat: "decor", drop: Some("heather") },
    Block { id: 123, name: "cotton_grass", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cotton"), cat: "decor", drop: Some("cotton_grass") },
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
    pub const SNOW_LAYER: u8 = 68;
    pub const PINECONE: u8 = 69;
    pub const FROST_GRASS: u8 = 70;
    pub const SNOW_LAYER_2: u8 = 71;
    pub const SNOW_LAYER_3: u8 = 72;
    pub const SNOW_LAYER_4: u8 = 73;
    pub const SNOW_LAYER_5: u8 = 74;
    pub const SNOW_LAYER_6: u8 = 75;
    pub const SNOW_LAYER_7: u8 = 76;
    pub const SNOW_LAYER_8: u8 = 77;
    pub const SNOW_LAYER_9: u8 = 78;
    pub const STEPPE_GRASS: u8 = 79;
    pub const DRY_EARTH: u8 = 80;
    pub const FEATHER_GRASS: u8 = 81;
    pub const SAGEBRUSH: u8 = 82;
    pub const DEAD_BUSH: u8 = 83;
    pub const DEAD_LOG: u8 = 84;
    pub const DEAD_LOG_X: u8 = 85;
    pub const DEAD_LOG_Z: u8 = 86;
    pub const LICHEN_STONE: u8 = 87;
    pub const BLUEBERRY: u8 = 88;
    pub const ROWAN: u8 = 89;
    pub const FERN_DRY: u8 = 90;
    pub const ACACIA_LOG: u8 = 91;
    pub const ACACIA_LOG_X: u8 = 92;
    pub const ACACIA_LOG_Z: u8 = 93;
    pub const ACACIA_LEAVES: u8 = 94;
    pub const BONE: u8 = 95;
    pub const BIRCH_LEAVES_GOLD: u8 = 96;
    pub const OAK_BRANCH: u8 = 97;
    pub const BIRCH_BRANCH: u8 = 98;
    pub const BEECH_BRANCH: u8 = 99;
    pub const SPRUCE_BRANCH: u8 = 100;
    pub const DEAD_BRANCH: u8 = 101;
    pub const ACACIA_BRANCH: u8 = 102;
    pub const OAK_HOLLOW: u8 = 103;
    pub const OAK_HOLLOW_X: u8 = 104;
    pub const OAK_HOLLOW_Z: u8 = 105;
    pub const SPRUCE_HOLLOW: u8 = 106;
    pub const SPRUCE_HOLLOW_X: u8 = 107;
    pub const SPRUCE_HOLLOW_Z: u8 = 108;
    pub const DEAD_HOLLOW: u8 = 109;
    pub const DEAD_HOLLOW_X: u8 = 110;
    pub const DEAD_HOLLOW_Z: u8 = 111;
    pub const GLOWCAP_BLUE: u8 = 112;
    pub const GLOWCAP_PINK: u8 = 113;
    pub const GLOWCAP_VIOLET: u8 = 114;
    pub const GLOWCAP_CYAN: u8 = 115;
    pub const SANDSTONE: u8 = 116;
    pub const RED_SANDSTONE: u8 = 117;
    pub const CACTUS: u8 = 118;
    pub const PALM_LOG: u8 = 119;
    pub const PALM_LEAVES: u8 = 120;
    pub const MOOR_GRASS: u8 = 121;
    pub const HEATHER: u8 = 122;
    pub const COTTON_GRASS: u8 = 123;
}
