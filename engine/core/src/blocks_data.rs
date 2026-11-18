// Généré par engine/tools/gen_blocks.mjs. Les identifiants sont stables : ils vivent dans les sauvegardes.
use crate::blocks::{Block, Shape};

pub const VARIANTS: u32 = 4;
pub const MATERIALS: [&str; 113] = ["grass_top", "grass_side", "dirt", "path", "stone", "mossy_stone", "limestone", "granite", "gravel", "sand", "bark_oak", "bark_birch", "bark_beech", "leaf_oak", "leaf_birch", "leaf_beech", "planks", "forest_soil", "water_foam", "end_oak", "end_birch", "end_beech", "snow", "snow_side", "ice", "bark_spruce", "end_spruce", "leaf_spruce", "leaf_snow", "grass_dry_top", "grass_dry_side", "dry_earth", "bark_dead", "end_dead", "lichen", "bark_acacia", "end_acacia", "leaf_acacia", "bone", "wood_inner", "sandstone", "sandstone_top", "red_sandstone", "cactus_side", "cactus_top", "bark_palm", "end_palm", "leaf_palm", "ore_copper", "ore_tin", "ore_iron", "ore_coal", "clay", "stone_bricks", "bricks", "thatch", "planks_birch", "planks_spruce", "cobble", "farmland_top", "plaster", "dark_timber", "roof_tiles", "slate", "marble", "ashlar", "fieldstone", "setts", "polished_stone", "checker", "terracotta", "parquet", "shingles", "wool", "rug", "frame_square", "frame_cross", "frame_diag", "wattle", "books", "hay", "crate", "chiseled", "stained_glass", "iron_plate", "verdigris", "coral", "planks_wreck", "lime_masonry", "rubble", "beam_end", "boards", "tiles_moss", "path_top", "flowstone", "basalt", "obsidian", "lava", "magma", "crystal_violet", "crystal_blue", "crystal_pink", "crystal_white", "abyss_rock", "glow_moss", "mycelium", "mushroom_stem", "cap_red", "cap_violet", "cap_blue", "gills", "ice_blue", "rail_bed"];

pub const BLOCKS: [Block; 1172] = [
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
    Block { id: 193, name: "trapdoor_1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 194, name: "trapdoor_2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 195, name: "trapdoor_3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 196, name: "trapdoor_open", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 197, name: "trapdoor_open_1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 198, name: "trapdoor_open_2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 199, name: "trapdoor_open_3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("trapdoor") },
    Block { id: 200, name: "ladder", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("ladder"), cat: "wood", drop: Some("ladder") },
    Block { id: 201, name: "ladder_1", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("ladder"), cat: "wood", drop: Some("ladder") },
    Block { id: 202, name: "ladder_2", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("ladder"), cat: "wood", drop: Some("ladder") },
    Block { id: 203, name: "ladder_3", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("ladder"), cat: "wood", drop: Some("ladder") },
    Block { id: 204, name: "window", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("window"), cat: "wood", drop: Some("window") },
    Block { id: 205, name: "window_x", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("window"), cat: "wood", drop: Some("window") },
    Block { id: 206, name: "wood_pillar", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("pillar"), cat: "wood", drop: Some("wood_pillar") },
    Block { id: 207, name: "stone_pillar", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("pillar"), cat: "rock", drop: Some("stone_pillar") },
    Block { id: 208, name: "stone_wall", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall"), cat: "rock", drop: Some("stone_wall") },
    Block { id: 209, name: "stone_wall_x", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall"), cat: "rock", drop: Some("stone_wall") },
    Block { id: 210, name: "carrot_0", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carrot"), cat: "decor", drop: Some("carrot") },
    Block { id: 211, name: "carrot_1", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carrot"), cat: "decor", drop: Some("carrot") },
    Block { id: 212, name: "carrot_2", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carrot"), cat: "decor", drop: Some("carrot") },
    Block { id: 213, name: "carrot_3", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carrot"), cat: "decor", drop: Some("carrot") },
    Block { id: 214, name: "potato_0", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("potato"), cat: "decor", drop: Some("potato") },
    Block { id: 215, name: "potato_1", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("potato"), cat: "decor", drop: Some("potato") },
    Block { id: 216, name: "potato_2", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("potato"), cat: "decor", drop: Some("potato") },
    Block { id: 217, name: "potato_3", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("potato"), cat: "decor", drop: Some("potato") },
    Block { id: 218, name: "planks_stairs_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 219, name: "planks_stairs_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 220, name: "planks_stairs_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 221, name: "planks_stairs_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("planks_stairs") },
    Block { id: 222, name: "stone_bricks_stairs_top", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 223, name: "stone_bricks_stairs_top_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 224, name: "stone_bricks_stairs_top_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 225, name: "stone_bricks_stairs_top_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_bricks_stairs") },
    Block { id: 226, name: "cobblestone_stairs_top", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 227, name: "cobblestone_stairs_top_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 228, name: "cobblestone_stairs_top_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 229, name: "cobblestone_stairs_top_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("cobblestone_stairs") },
    Block { id: 230, name: "thatch_stairs_top", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 231, name: "thatch_stairs_top_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 232, name: "thatch_stairs_top_2", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 233, name: "thatch_stairs_top_3", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "foliage", drop: Some("thatch_stairs") },
    Block { id: 234, name: "planks_vslab", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("planks_vslab") },
    Block { id: 235, name: "planks_vslab_1", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("planks_vslab") },
    Block { id: 236, name: "planks_vslab_2", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("planks_vslab") },
    Block { id: 237, name: "planks_vslab_3", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("planks_vslab") },
    Block { id: 238, name: "stone_bricks_vslab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_bricks_vslab") },
    Block { id: 239, name: "stone_bricks_vslab_1", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_bricks_vslab") },
    Block { id: 240, name: "stone_bricks_vslab_2", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_bricks_vslab") },
    Block { id: 241, name: "stone_bricks_vslab_3", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_bricks_vslab") },
    Block { id: 242, name: "cobblestone_vslab", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("cobblestone_vslab") },
    Block { id: 243, name: "cobblestone_vslab_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("cobblestone_vslab") },
    Block { id: 244, name: "cobblestone_vslab_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("cobblestone_vslab") },
    Block { id: 245, name: "cobblestone_vslab_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("cobblestone_vslab") },
    Block { id: 246, name: "shutter", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 247, name: "shutter_1", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 248, name: "shutter_2", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 249, name: "shutter_3", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 250, name: "shutter_open", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 251, name: "shutter_open_1", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 252, name: "shutter_open_2", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 253, name: "shutter_open_3", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trapdoor"), cat: "wood", drop: Some("shutter") },
    Block { id: 254, name: "plaster", shape: Shape::Cube, hardness: 0.7, faces: [240,240,240], dir_faces: [240,240,240,240], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("plaster") },
    Block { id: 255, name: "timber", shape: Shape::Cube, hardness: 0.9, faces: [244,244,244], dir_faces: [244,244,244,244], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("timber") },
    Block { id: 256, name: "roof_tiles", shape: Shape::Cube, hardness: 0.9, faces: [248,248,248], dir_faces: [248,248,248,248], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("roof_tiles") },
    Block { id: 257, name: "slate", shape: Shape::Cube, hardness: 1.1, faces: [252,252,252], dir_faces: [252,252,252,252], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("slate") },
    Block { id: 258, name: "barrel", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("barrel"), cat: "wood", drop: Some("barrel") },
    Block { id: 259, name: "marble", shape: Shape::Cube, hardness: 1.8, faces: [256,256,256], dir_faces: [256,256,256,256], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("marble") },
    Block { id: 260, name: "ashlar", shape: Shape::Cube, hardness: 1.6, faces: [260,260,260], dir_faces: [260,260,260,260], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("ashlar") },
    Block { id: 261, name: "fieldstone", shape: Shape::Cube, hardness: 1.5, faces: [264,264,264], dir_faces: [264,264,264,264], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("fieldstone") },
    Block { id: 262, name: "setts", shape: Shape::Cube, hardness: 1.5, faces: [268,268,268], dir_faces: [268,268,268,268], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("setts") },
    Block { id: 263, name: "polished_stone", shape: Shape::Cube, hardness: 1.6, faces: [272,272,272], dir_faces: [272,272,272,272], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("polished_stone") },
    Block { id: 264, name: "shingles", shape: Shape::Cube, hardness: 0.8, faces: [288,288,288], dir_faces: [288,288,288,288], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("shingles") },
    Block { id: 265, name: "copper_roof", shape: Shape::Cube, hardness: 1.4, faces: [340,340,340], dir_faces: [340,340,340,340], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("copper_roof") },
    Block { id: 266, name: "checker_floor", shape: Shape::Cube, hardness: 1.6, faces: [276,276,276], dir_faces: [276,276,276,276], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("checker_floor") },
    Block { id: 267, name: "terracotta_floor", shape: Shape::Cube, hardness: 1.0, faces: [280,280,280], dir_faces: [280,280,280,280], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("terracotta_floor") },
    Block { id: 268, name: "parquet", shape: Shape::Cube, hardness: 0.8, faces: [284,284,284], dir_faces: [284,284,284,284], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("parquet") },
    Block { id: 269, name: "chiseled_stone", shape: Shape::Cube, hardness: 1.6, faces: [328,328,328], dir_faces: [328,328,328,328], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("chiseled_stone") },
    Block { id: 270, name: "stained_glass", shape: Shape::Cube, hardness: 0.5, faces: [332,332,332], dir_faces: [332,332,332,332], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("stained_glass") },
    Block { id: 271, name: "iron_block", shape: Shape::Cube, hardness: 2.5, faces: [336,336,336], dir_faces: [336,336,336,336], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("iron_block") },
    Block { id: 272, name: "marble_black", shape: Shape::Cube, hardness: 1.8, faces: [256,256,256], dir_faces: [256,256,256,256], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.3,0.3,0.32], model: None, cat: "rock", drop: Some("marble_black") },
    Block { id: 273, name: "bricks_dark", shape: Shape::Cube, hardness: 1.5, faces: [216,216,216], dir_faces: [216,216,216,216], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.55,0.5,0.48], model: None, cat: "rock", drop: Some("bricks_dark") },
    Block { id: 274, name: "stone_bricks_dark", shape: Shape::Cube, hardness: 1.6, faces: [212,212,212], dir_faces: [212,212,212,212], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.62,0.62,0.66], model: None, cat: "rock", drop: Some("stone_bricks_dark") },
    Block { id: 275, name: "timber_frame", shape: Shape::Cube, hardness: 0.9, faces: [300,300,300], dir_faces: [300,300,300,300], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("timber_frame") },
    Block { id: 276, name: "timber_cross", shape: Shape::Cube, hardness: 0.9, faces: [304,304,304], dir_faces: [304,304,304,304], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("timber_cross") },
    Block { id: 277, name: "timber_brace", shape: Shape::Cube, hardness: 0.9, faces: [308,308,308], dir_faces: [308,308,308,308], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("timber_brace") },
    Block { id: 278, name: "wattle", shape: Shape::Cube, hardness: 0.5, faces: [312,312,312], dir_faces: [312,312,312,312], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("wattle") },
    Block { id: 279, name: "bookshelf", shape: Shape::Cube, hardness: 0.8, faces: [64,316,64], dir_faces: [316,64,64,316], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("bookshelf") },
    Block { id: 280, name: "hay_bale", shape: Shape::Cube, hardness: 0.4, faces: [320,320,320], dir_faces: [320,320,320,320], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "foliage", drop: Some("hay_bale") },
    Block { id: 281, name: "crate", shape: Shape::Cube, hardness: 0.7, faces: [324,324,324], dir_faces: [324,324,324,324], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("crate") },
    Block { id: 282, name: "limestone_masonry", shape: Shape::Cube, hardness: 1.6, faces: [352,352,352], dir_faces: [352,352,352,352], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("limestone_masonry") },
    Block { id: 283, name: "rubble_wall", shape: Shape::Cube, hardness: 1.6, faces: [356,356,356], dir_faces: [356,356,356,356], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("rubble_wall") },
    Block { id: 284, name: "barn_boards", shape: Shape::Cube, hardness: 0.8, faces: [364,364,364], dir_faces: [364,364,364,364], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("barn_boards") },
    Block { id: 285, name: "mossy_roof_tiles", shape: Shape::Cube, hardness: 0.9, faces: [368,368,368], dir_faces: [368,368,368,368], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("mossy_roof_tiles") },
    Block { id: 286, name: "dirt_path", shape: Shape::Cube, hardness: 0.5, faces: [372,8,8], dir_faces: [8,372,8,8], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("dirt") },
    Block { id: 287, name: "oak_beam", shape: Shape::Cube, hardness: 1.0, faces: [360,244,360], dir_faces: [244,360,360,244], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("oak_beam") },
    Block { id: 288, name: "oak_beam_x", shape: Shape::Cube, hardness: 1.0, faces: [244,360,244], dir_faces: [360,244,244,244], grain: [0,1,1,1], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("oak_beam") },
    Block { id: 289, name: "oak_beam_z", shape: Shape::Cube, hardness: 1.0, faces: [244,244,360], dir_faces: [244,244,244,360], grain: [1,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("oak_beam") },
    Block { id: 290, name: "planks_dark", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.55,0.45,0.4], model: None, cat: "wood", drop: Some("planks_dark") },
    Block { id: 291, name: "planks_red", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.95,0.5,0.42], model: None, cat: "wood", drop: Some("planks_red") },
    Block { id: 292, name: "planks_white", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.45,1.4,1.3], model: None, cat: "wood", drop: Some("planks_white") },
    Block { id: 293, name: "planks_blue", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.6,0.78,1.0], model: None, cat: "wood", drop: Some("planks_blue") },
    Block { id: 294, name: "planks_green", shape: Shape::Cube, hardness: 0.8, faces: [64,64,64], dir_faces: [64,64,64,64], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.62,0.85,0.58], model: None, cat: "wood", drop: Some("planks_green") },
    Block { id: 295, name: "plaster_ochre", shape: Shape::Cube, hardness: 0.7, faces: [240,240,240], dir_faces: [240,240,240,240], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,0.82,0.55], model: None, cat: "rock", drop: Some("plaster_ochre") },
    Block { id: 296, name: "plaster_rose", shape: Shape::Cube, hardness: 0.7, faces: [240,240,240], dir_faces: [240,240,240,240], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,0.8,0.76], model: None, cat: "rock", drop: Some("plaster_rose") },
    Block { id: 297, name: "plaster_blue", shape: Shape::Cube, hardness: 0.7, faces: [240,240,240], dir_faces: [240,240,240,240], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.82,0.9,1.02], model: None, cat: "rock", drop: Some("plaster_blue") },
    Block { id: 298, name: "wool_white", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "foliage", drop: Some("wool_white") },
    Block { id: 299, name: "wool_red", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.75,0.2,0.18], model: None, cat: "foliage", drop: Some("wool_red") },
    Block { id: 300, name: "wool_blue", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.28,0.4,0.8], model: None, cat: "foliage", drop: Some("wool_blue") },
    Block { id: 301, name: "wool_green", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.35,0.6,0.3], model: None, cat: "foliage", drop: Some("wool_green") },
    Block { id: 302, name: "wool_yellow", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,0.85,0.3], model: None, cat: "foliage", drop: Some("wool_yellow") },
    Block { id: 303, name: "wool_black", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.22,0.22,0.24], model: None, cat: "foliage", drop: Some("wool_black") },
    Block { id: 304, name: "wool_brown", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.55,0.38,0.25], model: None, cat: "foliage", drop: Some("wool_brown") },
    Block { id: 305, name: "wool_purple", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.55,0.3,0.65], model: None, cat: "foliage", drop: Some("wool_purple") },
    Block { id: 306, name: "wool_orange", shape: Shape::Cube, hardness: 0.3, faces: [292,292,292], dir_faces: [292,292,292,292], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,0.55,0.22], model: None, cat: "foliage", drop: Some("wool_orange") },
    Block { id: 307, name: "marble_stairs", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 308, name: "marble_stairs_1", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 309, name: "marble_stairs_2", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 310, name: "marble_stairs_3", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 311, name: "marble_stairs_top", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 312, name: "marble_stairs_top_1", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 313, name: "marble_stairs_top_2", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 314, name: "marble_stairs_top_3", shape: Shape::Deco, hardness: 1.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("marble_stairs") },
    Block { id: 315, name: "marble_slab", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_slab") },
    Block { id: 316, name: "marble_slab_top", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_slab") },
    Block { id: 317, name: "marble_vslab", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_vslab") },
    Block { id: 318, name: "marble_vslab_1", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_vslab") },
    Block { id: 319, name: "marble_vslab_2", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_vslab") },
    Block { id: 320, name: "marble_vslab_3", shape: Shape::Deco, hardness: 1.08, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("marble_vslab") },
    Block { id: 321, name: "ashlar_stairs", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 322, name: "ashlar_stairs_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 323, name: "ashlar_stairs_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 324, name: "ashlar_stairs_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 325, name: "ashlar_stairs_top", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 326, name: "ashlar_stairs_top_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 327, name: "ashlar_stairs_top_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 328, name: "ashlar_stairs_top_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("ashlar_stairs") },
    Block { id: 329, name: "ashlar_slab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_slab") },
    Block { id: 330, name: "ashlar_slab_top", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_slab") },
    Block { id: 331, name: "ashlar_vslab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_vslab") },
    Block { id: 332, name: "ashlar_vslab_1", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_vslab") },
    Block { id: 333, name: "ashlar_vslab_2", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_vslab") },
    Block { id: 334, name: "ashlar_vslab_3", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("ashlar_vslab") },
    Block { id: 335, name: "fieldstone_stairs", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 336, name: "fieldstone_stairs_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 337, name: "fieldstone_stairs_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 338, name: "fieldstone_stairs_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 339, name: "fieldstone_stairs_top", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 340, name: "fieldstone_stairs_top_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 341, name: "fieldstone_stairs_top_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 342, name: "fieldstone_stairs_top_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("fieldstone_stairs") },
    Block { id: 343, name: "fieldstone_slab", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_slab") },
    Block { id: 344, name: "fieldstone_slab_top", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_slab") },
    Block { id: 345, name: "fieldstone_vslab", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_vslab") },
    Block { id: 346, name: "fieldstone_vslab_1", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_vslab") },
    Block { id: 347, name: "fieldstone_vslab_2", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_vslab") },
    Block { id: 348, name: "fieldstone_vslab_3", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("fieldstone_vslab") },
    Block { id: 349, name: "setts_stairs", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 350, name: "setts_stairs_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 351, name: "setts_stairs_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 352, name: "setts_stairs_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 353, name: "setts_stairs_top", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 354, name: "setts_stairs_top_1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 355, name: "setts_stairs_top_2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 356, name: "setts_stairs_top_3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("setts_stairs") },
    Block { id: 357, name: "setts_slab", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_slab") },
    Block { id: 358, name: "setts_slab_top", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_slab") },
    Block { id: 359, name: "setts_vslab", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_vslab") },
    Block { id: 360, name: "setts_vslab_1", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_vslab") },
    Block { id: 361, name: "setts_vslab_2", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_vslab") },
    Block { id: 362, name: "setts_vslab_3", shape: Shape::Deco, hardness: 0.8999999999999999, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("setts_vslab") },
    Block { id: 363, name: "polished_stone_stairs", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 364, name: "polished_stone_stairs_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 365, name: "polished_stone_stairs_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 366, name: "polished_stone_stairs_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 367, name: "polished_stone_stairs_top", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 368, name: "polished_stone_stairs_top_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 369, name: "polished_stone_stairs_top_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 370, name: "polished_stone_stairs_top_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("polished_stone_stairs") },
    Block { id: 371, name: "polished_stone_slab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_slab") },
    Block { id: 372, name: "polished_stone_slab_top", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_slab") },
    Block { id: 373, name: "polished_stone_vslab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_vslab") },
    Block { id: 374, name: "polished_stone_vslab_1", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_vslab") },
    Block { id: 375, name: "polished_stone_vslab_2", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_vslab") },
    Block { id: 376, name: "polished_stone_vslab_3", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("polished_stone_vslab") },
    Block { id: 377, name: "shingles_stairs", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 378, name: "shingles_stairs_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 379, name: "shingles_stairs_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 380, name: "shingles_stairs_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 381, name: "shingles_stairs_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 382, name: "shingles_stairs_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 383, name: "shingles_stairs_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 384, name: "shingles_stairs_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("shingles_stairs") },
    Block { id: 385, name: "shingles_slab", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_slab") },
    Block { id: 386, name: "shingles_slab_top", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_slab") },
    Block { id: 387, name: "shingles_vslab", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_vslab") },
    Block { id: 388, name: "shingles_vslab_1", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_vslab") },
    Block { id: 389, name: "shingles_vslab_2", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_vslab") },
    Block { id: 390, name: "shingles_vslab_3", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("shingles_vslab") },
    Block { id: 391, name: "roof_tiles_stairs", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 392, name: "roof_tiles_stairs_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 393, name: "roof_tiles_stairs_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 394, name: "roof_tiles_stairs_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 395, name: "roof_tiles_stairs_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 396, name: "roof_tiles_stairs_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 397, name: "roof_tiles_stairs_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 398, name: "roof_tiles_stairs_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("roof_tiles_stairs") },
    Block { id: 399, name: "roof_tiles_slab", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_slab") },
    Block { id: 400, name: "roof_tiles_slab_top", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_slab") },
    Block { id: 401, name: "roof_tiles_vslab", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_vslab") },
    Block { id: 402, name: "roof_tiles_vslab_1", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_vslab") },
    Block { id: 403, name: "roof_tiles_vslab_2", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_vslab") },
    Block { id: 404, name: "roof_tiles_vslab_3", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("roof_tiles_vslab") },
    Block { id: 405, name: "slate_stairs", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 406, name: "slate_stairs_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 407, name: "slate_stairs_2", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 408, name: "slate_stairs_3", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 409, name: "slate_stairs_top", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 410, name: "slate_stairs_top_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 411, name: "slate_stairs_top_2", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 412, name: "slate_stairs_top_3", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("slate_stairs") },
    Block { id: 413, name: "slate_slab", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_slab") },
    Block { id: 414, name: "slate_slab_top", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_slab") },
    Block { id: 415, name: "slate_vslab", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_vslab") },
    Block { id: 416, name: "slate_vslab_1", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_vslab") },
    Block { id: 417, name: "slate_vslab_2", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_vslab") },
    Block { id: 418, name: "slate_vslab_3", shape: Shape::Deco, hardness: 0.66, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("slate_vslab") },
    Block { id: 419, name: "copper_roof_stairs", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 420, name: "copper_roof_stairs_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 421, name: "copper_roof_stairs_2", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 422, name: "copper_roof_stairs_3", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 423, name: "copper_roof_stairs_top", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 424, name: "copper_roof_stairs_top_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 425, name: "copper_roof_stairs_top_2", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 426, name: "copper_roof_stairs_top_3", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("copper_roof_stairs") },
    Block { id: 427, name: "copper_roof_slab", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_slab") },
    Block { id: 428, name: "copper_roof_slab_top", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_slab") },
    Block { id: 429, name: "copper_roof_vslab", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_vslab") },
    Block { id: 430, name: "copper_roof_vslab_1", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_vslab") },
    Block { id: 431, name: "copper_roof_vslab_2", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_vslab") },
    Block { id: 432, name: "copper_roof_vslab_3", shape: Shape::Deco, hardness: 0.84, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("copper_roof_vslab") },
    Block { id: 433, name: "plaster_stairs", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 434, name: "plaster_stairs_1", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 435, name: "plaster_stairs_2", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 436, name: "plaster_stairs_3", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 437, name: "plaster_stairs_top", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 438, name: "plaster_stairs_top_1", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 439, name: "plaster_stairs_top_2", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 440, name: "plaster_stairs_top_3", shape: Shape::Deco, hardness: 0.7, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("plaster_stairs") },
    Block { id: 441, name: "plaster_slab", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_slab") },
    Block { id: 442, name: "plaster_slab_top", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_slab") },
    Block { id: 443, name: "plaster_vslab", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_vslab") },
    Block { id: 444, name: "plaster_vslab_1", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_vslab") },
    Block { id: 445, name: "plaster_vslab_2", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_vslab") },
    Block { id: 446, name: "plaster_vslab_3", shape: Shape::Deco, hardness: 0.42, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("plaster_vslab") },
    Block { id: 447, name: "timber_stairs", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 448, name: "timber_stairs_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 449, name: "timber_stairs_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 450, name: "timber_stairs_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 451, name: "timber_stairs_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 452, name: "timber_stairs_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 453, name: "timber_stairs_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 454, name: "timber_stairs_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("timber_stairs") },
    Block { id: 455, name: "timber_slab", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_slab") },
    Block { id: 456, name: "timber_slab_top", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_slab") },
    Block { id: 457, name: "timber_vslab", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_vslab") },
    Block { id: 458, name: "timber_vslab_1", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_vslab") },
    Block { id: 459, name: "timber_vslab_2", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_vslab") },
    Block { id: 460, name: "timber_vslab_3", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("timber_vslab") },
    Block { id: 461, name: "limestone_masonry_stairs", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_masonry_stairs") },
    Block { id: 462, name: "limestone_masonry_stairs_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_masonry_stairs") },
    Block { id: 463, name: "limestone_masonry_stairs_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_masonry_stairs") },
    Block { id: 464, name: "limestone_masonry_stairs_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_masonry_stairs") },
    Block { id: 465, name: "limestone_masonry_stairs_top", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_masonry_stairs") },
    Block { id: 466, name: "limestone_masonry_stairs_top_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_masonry_stairs") },
    Block { id: 467, name: "limestone_masonry_stairs_top_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_masonry_stairs") },
    Block { id: 468, name: "limestone_masonry_stairs_top_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_masonry_stairs") },
    Block { id: 469, name: "limestone_masonry_slab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_masonry_slab") },
    Block { id: 470, name: "limestone_masonry_slab_top", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_masonry_slab") },
    Block { id: 471, name: "limestone_masonry_vslab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_masonry_vslab") },
    Block { id: 472, name: "limestone_masonry_vslab_1", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_masonry_vslab") },
    Block { id: 473, name: "limestone_masonry_vslab_2", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_masonry_vslab") },
    Block { id: 474, name: "limestone_masonry_vslab_3", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_masonry_vslab") },
    Block { id: 475, name: "rubble_wall_stairs", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("rubble_wall_stairs") },
    Block { id: 476, name: "rubble_wall_stairs_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("rubble_wall_stairs") },
    Block { id: 477, name: "rubble_wall_stairs_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("rubble_wall_stairs") },
    Block { id: 478, name: "rubble_wall_stairs_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("rubble_wall_stairs") },
    Block { id: 479, name: "rubble_wall_stairs_top", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("rubble_wall_stairs") },
    Block { id: 480, name: "rubble_wall_stairs_top_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("rubble_wall_stairs") },
    Block { id: 481, name: "rubble_wall_stairs_top_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("rubble_wall_stairs") },
    Block { id: 482, name: "rubble_wall_stairs_top_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("rubble_wall_stairs") },
    Block { id: 483, name: "rubble_wall_slab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("rubble_wall_slab") },
    Block { id: 484, name: "rubble_wall_slab_top", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("rubble_wall_slab") },
    Block { id: 485, name: "rubble_wall_vslab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("rubble_wall_vslab") },
    Block { id: 486, name: "rubble_wall_vslab_1", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("rubble_wall_vslab") },
    Block { id: 487, name: "rubble_wall_vslab_2", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("rubble_wall_vslab") },
    Block { id: 488, name: "rubble_wall_vslab_3", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("rubble_wall_vslab") },
    Block { id: 489, name: "mossy_roof_tiles_stairs", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("mossy_roof_tiles_stairs") },
    Block { id: 490, name: "mossy_roof_tiles_stairs_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("mossy_roof_tiles_stairs") },
    Block { id: 491, name: "mossy_roof_tiles_stairs_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("mossy_roof_tiles_stairs") },
    Block { id: 492, name: "mossy_roof_tiles_stairs_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("mossy_roof_tiles_stairs") },
    Block { id: 493, name: "mossy_roof_tiles_stairs_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("mossy_roof_tiles_stairs") },
    Block { id: 494, name: "mossy_roof_tiles_stairs_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("mossy_roof_tiles_stairs") },
    Block { id: 495, name: "mossy_roof_tiles_stairs_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("mossy_roof_tiles_stairs") },
    Block { id: 496, name: "mossy_roof_tiles_stairs_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("mossy_roof_tiles_stairs") },
    Block { id: 497, name: "mossy_roof_tiles_slab", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("mossy_roof_tiles_slab") },
    Block { id: 498, name: "mossy_roof_tiles_slab_top", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("mossy_roof_tiles_slab") },
    Block { id: 499, name: "mossy_roof_tiles_vslab", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("mossy_roof_tiles_vslab") },
    Block { id: 500, name: "mossy_roof_tiles_vslab_1", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("mossy_roof_tiles_vslab") },
    Block { id: 501, name: "mossy_roof_tiles_vslab_2", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("mossy_roof_tiles_vslab") },
    Block { id: 502, name: "mossy_roof_tiles_vslab_3", shape: Shape::Deco, hardness: 0.54, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("mossy_roof_tiles_vslab") },
    Block { id: 503, name: "barn_boards_stairs", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("barn_boards_stairs") },
    Block { id: 504, name: "barn_boards_stairs_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("barn_boards_stairs") },
    Block { id: 505, name: "barn_boards_stairs_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("barn_boards_stairs") },
    Block { id: 506, name: "barn_boards_stairs_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("barn_boards_stairs") },
    Block { id: 507, name: "barn_boards_stairs_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("barn_boards_stairs") },
    Block { id: 508, name: "barn_boards_stairs_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("barn_boards_stairs") },
    Block { id: 509, name: "barn_boards_stairs_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("barn_boards_stairs") },
    Block { id: 510, name: "barn_boards_stairs_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "wood", drop: Some("barn_boards_stairs") },
    Block { id: 511, name: "barn_boards_slab", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("barn_boards_slab") },
    Block { id: 512, name: "barn_boards_slab_top", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("barn_boards_slab") },
    Block { id: 513, name: "barn_boards_vslab", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("barn_boards_vslab") },
    Block { id: 514, name: "barn_boards_vslab_1", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("barn_boards_vslab") },
    Block { id: 515, name: "barn_boards_vslab_2", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("barn_boards_vslab") },
    Block { id: 516, name: "barn_boards_vslab_3", shape: Shape::Deco, hardness: 0.48, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "wood", drop: Some("barn_boards_vslab") },
    Block { id: 517, name: "roof_tiles_slope", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_slope") },
    Block { id: 518, name: "roof_tiles_slope_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_slope") },
    Block { id: 519, name: "roof_tiles_slope_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_slope") },
    Block { id: 520, name: "roof_tiles_slope_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_slope") },
    Block { id: 521, name: "roof_tiles_slope_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_slope") },
    Block { id: 522, name: "roof_tiles_slope_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_slope") },
    Block { id: 523, name: "roof_tiles_slope_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_slope") },
    Block { id: 524, name: "roof_tiles_slope_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_slope") },
    Block { id: 525, name: "roof_tiles_corner", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_corner") },
    Block { id: 526, name: "roof_tiles_corner_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_corner") },
    Block { id: 527, name: "roof_tiles_corner_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_corner") },
    Block { id: 528, name: "roof_tiles_corner_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_corner") },
    Block { id: 529, name: "roof_tiles_corner_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_corner") },
    Block { id: 530, name: "roof_tiles_corner_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_corner") },
    Block { id: 531, name: "roof_tiles_corner_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_corner") },
    Block { id: 532, name: "roof_tiles_corner_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_corner") },
    Block { id: 533, name: "roof_tiles_valley", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_valley") },
    Block { id: 534, name: "roof_tiles_valley_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_valley") },
    Block { id: 535, name: "roof_tiles_valley_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_valley") },
    Block { id: 536, name: "roof_tiles_valley_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_valley") },
    Block { id: 537, name: "roof_tiles_valley_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_valley") },
    Block { id: 538, name: "roof_tiles_valley_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_valley") },
    Block { id: 539, name: "roof_tiles_valley_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_valley") },
    Block { id: 540, name: "roof_tiles_valley_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_valley") },
    Block { id: 541, name: "roof_tiles_ridge", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_ridge") },
    Block { id: 542, name: "roof_tiles_ridge_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_ridge") },
    Block { id: 543, name: "roof_tiles_ridge_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_ridge") },
    Block { id: 544, name: "roof_tiles_ridge_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_ridge") },
    Block { id: 545, name: "roof_tiles_peak", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_peak") },
    Block { id: 546, name: "roof_tiles_peak_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("roof_tiles_peak") },
    Block { id: 547, name: "slate_slope", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_slope") },
    Block { id: 548, name: "slate_slope_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_slope") },
    Block { id: 549, name: "slate_slope_2", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_slope") },
    Block { id: 550, name: "slate_slope_3", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_slope") },
    Block { id: 551, name: "slate_slope_top", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_slope") },
    Block { id: 552, name: "slate_slope_top_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_slope") },
    Block { id: 553, name: "slate_slope_top_2", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_slope") },
    Block { id: 554, name: "slate_slope_top_3", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_slope") },
    Block { id: 555, name: "slate_corner", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_corner") },
    Block { id: 556, name: "slate_corner_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_corner") },
    Block { id: 557, name: "slate_corner_2", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_corner") },
    Block { id: 558, name: "slate_corner_3", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_corner") },
    Block { id: 559, name: "slate_corner_top", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_corner") },
    Block { id: 560, name: "slate_corner_top_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_corner") },
    Block { id: 561, name: "slate_corner_top_2", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_corner") },
    Block { id: 562, name: "slate_corner_top_3", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_corner") },
    Block { id: 563, name: "slate_valley", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_valley") },
    Block { id: 564, name: "slate_valley_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_valley") },
    Block { id: 565, name: "slate_valley_2", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_valley") },
    Block { id: 566, name: "slate_valley_3", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_valley") },
    Block { id: 567, name: "slate_valley_top", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_valley") },
    Block { id: 568, name: "slate_valley_top_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_valley") },
    Block { id: 569, name: "slate_valley_top_2", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_valley") },
    Block { id: 570, name: "slate_valley_top_3", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_valley") },
    Block { id: 571, name: "slate_ridge", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_ridge") },
    Block { id: 572, name: "slate_ridge_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_ridge") },
    Block { id: 573, name: "slate_ridge_top", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_ridge") },
    Block { id: 574, name: "slate_ridge_top_1", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_ridge") },
    Block { id: 575, name: "slate_peak", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_peak") },
    Block { id: 576, name: "slate_peak_top", shape: Shape::Deco, hardness: 1.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("slate_peak") },
    Block { id: 577, name: "mossy_roof_tiles_slope", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_slope") },
    Block { id: 578, name: "mossy_roof_tiles_slope_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_slope") },
    Block { id: 579, name: "mossy_roof_tiles_slope_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_slope") },
    Block { id: 580, name: "mossy_roof_tiles_slope_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_slope") },
    Block { id: 581, name: "mossy_roof_tiles_slope_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_slope") },
    Block { id: 582, name: "mossy_roof_tiles_slope_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_slope") },
    Block { id: 583, name: "mossy_roof_tiles_slope_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_slope") },
    Block { id: 584, name: "mossy_roof_tiles_slope_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_slope") },
    Block { id: 585, name: "mossy_roof_tiles_corner", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_corner") },
    Block { id: 586, name: "mossy_roof_tiles_corner_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_corner") },
    Block { id: 587, name: "mossy_roof_tiles_corner_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_corner") },
    Block { id: 588, name: "mossy_roof_tiles_corner_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_corner") },
    Block { id: 589, name: "mossy_roof_tiles_corner_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_corner") },
    Block { id: 590, name: "mossy_roof_tiles_corner_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_corner") },
    Block { id: 591, name: "mossy_roof_tiles_corner_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_corner") },
    Block { id: 592, name: "mossy_roof_tiles_corner_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_corner") },
    Block { id: 593, name: "mossy_roof_tiles_valley", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_valley") },
    Block { id: 594, name: "mossy_roof_tiles_valley_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_valley") },
    Block { id: 595, name: "mossy_roof_tiles_valley_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_valley") },
    Block { id: 596, name: "mossy_roof_tiles_valley_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_valley") },
    Block { id: 597, name: "mossy_roof_tiles_valley_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_valley") },
    Block { id: 598, name: "mossy_roof_tiles_valley_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_valley") },
    Block { id: 599, name: "mossy_roof_tiles_valley_top_2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_valley") },
    Block { id: 600, name: "mossy_roof_tiles_valley_top_3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_valley") },
    Block { id: 601, name: "mossy_roof_tiles_ridge", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_ridge") },
    Block { id: 602, name: "mossy_roof_tiles_ridge_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_ridge") },
    Block { id: 603, name: "mossy_roof_tiles_ridge_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_ridge") },
    Block { id: 604, name: "mossy_roof_tiles_ridge_top_1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_ridge") },
    Block { id: 605, name: "mossy_roof_tiles_peak", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_peak") },
    Block { id: 606, name: "mossy_roof_tiles_peak_top", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("mossy_roof_tiles_peak") },
    Block { id: 607, name: "shingles_slope", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_slope") },
    Block { id: 608, name: "shingles_slope_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_slope") },
    Block { id: 609, name: "shingles_slope_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_slope") },
    Block { id: 610, name: "shingles_slope_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_slope") },
    Block { id: 611, name: "shingles_slope_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_slope") },
    Block { id: 612, name: "shingles_slope_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_slope") },
    Block { id: 613, name: "shingles_slope_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_slope") },
    Block { id: 614, name: "shingles_slope_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_slope") },
    Block { id: 615, name: "shingles_corner", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_corner") },
    Block { id: 616, name: "shingles_corner_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_corner") },
    Block { id: 617, name: "shingles_corner_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_corner") },
    Block { id: 618, name: "shingles_corner_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_corner") },
    Block { id: 619, name: "shingles_corner_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_corner") },
    Block { id: 620, name: "shingles_corner_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_corner") },
    Block { id: 621, name: "shingles_corner_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_corner") },
    Block { id: 622, name: "shingles_corner_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_corner") },
    Block { id: 623, name: "shingles_valley", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_valley") },
    Block { id: 624, name: "shingles_valley_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_valley") },
    Block { id: 625, name: "shingles_valley_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_valley") },
    Block { id: 626, name: "shingles_valley_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_valley") },
    Block { id: 627, name: "shingles_valley_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_valley") },
    Block { id: 628, name: "shingles_valley_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_valley") },
    Block { id: 629, name: "shingles_valley_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_valley") },
    Block { id: 630, name: "shingles_valley_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_valley") },
    Block { id: 631, name: "shingles_ridge", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_ridge") },
    Block { id: 632, name: "shingles_ridge_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_ridge") },
    Block { id: 633, name: "shingles_ridge_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_ridge") },
    Block { id: 634, name: "shingles_ridge_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_ridge") },
    Block { id: 635, name: "shingles_peak", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_peak") },
    Block { id: 636, name: "shingles_peak_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("shingles_peak") },
    Block { id: 637, name: "thatch_slope", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_slope") },
    Block { id: 638, name: "thatch_slope_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_slope") },
    Block { id: 639, name: "thatch_slope_2", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_slope") },
    Block { id: 640, name: "thatch_slope_3", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_slope") },
    Block { id: 641, name: "thatch_slope_top", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_slope") },
    Block { id: 642, name: "thatch_slope_top_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_slope") },
    Block { id: 643, name: "thatch_slope_top_2", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_slope") },
    Block { id: 644, name: "thatch_slope_top_3", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_slope") },
    Block { id: 645, name: "thatch_corner", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_corner") },
    Block { id: 646, name: "thatch_corner_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_corner") },
    Block { id: 647, name: "thatch_corner_2", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_corner") },
    Block { id: 648, name: "thatch_corner_3", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_corner") },
    Block { id: 649, name: "thatch_corner_top", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_corner") },
    Block { id: 650, name: "thatch_corner_top_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_corner") },
    Block { id: 651, name: "thatch_corner_top_2", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_corner") },
    Block { id: 652, name: "thatch_corner_top_3", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_corner") },
    Block { id: 653, name: "thatch_valley", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_valley") },
    Block { id: 654, name: "thatch_valley_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_valley") },
    Block { id: 655, name: "thatch_valley_2", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_valley") },
    Block { id: 656, name: "thatch_valley_3", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_valley") },
    Block { id: 657, name: "thatch_valley_top", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_valley") },
    Block { id: 658, name: "thatch_valley_top_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_valley") },
    Block { id: 659, name: "thatch_valley_top_2", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_valley") },
    Block { id: 660, name: "thatch_valley_top_3", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_valley") },
    Block { id: 661, name: "thatch_ridge", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_ridge") },
    Block { id: 662, name: "thatch_ridge_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_ridge") },
    Block { id: 663, name: "thatch_ridge_top", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_ridge") },
    Block { id: 664, name: "thatch_ridge_top_1", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_ridge") },
    Block { id: 665, name: "thatch_peak", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_peak") },
    Block { id: 666, name: "thatch_peak_top", shape: Shape::Deco, hardness: 0.35, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "foliage", drop: Some("thatch_peak") },
    Block { id: 667, name: "copper_roof_slope", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_slope") },
    Block { id: 668, name: "copper_roof_slope_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_slope") },
    Block { id: 669, name: "copper_roof_slope_2", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_slope") },
    Block { id: 670, name: "copper_roof_slope_3", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_slope") },
    Block { id: 671, name: "copper_roof_slope_top", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_slope") },
    Block { id: 672, name: "copper_roof_slope_top_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_slope") },
    Block { id: 673, name: "copper_roof_slope_top_2", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_slope") },
    Block { id: 674, name: "copper_roof_slope_top_3", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_slope") },
    Block { id: 675, name: "copper_roof_corner", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_corner") },
    Block { id: 676, name: "copper_roof_corner_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_corner") },
    Block { id: 677, name: "copper_roof_corner_2", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_corner") },
    Block { id: 678, name: "copper_roof_corner_3", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_corner") },
    Block { id: 679, name: "copper_roof_corner_top", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_corner") },
    Block { id: 680, name: "copper_roof_corner_top_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_corner") },
    Block { id: 681, name: "copper_roof_corner_top_2", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_corner") },
    Block { id: 682, name: "copper_roof_corner_top_3", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_corner") },
    Block { id: 683, name: "copper_roof_valley", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_valley") },
    Block { id: 684, name: "copper_roof_valley_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_valley") },
    Block { id: 685, name: "copper_roof_valley_2", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_valley") },
    Block { id: 686, name: "copper_roof_valley_3", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_valley") },
    Block { id: 687, name: "copper_roof_valley_top", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_valley") },
    Block { id: 688, name: "copper_roof_valley_top_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_valley") },
    Block { id: 689, name: "copper_roof_valley_top_2", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_valley") },
    Block { id: 690, name: "copper_roof_valley_top_3", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_valley") },
    Block { id: 691, name: "copper_roof_ridge", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_ridge") },
    Block { id: 692, name: "copper_roof_ridge_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_ridge") },
    Block { id: 693, name: "copper_roof_ridge_top", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_ridge") },
    Block { id: 694, name: "copper_roof_ridge_top_1", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_ridge") },
    Block { id: 695, name: "copper_roof_peak", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_peak") },
    Block { id: 696, name: "copper_roof_peak_top", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "rock", drop: Some("copper_roof_peak") },
    Block { id: 697, name: "planks_slope", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_slope") },
    Block { id: 698, name: "planks_slope_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_slope") },
    Block { id: 699, name: "planks_slope_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_slope") },
    Block { id: 700, name: "planks_slope_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_slope") },
    Block { id: 701, name: "planks_slope_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_slope") },
    Block { id: 702, name: "planks_slope_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_slope") },
    Block { id: 703, name: "planks_slope_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_slope") },
    Block { id: 704, name: "planks_slope_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_slope") },
    Block { id: 705, name: "planks_corner", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_corner") },
    Block { id: 706, name: "planks_corner_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_corner") },
    Block { id: 707, name: "planks_corner_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_corner") },
    Block { id: 708, name: "planks_corner_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_corner") },
    Block { id: 709, name: "planks_corner_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_corner") },
    Block { id: 710, name: "planks_corner_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_corner") },
    Block { id: 711, name: "planks_corner_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_corner") },
    Block { id: 712, name: "planks_corner_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_corner") },
    Block { id: 713, name: "planks_valley", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_valley") },
    Block { id: 714, name: "planks_valley_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_valley") },
    Block { id: 715, name: "planks_valley_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_valley") },
    Block { id: 716, name: "planks_valley_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_valley") },
    Block { id: 717, name: "planks_valley_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_valley") },
    Block { id: 718, name: "planks_valley_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_valley") },
    Block { id: 719, name: "planks_valley_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_valley") },
    Block { id: 720, name: "planks_valley_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_valley") },
    Block { id: 721, name: "planks_ridge", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_ridge") },
    Block { id: 722, name: "planks_ridge_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_ridge") },
    Block { id: 723, name: "planks_ridge_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_ridge") },
    Block { id: 724, name: "planks_ridge_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_ridge") },
    Block { id: 725, name: "planks_peak", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_peak") },
    Block { id: 726, name: "planks_peak_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slope"), cat: "wood", drop: Some("planks_peak") },
    Block { id: 727, name: "chair", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chair"), cat: "wood", drop: Some("chair") },
    Block { id: 728, name: "chair@1", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chair"), cat: "wood", drop: Some("chair") },
    Block { id: 729, name: "chair@2", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chair"), cat: "wood", drop: Some("chair") },
    Block { id: 730, name: "chair@3", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chair"), cat: "wood", drop: Some("chair") },
    Block { id: 731, name: "bench", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bench"), cat: "wood", drop: Some("bench") },
    Block { id: 732, name: "bench@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bench"), cat: "wood", drop: Some("bench") },
    Block { id: 733, name: "bench@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bench"), cat: "wood", drop: Some("bench") },
    Block { id: 734, name: "bench@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bench"), cat: "wood", drop: Some("bench") },
    Block { id: 735, name: "armchair", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("armchair"), cat: "wood", drop: Some("armchair") },
    Block { id: 736, name: "armchair@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("armchair"), cat: "wood", drop: Some("armchair") },
    Block { id: 737, name: "armchair@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("armchair"), cat: "wood", drop: Some("armchair") },
    Block { id: 738, name: "armchair@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("armchair"), cat: "wood", drop: Some("armchair") },
    Block { id: 739, name: "throne", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("throne"), cat: "wood", drop: Some("throne") },
    Block { id: 740, name: "throne@1", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("throne"), cat: "wood", drop: Some("throne") },
    Block { id: 741, name: "throne@2", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("throne"), cat: "wood", drop: Some("throne") },
    Block { id: 742, name: "throne@3", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("throne"), cat: "wood", drop: Some("throne") },
    Block { id: 743, name: "cupboard", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cupboard"), cat: "wood", drop: Some("cupboard") },
    Block { id: 744, name: "cupboard@1", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cupboard"), cat: "wood", drop: Some("cupboard") },
    Block { id: 745, name: "cupboard@2", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cupboard"), cat: "wood", drop: Some("cupboard") },
    Block { id: 746, name: "cupboard@3", shape: Shape::Deco, hardness: 0.9, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cupboard"), cat: "wood", drop: Some("cupboard") },
    Block { id: 747, name: "fireplace", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("fireplace"), cat: "wood", drop: Some("fireplace") },
    Block { id: 748, name: "fireplace@1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("fireplace"), cat: "wood", drop: Some("fireplace") },
    Block { id: 749, name: "fireplace@2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("fireplace"), cat: "wood", drop: Some("fireplace") },
    Block { id: 750, name: "fireplace@3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("fireplace"), cat: "wood", drop: Some("fireplace") },
    Block { id: 751, name: "loom", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("loom"), cat: "wood", drop: Some("loom") },
    Block { id: 752, name: "loom@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("loom"), cat: "wood", drop: Some("loom") },
    Block { id: 753, name: "loom@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("loom"), cat: "wood", drop: Some("loom") },
    Block { id: 754, name: "loom@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("loom"), cat: "wood", drop: Some("loom") },
    Block { id: 755, name: "spinning_wheel", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("spinning_wheel"), cat: "wood", drop: Some("spinning_wheel") },
    Block { id: 756, name: "spinning_wheel@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("spinning_wheel"), cat: "wood", drop: Some("spinning_wheel") },
    Block { id: 757, name: "spinning_wheel@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("spinning_wheel"), cat: "wood", drop: Some("spinning_wheel") },
    Block { id: 758, name: "spinning_wheel@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("spinning_wheel"), cat: "wood", drop: Some("spinning_wheel") },
    Block { id: 759, name: "grindstone", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grindstone"), cat: "wood", drop: Some("grindstone") },
    Block { id: 760, name: "grindstone@1", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grindstone"), cat: "wood", drop: Some("grindstone") },
    Block { id: 761, name: "grindstone@2", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grindstone"), cat: "wood", drop: Some("grindstone") },
    Block { id: 762, name: "grindstone@3", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grindstone"), cat: "wood", drop: Some("grindstone") },
    Block { id: 763, name: "signpost", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("signpost"), cat: "wood", drop: Some("signpost") },
    Block { id: 764, name: "signpost@1", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("signpost"), cat: "wood", drop: Some("signpost") },
    Block { id: 765, name: "signpost@2", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("signpost"), cat: "wood", drop: Some("signpost") },
    Block { id: 766, name: "signpost@3", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("signpost"), cat: "wood", drop: Some("signpost") },
    Block { id: 767, name: "painting", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("painting"), cat: "wood", drop: Some("painting") },
    Block { id: 768, name: "painting@1", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("painting"), cat: "wood", drop: Some("painting") },
    Block { id: 769, name: "painting@2", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("painting"), cat: "wood", drop: Some("painting") },
    Block { id: 770, name: "painting@3", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("painting"), cat: "wood", drop: Some("painting") },
    Block { id: 771, name: "banner_red", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_red"), cat: "wood", drop: Some("banner_red") },
    Block { id: 772, name: "banner_red@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_red"), cat: "wood", drop: Some("banner_red") },
    Block { id: 773, name: "banner_red@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_red"), cat: "wood", drop: Some("banner_red") },
    Block { id: 774, name: "banner_red@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_red"), cat: "wood", drop: Some("banner_red") },
    Block { id: 775, name: "banner_blue", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_blue"), cat: "wood", drop: Some("banner_blue") },
    Block { id: 776, name: "banner_blue@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_blue"), cat: "wood", drop: Some("banner_blue") },
    Block { id: 777, name: "banner_blue@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_blue"), cat: "wood", drop: Some("banner_blue") },
    Block { id: 778, name: "banner_blue@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_blue"), cat: "wood", drop: Some("banner_blue") },
    Block { id: 779, name: "banner_green", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_green"), cat: "wood", drop: Some("banner_green") },
    Block { id: 780, name: "banner_green@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_green"), cat: "wood", drop: Some("banner_green") },
    Block { id: 781, name: "banner_green@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_green"), cat: "wood", drop: Some("banner_green") },
    Block { id: 782, name: "banner_green@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_green"), cat: "wood", drop: Some("banner_green") },
    Block { id: 783, name: "banner_gold", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_gold"), cat: "wood", drop: Some("banner_gold") },
    Block { id: 784, name: "banner_gold@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_gold"), cat: "wood", drop: Some("banner_gold") },
    Block { id: 785, name: "banner_gold@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_gold"), cat: "wood", drop: Some("banner_gold") },
    Block { id: 786, name: "banner_gold@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("banner_gold"), cat: "wood", drop: Some("banner_gold") },
    Block { id: 787, name: "weapon_rack", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("weapon_rack"), cat: "wood", drop: Some("weapon_rack") },
    Block { id: 788, name: "weapon_rack@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("weapon_rack"), cat: "wood", drop: Some("weapon_rack") },
    Block { id: 789, name: "weapon_rack@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("weapon_rack"), cat: "wood", drop: Some("weapon_rack") },
    Block { id: 790, name: "weapon_rack@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("weapon_rack"), cat: "wood", drop: Some("weapon_rack") },
    Block { id: 791, name: "wall_sconce", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 11, tint: [1.0,1.0,1.0], model: Some("wall_sconce"), cat: "wood", drop: Some("wall_sconce") },
    Block { id: 792, name: "wall_sconce@1", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 11, tint: [1.0,1.0,1.0], model: Some("wall_sconce"), cat: "wood", drop: Some("wall_sconce") },
    Block { id: 793, name: "wall_sconce@2", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 11, tint: [1.0,1.0,1.0], model: Some("wall_sconce"), cat: "wood", drop: Some("wall_sconce") },
    Block { id: 794, name: "wall_sconce@3", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 11, tint: [1.0,1.0,1.0], model: Some("wall_sconce"), cat: "wood", drop: Some("wall_sconce") },
    Block { id: 795, name: "curtain", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("curtain"), cat: "wood", drop: Some("curtain") },
    Block { id: 796, name: "curtain@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("curtain"), cat: "wood", drop: Some("curtain") },
    Block { id: 797, name: "curtain@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("curtain"), cat: "wood", drop: Some("curtain") },
    Block { id: 798, name: "curtain@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("curtain"), cat: "wood", drop: Some("curtain") },
    Block { id: 799, name: "wall_shelf", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_shelf"), cat: "wood", drop: Some("wall_shelf") },
    Block { id: 800, name: "wall_shelf@1", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_shelf"), cat: "wood", drop: Some("wall_shelf") },
    Block { id: 801, name: "wall_shelf@2", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_shelf"), cat: "wood", drop: Some("wall_shelf") },
    Block { id: 802, name: "wall_shelf@3", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_shelf"), cat: "wood", drop: Some("wall_shelf") },
    Block { id: 803, name: "table", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("table"), cat: "wood", drop: Some("table") },
    Block { id: 804, name: "round_table", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("round_table"), cat: "wood", drop: Some("round_table") },
    Block { id: 805, name: "stool", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stool"), cat: "wood", drop: Some("stool") },
    Block { id: 806, name: "candle", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 9, tint: [1.0,1.0,1.0], model: Some("candle"), cat: "wood", drop: Some("candle") },
    Block { id: 807, name: "candelabra", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 12, tint: [1.0,1.0,1.0], model: Some("candelabra"), cat: "wood", drop: Some("candelabra") },
    Block { id: 808, name: "chandelier", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 14, tint: [1.0,1.0,1.0], model: Some("chandelier"), cat: "wood", drop: Some("chandelier") },
    Block { id: 809, name: "brazier", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 14, tint: [1.0,1.0,1.0], model: Some("brazier"), cat: "wood", drop: Some("brazier") },
    Block { id: 810, name: "sack", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sack"), cat: "wood", drop: Some("sack") },
    Block { id: 811, name: "amphora", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("amphora"), cat: "wood", drop: Some("amphora") },
    Block { id: 812, name: "flower_pot", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("flower_pot"), cat: "wood", drop: Some("flower_pot") },
    Block { id: 813, name: "potted_fern", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("potted_fern"), cat: "wood", drop: Some("potted_fern") },
    Block { id: 814, name: "cauldron", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cauldron"), cat: "wood", drop: Some("cauldron") },
    Block { id: 815, name: "bucket", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bucket"), cat: "wood", drop: Some("bucket") },
    Block { id: 816, name: "log_pile", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("log_pile"), cat: "wood", drop: Some("log_pile") },
    Block { id: 817, name: "carpet_red", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carpet_red"), cat: "wood", drop: Some("carpet_red") },
    Block { id: 818, name: "carpet_blue", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carpet_blue"), cat: "wood", drop: Some("carpet_blue") },
    Block { id: 819, name: "carpet_green", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("carpet_green"), cat: "wood", drop: Some("carpet_green") },
    Block { id: 820, name: "iron_bars", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("iron_bars"), cat: "rock", drop: Some("iron_bars") },
    Block { id: 821, name: "books_pile", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("books_pile"), cat: "wood", drop: Some("books_pile") },
    Block { id: 822, name: "bottles", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bottles"), cat: "wood", drop: Some("bottles") },
    Block { id: 823, name: "beehive", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("beehive"), cat: "wood", drop: Some("beehive") },
    Block { id: 824, name: "cheese", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cheese"), cat: "wood", drop: Some("cheese") },
    Block { id: 825, name: "bread_basket", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bread_basket"), cat: "wood", drop: Some("bread_basket") },
    Block { id: 826, name: "torch_wall", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("torch_wall"), cat: "light", drop: Some("torch") },
    Block { id: 827, name: "torch_wall@1", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("torch_wall"), cat: "light", drop: Some("torch") },
    Block { id: 828, name: "torch_wall@2", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("torch_wall"), cat: "light", drop: Some("torch") },
    Block { id: 829, name: "torch_wall@3", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 13, tint: [1.0,1.0,1.0], model: Some("torch_wall"), cat: "light", drop: Some("torch") },
    Block { id: 830, name: "bed@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed"), cat: "wood", drop: Some("bed") },
    Block { id: 831, name: "bed@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed"), cat: "wood", drop: Some("bed") },
    Block { id: 832, name: "bed@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed"), cat: "wood", drop: Some("bed") },
    Block { id: 833, name: "bed_foot", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed_foot"), cat: "wood", drop: Some("bed") },
    Block { id: 834, name: "bed_foot@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed_foot"), cat: "wood", drop: Some("bed") },
    Block { id: 835, name: "bed_foot@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed_foot"), cat: "wood", drop: Some("bed") },
    Block { id: 836, name: "bed_foot@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bed_foot"), cat: "wood", drop: Some("bed") },
    Block { id: 837, name: "chest_open", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chest_open"), cat: "wood", drop: Some("chest") },
    Block { id: 838, name: "chest@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chest"), cat: "wood", drop: Some("chest") },
    Block { id: 839, name: "chest_open@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chest_open"), cat: "wood", drop: Some("chest") },
    Block { id: 840, name: "chest@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chest"), cat: "wood", drop: Some("chest") },
    Block { id: 841, name: "chest_open@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chest_open"), cat: "wood", drop: Some("chest") },
    Block { id: 842, name: "chest@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chest"), cat: "wood", drop: Some("chest") },
    Block { id: 843, name: "chest_open@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chest_open"), cat: "wood", drop: Some("chest") },
    Block { id: 844, name: "furnace_lit", shape: Shape::Deco, hardness: 1.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 14, tint: [1.0,1.0,1.0], model: Some("furnace"), cat: "rock", drop: Some("furnace") },
    Block { id: 845, name: "edelweiss", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("edelweiss"), cat: "decor", drop: None },
    Block { id: 846, name: "gentian", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("gentian"), cat: "decor", drop: None },
    Block { id: 847, name: "seagrass", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("seagrass"), cat: "decor", drop: None },
    Block { id: 848, name: "kelp", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("kelp"), cat: "decor", drop: None },
    Block { id: 849, name: "kelp_top", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("kelp_top"), cat: "decor", drop: None },
    Block { id: 850, name: "coral", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("coral"), cat: "decor", drop: None },
    Block { id: 851, name: "coral_fan", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("coral_fan"), cat: "decor", drop: None },
    Block { id: 852, name: "shell", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("shell"), cat: "decor", drop: Some("shell") },
    Block { id: 853, name: "marram", shape: Shape::Deco, hardness: 0.02, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("marram"), cat: "decor", drop: None },
    Block { id: 854, name: "coral_block_red", shape: Shape::Cube, hardness: 0.9, faces: [344,344,344], dir_faces: [344,344,344,344], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.06,0.44,0.4], model: None, cat: "rock", drop: Some("coral_block_red") },
    Block { id: 855, name: "coral_block_orange", shape: Shape::Cube, hardness: 0.9, faces: [344,344,344], dir_faces: [344,344,344,344], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.1,0.62,0.26], model: None, cat: "rock", drop: Some("coral_block_orange") },
    Block { id: 856, name: "coral_block_purple", shape: Shape::Cube, hardness: 0.9, faces: [344,344,344], dir_faces: [344,344,344,344], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.78,0.4,0.94], model: None, cat: "rock", drop: Some("coral_block_purple") },
    Block { id: 857, name: "coral_block_yellow", shape: Shape::Cube, hardness: 0.9, faces: [344,344,344], dir_faces: [344,344,344,344], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.1,0.94,0.42], model: None, cat: "rock", drop: Some("coral_block_yellow") },
    Block { id: 858, name: "coral_block_blue", shape: Shape::Cube, hardness: 0.9, faces: [344,344,344], dir_faces: [344,344,344,344], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.4,0.8,0.98], model: None, cat: "rock", drop: Some("coral_block_blue") },
    Block { id: 859, name: "wreck_planks", shape: Shape::Cube, hardness: 0.8, faces: [348,348,348], dir_faces: [348,348,348,348], grain: [0,2,2,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("wreck_planks") },
    Block { id: 860, name: "wall_clock", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_clock"), cat: "wood", drop: Some("wall_clock") },
    Block { id: 861, name: "wall_clock@1", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_clock"), cat: "wood", drop: Some("wall_clock") },
    Block { id: 862, name: "wall_clock@2", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_clock"), cat: "wood", drop: Some("wall_clock") },
    Block { id: 863, name: "wall_clock@3", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("wall_clock"), cat: "wood", drop: Some("wall_clock") },
    Block { id: 864, name: "hides", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hides"), cat: "wood", drop: Some("hides") },
    Block { id: 865, name: "hides@1", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hides"), cat: "wood", drop: Some("hides") },
    Block { id: 866, name: "hides@2", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hides"), cat: "wood", drop: Some("hides") },
    Block { id: 867, name: "hides@3", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("hides"), cat: "wood", drop: Some("hides") },
    Block { id: 868, name: "sawhorse", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sawhorse"), cat: "wood", drop: Some("sawhorse") },
    Block { id: 869, name: "sawhorse@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sawhorse"), cat: "wood", drop: Some("sawhorse") },
    Block { id: 870, name: "sawhorse@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sawhorse"), cat: "wood", drop: Some("sawhorse") },
    Block { id: 871, name: "sawhorse@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sawhorse"), cat: "wood", drop: Some("sawhorse") },
    Block { id: 872, name: "bell", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("bell"), cat: "rock", drop: Some("bell") },
    Block { id: 873, name: "chain", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chain"), cat: "rock", drop: Some("chain") },
    Block { id: 874, name: "cross", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cross"), cat: "rock", drop: Some("cross") },
    Block { id: 875, name: "flag", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("flag"), cat: "wood", drop: Some("flag") },
    Block { id: 876, name: "tanning_vat", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("tanning_vat"), cat: "rock", drop: Some("tanning_vat") },
    Block { id: 877, name: "lime_vat", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lime_vat"), cat: "rock", drop: Some("lime_vat") },
    Block { id: 878, name: "chopping_block", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("chopping_block"), cat: "wood", drop: Some("chopping_block") },
    Block { id: 879, name: "cart", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cart"), cat: "wood", drop: Some("cart") },
    Block { id: 880, name: "cart@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cart"), cat: "wood", drop: Some("cart") },
    Block { id: 881, name: "cart@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cart"), cat: "wood", drop: Some("cart") },
    Block { id: 882, name: "cart@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cart"), cat: "wood", drop: Some("cart") },
    Block { id: 883, name: "trough", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trough"), cat: "wood", drop: Some("trough") },
    Block { id: 884, name: "trough@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trough"), cat: "wood", drop: Some("trough") },
    Block { id: 885, name: "trough@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trough"), cat: "wood", drop: Some("trough") },
    Block { id: 886, name: "trough@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("trough"), cat: "wood", drop: Some("trough") },
    Block { id: 887, name: "tool_rack", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("tool_rack"), cat: "wood", drop: Some("tool_rack") },
    Block { id: 888, name: "tool_rack@1", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("tool_rack"), cat: "wood", drop: Some("tool_rack") },
    Block { id: 889, name: "tool_rack@2", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("tool_rack"), cat: "wood", drop: Some("tool_rack") },
    Block { id: 890, name: "tool_rack@3", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("tool_rack"), cat: "wood", drop: Some("tool_rack") },
    Block { id: 891, name: "coal_heap", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("coal_heap"), cat: "rock", drop: Some("charcoal") },
    Block { id: 892, name: "fishing_net", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fishing_net"), cat: "wood", drop: Some("fishing_net") },
    Block { id: 893, name: "fishing_net@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fishing_net"), cat: "wood", drop: Some("fishing_net") },
    Block { id: 894, name: "fishing_net@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fishing_net"), cat: "wood", drop: Some("fishing_net") },
    Block { id: 895, name: "fishing_net@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fishing_net"), cat: "wood", drop: Some("fishing_net") },
    Block { id: 896, name: "fish_rack", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fish_rack"), cat: "wood", drop: Some("fish_rack") },
    Block { id: 897, name: "fish_rack@1", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fish_rack"), cat: "wood", drop: Some("fish_rack") },
    Block { id: 898, name: "fish_rack@2", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fish_rack"), cat: "wood", drop: Some("fish_rack") },
    Block { id: 899, name: "fish_rack@3", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fish_rack"), cat: "wood", drop: Some("fish_rack") },
    Block { id: 900, name: "anchor", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("anchor"), cat: "wood", drop: Some("anchor") },
    Block { id: 901, name: "anchor@1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("anchor"), cat: "wood", drop: Some("anchor") },
    Block { id: 902, name: "anchor@2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("anchor"), cat: "wood", drop: Some("anchor") },
    Block { id: 903, name: "anchor@3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("anchor"), cat: "wood", drop: Some("anchor") },
    Block { id: 904, name: "oars", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("oars"), cat: "wood", drop: Some("oars") },
    Block { id: 905, name: "oars@1", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("oars"), cat: "wood", drop: Some("oars") },
    Block { id: 906, name: "oars@2", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("oars"), cat: "wood", drop: Some("oars") },
    Block { id: 907, name: "oars@3", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("oars"), cat: "wood", drop: Some("oars") },
    Block { id: 908, name: "rope_rail", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("rope_rail"), cat: "wood", drop: Some("rope_rail") },
    Block { id: 909, name: "rope_rail@1", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("rope_rail"), cat: "wood", drop: Some("rope_rail") },
    Block { id: 910, name: "rope_rail@2", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("rope_rail"), cat: "wood", drop: Some("rope_rail") },
    Block { id: 911, name: "rope_rail@3", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("rope_rail"), cat: "wood", drop: Some("rope_rail") },
    Block { id: 912, name: "sail", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sail"), cat: "wood", drop: Some("sail") },
    Block { id: 913, name: "sail@1", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sail"), cat: "wood", drop: Some("sail") },
    Block { id: 914, name: "sail@2", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sail"), cat: "wood", drop: Some("sail") },
    Block { id: 915, name: "sail@3", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sail"), cat: "wood", drop: Some("sail") },
    Block { id: 916, name: "boat_bench", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("boat_bench"), cat: "wood", drop: Some("boat_bench") },
    Block { id: 917, name: "boat_bench@1", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("boat_bench"), cat: "wood", drop: Some("boat_bench") },
    Block { id: 918, name: "boat_bench@2", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("boat_bench"), cat: "wood", drop: Some("boat_bench") },
    Block { id: 919, name: "boat_bench@3", shape: Shape::Deco, hardness: 0.4, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("boat_bench"), cat: "wood", drop: Some("boat_bench") },
    Block { id: 920, name: "creel", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("creel"), cat: "wood", drop: Some("creel") },
    Block { id: 921, name: "mooring_post", shape: Shape::Deco, hardness: 1.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("mooring_post"), cat: "wood", drop: Some("mooring_post") },
    Block { id: 922, name: "buoy", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("buoy"), cat: "wood", drop: Some("buoy") },
    Block { id: 923, name: "net_pile", shape: Shape::Deco, hardness: 0.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("net_pile"), cat: "wood", drop: Some("net_pile") },
    Block { id: 924, name: "pennant", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("pennant"), cat: "wood", drop: Some("pennant") },
    Block { id: 925, name: "fish_crate", shape: Shape::Deco, hardness: 0.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("fish_crate"), cat: "wood", drop: Some("fish_crate") },
    Block { id: 926, name: "flowstone", shape: Shape::Cube, hardness: 1.3, faces: [376,376,376], dir_faces: [376,376,376,376], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("flowstone") },
    Block { id: 927, name: "dripstone_down_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("flowstone") },
    Block { id: 928, name: "dripstone_down_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("flowstone") },
    Block { id: 929, name: "dripstone_down_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("flowstone") },
    Block { id: 930, name: "dripstone_up_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("flowstone") },
    Block { id: 931, name: "dripstone_up_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("flowstone") },
    Block { id: 932, name: "dripstone_up_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("flowstone") },
    Block { id: 933, name: "basalt", shape: Shape::Cube, hardness: 1.8, faces: [380,380,380], dir_faces: [380,380,380,380], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("basalt") },
    Block { id: 934, name: "obsidian", shape: Shape::Cube, hardness: 6.0, faces: [384,384,384], dir_faces: [384,384,384,384], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("obsidian") },
    Block { id: 935, name: "lava", shape: Shape::Cube, hardness: 100.0, faces: [388,388,388], dir_faces: [388,388,388,388], grain: [0,0,0,0], solid: false, opaque: true, light: 15, tint: [1.0,1.0,1.0], model: None, cat: "liquid", drop: None },
    Block { id: 936, name: "magma", shape: Shape::Cube, hardness: 1.6, faces: [392,392,392], dir_faces: [392,392,392,392], grain: [0,0,0,0], solid: true, opaque: true, light: 9, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("magma") },
    Block { id: 937, name: "crystal_violet", shape: Shape::Cube, hardness: 1.2, faces: [396,396,396], dir_faces: [396,396,396,396], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("crystal_violet") },
    Block { id: 938, name: "crystal_blue", shape: Shape::Cube, hardness: 1.2, faces: [400,400,400], dir_faces: [400,400,400,400], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("crystal_blue") },
    Block { id: 939, name: "crystal_pink", shape: Shape::Cube, hardness: 1.2, faces: [404,404,404], dir_faces: [404,404,404,404], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("crystal_pink") },
    Block { id: 940, name: "crystal_white", shape: Shape::Cube, hardness: 1.2, faces: [408,408,408], dir_faces: [408,408,408,408], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("crystal_white") },
    Block { id: 941, name: "abyss_rock", shape: Shape::Cube, hardness: 2.2, faces: [412,412,412], dir_faces: [412,412,412,412], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "rock", drop: Some("abyss_rock") },
    Block { id: 942, name: "glow_moss", shape: Shape::Cube, hardness: 0.6, faces: [416,416,416], dir_faces: [416,416,416,416], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("glow_moss") },
    Block { id: 943, name: "mycelium", shape: Shape::Cube, hardness: 0.6, faces: [420,8,420], dir_faces: [8,420,420,8], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("mycelium") },
    Block { id: 944, name: "mushroom_stem", shape: Shape::Cube, hardness: 0.6, faces: [424,424,424], dir_faces: [424,424,424,424], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("mushroom_stem") },
    Block { id: 945, name: "mushroom_cap_red", shape: Shape::Cube, hardness: 0.5, faces: [428,428,428], dir_faces: [428,428,428,428], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("mushroom_cap_red") },
    Block { id: 946, name: "mushroom_cap_violet", shape: Shape::Cube, hardness: 0.5, faces: [432,432,432], dir_faces: [432,432,432,432], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("mushroom_cap_violet") },
    Block { id: 947, name: "mushroom_cap_blue", shape: Shape::Cube, hardness: 0.5, faces: [436,436,436], dir_faces: [436,436,436,436], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("mushroom_cap_blue") },
    Block { id: 948, name: "mushroom_gills", shape: Shape::Cube, hardness: 0.4, faces: [440,440,440], dir_faces: [440,440,440,440], grain: [0,0,0,0], solid: true, opaque: true, light: 10, tint: [1.0,1.0,1.0], model: None, cat: "wood", drop: Some("mushroom_gills") },
    Block { id: 949, name: "ice_blue", shape: Shape::Cube, hardness: 0.8, faces: [444,444,444], dir_faces: [444,444,444,444], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [1.0,1.0,1.0], model: None, cat: "terrain", drop: Some("ice_blue") },
    Block { id: 950, name: "oak_post", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post") },
    Block { id: 951, name: "oak_post_x", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post") },
    Block { id: 952, name: "oak_post_z", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post") },
    Block { id: 953, name: "oak_post_diag", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_diag") },
    Block { id: 954, name: "oak_post_diag_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_diag") },
    Block { id: 955, name: "oak_post_diag_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_diag") },
    Block { id: 956, name: "oak_post_diag_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_diag") },
    Block { id: 957, name: "oak_post_diag_flat", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_diag") },
    Block { id: 958, name: "oak_post_diag_flat_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_diag") },
    Block { id: 959, name: "oak_post_angle", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_angle") },
    Block { id: 960, name: "oak_post_angle_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_angle") },
    Block { id: 961, name: "oak_post_angle_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_angle") },
    Block { id: 962, name: "oak_post_angle_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_angle") },
    Block { id: 963, name: "oak_post_angle_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_angle") },
    Block { id: 964, name: "oak_post_angle_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_angle") },
    Block { id: 965, name: "oak_post_angle_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_angle") },
    Block { id: 966, name: "oak_post_angle_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("oak_post_angle") },
    Block { id: 967, name: "birch_post", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post") },
    Block { id: 968, name: "birch_post_x", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post") },
    Block { id: 969, name: "birch_post_z", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post") },
    Block { id: 970, name: "birch_post_diag", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_diag") },
    Block { id: 971, name: "birch_post_diag_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_diag") },
    Block { id: 972, name: "birch_post_diag_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_diag") },
    Block { id: 973, name: "birch_post_diag_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_diag") },
    Block { id: 974, name: "birch_post_diag_flat", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_diag") },
    Block { id: 975, name: "birch_post_diag_flat_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_diag") },
    Block { id: 976, name: "birch_post_angle", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_angle") },
    Block { id: 977, name: "birch_post_angle_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_angle") },
    Block { id: 978, name: "birch_post_angle_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_angle") },
    Block { id: 979, name: "birch_post_angle_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_angle") },
    Block { id: 980, name: "birch_post_angle_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_angle") },
    Block { id: 981, name: "birch_post_angle_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_angle") },
    Block { id: 982, name: "birch_post_angle_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_angle") },
    Block { id: 983, name: "birch_post_angle_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("birch_post_angle") },
    Block { id: 984, name: "beech_post", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post") },
    Block { id: 985, name: "beech_post_x", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post") },
    Block { id: 986, name: "beech_post_z", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post") },
    Block { id: 987, name: "beech_post_diag", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_diag") },
    Block { id: 988, name: "beech_post_diag_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_diag") },
    Block { id: 989, name: "beech_post_diag_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_diag") },
    Block { id: 990, name: "beech_post_diag_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_diag") },
    Block { id: 991, name: "beech_post_diag_flat", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_diag") },
    Block { id: 992, name: "beech_post_diag_flat_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_diag") },
    Block { id: 993, name: "beech_post_angle", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_angle") },
    Block { id: 994, name: "beech_post_angle_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_angle") },
    Block { id: 995, name: "beech_post_angle_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_angle") },
    Block { id: 996, name: "beech_post_angle_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_angle") },
    Block { id: 997, name: "beech_post_angle_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_angle") },
    Block { id: 998, name: "beech_post_angle_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_angle") },
    Block { id: 999, name: "beech_post_angle_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_angle") },
    Block { id: 1000, name: "beech_post_angle_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("beech_post_angle") },
    Block { id: 1001, name: "spruce_post", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post") },
    Block { id: 1002, name: "spruce_post_x", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post") },
    Block { id: 1003, name: "spruce_post_z", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post") },
    Block { id: 1004, name: "spruce_post_diag", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_diag") },
    Block { id: 1005, name: "spruce_post_diag_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_diag") },
    Block { id: 1006, name: "spruce_post_diag_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_diag") },
    Block { id: 1007, name: "spruce_post_diag_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_diag") },
    Block { id: 1008, name: "spruce_post_diag_flat", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_diag") },
    Block { id: 1009, name: "spruce_post_diag_flat_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_diag") },
    Block { id: 1010, name: "spruce_post_angle", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_angle") },
    Block { id: 1011, name: "spruce_post_angle_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_angle") },
    Block { id: 1012, name: "spruce_post_angle_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_angle") },
    Block { id: 1013, name: "spruce_post_angle_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_angle") },
    Block { id: 1014, name: "spruce_post_angle_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_angle") },
    Block { id: 1015, name: "spruce_post_angle_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_angle") },
    Block { id: 1016, name: "spruce_post_angle_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_angle") },
    Block { id: 1017, name: "spruce_post_angle_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("spruce_post_angle") },
    Block { id: 1018, name: "acacia_post", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post") },
    Block { id: 1019, name: "acacia_post_x", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post") },
    Block { id: 1020, name: "acacia_post_z", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post") },
    Block { id: 1021, name: "acacia_post_diag", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_diag") },
    Block { id: 1022, name: "acacia_post_diag_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_diag") },
    Block { id: 1023, name: "acacia_post_diag_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_diag") },
    Block { id: 1024, name: "acacia_post_diag_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_diag") },
    Block { id: 1025, name: "acacia_post_diag_flat", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_diag") },
    Block { id: 1026, name: "acacia_post_diag_flat_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_diag") },
    Block { id: 1027, name: "acacia_post_angle", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_angle") },
    Block { id: 1028, name: "acacia_post_angle_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_angle") },
    Block { id: 1029, name: "acacia_post_angle_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_angle") },
    Block { id: 1030, name: "acacia_post_angle_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_angle") },
    Block { id: 1031, name: "acacia_post_angle_top", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_angle") },
    Block { id: 1032, name: "acacia_post_angle_top_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_angle") },
    Block { id: 1033, name: "acacia_post_angle_top_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_angle") },
    Block { id: 1034, name: "acacia_post_angle_top_3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("post"), cat: "wood", drop: Some("acacia_post_angle") },
    Block { id: 1035, name: "stone_point_down_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("stone") },
    Block { id: 1036, name: "stone_point_down_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("stone") },
    Block { id: 1037, name: "stone_point_down_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("stone") },
    Block { id: 1038, name: "stone_point_up_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("stone") },
    Block { id: 1039, name: "stone_point_up_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("stone") },
    Block { id: 1040, name: "stone_point_up_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("stone") },
    Block { id: 1041, name: "ice_point_down_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("ice") },
    Block { id: 1042, name: "ice_point_down_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("ice") },
    Block { id: 1043, name: "ice_point_down_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("ice") },
    Block { id: 1044, name: "ice_point_up_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("ice") },
    Block { id: 1045, name: "ice_point_up_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("ice") },
    Block { id: 1046, name: "ice_point_up_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("ice") },
    Block { id: 1047, name: "basalt_point_down_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("basalt") },
    Block { id: 1048, name: "basalt_point_down_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("basalt") },
    Block { id: 1049, name: "basalt_point_down_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("basalt") },
    Block { id: 1050, name: "basalt_point_up_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("basalt") },
    Block { id: 1051, name: "basalt_point_up_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("basalt") },
    Block { id: 1052, name: "basalt_point_up_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("basalt") },
    Block { id: 1053, name: "sandstone_point_down_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("sandstone") },
    Block { id: 1054, name: "sandstone_point_down_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("sandstone") },
    Block { id: 1055, name: "sandstone_point_down_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("sandstone") },
    Block { id: 1056, name: "sandstone_point_up_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("sandstone") },
    Block { id: 1057, name: "sandstone_point_up_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("sandstone") },
    Block { id: 1058, name: "sandstone_point_up_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("sandstone") },
    Block { id: 1059, name: "crystal_violet_point_down_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_violet") },
    Block { id: 1060, name: "crystal_violet_point_down_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_violet") },
    Block { id: 1061, name: "crystal_violet_point_down_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_violet") },
    Block { id: 1062, name: "crystal_violet_point_up_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_violet") },
    Block { id: 1063, name: "crystal_violet_point_up_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_violet") },
    Block { id: 1064, name: "crystal_violet_point_up_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_violet") },
    Block { id: 1065, name: "crystal_blue_point_down_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_blue") },
    Block { id: 1066, name: "crystal_blue_point_down_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_blue") },
    Block { id: 1067, name: "crystal_blue_point_down_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_blue") },
    Block { id: 1068, name: "crystal_blue_point_up_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_blue") },
    Block { id: 1069, name: "crystal_blue_point_up_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_blue") },
    Block { id: 1070, name: "crystal_blue_point_up_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_blue") },
    Block { id: 1071, name: "crystal_pink_point_down_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_pink") },
    Block { id: 1072, name: "crystal_pink_point_down_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_pink") },
    Block { id: 1073, name: "crystal_pink_point_down_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_pink") },
    Block { id: 1074, name: "crystal_pink_point_up_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_pink") },
    Block { id: 1075, name: "crystal_pink_point_up_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_pink") },
    Block { id: 1076, name: "crystal_pink_point_up_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_pink") },
    Block { id: 1077, name: "crystal_white_point_down_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_white") },
    Block { id: 1078, name: "crystal_white_point_down_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_white") },
    Block { id: 1079, name: "crystal_white_point_down_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_white") },
    Block { id: 1080, name: "crystal_white_point_up_0", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_white") },
    Block { id: 1081, name: "crystal_white_point_up_1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_white") },
    Block { id: 1082, name: "crystal_white_point_up_2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("dripstone"), cat: "rock", drop: Some("crystal_white") },
    Block { id: 1083, name: "sapling", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("sapling"), cat: "plant", drop: Some("sapling") },
    Block { id: 1084, name: "gravestone", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("gravestone"), cat: "rock", drop: Some("gravestone") },
    Block { id: 1085, name: "gravestone@1", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("gravestone"), cat: "rock", drop: Some("gravestone") },
    Block { id: 1086, name: "gravestone@2", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("gravestone"), cat: "rock", drop: Some("gravestone") },
    Block { id: 1087, name: "gravestone@3", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("gravestone"), cat: "rock", drop: Some("gravestone") },
    Block { id: 1088, name: "grave_cross", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grave_cross"), cat: "wood", drop: Some("grave_cross") },
    Block { id: 1089, name: "grave_cross@1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grave_cross"), cat: "wood", drop: Some("grave_cross") },
    Block { id: 1090, name: "grave_cross@2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grave_cross"), cat: "wood", drop: Some("grave_cross") },
    Block { id: 1091, name: "grave_cross@3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("grave_cross"), cat: "wood", drop: Some("grave_cross") },
    Block { id: 1092, name: "stone_slab", shape: Shape::Deco, hardness: 0.78, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_slab") },
    Block { id: 1093, name: "stone_slab_top", shape: Shape::Deco, hardness: 0.78, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_slab") },
    Block { id: 1094, name: "stone_stairs", shape: Shape::Deco, hardness: 1.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_stairs") },
    Block { id: 1095, name: "stone_stairs_1", shape: Shape::Deco, hardness: 1.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_stairs") },
    Block { id: 1096, name: "stone_stairs_2", shape: Shape::Deco, hardness: 1.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_stairs") },
    Block { id: 1097, name: "stone_stairs_3", shape: Shape::Deco, hardness: 1.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_stairs") },
    Block { id: 1098, name: "stone_stairs_top", shape: Shape::Deco, hardness: 1.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_stairs") },
    Block { id: 1099, name: "stone_stairs_top_1", shape: Shape::Deco, hardness: 1.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_stairs") },
    Block { id: 1100, name: "stone_stairs_top_2", shape: Shape::Deco, hardness: 1.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_stairs") },
    Block { id: 1101, name: "stone_stairs_top_3", shape: Shape::Deco, hardness: 1.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("stone_stairs") },
    Block { id: 1102, name: "stone_vslab", shape: Shape::Deco, hardness: 0.78, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_vslab") },
    Block { id: 1103, name: "stone_vslab_1", shape: Shape::Deco, hardness: 0.78, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_vslab") },
    Block { id: 1104, name: "stone_vslab_2", shape: Shape::Deco, hardness: 0.78, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_vslab") },
    Block { id: 1105, name: "stone_vslab_3", shape: Shape::Deco, hardness: 0.78, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("stone_vslab") },
    Block { id: 1106, name: "limestone_slab", shape: Shape::Deco, hardness: 0.72, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_slab") },
    Block { id: 1107, name: "limestone_slab_top", shape: Shape::Deco, hardness: 0.72, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_slab") },
    Block { id: 1108, name: "limestone_stairs", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_stairs") },
    Block { id: 1109, name: "limestone_stairs_1", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_stairs") },
    Block { id: 1110, name: "limestone_stairs_2", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_stairs") },
    Block { id: 1111, name: "limestone_stairs_3", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_stairs") },
    Block { id: 1112, name: "limestone_stairs_top", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_stairs") },
    Block { id: 1113, name: "limestone_stairs_top_1", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_stairs") },
    Block { id: 1114, name: "limestone_stairs_top_2", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_stairs") },
    Block { id: 1115, name: "limestone_stairs_top_3", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("limestone_stairs") },
    Block { id: 1116, name: "limestone_vslab", shape: Shape::Deco, hardness: 0.72, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_vslab") },
    Block { id: 1117, name: "limestone_vslab_1", shape: Shape::Deco, hardness: 0.72, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_vslab") },
    Block { id: 1118, name: "limestone_vslab_2", shape: Shape::Deco, hardness: 0.72, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_vslab") },
    Block { id: 1119, name: "limestone_vslab_3", shape: Shape::Deco, hardness: 0.72, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("limestone_vslab") },
    Block { id: 1120, name: "granite_slab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("granite_slab") },
    Block { id: 1121, name: "granite_slab_top", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("granite_slab") },
    Block { id: 1122, name: "granite_stairs", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("granite_stairs") },
    Block { id: 1123, name: "granite_stairs_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("granite_stairs") },
    Block { id: 1124, name: "granite_stairs_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("granite_stairs") },
    Block { id: 1125, name: "granite_stairs_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("granite_stairs") },
    Block { id: 1126, name: "granite_stairs_top", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("granite_stairs") },
    Block { id: 1127, name: "granite_stairs_top_1", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("granite_stairs") },
    Block { id: 1128, name: "granite_stairs_top_2", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("granite_stairs") },
    Block { id: 1129, name: "granite_stairs_top_3", shape: Shape::Deco, hardness: 1.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("granite_stairs") },
    Block { id: 1130, name: "granite_vslab", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("granite_vslab") },
    Block { id: 1131, name: "granite_vslab_1", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("granite_vslab") },
    Block { id: 1132, name: "granite_vslab_2", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("granite_vslab") },
    Block { id: 1133, name: "granite_vslab_3", shape: Shape::Deco, hardness: 0.96, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("granite_vslab") },
    Block { id: 1134, name: "sandstone_slab", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("sandstone_slab") },
    Block { id: 1135, name: "sandstone_slab_top", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("sandstone_slab") },
    Block { id: 1136, name: "sandstone_stairs", shape: Shape::Deco, hardness: 1.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("sandstone_stairs") },
    Block { id: 1137, name: "sandstone_stairs_1", shape: Shape::Deco, hardness: 1.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("sandstone_stairs") },
    Block { id: 1138, name: "sandstone_stairs_2", shape: Shape::Deco, hardness: 1.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("sandstone_stairs") },
    Block { id: 1139, name: "sandstone_stairs_3", shape: Shape::Deco, hardness: 1.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("sandstone_stairs") },
    Block { id: 1140, name: "sandstone_stairs_top", shape: Shape::Deco, hardness: 1.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("sandstone_stairs") },
    Block { id: 1141, name: "sandstone_stairs_top_1", shape: Shape::Deco, hardness: 1.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("sandstone_stairs") },
    Block { id: 1142, name: "sandstone_stairs_top_2", shape: Shape::Deco, hardness: 1.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("sandstone_stairs") },
    Block { id: 1143, name: "sandstone_stairs_top_3", shape: Shape::Deco, hardness: 1.0, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("stairs"), cat: "rock", drop: Some("sandstone_stairs") },
    Block { id: 1144, name: "sandstone_vslab", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("sandstone_vslab") },
    Block { id: 1145, name: "sandstone_vslab_1", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("sandstone_vslab") },
    Block { id: 1146, name: "sandstone_vslab_2", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("sandstone_vslab") },
    Block { id: 1147, name: "sandstone_vslab_3", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("slab"), cat: "rock", drop: Some("sandstone_vslab") },
    Block { id: 1148, name: "cobweb", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("cobweb"), cat: "foliage", drop: Some("silk") },
    Block { id: 1149, name: "glow_jar", shape: Shape::Deco, hardness: 0.3, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 9, tint: [1.0,1.0,1.0], model: Some("glow_jar"), cat: "light", drop: Some("glow_jar") },
    Block { id: 1150, name: "glow_vine", shape: Shape::Deco, hardness: 0.05, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("glow_vine"), cat: "foliage", drop: Some("glow_vine") },
    Block { id: 1151, name: "roots", shape: Shape::Deco, hardness: 0.1, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("roots"), cat: "foliage", drop: Some("stick") },
    Block { id: 1152, name: "rail_x", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("rail"), cat: "wood", drop: Some("rail") },
    Block { id: 1153, name: "rail_z", shape: Shape::Deco, hardness: 0.6, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("rail"), cat: "wood", drop: Some("rail") },
    Block { id: 1154, name: "iron_bars@1", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("iron_bars"), cat: "rock", drop: Some("iron_bars") },
    Block { id: 1155, name: "iron_bars@2", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("iron_bars"), cat: "rock", drop: Some("iron_bars") },
    Block { id: 1156, name: "iron_bars@3", shape: Shape::Deco, hardness: 1.5, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: true, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("iron_bars"), cat: "rock", drop: Some("iron_bars") },
    Block { id: 1157, name: "lever", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lever"), cat: "wood", drop: Some("lever") },
    Block { id: 1158, name: "lever_on", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lever_on"), cat: "wood", drop: Some("lever") },
    Block { id: 1159, name: "lever@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lever"), cat: "wood", drop: Some("lever") },
    Block { id: 1160, name: "lever_on@1", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lever_on"), cat: "wood", drop: Some("lever") },
    Block { id: 1161, name: "lever@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lever"), cat: "wood", drop: Some("lever") },
    Block { id: 1162, name: "lever_on@2", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lever_on"), cat: "wood", drop: Some("lever") },
    Block { id: 1163, name: "lever@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lever"), cat: "wood", drop: Some("lever") },
    Block { id: 1164, name: "lever_on@3", shape: Shape::Deco, hardness: 0.8, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("lever_on"), cat: "wood", drop: Some("lever") },
    Block { id: 1165, name: "pressure_plate", shape: Shape::Cube, hardness: 1.6, faces: [328,328,328], dir_faces: [328,328,328,328], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.95,0.82,0.58], model: None, cat: "rock", drop: Some("pressure_plate") },
    Block { id: 1166, name: "pressure_plate_on", shape: Shape::Cube, hardness: 1.6, faces: [328,328,328], dir_faces: [328,328,328,328], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.62,0.52,0.36], model: None, cat: "rock", drop: Some("pressure_plate") },
    Block { id: 1167, name: "push_block", shape: Shape::Cube, hardness: 2.5, faces: [260,260,260], dir_faces: [260,260,260,260], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.78,0.86,1.05], model: None, cat: "rock", drop: Some("push_block") },
    Block { id: 1168, name: "crumbling_floor", shape: Shape::Cube, hardness: 0.4, faces: [264,264,264], dir_faces: [264,264,264,264], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.82,0.72,0.6], model: None, cat: "rock", drop: Some("crumbling_floor") },
    Block { id: 1169, name: "dart_trap", shape: Shape::Cube, hardness: 2.0, faces: [328,328,328], dir_faces: [328,328,328,328], grain: [0,0,0,0], solid: true, opaque: true, light: 0, tint: [0.85,0.55,0.48], model: None, cat: "rock", drop: Some("dart_trap") },
    Block { id: 1170, name: "spikes", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("spikes"), cat: "rock", drop: Some("spikes") },
    Block { id: 1171, name: "spikes_down", shape: Shape::Deco, hardness: 1.2, faces: [0, 0, 0], dir_faces: [0, 0, 0, 0], grain: [0,0,0,0], solid: false, opaque: false, light: 0, tint: [1.0,1.0,1.0], model: Some("spikes_down"), cat: "rock", drop: Some("spikes") },
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
    pub const TRAPDOOR_1: crate::blocks::Id = 193;
    pub const TRAPDOOR_2: crate::blocks::Id = 194;
    pub const TRAPDOOR_3: crate::blocks::Id = 195;
    pub const TRAPDOOR_OPEN: crate::blocks::Id = 196;
    pub const TRAPDOOR_OPEN_1: crate::blocks::Id = 197;
    pub const TRAPDOOR_OPEN_2: crate::blocks::Id = 198;
    pub const TRAPDOOR_OPEN_3: crate::blocks::Id = 199;
    pub const LADDER: crate::blocks::Id = 200;
    pub const LADDER_1: crate::blocks::Id = 201;
    pub const LADDER_2: crate::blocks::Id = 202;
    pub const LADDER_3: crate::blocks::Id = 203;
    pub const WINDOW: crate::blocks::Id = 204;
    pub const WINDOW_X: crate::blocks::Id = 205;
    pub const WOOD_PILLAR: crate::blocks::Id = 206;
    pub const STONE_PILLAR: crate::blocks::Id = 207;
    pub const STONE_WALL: crate::blocks::Id = 208;
    pub const STONE_WALL_X: crate::blocks::Id = 209;
    pub const CARROT_0: crate::blocks::Id = 210;
    pub const CARROT_1: crate::blocks::Id = 211;
    pub const CARROT_2: crate::blocks::Id = 212;
    pub const CARROT_3: crate::blocks::Id = 213;
    pub const POTATO_0: crate::blocks::Id = 214;
    pub const POTATO_1: crate::blocks::Id = 215;
    pub const POTATO_2: crate::blocks::Id = 216;
    pub const POTATO_3: crate::blocks::Id = 217;
    pub const PLANKS_STAIRS_TOP: crate::blocks::Id = 218;
    pub const PLANKS_STAIRS_TOP_1: crate::blocks::Id = 219;
    pub const PLANKS_STAIRS_TOP_2: crate::blocks::Id = 220;
    pub const PLANKS_STAIRS_TOP_3: crate::blocks::Id = 221;
    pub const STONE_BRICKS_STAIRS_TOP: crate::blocks::Id = 222;
    pub const STONE_BRICKS_STAIRS_TOP_1: crate::blocks::Id = 223;
    pub const STONE_BRICKS_STAIRS_TOP_2: crate::blocks::Id = 224;
    pub const STONE_BRICKS_STAIRS_TOP_3: crate::blocks::Id = 225;
    pub const COBBLESTONE_STAIRS_TOP: crate::blocks::Id = 226;
    pub const COBBLESTONE_STAIRS_TOP_1: crate::blocks::Id = 227;
    pub const COBBLESTONE_STAIRS_TOP_2: crate::blocks::Id = 228;
    pub const COBBLESTONE_STAIRS_TOP_3: crate::blocks::Id = 229;
    pub const THATCH_STAIRS_TOP: crate::blocks::Id = 230;
    pub const THATCH_STAIRS_TOP_1: crate::blocks::Id = 231;
    pub const THATCH_STAIRS_TOP_2: crate::blocks::Id = 232;
    pub const THATCH_STAIRS_TOP_3: crate::blocks::Id = 233;
    pub const PLANKS_VSLAB: crate::blocks::Id = 234;
    pub const PLANKS_VSLAB_1: crate::blocks::Id = 235;
    pub const PLANKS_VSLAB_2: crate::blocks::Id = 236;
    pub const PLANKS_VSLAB_3: crate::blocks::Id = 237;
    pub const STONE_BRICKS_VSLAB: crate::blocks::Id = 238;
    pub const STONE_BRICKS_VSLAB_1: crate::blocks::Id = 239;
    pub const STONE_BRICKS_VSLAB_2: crate::blocks::Id = 240;
    pub const STONE_BRICKS_VSLAB_3: crate::blocks::Id = 241;
    pub const COBBLESTONE_VSLAB: crate::blocks::Id = 242;
    pub const COBBLESTONE_VSLAB_1: crate::blocks::Id = 243;
    pub const COBBLESTONE_VSLAB_2: crate::blocks::Id = 244;
    pub const COBBLESTONE_VSLAB_3: crate::blocks::Id = 245;
    pub const SHUTTER: crate::blocks::Id = 246;
    pub const SHUTTER_1: crate::blocks::Id = 247;
    pub const SHUTTER_2: crate::blocks::Id = 248;
    pub const SHUTTER_3: crate::blocks::Id = 249;
    pub const SHUTTER_OPEN: crate::blocks::Id = 250;
    pub const SHUTTER_OPEN_1: crate::blocks::Id = 251;
    pub const SHUTTER_OPEN_2: crate::blocks::Id = 252;
    pub const SHUTTER_OPEN_3: crate::blocks::Id = 253;
    pub const PLASTER: crate::blocks::Id = 254;
    pub const TIMBER: crate::blocks::Id = 255;
    pub const ROOF_TILES: crate::blocks::Id = 256;
    pub const SLATE: crate::blocks::Id = 257;
    pub const BARREL: crate::blocks::Id = 258;
    pub const MARBLE: crate::blocks::Id = 259;
    pub const ASHLAR: crate::blocks::Id = 260;
    pub const FIELDSTONE: crate::blocks::Id = 261;
    pub const SETTS: crate::blocks::Id = 262;
    pub const POLISHED_STONE: crate::blocks::Id = 263;
    pub const SHINGLES: crate::blocks::Id = 264;
    pub const COPPER_ROOF: crate::blocks::Id = 265;
    pub const CHECKER_FLOOR: crate::blocks::Id = 266;
    pub const TERRACOTTA_FLOOR: crate::blocks::Id = 267;
    pub const PARQUET: crate::blocks::Id = 268;
    pub const CHISELED_STONE: crate::blocks::Id = 269;
    pub const STAINED_GLASS: crate::blocks::Id = 270;
    pub const IRON_BLOCK: crate::blocks::Id = 271;
    pub const MARBLE_BLACK: crate::blocks::Id = 272;
    pub const BRICKS_DARK: crate::blocks::Id = 273;
    pub const STONE_BRICKS_DARK: crate::blocks::Id = 274;
    pub const TIMBER_FRAME: crate::blocks::Id = 275;
    pub const TIMBER_CROSS: crate::blocks::Id = 276;
    pub const TIMBER_BRACE: crate::blocks::Id = 277;
    pub const WATTLE: crate::blocks::Id = 278;
    pub const BOOKSHELF: crate::blocks::Id = 279;
    pub const HAY_BALE: crate::blocks::Id = 280;
    pub const CRATE: crate::blocks::Id = 281;
    pub const LIMESTONE_MASONRY: crate::blocks::Id = 282;
    pub const RUBBLE_WALL: crate::blocks::Id = 283;
    pub const BARN_BOARDS: crate::blocks::Id = 284;
    pub const MOSSY_ROOF_TILES: crate::blocks::Id = 285;
    pub const DIRT_PATH: crate::blocks::Id = 286;
    pub const OAK_BEAM: crate::blocks::Id = 287;
    pub const OAK_BEAM_X: crate::blocks::Id = 288;
    pub const OAK_BEAM_Z: crate::blocks::Id = 289;
    pub const PLANKS_DARK: crate::blocks::Id = 290;
    pub const PLANKS_RED: crate::blocks::Id = 291;
    pub const PLANKS_WHITE: crate::blocks::Id = 292;
    pub const PLANKS_BLUE: crate::blocks::Id = 293;
    pub const PLANKS_GREEN: crate::blocks::Id = 294;
    pub const PLASTER_OCHRE: crate::blocks::Id = 295;
    pub const PLASTER_ROSE: crate::blocks::Id = 296;
    pub const PLASTER_BLUE: crate::blocks::Id = 297;
    pub const WOOL_WHITE: crate::blocks::Id = 298;
    pub const WOOL_RED: crate::blocks::Id = 299;
    pub const WOOL_BLUE: crate::blocks::Id = 300;
    pub const WOOL_GREEN: crate::blocks::Id = 301;
    pub const WOOL_YELLOW: crate::blocks::Id = 302;
    pub const WOOL_BLACK: crate::blocks::Id = 303;
    pub const WOOL_BROWN: crate::blocks::Id = 304;
    pub const WOOL_PURPLE: crate::blocks::Id = 305;
    pub const WOOL_ORANGE: crate::blocks::Id = 306;
    pub const MARBLE_STAIRS: crate::blocks::Id = 307;
    pub const MARBLE_STAIRS_1: crate::blocks::Id = 308;
    pub const MARBLE_STAIRS_2: crate::blocks::Id = 309;
    pub const MARBLE_STAIRS_3: crate::blocks::Id = 310;
    pub const MARBLE_STAIRS_TOP: crate::blocks::Id = 311;
    pub const MARBLE_STAIRS_TOP_1: crate::blocks::Id = 312;
    pub const MARBLE_STAIRS_TOP_2: crate::blocks::Id = 313;
    pub const MARBLE_STAIRS_TOP_3: crate::blocks::Id = 314;
    pub const MARBLE_SLAB: crate::blocks::Id = 315;
    pub const MARBLE_SLAB_TOP: crate::blocks::Id = 316;
    pub const MARBLE_VSLAB: crate::blocks::Id = 317;
    pub const MARBLE_VSLAB_1: crate::blocks::Id = 318;
    pub const MARBLE_VSLAB_2: crate::blocks::Id = 319;
    pub const MARBLE_VSLAB_3: crate::blocks::Id = 320;
    pub const ASHLAR_STAIRS: crate::blocks::Id = 321;
    pub const ASHLAR_STAIRS_1: crate::blocks::Id = 322;
    pub const ASHLAR_STAIRS_2: crate::blocks::Id = 323;
    pub const ASHLAR_STAIRS_3: crate::blocks::Id = 324;
    pub const ASHLAR_STAIRS_TOP: crate::blocks::Id = 325;
    pub const ASHLAR_STAIRS_TOP_1: crate::blocks::Id = 326;
    pub const ASHLAR_STAIRS_TOP_2: crate::blocks::Id = 327;
    pub const ASHLAR_STAIRS_TOP_3: crate::blocks::Id = 328;
    pub const ASHLAR_SLAB: crate::blocks::Id = 329;
    pub const ASHLAR_SLAB_TOP: crate::blocks::Id = 330;
    pub const ASHLAR_VSLAB: crate::blocks::Id = 331;
    pub const ASHLAR_VSLAB_1: crate::blocks::Id = 332;
    pub const ASHLAR_VSLAB_2: crate::blocks::Id = 333;
    pub const ASHLAR_VSLAB_3: crate::blocks::Id = 334;
    pub const FIELDSTONE_STAIRS: crate::blocks::Id = 335;
    pub const FIELDSTONE_STAIRS_1: crate::blocks::Id = 336;
    pub const FIELDSTONE_STAIRS_2: crate::blocks::Id = 337;
    pub const FIELDSTONE_STAIRS_3: crate::blocks::Id = 338;
    pub const FIELDSTONE_STAIRS_TOP: crate::blocks::Id = 339;
    pub const FIELDSTONE_STAIRS_TOP_1: crate::blocks::Id = 340;
    pub const FIELDSTONE_STAIRS_TOP_2: crate::blocks::Id = 341;
    pub const FIELDSTONE_STAIRS_TOP_3: crate::blocks::Id = 342;
    pub const FIELDSTONE_SLAB: crate::blocks::Id = 343;
    pub const FIELDSTONE_SLAB_TOP: crate::blocks::Id = 344;
    pub const FIELDSTONE_VSLAB: crate::blocks::Id = 345;
    pub const FIELDSTONE_VSLAB_1: crate::blocks::Id = 346;
    pub const FIELDSTONE_VSLAB_2: crate::blocks::Id = 347;
    pub const FIELDSTONE_VSLAB_3: crate::blocks::Id = 348;
    pub const SETTS_STAIRS: crate::blocks::Id = 349;
    pub const SETTS_STAIRS_1: crate::blocks::Id = 350;
    pub const SETTS_STAIRS_2: crate::blocks::Id = 351;
    pub const SETTS_STAIRS_3: crate::blocks::Id = 352;
    pub const SETTS_STAIRS_TOP: crate::blocks::Id = 353;
    pub const SETTS_STAIRS_TOP_1: crate::blocks::Id = 354;
    pub const SETTS_STAIRS_TOP_2: crate::blocks::Id = 355;
    pub const SETTS_STAIRS_TOP_3: crate::blocks::Id = 356;
    pub const SETTS_SLAB: crate::blocks::Id = 357;
    pub const SETTS_SLAB_TOP: crate::blocks::Id = 358;
    pub const SETTS_VSLAB: crate::blocks::Id = 359;
    pub const SETTS_VSLAB_1: crate::blocks::Id = 360;
    pub const SETTS_VSLAB_2: crate::blocks::Id = 361;
    pub const SETTS_VSLAB_3: crate::blocks::Id = 362;
    pub const POLISHED_STONE_STAIRS: crate::blocks::Id = 363;
    pub const POLISHED_STONE_STAIRS_1: crate::blocks::Id = 364;
    pub const POLISHED_STONE_STAIRS_2: crate::blocks::Id = 365;
    pub const POLISHED_STONE_STAIRS_3: crate::blocks::Id = 366;
    pub const POLISHED_STONE_STAIRS_TOP: crate::blocks::Id = 367;
    pub const POLISHED_STONE_STAIRS_TOP_1: crate::blocks::Id = 368;
    pub const POLISHED_STONE_STAIRS_TOP_2: crate::blocks::Id = 369;
    pub const POLISHED_STONE_STAIRS_TOP_3: crate::blocks::Id = 370;
    pub const POLISHED_STONE_SLAB: crate::blocks::Id = 371;
    pub const POLISHED_STONE_SLAB_TOP: crate::blocks::Id = 372;
    pub const POLISHED_STONE_VSLAB: crate::blocks::Id = 373;
    pub const POLISHED_STONE_VSLAB_1: crate::blocks::Id = 374;
    pub const POLISHED_STONE_VSLAB_2: crate::blocks::Id = 375;
    pub const POLISHED_STONE_VSLAB_3: crate::blocks::Id = 376;
    pub const SHINGLES_STAIRS: crate::blocks::Id = 377;
    pub const SHINGLES_STAIRS_1: crate::blocks::Id = 378;
    pub const SHINGLES_STAIRS_2: crate::blocks::Id = 379;
    pub const SHINGLES_STAIRS_3: crate::blocks::Id = 380;
    pub const SHINGLES_STAIRS_TOP: crate::blocks::Id = 381;
    pub const SHINGLES_STAIRS_TOP_1: crate::blocks::Id = 382;
    pub const SHINGLES_STAIRS_TOP_2: crate::blocks::Id = 383;
    pub const SHINGLES_STAIRS_TOP_3: crate::blocks::Id = 384;
    pub const SHINGLES_SLAB: crate::blocks::Id = 385;
    pub const SHINGLES_SLAB_TOP: crate::blocks::Id = 386;
    pub const SHINGLES_VSLAB: crate::blocks::Id = 387;
    pub const SHINGLES_VSLAB_1: crate::blocks::Id = 388;
    pub const SHINGLES_VSLAB_2: crate::blocks::Id = 389;
    pub const SHINGLES_VSLAB_3: crate::blocks::Id = 390;
    pub const ROOF_TILES_STAIRS: crate::blocks::Id = 391;
    pub const ROOF_TILES_STAIRS_1: crate::blocks::Id = 392;
    pub const ROOF_TILES_STAIRS_2: crate::blocks::Id = 393;
    pub const ROOF_TILES_STAIRS_3: crate::blocks::Id = 394;
    pub const ROOF_TILES_STAIRS_TOP: crate::blocks::Id = 395;
    pub const ROOF_TILES_STAIRS_TOP_1: crate::blocks::Id = 396;
    pub const ROOF_TILES_STAIRS_TOP_2: crate::blocks::Id = 397;
    pub const ROOF_TILES_STAIRS_TOP_3: crate::blocks::Id = 398;
    pub const ROOF_TILES_SLAB: crate::blocks::Id = 399;
    pub const ROOF_TILES_SLAB_TOP: crate::blocks::Id = 400;
    pub const ROOF_TILES_VSLAB: crate::blocks::Id = 401;
    pub const ROOF_TILES_VSLAB_1: crate::blocks::Id = 402;
    pub const ROOF_TILES_VSLAB_2: crate::blocks::Id = 403;
    pub const ROOF_TILES_VSLAB_3: crate::blocks::Id = 404;
    pub const SLATE_STAIRS: crate::blocks::Id = 405;
    pub const SLATE_STAIRS_1: crate::blocks::Id = 406;
    pub const SLATE_STAIRS_2: crate::blocks::Id = 407;
    pub const SLATE_STAIRS_3: crate::blocks::Id = 408;
    pub const SLATE_STAIRS_TOP: crate::blocks::Id = 409;
    pub const SLATE_STAIRS_TOP_1: crate::blocks::Id = 410;
    pub const SLATE_STAIRS_TOP_2: crate::blocks::Id = 411;
    pub const SLATE_STAIRS_TOP_3: crate::blocks::Id = 412;
    pub const SLATE_SLAB: crate::blocks::Id = 413;
    pub const SLATE_SLAB_TOP: crate::blocks::Id = 414;
    pub const SLATE_VSLAB: crate::blocks::Id = 415;
    pub const SLATE_VSLAB_1: crate::blocks::Id = 416;
    pub const SLATE_VSLAB_2: crate::blocks::Id = 417;
    pub const SLATE_VSLAB_3: crate::blocks::Id = 418;
    pub const COPPER_ROOF_STAIRS: crate::blocks::Id = 419;
    pub const COPPER_ROOF_STAIRS_1: crate::blocks::Id = 420;
    pub const COPPER_ROOF_STAIRS_2: crate::blocks::Id = 421;
    pub const COPPER_ROOF_STAIRS_3: crate::blocks::Id = 422;
    pub const COPPER_ROOF_STAIRS_TOP: crate::blocks::Id = 423;
    pub const COPPER_ROOF_STAIRS_TOP_1: crate::blocks::Id = 424;
    pub const COPPER_ROOF_STAIRS_TOP_2: crate::blocks::Id = 425;
    pub const COPPER_ROOF_STAIRS_TOP_3: crate::blocks::Id = 426;
    pub const COPPER_ROOF_SLAB: crate::blocks::Id = 427;
    pub const COPPER_ROOF_SLAB_TOP: crate::blocks::Id = 428;
    pub const COPPER_ROOF_VSLAB: crate::blocks::Id = 429;
    pub const COPPER_ROOF_VSLAB_1: crate::blocks::Id = 430;
    pub const COPPER_ROOF_VSLAB_2: crate::blocks::Id = 431;
    pub const COPPER_ROOF_VSLAB_3: crate::blocks::Id = 432;
    pub const PLASTER_STAIRS: crate::blocks::Id = 433;
    pub const PLASTER_STAIRS_1: crate::blocks::Id = 434;
    pub const PLASTER_STAIRS_2: crate::blocks::Id = 435;
    pub const PLASTER_STAIRS_3: crate::blocks::Id = 436;
    pub const PLASTER_STAIRS_TOP: crate::blocks::Id = 437;
    pub const PLASTER_STAIRS_TOP_1: crate::blocks::Id = 438;
    pub const PLASTER_STAIRS_TOP_2: crate::blocks::Id = 439;
    pub const PLASTER_STAIRS_TOP_3: crate::blocks::Id = 440;
    pub const PLASTER_SLAB: crate::blocks::Id = 441;
    pub const PLASTER_SLAB_TOP: crate::blocks::Id = 442;
    pub const PLASTER_VSLAB: crate::blocks::Id = 443;
    pub const PLASTER_VSLAB_1: crate::blocks::Id = 444;
    pub const PLASTER_VSLAB_2: crate::blocks::Id = 445;
    pub const PLASTER_VSLAB_3: crate::blocks::Id = 446;
    pub const TIMBER_STAIRS: crate::blocks::Id = 447;
    pub const TIMBER_STAIRS_1: crate::blocks::Id = 448;
    pub const TIMBER_STAIRS_2: crate::blocks::Id = 449;
    pub const TIMBER_STAIRS_3: crate::blocks::Id = 450;
    pub const TIMBER_STAIRS_TOP: crate::blocks::Id = 451;
    pub const TIMBER_STAIRS_TOP_1: crate::blocks::Id = 452;
    pub const TIMBER_STAIRS_TOP_2: crate::blocks::Id = 453;
    pub const TIMBER_STAIRS_TOP_3: crate::blocks::Id = 454;
    pub const TIMBER_SLAB: crate::blocks::Id = 455;
    pub const TIMBER_SLAB_TOP: crate::blocks::Id = 456;
    pub const TIMBER_VSLAB: crate::blocks::Id = 457;
    pub const TIMBER_VSLAB_1: crate::blocks::Id = 458;
    pub const TIMBER_VSLAB_2: crate::blocks::Id = 459;
    pub const TIMBER_VSLAB_3: crate::blocks::Id = 460;
    pub const LIMESTONE_MASONRY_STAIRS: crate::blocks::Id = 461;
    pub const LIMESTONE_MASONRY_STAIRS_1: crate::blocks::Id = 462;
    pub const LIMESTONE_MASONRY_STAIRS_2: crate::blocks::Id = 463;
    pub const LIMESTONE_MASONRY_STAIRS_3: crate::blocks::Id = 464;
    pub const LIMESTONE_MASONRY_STAIRS_TOP: crate::blocks::Id = 465;
    pub const LIMESTONE_MASONRY_STAIRS_TOP_1: crate::blocks::Id = 466;
    pub const LIMESTONE_MASONRY_STAIRS_TOP_2: crate::blocks::Id = 467;
    pub const LIMESTONE_MASONRY_STAIRS_TOP_3: crate::blocks::Id = 468;
    pub const LIMESTONE_MASONRY_SLAB: crate::blocks::Id = 469;
    pub const LIMESTONE_MASONRY_SLAB_TOP: crate::blocks::Id = 470;
    pub const LIMESTONE_MASONRY_VSLAB: crate::blocks::Id = 471;
    pub const LIMESTONE_MASONRY_VSLAB_1: crate::blocks::Id = 472;
    pub const LIMESTONE_MASONRY_VSLAB_2: crate::blocks::Id = 473;
    pub const LIMESTONE_MASONRY_VSLAB_3: crate::blocks::Id = 474;
    pub const RUBBLE_WALL_STAIRS: crate::blocks::Id = 475;
    pub const RUBBLE_WALL_STAIRS_1: crate::blocks::Id = 476;
    pub const RUBBLE_WALL_STAIRS_2: crate::blocks::Id = 477;
    pub const RUBBLE_WALL_STAIRS_3: crate::blocks::Id = 478;
    pub const RUBBLE_WALL_STAIRS_TOP: crate::blocks::Id = 479;
    pub const RUBBLE_WALL_STAIRS_TOP_1: crate::blocks::Id = 480;
    pub const RUBBLE_WALL_STAIRS_TOP_2: crate::blocks::Id = 481;
    pub const RUBBLE_WALL_STAIRS_TOP_3: crate::blocks::Id = 482;
    pub const RUBBLE_WALL_SLAB: crate::blocks::Id = 483;
    pub const RUBBLE_WALL_SLAB_TOP: crate::blocks::Id = 484;
    pub const RUBBLE_WALL_VSLAB: crate::blocks::Id = 485;
    pub const RUBBLE_WALL_VSLAB_1: crate::blocks::Id = 486;
    pub const RUBBLE_WALL_VSLAB_2: crate::blocks::Id = 487;
    pub const RUBBLE_WALL_VSLAB_3: crate::blocks::Id = 488;
    pub const MOSSY_ROOF_TILES_STAIRS: crate::blocks::Id = 489;
    pub const MOSSY_ROOF_TILES_STAIRS_1: crate::blocks::Id = 490;
    pub const MOSSY_ROOF_TILES_STAIRS_2: crate::blocks::Id = 491;
    pub const MOSSY_ROOF_TILES_STAIRS_3: crate::blocks::Id = 492;
    pub const MOSSY_ROOF_TILES_STAIRS_TOP: crate::blocks::Id = 493;
    pub const MOSSY_ROOF_TILES_STAIRS_TOP_1: crate::blocks::Id = 494;
    pub const MOSSY_ROOF_TILES_STAIRS_TOP_2: crate::blocks::Id = 495;
    pub const MOSSY_ROOF_TILES_STAIRS_TOP_3: crate::blocks::Id = 496;
    pub const MOSSY_ROOF_TILES_SLAB: crate::blocks::Id = 497;
    pub const MOSSY_ROOF_TILES_SLAB_TOP: crate::blocks::Id = 498;
    pub const MOSSY_ROOF_TILES_VSLAB: crate::blocks::Id = 499;
    pub const MOSSY_ROOF_TILES_VSLAB_1: crate::blocks::Id = 500;
    pub const MOSSY_ROOF_TILES_VSLAB_2: crate::blocks::Id = 501;
    pub const MOSSY_ROOF_TILES_VSLAB_3: crate::blocks::Id = 502;
    pub const BARN_BOARDS_STAIRS: crate::blocks::Id = 503;
    pub const BARN_BOARDS_STAIRS_1: crate::blocks::Id = 504;
    pub const BARN_BOARDS_STAIRS_2: crate::blocks::Id = 505;
    pub const BARN_BOARDS_STAIRS_3: crate::blocks::Id = 506;
    pub const BARN_BOARDS_STAIRS_TOP: crate::blocks::Id = 507;
    pub const BARN_BOARDS_STAIRS_TOP_1: crate::blocks::Id = 508;
    pub const BARN_BOARDS_STAIRS_TOP_2: crate::blocks::Id = 509;
    pub const BARN_BOARDS_STAIRS_TOP_3: crate::blocks::Id = 510;
    pub const BARN_BOARDS_SLAB: crate::blocks::Id = 511;
    pub const BARN_BOARDS_SLAB_TOP: crate::blocks::Id = 512;
    pub const BARN_BOARDS_VSLAB: crate::blocks::Id = 513;
    pub const BARN_BOARDS_VSLAB_1: crate::blocks::Id = 514;
    pub const BARN_BOARDS_VSLAB_2: crate::blocks::Id = 515;
    pub const BARN_BOARDS_VSLAB_3: crate::blocks::Id = 516;
    pub const ROOF_TILES_SLOPE: crate::blocks::Id = 517;
    pub const ROOF_TILES_SLOPE_1: crate::blocks::Id = 518;
    pub const ROOF_TILES_SLOPE_2: crate::blocks::Id = 519;
    pub const ROOF_TILES_SLOPE_3: crate::blocks::Id = 520;
    pub const ROOF_TILES_SLOPE_TOP: crate::blocks::Id = 521;
    pub const ROOF_TILES_SLOPE_TOP_1: crate::blocks::Id = 522;
    pub const ROOF_TILES_SLOPE_TOP_2: crate::blocks::Id = 523;
    pub const ROOF_TILES_SLOPE_TOP_3: crate::blocks::Id = 524;
    pub const ROOF_TILES_CORNER: crate::blocks::Id = 525;
    pub const ROOF_TILES_CORNER_1: crate::blocks::Id = 526;
    pub const ROOF_TILES_CORNER_2: crate::blocks::Id = 527;
    pub const ROOF_TILES_CORNER_3: crate::blocks::Id = 528;
    pub const ROOF_TILES_CORNER_TOP: crate::blocks::Id = 529;
    pub const ROOF_TILES_CORNER_TOP_1: crate::blocks::Id = 530;
    pub const ROOF_TILES_CORNER_TOP_2: crate::blocks::Id = 531;
    pub const ROOF_TILES_CORNER_TOP_3: crate::blocks::Id = 532;
    pub const ROOF_TILES_VALLEY: crate::blocks::Id = 533;
    pub const ROOF_TILES_VALLEY_1: crate::blocks::Id = 534;
    pub const ROOF_TILES_VALLEY_2: crate::blocks::Id = 535;
    pub const ROOF_TILES_VALLEY_3: crate::blocks::Id = 536;
    pub const ROOF_TILES_VALLEY_TOP: crate::blocks::Id = 537;
    pub const ROOF_TILES_VALLEY_TOP_1: crate::blocks::Id = 538;
    pub const ROOF_TILES_VALLEY_TOP_2: crate::blocks::Id = 539;
    pub const ROOF_TILES_VALLEY_TOP_3: crate::blocks::Id = 540;
    pub const ROOF_TILES_RIDGE: crate::blocks::Id = 541;
    pub const ROOF_TILES_RIDGE_1: crate::blocks::Id = 542;
    pub const ROOF_TILES_RIDGE_TOP: crate::blocks::Id = 543;
    pub const ROOF_TILES_RIDGE_TOP_1: crate::blocks::Id = 544;
    pub const ROOF_TILES_PEAK: crate::blocks::Id = 545;
    pub const ROOF_TILES_PEAK_TOP: crate::blocks::Id = 546;
    pub const SLATE_SLOPE: crate::blocks::Id = 547;
    pub const SLATE_SLOPE_1: crate::blocks::Id = 548;
    pub const SLATE_SLOPE_2: crate::blocks::Id = 549;
    pub const SLATE_SLOPE_3: crate::blocks::Id = 550;
    pub const SLATE_SLOPE_TOP: crate::blocks::Id = 551;
    pub const SLATE_SLOPE_TOP_1: crate::blocks::Id = 552;
    pub const SLATE_SLOPE_TOP_2: crate::blocks::Id = 553;
    pub const SLATE_SLOPE_TOP_3: crate::blocks::Id = 554;
    pub const SLATE_CORNER: crate::blocks::Id = 555;
    pub const SLATE_CORNER_1: crate::blocks::Id = 556;
    pub const SLATE_CORNER_2: crate::blocks::Id = 557;
    pub const SLATE_CORNER_3: crate::blocks::Id = 558;
    pub const SLATE_CORNER_TOP: crate::blocks::Id = 559;
    pub const SLATE_CORNER_TOP_1: crate::blocks::Id = 560;
    pub const SLATE_CORNER_TOP_2: crate::blocks::Id = 561;
    pub const SLATE_CORNER_TOP_3: crate::blocks::Id = 562;
    pub const SLATE_VALLEY: crate::blocks::Id = 563;
    pub const SLATE_VALLEY_1: crate::blocks::Id = 564;
    pub const SLATE_VALLEY_2: crate::blocks::Id = 565;
    pub const SLATE_VALLEY_3: crate::blocks::Id = 566;
    pub const SLATE_VALLEY_TOP: crate::blocks::Id = 567;
    pub const SLATE_VALLEY_TOP_1: crate::blocks::Id = 568;
    pub const SLATE_VALLEY_TOP_2: crate::blocks::Id = 569;
    pub const SLATE_VALLEY_TOP_3: crate::blocks::Id = 570;
    pub const SLATE_RIDGE: crate::blocks::Id = 571;
    pub const SLATE_RIDGE_1: crate::blocks::Id = 572;
    pub const SLATE_RIDGE_TOP: crate::blocks::Id = 573;
    pub const SLATE_RIDGE_TOP_1: crate::blocks::Id = 574;
    pub const SLATE_PEAK: crate::blocks::Id = 575;
    pub const SLATE_PEAK_TOP: crate::blocks::Id = 576;
    pub const MOSSY_ROOF_TILES_SLOPE: crate::blocks::Id = 577;
    pub const MOSSY_ROOF_TILES_SLOPE_1: crate::blocks::Id = 578;
    pub const MOSSY_ROOF_TILES_SLOPE_2: crate::blocks::Id = 579;
    pub const MOSSY_ROOF_TILES_SLOPE_3: crate::blocks::Id = 580;
    pub const MOSSY_ROOF_TILES_SLOPE_TOP: crate::blocks::Id = 581;
    pub const MOSSY_ROOF_TILES_SLOPE_TOP_1: crate::blocks::Id = 582;
    pub const MOSSY_ROOF_TILES_SLOPE_TOP_2: crate::blocks::Id = 583;
    pub const MOSSY_ROOF_TILES_SLOPE_TOP_3: crate::blocks::Id = 584;
    pub const MOSSY_ROOF_TILES_CORNER: crate::blocks::Id = 585;
    pub const MOSSY_ROOF_TILES_CORNER_1: crate::blocks::Id = 586;
    pub const MOSSY_ROOF_TILES_CORNER_2: crate::blocks::Id = 587;
    pub const MOSSY_ROOF_TILES_CORNER_3: crate::blocks::Id = 588;
    pub const MOSSY_ROOF_TILES_CORNER_TOP: crate::blocks::Id = 589;
    pub const MOSSY_ROOF_TILES_CORNER_TOP_1: crate::blocks::Id = 590;
    pub const MOSSY_ROOF_TILES_CORNER_TOP_2: crate::blocks::Id = 591;
    pub const MOSSY_ROOF_TILES_CORNER_TOP_3: crate::blocks::Id = 592;
    pub const MOSSY_ROOF_TILES_VALLEY: crate::blocks::Id = 593;
    pub const MOSSY_ROOF_TILES_VALLEY_1: crate::blocks::Id = 594;
    pub const MOSSY_ROOF_TILES_VALLEY_2: crate::blocks::Id = 595;
    pub const MOSSY_ROOF_TILES_VALLEY_3: crate::blocks::Id = 596;
    pub const MOSSY_ROOF_TILES_VALLEY_TOP: crate::blocks::Id = 597;
    pub const MOSSY_ROOF_TILES_VALLEY_TOP_1: crate::blocks::Id = 598;
    pub const MOSSY_ROOF_TILES_VALLEY_TOP_2: crate::blocks::Id = 599;
    pub const MOSSY_ROOF_TILES_VALLEY_TOP_3: crate::blocks::Id = 600;
    pub const MOSSY_ROOF_TILES_RIDGE: crate::blocks::Id = 601;
    pub const MOSSY_ROOF_TILES_RIDGE_1: crate::blocks::Id = 602;
    pub const MOSSY_ROOF_TILES_RIDGE_TOP: crate::blocks::Id = 603;
    pub const MOSSY_ROOF_TILES_RIDGE_TOP_1: crate::blocks::Id = 604;
    pub const MOSSY_ROOF_TILES_PEAK: crate::blocks::Id = 605;
    pub const MOSSY_ROOF_TILES_PEAK_TOP: crate::blocks::Id = 606;
    pub const SHINGLES_SLOPE: crate::blocks::Id = 607;
    pub const SHINGLES_SLOPE_1: crate::blocks::Id = 608;
    pub const SHINGLES_SLOPE_2: crate::blocks::Id = 609;
    pub const SHINGLES_SLOPE_3: crate::blocks::Id = 610;
    pub const SHINGLES_SLOPE_TOP: crate::blocks::Id = 611;
    pub const SHINGLES_SLOPE_TOP_1: crate::blocks::Id = 612;
    pub const SHINGLES_SLOPE_TOP_2: crate::blocks::Id = 613;
    pub const SHINGLES_SLOPE_TOP_3: crate::blocks::Id = 614;
    pub const SHINGLES_CORNER: crate::blocks::Id = 615;
    pub const SHINGLES_CORNER_1: crate::blocks::Id = 616;
    pub const SHINGLES_CORNER_2: crate::blocks::Id = 617;
    pub const SHINGLES_CORNER_3: crate::blocks::Id = 618;
    pub const SHINGLES_CORNER_TOP: crate::blocks::Id = 619;
    pub const SHINGLES_CORNER_TOP_1: crate::blocks::Id = 620;
    pub const SHINGLES_CORNER_TOP_2: crate::blocks::Id = 621;
    pub const SHINGLES_CORNER_TOP_3: crate::blocks::Id = 622;
    pub const SHINGLES_VALLEY: crate::blocks::Id = 623;
    pub const SHINGLES_VALLEY_1: crate::blocks::Id = 624;
    pub const SHINGLES_VALLEY_2: crate::blocks::Id = 625;
    pub const SHINGLES_VALLEY_3: crate::blocks::Id = 626;
    pub const SHINGLES_VALLEY_TOP: crate::blocks::Id = 627;
    pub const SHINGLES_VALLEY_TOP_1: crate::blocks::Id = 628;
    pub const SHINGLES_VALLEY_TOP_2: crate::blocks::Id = 629;
    pub const SHINGLES_VALLEY_TOP_3: crate::blocks::Id = 630;
    pub const SHINGLES_RIDGE: crate::blocks::Id = 631;
    pub const SHINGLES_RIDGE_1: crate::blocks::Id = 632;
    pub const SHINGLES_RIDGE_TOP: crate::blocks::Id = 633;
    pub const SHINGLES_RIDGE_TOP_1: crate::blocks::Id = 634;
    pub const SHINGLES_PEAK: crate::blocks::Id = 635;
    pub const SHINGLES_PEAK_TOP: crate::blocks::Id = 636;
    pub const THATCH_SLOPE: crate::blocks::Id = 637;
    pub const THATCH_SLOPE_1: crate::blocks::Id = 638;
    pub const THATCH_SLOPE_2: crate::blocks::Id = 639;
    pub const THATCH_SLOPE_3: crate::blocks::Id = 640;
    pub const THATCH_SLOPE_TOP: crate::blocks::Id = 641;
    pub const THATCH_SLOPE_TOP_1: crate::blocks::Id = 642;
    pub const THATCH_SLOPE_TOP_2: crate::blocks::Id = 643;
    pub const THATCH_SLOPE_TOP_3: crate::blocks::Id = 644;
    pub const THATCH_CORNER: crate::blocks::Id = 645;
    pub const THATCH_CORNER_1: crate::blocks::Id = 646;
    pub const THATCH_CORNER_2: crate::blocks::Id = 647;
    pub const THATCH_CORNER_3: crate::blocks::Id = 648;
    pub const THATCH_CORNER_TOP: crate::blocks::Id = 649;
    pub const THATCH_CORNER_TOP_1: crate::blocks::Id = 650;
    pub const THATCH_CORNER_TOP_2: crate::blocks::Id = 651;
    pub const THATCH_CORNER_TOP_3: crate::blocks::Id = 652;
    pub const THATCH_VALLEY: crate::blocks::Id = 653;
    pub const THATCH_VALLEY_1: crate::blocks::Id = 654;
    pub const THATCH_VALLEY_2: crate::blocks::Id = 655;
    pub const THATCH_VALLEY_3: crate::blocks::Id = 656;
    pub const THATCH_VALLEY_TOP: crate::blocks::Id = 657;
    pub const THATCH_VALLEY_TOP_1: crate::blocks::Id = 658;
    pub const THATCH_VALLEY_TOP_2: crate::blocks::Id = 659;
    pub const THATCH_VALLEY_TOP_3: crate::blocks::Id = 660;
    pub const THATCH_RIDGE: crate::blocks::Id = 661;
    pub const THATCH_RIDGE_1: crate::blocks::Id = 662;
    pub const THATCH_RIDGE_TOP: crate::blocks::Id = 663;
    pub const THATCH_RIDGE_TOP_1: crate::blocks::Id = 664;
    pub const THATCH_PEAK: crate::blocks::Id = 665;
    pub const THATCH_PEAK_TOP: crate::blocks::Id = 666;
    pub const COPPER_ROOF_SLOPE: crate::blocks::Id = 667;
    pub const COPPER_ROOF_SLOPE_1: crate::blocks::Id = 668;
    pub const COPPER_ROOF_SLOPE_2: crate::blocks::Id = 669;
    pub const COPPER_ROOF_SLOPE_3: crate::blocks::Id = 670;
    pub const COPPER_ROOF_SLOPE_TOP: crate::blocks::Id = 671;
    pub const COPPER_ROOF_SLOPE_TOP_1: crate::blocks::Id = 672;
    pub const COPPER_ROOF_SLOPE_TOP_2: crate::blocks::Id = 673;
    pub const COPPER_ROOF_SLOPE_TOP_3: crate::blocks::Id = 674;
    pub const COPPER_ROOF_CORNER: crate::blocks::Id = 675;
    pub const COPPER_ROOF_CORNER_1: crate::blocks::Id = 676;
    pub const COPPER_ROOF_CORNER_2: crate::blocks::Id = 677;
    pub const COPPER_ROOF_CORNER_3: crate::blocks::Id = 678;
    pub const COPPER_ROOF_CORNER_TOP: crate::blocks::Id = 679;
    pub const COPPER_ROOF_CORNER_TOP_1: crate::blocks::Id = 680;
    pub const COPPER_ROOF_CORNER_TOP_2: crate::blocks::Id = 681;
    pub const COPPER_ROOF_CORNER_TOP_3: crate::blocks::Id = 682;
    pub const COPPER_ROOF_VALLEY: crate::blocks::Id = 683;
    pub const COPPER_ROOF_VALLEY_1: crate::blocks::Id = 684;
    pub const COPPER_ROOF_VALLEY_2: crate::blocks::Id = 685;
    pub const COPPER_ROOF_VALLEY_3: crate::blocks::Id = 686;
    pub const COPPER_ROOF_VALLEY_TOP: crate::blocks::Id = 687;
    pub const COPPER_ROOF_VALLEY_TOP_1: crate::blocks::Id = 688;
    pub const COPPER_ROOF_VALLEY_TOP_2: crate::blocks::Id = 689;
    pub const COPPER_ROOF_VALLEY_TOP_3: crate::blocks::Id = 690;
    pub const COPPER_ROOF_RIDGE: crate::blocks::Id = 691;
    pub const COPPER_ROOF_RIDGE_1: crate::blocks::Id = 692;
    pub const COPPER_ROOF_RIDGE_TOP: crate::blocks::Id = 693;
    pub const COPPER_ROOF_RIDGE_TOP_1: crate::blocks::Id = 694;
    pub const COPPER_ROOF_PEAK: crate::blocks::Id = 695;
    pub const COPPER_ROOF_PEAK_TOP: crate::blocks::Id = 696;
    pub const PLANKS_SLOPE: crate::blocks::Id = 697;
    pub const PLANKS_SLOPE_1: crate::blocks::Id = 698;
    pub const PLANKS_SLOPE_2: crate::blocks::Id = 699;
    pub const PLANKS_SLOPE_3: crate::blocks::Id = 700;
    pub const PLANKS_SLOPE_TOP: crate::blocks::Id = 701;
    pub const PLANKS_SLOPE_TOP_1: crate::blocks::Id = 702;
    pub const PLANKS_SLOPE_TOP_2: crate::blocks::Id = 703;
    pub const PLANKS_SLOPE_TOP_3: crate::blocks::Id = 704;
    pub const PLANKS_CORNER: crate::blocks::Id = 705;
    pub const PLANKS_CORNER_1: crate::blocks::Id = 706;
    pub const PLANKS_CORNER_2: crate::blocks::Id = 707;
    pub const PLANKS_CORNER_3: crate::blocks::Id = 708;
    pub const PLANKS_CORNER_TOP: crate::blocks::Id = 709;
    pub const PLANKS_CORNER_TOP_1: crate::blocks::Id = 710;
    pub const PLANKS_CORNER_TOP_2: crate::blocks::Id = 711;
    pub const PLANKS_CORNER_TOP_3: crate::blocks::Id = 712;
    pub const PLANKS_VALLEY: crate::blocks::Id = 713;
    pub const PLANKS_VALLEY_1: crate::blocks::Id = 714;
    pub const PLANKS_VALLEY_2: crate::blocks::Id = 715;
    pub const PLANKS_VALLEY_3: crate::blocks::Id = 716;
    pub const PLANKS_VALLEY_TOP: crate::blocks::Id = 717;
    pub const PLANKS_VALLEY_TOP_1: crate::blocks::Id = 718;
    pub const PLANKS_VALLEY_TOP_2: crate::blocks::Id = 719;
    pub const PLANKS_VALLEY_TOP_3: crate::blocks::Id = 720;
    pub const PLANKS_RIDGE: crate::blocks::Id = 721;
    pub const PLANKS_RIDGE_1: crate::blocks::Id = 722;
    pub const PLANKS_RIDGE_TOP: crate::blocks::Id = 723;
    pub const PLANKS_RIDGE_TOP_1: crate::blocks::Id = 724;
    pub const PLANKS_PEAK: crate::blocks::Id = 725;
    pub const PLANKS_PEAK_TOP: crate::blocks::Id = 726;
    pub const CHAIR: crate::blocks::Id = 727;
    pub const BENCH: crate::blocks::Id = 731;
    pub const ARMCHAIR: crate::blocks::Id = 735;
    pub const THRONE: crate::blocks::Id = 739;
    pub const CUPBOARD: crate::blocks::Id = 743;
    pub const FIREPLACE: crate::blocks::Id = 747;
    pub const LOOM: crate::blocks::Id = 751;
    pub const SPINNING_WHEEL: crate::blocks::Id = 755;
    pub const GRINDSTONE: crate::blocks::Id = 759;
    pub const SIGNPOST: crate::blocks::Id = 763;
    pub const PAINTING: crate::blocks::Id = 767;
    pub const BANNER_RED: crate::blocks::Id = 771;
    pub const BANNER_BLUE: crate::blocks::Id = 775;
    pub const BANNER_GREEN: crate::blocks::Id = 779;
    pub const BANNER_GOLD: crate::blocks::Id = 783;
    pub const WEAPON_RACK: crate::blocks::Id = 787;
    pub const WALL_SCONCE: crate::blocks::Id = 791;
    pub const CURTAIN: crate::blocks::Id = 795;
    pub const WALL_SHELF: crate::blocks::Id = 799;
    pub const TABLE: crate::blocks::Id = 803;
    pub const ROUND_TABLE: crate::blocks::Id = 804;
    pub const STOOL: crate::blocks::Id = 805;
    pub const CANDLE: crate::blocks::Id = 806;
    pub const CANDELABRA: crate::blocks::Id = 807;
    pub const CHANDELIER: crate::blocks::Id = 808;
    pub const BRAZIER: crate::blocks::Id = 809;
    pub const SACK: crate::blocks::Id = 810;
    pub const AMPHORA: crate::blocks::Id = 811;
    pub const FLOWER_POT: crate::blocks::Id = 812;
    pub const POTTED_FERN: crate::blocks::Id = 813;
    pub const CAULDRON: crate::blocks::Id = 814;
    pub const BUCKET: crate::blocks::Id = 815;
    pub const LOG_PILE: crate::blocks::Id = 816;
    pub const CARPET_RED: crate::blocks::Id = 817;
    pub const CARPET_BLUE: crate::blocks::Id = 818;
    pub const CARPET_GREEN: crate::blocks::Id = 819;
    pub const IRON_BARS: crate::blocks::Id = 820;
    pub const BOOKS_PILE: crate::blocks::Id = 821;
    pub const BOTTLES: crate::blocks::Id = 822;
    pub const BEEHIVE: crate::blocks::Id = 823;
    pub const CHEESE: crate::blocks::Id = 824;
    pub const BREAD_BASKET: crate::blocks::Id = 825;
    pub const TORCH_WALL: crate::blocks::Id = 826;
    pub const BED_FOOT: crate::blocks::Id = 833;
    pub const CHEST_OPEN: crate::blocks::Id = 837;
    pub const FURNACE_LIT: crate::blocks::Id = 844;
    pub const EDELWEISS: crate::blocks::Id = 845;
    pub const GENTIAN: crate::blocks::Id = 846;
    pub const SEAGRASS: crate::blocks::Id = 847;
    pub const KELP: crate::blocks::Id = 848;
    pub const KELP_TOP: crate::blocks::Id = 849;
    pub const CORAL: crate::blocks::Id = 850;
    pub const CORAL_FAN: crate::blocks::Id = 851;
    pub const SHELL: crate::blocks::Id = 852;
    pub const MARRAM: crate::blocks::Id = 853;
    pub const CORAL_BLOCK_RED: crate::blocks::Id = 854;
    pub const CORAL_BLOCK_ORANGE: crate::blocks::Id = 855;
    pub const CORAL_BLOCK_PURPLE: crate::blocks::Id = 856;
    pub const CORAL_BLOCK_YELLOW: crate::blocks::Id = 857;
    pub const CORAL_BLOCK_BLUE: crate::blocks::Id = 858;
    pub const WRECK_PLANKS: crate::blocks::Id = 859;
    pub const WALL_CLOCK: crate::blocks::Id = 860;
    pub const HIDES: crate::blocks::Id = 864;
    pub const SAWHORSE: crate::blocks::Id = 868;
    pub const BELL: crate::blocks::Id = 872;
    pub const CHAIN: crate::blocks::Id = 873;
    pub const CROSS: crate::blocks::Id = 874;
    pub const FLAG: crate::blocks::Id = 875;
    pub const TANNING_VAT: crate::blocks::Id = 876;
    pub const LIME_VAT: crate::blocks::Id = 877;
    pub const CHOPPING_BLOCK: crate::blocks::Id = 878;
    pub const CART: crate::blocks::Id = 879;
    pub const TROUGH: crate::blocks::Id = 883;
    pub const TOOL_RACK: crate::blocks::Id = 887;
    pub const COAL_HEAP: crate::blocks::Id = 891;
    pub const FISHING_NET: crate::blocks::Id = 892;
    pub const FISH_RACK: crate::blocks::Id = 896;
    pub const ANCHOR: crate::blocks::Id = 900;
    pub const OARS: crate::blocks::Id = 904;
    pub const ROPE_RAIL: crate::blocks::Id = 908;
    pub const SAIL: crate::blocks::Id = 912;
    pub const BOAT_BENCH: crate::blocks::Id = 916;
    pub const CREEL: crate::blocks::Id = 920;
    pub const MOORING_POST: crate::blocks::Id = 921;
    pub const BUOY: crate::blocks::Id = 922;
    pub const NET_PILE: crate::blocks::Id = 923;
    pub const PENNANT: crate::blocks::Id = 924;
    pub const FISH_CRATE: crate::blocks::Id = 925;
    pub const FLOWSTONE: crate::blocks::Id = 926;
    pub const DRIPSTONE_DOWN_0: crate::blocks::Id = 927;
    pub const DRIPSTONE_DOWN_1: crate::blocks::Id = 928;
    pub const DRIPSTONE_DOWN_2: crate::blocks::Id = 929;
    pub const DRIPSTONE_UP_0: crate::blocks::Id = 930;
    pub const DRIPSTONE_UP_1: crate::blocks::Id = 931;
    pub const DRIPSTONE_UP_2: crate::blocks::Id = 932;
    pub const BASALT: crate::blocks::Id = 933;
    pub const OBSIDIAN: crate::blocks::Id = 934;
    pub const LAVA: crate::blocks::Id = 935;
    pub const MAGMA: crate::blocks::Id = 936;
    pub const CRYSTAL_VIOLET: crate::blocks::Id = 937;
    pub const CRYSTAL_BLUE: crate::blocks::Id = 938;
    pub const CRYSTAL_PINK: crate::blocks::Id = 939;
    pub const CRYSTAL_WHITE: crate::blocks::Id = 940;
    pub const ABYSS_ROCK: crate::blocks::Id = 941;
    pub const GLOW_MOSS: crate::blocks::Id = 942;
    pub const MYCELIUM: crate::blocks::Id = 943;
    pub const MUSHROOM_STEM: crate::blocks::Id = 944;
    pub const MUSHROOM_CAP_RED: crate::blocks::Id = 945;
    pub const MUSHROOM_CAP_VIOLET: crate::blocks::Id = 946;
    pub const MUSHROOM_CAP_BLUE: crate::blocks::Id = 947;
    pub const MUSHROOM_GILLS: crate::blocks::Id = 948;
    pub const ICE_BLUE: crate::blocks::Id = 949;
    pub const OAK_POST: crate::blocks::Id = 950;
    pub const OAK_POST_X: crate::blocks::Id = 951;
    pub const OAK_POST_Z: crate::blocks::Id = 952;
    pub const OAK_POST_DIAG: crate::blocks::Id = 953;
    pub const OAK_POST_DIAG_1: crate::blocks::Id = 954;
    pub const OAK_POST_DIAG_2: crate::blocks::Id = 955;
    pub const OAK_POST_DIAG_3: crate::blocks::Id = 956;
    pub const OAK_POST_DIAG_FLAT: crate::blocks::Id = 957;
    pub const OAK_POST_DIAG_FLAT_1: crate::blocks::Id = 958;
    pub const OAK_POST_ANGLE: crate::blocks::Id = 959;
    pub const OAK_POST_ANGLE_1: crate::blocks::Id = 960;
    pub const OAK_POST_ANGLE_2: crate::blocks::Id = 961;
    pub const OAK_POST_ANGLE_3: crate::blocks::Id = 962;
    pub const OAK_POST_ANGLE_TOP: crate::blocks::Id = 963;
    pub const OAK_POST_ANGLE_TOP_1: crate::blocks::Id = 964;
    pub const OAK_POST_ANGLE_TOP_2: crate::blocks::Id = 965;
    pub const OAK_POST_ANGLE_TOP_3: crate::blocks::Id = 966;
    pub const BIRCH_POST: crate::blocks::Id = 967;
    pub const BIRCH_POST_X: crate::blocks::Id = 968;
    pub const BIRCH_POST_Z: crate::blocks::Id = 969;
    pub const BIRCH_POST_DIAG: crate::blocks::Id = 970;
    pub const BIRCH_POST_DIAG_1: crate::blocks::Id = 971;
    pub const BIRCH_POST_DIAG_2: crate::blocks::Id = 972;
    pub const BIRCH_POST_DIAG_3: crate::blocks::Id = 973;
    pub const BIRCH_POST_DIAG_FLAT: crate::blocks::Id = 974;
    pub const BIRCH_POST_DIAG_FLAT_1: crate::blocks::Id = 975;
    pub const BIRCH_POST_ANGLE: crate::blocks::Id = 976;
    pub const BIRCH_POST_ANGLE_1: crate::blocks::Id = 977;
    pub const BIRCH_POST_ANGLE_2: crate::blocks::Id = 978;
    pub const BIRCH_POST_ANGLE_3: crate::blocks::Id = 979;
    pub const BIRCH_POST_ANGLE_TOP: crate::blocks::Id = 980;
    pub const BIRCH_POST_ANGLE_TOP_1: crate::blocks::Id = 981;
    pub const BIRCH_POST_ANGLE_TOP_2: crate::blocks::Id = 982;
    pub const BIRCH_POST_ANGLE_TOP_3: crate::blocks::Id = 983;
    pub const BEECH_POST: crate::blocks::Id = 984;
    pub const BEECH_POST_X: crate::blocks::Id = 985;
    pub const BEECH_POST_Z: crate::blocks::Id = 986;
    pub const BEECH_POST_DIAG: crate::blocks::Id = 987;
    pub const BEECH_POST_DIAG_1: crate::blocks::Id = 988;
    pub const BEECH_POST_DIAG_2: crate::blocks::Id = 989;
    pub const BEECH_POST_DIAG_3: crate::blocks::Id = 990;
    pub const BEECH_POST_DIAG_FLAT: crate::blocks::Id = 991;
    pub const BEECH_POST_DIAG_FLAT_1: crate::blocks::Id = 992;
    pub const BEECH_POST_ANGLE: crate::blocks::Id = 993;
    pub const BEECH_POST_ANGLE_1: crate::blocks::Id = 994;
    pub const BEECH_POST_ANGLE_2: crate::blocks::Id = 995;
    pub const BEECH_POST_ANGLE_3: crate::blocks::Id = 996;
    pub const BEECH_POST_ANGLE_TOP: crate::blocks::Id = 997;
    pub const BEECH_POST_ANGLE_TOP_1: crate::blocks::Id = 998;
    pub const BEECH_POST_ANGLE_TOP_2: crate::blocks::Id = 999;
    pub const BEECH_POST_ANGLE_TOP_3: crate::blocks::Id = 1000;
    pub const SPRUCE_POST: crate::blocks::Id = 1001;
    pub const SPRUCE_POST_X: crate::blocks::Id = 1002;
    pub const SPRUCE_POST_Z: crate::blocks::Id = 1003;
    pub const SPRUCE_POST_DIAG: crate::blocks::Id = 1004;
    pub const SPRUCE_POST_DIAG_1: crate::blocks::Id = 1005;
    pub const SPRUCE_POST_DIAG_2: crate::blocks::Id = 1006;
    pub const SPRUCE_POST_DIAG_3: crate::blocks::Id = 1007;
    pub const SPRUCE_POST_DIAG_FLAT: crate::blocks::Id = 1008;
    pub const SPRUCE_POST_DIAG_FLAT_1: crate::blocks::Id = 1009;
    pub const SPRUCE_POST_ANGLE: crate::blocks::Id = 1010;
    pub const SPRUCE_POST_ANGLE_1: crate::blocks::Id = 1011;
    pub const SPRUCE_POST_ANGLE_2: crate::blocks::Id = 1012;
    pub const SPRUCE_POST_ANGLE_3: crate::blocks::Id = 1013;
    pub const SPRUCE_POST_ANGLE_TOP: crate::blocks::Id = 1014;
    pub const SPRUCE_POST_ANGLE_TOP_1: crate::blocks::Id = 1015;
    pub const SPRUCE_POST_ANGLE_TOP_2: crate::blocks::Id = 1016;
    pub const SPRUCE_POST_ANGLE_TOP_3: crate::blocks::Id = 1017;
    pub const ACACIA_POST: crate::blocks::Id = 1018;
    pub const ACACIA_POST_X: crate::blocks::Id = 1019;
    pub const ACACIA_POST_Z: crate::blocks::Id = 1020;
    pub const ACACIA_POST_DIAG: crate::blocks::Id = 1021;
    pub const ACACIA_POST_DIAG_1: crate::blocks::Id = 1022;
    pub const ACACIA_POST_DIAG_2: crate::blocks::Id = 1023;
    pub const ACACIA_POST_DIAG_3: crate::blocks::Id = 1024;
    pub const ACACIA_POST_DIAG_FLAT: crate::blocks::Id = 1025;
    pub const ACACIA_POST_DIAG_FLAT_1: crate::blocks::Id = 1026;
    pub const ACACIA_POST_ANGLE: crate::blocks::Id = 1027;
    pub const ACACIA_POST_ANGLE_1: crate::blocks::Id = 1028;
    pub const ACACIA_POST_ANGLE_2: crate::blocks::Id = 1029;
    pub const ACACIA_POST_ANGLE_3: crate::blocks::Id = 1030;
    pub const ACACIA_POST_ANGLE_TOP: crate::blocks::Id = 1031;
    pub const ACACIA_POST_ANGLE_TOP_1: crate::blocks::Id = 1032;
    pub const ACACIA_POST_ANGLE_TOP_2: crate::blocks::Id = 1033;
    pub const ACACIA_POST_ANGLE_TOP_3: crate::blocks::Id = 1034;
    pub const STONE_POINT_DOWN_0: crate::blocks::Id = 1035;
    pub const STONE_POINT_DOWN_1: crate::blocks::Id = 1036;
    pub const STONE_POINT_DOWN_2: crate::blocks::Id = 1037;
    pub const STONE_POINT_UP_0: crate::blocks::Id = 1038;
    pub const STONE_POINT_UP_1: crate::blocks::Id = 1039;
    pub const STONE_POINT_UP_2: crate::blocks::Id = 1040;
    pub const ICE_POINT_DOWN_0: crate::blocks::Id = 1041;
    pub const ICE_POINT_DOWN_1: crate::blocks::Id = 1042;
    pub const ICE_POINT_DOWN_2: crate::blocks::Id = 1043;
    pub const ICE_POINT_UP_0: crate::blocks::Id = 1044;
    pub const ICE_POINT_UP_1: crate::blocks::Id = 1045;
    pub const ICE_POINT_UP_2: crate::blocks::Id = 1046;
    pub const BASALT_POINT_DOWN_0: crate::blocks::Id = 1047;
    pub const BASALT_POINT_DOWN_1: crate::blocks::Id = 1048;
    pub const BASALT_POINT_DOWN_2: crate::blocks::Id = 1049;
    pub const BASALT_POINT_UP_0: crate::blocks::Id = 1050;
    pub const BASALT_POINT_UP_1: crate::blocks::Id = 1051;
    pub const BASALT_POINT_UP_2: crate::blocks::Id = 1052;
    pub const SANDSTONE_POINT_DOWN_0: crate::blocks::Id = 1053;
    pub const SANDSTONE_POINT_DOWN_1: crate::blocks::Id = 1054;
    pub const SANDSTONE_POINT_DOWN_2: crate::blocks::Id = 1055;
    pub const SANDSTONE_POINT_UP_0: crate::blocks::Id = 1056;
    pub const SANDSTONE_POINT_UP_1: crate::blocks::Id = 1057;
    pub const SANDSTONE_POINT_UP_2: crate::blocks::Id = 1058;
    pub const CRYSTAL_VIOLET_POINT_DOWN_0: crate::blocks::Id = 1059;
    pub const CRYSTAL_VIOLET_POINT_DOWN_1: crate::blocks::Id = 1060;
    pub const CRYSTAL_VIOLET_POINT_DOWN_2: crate::blocks::Id = 1061;
    pub const CRYSTAL_VIOLET_POINT_UP_0: crate::blocks::Id = 1062;
    pub const CRYSTAL_VIOLET_POINT_UP_1: crate::blocks::Id = 1063;
    pub const CRYSTAL_VIOLET_POINT_UP_2: crate::blocks::Id = 1064;
    pub const CRYSTAL_BLUE_POINT_DOWN_0: crate::blocks::Id = 1065;
    pub const CRYSTAL_BLUE_POINT_DOWN_1: crate::blocks::Id = 1066;
    pub const CRYSTAL_BLUE_POINT_DOWN_2: crate::blocks::Id = 1067;
    pub const CRYSTAL_BLUE_POINT_UP_0: crate::blocks::Id = 1068;
    pub const CRYSTAL_BLUE_POINT_UP_1: crate::blocks::Id = 1069;
    pub const CRYSTAL_BLUE_POINT_UP_2: crate::blocks::Id = 1070;
    pub const CRYSTAL_PINK_POINT_DOWN_0: crate::blocks::Id = 1071;
    pub const CRYSTAL_PINK_POINT_DOWN_1: crate::blocks::Id = 1072;
    pub const CRYSTAL_PINK_POINT_DOWN_2: crate::blocks::Id = 1073;
    pub const CRYSTAL_PINK_POINT_UP_0: crate::blocks::Id = 1074;
    pub const CRYSTAL_PINK_POINT_UP_1: crate::blocks::Id = 1075;
    pub const CRYSTAL_PINK_POINT_UP_2: crate::blocks::Id = 1076;
    pub const CRYSTAL_WHITE_POINT_DOWN_0: crate::blocks::Id = 1077;
    pub const CRYSTAL_WHITE_POINT_DOWN_1: crate::blocks::Id = 1078;
    pub const CRYSTAL_WHITE_POINT_DOWN_2: crate::blocks::Id = 1079;
    pub const CRYSTAL_WHITE_POINT_UP_0: crate::blocks::Id = 1080;
    pub const CRYSTAL_WHITE_POINT_UP_1: crate::blocks::Id = 1081;
    pub const CRYSTAL_WHITE_POINT_UP_2: crate::blocks::Id = 1082;
    pub const SAPLING: crate::blocks::Id = 1083;
    pub const GRAVESTONE: crate::blocks::Id = 1084;
    pub const GRAVE_CROSS: crate::blocks::Id = 1088;
    pub const STONE_SLAB: crate::blocks::Id = 1092;
    pub const STONE_SLAB_TOP: crate::blocks::Id = 1093;
    pub const STONE_STAIRS: crate::blocks::Id = 1094;
    pub const STONE_STAIRS_1: crate::blocks::Id = 1095;
    pub const STONE_STAIRS_2: crate::blocks::Id = 1096;
    pub const STONE_STAIRS_3: crate::blocks::Id = 1097;
    pub const STONE_STAIRS_TOP: crate::blocks::Id = 1098;
    pub const STONE_STAIRS_TOP_1: crate::blocks::Id = 1099;
    pub const STONE_STAIRS_TOP_2: crate::blocks::Id = 1100;
    pub const STONE_STAIRS_TOP_3: crate::blocks::Id = 1101;
    pub const STONE_VSLAB: crate::blocks::Id = 1102;
    pub const STONE_VSLAB_1: crate::blocks::Id = 1103;
    pub const STONE_VSLAB_2: crate::blocks::Id = 1104;
    pub const STONE_VSLAB_3: crate::blocks::Id = 1105;
    pub const LIMESTONE_SLAB: crate::blocks::Id = 1106;
    pub const LIMESTONE_SLAB_TOP: crate::blocks::Id = 1107;
    pub const LIMESTONE_STAIRS: crate::blocks::Id = 1108;
    pub const LIMESTONE_STAIRS_1: crate::blocks::Id = 1109;
    pub const LIMESTONE_STAIRS_2: crate::blocks::Id = 1110;
    pub const LIMESTONE_STAIRS_3: crate::blocks::Id = 1111;
    pub const LIMESTONE_STAIRS_TOP: crate::blocks::Id = 1112;
    pub const LIMESTONE_STAIRS_TOP_1: crate::blocks::Id = 1113;
    pub const LIMESTONE_STAIRS_TOP_2: crate::blocks::Id = 1114;
    pub const LIMESTONE_STAIRS_TOP_3: crate::blocks::Id = 1115;
    pub const LIMESTONE_VSLAB: crate::blocks::Id = 1116;
    pub const LIMESTONE_VSLAB_1: crate::blocks::Id = 1117;
    pub const LIMESTONE_VSLAB_2: crate::blocks::Id = 1118;
    pub const LIMESTONE_VSLAB_3: crate::blocks::Id = 1119;
    pub const GRANITE_SLAB: crate::blocks::Id = 1120;
    pub const GRANITE_SLAB_TOP: crate::blocks::Id = 1121;
    pub const GRANITE_STAIRS: crate::blocks::Id = 1122;
    pub const GRANITE_STAIRS_1: crate::blocks::Id = 1123;
    pub const GRANITE_STAIRS_2: crate::blocks::Id = 1124;
    pub const GRANITE_STAIRS_3: crate::blocks::Id = 1125;
    pub const GRANITE_STAIRS_TOP: crate::blocks::Id = 1126;
    pub const GRANITE_STAIRS_TOP_1: crate::blocks::Id = 1127;
    pub const GRANITE_STAIRS_TOP_2: crate::blocks::Id = 1128;
    pub const GRANITE_STAIRS_TOP_3: crate::blocks::Id = 1129;
    pub const GRANITE_VSLAB: crate::blocks::Id = 1130;
    pub const GRANITE_VSLAB_1: crate::blocks::Id = 1131;
    pub const GRANITE_VSLAB_2: crate::blocks::Id = 1132;
    pub const GRANITE_VSLAB_3: crate::blocks::Id = 1133;
    pub const SANDSTONE_SLAB: crate::blocks::Id = 1134;
    pub const SANDSTONE_SLAB_TOP: crate::blocks::Id = 1135;
    pub const SANDSTONE_STAIRS: crate::blocks::Id = 1136;
    pub const SANDSTONE_STAIRS_1: crate::blocks::Id = 1137;
    pub const SANDSTONE_STAIRS_2: crate::blocks::Id = 1138;
    pub const SANDSTONE_STAIRS_3: crate::blocks::Id = 1139;
    pub const SANDSTONE_STAIRS_TOP: crate::blocks::Id = 1140;
    pub const SANDSTONE_STAIRS_TOP_1: crate::blocks::Id = 1141;
    pub const SANDSTONE_STAIRS_TOP_2: crate::blocks::Id = 1142;
    pub const SANDSTONE_STAIRS_TOP_3: crate::blocks::Id = 1143;
    pub const SANDSTONE_VSLAB: crate::blocks::Id = 1144;
    pub const SANDSTONE_VSLAB_1: crate::blocks::Id = 1145;
    pub const SANDSTONE_VSLAB_2: crate::blocks::Id = 1146;
    pub const SANDSTONE_VSLAB_3: crate::blocks::Id = 1147;
    pub const COBWEB: crate::blocks::Id = 1148;
    pub const GLOW_JAR: crate::blocks::Id = 1149;
    pub const GLOW_VINE: crate::blocks::Id = 1150;
    pub const ROOTS: crate::blocks::Id = 1151;
    pub const RAIL_X: crate::blocks::Id = 1152;
    pub const RAIL_Z: crate::blocks::Id = 1153;
    pub const LEVER: crate::blocks::Id = 1157;
    pub const LEVER_ON: crate::blocks::Id = 1158;
    pub const PRESSURE_PLATE: crate::blocks::Id = 1165;
    pub const PRESSURE_PLATE_ON: crate::blocks::Id = 1166;
    pub const PUSH_BLOCK: crate::blocks::Id = 1167;
    pub const CRUMBLING_FLOOR: crate::blocks::Id = 1168;
    pub const DART_TRAP: crate::blocks::Id = 1169;
    pub const SPIKES: crate::blocks::Id = 1170;
    pub const SPIKES_DOWN: crate::blocks::Id = 1171;
}
