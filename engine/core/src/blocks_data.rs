// Généré par engine/tools/gen_blocks.mjs. Les identifiants sont stables : ils vivent dans les sauvegardes.
use crate::blocks::{Block, Shape};

pub const VARIANTS: u32 = 4;
pub const MATERIALS: [&str; 60] = ["grass_top", "grass_side", "dirt", "path", "stone", "mossy_stone", "limestone", "granite", "gravel", "sand", "bark_oak", "bark_birch", "bark_beech", "leaf_oak", "leaf_birch", "leaf_beech", "planks", "forest_soil", "water_foam", "end_oak", "end_birch", "end_beech", "snow", "snow_side", "ice", "bark_spruce", "end_spruce", "leaf_spruce", "leaf_snow", "grass_dry_top", "grass_dry_side", "dry_earth", "bark_dead", "end_dead", "lichen", "bark_acacia", "end_acacia", "leaf_acacia", "bone", "wood_inner", "sandstone", "sandstone_top", "red_sandstone", "cactus_side", "cactus_top", "bark_palm", "end_palm", "leaf_palm", "ore_copper", "ore_tin", "ore_iron", "ore_coal", "clay", "stone_bricks", "bricks", "thatch", "planks_birch", "planks_spruce", "cobble", "farmland_top"];

pub const BLOCKS: [Block; 251] = [
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
    Block { id: 124, name: "campfire", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("campfire"), cat: "light", drop: Some("campfire") },
    Block { id: 125, name: "workbench", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("workbench"), cat: "wood", drop: Some("workbench") },
    Block { id: 126, name: "twigs", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("twigs"), cat: "decor", drop: Some("stick") },
    Block { id: 127, name: "pebbles", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("pebbles"), cat: "decor", drop: Some("pebble") },
    Block { id: 128, name: "blueberry_picked", shape: Shape::Deco, hardness: 0.03, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("blueberry"), cat: "decor", drop: Some("blueberry") },
    Block { id: 129, name: "rowan_picked", shape: Shape::Deco, hardness: 0.06, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("rowan"), cat: "decor", drop: Some("rowan") },
    Block { id: 130, name: "coal_ore", shape: Shape::Cube, hardness: 1.6, faces: [204,204,204], dir_faces: [204,204,204,204], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("coal") },
    Block { id: 131, name: "copper_ore", shape: Shape::Cube, hardness: 2.0, faces: [192,192,192], dir_faces: [192,192,192,192], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("copper_ore") },
    Block { id: 132, name: "tin_ore", shape: Shape::Cube, hardness: 2.0, faces: [196,196,196], dir_faces: [196,196,196,196], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("tin_ore") },
    Block { id: 133, name: "iron_ore", shape: Shape::Cube, hardness: 2.4, faces: [200,200,200], dir_faces: [200,200,200,200], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("iron_ore") },
    Block { id: 134, name: "clay", shape: Shape::Cube, hardness: 0.55, faces: [208,208,208], dir_faces: [208,208,208,208], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("clay") },
    Block { id: 135, name: "stone_bricks", shape: Shape::Cube, hardness: 1.6, faces: [212,212,212], dir_faces: [212,212,212,212], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("stone_bricks") },
    Block { id: 136, name: "clay_bricks", shape: Shape::Cube, hardness: 1.5, faces: [216,216,216], dir_faces: [216,216,216,216], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("clay_bricks") },
    Block { id: 137, name: "cobblestone", shape: Shape::Cube, hardness: 1.5, faces: [232,232,232], dir_faces: [232,232,232,232], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("cobblestone") },
    Block { id: 138, name: "thatch", shape: Shape::Cube, hardness: 0.35, faces: [220,220,220], dir_faces: [220,220,220,220], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "foliage", drop: Some("thatch") },
    Block { id: 139, name: "birch_planks", shape: Shape::Cube, hardness: 0.7, faces: [224,224,224], dir_faces: [224,224,224,224], grain: [0,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("birch_planks") },
    Block { id: 140, name: "spruce_planks", shape: Shape::Cube, hardness: 0.8, faces: [228,228,228], dir_faces: [228,228,228,228], grain: [0,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("spruce_planks") },
    Block { id: 141, name: "farmland", shape: Shape::Cube, hardness: 0.45, faces: [236,8,8], dir_faces: [8,236,8,8], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("dirt") },
    Block { id: 142, name: "furnace", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 7, tint: [1.0,1.0,1.0], model: Some("furnace"), cat: "rock", drop: Some("furnace") },
    Block { id: 143, name: "anvil", shape: Shape::Deco, hardness: 2.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("anvil"), cat: "rock", drop: Some("anvil") },
    Block { id: 144, name: "mason_bench", shape: Shape::Deco, hardness: 1.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("mason_bench"), cat: "wood", drop: Some("mason_bench") },
    Block { id: 145, name: "chest", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chest"), cat: "wood", drop: Some("chest") },
    Block { id: 146, name: "bedroll", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bedroll"), cat: "decor", drop: Some("bedroll") },
    Block { id: 147, name: "bed", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed"), cat: "wood", drop: Some("bed") },
    Block { id: 148, name: "torch", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("torch"), cat: "light", drop: Some("torch") },
    Block { id: 149, name: "campfire_out", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("campfire"), cat: "light", drop: Some("campfire") },
    Block { id: 150, name: "wheat_0", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wheat"), cat: "decor", drop: Some("seeds") },
    Block { id: 151, name: "wheat_1", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wheat"), cat: "decor", drop: Some("seeds") },
    Block { id: 152, name: "wheat_2", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wheat"), cat: "decor", drop: Some("seeds") },
    Block { id: 153, name: "wheat_3", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wheat"), cat: "decor", drop: Some("wheat") },
    Block { id: 154, name: "planks_slab", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("planks_slab") },
    Block { id: 155, name: "planks_slab_top", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("planks_slab") },
    Block { id: 156, name: "stone_bricks_slab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_bricks_slab") },
    Block { id: 157, name: "stone_bricks_slab_top", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_bricks_slab") },
    Block { id: 158, name: "cobblestone_slab", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("cobblestone_slab") },
    Block { id: 159, name: "cobblestone_slab_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("cobblestone_slab") },
    Block { id: 160, name: "planks_stairs", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 161, name: "planks_stairs_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 162, name: "planks_stairs_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 163, name: "planks_stairs_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 164, name: "stone_bricks_stairs", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 165, name: "stone_bricks_stairs_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 166, name: "stone_bricks_stairs_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 167, name: "stone_bricks_stairs_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 168, name: "cobblestone_stairs", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 169, name: "cobblestone_stairs_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 170, name: "cobblestone_stairs_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 171, name: "cobblestone_stairs_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 172, name: "thatch_stairs", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 173, name: "thatch_stairs_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 174, name: "thatch_stairs_2", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 175, name: "thatch_stairs_3", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 176, name: "door", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: Some("door") },
    Block { id: 177, name: "door_001", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: None },
    Block { id: 178, name: "door_010", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: Some("door") },
    Block { id: 179, name: "door_011", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: None },
    Block { id: 180, name: "door_100", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: Some("door") },
    Block { id: 181, name: "door_101", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: None },
    Block { id: 182, name: "door_110", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: Some("door") },
    Block { id: 183, name: "door_111", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: None },
    Block { id: 184, name: "door_200", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: Some("door") },
    Block { id: 185, name: "door_201", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: None },
    Block { id: 186, name: "door_210", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: Some("door") },
    Block { id: 187, name: "door_211", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: None },
    Block { id: 188, name: "door_300", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: Some("door") },
    Block { id: 189, name: "door_301", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: None },
    Block { id: 190, name: "door_310", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: Some("door") },
    Block { id: 191, name: "door_311", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("door"), cat: "wood", drop: None },
    Block { id: 192, name: "trapdoor", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 193, name: "trapdoor_open", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 194, name: "trapdoor_open_1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 195, name: "trapdoor_open_2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 196, name: "trapdoor_open_3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 197, name: "ladder", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("ladder"), cat: "wood", drop: Some("ladder") },
    Block { id: 198, name: "ladder_1", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("ladder"), cat: "wood", drop: Some("ladder") },
    Block { id: 199, name: "ladder_2", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("ladder"), cat: "wood", drop: Some("ladder") },
    Block { id: 200, name: "ladder_3", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("ladder"), cat: "wood", drop: Some("ladder") },
    Block { id: 201, name: "window", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("window"), cat: "wood", drop: Some("window") },
    Block { id: 202, name: "window_x", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("window"), cat: "wood", drop: Some("window") },
    Block { id: 203, name: "wood_pillar", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("pillar"), cat: "wood", drop: Some("wood_pillar") },
    Block { id: 204, name: "stone_pillar", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("pillar"), cat: "rock", drop: Some("stone_pillar") },
    Block { id: 205, name: "stone_wall", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall"), cat: "rock", drop: Some("stone_wall") },
    Block { id: 206, name: "stone_wall_x", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall"), cat: "rock", drop: Some("stone_wall") },
    Block { id: 207, name: "carrot_0", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carrot"), cat: "decor", drop: Some("carrot") },
    Block { id: 208, name: "carrot_1", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carrot"), cat: "decor", drop: Some("carrot") },
    Block { id: 209, name: "carrot_2", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carrot"), cat: "decor", drop: Some("carrot") },
    Block { id: 210, name: "carrot_3", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carrot"), cat: "decor", drop: Some("carrot") },
    Block { id: 211, name: "potato_0", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("potato"), cat: "decor", drop: Some("potato") },
    Block { id: 212, name: "potato_1", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("potato"), cat: "decor", drop: Some("potato") },
    Block { id: 213, name: "potato_2", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("potato"), cat: "decor", drop: Some("potato") },
    Block { id: 214, name: "potato_3", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("potato"), cat: "decor", drop: Some("potato") },
    Block { id: 215, name: "planks_stairs_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 216, name: "planks_stairs_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 217, name: "planks_stairs_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 218, name: "planks_stairs_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 219, name: "stone_bricks_stairs_top", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 220, name: "stone_bricks_stairs_top_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 221, name: "stone_bricks_stairs_top_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 222, name: "stone_bricks_stairs_top_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 223, name: "cobblestone_stairs_top", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 224, name: "cobblestone_stairs_top_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 225, name: "cobblestone_stairs_top_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 226, name: "cobblestone_stairs_top_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 227, name: "thatch_stairs_top", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 228, name: "thatch_stairs_top_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 229, name: "thatch_stairs_top_2", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 230, name: "thatch_stairs_top_3", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 231, name: "planks_vslab", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("planks_vslab") },
    Block { id: 232, name: "planks_vslab_1", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("planks_vslab") },
    Block { id: 233, name: "planks_vslab_2", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("planks_vslab") },
    Block { id: 234, name: "planks_vslab_3", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("planks_vslab") },
    Block { id: 235, name: "stone_bricks_vslab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_bricks_vslab") },
    Block { id: 236, name: "stone_bricks_vslab_1", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_bricks_vslab") },
    Block { id: 237, name: "stone_bricks_vslab_2", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_bricks_vslab") },
    Block { id: 238, name: "stone_bricks_vslab_3", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_bricks_vslab") },
    Block { id: 239, name: "cobblestone_vslab", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("cobblestone_vslab") },
    Block { id: 240, name: "cobblestone_vslab_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("cobblestone_vslab") },
    Block { id: 241, name: "cobblestone_vslab_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("cobblestone_vslab") },
    Block { id: 242, name: "cobblestone_vslab_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("cobblestone_vslab") },
    Block { id: 243, name: "shutter", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 244, name: "shutter_1", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 245, name: "shutter_2", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 246, name: "shutter_3", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 247, name: "shutter_open", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 248, name: "shutter_open_1", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 249, name: "shutter_open_2", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 250, name: "shutter_open_3", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
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
    pub const CAMPFIRE: u8 = 124;
    pub const WORKBENCH: u8 = 125;
    pub const TWIGS: u8 = 126;
    pub const PEBBLES: u8 = 127;
    pub const BLUEBERRY_PICKED: u8 = 128;
    pub const ROWAN_PICKED: u8 = 129;
    pub const COAL_ORE: u8 = 130;
    pub const COPPER_ORE: u8 = 131;
    pub const TIN_ORE: u8 = 132;
    pub const IRON_ORE: u8 = 133;
    pub const CLAY: u8 = 134;
    pub const STONE_BRICKS: u8 = 135;
    pub const CLAY_BRICKS: u8 = 136;
    pub const COBBLESTONE: u8 = 137;
    pub const THATCH: u8 = 138;
    pub const BIRCH_PLANKS: u8 = 139;
    pub const SPRUCE_PLANKS: u8 = 140;
    pub const FARMLAND: u8 = 141;
    pub const FURNACE: u8 = 142;
    pub const ANVIL: u8 = 143;
    pub const MASON_BENCH: u8 = 144;
    pub const CHEST: u8 = 145;
    pub const BEDROLL: u8 = 146;
    pub const BED: u8 = 147;
    pub const TORCH: u8 = 148;
    pub const CAMPFIRE_OUT: u8 = 149;
    pub const WHEAT_0: u8 = 150;
    pub const WHEAT_1: u8 = 151;
    pub const WHEAT_2: u8 = 152;
    pub const WHEAT_3: u8 = 153;
    pub const PLANKS_SLAB: u8 = 154;
    pub const PLANKS_SLAB_TOP: u8 = 155;
    pub const STONE_BRICKS_SLAB: u8 = 156;
    pub const STONE_BRICKS_SLAB_TOP: u8 = 157;
    pub const COBBLESTONE_SLAB: u8 = 158;
    pub const COBBLESTONE_SLAB_TOP: u8 = 159;
    pub const PLANKS_STAIRS: u8 = 160;
    pub const PLANKS_STAIRS_1: u8 = 161;
    pub const PLANKS_STAIRS_2: u8 = 162;
    pub const PLANKS_STAIRS_3: u8 = 163;
    pub const STONE_BRICKS_STAIRS: u8 = 164;
    pub const STONE_BRICKS_STAIRS_1: u8 = 165;
    pub const STONE_BRICKS_STAIRS_2: u8 = 166;
    pub const STONE_BRICKS_STAIRS_3: u8 = 167;
    pub const COBBLESTONE_STAIRS: u8 = 168;
    pub const COBBLESTONE_STAIRS_1: u8 = 169;
    pub const COBBLESTONE_STAIRS_2: u8 = 170;
    pub const COBBLESTONE_STAIRS_3: u8 = 171;
    pub const THATCH_STAIRS: u8 = 172;
    pub const THATCH_STAIRS_1: u8 = 173;
    pub const THATCH_STAIRS_2: u8 = 174;
    pub const THATCH_STAIRS_3: u8 = 175;
    pub const DOOR: u8 = 176;
    pub const DOOR_001: u8 = 177;
    pub const DOOR_010: u8 = 178;
    pub const DOOR_011: u8 = 179;
    pub const DOOR_100: u8 = 180;
    pub const DOOR_101: u8 = 181;
    pub const DOOR_110: u8 = 182;
    pub const DOOR_111: u8 = 183;
    pub const DOOR_200: u8 = 184;
    pub const DOOR_201: u8 = 185;
    pub const DOOR_210: u8 = 186;
    pub const DOOR_211: u8 = 187;
    pub const DOOR_300: u8 = 188;
    pub const DOOR_301: u8 = 189;
    pub const DOOR_310: u8 = 190;
    pub const DOOR_311: u8 = 191;
    pub const TRAPDOOR: u8 = 192;
    pub const TRAPDOOR_OPEN: u8 = 193;
    pub const TRAPDOOR_OPEN_1: u8 = 194;
    pub const TRAPDOOR_OPEN_2: u8 = 195;
    pub const TRAPDOOR_OPEN_3: u8 = 196;
    pub const LADDER: u8 = 197;
    pub const LADDER_1: u8 = 198;
    pub const LADDER_2: u8 = 199;
    pub const LADDER_3: u8 = 200;
    pub const WINDOW: u8 = 201;
    pub const WINDOW_X: u8 = 202;
    pub const WOOD_PILLAR: u8 = 203;
    pub const STONE_PILLAR: u8 = 204;
    pub const STONE_WALL: u8 = 205;
    pub const STONE_WALL_X: u8 = 206;
    pub const CARROT_0: u8 = 207;
    pub const CARROT_1: u8 = 208;
    pub const CARROT_2: u8 = 209;
    pub const CARROT_3: u8 = 210;
    pub const POTATO_0: u8 = 211;
    pub const POTATO_1: u8 = 212;
    pub const POTATO_2: u8 = 213;
    pub const POTATO_3: u8 = 214;
    pub const PLANKS_STAIRS_TOP: u8 = 215;
    pub const PLANKS_STAIRS_TOP_1: u8 = 216;
    pub const PLANKS_STAIRS_TOP_2: u8 = 217;
    pub const PLANKS_STAIRS_TOP_3: u8 = 218;
    pub const STONE_BRICKS_STAIRS_TOP: u8 = 219;
    pub const STONE_BRICKS_STAIRS_TOP_1: u8 = 220;
    pub const STONE_BRICKS_STAIRS_TOP_2: u8 = 221;
    pub const STONE_BRICKS_STAIRS_TOP_3: u8 = 222;
    pub const COBBLESTONE_STAIRS_TOP: u8 = 223;
    pub const COBBLESTONE_STAIRS_TOP_1: u8 = 224;
    pub const COBBLESTONE_STAIRS_TOP_2: u8 = 225;
    pub const COBBLESTONE_STAIRS_TOP_3: u8 = 226;
    pub const THATCH_STAIRS_TOP: u8 = 227;
    pub const THATCH_STAIRS_TOP_1: u8 = 228;
    pub const THATCH_STAIRS_TOP_2: u8 = 229;
    pub const THATCH_STAIRS_TOP_3: u8 = 230;
    pub const PLANKS_VSLAB: u8 = 231;
    pub const PLANKS_VSLAB_1: u8 = 232;
    pub const PLANKS_VSLAB_2: u8 = 233;
    pub const PLANKS_VSLAB_3: u8 = 234;
    pub const STONE_BRICKS_VSLAB: u8 = 235;
    pub const STONE_BRICKS_VSLAB_1: u8 = 236;
    pub const STONE_BRICKS_VSLAB_2: u8 = 237;
    pub const STONE_BRICKS_VSLAB_3: u8 = 238;
    pub const COBBLESTONE_VSLAB: u8 = 239;
    pub const COBBLESTONE_VSLAB_1: u8 = 240;
    pub const COBBLESTONE_VSLAB_2: u8 = 241;
    pub const COBBLESTONE_VSLAB_3: u8 = 242;
    pub const SHUTTER: u8 = 243;
    pub const SHUTTER_1: u8 = 244;
    pub const SHUTTER_2: u8 = 245;
    pub const SHUTTER_3: u8 = 246;
    pub const SHUTTER_OPEN: u8 = 247;
    pub const SHUTTER_OPEN_1: u8 = 248;
    pub const SHUTTER_OPEN_2: u8 = 249;
    pub const SHUTTER_OPEN_3: u8 = 250;
}
