// Empreintes du prototype : engine/core/tests/parity.txt. Le moteur Rust doit retrouver exactement ces valeurs.
import { writeFileSync } from 'node:fs';
import { hashInt, Noise, Rng } from '../../src/core/noise.js';
import { WorldGen } from '../../src/world/gen.js';
import { ChunkStore } from '../../src/world/store.js';
import { meshChunk, farMesh } from '../../src/world/mesher.js';
import { buildTextures } from '../../src/render/textures.js';
const fnv = (u8) => { let h = 0x811c9dc5; for (let i = 0; i < u8.length; i++) { h ^= u8[i]; h = Math.imul(h, 0x01000193); } return h >>> 0; };
const out = [], bits = (v) => { const d = new DataView(new ArrayBuffer(8)); d.setFloat64(0, v); return d.getBigUint64(0).toString(16); };
for (const v of [0.3, 0.9, 1.7, 2.5, 3.1415926, 4.71, 6.283, -0.7, -2.2, -5.5, 37.3, 141.7, -96.2, 1000.5]) out.push(`trig ${v} ${bits(Math.sin(v))} ${bits(Math.cos(v))}`);
for (const [a, b, c] of [[1, 2, 3], [0.1, 0.2, 0.9], [3.3, -7.7, 12.1]]) out.push(`hyp ${a} ${b} ${c} ${bits(Math.hypot(a, b))} ${bits(Math.hypot(a, b, c))}`);
out.push(`hash ${hashInt(-5, 7, 20261104, 21)}`);
const r = new Rng(12345); out.push(`rng ${bits(r.next())} ${bits(r.next())}`);
const n = new Noise(77); out.push(`noise ${bits(n.n2(12.5, -3.25))} ${bits(n.n3(1.2, 2.3, -3.4))} ${bits(n.fbm2(0.7, 0.2, 3))}`);
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
writeFileSync(new URL('../core/tests/parity.txt', import.meta.url), out.join('\n') + '\n');
console.log(out.length, 'empreintes');
