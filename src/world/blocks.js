// VoxelWorld/blocks.js — registre de blocs défini par des données.
// Ajouter un bloc = ajouter une ligne ici, sans toucher au moteur.

export const CX = 32, CH = 128, CA = CX * CX * CH; // taille d'une colonne de chunk
export const idx = (x, y, z) => x + z * CX + y * CX * CX;

// Matériaux : chaque matériau occupe VARIANTS couches consécutives du texture array.
export const VARIANTS = 4;
export const MATERIALS = ['grass_top', 'grass_side', 'dirt', 'path', 'stone', 'mossy_stone', 'limestone', 'granite', 'gravel', 'sand',
  'bark_oak', 'bark_birch', 'bark_beech', 'leaf_oak', 'leaf_birch', 'leaf_beech', 'planks', 'forest_soil', 'water_foam', 'end_oak', 'end_birch', 'end_beech',
  'snow', 'snow_side', 'ice', 'bark_spruce', 'end_spruce', 'leaf_spruce', 'leaf_snow',
  'grass_dry_top', 'grass_dry_side', 'dry_earth', 'bark_dead', 'end_dead', 'lichen', 'bark_acacia', 'end_acacia', 'leaf_acacia', 'bone', 'wood_inner',
  'sandstone', 'sandstone_top', 'red_sandstone', 'cactus_side', 'cactus_top', 'bark_palm', 'end_palm', 'leaf_palm',
  // Survie : minerais, argile, matériaux de construction, terre labourée (moteur final)
  'ore_copper', 'ore_tin', 'ore_iron', 'ore_coal', 'clay', 'stone_bricks', 'bricks', 'thatch', 'planks_birch', 'planks_spruce', 'cobble', 'farmland_top',
  // Village médiéval : torchis blanc, poutres sombres, tuiles de terre cuite, ardoise (quatre de plus : le tableau de textures est plein à 256 couches).
  'plaster', 'dark_timber', 'roof_tiles', 'slate',
  // Richesse (au-delà de 256 couches : le moteur demande à la carte graphique la limite qu'elle offre) : pierres taillées, sols,
  // toits, tissus, pans de bois, mobilier.
  'marble', 'ashlar', 'fieldstone', 'setts', 'polished_stone', 'checker', 'terracotta', 'parquet', 'shingles', 'wool', 'rug',
  'frame_square', 'frame_cross', 'frame_diag', 'wattle', 'books', 'hay', 'crate', 'chiseled', 'stained_glass', 'iron_plate', 'verdigris',
  // Mer : corail massif (teinté par bloc) et bordages d'épave rongés d'algues.
  'coral', 'planks_wreck',
  // Refonte des villages : pierre calcaire appareillée, moellons gris, bout de poutre, bardage de grange, tuiles moussues, chemin de terre.
  'lime_masonry', 'rubble', 'beam_end', 'boards', 'tiles_moss', 'path_top'];
export const matLayer = (name) => MATERIALS.indexOf(name) * VARIANTS;

const defs = [
  // nom, shape, matériaux [haut, côté, bas], dureté (s), propriétés
  ['air', 'air', null, 0, {}],
  ['grass', 'cube', ['grass_top', 'grass_side', 'dirt'], 0.5, { drop: 'dirt', cat: 'terrain' }],
  ['dirt', 'cube', ['dirt', 'dirt', 'dirt'], 0.45, { cat: 'terrain' }],
  ['path', 'cube', ['path', 'dirt', 'dirt'], 0.45, { drop: 'dirt', cat: 'terrain' }],
  ['forest_soil', 'cube', ['forest_soil', 'dirt', 'dirt'], 0.5, { drop: 'dirt', cat: 'terrain' }],
  ['stone', 'cube', ['stone', 'stone', 'stone'], 1.3, { cat: 'rock' }],
  ['mossy_stone', 'cube', ['mossy_stone', 'mossy_stone', 'stone'], 1.3, { cat: 'rock' }],
  ['limestone', 'cube', ['limestone', 'limestone', 'limestone'], 1.2, { cat: 'rock' }],
  ['granite', 'cube', ['granite', 'granite', 'granite'], 1.6, { cat: 'rock' }],
  ['gravel', 'cube', ['gravel', 'gravel', 'gravel'], 0.5, { cat: 'terrain' }],
  ['sand', 'cube', ['sand', 'sand', 'sand'], 0.4, { cat: 'terrain' }],
  ['oak_log', 'cube', ['end_oak', 'bark_oak', 'end_oak'], 1.0, { cat: 'wood' }],
  ['birch_log', 'cube', ['end_birch', 'bark_birch', 'end_birch'], 0.9, { cat: 'wood' }],
  ['beech_log', 'cube', ['end_beech', 'bark_beech', 'end_beech'], 1.0, { cat: 'wood' }],
  ['oak_leaves', 'leaf', ['leaf_oak', 'leaf_oak', 'leaf_oak'], 0.12, { cat: 'foliage', drop: null, tint: [1, 1, 0.9] }],
  ['birch_leaves', 'leaf', ['leaf_birch', 'leaf_birch', 'leaf_birch'], 0.12, { cat: 'foliage', drop: null, tint: [1.08, 1.08, 0.82] }],
  ['beech_leaves', 'leaf', ['leaf_beech', 'leaf_beech', 'leaf_beech'], 0.12, { cat: 'foliage', drop: null, tint: [0.92, 1.02, 0.86] }],
  ['planks', 'cube', ['planks', 'planks', 'planks'], 0.8, { cat: 'wood' }],
  ['water', 'water', null, 0, { cat: 'fluid' }],
  // Décor : vrais blocs persistants, sans collision, rendus en géométrie enrichie.
  ['tall_grass', 'deco', null, 0.02, { model: 'grass', drop: null }],
  ['fern', 'deco', null, 0.02, { model: 'fern', drop: null }],
  ['daisy', 'deco', null, 0.02, { model: 'daisy', drop: null }],
  ['lupine', 'deco', null, 0.02, { model: 'lupine', drop: null }],
  ['pink_flower', 'deco', null, 0.02, { model: 'pink', drop: null }],
  ['yellow_flower', 'deco', null, 0.02, { model: 'yellow', drop: null }],
  ['mushroom', 'deco', null, 0.02, { model: 'mushroom', drop: null }],
  ['lily_pad', 'deco', null, 0.02, { model: 'lily', drop: null }],
  ['flagstone', 'deco', null, 0.3, { model: 'flagstone', drop: 'stone' }],
  ['lantern', 'deco', null, 0.3, { model: 'lantern', light: 14, cat: 'light' }],
  ['vine', 'deco', null, 0.02, { model: 'vine', drop: null }],
  ['shelf_fungus', 'deco', null, 0.02, { model: 'fungus', drop: null }],
  ['fence', 'deco', null, 0.6, { model: 'fence', solid: true, drop: 'planks', cat: 'wood' }],
  // Variantes orientées, toujours en fin de liste : les identifiants des sauvegardes restent stables.
  // Matériaux donnés dans l'ordre [faces Y, faces X, faces Z].
  ['oak_log_x', 'cube', ['bark_oak', 'end_oak', 'bark_oak'], 1.0, { cat: 'wood', drop: 'oak_log', axis: 'x' }],
  ['oak_log_z', 'cube', ['bark_oak', 'bark_oak', 'end_oak'], 1.0, { cat: 'wood', drop: 'oak_log', axis: 'z' }],
  ['birch_log_x', 'cube', ['bark_birch', 'end_birch', 'bark_birch'], 0.9, { cat: 'wood', drop: 'birch_log', axis: 'x' }],
  ['birch_log_z', 'cube', ['bark_birch', 'bark_birch', 'end_birch'], 0.9, { cat: 'wood', drop: 'birch_log', axis: 'z' }],
  ['beech_log_x', 'cube', ['bark_beech', 'end_beech', 'bark_beech'], 1.0, { cat: 'wood', drop: 'beech_log', axis: 'x' }],
  ['beech_log_z', 'cube', ['bark_beech', 'bark_beech', 'end_beech'], 1.0, { cat: 'wood', drop: 'beech_log', axis: 'z' }],
  ['planks_z', 'cube', ['planks', 'planks', 'planks'], 0.8, { cat: 'wood', drop: 'planks', axis: 'pz' }],
  ['planks_y', 'cube', ['planks', 'planks', 'planks'], 0.8, { cat: 'wood', drop: 'planks', axis: 'py' }],
  // eau qui coule : le chiffre est son niveau, de 1 (filet) à 7 (presque plein)
  ['water_1', 'water', null, 0, { cat: 'fluid', drop: null }],
  ['water_2', 'water', null, 0, { cat: 'fluid', drop: null }],
  ['water_3', 'water', null, 0, { cat: 'fluid', drop: null }],
  ['water_4', 'water', null, 0, { cat: 'fluid', drop: null }],
  ['water_5', 'water', null, 0, { cat: 'fluid', drop: null }],
  ['water_6', 'water', null, 0, { cat: 'fluid', drop: null }],
  ['water_7', 'water', null, 0, { cat: 'fluid', drop: null }],
  // variantes de biome : mêmes matériaux, teinte différente
  ['grass_deep', 'cube', ['grass_top', 'grass_side', 'dirt'], 0.5, { drop: 'dirt', cat: 'terrain' }],
  ['oak_leaves_deep', 'leaf', ['leaf_oak', 'leaf_oak', 'leaf_oak'], 0.12, { cat: 'foliage', drop: null, tint: [0.56, 0.68, 0.56] }],
  ['beech_leaves_deep', 'leaf', ['leaf_beech', 'leaf_beech', 'leaf_beech'], 0.12, { cat: 'foliage', drop: null, tint: [0.5, 0.66, 0.54] }],
  ['glowcap', 'deco', null, 0.02, { model: 'glowcap', drop: 'glowcap' }],
  // troncs posés par le joueur : mêmes matériaux, jamais de mousse (bit 4 du sens du fil)
  ['oak_log_c', 'cube', ['end_oak', 'bark_oak', 'end_oak'], 1.0, { cat: 'wood', drop: 'oak_log', clean: true }],
  ['oak_log_x_c', 'cube', ['bark_oak', 'end_oak', 'bark_oak'], 1.0, { cat: 'wood', drop: 'oak_log', axis: 'x', clean: true }],
  ['oak_log_z_c', 'cube', ['bark_oak', 'bark_oak', 'end_oak'], 1.0, { cat: 'wood', drop: 'oak_log', axis: 'z', clean: true }],
  ['beech_log_c', 'cube', ['end_beech', 'bark_beech', 'end_beech'], 1.0, { cat: 'wood', drop: 'beech_log', clean: true }],
  ['beech_log_x_c', 'cube', ['bark_beech', 'end_beech', 'bark_beech'], 1.0, { cat: 'wood', drop: 'beech_log', axis: 'x', clean: true }],
  ['beech_log_z_c', 'cube', ['bark_beech', 'bark_beech', 'end_beech'], 1.0, { cat: 'wood', drop: 'beech_log', axis: 'z', clean: true }],
  // Taïga. Nouveaux blocs toujours en fin de liste : les identifiants déjà sauvegardés ne bougent pas.
  ['snow', 'cube', ['snow', 'snow', 'snow'], 0.4, { cat: 'terrain' }], // bloc de neige plein, sur toutes ses faces
  ['ice', 'cube', ['ice', 'ice', 'ice'], 0.5, { cat: 'terrain' }],
  ['spruce_log', 'cube', ['end_spruce', 'bark_spruce', 'end_spruce'], 1.0, { cat: 'wood' }],
  ['spruce_log_x', 'cube', ['bark_spruce', 'end_spruce', 'bark_spruce'], 1.0, { cat: 'wood', drop: 'spruce_log', axis: 'x' }],
  ['spruce_log_z', 'cube', ['bark_spruce', 'bark_spruce', 'end_spruce'], 1.0, { cat: 'wood', drop: 'spruce_log', axis: 'z' }],
  ['spruce_log_c', 'cube', ['end_spruce', 'bark_spruce', 'end_spruce'], 1.0, { cat: 'wood', drop: 'spruce_log', clean: true }],
  ['spruce_log_x_c', 'cube', ['bark_spruce', 'end_spruce', 'bark_spruce'], 1.0, { cat: 'wood', drop: 'spruce_log', axis: 'x', clean: true }],
  ['spruce_log_z_c', 'cube', ['bark_spruce', 'bark_spruce', 'end_spruce'], 1.0, { cat: 'wood', drop: 'spruce_log', axis: 'z', clean: true }],
  ['spruce_leaves', 'leaf', ['leaf_spruce', 'leaf_spruce', 'leaf_spruce'], 0.12, { cat: 'foliage', drop: null }],
  ['spruce_leaves_snow', 'leaf', ['snow', 'leaf_snow', 'leaf_spruce'], 0.12, { cat: 'foliage', drop: null }],
  ['icicle', 'deco', null, 0.02, { model: 'icicle', drop: null }],
  ['snow_layer', 'deco', null, 0.1, { model: 'snowlayer', drop: 'snow_layer' }],
  ['pinecone', 'deco', null, 0.02, { model: 'pinecone', drop: 'pinecone' }],
  ['frost_grass', 'deco', null, 0.02, { model: 'frost', drop: null }],
  // pellicule de neige plus épaisse : n dixièmes de bloc
  ['snow_layer_2', 'deco', null, 0.1, { model: 'snowlayer', drop: 'snow_layer' }],
  ['snow_layer_3', 'deco', null, 0.1, { model: 'snowlayer', drop: 'snow_layer' }],
  ['snow_layer_4', 'deco', null, 0.1, { model: 'snowlayer', drop: 'snow_layer' }],
  ['snow_layer_5', 'deco', null, 0.1, { model: 'snowlayer', drop: 'snow_layer' }],
  ['snow_layer_6', 'deco', null, 0.1, { model: 'snowlayer', drop: 'snow_layer' }],
  ['snow_layer_7', 'deco', null, 0.1, { model: 'snowlayer', drop: 'snow_layer' }],
  ['snow_layer_8', 'deco', null, 0.1, { model: 'snowlayer', drop: 'snow_layer' }],
  ['snow_layer_9', 'deco', null, 0.1, { model: 'snowlayer', drop: 'snow_layer' }],
  // Steppe sèche
  ['steppe_grass', 'cube', ['grass_dry_top', 'grass_dry_side', 'dirt'], 0.5, { drop: 'dirt', cat: 'terrain' }],
  ['dry_earth', 'cube', ['dry_earth', 'dry_earth', 'dirt'], 0.45, { cat: 'terrain' }],
  ['feather_grass', 'deco', null, 0.02, { model: 'feather' }],
  ['sagebrush', 'deco', null, 0.05, { model: 'sage' }],
  ['dead_bush', 'deco', null, 0.02, { model: 'deadbush' }],
  // Bois mort (arbres secs de la steppe, sapins morts de la taïga), roche à lichens, sous-bois de la taïga
  ['dead_log', 'cube', ['end_dead', 'bark_dead', 'end_dead'], 0.9, { cat: 'wood' }],
  ['dead_log_x', 'cube', ['bark_dead', 'end_dead', 'bark_dead'], 0.9, { cat: 'wood', drop: 'dead_log', axis: 'x' }],
  ['dead_log_z', 'cube', ['bark_dead', 'bark_dead', 'end_dead'], 0.9, { cat: 'wood', drop: 'dead_log', axis: 'z' }],
  ['lichen_stone', 'cube', ['lichen', 'lichen', 'stone'], 1.3, { cat: 'rock' }],
  ['blueberry', 'deco', null, 0.03, { model: 'blueberry' }],
  ['rowan', 'deco', null, 0.06, { model: 'rowan' }],
  ['fern_dry', 'deco', null, 0.02, { model: 'fern_dry' }],
  // Acacias et ossements de la steppe
  ['acacia_log', 'cube', ['end_acacia', 'bark_acacia', 'end_acacia'], 1.0, { cat: 'wood' }],
  ['acacia_log_x', 'cube', ['bark_acacia', 'end_acacia', 'bark_acacia'], 1.0, { cat: 'wood', drop: 'acacia_log', axis: 'x' }],
  ['acacia_log_z', 'cube', ['bark_acacia', 'bark_acacia', 'end_acacia'], 1.0, { cat: 'wood', drop: 'acacia_log', axis: 'z' }],
  ['acacia_leaves', 'leaf', ['leaf_acacia', 'leaf_acacia', 'leaf_acacia'], 0.12, { cat: 'foliage', drop: null, tint: [0.66, 0.66, 0.46] }],
  ['bone', 'cube', ['bone', 'bone', 'bone'], 0.9, { cat: 'rock' }],
  ['birch_leaves_gold', 'leaf', ['leaf_birch', 'leaf_birch', 'leaf_birch'], 0.12, { cat: 'foliage', drop: null, tint: [1.45, 1.05, 0.35] }],
  // Branches fines : une section plus petite qu'un bloc, reliée aux bois et aux feuillages voisins
  ['oak_branch', 'deco', null, 0.4, { model: 'branch', solid: true, cat: 'wood', drop: 'oak_log' }],
  ['birch_branch', 'deco', null, 0.4, { model: 'branch', solid: true, cat: 'wood', drop: 'birch_log' }],
  ['beech_branch', 'deco', null, 0.4, { model: 'branch', solid: true, cat: 'wood', drop: 'beech_log' }],
  ['spruce_branch', 'deco', null, 0.4, { model: 'branch', solid: true, cat: 'wood', drop: 'spruce_log' }],
  ['dead_branch', 'deco', null, 0.4, { model: 'branch', solid: true, cat: 'wood', drop: 'dead_log' }],
  ['acacia_branch', 'deco', null, 0.4, { model: 'branch', solid: true, cat: 'wood', drop: 'acacia_log' }],
  // Troncs creux : un bloc de tronc vidé de son cœur, couché (x, z) ou debout
  ['oak_hollow', 'deco', null, 0.8, { model: 'hollow', solid: true, cat: 'wood' }],
  ['oak_hollow_x', 'deco', null, 0.8, { model: 'hollow', solid: true, cat: 'wood', drop: 'oak_hollow' }],
  ['oak_hollow_z', 'deco', null, 0.8, { model: 'hollow', solid: true, cat: 'wood', drop: 'oak_hollow' }],
  ['spruce_hollow', 'deco', null, 0.8, { model: 'hollow', solid: true, cat: 'wood' }],
  ['spruce_hollow_x', 'deco', null, 0.8, { model: 'hollow', solid: true, cat: 'wood', drop: 'spruce_hollow' }],
  ['spruce_hollow_z', 'deco', null, 0.8, { model: 'hollow', solid: true, cat: 'wood', drop: 'spruce_hollow' }],
  ['dead_hollow', 'deco', null, 0.8, { model: 'hollow', solid: true, cat: 'wood' }],
  ['dead_hollow_x', 'deco', null, 0.8, { model: 'hollow', solid: true, cat: 'wood', drop: 'dead_hollow' }],
  ['dead_hollow_z', 'deco', null, 0.8, { model: 'hollow', solid: true, cat: 'wood', drop: 'dead_hollow' }],
  // Champignons lumineux : une couleur par bloc (le premier, 'glowcap', est le vert)
  ['glowcap_blue', 'deco', null, 0.02, { model: 'glowcap', drop: 'glowcap_blue' }],
  ['glowcap_pink', 'deco', null, 0.02, { model: 'glowcap', drop: 'glowcap_pink' }],
  ['glowcap_violet', 'deco', null, 0.02, { model: 'glowcap', drop: 'glowcap_violet' }],
  ['glowcap_cyan', 'deco', null, 0.02, { model: 'glowcap', drop: 'glowcap_cyan' }],
  // Désert
  ['sandstone', 'cube', ['sandstone_top', 'sandstone', 'sandstone_top'], 1.0, { cat: 'rock' }],
  ['red_sandstone', 'cube', ['red_sandstone', 'red_sandstone', 'red_sandstone'], 1.0, { cat: 'rock' }],
  ['cactus', 'deco', null, 0.5, { model: 'cactus', solid: true, cat: 'foliage', drop: 'cactus' }], // un fût côtelé plus fin qu'un bloc, relié à ses voisins
  ['palm_log', 'cube', ['end_palm', 'bark_palm', 'end_palm'], 0.9, { cat: 'wood' }],
  ['palm_leaves', 'leaf', ['leaf_palm', 'leaf_palm', 'leaf_palm'], 0.12, { cat: 'foliage', drop: null, tint: [0.8, 0.86, 0.6] }],
  // Hautes terres
  ['moor_grass', 'cube', ['grass_top', 'grass_side', 'dirt'], 0.5, { drop: 'dirt', cat: 'terrain' }],
  ['heather', 'deco', null, 0.04, { model: 'heather' }],
  ['cotton_grass', 'deco', null, 0.02, { model: 'cotton' }],
  // Survie : stations de fabrication
  ['campfire', 'deco', null, 0.4, { model: 'campfire', light: 13, cat: 'light', drop: 'campfire' }], // cuit la viande, brûle qui marche dedans
  ['workbench', 'deco', null, 0.8, { model: 'workbench', solid: true, cat: 'wood', drop: 'workbench' }],
  // Survie : ce qui traîne au sol et se ramasse à la main ; buissons cueillis, qui repoussent en un jour
  ['twigs', 'deco', null, 0.05, { model: 'twigs', drop: 'stick' }],
  ['pebbles', 'deco', null, 0.05, { model: 'pebbles', drop: 'pebble' }],
  ['blueberry_picked', 'deco', null, 0.03, { model: 'blueberry', drop: 'blueberry' }],
  ['rowan_picked', 'deco', null, 0.06, { model: 'rowan', drop: 'rowan' }],
  // Sous terre : minerais en filons, charbon ; argile au bord de l'eau
  ['coal_ore', 'cube', ['ore_coal', 'ore_coal', 'ore_coal'], 1.6, { cat: 'rock', drop: 'coal' }],
  ['copper_ore', 'cube', ['ore_copper', 'ore_copper', 'ore_copper'], 2.0, { cat: 'rock' }],
  ['tin_ore', 'cube', ['ore_tin', 'ore_tin', 'ore_tin'], 2.0, { cat: 'rock' }],
  ['iron_ore', 'cube', ['ore_iron', 'ore_iron', 'ore_iron'], 2.4, { cat: 'rock' }],
  ['clay', 'cube', ['clay', 'clay', 'clay'], 0.55, { cat: 'terrain' }],
  // Construction
  ['stone_bricks', 'cube', ['stone_bricks', 'stone_bricks', 'stone_bricks'], 1.6, { cat: 'rock' }],
  ['clay_bricks', 'cube', ['bricks', 'bricks', 'bricks'], 1.5, { cat: 'rock' }],
  ['cobblestone', 'cube', ['cobble', 'cobble', 'cobble'], 1.5, { cat: 'rock' }],
  ['thatch', 'cube', ['thatch', 'thatch', 'thatch'], 0.35, { cat: 'foliage' }],
  ['birch_planks', 'cube', ['planks_birch', 'planks_birch', 'planks_birch'], 0.7, { cat: 'wood' }],
  ['spruce_planks', 'cube', ['planks_spruce', 'planks_spruce', 'planks_spruce'], 0.8, { cat: 'wood' }],
  ['farmland', 'cube', ['farmland_top', 'dirt', 'dirt'], 0.45, { cat: 'terrain', drop: 'dirt' }],
  // Ateliers, rangement, repos, lumière
  ['furnace', 'deco', null, 1.4, { model: 'furnace', solid: true, light: 7, cat: 'rock' }],
  ['anvil', 'deco', null, 2.0, { model: 'anvil', solid: true, cat: 'rock' }],
  ['mason_bench', 'deco', null, 1.0, { model: 'mason_bench', solid: true, cat: 'wood' }],
  ['chest', 'deco', null, 0.8, { model: 'chest', solid: true, cat: 'wood' }],
  ['bedroll', 'deco', null, 0.2, { model: 'bedroll' }],
  ['bed', 'deco', null, 0.8, { model: 'bed', solid: true, cat: 'wood' }],
  ['torch', 'deco', null, 0.05, { model: 'torch', light: 13, cat: 'light' }],
  ['campfire_out', 'deco', null, 0.4, { model: 'campfire', cat: 'light', drop: 'campfire' }], // feu éteint : il faut le recharger
  // Blé cultivé : quatre stades de croissance
  ['wheat_0', 'deco', null, 0.02, { model: 'wheat', drop: 'seeds' }],
  ['wheat_1', 'deco', null, 0.02, { model: 'wheat', drop: 'seeds' }],
  ['wheat_2', 'deco', null, 0.02, { model: 'wheat', drop: 'seeds' }],
  ['wheat_3', 'deco', null, 0.02, { model: 'wheat', drop: 'wheat' }],
  // Construction : formes orientées. Le nom porte l'orientation (_1 à _3 : un quart de tour de plus), la moitié haute (_top),
  // la porte ouverte et sa moitié haute (door_<sens><ouverte><haut>). La variante de base est l'objet que l'on pose.
  ['planks_slab', 'deco', null, 0.48, { model: 'slab', solid: true, cat: 'wood', drop: 'planks_slab' }],
  ['planks_slab_top', 'deco', null, 0.48, { model: 'slab', solid: true, cat: 'wood', drop: 'planks_slab' }],
  ['stone_bricks_slab', 'deco', null, 0.96, { model: 'slab', solid: true, cat: 'rock', drop: 'stone_bricks_slab' }],
  ['stone_bricks_slab_top', 'deco', null, 0.96, { model: 'slab', solid: true, cat: 'rock', drop: 'stone_bricks_slab' }],
  ['cobblestone_slab', 'deco', null, 0.9, { model: 'slab', solid: true, cat: 'rock', drop: 'cobblestone_slab' }],
  ['cobblestone_slab_top', 'deco', null, 0.9, { model: 'slab', solid: true, cat: 'rock', drop: 'cobblestone_slab' }],
  ['planks_stairs', 'deco', null, 0.8, { model: 'stairs', solid: true, cat: 'wood', drop: 'planks_stairs' }],
  ['planks_stairs_1', 'deco', null, 0.8, { model: 'stairs', solid: true, cat: 'wood', drop: 'planks_stairs' }],
  ['planks_stairs_2', 'deco', null, 0.8, { model: 'stairs', solid: true, cat: 'wood', drop: 'planks_stairs' }],
  ['planks_stairs_3', 'deco', null, 0.8, { model: 'stairs', solid: true, cat: 'wood', drop: 'planks_stairs' }],
  ['stone_bricks_stairs', 'deco', null, 1.6, { model: 'stairs', solid: true, cat: 'rock', drop: 'stone_bricks_stairs' }],
  ['stone_bricks_stairs_1', 'deco', null, 1.6, { model: 'stairs', solid: true, cat: 'rock', drop: 'stone_bricks_stairs' }],
  ['stone_bricks_stairs_2', 'deco', null, 1.6, { model: 'stairs', solid: true, cat: 'rock', drop: 'stone_bricks_stairs' }],
  ['stone_bricks_stairs_3', 'deco', null, 1.6, { model: 'stairs', solid: true, cat: 'rock', drop: 'stone_bricks_stairs' }],
  ['cobblestone_stairs', 'deco', null, 1.5, { model: 'stairs', solid: true, cat: 'rock', drop: 'cobblestone_stairs' }],
  ['cobblestone_stairs_1', 'deco', null, 1.5, { model: 'stairs', solid: true, cat: 'rock', drop: 'cobblestone_stairs' }],
  ['cobblestone_stairs_2', 'deco', null, 1.5, { model: 'stairs', solid: true, cat: 'rock', drop: 'cobblestone_stairs' }],
  ['cobblestone_stairs_3', 'deco', null, 1.5, { model: 'stairs', solid: true, cat: 'rock', drop: 'cobblestone_stairs' }],
  ['thatch_stairs', 'deco', null, 0.35, { model: 'stairs', solid: true, cat: 'foliage', drop: 'thatch_stairs' }],
  ['thatch_stairs_1', 'deco', null, 0.35, { model: 'stairs', solid: true, cat: 'foliage', drop: 'thatch_stairs' }],
  ['thatch_stairs_2', 'deco', null, 0.35, { model: 'stairs', solid: true, cat: 'foliage', drop: 'thatch_stairs' }],
  ['thatch_stairs_3', 'deco', null, 0.35, { model: 'stairs', solid: true, cat: 'foliage', drop: 'thatch_stairs' }],
  ['door', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: 'door' }],
  ['door_001', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: null }],
  ['door_010', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: 'door' }],
  ['door_011', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: null }],
  ['door_100', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: 'door' }],
  ['door_101', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: null }],
  ['door_110', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: 'door' }],
  ['door_111', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: null }],
  ['door_200', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: 'door' }],
  ['door_201', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: null }],
  ['door_210', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: 'door' }],
  ['door_211', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: null }],
  ['door_300', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: 'door' }],
  ['door_301', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: null }],
  ['door_310', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: 'door' }],
  ['door_311', 'deco', null, 0.7, { model: 'door', solid: true, cat: 'wood', drop: null }],
  ['trapdoor', 'deco', null, 0.6, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'trapdoor' }],
  ['trapdoor_open', 'deco', null, 0.6, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'trapdoor' }],
  ['trapdoor_open_1', 'deco', null, 0.6, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'trapdoor' }],
  ['trapdoor_open_2', 'deco', null, 0.6, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'trapdoor' }],
  ['trapdoor_open_3', 'deco', null, 0.6, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'trapdoor' }],
  ['ladder', 'deco', null, 0.4, { model: 'ladder', cat: 'wood', drop: 'ladder' }],
  ['ladder_1', 'deco', null, 0.4, { model: 'ladder', cat: 'wood', drop: 'ladder' }],
  ['ladder_2', 'deco', null, 0.4, { model: 'ladder', cat: 'wood', drop: 'ladder' }],
  ['ladder_3', 'deco', null, 0.4, { model: 'ladder', cat: 'wood', drop: 'ladder' }],
  ['window', 'deco', null, 0.5, { model: 'window', solid: true, cat: 'wood', drop: 'window' }],
  ['window_x', 'deco', null, 0.5, { model: 'window', solid: true, cat: 'wood', drop: 'window' }],
  ['wood_pillar', 'deco', null, 0.9, { model: 'pillar', solid: true, cat: 'wood', drop: 'wood_pillar' }],
  ['stone_pillar', 'deco', null, 1.6, { model: 'pillar', solid: true, cat: 'rock', drop: 'stone_pillar' }],
  ['stone_wall', 'deco', null, 1.6, { model: 'wall', solid: true, cat: 'rock', drop: 'stone_wall' }],
  ['stone_wall_x', 'deco', null, 1.6, { model: 'wall', solid: true, cat: 'rock', drop: 'stone_wall' }],
  // Cultures : carottes et pommes de terre, quatre stades comme le blé ; on les trouve aussi à l'état sauvage dans les prairies.
  ['carrot_0', 'deco', null, 0.02, { model: 'carrot', drop: 'carrot' }],
  ['carrot_1', 'deco', null, 0.02, { model: 'carrot', drop: 'carrot' }],
  ['carrot_2', 'deco', null, 0.02, { model: 'carrot', drop: 'carrot' }],
  ['carrot_3', 'deco', null, 0.02, { model: 'carrot', drop: 'carrot' }],
  ['potato_0', 'deco', null, 0.02, { model: 'potato', drop: 'potato' }],
  ['potato_1', 'deco', null, 0.02, { model: 'potato', drop: 'potato' }],
  ['potato_2', 'deco', null, 0.02, { model: 'potato', drop: 'potato' }],
  ['potato_3', 'deco', null, 0.02, { model: 'potato', drop: 'potato' }],
  // Escaliers renversés (marche au plafond), dalles verticales, volets : ajoutés en fin de liste, les identifiants ne bougent pas.
  ['planks_stairs_top', 'deco', null, 0.8, { model: 'stairs', solid: true, cat: 'wood', drop: 'planks_stairs' }],
  ['planks_stairs_top_1', 'deco', null, 0.8, { model: 'stairs', solid: true, cat: 'wood', drop: 'planks_stairs' }],
  ['planks_stairs_top_2', 'deco', null, 0.8, { model: 'stairs', solid: true, cat: 'wood', drop: 'planks_stairs' }],
  ['planks_stairs_top_3', 'deco', null, 0.8, { model: 'stairs', solid: true, cat: 'wood', drop: 'planks_stairs' }],
  ['stone_bricks_stairs_top', 'deco', null, 1.6, { model: 'stairs', solid: true, cat: 'rock', drop: 'stone_bricks_stairs' }],
  ['stone_bricks_stairs_top_1', 'deco', null, 1.6, { model: 'stairs', solid: true, cat: 'rock', drop: 'stone_bricks_stairs' }],
  ['stone_bricks_stairs_top_2', 'deco', null, 1.6, { model: 'stairs', solid: true, cat: 'rock', drop: 'stone_bricks_stairs' }],
  ['stone_bricks_stairs_top_3', 'deco', null, 1.6, { model: 'stairs', solid: true, cat: 'rock', drop: 'stone_bricks_stairs' }],
  ['cobblestone_stairs_top', 'deco', null, 1.5, { model: 'stairs', solid: true, cat: 'rock', drop: 'cobblestone_stairs' }],
  ['cobblestone_stairs_top_1', 'deco', null, 1.5, { model: 'stairs', solid: true, cat: 'rock', drop: 'cobblestone_stairs' }],
  ['cobblestone_stairs_top_2', 'deco', null, 1.5, { model: 'stairs', solid: true, cat: 'rock', drop: 'cobblestone_stairs' }],
  ['cobblestone_stairs_top_3', 'deco', null, 1.5, { model: 'stairs', solid: true, cat: 'rock', drop: 'cobblestone_stairs' }],
  ['thatch_stairs_top', 'deco', null, 0.35, { model: 'stairs', solid: true, cat: 'foliage', drop: 'thatch_stairs' }],
  ['thatch_stairs_top_1', 'deco', null, 0.35, { model: 'stairs', solid: true, cat: 'foliage', drop: 'thatch_stairs' }],
  ['thatch_stairs_top_2', 'deco', null, 0.35, { model: 'stairs', solid: true, cat: 'foliage', drop: 'thatch_stairs' }],
  ['thatch_stairs_top_3', 'deco', null, 0.35, { model: 'stairs', solid: true, cat: 'foliage', drop: 'thatch_stairs' }],
  ['planks_vslab', 'deco', null, 0.48, { model: 'slab', solid: true, cat: 'wood', drop: 'planks_vslab' }],
  ['planks_vslab_1', 'deco', null, 0.48, { model: 'slab', solid: true, cat: 'wood', drop: 'planks_vslab' }],
  ['planks_vslab_2', 'deco', null, 0.48, { model: 'slab', solid: true, cat: 'wood', drop: 'planks_vslab' }],
  ['planks_vslab_3', 'deco', null, 0.48, { model: 'slab', solid: true, cat: 'wood', drop: 'planks_vslab' }],
  ['stone_bricks_vslab', 'deco', null, 0.96, { model: 'slab', solid: true, cat: 'rock', drop: 'stone_bricks_vslab' }],
  ['stone_bricks_vslab_1', 'deco', null, 0.96, { model: 'slab', solid: true, cat: 'rock', drop: 'stone_bricks_vslab' }],
  ['stone_bricks_vslab_2', 'deco', null, 0.96, { model: 'slab', solid: true, cat: 'rock', drop: 'stone_bricks_vslab' }],
  ['stone_bricks_vslab_3', 'deco', null, 0.96, { model: 'slab', solid: true, cat: 'rock', drop: 'stone_bricks_vslab' }],
  ['cobblestone_vslab', 'deco', null, 0.9, { model: 'slab', solid: true, cat: 'rock', drop: 'cobblestone_vslab' }],
  ['cobblestone_vslab_1', 'deco', null, 0.9, { model: 'slab', solid: true, cat: 'rock', drop: 'cobblestone_vslab' }],
  ['cobblestone_vslab_2', 'deco', null, 0.9, { model: 'slab', solid: true, cat: 'rock', drop: 'cobblestone_vslab' }],
  ['cobblestone_vslab_3', 'deco', null, 0.9, { model: 'slab', solid: true, cat: 'rock', drop: 'cobblestone_vslab' }],
  ['shutter', 'deco', null, 0.5, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'shutter' }],
  ['shutter_1', 'deco', null, 0.5, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'shutter' }],
  ['shutter_2', 'deco', null, 0.5, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'shutter' }],
  ['shutter_3', 'deco', null, 0.5, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'shutter' }],
  ['shutter_open', 'deco', null, 0.5, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'shutter' }],
  ['shutter_open_1', 'deco', null, 0.5, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'shutter' }],
  ['shutter_open_2', 'deco', null, 0.5, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'shutter' }],
  ['shutter_open_3', 'deco', null, 0.5, { model: 'trapdoor', solid: true, cat: 'wood', drop: 'shutter' }],
  // Village médiéval. Les identifiants s'arrêtent à 255 : ce sont les derniers.
  ['plaster', 'cube', ['plaster', 'plaster', 'plaster'], 0.7, { cat: 'rock' }],
  ['timber', 'cube', ['dark_timber', 'dark_timber', 'dark_timber'], 0.9, { cat: 'wood' }],
  ['roof_tiles', 'cube', ['roof_tiles', 'roof_tiles', 'roof_tiles'], 0.9, { cat: 'rock' }],
  ['slate', 'cube', ['slate', 'slate', 'slate'], 1.1, { cat: 'rock' }],
  ['barrel', 'deco', null, 0.8, { model: 'barrel', solid: true, cat: 'wood' }],
];

// ---------------------------------------------------------------- richesse : identifiants 256 et au-delà (16 bits)
const cube = (n, m, h, cat, p = {}) => defs.push([n, 'cube', Array.isArray(m) ? m : [m, m, m], h, { cat, ...p }]);
// Escaliers (quatre sens, haut ou bas), dalles (basse, haute), dalles verticales (quatre sens) d'une matière.
const forms = (n, h, cat) => {
  for (const top of ['', '_top']) for (const f of ['', '_1', '_2', '_3']) defs.push([`${n}_stairs${top}${f}`, 'deco', null, h, { model: 'stairs', solid: true, cat, drop: `${n}_stairs` }]);
  defs.push([`${n}_slab`, 'deco', null, h * 0.6, { model: 'slab', solid: true, cat, drop: `${n}_slab` }], [`${n}_slab_top`, 'deco', null, h * 0.6, { model: 'slab', solid: true, cat, drop: `${n}_slab` }]);
  for (const f of ['', '_1', '_2', '_3']) defs.push([`${n}_vslab${f}`, 'deco', null, h * 0.6, { model: 'slab', solid: true, cat, drop: `${n}_vslab` }]);
};
// Couverture de toit : la pente à 45° (quatre sens, posée ou renversée), l'arêtier (angle sortant des croupes et des
// pyramides), la noue (angle rentrant où deux toits se croisent), le faîtage en Λ (deux sens) et l'épi qui coiffe une pyramide.
const slopes = (n, h, cat) => {
  const add = (k, d) => defs.push([k, 'deco', null, h, { model: 'slope', solid: true, cat, drop: d }]);
  for (const top of ['', '_top']) for (const f of ['', '_1', '_2', '_3']) add(`${n}_slope${top}${f}`, `${n}_slope`);
  for (const k of ['corner', 'valley']) for (const f of ['', '_1', '_2', '_3']) add(`${n}_${k}${f}`, `${n}_${k}`);
  for (const f of ['', '_1']) add(`${n}_ridge${f}`, `${n}_ridge`);
  add(`${n}_peak`, `${n}_peak`);
};
// Meuble orientable : quatre sens, « nom », « nom@1 » à « nom@3 ».
const turner = (n, h, cat, p = {}) => { for (const f of ['', '@1', '@2', '@3']) defs.push([`${n}${f}`, 'deco', null, h, { model: n, cat, drop: n, ...p }]); };
const deco = (n, h, cat, p = {}) => defs.push([n, 'deco', null, h, { model: n, cat, ...p }]);

// Pierres taillées, sols, toits.
for (const [n, m, h] of [['marble', 'marble', 1.8], ['ashlar', 'ashlar', 1.6], ['fieldstone', 'fieldstone', 1.5], ['setts', 'setts', 1.5], ['polished_stone', 'polished_stone', 1.6], ['shingles', 'shingles', 0.8], ['copper_roof', 'verdigris', 1.4]]) cube(n, m, h, n === 'shingles' ? 'wood' : 'rock');
cube('checker_floor', 'checker', 1.6, 'rock'); cube('terracotta_floor', 'terracotta', 1.0, 'rock'); cube('parquet', 'parquet', 0.8, 'wood');
cube('chiseled_stone', 'chiseled', 1.6, 'rock'); cube('stained_glass', 'stained_glass', 0.5, 'rock'); cube('iron_block', 'iron_plate', 2.5, 'rock');
cube('marble_black', 'marble', 1.8, 'rock', { tint: [0.3, 0.3, 0.32] }); cube('bricks_dark', 'bricks', 1.5, 'rock', { tint: [0.55, 0.5, 0.48] }); cube('stone_bricks_dark', 'stone_bricks', 1.6, 'rock', { tint: [0.62, 0.62, 0.66] });
// Pans de bois et matériaux de maison.
cube('timber_frame', 'frame_square', 0.9, 'wood'); cube('timber_cross', 'frame_cross', 0.9, 'wood'); cube('timber_brace', 'frame_diag', 0.9, 'wood'); cube('wattle', 'wattle', 0.5, 'wood');
cube('bookshelf', ['planks', 'books', 'planks'], 0.8, 'wood'); cube('hay_bale', 'hay', 0.4, 'foliage'); cube('crate', 'crate', 0.7, 'wood');
// Refonte des villages : les matériaux des planches architecturales.
cube('limestone_masonry', 'lime_masonry', 1.6, 'rock'); cube('rubble_wall', 'rubble', 1.6, 'rock'); cube('barn_boards', 'boards', 0.8, 'wood'); cube('mossy_roof_tiles', 'tiles_moss', 0.9, 'rock');
cube('dirt_path', ['path_top', 'dirt', 'dirt'], 0.5, 'terrain', { drop: 'dirt' });
defs.push(['oak_beam', 'cube', ['beam_end', 'dark_timber', 'beam_end'], 1.0, { cat: 'wood' }], ['oak_beam_x', 'cube', ['dark_timber', 'beam_end', 'dark_timber'], 1.0, { cat: 'wood', drop: 'oak_beam', axis: 'x' }], ['oak_beam_z', 'cube', ['dark_timber', 'dark_timber', 'beam_end'], 1.0, { cat: 'wood', drop: 'oak_beam', axis: 'z' }]);
// Bois peints et enduits colorés, laines teintes : une texture, une teinte.
for (const [n, t] of [['planks_dark', [0.55, 0.45, 0.4]], ['planks_red', [0.95, 0.5, 0.42]], ['planks_white', [1.45, 1.4, 1.3]], ['planks_blue', [0.6, 0.78, 1.0]], ['planks_green', [0.62, 0.85, 0.58]]]) cube(n, 'planks', 0.8, 'wood', { tint: t });
for (const [n, t] of [['plaster_ochre', [1.0, 0.82, 0.55]], ['plaster_rose', [1.0, 0.8, 0.76]], ['plaster_blue', [0.82, 0.9, 1.02]]]) cube(n, 'plaster', 0.7, 'rock', { tint: t });
for (const [n, t] of [['white', [1, 1, 1]], ['red', [0.75, 0.2, 0.18]], ['blue', [0.28, 0.4, 0.8]], ['green', [0.35, 0.6, 0.3]], ['yellow', [1.0, 0.85, 0.3]], ['black', [0.22, 0.22, 0.24]], ['brown', [0.55, 0.38, 0.25]], ['purple', [0.55, 0.3, 0.65]], ['orange', [1.0, 0.55, 0.22]]]) cube(`wool_${n}`, 'wool', 0.3, 'foliage', { tint: t });
// Formes (escaliers, dalles, dalles verticales) des matières qui en ont besoin.
for (const [n, h, c] of [['marble', 1.8, 'rock'], ['ashlar', 1.6, 'rock'], ['fieldstone', 1.5, 'rock'], ['setts', 1.5, 'rock'], ['polished_stone', 1.6, 'rock'], ['shingles', 0.8, 'wood'], ['roof_tiles', 0.9, 'rock'], ['slate', 1.1, 'rock'], ['copper_roof', 1.4, 'rock'], ['plaster', 0.7, 'rock'], ['timber', 0.9, 'wood'], ['limestone_masonry', 1.6, 'rock'], ['rubble_wall', 1.6, 'rock'], ['mossy_roof_tiles', 0.9, 'rock'], ['barn_boards', 0.8, 'wood']]) forms(n, h, c);
for (const [n, h, c] of [['roof_tiles', 0.9, 'rock'], ['slate', 1.1, 'rock'], ['mossy_roof_tiles', 0.9, 'rock'], ['shingles', 0.8, 'wood'], ['thatch', 0.35, 'foliage'], ['copper_roof', 1.4, 'rock'], ['planks', 0.8, 'wood']]) slopes(n, h, c);
// Mobilier : orientable vers celui qui le pose.
for (const [n, h, p] of [['chair', 0.5, { solid: true }], ['bench', 0.6, { solid: true }], ['armchair', 0.6, { solid: true }], ['throne', 1.2, { solid: true }], ['cupboard', 0.9, { solid: true }],
  ['fireplace', 1.6, { solid: true, light: 13 }], ['loom', 0.8, { solid: true }], ['spinning_wheel', 0.6, { solid: true }], ['grindstone', 1.2, { solid: true }], ['signpost', 0.5, { solid: true }],
  ['painting', 0.3, {}], ['banner_red', 0.2, {}], ['banner_blue', 0.2, {}], ['banner_green', 0.2, {}], ['banner_gold', 0.2, {}], ['weapon_rack', 0.6, {}], ['wall_sconce', 0.3, { light: 11 }],
  ['curtain', 0.2, {}], ['wall_shelf', 0.4, {}]]) turner(n, h, 'wood', p);
// Objets posés.
for (const [n, h, p] of [['table', 0.8, { solid: true }], ['round_table', 0.8, { solid: true }], ['stool', 0.4, {}], ['candle', 0.1, { light: 9 }], ['candelabra', 0.4, { light: 12 }],
  ['chandelier', 0.5, { light: 14 }], ['brazier', 0.8, { light: 14, solid: true }], ['sack', 0.3, {}], ['amphora', 0.5, {}], ['flower_pot', 0.3, {}], ['potted_fern', 0.3, {}],
  ['cauldron', 1.2, { solid: true }], ['bucket', 0.3, {}], ['log_pile', 0.8, { solid: true }], ['carpet_red', 0.1, {}], ['carpet_blue', 0.1, {}], ['carpet_green', 0.1, {}],
  ['iron_bars', 1.5, { solid: true }], ['books_pile', 0.2, {}], ['bottles', 0.2, {}], ['beehive', 0.5, {}], ['cheese', 0.2, {}], ['bread_basket', 0.2, {}]]) deco(n, h, 'wood', p);
// Les blocs ajoutés ensuite vont toujours en fin de liste : les numéros des blocs déjà sauvegardés ne bougent pas.
// Torche fixée au mur, penchée hors du mur : quatre sens.
turner('torch_wall', 0.05, 'light', { light: 13, drop: 'torch' });
// Le lit fait deux blocs de long : la tête (le bloc « bed » d'origine, désormais orientable) et le pied.
for (const f of ['@1', '@2', '@3']) defs.push([`bed${f}`, 'deco', null, 0.8, { model: 'bed', solid: true, cat: 'wood', drop: 'bed' }]);
turner('bed_foot', 0.8, 'wood', { solid: true, drop: 'bed' });
// Le coffre ouvert (couvercle levé, le temps qu'on y fouille) et le four allumé (pendant une fonte, puis il refroidit).
defs.push(['chest_open', 'deco', null, 0.8, { model: 'chest_open', solid: true, cat: 'wood', drop: 'chest' }]);
defs.push(['furnace_lit', 'deco', null, 1.4, { model: 'furnace', solid: true, light: 14, cat: 'rock', drop: 'furnace' }]);
// Fleurs d'altitude : l'edelweiss laineux des rocailles et la gentiane bleue des alpages.
defs.push(['edelweiss', 'deco', null, 0.02, { model: 'edelweiss', drop: null }]);
defs.push(['gentian', 'deco', null, 0.02, { model: 'gentian', drop: null }]);
// Mer : plantes immergées (l'eau remplit leur case), coquillages et oyats sur les plages.
defs.push(['seagrass', 'deco', null, 0.02, { model: 'seagrass', drop: null }]);
defs.push(['kelp', 'deco', null, 0.02, { model: 'kelp', drop: null }]);
defs.push(['kelp_top', 'deco', null, 0.02, { model: 'kelp_top', drop: null }]);
defs.push(['coral', 'deco', null, 0.3, { model: 'coral', drop: null }]);
defs.push(['coral_fan', 'deco', null, 0.2, { model: 'coral_fan', drop: null }]);
defs.push(['shell', 'deco', null, 0.05, { model: 'shell' }]);
defs.push(['marram', 'deco', null, 0.02, { model: 'marram', drop: null }]);
// Grands coraux : des blocs pleins de cinq couleurs, sur une même matière poreuse ; bois d'épave.
for (const [n, t] of [['coral_block_red', [1.06, 0.44, 0.4]], ['coral_block_orange', [1.1, 0.62, 0.26]], ['coral_block_purple', [0.78, 0.4, 0.94]], ['coral_block_yellow', [1.1, 0.94, 0.42]], ['coral_block_blue', [0.4, 0.8, 0.98]]]) cube(n, 'coral', 0.9, 'rock', { tint: t });
cube('wreck_planks', 'planks_wreck', 0.8, 'wood');
// Patrons médiévaux : cloche, chaîne, horloge, croix, drapeau, cuves, peaux, chevalet de sciage, billot.
for (const [n, h, p] of [['wall_clock', 0.4, {}], ['hides', 0.3, {}], ['sawhorse', 0.6, { solid: true }]]) turner(n, h, 'wood', p);
defs.push(['bell', 'deco', null, 0.8, { model: 'bell', cat: 'rock' }], ['chain', 'deco', null, 0.5, { model: 'chain', cat: 'rock' }], ['cross', 'deco', null, 0.6, { model: 'cross', cat: 'rock' }], ['flag', 'deco', null, 0.3, { model: 'flag', cat: 'wood' }],
  ['tanning_vat', 'deco', null, 1.5, { model: 'tanning_vat', solid: true, cat: 'rock' }], ['lime_vat', 'deco', null, 1.5, { model: 'lime_vat', solid: true, cat: 'rock' }], ['chopping_block', 'deco', null, 0.8, { model: 'chopping_block', solid: true, cat: 'wood' }]);
// Refonte des villages : charrette, abreuvoir, râtelier d'outils des champs (orientables), tas de charbon de bois.
for (const [n, h, p] of [['cart', 0.8, { solid: true }], ['trough', 0.6, { solid: true }], ['tool_rack', 0.4, {}]]) turner(n, h, 'wood', p);
defs.push(['coal_heap', 'deco', null, 0.6, { model: 'coal_heap', cat: 'rock', drop: 'charcoal' }]);
// Port de pêche : filets, séchoirs, nasses, bittes, bouées, ancre, avirons, garde-corps de corde, voiles, fanion.
for (const [n, h, p] of [['fishing_net', 0.2, {}], ['fish_rack', 0.5, {}], ['anchor', 1.5, {}], ['oars', 0.4, {}], ['rope_rail', 0.5, { solid: true }], ['sail', 0.2, {}], ['boat_bench', 0.4, {}]]) turner(n, h, 'wood', p);
for (const [n, h, p] of [['creel', 0.3, {}], ['mooring_post', 1.0, { solid: true }], ['buoy', 0.3, {}], ['net_pile', 0.2, {}], ['pennant', 0.3, {}], ['fish_crate', 0.5, {}]]) defs.push([n, 'deco', null, h, { model: n, cat: 'wood', ...p }]);

export const BLOCKS = defs.map(([name, shape, mats, hardness, p], id) => ({
  id, name, shape, hardness,
  faces: mats ? mats.map(matLayer) : null,
  // Matériau par direction [X, Y+, Y-, Z], et sens du fil : 0 libre, 1 tourné d'un quart, 2 fixe.
  dirFaces: !mats ? null : p.axis === 'x' || p.axis === 'z' ? [mats[1], mats[0], mats[0], mats[2]].map(matLayer) : [mats[1], mats[0], mats[2], mats[1]].map(matLayer),
  grain: (p.axis === 'x' ? [0, 1, 1, 1] : p.axis === 'z' ? [1, 2, 2, 0] : p.axis === 'pz' ? [0, 1, 1, 0] : p.axis === 'py' ? [1, 0, 0, 1] : name.endsWith('planks') ? [0, 2, 2, 0] : [0, 0, 0, 0]).map((g) => g + (p.clean ? 4 : 0)),
  solid: shape === 'cube' || shape === 'leaf' || !!p.solid,
  opaque: shape === 'cube',
  light: p.light || 0,
  tint: p.tint || [1, 1, 1],
  model: p.model || null,
  cat: p.cat || 'decor',
  drop: p.drop === undefined ? name : p.drop,
}));
export const B = Object.fromEntries(BLOCKS.map((b) => [b.name, b.id]));

// Tables plates, pour les boucles critiques.
export const IS_SOLID = Uint8Array.from(BLOCKS.map((b) => (b.solid ? 1 : 0)));
export const IS_OPAQUE = Uint8Array.from(BLOCKS.map((b) => (b.opaque ? 1 : 0)));
export const IS_LEAF = Uint8Array.from(BLOCKS.map((b) => (b.shape === 'leaf' ? 1 : 0)));
export const IS_DECO = Uint8Array.from(BLOCKS.map((b) => (b.shape === 'deco' ? 1 : 0)));
export const EMIT = Uint8Array.from(BLOCKS.map((b) => b.light));

// Orientation : les bois suivent la direction donnée, verticale dès qu'elle s'écarte assez de l'horizon.
// Blocs orientables, par axe x, y, z.
const ORIENT = { oak_hollow: ['oak_hollow_x', 'oak_hollow', 'oak_hollow_z'], spruce_hollow: ['spruce_hollow_x', 'spruce_hollow', 'spruce_hollow_z'], dead_hollow: ['dead_hollow_x', 'dead_hollow', 'dead_hollow_z'], acacia_log: ['acacia_log_x', 'acacia_log', 'acacia_log_z'], dead_log: ['dead_log_x', 'dead_log', 'dead_log_z'], spruce_log: ['spruce_log_x', 'spruce_log', 'spruce_log_z'], oak_log: ['oak_log_x', 'oak_log', 'oak_log_z'], birch_log: ['birch_log_x', 'birch_log', 'birch_log_z'], beech_log: ['beech_log_x', 'beech_log', 'beech_log_z'], planks: ['planks', 'planks_y', 'planks_z'] };
export const canOrient = (id) => !!ORIENT[BLOCKS[id].name];
export const axisOf = (id) => { const n = BLOCKS[id].name; for (const k in ORIENT) { const i = ORIENT[k].indexOf(n); if (i >= 0) return 'xyz'[i]; } return null; };
export const MODES = ['selon le regard', 'debout', 'dans le sens du regard', 'en travers du regard'];
// Axe du bloc selon le regard (vecteur unitaire). `prev` évite de basculer sans arrêt quand on vise juste entre deux directions.
export function gazeAxis(dx, dy, dz, mode = 0, prev = null) {
  const ax = Math.abs(dx), az = Math.abs(dz);
  let h = ax >= az ? 'x' : 'z';
  if (prev === 'x' && ax * 1.25 > az) h = 'x'; else if (prev === 'z' && az * 1.25 > ax) h = 'z';
  if (mode === 1) return 'y';
  if (mode === 2) return h;
  if (mode === 3) return h === 'x' ? 'z' : 'x';
  return Math.abs(dy) > (prev === 'y' ? 0.74 : 0.84) ? 'y' : h; // au-delà d'environ 57° vers le haut ou le bas, le bloc se dresse
}
export function orientedBlock(id, axis) { const v = ORIENT[BLOCKS[id].name]; return v ? B[v['xyz'.indexOf(axis)]] : id; }
