// Produit engine/core/src/sounds_data.rs depuis les règles sonores validées à l'oreille sur le prototype.
import { writeFileSync } from 'node:fs';
globalThis.window = {};
const { SOUNDS, TOOL } = await import('../../src/game/audio.js');
const f = (v) => (Number.isInteger(v) ? v + '.0' : String(v));
const L = (l) => `L { set: "${l.set}", rate: ${f(l.rate)}, vol: ${f(l.vol)}, lp: ${f(l.lp)}, at: ${f(l.at)} }`;
const ls = (a) => `&[${a.map(L).join(', ')}]`;
const rows = Object.entries(SOUNDS).map(([k, s]) => `    ("${k}", Mat { m: "${s.m}", step: ${ls(s.step)}, hit: ${ls(s.hit)}, brk: ${ls(s.brk)}, put: ${ls(s.put)} }),`);
const tools = Object.entries(TOOL).flatMap(([t, m]) => Object.entries(m).map(([k, l]) => `    ("${t}", "${k}", ${l ? `Some(${L(l)})` : 'None'}),`));
writeFileSync(new URL('../core/src/sounds_data.rs', import.meta.url), `// Généré par engine/tools/gen_sounds.mjs.
use crate::audio::{Mat, L};
pub const SOUNDS: [(&str, Mat); ${rows.length}] = [
${rows.join('\n')}
];
pub const TOOLS: [(&str, &str, Option<L>); ${tools.length}] = [
${tools.join('\n')}
];
`);
