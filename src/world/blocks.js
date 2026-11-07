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
  'sandstone', 'sandstone_top', 'red_sandstone', 'cactus_side', 'cactus_top', 'bark_palm', 'end_palm', 'leaf_palm'];
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
];

export const BLOCKS = defs.map(([name, shape, mats, hardness, p], id) => ({
  id, name, shape, hardness,
  faces: mats ? mats.map(matLayer) : null,
  // Matériau par direction [X, Y+, Y-, Z], et sens du fil : 0 libre, 1 tourné d'un quart, 2 fixe.
  dirFaces: !mats ? null : p.axis === 'x' || p.axis === 'z' ? [mats[1], mats[0], mats[0], mats[2]].map(matLayer) : [mats[1], mats[0], mats[2], mats[1]].map(matLayer),
  grain: (p.axis === 'x' ? [0, 1, 1, 1] : p.axis === 'z' ? [1, 2, 2, 0] : p.axis === 'pz' ? [0, 1, 1, 0] : p.axis === 'py' ? [1, 0, 0, 1] : name === 'planks' ? [0, 2, 2, 0] : [0, 0, 0, 0]).map((g) => g + (p.clean ? 4 : 0)),
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
