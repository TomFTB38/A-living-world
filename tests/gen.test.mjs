import test from 'node:test'; import assert from 'node:assert/strict';
import { WorldGen } from '../src/world/gen.js'; import { B, CA, IS_SOLID } from '../src/world/blocks.js';
const hash = (v) => { let h = 2166136261; for (let i = 0; i < v.length; i++) h = Math.imul(h ^ v[i], 16777619); return h >>> 0; };
test('déterminisme indépendant de l\'ordre de chargement', () => {
  const coords = [[0,0],[1,0],[-3,2],[5,-4],[2,2]];
  const a = new WorldGen(1234), b = new WorldGen(1234);
  const ha = coords.map(([x,z]) => hash(a.genChunk(x,z)));
  const hb = new Array(coords.length);
  for (let i = coords.length-1; i >= 0; i--) hb[i] = hash(b.genChunk(...coords[i]));
  assert.deepEqual(ha, hb);
  // cache de tampons vidé : même résultat
  a.stamps.clear(); assert.equal(hash(a.genChunk(1,0)), ha[1]);
});
test('deux seeds différentes produisent deux mondes différents', () => {
  assert.notEqual(hash(new WorldGen(1).genChunk(0,0)), hash(new WorldGen(2).genChunk(0,0)));
});
test('un arbre franchit proprement la frontière des chunks', () => {
  const g = new WorldGen(77); let wood = 0, leaves = 0;
  for (let x=-3;x<3;x++) for (let z=-3;z<3;z++) { const v = g.genChunk(x,z); for (let i=0;i<CA;i++){ if(v[i]===B.oak_log||v[i]===B.birch_log||v[i]===B.beech_log) wood++; if(v[i]===B.oak_leaves||v[i]===B.birch_leaves||v[i]===B.beech_leaves) leaves++; } }
  assert.ok(wood > 500, 'bois '+wood); assert.ok(leaves > 3000, 'feuilles '+leaves);
});
test('le monde contient eau, grottes ou surplombs, et décor', () => {
  const g = new WorldGen(7); const c = {};
  let air_under = 0;
  for (let x=-8;x<8;x+=1) for (let z=-8;z<8;z+=1) { const v=g.genChunk(x,z); for(let i=0;i<CA;i++) c[v[i]]=(c[v[i]]||0)+1;
    for (let p=0;p<32*32;p++){ let seen=false; for(let y=127;y>=0;y--){const b=v[p+y*1024]; if(IS_SOLID[b]&&b<14) seen=true; else if(seen&&b===0){air_under++;break;}} } }
  console.log(Object.entries(B).filter(([n,i])=>c[i]).map(([n,i])=>n+':'+c[i]).join(' '), 'colonnes_avec_vide', air_under);
  assert.ok(c[B.water] > 100); assert.ok(c[B.tall_grass] > 1000); assert.ok(c[B.daisy] > 10); assert.ok(air_under > 0);
});
test('les arbres sont bien estampés loin de l\'origine, à z négatif', () => {
  const g = new WorldGen(20261104); let n = 0;
  for (let x = -7; x < -3; x++) for (let z = -6; z < -2; z++) { const v = g.genChunk(x, z); for (let i = 0; i < CA; i++) if (v[i] >= B.oak_leaves && v[i] <= B.beech_leaves) n++; }
  assert.ok(n > 20000, 'feuilles ' + n);
});
test('lianes, champignons, clôtures et lanternes sont générés et déterministes', () => {
  const g = new WorldGen(20261104), c = {};
  for (let x = -12; x < -6; x++) for (let z = -7; z < -1; z++) { const v = g.genChunk(x, z); for (let i = 0; i < CA; i++) c[v[i]] = (c[v[i]] || 0) + 1; }
  console.log('vigne', c[B.vine], 'champignon', c[B.shelf_fungus], 'clôture', c[B.fence], 'lanterne', c[B.lantern]);
  assert.ok(c[B.vine] > 100); assert.ok(c[B.shelf_fungus] > 3); assert.ok(c[B.fence] > 10);
  assert.equal(hash(new WorldGen(20261104).genChunk(-9, -4)), hash(g.genChunk(-9, -4)));
});
