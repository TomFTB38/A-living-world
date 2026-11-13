//! Meubles et objets de décor : des modèles fins, bâtis dans le cube unité pour le sens 0 (le dos vers +z, la face vers
//! le joueur qui les pose), puis tournés au quart de tour (suffixe « @1 » à « @3 » du nom de bloc) et posés dans le chunk.
use crate::blocks::{by_name, BLOCKS};
use crate::mesher::{bx, kind, obox, VBuf, VSIZE};
use crate::noise::Rng;

type C = [f64; 3];
const WOOD: C = [0.46, 0.31, 0.17]; const DARK: C = [0.3, 0.19, 0.1]; const IRON: C = [0.17, 0.17, 0.19]; const GOLD: C = [0.85, 0.66, 0.26];
const FLAME: C = [2.6, 1.4, 0.4]; const CORE: C = [2.9, 2.1, 0.9]; const WAX: C = [0.92, 0.88, 0.76]; const CLAY: C = [0.66, 0.36, 0.22];

/// Les modèles de ce module.
pub const MODELS: [&str; 63] = ["cart", "trough", "coal_heap", "tool_rack", "fishing_net", "fish_rack", "creel", "mooring_post", "buoy", "net_pile", "anchor", "oars", "rope_rail", "sail", "pennant", "fish_crate", "boat_bench", "chest_open", "bed", "bed_foot", "torch_wall", "chair", "bench", "armchair", "cupboard", "fireplace", "painting", "banner_red", "banner_blue", "banner_green", "banner_gold", "weapon_rack", "wall_sconce",
    "loom", "spinning_wheel", "grindstone", "signpost", "throne", "curtain", "wall_shelf", "table", "round_table", "stool", "candle", "candelabra", "chandelier", "brazier", "sack",
    "amphora", "flower_pot", "cauldron", "bucket", "log_pile", "carpet_red", "carpet_blue", "carpet_green", "iron_bars", "potted_fern", "books_pile", "bottles", "beehive", "cheese", "bread_basket"];
pub fn has(model: &str) -> bool { MODELS.contains(&model) }
/// La couche de texture d'une matière (« rug », « hay »…) ou, à défaut, de la face d'un bloc du même nom.
fn layer(n: &str) -> f64 {
    if let Some(m) = crate::blocks::MATERIALS.iter().position(|x| *x == n) { return (m * crate::blocks::VARIANTS as usize) as f64; }
    by_name(n).map_or(0.0, |b| BLOCKS[b as usize].faces[0] as f64)
}

/// Un atelier de modèle : des boîtes dans le cube unité, en couleur unie, en matière, ou lumineuses.
struct K { o: VBuf, s: f64, l: f64 }
impl K {
    fn b(&mut self, bb: [f64; 6], c: C) { let size = (bb[3] - bb[0]).max(bb[4] - bb[1]).max(bb[5] - bb[2]); if let Some((ly, t)) = crate::mesher::auto_tex(c, size) { let h = [(bb[3] - bb[0]) * 0.5, (bb[4] - bb[1]) * 0.5, (bb[5] - bb[2]) * 0.5]; obox(&mut self.o, [bb[0] + h[0], bb[1] + h[1], bb[2] + h[2]], [[h[0], 0.0, 0.0], [0.0, h[1], 0.0], [0.0, 0.0, h[2]]], t, ly, self.s, self.l); return; } bx(&mut self.o, bb[0], bb[1], bb[2], bb[3], bb[4], bb[5], c[0], c[1], c[2], 0.0, self.s, self.l, kind::FLAT, true, 0.0, 255.0); }
    fn e(&mut self, bb: [f64; 6], c: C) { bx(&mut self.o, bb[0], bb[1], bb[2], bb[3], bb[4], bb[5], c[0], c[1], c[2], 0.0, self.s, 255.0, kind::EMIT, true, 0.0, 255.0); }
    fn t(&mut self, bb: [f64; 6], m: &str, c: C) { let h = [(bb[3] - bb[0]) * 0.5, (bb[4] - bb[1]) * 0.5, (bb[5] - bb[2]) * 0.5]; obox(&mut self.o, [bb[0] + h[0], bb[1] + h[1], bb[2] + h[2]], [[h[0], 0.0, 0.0], [0.0, h[1], 0.0], [0.0, 0.0, h[2]]], c, layer(m), self.s, self.l); }
    /// Boîte orientée (centre, demi-axes), couleur unie.
    fn r(&mut self, c: [f64; 3], a: [[f64; 3]; 3], col: C) { let mut a = a; let d = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1]) - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0]) + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]); if d < 0.0 { a[2] = [-a[2][0], -a[2][1], -a[2][2]]; } obox(&mut self.o, c, a, col, -1.0, self.s, self.l); }
    /// Une flamme : cœur clair dans une enveloppe orangée.
    fn flame(&mut self, x: f64, y: f64, z: f64, h: f64) { let w = h * 0.32; self.e([x - w, y, z - w, x + w, y + h, z + w], FLAME); self.e([x - w * 0.5, y + h * 0.15, z - w * 0.5, x + w * 0.5, y + h * 1.15, z + w * 0.5], CORE); }
    fn candle(&mut self, x: f64, y: f64, z: f64, h: f64) { self.b([x - 0.035, y, z - 0.035, x + 0.035, y + h, z + 0.035], WAX); self.b([x - 0.006, y + h, z - 0.006, x + 0.006, y + h + 0.03, z + 0.006], [0.1, 0.08, 0.06]); self.flame(x, y + h + 0.02, z, 0.09); }
}
fn tone(c: C, k: f64) -> C { [c[0] * k, c[1] * k, c[2] * k] }

/// Dessine le meuble `model` du bloc `name` en (x, y, z). Rend faux si ce n'est pas un modèle de ce module.
#[allow(clippy::too_many_arguments)]
pub fn draw(o: &mut VBuf, model: &str, name: &str, x: f64, y: f64, z: f64, s: f64, l: f64, r: &mut Rng) -> bool {
    if !has(model) { return false; }
    let f: u8 = name.rsplit_once('@').and_then(|(_, d)| d.parse().ok()).unwrap_or(0);
    let mut k = K { o: VBuf::new(600), s, l }; k.o.scale = o.scale;
    build(&mut k, model, r);
    // Quart de tour autour du centre du bloc, comme les formes de construction : (x, z) devient (z, 1 - x).
    for v in k.o.data.chunks_exact(VSIZE) {
        let mut v = v.to_vec(); let g = |v: &[u8], i: usize| i16::from_le_bytes([v[i], v[i + 1]]) as f64 / o.scale;
        let (mut px, py, mut pz) = (g(&v, 0), g(&v, 2), g(&v, 4)); let (mut nx, mut nz) = (v[8] as i8 as i32, v[10] as i8 as i32);
        for _ in 0..f % 4 { (px, pz) = (pz, 1.0 - px); (nx, nz) = (nz, -nx); }
        for (i, val) in [(0, px + x), (2, py + y), (4, pz + z)] { v[i..i + 2].copy_from_slice(&((val * o.scale).round() as i16).to_le_bytes()); }
        v[8] = nx as i8 as u8; v[10] = nz as i8 as u8; o.data.extend_from_slice(&v); o.n += 1;
    }
    true
}

fn build(k: &mut K, m: &str, r: &mut Rng) {
    const ROPE: C = [0.66, 0.56, 0.38]; const ROPE2: C = [0.52, 0.43, 0.28]; const NET: C = [0.55, 0.48, 0.34]; const CORK: C = [0.72, 0.5, 0.28]; const OLD: C = [0.4, 0.32, 0.24]; const SILVER: C = [0.7, 0.74, 0.78]; const SILVER2: C = [0.42, 0.48, 0.55];
    let fish = |k: &mut K, x: f64, y: f64, z: f64, hang: bool, rr: &mut Rng| { // un poisson : corps fuselé argenté, dos sombre, queue fourchue ; pendu tête en bas ou couché
        let t = rr.range(0.9, 1.1); let (bc, bd) = (tone(SILVER, t), tone(SILVER2, t));
        if hang { k.b([x - 0.03, y - 0.26, z - 0.012, x + 0.03, y - 0.06, z + 0.012], bc); k.b([x - 0.022, y - 0.24, z + 0.012, x + 0.022, y - 0.08, z + 0.016], bd); k.b([x - 0.04, y - 0.04, z - 0.008, x + 0.04, y, z + 0.008], bd); k.b([x - 0.004, y - 0.06, z - 0.004, x + 0.004, y + 0.02, z + 0.004], ROPE2); }
        else { k.b([x - 0.13, y, z - 0.035, x + 0.1, y + 0.05, z + 0.035], bc); k.b([x - 0.1, y + 0.05, z - 0.02, x + 0.08, y + 0.06, z + 0.02], bd); k.b([x + 0.1, y + 0.005, z - 0.05, x + 0.16, y + 0.04, z + 0.05], bd); k.b([x - 0.12, y + 0.02, z - 0.036, x - 0.1, y + 0.035, z - 0.03], [0.05, 0.05, 0.05]); }
    };
    match m {
        // Refonte des villages : charrette, abreuvoir, tas de charbon de bois, râtelier d'outils des champs.
        "cart" => { // charrette à bras : caisse de planches à ridelles, deux grandes roues à rayons, brancards posés au sol, un peu de foin
            k.b([0.12, 0.42, 0.08, 0.88, 0.48, 0.92], WOOD); for x in [0.12, 0.84] { k.b([x, 0.48, 0.08, x + 0.04, 0.7, 0.92], tone(WOOD, 0.9)); } for z in [0.08, 0.88] { k.b([0.12, 0.48, z, 0.88, 0.66, z + 0.04], tone(WOOD, 0.9)); }
            for i in 0..4 { let z = 0.18 + i as f64 * 0.2; k.b([0.1, 0.5, z, 0.14, 0.72, z + 0.03], tone(WOOD, 0.8)); k.b([0.86, 0.5, z, 0.9, 0.72, z + 0.03], tone(WOOD, 0.8)); }
            for sx in [0.02, 0.92] { // roue : jante en segments, moyeu, six rayons
                let (cy, cz, rr) = (0.36, 0.5, 0.34); for a in 0..12 { let t = a as f64 / 12.0 * std::f64::consts::TAU; let (y, z) = (cy + rr * t.sin(), cz + rr * t.cos()); k.b([sx, y - 0.04, z - 0.04, sx + 0.06, y + 0.04, z + 0.04], DARK); }
                for a in 0..6 { let t = a as f64 / 6.0 * std::f64::consts::PI; for j in 1..4 { let rj = j as f64 * 0.085; for sg in [-1.0, 1.0] { let (y, z) = (cy + sg * rj * t.sin(), cz + sg * rj * t.cos()); k.b([sx + 0.01, y - 0.018, z - 0.018, sx + 0.05, y + 0.018, z + 0.018], tone(WOOD, 0.85)); } } }
                k.b([sx - 0.01, cy - 0.06, cz - 0.06, sx + 0.07, cy + 0.06, cz + 0.06], IRON); }
            k.b([0.06, 0.32, 0.47, 0.94, 0.38, 0.53], tone(WOOD, 0.7)); // l'essieu
            for x in [0.26, 0.7] { k.b([x, 0.04, -0.02, x + 0.05, 0.1, 0.1], WOOD); k.b([x, 0.1, 0.06, x + 0.05, 0.42, 0.11], tone(WOOD, 0.9)); } // brancards posés au sol
            for _ in 0..14 { let x = r.range(0.2, 0.8); let z = r.range(0.2, 0.8); k.b([x - 0.08, 0.48, z - 0.06, x + 0.08, 0.48 + r.range(0.06, 0.16), z + 0.06], [0.82, 0.7, 0.36]); }
        }
        "trough" => { // abreuvoir de bois cerclé sur deux tréteaux, plein d'eau
            for x in [0.1, 0.8] { k.b([x, 0.0, 0.2, x + 0.1, 0.32, 0.8], tone(WOOD, 0.85)); }
            k.b([0.02, 0.3, 0.22, 0.98, 0.36, 0.78], WOOD); for z in [0.22, 0.72] { k.b([0.02, 0.36, z, 0.98, 0.6, z + 0.06], WOOD); } for x in [0.02, 0.92] { k.b([x, 0.36, 0.22, x + 0.06, 0.6, 0.78], WOOD); }
            for x in [0.2, 0.75] { k.b([x, 0.3, 0.2, x + 0.04, 0.61, 0.8], IRON); } k.b([0.08, 0.5, 0.28, 0.92, 0.53, 0.72], [0.22, 0.36, 0.44]);
        }
        "coal_heap" => { // tas de charbon de bois : des morceaux noirs luisants en dôme, quelques braises éteintes
            for _ in 0..40 { let a = r.range(0.0, std::f64::consts::TAU); let d = r.range(0.0, 0.42); let (x, z) = (0.5 + d * a.cos(), 0.5 + d * a.sin()); let top = 0.42 * (1.0 - (d / 0.45).powi(2)).max(0.0); let s = r.range(0.05, 0.1);
                k.b([x - s, (top - s).max(0.0), z - s, x + s, top + s * 0.6, z + s], tone([0.1, 0.1, 0.11], r.range(0.7, 1.4))); }
        }
        "tool_rack" => { // râtelier contre le mur : une traverse, fourche, râteau, faux, pelle, pendus
            k.b([0.04, 0.78, 0.86, 0.96, 0.86, 0.94], WOOD); for x in [0.08, 0.88] { k.b([x, 0.0, 0.88, x + 0.04, 0.9, 0.94], tone(WOOD, 0.85)); }
            let handle = tone(WOOD, 1.2);
            k.b([0.2, 0.05, 0.82, 0.24, 0.95, 0.86], handle); for t in 0..3 { let x = 0.17 + t as f64 * 0.035; k.b([x, 0.02, 0.81, x + 0.012, 0.22, 0.83], IRON); } k.b([0.16, 0.2, 0.81, 0.28, 0.23, 0.84], IRON); // fourche
            k.b([0.42, 0.05, 0.82, 0.46, 0.95, 0.86], handle); k.b([0.32, 0.06, 0.8, 0.56, 0.1, 0.84], tone(WOOD, 0.8)); for t in 0..6 { let x = 0.33 + t as f64 * 0.04; k.b([x, 0.0, 0.8, x + 0.012, 0.06, 0.82], tone(WOOD, 0.8)); } // râteau
            k.b([0.64, 0.15, 0.82, 0.68, 0.95, 0.86], handle); k.b([0.6, 0.02, 0.8, 0.72, 0.18, 0.83], IRON); // pelle
            k.b([0.82, 0.3, 0.82, 0.85, 0.95, 0.86], handle); k.b([0.66, 0.84, 0.8, 0.84, 0.88, 0.83], tone(IRON, 1.2)); // faux
        }
        // Mobilier de port : filets, séchoirs, nasses, bittes, bouées, ancre, avirons, garde-corps de corde, voiles, fanion.
        "fishing_net" => { // filet pendu contre le mur ou la clôture, en mailles lâches qui s'affaissent, flotteurs de liège en haut
            for i in 0..9 { let x = 0.06 + i as f64 * 0.11; let sag = |y: f64| 0.03 * (1.0 - (2.0 * y - 1.0).powi(2)); for j in 0..8 { let y = 0.1 + j as f64 * 0.11; k.b([x - 0.008, y, 0.9 - sag(y), x + 0.008, y + 0.11, 0.915 - sag(y)], NET); } }
            for j in 0..9 { let y = 0.1 + j as f64 * 0.11; k.b([0.04, y - 0.007, 0.89, 0.96, y + 0.007, 0.92], tone(NET, 0.9)); }
            for i in 0..5 { let x = 0.12 + i as f64 * 0.19; k.b([x - 0.04, 0.94, 0.88, x + 0.04, 0.99, 0.93], CORK); } k.b([0.0, 0.96, 0.89, 1.0, 0.98, 0.92], ROPE);
            for i in 0..3 { let x = 0.25 + i as f64 * 0.25; k.b([x - 0.02, 0.02, 0.86, x + 0.02, 0.1, 0.92], tone(NET, 0.8)); } // les plombs du bas
        }
        "fish_rack" => { // séchoir : deux montants, une perche, une rangée de poissons pendus
            for x in [0.06, 0.88] { k.b([x, 0.0, 0.44, x + 0.06, 0.95, 0.5], WOOD); } k.b([0.0, 0.88, 0.45, 1.0, 0.93, 0.49], tone(WOOD, 0.9)); k.b([0.2, 0.4, 0.45, 0.8, 0.43, 0.49], tone(WOOD, 0.8));
            for i in 0..6 { fish(k, 0.17 + i as f64 * 0.13, 0.88, 0.47, true, r); }
        }
        "creel" => { // nasse en osier : une demi-voûte de baguettes sur un cadre, l'entrée en entonnoir, une corde
            const WICK: C = [0.62, 0.48, 0.28];
            k.b([0.14, 0.0, 0.2, 0.86, 0.04, 0.8], tone(WICK, 0.85));
            for i in 0..7 { let a = i as f64 / 6.0 * std::f64::consts::PI; let (cx, cy) = (0.5 - 0.34 * a.cos(), 0.36 * a.sin()); k.b([cx - 0.035, cy, 0.2, cx + 0.035, cy + 0.05, 0.8], if i % 2 == 0 { WICK } else { tone(WICK, 0.85) }); }
            for z in [0.2, 0.42, 0.6, 0.78] { for i in 0..7 { let a = i as f64 / 6.0 * std::f64::consts::PI; let (cx, cy) = (0.5 - 0.35 * a.cos(), 0.37 * a.sin()); k.b([cx - 0.02, cy - 0.01, z, cx + 0.02, cy + 0.05, z + 0.02], tone(WICK, 0.75)); } }
            k.b([0.42, 0.12, 0.16, 0.58, 0.26, 0.2], [0.18, 0.12, 0.07]); k.b([0.46, 0.36, 0.5, 0.54, 0.42, 0.9], ROPE);
        }
        "mooring_post" => { // bitte d'amarrage : un gros pieu goudronné à tête bombée, une corde lovée autour
            const TAR: C = [0.2, 0.15, 0.11]; k.b([0.3, 0.0, 0.3, 0.7, 0.78, 0.7], TAR); k.b([0.26, 0.78, 0.26, 0.74, 0.86, 0.74], tone(TAR, 1.2)); k.b([0.32, 0.86, 0.32, 0.68, 0.9, 0.68], tone(TAR, 1.1));
            for (i, y) in [0.42, 0.48, 0.54].iter().enumerate() { let w = 0.25 - i as f64 * 0.01; k.b([0.5 - w, *y, 0.5 - w, 0.5 + w, y + 0.05, 0.5 + w], if i % 2 == 0 { ROPE } else { ROPE2 }); }
            k.b([0.66, 0.0, 0.4, 0.95, 0.06, 0.6], ROPE); k.b([0.72, 0.06, 0.45, 0.9, 0.09, 0.55], ROPE2); // la corde qui traîne
        }
        "buoy" => { // bouée : une sphère rouge et blanche en tranches, un anneau et un bout de corde
            let cols = [[0.85, 0.2, 0.15], [0.92, 0.9, 0.86], [0.85, 0.2, 0.15]];
            for (i, (y0, y1, rr)) in [(0.0, 0.08, 0.14), (0.08, 0.18, 0.22), (0.18, 0.32, 0.26), (0.32, 0.44, 0.22), (0.44, 0.52, 0.14)].iter().enumerate() { k.b([0.5 - rr, *y0, 0.5 - rr, 0.5 + rr, *y1, 0.5 + rr], cols[(i / 2).min(2)]); }
            k.b([0.47, 0.52, 0.47, 0.53, 0.62, 0.53], IRON); k.b([0.5, 0.0, 0.48, 0.92, 0.04, 0.52], ROPE);
        }
        "net_pile" => { // un tas de filets emmêlés, quelques flotteurs, sur le sol
            for _ in 0..9 { let (x, z, w, h) = (r.range(0.15, 0.85), r.range(0.15, 0.85), r.range(0.12, 0.26), r.range(0.05, 0.14)); k.b([x - w, 0.0, z - w * 0.8, x + w, h, z + w * 0.8], tone(NET, r.range(0.8, 1.05))); }
            for _ in 0..4 { let (x, z) = (r.range(0.15, 0.85), r.range(0.15, 0.85)); k.b([x - 0.04, 0.08, z - 0.04, x + 0.04, 0.15, z + 0.04], CORK); }
        }
        "anchor" => { // ancre de fer couchée contre le mur : la verge, l'organeau, le jas de bois, les deux pattes
            k.b([0.46, 0.06, 0.82, 0.54, 0.92, 0.9], IRON); k.b([0.42, 0.9, 0.8, 0.58, 0.98, 0.92], IRON); k.b([0.2, 0.72, 0.8, 0.8, 0.78, 0.9], DARK);
            for sx in [-1.0f64, 1.0] { for i in 0..4 { let t = i as f64 / 3.0; let (x, y) = (0.5 + sx * (0.08 + 0.26 * t), 0.06 + 0.22 * t * t); k.b([x - 0.05, y, 0.81, x + 0.05, y + 0.07, 0.89], IRON); } k.b([0.5 + sx * 0.36 - 0.05, 0.26, 0.8, 0.5 + sx * 0.36 + 0.05, 0.36, 0.9], tone(IRON, 1.3)); }
            k.b([0.3, 0.0, 0.7, 0.75, 0.04, 0.95], ROPE);
        }
        "oars" => { // deux avirons appuyés au mur, croisés
            for (x0, x1) in [(0.2, 0.62), (0.8, 0.38)] { for i in 0..10 { let t = i as f64 / 9.0; let x = x0 + (x1 - x0) * t; let y = t * 0.95; k.b([x - 0.025, y, 0.82, x + 0.025, y + 0.11, 0.88], if i < 3 { tone(WOOD, 1.15) } else { WOOD }); }
                k.b([x0 - 0.07, 0.0, 0.83, x0 + 0.07, 0.3, 0.87], tone(WOOD, 1.2)); }
        }
        "rope_rail" => { // garde-corps : deux poteaux aux bords, deux cordes tendues qui pendent un peu
            for x in [0.0, 0.88] { k.b([x, 0.0, 0.44, x + 0.12, 1.0, 0.56], tone(WOOD, 0.85)); k.b([x - 0.01, 0.98, 0.43, x + 0.13, 1.02, 0.57], tone(WOOD, 0.75)); }
            for (y, d) in [(0.88, 0.05), (0.52, 0.04)] { for i in 0..8 { let t = (i as f64 + 0.5) / 8.0; let sag = d * (1.0 - (2.0 * t - 1.0).powi(2)); let x = 0.1 + 0.8 * t; k.b([x - 0.055, y - sag, 0.475, x + 0.055, y - sag + 0.045, 0.525], if i % 2 == 0 { ROPE } else { ROPE2 }); } }
        }
        "sail" => { // un pan de voile écrue, gonflée par le vent, avec une couture et une pièce rapiécée
            const CLOTH: C = [0.88, 0.84, 0.74];
            for i in 0..10 { let x = 0.05 * 0.0 + i as f64 * 0.1; let belly = 0.12 * (1.0 - (2.0 * ((i as f64 + 0.5) / 10.0) - 1.0).powi(2)); k.b([x, 0.0, 0.46 - belly, x + 0.1, 1.0, 0.5 - belly], if i % 4 == 0 { tone(CLOTH, 0.9) } else { CLOTH }); }
            k.b([0.3, 0.35, 0.38, 0.55, 0.62, 0.42], [0.74, 0.62, 0.48]); k.b([0.0, 0.48, 0.42, 1.0, 0.5, 0.5], tone(CLOTH, 0.82));
        }
        "pennant" => { // une hampe et un fanion à deux pointes, bleu et blanc
            k.b([0.46, 0.0, 0.46, 0.54, 1.0, 0.54], DARK); k.b([0.44, 0.98, 0.44, 0.56, 1.02, 0.56], GOLD);
            for i in 0..8 { let x = 0.54 + i as f64 * 0.055; let w = 0.2 - i as f64 * 0.012; let fl = (i as f64 * 0.8).sin() * 0.025; k.b([x, 0.92 - w * 2.0, 0.48 + fl, x + 0.055, 0.96, 0.52 + fl], if i % 3 == 2 { [0.92, 0.9, 0.86] } else { [0.18, 0.3, 0.62] }); }
        }
        "fish_crate" => { // une caisse ouverte pleine de poissons sur de la glace pilée
            k.b([0.1, 0.0, 0.18, 0.9, 0.06, 0.82], DARK); for (a, b2) in [([0.1, 0.06, 0.18], [0.9, 0.32, 0.24]), ([0.1, 0.06, 0.76], [0.9, 0.32, 0.82]), ([0.1, 0.06, 0.24], [0.16, 0.32, 0.76]), ([0.84, 0.06, 0.24], [0.9, 0.32, 0.76])] { k.b([a[0], a[1], a[2], b2[0], b2[1], b2[2]], WOOD); }
            k.b([0.16, 0.06, 0.24, 0.84, 0.27, 0.76], [0.86, 0.9, 0.94]); for i in 0..7 { fish(k, r.range(0.28, 0.72), 0.27 + (i % 2) as f64 * 0.03, 0.3 + i as f64 * 0.065, false, r); }
        }
        "boat_bench" => { k.b([0.0, 0.0, 0.35, 1.0, 0.08, 0.65], OLD); } // banc de nage d'une barque, posé en travers

        // Le coffre ouvert : la caisse cerclée, l'intérieur sombre, le couvercle levé sur ses charnières, vers l'arrière.
        "chest_open" => {
            const CW: C = [0.55, 0.37, 0.2]; const CW2: C = [0.6, 0.41, 0.22]; const BAND: C = [0.24, 0.24, 0.25];
            k.b([0.08, 0.0, 0.14, 0.92, 0.08, 0.86], CW); k.b([0.08, 0.08, 0.14, 0.92, 0.56, 0.2], CW); k.b([0.08, 0.08, 0.8, 0.92, 0.56, 0.86], CW); k.b([0.08, 0.08, 0.2, 0.14, 0.56, 0.8], CW); k.b([0.86, 0.08, 0.2, 0.92, 0.56, 0.8], CW);
            k.b([0.14, 0.08, 0.2, 0.86, 0.1, 0.8], [0.16, 0.1, 0.06]);
            for a in [0.16, 0.8] { k.b([a, 0.0, 0.13, a + 0.05, 0.565, 0.87], BAND); }
            let th = 1.75f64; let (dz, dy) = ([0.0, th.sin(), th.cos()], [0.0, th.cos(), -th.sin()]); let hinge = [0.5, 0.56, 0.13];
            let c = [hinge[0], hinge[1] + dz[1] * 0.37 + dy[1] * 0.11, hinge[2] + dz[2] * 0.37 + dy[2] * 0.11];
            k.r(c, [[0.43, 0.0, 0.0], [0.0, dy[1] * 0.11, dy[2] * 0.11], [0.0, dz[1] * 0.37, dz[2] * 0.37]], CW2);
            for a in [0.185, 0.825] { let cc = [a, c[1] + dy[1] * 0.005, c[2] + dy[2] * 0.005]; k.r(cc, [[0.026, 0.0, 0.0], [0.0, dy[1] * 0.115, dy[2] * 0.115], [0.0, dz[1] * 0.372, dz[2] * 0.372]], BAND); }
            k.b([0.44, 0.42, 0.86, 0.56, 0.54, 0.89], [0.7, 0.58, 0.3]);
        }
        // Le lit, deux blocs de long : la tête contre le mur (+z), le pied vers la pièce. Cadre de chêne, matelas de paille, drap, couverture de laine, oreiller.
        "bed" | "bed_foot" => {
            let head = m == "bed"; const SHEET: C = [0.86, 0.83, 0.74]; const WOOL: C = [0.5, 0.14, 0.12]; const WOOL2: C = [0.62, 0.22, 0.16];
            let (z0, z1) = if head { (0.0, 0.98) } else { (0.02, 1.0) };
            for x in [0.02, 0.9] { k.b([x, 0.18, z0, x + 0.08, 0.32, z1], WOOD); }
            for (x, z) in [(0.02, if head { 0.88 } else { 0.02 }), (0.9, if head { 0.88 } else { 0.02 })] { k.b([x, 0.0, z, x + 0.08, if head { 0.95 } else { 0.62 }, z + 0.1], DARK); }
            if head { k.t([0.1, 0.3, 0.9, 0.9, 0.86, 0.96], "planks", [1.0; 3]); k.b([0.04, 0.86, 0.88, 0.96, 0.92, 0.98], DARK); }
            else { k.t([0.1, 0.24, 0.03, 0.9, 0.56, 0.09], "planks", [1.0; 3]); k.b([0.04, 0.56, 0.02, 0.96, 0.62, 0.1], DARK); }
            k.b([0.08, 0.3, if head { 0.0 } else { 0.09 }, 0.92, 0.42, if head { 0.9 } else { 1.0 }], [0.78, 0.68, 0.44]);
            if head { k.b([0.07, 0.42, 0.0, 0.93, 0.45, 0.88], SHEET); k.b([0.16, 0.45, 0.56, 0.84, 0.56, 0.86], [0.93, 0.91, 0.86]); k.b([0.06, 0.45, 0.0, 0.94, 0.5, 0.36], WOOL); k.b([0.06, 0.5, 0.3, 0.94, 0.52, 0.38], SHEET); }
            else { k.b([0.06, 0.42, 0.09, 0.94, 0.5, 1.0], WOOL); for x in [0.05, 0.93] { k.b([x, 0.28, 0.1, x + 0.02, 0.5, 1.0], WOOL2); } k.b([0.06, 0.28, 0.09, 0.94, 0.5, 0.11], WOOL2); }
        }
        "torch_wall" => {
            // Une patte de fer scellée au mur, la torche penchée de 30° vers la pièce, la tête de fibres, la flamme.
            k.b([0.44, 0.34, 0.9, 0.56, 0.5, 1.0], IRON); k.b([0.46, 0.4, 0.74, 0.54, 0.44, 0.92], IRON);
            let (a, l) = (0.52f64, 0.62); let (dy, dz) = (a.cos(), -a.sin()); let c = [0.5, 0.2 + dy * l * 0.5, 0.94 + dz * l * 0.5];
            k.r(c, [[0.04, 0.0, 0.0], [0.0, dy * l * 0.5, dz * l * 0.5], [0.0, -dz * 0.04, dy * 0.04]], [0.42, 0.28, 0.15]);
            let top = [0.5, 0.2 + dy * l, 0.94 + dz * l]; k.r([top[0], top[1] - 0.04, top[2] + 0.02], [[0.07, 0.0, 0.0], [0.0, dy * 0.07, dz * 0.07], [0.0, -dz * 0.07, dy * 0.07]], [0.3, 0.24, 0.14]);
            k.flame(top[0], top[1] + 0.02, top[2], 0.24); k.flame(top[0] + 0.02, top[1] + 0.02, top[2] - 0.03, 0.17);
        }
        "chair" => {
            for (a, b) in [(0.18, 0.18), (0.76, 0.18), (0.18, 0.76), (0.76, 0.76)] { k.b([a, 0.0, b, a + 0.06, 0.46, b + 0.06], DARK); }
            k.t([0.14, 0.46, 0.14, 0.86, 0.52, 0.86], "planks", [1.0; 3]);
            for a in [0.18, 0.76] { k.b([a, 0.52, 0.76, a + 0.06, 1.08, 0.82], DARK); }
            for h in [0.68, 0.84, 1.0] { k.b([0.2, h, 0.77, 0.8, h + 0.07, 0.81], WOOD); }
            k.b([0.2, 0.16, 0.2, 0.8, 0.2, 0.24], DARK);
        }
        "bench" => {
            k.t([0.02, 0.42, 0.22, 0.98, 0.5, 0.78], "planks", [1.0; 3]);
            for a in [0.1, 0.8] { k.b([a, 0.0, 0.26, a + 0.1, 0.42, 0.74], DARK); k.b([a - 0.02, 0.0, 0.22, a + 0.12, 0.05, 0.78], DARK); }
            k.b([0.1, 0.18, 0.47, 0.9, 0.24, 0.53], WOOD);
        }
        "armchair" | "throne" => {
            let grand = m == "throne"; let (fr, cu) = if grand { (GOLD, [0.5, 0.08, 0.08]) } else { (DARK, [0.45, 0.12, 0.1]) };
            k.b([0.1, 0.0, 0.1, 0.9, 0.42, 0.9], tone(fr, 0.8)); k.b([0.14, 0.42, 0.12, 0.86, 0.56, 0.84], cu);
            k.b([0.1, 0.42, 0.8, 0.9, if grand { 1.45 } else { 1.12 }, 0.92], fr); k.b([0.16, 0.56, 0.76, 0.84, if grand { 1.32 } else { 1.04 }, 0.8], cu);
            for a in [0.06, 0.82] { k.b([a, 0.42, 0.12, a + 0.12, 0.72, 0.88], fr); }
            if grand { for a in [0.1, 0.84] { k.b([a, 1.45, 0.82, a + 0.06, 1.6, 0.9], GOLD); } k.b([0.44, 1.45, 0.82, 0.56, 1.65, 0.9], GOLD); }
        }
        "cupboard" => {
            k.t([0.04, 0.0, 0.18, 0.96, 0.96, 0.96], "planks", [0.8, 0.75, 0.72]); k.b([0.0, 0.94, 0.12, 1.0, 1.0, 1.0], DARK); k.b([0.02, 0.0, 0.16, 0.98, 0.06, 0.98], DARK);
            for (x0, x1) in [(0.08, 0.49), (0.51, 0.92)] { k.t([x0, 0.1, 0.15, x1, 0.9, 0.18], "planks", [0.95, 0.88, 0.8]); k.b([x0 + 0.05, 0.16, 0.14, x1 - 0.05, 0.84, 0.155], tone(WOOD, 0.8)); }
            for x in [0.44, 0.53] { k.b([x, 0.46, 0.12, x + 0.03, 0.56, 0.15], IRON); }
        }
        "fireplace" => {
            let st = "stone_bricks";
            k.t([0.0, 0.0, 0.0, 0.18, 1.0, 1.0], st, [1.0; 3]); k.t([0.82, 0.0, 0.0, 1.0, 1.0, 1.0], st, [1.0; 3]); k.t([0.0, 0.0, 0.7, 1.0, 1.0, 1.0], st, [0.8, 0.8, 0.8]);
            k.t([0.0, 0.7, 0.0, 1.0, 1.0, 0.7], st, [1.0; 3]); k.b([-0.04, 0.68, -0.06, 1.04, 0.78, 0.4], DARK); k.t([0.18, 0.0, 0.0, 0.82, 0.06, 0.7], "cobble", [0.6, 0.6, 0.6]);
            for (a, c) in [(0.3, 0.3), (0.42, 0.45)] { k.r([0.5, 0.11, c], [[0.26, 0.0, 0.0], [0.0, 0.05, 0.0], [0.0, 0.0, 0.05]], [0.3, 0.2, 0.12]); let _ = a; }
            k.e([0.32, 0.06, 0.32, 0.68, 0.1, 0.55], [1.6, 0.5, 0.12]); k.flame(0.42, 0.12, 0.42, 0.26); k.flame(0.56, 0.12, 0.4, 0.2); k.flame(0.5, 0.12, 0.5, 0.32);
            k.b([0.3, 0.78, 0.2, 0.36, 0.9, 0.26], CLAY); k.b([0.62, 0.78, 0.22, 0.7, 0.86, 0.3], [0.7, 0.66, 0.6]);
        }
        "painting" => {
            k.b([0.06, 0.1, 0.93, 0.94, 0.9, 1.0], GOLD); k.b([0.1, 0.14, 0.92, 0.9, 0.86, 0.93], [0.2, 0.1, 0.05]);
            let sky = [0.55, 0.68, 0.85]; k.b([0.13, 0.52, 0.915, 0.87, 0.83, 0.925], sky); k.b([0.13, 0.4, 0.913, 0.87, 0.55, 0.923], [0.42, 0.5, 0.58]);
            k.b([0.13, 0.17, 0.912, 0.87, 0.42, 0.922], [0.3, 0.45, 0.2]); k.b([0.4, 0.3, 0.91, 0.62, 0.48, 0.92], [0.85, 0.82, 0.74]); k.b([0.38, 0.48, 0.908, 0.64, 0.56, 0.918], [0.6, 0.2, 0.12]);
            k.b([0.7, 0.68, 0.91, 0.78, 0.76, 0.92], [1.0, 0.9, 0.6]);
        }
        "banner_red" | "banner_blue" | "banner_green" | "banner_gold" => {
            let c = match m { "banner_red" => [0.6, 0.1, 0.09], "banner_blue" => [0.12, 0.2, 0.5], "banner_green" => [0.16, 0.38, 0.16], _ => [0.82, 0.6, 0.18] };
            let emb = if m == "banner_gold" { [0.5, 0.1, 0.08] } else { GOLD };
            k.b([0.02, 0.92, 0.88, 0.98, 0.97, 0.94], DARK); for a in [0.0, 0.94] { k.b([a, 0.9, 0.86, a + 0.06, 0.99, 0.96], GOLD); }
            k.b([0.14, 0.18, 0.9, 0.86, 0.92, 0.93], c); k.b([0.14, 0.02, 0.9, 0.45, 0.18, 0.93], c); k.b([0.55, 0.02, 0.9, 0.86, 0.18, 0.93], c);
            k.b([0.46, 0.3, 0.887, 0.54, 0.8, 0.9], emb); k.b([0.3, 0.6, 0.887, 0.7, 0.68, 0.9], emb); k.b([0.14, 0.86, 0.887, 0.86, 0.89, 0.9], emb);
        }
        "weapon_rack" => {
            k.t([0.04, 0.1, 0.9, 0.96, 0.92, 0.98], "planks", [0.7, 0.62, 0.55]); for y in [0.35, 0.75] { for x in [0.15, 0.5, 0.85] { k.b([x - 0.03, y, 0.8, x + 0.03, y + 0.05, 0.9], DARK); } }
            k.b([0.2, 0.15, 0.84, 0.26, 0.95, 0.87], [0.7, 0.72, 0.76]); k.b([0.13, 0.3, 0.83, 0.33, 0.34, 0.88], GOLD); k.b([0.21, 0.18, 0.84, 0.25, 0.3, 0.87], DARK);
            k.b([0.48, 0.05, 0.84, 0.52, 1.05, 0.87], WOOD); k.b([0.46, 1.05, 0.84, 0.54, 1.2, 0.87], [0.7, 0.72, 0.76]);
            k.b([0.76, 0.12, 0.84, 0.8, 0.85, 0.87], WOOD); k.b([0.8, 0.6, 0.83, 0.96, 0.82, 0.88], [0.6, 0.62, 0.66]);
        }
        "wall_sconce" => {
            k.b([0.42, 0.3, 0.92, 0.58, 0.62, 1.0], IRON); k.b([0.47, 0.4, 0.6, 0.53, 0.45, 0.94], IRON); k.b([0.4, 0.42, 0.56, 0.6, 0.46, 0.7], IRON); k.candle(0.5, 0.46, 0.63, 0.16);
        }
        "loom" => {
            for (a, b) in [(0.05, 0.1), (0.89, 0.1), (0.05, 0.8), (0.89, 0.8)] { k.b([a, 0.0, b, a + 0.06, 1.0, b + 0.06], WOOD); }
            for y in [0.08, 0.95] { k.b([0.05, y, 0.1, 0.95, y + 0.05, 0.16], DARK); k.b([0.05, y, 0.8, 0.95, y + 0.05, 0.86], DARK); }
            k.b([0.1, 0.42, 0.2, 0.9, 0.47, 0.8], [0.62, 0.5, 0.35]); for i in 0..16 { let x = 0.12 + i as f64 * 0.05; k.b([x, 0.47, 0.5, x + 0.01, 0.95, 0.51], [0.85, 0.82, 0.74]); }
            k.b([0.12, 0.47, 0.49, 0.88, 0.7, 0.5], [0.55, 0.15, 0.12]); k.b([0.1, 0.25, 0.1, 0.9, 0.3, 0.16], WOOD);
        }
        "spinning_wheel" => {
            k.b([0.1, 0.18, 0.25, 0.9, 0.26, 0.75], WOOD); for (a, b) in [(0.12, 0.3), (0.12, 0.65), (0.82, 0.48)] { k.b([a, 0.0, b, a + 0.05, 0.2, b + 0.05], DARK); }
            k.b([0.3, 0.26, 0.47, 0.35, 0.7, 0.53], DARK); let (cx, cy, rr) = (0.4, 0.7, 0.28);
            for i in 0..16 { let a = i as f64 * std::f64::consts::TAU / 16.0; let (ca, sa) = (a.cos(), a.sin()); k.r([cx + ca * rr, cy + sa * rr, 0.5], [[-sa * 0.06, ca * 0.06, 0.0], [ca * 0.025, sa * 0.025, 0.0], [0.0, 0.0, 0.025]], WOOD); }
            for i in 0..4 { let a = i as f64 * std::f64::consts::FRAC_PI_4 * 2.0 + 0.3; k.r([cx, cy, 0.5], [[a.cos() * rr, a.sin() * rr, 0.0], [-a.sin() * 0.012, a.cos() * 0.012, 0.0], [0.0, 0.0, 0.012]], DARK); }
            k.b([0.7, 0.26, 0.45, 0.76, 0.6, 0.55], DARK); k.b([0.68, 0.55, 0.42, 0.82, 0.62, 0.58], [0.85, 0.8, 0.66]);
        }
        "grindstone" => {
            for a in [0.15, 0.78] { k.b([a, 0.0, 0.2, a + 0.07, 0.62, 0.8], WOOD); } k.b([0.15, 0.1, 0.45, 0.85, 0.16, 0.55], DARK);
            for i in 0..8 { let a = i as f64 * std::f64::consts::PI / 8.0; k.r([0.5, 0.55, 0.5], [[0.08, 0.0, 0.0], [0.0, a.cos() * 0.34, a.sin() * 0.34], [0.0, -a.sin() * 0.14, a.cos() * 0.14]], [0.62, 0.6, 0.55]); }
            k.b([0.08, 0.53, 0.47, 0.92, 0.57, 0.53], IRON); k.b([0.04, 0.5, 0.47, 0.08, 0.72, 0.53], IRON); k.b([0.0, 0.68, 0.45, 0.08, 0.76, 0.55], DARK);
        }
        "signpost" => {
            k.b([0.45, 0.0, 0.45, 0.55, 1.15, 0.55], WOOD); k.b([0.43, 1.12, 0.43, 0.57, 1.18, 0.57], DARK);
            k.r([0.6, 0.9, 0.5], [[0.32, 0.0, 0.0], [0.0, 0.08, 0.0], [0.0, 0.0, 0.025]], [0.6, 0.45, 0.28]); k.r([0.36, 0.7, 0.5], [[0.0, 0.0, 0.3], [0.0, 0.08, 0.0], [0.025, 0.0, 0.0]], [0.55, 0.4, 0.25]);
            k.b([0.88, 0.85, 0.48, 0.96, 0.95, 0.52], [0.6, 0.45, 0.28]);
        }
        "curtain" => {
            k.b([0.0, 0.94, 0.86, 1.0, 0.98, 0.9], IRON); let c = [0.5, 0.1, 0.1];
            for i in 0..10 { let x = i as f64 * 0.1; let d = if i % 2 == 0 { 0.82 } else { 0.86 }; k.b([x, 0.02, d, x + 0.1, 0.94, d + 0.05], tone(c, if i % 2 == 0 { 1.0 } else { 0.78 })); }
            k.b([0.0, 0.5, 0.8, 0.2, 0.56, 0.92], GOLD);
        }
        "wall_shelf" => {
            k.t([0.02, 0.48, 0.66, 0.98, 0.54, 1.0], "planks", [0.8, 0.75, 0.7]); for a in [0.12, 0.84] { k.b([a, 0.3, 0.9, a + 0.04, 0.48, 1.0], IRON); }
            k.b([0.1, 0.54, 0.74, 0.26, 0.76, 0.9], CLAY); k.b([0.13, 0.76, 0.77, 0.23, 0.8, 0.87], tone(CLAY, 0.8));
            k.b([0.34, 0.54, 0.76, 0.46, 0.7, 0.88], [0.25, 0.4, 0.3]); k.b([0.38, 0.7, 0.8, 0.42, 0.76, 0.84], [0.6, 0.45, 0.3]);
            for (i, c) in [[0.45, 0.1, 0.08], [0.12, 0.2, 0.38], [0.2, 0.32, 0.15]].iter().enumerate() { let x = 0.58 + i as f64 * 0.09; k.b([x, 0.54, 0.72, x + 0.08, 0.8 - i as f64 * 0.03, 0.94], *c); }
        }
        "table" => {
            k.t([0.0, 0.88, 0.0, 1.0, 0.96, 1.0], "planks", [1.0; 3]); k.b([0.04, 0.82, 0.04, 0.96, 0.88, 0.96], DARK);
            for (a, b) in [(0.06, 0.06), (0.86, 0.06), (0.06, 0.86), (0.86, 0.86)] { k.b([a, 0.0, b, a + 0.08, 0.82, b + 0.08], DARK); }
            k.b([0.1, 0.16, 0.47, 0.9, 0.22, 0.53], WOOD);
        }
        "round_table" => {
            for a in [0.0f64, 0.785] { k.r([0.5, 0.88, 0.5], [[a.cos() * 0.46, 0.0, a.sin() * 0.46], [0.0, 0.04, 0.0], [-a.sin() * 0.46, 0.0, a.cos() * 0.46]], WOOD); }
            k.b([0.44, 0.06, 0.44, 0.56, 0.84, 0.56], DARK); for a in [0.0f64, 1.571] { k.r([0.5, 0.04, 0.5], [[a.cos() * 0.3, 0.0, a.sin() * 0.3], [0.0, 0.04, 0.0], [-a.sin() * 0.06, 0.0, a.cos() * 0.06]], DARK); }
        }
        "stool" => {
            for a in [0.0f64, 0.785] { k.r([0.5, 0.5, 0.5], [[a.cos() * 0.2, 0.0, a.sin() * 0.2], [0.0, 0.035, 0.0], [-a.sin() * 0.2, 0.0, a.cos() * 0.2]], WOOD); }
            for i in 0..3 { let a = i as f64 * 2.094; let (cx, cz) = (0.5 + a.cos() * 0.13, 0.5 + a.sin() * 0.13); k.b([cx - 0.03, 0.0, cz - 0.03, cx + 0.03, 0.47, cz + 0.03], DARK); }
        }
        "candle" => { k.b([0.38, 0.0, 0.38, 0.62, 0.03, 0.62], [0.55, 0.42, 0.2]); k.candle(0.5, 0.03, 0.5, 0.26); }
        "candelabra" => {
            k.b([0.36, 0.0, 0.36, 0.64, 0.05, 0.64], GOLD); k.b([0.47, 0.05, 0.47, 0.53, 0.55, 0.53], GOLD); k.b([0.2, 0.52, 0.47, 0.8, 0.56, 0.53], GOLD);
            for x in [0.22, 0.78] { k.b([x - 0.03, 0.56, 0.47, x + 0.03, 0.6, 0.53], GOLD); k.candle(x, 0.6, 0.5, 0.16); } k.b([0.46, 0.55, 0.46, 0.54, 0.62, 0.54], GOLD); k.candle(0.5, 0.62, 0.5, 0.2);
        }
        "chandelier" => {
            k.b([0.48, 0.55, 0.48, 0.52, 1.0, 0.52], IRON);
            for i in 0..8 { let a = i as f64 * std::f64::consts::TAU / 8.0; let (ca, sa) = (a.cos(), a.sin()); k.r([0.5 + ca * 0.36, 0.42, 0.5 + sa * 0.36], [[-sa * 0.15, 0.0, ca * 0.15], [0.0, 0.04, 0.0], [ca * 0.04, 0.0, sa * 0.04]], DARK); }
            for i in 0..4 { let a = i as f64 * std::f64::consts::FRAC_PI_2 + 0.39; let (cx, cz) = (0.5 + a.cos() * 0.36, 0.5 + a.sin() * 0.36); k.b([0.48, 0.44, 0.48, cx.max(0.52), 0.47, cz.max(0.52)].map(|v| v), IRON); k.candle(cx, 0.46, cz, 0.12); }
            for i in 0..4 { let a = i as f64 * std::f64::consts::FRAC_PI_2; let (cx, cz) = (0.5 + a.cos() * 0.34, 0.5 + a.sin() * 0.34); k.r([(cx + 0.5) * 0.5, 0.72, (cz + 0.5) * 0.5], [[(cx - 0.5) * 0.5, 0.28, (cz - 0.5) * 0.5], [0.008, 0.0, 0.0], [0.0, 0.0, 0.008]], IRON); }
        }
        "brazier" => {
            for i in 0..3 { let a = i as f64 * 2.094; let (cx, cz) = (0.5 + a.cos() * 0.3, 0.5 + a.sin() * 0.3); k.r([(cx + 0.5) * 0.5, 0.3, (cz + 0.5) * 0.5], [[(cx - 0.5) * 0.5, -0.3, (cz - 0.5) * 0.5], [0.025, 0.0, 0.0], [0.0, 0.0, 0.025]], IRON); }
            k.b([0.22, 0.55, 0.22, 0.78, 0.6, 0.78], IRON); for (a, b) in [(0.18, 0.18), (0.18, 0.78)] { let _ = b; k.b([a, 0.6, 0.18, a + 0.04, 0.78, 0.82], IRON); k.b([0.78, 0.6, 0.18, 0.82, 0.78, 0.82], IRON); }
            k.b([0.18, 0.6, 0.18, 0.82, 0.78, 0.22], IRON); k.b([0.18, 0.6, 0.78, 0.82, 0.78, 0.82], IRON);
            k.e([0.22, 0.6, 0.22, 0.78, 0.7, 0.78], [1.7, 0.45, 0.1]); k.flame(0.4, 0.68, 0.45, 0.24); k.flame(0.6, 0.68, 0.55, 0.3); k.flame(0.5, 0.68, 0.38, 0.2);
        }
        "sack" => {
            let c = [0.62, 0.52, 0.36]; k.b([0.2, 0.0, 0.22, 0.8, 0.45, 0.78], c); k.b([0.24, 0.45, 0.26, 0.76, 0.6, 0.74], tone(c, 0.95)); k.b([0.34, 0.6, 0.36, 0.66, 0.68, 0.64], tone(c, 0.9));
            k.b([0.38, 0.68, 0.4, 0.62, 0.72, 0.6], [0.4, 0.3, 0.18]); k.b([0.36, 0.72, 0.38, 0.64, 0.8, 0.62], tone(c, 0.85));
        }
        "amphora" => {
            for (y0, y1, rr) in [(0.0, 0.06, 0.1), (0.06, 0.2, 0.18), (0.2, 0.5, 0.24), (0.5, 0.66, 0.18), (0.66, 0.8, 0.09), (0.8, 0.86, 0.12)] { k.b([0.5 - rr, y0, 0.5 - rr, 0.5 + rr, y1, 0.5 + rr], CLAY); }
            k.b([0.28, 0.58, 0.47, 0.33, 0.78, 0.53], tone(CLAY, 0.9)); k.b([0.67, 0.58, 0.47, 0.72, 0.78, 0.53], tone(CLAY, 0.9)); k.b([0.25, 0.35, 0.25, 0.75, 0.39, 0.75], [0.25, 0.12, 0.07]);
        }
        "flower_pot" | "potted_fern" => {
            k.b([0.3, 0.0, 0.3, 0.7, 0.3, 0.7], CLAY); k.b([0.27, 0.27, 0.27, 0.73, 0.33, 0.73], tone(CLAY, 1.1)); k.b([0.32, 0.3, 0.32, 0.68, 0.31, 0.68], [0.25, 0.17, 0.1]);
            let cols = [[0.8, 0.15, 0.18], [0.95, 0.85, 0.3], [0.75, 0.45, 0.85], [0.95, 0.95, 0.9]]; let fl = cols[r.int(0, 3) as usize];
            let n = if m == "potted_fern" { 9 } else { 6 };
            for i in 0..n { let a = i as f64 * std::f64::consts::TAU / n as f64 + r.range(0.0, 0.5); let len = r.range(0.2, 0.32); let (ca, sa) = (a.cos(), a.sin());
                let c = [0.5 + ca * len * 0.5, 0.42 + len * 0.3, 0.5 + sa * len * 0.5]; k.r(c, [[ca * len * 0.5, len * 0.3, sa * len * 0.5], [-sa * 0.05, 0.0, ca * 0.05], [0.0, 0.01, 0.0]], [0.2, 0.42, 0.15]);
                if m == "flower_pot" { let p = [0.5 + ca * len * 0.9, 0.5 + len * 0.6, 0.5 + sa * len * 0.9]; k.b([p[0] - 0.05, p[1], p[2] - 0.05, p[0] + 0.05, p[1] + 0.07, p[2] + 0.05], fl); } }
        }
        "cauldron" => {
            for (y0, y1, rr) in [(0.08, 0.16, 0.3), (0.16, 0.5, 0.38), (0.5, 0.62, 0.35)] { k.b([0.5 - rr, y0, 0.5 - rr, 0.5 + rr, y1, 0.5 + rr], IRON); }
            k.b([0.17, 0.56, 0.17, 0.83, 0.6, 0.83], [0.35, 0.42, 0.18]); for (a, b) in [(0.2, 0.2), (0.74, 0.2), (0.47, 0.76)] { k.b([a, 0.0, b, a + 0.06, 0.1, b + 0.06], IRON); }
            k.b([0.1, 0.55, 0.47, 0.15, 0.62, 0.53], IRON); k.b([0.85, 0.55, 0.47, 0.9, 0.62, 0.53], IRON);
        }
        "bucket" => {
            k.b([0.32, 0.0, 0.32, 0.68, 0.4, 0.68], WOOD); for y in [0.06, 0.32] { k.b([0.31, y, 0.31, 0.69, y + 0.04, 0.69], IRON); }
            k.b([0.34, 0.38, 0.34, 0.66, 0.39, 0.66], [0.3, 0.45, 0.6]); k.b([0.3, 0.4, 0.48, 0.33, 0.55, 0.52], IRON); k.b([0.67, 0.4, 0.48, 0.7, 0.55, 0.52], IRON); k.b([0.3, 0.55, 0.48, 0.7, 0.58, 0.52], IRON);
        }
        "log_pile" => {
            for row in 0..3 { for i in 0..(4 - row) { let (y, x) = (0.13 + row as f64 * 0.24, 0.13 + i as f64 * 0.25 + row as f64 * 0.125);
                k.t([x - 0.12, y - 0.12, 0.02, x + 0.12, y + 0.12, 0.98], "bark_oak", [1.0; 3]); k.t([x - 0.1, y - 0.1, 0.0, x + 0.1, y + 0.1, 0.02], "end_oak", [1.0; 3]); k.t([x - 0.1, y - 0.1, 0.98, x + 0.1, y + 0.1, 1.0], "end_oak", [1.0; 3]); } }
        }
        "carpet_red" | "carpet_blue" | "carpet_green" => {
            let c = match m { "carpet_red" => [1.0, 0.95, 0.95], "carpet_blue" => [0.5, 0.75, 1.6], _ => [0.75, 1.3, 0.7] }; k.t([0.0, 0.0, 0.0, 1.0, 0.05, 1.0], "rug", c);
        }
        "iron_bars" => {
            for i in 0..5 { let x = 0.1 + i as f64 * 0.2; k.b([x - 0.03, 0.0, 0.47, x + 0.03, 1.0, 0.53], IRON); } for y in [0.08, 0.9] { k.b([0.0, y, 0.46, 1.0, y + 0.05, 0.54], IRON); }
        }
        "books_pile" => {
            let cols = [[0.45, 0.1, 0.08], [0.12, 0.2, 0.38], [0.2, 0.32, 0.15], [0.45, 0.3, 0.14], [0.3, 0.12, 0.25]]; let mut y = 0.0;
            for i in 0..r.int(3, 5) { let (w, d, h) = (r.range(0.22, 0.3), r.range(0.15, 0.2), r.range(0.06, 0.09)); let a = r.range(-0.4, 0.4); let c = cols[(i + r.int(0, 4)) as usize % 5];
                k.r([0.5, y + h * 0.5, 0.5], [[a.cos() * w, 0.0, a.sin() * w], [0.0, h * 0.5, 0.0], [-a.sin() * d, 0.0, a.cos() * d]], c); k.r([0.5 + a.cos() * 0.01, y + h * 0.5, 0.5 + a.sin() * 0.01], [[a.cos() * (w - 0.01), 0.0, a.sin() * (w - 0.01)], [0.0, h * 0.4, 0.0], [-a.sin() * (d + 0.005), 0.0, a.cos() * (d + 0.005)]], [0.9, 0.86, 0.74]); y += h; }
        }
        "bottles" => {
            let cols = [[0.15, 0.32, 0.18], [0.35, 0.2, 0.08], [0.2, 0.25, 0.4], [0.5, 0.15, 0.2]];
            for (i, (bx_, bz)) in [(0.3, 0.35), (0.55, 0.3), (0.42, 0.62), (0.7, 0.58)].iter().enumerate() { let c = cols[(i + r.int(0, 3) as usize) % 4]; let h = r.range(0.28, 0.4);
                k.b([bx_ - 0.07, 0.0, bz - 0.07, bx_ + 0.07, h, bz + 0.07], c); k.b([bx_ - 0.03, h, bz - 0.03, bx_ + 0.03, h + 0.12, bz + 0.03], c); k.b([bx_ - 0.025, h + 0.12, bz - 0.025, bx_ + 0.025, h + 0.15, bz + 0.025], [0.6, 0.45, 0.3]); }
        }
        "beehive" => {
            for (i, (y0, rr)) in [(0.0, 0.36), (0.14, 0.38), (0.28, 0.35), (0.42, 0.3), (0.56, 0.22), (0.68, 0.12)].iter().enumerate() { k.t([0.5 - rr, *y0, 0.5 - rr, 0.5 + rr, y0 + 0.15, 0.5 + rr], "hay", [0.9 - 0.04 * i as f64; 3]); }
            k.b([0.43, 0.02, 0.12, 0.57, 0.1, 0.15], [0.08, 0.06, 0.04]);
        }
        "cheese" => {
            let (c, rind) = ([0.95, 0.78, 0.3], [0.8, 0.55, 0.18]);
            for (y, rr) in [(0.0, 0.3), (0.18, 0.26)] { for a in [0.0f64, 0.785] { k.r([0.5, y + 0.09, 0.5], [[a.cos() * rr, 0.0, a.sin() * rr], [0.0, 0.09, 0.0], [-a.sin() * rr, 0.0, a.cos() * rr]], rind); } k.b([0.5 - rr * 0.6, y + 0.18, 0.5 - rr * 0.6, 0.5 + rr * 0.6, y + 0.181, 0.5 + rr * 0.6], c); }
        }
        "bread_basket" => {
            k.t([0.15, 0.0, 0.2, 0.85, 0.22, 0.8], "wattle", [1.2, 1.1, 0.9]); k.b([0.18, 0.2, 0.23, 0.82, 0.21, 0.77], [0.4, 0.3, 0.18]);
            for (x, z, a) in [(0.35, 0.4, 0.3f64), (0.62, 0.55, -0.4), (0.45, 0.62, 1.2)] { k.r([x, 0.28, z], [[a.cos() * 0.15, 0.0, a.sin() * 0.15], [0.0, 0.07, 0.0], [-a.sin() * 0.08, 0.0, a.cos() * 0.08]], [0.72, 0.45, 0.18]); }
        }
        _ => {}
    }
}
