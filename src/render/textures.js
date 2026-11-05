// Materials/textures.js — bibliothèque de matériaux procéduraux 64×64 (albédo, relief, rugosité), périodiques.
import { Rng, clamp, lerp, smooth } from '../core/noise.js';
import { MATERIALS, VARIANTS } from '../world/blocks.js';

export const TEX = 64;

class Tile {
  constructor(seed, n = TEX) { this.n = n; const r = new Rng(seed); this.p = new Float32Array(1024); for (let i = 0; i < 1024; i++) this.p[i] = r.next(); }
  val(x, y, f) { // bruit de valeur périodique
    const fx = x * f / this.n, fy = y * f / this.n, X = Math.floor(fx), Y = Math.floor(fy), a = fx - X, b = fy - Y;
    const h = (i, j) => this.p[(((i % f) + f) % f) * 37 + (((j % f) + f) % f) * 101 & 1023];
    const u = a * a * (3 - 2 * a), v = b * b * (3 - 2 * b);
    return lerp(lerp(h(X, Y), h(X + 1, Y), u), lerp(h(X, Y + 1), h(X + 1, Y + 1), u), v);
  }
  fbm(x, y, f, o = 3) { let s = 0, a = 1, n = 0; for (let i = 0; i < o; i++) { s += a * this.val(x, y, f); n += a; a *= 0.5; f *= 2; } return s / n; }
  // distance à la cellule de Voronoï périodique, renvoie F1, F2, id
  vor(x, y, f) {
    const c = this.n / f, X = Math.floor(x / c), Y = Math.floor(y / c);
    let d1 = 1e9, d2 = 1e9, id = 0;
    for (let j = -1; j <= 1; j++) for (let i = -1; i <= 1; i++) {
      const a = X + i, b = Y + j, wa = ((a % f) + f) % f, wb = ((b % f) + f) % f, k = wa * 31 + wb * 67 & 1023;
      const px = (a + 0.15 + 0.7 * this.p[k]) * c, py = (b + 0.15 + 0.7 * this.p[(k + 211) & 1023]) * c, d = Math.hypot(px - x, py - y);
      if (d < d1) { d2 = d1; d1 = d; id = k; } else if (d < d2) d2 = d;
    }
    return [d1 / c, d2 / c, id];
  }
}

const mix3 = (a, b, t) => [lerp(a[0], b[0], t), lerp(a[1], b[1], t), lerp(a[2], b[2], t)];
const hex = (h) => [parseInt(h.slice(1, 3), 16) / 255, parseInt(h.slice(3, 5), 16) / 255, parseInt(h.slice(5, 7), 16) / 255];
const pal = (arr, t) => { t = clamp(t, 0, 0.9999) * (arr.length - 1); const i = Math.floor(t); return mix3(arr[i], arr[i + 1], t - i); };

const GRASS = ['#556b2a', '#6f8834', '#8ba042', '#a9b658'].map(hex);
const DIRT = ['#4d3019', '#6e4725', '#8a5f35', '#a57c4d'].map(hex);
const STONE = ['#5c564c', '#7f786b', '#a0998b', '#bfb8aa'].map(hex);
const LEAF = ['#2f5518', '#4f7f25', '#86ac38', '#c6d862'].map(hex);

function grassTop(t, x, y) {
  const n = t.fbm(x, y, 4, 3), s = t.val(x, y, 32), b = t.val(x + 7, y * 0.45, 24);
  const v = 0.2 + 0.55 * n + 0.32 * (s - 0.5) + 0.25 * (b - 0.5);
  return { c: pal(GRASS, v), h: 0.4 + 0.5 * s, r: 0.72 };
}
function dirt(t, x, y, a = DIRT) {
  const n = t.fbm(x, y, 5, 3), s = t.val(x, y, 32), [d1, d2, id] = t.vor(x, y, 12), rim = smooth(0, 0.1, d2 - d1), pb = t.p[id] > 0.72 ? rim : 0;
  const v = 0.08 + 0.72 * n + 0.34 * (s - 0.5) - 0.22 * (1 - rim) * 0.5 + 0.34 * pb;
  return { c: pal(a, v), h: 0.3 + 0.3 * n + 0.2 * s + 0.4 * pb, r: 0.86 };
}
function grassSide(t, x, y) {
  const fringe = 13 + 9 * t.val(x, 0, 16) + 5 * t.val(x, 0, 32);
  if (y < fringe) { const g = grassTop(t, x, y); g.c = g.c.map((c) => c * (0.72 + 0.38 * (1 - y / fringe))); return g; }
  const d = dirt(t, x, y), e = smooth(fringe + 5, fringe, y); d.c = d.c.map((c) => c * (1 - 0.35 * e)); return d;
}
function stone(t, x, y, a = STONE, cells = 5) {
  const [d1, d2, id] = t.vor(x, y, cells), crack = smooth(0.0, 0.11, d2 - d1), n = t.fbm(x, y, 8, 3), s = t.val(x, y, 32);
  const tone = 0.34 + 0.32 * t.p[id] + 0.55 * (n - 0.5) + 0.3 * (s - 0.5) + 0.12 * crack;
  return { c: pal(a, tone * (0.72 + 0.28 * crack)), h: 0.2 + 0.6 * crack + 0.2 * n, r: 0.8 };
}
function mossy(t, x, y) {
  const st = stone(t, x, y), m = t.fbm(x, y, 6, 3) + (32 - y) / 90;
  const k = smooth(0.46, 0.56, m);
  const g = grassTop(t, x * 2, y * 2);
  return { c: mix3(st.c, g.c.map((c) => c * 0.82), k), h: lerp(st.h, 0.6 + 0.3 * t.val(x, y, 24), k), r: lerp(st.r, 0.9, k) };
}
// Plaques d'écorce allongées : Voronoï étiré, périodique car x est multiplié par un entier.
function bark(t, x, y, colors, relief = 1) {
  const [d1, d2, id] = t.vor(x * 3, y, 6), crack = smooth(0, 0.2, d2 - d1), n = t.fbm(x, y, 7, 3), s = t.val(x, y, 32), tone = t.p[id];
  const c = pal(colors, 0.2 + 0.46 * tone + 0.42 * (n - 0.5) + 0.2 * (s - 0.5) + 0.22 * crack);
  const k = 1 - relief * 0.5 * (1 - crack);
  return { c: c.map((v) => v * k), h: crack * (0.55 + 0.45 * tone) + 0.15 * n, r: 0.84 };
}
const OAK = ['#3a2210', '#6b4424', '#9a6a3a', '#c39258'].map(hex), BEECH = ['#6a655b', '#8a8478', '#a8a295', '#c4beb0'].map(hex);
const BIRCH = ['#d3cdbc', '#eae5d8', '#faf7ef'].map(hex), BIRCH_DARK = ['#17140f', '#3e3428'].map(hex);
function birch(t, x, y) {
  const [d1, , id] = t.vor(x, y * 3, 7), lens = t.p[id] > 0.6 && d1 < 0.3, scar = t.fbm(x, y, 3, 2) > 0.67;
  if (lens || scar) return { c: pal(BIRCH_DARK, t.val(x, y, 32)), h: 0.12, r: 0.72 };
  const band = 0.5 + 0.5 * Math.sin((y / 64) * Math.PI * 12 + 4 * t.val(x, y, 4)), n = t.fbm(x, y, 6, 3), warm = t.val(x, y, 5);
  const c = pal(BIRCH, 0.25 + 0.55 * n + 0.18 * band);
  return { c: [c[0], c[1] * (0.985 - 0.03 * warm), c[2] * (0.97 - 0.09 * warm)], h: 0.55 + 0.25 * band * n, r: 0.5 };
}
function leaf(t, x, y, shift) {
  const [d1, d2, id] = t.vor(x, y, 11), rim = smooth(0, 0.14, d2 - d1), vein = smooth(0.26, 0.05, d1), m = t.fbm(x, y, 5, 2), g = t.val(x, y, 32);
  const c = pal(LEAF, 0.12 + 0.56 * t.p[id] + 0.3 * (m - 0.2) + 0.12 * vein + 0.1 * (g - 0.5));
  const k = 0.55 + 0.45 * rim;
  return { c: [c[0] * shift[0] * k, c[1] * shift[1] * k, c[2] * shift[2] * k], h: rim * (0.5 + 0.5 * t.p[id]), r: 0.5 };
}
function planks(t, x, y) {
  const row = Math.floor(y / 16), f = (y % 16) / 16, seam = smooth(0, 0.07, f) * smooth(1, 0.93, f), g = t.val(x + row * 19, y * 2.2, 12), tone = t.p[row * 13 & 1023];
  return { c: pal(['#6c4524', '#94673a', '#b98a52'].map(hex), 0.15 + 0.45 * tone + 0.4 * g).map((c) => c * (0.5 + 0.5 * seam)), h: seam * (0.7 + 0.3 * g), r: 0.7 };
}

const GEN = {
  grass_top: grassTop, grass_side: grassSide, dirt: (t, x, y) => dirt(t, x, y),
  path: (t, x, y) => dirt(t, x, y, ['#7c5b35', '#a07a4c', '#bc9663', '#d2b080'].map(hex)),
  forest_soil: (t, x, y) => { const d = dirt(t, x, y, ['#2f2413', '#4a3a1f', '#5e5127', '#6f6a2f'].map(hex)); return d; },
  stone: (t, x, y) => stone(t, x, y), mossy_stone: mossy,
  limestone: (t, x, y) => stone(t, x, y, ['#6f685b', '#92897a', '#ada697', '#c8c1b1'].map(hex), 4),
  granite: (t, x, y) => stone(t, x, y, ['#403d3c', '#5c5855', '#7a7470', '#938c86'].map(hex), 7),
  gravel: (t, x, y) => stone(t, x, y, ['#6c6761', '#8e8679', '#a89f91', '#c0b7a9'].map(hex), 11),
  sand: (t, x, y) => dirt(t, x, y, ['#a89268', '#c2ac7e', '#d2bf92', '#e0cfa6'].map(hex)),
  bark_oak: (t, x, y) => bark(t, x, y, OAK),
  bark_birch: birch,
  bark_beech: (t, x, y) => bark(t, x, y, BEECH, 0.45),
  leaf_oak: (t, x, y) => leaf(t, x, y, [1, 1, 1]), leaf_birch: (t, x, y) => leaf(t, x, y, [1.2, 1.12, 0.9]), leaf_beech: (t, x, y) => leaf(t, x, y, [0.86, 1, 0.82]),
  planks, water_foam: (t, x, y) => { const f = t.fbm(x, y, 8, 3); return { c: [f, f, f], h: f, r: 0.3 }; },
};

const DETAIL = { bark_oak: 0.3, bark_birch: 0.08, bark_beech: 0.18, leaf_oak: 0.16, leaf_birch: 0.16, leaf_beech: 0.16, stone: 0.2, mossy_stone: 0.2, limestone: 0.2, granite: 0.2, gravel: 0.22, dirt: 0.22, path: 0.2, forest_soil: 0.24, grass_top: 0.22, grass_side: 0.2, planks: 0.14, sand: 0.14 };

// Rend deux piles RGBA : albédo, normale XY + rugosité. Les motifs vivent dans un repère logique
// de 64 unités ; à 128 pixels, une octave fine ajoute un vrai détail au lieu d'agrandir le motif.
export function buildTextures(size = 128) {
  const layers = MATERIALS.length * VARIANTS, albedo = new Uint8Array(size * size * 4 * layers), nrm = new Uint8Array(size * size * 4 * layers);
  const H = new Float32Array(size * size), k = TEX / size, gain = (1.6 * size) / TEX;
  const wrap = (v) => (v + size) % size;
  MATERIALS.forEach((name, m) => {
    const gen = GEN[name], amp = size > TEX ? DETAIL[name] || 0 : 0;
    for (let v = 0; v < VARIANTS; v++) {
      const t = new Tile(1000 + m * 17 + v * 3), layer = m * VARIANTS + v, base = layer * size * size * 4;
      for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) {
        const lx = x * k, ly = y * k, s = gen(t, lx, ly), p = base + (x + y * size) * 4, d = amp ? t.val(lx, ly, size) - 0.5 : 0, g = 1 + d * amp * 2;
        albedo[p] = clamp(s.c[0] * g, 0, 1) * 255; albedo[p + 1] = clamp(s.c[1] * g, 0, 1) * 255; albedo[p + 2] = clamp(s.c[2] * g, 0, 1) * 255; albedo[p + 3] = 255;
        H[x + y * size] = s.h + d * amp; nrm[p + 2] = s.r * 255; nrm[p + 3] = 255;
      }
      for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) {
        const p = base + (x + y * size) * 4, dx = H[wrap(x + 1) + y * size] - H[wrap(x - 1) + y * size], dy = H[x + wrap(y + 1) * size] - H[x + wrap(y - 1) * size];
        nrm[p] = clamp(0.5 - dx * gain, 0, 1) * 255; nrm[p + 1] = clamp(0.5 - dy * gain, 0, 1) * 255;
      }
    }
  });
  return { albedo, nrm, layers, size };
}

// Bruit 3D périodique pour les nuages et la brume.
export function buildNoise3D(n = 40) {
  const data = new Uint8Array(n * n * n), r = new Rng(9001), g = new Float32Array(n * n * n);
  for (let i = 0; i < g.length; i++) g[i] = r.next();
  const samp = (x, y, z, f) => {
    const s = n / f, fx = x / s, fy = y / s, fz = z / s, X = Math.floor(fx), Y = Math.floor(fy), Z = Math.floor(fz);
    const u = fx - X, v = fy - Y, w = fz - Z, su = u * u * (3 - 2 * u), sv = v * v * (3 - 2 * v), sw = w * w * (3 - 2 * w);
    const h = (i, j, k) => g[((i % f) * 3 + (j % f) * n * 5 + (k % f) * n * n * 7) % g.length];
    return lerp(lerp(lerp(h(X, Y, Z), h(X + 1, Y, Z), su), lerp(h(X, Y + 1, Z), h(X + 1, Y + 1, Z), su), sv),
      lerp(lerp(h(X, Y, Z + 1), h(X + 1, Y, Z + 1), su), lerp(h(X, Y + 1, Z + 1), h(X + 1, Y + 1, Z + 1), su), sv), sw);
  };
  for (let z = 0; z < n; z++) for (let y = 0; y < n; y++) for (let x = 0; x < n; x++) {
    const a = samp(x, y, z, 4) * 0.55 + samp(x, y, z, 8) * 0.28 + samp(x, y, z, 16) * 0.17;
    data[x + y * n + z * n * n] = clamp((a - 0.2) / 0.6, 0, 1) * 255;
  }
  return { data, n };
}
