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
// Écorce en côtes verticales : colonnes de largeur régulière, ondulées et coupées par endroits.
function bark(t, x, y, colors, relief = 1) {
  const COLS = 12, w = x + 2.6 * t.fbm(x, y, 4, 2) - 1.3, q = ((w + 64) % 64) * COLS / 64, col = Math.floor(q) % COLS, u = q - Math.floor(q);
  const tc = t.p[(col * 61 + 7) & 1023], seg = ((y + tc * 64) % 64) / 64 * 5, sid = Math.floor(seg), sf = seg - sid, ts = t.p[(col * 17 + sid * 131) & 1023];
  const ridge = Math.pow(Math.sin(u * Math.PI), 0.7), gap = smooth(0, 0.07, sf) * smooth(1, 0.93, sf), n = t.fbm(x, y, 8, 3), fine = t.val(x, y * 0.5, 32);
  const c = pal(colors, 0.16 + 0.34 * tc + 0.26 * ts + 0.3 * (n - 0.5) + 0.22 * ridge + 0.16 * (fine - 0.5));
  const k = 1 - relief * (0.58 * (1 - ridge) + 0.3 * (1 - gap));
  return { c: c.map((v) => v * k), h: ridge * (0.5 + 0.5 * ts) * (0.4 + 0.6 * gap), r: 0.84 };
}
const OAK = ['#4a2c14', '#7a4f29', '#a8743f', '#cf9d62'].map(hex), BEECH = ['#726c60', '#938c7f', '#b0a99b', '#c9c3b5'].map(hex);
const BIRCH = ['#d6d0bf', '#ece7da', '#fbf8f0'].map(hex), BIRCH_DARK = ['#17140f', '#3e3428'].map(hex);
// Bouleau : marques noires franches, en rectangles irréguliers alignés sur les rangées.
function birch(t, x, y) {
  const row = Math.floor(y / 9), jit = Math.floor(t.p[(row * 29) & 1023] * 11), cell = Math.floor((x + jit) / 11) % 6, id = t.p[(cell * 37 + row * 101) & 1023];
  const ry = y / 9 - row, rx = ((x + jit) / 11) % 1, inMark = id > 0.6 && ry > 0.16 + 0.2 * id && ry < 0.86 && rx > 0.1 && rx < 0.95;
  if (inMark) return { c: pal(BIRCH_DARK, t.val(x, y, 32)), h: 0.12, r: 0.72 };
  const n = t.fbm(x, y, 6, 3), warm = t.val(x, y, 5);
  const c = pal(BIRCH, 0.22 + 0.7 * n);
  return { c: [c[0], c[1] * (0.985 - 0.03 * warm), c[2] * (0.97 - 0.09 * warm)], h: 0.5 + 0.3 * n, r: 0.5 };
}
// Cœur de l'arbre : cernes concentriques déformés, moelle sombre, fentes radiales, couronne d'écorce.
function woodEnd(t, x, y, wood, rim, barkFn) {
  const dx = x - 31.5, dy = y - 31.5, r = Math.hypot(dx, dy), box = Math.max(Math.abs(dx), Math.abs(dy)), a = Math.atan2(dy, dx);
  const edge = box > 27.5 + 1.6 * t.val(x, y, 8);
  if (edge) { const b = barkFn(t, x, y); return { c: b.c.map((v) => v * rim), h: b.h, r: 0.85 }; }
  const wob = 3.2 * t.fbm(x, y, 4, 2) - 1.6, q = r * 0.62 + wob, ring = 0.5 + 0.5 * Math.sin(q * 2.2), late = smooth(0.72, 0.96, ring);
  const crack = Math.abs(Math.sin(a * 3 + 1.2 + 2 * t.val(x, y, 3))) < 0.035 && r > 5 && r < 24 ? 1 : 0, pith = smooth(4.5, 2, r);
  const c = pal(wood, 0.7 - 0.27 * late + 0.14 * (t.val(x, y, 32) - 0.5) - 0.3 * pith - 0.12 * smooth(26, 21, r));
  const k = (1 - 0.42 * crack) * (1 - 0.16 * smooth(21, 27, box));
  return { c: c.map((v) => v * k), h: 0.45 + 0.3 * ring - 0.4 * crack, r: 0.7 };
}
const OAK_W = ['#5d3a1c', '#8e6133', '#bd8c52', '#dcb67e'].map(hex), BIRCH_W = ['#a98a5d', '#cdb080', '#e6d3a8', '#f3e8c9'].map(hex), BEECH_W = ['#8a6a48', '#b28f66', '#d0b08a', '#e4cdaa'].map(hex);
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
  bark_beech: (t, x, y) => bark(t, x, y, BEECH, 0.28),
  leaf_oak: (t, x, y) => leaf(t, x, y, [1, 1, 1]), leaf_birch: (t, x, y) => leaf(t, x, y, [1.2, 1.12, 0.9]), leaf_beech: (t, x, y) => leaf(t, x, y, [0.86, 1, 0.82]),
  end_oak: (t, x, y) => woodEnd(t, x, y, OAK_W, 0.78, (a, b, c) => bark(a, b, c, OAK)), end_birch: (t, x, y) => woodEnd(t, x, y, BIRCH_W, 0.95, birch), end_beech: (t, x, y) => woodEnd(t, x, y, BEECH_W, 0.85, (a, b, c) => bark(a, b, c, BEECH, 0.28)),
  planks, water_foam: (t, x, y) => { const f = t.fbm(x, y, 8, 3); return { c: [f, f, f], h: f, r: 0.3 }; },
};

const DETAIL = { bark_oak: 0.2, bark_birch: 0.08, bark_beech: 0.14, end_oak: 0.1, end_birch: 0.08, end_beech: 0.08, leaf_oak: 0.16, leaf_birch: 0.16, leaf_beech: 0.16, stone: 0.2, mossy_stone: 0.2, limestone: 0.2, granite: 0.2, gravel: 0.22, dirt: 0.22, path: 0.2, forest_soil: 0.24, grass_top: 0.22, grass_side: 0.2, planks: 0.14, sand: 0.14 };

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
