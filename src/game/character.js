// Characters/character.js — personnage semi-voxel construit par code, animé de façon procédurale.
import { VBuf, box, KIND } from '../world/mesher.js';
import { hashFloat } from '../core/noise.js';
import { M } from '../core/math.js';

const C = { skin: [0.93, 0.7, 0.53], hair: [0.36, 0.2, 0.1], hair2: [0.46, 0.27, 0.13], shirt: [0.88, 0.82, 0.68], shirt2: [0.78, 0.72, 0.58], pants: [0.37, 0.3, 0.24], boot: [0.3, 0.17, 0.09], cuff: [0.62, 0.58, 0.5], bag: [0.46, 0.28, 0.14], bag2: [0.58, 0.38, 0.2], roll: [0.8, 0.68, 0.46], strap: [0.32, 0.19, 0.1], handle: [0.47, 0.27, 0.13], steel: [0.42, 0.45, 0.5], glove: [0.42, 0.24, 0.12] };
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
      Bx(o, [-0.03, -0.62, -0.2, 0.03, -0.56, 0.72], C.handle); // manche
      for (let i = 0; i < 7; i++) { const t = (i - 3) / 3, y = -0.6 + (1 - t * t) * 0.14; Bx(o, [-0.045, y - 0.05, 0.62 + t * 0.34 * 0 , 0.045, y + 0.06, 0.72], C.steel); Bx(o, [-0.04, y - 0.28 * Math.abs(t) - 0.05, 0.65, 0.04, y + 0.03, 0.72 + 0.03 * (1 - Math.abs(t))], C.steel); }
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

export class CharacterAnim {
  constructor(model) { this.m = model; this.phase = 0; this.swing = 0; this.blend = 0; }
  actors(px, py, pz, yaw, speed, dt, mining, crouch) {
    this.phase += dt * (3.2 + speed * 1.45) * (speed > 0.2 ? 1 : 0);
    this.blend += ((speed > 0.2 ? Math.min(1, speed / 5) : 0) - this.blend) * Math.min(1, dt * 10);
    if (mining) this.swing += dt * 8; else this.swing = 0;
    const m = this.m, a = this.blend, s = Math.sin(this.phase) * 0.85 * a, bob = Math.abs(Math.cos(this.phase)) * 0.04 * a;
    const hip = 0.8 + bob - (crouch ? 0.18 : 0), pitch = crouch ? 0.25 : 0.05 * a;
    const base = M.trs(px, py, pz, yaw), out = [];
    const put = (name, x, y, z, rx, ry = 0) => { const r = m.rng[name], t = M.mul(base, M.mul(M.trs(x, y, z, ry, rx), M.id())); out.push({ mesh: m.mesh, model: t, count: r.count, offset: r.offset }); };
    put('torso', 0, hip, 0, pitch);
    put('legL', -0.13, hip, 0, -s); put('legR', 0.13, hip, 0, s);
    put('head', 0, hip + 0.64, pitch * 0.5, 0);
    put('armL', -0.34, hip + 0.58, 0, s * 0.8 + 0.08);
    const sw = mining ? -1.2 - Math.sin(this.swing) * 0.85 : -0.55 - s * 0.35;
    put('armR', 0.34, hip + 0.58, 0.02, sw);
    return out;
  }
}
