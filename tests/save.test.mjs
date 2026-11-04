import test from 'node:test'; import assert from 'node:assert/strict';
import { serialize, deserialize, atomicWrite, safeRead, SAVE_VERSION } from '../src/game/save.js';
const mem = () => { const m = new Map(); return { getItem: (k) => (m.has(k) ? m.get(k) : null), setItem: (k, v) => m.set(k, v), removeItem: (k) => m.delete(k), m }; };
const st = { id: 'w1', seed: 42, genVersion: 1, time: 12, day: 1, player: { p: [1, 2, 3], yaw: 0, pitch: 0 }, inventory: [], edits: [['0,0', [[5, 3]]]] };
test('aller-retour fidèle', () => { const b = deserialize(serialize(st)); assert.equal(b.version, SAVE_VERSION); assert.deepEqual(b.edits, st.edits); });
test('corruption détectée', () => { const t = serialize(st).replace('"seed\\":42', '"seed\\":43'); assert.throws(() => deserialize(t), /corrompue/); });
test('version future refusée', () => { const body = JSON.stringify({ version: 99 }); let h = 2166136261; for (let i = 0; i < body.length; i++) h = Math.imul(h ^ body.charCodeAt(i), 16777619); assert.throws(() => deserialize(JSON.stringify({ sum: (h >>> 0).toString(16), body })), /récente/); });
test('écriture interrompue : la version précédente survit', () => {
  const s = mem(); atomicWrite(s, 'k', serialize(st));
  s.setItem('k', 'déchet partiel'); s.setItem('k.tmp', serialize({ ...st, seed: 7 }));
  assert.equal(safeRead(s, 'k').seed, 7);
});
