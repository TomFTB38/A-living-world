// Produit engine/core/src/blocks_data.rs depuis le registre du prototype : une seule source de vérité pendant le portage.
import { writeFileSync } from 'node:fs';
import { BLOCKS, B, VARIANTS, MATERIALS } from '../../src/world/blocks.js';
const f = (v) => (Number.isInteger(v) ? v + '.0' : String(v));
const o = (v) => (v === null || v === undefined ? 'None' : `Some("${v}")`);
const rows = BLOCKS.map((b) => `    Block { id: ${b.id}, name: "${b.name}", shape: Shape::${b.shape[0].toUpperCase() + b.shape.slice(1)}, hardness: ${f(b.hardness)}, faces: ${b.faces ? `[${b.faces}]` : '[0, 0, 0]'}, dir_faces: ${b.dirFaces ? `[${b.dirFaces}]` : '[0, 0, 0, 0]'}, grain: [${b.grain}], solid: ${b.solid}, opaque: ${b.opaque}, light: ${b.light}, tint: [${b.tint.map(f)}], model: ${o(b.model)}, cat: "${b.cat}", drop: ${o(b.drop)} },`);
const ids = Object.entries(B).filter(([k]) => !k.includes('@')).map(([k, v]) => `    pub const ${k.toUpperCase()}: crate::blocks::Id = ${v};`);
writeFileSync(new URL('../core/src/blocks_data.rs', import.meta.url), `// Généré par engine/tools/gen_blocks.mjs. Les identifiants sont stables : ils vivent dans les sauvegardes.
use crate::blocks::{Block, Shape};

pub const VARIANTS: u32 = ${VARIANTS};
pub const MATERIALS: [&str; ${MATERIALS.length}] = [${MATERIALS.map((m) => `"${m}"`).join(', ')}];

pub const BLOCKS: [Block; ${BLOCKS.length}] = [
${rows.join('\n')}
];

#[allow(non_snake_case)]
pub mod B {
${ids.join('\n')}
}
`);
