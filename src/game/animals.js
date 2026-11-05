// Animals/animals.js — faune simple : habitat, comportements, réactions au joueur.
import { VBuf, box, KIND } from '../world/mesher.js';
import { Rng } from '../core/noise.js';
import { M } from '../core/math.js';
import { B, IS_SOLID, BLOCKS } from '../world/blocks.js';

const P = (o, a, c) => box(o, a[0], a[1], a[2], a[3], a[4], a[5], c[0], c[1], c[2], 0, 255, 0, KIND.FLAT, true);
const FUR = [0.68, 0.45, 0.26], FUR2 = [0.56, 0.36, 0.2], CREAM = [0.92, 0.84, 0.7], DARK = [0.15, 0.1, 0.07], ANT = [0.74, 0.62, 0.45];

function part(f) { const o = new VBuf(300); f(o); return o; }
export function pack(r, parts) {
  const all = new VBuf(3000), rng = {};
  for (const n in parts) {
    const p = parts[n], st = all.n;
    if (all.n + p.n > all.cap) { const u = all.u8; all.alloc((all.n + p.n) * 2); all.u8.set(u); }
    all.u8.set(p.u8.subarray(0, p.n * 20), all.n * 20); all.n += p.n;
    rng[n] = { offset: (st / 4) * 6 * 4, count: (p.n / 4) * 6 };
  }
  return { mesh: r.mesh(all.take(), all.n), rng };
}

export function buildFauna(r) {
  return pack(r, {
    deerBody: part((o) => {
      P(o, [-0.2, 0, -0.55, 0.2, 0.4, 0.55], FUR); P(o, [-0.16, -0.05, -0.45, 0.16, 0.02, 0.45], CREAM);
      for (const [x, z] of [[-0.11, -0.3], [0.09, -0.12], [-0.06, 0.12], [0.12, 0.32], [0.02, -0.42], [-0.13, 0.36]]) P(o, [x - 0.035, 0.4, z - 0.035, x + 0.035, 0.42, z + 0.035], CREAM);
      P(o, [-0.07, 0.24, -0.62, 0.07, 0.4, -0.55], CREAM);
    }),
    deerHead: part((o) => {
      P(o, [-0.09, 0, -0.08, 0.09, 0.42, 0.1], FUR2); P(o, [-0.11, 0.38, -0.08, 0.11, 0.58, 0.16], FUR); P(o, [-0.07, 0.4, 0.16, 0.07, 0.51, 0.3], FUR2);
      P(o, [-0.04, 0.44, 0.3, 0.04, 0.5, 0.33], DARK);
      P(o, [-0.13, 0.48, 0.05, -0.1, 0.51, 0.08], DARK); P(o, [0.1, 0.48, 0.05, 0.13, 0.51, 0.08], DARK);
      P(o, [-0.25, 0.54, -0.04, -0.11, 0.6, 0.03], FUR2); P(o, [0.11, 0.54, -0.04, 0.25, 0.6, 0.03], FUR2);
      for (const sx of [-1, 1]) {
        P(o, [Math.min(sx * 0.06, sx * 0.1), 0.58, -0.03, Math.max(sx * 0.06, sx * 0.1), 0.92, 0.01], ANT);
        P(o, [Math.min(sx * 0.1, sx * 0.24), 0.8, -0.03, Math.max(sx * 0.1, sx * 0.24), 0.84, 0.01], ANT);
        P(o, [Math.min(sx * 0.2, sx * 0.24), 0.84, -0.03, Math.max(sx * 0.2, sx * 0.24), 1.04, 0.01], ANT);
        P(o, [Math.min(sx * 0.08, sx * 0.12), 0.68, 0.01, Math.max(sx * 0.08, sx * 0.12), 0.72, 0.15], ANT);
      }
    }),
    deerLeg: part((o) => { P(o, [-0.06, -0.48, -0.07, 0.06, 0, 0.07], FUR2); P(o, [-0.05, -0.62, -0.06, 0.05, -0.48, 0.06], CREAM); P(o, [-0.055, -0.7, -0.065, 0.055, -0.62, 0.065], DARK); }),
    bird: part((o) => {
      const blue = [0.22, 0.5, 0.86];
      P(o, [-0.055, 0, -0.1, 0.055, 0.1, 0.09], blue); P(o, [-0.056, 0, -0.03, 0.056, 0.05, 0.1], [0.95, 0.9, 0.82]);
      P(o, [-0.05, 0.08, 0.06, 0.05, 0.17, 0.15], blue); P(o, [-0.015, 0.11, 0.15, 0.015, 0.13, 0.19], [0.92, 0.65, 0.15]);
      P(o, [-0.03, 0.04, -0.22, 0.03, 0.07, -0.1], [0.14, 0.3, 0.6]);
      P(o, [-0.03, 0.14, 0.14, -0.02, 0.15, 0.15], DARK); P(o, [0.02, 0.14, 0.14, 0.03, 0.15, 0.15], DARK);
      P(o, [-0.04, -0.06, 0, -0.025, 0, 0.015], DARK); P(o, [0.025, -0.06, 0, 0.04, 0, 0.015], DARK);
    }),
    wing: part((o) => { P(o, [0, 0, -0.07, 0.17, 0.015, 0.08], [0.15, 0.36, 0.72]); }),
  });
}

const MAX_DEER = 5, MAX_BIRDS = 5;

export class Fauna {
  constructor(model, seed) { this.m = model; this.rng = new Rng(seed ^ 0x51ed); this.deer = []; this.birds = []; this.t = 0; }
  ground(w, x, z, y0) {
    const fx = Math.floor(x), fz = Math.floor(z);
    for (let y = Math.floor(y0) + 2; y > Math.floor(y0) - 22 && y > 1; y--) {
      const b = w.get(fx, y, fz);
      if (b === -1) return null;
      if (b === B.water) return null;
      if (IS_SOLID[b] && BLOCKS[b].shape !== 'leaf') return BLOCKS[b].cat === 'wood' ? null : y + 1;
    }
    return null;
  }
  // Le corps tient-il là ? Pattes, flancs, tête et croupe doivent rester hors des blocs solides.
  fits(w, x, y, z, yaw, h) {
    const s = Math.sin(yaw), c = Math.cos(yaw), Y = Math.floor(y + 0.05);
    for (const [f, l] of [[0.75, 0], [0.5, 0.25], [0.5, -0.25], [0, 0.25], [0, -0.25], [-0.6, 0.25], [-0.6, -0.25]]) {
      const bx = Math.floor(x + s * f + c * l), bz = Math.floor(z + c * f - s * l);
      for (let k = 0; k < h; k++) { const b = w.get(bx, Y + k, bz); if (b === -1 || IS_SOLID[b] || b === B.water) return false; }
    }
    return true;
  }
  // Un cerf ne pivote que si son corps tient dans la nouvelle orientation.
  turn(d, w, yaw) { if (this.fits(w, d.p[0], d.gy ?? d.p[1], d.p[2], yaw, 2)) { d.yaw = yaw; return true; } return false; }
  spawnSpot(w, p, a0, a1) {
    const r = this.rng, a = r.range(0, 6.283), d = r.range(a0, a1), x = p[0] + Math.cos(a) * d, z = p[2] + Math.sin(a) * d, y = this.ground(w, x, z, p[1]);
    return y === null ? null : [x, y, z];
  }
  update(dt, w, pl) {
    const r = this.rng, p = pl.p; this.t += dt;
    if (this.deer.length < MAX_DEER && r.next() < dt * 2) { const s = this.spawnSpot(w, p, 16, 60), yaw0 = r.range(0, 6.28); if (s && this.fits(w, s[0], s[1], s[2], yaw0, 2)) this.deer.push({ p: s, gy: s[1], yaw: yaw0, state: 'graze', t: r.range(2, 7), phase: r.range(0, 6), head: 1 }); }
    if (this.birds.length < MAX_BIRDS && r.next() < dt * 2) { const s = this.spawnSpot(w, p, 7, 34); if (s && !IS_SOLID[w.get(Math.floor(s[0]), Math.floor(s[1]), Math.floor(s[2]))]) this.birds.push({ p: s, yaw: r.range(0, 6.28), state: 'perch', t: r.range(1, 5), v: [0, 0, 0], flap: 0 }); }
    for (const d of this.deer) this.stepDeer(d, dt, w, p);
    this.deer = this.deer.filter((d) => Math.hypot(d.p[0] - p[0], d.p[2] - p[2]) < 92 && w.get(Math.floor(d.p[0]), Math.floor(d.p[1]), Math.floor(d.p[2])) !== -1);
    for (const b of this.birds) this.stepBird(b, dt, w, p);
    this.birds = this.birds.filter((b) => b.state !== 'gone' && Math.hypot(b.p[0] - p[0], b.p[2] - p[2]) < 70 && b.p[1] < p[1] + 45);
  }
  stepDeer(d, dt, w, p) {
    const r = this.rng, dist = Math.hypot(d.p[0] - p[0], d.p[2] - p[2]);
    if (dist < 8 && d.state !== 'flee') { d.state = 'flee'; d.t = r.range(3, 5); const a = Math.atan2(d.p[0] - p[0], d.p[2] - p[2]); for (const o of [0, 0.5, -0.5, 1, -1, 1.6, -1.6]) if (this.turn(d, w, a + o)) break; }
    d.t -= dt;
    if (d.t <= 0) { if (d.state === 'graze') { d.state = 'walk'; d.t = r.range(3, 8); this.turn(d, w, d.yaw + r.range(-1.3, 1.3)); } else { d.state = 'graze'; d.t = r.range(3, 9); } }
    const sp = d.state === 'flee' ? 6 : d.state === 'walk' ? 1.3 : 0;
    d.speed = sp; d.head += ((d.state === 'graze' ? 1 : 0) - d.head) * Math.min(1, dt * 3);
    if (sp) {
      d.phase += dt * (3 + sp * 2);
      const nx = d.p[0] + Math.sin(d.yaw) * sp * dt, nz = d.p[2] + Math.cos(d.yaw) * sp * dt, y = this.ground(w, nx, nz, d.gy ?? d.p[1]);
      if (y !== null && Math.abs(y - (d.gy ?? d.p[1])) <= 1 && this.fits(w, nx, y, nz, d.yaw, 2) && this.fits(w, nx, Math.max(y, d.gy ?? y), nz, d.yaw, 2)) { d.p[0] = nx; d.p[2] = nz; d.gy = y; }
      else { d.blocked = (d.blocked || 0) + 1; for (let i = 0; i < 6; i++) if (this.turn(d, w, d.yaw + r.range(0.6, 3.2))) break; if (d.blocked > 40) { d.state = 'graze'; d.t = r.range(1, 3); d.blocked = 0; } }
      d.p[1] += ((d.gy ?? d.p[1]) - d.p[1]) * Math.min(1, dt * 9);
    }
  }
  stepBird(b, dt, w, p) {
    const r = this.rng, dist = Math.hypot(b.p[0] - p[0], b.p[2] - p[2]);
    if (b.state === 'perch') {
      b.t -= dt;
      if (b.t <= 0) { b.t = r.range(1, 4); b.yaw += r.range(-1.5, 1.5); b.hop = 1; }
      b.hop = Math.max(0, (b.hop || 0) - dt * 5);
      if (dist < 5) { b.state = 'fly'; b.v = [Math.sin(b.yaw) * 4, 4.5, Math.cos(b.yaw) * 4]; }
    } else {
      b.flap += dt * 30; b.v[1] += dt * 0.7;
      const nx = b.p[0] + b.v[0] * dt, ny = b.p[1] + b.v[1] * dt, nz = b.p[2] + b.v[2] * dt, cell = w.get(Math.floor(nx), Math.floor(ny + 0.1), Math.floor(nz));
      // Dans la couronne ou contre un bloc : l'oiseau vire au lieu de traverser, et disparaît s'il reste bloqué.
      if (cell > 0 && (IS_SOLID[cell] || BLOCKS[cell].shape === 'deco')) { b.v[0] = -b.v[2] * 1.1; b.v[2] = b.v[0] * -1.1; b.yaw += 1.57; b.stuck = (b.stuck || 0) + dt; if (b.stuck > 1) b.state = 'gone'; }
      else { b.p[0] = nx; b.p[1] = ny; b.p[2] = nz; b.yaw = Math.atan2(b.v[0], b.v[2]); }
    }
  }
  actors() {
    const out = [], m = this.m, put = (base, name, x, y, z, ry, rx) => { const r = m.rng[name]; out.push({ mesh: m.mesh, model: M.mul(base, M.trs(x, y, z, ry, rx)), count: r.count, offset: r.offset }); };
    for (const d of this.deer) {
      const base = M.trs(d.p[0], d.p[1], d.p[2], d.yaw), s = d.speed ? Math.sin(d.phase) * 0.75 : 0, hip = 0.72;
      put(base, 'deerBody', 0, hip, 0, 0, 0);
      put(base, 'deerHead', 0, hip + 0.28, 0.45, 0, 0.42 + 1.05 * d.head);
      put(base, 'deerLeg', -0.13, hip, 0.44, 0, s); put(base, 'deerLeg', 0.13, hip, 0.44, 0, -s);
      put(base, 'deerLeg', -0.13, hip, -0.44, 0, -s); put(base, 'deerLeg', 0.13, hip, -0.44, 0, s);
    }
    for (const b of this.birds) {
      const base = M.trs(b.p[0], b.p[1] + 0.06 + (b.hop || 0) * 0.1, b.p[2], b.yaw), f = b.state === 'fly' ? Math.sin(b.flap) * 1 : -1.25;
      put(base, 'bird', 0, 0, 0, 0, 0);
      out.push({ mesh: m.mesh, model: M.mul(base, M.mul(M.trs(0.055, 0.08, 0, 0, 0), rotZ(f))), count: m.rng.wing.count, offset: m.rng.wing.offset });
      out.push({ mesh: m.mesh, model: M.mul(base, M.mul(M.trs(-0.055, 0.08, 0, 0, 0), rotZ(Math.PI - f))), count: m.rng.wing.count, offset: m.rng.wing.offset });
    }
    return out;
  }
}
function rotZ(a) { const c = Math.cos(a), s = Math.sin(a), m = M.id(); m[0] = c; m[1] = s; m[4] = -s; m[5] = c; return m; }
