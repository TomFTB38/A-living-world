import test from 'node:test'; import assert from 'node:assert/strict';
import { ChunkStore } from '../src/world/store.js'; import { meshChunk } from '../src/world/mesher.js';
import { B, idx } from '../src/world/blocks.js';
test('modifications prioritaires sur la génération', () => {
  const s = new ChunkStore(5, 2), i = idx(3, 90, 3);
  s.edit(0, 0, i, B.planks); s.get(1, 0); s.get(2, 0); s.get(3, 0); // purge le cache
  assert.equal(s.get(0, 0)[i], B.planks);
  const s2 = new ChunkStore(5); s2.setEdits([['0,0', [[i, B.stone]]]]); assert.equal(s2.get(0, 0)[i], B.stone);
});
test('mesh : les trois LOD se réduisent, et poser un bloc change le maillage', () => {
  const s = new ChunkStore(1234); const g = (x, z) => s.get(x, z);
  const t0 = performance.now(); const a = meshChunk(g, 0, 0, 0); const t1 = performance.now();
  const b = meshChunk(g, 0, 0, 1), c = meshChunk(g, 0, 0, 2); const t2 = performance.now();
  console.log('quads', a.nO / 4, a.nD / 4, a.nW / 4, '|', b.nO / 4, b.nD / 4, '|', c.nO / 4, c.nD / 4, 'ms', (t1 - t0).toFixed(0), ((t2 - t1) / 2).toFixed(0));
  assert.ok(a.nO + a.nD > b.nO + b.nD && b.nO + b.nD > c.nO + c.nD);
  s.edit(0, 0, idx(5, 120, 5), B.stone); const d = meshChunk(g, 0, 0, 2); assert.equal(d.nO, c.nO + 24);
});
