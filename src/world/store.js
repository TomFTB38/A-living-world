// VoxelWorld/store.js — voxels générés + modifications du joueur (delta). La génération n'écrase jamais un delta.
import { CA } from './blocks.js';
import { WorldGen } from './gen.js';
import { meshChunk, farMesh } from './mesher.js';

export const chunkKey = (cx, cz) => cx + ',' + cz;

export class ChunkStore {
  constructor(seed, limit = 90) { this.gen = new WorldGen(seed); this.cache = new Map(); this.edits = new Map(); this.limit = limit; }
  setEdits(list) { this.edits = new Map(list.map(([k, e]) => [k, new Map(e)])); this.cache.clear(); }
  edit(cx, cz, i, id) {
    const k = chunkKey(cx, cz);
    if (!this.edits.has(k)) this.edits.set(k, new Map());
    this.edits.get(k).set(i, id);
    const c = this.cache.get(k); if (c) c[i] = id;
  }
  get(cx, cz) {
    const k = chunkKey(cx, cz);
    let c = this.cache.get(k);
    if (c) { this.cache.delete(k); this.cache.set(k, c); return c; }
    c = this.gen.genChunk(cx, cz);
    const e = this.edits.get(k);
    if (e) for (const [i, id] of e) c[i] = id;
    this.cache.set(k, c);
    if (this.cache.size > this.limit) this.cache.delete(this.cache.keys().next().value);
    return c;
  }
}

// Partagé entre les workers et le mode de secours sur le fil principal.
export class WorldService {
  constructor(post) { this.post = post; this.store = null; }
  handle(m) {
    switch (m.t) {
      case 'init': this.store = new ChunkStore(m.seed); this.store.setEdits(m.edits || []); break;
      case 'edit': this.store.edit(m.cx, m.cz, m.i, m.id); break;
      case 'mesh': {
        const t0 = performance.now(), s = this.store;
        const r = meshChunk((x, z) => s.get(x, z), m.cx, m.cz, m.lod);
        const voxels = m.voxels ? s.get(m.cx, m.cz).slice() : null;
        const out = { t: 'mesh', cx: m.cx, cz: m.cz, lod: m.lod, ver: m.ver, ms: performance.now() - t0, voxels, ...r };
        const tr = [r.opaque, r.deco, r.water]; if (voxels) tr.push(voxels.buffer);
        this.post(out, tr); break;
      }
      case 'far': { const r = farMesh(this.store.gen, m.cx, m.cz); this.post({ t: 'far', cx: m.cx, cz: m.cz, ...r }, [r.far]); break; }
    }
  }
}
void CA;
