// VoxelWorld/blocks.js — registre de blocs défini par des données.
// Ajouter un bloc = ajouter une ligne ici, sans toucher au moteur.

export const CX = 32, CH = 128, CA = CX * CX * CH; // taille d'une colonne de chunk
export const idx = (x, y, z) => x + z * CX + y * CX * CX;

// Matériaux : chaque matériau occupe VARIANTS couches consécutives du texture array.
export const VARIANTS = 4;
export const MATERIALS = ['grass_top', 'grass_side', 'dirt', 'path', 'stone', 'mossy_stone', 'limestone', 'granite', 'gravel', 'sand',
  'bark_oak', 'bark_birch', 'bark_beech', 'leaf_oak', 'leaf_birch', 'leaf_beech', 'planks', 'forest_soil', 'water_foam', 'end_oak', 'end_birch', 'end_beech'];
export const matLayer = (name) => MATERIALS.indexOf(name) * VARIANTS;

const defs = [
  // nom, shape, matériaux [haut, côté, bas], dureté (s), propriétés
  ['air', 'air', null, 0, {}],
  ['grass', 'cube', ['grass_top', 'grass_side', 'dirt'], 0.5, { drop: 'dirt', cat: 'terrain' }],
  ['dirt', 'cube', ['dirt', 'dirt', 'dirt'], 0.45, { cat: 'terrain' }],
  ['path', 'cube', ['path', 'dirt', 'dirt'], 0.45, { drop: 'dirt', cat: 'terrain' }],
  ['forest_soil', 'cube', ['forest_soil', 'dirt', 'dirt'], 0.5, { drop: 'dirt', cat: 'terrain' }],
  ['stone', 'cube', ['stone', 'stone', 'stone'], 1.3, { cat: 'rock' }],
  ['mossy_stone', 'cube', ['mossy_stone', 'mossy_stone', 'stone'], 1.3, { drop: 'stone', cat: 'rock' }],
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
];

export const BLOCKS = defs.map(([name, shape, mats, hardness, p], id) => ({
  id, name, shape, hardness,
  faces: mats ? mats.map(matLayer) : null,
  // Matériau par direction [X, Y+, Y-, Z], et sens du fil : 0 libre, 1 tourné d'un quart, 2 fixe.
  dirFaces: !mats ? null : p.axis === 'x' || p.axis === 'z' ? [mats[1], mats[0], mats[0], mats[2]].map(matLayer) : [mats[1], mats[0], mats[2], mats[1]].map(matLayer),
  grain: p.axis === 'x' ? [0, 1, 1, 1] : p.axis === 'z' ? [1, 2, 2, 0] : p.axis === 'pz' ? [0, 1, 1, 0] : name === 'planks' ? [0, 2, 2, 0] : [0, 0, 0, 0],
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
const ORIENT = { oak_log: ['oak_log_x', 'oak_log_z'], birch_log: ['birch_log_x', 'birch_log_z'], beech_log: ['beech_log_x', 'beech_log_z'] };
export function orientedBlock(id, dx, dy, dz) {
  const name = BLOCKS[id].name, ax = Math.abs(dx), az = Math.abs(dz);
  if (name === 'planks') return az > ax ? B.planks_z : id;
  const v = ORIENT[name];
  if (!v || Math.abs(dy) > Math.hypot(dx, dz)) return id;
  return B[ax >= az ? v[0] : v[1]];
}
