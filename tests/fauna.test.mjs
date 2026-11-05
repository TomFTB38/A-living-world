import test from 'node:test'; import assert from 'node:assert/strict';
import { ChunkStore } from '../src/world/store.js'; import { Fauna } from '../src/game/animals.js';
import { CX, CH, idx, IS_SOLID, B } from '../src/world/blocks.js';
const world = () => { const s = new ChunkStore(20261104, 200); return { get(x, y, z) { if (y < 0) return B.stone; if (y >= CH) return 0; const cx = Math.floor(x / CX), cz = Math.floor(z / CX); return s.get(cx, cz)[idx(x - cx * CX, y, z - cz * CX)]; } }; };
test('après 3 minutes simulées, cerfs et oiseaux ne sont jamais dans un bloc', () => {
  const w = world(), f = new Fauna(null, 77), pl = { p: [-267.5, 53, -131.5] };
  let deerFrames = 0, inside = 0, birdFrames = 0, birdIn = 0, moved = 0, last = new Map(), levels = new Set();
  for (let i = 0; i < 180 * 30; i++) {
    pl.p[0] += Math.sin(i / 400) * 0.08; pl.p[2] += Math.cos(i / 300) * 0.08; // le joueur passe et fait fuir
    f.update(1 / 30, w, pl);
    for (const d of f.deer) {
      deerFrames++; levels.add(d.gy); if (!f.fits(w, d.p[0], d.gy ?? d.p[1], d.p[2], d.yaw, 2)) inside++;
      const l = last.get(d); if (l) moved += Math.hypot(d.p[0] - l[0], d.p[2] - l[1]); last.set(d, [d.p[0], d.p[2]]);
    }
    for (const b of f.birds) { birdFrames++; const c = w.get(Math.floor(b.p[0]), Math.floor(b.p[1] + 0.1), Math.floor(b.p[2])); if (IS_SOLID[c]) birdIn++; }
  }
  console.log('niveaux', levels.size, 'cerf', deerFrames, inside, 'distance', moved.toFixed(0), 'oiseau', birdFrames, birdIn);
  assert.ok(deerFrames > 1000); assert.ok(moved > 100, 'les cerfs bougent');
  assert.equal(inside, 0); assert.equal(birdIn, 0);
});

test('un cerf monte et redescend une marche d\'un bloc, sans entrer dedans', () => {
  // sol à y=9, marche à y=10 pour 20 <= x < 40
  const w = { get: (x, y) => { const top = x >= 20 && x < 40 ? 10 : 9; return y <= top ? B.dirt : 0; } };
  const f = new Fauna(null, 5), d = { p: [10.5, 10, 8.5], gy: 10, yaw: Math.PI / 2, state: 'walk', t: 1e9, phase: 0, head: 0 }, seen = new Set();
  f.update = null; f.rng = { range: (a) => a, next: () => 0.5 };
  const far = [1000, 10, 1000];
  for (let i = 0; i < 2600 && d.p[0] < 46; i++) { f.stepDeer(d, 1 / 30, w, far); d.yaw = Math.PI / 2; seen.add(d.gy); assert.ok(f.fits(w, d.p[0], d.gy, d.p[2], d.yaw, 2), 'dans un bloc à x=' + d.p[0]); }
  assert.ok(d.p[0] >= 46, 'bloqué à x=' + d.p[0]); assert.deepEqual([...seen].sort(), [10, 11]);
});
