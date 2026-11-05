import test from 'node:test'; import assert from 'node:assert/strict';
import { ChunkStore } from '../src/world/store.js'; import { meshChunk } from '../src/world/mesher.js';
import { B, idx, orientedBlock, gazeAxis, matLayer, BLOCKS } from '../src/world/blocks.js';
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
test('orientation selon le regard : le cœur du bois fait face à la direction regardée', () => {
  const ax = (x, y, z, m, p) => gazeAxis(x, y, z, m, p), o = (id, x, y, z) => orientedBlock(id, ax(x, y, z));
  assert.equal(o(B.oak_log, 0.1, -0.9, 0.2), B.oak_log, 'regard vers le bas : verticale');
  assert.equal(o(B.oak_log, 0.8, -0.6, 0.1), B.oak_log_x, 'vers le sol à 37° : encore couché');
  assert.equal(o(B.oak_log, 0.9, -0.2, 0.1), B.oak_log_x); assert.equal(o(B.birch_log, -0.2, 0.1, -0.9), B.birch_log_z);
  assert.equal(o(B.planks, 0.1, 0, 0.9), B.planks_z); assert.equal(o(B.planks, 0.1, 0.95, 0), B.planks_y);
  assert.equal(o(B.stone, 0.9, 0, 0), B.stone, 'la pierre ne tourne pas');
  assert.equal(ax(0.71, 0, 0.7, 0, 'z'), 'z', 'pas de bascule à la diagonale'); assert.equal(ax(0.9, 0, 0.3, 0, 'z'), 'x');
  assert.equal(ax(0.3, 0.8, 0.1, 0, 'y'), 'y', 'pas de bascule au seuil vertical'); assert.equal(ax(0.3, 0.8, 0.1, 0, 'x'), 'x');
  assert.equal(ax(0.9, 0, 0.1, 1), 'y'); assert.equal(ax(0.1, 0.9, 0.3, 2), 'z'); assert.equal(ax(0.9, 0, 0.1, 3), 'z', 'en travers du regard');
  for (const [id, base] of [[B.oak_log_x, 'oak_log'], [B.birch_log_z, 'birch_log'], [B.planks_z, 'planks']]) assert.equal(BLOCKS[id].drop, base);
  // Un bloc isolé dans le ciel : on vérifie le matériau de chaque face.
  const s = new ChunkStore(5), y = 125, layersByNormal = (mesh, n) => { const i16 = new Int16Array(mesh.opaque), i8 = new Int8Array(mesh.opaque), out = new Set(); for (let v = 0; v < mesh.nO; v++) { const q = v * 20; if (i16[v * 10 + 1] / 128 >= y && i8[q + 8 + n] > 100) out.add(i16[v * 10 + 3]); } return out; };
  s.edit(0, 0, idx(5, y, 5), B.oak_log_x);
  const m = meshChunk((a, b) => s.get(a, b), 0, 0, 2);
  assert.deepEqual([...layersByNormal(m, 0)], [matLayer('end_oak')], 'cernes sur X'); assert.deepEqual([...layersByNormal(m, 1)], [matLayer('bark_oak')], 'écorce sur Y');
});
