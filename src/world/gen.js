// ProceduralGeneration/gen.js — monde déterministe, fonction pure de (seed, x, y, z).
import { Noise, Rng, hashInt, hashFloat, clamp, lerp, smooth } from '../core/noise.js';
import { CX, CH, CA, idx, B, IS_SOLID, IS_DECO, orientedBlock } from './blocks.js';

export const GEN_VERSION = 1;
const TREE_CELL = 13, ROCK_CELL = 19, POND_CELL = 96, STAMP_REACH = 38;

export class WorldGen {
  constructor(seed) {
    this.seed = seed | 0;
    const s = this.seed;
    this.nBase = new Noise(s + 1); this.nHill = new Noise(s + 2); this.nWarp = new Noise(s + 3);
    this.nRiver = new Noise(s + 4); this.nPath = new Noise(s + 5); this.nForest = new Noise(s + 6);
    this.nCliff = new Noise(s + 7); this.nCave = new Noise(s + 8); this.nDeco = new Noise(s + 9);
    this.stamps = new Map();
    this.c = { h: 0, water: 0, kind: 0, forest: 0, path: 0, bank: 0, cliff: 0 };
  }

  pondAt(gx, gz) {
    const s = this.seed;
    if (hashFloat(gx, gz, s, 71) > 0.42) return null;
    const x = (gx + 0.2 + 0.6 * hashFloat(gx, gz, s, 72)) * POND_CELL, z = (gz + 0.2 + 0.6 * hashFloat(gx, gz, s, 73)) * POND_CELL;
    return { x, z, r: 7 + 8 * hashFloat(gx, gz, s, 74), level: Math.floor(this.rawHeight(x, z).hs) - 1 };
  }

  rawHeight(x, z) {
    const wx = x + 34 * this.nWarp.n2(x / 170, z / 170), wz = z + 34 * this.nWarp.n2(x / 170 + 40, z / 170 + 40);
    const low = 52 + 15 * this.nBase.fbm2(wx / 340, wz / 340, 3);
    const hs = low + 5.5 * this.nHill.fbm2(wx / 96, wz / 96, 3);
    return { wx, wz, low, hs };
  }

  // Remplit this.c pour la colonne (x,z) : altitude, eau, chemin, densité forestière.
  column(x, z) {
    const c = this.c, { wx, wz, low, hs } = this.rawHeight(x, z);
    // terrasses : les ressauts de 3-5 blocs qui structurent la référence
    const stepN = this.nHill.n2(x / 52 + 11, z / 52 - 7), S = 4;
    const q = hs / S, fl = Math.floor(q), fr = q - fl;
    const terr = (fl + smooth(0.32, 0.68, fr)) * S;
    let h = lerp(hs, terr, smooth(-0.35, 0.25, stepN));
    h += 1.6 * this.nHill.n2(x / 19, z / 19);

    // rivière : ligne de niveau du bruit déformé, niveaux quantifiés = cascades
    const rs = this.nRiver.fbm2(wx / 400, wz / 400, 2, 2.1, 0.32), rn = Math.abs(rs);
    const dR = rn * 300, wid = 2.6 + 1.8 * (0.5 + 0.5 * this.nRiver.n2(x / 60, z / 60));
    const L = Math.floor((low - 4) / 4) * 4 + 1; // paliers de 4 blocs : de vraies cascades
    let water = 0, bank = 0, still = 0;
    if (dR < wid + 13) {
      bank = 1 - smooth(wid, wid + 13, dR);
      const target = L + 1 + 3 * smooth(wid, wid + 8, dR);
      if (h > target) h = lerp(h, target, smooth(0, 1, bank * 1.25));
      if (dR < wid) { h = L - 2 - (1 - dR / wid) * 1.6; water = L; } else if (h < L + 1) h = L + 1;
    }
    // étangs
    const gx = Math.floor(x / POND_CELL), gz = Math.floor(z / POND_CELL);
    for (let i = -1; i <= 1; i++) for (let j = -1; j <= 1; j++) {
      const p = this.pondAt(gx + i, gz + j);
      if (!p) continue;
      const d = Math.hypot(x - p.x, z - p.z) + 3 * this.nDeco.n2(x / 9, z / 9);
      if (d > p.r * 2) continue;
      const bowl = lerp(p.level - 3, h, smooth(p.r * 0.7, p.r * 1.8, d));
      if (bowl < h) h = bowl;
      if (d < p.r * 1.05 && h < p.level && p.level > water) { water = p.level; still = 1; }
    }
    // chemin
    const pn = Math.abs(this.nPath.n2(wx / 210 + 9, wz / 210 + 3)) * 165;
    let path = 0;
    // Deux réseaux : les chemins de traverse, et un sentier qui longe chaque ruisseau sur une rive.
    const bankD = Math.abs(rs * 300 - (wid + 6)), wob = this.nDeco.n2(x / 5, z / 5) * 0.6;
    if (!water) {
      const edge = 1 - smooth(0.9, 2.3, bankD + wob);
      if (bank < 0.45) {
        path = 1 - smooth(0.9, 2.3, pn + wob);
        const near = 1 - smooth(1.5, 8, pn);
        if (near > 0) h = lerp(h, hs, near * 0.82);
      }
      if (edge > path) path = edge;
      const walk = 1 - smooth(1.5, 5, bankD);
      if (walk > 0) h = lerp(h, low - 1, walk); // niveau continu : le sentier reste praticable au passage des chutes
    }
    const f = this.nForest.fbm2(x / 170, z / 170, 3) * 1.5 + 0.78 + 0.12 * this.nForest.n2(x / 31, z / 31);
    c.h = h; c.water = water; c.still = still; c.forest = clamp(f, 0, 1); c.path = path; c.bank = bank;
    c.cliff = smooth(0.1, 0.55, this.nCliff.n2(x / 120, z / 120));
    return c;
  }

  stoneAt(x, y, z) {
    const b = (y + 5 * this.nCliff.n2(x / 70, z / 70)) / 7;
    const k = ((Math.floor(b) % 3) + 3) % 3;
    return k === 2 ? B.limestone : k === 1 ? B.granite : B.stone;
  }

  isCave(x, y, z) {
    const a = this.nCave.n3(x / 30, y / 18, z / 30), b = this.nCave.n3(x / 30 + 50, y / 18 - 20, z / 30 + 90);
    return a * a + b * b < 0.0065;
  }

  // Terrain + décor + empreintes, sans aucune dépendance à l'ordre de chargement.
  genChunk(cx, cz) {
    const v = new Uint8Array(CA), ox = cx * CX, oz = cz * CX;
    const heights = new Int16Array(CX * CX), waters = new Int16Array(CX * CX);
    const info = new Float32Array(CX * CX * 2);
    for (let z = 0; z < CX; z++) for (let x = 0; x < CX; x++) {
      const wx = ox + x, wz = oz + z, c = this.column(wx, wz);
      const hf = c.h, cliff = c.cliff, fr = c.forest, path = c.path, water = c.water;
      let top = 0;
      const ymax = Math.min(CH - 2, Math.ceil(hf + 7));
      for (let y = 0; y <= ymax; y++) {
        let d = hf - y;
        if (cliff > 0.01 && y > hf - 12) d += cliff * 6 * (1 - clamp((y - hf) / 8, 0, 1)) * this.nCave.n3(wx / 25, y / 15, wz / 25);
        if (d > 0) {
          if (y > 2 && y < hf - 3 && this.isCave(wx, y, wz)) continue;
          v[idx(x, y, z)] = B.stone; top = y;
        }
      }
      // habillage : grès de strates, sous-couche, surface
      let depth = 0;
      for (let y = top; y >= 0; y--) {
        const i = idx(x, y, z);
        if (!v[i]) { depth = 0; continue; }
        const steepCut = depth > (path > 0.3 ? 4 : 2 + ((wx ^ wz) & 1));
        if (depth === 0) {
          if (path > 0.45) v[i] = B.path;
          else if (water && y < water + 1) v[i] = this.nDeco.n2(wx / 7, wz / 7) > 0 ? B.gravel : B.sand;
          else if (c.bank > 0.72 && y <= water + 2) v[i] = B.gravel;
          else if (fr > 0.72 && this.nDeco.n2(wx / 13, wz / 13) > 0.25) v[i] = B.forest_soil;
          else if (cliff > 0.45 && this.nDeco.n2(wx / 11, wz / 11) > 0.35) v[i] = B.mossy_stone;
          else v[i] = B.grass;
        } else if (steepCut) v[i] = this.stoneAt(wx, y, wz);
        else v[i] = B.dirt;
        depth++;
      }
      if (water) {
        for (let y = top + 1; y <= water; y++) v[idx(x, y, z)] = B.water;
        // Rochers qui affleurent dans le lit, posés en groupes
        if (this.nDeco.n2(wx / 4.3 + 31, wz / 4.3 - 12) > 0.56) { const crown = water + (hashFloat(wx, wz, this.seed, 9) > 0.55 ? 1 : 0); for (let y = top + 1; y <= crown && y < CH - 1; y++) v[idx(x, y, z)] = y === crown ? B.mossy_stone : B.stone; top = crown; }
      }
      heights[x + z * CX] = top; waters[x + z * CX] = water;
      info[(x + z * CX) * 2] = fr; info[(x + z * CX) * 2 + 1] = path;
    }
    this.stampAll(v, cx, cz, heights);
    this.scatterDeco(v, ox, oz, heights, waters, info);
    this.hangVines(v, ox, oz);
    return v;
  }

  scatterDeco(v, ox, oz, heights, waters, info) {
    const s = this.seed, nd = this.nDeco;
    for (let z = 0; z < CX; z++) for (let x = 0; x < CX; x++) {
      const y = heights[x + z * CX] + 1;
      if (y >= CH - 1) continue;
      const wx = ox + x, wz = oz + z, ground = v[idx(x, y - 1, z)], here = v[idx(x, y, z)];
      if (waters[x + z * CX]) {
        if (here === B.water) { // nénuphars sur eaux calmes
          const wy = waters[x + z * CX];
          if (nd.n2(wx / 8, wz / 8) > 0.42 && hashFloat(wx, wz, s, 5) < 0.45 && v[idx(x, wy + 1, z)] === 0) v[idx(x, wy + 1, z)] = B.lily_pad;
        }
        continue;
      }
      if (here !== 0) continue;
      const r = hashFloat(wx, wz, s, 3), fr = info[(x + z * CX) * 2], path = info[(x + z * CX) * 2 + 1];
      if (ground === B.path) { if (r < 0.055) v[idx(x, y, z)] = B.flagstone; continue; }
      if (ground !== B.grass && ground !== B.forest_soil && ground !== B.mossy_stone) continue;
      if (path > 0.02 && path < 0.4 && nd.n2(wx / 40 - 9, wz / 40 + 5) > 0.2) {
        v[idx(x, y, z)] = B.fence;
        if (hashFloat(wx, wz, s, 8) < 0.07) { v[idx(x, y + 1, z)] = B.fence; v[idx(x, y + 2, z)] = B.fence; v[idx(x, y + 3, z)] = B.lantern; }
        continue;
      }
      const patch = nd.n2(wx / 23, wz / 23), patch2 = nd.n2(wx / 17 + 60, wz / 17 - 30);
      const open = 1 - fr;
      let id = 0;
      const flowerP = 0.14 + 0.62 * smooth(0.05, 0.6, patch) * (0.65 + open * 0.5);
      if (r < flowerP) {
        const t = hashFloat(wx, wz, s, 4);
        if (patch2 > 0.3) id = t < 0.55 ? B.lupine : B.pink_flower;
        else if (patch2 > -0.15) id = t < 0.75 ? B.daisy : B.yellow_flower;
        else id = t < 0.5 ? B.yellow_flower : B.daisy;
        if (hashFloat(wx, wz, s, 6) > 0.55) id = B.tall_grass;
      } else if (r < flowerP + 0.1 + 0.24 * fr * smooth(-0.2, 0.3, patch2)) id = B.fern;
      else if (r < 0.985) id = B.tall_grass;
      if (fr > 0.6 && hashFloat(wx, wz, s, 7) < 0.012) id = B.mushroom;
      if (ground === B.mossy_stone && r > 0.5) id = 0;
      if (path > 0.05) id = r < 0.5 ? B.tall_grass : 0;
      v[idx(x, y, z)] = id;
    }
  }

  // Lianes sous les couronnes : dépendent seulement des voxels déjà déterminés de ce chunk.
  hangVines(v, ox, oz) {
    const s = this.seed;
    for (let z = 0; z < CX; z++) for (let x = 0; x < CX; x++) {
      const wx = ox + x, wz = oz + z;
      if (hashFloat(wx, wz, s, 11) > 0.075) continue;
      for (let y = CH - 2; y > 4; y--) {
        const b = v[idx(x, y, z)];
        if (b >= B.oak_leaves && b <= B.beech_leaves && v[idx(x, y - 1, z)] === 0) {
          const n = 2 + Math.floor(hashFloat(wx, wz, s, 12) * 5);
          for (let k = 1; k <= n && v[idx(x, y - k, z)] === 0; k++) v[idx(x, y - k, z)] = B.vine;
          break;
        }
      }
    }
  }

  // Les arbres et rochers sont calculés par cellule de grille puis estampés dans chaque chunk touché.
  stampAll(v, cx, cz, heights) {
    const x0 = cx * CX - STAMP_REACH, z0 = cz * CX - STAMP_REACH, x1 = cx * CX + CX + STAMP_REACH, z1 = cz * CX + CX + STAMP_REACH;
    const ox = cx * CX, oz = cz * CX;
    const apply = (st) => {
      const d = st.data;
      for (let i = 0; i < d.length; i += 4) {
        const x = d[i] - ox, y = d[i + 1], z = d[i + 2] - oz, id = d[i + 3];
        if (x < 0 || z < 0 || x >= CX || z >= CX || y < 0 || y >= CH) continue;
        const k = idx(x, y, z), cur = v[k];
        if (st.hard ? (cur === 0 || cur === B.water) : cur === 0) v[k] = id;
        else if (id !== 0 && !st.hard && (st.force || IS_DECO[cur])) v[k] = id;
        if (st.force && IS_SOLID[id] && y > heights[x + z * CX]) heights[x + z * CX] = y;
      }
    };
    for (let gz = Math.floor(z0 / ROCK_CELL); gz <= Math.floor(z1 / ROCK_CELL); gz++)
      for (let gx = Math.floor(x0 / ROCK_CELL); gx <= Math.floor(x1 / ROCK_CELL); gx++) { const st = this.rockStamp(gx, gz); if (st) apply(st); }
    const trees = [];
    for (let gz = Math.floor(z0 / TREE_CELL); gz <= Math.floor(z1 / TREE_CELL); gz++)
      for (let gx = Math.floor(x0 / TREE_CELL); gx <= Math.floor(x1 / TREE_CELL); gx++) { const st = this.treeStamp(gx, gz); if (st) trees.push(st); }
    // Ordre : bois d'abord pour que les troncs traversent les feuillages voisins, ordre stable.
    for (const st of trees) apply(st);
  }

  cached(key, make) {
    if (this.stamps.has(key)) return this.stamps.get(key);
    const st = make();
    this.stamps.set(key, st);
    if (this.stamps.size > 2600) this.stamps.delete(this.stamps.keys().next().value);
    return st;
  }

  rockStamp(gx, gz) {
    return this.cached('r' + gx + ',' + gz, () => {
      const s = this.seed;
      if (hashFloat(gx, gz, s, 21) > 0.62) return null;
      const x = Math.floor((gx + hashFloat(gx, gz, s, 22)) * ROCK_CELL), z = Math.floor((gz + hashFloat(gx, gz, s, 23)) * ROCK_CELL);
      const c = this.column(x, z);
      if (c.water || c.path > 0.1) return null;
      const r = new Rng(hashInt(gx, gz, s, 24)), data = [];
      const n = r.int(1, 3), base = Math.floor(c.h);
      for (let k = 0; k < n; k++) {
        const rx = r.range(1.6, 4.2), ry = r.range(1.3, 3), rz = r.range(1.6, 4.2);
        const ax = x + (k ? r.range(-4, 4) : 0), az = z + (k ? r.range(-4, 4) : 0);
        for (let dy = -2; dy <= ry + 1; dy++) for (let dz = -6; dz <= 6; dz++) for (let dx = -6; dx <= 6; dx++) {
          const nn = this.nDeco.n2((ax + dx) / 3.1, (az + dz + dy * 3) / 3.1) * 0.28;
          const e = (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry) + (dz * dz) / (rz * rz);
          if (e + nn < 1) {
            const e2 = (dx * dx) / (rx * rx) + ((dy + 1) * (dy + 1)) / (ry * ry) + (dz * dz) / (rz * rz) + nn;
            const top = e2 >= 1;
            data.push(Math.round(ax + dx), base + dy, Math.round(az + dz), top && r.next() < 0.75 ? B.mossy_stone : this.stoneAt(ax, base + dy, az));
          }
        }
      }
      return { data: Int32Array.from(data), hard: false, force: true };
    });
  }

  treeStamp(gx, gz) {
    return this.cached('t' + gx + ',' + gz, () => {
      const s = this.seed;
      const x = Math.floor((gx + 0.15 + 0.7 * hashFloat(gx, gz, s, 31)) * TREE_CELL), z = Math.floor((gz + 0.15 + 0.7 * hashFloat(gx, gz, s, 32)) * TREE_CELL);
      const c = this.column(x, z), forest = c.forest;
      if (c.water || c.path > 0 || c.bank > 0.4) return null;
      const pick = hashFloat(gx, gz, s, 33);
      const glade = smooth(-0.1, 0.45, this.nForest.n2(x / 46 + 70, z / 46 - 40));
      if (pick > (0.08 + 0.56 * smooth(0.25, 0.55, forest)) * (1 - 0.85 * glade)) return null;
      const r = new Rng(hashInt(gx, gz, s, 34));
      const sp = this.nForest.n2(gx / 5.1 + 3, gz / 5.1 - 8), roll = r.next();
      const y = Math.floor(c.h);
      const age = r.next();
      if (roll < 0.34 + sp * 0.3) return growBirch(x, y, z, r, age);
      if (roll < 0.52) return growBeech(x, y, z, r, age);
      return growOak(x, y, z, r, age * age * age);
    });
  }

  // Échantillon de relief pour les horizons lointains.
  farSample(x, z) {
    const c = this.column(x, z);
    return { h: c.h, water: c.water, forest: c.forest };
  }
}

/* ---- Croissance des arbres : troncs, branches primaires et secondaires, couronne, racines. ---- */
class Grower {
  constructor() { this.wood = new Map(); this.leaf = new Map(); this.extra = []; }
  put(m, x, y, z, id) { m.set(x + ',' + y + ',' + z, id); }
  sphere(m, cx, cy, cz, r, id, sq = 1) {
    const R = Math.ceil(r + 1);
    for (let dy = -R; dy <= R; dy++) for (let dz = -R; dz <= R; dz++) for (let dx = -R; dx <= R; dx++) {
      const yy = dy / sq;
      const xx = Math.round(cx) + dx - cx, zz = Math.round(cz) + dz - cz, y2 = Math.round(cy) + dy - cy;
      if (xx * xx + y2 * y2 / (sq * sq) + zz * zz <= r * r) this.put(m, Math.round(cx) + dx, Math.round(cy) + dy, Math.round(cz) + dz, id);
      void yy;
    }
  }
  limb(p, dir, len, r0, r1, id, rng, curl = 0.12, up = 0.04) {
    const n = Math.ceil(len), pts = [];
    let [x, y, z] = p, [dx, dy, dz] = dir;
    for (let i = 0; i <= n; i++) {
      const t = i / n;
      this.sphere(this.wood, x, y, z, lerp(r0, r1, t), orientedBlock(id, dx, dy, dz));
      pts.push([x, y, z, dx, dy, dz]);
      dx += rng.range(-curl, curl); dz += rng.range(-curl, curl); dy += rng.range(-curl * 0.5, curl * 0.5) + up;
      const l = Math.hypot(dx, dy, dz); dx /= l; dy /= l; dz /= l;
      x += dx; y += dy; z += dz;
    }
    return pts;
  }
  blob(cx, cy, cz, rx, ry, id, rng) {
    const X = Math.ceil(rx + 1), Y = Math.ceil(ry + 1), seed = rng.next() * 100;
    for (let dy = -Y; dy <= Y; dy++) for (let dz = -X; dz <= X; dz++) for (let dx = -X; dx <= X; dx++) {
      const e = (dx * dx + dz * dz) / (rx * rx) + (dy * dy) / (ry * ry);
      if (e > 1.15) continue;
      const wob = Math.sin(dx * 1.3 + seed) * Math.sin(dy * 1.7 + seed * 2) * Math.cos(dz * 1.1 - seed) * 0.5;
      if (e + wob < 0.95) this.put(this.leaf, Math.round(cx) + dx, Math.round(cy) + dy, Math.round(cz) + dz, id);
    }
  }
  finish() {
    const out = [];
    for (const [k, id] of this.leaf) { if (!this.wood.has(k)) { const a = k.split(','); out.push(+a[0], +a[1], +a[2], id); } }
    for (const [k, id] of this.wood) { const a = k.split(','); out.push(+a[0], +a[1], +a[2], id); }
    out.push(...this.extra);
    return { data: Int32Array.from(out), hard: false, force: false };
  }
}

function growOak(x, y, z, r, age) {
  const g = new Grower(), log = B.oak_log, lf = B.oak_leaves;
  const rad = lerp(1.1, 3.1, age), H = lerp(10, 20, age);
  const lean = [r.range(-0.1, 0.1), 1, r.range(-0.1, 0.1)];
  const trunk = g.limb([x, y - 1, z], lean, H, rad, rad * 0.72, log, r, 0.05, 0.03);
  // contreforts et racines apparentes
  const roots = 3 + Math.floor(age * 3);
  for (let i = 0; i < roots; i++) {
    const a = (i / roots) * 6.283 + r.range(-0.3, 0.3);
    g.limb([x + Math.cos(a) * rad * 0.6, y + rad * 0.9, z + Math.sin(a) * rad * 0.6], [Math.cos(a), -0.45, Math.sin(a)], rad * 2 + 2.5, rad * 0.42, 0.42, log, r, 0.14, -0.01);
  }
  const top = trunk[trunk.length - 1], arms = 3 + Math.floor(age * 3);
  for (let i = 0; i < arms; i++) {
    const a = (i / arms) * 6.283 + r.range(-0.4, 0.4), at = trunk[Math.floor(trunk.length * r.range(0.68, 0.98))];
    const L = lerp(5, 13, age) * r.range(0.8, 1.15);
    const pts = g.limb(at, [Math.cos(a), r.range(0.3, 0.7), Math.sin(a)], L, rad * 0.55, 0.65, log, r, 0.14, 0.03);
    const tip = pts[pts.length - 1];
    g.blob(tip[0], tip[1] + 1.5, tip[2], lerp(3.2, 5.2, age) * r.range(0.85, 1.15), lerp(2.6, 3.9, age), lf, r);
    for (let b = 0; b < 2 + age * 2; b++) {
      const q = pts[Math.floor(pts.length * r.range(0.45, 0.95))];
      const a2 = a + r.range(-1.2, 1.2);
      const pts2 = g.limb(q, [Math.cos(a2), r.range(0, 0.5), Math.sin(a2)], L * 0.5, 0.75, 0.4, log, r, 0.2, 0.04);
      const t2 = pts2[pts2.length - 1];
      g.blob(t2[0], t2[1] + 1, t2[2], lerp(2.6, 4, age) * r.range(0.8, 1.1), lerp(2.2, 3.2, age), lf, r);
    }
  }
  g.blob(top[0], top[1] + 3, top[2], lerp(4, 6.5, age), lerp(3, 4.4, age), lf, r);
  // Polypores : on part du cœur du tronc jusqu'à la première case libre, donc toujours collés à l'écorce.
  const fungi = Math.floor(age * 5) + (r.next() < 0.5 ? 1 : 0), DIR = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  for (let i = 0; i < fungi; i++) {
    const q = trunk[Math.floor(trunk.length * r.range(0.08, 0.5))], [dx, dz] = DIR[r.int(0, 3)], yy = Math.round(q[1]);
    let cx = Math.round(q[0]), cz = Math.round(q[2]), steps = 0;
    if (!g.wood.has(cx + ',' + yy + ',' + cz)) continue;
    while (g.wood.has(cx + ',' + yy + ',' + cz) && steps++ < 12) { cx += dx; cz += dz; }
    g.extra.push(cx, yy, cz, B.shelf_fungus);
  }
  return g.finish();
}

function growBirch(x, y, z, r, age) {
  const g = new Grower(), log = B.birch_log, lf = B.birch_leaves;
  const H = lerp(11, 22, age), rad = lerp(0.55, 1.15, age);
  const trunk = g.limb([x, y - 1, z], [r.range(-0.06, 0.06), 1, r.range(-0.06, 0.06)], H, rad, 0.42, log, r, 0.05, 0.02);
  const n = 5 + Math.floor(age * 5);
  for (let i = 0; i < n; i++) {
    const t = r.range(0.48, 0.98), p = trunk[Math.floor(trunk.length * t)], a = r.range(0, 6.283);
    const L = lerp(2, 6, 1 - t * 0.7) * lerp(0.7, 1.1, age);
    const pts = g.limb(p, [Math.cos(a), 0.55, Math.sin(a)], L, 0.45, 0.4, log, r, 0.15, 0.02);
    const tip = pts[pts.length - 1];
    g.blob(tip[0], tip[1] + 0.5, tip[2], r.range(2, 3.1), r.range(1.8, 2.8), lf, r);
  }
  const top = trunk[trunk.length - 1];
  g.blob(top[0], top[1] + 1, top[2], 2.6, 3.2, lf, r);
  return g.finish();
}

function growBeech(x, y, z, r, age) {
  const g = new Grower(), log = B.beech_log, lf = B.beech_leaves;
  const H = lerp(12, 24, age), rad = lerp(0.8, 1.9, age);
  const trunk = g.limb([x, y - 1, z], [r.range(-0.05, 0.05), 1, r.range(-0.05, 0.05)], H, rad, 0.7, log, r, 0.04, 0.02);
  const n = 5 + Math.floor(age * 4);
  for (let i = 0; i < n; i++) {
    const t = r.range(0.55, 1), p = trunk[Math.min(trunk.length - 1, Math.floor(trunk.length * t))], a = (i / n) * 6.283 + r.range(-0.5, 0.5);
    const L = lerp(3, 9, age) * r.range(0.7, 1.1);
    const pts = g.limb(p, [Math.cos(a), 0.65, Math.sin(a)], L, rad * 0.45, 0.45, log, r, 0.1, 0.04);
    const tip = pts[pts.length - 1];
    g.blob(tip[0], tip[1] + 1, tip[2], lerp(3.2, 5, age), lerp(2.6, 4, age), lf, r);
  }
  return g.finish();
}
