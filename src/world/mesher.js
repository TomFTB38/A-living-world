// VoxelWorld/mesher.js — greedy meshing, AO par sommet, lumière voxel (ciel + blocs), géométrie enrichie.
import { Rng, hashInt } from '../core/noise.js';
import { CX, CH, CA, BLOCKS, B, IS_OPAQUE, IS_SOLID, IS_LEAF, IS_DECO, EMIT, matLayer } from './blocks.js';

export const VSIZE = 20;
export const KIND = { TEX: 0, LEAF: 1, FLAT: 2, FOLIAGE: 3, EMIT: 4 };
const PAD = 8, W = CX + PAD * 2, WW = W * W;
const POSK = 128;

export class VBuf {
  constructor(cap = 4096) { this.alloc(cap); this.n = 0; this.scale = POSK; this.rot = 0; }
  alloc(cap) { this.cap = cap; this.ab = new ArrayBuffer(cap * VSIZE); this.i16 = new Int16Array(this.ab); this.u8 = new Uint8Array(this.ab); this.i8 = new Int8Array(this.ab); }
  v(x, y, z, layer, nx, ny, nz, wind, ao, sky, blk, kind, r, g, b) {
    if (this.n === this.cap) { const o = this.u8; this.alloc(this.cap * 2); this.u8.set(o); }
    const p = this.n * 10, q = this.n * VSIZE, S = this.scale, h = this.i16, u = this.u8, s = this.i8;
    h[p] = Math.round(x * S); h[p + 1] = Math.round(y * S); h[p + 2] = Math.round(z * S); h[p + 3] = layer;
    s[q + 8] = nx * 127; s[q + 9] = ny * 127; s[q + 10] = nz * 127; u[q + 11] = wind >> 1;
    u[q + 12] = ao; u[q + 13] = sky; u[q + 14] = blk; u[q + 15] = kind;
    u[q + 16] = r < 2 ? r * 127.5 : 255; u[q + 17] = g < 2 ? g * 127.5 : 255; u[q + 18] = b < 2 ? b * 127.5 : 255; u[q + 19] = this.rot;
    this.n++;
  }
  take() { return this.ab.slice(0, this.n * VSIZE); }
}

const vol = new Uint8Array(WW * CH), sky = new Uint8Array(WW * CH), blk = new Uint8Array(WW * CH);
const m1 = new Int32Array(CX * CH), m2 = new Int32Array(CX * CH);
const at = (x, y, z) => x + PAD + (z + PAD) * W + y * WW;
const AO = [0.42, 0.62, 0.82, 1];

function fillVolume(getChunk, cx, cz) {
  let ymax = 0, ymin = CH - 1;
  for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) {
    const c = getChunk(cx + dx, cz + dz);
    const x0 = dx < 0 ? CX - PAD : 0, x1 = dx > 0 ? PAD : CX, z0 = dz < 0 ? CX - PAD : 0, z1 = dz > 0 ? PAD : CX;
    for (let y = 0; y < CH; y++) {
      for (let z = z0; z < z1; z++) {
        const zz = z + dz * CX + PAD, s = x0 + z * CX + y * CX * CX, d = x0 + dx * CX + PAD + zz * W + y * WW;
        vol.set(c.subarray(s, s + (x1 - x0)), d);
      }
    }
  }
  const top = new Int16Array(WW);
  for (let i = 0; i < WW; i++) {
    let y = CH - 1;
    while (y > 0 && vol[i + y * WW] === 0) y--;
    top[i] = y; if (y > ymax) ymax = y;
  }
  for (let i = 0; i < WW; i++) { let y = 0; while (y < ymax && !IS_OPAQUE[vol[i + y * WW]]) y++; let l = top[i]; if (l < ymin) ymin = l; void y; }
  return { ymax, top };
}

function computeLight(top, ymax) {
  sky.fill(0); blk.fill(0);
  let ylow = CH;
  for (let i = 0; i < WW; i++) {
    let s = 15;
    for (let y = CH - 1; y >= 0; y--) {
      const k = i + y * WW;
      if (y > top[i]) { sky[k] = 15; continue; }
      const b = vol[k];
      if (IS_OPAQUE[b]) { s = 0; if (y < ylow) ylow = y; break; }
      if (IS_LEAF[b]) s = s > 1 ? s - 1 : 0;
      else if (b === B.water) s = s > 0 ? s - 1 : 0;
      sky[k] = s;
      const e = EMIT[b]; if (e) blk[k] = e;
    }
  }
  ylow = Math.max(1, ylow - 2);
  const yhi = Math.min(CH - 2, ymax + 1);
  const relax = (L) => {
    for (let y = yhi; y >= ylow; y--) for (let z = 1; z < W; z++) for (let x = 1; x < W; x++) {
      const k = x + z * W + y * WW;
      if (IS_OPAQUE[vol[k]]) continue;
      let v = L[k], c = L[k - 1] - 1; if (c > v) v = c; c = L[k - W] - 1; if (c > v) v = c; c = L[k + WW] - 1; if (c > v) v = c;
      L[k] = v;
    }
    for (let y = ylow; y <= yhi; y++) for (let z = W - 2; z >= 0; z--) for (let x = W - 2; x >= 0; x--) {
      const k = x + z * W + y * WW;
      if (IS_OPAQUE[vol[k]]) continue;
      let v = L[k], c = L[k + 1] - 1; if (c > v) v = c; c = L[k + W] - 1; if (c > v) v = c; c = L[k - WW] - 1; if (c > v) v = c;
      L[k] = v;
    }
  };
  relax(sky); relax(blk);
}

const q = [0, 0, 0], du = [0, 0, 0], dv = [0, 0, 0], nrm = [0, 0, 0];
const px = [0, 0, 0, 0], py = [0, 0, 0, 0], pz = [0, 0, 0, 0];

function greedy(out, ymin, ymax, lod) {
  const solidLeaf = lod === 2;
  for (let d = 0; d < 3; d++) {
    const u = (d + 1) % 3, v = (d + 2) % 3;
    const dims = [CX, ymax - ymin + 1, CX], off = [0, ymin, 0];
    for (let sign = -1; sign <= 1; sign += 2) {
      nrm[0] = nrm[1] = nrm[2] = 0; nrm[d] = sign;
      const nW = nrm[0] + nrm[2] * W + nrm[1] * WW;
      const uW = u === 0 ? 1 : u === 1 ? WW : W, vW = v === 0 ? 1 : v === 1 ? WW : W;
      for (let s = 0; s < dims[d]; s++) {
        let any = false;
        const nu = dims[u], nv = dims[v];
        m1.fill(-1, 0, nu * nv);
        for (let j = 0; j < nv; j++) for (let i = 0; i < nu; i++) {
          q[d] = s + off[d]; q[u] = i + off[u]; q[v] = j + off[v];
          const k = at(q[0], q[1], q[2]), b = vol[k];
          if (!(IS_OPAQUE[b] || (solidLeaf && IS_LEAF[b]))) continue;
          const f = k + nW;
          if (q[1] + nrm[1] >= CH || q[1] + nrm[1] < 0) { if (q[1] + nrm[1] < 0) continue; }
          const nb = q[1] + nrm[1] >= CH ? 0 : vol[f];
          if (IS_OPAQUE[nb] || (solidLeaf && IS_LEAF[nb])) continue;
          const di = d === 0 ? 0 : d === 2 ? 3 : sign > 0 ? 1 : 2, face = BLOCKS[b].dirFaces[di], grain = BLOCKS[b].grain[di];
          let ao = 0, skyn = 0, blkn = 0;
          for (let c = 0; c < 4; c++) {
            const su = c === 1 || c === 2 ? 1 : -1, sv = c >= 2 ? 1 : -1;
            const a = f + su * uW, e = f + sv * vW, g = a + sv * vW;
            const s1 = IS_SOLID[vol[a]], s2 = IS_SOLID[vol[e]], s3 = IS_SOLID[vol[g]];
            ao |= (s1 && s2 ? 0 : 3 - s1 - s2 - s3) << (c * 2);
            let n = 1, ss = sky[f], bb = blk[f];
            if (!IS_OPAQUE[vol[a]]) { ss += sky[a]; bb += blk[a]; n++; }
            if (!IS_OPAQUE[vol[e]]) { ss += sky[e]; bb += blk[e]; n++; }
            if (!IS_OPAQUE[vol[g]]) { ss += sky[g]; bb += blk[g]; n++; }
            skyn |= Math.round(ss / n) << (c * 4); blkn |= Math.round(bb / n) << (c * 4);
          }
          const pos = i + j * nu;
          m1[pos] = (face << 8) | ao | (IS_LEAF[b] ? 1 << 24 : 0) | (grain << 25); m2[pos] = skyn | (blkn << 16); any = true;
        }
        if (!any) continue;
        for (let j = 0; j < nv; j++) for (let i = 0; i < nu;) {
          const c1 = m1[i + j * nu];
          if (c1 === -1) { i++; continue; }
          const c2 = m2[i + j * nu];
          let w = 1;
          while (i + w < nu && m1[i + w + j * nu] === c1 && m2[i + w + j * nu] === c2) w++;
          let h = 1;
          outer: for (; j + h < nv; h++) for (let t = 0; t < w; t++) if (m1[i + t + (j + h) * nu] !== c1 || m2[i + t + (j + h) * nu] !== c2) break outer;
          for (let b = 0; b < h; b++) for (let t = 0; t < w; t++) m1[i + t + (j + b) * nu] = -1;
          q[d] = s + off[d] + (sign > 0 ? 1 : 0); q[u] = i + off[u]; q[v] = j + off[v];
          du[0] = du[1] = du[2] = 0; dv[0] = dv[1] = dv[2] = 0; du[u] = w; dv[v] = h;
          px[0] = q[0]; py[0] = q[1]; pz[0] = q[2];
          px[1] = q[0] + du[0]; py[1] = q[1] + du[1]; pz[1] = q[2] + du[2];
          px[2] = px[1] + dv[0]; py[2] = py[1] + dv[1]; pz[2] = pz[1] + dv[2];
          px[3] = q[0] + dv[0]; py[3] = q[1] + dv[1]; pz[3] = q[2] + dv[2];
          out.rot = (c1 >> 25) & 3;
          const a0 = c1 & 3, a1 = (c1 >> 2) & 3, a2 = (c1 >> 4) & 3, a3 = (c1 >> 6) & 3;
          const flip = a0 + a2 < a1 + a3 ? 1 : 0, layer = (c1 >> 8) & 0xffff, kind = c1 & (1 << 24) ? KIND.LEAF : KIND.TEX;
          for (let c = 0; c < 4; c++) {
            const o = sign > 0 ? (c + flip) & 3 : (4 - c + flip) & 3;
            out.v(px[o], py[o], pz[o], layer, nrm[0], nrm[1], nrm[2], 0, AO[(c1 >> (o * 2)) & 3] * 255, ((c2 >> (o * 4)) & 15) * 17, ((c2 >> (16 + o * 4)) & 15) * 17, kind, 1, 1, 1);
          }
          out.rot = 0;
          i += w;
        }
      }
    }
  }
}

/* --- Géométrie enrichie --- */
export function box(o, x0, y0, z0, x1, y1, z1, r, g, b, wind, s, l, kind = KIND.FLAT, bottom = false, windT = wind, ao = 255) {
  const face = (ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, nx, ny, nz, wa, wb, aoL = 255) => {
    const L = 0.86 + 0.14 * ny + 0.05 * nx; // très léger modelé pour lire les volumes
    o.v(ax, ay, az, 0, nx, ny, nz, wa, aoL, s, l, kind, r * L, g * L, b * L);
    o.v(bx, by, bz, 0, nx, ny, nz, wa, aoL, s, l, kind, r * L, g * L, b * L);
    o.v(cx, cy, cz, 0, nx, ny, nz, wb, ao, s, l, kind, r * L, g * L, b * L);
    o.v(dx, dy, dz, 0, nx, ny, nz, wb, ao, s, l, kind, r * L, g * L, b * L);
  };
  face(x0, y1, z1, x1, y1, z1, x1, y1, z0, x0, y1, z0, 0, 1, 0, windT, windT);
  if (bottom) face(x0, y0, z0, x1, y0, z0, x1, y0, z1, x0, y0, z1, 0, -1, 0, wind, wind);
  face(x0, y0, z1, x1, y0, z1, x1, y1, z1, x0, y1, z1, 0, 0, 1, wind, windT, ao * 0.8);
  face(x1, y0, z0, x0, y0, z0, x0, y1, z0, x1, y1, z0, 0, 0, -1, wind, windT, ao * 0.8);
  face(x1, y0, z1, x1, y0, z0, x1, y1, z0, x1, y1, z1, 1, 0, 0, wind, windT, ao * 0.8);
  face(x0, y0, z0, x0, y0, z1, x0, y1, z1, x0, y1, z0, -1, 0, 0, wind, windT, ao * 0.8);
}

function blade(o, x, y, z, a, w, h, lean, r, g, b, s, l, sky2) {
  const ca = Math.cos(a), sa = Math.sin(a), wx = -sa * w, wz = ca * w, tx = x + ca * lean, tz = z + sa * lean, ty = y + h;
  const nx = ca * 0.35, nz = sa * 0.35, ny = 0.93, mh = 0.55;
  const mx = x + ca * lean * 0.35, mz = z + sa * lean * 0.35, my = y + h * mh;
  // deux segments effilés : la pointe porte le vent
  const dark = 0.62;
  o.v(x - wx, y, z - wz, 0, nx, ny, nz, 0, 150, sky2, l, KIND.FOLIAGE, r * dark, g * dark, b * dark);
  o.v(x + wx, y, z + wz, 0, nx, ny, nz, 0, 150, sky2, l, KIND.FOLIAGE, r * dark, g * dark, b * dark);
  o.v(mx + wx * 0.8, my, mz + wz * 0.8, 0, nx, ny, nz, 60, 225, s, l, KIND.FOLIAGE, r * 0.9, g * 0.9, b * 0.9);
  o.v(mx - wx * 0.8, my, mz - wz * 0.8, 0, nx, ny, nz, 60, 225, s, l, KIND.FOLIAGE, r * 0.9, g * 0.9, b * 0.9);
  o.v(mx - wx * 0.8, my, mz - wz * 0.8, 0, nx, ny, nz, 60, 225, s, l, KIND.FOLIAGE, r * 0.9, g * 0.9, b * 0.9);
  o.v(mx + wx * 0.8, my, mz + wz * 0.8, 0, nx, ny, nz, 60, 225, s, l, KIND.FOLIAGE, r * 0.9, g * 0.9, b * 0.9);
  o.v(tx + wx * 0.1, ty, tz + wz * 0.1, 0, nx, ny, nz, 170, 255, s, l, KIND.FOLIAGE, r * 1.12, g * 1.12, b * 1.12);
  o.v(tx - wx * 0.1, ty, tz - wz * 0.1, 0, nx, ny, nz, 170, 255, s, l, KIND.FOLIAGE, r * 1.12, g * 1.12, b * 1.12);
}

function stem(o, x, y, z, h, s, l) { box(o, x - 0.016, y, z - 0.016, x + 0.016, y + h, z + 0.016, 0.25, 0.42, 0.13, 0, s, l, KIND.FOLIAGE, false, 90); }

export function decoModel(o, b, x, y, z, wx, wz, s, l, lod) {
  const r = new Rng(hashInt(wx, y, wz, 91)), m = BLOCKS[b].model;
  if (m === 'lantern') {
    box(o, x + 0.32, y, z + 0.32, x + 0.68, y + 0.08, z + 0.68, 0.16, 0.1, 0.06, 0, s, l);
    box(o, x + 0.36, y + 0.08, z + 0.36, x + 0.64, y + 0.54, z + 0.64, 2, 1.3, 0.5, 0, s, 255, KIND.EMIT);
    box(o, x + 0.32, y + 0.54, z + 0.32, x + 0.68, y + 0.62, z + 0.68, 0.16, 0.1, 0.06, 0, s, l);
    return;
  }
  if (m === 'vine') {
    const n = r.int(1, 2);
    for (let i = 0; i < n; i++) {
      const a = x + r.range(0.2, 0.8), c = z + r.range(0.2, 0.8), t = r.range(0.8, 1.1);
      box(o, a - 0.02, y, c - 0.02, a + 0.02, y + 1, c + 0.02, 0.26 * t, 0.42 * t, 0.15 * t, 70, s, l, KIND.FOLIAGE, false, 90);
      if (lod === 0) for (let k = 0; k < 3; k++) { const yy = y + 0.14 + k * 0.3, q = r.range(0.03, 0.055), dx = r.range(-0.09, 0.09), dz = r.range(-0.09, 0.09); box(o, a + dx - q, yy, c + dz - q, a + dx + q, yy + 0.07, c + dz + q, 0.3 * t, 0.5 * t, 0.17 * t, 80, s, l, KIND.FOLIAGE, true); }
    }
    return;
  }
  if (m === 'fungus') {
    let dx = 0, dz = 0;
    if (IS_OPAQUE[vol[at(x + 1, y, z)]]) dx = 1; else if (IS_OPAQUE[vol[at(x - 1, y, z)]]) dx = -1; else if (IS_OPAQUE[vol[at(x, y, z + 1)]]) dz = 1; else if (IS_OPAQUE[vol[at(x, y, z - 1)]]) dz = -1;
    const n = r.int(2, 3);
    for (let i = 0; i < n; i++) {
      const w = r.range(0.2, 0.34), yy = y + 0.2 + i * 0.26, a = x + 0.5 + dx * (0.5 - w * 0.75) + (dz ? r.range(-0.25, 0.25) : 0), c = z + 0.5 + dz * (0.5 - w * 0.75) + (dx ? r.range(-0.25, 0.25) : 0), t = r.range(0.9, 1.1);
      box(o, a - w, yy, c - w, a + w, yy + 0.07, c + w, 0.82 * t, 0.68 * t, 0.5 * t, 0, s, l, KIND.FLAT, true);
      box(o, a - w * 0.72, yy + 0.07, c - w * 0.72, a + w * 0.72, yy + 0.12, c + w * 0.72, 0.9 * t, 0.77 * t, 0.6 * t, 0, s, l);
    }
    return;
  }
  if (m === 'fence') {
    const wood = (a, e, c, a1, e1, c1, k = 1) => box(o, a, e, c, a1, e1, c1, 0.46 * k, 0.31 * k, 0.18 * k, 0, s, l, KIND.FLAT);
    wood(x + 0.39, y, z + 0.39, x + 0.61, y + 1, z + 0.61);
    wood(x + 0.35, y + 0.97, z + 0.35, x + 0.65, y + 1.04, z + 0.65, 1.12);
    const con = (b2) => b2 === B.fence || BLOCKS[b2].opaque;
    if (vol[at(x, y - 1, z)] === B.fence || vol[at(x, y + 1, z)] === B.fence && vol[at(x, y + 2, z)] === B.fence) return;
    for (const yy of [0.34, 0.72]) {
      if (con(vol[at(x + 1, y, z)])) wood(x + 0.66, y + yy, z + 0.42, x + 1.34, y + yy + 0.13, z + 0.58, 0.92);
      if (con(vol[at(x, y, z + 1)])) wood(x + 0.42, y + yy, z + 0.66, x + 0.58, y + yy + 0.13, z + 1.34, 0.92);
      for (const dz of [-1, 1]) if (vol[at(x + 1, y, z + dz)] === B.fence && !con(vol[at(x + 1, y, z)]) && !con(vol[at(x, y, z + dz)])) { // liaison en diagonale
        wood(x + 0.61, y + yy, z + 0.42, x + 1.58, y + yy + 0.13, z + 0.58, 0.92);
        wood(x + 1.42, y + yy, Math.min(z + 0.42, z + 0.5 + dz), x + 1.58, y + yy + 0.13, Math.max(z + 0.58, z + 0.5 + dz), 0.92);
      }
    }
    return;
  }
  if (m === 'flagstone') {
    const w = r.range(0.32, 0.46), d = r.range(0.28, 0.44), g = r.range(0.42, 0.56), a = x + 0.5 + r.range(-0.08, 0.08), c = z + 0.5 + r.range(-0.08, 0.08);
    box(o, a - w, y, c - d, a + w, y + 0.13, c + d, g * 1.02, g, g * 0.92, 0, s, l); return;
  }
  if (m === 'lily') {
    const n = lod === 0 ? r.int(1, 2) : 1;
    for (let i = 0; i < n; i++) {
      const a = x + r.range(0.25, 0.75), c = z + r.range(0.25, 0.75), w = r.range(0.2, 0.34), g = r.range(0.85, 1.1);
      box(o, a - w, y - 0.1, c - w, a + w, y - 0.06, c + w, 0.2 * g, 0.5 * g, 0.16 * g, 0, s, l, KIND.FOLIAGE);
    }
    return;
  }
  if (lod === 1) { // version distante : silhouette et couleur seulement
    if (m === 'grass') { const t = r.range(0.75, 1.15), a = r.range(0, 6.28); blade(o, x + 0.5, y, z + 0.5, a, 0.42, r.range(0.55, 0.9), 0.15, 0.42 * t, 0.56 * t, 0.15 * t, s, l, s); blade(o, x + 0.5, y, z + 0.5, a + 1.57, 0.42, r.range(0.5, 0.8), 0.15, 0.45 * t, 0.58 * t, 0.15 * t, s, l, s); return; }
    if (m === 'fern') { blade(o, x + 0.5, y, z + 0.5, r.range(0, 3), 0.5, 0.6, 0.3, 0.23, 0.46, 0.13, s, l, s); blade(o, x + 0.5, y, z + 0.5, r.range(3, 6), 0.5, 0.6, 0.3, 0.26, 0.5, 0.14, s, l, s); return; }
    const col = m === 'lupine' ? [0.44, 0.26, 0.72] : m === 'pink' ? [0.9, 0.42, 0.6] : m === 'yellow' ? [0.95, 0.74, 0.16] : m === 'mushroom' ? [0.75, 0.6, 0.45] : [0.95, 0.93, 0.85];
    const hh = m === 'lupine' ? 0.8 : 0.4;
    box(o, x + 0.34, y + hh * 0.4, z + 0.34, x + 0.66, y + hh, z + 0.66, col[0], col[1], col[2], 40, s, l, KIND.FOLIAGE, false, 70);
    return;
  }
  const cx = x + r.range(0.25, 0.75), cz = z + r.range(0.25, 0.75);
  switch (m) {
    case 'grass': {
      const n = r.int(12, 16), base = r.range(0.82, 1.14), warm = r.next();
      for (let i = 0; i < n; i++) {
        const t = base * r.range(0.86, 1.14);
        blade(o, x + r.next(), y, z + r.next(), r.range(0, 6.28), r.range(0.02, 0.036), r.range(0.22, 0.66), r.range(0.04, 0.2),
          (0.43 + 0.13 * warm) * t, (0.54 + 0.05 * warm) * t, (0.2 + 0.03 * warm) * t, s, l, Math.max(0, s - 25));
      }
      for (let i = 0; i < 4; i++) blade(o, x + r.next(), y, z + r.next(), r.range(0, 6.28), r.range(0.06, 0.1), r.range(0.1, 0.2), 0.1, 0.4 * base, 0.5 * base, 0.2 * base, s, l, Math.max(0, s - 40)); // tapis bas
      return;
    }
    case 'fern': {
      const n = r.int(5, 8), h0 = r.range(0.55, 1), t = r.range(0.85, 1.15), start = r.range(0, 6.28);
      for (let i = 0; i < n; i++) {
        const a = start + (i / n) * 6.28 + r.range(-0.25, 0.25), ca = Math.cos(a), sa = Math.sin(a), L = r.range(0.55, 0.95);
        const seg = 8;
        for (let k = 1; k <= seg; k++) {
          const f = k / seg, dist = L * f, hh = h0 * Math.sin(f * 2.1) * (1 - f * 0.25), lw = 0.15 * (1 - f * 0.86);
          const ax = cx + ca * dist, az = cz + sa * dist, ay = y + hh;
          const f0 = (k - 1) / seg, bx = cx + ca * L * f0, bz = cz + sa * L * f0, by = y + h0 * Math.sin(f0 * 2.1) * (1 - f0 * 0.25);
          const px2 = -sa * lw, pz2 = ca * lw, w = 40 + 170 * f;
          const G = 0.42 * t + 0.05 * (k & 1), R = 0.3 * t, Bl = 0.17 * t;
          for (const side of [-1, 1]) {
            o.v(bx, by, bz, 0, 0, 1, 0, w, 190, s, l, KIND.FOLIAGE, R * 0.8, G * 0.8, Bl);
            o.v(ax, ay, az, 0, 0, 1, 0, w, 190, s, l, KIND.FOLIAGE, R, G, Bl);
            o.v((ax + bx) / 2 + px2 * side * 1.15, (ay + by) / 2 - 0.05, (az + bz) / 2 + pz2 * side * 1.15, 0, 0, 1, 0, w, 255, s, l, KIND.FOLIAGE, R * 1.2, G * 1.2, Bl);
            o.v(bx + px2 * side * 0.8, by - 0.06, bz + pz2 * side * 0.8, 0, 0, 1, 0, w, 255, s, l, KIND.FOLIAGE, R * 1.1, G * 1.1, Bl);
          }
        }
      }
      return;
    }
    case 'daisy': {
      const n = r.int(2, 3);
      for (let i = 0; i < n; i++) {
        const a = x + r.range(0.2, 0.8), c = z + r.range(0.2, 0.8), h = r.range(0.32, 0.62), w = r.range(0.055, 0.075);
        stem(o, a, y, c, h, s, l);
        for (let p = 0; p < 5; p++) { const t = (p / 5) * 6.28 + 0.4, ex = Math.cos(t) * w * 1.7, ez = Math.sin(t) * w * 1.7; box(o, a + ex - w, y + h, c + ez - w, a + ex + w, y + h + 0.03, c + ez + w, 1.25, 1.22, 1.12, 70, s, l, KIND.FOLIAGE); }
        box(o, a - w * 0.8, y + h, c - w * 0.8, a + w * 0.8, y + h + 0.06, c + w * 0.8, 1.3, 0.92, 0.1, 70, s, l, KIND.FOLIAGE);
      }
      return;
    }
    case 'lupine': {
      const n = r.int(1, 3);
      for (let i = 0; i < n; i++) {
        const a = x + r.range(0.25, 0.75), c = z + r.range(0.25, 0.75), H = r.range(0.75, 1.3), tone = r.range(0.85, 1.15);
        stem(o, a, y, c, H * 0.6, s, l);
        const seg = 7;
        for (let k = 0; k < seg; k++) {
          const f = k / seg, w = 0.095 * (1 - f * 0.65), yy = y + H * (0.42 + 0.58 * f), hh = H * 0.075;
          const col = k > seg - 3 ? 1.15 : 1;
          box(o, a - w, yy, c - w, a + w, yy + hh, c + w, 0.5 * tone * col, 0.3 * tone * col, 0.82 * tone * col, 60 + 12 * k, s, l, KIND.FOLIAGE);
        }
        for (let k = 0; k < 3; k++) { const t = r.range(0, 6.28); blade(o, a, y, c, t, 0.045, 0.3, 0.2, 0.3, 0.46, 0.17, s, l, s); }
      }
      return;
    }
    case 'pink': case 'yellow': {
      const col = m === 'pink' ? [1.05, 0.42, 0.62] : [1.35, 0.98, 0.16], n = r.int(3, 5);
      for (let i = 0; i < n; i++) {
        const a = x + r.range(0.15, 0.85), c = z + r.range(0.15, 0.85), h = r.range(0.18, 0.4), w = r.range(0.035, 0.055);
        stem(o, a, y, c, h, s, l);
        box(o, a - w, y + h, c - w, a + w, y + h + 0.07, c + w, col[0], col[1], col[2], 80, s, l, KIND.FOLIAGE);
      }
      blade(o, x + 0.5, y, z + 0.5, r.range(0, 6.3), 0.04, 0.34, 0.15, 0.36, 0.5, 0.18, s, l, s);
      return;
    }
    case 'mushroom': {
      const n = r.int(1, 3);
      for (let i = 0; i < n; i++) {
        const a = x + r.range(0.3, 0.7), c = z + r.range(0.3, 0.7), h = r.range(0.14, 0.36), w = r.range(0.11, 0.2);
        box(o, a - 0.04, y, c - 0.04, a + 0.04, y + h, c + 0.04, 0.9, 0.82, 0.68, 0, s, l);
        box(o, a - w, y + h, c - w, a + w, y + h + 0.1, c + w, 0.78, 0.6, 0.42, 0, s, l);
        box(o, a - w * 0.6, y + h + 0.1, c - w * 0.6, a + w * 0.6, y + h + 0.16, c + w * 0.6, 0.85, 0.68, 0.5, 0, s, l);
      }
    }
  }
}

function leafCubes(o, b, x, y, z, wx, wz, s, l, lod) {
  const layer = BLOCKS[b].faces[0], t = BLOCKS[b].tint;
  const r = new Rng(hashInt(wx, y, wz, 77)), zone = hashInt(wx >> 3, y >> 2, wz >> 3, 7) / 4294967296, lit = 0.74 + 0.4 * (s / 255);
  const n = lod === 0 ? 3 : 1;
  for (let i = 0; i < n; i++) {
    const size = lod === 0 ? r.range(0.5, 0.86) : r.range(1, 1.14);
    const a = x + (lod === 0 ? r.range(0, 1 - size * 0.55) : r.range(-0.1, 0.02)), e = y + (lod === 0 ? r.range(0, 1 - size * 0.55) : r.range(-0.1, 0.02)), c = z + (lod === 0 ? r.range(0, 1 - size * 0.55) : r.range(-0.1, 0.02));
    const k = r.range(0.8, 1.22) * lit, warm = r.range(0.92, 1.1) * (0.92 + 0.34 * zone);
    const R = t[0] * k * warm, G = t[1] * k, B2 = t[2] * k * 0.92;
    const a1 = a + size, e1 = e + size, c1 = c + size;
    const quad = (p, nx, ny, nz, ao) => { for (let j = 0; j < 12; j += 3) o.v(p[j], p[j + 1], p[j + 2], layer, nx, ny, nz, 110, ao, s, l, KIND.LEAF, R, G, B2); };
    quad([a, e1, c1, a1, e1, c1, a1, e1, c, a, e1, c], 0, 1, 0, 255);
    quad([a, e, c1, a1, e, c1, a1, e1, c1, a, e1, c1], 0, 0, 1, 215);
    quad([a1, e, c, a, e, c, a, e1, c, a1, e1, c], 0, 0, -1, 215);
    quad([a1, e, c1, a1, e, c, a1, e1, c, a1, e1, c1], 1, 0, 0, 215);
    quad([a, e, c, a, e, c1, a, e1, c1, a, e1, c], -1, 0, 0, 215);
    quad([a, e, c, a1, e, c, a1, e, c1, a, e, c1], 0, -1, 0, 150);
  }
}

// lod 0 : proche, tout le décor ; lod 1 : moyen, décor simplifié ; lod 2 : loin, blocs seuls.
export function meshChunk(getChunk, cx, cz, lod) {
  const { ymax, top } = fillVolume(getChunk, cx, cz);
  computeLight(top, ymax);
  // Bas de la zone utile : première couche contenant autre chose que du plein (grottes et galeries comprises).
  let ymin = 1;
  scan: for (; ymin < ymax; ymin++) for (let z = -1; z <= CX; z++) for (let x = -1; x <= CX; x++) if (!IS_OPAQUE[vol[at(x, ymin, z)]]) break scan;
  ymin = Math.max(1, ymin - 1);
  const opaque = new VBuf(40000), deco = new VBuf(lod === 2 ? 16 : 60000), water = new VBuf(4000);
  greedy(opaque, ymin, Math.min(ymax, CH - 1), lod);
  for (let y = ymin; y <= ymax; y++) for (let z = 0; z < CX; z++) for (let x = 0; x < CX; x++) {
    const k = at(x, y, z), b = vol[k];
    if (b === 0 || IS_OPAQUE[b]) continue;
    if (b === B.water) {
      const up = vol[k + WW], topY = up === B.water ? y + 1 : y + 0.875;
      const s = sky[k + WW] * 17, l = blk[k] * 17;
      if (up !== B.water && !IS_OPAQUE[up]) { water.v(x, topY, z + 1, 0, 0, 1, 0, 0, 255, s, l, 0, 1, 1, 1); water.v(x + 1, topY, z + 1, 0, 0, 1, 0, 0, 255, s, l, 0, 1, 1, 1); water.v(x + 1, topY, z, 0, 0, 1, 0, 0, 255, s, l, 0, 1, 1, 1); water.v(x, topY, z, 0, 0, 1, 0, 0, 255, s, l, 0, 1, 1, 1); }
      const side = (n, nx, nz, ax, az, bx, bz) => {
        const v = vol[n]; if (v === B.water || IS_OPAQUE[v]) return;
        const ss = Math.max(sky[n], sky[k]) * 17;
        water.v(ax, y, az, 0, nx, 0, nz, 0, 255, ss, l, 0, 1, 1, 1); water.v(bx, y, bz, 0, nx, 0, nz, 0, 255, ss, l, 0, 1, 1, 1);
        water.v(bx, topY, bz, 0, nx, 0, nz, 0, 255, ss, l, 0, 1, 1, 1); water.v(ax, topY, az, 0, nx, 0, nz, 0, 255, ss, l, 0, 1, 1, 1);
      };
      side(k + 1, 1, 0, x + 1, z + 1, x + 1, z); side(k - 1, -1, 0, x, z, x, z + 1); side(k + W, 0, 1, x, z + 1, x + 1, z + 1); side(k - W, 0, -1, x + 1, z, x, z);
    } else if (IS_LEAF[b] && lod < 2) {
      const exposed = !IS_SOLID[vol[k + 1]] || !IS_SOLID[vol[k - 1]] || !IS_SOLID[vol[k + W]] || !IS_SOLID[vol[k - W]] || !IS_SOLID[vol[k + WW]] || !IS_SOLID[vol[k - WW]];
      if (exposed) leafCubes(opaque, b, x, y, z, cx * CX + x, cz * CX + z, sky[k] * 17, blk[k] * 17, lod);
    } else if (IS_DECO[b] && (lod < 2 || EMIT[b] || b === B.fence)) {
      decoModel(deco, b, x, y, z, cx * CX + x, cz * CX + z, sky[k] * 17, Math.max(blk[k], EMIT[b]) * 17, lod);
    }
  }
  return { opaque: opaque.take(), deco: deco.take(), water: water.take(), nO: opaque.n, nD: deco.n, nW: water.n, ymin, ymax: ymax + 2 };
}

// Anneau de relief lointain : colonnes grossières pour donner un horizon au-delà des chunks.
export function farMesh(gen, ccx, ccz, cells = 110, step = 16) {
  const o = new VBuf(cells * cells * 4); o.scale = 8;
  const N = cells + 1, hs = new Float32Array(N * N), cols = new Float32Array(N * N * 3), ox = ccx * CX - (cells / 2) * step, oz = ccz * CX - (cells / 2) * step;
  for (let j = 0; j < N; j++) for (let i = 0; i < N; i++) {
    const wx = ox + i * step, wz = oz + j * step, c = gen.farSample(wx, wz), k = i + j * N;
    const canopy = c.water ? 0 : c.forest > 0.35 ? 13 * Math.min(1, (c.forest - 0.35) * 3) : 0;
    hs[k] = (c.water ? c.water : c.h) + canopy;
    const t = (gen.nDeco.n2(wx / 40, wz / 40) + 1) * 0.5;
    if (c.water) { cols[k * 3] = 0.22; cols[k * 3 + 1] = 0.36; cols[k * 3 + 2] = 0.42; }
    else if (canopy > 3) { cols[k * 3] = 0.2 + 0.1 * t; cols[k * 3 + 1] = 0.34 + 0.1 * t; cols[k * 3 + 2] = 0.09; }
    else { cols[k * 3] = 0.42 + 0.1 * t; cols[k * 3 + 1] = 0.5 + 0.05 * t; cols[k * 3 + 2] = 0.2; }
  }
  for (let j = 0; j < cells; j++) for (let i = 0; i < cells; i++) {
    const k = i + j * N, a = hs[k], b = hs[k + 1], c = hs[k + N + 1], d = hs[k + N];
    let nx = a - b + d - c, nz = a - d + b - c, ny = 2 * step;
    const l = Math.hypot(nx, ny, nz); nx /= l; ny /= l; nz /= l;
    const x0 = (i - cells / 2) * step, z0 = (j - cells / 2) * step, x1 = x0 + step, z1 = z0 + step;
    const r = cols[k * 3], g = cols[k * 3 + 1], bl = cols[k * 3 + 2];
    o.v(x0, d, z1, 0, nx, ny, nz, 0, 255, 255, 0, KIND.FLAT, r, g, bl); o.v(x1, c, z1, 0, nx, ny, nz, 0, 255, 255, 0, KIND.FLAT, r, g, bl);
    o.v(x1, b, z0, 0, nx, ny, nz, 0, 255, 255, 0, KIND.FLAT, r, g, bl); o.v(x0, a, z0, 0, nx, ny, nz, 0, 255, 255, 0, KIND.FLAT, r, g, bl);
  }
  return { far: o.take(), nF: o.n };
}
