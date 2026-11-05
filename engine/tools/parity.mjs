// Empreintes du prototype : engine/core/tests/parity.txt. Le moteur Rust doit retrouver exactement ces valeurs.
import { writeFileSync } from 'node:fs';
import { hashInt, Noise, Rng } from '../../src/core/noise.js';
import { WorldGen } from '../../src/world/gen.js';
import { ChunkStore } from '../../src/world/store.js';
import { meshChunk, farMesh } from '../../src/world/mesher.js';
import { buildFauna, Fauna } from '../../src/game/animals.js';
import { kindAt } from '../../src/game/weather.js';
import { B } from '../../src/world/blocks.js';
import { buildCharacter } from '../../src/game/character.js';
import { buildItems } from '../../src/game/items.js';
import { buildTextures, buildNoise3D } from '../../src/render/textures.js';
const fnv = (u8) => { let h = 0x811c9dc5; for (let i = 0; i < u8.length; i++) { h ^= u8[i]; h = Math.imul(h, 0x01000193); } return h >>> 0; };
const out = [], bits = (v) => { const d = new DataView(new ArrayBuffer(8)); d.setFloat64(0, v); return d.getBigUint64(0).toString(16); };
for (const v of [0.3, 0.9, 1.7, 2.5, 3.1415926, 4.71, 6.283, -0.7, -2.2, -5.5, 37.3, 141.7, -96.2, 1000.5]) out.push(`trig ${v} ${bits(Math.sin(v))} ${bits(Math.cos(v))}`);
for (const [a, b, c] of [[1, 2, 3], [0.1, 0.2, 0.9], [3.3, -7.7, 12.1]]) out.push(`hyp ${a} ${b} ${c} ${bits(Math.hypot(a, b))} ${bits(Math.hypot(a, b, c))}`);
out.push(`hash ${hashInt(-5, 7, 20261104, 21)}`);
const r = new Rng(12345); out.push(`rng ${bits(r.next())} ${bits(r.next())}`);
const n = new Noise(77); out.push(`noise ${bits(n.n2(12.5, -3.25))} ${bits(n.n3(1.2, 2.3, -3.4))} ${bits(n.fbm2(0.7, 0.2, 3))}`);
const stub = { mesh: (ab, n) => ({ ab, n }) };
const ch = buildCharacter(stub); out.push(`character ${ch.mesh.n} ${fnv(new Uint8Array(ch.mesh.ab))}`);
const it = buildItems(stub); for (const k in it) { const q = it[k], a = new Uint8Array(it.pickaxe.mesh.ab), st = q.offset / 24 * 4 * 20, n = q.count / 6 * 4; out.push(`item ${k} ${n} ${fnv(a.subarray(st, st + n * 20))}`); }
const fa = buildFauna(stub); for (const k in fa.rng) { const q = fa.rng[k], a = new Uint8Array(fa.mesh.ab), st = q.offset / 24 * 4 * 20, n = q.count / 6 * 4; out.push(`fauna ${k} ${n} ${fnv(a.subarray(st, st + n * 20))}`); }
for (const t of [0, 100, 300, 700, 1500, 4000, 9999, 20000]) out.push(`weather ${t} ${kindAt(20261104, t)}`);
{ // Parcours de la faune sur un petit monde fixe : même hasard, mêmes trajectoires.
  const w = { get(x, y, z) { if (Math.abs(x) > 300 || Math.abs(z) > 300 || y < 0) return -1; const h = 40 + ((((x >> 3) * 7 + (z >> 3) * 13) & 3)); if (y < h) return B.dirt; if ((x & 63) === 5 && (z & 63) === 9 && y < h + 5) return B.stone; return 0; } };
  const f = new Fauna(null, 20261104); let ev = 0; f.emit = () => { ev++; };
  for (let k = 0; k < 3600; k++) {
    const p = [0.5 + (Math.floor(k / 60) % 40), 44.2, 0.5];
    f.update(1 / 60, w, { p }, k > 2400 ? 0 : 1);
    if (k % 300 === 299) { let a = 0; for (const d of f.deer) a += d.p[0] + d.p[1] * 3 + d.p[2] * 7 + d.yaw; for (const b of f.birds) a += b.p[0] + b.p[1] * 3 + b.p[2] * 7; out.push(`ftrace ${k} ${f.deer.length} ${f.birds.length} ${ev} ${a}`); }
  }
}
for (const seed of [20261104, 7, -1234]) {
  const g = new WorldGen(seed);
  for (const [cx, cz] of [[0, 0], [-2, 1], [3, -4], [-1, 0], [5, 5], [-8, -9]]) out.push(`chunk ${seed} ${cx} ${cz} ${fnv(g.genChunk(cx, cz))}`);
}
for (const seed of [20261104, -1234]) {
  const st = new ChunkStore(seed, 200), get = (a, b) => st.get(a, b);
  for (const [cx, cz] of [[0, 0], [-2, 1], [3, -4], [-1, 0]]) for (const lod of [0, 1, 2]) {
    const m = meshChunk(get, cx, cz, lod), h = (b) => fnv(new Uint8Array(b));
    out.push(`mesh ${seed} ${cx} ${cz} ${lod} ${m.nO} ${m.nD} ${m.nW} ${h(m.opaque)} ${h(m.deco)} ${h(m.water)} ${m.ymin} ${m.ymax}`);
  }
  const f = farMesh(st.gen, 1, -2); out.push(`far ${seed} ${f.nF} ${fnv(new Uint8Array(f.far))}`);
}
const tx = buildTextures(128); out.push(`tex ${fnv(tx.albedo)} ${fnv(tx.nrm)}`);
out.push(`noise3 ${fnv(buildNoise3D(40).data)}`);
writeFileSync(new URL('../core/tests/parity.txt', import.meta.url), out.join('\n') + '\n');
console.log(out.length, 'empreintes');
