import test from 'node:test'; import assert from 'node:assert/strict';
import { SOUNDS, soundOf } from '../src/game/audio.js';
import { pickMood, MOODS } from '../src/game/music.js';
import { BLOCKS } from '../src/world/blocks.js';
const base = { hp: 100, food: 100, water: 100 };
test('chaque bloc a une signature sonore, et deux blocs ne partagent jamais la même', () => {
  for (const b of BLOCKS) if (b.name !== 'air') assert.ok(SOUNDS[b.name.replace(/_(x|y|z)$/, '')], b.name);
  const keys = Object.values(SOUNDS).map((s) => JSON.stringify(s)); assert.equal(new Set(keys).size, keys.length);
  assert.equal(soundOf('oak_log_x'), SOUNDS.oak_log);
});
test("l'humeur suit la situation, le danger d'abord", () => {
  assert.equal(pickMood(base), 'calme'); assert.equal(pickMood({ ...base, night: true }), 'nuit');
  assert.equal(pickMood({ ...base, building: true, night: true }), 'construction');
  assert.equal(pickMood({ ...base, underground: true, building: true }), 'suspense');
  assert.equal(pickMood({ ...base, hurt: true, underground: true }), 'danger'); assert.equal(pickMood({ ...base, enemy: true }), 'danger');
  assert.equal(pickMood({ ...base, food: 10 }), 'danger'); assert.equal(pickMood({ ...base, water: 10 }), 'danger');
  assert.equal(pickMood({ ...base, hp: 20 }), 'calme'); assert.equal(pickMood({ ...base, falling: true }), 'calme');
  for (const m of ['calme', 'nuit', 'construction', 'suspense', 'danger']) assert.ok(MOODS[m]);
});
