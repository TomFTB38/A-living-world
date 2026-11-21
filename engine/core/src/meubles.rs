//! Meubles et objets de décor : des modèles fins, bâtis dans le cube unité pour le sens 0 (le dos vers +z, la face vers
//! le joueur qui les pose), puis tournés au quart de tour (suffixe « @1 » à « @3 » du nom de bloc) et posés dans le chunk.
use crate::blocks::{by_name, BLOCKS};
use crate::mesher::{bx, cyl, kind, obox, VBuf, VSIZE};
use crate::noise::Rng;

type C = [f64; 3];
const WOOD: C = [0.46, 0.31, 0.17]; const DARK: C = [0.3, 0.19, 0.1]; const IRON: C = [0.17, 0.17, 0.19]; const GOLD: C = [0.85, 0.66, 0.26];
const FLAME: C = [2.6, 1.4, 0.4]; const CORE: C = [2.9, 2.1, 0.9]; const WAX: C = [0.92, 0.88, 0.76]; const CLAY: C = [0.66, 0.36, 0.22];

/// Les modèles de ce module.
pub const MODELS: [&str; 119] = ["sapling", "chest", "gravestone", "grave_cross", "bell", "chain", "wall_clock", "cross", "flag", "tanning_vat", "lime_vat", "hides", "sawhorse", "chopping_block", "cart", "trough", "coal_heap", "tool_rack", "fishing_net", "fish_rack", "creel", "mooring_post", "buoy", "net_pile", "anchor", "oars", "rope_rail", "sail", "pennant", "fish_crate", "boat_bench", "chest_open", "bed", "bed_foot", "torch_wall", "chair", "bench", "armchair", "cupboard", "fireplace", "painting", "banner_red", "banner_blue", "banner_green", "banner_gold", "weapon_rack", "wall_sconce",
    "loom", "spinning_wheel", "grindstone", "signpost", "throne", "curtain", "wall_shelf", "table", "round_table", "stool", "candle", "candelabra", "chandelier", "brazier", "sack",
    "amphora", "flower_pot", "cauldron", "bucket", "log_pile", "carpet_red", "carpet_blue", "carpet_green", "iron_bars", "potted_fern", "books_pile", "bottles", "beehive", "cheese", "bread_basket",
    "lever", "lever_on", "spikes", "spikes_down", "dart_trap",
    "column_base", "column_shaft", "column_capital", "statue_base", "statue_top", "tomb_head", "tomb_foot", "wall_relief", "lintel", "funeral_urn", "altar", "brazier_stand", "great_chain",
    "sun_mirror", "sun_disc", "sun_receptor", "sun_receptor_on", "light_beam", "ossuary_wall", "bone_pile",
    "balustrade_stone", "balustrade_wood", "iron_railing", "picket_fence", "garden_gate", "flower_box", "awning_red", "awning_blue", "wall_lantern", "window_arched", "corbel", "chimney_pot", "stone_arch", "lattice_roses", "drainpipe", "hanging_sign", "shelf"];
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
    /// Pièce ronde entre deux centres (cylindre, tronc de cône ; `hole` : anneau), matière devinée à la couleur.
    fn cy(&mut self, p0: [f64; 3], p1: [f64; 3], r0: f64, r1: f64, hole: f64, c: C) { let n = if r0.max(r1) < 0.1 { 10 } else { 16 }; cyl(&mut self.o, p0, p1, r0, r1, hole, c, -1.0, self.s, self.l, n); }
    /// Pièce tournée d'axe vertical en (cx, cz) : un profil (hauteur, rayon), d'une seule couleur.
    fn lathe(&mut self, cx: f64, cz: f64, prof: &[(f64, f64)], c: C) { for w in prof.windows(2) { self.cy([cx, w[0].0, cz], [cx, w[1].0, cz], w[0].1, w[1].1, 0.0, c); } }
    /// Pièce tournée dans une matière donnée (« hay »…), teintée.
    fn lathe_t(&mut self, cx: f64, cz: f64, prof: &[(f64, f64)], m: &str, c: C) { for w in prof.windows(2) { cyl(&mut self.o, [cx, w[0].0, cz], [cx, w[1].0, cz], w[0].1, w[1].1, 0.0, c, layer(m), self.s, self.l, 16); } }
    /// Une flamme : cœur clair dans une enveloppe orangée.
    fn flame(&mut self, x: f64, y: f64, z: f64, h: f64) { let w = h * 0.32; self.e([x - w, y, z - w, x + w, y + h, z + w], FLAME); self.e([x - w * 0.5, y + h * 0.15, z - w * 0.5, x + w * 0.5, y + h * 1.15, z + w * 0.5], CORE); }
    fn candle(&mut self, x: f64, y: f64, z: f64, h: f64) { self.b([x - 0.035, y, z - 0.035, x + 0.035, y + h, z + 0.035], WAX); self.b([x - 0.006, y + h, z - 0.006, x + 0.006, y + h + 0.03, z + 0.006], [0.1, 0.08, 0.06]); self.flame(x, y + h + 0.02, z, 0.09); }
}
fn tone(c: C, k: f64) -> C { [c[0] * k, c[1] * k, c[2] * k] }

/// Un crâne, la face vers -z : la calotte, la mâchoire, deux orbites et le nez sombres. `c` : le centre de la face avant.
fn skull(k: &mut K, c: [f64; 3], s: f64, bone: C) {
    let (x, y, z) = (c[0], c[1], c[2]); let dark = [0.12, 0.1, 0.08];
    k.b([x - 0.075 * s, y - 0.045 * s, z, x + 0.075 * s, y + 0.075 * s, z + 0.15 * s], bone); k.b([x - 0.06 * s, y + 0.075 * s, z + 0.015 * s, x + 0.06 * s, y + 0.1 * s, z + 0.13 * s], bone);
    k.b([x - 0.05 * s, y - 0.095 * s, z + 0.012 * s, x + 0.05 * s, y - 0.045 * s, z + 0.11 * s], tone(bone, 0.86));
    for sx in [-1.0, 1.0] { k.b([x + sx * 0.04 * s - 0.022 * s, y - 0.002 * s, z - 0.004 * s, x + sx * 0.04 * s + 0.022 * s, y + 0.038 * s, z + 0.01 * s], dark); }
    k.b([x - 0.01 * s, y - 0.035 * s, z - 0.004 * s, x + 0.01 * s, y - 0.01 * s, z + 0.01 * s], dark);
}
fn ossuary(k: &mut K, m: &str, r: &mut Rng) {
    let bone = |r: &mut Rng| tone([0.86, 0.81, 0.68], r.range(0.8, 1.05));
    match m {
        "ossuary_wall" => { // le fond de pierre sombre ; trois lits de longs os ; deux rangs de crânes ; çà et là un crâne manque
            k.t([0.0, 0.0, 0.2, 1.0, 1.0, 1.0], "stone_bricks", [0.5, 0.5, 0.53]); k.b([0.0, 0.0, 0.16, 1.0, 1.0, 0.2], [0.16, 0.14, 0.12]);
            for y in [0.08, 0.44, 0.8] { for j in 0..4 { let yy = y + j as f64 * 0.032 - 0.04; let c = bone(r); let dx = r.range(-0.06, 0.06); k.cy([0.0 + dx.max(0.0), yy, 0.06 + (j % 2) as f64 * 0.04], [1.0 + dx.min(0.0), yy, 0.06 + (j % 2) as f64 * 0.04], 0.022, 0.022, 0.0, c); }
                for x in [0.02, 0.98] { k.b([x - 0.025, y - 0.07, 0.03, x + 0.025, y + 0.07, 0.12], bone(r)); } }
            for y in [0.26, 0.62] { for i in 0..4 { if r.next() < 0.1 { continue; } let x = 0.13 + i as f64 * 0.245 + r.range(-0.015, 0.015); let c = bone(r); skull(k, [x, y, 0.04 + r.range(0.0, 0.02)], 1.15, c); } }
        }
        _ => { // le tas d'ossements : de longs os croisés, des côtes, deux crânes
            for _ in 0..7 { let a = r.range(0.0, std::f64::consts::PI); let (ca, sa) = (a.cos(), a.sin()); let (x, z) = (r.range(0.25, 0.75), r.range(0.25, 0.75)); let y = r.range(0.02, 0.1); let l = r.range(0.18, 0.3); let c = bone(r);
                k.cy([x - ca * l, y, z - sa * l], [x + ca * l, y + r.range(0.0, 0.06), z + sa * l], 0.024, 0.024, 0.0, c); k.b([x - ca * l - 0.035, y - 0.02, z - sa * l - 0.035, x - ca * l + 0.035, y + 0.035, z - sa * l + 0.035], c); }
            let c = bone(r); skull(k, [0.42, 0.1, 0.36], 1.2, c); let c = bone(r); skull(k, [0.66, 0.08, 0.58], 1.0, c);
        }
    }
}
fn solar(k: &mut K, m: &str, r: &mut Rng) {
    use std::f64::consts::TAU; let gold = [0.95, 0.76, 0.3]; let stone = tone(LIME, r.range(0.88, 1.0));
    match m {
        "sun_mirror" => { // le pied de pierre, le pivot de bronze, la glace d'or polie sur la diagonale, son cadre
            k.t([0.18, 0.0, 0.18, 0.82, 0.1, 0.82], "polished_stone", tone(stone, 0.85)); k.cy([0.5, 0.1, 0.5], [0.5, 0.3, 0.5], 0.1, 0.08, 0.0, STONE); k.cy([0.5, 0.3, 0.5], [0.5, 0.35, 0.5], 0.12, 0.12, 0.0, [0.55, 0.36, 0.18]);
            let d = std::f64::consts::FRAC_1_SQRT_2; let a0 = [0.44 * d, 0.0, 0.44 * d]; let a2 = [0.025 * d, 0.0, -0.025 * d];
            k.r([0.5, 0.64, 0.5], [a0, [0.0, 0.28, 0.0], a2], [1.0, 0.9, 0.62]); k.r([0.5, 0.64, 0.5], [[0.47 * d, 0.0, 0.47 * d], [0.0, 0.31, 0.0], [0.012 * d, 0.0, -0.012 * d]], [0.5, 0.33, 0.15]);
            k.e([0.47, 0.62, 0.47, 0.53, 0.66, 0.53], [1.4, 1.2, 0.7]);
        }
        "sun_disc" => { // un autel rond, et dressé dessus un disque d'or aux rayons, tourné vers l'est (sa face est vers -z, tournée à la pose)
            k.t([0.1, 0.0, 0.1, 0.9, 0.12, 0.9], "polished_stone", tone(stone, 0.85)); k.lathe(0.5, 0.5, &[(0.12, 0.3), (0.2, 0.26), (0.28, 0.22), (0.3, 0.0)], STONE);
            k.cy([0.5, 0.64, 0.46], [0.5, 0.64, 0.54], 0.26, 0.26, 0.0, gold); k.cy([0.5, 0.64, 0.44], [0.5, 0.64, 0.46], 0.14, 0.14, 0.0, [1.0, 0.86, 0.45]); k.e([0.44, 0.58, 0.42, 0.56, 0.7, 0.44], [1.8, 1.5, 0.7]);
            for i in 0..12 { let a = i as f64 * TAU / 12.0; let (ca, sa) = (a.cos(), a.sin()); k.r([0.5 + ca * 0.32, 0.64 + sa * 0.32, 0.5], [[ca * 0.06, sa * 0.06, 0.0], [-sa * 0.02, ca * 0.02, 0.0], [0.0, 0.0, 0.02]], gold); }
            k.b([0.46, 0.3, 0.47, 0.54, 0.4, 0.53], [0.55, 0.36, 0.18]);
        }
        "sun_receptor" | "sun_receptor_on" => { // un bloc de pierre sculpté d'un soleil, un cristal serti au cœur : terne, ou embrasé
            let lit = m == "sun_receptor_on"; k.t([0.0, 0.0, 0.0, 1.0, 1.0, 1.0], "ashlar", tone(stone, 0.85));
            for i in 0..8 { let a = i as f64 * TAU / 8.0; let (ca, sa) = (a.cos(), a.sin()); k.r([0.5 + ca * 0.3, 0.5 + sa * 0.3, -0.015], [[ca * 0.08, sa * 0.08, 0.0], [-sa * 0.03, ca * 0.03, 0.0], [0.0, 0.0, 0.015]], if lit { gold } else { STONE2 }); }
            k.cy([0.5, 0.5, -0.03], [0.5, 0.5, 0.002], 0.19, 0.19, 0.78, if lit { gold } else { STONE2 });
            if lit { k.e([0.38, 0.38, -0.07, 0.62, 0.62, 0.0], [1.9, 1.6, 0.8]); } else { k.b([0.39, 0.39, -0.06, 0.61, 0.61, 0.0], [0.35, 0.42, 0.5]); }
        }
        _ => { // le rayon : une barre de lumière dorée le long du bloc (en z ; tournée pour aller en x)
            k.e([0.46, 0.55, 0.0, 0.54, 0.63, 1.0], [2.0, 1.7, 0.9]); k.e([0.485, 0.575, -0.002, 0.515, 0.605, 1.002], [2.4, 2.2, 1.6]);
        }
    }
}

/// La pierre des anciens : calcaire vieilli, ses ombres, la mousse.
const STONE: C = [0.71, 0.67, 0.58]; const STONE2: C = [0.52, 0.49, 0.43]; const MOSS: C = [0.3, 0.4, 0.17];
const LIME: C = [0.98, 0.94, 0.84];
fn ancient(k: &mut K, m: &str, r: &mut Rng) {
    use std::f64::consts::TAU;
    // la mousse : quelques plaques sur les surfaces du dessus ou les flancs, au hasard
    let moss = |k: &mut K, r: &mut Rng, n: usize, y0: f64, y1: f64, rad: f64| { for _ in 0..n { if r.next() < 0.45 { continue; } let a = r.range(0.0, TAU); let y = r.range(y0, y1); let (x, z) = (0.5 + a.cos() * rad, 0.5 + a.sin() * rad); let w = r.range(0.05, 0.11);
        k.b([x - w, y, z - w, x + w, y + r.range(0.03, 0.12), z + w], tone(MOSS, r.range(0.8, 1.2))); } };
    // un membre (bras, avant-bras) : une boîte orientée entre deux points
    let limb = |k: &mut K, p0: [f64; 3], p1: [f64; 3], w: f64, c: C| { let a0 = [(p1[0] - p0[0]) * 0.5, (p1[1] - p0[1]) * 0.5, (p1[2] - p0[2]) * 0.5]; let l = (a0[0] * a0[0] + a0[1] * a0[1] + a0[2] * a0[2]).sqrt().max(1e-6);
        let mut a1 = [a0[2], 0.0, -a0[0]]; let l1 = (a1[0] * a1[0] + a1[2] * a1[2]).sqrt(); if l1 < 1e-6 { a1 = [1.0, 0.0, 0.0]; } else { a1 = [a1[0] / l1, 0.0, a1[2] / l1]; }
        let n = [a0[0] / l, a0[1] / l, a0[2] / l]; let a2 = [n[1] * a1[2] - n[2] * a1[1], n[2] * a1[0] - n[0] * a1[2], n[0] * a1[1] - n[1] * a1[0]];
        k.r([(p0[0] + p1[0]) * 0.5, (p0[1] + p1[1]) * 0.5, (p0[2] + p1[2]) * 0.5], [a0, [a1[0] * w, 0.0, a1[2] * w], [a2[0] * w, a2[1] * w, a2[2] * w]], c); };
    // un fût cannelé : un cœur sombre, douze listels clairs
    let shaft = |k: &mut K, y0: f64, y1: f64| { k.cy([0.5, y0, 0.5], [0.5, y1, 0.5], 0.31, 0.31, 0.0, tone(STONE, 0.78)); for i in 0..12 { let a = i as f64 * TAU / 12.0; k.cy([0.5 + a.cos() * 0.31, y0, 0.5 + a.sin() * 0.31], [0.5 + a.cos() * 0.31, y1, 0.5 + a.sin() * 0.31], 0.032, 0.032, 0.0, STONE); } };
    // une face sculptée (+ un cadre) : boîtes décrites en (u, v) sur la face avant (z = 0, vers -z), en relief de `d`
    let front = |k: &mut K, u0: f64, v0: f64, u1: f64, v1: f64, d: f64, c: C| k.b([u0, v0, -d, u1, v1, 0.002], c);
    let t = tone(LIME, r.range(0.9, 1.04));
    match m {
        "column_base" => {
            k.t([0.03, 0.0, 0.03, 0.97, 0.16, 0.97], "polished_stone", tone(t, 0.85)); k.lathe(0.5, 0.5, &[(0.16, 0.44), (0.22, 0.45), (0.27, 0.4), (0.31, 0.37), (0.36, 0.38), (0.4, 0.34)], STONE);
            shaft(k, 0.4, 1.0); moss(k, r, 4, 0.12, 0.3, 0.42);
        }
        "column_shaft" => { shaft(k, 0.0, 1.0); if r.next() < 0.35 { k.cy([0.5, 0.46, 0.5], [0.5, 0.52, 0.5], 0.345, 0.345, 0.0, STONE2); } if r.next() < 0.3 { let y = r.range(0.2, 0.7); k.b([0.17, y, 0.47, 0.19, y + 0.25, 0.5], [0.12, 0.11, 0.1]); } moss(k, r, 3, 0.0, 0.8, 0.33); }
        "column_capital" => {
            shaft(k, 0.0, 0.32); k.cy([0.5, 0.32, 0.5], [0.5, 0.38, 0.5], 0.36, 0.36, 0.0, STONE2);
            k.lathe(0.5, 0.5, &[(0.38, 0.33), (0.5, 0.4), (0.6, 0.47), (0.66, 0.5)], STONE);
            // une couronne de feuilles d'acanthe, recourbées
            for i in 0..8 { let a = i as f64 * TAU / 8.0 + 0.2; let (ca, sa) = (a.cos(), a.sin()); limb(k, [0.5 + ca * 0.34, 0.4, 0.5 + sa * 0.34], [0.5 + ca * 0.47, 0.6, 0.5 + sa * 0.47], 0.045, tone(STONE, 1.05)); k.b([0.5 + ca * 0.47 - 0.03, 0.58, 0.5 + sa * 0.47 - 0.03, 0.5 + ca * 0.47 + 0.03, 0.64, 0.5 + sa * 0.47 + 0.03], STONE2); }
            k.t([0.0, 0.66, 0.0, 1.0, 0.84, 1.0], "polished_stone", t); k.b([0.04, 0.84, 0.04, 0.96, 0.92, 0.96], STONE2); k.t([0.0, 0.92, 0.0, 1.0, 1.0, 1.0], "ashlar", tone(t, 0.9));
            // des volutes aux quatre coins
            for (x, z) in [(0.08, 0.08), (0.92, 0.08), (0.08, 0.92), (0.92, 0.92)] { k.cy([x, 0.62, z], [x, 0.7, z], 0.07, 0.07, 0.4, STONE2); } moss(k, r, 3, 0.84, 1.0, 0.4);
        }
        "statue_base" => { // le piédestal, la robe, les pieds, l'épée plantée entre eux
            k.t([0.08, 0.0, 0.1, 0.92, 0.22, 0.9], "polished_stone", tone(t, 0.85)); k.b([0.05, 0.22, 0.07, 0.95, 0.28, 0.93], STONE2);
            for i in 0..6 { let x = 0.22 + i as f64 * 0.11; k.b([x, 0.08, 0.095, x + 0.06, 0.14, 0.1], [0.2, 0.18, 0.16]); }
            k.b([0.3, 0.28, 0.2, 0.44, 0.36, 0.4], STONE); k.b([0.56, 0.28, 0.2, 0.7, 0.36, 0.4], STONE);
            k.lathe(0.5, 0.54, &[(0.28, 0.27), (0.55, 0.24), (1.0, 0.21)], STONE); for i in 0..5 { let x = 0.36 + i as f64 * 0.07; k.b([x - 0.012, 0.32, 0.28, x + 0.012, 1.0, 0.31], tone(STONE, 0.86)); }
            k.b([0.478, 0.3, 0.185, 0.522, 1.0, 0.2], tone(STONE, 0.92)); k.b([0.493, 0.3, 0.18, 0.507, 1.0, 0.185], tone(STONE, 1.08)); moss(k, r, 3, 0.22, 0.3, 0.45);
        }
        "statue_top" => { // le buste en armure, les épaulières, les mains jointes sur le pommeau, la tête casquée au cimier
            k.b([0.478, 0.0, 0.185, 0.522, 0.05, 0.2], tone(STONE, 0.92)); k.b([0.34, 0.05, 0.17, 0.66, 0.09, 0.23], STONE2); k.cy([0.5, 0.09, 0.2], [0.5, 0.24, 0.2], 0.022, 0.022, 0.0, STONE2); k.cy([0.5, 0.24, 0.2], [0.5, 0.29, 0.2], 0.04, 0.03, 0.0, STONE2);
            k.lathe(0.5, 0.54, &[(0.0, 0.21), (0.25, 0.22), (0.42, 0.26), (0.46, 0.2)], STONE); k.b([0.34, 0.12, 0.27, 0.66, 0.42, 0.31], tone(STONE, 0.95)); k.b([0.35, 0.2, 0.26, 0.65, 0.23, 0.28], STONE2);
            for sx in [-1.0, 1.0] { let x = 0.5 + sx * 0.23; k.b([x - 0.1, 0.38, 0.38, x + 0.1, 0.5, 0.7], STONE2); k.b([x - 0.09, 0.47, 0.4, x + 0.09, 0.53, 0.68], STONE);
                limb(k, [x, 0.42, 0.5], [0.5 + sx * 0.2, 0.2, 0.36], 0.055, STONE); limb(k, [0.5 + sx * 0.2, 0.2, 0.36], [0.5 + sx * 0.05, 0.16, 0.22], 0.05, STONE); }
            k.b([0.42, 0.11, 0.15, 0.58, 0.22, 0.25], tone(STONE, 1.05));
            k.cy([0.5, 0.46, 0.53], [0.5, 0.54, 0.53], 0.07, 0.07, 0.0, STONE);
            k.lathe(0.5, 0.5, &[(0.52, 0.06), (0.56, 0.11), (0.64, 0.13), (0.74, 0.125), (0.81, 0.09), (0.85, 0.0)], STONE); k.cy([0.5, 0.68, 0.5], [0.5, 0.73, 0.5], 0.14, 0.14, 0.0, STONE2);
            k.b([0.485, 0.73, 0.36, 0.515, 0.93, 0.66], STONE2); k.b([0.485, 0.85, 0.62, 0.515, 0.9, 0.72], STONE2);
            k.b([0.43, 0.63, 0.365, 0.47, 0.655, 0.375], [0.12, 0.11, 0.1]); k.b([0.53, 0.63, 0.365, 0.57, 0.655, 0.375], [0.12, 0.11, 0.1]); k.b([0.485, 0.58, 0.355, 0.515, 0.64, 0.375], STONE);
            k.b([0.42, 0.5, 0.37, 0.58, 0.58, 0.42], STONE2); moss(k, r, 4, 0.48, 0.55, 0.28);
        }
        "tomb_head" | "tomb_foot" => { // le sarcophage, ses panneaux sculptés, son couvercle ; le gisant, tête sur un coussin, mains jointes
            let head = m == "tomb_head"; let (x0, x1) = if head { (0.04, 1.0) } else { (0.0, 0.96) };
            k.t([x0, 0.0, 0.08, x1, 0.1, 0.92], "polished_stone", tone(t, 0.8)); k.t([x0 + 0.04, 0.1, 0.12, x1 - if head { 0.0 } else { 0.04 }, 0.5, 0.88], "ashlar", t);
            for z in [0.1, 0.88] { let (z0, z1) = (z, z + 0.02); let p = if head { (0.14, 0.96) } else { (0.04, 0.86) };
                for bb in [[p.0, 0.15, z0, p.1, 0.18, z1], [p.0, 0.42, z0, p.1, 0.45, z1], [p.0, 0.15, z0, p.0 + 0.03, 0.45, z1], [p.1 - 0.03, 0.15, z0, p.1, 0.45, z1]] { k.b(bb, STONE2); }
                let c = (p.0 + p.1) * 0.5; k.cy([c, 0.3, z0 - 0.002], [c, 0.3, z1 + 0.002], 0.08, 0.08, 0.5, STONE2); k.b([c - 0.012, 0.22, z0 - 0.004, c + 0.012, 0.38, z1 + 0.004], STONE2); }
            let end = if head { x0 + 0.04 } else { x1 - 0.04 }; k.b([end - 0.02, 0.15, 0.2, end + 0.02, 0.45, 0.8], STONE2);
            k.t([x0 - 0.02, 0.5, 0.09, x1 + if head { 0.0 } else { 0.02 }, 0.6, 0.91], "polished_stone", tone(t, 0.95));
            if head { k.b([0.1, 0.6, 0.3, 0.34, 0.66, 0.7], STONE); k.lathe(0.24, 0.5, &[(0.64, 0.05), (0.68, 0.09), (0.74, 0.1), (0.79, 0.07), (0.81, 0.0)], STONE);
                k.b([0.34, 0.6, 0.27, 0.52, 0.76, 0.73], STONE); k.b([0.52, 0.6, 0.31, 1.0, 0.74, 0.69], STONE); k.b([0.66, 0.74, 0.44, 0.78, 0.81, 0.56], tone(STONE, 1.06));
                for i in 0..3 { let z = 0.38 + i as f64 * 0.12; k.b([0.55, 0.735, z, 1.0, 0.745, z + 0.015], STONE2); } }
            else { k.b([0.0, 0.6, 0.33, 0.6, 0.72, 0.67], STONE); k.b([0.6, 0.6, 0.35, 0.7, 0.84, 0.48], STONE); k.b([0.6, 0.6, 0.52, 0.7, 0.84, 0.65], STONE); k.b([0.0, 0.715, 0.48, 0.55, 0.725, 0.52], STONE2); }
            moss(k, r, 3, 0.5, 0.6, 0.42);
        }
        "wall_relief" => { // un bloc de pierre au bas-relief : un cadre, et l'une de trois scènes (un soleil, un porteur de lance, un serpent)
            k.t([0.0, 0.0, 0.0, 1.0, 1.0, 1.0], "ashlar", tone(t, 0.8)); let rel = tone(STONE, 1.08);
            for (u0, v0, u1, v1) in [(0.06, 0.06, 0.94, 0.12), (0.06, 0.88, 0.94, 0.94), (0.06, 0.12, 0.12, 0.88), (0.88, 0.12, 0.94, 0.88)] { front(k, u0, v0, u1, v1, 0.035, STONE2); }
            match r.int(0, 2) {
                0 => { k.cy([0.5, 0.5, -0.035], [0.5, 0.5, 0.002], 0.15, 0.15, 0.0, rel); k.cy([0.5, 0.5, -0.045], [0.5, 0.5, -0.03], 0.07, 0.07, 0.0, tone(rel, 0.9));
                    for i in 0..12 { let a = i as f64 * TAU / 12.0; let (ca, sa) = (a.cos(), a.sin()); limb(k, [0.5 + ca * 0.19, 0.5 + sa * 0.19, -0.015], [0.5 + ca * 0.32, 0.5 + sa * 0.32, -0.015], 0.018, rel); } }
                1 => { k.cy([0.42, 0.74, -0.03], [0.42, 0.74, 0.002], 0.06, 0.06, 0.0, rel); front(k, 0.36, 0.42, 0.5, 0.68, 0.03, rel); front(k, 0.35, 0.2, 0.42, 0.42, 0.03, rel); front(k, 0.45, 0.2, 0.52, 0.42, 0.03, rel);
                    limb(k, [0.48, 0.62, -0.015], [0.62, 0.7, -0.015], 0.025, rel); front(k, 0.62, 0.16, 0.65, 0.86, 0.025, tone(rel, 0.9)); limb(k, [0.6, 0.86, -0.015], [0.635, 0.94, -0.015], 0.02, tone(rel, 0.9)); limb(k, [0.67, 0.86, -0.015], [0.635, 0.94, -0.015], 0.02, tone(rel, 0.9)); }
                _ => { let mut prev = [0.18, 0.5, -0.015]; for i in 1..=14 { let u = 0.18 + i as f64 * 0.045; let p = [u, 0.5 + (u * 14.0).sin() * 0.16, -0.015]; limb(k, prev, p, 0.03, rel); prev = p; } k.cy([prev[0] + 0.02, prev[1], -0.03], [prev[0] + 0.02, prev[1], 0.002], 0.05, 0.05, 0.0, rel); }
            }
            if r.next() < 0.4 { let x = r.range(0.1, 0.8); front(k, x, 0.0, x + r.range(0.08, 0.2), r.range(0.05, 0.25), 0.02, tone(MOSS, 0.9)); }
        }
        "lintel" => { // un linteau : la frise à méandres, les denticules, la corniche en surplomb
            k.t([0.0, 0.0, 0.0, 1.0, 1.0, 1.0], "ashlar", tone(t, 0.85)); front(k, 0.0, 0.84, 1.0, 1.0, 0.1, tone(STONE, 1.05)); front(k, 0.0, 0.8, 1.0, 0.84, 0.06, STONE2);
            for i in 0..6 { let u = 0.04 + i as f64 * 0.167; front(k, u, 0.68, u + 0.09, 0.8, 0.045, STONE); }
            for i in 0..2 { let u = i as f64 * 0.5; for (a, b, c, d) in [(0.04, 0.18, 0.46, 0.23), (0.04, 0.18, 0.09, 0.56), (0.04, 0.51, 0.36, 0.56), (0.31, 0.3, 0.36, 0.56), (0.18, 0.3, 0.36, 0.35), (0.18, 0.3, 0.23, 0.45), (0.41, 0.18, 0.46, 0.62)] { front(k, u + a, b, u + c, d, 0.03, STONE); } }
            front(k, 0.0, 0.08, 1.0, 0.12, 0.04, STONE2);
        }
        "funeral_urn" => { // une urne de pierre sur son socle, deux anses, un couvercle à bouton
            k.b([0.28, 0.0, 0.28, 0.72, 0.08, 0.72], STONE2); k.lathe(0.5, 0.5, &[(0.08, 0.1), (0.14, 0.17), (0.32, 0.23), (0.5, 0.21), (0.6, 0.13), (0.64, 0.15), (0.68, 0.15)], STONE);
            k.lathe(0.5, 0.5, &[(0.68, 0.14), (0.74, 0.09), (0.78, 0.04), (0.8, 0.05), (0.84, 0.0)], tone(STONE, 0.95)); k.cy([0.5, 0.3, 0.5], [0.5, 0.34, 0.5], 0.232, 0.232, 0.85, STONE2);
            for sx in [-1.0, 1.0] { let x = 0.5 + sx * 0.22; limb(k, [x, 0.58, 0.5], [x + sx * 0.06, 0.48, 0.5], 0.02, STONE2); limb(k, [x + sx * 0.06, 0.48, 0.5], [x, 0.4, 0.5], 0.02, STONE2); }
            moss(k, r, 2, 0.08, 0.2, 0.22);
        }
        "altar" => { // un autel : socle, bloc sculpté (un soleil cerclé d'or sur la face), table en surplomb
            k.t([0.02, 0.0, 0.04, 0.98, 0.12, 0.96], "polished_stone", tone(t, 0.8)); k.t([0.08, 0.12, 0.1, 0.92, 0.86, 0.9], "ashlar", t); k.t([0.0, 0.86, 0.02, 1.0, 1.0, 0.98], "polished_stone", t);
            for (u0, v0, u1, v1) in [(0.14, 0.18, 0.86, 0.22), (0.14, 0.76, 0.86, 0.8), (0.14, 0.22, 0.18, 0.76), (0.82, 0.22, 0.86, 0.76)] { k.b([u0, v0, 0.08, u1, v1, 0.1], STONE2); }
            k.cy([0.5, 0.49, 0.075], [0.5, 0.49, 0.1], 0.17, 0.17, 0.82, GOLD); k.cy([0.5, 0.49, 0.08], [0.5, 0.49, 0.1], 0.12, 0.12, 0.0, tone(STONE, 1.05));
            for i in 0..8 { let a = i as f64 * TAU / 8.0; let (ca, sa) = (a.cos(), a.sin()); limb(k, [0.5 + ca * 0.19, 0.49 + sa * 0.19, 0.09], [0.5 + ca * 0.26, 0.49 + sa * 0.26, 0.09], 0.015, STONE2); }
            moss(k, r, 2, 0.0, 0.12, 0.45);
        }
        "great_chain" => { // une lourde chaîne de fer : quatre gros maillons par bloc, l'un de face, l'autre de profil, rouillés par endroits
            for i in 0..4 { let y0 = i as f64 * 0.25 - 0.04; let (y1, w, t) = (y0 + 0.33, 0.14, 0.055); let c = if r.next() < 0.3 { [0.36, 0.22, 0.14] } else { [0.24, 0.24, 0.26] };
                if i % 2 == 0 { k.b([0.5 - w, y0, 0.5 - t * 0.5, 0.5 - w + t, y1, 0.5 + t * 0.5], c); k.b([0.5 + w - t, y0, 0.5 - t * 0.5, 0.5 + w, y1, 0.5 + t * 0.5], c); k.b([0.5 - w, y0, 0.5 - t * 0.5, 0.5 + w, y0 + t, 0.5 + t * 0.5], c); k.b([0.5 - w, y1 - t, 0.5 - t * 0.5, 0.5 + w, y1, 0.5 + t * 0.5], c); }
                else { k.b([0.5 - t * 0.5, y0, 0.5 - w, 0.5 + t * 0.5, y1, 0.5 - w + t], c); k.b([0.5 - t * 0.5, y0, 0.5 + w - t, 0.5 + t * 0.5, y1, 0.5 + w], c); k.b([0.5 - t * 0.5, y0, 0.5 - w, 0.5 + t * 0.5, y0 + t, 0.5 + w], c); k.b([0.5 - t * 0.5, y1 - t, 0.5 - w, 0.5 + t * 0.5, y1, 0.5 + w], c); } }
        }
        _ => { // le brasier sur pied : un fût cannelé court, une vasque de bronze, les braises et les flammes
            k.t([0.18, 0.0, 0.18, 0.82, 0.1, 0.82], "polished_stone", tone(t, 0.85)); k.cy([0.5, 0.1, 0.5], [0.5, 0.56, 0.5], 0.14, 0.14, 0.0, tone(STONE, 0.8));
            for i in 0..8 { let a = i as f64 * TAU / 8.0; k.cy([0.5 + a.cos() * 0.14, 0.1, 0.5 + a.sin() * 0.14], [0.5 + a.cos() * 0.14, 0.56, 0.5 + a.sin() * 0.14], 0.022, 0.022, 0.0, STONE); }
            let bronze = [0.55, 0.36, 0.18]; k.lathe(0.5, 0.5, &[(0.56, 0.16), (0.62, 0.3), (0.72, 0.38), (0.76, 0.4)], bronze); k.cy([0.5, 0.74, 0.5], [0.5, 0.78, 0.5], 0.41, 0.41, 0.88, tone(bronze, 1.2));
            k.e([0.2, 0.7, 0.2, 0.8, 0.74, 0.8], [1.7, 0.45, 0.1]); k.flame(0.42, 0.72, 0.45, 0.26); k.flame(0.58, 0.72, 0.55, 0.32); k.flame(0.5, 0.72, 0.38, 0.22);
        }
    }
}

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
        // Le catalogue des bâtisseurs : balustrades, garde-corps, clôtures, jardinières, auvents, lanternes murales, fenêtre cintrée,
        // console, souche de cheminée, arc de pierre, treillis fleuri, descente d'eau, enseigne. Les pièces de façade sont plaquées
        // contre la face +z (le mur), les pièces de clôture courent selon x au milieu du bloc.
        "balustrade_stone" => { // une plinthe, une main courante, trois balustres tournés en vase
            k.b([0.0, 0.0, 0.3, 1.0, 0.14, 0.7], STONE2); k.b([0.0, 0.84, 0.26, 1.0, 1.0, 0.74], STONE); k.b([0.0, 0.8, 0.3, 1.0, 0.84, 0.7], STONE2);
            for x in [0.17, 0.5, 0.83] { k.lathe(x, 0.5, &[(0.14, 0.07), (0.22, 0.1), (0.36, 0.13), (0.5, 0.08), (0.6, 0.06), (0.7, 0.09), (0.8, 0.07)], STONE); }
        }
        "balustrade_wood" => { // une lisse basse, une main courante, des balustres carrés, un poteau à chaque bout
            k.b([0.0, 0.08, 0.42, 1.0, 0.16, 0.58], DARK); k.b([0.0, 0.84, 0.38, 1.0, 0.96, 0.62], WOOD);
            for i in 0..5 { let x = 0.1 + i as f64 * 0.2; k.b([x - 0.03, 0.16, 0.46, x + 0.03, 0.84, 0.54], tone(WOOD, 0.9)); }
        }
        "iron_railing" => { // un garde-corps de fer forgé : des barreaux à pointe de lance, deux traverses, des volutes
            k.b([0.0, 0.12, 0.47, 1.0, 0.17, 0.53], IRON); k.b([0.0, 0.78, 0.47, 1.0, 0.83, 0.53], IRON);
            for i in 0..8 { let x = 0.0625 + i as f64 * 0.125; k.b([x - 0.014, 0.0, 0.485, x + 0.014, 0.92, 0.515], IRON); k.b([x - 0.03, 0.92, 0.48, x + 0.03, 0.95, 0.52], IRON); k.b([x - 0.012, 0.95, 0.488, x + 0.012, 1.0, 0.512], IRON); }
            for i in 0..4 { let x = 0.125 + i as f64 * 0.25; k.b([x - 0.06, 0.4, 0.49, x + 0.06, 0.43, 0.51], IRON); k.b([x - 0.06, 0.55, 0.49, x + 0.06, 0.58, 0.51], IRON); k.b([x - 0.06, 0.43, 0.49, x - 0.03, 0.55, 0.51], IRON); k.b([x + 0.03, 0.43, 0.49, x + 0.06, 0.55, 0.51], IRON); }
        }
        "picket_fence" => { // la palissade blanche : quatre piquets pointus sur deux lisses
            let white = [0.92, 0.9, 0.84]; k.b([0.0, 0.2, 0.53, 1.0, 0.28, 0.58], tone(white, 0.85)); k.b([0.0, 0.6, 0.53, 1.0, 0.68, 0.58], tone(white, 0.85));
            for i in 0..4 { let x = 0.125 + i as f64 * 0.25; k.b([x - 0.05, 0.0, 0.47, x + 0.05, 0.8, 0.53], white); k.b([x - 0.035, 0.8, 0.47, x + 0.035, 0.86, 0.53], white); k.b([x - 0.015, 0.86, 0.47, x + 0.015, 0.9, 0.53], white); }
        }
        "garden_gate" => { // le portillon : deux montants, des planches, une écharpe en Z, un loquet de fer
            k.b([0.0, 0.0, 0.44, 0.08, 1.0, 0.56], DARK); k.b([0.92, 0.0, 0.44, 1.0, 1.0, 0.56], DARK);
            for i in 0..5 { let x = 0.13 + i as f64 * 0.165; k.b([x - 0.07, 0.06, 0.47, x + 0.07, 0.82 + 0.04 * (1.0 - ((x - 0.5) * 2.0).powi(2)), 0.53], WOOD); }
            k.b([0.08, 0.14, 0.53, 0.92, 0.22, 0.57], DARK); k.b([0.08, 0.64, 0.53, 0.92, 0.72, 0.57], DARK);
            let (dx, dy) = (0.76f64, 0.46f64); let l = (dx * dx + dy * dy).sqrt(); k.r([0.5, 0.43, 0.55], [[dx * 0.5, dy * 0.5, 0.0], [-dy / l * 0.035, dx / l * 0.035, 0.0], [0.0, 0.0, 0.02]], DARK);
            k.b([0.8, 0.42, 0.57, 0.88, 0.48, 0.6], IRON);
        }
        "flower_box" => { // la jardinière sous la fenêtre : une caisse peinte, la terre, des fleurs et du feuillage qui débordent
            k.b([0.04, 0.0, 0.7, 0.96, 0.22, 1.0], [0.36, 0.5, 0.34]); k.b([0.02, 0.18, 0.68, 0.98, 0.24, 1.0], tone([0.36, 0.5, 0.34], 0.8)); k.b([0.07, 0.2, 0.73, 0.93, 0.23, 0.97], [0.25, 0.17, 0.1]);
            let cols = [[0.85, 0.15, 0.15], [0.95, 0.8, 0.2], [0.92, 0.5, 0.7], [0.95, 0.95, 0.92]];
            for i in 0..9 { let x = 0.1 + i as f64 * 0.1 + r.range(-0.02, 0.02); let zc = 0.8 + r.range(-0.05, 0.08); let h = r.range(0.32, 0.46);
                k.b([x - 0.008, 0.22, zc - 0.008, x + 0.008, h, zc + 0.008], [0.22, 0.45, 0.16]); k.b([x - 0.045, 0.24, zc - 0.04, x + 0.045, 0.3, zc + 0.04], [0.28, 0.55, 0.2]);
                let c = cols[(r.next() * 4.0) as usize % 4]; k.b([x - 0.035, h, zc - 0.035, x + 0.035, h + 0.05, zc + 0.035], c); k.b([x - 0.012, h + 0.05, zc - 0.012, x + 0.012, h + 0.06, zc + 0.012], [0.95, 0.85, 0.3]); }
            for i in 0..6 { let x = 0.1 + i as f64 * 0.16; k.b([x - 0.04, 0.02, 0.66, x + 0.04, 0.22, 0.7], [0.24, 0.48, 0.18]); }
        }
        "awning_red" | "awning_blue" => { // l'auvent de toile rayée, tendu du mur vers l'avant, festonné, sur deux tringles de fer
            let c = if m == "awning_red" { [0.72, 0.14, 0.12] } else { [0.15, 0.3, 0.62] }; let white = [0.93, 0.9, 0.82];
            let (dy, dz) = (-0.42f64, -0.85f64); let l = (dy * dy + dz * dz).sqrt();
            for i in 0..8 { let x0 = i as f64 * 0.125; k.r([x0 + 0.0625, 1.0 + dy * 0.5, 1.0 + dz * 0.5], [[0.0625, 0.0, 0.0], [0.0, dy * 0.5, dz * 0.5], [0.0, -dz / l * 0.012, dy / l * 0.012]], if i % 2 == 0 { c } else { white }); }
            for i in 0..8 { let x = i as f64 * 0.125 + 0.0625; k.b([x - 0.06, 0.48, 0.12, x + 0.06, 0.58, 0.16], if i % 2 == 0 { c } else { white }); k.b([x - 0.035, 0.44, 0.12, x + 0.035, 0.48, 0.16], if i % 2 == 0 { c } else { white }); }
            for x in [0.06, 0.94] { k.r([x, 0.6, 0.55], [[0.012, 0.0, 0.0], [0.0, 0.2, 0.42], [0.0, 0.011, -0.005]], IRON); }
        }
        "wall_lantern" => { // la lanterne murale : une potence de fer à volute, une cage à quatre vitres, une flamme, un chapeau
            k.b([0.46, 0.74, 0.62, 0.54, 0.78, 1.0], IRON); k.b([0.47, 0.5, 0.94, 0.53, 0.8, 1.0], IRON); k.b([0.47, 0.6, 0.8, 0.53, 0.63, 0.92], IRON);
            k.b([0.49, 0.62, 0.48, 0.51, 0.74, 0.5], IRON);
            k.b([0.38, 0.3, 0.36, 0.62, 0.33, 0.6], IRON); for (x, z) in [(0.38, 0.36), (0.6, 0.36), (0.38, 0.58), (0.6, 0.58)] { k.b([x, 0.33, z, x + 0.02, 0.56, z + 0.02], IRON); }
            k.e([0.4, 0.33, 0.38, 0.6, 0.56, 0.58], [1.6, 1.15, 0.55]); k.candle(0.5, 0.33, 0.48, 0.08);
            k.b([0.36, 0.56, 0.34, 0.64, 0.59, 0.62], IRON); k.b([0.41, 0.59, 0.39, 0.59, 0.62, 0.57], IRON); k.b([0.46, 0.62, 0.44, 0.54, 0.64, 0.52], IRON);
        }
        "window_arched" => { // la fenêtre cintrée : un encadrement de pierre, un appui, l'arc en plein cintre, un vitrail au plomb
            let glass = [0.5, 0.66, 0.74]; let lead = [0.16, 0.16, 0.18];
            k.b([0.0, 0.0, 0.25, 1.0, 0.1, 0.75], STONE2); k.b([0.0, 0.1, 0.3, 0.16, 1.0, 0.7], STONE); k.b([0.84, 0.1, 0.3, 1.0, 1.0, 0.7], STONE);
            for i in 0..10 { let y0 = 0.6 + i as f64 * 0.04; let yc = y0 + 0.02 - 0.6; let hw = (0.34f64 * 0.34 - yc * yc).max(0.0).sqrt(); if hw < 0.34 { k.b([0.16, y0, 0.3, 0.5 - hw, y0 + 0.04, 0.7], STONE); k.b([0.5 + hw, y0, 0.3, 0.84, y0 + 0.04, 0.7], STONE); } }
            k.b([0.44, 0.9, 0.28, 0.56, 1.0, 0.72], STONE2);
            for i in 0..12 { let y0 = 0.1 + i as f64 * 0.075; let yc = (y0 + 0.0375 - 0.6).max(0.0); let hw = (0.34f64 * 0.34 - yc * yc).max(0.0).sqrt(); if hw > 0.0 { k.b([0.5 - hw, y0, 0.49, 0.5 + hw, y0 + 0.075, 0.51], glass); } }
            k.b([0.49, 0.1, 0.48, 0.51, 0.92, 0.52], lead); for y in [0.3, 0.5, 0.7] { k.b([0.16, y, 0.48, 0.84, y + 0.015, 0.52], lead); }
        }
        "corbel" => { // la console de pierre sous un balcon ou une poutre : trois ressauts et une moulure
            k.b([0.25, 0.78, 0.35, 0.75, 1.0, 1.0], STONE); k.b([0.28, 0.56, 0.55, 0.72, 0.78, 1.0], STONE2); k.b([0.32, 0.36, 0.72, 0.68, 0.56, 1.0], STONE); k.b([0.36, 0.2, 0.84, 0.64, 0.36, 1.0], STONE2);
            k.b([0.24, 0.74, 0.33, 0.76, 0.78, 1.0], tone(STONE, 1.1));
        }
        "chimney_pot" => { // la souche de cheminée : un fût de brique, un bandeau, deux mitrons de terre cuite
            let brick = [0.6, 0.3, 0.22]; k.b([0.15, 0.0, 0.15, 0.85, 0.72, 0.85], brick); for j in 0..6 { let y = 0.1 + j as f64 * 0.12; k.b([0.14, y, 0.14, 0.86, y + 0.012, 0.86], [0.72, 0.66, 0.58]); }
            k.b([0.1, 0.72, 0.1, 0.9, 0.8, 0.9], STONE2); for x in [0.33, 0.67] { k.lathe(x, 0.5, &[(0.8, 0.1), (0.92, 0.085), (0.97, 0.1), (1.0, 0.1)], CLAY); k.b([x - 0.05, 0.99, 0.45, x + 0.05, 1.0, 0.55], [0.08, 0.06, 0.05]); }
        }
        "stone_arch" => { // l'arc de pierre au-dessus d'une baie : des claveaux autour d'un plein cintre, la clé au sommet
            for i in 0..10 { let y0 = i as f64 * 0.1; let yc = y0 + 0.05; let hw = (0.25 - yc * yc).max(0.0).sqrt(); let c = if i % 2 == 0 { STONE } else { tone(STONE, 0.93) };
                if hw > 0.0 { k.b([0.0, y0, 0.15, 0.5 - hw, y0 + 0.1, 0.85], c); k.b([0.5 + hw, y0, 0.15, 1.0, y0 + 0.1, 0.85], c); } else { k.b([0.0, y0, 0.15, 1.0, y0 + 0.1, 0.85], c); } }
            k.b([0.42, 0.48, 0.12, 0.58, 0.8, 0.88], STONE2);
        }
        "lattice_roses" => { // le treillis contre le mur, en losanges, et le rosier grimpant qui s'y accroche
            let lat = [0.62, 0.5, 0.34];
            for i in 0..5 { let c = 0.1 + i as f64 * 0.2; k.b([c - 0.015, 0.0, 0.93, c + 0.015, 1.0, 0.96], lat); k.b([0.0, c - 0.015, 0.92, 1.0, c + 0.015, 0.95], lat); }
            k.b([0.0, 0.0, 0.93, 0.03, 1.0, 0.97], lat); k.b([0.97, 0.0, 0.93, 1.0, 1.0, 0.97], lat);
            for _ in 0..22 { let (x, y) = (r.range(0.08, 0.92), r.range(0.05, 0.95)); k.b([x - 0.04, y - 0.03, 0.86, x + 0.04, y + 0.03, 0.92], [0.24, 0.46, 0.18]); }
            for _ in 0..12 { let (x, y) = (r.range(0.1, 0.9), r.range(0.3, 0.95)); k.b([x - 0.035, y - 0.035, 0.84, x + 0.035, y + 0.035, 0.9], [0.8, 0.12, 0.2]); k.b([x - 0.015, y - 0.015, 0.83, x + 0.015, y + 0.015, 0.84], [0.55, 0.05, 0.1]); }
        }
        "drainpipe" => { // la descente d'eau de zinc, ses colliers, et le dauphin qui la termine en bas
            let zinc = [0.55, 0.57, 0.58]; k.cy([0.5, 0.12, 0.9], [0.5, 1.0, 0.9], 0.05, 0.05, 0.0, zinc);
            for y in [0.3, 0.8] { k.b([0.43, y, 0.84, 0.57, y + 0.04, 1.0], tone(zinc, 0.7)); } k.cy([0.5, 0.14, 0.9], [0.5, 0.07, 0.8], 0.05, 0.055, 0.0, zinc);
        }
        "hanging_sign" => { // l'enseigne : une potence de fer, deux chaînettes, un panneau peint (une chope d'or) encadré
            k.b([0.47, 0.86, 0.2, 0.53, 0.9, 1.0], IRON); k.b([0.47, 0.7, 0.94, 0.53, 0.9, 1.0], IRON); k.r([0.5, 0.79, 0.86], [[0.03, 0.0, 0.0], [0.0, 0.09, -0.08], [0.0, 0.004, 0.004]], IRON);
            for z in [0.3, 0.7] { k.b([0.495, 0.68, z - 0.005, 0.505, 0.86, z + 0.005], IRON); }
            k.b([0.47, 0.2, 0.18, 0.53, 0.68, 0.82], DARK); k.b([0.465, 0.24, 0.22, 0.535, 0.64, 0.78], [0.2, 0.32, 0.18]);
            k.b([0.46, 0.34, 0.4, 0.54, 0.54, 0.56], GOLD); k.b([0.46, 0.38, 0.56, 0.54, 0.5, 0.62], GOLD); k.b([0.46, 0.54, 0.38, 0.54, 0.58, 0.58], [0.95, 0.95, 0.9]);
        }
        // Patrons médiévaux : cloche, chaîne, horloge murale, croix, drapeau, cuves du tanneur, peaux qui sèchent, chevalet, billot.
        "bell" => { // cloche de bronze tournée sous son joug de chêne, lèvre évasée, battant de fer
            let bz = [0.72, 0.5, 0.24]; k.b([0.05, 0.9, 0.44, 0.95, 1.0, 0.56], DARK); k.b([0.44, 0.8, 0.46, 0.56, 0.9, 0.54], IRON);
            k.lathe(0.5, 0.5, &[(0.2, 0.31), (0.24, 0.27), (0.4, 0.23), (0.6, 0.18), (0.74, 0.13), (0.82, 0.06)], bz); k.cy([0.5, 0.16, 0.5], [0.5, 0.21, 0.5], 0.33, 0.31, 0.8, tone(bz, 1.2));
            k.b([0.47, 0.08, 0.47, 0.53, 0.3, 0.53], IRON);
        }
        "chain" => { for i in 0..8 { let y = i as f64 * 0.125; if i % 2 == 0 { k.b([0.44, y, 0.47, 0.56, y + 0.15, 0.53], IRON); } else { k.b([0.47, y, 0.44, 0.53, y + 0.15, 0.56], tone(IRON, 1.3)); } } }
        "wall_clock" => { // horloge : cadran blanc cerclé de pierre, chiffres, aiguilles de fer
            k.cy([0.5, 0.5, 0.86], [0.5, 0.5, 0.98], 0.46, 0.46, 0.0, [0.62, 0.6, 0.56]); k.cy([0.5, 0.5, 0.84], [0.5, 0.5, 0.86], 0.39, 0.39, 0.0, [0.85, 0.66, 0.26]); k.cy([0.5, 0.5, 0.82], [0.5, 0.5, 0.84], 0.35, 0.35, 0.0, [0.93, 0.9, 0.82]);
            for h in 0..12 { let a = h as f64 / 12.0 * std::f64::consts::TAU; let (x, y) = (0.5 + 0.27 * a.sin(), 0.5 + 0.27 * a.cos()); k.b([x - 0.02, y - 0.02, 0.81, x + 0.02, y + 0.02, 0.82], IRON); }
            k.b([0.48, 0.48, 0.8, 0.52, 0.74, 0.81], IRON); k.b([0.48, 0.48, 0.79, 0.68, 0.52, 0.8], IRON); k.b([0.46, 0.46, 0.78, 0.54, 0.54, 0.8], GOLD);
        }
        "gravestone" => { // stèle de pierre au sommet arrondi, mangée de mousse, un tertre de terre devant
            const ST: C = [0.5, 0.5, 0.47]; const MOSS: C = [0.3, 0.42, 0.2]; const EARTH: C = [0.36, 0.26, 0.17];
            k.b([0.2, 0.0, 0.1, 0.8, 0.58, 0.24], ST); k.b([0.26, 0.58, 0.1, 0.74, 0.7, 0.24], ST); k.b([0.34, 0.7, 0.1, 0.66, 0.78, 0.24], tone(ST, 0.92)); k.b([0.16, 0.0, 0.06, 0.84, 0.1, 0.28], tone(ST, 0.8));
            k.b([0.4, 0.34, 0.24, 0.6, 0.38, 0.25], tone(ST, 0.6)); k.b([0.47, 0.26, 0.24, 0.53, 0.5, 0.25], tone(ST, 0.6)); // une croix gravée
            k.b([0.2, 0.1, 0.24, 0.42, 0.2, 0.26], MOSS); k.b([0.6, 0.44, 0.09, 0.8, 0.58, 0.1], MOSS); k.b([0.2, 0.5, 0.24, 0.3, 0.58, 0.26], MOSS);
            k.b([0.22, 0.0, 0.3, 0.78, 0.07, 0.98], EARTH); k.b([0.28, 0.07, 0.36, 0.72, 0.12, 0.92], tone(EARTH, 1.1)); }
        "grave_cross" => { // croix de bois plantée de travers, un tertre devant
            const EARTH: C = [0.36, 0.26, 0.17];
            for i in 0..8 { let t = i as f64 / 8.0; k.b([0.44 + t * 0.06, t * 0.84, 0.12, 0.54 + t * 0.06, t * 0.84 + 0.11, 0.2], DARK); } k.b([0.24, 0.52, 0.11, 0.8, 0.61, 0.21], WOOD);
            k.b([0.22, 0.0, 0.3, 0.78, 0.07, 0.98], EARTH); k.b([0.28, 0.07, 0.36, 0.72, 0.12, 0.92], tone(EARTH, 1.1)); }
        "cross" => { k.b([0.42, 0.0, 0.42, 0.58, 1.0, 0.58], GOLD); k.b([0.14, 0.6, 0.42, 0.86, 0.76, 0.58], GOLD); k.b([0.38, 0.0, 0.38, 0.62, 0.08, 0.62], tone(GOLD, 0.8)); }
        "flag" => { // mât et drapeau bleu frappé d'une fleur d'or
            k.b([0.45, 0.0, 0.45, 0.55, 1.0, 0.55], DARK); k.b([0.43, 0.98, 0.43, 0.57, 1.04, 0.57], GOLD);
            for i in 0..6 { let x0 = 0.55 + i as f64 * 0.07; let dy = (i as f64 * 0.9).sin() * 0.03; k.b([x0, 0.55 + dy, 0.48, x0 + 0.07, 0.95 + dy, 0.52], if i % 3 == 1 { [0.2, 0.3, 0.6] } else { [0.14, 0.22, 0.5] }); }
            k.b([0.68, 0.68, 0.47, 0.8, 0.82, 0.53], GOLD);
        }
        "tanning_vat" | "lime_vat" => { // cuve de pierre maçonnée, le bain dedans (tan brun ou chaux laiteuse), un bâton
            let st = [0.6, 0.58, 0.54]; k.b([0.0, 0.0, 0.0, 1.0, 0.12, 1.0], tone(st, 0.85));
            for (a, b2) in [([0.0, 0.0], [1.0, 0.12]), ([0.0, 0.88], [1.0, 1.0])] { k.b([a[0], 0.12, a[1], b2[0], 0.86, b2[1]], st); k.b([a[1], 0.12, a[0], b2[1], 0.86, b2[0]], tone(st, 0.95)); }
            let liq = if m == "tanning_vat" { [0.36, 0.22, 0.1] } else { [0.78, 0.84, 0.86] }; k.b([0.12, 0.12, 0.12, 0.88, 0.7, 0.88], liq);
            k.b([0.6, 0.5, 0.3, 0.66, 1.1, 0.36], WOOD);
        }
        "hides" => { // peaux qui sèchent sur une perche, contre le mur
            k.b([0.0, 0.88, 0.8, 1.0, 0.94, 0.86], WOOD); let hc = [[0.62, 0.42, 0.26], [0.78, 0.6, 0.42], [0.48, 0.3, 0.18]];
            for (i, x) in [0.06, 0.38, 0.7].iter().enumerate() { let c = hc[i]; k.b([*x, 0.3, 0.84, x + 0.26, 0.88, 0.86], c); k.b([x + 0.03, 0.2, 0.84, x + 0.23, 0.3, 0.86], tone(c, 0.9)); k.b([*x, 0.86, 0.79, x + 0.26, 0.9, 0.87], tone(c, 0.8)); }
        }
        "sawhorse" => { // chevalet de sciage : deux X de chêne, une bûche dessus, la scie posée
            for z in [0.15, 0.78] { for (a, b2) in [(0.1, 0.9), (0.9, 0.1)] { for i in 0..6 { let t = i as f64 / 5.0; let x = a + (b2 - a) * t; k.b([x - 0.05, t * 0.6, z, x + 0.05, t * 0.6 + 0.12, z + 0.07], WOOD); } } }
            k.b([0.2, 0.55, 0.0, 0.8, 0.85, 1.0], [0.5, 0.36, 0.22]); k.b([0.24, 0.85, 0.0, 0.76, 0.88, 0.05], [0.8, 0.68, 0.48]); k.b([0.24, 0.85, 0.95, 0.76, 0.88, 1.0], [0.8, 0.68, 0.48]);
            k.b([0.35, 0.86, 0.3, 0.38, 0.98, 0.9], [0.7, 0.72, 0.76]); k.b([0.33, 0.86, 0.9, 0.4, 1.0, 1.0], WOOD);
        }
        "chopping_block" => { // billot : un gros rond de chêne, sa tranche claire, la hache plantée
            k.lathe(0.5, 0.5, &[(0.0, 0.34), (0.06, 0.32), (0.55, 0.32)], [0.48, 0.34, 0.2]); k.cy([0.5, 0.55, 0.5], [0.5, 0.57, 0.5], 0.3, 0.3, 0.0, [0.76, 0.62, 0.42]);
            k.b([0.48, 0.5, 0.45, 0.62, 0.66, 0.55], [0.7, 0.72, 0.76]); k.b([0.55, 0.6, 0.48, 0.6, 1.05, 0.52], WOOD);
        }
        // Refonte des villages : charrette, abreuvoir, tas de charbon de bois, râtelier d'outils des champs.
        "cart" => { // charrette à bras : caisse de planches à ridelles, deux grandes roues à rayons, brancards posés au sol, un peu de foin
            k.b([0.12, 0.42, 0.08, 0.88, 0.48, 0.92], WOOD); for x in [0.12, 0.84] { k.b([x, 0.48, 0.08, x + 0.04, 0.7, 0.92], tone(WOOD, 0.9)); } for z in [0.08, 0.88] { k.b([0.12, 0.48, z, 0.88, 0.66, z + 0.04], tone(WOOD, 0.9)); }
            for i in 0..4 { let z = 0.18 + i as f64 * 0.2; k.b([0.1, 0.5, z, 0.14, 0.72, z + 0.03], tone(WOOD, 0.8)); k.b([0.86, 0.5, z, 0.9, 0.72, z + 0.03], tone(WOOD, 0.8)); }
            for sx in [0.02, 0.92] { // roue : jante en segments, moyeu, six rayons
                let (cy, cz, rr) = (0.36, 0.5, 0.34); k.cy([sx, cy, cz], [sx + 0.06, cy, cz], rr + 0.04, rr + 0.04, 0.8, DARK); // une jante ronde
                for a in 0..6 { let t = a as f64 / 6.0 * std::f64::consts::PI; for j in 1..4 { let rj = j as f64 * 0.085; for sg in [-1.0, 1.0] { let (y, z) = (cy + sg * rj * t.sin(), cz + sg * rj * t.cos()); k.b([sx + 0.01, y - 0.018, z - 0.018, sx + 0.05, y + 0.018, z + 0.018], tone(WOOD, 0.85)); } } }
                k.cy([sx - 0.01, cy, cz], [sx + 0.07, cy, cz], 0.06, 0.06, 0.0, IRON); }
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
        "mooring_post" => { // bitte d'amarrage : un gros pieu rond goudronné à tête bombée, une corde lovée autour
            const TAR: C = [0.2, 0.15, 0.11]; k.lathe(0.5, 0.5, &[(0.0, 0.2), (0.78, 0.2)], TAR); k.lathe(0.5, 0.5, &[(0.78, 0.24), (0.86, 0.24), (0.9, 0.16)], tone(TAR, 1.2));
            for (i, y) in [0.42, 0.48, 0.54].iter().enumerate() { let w = 0.25 - i as f64 * 0.01; k.cy([0.5, *y, 0.5], [0.5, y + 0.05, 0.5], w, w, 0.75, if i % 2 == 0 { ROPE } else { ROPE2 }); }
            k.b([0.66, 0.0, 0.4, 0.95, 0.06, 0.6], ROPE); k.b([0.72, 0.06, 0.45, 0.9, 0.09, 0.55], ROPE2); // la corde qui traîne
        }
        "buoy" => { // bouée : une sphère rouge à bande blanche, un anneau et un bout de corde
            let (red, white) = ([0.85, 0.2, 0.15], [0.92, 0.9, 0.86]);
            k.lathe(0.5, 0.5, &[(0.0, 0.1), (0.07, 0.2), (0.18, 0.26)], red); k.lathe(0.5, 0.5, &[(0.18, 0.26), (0.26, 0.275), (0.34, 0.26)], white); k.lathe(0.5, 0.5, &[(0.34, 0.26), (0.45, 0.2), (0.52, 0.1)], red);
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
        "sapling" => { // un plant de chêne : une tige fine, trois bouquets de feuilles, une motte de terre au pied
            const STEM: C = [0.42, 0.3, 0.18]; const LEAF: C = [0.3, 0.5, 0.17]; const EARTH: C = [0.36, 0.26, 0.17];
            k.b([0.36, 0.0, 0.36, 0.64, 0.06, 0.64], EARTH); k.b([0.47, 0.0, 0.47, 0.53, 0.62, 0.53], STEM); k.b([0.5, 0.3, 0.48, 0.66, 0.34, 0.52], STEM);
            k.b([0.3, 0.5, 0.32, 0.6, 0.78, 0.62], LEAF); k.b([0.5, 0.38, 0.42, 0.74, 0.58, 0.64], tone(LEAF, 1.12)); k.b([0.4, 0.66, 0.42, 0.62, 0.86, 0.6], tone(LEAF, 0.9)); }
        "chest" => { // planches cerclées de fer, couvercle bombé, la serrure sur le devant (côté -z, comme les portes d'une armoire)
            const CW: C = [0.55, 0.37, 0.2]; const BAND: C = [0.24, 0.24, 0.25];
            k.b([0.08, 0.0, 0.14, 0.92, 0.56, 0.86], CW); k.b([0.07, 0.56, 0.13, 0.93, 0.72, 0.87], tone(CW, 1.1)); k.b([0.1, 0.72, 0.18, 0.9, 0.78, 0.82], tone(CW, 1.05));
            for a in [0.16, 0.8] { k.b([a, 0.0, 0.13, a + 0.05, 0.785, 0.87], BAND); } k.b([0.44, 0.42, 0.11, 0.56, 0.6, 0.14], [0.7, 0.58, 0.3]); }
        "chest_open" => {
            const CW: C = [0.55, 0.37, 0.2]; const CW2: C = [0.6, 0.41, 0.22]; const BAND: C = [0.24, 0.24, 0.25];
            k.b([0.08, 0.0, 0.14, 0.92, 0.08, 0.86], CW); k.b([0.08, 0.08, 0.14, 0.92, 0.56, 0.2], CW); k.b([0.08, 0.08, 0.8, 0.92, 0.56, 0.86], CW); k.b([0.08, 0.08, 0.2, 0.14, 0.56, 0.8], CW); k.b([0.86, 0.08, 0.2, 0.92, 0.56, 0.8], CW);
            k.b([0.14, 0.08, 0.2, 0.86, 0.1, 0.8], [0.16, 0.1, 0.06]);
            for a in [0.16, 0.8] { k.b([a, 0.0, 0.13, a + 0.05, 0.565, 0.87], BAND); }
            let th = 1.75f64; let (dz, dy) = ([0.0, th.sin(), -th.cos()], [0.0, th.cos(), th.sin()]); let hinge = [0.5, 0.56, 0.87]; // la charnière au dos (+z), la serrure devant (-z)
            let c = [hinge[0], hinge[1] + dz[1] * 0.37 + dy[1] * 0.11, hinge[2] + dz[2] * 0.37 + dy[2] * 0.11];
            k.r(c, [[0.43, 0.0, 0.0], [0.0, dy[1] * 0.11, dy[2] * 0.11], [0.0, dz[1] * 0.37, dz[2] * 0.37]], CW2);
            for a in [0.185, 0.825] { let cc = [a, c[1] + dy[1] * 0.005, c[2] + dy[2] * 0.005]; k.r(cc, [[0.026, 0.0, 0.0], [0.0, dy[1] * 0.115, dy[2] * 0.115], [0.0, dz[1] * 0.372, dz[2] * 0.372]], BAND); }
            k.b([0.44, 0.42, 0.11, 0.56, 0.54, 0.14], [0.7, 0.58, 0.3]);
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
            k.cy([cx, cy, 0.475], [cx, cy, 0.525], rr + 0.025, rr + 0.025, 0.84, WOOD); k.cy([cx, cy, 0.46], [cx, cy, 0.54], 0.04, 0.04, 0.0, DARK); // une roue ronde et son moyeu
            for i in 0..4 { let a = i as f64 * std::f64::consts::FRAC_PI_4 * 2.0 + 0.3; k.r([cx, cy, 0.5], [[a.cos() * rr, a.sin() * rr, 0.0], [-a.sin() * 0.012, a.cos() * 0.012, 0.0], [0.0, 0.0, 0.012]], DARK); }
            k.b([0.7, 0.26, 0.45, 0.76, 0.6, 0.55], DARK); k.b([0.68, 0.55, 0.42, 0.82, 0.62, 0.58], [0.85, 0.8, 0.66]);
        }
        "grindstone" => { // une meule de grès ronde sur son bâti, l'axe et la manivelle
            for a in [0.15, 0.78] { k.b([a, 0.0, 0.2, a + 0.07, 0.62, 0.8], WOOD); } k.b([0.15, 0.1, 0.45, 0.85, 0.16, 0.55], DARK);
            k.cy([0.42, 0.55, 0.5], [0.58, 0.55, 0.5], 0.34, 0.34, 0.0, [0.62, 0.6, 0.55]);
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
        "shelf" => { // l'étagère vide, pour y poser ce qu'on veut : deux montants, trois planches, un fond
            for x in [0.0, 0.94] { k.b([x, 0.0, 0.56, x + 0.06, 1.0, 1.0], DARK); } k.b([0.06, 0.02, 0.96, 0.94, 0.98, 1.0], tone(WOOD, 0.8));
            for y in [0.06, 0.52, 0.94] { k.t([0.06, y, 0.56, 0.94, y + 0.06, 0.96], "planks", [0.95, 0.9, 0.85]); }
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
        "round_table" => { // un vrai plateau rond, un fût tourné, un pied évasé
            k.cy([0.5, 0.84, 0.5], [0.5, 0.92, 0.5], 0.46, 0.46, 0.0, WOOD); k.cy([0.5, 0.8, 0.5], [0.5, 0.84, 0.5], 0.3, 0.4, 0.0, DARK);
            k.lathe(0.5, 0.5, &[(0.0, 0.3), (0.05, 0.28), (0.1, 0.1), (0.2, 0.07), (0.45, 0.09), (0.7, 0.06), (0.8, 0.1)], DARK);
        }
        "stool" => { // assise ronde, trois pieds tournés
            k.cy([0.5, 0.465, 0.5], [0.5, 0.535, 0.5], 0.21, 0.21, 0.0, WOOD);
            for i in 0..3 { let a = i as f64 * 2.094; let (ca, sa) = (a.cos(), a.sin()); k.cy([0.5 + ca * 0.17, 0.0, 0.5 + sa * 0.17], [0.5 + ca * 0.12, 0.47, 0.5 + sa * 0.12], 0.03, 0.03, 0.0, DARK); }
        }
        "candle" => { k.b([0.38, 0.0, 0.38, 0.62, 0.03, 0.62], [0.55, 0.42, 0.2]); k.candle(0.5, 0.03, 0.5, 0.26); }
        "candelabra" => {
            k.b([0.36, 0.0, 0.36, 0.64, 0.05, 0.64], GOLD); k.b([0.47, 0.05, 0.47, 0.53, 0.55, 0.53], GOLD); k.b([0.2, 0.52, 0.47, 0.8, 0.56, 0.53], GOLD);
            for x in [0.22, 0.78] { k.b([x - 0.03, 0.56, 0.47, x + 0.03, 0.6, 0.53], GOLD); k.candle(x, 0.6, 0.5, 0.16); } k.b([0.46, 0.55, 0.46, 0.54, 0.62, 0.54], GOLD); k.candle(0.5, 0.62, 0.5, 0.2);
        }
        "chandelier" => {
            k.b([0.48, 0.55, 0.48, 0.52, 1.0, 0.52], IRON);
            k.cy([0.5, 0.38, 0.5], [0.5, 0.46, 0.5], 0.4, 0.4, 0.8, DARK); // une couronne ronde
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
        "amphora" => { // panse tournée, col étroit, lèvre, deux anses, une bande peinte
            k.lathe(0.5, 0.5, &[(0.0, 0.09), (0.05, 0.1), (0.18, 0.2), (0.34, 0.25), (0.5, 0.22), (0.64, 0.12), (0.78, 0.075), (0.82, 0.12), (0.86, 0.12)], CLAY);
            k.cy([0.5, 0.35, 0.5], [0.5, 0.39, 0.5], 0.256, 0.25, 0.9, [0.25, 0.12, 0.07]);
            k.b([0.28, 0.58, 0.47, 0.33, 0.78, 0.53], tone(CLAY, 0.9)); k.b([0.67, 0.58, 0.47, 0.72, 0.78, 0.53], tone(CLAY, 0.9)); k.b([0.33, 0.74, 0.47, 0.42, 0.78, 0.53], tone(CLAY, 0.9)); k.b([0.58, 0.74, 0.47, 0.67, 0.78, 0.53], tone(CLAY, 0.9));
        }
        "flower_pot" | "potted_fern" => {
            k.lathe(0.5, 0.5, &[(0.0, 0.15), (0.27, 0.21)], CLAY); k.lathe(0.5, 0.5, &[(0.27, 0.235), (0.33, 0.235)], tone(CLAY, 1.1)); k.cy([0.5, 0.33, 0.5], [0.5, 0.335, 0.5], 0.2, 0.2, 0.0, [0.25, 0.17, 0.1]);
            let cols = [[0.8, 0.15, 0.18], [0.95, 0.85, 0.3], [0.75, 0.45, 0.85], [0.95, 0.95, 0.9]]; let fl = cols[r.int(0, 3) as usize];
            let n = if m == "potted_fern" { 9 } else { 6 };
            for i in 0..n { let a = i as f64 * std::f64::consts::TAU / n as f64 + r.range(0.0, 0.5); let len = r.range(0.2, 0.32); let (ca, sa) = (a.cos(), a.sin());
                let c = [0.5 + ca * len * 0.5, 0.42 + len * 0.3, 0.5 + sa * len * 0.5]; k.r(c, [[ca * len * 0.5, len * 0.3, sa * len * 0.5], [-sa * 0.05, 0.0, ca * 0.05], [0.0, 0.01, 0.0]], [0.2, 0.42, 0.15]);
                if m == "flower_pot" { let p = [0.5 + ca * len * 0.9, 0.5 + len * 0.6, 0.5 + sa * len * 0.9]; k.b([p[0] - 0.05, p[1], p[2] - 0.05, p[0] + 0.05, p[1] + 0.07, p[2] + 0.05], fl); } }
        }
        "cauldron" => { // un chaudron de fer rond et ventru, son bouillon, trois pieds, deux oreilles
            k.lathe(0.5, 0.5, &[(0.08, 0.22), (0.16, 0.34), (0.34, 0.39), (0.5, 0.37), (0.58, 0.34), (0.62, 0.36)], IRON);
            k.cy([0.5, 0.56, 0.5], [0.5, 0.6, 0.5], 0.33, 0.33, 0.0, [0.35, 0.42, 0.18]); k.cy([0.5, 0.58, 0.5], [0.5, 0.625, 0.5], 0.37, 0.37, 0.88, tone(IRON, 1.3));
            for i in 0..3 { let a = i as f64 * 2.094 + 0.5; let (ca, sa) = (a.cos(), a.sin()); k.cy([0.5 + ca * 0.27, 0.0, 0.5 + sa * 0.27], [0.5 + ca * 0.24, 0.12, 0.5 + sa * 0.24], 0.035, 0.04, 0.0, IRON); }
            k.b([0.1, 0.55, 0.47, 0.15, 0.62, 0.53], IRON); k.b([0.85, 0.55, 0.47, 0.9, 0.62, 0.53], IRON);
        }
        "bucket" => { // un seau de douelles, évasé, deux cercles de fer, de l'eau, une anse
            k.cy([0.5, 0.0, 0.5], [0.5, 0.4, 0.5], 0.16, 0.2, 0.0, WOOD); for (y, rr) in [(0.06, 0.172), (0.32, 0.196)] { k.cy([0.5, y, 0.5], [0.5, y + 0.04, 0.5], rr, rr + 0.002, 0.9, IRON); }
            k.cy([0.5, 0.39, 0.5], [0.5, 0.405, 0.5], 0.17, 0.17, 0.0, [0.3, 0.45, 0.6]); k.b([0.3, 0.38, 0.48, 0.33, 0.55, 0.52], IRON); k.b([0.67, 0.38, 0.48, 0.7, 0.55, 0.52], IRON); k.b([0.3, 0.55, 0.48, 0.7, 0.58, 0.52], IRON);
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
        // le levier : une platine de fer contre le mur, une manette de bois au pommeau rouge, levée (« lever ») ou abaissée (« lever_on »)
        "lever" | "lever_on" => {
            k.b([0.3, 0.3, 0.92, 0.7, 0.7, 1.0], IRON); k.b([0.36, 0.36, 0.89, 0.64, 0.64, 0.92], [0.24, 0.24, 0.26]); for (x, y) in [(0.33, 0.33), (0.67, 0.33), (0.33, 0.67), (0.67, 0.67)] { k.b([x - 0.02, y - 0.02, 0.9, x + 0.02, y + 0.02, 0.93], [0.4, 0.4, 0.42]); }
            let up = m == "lever"; let tip = if up { [0.5, 0.86, 0.62] } else { [0.5, 0.14, 0.62] };
            k.cy([0.5, 0.5, 0.88], tip, 0.035, 0.03, 0.0, WOOD); k.cy(tip, [tip[0], tip[1] + if up { 0.05 } else { -0.05 }, tip[2] - 0.02], 0.06, 0.055, 0.0, [0.62, 0.12, 0.1]);
        }
        // le lance-fléchettes : un bloc de pierre taillée ; sur chaque face, un masque grimaçant en relief dans un cadre, les yeux
        // creux, et une bouche sombre d'où dépassent trois pointes de fléchettes en bronze
        "dart_trap" => {
            k.t([0.0, 0.0, 0.0, 1.0, 1.0, 1.0], "polished_stone", [0.78, 0.76, 0.72]);
            let (frame, mask, dark, bronze) = ([0.36, 0.34, 0.32], [0.62, 0.58, 0.52], [0.04, 0.03, 0.03], [0.72, 0.5, 0.22]);
            // une boîte décrite sur la face +z (u en travers, v en hauteur, d vers l'extérieur depuis la face), puis tournée sur les quatre côtés
            let face = |k: &mut K, q: u8, u0: f64, v0: f64, u1: f64, v1: f64, d0: f64, d1: f64, c: C| {
                let (a0, a1, b0, b1) = (u0, u1, 1.0 + d0, 1.0 + d1); let pts = [(a0, b0), (a1, b1)];
                let rot = |(x, z): (f64, f64)| -> (f64, f64) { let (mut x, mut z) = (x - 0.5, z - 0.5); for _ in 0..q { (x, z) = (z, -x); } (x + 0.5, z + 0.5) };
                let (p, r) = (rot(pts[0]), rot(pts[1])); k.b([p.0.min(r.0), v0, p.1.min(r.1), p.0.max(r.0), v1, p.1.max(r.1)], c); };
            for q in 0..4u8 {
                for (u0, v0, u1, v1) in [(0.12, 0.12, 0.88, 0.18), (0.12, 0.82, 0.88, 0.88), (0.12, 0.18, 0.18, 0.82), (0.82, 0.18, 0.88, 0.82)] { face(k, q, u0, v0, u1, v1, -0.002, 0.02, frame); }
                face(k, q, 0.25, 0.24, 0.75, 0.76, -0.002, 0.035, mask); face(k, q, 0.3, 0.6, 0.45, 0.66, 0.035, 0.05, tone(mask, 0.8)); face(k, q, 0.55, 0.6, 0.7, 0.66, 0.035, 0.05, tone(mask, 0.8));
                face(k, q, 0.33, 0.5, 0.43, 0.57, 0.03, 0.037, dark); face(k, q, 0.57, 0.5, 0.67, 0.57, 0.03, 0.037, dark);
                face(k, q, 0.3, 0.3, 0.7, 0.42, 0.03, 0.038, dark);
                for u in [0.38, 0.5, 0.62] { face(k, q, u - 0.015, 0.345, u + 0.015, 0.375, 0.035, 0.07, bronze); }
            }
        }
        // L'architecture ancestrale des donjons : une colonne cannelée en trois pièces (base, fût, chapiteau), une statue de gardien
        // sur deux blocs, un sarcophage et son gisant sur deux blocs, un bas-relief, un linteau à frise, une urne funéraire, un autel
        // sculpté, un brasier sur pied. Pierre calcaire vieillie, rehaussée de mousse au hasard.
        "column_base" | "column_shaft" | "column_capital" | "statue_base" | "statue_top" | "tomb_head" | "tomb_foot" | "wall_relief" | "lintel" | "funeral_urn" | "altar" | "brazier_stand" | "great_chain" => ancient(k, m, r),
        // Le Temple du Soleil : un miroir d'or tournant sur son pied (la glace sur la diagonale du bloc), le disque solaire
        // qui renvoie la lumière tombée de l'oculus, le réceptacle de cristal (éteint, embrasé), et le rayon lui-même.
        "sun_mirror" | "sun_disc" | "sun_receptor" | "sun_receptor_on" | "light_beam" => solar(k, m, r),
        // Les Catacombes des Rois : un mur d'ossuaire (crânes rangés en rangs entre des lits de longs os, sur un fond de pierre
        // sombre), un tas d'ossements jeté au sol.
        "ossuary_wall" | "bone_pile" => ossuary(k, m, r),
        // les pics : une plaque de fer percée, et, sortis, des pointes acérées ; rentrés, seuls leurs bouts affleurent
        "spikes" | "spikes_down" => {
            k.b([0.02, 0.0, 0.02, 0.98, 0.06, 0.98], [0.22, 0.22, 0.24]); let out = m == "spikes";
            for i in 0..4 { for j in 0..4 { let (x, z) = (0.14 + i as f64 * 0.24, 0.14 + j as f64 * 0.24); k.b([x - 0.045, 0.06, z - 0.045, x + 0.045, 0.07, z + 0.045], [0.08, 0.08, 0.09]);
                if out { k.cy([x, 0.06, z], [x, 0.72, z], 0.05, 0.004, 0.0, [0.55, 0.55, 0.58]); } else { k.cy([x, 0.06, z], [x, 0.11, z], 0.04, 0.02, 0.0, [0.45, 0.45, 0.48]); } } }
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
        "beehive" => { // une ruche de paille tressée en dôme, son trou d'envol
            k.lathe_t(0.5, 0.5, &[(0.0, 0.36), (0.16, 0.39), (0.32, 0.37), (0.46, 0.31), (0.58, 0.23), (0.7, 0.13), (0.8, 0.0)], "hay", [0.82, 0.82, 0.82]);
            for y in [0.15, 0.31, 0.45, 0.57] { let rr = 0.39 - (y - 0.15f64).max(0.0) * 0.36; k.cy([0.5, y, 0.5], [0.5, y + 0.012, 0.5], rr + 0.012, rr + 0.012, 0.92, [0.5, 0.4, 0.2]); }
            k.b([0.43, 0.02, 0.1, 0.57, 0.1, 0.15], [0.08, 0.06, 0.04]);
        }
        "cheese" => { // deux meules rondes empilées, croûte et dessus plus clair
            let (c, rind) = ([0.95, 0.78, 0.3], [0.8, 0.55, 0.18]);
            for (y, rr) in [(0.0, 0.3), (0.18, 0.26)] { k.lathe(0.5, 0.5, &[(y, rr - 0.02), (y + 0.03, rr), (y + 0.15, rr), (y + 0.18, rr - 0.02)], rind); k.cy([0.5, y + 0.18, 0.5], [0.5, y + 0.183, 0.5], rr - 0.05, rr - 0.05, 0.0, c); }
        }
        "bread_basket" => {
            k.t([0.15, 0.0, 0.2, 0.85, 0.22, 0.8], "wattle", [1.2, 1.1, 0.9]); k.b([0.18, 0.2, 0.23, 0.82, 0.21, 0.77], [0.4, 0.3, 0.18]);
            for (x, z, a) in [(0.35, 0.4, 0.3f64), (0.62, 0.55, -0.4), (0.45, 0.62, 1.2)] { k.r([x, 0.28, z], [[a.cos() * 0.15, 0.0, a.sin() * 0.15], [0.0, 0.07, 0.0], [-a.sin() * 0.08, 0.0, a.cos() * 0.08]], [0.72, 0.45, 0.18]); }
        }
        _ => {}
    }
}
