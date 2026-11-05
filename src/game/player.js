// Gameplay/player.js — déplacement à collisions voxel (AABB), marche, sprint, saut, nage, accroupissement.
import { IS_SOLID, B } from '../world/blocks.js';

const HALF = 0.3, GRAV = 26, JUMP = 9.2;

export class Player {
  constructor(x, y, z) { this.p = [x, y, z]; this.v = [0, 0, 0]; this.yaw = 0; this.pitch = 0; this.ground = false; this.crouch = false; this.height = 1.8; this.inWater = false; this.speed = 0; this.moveYaw = 0; }
  box(w, x, y, z) {
    const h = this.crouch ? 1.45 : this.height;
    for (let by = Math.floor(y); by <= Math.floor(y + h - 0.001); by++) for (let bz = Math.floor(z - HALF); bz <= Math.floor(z + HALF); bz++) for (let bx = Math.floor(x - HALF); bx <= Math.floor(x + HALF); bx++) if (w.solid(bx, by, bz)) return true;
    return false;
  }
  update(w, dt, input) {
    const { p, v } = this, fly = this.fly, sp = fly ? (input.sprint ? 22 : 11) : input.sprint ? 7.2 : input.crouch ? 2 : 4.5;
    this.crouch = input.crouch && !fly;
    const wy = w.get(Math.floor(p[0]), Math.floor(p[1] + 0.9), Math.floor(p[2]));
    this.inWater = wy === B.water;
    const s = Math.sin(this.yaw), c = Math.cos(this.yaw);
    let mx = input.x * c + input.z * s, mz = -input.x * s + input.z * c; const l = Math.hypot(mx, mz);
    if (l > 1) { mx /= l; mz /= l; }
    const accel = fly ? 9 : this.ground ? 14 : this.inWater ? 5 : 3.5, sw = this.inWater && !fly ? 0.55 : 1;
    v[0] += (mx * sp * sw - v[0]) * Math.min(1, accel * dt); v[2] += (mz * sp * sw - v[2]) * Math.min(1, accel * dt);
    if (fly) v[1] += ((input.jump ? sp : 0) - (input.crouch ? sp : 0) - v[1]) * Math.min(1, 9 * dt);
    else if (this.inWater) { v[1] += (input.jump ? 3.5 : -1.2 - v[1]) * Math.min(1, 6 * dt); }
    else { v[1] -= GRAV * dt; if (input.jump && this.ground) { v[1] = JUMP; this.ground = false; } }
    if (v[1] < -40) v[1] = -40;
    // X et Z, avec montée automatique d'une marche quand on avance au sol
    for (const a of [0, 2]) {
      const n = p.slice(); n[a] += v[a] * dt;
      if (!this.box(w, n[0], n[1], n[2])) p[a] = n[a];
      else if (this.ground && !fly && !this.box(w, n[0], n[1] + 1.05, n[2]) && !this.box(w, p[0], p[1] + 1.05, p[2])) { p[a] = n[a]; p[1] += 1.02; this.stepped = 0.12; }
      else v[a] = 0;
    }
    const ny = p[1] + v[1] * dt;
    if (!this.box(w, p[0], ny, p[2])) { p[1] = ny; this.ground = false; }
    else {
      if (v[1] < 0) { this.ground = true; p[1] = Math.ceil(ny - 0.001); if (this.box(w, p[0], p[1], p[2])) p[1] = Math.floor(ny) + 1; }
      v[1] = 0;
    }
    this.speed = Math.hypot(v[0], v[2]);
    if (l > 0.05) { const t = Math.atan2(mx, mz); let d = t - this.moveYaw; d = Math.atan2(Math.sin(d), Math.cos(d)); this.moveYaw += d * Math.min(1, dt * 11); }
  }
}

// Parcours DDA : renvoie le premier bloc ciblable et la cellule d'avant.
export function raycast(w, o, d, max, skip) {
  let x = Math.floor(o[0]), y = Math.floor(o[1]), z = Math.floor(o[2]);
  const sx = Math.sign(d[0]), sy = Math.sign(d[1]), sz = Math.sign(d[2]);
  const tdx = Math.abs(1 / d[0]), tdy = Math.abs(1 / d[1]), tdz = Math.abs(1 / d[2]);
  let tx = (sx > 0 ? x + 1 - o[0] : o[0] - x) * tdx, ty = (sy > 0 ? y + 1 - o[1] : o[1] - y) * tdy, tz = (sz > 0 ? z + 1 - o[2] : o[2] - z) * tdz;
  let px = x, py = y, pz = z, t = 0;
  for (let i = 0; i < 80 && t <= max; i++) {
    const b = w.get(x, y, z);
    if (b > 0 && b !== B.water && !(skip && skip(x, y, z))) return { x, y, z, px, py, pz, id: b, t };
    px = x; py = y; pz = z;
    if (tx < ty && tx < tz) { x += sx; t = tx; tx += tdx; } else if (ty < tz) { y += sy; t = ty; ty += tdy; } else { z += sz; t = tz; tz += tdz; }
  }
  return null;
}
void IS_SOLID;
