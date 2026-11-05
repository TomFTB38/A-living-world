// Animals/animals.js — faune simple : habitat, comportements, réactions au joueur.
import { VBuf, box, KIND } from '../world/mesher.js';
import { Rng } from '../core/noise.js';
import { M } from '../core/math.js';
import { B, IS_SOLID, BLOCKS } from '../world/blocks.js';

const BIRD = 1.7; // à l'échelle réelle, un oiseau disparaît dans l'herbe et le feuillage
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
      const blue = [0.22, 0.5, 0.86], B2 = (a, c) => P(o, a.map((v) => v * BIRD), c);
      B2([-0.055, 0, -0.1, 0.055, 0.1, 0.09], blue); B2([-0.056, 0, -0.03, 0.056, 0.05, 0.1], [0.95, 0.9, 0.82]);
      B2([-0.05, 0.08, 0.06, 0.05, 0.17, 0.15], blue); B2([-0.015, 0.11, 0.15, 0.015, 0.13, 0.19], [0.92, 0.65, 0.15]);
      B2([-0.03, 0.04, -0.22, 0.03, 0.07, -0.1], [0.14, 0.3, 0.6]);
      B2([-0.03, 0.14, 0.14, -0.02, 0.15, 0.15], DARK); B2([0.02, 0.14, 0.14, 0.03, 0.15, 0.15], DARK);
      B2([-0.04, -0.06, 0, -0.025, 0, 0.015], DARK); B2([0.025, -0.06, 0, 0.04, 0, 0.015], DARK);
    }),
    wing: part((o) => { P(o, [0, 0, -0.07 * BIRD, 0.17 * BIRD, 0.015 * BIRD, 0.08 * BIRD], [0.15, 0.36, 0.72]); }),
  });
}

const MAX_DEER = 5, MAX_BIRDS = 9;

export class Fauna {
  constructor(model, seed) { this.m = model; this.rng = new Rng(seed ^ 0x51ed); this.deer = []; this.birds = []; this.t = 0; this.emit = () => {}; this.day = 1; }
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
  update(dt, w, pl, day = 1) {
    const r = this.rng, p = pl.p; this.t += dt; this.day = day;
    if (this.deer.length < MAX_DEER && r.next() < dt * 2) { const s = this.spawnSpot(w, p, 16, 60), yaw0 = r.range(0, 6.28); if (s && this.fits(w, s[0], s[1], s[2], yaw0, 2)) this.deer.push({ p: s, gy: s[1], yaw: yaw0, state: 'graze', t: r.range(2, 7), phase: r.range(0, 6), head: 1 }); }
    // Les oiseaux arrivent en vol, en petits groupes, puis se posent à découvert.
    if (day > 0.25 && this.birds.length < MAX_BIRDS - 2 && r.next() < dt * 1.2) this.flock(w, p);
    for (const d of this.deer) this.stepDeer(d, dt, w, p);
    this.deer = this.deer.filter((d) => Math.hypot(d.p[0] - p[0], d.p[2] - p[2]) < 92 && w.get(Math.floor(d.p[0]), Math.floor(d.p[1]), Math.floor(d.p[2])) !== -1);
    for (const b of this.birds) this.stepBird(b, dt, w, p);
    this.birds = this.birds.filter((b) => b.state !== 'gone' && Math.hypot(b.p[0] - p[0], b.p[2] - p[2]) < 82);
  }
  stepDeer(d, dt, w, p) {
    const r = this.rng, dist = Math.hypot(d.p[0] - p[0], d.p[2] - p[2]);
    if (dist < 8 && d.state !== 'flee') { d.state = 'flee'; d.t = r.range(3, 5); this.emit('deerAlarm', d.p); const a = Math.atan2(d.p[0] - p[0], d.p[2] - p[2]); for (const o of [0, 0.5, -0.5, 1, -1, 1.6, -1.6]) if (this.turn(d, w, a + o)) break; }
    d.t -= dt;
    if (d.t <= 0) { if (d.state === 'graze') { d.state = 'walk'; d.t = r.range(3, 8); this.turn(d, w, d.yaw + r.range(-1.3, 1.3)); } else { d.state = 'graze'; d.t = r.range(3, 9); } }
    const sp = d.state === 'flee' ? 6 : d.state === 'walk' ? 1.3 : 0;
    if (d.state === 'graze' && r.next() < dt * 0.05) this.emit('deerCall', d.p);
    if (sp) { d.hoof = (d.hoof ?? 0) - dt * (2.2 + sp * 1.1); if (d.hoof <= 0) { d.hoof = 1; this.emit('hoof', d.p, sp > 3); } }
    d.speed = sp; d.head += ((d.state === 'graze' ? 1 : 0) - d.head) * Math.min(1, dt * 3);
    if (sp) {
      d.phase += dt * (3 + sp * 2);
      const sy = Math.sin(d.yaw), cy = Math.cos(d.yaw), nx = d.p[0] + sy * sp * dt, nz = d.p[2] + cy * sp * dt, base = d.gy ?? d.p[1];
      // Le corps se pose sur le sol le plus haut sous ses pattes : il monte et descend un bloc sans entrer dedans.
      const gc = this.ground(w, nx, nz, base), gf = this.ground(w, nx + sy * 0.8, nz + cy * 0.8, base), gr = this.ground(w, nx - sy * 0.65, nz - cy * 0.65, base);
      const y = gc === null || gf === null || gr === null ? null : Math.max(gc, gf, gr);
      if (y !== null && Math.abs(y - base) <= 1 && this.fits(w, nx, y, nz, d.yaw, 2)) { d.p[0] = nx; d.p[2] = nz; d.gy = y; d.blocked = 0; }
      else { d.blocked = (d.blocked || 0) + 1; for (let i = 0; i < 6; i++) if (this.turn(d, w, d.yaw + r.range(0.6, 3.2))) break; if (d.blocked > 40) { d.state = 'graze'; d.t = r.range(1, 3); d.blocked = 0; } }
      d.p[1] += ((d.gy ?? d.p[1]) - d.p[1]) * Math.min(1, dt * 9);
    }
  }
  // Dessus d'une colonne : y compris le feuillage. `clear` indique qu'un oiseau y resterait visible.
  topAt(w, x, z, y0) {
    const fx = Math.floor(x), fz = Math.floor(z); let deco = false;
    for (let y = Math.floor(y0) + 26; y > Math.floor(y0) - 26 && y > 1; y--) {
      const b = w.get(fx, y, fz);
      if (b === -1) return null;
      if (b === B.water) return { y: y + 1, clear: false };
      if (IS_SOLID[b]) return { y: y + 1, clear: !deco };
      if (b > 0) deco = true;
    }
    return null;
  }
  flock(w, p) {
    const r = this.rng, a = r.range(0, 6.283), d = r.range(18, 36), cx = p[0] + Math.cos(a) * d, cz = p[2] + Math.sin(a) * d, top = this.topAt(w, cx, cz, p[1]);
    if (!top) return;
    for (let i = 0, n = 1 + Math.floor(r.next() * 3); i < n; i++) {
      const b = { p: [cx + r.range(-2, 2), top.y + r.range(7, 13), cz + r.range(-2, 2)], yaw: r.range(0, 6.28), state: 'fly', t: 0, v: [0, 0, 0], flap: r.range(0, 6), voice: Math.floor(r.next() * 3), sing: r.range(1, 5), hop: 0 };
      const c = w.get(Math.floor(b.p[0]), Math.floor(b.p[1] + 0.1), Math.floor(b.p[2])); if (c !== 0) continue;
      this.goal(b, w, p); this.birds.push(b);
    }
  }
  // Cherche un perchoir dégagé, loin du joueur ; à défaut, l'oiseau poursuit son vol.
  goal(b, w, p) {
    const r = this.rng;
    // Un essai sur deux autour du joueur, plutôt bas : on doit pouvoir les voir et les entendre sans lever la tête.
    for (let i = 0; i < 12; i++) {
      const c = i % 2 ? b.p : p, a = r.range(0, 6.283), d = r.range(8, 24), x = Math.floor(c[0] + Math.cos(a) * d) + 0.5, z = Math.floor(c[2] + Math.sin(a) * d) + 0.5, top = this.topAt(w, x, z, b.p[1]);
      if (top && top.clear && Math.hypot(x - p[0], z - p[2]) > 7 && (top.y < p[1] + 9 || i > 9)) { b.tg = [x, top.y, z]; b.land = true; return; }
    }
    const a = r.range(0, 6.283), top = this.topAt(w, b.p[0], b.p[2], b.p[1]);
    b.tg = [b.p[0] + Math.cos(a) * 24, (top ? top.y : b.p[1]) + 8, b.p[2] + Math.sin(a) * 24]; b.land = false;
  }
  stepBird(b, dt, w, p) {
    const r = this.rng, dist = Math.hypot(b.p[0] - p[0], b.p[2] - p[2]);
    if (b.state === 'perch') {
      b.t -= dt; b.sing -= dt; b.hop = Math.max(0, b.hop - dt * 5);
      if (b.sing <= 0) { b.sing = r.range(3, 9); if (this.day > 0.25) this.emit('bird', b.p, b.voice); b.hop = 0.6; }
      if (r.next() < dt * 0.5) { b.yaw += r.range(-1.5, 1.5); b.hop = 1; }
      if (dist < 6 || (b.t <= 0 && this.day > 0.25)) { b.state = 'fly'; this.emit('wings', b.p); this.goal(b, w, p); b.v[1] = 3.5; }
      return;
    }
    b.flap += dt * 30; b.scan = (b.scan || 0) - dt;
    if (b.scan <= 0) { b.scan = 0.25; const top = this.topAt(w, b.p[0], b.p[2], b.p[1]); b.surf = top ? top.y : b.surf ?? b.p[1]; }
    const dx = b.tg[0] - b.p[0], dz = b.tg[2] - b.p[2], h = Math.hypot(dx, dz), sp = Math.min(6.5, 1.6 + h * 1.4), cruise = Math.max(b.surf + 3.5, b.tg[1] + (h > 7 ? 3.5 : 0));
    const k = Math.min(1, dt * 3.2), wantY = h < 2.5 ? b.tg[1] + 0.1 - b.p[1] : cruise - b.p[1];
    b.v[0] += (dx / (h || 1) * sp - b.v[0]) * k; b.v[2] += (dz / (h || 1) * sp - b.v[2]) * k; b.v[1] += (Math.max(-4, Math.min(4.5, wantY * 1.8)) - b.v[1]) * k;
    const nx = b.p[0] + b.v[0] * dt, ny = b.p[1] + b.v[1] * dt, nz = b.p[2] + b.v[2] * dt, cell = w.get(Math.floor(nx), Math.floor(ny + 0.1), Math.floor(nz));
    // Un obstacle devant : l'oiseau monte au lieu de traverser.
    if (cell > 0 && IS_SOLID[cell]) {
      const above = w.get(Math.floor(b.p[0]), Math.floor(b.p[1] + 0.1 + 5 * dt), Math.floor(b.p[2]));
      if (above > 0 && IS_SOLID[above]) { b.v[0] *= -1; b.v[2] *= -1; b.surf = b.p[1]; } else { b.v[1] = 5; b.p[1] += 5 * dt; }
      b.stuck = (b.stuck || 0) + dt; if (b.stuck > 2.5) b.state = 'gone'; return;
    }
    b.stuck = 0; b.p[0] = nx; b.p[1] = ny; b.p[2] = nz; if (h > 0.3) b.yaw = Math.atan2(b.v[0], b.v[2]);
    if (h < 0.5 && Math.abs(b.tg[1] - b.p[1]) < 0.6) {
      if (b.land) { const c = w.get(Math.floor(b.tg[0]), Math.floor(b.tg[1]), Math.floor(b.tg[2])); if (c === 0) { b.p = [...b.tg]; b.state = 'perch'; b.t = r.range(5, 16); b.v = [0, 0, 0]; return; } }
      this.goal(b, w, p);
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
      const base = M.trs(b.p[0], b.p[1] + 0.1 + (b.hop || 0) * 0.14, b.p[2], b.yaw), f = b.state === 'fly' ? Math.sin(b.flap) * 1 : -1.25;
      put(base, 'bird', 0, 0, 0, 0, 0);
      out.push({ mesh: m.mesh, model: M.mul(base, M.mul(M.trs(0.055 * BIRD, 0.08 * BIRD, 0, 0, 0), rotZ(f))), count: m.rng.wing.count, offset: m.rng.wing.offset });
      out.push({ mesh: m.mesh, model: M.mul(base, M.mul(M.trs(-0.055 * BIRD, 0.08 * BIRD, 0, 0, 0), rotZ(Math.PI - f))), count: m.rng.wing.count, offset: m.rng.wing.offset });
    }
    return out;
  }
}
function rotZ(a) { const c = Math.cos(a), s = Math.sin(a), m = M.id(); m[0] = c; m[1] = s; m[4] = -s; m[5] = c; return m; }
