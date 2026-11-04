// VoxelWorld/client.js — streaming : quels chunks charger, à quel niveau de détail, et accès aux voxels.
import { CX, CH, idx, IS_SOLID, B } from '../world/blocks.js';
import { WorldService, chunkKey } from '../world/store.js';

const WORKER_SRC = '__WORKER_SOURCE__';

export class ClientWorld {
  constructor(renderer, seed, edits, opts) {
    this.r = renderer; this.seed = seed; this.chunks = new Map(); this.edits = edits; this.radius = opts.radius;
    this.pending = 0; this.queue = []; this.stats = { genMs: 0, meshed: 0, loaded: 0 }; this.far = null; this.farBusy = false; this.farKey = null;
    this.workers = []; this.startWorkers(opts.workers);
  }
  startWorkers(n) {
    const init = { t: 'init', seed: this.seed, edits: [...this.edits].map(([k, m]) => [k, [...m]]) };
    const onmsg = (m) => this.onMessage(m);
    try {
      if (typeof Worker === 'undefined' || WORKER_SRC.startsWith('__')) throw new Error('pas de worker');
      const url = URL.createObjectURL(new Blob([WORKER_SRC], { type: 'text/javascript' }));
      for (let i = 0; i < n; i++) {
        const w = new Worker(url); w.onmessage = (e) => onmsg(e.data); w.onerror = () => this.fallback(init, onmsg);
        w.postMessage(init); this.workers.push(w);
      }
    } catch (e) { this.fallback(init, onmsg); }
  }
  // Secours si la politique du site interdit les workers : tout se fait par tranches sur le fil principal.
  fallback(init, onmsg) {
    if (this.local) return;
    for (const w of this.workers) w.terminate();
    this.workers = []; this.pending = 0;
    for (const c of this.chunks.values()) c.busy = false;
    this.local = new WorldService((m) => onmsg(m)); this.local.handle(init); this.localQ = [];
  }
  send(m, slot = 0) { if (this.local) this.localQ.push(m); else this.workers[slot % this.workers.length].postMessage(m); }
  tickLocal() { if (this.local && this.localQ.length) this.local.handle(this.localQ.shift()); }

  onMessage(m) {
    if (m.t === 'far') { const r = this.r; if (this.far) r.freeMesh(this.far.mesh); this.far = { mesh: r.mesh(m.far, m.nF), org: [m.cx * CX, 0, m.cz * CX] }; this.farBusy = false; return; }
    this.pending--;
    const c = this.chunks.get(chunkKey(m.cx, m.cz));
    if (!c) return;
    c.busy = false;
    if (m.voxels) c.voxels = m.voxels;
    if (m.ver !== c.ver) { c.dirty = true; if (!c.voxels) return; }
    const r = this.r;
    r.freeMesh(c.opaque); r.freeMesh(c.deco); r.freeMesh(c.water);
    c.opaque = r.mesh(m.opaque, m.nO); c.deco = m.nD ? r.mesh(m.deco, m.nD) : null; c.water = m.nW ? r.mesh(m.water, m.nW) : null;
    c.lod = m.lod; c.ymin = m.ymin; c.ymax = m.ymax; c.ready = true;
    this.stats.genMs = this.stats.genMs * 0.9 + m.ms * 0.1; this.stats.meshed++;
    this.r.lightDirty = true;
  }

  key(x, z) { return chunkKey(Math.floor(x / CX), Math.floor(z / CX)); }
  get(x, y, z) {
    if (y < 0) return B.stone; if (y >= CH) return 0;
    const c = this.chunks.get(this.key(x, z));
    if (!c || !c.voxels) return -1;
    return c.voxels[idx(((x % CX) + CX) % CX, y, ((z % CX) + CX) % CX)];
  }
  solid(x, y, z) { const b = this.get(x, y, z); return b === -1 || IS_SOLID[b] === 1; }
  set(x, y, z, id) {
    const cx = Math.floor(x / CX), cz = Math.floor(z / CX), lx = ((x % CX) + CX) % CX, lz = ((z % CX) + CX) % CX, i = idx(lx, y, lz), k = chunkKey(cx, cz);
    const c = this.chunks.get(k); if (!c || !c.voxels) return false;
    c.voxels[i] = id;
    if (!this.edits.has(k)) this.edits.set(k, new Map());
    this.edits.get(k).set(i, id);
    if (this.local) this.local.handle({ t: 'edit', cx, cz, i, id }); else for (const w of this.workers) w.postMessage({ t: 'edit', cx, cz, i, id });
    this.dirty(cx, cz);
    const P = 8; // rayon de propagation pris en compte par le volume de lumière
    for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) {
      if (!dx && !dz) continue;
      if ((dx < 0 && lx >= P) || (dx > 0 && lx < CX - P) || (dz < 0 && lz >= P) || (dz > 0 && lz < CX - P)) continue;
      this.dirty(cx + dx, cz + dz);
    }
    return true;
  }
  dirty(cx, cz) { const c = this.chunks.get(chunkKey(cx, cz)); if (c) { c.ver++; c.dirty = true; } }

  update(px, pz, maxInflight) {
    const pcx = Math.floor(px / CX), pcz = Math.floor(pz / CX), R = this.radius;
    if (this.local) this.tickLocal();
    const wanted = [];
    for (let dz = -R; dz <= R; dz++) for (let dx = -R; dx <= R; dx++) {
      const d = Math.hypot(dx, dz); if (d > R + 0.5) continue;
      const cx = pcx + dx, cz = pcz + dz, k = chunkKey(cx, cz);
      let c = this.chunks.get(k);
      if (!c) { c = { cx, cz, wx: cx * CX + CX / 2, wz: cz * CX + CX / 2, org: [cx * CX, 0, cz * CX], ver: 0, lod: -1, ready: false, busy: false, dirty: true, voxels: null, ymin: 0, ymax: CH }; this.chunks.set(k, c); }
      c.d = d;
      const L = this.r.preset.lod, want = d <= L[0] ? 0 : d <= L[1] ? 1 : 2, needVox = d <= 3.2 && !c.voxels;
      // hystérésis : on ne dégrade pas un chunk tant qu'il ne s'est pas franchement éloigné
      const keep = c.ready && c.lod <= want && d <= (c.lod === 0 ? L[0] + 0.8 : c.lod === 1 ? L[1] + 0.8 : 99);
      if (!c.busy && (c.dirty || !c.ready || !keep || needVox)) wanted.push([c.dirty && c.ready ? -10 + d : d + (c.ready ? 3 : 0), c, want, needVox]);
    }
    wanted.sort((a, b) => a[0] - b[0]);
    for (const [, c, want, vox] of wanted) {
      if (this.pending >= maxInflight) break;
      c.busy = true; c.dirty = false; this.pending++;
      this.send({ t: 'mesh', cx: c.cx, cz: c.cz, lod: want, voxels: vox, ver: c.ver }, (((c.cx >> 1) * 31 + (c.cz >> 1)) & 0xffff));
    }
    this.queued = wanted.length;
    for (const [k, c] of this.chunks) if (Math.hypot(c.cx - pcx, c.cz - pcz) > R + 2.5 && !c.busy) { const r = this.r; r.freeMesh(c.opaque); r.freeMesh(c.deco); r.freeMesh(c.water); this.chunks.delete(k); }
    const fk = (pcx >> 4) + ',' + (pcz >> 4);
    if (fk !== this.farKey && !this.farBusy) { this.farKey = fk; this.farBusy = true; this.send({ t: 'far', cx: (pcx >> 4) * 16, cz: (pcz >> 4) * 16 }, 0); }
  }
  ready() { let n = 0, t = 0; for (const c of this.chunks.values()) { t++; if (c.ready) n++; } return t ? n / t : 0; }
  nearReady(px, pz) { const c = this.chunks.get(this.key(px, pz)); return !!(c && c.voxels && c.ready); }
}
