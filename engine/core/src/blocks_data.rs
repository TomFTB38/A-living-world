// Généré par engine/tools/gen_blocks.mjs. Les identifiants sont stables : ils vivent dans les sauvegardes.
use crate::blocks::{Block, Shape};

pub const VARIANTS: u32 = 4;
pub const MATERIALS: [&str; 86] = ["grass_top", "grass_side", "dirt", "path", "stone", "mossy_stone", "limestone", "granite", "gravel", "sand", "bark_oak", "bark_birch", "bark_beech", "leaf_oak", "leaf_birch", "leaf_beech", "planks", "forest_soil", "water_foam", "end_oak", "end_birch", "end_beech", "snow", "snow_side", "ice", "bark_spruce", "end_spruce", "leaf_spruce", "leaf_snow", "grass_dry_top", "grass_dry_side", "dry_earth", "bark_dead", "end_dead", "lichen", "bark_acacia", "end_acacia", "leaf_acacia", "bone", "wood_inner", "sandstone", "sandstone_top", "red_sandstone", "cactus_side", "cactus_top", "bark_palm", "end_palm", "leaf_palm", "ore_copper", "ore_tin", "ore_iron", "ore_coal", "clay", "stone_bricks", "bricks", "thatch", "planks_birch", "planks_spruce", "cobble", "farmland_top", "plaster", "dark_timber", "roof_tiles", "slate", "marble", "ashlar", "fieldstone", "setts", "polished_stone", "checker", "terracotta", "parquet", "shingles", "wool", "rug", "frame_square", "frame_cross", "frame_diag", "wattle", "books", "hay", "crate", "chiseled", "stained_glass", "iron_plate", "verdigris"];

pub const BLOCKS: [Block; 560] = [
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
    Block { id: 251, name: "plaster", shape: Shape::Cube, hardness: 0.7, faces: [240,240,240], dir_faces: [240,240,240,240], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("plaster") },
    Block { id: 252, name: "timber", shape: Shape::Cube, hardness: 0.9, faces: [244,244,244], dir_faces: [244,244,244,244], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("timber") },
    Block { id: 253, name: "roof_tiles", shape: Shape::Cube, hardness: 0.9, faces: [248,248,248], dir_faces: [248,248,248,248], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("roof_tiles") },
    Block { id: 254, name: "slate", shape: Shape::Cube, hardness: 1.1, faces: [252,252,252], dir_faces: [252,252,252,252], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("slate") },
    Block { id: 255, name: "barrel", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("barrel"), cat: "wood", drop: Some("barrel") },
    Block { id: 256, name: "marble", shape: Shape::Cube, hardness: 1.8, faces: [256,256,256], dir_faces: [256,256,256,256], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("marble") },
    Block { id: 257, name: "ashlar", shape: Shape::Cube, hardness: 1.6, faces: [260,260,260], dir_faces: [260,260,260,260], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("ashlar") },
    Block { id: 258, name: "fieldstone", shape: Shape::Cube, hardness: 1.5, faces: [264,264,264], dir_faces: [264,264,264,264], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("fieldstone") },
    Block { id: 259, name: "setts", shape: Shape::Cube, hardness: 1.5, faces: [268,268,268], dir_faces: [268,268,268,268], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("setts") },
    Block { id: 260, name: "polished_stone", shape: Shape::Cube, hardness: 1.6, faces: [272,272,272], dir_faces: [272,272,272,272], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("polished_stone") },
    Block { id: 261, name: "shingles", shape: Shape::Cube, hardness: 0.8, faces: [288,288,288], dir_faces: [288,288,288,288], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("shingles") },
    Block { id: 262, name: "copper_roof", shape: Shape::Cube, hardness: 1.4, faces: [340,340,340], dir_faces: [340,340,340,340], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("copper_roof") },
    Block { id: 263, name: "checker_floor", shape: Shape::Cube, hardness: 1.6, faces: [276,276,276], dir_faces: [276,276,276,276], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("checker_floor") },
    Block { id: 264, name: "terracotta_floor", shape: Shape::Cube, hardness: 1.0, faces: [280,280,280], dir_faces: [280,280,280,280], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("terracotta_floor") },
    Block { id: 265, name: "parquet", shape: Shape::Cube, hardness: 0.8, faces: [284,284,284], dir_faces: [284,284,284,284], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("parquet") },
    Block { id: 266, name: "chiseled_stone", shape: Shape::Cube, hardness: 1.6, faces: [328,328,328], dir_faces: [328,328,328,328], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("chiseled_stone") },
    Block { id: 267, name: "stained_glass", shape: Shape::Cube, hardness: 0.5, faces: [332,332,332], dir_faces: [332,332,332,332], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("stained_glass") },
    Block { id: 268, name: "iron_block", shape: Shape::Cube, hardness: 2.5, faces: [336,336,336], dir_faces: [336,336,336,336], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("iron_block") },
    Block { id: 269, name: "marble_black", shape: Shape::Cube, hardness: 1.8, faces: [256,256,256], dir_faces: [256,256,256,256], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.3,0.3,0.32], model: None, cat: "rock", drop: Some("marble_black") },
    Block { id: 270, name: "bricks_dark", shape: Shape::Cube, hardness: 1.5, faces: [216,216,216], dir_faces: [216,216,216,216], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.55,0.5,0.48], model: None, cat: "rock", drop: Some("bricks_dark") },
    Block { id: 271, name: "stone_bricks_dark", shape: Shape::Cube, hardness: 1.6, faces: [212,212,212], dir_faces: [212,212,212,212], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.62,0.62,0.66], model: None, cat: "rock", drop: Some("stone_bricks_dark") },
    Block { id: 272, name: "timber_frame", shape: Shape::Cube, hardness: 0.9, faces: [300,300,300], dir_faces: [300,300,300,300], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("timber_frame") },
    Block { id: 273, name: "timber_cross", shape: Shape::Cube, hardness: 0.9, faces: [304,304,304], dir_faces: [304,304,304,304], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("timber_cross") },
    Block { id: 274, name: "timber_brace", shape: Shape::Cube, hardness: 0.9, faces: [308,308,308], dir_faces: [308,308,308,308], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("timber_brace") },
    Block { id: 275, name: "wattle", shape: Shape::Cube, hardness: 0.5, faces: [312,312,312], dir_faces: [312,312,312,312], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("wattle") },
    Block { id: 276, name: "bookshelf", shape: Shape::Cube, hardness: 0.8, faces: [64,316,64], dir_faces: [316,64,64,316], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("bookshelf") },
    Block { id: 277, name: "hay_bale", shape: Shape::Cube, hardness: 0.4, faces: [320,320,320], dir_faces: [320,320,320,320], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "foliage", drop: Some("hay_bale") },
    Block { id: 278, name: "crate", shape: Shape::Cube, hardness: 0.7, faces: [324,324,324], dir_faces: [324,324,324,324], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("crate") },
    Block { id: 279, name: "planks_dark", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.55,0.45,0.4], model: None, cat: "wood", drop: Some("planks_dark") },
    Block { id: 280, name: "planks_red", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.95,0.5,0.42], model: None, cat: "wood", drop: Some("planks_red") },
    Block { id: 281, name: "planks_white", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.45,1.4,1.3], model: None, cat: "wood", drop: Some("planks_white") },
    Block { id: 282, name: "planks_blue", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.6,0.78,1.0], model: None, cat: "wood", drop: Some("planks_blue") },
    Block { id: 283, name: "planks_green", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.62,0.85,0.58], model: None, cat: "wood", drop: Some("planks_green") },
    Block { id: 284, name: "plaster_ochre", shape: Shape::Cube, hardness: 0.7, faces: [240,240,240], dir_faces: [240,240,240,240], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,0.82,0.55], model: None, cat: "rock", drop: Some("plaster_ochre") },
    Block { id: 285, name: "plaster_rose", shape: Shape::Cube, hardness: 0.7, faces: [240,240,240], dir_faces: [240,240,240,240], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,0.8,0.76], model: None, cat: "rock", drop: Some("plaster_rose") },
    Block { id: 286, name: "plaster_blue", shape: Shape::Cube, hardness: 0.7, faces: [240,240,240], dir_faces: [240,240,240,240], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.82,0.9,1.02], model: None, cat: "rock", drop: Some("plaster_blue") },
    Block { id: 287, name: "wool_white", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "foliage", drop: Some("wool_white") },
    Block { id: 288, name: "wool_red", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.75,0.2,0.18], model: None, cat: "foliage", drop: Some("wool_red") },
    Block { id: 289, name: "wool_blue", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.28,0.4,0.8], model: None, cat: "foliage", drop: Some("wool_blue") },
    Block { id: 290, name: "wool_green", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.35,0.6,0.3], model: None, cat: "foliage", drop: Some("wool_green") },
    Block { id: 291, name: "wool_yellow", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,0.85,0.3], model: None, cat: "foliage", drop: Some("wool_yellow") },
    Block { id: 292, name: "wool_black", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.22,0.22,0.24], model: None, cat: "foliage", drop: Some("wool_black") },
    Block { id: 293, name: "wool_brown", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.55,0.38,0.25], model: None, cat: "foliage", drop: Some("wool_brown") },
    Block { id: 294, name: "wool_purple", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.55,0.3,0.65], model: None, cat: "foliage", drop: Some("wool_purple") },
    Block { id: 295, name: "wool_orange", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,0.55,0.22], model: None, cat: "foliage", drop: Some("wool_orange") },
    Block { id: 296, name: "marble_stairs", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 297, name: "marble_stairs_1", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 298, name: "marble_stairs_2", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 299, name: "marble_stairs_3", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 300, name: "marble_stairs_top", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 301, name: "marble_stairs_top_1", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 302, name: "marble_stairs_top_2", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 303, name: "marble_stairs_top_3", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 304, name: "marble_slab", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_slab") },
    Block { id: 305, name: "marble_slab_top", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_slab") },
    Block { id: 306, name: "marble_vslab", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_vslab") },
    Block { id: 307, name: "marble_vslab_1", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_vslab") },
    Block { id: 308, name: "marble_vslab_2", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_vslab") },
    Block { id: 309, name: "marble_vslab_3", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_vslab") },
    Block { id: 310, name: "ashlar_stairs", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 311, name: "ashlar_stairs_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 312, name: "ashlar_stairs_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 313, name: "ashlar_stairs_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 314, name: "ashlar_stairs_top", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 315, name: "ashlar_stairs_top_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 316, name: "ashlar_stairs_top_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 317, name: "ashlar_stairs_top_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 318, name: "ashlar_slab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_slab") },
    Block { id: 319, name: "ashlar_slab_top", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_slab") },
    Block { id: 320, name: "ashlar_vslab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_vslab") },
    Block { id: 321, name: "ashlar_vslab_1", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_vslab") },
    Block { id: 322, name: "ashlar_vslab_2", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_vslab") },
    Block { id: 323, name: "ashlar_vslab_3", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_vslab") },
    Block { id: 324, name: "fieldstone_stairs", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 325, name: "fieldstone_stairs_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 326, name: "fieldstone_stairs_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 327, name: "fieldstone_stairs_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 328, name: "fieldstone_stairs_top", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 329, name: "fieldstone_stairs_top_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 330, name: "fieldstone_stairs_top_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 331, name: "fieldstone_stairs_top_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 332, name: "fieldstone_slab", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_slab") },
    Block { id: 333, name: "fieldstone_slab_top", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_slab") },
    Block { id: 334, name: "fieldstone_vslab", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_vslab") },
    Block { id: 335, name: "fieldstone_vslab_1", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_vslab") },
    Block { id: 336, name: "fieldstone_vslab_2", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_vslab") },
    Block { id: 337, name: "fieldstone_vslab_3", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_vslab") },
    Block { id: 338, name: "setts_stairs", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 339, name: "setts_stairs_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 340, name: "setts_stairs_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 341, name: "setts_stairs_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 342, name: "setts_stairs_top", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 343, name: "setts_stairs_top_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 344, name: "setts_stairs_top_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 345, name: "setts_stairs_top_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 346, name: "setts_slab", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_slab") },
    Block { id: 347, name: "setts_slab_top", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_slab") },
    Block { id: 348, name: "setts_vslab", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_vslab") },
    Block { id: 349, name: "setts_vslab_1", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_vslab") },
    Block { id: 350, name: "setts_vslab_2", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_vslab") },
    Block { id: 351, name: "setts_vslab_3", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_vslab") },
    Block { id: 352, name: "polished_stone_stairs", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 353, name: "polished_stone_stairs_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 354, name: "polished_stone_stairs_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 355, name: "polished_stone_stairs_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 356, name: "polished_stone_stairs_top", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 357, name: "polished_stone_stairs_top_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 358, name: "polished_stone_stairs_top_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 359, name: "polished_stone_stairs_top_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 360, name: "polished_stone_slab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_slab") },
    Block { id: 361, name: "polished_stone_slab_top", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_slab") },
    Block { id: 362, name: "polished_stone_vslab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_vslab") },
    Block { id: 363, name: "polished_stone_vslab_1", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_vslab") },
    Block { id: 364, name: "polished_stone_vslab_2", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_vslab") },
    Block { id: 365, name: "polished_stone_vslab_3", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_vslab") },
    Block { id: 366, name: "shingles_stairs", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 367, name: "shingles_stairs_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 368, name: "shingles_stairs_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 369, name: "shingles_stairs_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 370, name: "shingles_stairs_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 371, name: "shingles_stairs_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 372, name: "shingles_stairs_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 373, name: "shingles_stairs_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 374, name: "shingles_slab", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_slab") },
    Block { id: 375, name: "shingles_slab_top", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_slab") },
    Block { id: 376, name: "shingles_vslab", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_vslab") },
    Block { id: 377, name: "shingles_vslab_1", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_vslab") },
    Block { id: 378, name: "shingles_vslab_2", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_vslab") },
    Block { id: 379, name: "shingles_vslab_3", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_vslab") },
    Block { id: 380, name: "roof_tiles_stairs", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 381, name: "roof_tiles_stairs_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 382, name: "roof_tiles_stairs_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 383, name: "roof_tiles_stairs_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 384, name: "roof_tiles_stairs_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 385, name: "roof_tiles_stairs_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 386, name: "roof_tiles_stairs_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 387, name: "roof_tiles_stairs_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 388, name: "roof_tiles_slab", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_slab") },
    Block { id: 389, name: "roof_tiles_slab_top", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_slab") },
    Block { id: 390, name: "roof_tiles_vslab", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_vslab") },
    Block { id: 391, name: "roof_tiles_vslab_1", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_vslab") },
    Block { id: 392, name: "roof_tiles_vslab_2", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_vslab") },
    Block { id: 393, name: "roof_tiles_vslab_3", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_vslab") },
    Block { id: 394, name: "slate_stairs", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 395, name: "slate_stairs_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 396, name: "slate_stairs_2", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 397, name: "slate_stairs_3", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 398, name: "slate_stairs_top", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 399, name: "slate_stairs_top_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 400, name: "slate_stairs_top_2", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 401, name: "slate_stairs_top_3", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 402, name: "slate_slab", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_slab") },
    Block { id: 403, name: "slate_slab_top", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_slab") },
    Block { id: 404, name: "slate_vslab", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_vslab") },
    Block { id: 405, name: "slate_vslab_1", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_vslab") },
    Block { id: 406, name: "slate_vslab_2", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_vslab") },
    Block { id: 407, name: "slate_vslab_3", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_vslab") },
    Block { id: 408, name: "copper_roof_stairs", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 409, name: "copper_roof_stairs_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 410, name: "copper_roof_stairs_2", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 411, name: "copper_roof_stairs_3", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 412, name: "copper_roof_stairs_top", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 413, name: "copper_roof_stairs_top_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 414, name: "copper_roof_stairs_top_2", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 415, name: "copper_roof_stairs_top_3", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 416, name: "copper_roof_slab", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_slab") },
    Block { id: 417, name: "copper_roof_slab_top", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_slab") },
    Block { id: 418, name: "copper_roof_vslab", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_vslab") },
    Block { id: 419, name: "copper_roof_vslab_1", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_vslab") },
    Block { id: 420, name: "copper_roof_vslab_2", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_vslab") },
    Block { id: 421, name: "copper_roof_vslab_3", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_vslab") },
    Block { id: 422, name: "plaster_stairs", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 423, name: "plaster_stairs_1", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 424, name: "plaster_stairs_2", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 425, name: "plaster_stairs_3", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 426, name: "plaster_stairs_top", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 427, name: "plaster_stairs_top_1", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 428, name: "plaster_stairs_top_2", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 429, name: "plaster_stairs_top_3", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 430, name: "plaster_slab", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_slab") },
    Block { id: 431, name: "plaster_slab_top", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_slab") },
    Block { id: 432, name: "plaster_vslab", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_vslab") },
    Block { id: 433, name: "plaster_vslab_1", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_vslab") },
    Block { id: 434, name: "plaster_vslab_2", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_vslab") },
    Block { id: 435, name: "plaster_vslab_3", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_vslab") },
    Block { id: 436, name: "timber_stairs", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 437, name: "timber_stairs_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 438, name: "timber_stairs_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 439, name: "timber_stairs_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 440, name: "timber_stairs_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 441, name: "timber_stairs_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 442, name: "timber_stairs_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 443, name: "timber_stairs_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 444, name: "timber_slab", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_slab") },
    Block { id: 445, name: "timber_slab_top", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_slab") },
    Block { id: 446, name: "timber_vslab", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_vslab") },
    Block { id: 447, name: "timber_vslab_1", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_vslab") },
    Block { id: 448, name: "timber_vslab_2", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_vslab") },
    Block { id: 449, name: "timber_vslab_3", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_vslab") },
    Block { id: 450, name: "chair", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chair"), cat: "wood", drop: Some("chair") },
    Block { id: 451, name: "chair@1", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chair"), cat: "wood", drop: Some("chair") },
    Block { id: 452, name: "chair@2", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chair"), cat: "wood", drop: Some("chair") },
    Block { id: 453, name: "chair@3", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chair"), cat: "wood", drop: Some("chair") },
    Block { id: 454, name: "bench", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bench"), cat: "wood", drop: Some("bench") },
    Block { id: 455, name: "bench@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bench"), cat: "wood", drop: Some("bench") },
    Block { id: 456, name: "bench@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bench"), cat: "wood", drop: Some("bench") },
    Block { id: 457, name: "bench@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bench"), cat: "wood", drop: Some("bench") },
    Block { id: 458, name: "armchair", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("armchair"), cat: "wood", drop: Some("armchair") },
    Block { id: 459, name: "armchair@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("armchair"), cat: "wood", drop: Some("armchair") },
    Block { id: 460, name: "armchair@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("armchair"), cat: "wood", drop: Some("armchair") },
    Block { id: 461, name: "armchair@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("armchair"), cat: "wood", drop: Some("armchair") },
    Block { id: 462, name: "throne", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("throne"), cat: "wood", drop: Some("throne") },
    Block { id: 463, name: "throne@1", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("throne"), cat: "wood", drop: Some("throne") },
    Block { id: 464, name: "throne@2", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("throne"), cat: "wood", drop: Some("throne") },
    Block { id: 465, name: "throne@3", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("throne"), cat: "wood", drop: Some("throne") },
    Block { id: 466, name: "cupboard", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cupboard"), cat: "wood", drop: Some("cupboard") },
    Block { id: 467, name: "cupboard@1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cupboard"), cat: "wood", drop: Some("cupboard") },
    Block { id: 468, name: "cupboard@2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cupboard"), cat: "wood", drop: Some("cupboard") },
    Block { id: 469, name: "cupboard@3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cupboard"), cat: "wood", drop: Some("cupboard") },
    Block { id: 470, name: "fireplace", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("fireplace"), cat: "wood", drop: Some("fireplace") },
    Block { id: 471, name: "fireplace@1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("fireplace"), cat: "wood", drop: Some("fireplace") },
    Block { id: 472, name: "fireplace@2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("fireplace"), cat: "wood", drop: Some("fireplace") },
    Block { id: 473, name: "fireplace@3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("fireplace"), cat: "wood", drop: Some("fireplace") },
    Block { id: 474, name: "loom", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("loom"), cat: "wood", drop: Some("loom") },
    Block { id: 475, name: "loom@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("loom"), cat: "wood", drop: Some("loom") },
    Block { id: 476, name: "loom@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("loom"), cat: "wood", drop: Some("loom") },
    Block { id: 477, name: "loom@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("loom"), cat: "wood", drop: Some("loom") },
    Block { id: 478, name: "spinning_wheel", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("spinning_wheel"), cat: "wood", drop: Some("spinning_wheel") },
    Block { id: 479, name: "spinning_wheel@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("spinning_wheel"), cat: "wood", drop: Some("spinning_wheel") },
    Block { id: 480, name: "spinning_wheel@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("spinning_wheel"), cat: "wood", drop: Some("spinning_wheel") },
    Block { id: 481, name: "spinning_wheel@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("spinning_wheel"), cat: "wood", drop: Some("spinning_wheel") },
    Block { id: 482, name: "grindstone", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grindstone"), cat: "wood", drop: Some("grindstone") },
    Block { id: 483, name: "grindstone@1", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grindstone"), cat: "wood", drop: Some("grindstone") },
    Block { id: 484, name: "grindstone@2", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grindstone"), cat: "wood", drop: Some("grindstone") },
    Block { id: 485, name: "grindstone@3", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grindstone"), cat: "wood", drop: Some("grindstone") },
    Block { id: 486, name: "signpost", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("signpost"), cat: "wood", drop: Some("signpost") },
    Block { id: 487, name: "signpost@1", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("signpost"), cat: "wood", drop: Some("signpost") },
    Block { id: 488, name: "signpost@2", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("signpost"), cat: "wood", drop: Some("signpost") },
    Block { id: 489, name: "signpost@3", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("signpost"), cat: "wood", drop: Some("signpost") },
    Block { id: 490, name: "painting", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("painting"), cat: "wood", drop: Some("painting") },
    Block { id: 491, name: "painting@1", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("painting"), cat: "wood", drop: Some("painting") },
    Block { id: 492, name: "painting@2", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("painting"), cat: "wood", drop: Some("painting") },
    Block { id: 493, name: "painting@3", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("painting"), cat: "wood", drop: Some("painting") },
    Block { id: 494, name: "banner_red", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_red"), cat: "wood", drop: Some("banner_red") },
    Block { id: 495, name: "banner_red@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_red"), cat: "wood", drop: Some("banner_red") },
    Block { id: 496, name: "banner_red@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_red"), cat: "wood", drop: Some("banner_red") },
    Block { id: 497, name: "banner_red@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_red"), cat: "wood", drop: Some("banner_red") },
    Block { id: 498, name: "banner_blue", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_blue"), cat: "wood", drop: Some("banner_blue") },
    Block { id: 499, name: "banner_blue@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_blue"), cat: "wood", drop: Some("banner_blue") },
    Block { id: 500, name: "banner_blue@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_blue"), cat: "wood", drop: Some("banner_blue") },
    Block { id: 501, name: "banner_blue@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_blue"), cat: "wood", drop: Some("banner_blue") },
    Block { id: 502, name: "banner_green", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_green"), cat: "wood", drop: Some("banner_green") },
    Block { id: 503, name: "banner_green@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_green"), cat: "wood", drop: Some("banner_green") },
    Block { id: 504, name: "banner_green@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_green"), cat: "wood", drop: Some("banner_green") },
    Block { id: 505, name: "banner_green@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_green"), cat: "wood", drop: Some("banner_green") },
    Block { id: 506, name: "banner_gold", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_gold"), cat: "wood", drop: Some("banner_gold") },
    Block { id: 507, name: "banner_gold@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_gold"), cat: "wood", drop: Some("banner_gold") },
    Block { id: 508, name: "banner_gold@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_gold"), cat: "wood", drop: Some("banner_gold") },
    Block { id: 509, name: "banner_gold@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_gold"), cat: "wood", drop: Some("banner_gold") },
    Block { id: 510, name: "weapon_rack", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("weapon_rack"), cat: "wood", drop: Some("weapon_rack") },
    Block { id: 511, name: "weapon_rack@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("weapon_rack"), cat: "wood", drop: Some("weapon_rack") },
    Block { id: 512, name: "weapon_rack@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("weapon_rack"), cat: "wood", drop: Some("weapon_rack") },
    Block { id: 513, name: "weapon_rack@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("weapon_rack"), cat: "wood", drop: Some("weapon_rack") },
    Block { id: 514, name: "wall_sconce", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 11, tint: [1.0,1.0,1.0], model: Some("wall_sconce"), cat: "wood", drop: Some("wall_sconce") },
    Block { id: 515, name: "wall_sconce@1", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 11, tint: [1.0,1.0,1.0], model: Some("wall_sconce"), cat: "wood", drop: Some("wall_sconce") },
    Block { id: 516, name: "wall_sconce@2", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 11, tint: [1.0,1.0,1.0], model: Some("wall_sconce"), cat: "wood", drop: Some("wall_sconce") },
    Block { id: 517, name: "wall_sconce@3", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 11, tint: [1.0,1.0,1.0], model: Some("wall_sconce"), cat: "wood", drop: Some("wall_sconce") },
    Block { id: 518, name: "curtain", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("curtain"), cat: "wood", drop: Some("curtain") },
    Block { id: 519, name: "curtain@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("curtain"), cat: "wood", drop: Some("curtain") },
    Block { id: 520, name: "curtain@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("curtain"), cat: "wood", drop: Some("curtain") },
    Block { id: 521, name: "curtain@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("curtain"), cat: "wood", drop: Some("curtain") },
    Block { id: 522, name: "wall_shelf", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_shelf"), cat: "wood", drop: Some("wall_shelf") },
    Block { id: 523, name: "wall_shelf@1", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_shelf"), cat: "wood", drop: Some("wall_shelf") },
    Block { id: 524, name: "wall_shelf@2", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_shelf"), cat: "wood", drop: Some("wall_shelf") },
    Block { id: 525, name: "wall_shelf@3", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_shelf"), cat: "wood", drop: Some("wall_shelf") },
    Block { id: 526, name: "table", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("table"), cat: "wood", drop: Some("table") },
    Block { id: 527, name: "round_table", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("round_table"), cat: "wood", drop: Some("round_table") },
    Block { id: 528, name: "stool", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stool"), cat: "wood", drop: Some("stool") },
    Block { id: 529, name: "candle", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 9, tint: [1.0,1.0,1.0], model: Some("candle"), cat: "wood", drop: Some("candle") },
    Block { id: 530, name: "candelabra", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 12, tint: [1.0,1.0,1.0], model: Some("candelabra"), cat: "wood", drop: Some("candelabra") },
    Block { id: 531, name: "chandelier", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 14, tint: [1.0,1.0,1.0], model: Some("chandelier"), cat: "wood", drop: Some("chandelier") },
    Block { id: 532, name: "brazier", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 14, tint: [1.0,1.0,1.0], model: Some("brazier"), cat: "wood", drop: Some("brazier") },
    Block { id: 533, name: "sack", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sack"), cat: "wood", drop: Some("sack") },
    Block { id: 534, name: "amphora", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("amphora"), cat: "wood", drop: Some("amphora") },
    Block { id: 535, name: "flower_pot", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("flower_pot"), cat: "wood", drop: Some("flower_pot") },
    Block { id: 536, name: "potted_fern", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("potted_fern"), cat: "wood", drop: Some("potted_fern") },
    Block { id: 537, name: "cauldron", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cauldron"), cat: "wood", drop: Some("cauldron") },
    Block { id: 538, name: "bucket", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bucket"), cat: "wood", drop: Some("bucket") },
    Block { id: 539, name: "log_pile", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("log_pile"), cat: "wood", drop: Some("log_pile") },
    Block { id: 540, name: "carpet_red", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carpet_red"), cat: "wood", drop: Some("carpet_red") },
    Block { id: 541, name: "carpet_blue", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carpet_blue"), cat: "wood", drop: Some("carpet_blue") },
    Block { id: 542, name: "carpet_green", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carpet_green"), cat: "wood", drop: Some("carpet_green") },
    Block { id: 543, name: "iron_bars", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("iron_bars"), cat: "wood", drop: Some("iron_bars") },
    Block { id: 544, name: "books_pile", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("books_pile"), cat: "wood", drop: Some("books_pile") },
    Block { id: 545, name: "bottles", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bottles"), cat: "wood", drop: Some("bottles") },
    Block { id: 546, name: "beehive", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("beehive"), cat: "wood", drop: Some("beehive") },
    Block { id: 547, name: "cheese", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cheese"), cat: "wood", drop: Some("cheese") },
    Block { id: 548, name: "bread_basket", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bread_basket"), cat: "wood", drop: Some("bread_basket") },
    Block { id: 549, name: "torch_wall", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("torch_wall"), cat: "light", drop: Some("torch") },
    Block { id: 550, name: "torch_wall@1", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("torch_wall"), cat: "light", drop: Some("torch") },
    Block { id: 551, name: "torch_wall@2", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("torch_wall"), cat: "light", drop: Some("torch") },
    Block { id: 552, name: "torch_wall@3", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("torch_wall"), cat: "light", drop: Some("torch") },
    Block { id: 553, name: "bed@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed"), cat: "wood", drop: Some("bed") },
    Block { id: 554, name: "bed@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed"), cat: "wood", drop: Some("bed") },
    Block { id: 555, name: "bed@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed"), cat: "wood", drop: Some("bed") },
    Block { id: 556, name: "bed_foot", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed_foot"), cat: "wood", drop: Some("bed") },
    Block { id: 557, name: "bed_foot@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed_foot"), cat: "wood", drop: Some("bed") },
    Block { id: 558, name: "bed_foot@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed_foot"), cat: "wood", drop: Some("bed") },
    Block { id: 559, name: "bed_foot@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed_foot"), cat: "wood", drop: Some("bed") },
];

#[allow(non_snake_case)]
pub mod B {
    pub const AIR: crate::blocks::Id = 0;
    pub const GRASS: crate::blocks::Id = 1;
    pub const DIRT: crate::blocks::Id = 2;
    pub const PATH: crate::blocks::Id = 3;
    pub const FOREST_SOIL: crate::blocks::Id = 4;
    pub const STONE: crate::blocks::Id = 5;
    pub const MOSSY_STONE: crate::blocks::Id = 6;
    pub const LIMESTONE: crate::blocks::Id = 7;
    pub const GRANITE: crate::blocks::Id = 8;
    pub const GRAVEL: crate::blocks::Id = 9;
    pub const SAND: crate::blocks::Id = 10;
    pub const OAK_LOG: crate::blocks::Id = 11;
    pub const BIRCH_LOG: crate::blocks::Id = 12;
    pub const BEECH_LOG: crate::blocks::Id = 13;
    pub const OAK_LEAVES: crate::blocks::Id = 14;
    pub const BIRCH_LEAVES: crate::blocks::Id = 15;
    pub const BEECH_LEAVES: crate::blocks::Id = 16;
    pub const PLANKS: crate::blocks::Id = 17;
    pub const WATER: crate::blocks::Id = 18;
    pub const TALL_GRASS: crate::blocks::Id = 19;
    pub const FERN: crate::blocks::Id = 20;
    pub const DAISY: crate::blocks::Id = 21;
    pub const LUPINE: crate::blocks::Id = 22;
    pub const PINK_FLOWER: crate::blocks::Id = 23;
    pub const YELLOW_FLOWER: crate::blocks::Id = 24;
    pub const MUSHROOM: crate::blocks::Id = 25;
    pub const LILY_PAD: crate::blocks::Id = 26;
    pub const FLAGSTONE: crate::blocks::Id = 27;
    pub const LANTERN: crate::blocks::Id = 28;
    pub const VINE: crate::blocks::Id = 29;
    pub const SHELF_FUNGUS: crate::blocks::Id = 30;
    pub const FENCE: crate::blocks::Id = 31;
    pub const OAK_LOG_X: crate::blocks::Id = 32;
    pub const OAK_LOG_Z: crate::blocks::Id = 33;
    pub const BIRCH_LOG_X: crate::blocks::Id = 34;
    pub const BIRCH_LOG_Z: crate::blocks::Id = 35;
    pub const BEECH_LOG_X: crate::blocks::Id = 36;
    pub const BEECH_LOG_Z: crate::blocks::Id = 37;
    pub const PLANKS_Z: crate::blocks::Id = 38;
    pub const PLANKS_Y: crate::blocks::Id = 39;
    pub const WATER_1: crate::blocks::Id = 40;
    pub const WATER_2: crate::blocks::Id = 41;
    pub const WATER_3: crate::blocks::Id = 42;
    pub const WATER_4: crate::blocks::Id = 43;
    pub const WATER_5: crate::blocks::Id = 44;
    pub const WATER_6: crate::blocks::Id = 45;
    pub const WATER_7: crate::blocks::Id = 46;
    pub const GRASS_DEEP: crate::blocks::Id = 47;
    pub const OAK_LEAVES_DEEP: crate::blocks::Id = 48;
    pub const BEECH_LEAVES_DEEP: crate::blocks::Id = 49;
    pub const GLOWCAP: crate::blocks::Id = 50;
    pub const OAK_LOG_C: crate::blocks::Id = 51;
    pub const OAK_LOG_X_C: crate::blocks::Id = 52;
    pub const OAK_LOG_Z_C: crate::blocks::Id = 53;
    pub const BEECH_LOG_C: crate::blocks::Id = 54;
    pub const BEECH_LOG_X_C: crate::blocks::Id = 55;
    pub const BEECH_LOG_Z_C: crate::blocks::Id = 56;
    pub const SNOW: crate::blocks::Id = 57;
    pub const ICE: crate::blocks::Id = 58;
    pub const SPRUCE_LOG: crate::blocks::Id = 59;
    pub const SPRUCE_LOG_X: crate::blocks::Id = 60;
    pub const SPRUCE_LOG_Z: crate::blocks::Id = 61;
    pub const SPRUCE_LOG_C: crate::blocks::Id = 62;
    pub const SPRUCE_LOG_X_C: crate::blocks::Id = 63;
    pub const SPRUCE_LOG_Z_C: crate::blocks::Id = 64;
    pub const SPRUCE_LEAVES: crate::blocks::Id = 65;
    pub const SPRUCE_LEAVES_SNOW: crate::blocks::Id = 66;
    pub const ICICLE: crate::blocks::Id = 67;
    pub const SNOW_LAYER: crate::blocks::Id = 68;
    pub const PINECONE: crate::blocks::Id = 69;
    pub const FROST_GRASS: crate::blocks::Id = 70;
    pub const SNOW_LAYER_2: crate::blocks::Id = 71;
    pub const SNOW_LAYER_3: crate::blocks::Id = 72;
    pub const SNOW_LAYER_4: crate::blocks::Id = 73;
    pub const SNOW_LAYER_5: crate::blocks::Id = 74;
    pub const SNOW_LAYER_6: crate::blocks::Id = 75;
    pub const SNOW_LAYER_7: crate::blocks::Id = 76;
    pub const SNOW_LAYER_8: crate::blocks::Id = 77;
    pub const SNOW_LAYER_9: crate::blocks::Id = 78;
    pub const STEPPE_GRASS: crate::blocks::Id = 79;
    pub const DRY_EARTH: crate::blocks::Id = 80;
    pub const FEATHER_GRASS: crate::blocks::Id = 81;
    pub const SAGEBRUSH: crate::blocks::Id = 82;
    pub const DEAD_BUSH: crate::blocks::Id = 83;
    pub const DEAD_LOG: crate::blocks::Id = 84;
    pub const DEAD_LOG_X: crate::blocks::Id = 85;
    pub const DEAD_LOG_Z: crate::blocks::Id = 86;
    pub const LICHEN_STONE: crate::blocks::Id = 87;
    pub const BLUEBERRY: crate::blocks::Id = 88;
    pub const ROWAN: crate::blocks::Id = 89;
    pub const FERN_DRY: crate::blocks::Id = 90;
    pub const ACACIA_LOG: crate::blocks::Id = 91;
    pub const ACACIA_LOG_X: crate::blocks::Id = 92;
    pub const ACACIA_LOG_Z: crate::blocks::Id = 93;
    pub const ACACIA_LEAVES: crate::blocks::Id = 94;
    pub const BONE: crate::blocks::Id = 95;
    pub const BIRCH_LEAVES_GOLD: crate::blocks::Id = 96;
    pub const OAK_BRANCH: crate::blocks::Id = 97;
    pub const BIRCH_BRANCH: crate::blocks::Id = 98;
    pub const BEECH_BRANCH: crate::blocks::Id = 99;
    pub const SPRUCE_BRANCH: crate::blocks::Id = 100;
    pub const DEAD_BRANCH: crate::blocks::Id = 101;
    pub const ACACIA_BRANCH: crate::blocks::Id = 102;
    pub const OAK_HOLLOW: crate::blocks::Id = 103;
    pub const OAK_HOLLOW_X: crate::blocks::Id = 104;
    pub const OAK_HOLLOW_Z: crate::blocks::Id = 105;
    pub const SPRUCE_HOLLOW: crate::blocks::Id = 106;
    pub const SPRUCE_HOLLOW_X: crate::blocks::Id = 107;
    pub const SPRUCE_HOLLOW_Z: crate::blocks::Id = 108;
    pub const DEAD_HOLLOW: crate::blocks::Id = 109;
    pub const DEAD_HOLLOW_X: crate::blocks::Id = 110;
    pub const DEAD_HOLLOW_Z: crate::blocks::Id = 111;
    pub const GLOWCAP_BLUE: crate::blocks::Id = 112;
    pub const GLOWCAP_PINK: crate::blocks::Id = 113;
    pub const GLOWCAP_VIOLET: crate::blocks::Id = 114;
    pub const GLOWCAP_CYAN: crate::blocks::Id = 115;
    pub const SANDSTONE: crate::blocks::Id = 116;
    pub const RED_SANDSTONE: crate::blocks::Id = 117;
    pub const CACTUS: crate::blocks::Id = 118;
    pub const PALM_LOG: crate::blocks::Id = 119;
    pub const PALM_LEAVES: crate::blocks::Id = 120;
    pub const MOOR_GRASS: crate::blocks::Id = 121;
    pub const HEATHER: crate::blocks::Id = 122;
    pub const COTTON_GRASS: crate::blocks::Id = 123;
    pub const CAMPFIRE: crate::blocks::Id = 124;
    pub const WORKBENCH: crate::blocks::Id = 125;
    pub const TWIGS: crate::blocks::Id = 126;
    pub const PEBBLES: crate::blocks::Id = 127;
    pub const BLUEBERRY_PICKED: crate::blocks::Id = 128;
    pub const ROWAN_PICKED: crate::blocks::Id = 129;
    pub const COAL_ORE: crate::blocks::Id = 130;
    pub const COPPER_ORE: crate::blocks::Id = 131;
    pub const TIN_ORE: crate::blocks::Id = 132;
    pub const IRON_ORE: crate::blocks::Id = 133;
    pub const CLAY: crate::blocks::Id = 134;
    pub const STONE_BRICKS: crate::blocks::Id = 135;
    pub const CLAY_BRICKS: crate::blocks::Id = 136;
    pub const COBBLESTONE: crate::blocks::Id = 137;
    pub const THATCH: crate::blocks::Id = 138;
    pub const BIRCH_PLANKS: crate::blocks::Id = 139;
    pub const SPRUCE_PLANKS: crate::blocks::Id = 140;
    pub const FARMLAND: crate::blocks::Id = 141;
    pub const FURNACE: crate::blocks::Id = 142;
    pub const ANVIL: crate::blocks::Id = 143;
    pub const MASON_BENCH: crate::blocks::Id = 144;
    pub const CHEST: crate::blocks::Id = 145;
    pub const BEDROLL: crate::blocks::Id = 146;
    pub const BED: crate::blocks::Id = 147;
    pub const TORCH: crate::blocks::Id = 148;
    pub const CAMPFIRE_OUT: crate::blocks::Id = 149;
    pub const WHEAT_0: crate::blocks::Id = 150;
    pub const WHEAT_1: crate::blocks::Id = 151;
    pub const WHEAT_2: crate::blocks::Id = 152;
    pub const WHEAT_3: crate::blocks::Id = 153;
    pub const PLANKS_SLAB: crate::blocks::Id = 154;
    pub const PLANKS_SLAB_TOP: crate::blocks::Id = 155;
    pub const STONE_BRICKS_SLAB: crate::blocks::Id = 156;
    pub const STONE_BRICKS_SLAB_TOP: crate::blocks::Id = 157;
    pub const COBBLESTONE_SLAB: crate::blocks::Id = 158;
    pub const COBBLESTONE_SLAB_TOP: crate::blocks::Id = 159;
    pub const PLANKS_STAIRS: crate::blocks::Id = 160;
    pub const PLANKS_STAIRS_1: crate::blocks::Id = 161;
    pub const PLANKS_STAIRS_2: crate::blocks::Id = 162;
    pub const PLANKS_STAIRS_3: crate::blocks::Id = 163;
    pub const STONE_BRICKS_STAIRS: crate::blocks::Id = 164;
    pub const STONE_BRICKS_STAIRS_1: crate::blocks::Id = 165;
    pub const STONE_BRICKS_STAIRS_2: crate::blocks::Id = 166;
    pub const STONE_BRICKS_STAIRS_3: crate::blocks::Id = 167;
    pub const COBBLESTONE_STAIRS: crate::blocks::Id = 168;
    pub const COBBLESTONE_STAIRS_1: crate::blocks::Id = 169;
    pub const COBBLESTONE_STAIRS_2: crate::blocks::Id = 170;
    pub const COBBLESTONE_STAIRS_3: crate::blocks::Id = 171;
    pub const THATCH_STAIRS: crate::blocks::Id = 172;
    pub const THATCH_STAIRS_1: crate::blocks::Id = 173;
    pub const THATCH_STAIRS_2: crate::blocks::Id = 174;
    pub const THATCH_STAIRS_3: crate::blocks::Id = 175;
    pub const DOOR: crate::blocks::Id = 176;
    pub const DOOR_001: crate::blocks::Id = 177;
    pub const DOOR_010: crate::blocks::Id = 178;
    pub const DOOR_011: crate::blocks::Id = 179;
    pub const DOOR_100: crate::blocks::Id = 180;
    pub const DOOR_101: crate::blocks::Id = 181;
    pub const DOOR_110: crate::blocks::Id = 182;
    pub const DOOR_111: crate::blocks::Id = 183;
    pub const DOOR_200: crate::blocks::Id = 184;
    pub const DOOR_201: crate::blocks::Id = 185;
    pub const DOOR_210: crate::blocks::Id = 186;
    pub const DOOR_211: crate::blocks::Id = 187;
    pub const DOOR_300: crate::blocks::Id = 188;
    pub const DOOR_301: crate::blocks::Id = 189;
    pub const DOOR_310: crate::blocks::Id = 190;
    pub const DOOR_311: crate::blocks::Id = 191;
    pub const TRAPDOOR: crate::blocks::Id = 192;
    pub const TRAPDOOR_OPEN: crate::blocks::Id = 193;
    pub const TRAPDOOR_OPEN_1: crate::blocks::Id = 194;
    pub const TRAPDOOR_OPEN_2: crate::blocks::Id = 195;
    pub const TRAPDOOR_OPEN_3: crate::blocks::Id = 196;
    pub const LADDER: crate::blocks::Id = 197;
    pub const LADDER_1: crate::blocks::Id = 198;
    pub const LADDER_2: crate::blocks::Id = 199;
    pub const LADDER_3: crate::blocks::Id = 200;
    pub const WINDOW: crate::blocks::Id = 201;
    pub const WINDOW_X: crate::blocks::Id = 202;
    pub const WOOD_PILLAR: crate::blocks::Id = 203;
    pub const STONE_PILLAR: crate::blocks::Id = 204;
    pub const STONE_WALL: crate::blocks::Id = 205;
    pub const STONE_WALL_X: crate::blocks::Id = 206;
    pub const CARROT_0: crate::blocks::Id = 207;
    pub const CARROT_1: crate::blocks::Id = 208;
    pub const CARROT_2: crate::blocks::Id = 209;
    pub const CARROT_3: crate::blocks::Id = 210;
    pub const POTATO_0: crate::blocks::Id = 211;
    pub const POTATO_1: crate::blocks::Id = 212;
    pub const POTATO_2: crate::blocks::Id = 213;
    pub const POTATO_3: crate::blocks::Id = 214;
    pub const PLANKS_STAIRS_TOP: crate::blocks::Id = 215;
    pub const PLANKS_STAIRS_TOP_1: crate::blocks::Id = 216;
    pub const PLANKS_STAIRS_TOP_2: crate::blocks::Id = 217;
    pub const PLANKS_STAIRS_TOP_3: crate::blocks::Id = 218;
    pub const STONE_BRICKS_STAIRS_TOP: crate::blocks::Id = 219;
    pub const STONE_BRICKS_STAIRS_TOP_1: crate::blocks::Id = 220;
    pub const STONE_BRICKS_STAIRS_TOP_2: crate::blocks::Id = 221;
    pub const STONE_BRICKS_STAIRS_TOP_3: crate::blocks::Id = 222;
    pub const COBBLESTONE_STAIRS_TOP: crate::blocks::Id = 223;
    pub const COBBLESTONE_STAIRS_TOP_1: crate::blocks::Id = 224;
    pub const COBBLESTONE_STAIRS_TOP_2: crate::blocks::Id = 225;
    pub const COBBLESTONE_STAIRS_TOP_3: crate::blocks::Id = 226;
    pub const THATCH_STAIRS_TOP: crate::blocks::Id = 227;
    pub const THATCH_STAIRS_TOP_1: crate::blocks::Id = 228;
    pub const THATCH_STAIRS_TOP_2: crate::blocks::Id = 229;
    pub const THATCH_STAIRS_TOP_3: crate::blocks::Id = 230;
    pub const PLANKS_VSLAB: crate::blocks::Id = 231;
    pub const PLANKS_VSLAB_1: crate::blocks::Id = 232;
    pub const PLANKS_VSLAB_2: crate::blocks::Id = 233;
    pub const PLANKS_VSLAB_3: crate::blocks::Id = 234;
    pub const STONE_BRICKS_VSLAB: crate::blocks::Id = 235;
    pub const STONE_BRICKS_VSLAB_1: crate::blocks::Id = 236;
    pub const STONE_BRICKS_VSLAB_2: crate::blocks::Id = 237;
    pub const STONE_BRICKS_VSLAB_3: crate::blocks::Id = 238;
    pub const COBBLESTONE_VSLAB: crate::blocks::Id = 239;
    pub const COBBLESTONE_VSLAB_1: crate::blocks::Id = 240;
    pub const COBBLESTONE_VSLAB_2: crate::blocks::Id = 241;
    pub const COBBLESTONE_VSLAB_3: crate::blocks::Id = 242;
    pub const SHUTTER: crate::blocks::Id = 243;
    pub const SHUTTER_1: crate::blocks::Id = 244;
    pub const SHUTTER_2: crate::blocks::Id = 245;
    pub const SHUTTER_3: crate::blocks::Id = 246;
    pub const SHUTTER_OPEN: crate::blocks::Id = 247;
    pub const SHUTTER_OPEN_1: crate::blocks::Id = 248;
    pub const SHUTTER_OPEN_2: crate::blocks::Id = 249;
    pub const SHUTTER_OPEN_3: crate::blocks::Id = 250;
    pub const PLASTER: crate::blocks::Id = 251;
    pub const TIMBER: crate::blocks::Id = 252;
    pub const ROOF_TILES: crate::blocks::Id = 253;
    pub const SLATE: crate::blocks::Id = 254;
    pub const BARREL: crate::blocks::Id = 255;
    pub const MARBLE: crate::blocks::Id = 256;
    pub const ASHLAR: crate::blocks::Id = 257;
    pub const FIELDSTONE: crate::blocks::Id = 258;
    pub const SETTS: crate::blocks::Id = 259;
    pub const POLISHED_STONE: crate::blocks::Id = 260;
    pub const SHINGLES: crate::blocks::Id = 261;
    pub const COPPER_ROOF: crate::blocks::Id = 262;
    pub const CHECKER_FLOOR: crate::blocks::Id = 263;
    pub const TERRACOTTA_FLOOR: crate::blocks::Id = 264;
    pub const PARQUET: crate::blocks::Id = 265;
    pub const CHISELED_STONE: crate::blocks::Id = 266;
    pub const STAINED_GLASS: crate::blocks::Id = 267;
    pub const IRON_BLOCK: crate::blocks::Id = 268;
    pub const MARBLE_BLACK: crate::blocks::Id = 269;
    pub const BRICKS_DARK: crate::blocks::Id = 270;
    pub const STONE_BRICKS_DARK: crate::blocks::Id = 271;
    pub const TIMBER_FRAME: crate::blocks::Id = 272;
    pub const TIMBER_CROSS: crate::blocks::Id = 273;
    pub const TIMBER_BRACE: crate::blocks::Id = 274;
    pub const WATTLE: crate::blocks::Id = 275;
    pub const BOOKSHELF: crate::blocks::Id = 276;
    pub const HAY_BALE: crate::blocks::Id = 277;
    pub const CRATE: crate::blocks::Id = 278;
    pub const PLANKS_DARK: crate::blocks::Id = 279;
    pub const PLANKS_RED: crate::blocks::Id = 280;
    pub const PLANKS_WHITE: crate::blocks::Id = 281;
    pub const PLANKS_BLUE: crate::blocks::Id = 282;
    pub const PLANKS_GREEN: crate::blocks::Id = 283;
    pub const PLASTER_OCHRE: crate::blocks::Id = 284;
    pub const PLASTER_ROSE: crate::blocks::Id = 285;
    pub const PLASTER_BLUE: crate::blocks::Id = 286;
    pub const WOOL_WHITE: crate::blocks::Id = 287;
    pub const WOOL_RED: crate::blocks::Id = 288;
    pub const WOOL_BLUE: crate::blocks::Id = 289;
    pub const WOOL_GREEN: crate::blocks::Id = 290;
    pub const WOOL_YELLOW: crate::blocks::Id = 291;
    pub const WOOL_BLACK: crate::blocks::Id = 292;
    pub const WOOL_BROWN: crate::blocks::Id = 293;
    pub const WOOL_PURPLE: crate::blocks::Id = 294;
    pub const WOOL_ORANGE: crate::blocks::Id = 295;
    pub const MARBLE_STAIRS: crate::blocks::Id = 296;
    pub const MARBLE_STAIRS_1: crate::blocks::Id = 297;
    pub const MARBLE_STAIRS_2: crate::blocks::Id = 298;
    pub const MARBLE_STAIRS_3: crate::blocks::Id = 299;
    pub const MARBLE_STAIRS_TOP: crate::blocks::Id = 300;
    pub const MARBLE_STAIRS_TOP_1: crate::blocks::Id = 301;
    pub const MARBLE_STAIRS_TOP_2: crate::blocks::Id = 302;
    pub const MARBLE_STAIRS_TOP_3: crate::blocks::Id = 303;
    pub const MARBLE_SLAB: crate::blocks::Id = 304;
    pub const MARBLE_SLAB_TOP: crate::blocks::Id = 305;
    pub const MARBLE_VSLAB: crate::blocks::Id = 306;
    pub const MARBLE_VSLAB_1: crate::blocks::Id = 307;
    pub const MARBLE_VSLAB_2: crate::blocks::Id = 308;
    pub const MARBLE_VSLAB_3: crate::blocks::Id = 309;
    pub const ASHLAR_STAIRS: crate::blocks::Id = 310;
    pub const ASHLAR_STAIRS_1: crate::blocks::Id = 311;
    pub const ASHLAR_STAIRS_2: crate::blocks::Id = 312;
    pub const ASHLAR_STAIRS_3: crate::blocks::Id = 313;
    pub const ASHLAR_STAIRS_TOP: crate::blocks::Id = 314;
    pub const ASHLAR_STAIRS_TOP_1: crate::blocks::Id = 315;
    pub const ASHLAR_STAIRS_TOP_2: crate::blocks::Id = 316;
    pub const ASHLAR_STAIRS_TOP_3: crate::blocks::Id = 317;
    pub const ASHLAR_SLAB: crate::blocks::Id = 318;
    pub const ASHLAR_SLAB_TOP: crate::blocks::Id = 319;
    pub const ASHLAR_VSLAB: crate::blocks::Id = 320;
    pub const ASHLAR_VSLAB_1: crate::blocks::Id = 321;
    pub const ASHLAR_VSLAB_2: crate::blocks::Id = 322;
    pub const ASHLAR_VSLAB_3: crate::blocks::Id = 323;
    pub const FIELDSTONE_STAIRS: crate::blocks::Id = 324;
    pub const FIELDSTONE_STAIRS_1: crate::blocks::Id = 325;
    pub const FIELDSTONE_STAIRS_2: crate::blocks::Id = 326;
    pub const FIELDSTONE_STAIRS_3: crate::blocks::Id = 327;
    pub const FIELDSTONE_STAIRS_TOP: crate::blocks::Id = 328;
    pub const FIELDSTONE_STAIRS_TOP_1: crate::blocks::Id = 329;
    pub const FIELDSTONE_STAIRS_TOP_2: crate::blocks::Id = 330;
    pub const FIELDSTONE_STAIRS_TOP_3: crate::blocks::Id = 331;
    pub const FIELDSTONE_SLAB: crate::blocks::Id = 332;
    pub const FIELDSTONE_SLAB_TOP: crate::blocks::Id = 333;
    pub const FIELDSTONE_VSLAB: crate::blocks::Id = 334;
    pub const FIELDSTONE_VSLAB_1: crate::blocks::Id = 335;
    pub const FIELDSTONE_VSLAB_2: crate::blocks::Id = 336;
    pub const FIELDSTONE_VSLAB_3: crate::blocks::Id = 337;
    pub const SETTS_STAIRS: crate::blocks::Id = 338;
    pub const SETTS_STAIRS_1: crate::blocks::Id = 339;
    pub const SETTS_STAIRS_2: crate::blocks::Id = 340;
    pub const SETTS_STAIRS_3: crate::blocks::Id = 341;
    pub const SETTS_STAIRS_TOP: crate::blocks::Id = 342;
    pub const SETTS_STAIRS_TOP_1: crate::blocks::Id = 343;
    pub const SETTS_STAIRS_TOP_2: crate::blocks::Id = 344;
    pub const SETTS_STAIRS_TOP_3: crate::blocks::Id = 345;
    pub const SETTS_SLAB: crate::blocks::Id = 346;
    pub const SETTS_SLAB_TOP: crate::blocks::Id = 347;
    pub const SETTS_VSLAB: crate::blocks::Id = 348;
    pub const SETTS_VSLAB_1: crate::blocks::Id = 349;
    pub const SETTS_VSLAB_2: crate::blocks::Id = 350;
    pub const SETTS_VSLAB_3: crate::blocks::Id = 351;
    pub const POLISHED_STONE_STAIRS: crate::blocks::Id = 352;
    pub const POLISHED_STONE_STAIRS_1: crate::blocks::Id = 353;
    pub const POLISHED_STONE_STAIRS_2: crate::blocks::Id = 354;
    pub const POLISHED_STONE_STAIRS_3: crate::blocks::Id = 355;
    pub const POLISHED_STONE_STAIRS_TOP: crate::blocks::Id = 356;
    pub const POLISHED_STONE_STAIRS_TOP_1: crate::blocks::Id = 357;
    pub const POLISHED_STONE_STAIRS_TOP_2: crate::blocks::Id = 358;
    pub const POLISHED_STONE_STAIRS_TOP_3: crate::blocks::Id = 359;
    pub const POLISHED_STONE_SLAB: crate::blocks::Id = 360;
    pub const POLISHED_STONE_SLAB_TOP: crate::blocks::Id = 361;
    pub const POLISHED_STONE_VSLAB: crate::blocks::Id = 362;
    pub const POLISHED_STONE_VSLAB_1: crate::blocks::Id = 363;
    pub const POLISHED_STONE_VSLAB_2: crate::blocks::Id = 364;
    pub const POLISHED_STONE_VSLAB_3: crate::blocks::Id = 365;
    pub const SHINGLES_STAIRS: crate::blocks::Id = 366;
    pub const SHINGLES_STAIRS_1: crate::blocks::Id = 367;
    pub const SHINGLES_STAIRS_2: crate::blocks::Id = 368;
    pub const SHINGLES_STAIRS_3: crate::blocks::Id = 369;
    pub const SHINGLES_STAIRS_TOP: crate::blocks::Id = 370;
    pub const SHINGLES_STAIRS_TOP_1: crate::blocks::Id = 371;
    pub const SHINGLES_STAIRS_TOP_2: crate::blocks::Id = 372;
    pub const SHINGLES_STAIRS_TOP_3: crate::blocks::Id = 373;
    pub const SHINGLES_SLAB: crate::blocks::Id = 374;
    pub const SHINGLES_SLAB_TOP: crate::blocks::Id = 375;
    pub const SHINGLES_VSLAB: crate::blocks::Id = 376;
    pub const SHINGLES_VSLAB_1: crate::blocks::Id = 377;
    pub const SHINGLES_VSLAB_2: crate::blocks::Id = 378;
    pub const SHINGLES_VSLAB_3: crate::blocks::Id = 379;
    pub const ROOF_TILES_STAIRS: crate::blocks::Id = 380;
    pub const ROOF_TILES_STAIRS_1: crate::blocks::Id = 381;
    pub const ROOF_TILES_STAIRS_2: crate::blocks::Id = 382;
    pub const ROOF_TILES_STAIRS_3: crate::blocks::Id = 383;
    pub const ROOF_TILES_STAIRS_TOP: crate::blocks::Id = 384;
    pub const ROOF_TILES_STAIRS_TOP_1: crate::blocks::Id = 385;
    pub const ROOF_TILES_STAIRS_TOP_2: crate::blocks::Id = 386;
    pub const ROOF_TILES_STAIRS_TOP_3: crate::blocks::Id = 387;
    pub const ROOF_TILES_SLAB: crate::blocks::Id = 388;
    pub const ROOF_TILES_SLAB_TOP: crate::blocks::Id = 389;
    pub const ROOF_TILES_VSLAB: crate::blocks::Id = 390;
    pub const ROOF_TILES_VSLAB_1: crate::blocks::Id = 391;
    pub const ROOF_TILES_VSLAB_2: crate::blocks::Id = 392;
    pub const ROOF_TILES_VSLAB_3: crate::blocks::Id = 393;
    pub const SLATE_STAIRS: crate::blocks::Id = 394;
    pub const SLATE_STAIRS_1: crate::blocks::Id = 395;
    pub const SLATE_STAIRS_2: crate::blocks::Id = 396;
    pub const SLATE_STAIRS_3: crate::blocks::Id = 397;
    pub const SLATE_STAIRS_TOP: crate::blocks::Id = 398;
    pub const SLATE_STAIRS_TOP_1: crate::blocks::Id = 399;
    pub const SLATE_STAIRS_TOP_2: crate::blocks::Id = 400;
    pub const SLATE_STAIRS_TOP_3: crate::blocks::Id = 401;
    pub const SLATE_SLAB: crate::blocks::Id = 402;
    pub const SLATE_SLAB_TOP: crate::blocks::Id = 403;
    pub const SLATE_VSLAB: crate::blocks::Id = 404;
    pub const SLATE_VSLAB_1: crate::blocks::Id = 405;
    pub const SLATE_VSLAB_2: crate::blocks::Id = 406;
    pub const SLATE_VSLAB_3: crate::blocks::Id = 407;
    pub const COPPER_ROOF_STAIRS: crate::blocks::Id = 408;
    pub const COPPER_ROOF_STAIRS_1: crate::blocks::Id = 409;
    pub const COPPER_ROOF_STAIRS_2: crate::blocks::Id = 410;
    pub const COPPER_ROOF_STAIRS_3: crate::blocks::Id = 411;
    pub const COPPER_ROOF_STAIRS_TOP: crate::blocks::Id = 412;
    pub const COPPER_ROOF_STAIRS_TOP_1: crate::blocks::Id = 413;
    pub const COPPER_ROOF_STAIRS_TOP_2: crate::blocks::Id = 414;
    pub const COPPER_ROOF_STAIRS_TOP_3: crate::blocks::Id = 415;
    pub const COPPER_ROOF_SLAB: crate::blocks::Id = 416;
    pub const COPPER_ROOF_SLAB_TOP: crate::blocks::Id = 417;
    pub const COPPER_ROOF_VSLAB: crate::blocks::Id = 418;
    pub const COPPER_ROOF_VSLAB_1: crate::blocks::Id = 419;
    pub const COPPER_ROOF_VSLAB_2: crate::blocks::Id = 420;
    pub const COPPER_ROOF_VSLAB_3: crate::blocks::Id = 421;
    pub const PLASTER_STAIRS: crate::blocks::Id = 422;
    pub const PLASTER_STAIRS_1: crate::blocks::Id = 423;
    pub const PLASTER_STAIRS_2: crate::blocks::Id = 424;
    pub const PLASTER_STAIRS_3: crate::blocks::Id = 425;
    pub const PLASTER_STAIRS_TOP: crate::blocks::Id = 426;
    pub const PLASTER_STAIRS_TOP_1: crate::blocks::Id = 427;
    pub const PLASTER_STAIRS_TOP_2: crate::blocks::Id = 428;
    pub const PLASTER_STAIRS_TOP_3: crate::blocks::Id = 429;
    pub const PLASTER_SLAB: crate::blocks::Id = 430;
    pub const PLASTER_SLAB_TOP: crate::blocks::Id = 431;
    pub const PLASTER_VSLAB: crate::blocks::Id = 432;
    pub const PLASTER_VSLAB_1: crate::blocks::Id = 433;
    pub const PLASTER_VSLAB_2: crate::blocks::Id = 434;
    pub const PLASTER_VSLAB_3: crate::blocks::Id = 435;
    pub const TIMBER_STAIRS: crate::blocks::Id = 436;
    pub const TIMBER_STAIRS_1: crate::blocks::Id = 437;
    pub const TIMBER_STAIRS_2: crate::blocks::Id = 438;
    pub const TIMBER_STAIRS_3: crate::blocks::Id = 439;
    pub const TIMBER_STAIRS_TOP: crate::blocks::Id = 440;
    pub const TIMBER_STAIRS_TOP_1: crate::blocks::Id = 441;
    pub const TIMBER_STAIRS_TOP_2: crate::blocks::Id = 442;
    pub const TIMBER_STAIRS_TOP_3: crate::blocks::Id = 443;
    pub const TIMBER_SLAB: crate::blocks::Id = 444;
    pub const TIMBER_SLAB_TOP: crate::blocks::Id = 445;
    pub const TIMBER_VSLAB: crate::blocks::Id = 446;
    pub const TIMBER_VSLAB_1: crate::blocks::Id = 447;
    pub const TIMBER_VSLAB_2: crate::blocks::Id = 448;
    pub const TIMBER_VSLAB_3: crate::blocks::Id = 449;
    pub const CHAIR: crate::blocks::Id = 450;
    pub const BENCH: crate::blocks::Id = 454;
    pub const ARMCHAIR: crate::blocks::Id = 458;
    pub const THRONE: crate::blocks::Id = 462;
    pub const CUPBOARD: crate::blocks::Id = 466;
    pub const FIREPLACE: crate::blocks::Id = 470;
    pub const LOOM: crate::blocks::Id = 474;
    pub const SPINNING_WHEEL: crate::blocks::Id = 478;
    pub const GRINDSTONE: crate::blocks::Id = 482;
    pub const SIGNPOST: crate::blocks::Id = 486;
    pub const PAINTING: crate::blocks::Id = 490;
    pub const BANNER_RED: crate::blocks::Id = 494;
    pub const BANNER_BLUE: crate::blocks::Id = 498;
    pub const BANNER_GREEN: crate::blocks::Id = 502;
    pub const BANNER_GOLD: crate::blocks::Id = 506;
    pub const WEAPON_RACK: crate::blocks::Id = 510;
    pub const WALL_SCONCE: crate::blocks::Id = 514;
    pub const CURTAIN: crate::blocks::Id = 518;
    pub const WALL_SHELF: crate::blocks::Id = 522;
    pub const TABLE: crate::blocks::Id = 526;
    pub const ROUND_TABLE: crate::blocks::Id = 527;
    pub const STOOL: crate::blocks::Id = 528;
    pub const CANDLE: crate::blocks::Id = 529;
    pub const CANDELABRA: crate::blocks::Id = 530;
    pub const CHANDELIER: crate::blocks::Id = 531;
    pub const BRAZIER: crate::blocks::Id = 532;
    pub const SACK: crate::blocks::Id = 533;
    pub const AMPHORA: crate::blocks::Id = 534;
    pub const FLOWER_POT: crate::blocks::Id = 535;
    pub const POTTED_FERN: crate::blocks::Id = 536;
    pub const CAULDRON: crate::blocks::Id = 537;
    pub const BUCKET: crate::blocks::Id = 538;
    pub const LOG_PILE: crate::blocks::Id = 539;
    pub const CARPET_RED: crate::blocks::Id = 540;
    pub const CARPET_BLUE: crate::blocks::Id = 541;
    pub const CARPET_GREEN: crate::blocks::Id = 542;
    pub const IRON_BARS: crate::blocks::Id = 543;
    pub const BOOKS_PILE: crate::blocks::Id = 544;
    pub const BOTTLES: crate::blocks::Id = 545;
    pub const BEEHIVE: crate::blocks::Id = 546;
    pub const CHEESE: crate::blocks::Id = 547;
    pub const BREAD_BASKET: crate::blocks::Id = 548;
    pub const TORCH_WALL: crate::blocks::Id = 549;
    pub const BED_FOOT: crate::blocks::Id = 556;
}
