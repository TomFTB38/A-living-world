// Characters/character.js — personnage semi-voxel construit par code, animé de façon procédurale.
import { VBuf, box, KIND } from '../world/mesher.js';
import { hashFloat } from '../core/noise.js';
import { M } from '../core/math.js';

const C = { skin: [0.93, 0.7, 0.53], hair: [0.27, 0.15, 0.08], hair2: [0.36, 0.2, 0.1], shirt: [0.88, 0.82, 0.68], shirt2: [0.78, 0.72, 0.58], pants: [0.37, 0.3, 0.24], boot: [0.3, 0.17, 0.09], cuff: [0.62, 0.58, 0.5], bag: [0.46, 0.28, 0.14], bag2: [0.58, 0.38, 0.2], roll: [0.8, 0.68, 0.46], strap: [0.32, 0.19, 0.1], handle: [0.47, 0.27, 0.13], steel: [0.42, 0.45, 0.5], glove: [0.42, 0.24, 0.12] };
const k = KIND.FLAT, sk = 255;
const Bx = (o, a, c) => box(o, a[0], a[1], a[2], a[3], a[4], a[5], c[0], c[1], c[2], 0, sk, 0, k, true);

function part(f) { const o = new VBuf(600); f(o); return o; }

export function buildCharacter(r) {
  const parts = {
    torso: part((o) => {
      Bx(o, [-0.25, 0, -0.15, 0.25, 0.62, 0.15], C.shirt); Bx(o, [-0.26, 0, -0.16, 0.26, 0.1, 0.16], C.shirt2);
      Bx(o, [-0.2, 0.05, -0.34, 0.2, 0.56, -0.15], C.bag); Bx(o, [-0.15, 0.1, -0.38, 0.15, 0.3, -0.34], C.bag2); // sac à dos (Z- = dos)
      Bx(o, [-0.04, 0.32, -0.39, 0.04, 0.4, -0.33], C.strap);
      Bx(o, [-0.24, 0.1, -0.16, -0.16, 0.62, 0.16], C.strap); Bx(o, [0.16, 0.1, -0.16, 0.24, 0.62, 0.16], C.strap);
      for (let i = 0; i < 6; i++) { const a = (i / 6) * 6.283, y = 0.72 + Math.cos(a) * 0.12, z = -0.24 + Math.sin(a) * 0.12; Bx(o, [-0.3, y - 0.07, z - 0.07, 0.3, y + 0.07, z + 0.07], i % 2 ? C.roll : [0.72, 0.6, 0.4]); } // paquetage roulé
    }),
    head: part((o) => {
      Bx(o, [-0.24, 0, -0.24, 0.24, 0.46, 0.24], C.skin);
      Bx(o, [-0.15, 0.16, 0.24, -0.05, 0.26, 0.25], [0.1, 0.07, 0.05]); Bx(o, [0.05, 0.16, 0.24, 0.15, 0.26, 0.25], [0.1, 0.07, 0.05]); // yeux
      for (let x = -3; x <= 3; x++) for (let z = -3; z <= 3; z++) for (let y = 0; y < 3; y++) { // chevelure en petites mèches
        if (z > 1 && y < 2 && Math.abs(x) < 3) continue;
        const h = hashFloat(x, y, z, 5), e = y === 0 ? 0.3 : 0.2; if (y === 2 && h < 0.3) continue;
        const X = x * 0.085, Z = z * 0.085, Y = y === 0 ? 0.26 : 0.4 + (y - 1) * 0.1, s = 0.075 + 0.03 * h;
        if (y === 0 && Math.abs(x) < 3 && Math.abs(z) < 3) continue;
        const c = h > 0.5 ? C.hair : C.hair2;
        Bx(o, [X - s, Y - e * 0.5, Z - s, X + s, Y + 0.1 + 0.12 * h, Z + s], c);
      }
    }),
    legL: part((o) => { Bx(o, [-0.11, -0.46, -0.12, 0.11, 0, 0.12], C.pants); Bx(o, [-0.13, -0.55, -0.14, 0.13, -0.46, 0.14], C.cuff); Bx(o, [-0.13, -0.78, -0.14, 0.13, -0.55, 0.2], C.boot); }),
    armL: part((o) => { Bx(o, [-0.09, -0.26, -0.1, 0.09, 0, 0.1], C.shirt); Bx(o, [-0.1, -0.32, -0.11, 0.1, -0.26, 0.11], C.shirt2); Bx(o, [-0.085, -0.56, -0.09, 0.085, -0.32, 0.09], C.skin); Bx(o, [-0.1, -0.66, -0.11, 0.1, -0.54, 0.11], C.glove); }),
    armR: part((o) => {
      Bx(o, [-0.09, -0.26, -0.1, 0.09, 0, 0.1], C.shirt); Bx(o, [-0.1, -0.32, -0.11, 0.1, -0.26, 0.11], C.shirt2); Bx(o, [-0.085, -0.56, -0.09, 0.085, -0.32, 0.09], C.skin); Bx(o, [-0.1, -0.66, -0.11, 0.1, -0.54, 0.11], C.glove);
    }),
  };
  parts.legR = parts.legL;
  const all = new VBuf(5000), rng = {};
  for (const n of ['torso', 'head', 'legL', 'armL', 'armR']) {
    const p = parts[n], st = all.n; all.u8.length; for (let i = 0; i < p.n; i++) { /* copie */ }
    if (all.n + p.n > all.cap) { const o = all.u8; all.alloc((all.n + p.n) * 2); all.u8.set(o); }
    all.u8.set(p.u8.subarray(0, p.n * 20), all.n * 20); all.n += p.n;
    rng[n] = { offset: (st / 4) * 6 * 4, count: (p.n / 4) * 6 };
  }
  rng.legR = rng.legL;
  return { mesh: r.mesh(all.take(), all.n), rng };
}

const rollZ = (a) => { const c = Math.cos(a), n = Math.sin(a), m = M.id(); m[0] = c; m[1] = n; m[4] = -n; m[5] = c; return m; };
// Façon de tenir l'objet, dans le repère du bras (la main est en bas, l'avant du personnage en +Z).
const HOLD = {
  tool: M.mul(M.trs(0, -0.6, 0.02, -Math.PI / 2), rollZ(-Math.PI / 4)),
  block: M.trs(-0.14, -0.76, -0.14, 0, 0, 0.28),
  deco: M.trs(-0.28, -0.72, -0.1, 0, 0, 0.55),
};

export class CharacterAnim {
  constructor(model) { this.m = model; this.phase = 0; this.walk = 0; this.run = 0; this.low = 0; this.air = 0; this.rise = 0; this.swing = 0; this.punch = 0; this.landed = 0; }
  update(dt, s) {
    const ease = (c, t, k) => c + (t - c) * Math.min(1, dt * k);
    const was = this.air;
    this.walk = ease(this.walk, Math.min(1, s.speed / 4.5), 10); this.run = ease(this.run, s.speed > 5.2 ? 1 : 0, 8); this.low = ease(this.low, s.crouch ? 1 : 0, 13);
    this.air = ease(this.air, s.ground || s.fly ? 0 : 1, 12); this.rise = ease(this.rise, s.vy > 0 ? 1 : 0, 12);
    if (was > 0.5 && this.air <= 0.5) this.landed = 1;
    this.landed = Math.max(0, this.landed - dt * 5);
    if (s.speed > 0.2) this.phase += dt * (3.4 + s.speed * 1.35);
    if (s.mining) this.swing = (this.swing + dt * 3.3) % 1;
    else if (this.swing > 0) { this.swing += dt * 3.3; if (this.swing >= 1) this.swing = 0; }
    this.punch = Math.max(0, this.punch - dt * 4.6);
  }
  trigger() { this.punch = 1; }
  pose() {
    const w = this.walk, s = Math.sin(this.phase), ground = 1 - this.air, a = this.air;
    const leg = s * (0.72 + 0.42 * this.run) * w * (1 - 0.45 * this.low) * ground, arm = s * (0.55 + 0.6 * this.run) * w * ground;
    const hip = 0.8 + Math.abs(Math.cos(this.phase)) * 0.045 * w * ground - 0.24 * this.low - 0.1 * this.landed + 0.03 * a;
    const lean = 0.05 * w + 0.24 * this.run + 0.44 * this.low + 0.14 * this.landed - 0.08 * a * this.rise;
    const legL = -leg - 0.58 * this.low - (0.62 * this.rise + 0.25) * a, legR = leg - 0.58 * this.low + (0.3 * this.rise - 0.1) * a;
    const armL = arm + 0.1 - 0.95 * a * (0.5 + 0.5 * this.rise) - 0.3 * this.low;
    const swingA = this.swing > 0 ? -1.5 + Math.cos(this.swing * 6.283) * 0.95 : 0, punchA = -Math.sin(this.punch * Math.PI) * 1.05;
    const armR = (this.swing > 0 ? swingA : -0.5 - arm * 0.35 - 0.7 * a) + punchA;
    return { hip, lean, legL, legR, armL, armR };
  }
  actors(px, py, pz, yaw, pitch, held) {
    const m = this.m, q = this.pose(), base = M.trs(px, py, pz, yaw), out = [], T = M.mul(base, M.trs(0, q.hip, 0, 0, q.lean));
    const put = (name, mat) => { const r = m.rng[name]; out.push({ mesh: m.mesh, model: mat, count: r.count, offset: r.offset, hero: true }); };
    put('torso', T); put('head', M.mul(T, M.trs(0, 0.64, 0, 0, -q.lean * 0.75 - pitch * 0.5)));
    put('legL', M.mul(base, M.trs(0.13, q.hip, 0, 0, q.legL))); put('legR', M.mul(base, M.trs(-0.13, q.hip, 0, 0, q.legR)));
    const armR = M.mul(T, M.trs(-0.34, 0.58, 0.02, 0, q.armR));
    put('armL', M.mul(T, M.trs(0.34, 0.58, 0, 0, q.armL))); put('armR', armR);
    if (held) out.push({ mesh: held.mesh, model: M.mul(armR, HOLD[held.kind]), count: held.count, offset: held.offset, hero: true, local: held.local });
    return out;
  }
  // Bras vu à la première personne : mêmes objets, attachés à la caméra.
  viewActors(invView, held) {
    const m = this.m, s = Math.sin(this.phase), c = Math.cos(this.phase * 0.5), w = this.walk * (1 - this.air), V = M.mul(invView, M.trs(0, 0, 0, Math.PI));
    const strike = this.swing > 0 ? Math.sin(this.swing * Math.PI) : 0, push = Math.sin(this.punch * Math.PI);
    const x = -0.25 + c * 0.022 * w - 0.05 * this.run, y = -0.2 - Math.abs(s) * 0.022 * w - 0.05 * this.low + 0.07 * this.air * (this.rise - 0.5) - 0.06 * strike;
    const z = -0.08 + 0.2 * push + 0.12 * strike, rx = -1.66 + 0.22 * this.run + 0.85 * strike - 0.3 * push;
    const arm = M.mul(V, M.trs(x, y, z, 0.2 + 0.3 * strike, rx, 0.58)), r = m.rng.armR;
    const out = [{ mesh: m.mesh, model: arm, count: r.count, offset: r.offset, hero: true, view: true }];
    if (held) out.push({ mesh: held.mesh, model: M.mul(M.mul(arm, HOLD[held.kind]), held.kind === 'tool' ? M.trs(0, 0, 0, 0, 0, 0.5) : held.kind === 'block' ? M.trs(0.04, 0.04, 0.04, 0, 0, 0.72) : M.id()), count: held.count, offset: held.offset, hero: true, view: true, local: held.local });
    return out;
  }
}
