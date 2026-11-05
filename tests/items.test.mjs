import test from 'node:test'; import assert from 'node:assert/strict';
import { ITEMS, CATEGORIES, buildToolMesh } from '../src/game/items.js';
import { BLOCKS, B } from '../src/world/blocks.js';
test('catalogue créatif : chaque objet est classé une fois, et tous les blocs de base y sont', () => {
  const all = CATEGORIES.flatMap(([, l]) => l); assert.equal(new Set(all).size, all.length);
  for (const n of all) assert.ok(ITEMS[n], n);
  const variants = /_(x|z)$/; for (const b of BLOCKS) if (b.name !== 'air' && !variants.test(b.name)) assert.ok(all.includes(b.name), 'manque ' + b.name);
});
test('tout ce qui se récolte correspond à un objet', () => { for (const b of BLOCKS) if (b.drop && b.name !== 'air' && b.name !== 'water') assert.ok(ITEMS[b.drop], b.name + ' -> ' + b.drop); });
test('les outils ont un vrai volume', () => { for (const t of ['pickaxe', 'axe', 'sword']) assert.ok(buildToolMesh(t).n > 200, t); });
