//! Génération procédurale : le monde est une fonction pure de (seed, x, y, z).
use crate::biome::{Biome, Climate, Mix};
use crate::blocks::*;
use crate::det::{cos, hypot2, hypot3, round, sin};
use crate::noise::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

const TREE_CELL: i32 = 13; const ROCK_CELL: i32 = 19; const POND_CELL: i32 = 96; const STAMP_REACH: i32 = 38;

#[derive(Clone, Copy, Default, Debug)]
pub struct Column { pub mix: Mix, pub wet: bool, pub h: f64, pub water: f64, pub still: bool, pub forest: f64, pub path: f64, pub bank: f64, pub cliff: f64 }
#[derive(Clone, Copy)] struct Pond { x: f64, z: f64, r: f64, level: f64 }
struct Raw { wx: f64, wz: f64, low: f64, hs: f64, massif: f64 }
/// Empreinte : voxels (x, y, z, id) d'un arbre ou d'un rocher, posés dans chaque chunk touché.
pub struct Stamp { data: Vec<[i32; 4]>, force: bool }

struct Bed { d_r: f64, wid: f64, l: f64, grad: f64 }
const LIP: f64 = 0.55; // part du lit qui se déverse au bord d'un palier
fn fdiv(a: i32, b: i32) -> i32 { a.div_euclid(b) }
#[inline] fn ri(x: f64) -> i32 { round(x) as i32 }

pub struct WorldGen {
    pub seed: i32,
    n_base: Noise, n_hill: Noise, n_warp: Noise, n_river: Noise, n_path: Noise, n_forest: Noise, n_cliff: Noise, n_cave: Noise, pub n_deco: Noise,
    pub climate: Climate,
    ponds: RefCell<HashMap<(i32, i32), Option<Pond>>>,
    stamps: RefCell<HashMap<(u8, i32, i32), Option<Rc<Stamp>>>>,
}

impl WorldGen {
    pub fn new(seed: i32) -> Self {
        let s = seed; let n = |k: i32| Noise::new(s.wrapping_add(k));
        WorldGen { seed: s, n_base: n(1), n_hill: n(2), n_warp: n(3), n_river: n(4), n_path: n(5), n_forest: n(6), n_cliff: n(7), n_cave: n(8), n_deco: n(9), climate: Climate::new(s), ponds: RefCell::new(HashMap::new()), stamps: RefCell::new(HashMap::new()) }
    }
    fn raw_height(&self, x: f64, z: f64) -> Raw {
        let wx = x + 34.0 * self.n_warp.n2(x / 170.0, z / 170.0); let wz = z + 34.0 * self.n_warp.n2(x / 170.0 + 40.0, z / 170.0 + 40.0);
        // Grand relief : des massifs rares et lointains, posés sur les collines, jamais au-dessus du plafond du monde.
        let massif = smooth(0.14, 0.5, self.n_base.fbm(wx / 1500.0 + 210.0, wz / 1500.0 - 130.0, 3));
        let ridge = 1.0 - self.n_hill.fbm(wx / 330.0 + 80.0, wz / 330.0 + 25.0, 3).abs() * 1.6;
        let low = 52.0 + 15.0 * self.n_base.fbm(wx / 340.0, wz / 340.0, 3) + massif * (14.0 + 34.0 * ridge.max(0.0) * ridge.max(0.0));
        let hs = low + 5.5 * self.n_hill.fbm(wx / 96.0, wz / 96.0, 3);
        Raw { wx, wz, low, hs, massif }
    }
    fn pond_at(&self, gx: i32, gz: i32) -> Option<Pond> {
        if let Some(p) = self.ponds.borrow().get(&(gx, gz)) { return *p; }
        let s = self.seed;
        let p = if hash_float(gx, gz, s, 71) > 0.42 { None } else {
            let x = (gx as f64 + 0.2 + 0.6 * hash_float(gx, gz, s, 72)) * POND_CELL as f64; let z = (gz as f64 + 0.2 + 0.6 * hash_float(gx, gz, s, 73)) * POND_CELL as f64;
            // Un lac se pose à plat, au niveau du point bas de sa rive ; sur une pente, il n'y a pas de lac.
            let r = 7.0 + 8.0 * hash_float(gx, gz, s, 74); let mid = self.raw_height(x, z); let (mut lo, mut hi) = (mid.hs, mid.hs);
            for k in 0..8 { let a = k as f64 * 0.7854; let q = self.raw_height(x + cos(a) * r * 1.2, z + sin(a) * r * 1.2).hs; lo = lo.min(q); hi = hi.max(q); }
            if hi - lo > 5.0 || mid.massif > 0.02 { None } else { Some(Pond { x, z, r, level: lo.floor() - 1.0 }) }
        };
        self.ponds.borrow_mut().insert((gx, gz), p);
        p
    }
    /// Altitude, eau, chemin et densité forestière d'une colonne.
    /// Lit de rivière en ce point. Un massif tarit la rivière : elle y prend sa source au lieu de gravir la pente.
    fn bed(&self, x: f64, z: f64, wx: f64, wz: f64, rs: f64, low: f64, massif: f64) -> Bed {
        let (gx0, gz0) = (self.n_river.fbm2((wx + 1.0) / 400.0, wz / 400.0, 2, 2.1, 0.32) - rs, self.n_river.fbm2(wx / 400.0, (wz + 1.0) / 400.0, 2, 2.1, 0.32) - rs); let grad = hypot2(gx0, gz0).max(1e-5);
        let dry = 1.0 - smooth(0.04, 0.26, massif); let w0 = (1.7 + 1.2 * (0.5 + 0.5 * self.n_river.n2(x / 60.0, z / 60.0))) * dry;
        Bed { d_r: rs.abs() / grad, wid: if w0 < 0.8 { 0.0 } else { w0 }, l: ((low - 4.0) / 4.0).floor() * 4.0 + 1.0, grad }
    }
    /// Niveau de la rivière ici, s'il y en a une, et la position dans le lit (0 au milieu, 1 au bord).
    fn river_level(&self, x: f64, z: f64) -> Option<(f64, f64)> {
        let Raw { wx, wz, low, massif, .. } = self.raw_height(x, z); let b = self.bed(x, z, wx, wz, self.n_river.fbm2(wx / 400.0, wz / 400.0, 2, 2.1, 0.32), low, massif);
        (b.wid > 0.0 && b.d_r < b.wid).then_some((b.l, b.d_r / b.wid))
    }
    /// Sommet de la chute qui tombe dans cette colonne : le palier voisin plus haut, s'il s'y déverse.
    fn fall_top(&self, x: f64, z: f64, water: f64) -> f64 { let mut t = 0.0f64; for (a, b) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] { if let Some((l, k)) = self.river_level(x + a, z + b) { if l > water && k < LIP { t = t.max(l); } } } t }
    pub fn column(&self, x: f64, z: f64) -> Column {
        let Raw { wx, wz, low, hs, massif } = self.raw_height(x, z);
        let step_n = self.n_hill.n2(x / 52.0 + 11.0, z / 52.0 - 7.0); let s = 4.0;
        let q = hs / s; let fl = q.floor(); let fr = q - fl;
        let terr = (fl + smooth(0.32, 0.68, fr)) * s;
        let mut h = lerp(hs, terr, smooth(-0.35, 0.25, step_n) * (1.0 - massif)); // les terrasses sont pour les collines : une montagne en escalier ferait une pyramide
        h += 1.6 * self.n_hill.n2(x / 19.0, z / 19.0);
        // rivière : niveaux quantifiés = cascades
        let rs = self.n_river.fbm2(wx / 400.0, wz / 400.0, 2, 2.1, 0.32);
        let Bed { d_r, wid, l, grad } = self.bed(x, z, wx, wz, rs, low, massif);
        let (mut water, mut bank, mut still) = (0.0, 0.0, false); let mut wet = false;
        if wid > 0.0 && d_r < wid + 13.0 {
            wet = true; bank = 1.0 - smooth(wid, wid + 13.0, d_r);
            let target = l + 1.0 + 3.0 * smooth(wid, wid + 8.0, d_r);
            if h > target { h = lerp(h, target, smooth(0.0, 1.0, bank * 1.25)); }
            // Au bord d'un palier, seul le milieu du lit se déverse : les côtés forment une margelle de roche.
            let lip = d_r >= wid * LIP && d_r < wid && [(3.0, 0.0), (-3.0, 0.0), (0.0, 3.0), (0.0, -3.0)].iter().any(|(a, b)| self.river_level(x + a, z + b).is_some_and(|n| n.0 < l));
            if d_r < wid && !lip { h = l - 2.0 - (1.0 - d_r / wid) * 1.6; water = l; }
            else {
                // La berge retient le niveau le plus haut qu'elle touche, sinon l'eau d'un palier fuirait sur le côté.
                let mut top = l; if d_r < wid + 5.0 { for a in -3..=3 { for b in -3..=3 { if let Some(n) = self.river_level(x + a as f64, z + b as f64) { top = top.max(n.0); } } } }
                if h < top + 1.0 { h = top + 1.0; }
            }
        }
        // étangs
        let (gx, gz) = ((x / POND_CELL as f64).floor() as i32, (z / POND_CELL as f64).floor() as i32);
        for i in -1..=1 { for j in -1..=1 {
            let Some(p) = self.pond_at(gx + i, gz + j) else { continue };
            let d = hypot2(x - p.x, z - p.z) + 3.0 * self.n_deco.n2(x / 9.0, z / 9.0);
            if d > p.r * 2.0 { continue; }
            wet = true;
            let bowl = lerp(p.level - 3.0, h, smooth(p.r * 0.7, p.r * 1.8, d));
            if bowl < h { h = bowl; }
            if d < p.r * 1.05 && h < p.level && p.level > water { water = p.level; still = true; }
            // La berge retient l'eau : le sol remonte jusqu'au-dessus du niveau du lac, puis rejoint le terrain en pente douce.
            if d >= p.r * 1.05 { let rim = p.level + 1.0 - (d - p.r * 1.05) * 0.8; if h < rim { h = rim; } }
        } }
        // chemins : réseau de traverse, et un sentier le long de chaque ruisseau
        let pn = self.n_path.n2(wx / 210.0 + 9.0, wz / 210.0 + 3.0).abs() * 165.0;
        let mut path = 0.0;
        let bank_d = (rs / grad - (wid + 6.0)).abs(); let wob = self.n_deco.n2(x / 5.0, z / 5.0) * 0.6;
        if water == 0.0 {
            let edge = 1.0 - smooth(0.9, 2.3, bank_d + wob);
            if bank < 0.45 {
                path = 1.0 - smooth(0.9, 2.3, pn + wob);
                let near = 1.0 - smooth(1.5, 8.0, pn);
                if near > 0.0 { h = lerp(h, hs, near * 0.82); }
            }
            if edge > path { path = edge; }
            let walk = 1.0 - smooth(1.5, 5.0, bank_d);
            if walk > 0.0 { h = lerp(h, low - 1.0, walk); }
        }
        let f = self.n_forest.fbm(x / 170.0, z / 170.0, 3) * 1.5 + 0.78 + 0.12 * self.n_forest.n2(x / 31.0, z / 31.0);
        // La prairie éclaircit la forêt ; le relief et l'eau restent communs, pour que rien ne se voie à la frontière.
        let mix = self.climate.at(x, z); let f = f - 1.1 * mix.strong(Biome::Meadow) + 0.7 * mix.strong(Biome::DeepForest);
        Column { mix, wet, h: h.min(120.0), water, still, forest: clamp(f, 0.0, 1.0), path, bank, cliff: smooth(0.1, 0.55, self.n_cliff.n2(x / 120.0, z / 120.0)) }
    }
    fn stone_at(&self, x: f64, y: f64, z: f64) -> u8 {
        let b = (y + 5.0 * self.n_cliff.n2(x / 70.0, z / 70.0)) / 7.0;
        match (b.floor() as i64).rem_euclid(3) { 2 => B::LIMESTONE, 1 => B::GRANITE, _ => B::STONE }
    }
    fn is_cave(&self, x: f64, y: f64, z: f64) -> bool {
        let a = self.n_cave.n3(x / 30.0, y / 18.0, z / 30.0); let b = self.n_cave.n3(x / 30.0 + 50.0, y / 18.0 - 20.0, z / 30.0 + 90.0);
        a * a + b * b < 0.0065
    }
    /// Terrain, décor et empreintes, sans aucune dépendance à l'ordre de chargement.
    pub fn gen_chunk(&self, cx: i32, cz: i32) -> Vec<u8> {
        let mut v = vec![0u8; CA]; let (ox, oz) = (cx * CX as i32, cz * CX as i32);
        let mut heights = vec![0i32; CX * CX]; let mut waters = vec![0i32; CX * CX]; let mut info = vec![(0.0f64, 0.0f64, 0.0f64, 0.0f64); CX * CX];
        for z in 0..CX { for x in 0..CX {
            let (wxi, wzi) = (ox + x as i32, oz + z as i32); let (wx, wz) = (wxi as f64, wzi as f64);
            let c = self.column(wx, wz); let (hf, cliff, fr, path, water) = (c.h, c.cliff, c.forest, c.path, c.water);
            let mut top: i32 = 0; let deep = c.mix.strong(Biome::DeepForest); let taiga = c.mix.strong(Biome::Taiga); let cold = hash_float(wxi, wzi, self.seed, 42) < smooth(0.3, 0.7, taiga);
            // Par endroits, le toit des galeries s'efface : la grotte s'ouvre en surface, jamais sous l'eau ni sous un chemin.
            let entry = if water == 0.0 && path < 0.1 { smooth(0.3, 0.5, self.n_cave.n2(wx / 70.0, wz / 70.0)) } else { 0.0 };
            let ymax = (CH as f64 - 2.0).min((hf + 7.0).ceil()) as i32;
            for y in 0..=ymax {
                let yf = y as f64; let mut d = hf - yf;
                if cliff > 0.01 && !c.wet && yf > hf - 12.0 { d += cliff * 6.0 * (1.0 - clamp((yf - hf) / 8.0, 0.0, 1.0)) * self.n_cave.n3(wx / 25.0, yf / 15.0, wz / 25.0); }
                if d > 0.0 {
                    if y > 2 && !c.wet && yf < hf - 3.0 + entry * 5.0 && self.is_cave(wx, yf, wz) { continue; } // jamais de vide à côté de l'eau
                    v[idx(x, y as usize, z)] = B::STONE; top = y;
                }
            }
            // habillage : strates, sous-couche, surface
            let mut depth = 0;
            for y in (0..=top).rev() {
                let i = idx(x, y as usize, z);
                if v[i] == 0 { depth = 0; continue; }
                let steep = depth > if path > 0.3 { 4 } else { 2 + ((wxi ^ wzi) & 1) };
                if depth == 0 {
                    let yf = y as f64;
                    v[i] = if path > 0.45 { B::PATH }
                    else if water != 0.0 && yf < water + 1.0 { if self.n_deco.n2(wx / 7.0, wz / 7.0) > 0.0 { B::GRAVEL } else { B::SAND } }
                    else if c.bank > 0.72 && yf <= water + 2.0 { B::GRAVEL }
                    else if yf > 101.0 + 4.0 * self.n_deco.n2(wx / 13.0, wz / 13.0) { B::SNOW } // sommets enneigés, partout
                    else if cold { B::SNOW }
                    else if yf > 88.0 + 5.0 * self.n_deco.n2(wx / 17.0, wz / 17.0) { self.stone_at(wx, yf, wz) } // haute montagne : la roche à nu
                    else if fr > 0.72 && self.n_deco.n2(wx / 13.0, wz / 13.0) > 0.25 - 0.9 * deep { B::FOREST_SOIL }
                    else if cliff > 0.45 && !cold && self.n_deco.n2(wx / 11.0, wz / 11.0) > 0.35 { B::MOSSY_STONE }
                    else if hash_float(wxi, wzi, self.seed, 41) < deep { B::GRASS_DEEP } else { B::GRASS };
                } else if steep { v[i] = self.stone_at(wx, y as f64, wz); }
                else { v[i] = B::DIRT; }
                depth += 1;
            }
            let wi = water as i32;
            if wi != 0 {
                for y in (top + 1)..=wi { v[idx(x, y as usize, z)] = B::WATER; }
                if cold && wi > top { v[idx(x, wi as usize, z)] = B::ICE; } // lac et rivière gelés en surface
                // La chute rejoint le bassin : une colonne d'eau pleine, plaquée contre le palier du dessus.
                if !c.still { let t = self.fall_top(wx, wz, water) as i32; for y in (wi + 1)..=t.min(CH as i32 - 2) { if v[idx(x, y as usize, z)] == 0 { v[idx(x, y as usize, z)] = if cold { B::ICE } else { B::WATER }; } } } // chute gelée
                // rochers qui affleurent dans le lit
                if !cold && self.n_deco.n2(wx / 4.3 + 31.0, wz / 4.3 - 12.0) > 0.56 {
                    let crown = wi + if hash_float(wxi, wzi, self.seed, 9) > 0.55 { 1 } else { 0 };
                    let mut y = top + 1;
                    while y <= crown && y < CH as i32 - 1 { v[idx(x, y as usize, z)] = if y == crown && !cold { B::MOSSY_STONE } else { B::STONE }; y += 1; }
                    top = crown;
                }
            }
            heights[x + z * CX] = top; waters[x + z * CX] = wi; info[x + z * CX] = (fr, path, c.mix.strong(Biome::Meadow), deep);
        } }
        self.stamp_all(&mut v, cx, cz, &mut heights);
        self.scatter_deco(&mut v, ox, oz, &heights, &waters, &info);
        self.hang_vines(&mut v, ox, oz);
        v
    }
    fn scatter_deco(&self, v: &mut [u8], ox: i32, oz: i32, heights: &[i32], waters: &[i32], info: &[(f64, f64, f64, f64)]) {
        let s = self.seed; let nd = &self.n_deco;
        for z in 0..CX { for x in 0..CX {
            let y = (heights[x + z * CX] + 1) as usize;
            if y >= CH - 1 { continue; }
            let (wxi, wzi) = (ox + x as i32, oz + z as i32); let (wx, wz) = (wxi as f64, wzi as f64);
            let ground = v[idx(x, y - 1, z)]; let here = v[idx(x, y, z)];
            let wy = waters[x + z * CX];
            if wy != 0 {
                if here == B::WATER && nd.n2(wx / 8.0, wz / 8.0) > 0.42 && hash_float(wxi, wzi, s, 5) < 0.45 && v[idx(x, wy as usize, z)] == B::WATER && v[idx(x, wy as usize + 1, z)] == 0 { v[idx(x, wy as usize + 1, z)] = B::LILY_PAD; }
                continue;
            }
            if here != 0 { continue; }
            let r = hash_float(wxi, wzi, s, 3); let (fr, path, meadow, deep) = info[x + z * CX];
            if ground == B::PATH { if r < 0.055 { v[idx(x, y, z)] = B::FLAGSTONE; } continue; }
            if !is_grass(ground) && ground != B::FOREST_SOIL && ground != B::MOSSY_STONE { continue; }
            if path > 0.02 && path < 0.4 && nd.n2(wx / 40.0 - 9.0, wz / 40.0 + 5.0) > 0.2 {
                v[idx(x, y, z)] = B::FENCE;
                if hash_float(wxi, wzi, s, 8) < 0.07 { v[idx(x, y + 1, z)] = B::FENCE; v[idx(x, y + 2, z)] = B::FENCE; v[idx(x, y + 3, z)] = B::LANTERN; }
                continue;
            }
            let patch = nd.n2(wx / 23.0, wz / 23.0); let patch2 = nd.n2(wx / 17.0 + 60.0, wz / 17.0 - 30.0);
            let open = 1.0 - fr; let mut id = 0u8;
            let flower_p = 0.14 + 0.62 * smooth(0.05 - 0.5 * meadow, 0.6, patch) * (0.65 + open * 0.5) * (1.0 - 0.85 * deep) + 0.1 * meadow; // la prairie fleurit partout, pas seulement en taches
            if r < flower_p {
                let t = hash_float(wxi, wzi, s, 4);
                id = if patch2 > 0.3 { if t < 0.55 { B::LUPINE } else { B::PINK_FLOWER } }
                    else if patch2 > -0.15 { if t < 0.75 { B::DAISY } else { B::YELLOW_FLOWER } }
                    else if t < 0.5 { B::YELLOW_FLOWER } else { B::DAISY };
                if hash_float(wxi, wzi, s, 6) > 0.55 { id = B::TALL_GRASS; }
            } else if r < flower_p + 0.1 + 0.24 * fr * smooth(-0.2, 0.3, patch2) + 0.4 * deep { id = B::FERN; }
            else if r < 0.985 { id = B::TALL_GRASS; }
            if fr > 0.6 && hash_float(wxi, wzi, s, 7) < 0.012 + 0.02 * deep { id = B::MUSHROOM; }
            if deep > 0.3 && hash_float(wxi, wzi, s, 13) < (0.025 + 0.06 * smooth(0.2, 0.6, nd.n2(wx / 11.0 - 40.0, wz / 11.0 + 7.0))) * deep { id = B::GLOWCAP; } // en ronds de sorcière plus denses
            if ground == B::MOSSY_STONE && r > 0.5 { id = 0; }
            if path > 0.05 { id = if r < 0.5 { B::TALL_GRASS } else { 0 }; }
            v[idx(x, y, z)] = id;
        } }
    }
    fn hang_vines(&self, v: &mut [u8], ox: i32, oz: i32) {
        let s = self.seed;
        for z in 0..CX { for x in 0..CX {
            let (wx, wz) = (ox + x as i32, oz + z as i32);
            let m = self.climate.at(wx as f64, wz as f64); let (deep, taiga) = (m.strong(Biome::DeepForest), m.strong(Biome::Taiga)); if hash_float(wx, wz, s, 11) > 0.075 + 0.2 * deep { continue; }
            let mut y = CH - 2;
            while y > 4 {
                let b = v[idx(x, y, z)];
                if is_leaf(b) && v[idx(x, y - 1, z)] == 0 {
                    if taiga > 0.5 { v[idx(x, y - 1, z)] = B::ICICLE; break; } // au froid, des glaçons à la place des lianes
                    let n = 2 + (hash_float(wx, wz, s, 12) * (5.0 + 5.0 * deep)).floor() as usize;
                    let mut k = 1;
                    while k <= n && v[idx(x, y - k, z)] == 0 { v[idx(x, y - k, z)] = B::VINE; k += 1; }
                    break;
                }
                y -= 1;
            }
        } }
    }
    fn apply(st: &Stamp, v: &mut [u8], ox: i32, oz: i32, heights: &mut [i32]) {
        for d in &st.data {
            let (x, y, z, id) = (d[0] - ox, d[1], d[2] - oz, d[3] as u8);
            if x < 0 || z < 0 || x >= CX as i32 || z >= CX as i32 || y < 0 || y >= CH as i32 { continue; }
            let k = idx(x as usize, y as usize, z as usize); let cur = v[k];
            if cur == 0 { v[k] = id; } else if id != 0 && (st.force || is_deco(cur)) { v[k] = id; }
            if st.force && is_solid(id) && y > heights[x as usize + z as usize * CX] { heights[x as usize + z as usize * CX] = y; }
        }
    }
    fn stamp_all(&self, v: &mut [u8], cx: i32, cz: i32, heights: &mut [i32]) {
        let (ox, oz) = (cx * CX as i32, cz * CX as i32);
        let (x0, z0, x1, z1) = (ox - STAMP_REACH, oz - STAMP_REACH, ox + CX as i32 + STAMP_REACH, oz + CX as i32 + STAMP_REACH);
        for gz in fdiv(z0, ROCK_CELL)..=fdiv(z1, ROCK_CELL) { for gx in fdiv(x0, ROCK_CELL)..=fdiv(x1, ROCK_CELL) {
            if let Some(st) = self.cached(0, gx, gz) { Self::apply(&st, v, ox, oz, heights); }
        } }
        for gz in fdiv(z0, TREE_CELL)..=fdiv(z1, TREE_CELL) { for gx in fdiv(x0, TREE_CELL)..=fdiv(x1, TREE_CELL) {
            if let Some(st) = self.cached(1, gx, gz) { Self::apply(&st, v, ox, oz, heights); }
        } }
    }
    fn cached(&self, kind: u8, gx: i32, gz: i32) -> Option<Rc<Stamp>> {
        if let Some(s) = self.stamps.borrow().get(&(kind, gx, gz)) { return s.clone(); }
        let st = if kind == 0 { self.rock_stamp(gx, gz) } else { self.tree_stamp(gx, gz) }.map(Rc::new);
        let mut m = self.stamps.borrow_mut();
        if m.len() > 6000 { m.clear(); }
        m.insert((kind, gx, gz), st.clone());
        st
    }
    fn rock_stamp(&self, gx: i32, gz: i32) -> Option<Stamp> {
        let s = self.seed;
        if hash_float(gx, gz, s, 21) > 0.62 { return None; }
        let x = ((gx as f64 + hash_float(gx, gz, s, 22)) * ROCK_CELL as f64).floor(); let z = ((gz as f64 + hash_float(gx, gz, s, 23)) * ROCK_CELL as f64).floor();
        let c = self.column(x, z);
        if c.water != 0.0 || c.path > 0.1 { return None; }
        let mut r = Rng::new(hash_int(gx, gz, s, 24)); let mut data = Vec::new();
        let n = r.int(1, 3); let base = c.h.floor(); let cold = c.mix.strong(Biome::Taiga) > 0.6;
        for k in 0..n {
            let rx = r.range(1.6, 4.2); let ry = r.range(1.3, 3.0); let rz = r.range(1.6, 4.2);
            let ax = x + if k != 0 { r.range(-4.0, 4.0) } else { 0.0 }; let az = z + if k != 0 { r.range(-4.0, 4.0) } else { 0.0 };
            let mut dy = -2.0;
            while dy <= ry + 1.0 {
                for dzi in -6..=6 { for dxi in -6..=6 {
                    let (dx, dz) = (dxi as f64, dzi as f64);
                    let nn = self.n_deco.n2((ax + dx) / 3.1, (az + dz + dy * 3.0) / 3.1) * 0.28;
                    let e = dx * dx / (rx * rx) + dy * dy / (ry * ry) + dz * dz / (rz * rz);
                    if e + nn < 1.0 {
                        let e2 = dx * dx / (rx * rx) + (dy + 1.0) * (dy + 1.0) / (ry * ry) + dz * dz / (rz * rz) + nn;
                        let id = if e2 >= 1.0 && r.next() < 0.75 { if cold { B::SNOW } else { B::MOSSY_STONE } } else { self.stone_at(ax, base + dy, az) };
                        data.push([ri(ax + dx), (base + dy) as i32, ri(az + dz), id as i32]);
                    }
                } }
                dy += 1.0;
            }
        }
        Some(Stamp { data, force: true })
    }
    fn tree_stamp(&self, gx: i32, gz: i32) -> Option<Stamp> {
        let s = self.seed; let tc = TREE_CELL as f64;
        let x = ((gx as f64 + 0.15 + 0.7 * hash_float(gx, gz, s, 31)) * tc).floor(); let z = ((gz as f64 + 0.15 + 0.7 * hash_float(gx, gz, s, 32)) * tc).floor();
        let c = self.column(x, z);
        if c.water != 0.0 || c.path > 0.0 || c.bank > 0.4 || c.h > 86.0 { return None; }
        let pick = hash_float(gx, gz, s, 33);
        let glade = smooth(-0.1, 0.45, self.n_forest.n2(x / 46.0 + 70.0, z / 46.0 - 40.0));
        if pick > (0.08 + 0.56 * smooth(0.25, 0.55, c.forest)) * (1.0 - 0.85 * glade) * (1.0 - 0.8 * c.mix.strong(Biome::Meadow)) * (1.0 + 0.8 * c.mix.strong(Biome::DeepForest)) { return None; }
        let mut r = Rng::new(hash_int(gx, gz, s, 34));
        let sp = self.n_forest.n2(gx as f64 / 5.1 + 3.0, gz as f64 / 5.1 - 8.0); let roll = r.next();
        let y = c.h.floor(); let age = r.next(); let deep = c.mix.strong(Biome::DeepForest); let taiga = c.mix.strong(Biome::Taiga);
        if taiga > 0.5 && roll > 0.1 * (1.0 - taiga) * 2.0 { return Some(grow_spruce(x, y, z, &mut r, age, taiga > 0.6)); } // taïga : des épicéas, quelques bouleaux
        // Forêt profonde : presque plus de bouleaux, des géants aux racines énormes, un feuillage sombre.
        if deep > 0.5 { let mut st = if roll < 0.28 { grow_beech(x, y, z, &mut r, (0.6 + 0.6 * age).min(1.2)) } else { grow_oak(x, y, z, &mut r, 0.62 + 0.75 * age) }; for d in &mut st.data { if d[3] == B::OAK_LEAVES as i32 { d[3] = B::OAK_LEAVES_DEEP as i32; } else if d[3] == B::BEECH_LEAVES as i32 { d[3] = B::BEECH_LEAVES_DEEP as i32; } } return Some(st); }
        Some(if roll < 0.34 + sp * 0.3 { grow_birch(x, y, z, &mut r, age) } else if roll < 0.52 { grow_beech(x, y, z, &mut r, age) } else { grow_oak(x, y, z, &mut r, age * age * age) })
    }
}

/* ---- Croissance des arbres : troncs, branches, couronne, racines. ---- */
type Key = (i32, i32, i32);
#[derive(Default)]
struct Grower { wood: BTreeMap<Key, u8>, leaf: BTreeMap<Key, u8>, extra: Vec<[i32; 4]> }
impl Grower {
    fn sphere_wood(&mut self, cx: f64, cy: f64, cz: f64, r: f64, id: u8) {
        let rr = (r + 1.0).ceil() as i32; let (rx, ry, rz) = (round(cx), round(cy), round(cz));
        for dy in -rr..=rr { for dz in -rr..=rr { for dx in -rr..=rr {
            let (xx, zz, y2) = (rx + dx as f64 - cx, rz + dz as f64 - cz, ry + dy as f64 - cy);
            if xx * xx + y2 * y2 / 1.0 + zz * zz <= r * r { self.wood.insert((rx as i32 + dx, ry as i32 + dy, rz as i32 + dz), id); }
        } } }
    }
    #[allow(clippy::too_many_arguments)]
    fn limb(&mut self, p: &[f64], dir: [f64; 3], len: f64, r0: f64, r1: f64, id: u8, rng: &mut Rng, curl: f64, up: f64) -> Vec<[f64; 6]> {
        let n = len.ceil() as i32; let mut pts = Vec::with_capacity(n as usize + 1);
        let (mut x, mut y, mut z) = (p[0], p[1], p[2]); let [mut dx, mut dy, mut dz] = dir;
        for i in 0..=n {
            let t = i as f64 / n as f64;
            let axis = if dy.abs() > hypot2(dx, dz) { 'y' } else if dx.abs() >= dz.abs() { 'x' } else { 'z' };
            self.sphere_wood(x, y, z, lerp(r0, r1, t), oriented_block(id, axis));
            pts.push([x, y, z, dx, dy, dz]);
            dx += rng.range(-curl, curl); dz += rng.range(-curl, curl); dy += rng.range(-curl * 0.5, curl * 0.5) + up;
            let l = hypot3(dx, dy, dz); dx /= l; dy /= l; dz /= l;
            x += dx; y += dy; z += dz;
        }
        pts
    }
    fn blob(&mut self, cx: f64, cy: f64, cz: f64, rx: f64, ry: f64, id: u8, rng: &mut Rng) {
        let xr = (rx + 1.0).ceil() as i32; let yr = (ry + 1.0).ceil() as i32; let seed = rng.next() * 100.0;
        let (qx, qy, qz) = (round(cx) as i32, round(cy) as i32, round(cz) as i32);
        for dyi in -yr..=yr { for dzi in -xr..=xr { for dxi in -xr..=xr {
            let (dx, dy, dz) = (dxi as f64, dyi as f64, dzi as f64);
            let e = (dx * dx + dz * dz) / (rx * rx) + dy * dy / (ry * ry);
            if e > 1.15 { continue; }
            let wob = sin(dx * 1.3 + seed) * sin(dy * 1.7 + seed * 2.0) * cos(dz * 1.1 - seed) * 0.5;
            if e + wob < 0.95 { self.leaf.insert((qx + dxi, qy + dyi, qz + dzi), id); }
        } } }
    }
    fn finish(self) -> Stamp {
        let mut data = Vec::with_capacity(self.leaf.len() + self.wood.len() + self.extra.len());
        for (k, id) in &self.leaf { if !self.wood.contains_key(k) { data.push([k.0, k.1, k.2, *id as i32]); } }
        for (k, id) in &self.wood { data.push([k.0, k.1, k.2, *id as i32]); }
        data.extend_from_slice(&self.extra);
        Stamp { data, force: false }
    }
}
#[inline] fn at_frac(n: usize, t: f64) -> usize { (n as f64 * t).floor() as usize }

fn grow_oak(x: f64, y: f64, z: f64, r: &mut Rng, age: f64) -> Stamp {
    let mut g = Grower::default(); let (log, lf) = (B::OAK_LOG, B::OAK_LEAVES);
    let rad = lerp(0.75, 2.3, age); let hh = lerp(5.5, 14.0, age);
    let lean = [r.range(-0.1, 0.1), 1.0, r.range(-0.1, 0.1)];
    let trunk = g.limb(&[x, y - 1.0, z], lean, hh, rad, rad * 0.72, log, r, 0.05, 0.03);
    let roots = 3 + (age * 3.0).floor() as i32;
    for i in 0..roots {
        let a = (i as f64 / roots as f64) * 6.283 + r.range(-0.3, 0.3);
        g.limb(&[x + cos(a) * rad * 0.6, y + rad * 0.9, z + sin(a) * rad * 0.6], [cos(a), -0.45, sin(a)], rad * 2.0 + 2.5, rad * 0.42, 0.42, log, r, 0.14, -0.01);
    }
    let top = trunk[trunk.len() - 1]; let arms = 3 + (age * 3.0).floor() as i32;
    for i in 0..arms {
        let a = (i as f64 / arms as f64) * 6.283 + r.range(-0.4, 0.4); let t = r.range(0.68, 0.98); let at = trunk[at_frac(trunk.len(), t)];
        let ll = lerp(2.5, 8.5, age) * r.range(0.8, 1.15);
        let d = [cos(a), r.range(0.3, 0.7), sin(a)];
        let pts = g.limb(&at, d, ll, rad * 0.55, 0.65, log, r, 0.14, 0.03);
        let tip = pts[pts.len() - 1];
        let rx = lerp(2.3, 3.9, age) * r.range(0.85, 1.15);
        g.blob(tip[0], tip[1] + 1.5, tip[2], rx, lerp(1.9, 3.0, age), lf, r);
        let mut b = 0.0;
        while b < 2.0 + age * 2.0 {
            let t = r.range(0.45, 0.95); let q = pts[at_frac(pts.len(), t)];
            let a2 = a + r.range(-1.2, 1.2);
            let d2 = [cos(a2), r.range(0.0, 0.5), sin(a2)];
            let pts2 = g.limb(&q, d2, ll * 0.5, 0.75, 0.4, log, r, 0.2, 0.04);
            let t2 = pts2[pts2.len() - 1];
            let rx2 = lerp(1.9, 3.1, age) * r.range(0.8, 1.1);
            g.blob(t2[0], t2[1] + 1.0, t2[2], rx2, lerp(1.7, 2.5, age), lf, r);
            b += 1.0;
        }
    }
    g.blob(top[0], top[1] + 3.0, top[2], lerp(2.8, 4.8, age), lerp(2.3, 3.4, age), lf, r);
    // Polypores : du cœur du tronc jusqu'à la première case libre, donc toujours collés à l'écorce.
    let fungi = (age * 5.0).floor() as i32 + if r.next() < 0.5 { 1 } else { 0 };
    const DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for _ in 0..fungi {
        let t = r.range(0.08, 0.5); let q = trunk[at_frac(trunk.len(), t)]; let (dx, dz) = DIR[r.int(0, 3) as usize]; let yy = ri(q[1]);
        let (mut cx, mut cz, mut steps) = (ri(q[0]), ri(q[2]), 0);
        if !g.wood.contains_key(&(cx, yy, cz)) { continue; }
        while g.wood.contains_key(&(cx, yy, cz)) && steps < 12 { steps += 1; cx += dx; cz += dz; }
        g.extra.push([cx, yy, cz, B::SHELF_FUNGUS as i32]);
    }
    g.finish()
}
/// Épicéa : un tronc droit et des étages de branches qui s'élargissent vers le bas, enneigés sur le dessus.
fn grow_spruce(x: f64, y: f64, z: f64, r: &mut Rng, age: f64, snowy: bool) -> Stamp {
    let mut g = Grower::default(); let hh = lerp(7.0, 18.0, age); let rad = lerp(0.45, 1.0, age);
    let trunk = g.limb(&[x, y - 1.0, z], [r.range(-0.03, 0.03), 1.0, r.range(-0.03, 0.03)], hh, rad, 0.4, B::SPRUCE_LOG, r, 0.02, 0.0);
    let top = trunk[trunk.len() - 1]; let base = y + hh * 0.22; let ytop = top[1] + 2.0; let wide = lerp(2.4, 5.2, age); let tier = 2.0 + (age * 2.0).floor();
    let mut ly = base.floor();
    while ly <= ytop {
        let t = ((ytop - ly) / (ytop - base)).clamp(0.0, 1.0); let fr = ((ly - base) % tier) / tier; let rr = (0.55 + t * wide) * (1.0 - 0.38 * fr);
        let q = trunk[((ly - y + 1.0).max(0.0) as usize).min(trunk.len() - 1)]; let (cx, cz) = (round(q[0]) as i32, round(q[2]) as i32); let n = rr.ceil() as i32;
        for dz in -n..=n { for dx in -n..=n { let d = hypot2(dx as f64, dz as f64) + 0.5 * r.next(); if d <= rr { g.leaf.insert((cx + dx, ly as i32, cz + dz), B::SPRUCE_LEAVES); } } }
        ly += 1.0;
    }
    let (tx, tz) = (round(top[0]) as i32, round(top[2]) as i32); for k in 0..2 { g.leaf.insert((tx, ytop as i32 + 1 + k, tz), B::SPRUCE_LEAVES); }
    if snowy { let keys: Vec<Key> = g.leaf.keys().copied().collect(); for k in keys { let up = (k.0, k.1 + 1, k.2); if !g.leaf.contains_key(&up) && !g.wood.contains_key(&up) { g.leaf.insert(k, B::SPRUCE_LEAVES_SNOW); } } }
    g.finish()
}
fn grow_birch(x: f64, y: f64, z: f64, r: &mut Rng, age: f64) -> Stamp {
    let mut g = Grower::default(); let (log, lf) = (B::BIRCH_LOG, B::BIRCH_LEAVES);
    let hh = lerp(6.0, 15.0, age); let rad = lerp(0.45, 1.0, age);
    let d = [r.range(-0.06, 0.06), 1.0, r.range(-0.06, 0.06)];
    let trunk = g.limb(&[x, y - 1.0, z], d, hh, rad, 0.42, log, r, 0.05, 0.02);
    let n = 5 + (age * 5.0).floor() as i32;
    for _ in 0..n {
        let t = r.range(0.48, 0.98); let p = trunk[at_frac(trunk.len(), t)]; let a = r.range(0.0, 6.283);
        let ll = lerp(1.5, 4.5, 1.0 - t * 0.7) * lerp(0.7, 1.1, age);
        let pts = g.limb(&p, [cos(a), 0.55, sin(a)], ll, 0.45, 0.4, log, r, 0.15, 0.02);
        let tip = pts[pts.len() - 1];
        let (rx, ry) = (r.range(1.6, 2.6), r.range(1.5, 2.3));
        g.blob(tip[0], tip[1] + 0.5, tip[2], rx, ry, lf, r);
    }
    let top = trunk[trunk.len() - 1];
    g.blob(top[0], top[1] + 1.0, top[2], 2.2, 2.6, lf, r);
    g.finish()
}
fn grow_beech(x: f64, y: f64, z: f64, r: &mut Rng, age: f64) -> Stamp {
    let mut g = Grower::default(); let (log, lf) = (B::BEECH_LOG, B::BEECH_LEAVES);
    let hh = lerp(6.5, 16.0, age); let rad = lerp(0.6, 1.5, age);
    let d = [r.range(-0.05, 0.05), 1.0, r.range(-0.05, 0.05)];
    let trunk = g.limb(&[x, y - 1.0, z], d, hh, rad, 0.7, log, r, 0.04, 0.02);
    let n = 5 + (age * 4.0).floor() as i32;
    for i in 0..n {
        let t = r.range(0.55, 1.0); let p = trunk[(trunk.len() - 1).min(at_frac(trunk.len(), t))]; let a = (i as f64 / n as f64) * 6.283 + r.range(-0.5, 0.5);
        let ll = lerp(2.2, 6.5, age) * r.range(0.7, 1.1);
        let pts = g.limb(&p, [cos(a), 0.65, sin(a)], ll, rad * 0.45, 0.45, log, r, 0.1, 0.04);
        let tip = pts[pts.len() - 1];
        g.blob(tip[0], tip[1] + 1.0, tip[2], lerp(2.4, 3.8, age), lerp(2.0, 3.0, age), lf, r);
    }
    g.finish()
}
