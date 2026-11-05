import test from 'node:test'; import assert from 'node:assert/strict';
import { Weather, kindAt, SLOT } from '../src/game/weather.js';
test('météo déterministe et variée', () => {
  assert.equal(kindAt(5, 1000), kindAt(5, 1000)); const seen = new Set(); for (let i = 0; i < 200; i++) seen.add(kindAt(9, i * SLOT));
  assert.deepEqual([...seen].sort(), ['brouillard', 'clair', 'nuageux', 'pluie']);
});
test('changement progressif : pas de bascule instantanée', () => {
  const w = new Weather(1); w.forced = 'pluie'; w.update(1 / 60); assert.ok(w.state.rain < 0.01);
  for (let i = 0; i < 60 * 60; i++) w.update(1 / 60); assert.ok(w.state.rain > 0.95);
});
