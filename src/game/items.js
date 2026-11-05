// Gameplay/items.js — catalogue d'objets défini par des données, modèles d'outils et blocs tenus en main.
import { BLOCKS, B, matLayer } from '../world/blocks.js';
import { VBuf, box, KIND, decoModel } from '../world/mesher.js';

const LABELS = { grass: 'Herbe', dirt: 'Terre', path: 'Terre de chemin', forest_soil: 'Humus', gravel: 'Gravier', sand: 'Sable', stone: 'Pierre', mossy_stone: 'Pierre moussue', limestone: 'Calcaire', granite: 'Granit',
  oak_log: 'Bois de chêne', birch_log: 'Bois de bouleau', beech_log: 'Bois de hêtre', planks: 'Planches', fence: 'Clôture', oak_leaves: 'Feuilles de chêne', birch_leaves: 'Feuilles de bouleau', beech_leaves: 'Feuilles de hêtre',
  tall_grass: 'Herbes hautes', fern: 'Fougère', daisy: 'Marguerite', lupine: 'Lupin', pink_flower: 'Fleur rose', yellow_flower: 'Fleur jaune', mushroom: 'Champignon', shelf_fungus: 'Polypore', lily_pad: 'Nénuphar', vine: 'Liane', flagstone: 'Dalle', lantern: 'Lanterne', water: 'Eau' };
export const CATEGORIES = [
  ['Terrain', ['grass', 'dirt', 'path', 'forest_soil', 'gravel', 'sand', 'water']],
  ['Roches', ['stone', 'mossy_stone', 'limestone', 'granite', 'flagstone']],
  ['Bois', ['oak_log', 'birch_log', 'beech_log', 'planks', 'fence']],
  ['Feuillages', ['oak_leaves', 'birch_leaves', 'beech_leaves', 'vine']],
  ['Plantes', ['tall_grass', 'fern', 'daisy', 'lupine', 'pink_flower', 'yellow_flower', 'mushroom', 'shelf_fungus', 'lily_pad']],
  ['Lumière', ['lantern']],
  ['Outils', ['pickaxe', 'axe', 'sword']],
];
export const ITEMS = { pickaxe: { tool: 'rock', name: 'Pioche' }, axe: { tool: 'wood', name: 'Hache' }, sword: { tool: null, name: 'Épée' } };
for (const k in LABELS) ITEMS[k] = { block: B[k], name: LABELS[k] };
export const STACK = 99;

// Outils : image 56 × 56 décrite par des formes, puis extrudée en petits voxels.
const WOOD = [[0.3, 0.17, 0.08], [0.48, 0.29, 0.14], [0.66, 0.43, 0.22]], LEATHER = [[0.2, 0.11, 0.06], [0.36, 0.2, 0.1]];
const STEEL = [[0.4, 0.39, 0.38], [0.62, 0.61, 0.6], [0.84, 0.83, 0.82], [1, 0.99, 0.97]], GOLD = [[0.62, 0.42, 0.1], [0.92, 0.7, 0.2]];
const R2 = Math.SQRT1_2;
function handle(s, p, a, b) {
  if (s < a || s > b || Math.abs(p) > 0.042) return null;
  if (s < a + 0.26) return (Math.floor(s * 40) & 1) ? LEATHER[0] : LEATHER[1];
  return Math.abs(p) > 0.024 ? WOOD[0] : p > 0 ? WOOD[2] : WOOD[1];
}
const SHAPES = {
  pickaxe(x, y) {
    const dx = x - 0.34, dy = y - 0.34, r = Math.hypot(dx, dy), ang = Math.atan2(dy, dx) - Math.PI / 4, t = ang / 1.02;
    if (Math.abs(t) < 1) {
      const w = 0.018 + 0.066 * (1 - t * t), d = r - 0.62;
      if (Math.abs(d) < w) return d > w * 0.45 ? (Math.abs(t) > 0.6 ? STEEL[3] : STEEL[2]) : d < -w * 0.45 ? STEEL[0] : STEEL[1];
    }
    const s = (x + y) * R2, p = (y - x) * R2;
    if (s > 0.8 && s < 0.97 && Math.abs(p) < 0.06) return STEEL[0];
    return handle(s, p, 0.1, 0.93);
  },
  axe(x, y) {
    const s = (x + y) * R2, p = (y - x) * R2;
    if (p > 0 && p < 0.36) { const half = 0.075 + 0.2 * Math.pow(p / 0.36, 1.5), c = 0.88; if (Math.abs(s - c) < half) { const edge = 0.36 - p; return edge < 0.035 ? STEEL[3] : edge < 0.075 ? STEEL[2] : (s - c) > 0.02 ? STEEL[1] : STEEL[0]; } }
    if (p <= 0 && p > -0.11 && Math.abs(s - 0.88) < 0.065) return p < -0.08 ? STEEL[2] : STEEL[1];
    return handle(s, p, 0.1, 1.03);
  },
  sword(x, y) {
    const s = (x + y) * R2, p = (y - x) * R2;
    if (s > 0.34 && s < 1.28) { const w = s > 1.1 ? 0.058 * (1.28 - s) / 0.18 : 0.058; if (Math.abs(p) < w) return Math.abs(p) < 0.01 ? STEEL[0] : p > 0 ? STEEL[3] : STEEL[1]; }
    if (Math.abs(s - 0.32) < 0.032 && Math.abs(p) < 0.17) return Math.abs(p) > 0.13 ? GOLD[1] : GOLD[0];
    if (s > 0.1 && s < 0.3 && Math.abs(p) < 0.034) return (Math.floor(s * 44) & 1) ? LEATHER[0] : LEATHER[1];
    if (Math.hypot(s - 0.07, p) < 0.05) return GOLD[1];
    return null;
  },
};

// Taille d'un outil : 1 unité de forme = 0,86 bloc. L'origine du maillage est le point de prise en main.
export function buildToolMesh(name) {
  const o = new VBuf(8000), N = 56, S = 0.86, px = S / N, grip = 0.17 * R2 * S, f = SHAPES[name];
  for (let j = 0; j < N; j++) {
    let run = null;
    for (let i = 0; i <= N; i++) {
      const c = i < N ? f((i + 0.5) / N, (j + 0.5) / N) : null;
      if (run && (!c || c !== run.c)) {
        const th = run.c === STEEL[3] ? 0.012 : STEEL.includes(run.c) ? 0.022 : 0.03;
        box(o, run.i * px - grip, j * px - grip, -th, i * px - grip, (j + 1) * px - grip, th, run.c[0], run.c[1], run.c[2], 0, 255, 0, KIND.FLAT, true);
        run = null;
      }
      if (c && !run) run = { i, c };
    }
  }
  return o;
}

function cubeMesh(id) {
  const o = new VBuf(32), b = BLOCKS[id];
  if (id === B.water) { box(o, 0, 0, 0, 1, 0.9, 1, 0.3, 0.55, 0.75, 0, 255, 0, KIND.FLAT, true); return o; }
  const leaf = b.shape === 'leaf', kind = leaf ? KIND.LEAF : KIND.TEX, t = b.tint;
  const q = (di, pts, n) => { o.rot = b.grain[di]; for (const p of pts) o.v(p[0], p[1], p[2], b.dirFaces[di], n[0], n[1], n[2], 0, 255, 255, 0, kind, t[0], t[1], t[2]); o.rot = 0; };
  q(1, [[0, 1, 1], [1, 1, 1], [1, 1, 0], [0, 1, 0]], [0, 1, 0]); q(2, [[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]], [0, -1, 0]);
  q(3, [[0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]], [0, 0, 1]); q(3, [[1, 0, 0], [0, 0, 0], [0, 1, 0], [1, 1, 0]], [0, 0, -1]);
  q(0, [[1, 0, 1], [1, 0, 0], [1, 1, 0], [1, 1, 1]], [1, 0, 0]); q(0, [[0, 0, 0], [0, 0, 1], [0, 1, 1], [0, 1, 0]], [-1, 0, 0]);
  return o;
}

// Tous les objets tiennent dans un seul maillage ; chacun garde sa plage et sa façon d'être tenu.
export function buildItems(r) {
  const all = new VBuf(60000), table = {};
  for (const name in ITEMS) {
    const def = ITEMS[name], id = def.block; let o, kind;
    if (id === undefined) { o = buildToolMesh(name); kind = 'tool'; }
    else if (BLOCKS[id].shape === 'deco') { o = new VBuf(800); decoModel(o, id, 0, 0, 0, 7, 11, 255, id === B.lantern ? 255 : 0, 0); kind = 'deco'; }
    else { o = cubeMesh(id); kind = 'block'; }
    const st = all.n;
    if (all.n + o.n > all.cap) { const u = all.u8; all.alloc((all.n + o.n) * 2); all.u8.set(u); }
    all.u8.set(o.u8.subarray(0, o.n * 20), all.n * 20); all.n += o.n;
    table[name] = { offset: (st / 4) * 6 * 4, count: (o.n / 4) * 6, kind, local: kind === 'block' && id !== B.water && !0 };
  }
  const mesh = r.mesh(all.take(), all.n);
  for (const n in table) table[n].mesh = mesh;
  return table;
}
void matLayer;
