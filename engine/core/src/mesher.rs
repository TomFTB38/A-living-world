//! Maillage : faces fusionnées, occlusion par sommet, lumière voxel (ciel et blocs), géométrie enrichie.
use crate::blocks::*;
use crate::det::{cos, hypot3, round, sin};
use crate::gen::WorldGen;
use crate::noise::{hash_int, Rng};

pub const VSIZE: usize = 20;
/// Couleur d'un champignon lumineux, fixée par sa position : le maillage et sa lumière colorée s'accordent.
pub const GLOW: [[f64; 3]; 5] = [[0.3, 0.85, 0.3], [0.18, 0.42, 0.95], [0.95, 0.32, 0.7], [0.62, 0.3, 0.95], [0.15, 0.75, 0.78]];
pub fn glow_color(x: i32, y: i32, z: i32) -> [f64; 3] { GLOW[(crate::noise::hash_int(x, y, z, 93) % 5) as usize] }
pub mod kind { pub const TEX: u8 = 0; pub const LEAF: u8 = 1; pub const FLAT: u8 = 2; pub const FOLIAGE: u8 = 3; pub const EMIT: u8 = 4; }
use kind::*;
const PAD: i32 = 8; const W: i32 = CX as i32 + PAD * 2; const WW: i32 = W * W; const CHI: i32 = CH as i32;
const AO: [f64; 4] = [0.42, 0.62, 0.82, 1.0];
pub const POSK: f64 = 128.0;

// Conversions des tampons binaires du prototype : troncature, puis repli sur la largeur du type.
#[inline] fn u8c(v: f64) -> u8 { v as i64 as u8 }
#[inline] fn i8c(v: f64) -> u8 { v as i64 as u8 }
#[inline] fn i16c(v: f64) -> i16 { v as i64 as i16 }

/// Sommets de 20 octets : position i16 x3 + couche, normale i8 x3 + vent, ao, ciel, bloc, type, teinte rgb, rotation.
pub struct VBuf { pub data: Vec<u8>, pub n: usize, pub scale: f64, pub rot: u8 }
impl VBuf {
    pub fn new(cap: usize) -> Self { VBuf { data: Vec::with_capacity(cap * VSIZE), n: 0, scale: POSK, rot: 0 } }
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn v(&mut self, x: f64, y: f64, z: f64, layer: f64, nx: f64, ny: f64, nz: f64, wind: f64, ao: f64, sky: f64, blk: f64, kind: u8, r: f64, g: f64, b: f64) {
        let s = self.scale; let c = |v: f64| if v < 2.0 { u8c(v * 127.5) } else { 255 };
        let p = [i16c(round(x * s)), i16c(round(y * s)), i16c(round(z * s)), i16c(layer)];
        let d = &mut self.data;
        for q in p { d.extend_from_slice(&q.to_le_bytes()); }
        d.extend_from_slice(&[i8c(nx * 127.0), i8c(ny * 127.0), i8c(nz * 127.0), ((wind as i32) >> 1) as u8, u8c(ao), u8c(sky), u8c(blk), kind, c(r), c(g), c(b), self.rot]);
        self.n += 1;
    }
}

#[derive(Default)]
pub struct Mesh { pub opaque: Vec<u8>, pub deco: Vec<u8>, pub water: Vec<u8>, pub n_o: usize, pub n_d: usize, pub n_w: usize, pub ymin: i32, pub ymax: i32 }

/// Espace de travail réutilisable : un par fil.
pub struct Mesher { vol: Vec<u8>, sky: Vec<u8>, blk: Vec<u8>, m1: Vec<i32>, m2: Vec<i32> }
#[inline(always)] fn at(x: i32, y: i32, z: i32) -> i32 { x + PAD + (z + PAD) * W + y * WW }

impl Default for Mesher { fn default() -> Self { Self::new() } }
impl Mesher {
    pub fn new() -> Self { let n = (WW * CHI) as usize; Mesher { vol: vec![0; n], sky: vec![0; n], blk: vec![0; n], m1: vec![0; CX * CH], m2: vec![0; CX * CH] } }
    #[inline(always)] fn vk(&self, k: i32) -> u8 { *self.vol.get(k as usize).unwrap_or(&0) }

    fn fill_volume(&mut self, get: &mut dyn FnMut(i32, i32) -> std::sync::Arc<Vec<u8>>, cx: i32, cz: i32) -> (i32, Vec<i32>) {
        let (cxi, pad) = (CX as i32, PAD);
        for dz in -1..=1 { for dx in -1..=1 {
            let c = get(cx + dx, cz + dz);
            let (x0, x1) = (if dx < 0 { cxi - pad } else { 0 }, if dx > 0 { pad } else { cxi });
            let (z0, z1) = (if dz < 0 { cxi - pad } else { 0 }, if dz > 0 { pad } else { cxi });
            for y in 0..CHI { for z in z0..z1 {
                let zz = z + dz * cxi + pad; let s = (x0 + z * cxi + y * cxi * cxi) as usize; let d = (x0 + dx * cxi + pad + zz * W + y * WW) as usize; let n = (x1 - x0) as usize;
                self.vol[d..d + n].copy_from_slice(&c[s..s + n]);
            } }
        } }
        let mut top = vec![0i32; WW as usize]; let mut ymax = 0;
        for i in 0..WW {
            let mut y = CHI - 1;
            while y > 0 && self.vol[(i + y * WW) as usize] == 0 { y -= 1; }
            top[i as usize] = y; if y > ymax { ymax = y; }
        }
        (ymax, top)
    }
    fn compute_light(&mut self, top: &[i32], ymax: i32) {
        self.sky.fill(0); self.blk.fill(0);
        let mut ylow = CHI;
        for i in 0..WW {
            let mut s = 15u8; let t = top[i as usize]; let mut y = CHI - 1;
            while y >= 0 {
                let k = (i + y * WW) as usize;
                if y > t { self.sky[k] = 15; y -= 1; continue; }
                let b = self.vol[k];
                if is_opaque(b) { if y < ylow { ylow = y; } break; }
                if is_leaf(b) { s = if s > 1 { s - 1 } else { 0 }; } else if is_water(b) { s = s.saturating_sub(1); }
                self.sky[k] = s;
                y -= 1;
            }
        }
        // Une lanterne éclaire où qu'elle soit, même sous un toit ou dans une grotte : la lumière du ciel s'arrête au premier bloc, pas la sienne.
        for y in 0..=ymax.min(CHI - 1) { for i in 0..WW { let k = (i + y * WW) as usize; let e = emit(self.vol[k]); if e != 0 { self.blk[k] = e; if y - 15 < ylow { ylow = y - 15; } } } }
        let ylow = 1.max(ylow - 2); let yhi = (CHI - 2).min(ymax + 1);
        let vol = &self.vol;
        let relax = |l: &mut Vec<u8>| {
            let g = |l: &[u8], k: i32| l[k as usize] as i32 - 1;
            let mut y = yhi;
            while y >= ylow {
                for z in 1..W { for x in 1..W {
                    let k = x + z * W + y * WW;
                    if is_opaque(vol[k as usize]) { continue; }
                    let v = (l[k as usize] as i32).max(g(l, k - 1)).max(g(l, k - W)).max(g(l, k + WW));
                    l[k as usize] = v as u8;
                } }
                y -= 1;
            }
            for y in ylow..=yhi { for z in (0..=W - 2).rev() { for x in (0..=W - 2).rev() {
                let k = x + z * W + y * WW;
                if is_opaque(vol[k as usize]) { continue; }
                let v = (l[k as usize] as i32).max(g(l, k + 1)).max(g(l, k + W)).max(g(l, k - WW));
                l[k as usize] = v as u8;
            } } }
        };
        relax(&mut self.sky); relax(&mut self.blk);
    }
    fn greedy(&mut self, out: &mut VBuf, ymin: i32, ymax: i32, lod: u8) {
        let solid_leaf = lod == 2;
        let tall = |b: u8| is_opaque(b) || (solid_leaf && is_leaf(b));
        for d in 0..3usize {
            let (u, v) = ((d + 1) % 3, (d + 2) % 3);
            let dims = [CX as i32, ymax - ymin + 1, CX as i32]; let off = [0, ymin, 0];
            for sign in [-1i32, 1] {
                let mut nrm = [0i32; 3]; nrm[d] = sign;
                let n_w = nrm[0] + nrm[2] * W + nrm[1] * WW;
                let u_w = match u { 0 => 1, 1 => WW, _ => W }; let v_w = match v { 0 => 1, 1 => WW, _ => W };
                for s in 0..dims[d] {
                    let mut any = false; let (nu, nv) = (dims[u], dims[v]);
                    self.m1[..(nu * nv) as usize].fill(-1);
                    let mut q = [0i32; 3];
                    for j in 0..nv { for i in 0..nu {
                        q[d] = s + off[d]; q[u] = i + off[u]; q[v] = j + off[v];
                        let k = at(q[0], q[1], q[2]); let b = self.vol[k as usize];
                        if !tall(b) { continue; }
                        let f = k + n_w; let ny = q[1] + nrm[1];
                        if ny < 0 { continue; }
                        let nb = if ny >= CHI { 0 } else { self.vk(f) };
                        if tall(nb) { continue; }
                        let di = if d == 0 { 0 } else if d == 2 { 3 } else if sign > 0 { 1 } else { 2 };
                        let (face, grain) = (BLOCKS[b as usize].dir_faces[di] as i32, BLOCKS[b as usize].grain[di] as i32);
                        let (mut ao, mut skyn, mut blkn) = (0i32, 0i32, 0i32);
                        for c in 0..4 {
                            let su = if c == 1 || c == 2 { 1 } else { -1 }; let sv = if c >= 2 { 1 } else { -1 };
                            let (a, e) = (f + su * u_w, f + sv * v_w); let g = a + sv * v_w;
                            let (va, ve, vg) = (self.vk(a), self.vk(e), self.vk(g));
                            let (s1, s2, s3) = (is_solid(va) as i32, is_solid(ve) as i32, is_solid(vg) as i32);
                            ao |= (if s1 != 0 && s2 != 0 { 0 } else { 3 - s1 - s2 - s3 }) << (c * 2);
                            let lk = |t: &[u8], i: i32| *t.get(i as usize).unwrap_or(&0) as i32;
                            let (mut n, mut ss, mut bb) = (1, lk(&self.sky, f), lk(&self.blk, f));
                            if !is_opaque(va) { ss += lk(&self.sky, a); bb += lk(&self.blk, a); n += 1; }
                            if !is_opaque(ve) { ss += lk(&self.sky, e); bb += lk(&self.blk, e); n += 1; }
                            if !is_opaque(vg) { ss += lk(&self.sky, g); bb += lk(&self.blk, g); n += 1; }
                            skyn |= (round(ss as f64 / n as f64) as i32) << (c * 4); blkn |= (round(bb as f64 / n as f64) as i32) << (c * 4);
                        }
                        let pos = (i + j * nu) as usize;
                        let tn = if di != 2 && snow_level(self.vk(k + WW)) > 0 { 2 } else { tone(b) }; // neige posée dessus : le bloc en reçoit un peu
                        self.m1[pos] = (face << 8) | ao | (if is_leaf(b) { 1 << 24 } else { 0 }) | (grain << 25) | (tn << 28);
                        self.m2[pos] = skyn | (blkn << 16); any = true;
                    } }
                    if !any { continue; }
                    for j in 0..nv {
                        let mut i = 0;
                        while i < nu {
                            let c1 = self.m1[(i + j * nu) as usize];
                            if c1 == -1 { i += 1; continue; }
                            let c2 = self.m2[(i + j * nu) as usize];
                            let mut w = 1;
                            while i + w < nu && self.m1[(i + w + j * nu) as usize] == c1 && self.m2[(i + w + j * nu) as usize] == c2 { w += 1; }
                            let mut h = 1;
                            'o: while j + h < nv { for t in 0..w { let p = (i + t + (j + h) * nu) as usize; if self.m1[p] != c1 || self.m2[p] != c2 { break 'o; } } h += 1; }
                            for b in 0..h { for t in 0..w { self.m1[(i + t + (j + b) * nu) as usize] = -1; } }
                            q[d] = s + off[d] + if sign > 0 { 1 } else { 0 }; q[u] = i + off[u]; q[v] = j + off[v];
                            let (mut du, mut dv) = ([0i32; 3], [0i32; 3]); du[u] = w; dv[v] = h;
                            let px = [q[0], q[0] + du[0], q[0] + du[0] + dv[0], q[0] + dv[0]];
                            let py = [q[1], q[1] + du[1], q[1] + du[1] + dv[1], q[1] + dv[1]];
                            let pz = [q[2], q[2] + du[2], q[2] + du[2] + dv[2], q[2] + dv[2]];
                            out.rot = ((c1 >> 25) & 7) as u8; let tn = TONES[((c1 >> 28) & 7) as usize];
                            let (a0, a1, a2, a3) = (c1 & 3, (c1 >> 2) & 3, (c1 >> 4) & 3, (c1 >> 6) & 3);
                            let flip = if a0 + a2 < a1 + a3 { 1 } else { 0 }; let layer = ((c1 >> 8) & 0xffff) as f64; let kd = if c1 & (1 << 24) != 0 { LEAF } else { TEX };
                            for c in 0..4 {
                                let o = (if sign > 0 { (c + flip) & 3 } else { (4 - c + flip) & 3 }) as usize; let os = o as i32;
                                out.v(px[o] as f64, py[o] as f64, pz[o] as f64, layer, nrm[0] as f64, nrm[1] as f64, nrm[2] as f64, 0.0, AO[((c1 >> (os * 2)) & 3) as usize] * 255.0, (((c2 >> (os * 4)) & 15) * 17) as f64, (((c2 >> (16 + os * 4)) & 15) * 17) as f64, kd, tn[0], tn[1], tn[2]);
                            }
                            out.rot = 0;
                            i += w;
                        }
                    }
                }
            }
        }
    }

    /// lod 0 : proche, tout le décor ; 1 : moyen, décor simplifié ; 2 : loin, blocs seuls.
    pub fn mesh_chunk(&mut self, get: &mut dyn FnMut(i32, i32) -> std::sync::Arc<Vec<u8>>, cx: i32, cz: i32, lod: u8) -> Mesh {
        let (ymax, top) = self.fill_volume(get, cx, cz);
        self.compute_light(&top, ymax);
        let mut ymin = 1;
        'scan: while ymin < ymax { for z in -1..=CX as i32 { for x in -1..=CX as i32 { if !is_opaque(self.vol[at(x, ymin, z) as usize]) { break 'scan; } } } ymin += 1; }
        ymin = 1.max(ymin - 1);
        let mut opaque = VBuf::new(40000); let mut deco = VBuf::new(if lod == 2 { 16 } else { 60000 }); let mut water = VBuf::new(4000);
        self.greedy(&mut opaque, ymin, ymax.min(CHI - 1), lod);
        for y in ymin..=ymax { for z in 0..CX as i32 { for x in 0..CX as i32 {
            let k = at(x, y, z); let b = self.vol[k as usize];
            if b == 0 || is_opaque(b) { continue; }
            let (fx, fy, fz) = (x as f64, y as f64, z as f64);
            if is_water(b) {
                let up = self.vk(k + WW); let covered = is_water(up); let top_y = if covered { fy + 1.0 } else { fy + 0.875 * water_level(b) as f64 / 8.0 };
                let s = (self.sky[(k + WW) as usize] as i32 * 17) as f64; let l = (self.blk[k as usize] as i32 * 17) as f64;
                // Hauteur d'une colonne voisine, et de chaque coin : l'eau qui coule forme une pente continue, pas des marches.
                let flowing = (-1..=1).any(|dz| (-1..=1).any(|dx| { let v = self.vk(k + dx + dz * W); is_water(v) && v != B::WATER }));
                let height = |n: i32| -> Option<f64> { let v = self.vk(n); if is_water(v) { Some(if is_water(self.vk(n + WW)) { 1.0 } else { 0.875 * water_level(v) as f64 / 8.0 }) } else if is_opaque(v) { None } else { Some(0.0) } };
                let corner = |cx: i32, cz: i32| -> f64 {
                    if !flowing || covered { return top_y; }
                    let (mut sum, mut n) = (top_y - fy, 1.0); for q in [k + cx, k + cz * W, k + cx + cz * W] { if let Some(h) = height(q) { sum += h; n += 1.0; } } fy + (sum / n).max(0.04)
                };
                let (c00, c10, c11, c01) = (corner(-1, -1), corner(1, -1), corner(1, 1), corner(-1, 1));
                if !covered && !is_opaque(up) {
                    // écume au pied des chutes
                    let mut foam = 0.0;
                    'f: for dz in -2..=2 { for dx in -2..=2 { if is_water(self.vk(k + dx + dz * W + WW * 2)) { foam = 1.6; break 'f; } } }
                    for (a, e, c) in [(fx, c01, fz + 1.0), (fx + 1.0, c11, fz + 1.0), (fx + 1.0, c10, fz), (fx, c00, fz)] { water.v(a, e, c, 0.0, 0.0, 1.0, 0.0, 0.0, 255.0, s, l, 0, foam, 1.0, 1.0); }
                }
                let sides = [(k + 1, 1.0, 0.0, fx + 1.0, fz + 1.0, fx + 1.0, fz, c11, c10), (k - 1, -1.0, 0.0, fx, fz, fx, fz + 1.0, c00, c01), (k + W, 0.0, 1.0, fx, fz + 1.0, fx + 1.0, fz + 1.0, c01, c11), (k - W, 0.0, -1.0, fx + 1.0, fz, fx, fz, c10, c00)];
                for (n, nx, nz, ax, az, bx, bz, ca, cb) in sides {
                    let v = self.vk(n); if is_opaque(v) { continue; }
                    let mut low = fy;
                    if is_water(v) { if !flowing { continue; } low = fy + height(n).unwrap_or(0.0); if low >= ca.max(cb) - 0.01 { continue; } }
                    let ss = (self.sky[n as usize].max(self.sky[k as usize]) as i32 * 17) as f64;
                    for (a, e, c) in [(ax, low, az), (bx, low, bz), (bx, cb.max(low), bz), (ax, ca.max(low), az)] { water.v(a, e, c, 0.0, nx, 0.0, nz, 0.0, 255.0, ss, l, 0, 1.0, 1.0, 1.0); }
                }
            } else if is_leaf(b) && lod < 2 {
                let exposed = [k + 1, k - 1, k + W, k - W, k + WW, k - WW].iter().any(|n| !is_solid(self.vk(*n)));
                if exposed { leaf_cubes(&mut opaque, b, fx, fy, fz, cx * CX as i32 + x, y, cz * CX as i32 + z, self.sky[k as usize] as f64 * 17.0, self.blk[k as usize] as f64 * 17.0, lod); }
            } else if is_deco(b) && (lod < 2 || emit(b) != 0 || b == B::FENCE) {
                let l = self.blk[k as usize].max(emit(b)) as f64 * 17.0;
                self.deco_model(&mut deco, b, x, y, z, cx * CX as i32 + x, cz * CX as i32 + z, self.sky[k as usize] as f64 * 17.0, l, lod);
            }
        } } }
        Mesh { n_o: opaque.n, n_d: deco.n, n_w: water.n, opaque: opaque.data, deco: deco.data, water: water.data, ymin, ymax: ymax + 2 }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn deco_model(&self, o: &mut VBuf, b: u8, xi: i32, yi: i32, zi: i32, wx: i32, wz: i32, s: f64, l: f64, lod: u8) {
        let (x, y, z) = (xi as f64, yi as f64, zi as f64);
        let mut r = Rng::new(hash_int(wx, yi, wz, 91)); let m = BLOCKS[b as usize].model.unwrap_or("");
        let r_ = &mut r; let picked = BLOCKS[b as usize].name.ends_with("_picked"); // buisson cueilli : même forme, sans les baies
        let voxel = |dx: i32, dy: i32, dz: i32| self.vk(at(xi + dx, yi + dy, zi + dz));
        match m {
            "lantern" => {
                bx3(o, x + 0.32, y, z + 0.32, x + 0.68, y + 0.08, z + 0.68, 0.16, 0.1, 0.06, 0.0, s, l);
                bx(o, x + 0.36, y + 0.08, z + 0.36, x + 0.64, y + 0.54, z + 0.64, 2.0, 1.3, 0.5, 0.0, s, 255.0, EMIT, false, 0.0, 255.0);
                bx3(o, x + 0.32, y + 0.54, z + 0.32, x + 0.68, y + 0.62, z + 0.68, 0.16, 0.1, 0.06, 0.0, s, l);
                return;
            }
            "vine" => {
                let n = r_.int(1, 2);
                for _ in 0..n {
                    let a = x + r_.range(0.2, 0.8); let c = z + r_.range(0.2, 0.8); let t = r_.range(0.8, 1.1);
                    bx(o, a - 0.02, y, c - 0.02, a + 0.02, y + 1.0, c + 0.02, 0.26 * t, 0.42 * t, 0.15 * t, 70.0, s, l, FOLIAGE, false, 90.0, 255.0);
                    if lod == 0 { for k in 0..3 {
                        let yy = y + 0.14 + k as f64 * 0.3; let q = r_.range(0.03, 0.055); let dx = r_.range(-0.09, 0.09); let dz = r_.range(-0.09, 0.09);
                        bx(o, a + dx - q, yy, c + dz - q, a + dx + q, yy + 0.07, c + dz + q, 0.3 * t, 0.5 * t, 0.17 * t, 80.0, s, l, FOLIAGE, true, 80.0, 255.0);
                    } }
                }
                return;
            }
            "fungus" => {
                let (dx, dz) = if is_opaque(voxel(1, 0, 0)) { (1.0, 0.0) } else if is_opaque(voxel(-1, 0, 0)) { (-1.0, 0.0) } else if is_opaque(voxel(0, 0, 1)) { (0.0, 1.0) } else if is_opaque(voxel(0, 0, -1)) { (0.0, -1.0) } else { return };
                let n = r_.int(2, 3);
                for i in 0..n {
                    let w = r_.range(0.2, 0.34); let yy = y + 0.2 + i as f64 * 0.26;
                    let a = x + 0.5 + dx * (0.5 - w * 0.75) + if dz != 0.0 { r_.range(-0.25, 0.25) } else { 0.0 };
                    let c = z + 0.5 + dz * (0.5 - w * 0.75) + if dx != 0.0 { r_.range(-0.25, 0.25) } else { 0.0 };
                    let t = r_.range(0.9, 1.1);
                    bx(o, a - w, yy, c - w, a + w, yy + 0.07, c + w, 0.82 * t, 0.68 * t, 0.5 * t, 0.0, s, l, FLAT, true, 0.0, 255.0);
                    bx3(o, a - w * 0.72, yy + 0.07, c - w * 0.72, a + w * 0.72, yy + 0.12, c + w * 0.72, 0.9 * t, 0.77 * t, 0.6 * t, 0.0, s, l);
                }
                return;
            }
            "campfire" => {
                // Feu de camp : un cercle de pierres, des bûches croisées, des flammes qui éclairent d'elles-mêmes.
                for k in 0..8 { let a = k as f64 * 0.785 + 0.2; let (cx, cz) = (x + 0.5 + cos(a) * 0.36, z + 0.5 + sin(a) * 0.36); let t = 0.85 + 0.25 * r_.next(); let q = 0.075 + 0.025 * r_.next(); bx3(o, cx - q, y, cz - q, cx + q, y + 0.11 + 0.04 * r_.next(), cz + q, 0.46 * t, 0.45 * t, 0.43 * t, 0.0, s, l); }
                bx3(o, x + 0.2, y, z + 0.44, x + 0.8, y + 0.12, z + 0.56, 0.36, 0.22, 0.12, 0.0, s, l); bx3(o, x + 0.44, y + 0.08, z + 0.2, x + 0.56, y + 0.2, z + 0.8, 0.3, 0.18, 0.1, 0.0, s, l);
                bx3(o, x + 0.38, y + 0.02, z + 0.38, x + 0.62, y + 0.06, z + 0.62, 0.1, 0.07, 0.05, 0.0, s, l); // braises
                if b != B::CAMPFIRE { return; } // feu éteint : cendres froides, sans flammes
                for (dx, dz, h, w, c) in [(0.0, 0.0, 0.52, 0.11, [2.4, 1.2, 0.3]), (0.09, 0.05, 0.36, 0.08, [2.2, 0.9, 0.2]), (-0.08, -0.06, 0.4, 0.08, [2.3, 1.0, 0.25]), (0.03, -0.1, 0.28, 0.06, [2.6, 1.6, 0.5])] {
                    let (cx, cz) = (x + 0.5 + dx, z + 0.5 + dz); bx(o, cx - w, y + 0.1, cz - w, cx + w, y + 0.1 + h, cz + w, c[0], c[1], c[2], 0.0, s, 255.0, EMIT, false, 0.0, 255.0);
                }
                return;
            }
            "furnace" => {
                // Four : un massif de pierres sèches, une gueule rougeoyante, une cheminée courte.
                let st = |o: &mut VBuf, a: f64, e: f64, c: f64, a1: f64, e1: f64, c1: f64, k: f64| bx3(o, x + a, y + e, z + c, x + a1, y + e1, z + c1, 0.5 * k, 0.48 * k, 0.45 * k, 0.0, s, l);
                for (i, (e, e1)) in [(0.0, 0.3), (0.3, 0.6), (0.6, 0.86)].iter().enumerate() { let k = 0.85 + 0.12 * (i as f64 % 2.0); st(o, 0.04, *e, 0.04, 0.96, *e1, 0.96, k); }
                st(o, 0.3, 0.86, 0.3, 0.7, 1.0, 0.7, 0.75); bx3(o, x + 0.36, y + 0.98, z + 0.36, x + 0.64, y + 1.0, z + 0.64, 0.08, 0.07, 0.06, 0.0, s, l);
                bx(o, x + 0.28, y + 0.08, z + 0.955, x + 0.72, y + 0.42, z + 0.97, 2.2, 0.8, 0.25, 0.0, s, 255.0, EMIT, false, 0.0, 255.0); // gueule
                return;
            }
            "anvil" => {
                // Enclume posée sur un billot : pied de bois, table d'acier, bigorne.
                bx3(o, x + 0.22, y, z + 0.22, x + 0.78, y + 0.45, z + 0.78, 0.42, 0.3, 0.18, 0.0, s, l); bx3(o, x + 0.2, y + 0.43, z + 0.2, x + 0.8, y + 0.47, z + 0.8, 0.5, 0.36, 0.22, 0.0, s, l);
                let m = |o: &mut VBuf, a: f64, e: f64, c: f64, a1: f64, e1: f64, c1: f64, k: f64| bx3(o, x + a, y + e, z + c, x + a1, y + e1, z + c1, 0.3 * k, 0.31 * k, 0.33 * k, 0.0, s, l);
                m(o, 0.36, 0.47, 0.32, 0.64, 0.62, 0.68, 0.9); m(o, 0.26, 0.62, 0.22, 0.74, 0.78, 0.78, 1.1); m(o, 0.36, 0.64, 0.78, 0.64, 0.75, 0.92, 1.0); m(o, 0.42, 0.66, 0.92, 0.58, 0.72, 0.99, 1.15);
                return;
            }
            "mason_bench" => {
                // Banc de taille : un établi bas et massif, un bloc dégrossi, un ciseau et une massette.
                let wood = |o: &mut VBuf, a: f64, e: f64, c: f64, a1: f64, e1: f64, c1: f64, k: f64| bx3(o, x + a, y + e, z + c, x + a1, y + e1, z + c1, 0.5 * k, 0.36 * k, 0.22 * k, 0.0, s, l);
                wood(o, 0.02, 0.6, 0.02, 0.98, 0.74, 0.98, 1.0); for (a, c) in [(0.05, 0.05), (0.79, 0.05), (0.05, 0.79), (0.79, 0.79)] { wood(o, a, 0.0, c, a + 0.16, 0.6, c + 0.16, 0.8); }
                bx3(o, x + 0.18, y + 0.74, z + 0.22, x + 0.6, y + 1.0, z + 0.62, 0.62, 0.6, 0.55, 0.0, s, l); bx3(o, x + 0.6, y + 0.74, z + 0.3, x + 0.68, y + 0.92, z + 0.5, 0.7, 0.68, 0.62, 0.0, s, l);
                bx3(o, x + 0.68, y + 0.74, z + 0.66, x + 0.9, y + 0.78, z + 0.7, 0.6, 0.6, 0.62, 0.0, s, l); bx3(o, x + 0.7, y + 0.74, z + 0.2, x + 0.86, y + 0.84, z + 0.3, 0.35, 0.34, 0.33, 0.0, s, l);
                return;
            }
            "chest" => {
                // Coffre : planches cerclées de fer, couvercle bombé, serrure.
                let wood = |o: &mut VBuf, a: f64, e: f64, c: f64, a1: f64, e1: f64, c1: f64, k: f64| bx3(o, x + a, y + e, z + c, x + a1, y + e1, z + c1, 0.55 * k, 0.37 * k, 0.2 * k, 0.0, s, l);
                wood(o, 0.08, 0.0, 0.14, 0.92, 0.56, 0.86, 1.0); wood(o, 0.07, 0.56, 0.13, 0.93, 0.72, 0.87, 1.1); wood(o, 0.1, 0.72, 0.18, 0.9, 0.78, 0.82, 1.05);
                for a in [0.16, 0.8] { bx3(o, x + a, y, z + 0.13, x + a + 0.05, y + 0.785, z + 0.87, 0.24, 0.24, 0.25, 0.0, s, l); }
                bx3(o, x + 0.44, y + 0.42, z + 0.86, x + 0.56, y + 0.6, z + 0.89, 0.7, 0.58, 0.3, 0.0, s, l);
                return;
            }
            "bedroll" => {
                // Sac de couchage : une natte de fibres roulée en tête, une couverture de peau.
                bx3(o, x + 0.08, y, z + 0.02, x + 0.92, y + 0.06, z + 0.98, 0.56, 0.5, 0.3, 0.0, s, l); bx3(o, x + 0.1, y + 0.06, z + 0.3, x + 0.9, y + 0.12, z + 0.97, 0.5, 0.36, 0.22, 0.0, s, l);
                bx3(o, x + 0.1, y + 0.06, z + 0.04, x + 0.9, y + 0.2, z + 0.26, 0.64, 0.58, 0.36, 0.0, s, l); return;
            }
            "bed" => {
                // Lit : cadre de bois, matelas de paille, couverture, oreiller.
                let wood = |o: &mut VBuf, a: f64, e: f64, c: f64, a1: f64, e1: f64, c1: f64| bx3(o, x + a, y + e, z + c, x + a1, y + e1, z + c1, 0.5, 0.34, 0.2, 0.0, s, l);
                wood(o, 0.02, 0.12, 0.02, 0.98, 0.3, 0.98); for (a, c) in [(0.02, 0.02), (0.88, 0.02), (0.02, 0.88), (0.88, 0.88)] { wood(o, a, 0.0, c, a + 0.1, 0.12, c + 0.1); } wood(o, 0.02, 0.3, 0.02, 0.98, 0.7, 0.1);
                bx3(o, x + 0.06, y + 0.3, z + 0.1, x + 0.94, y + 0.42, z + 0.96, 0.8, 0.72, 0.48, 0.0, s, l); bx3(o, x + 0.05, y + 0.42, z + 0.36, x + 0.95, y + 0.47, z + 0.97, 0.36, 0.42, 0.56, 0.0, s, l);
                bx3(o, x + 0.18, y + 0.42, z + 0.12, x + 0.82, y + 0.52, z + 0.32, 0.9, 0.88, 0.82, 0.0, s, l); return;
            }
            "torch" => {
                // Torche plantée : un bâton, une tête de fibres enroulées, une flamme vive.
                bx3(o, x + 0.46, y, z + 0.46, x + 0.54, y + 0.62, z + 0.54, 0.42, 0.28, 0.15, 0.0, s, l); bx3(o, x + 0.43, y + 0.58, z + 0.43, x + 0.57, y + 0.72, z + 0.57, 0.3, 0.24, 0.14, 0.0, s, l);
                bx(o, x + 0.44, y + 0.72, z + 0.44, x + 0.56, y + 0.9, z + 0.56, 2.5, 1.3, 0.35, 0.0, s, 255.0, EMIT, false, 0.0, 255.0); bx(o, x + 0.47, y + 0.88, z + 0.47, x + 0.53, y + 0.98, z + 0.53, 2.8, 1.9, 0.7, 0.0, s, 255.0, EMIT, false, 0.0, 255.0);
                return;
            }
            "wheat" => {
                // Blé : de jeunes pousses vertes, puis des tiges qui montent et jaunissent, des épis au dernier stade.
                let st = BLOCKS[b as usize].name.as_bytes().last().map_or(0, |c| (c - b'0') as usize); let h = [0.18, 0.4, 0.7, 0.92][st.min(3)]; let col = [[0.36, 0.6, 0.2], [0.42, 0.62, 0.22], [0.62, 0.66, 0.26], [0.86, 0.72, 0.32]][st.min(3)];
                for _ in 0..r_.int(9, 12) { let (a, c) = (x + r_.range(0.12, 0.88), z + r_.range(0.12, 0.88)); let hh = h * r_.range(0.8, 1.08); let k = r_.range(0.85, 1.1);
                    rod(o, [a, y, c], [a + r_.range(-0.05, 0.05), y + hh, c + r_.range(-0.05, 0.05)], 0.012, 0.007, [col[0] * k, col[1] * k, col[2] * k], s, l, FOLIAGE, 0.0, 120.0);
                    if st == 3 { bx(o, a - 0.025, y + hh - 0.02, c - 0.025, a + 0.025, y + hh + 0.12, c + 0.025, 0.9 * k, 0.76 * k, 0.36 * k, 120.0, s, l, FOLIAGE, true, 140.0, 255.0); } }
                return;
            }
            "twigs" => {
                // Branches tombées : deux ou trois rameaux couchés, écorce grise ou brune, quelques brindilles en épi.
                for _ in 0..r_.int(2, 3) {
                    let (a, c) = (x + r_.range(0.2, 0.8), z + r_.range(0.2, 0.8)); let an = r_.range(0.0, 6.28); let len = r_.range(0.32, 0.5); let t = r_.range(0.8, 1.1); let col = if r_.next() < 0.5 { [0.36 * t, 0.27 * t, 0.18 * t] } else { [0.42 * t, 0.39 * t, 0.34 * t] };
                    let (p0, p1) = ([a - cos(an) * len, y + 0.03, c - sin(an) * len], [a + cos(an) * len, y + 0.025, c + sin(an) * len]); rod(o, p0, p1, 0.028, 0.016, col, s, l, FLAT, 0.0, 0.0);
                    for _ in 0..r_.int(1, 2) { let f = r_.range(0.2, 0.75); let q = [p0[0] + (p1[0] - p0[0]) * f, y + 0.03, p0[2] + (p1[2] - p0[2]) * f]; let b2 = an + r_.range(0.5, 0.9) * if r_.next() < 0.5 { 1.0 } else { -1.0 }; let ln = r_.range(0.1, 0.18);
                        rod(o, q, [q[0] + cos(b2) * ln, y + 0.035, q[2] + sin(b2) * ln], 0.012, 0.007, col, s, l, FLAT, 0.0, 0.0); }
                }
                return;
            }
            "pebbles" => {
                // Cailloux : quatre à six galets plats, gris et beiges, à demi enfoncés.
                for _ in 0..r_.int(4, 6) {
                    let (a, c) = (x + r_.range(0.15, 0.85), z + r_.range(0.15, 0.85)); let (wx, wz) = (r_.range(0.04, 0.09), r_.range(0.035, 0.08)); let h = r_.range(0.03, 0.07); let t = r_.range(0.75, 1.1); let warm = r_.next() * 0.08;
                    bx3(o, a - wx, y, c - wz, a + wx, y + h, c + wz, (0.52 + warm) * t, (0.5 + warm * 0.6) * t, 0.47 * t, 0.0, s, l); bx3(o, a - wx * 0.6, y + h, c - wz * 0.6, a + wx * 0.6, y + h + 0.015, c + wz * 0.6, (0.58 + warm) * t, (0.56 + warm * 0.6) * t, 0.53 * t, 0.0, s, l);
                }
                return;
            }
            "workbench" => {
                // Établi : un plateau épais sur quatre pieds, une planche de rangement, un maillet et une scie posés dessus.
                let wood = |o: &mut VBuf, a: f64, e: f64, c: f64, a1: f64, e1: f64, c1: f64, k: f64| bx3(o, x + a, y + e, z + c, x + a1, y + e1, z + c1, 0.58 * k, 0.4 * k, 0.24 * k, 0.0, s, l);
                wood(o, 0.02, 0.8, 0.02, 0.98, 0.96, 0.98, 1.0); wood(o, 0.04, 0.78, 0.04, 0.96, 0.8, 0.96, 0.7);
                for (a, c) in [(0.06, 0.06), (0.8, 0.06), (0.06, 0.8), (0.8, 0.8)] { wood(o, a, 0.0, c, a + 0.14, 0.8, c + 0.14, 0.78); }
                wood(o, 0.08, 0.26, 0.08, 0.92, 0.32, 0.92, 0.86);
                bx3(o, x + 0.18, y + 0.96, z + 0.2, x + 0.5, y + 1.08, z + 0.32, 0.4, 0.26, 0.14, 0.0, s, l); bx3(o, x + 0.3, y + 0.96, z + 0.32, x + 0.36, y + 1.0, z + 0.6, 0.62, 0.45, 0.26, 0.0, s, l); // maillet
                bx3(o, x + 0.58, y + 0.96, z + 0.5, x + 0.86, y + 0.975, z + 0.66, 0.7, 0.7, 0.72, 0.0, s, l); bx3(o, x + 0.5, y + 0.96, z + 0.54, x + 0.6, y + 1.0, z + 0.62, 0.3, 0.18, 0.1, 0.0, s, l); // scie
                return;
            }
            "fence" => {
                let wood =|o: &mut VBuf, a: f64, e: f64, c: f64, a1: f64, e1: f64, c1: f64, k: f64| bx3(o, a, e, c, a1, e1, c1, 0.46 * k, 0.31 * k, 0.18 * k, 0.0, s, l);
                wood(o, x + 0.39, y, z + 0.39, x + 0.61, y + 1.0, z + 0.61, 1.0);
                wood(o, x + 0.35, y + 0.97, z + 0.35, x + 0.65, y + 1.04, z + 0.65, 1.12);
                let con = |b2: u8| b2 == B::FENCE || BLOCKS[b2 as usize].opaque;
                if voxel(0, -1, 0) == B::FENCE || (voxel(0, 1, 0) == B::FENCE && voxel(0, 2, 0) == B::FENCE) { return; }
                for yy in [0.34, 0.72] {
                    if con(voxel(1, 0, 0)) { wood(o, x + 0.66, y + yy, z + 0.42, x + 1.34, y + yy + 0.13, z + 0.58, 0.92); }
                    if con(voxel(0, 0, 1)) { wood(o, x + 0.42, y + yy, z + 0.66, x + 0.58, y + yy + 0.13, z + 1.34, 0.92); }
                    for dzi in [-1, 1] { if voxel(1, 0, dzi) == B::FENCE && !con(voxel(1, 0, 0)) && !con(voxel(0, 0, dzi)) {
                        let dz = dzi as f64;
                        wood(o, x + 0.61, y + yy, z + 0.42, x + 1.58, y + yy + 0.13, z + 0.58, 0.92);
                        wood(o, x + 1.42, y + yy, (z + 0.42).min(z + 0.5 + dz), x + 1.58, y + yy + 0.13, (z + 0.58).max(z + 0.5 + dz), 0.92);
                    } }
                }
                return;
            }
            "hollow" => {
                // Tronc creux : quatre parois d'écorce autour d'un cœur vide, ouvert dans le sens du fil, cernes visibles sur la tranche.
                let name = BLOCKS[b as usize].name; let ax = if name.ends_with("_x") { 0 } else if name.ends_with("_z") { 2 } else { 1 };
                let log = by_name(&format!("{}_log{}", species(b), ["_x", "", "_z"][ax])).unwrap_or(B::OAK_LOG); let lb = BLOCKS[log as usize];
                let same = |b2: u8| b2 == b; let d = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
                let (fwd, back) = (voxel(d[ax][0], d[ax][1], d[ax][2]), voxel(-d[ax][0], -d[ax][1], -d[ax][2]));
                const T: f64 = 0.22; let (u, v) = ([0, 1, 2].into_iter().filter(|k| *k != ax).collect::<Vec<_>>()[0], [0, 1, 2].into_iter().filter(|k| *k != ax).collect::<Vec<_>>()[1]);
                // (u0, u1, v0, v1) de chaque paroi, dans la section ; la longueur va de 0 à 1 le long du fil.
                for (u0, u1, v0, v1) in [(0.0, 1.0, 0.0, T), (0.0, 1.0, 1.0 - T, 1.0), (0.0, T, T, 1.0 - T), (1.0 - T, 1.0, T, 1.0 - T)] {
                    let mut lo = [0.0; 3]; let mut hi = [1.0; 3]; lo[u] = u0; hi[u] = u1; lo[v] = v0; hi[v] = v1;
                    for f in 0..6 {
                        let k = f / 2; let pos = f % 2 == 1; let mut n = [0.0; 3]; n[k] = if pos { 1.0 } else { -1.0 };
                        if k == ax && ((pos && same(fwd)) || (!pos && same(back))) { continue; } // tronc continu : pas de tranche entre deux blocs
                        let outer = k != ax && ((pos && hi[k] >= 1.0) || (!pos && lo[k] <= 0.0));
                        if outer { let nb = voxel(n[0] as i32, n[1] as i32, n[2] as i32); if is_opaque(nb) { continue; } }
                        let di = if k == 1 { if pos { 1 } else { 2 } } else if k == 0 { 0 } else { 3 }; o.rot = lb.grain[di];
                        // Face intérieure (vers le cœur vide) : du bois nu aux fibres longues, pas de l'écorce, un peu dans l'ombre.
                        let inner = k != ax && !outer; let layer = if inner { mat_layer("wood_inner") as f64 } else { lb.dir_faces[di] as f64 }; let tone = if inner { 0.8 } else { 1.0 };
                        let w = if pos { hi[k] } else { lo[k] }; let (a1, a2) = ((k + 1) % 3, (k + 2) % 3);
                        let corner = |p: f64, q: f64| { let mut c = [0.0; 3]; c[k] = w; c[a1] = p; c[a2] = q; [x + c[0], y + c[1], z + c[2]] };
                        let mut pts = [corner(lo[a1], lo[a2]), corner(hi[a1], lo[a2]), corner(hi[a1], hi[a2]), corner(lo[a1], hi[a2])]; if !pos { pts.swap(1, 3); }
                        for pt in pts { o.v(pt[0], pt[1], pt[2], layer, n[0], n[1], n[2], 0.0, if inner { 170.0 } else { 235.0 }, s, l, kind::TEX, tone, tone, tone); }
                    }
                }
                o.rot = 0; return;
            }
            "branch" => {
                // Branche fine : un nœud central et un bras vers chaque bois ou feuillage voisin, comme une clôture.
                let col = match species(b) { "birch" => [0.86, 0.84, 0.8], "beech" => [0.5, 0.48, 0.44], "spruce" => [0.32, 0.23, 0.16], "dead" => [0.56, 0.53, 0.49], "acacia" => [0.36, 0.3, 0.25], _ => [0.42, 0.29, 0.17] };
                let k = r_.range(0.88, 1.08); let w = r_.range(0.13, 0.18); let (c0, c1) = (0.5 - w, 0.5 + w); let col = [col[0] * k, col[1] * k, col[2] * k];
                let joins = |b2: u8| is_log(b2) || is_branch(b2) || is_leaf(b2);
                let mut any = false; let arm = |o: &mut VBuf, a: [f64; 6]| { bx3(o, x + a[0], y + a[1], z + a[2], x + a[3], y + a[4], z + a[5], col[0], col[1], col[2], 0.0, s, l); };
                arm(o, [c0, c0, c0, c1, c1, c1]);
                if joins(voxel(1, 0, 0)) { arm(o, [c1, c0, c0, 1.0, c1, c1]); any = true; } if joins(voxel(-1, 0, 0)) { arm(o, [0.0, c0, c0, c0, c1, c1]); any = true; }
                if joins(voxel(0, 0, 1)) { arm(o, [c0, c0, c1, c1, c1, 1.0]); any = true; } if joins(voxel(0, 0, -1)) { arm(o, [c0, c0, 0.0, c1, c1, c0]); any = true; }
                if joins(voxel(0, 1, 0)) { arm(o, [c0, c1, c0, c1, 1.0, c1]); any = true; } if joins(voxel(0, -1, 0)) || is_opaque(voxel(0, -1, 0)) { arm(o, [c0, 0.0, c0, c1, c0, c1]); any = true; }
                if !any { arm(o, [c0, 0.0, c0, c1, 1.0, c1]); }
                return;
            }
            "flagstone" => {
                let w = r_.range(0.32, 0.46); let d = r_.range(0.28, 0.44); let g = r_.range(0.42, 0.56); let a = x + 0.5 + r_.range(-0.08, 0.08); let c = z + 0.5 + r_.range(-0.08, 0.08);
                bx3(o, a - w, y, c - d, a + w, y + 0.13, c + d, g * 1.02, g, g * 0.92, 0.0, s, l);
                return;
            }
            "lily" => {
                let n = if lod == 0 { r_.int(1, 2) } else { 1 };
                for _ in 0..n {
                    let a = x + r_.range(0.25, 0.75); let c = z + r_.range(0.25, 0.75); let w = r_.range(0.2, 0.34); let g = r_.range(0.85, 1.1);
                    bx(o, a - w, y - 0.1, c - w, a + w, y - 0.06, c + w, 0.2 * g, 0.5 * g, 0.16 * g, 0.0, s, l, FOLIAGE, false, 0.0, 255.0);
                }
                return;
            }
            _ => {}
        }
        if lod == 1 {
            let (cx, cz) = (x + 0.5, z + 0.5);
            if m == "grass" {
                let t = r_.range(0.75, 1.15); let a = r_.range(0.0, 6.28);
                let h = r_.range(0.55, 0.9); blade(o, cx, y, cz, a, 0.42, h, 0.15, 0.42 * t, 0.56 * t, 0.15 * t, s, l, s);
                let h = r_.range(0.5, 0.8); blade(o, cx, y, cz, a + 1.57, 0.42, h, 0.15, 0.45 * t, 0.58 * t, 0.15 * t, s, l, s);
                return;
            }
            if m == "fern" || m == "fern_dry" {
                let k = if m == "fern" { [0.23, 0.46, 0.13] } else { [0.5, 0.32, 0.17] };
                let a = r_.range(0.0, 3.0); blade(o, cx, y, cz, a, 0.5, 0.6, 0.3, k[0], k[1], k[2], s, l, s);
                let a = r_.range(3.0, 6.0); blade(o, cx, y, cz, a, 0.5, 0.6, 0.3, k[0] * 1.1, k[1] * 1.1, k[2] * 1.1, s, l, s);
                return;
            }
            let col = match m { "lupine" => [0.44, 0.26, 0.72], "pink" => [0.9, 0.42, 0.6], "yellow" => [0.95, 0.74, 0.16], "mushroom" => [0.75, 0.6, 0.45], "glowcap" => [0.3, 0.7, 0.8], _ => [0.95, 0.93, 0.85] };
            let hh = if m == "lupine" { 0.8 } else { 0.4 };
            bx(o, x + 0.34, y + hh * 0.4, z + 0.34, x + 0.66, y + hh, z + 0.66, col[0], col[1], col[2], 40.0, s, l, FOLIAGE, false, 70.0, 255.0);
            return;
        }
        let cx = x + r_.range(0.25, 0.75); let cz = z + r_.range(0.25, 0.75);
        match m {
            "grass" => {
                let n = r_.int(12, 16); let base = r_.range(0.82, 1.14); let warm = r_.next();
                for _ in 0..n {
                    let t = base * r_.range(0.86, 1.14);
                    let (a, c, ang, w, h, lean) = (x + r_.next(), z + r_.next(), r_.range(0.0, 6.28), r_.range(0.02, 0.036), r_.range(0.22, 0.66), r_.range(0.04, 0.2));
                    blade(o, a, y, c, ang, w, h, lean, (0.43 + 0.13 * warm) * t, (0.54 + 0.05 * warm) * t, (0.2 + 0.03 * warm) * t, s, l, (s - 25.0).max(0.0));
                }
                for _ in 0..4 {
                    let (a, c, ang, w, h) = (x + r_.next(), z + r_.next(), r_.range(0.0, 6.28), r_.range(0.06, 0.1), r_.range(0.1, 0.2));
                    blade(o, a, y, c, ang, w, h, 0.1, 0.4 * base, 0.5 * base, 0.2 * base, s, l, (s - 40.0).max(0.0));
                }
            }
            "fern" | "fern_dry" => {
                let dry = m == "fern_dry"; let n = r_.int(5, 8); let h0 = r_.range(0.55, 1.0); let t = r_.range(0.85, 1.15); let start = r_.range(0.0, 6.28);
                for i in 0..n {
                    let a = start + (i as f64 / n as f64) * 6.28 + r_.range(-0.25, 0.25); let (ca, sa) = (cos(a), sin(a)); let ll = r_.range(0.55, 0.95);
                    let seg = 8;
                    for k in 1..=seg {
                        let f = k as f64 / seg as f64; let dist = ll * f; let hh = h0 * sin(f * 2.1) * (1.0 - f * 0.25); let lw = 0.15 * (1.0 - f * 0.86);
                        let (ax, az, ay) = (cx + ca * dist, cz + sa * dist, y + hh);
                        let f0 = (k - 1) as f64 / seg as f64; let (bxx, bz, by) = (cx + ca * ll * f0, cz + sa * ll * f0, y + h0 * sin(f0 * 2.1) * (1.0 - f0 * 0.25));
                        let (px2, pz2, w) = (-sa * lw, ca * lw, 40.0 + 170.0 * f);
                        let (g, re, bl) = if dry { (0.33 * t + 0.04 * (k & 1) as f64, 0.58 * t, 0.2 * t) } else { (0.42 * t + 0.05 * (k & 1) as f64, 0.3 * t, 0.17 * t) }; // fougère rousse : brunie par le gel
                        for side in [-1.0, 1.0] {
                            o.v(bxx, by, bz, 0.0, 0.0, 1.0, 0.0, w, 190.0, s, l, FOLIAGE, re * 0.8, g * 0.8, bl);
                            o.v(ax, ay, az, 0.0, 0.0, 1.0, 0.0, w, 190.0, s, l, FOLIAGE, re, g, bl);
                            o.v((ax + bxx) / 2.0 + px2 * side * 1.15, (ay + by) / 2.0 - 0.05, (az + bz) / 2.0 + pz2 * side * 1.15, 0.0, 0.0, 1.0, 0.0, w, 255.0, s, l, FOLIAGE, re * 1.2, g * 1.2, bl);
                            o.v(bxx + px2 * side * 0.8, by - 0.06, bz + pz2 * side * 0.8, 0.0, 0.0, 1.0, 0.0, w, 255.0, s, l, FOLIAGE, re * 1.1, g * 1.1, bl);
                        }
                    }
                }
            }
            "daisy" => {
                let n = r_.int(2, 3);
                for _ in 0..n {
                    let a = x + r_.range(0.2, 0.8); let c = z + r_.range(0.2, 0.8); let h = r_.range(0.32, 0.62); let w = r_.range(0.055, 0.075);
                    stem(o, a, y, c, h, s, l);
                    for p in 0..5 {
                        let t = (p as f64 / 5.0) * 6.28 + 0.4; let (ex, ez) = (cos(t) * w * 1.7, sin(t) * w * 1.7);
                        bx(o, a + ex - w, y + h, c + ez - w, a + ex + w, y + h + 0.03, c + ez + w, 1.25, 1.22, 1.12, 70.0, s, l, FOLIAGE, false, 70.0, 255.0);
                    }
                    bx(o, a - w * 0.8, y + h, c - w * 0.8, a + w * 0.8, y + h + 0.06, c + w * 0.8, 1.3, 0.92, 0.1, 70.0, s, l, FOLIAGE, false, 70.0, 255.0);
                }
            }
            "lupine" => {
                let n = r_.int(1, 3);
                for _ in 0..n {
                    let a = x + r_.range(0.25, 0.75); let c = z + r_.range(0.25, 0.75); let hh = r_.range(0.75, 1.3); let tone = r_.range(0.85, 1.15);
                    stem(o, a, y, c, hh * 0.6, s, l);
                    let seg = 7;
                    for k in 0..seg {
                        let f = k as f64 / seg as f64; let w = 0.095 * (1.0 - f * 0.65); let yy = y + hh * (0.42 + 0.58 * f); let h2 = hh * 0.075;
                        let col = if k > seg - 3 { 1.15 } else { 1.0 }; let wi = 60.0 + 12.0 * k as f64;
                        bx(o, a - w, yy, c - w, a + w, yy + h2, c + w, 0.5 * tone * col, 0.3 * tone * col, 0.82 * tone * col, wi, s, l, FOLIAGE, false, wi, 255.0);
                    }
                    for _ in 0..3 { let t = r_.range(0.0, 6.28); blade(o, a, y, c, t, 0.045, 0.3, 0.2, 0.3, 0.46, 0.17, s, l, s); }
                }
            }
            "pink" | "yellow" => {
                let col = if m == "pink" { [1.05, 0.42, 0.62] } else { [1.35, 0.98, 0.16] }; let n = r_.int(3, 5);
                for _ in 0..n {
                    let a = x + r_.range(0.15, 0.85); let c = z + r_.range(0.15, 0.85); let h = r_.range(0.18, 0.4); let w = r_.range(0.035, 0.055);
                    stem(o, a, y, c, h, s, l);
                    bx(o, a - w, y + h, c - w, a + w, y + h + 0.07, c + w, col[0], col[1], col[2], 80.0, s, l, FOLIAGE, false, 80.0, 255.0);
                }
                let t = r_.range(0.0, 6.3); blade(o, x + 0.5, y, z + 0.5, t, 0.04, 0.34, 0.15, 0.36, 0.5, 0.18, s, l, s);
            }
            "snowlayer" => {
                // Pellicule de neige : une dalle fine, texturée, posée sur le bloc ; ses bords ne se dessinent que vers l'air.
                let h = snow_level(b) as f64 / 10.0; let layer = BLOCKS[B::SNOW as usize].faces[0] as f64; let (a, b, c, d, e) = (x, x + 1.0, z, z + 1.0, y + h);
                let mut q = |p: [[f64; 3]; 4], n: [f64; 3], ao: f64| { for v in p { o.v(v[0], v[1], v[2], layer, n[0], n[1], n[2], 0.0, ao, s, l, TEX, 1.0, 1.0, 1.0); } };
                q([[a, e, d], [b, e, d], [b, e, c], [a, e, c]], [0.0, 1.0, 0.0], 255.0);
                // Le bord se dessine depuis le haut de la pellicule voisine, s'il y en a une plus basse.
                let low = |v: u8| if is_opaque(v) { e } else { y + snow_level(v) as f64 / 10.0 };
                let f = low(voxel(0, 0, 1)); if f < e { q([[a, f, d], [b, f, d], [b, e, d], [a, e, d]], [0.0, 0.0, 1.0], 220.0); }
                let f = low(voxel(0, 0, -1)); if f < e { q([[b, f, c], [a, f, c], [a, e, c], [b, e, c]], [0.0, 0.0, -1.0], 220.0); }
                let f = low(voxel(1, 0, 0)); if f < e { q([[b, f, d], [b, f, c], [b, e, c], [b, e, d]], [1.0, 0.0, 0.0], 220.0); }
                let f = low(voxel(-1, 0, 0)); if f < e { q([[a, f, c], [a, f, d], [a, e, d], [a, e, c]], [-1.0, 0.0, 0.0], 220.0); }
            }
            "pinecone" => {
                // Pomme de pin : pendue sous une branche, ou tombée au sol.
                let hang = is_leaf(voxel(0, 1, 0)); let n = if hang { 1 } else { r_.int(1, 3) };
                for _ in 0..n {
                    let (a, c) = (x + r_.range(0.3, 0.7), z + r_.range(0.3, 0.7)); let k = r_.range(0.85, 1.15);
                    if hang { for (y0, y1, w) in [(0.9, 1.0, 0.035), (0.72, 0.9, 0.075), (0.6, 0.72, 0.05)] { bx3(o, a - w, y + y0, c - w, a + w, y + y1, c + w, 0.4 * k, 0.24 * k, 0.12 * k, 0.0, s, l); } }
                    else { for (x0, x1, w) in [(-0.13, -0.04, 0.05), (-0.04, 0.09, 0.075), (0.09, 0.15, 0.045)] { bx3(o, a + x0, y, c - w, a + x1, y + w * 1.6, c + w, 0.4 * k, 0.24 * k, 0.12 * k, 0.0, s, l); } }
                }
            }
            "frost" => {
                // Herbes givrées : des brins pâles qui percent la neige.
                let n = r_.int(6, 10);
                for _ in 0..n { let t = r_.range(0.85, 1.1); let (a, c, ang, w, h, lean) = (x + r_.next(), z + r_.next(), r_.range(0.0, 6.28), r_.range(0.02, 0.035), r_.range(0.2, 0.55), r_.range(0.05, 0.2)); blade(o, a, y, c, ang, w, h, lean, 0.78 * t, 0.83 * t, 0.86 * t, s, l, (s - 25.0).max(0.0)); }
            }
            "feather" => {
                // Stipe plumeuse : de longues tiges dorées qui s'arquent sous le vent, quelques brins verts au pied.
                let n = r_.int(10, 15); let base = r_.range(0.88, 1.1); let lean0 = r_.range(0.0, 6.28);
                for _ in 0..n {
                    let t = base * r_.range(0.88, 1.12); let ang = lean0 + r_.range(-0.9, 0.9);
                    blade(o, x + r_.range(0.2, 0.8), y, z + r_.range(0.2, 0.8), ang, r_.range(0.012, 0.024), r_.range(0.5, 1.05), r_.range(0.18, 0.42), 0.76 * t, 0.68 * t, 0.44 * t, s, l, (s - 25.0).max(0.0));
                }
                for _ in 0..4 { blade(o, x + r_.range(0.3, 0.7), y, z + r_.range(0.3, 0.7), r_.range(0.0, 6.28), r_.range(0.03, 0.05), r_.range(0.15, 0.3), 0.08, 0.46 * base, 0.5 * base, 0.28 * base, s, l, (s - 40.0).max(0.0)); }
            }
            "sage" => {
                // Armoise : des tiges ligneuses qui s'écartent du collet, couvertes de petites touffes de feuilles gris-vert argenté.
                let (a0, c0) = (x + r_.range(0.42, 0.58), z + r_.range(0.42, 0.58)); let size = r_.range(0.8, 1.15); let base = [0.5, 0.56, 0.47];
                let n = r_.int(5, 7); let spin = r_.range(0.0, 6.28);
                for i in 0..n {
                    let a = spin + i as f64 / n as f64 * 6.28 + r_.range(-0.3, 0.3); let (rr, hh) = (r_.range(0.12, 0.3) * size, r_.range(0.22, 0.44) * size);
                    let tip = [a0 + cos(a) * rr, y + hh, c0 + sin(a) * rr]; rod(o, [a0, y, c0], tip, 0.022, 0.012, [0.36, 0.3, 0.24], s, l, FLAT, 0.0, 40.0);
                    for q in 0..r_.int(9, 13) {
                        let f = 0.35 + 0.65 * q as f64 / 12.0; let (px, py, pz) = (a0 + (tip[0] - a0) * f + r_.range(-0.09, 0.09), y + (tip[1] - y) * f + r_.range(-0.05, 0.07), c0 + (tip[2] - c0) * f + r_.range(-0.09, 0.09));
                        let w = r_.range(0.03, 0.055) * size; let k = r_.range(0.82, 1.15) * (0.75 + 0.35 * f); let silver = r_.range(0.0, 0.12);
                        bx(o, px - w, py - w * 0.7, pz - w, px + w, py + w * 0.7, pz + w, (base[0] + silver) * k, (base[1] + silver) * k, (base[2] + silver) * k, 30.0 * f, s, l, FOLIAGE, true, 40.0 * f, 255.0);
                    }
                }
            }
            "cactus" => {
                // Cactus : un fût côtelé plus fin qu'un bloc, piqué d'aréoles claires ; il se raccorde aux cactus voisins pour former tronc et bras.
                let up = voxel(0, 1, 0) == b; let down = voxel(0, -1, 0) == b; let sides: Vec<(i32, i32)> = [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter().filter(|d| voxel(d.0, 0, d.1) == b && (!down || voxel(d.0, -1, d.1) != b)).collect(); // deux fûts côte à côte ne se soudent qu'au pied du bras, pas à chaque étage
                let w = if sides.is_empty() || up || down { 0.21 + 0.03 * ((hash_int(wx, 0, wz, 5) & 3) as f64 / 3.0) } else { 0.17 };
                let (cx, cz) = (x + 0.5, z + 0.5); let g = [0.19, 0.4, 0.2];
                let col = |o: &mut VBuf, y0: f64, y1: f64, r_: &mut Rng| {
                    rod(o, [cx, y + y0, cz], [cx, y + y1, cz], w, w, g, s, l, FLAT, 0.0, 0.0);
                    for k in 0..8 { let a = k as f64 * 0.785 + 0.39; let (rx, rz) = (cx + cos(a) * w * 1.02, cz + sin(a) * w * 1.02); rod(o, [rx, y + y0, rz], [rx, y + y1, rz], 0.035, 0.035, [g[0] * 1.15, g[1] * 1.15, g[2] * 1.1], s, l, FLAT, 0.0, 0.0);
                        let mut t = y0 + 0.08; while t < y1 - 0.04 { if r_.next() < 0.8 { let (sx, sz) = (cx + cos(a) * (w + 0.05), cz + sin(a) * (w + 0.05)); bx3(o, sx - 0.012, y + t, sz - 0.012, sx + 0.012, y + t + 0.024, sz + 0.012, 0.88, 0.85, 0.72, 0.0, s, l); } t += 0.16; } }
                };
                if up || down || sides.is_empty() || sides.len() == 1 && up {
                    let y0 = if down || sides.is_empty() { 0.0 } else { 0.5 - w }; let y1 = if up { 1.0 } else { 0.86 };
                    col(o, y0, y1, r_); if !up { rod(o, [cx, y + y1, cz], [cx, y + y1 + 0.06, cz], w * 0.85, w * 0.55, g, s, l, FLAT, 0.0, 0.0); } // sommet arrondi
                }
                for (dx, dz) in &sides { let e = [cx + *dx as f64 * 0.5, y + 0.5, cz + *dz as f64 * 0.5]; rod(o, [cx, y + 0.5, cz], e, w * 0.9, w * 0.9, g, s, l, FLAT, 0.0, 0.0); }
                if !up && !down && !sides.is_empty() && !(sides.len() == 1 && up) { rod(o, [cx, y + 0.5 - w, cz], [cx, y + 0.5 + w, cz], w, w, g, s, l, FLAT, 0.0, 0.0); } // coude d'un bras horizontal
                return;
            }
            "heather" => {
                // Bruyère : un coussin bas de rameaux ligneux serrés, couverts de minuscules feuilles et d'épis de fleurs mauves.
                let n = r_.int(6, 9); let bloom = r_.range(0.6, 1.0);
                for _ in 0..n {
                    let (a, c) = (x + r_.range(0.12, 0.88), z + r_.range(0.12, 0.88)); let h = r_.range(0.14, 0.32); let top = [a + r_.range(-0.06, 0.06), y + h, c + r_.range(-0.06, 0.06)];
                    rod(o, [a, y, c], top, 0.012, 0.007, [0.3, 0.24, 0.18], s, l, FLAT, 0.0, 30.0);
                    for k in 0..4 { let f = 0.3 + 0.17 * k as f64; let (px, py, pz) = (a + (top[0] - a) * f, y + h * f, c + (top[2] - c) * f); let w = 0.035 - 0.005 * k as f64; let g = r_.range(0.85, 1.1); bx(o, px - w, py, pz - w, px + w, py + 0.04, pz + w, 0.24 * g, 0.34 * g, 0.2 * g, 40.0 * f, s, l, FOLIAGE, true, 50.0 * f, 255.0); }
                    let k = r_.range(0.85, 1.15) * bloom; bx(o, top[0] - 0.022, top[1] - 0.09, top[2] - 0.022, top[0] + 0.022, top[1] + 0.02, top[2] + 0.022, 0.62 * k, 0.32 * k, 0.6 * k, 50.0, s, l, FOLIAGE, true, 60.0, 255.0);
                }
            }
            "cotton" => {
                // Linaigrette : de fines tiges qui portent chacune une houppe de coton blanc.
                for _ in 0..r_.int(5, 8) { let (a, c) = (x + r_.range(0.15, 0.85), z + r_.range(0.15, 0.85)); let h = r_.range(0.3, 0.55); let top = [a + r_.range(-0.05, 0.05), y + h, c + r_.range(-0.05, 0.05)];
                    rod(o, [a, y, c], top, 0.008, 0.006, [0.4, 0.46, 0.28], s, l, FOLIAGE, 0.0, 90.0); let w = r_.range(0.035, 0.05); bx(o, top[0] - w, top[1], top[2] - w, top[0] + w, top[1] + w * 1.6, top[2] + w, 0.95, 0.95, 0.92, 90.0, s, l, FOLIAGE, true, 100.0, 255.0); }
                for _ in 0..3 { blade(o, x + r_.range(0.2, 0.8), y, z + r_.range(0.2, 0.8), r_.range(0.0, 6.28), 0.03, r_.range(0.15, 0.3), 0.1, 0.38, 0.46, 0.22, s, l, (s - 40.0).max(0.0)); }
            }
            "deadbush" => {
                // Buisson mort : des tiges ligneuses en volume, qui se ramifient trois fois et finissent en brindilles grises.
                let (a0, c0) = (x + r_.range(0.42, 0.58), z + r_.range(0.42, 0.58)); let gray = r_.range(0.0, 1.0); let size = r_.range(0.8, 1.2);
                let col = |k: f64| [(0.42 + 0.1 * gray) * k, (0.35 + 0.12 * gray) * k, (0.27 + 0.15 * gray) * k];
                let n = r_.int(5, 8); let spin = r_.range(0.0, 6.28);
                let mut stack: Vec<([f64; 3], f64, f64, f64, f64, u8)> = (0..n).map(|i| ([a0, y, c0], spin + i as f64 / n as f64 * 6.28, r_.range(0.35, 0.95), r_.range(0.3, 0.5) * size, 0.022, 0u8)).collect();
                while let Some((p0, a, up, len, w, depth)) = stack.pop() {
                    let (h, sp) = (up * len, (1.0 - up * up).max(0.05).sqrt() * len); let p1 = [p0[0] + cos(a) * sp, p0[1] + h, p0[2] + sin(a) * sp];
                    rod(o, p0, p1, w, w * 0.6, col(r_.range(0.85, 1.15) * (1.0 + 0.08 * depth as f64)), s, l, FLAT, 0.0, 0.0);
                    if depth < 2 { for _ in 0..r_.int(2, 3) { let f = r_.range(0.45, 1.0); let q = [p0[0] + (p1[0] - p0[0]) * f, p0[1] + (p1[1] - p0[1]) * f, p0[2] + (p1[2] - p0[2]) * f]; stack.push((q, a + r_.range(-0.9, 0.9), (up + r_.range(-0.3, 0.2)).clamp(0.15, 0.95), len * r_.range(0.4, 0.6), w * 0.6, depth + 1)); } }
                }
            }
            "blueberry" => {
                // Myrtillier : des rameaux fins et anguleux, de petites feuilles ovales et quelques baies bleu nuit.
                let n = r_.int(5, 8);
                for _ in 0..n {
                    let (a, c) = (x + r_.range(0.15, 0.85), z + r_.range(0.15, 0.85)); let h = r_.range(0.14, 0.32); let t = r_.range(0.85, 1.15); let lean = (r_.range(-0.06, 0.06), r_.range(-0.06, 0.06));
                    let top = [a + lean.0, y + h, c + lean.1]; rod(o, [a, y, c], top, 0.01, 0.006, [0.3 * t, 0.36 * t, 0.16 * t], s, l, FLAT, 0.0, 30.0);
                    for k in 0..r_.int(5, 8) { let f = r_.range(0.35, 1.0); let (px, py, pz) = (a + lean.0 * f + r_.range(-0.05, 0.05), y + h * f, c + lean.1 * f + r_.range(-0.05, 0.05)); let (wl, wd) = if k % 2 == 0 { (0.035, 0.02) } else { (0.02, 0.035) }; let g = r_.range(0.85, 1.15);
                        bx(o, px - wl, py, pz - wd, px + wl, py + 0.012, pz + wd, 0.26 * t * g, 0.46 * t * g, 0.18 * t * g, 50.0 * f, s, l, FOLIAGE, true, 60.0 * f, 255.0); }
                    if r_.next() < 0.65 { let f = r_.range(0.3, 0.8); let (px, py, pz) = (a + lean.0 * f + r_.range(-0.03, 0.03), y + h * f - 0.03, c + lean.1 * f + r_.range(-0.03, 0.03)); if !picked { bx(o, px - 0.022, py, pz - 0.022, px + 0.022, py + 0.042, pz + 0.022, 0.12, 0.15, 0.38, 40.0, s, l, FLAT, true, 50.0, 255.0); } }
                }
            }
            "rowan" => {
                // Jeune sorbier : deux ou trois tiges fines, des feuilles composées en folioles qui roussissent, des grappes de baies rouges.
                let (a0, c0) = (x + 0.5, z + 0.5); let hh = r_.range(0.85, 1.4); let n = r_.int(2, 3);
                for _ in 0..n {
                    let top = [a0 + r_.range(-0.18, 0.18), y + hh * r_.range(0.75, 1.0), c0 + r_.range(-0.18, 0.18)]; rod(o, [a0 + r_.range(-0.04, 0.04), y, c0 + r_.range(-0.04, 0.04)], top, 0.025, 0.012, [0.4, 0.33, 0.26], s, l, FLAT, 0.0, 60.0);
                    for _ in 0..r_.int(4, 6) {
                        // Feuille composée : une nervure et des paires de folioles allongées de part et d'autre.
                        let f = r_.range(0.45, 1.0); let q = [a0 + (top[0] - a0) * f, y + (top[1] - y) * f, c0 + (top[2] - c0) * f]; let a = r_.range(0.0, 6.28); let len = r_.range(0.2, 0.32); let warm = r_.next();
                        let e = [q[0] + cos(a) * len, q[1] + r_.range(-0.04, 0.06), q[2] + sin(a) * len]; rod(o, q, e, 0.006, 0.004, [0.42, 0.4, 0.2], s, l, FOLIAGE, 60.0, 90.0);
                        let col = [(0.4 + 0.42 * warm), (0.44 - 0.16 * warm), 0.14 + 0.02 * warm];
                        for k in 1..=4 { let g = k as f64 / 4.5; let (mx, my, mz) = (q[0] + (e[0] - q[0]) * g, q[1] + (e[1] - q[1]) * g, q[2] + (e[2] - q[2]) * g); for side in [-1.0, 1.0] { let (px, pz) = (mx - sin(a) * 0.045 * side, mz + cos(a) * 0.045 * side); let kk = r_.range(0.85, 1.15);
                            bx(o, px - 0.028, my, pz - 0.028, px + 0.028, my + 0.01, pz + 0.028, col[0] * kk, col[1] * kk, col[2] * kk, 70.0, s, l, FOLIAGE, true, 90.0, 255.0); } }
                    }
                    for _ in 0..r_.int(1, 2) { let q = [top[0] + r_.range(-0.1, 0.1), top[1] - r_.range(0.05, 0.2), top[2] + r_.range(-0.1, 0.1)];
                        for (ox, oy, oz) in [(0.0, 0.0, 0.0), (0.04, -0.03, 0.02), (-0.035, -0.03, -0.02), (0.01, -0.06, -0.035), (-0.02, -0.07, 0.03), (0.03, -0.09, -0.01)] { if picked { continue; } bx(o, q[0] + ox - 0.022, q[1] + oy, q[2] + oz - 0.022, q[0] + ox + 0.022, q[1] + oy + 0.04, q[2] + oz + 0.022, 0.9, 0.13, 0.08, 60.0, s, l, FLAT, true, 70.0, 255.0); } }
                }
            }
            "icicle" => {
                // Glaçons : des pointes effilées qui pendent du bloc du dessus.
                let n = r_.int(2, 4);
                for _ in 0..n {
                    let a = x + r_.range(0.2, 0.8); let c = z + r_.range(0.2, 0.8); let len = r_.range(0.3, 0.95); let w0 = r_.range(0.05, 0.1);
                    for k in 0..3 { let f = k as f64 / 3.0; let w = w0 * (1.0 - f * 0.7); bx3(o, a - w, y + 1.0 - len * (f + 1.0 / 3.0), c - w, a + w, y + 1.0 - len * f, c + w, 0.78, 0.88, 0.98, 0.0, s, l); }
                }
            }
            "glowcap" => {
                // Champignon lumineux : un pied pâle, un chapeau qui éclaire le sous-bois.
                let n = r_.int(2, 4);
                for _ in 0..n {
                    let a = x + r_.range(0.25, 0.75); let c = z + r_.range(0.25, 0.75); let h = r_.range(0.2, 0.5); let w = r_.range(0.08, 0.17);
                    bx3(o, a - 0.03, y, c - 0.03, a + 0.03, y + h, c + 0.03, 0.82, 0.86, 0.9, 0.0, s, l);
                    let col = GLOW[glow_index(b).unwrap_or(0)]; // vert, bleu, rose, violet ou turquoise : la couleur est celle du bloc
                    bx(o, a - w, y + h, c - w, a + w, y + h + 0.09, c + w, col[0], col[1], col[2], 0.0, s, 255.0, EMIT, true, 0.0, 255.0);
                }
            }
            "mushroom" => {
                let n = r_.int(1, 3);
                for _ in 0..n {
                    let a = x + r_.range(0.3, 0.7); let c = z + r_.range(0.3, 0.7); let h = r_.range(0.14, 0.36); let w = r_.range(0.11, 0.2);
                    bx3(o, a - 0.04, y, c - 0.04, a + 0.04, y + h, c + 0.04, 0.9, 0.82, 0.68, 0.0, s, l);
                    bx3(o, a - w, y + h, c - w, a + w, y + h + 0.1, c + w, 0.78, 0.6, 0.42, 0.0, s, l);
                    bx3(o, a - w * 0.6, y + h + 0.1, c - w * 0.6, a + w * 0.6, y + h + 0.16, c + w * 0.6, 0.85, 0.68, 0.5, 0.0, s, l);
                }
            }
            _ => {}
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn face(o: &mut VBuf, p: [f64; 12], n: [f64; 3], wa: f64, wb: f64, ao_l: f64, ao: f64, col: [f64; 3], s: f64, l: f64, kd: u8) {
    let ll = 0.86 + 0.14 * n[1] + 0.05 * n[0]; let (r, g, b) = (col[0] * ll, col[1] * ll, col[2] * ll);
    o.v(p[0], p[1], p[2], 0.0, n[0], n[1], n[2], wa, ao_l, s, l, kd, r, g, b);
    o.v(p[3], p[4], p[5], 0.0, n[0], n[1], n[2], wa, ao_l, s, l, kd, r, g, b);
    o.v(p[6], p[7], p[8], 0.0, n[0], n[1], n[2], wb, ao, s, l, kd, r, g, b);
    o.v(p[9], p[10], p[11], 0.0, n[0], n[1], n[2], wb, ao, s, l, kd, r, g, b);
}
/// Tige en volume : un prisme à section carrée de `a` vers `b`, qui s'affine de `w0` à `w1`, fermé au bout.
#[allow(clippy::too_many_arguments)]
fn rod(o: &mut VBuf, a: [f64; 3], b: [f64; 3], w0: f64, w1: f64, col: [f64; 3], s: f64, l: f64, kd: u8, wa: f64, wb: f64) {
    let d0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]]; let dl = hypot3(d0[0], d0[1], d0[2]).max(1e-6); let d = [d0[0] / dl, d0[1] / dl, d0[2] / dl];
    let cr = |p: [f64; 3], q: [f64; 3]| [p[1] * q[2] - p[2] * q[1], p[2] * q[0] - p[0] * q[2], p[0] * q[1] - p[1] * q[0]]; let nz = |p: [f64; 3]| { let m = hypot3(p[0], p[1], p[2]).max(1e-6); [p[0] / m, p[1] / m, p[2] / m] };
    let u = nz(cr(d, if d[1].abs() > 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] })); let v = cr(u, d);
    let at = |p: [f64; 3], n: [f64; 3], r: [f64; 3], w: f64, sr: f64| [p[0] + (n[0] + r[0] * sr) * w, p[1] + (n[1] + r[1] * sr) * w, p[2] + (n[2] + r[2] * sr) * w];
    for n in [u, v, [-u[0], -u[1], -u[2]], [-v[0], -v[1], -v[2]]] {
        let r = cr(d, n); let (p0, p1, p2, p3) = (at(a, n, r, w0, -1.0), at(a, n, r, w0, 1.0), at(b, n, r, w1, 1.0), at(b, n, r, w1, -1.0));
        face(o, [p0[0], p0[1], p0[2], p1[0], p1[1], p1[2], p2[0], p2[1], p2[2], p3[0], p3[1], p3[2]], n, wa, wb, 200.0, 255.0, col, s, l, kd);
    }
    let c = |su: f64, sv: f64| [b[0] + (u[0] * su + v[0] * sv) * w1, b[1] + (u[1] * su + v[1] * sv) * w1, b[2] + (u[2] * su + v[2] * sv) * w1];
    let (q0, q1, q2, q3) = (c(-1.0, 1.0), c(1.0, 1.0), c(1.0, -1.0), c(-1.0, -1.0));
    face(o, [q0[0], q0[1], q0[2], q1[0], q1[1], q1[2], q2[0], q2[1], q2[2], q3[0], q3[1], q3[2]], d, wb, wb, 255.0, 255.0, col, s, l, kd);
}
/// Pavé coloré, pour le décor et les petits modèles.
#[allow(clippy::too_many_arguments)]
pub fn bx(o: &mut VBuf, x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64, r: f64, g: f64, b: f64, wind: f64, s: f64, l: f64, kd: u8, bottom: bool, wt: f64, ao: f64) {
    let c = [r, g, b];
    face(o, [x0, y1, z1, x1, y1, z1, x1, y1, z0, x0, y1, z0], [0.0, 1.0, 0.0], wt, wt, 255.0, ao, c, s, l, kd);
    if bottom { face(o, [x0, y0, z0, x1, y0, z0, x1, y0, z1, x0, y0, z1], [0.0, -1.0, 0.0], wind, wind, 255.0, ao, c, s, l, kd); }
    let sa = ao * 0.8;
    face(o, [x0, y0, z1, x1, y0, z1, x1, y1, z1, x0, y1, z1], [0.0, 0.0, 1.0], wind, wt, sa, ao, c, s, l, kd);
    face(o, [x1, y0, z0, x0, y0, z0, x0, y1, z0, x1, y1, z0], [0.0, 0.0, -1.0], wind, wt, sa, ao, c, s, l, kd);
    face(o, [x1, y0, z1, x1, y0, z0, x1, y1, z0, x1, y1, z1], [1.0, 0.0, 0.0], wind, wt, sa, ao, c, s, l, kd);
    face(o, [x0, y0, z0, x0, y0, z1, x0, y1, z1, x0, y1, z0], [-1.0, 0.0, 0.0], wind, wt, sa, ao, c, s, l, kd);
}
#[allow(clippy::too_many_arguments)]
fn bx3(o: &mut VBuf, x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64, r: f64, g: f64, b: f64, wind: f64, s: f64, l: f64) { bx(o, x0, y0, z0, x1, y1, z1, r, g, b, wind, s, l, FLAT, false, wind, 255.0) }
#[allow(clippy::too_many_arguments)]
fn blade(o: &mut VBuf, x: f64, y: f64, z: f64, a: f64, w: f64, h: f64, lean: f64, r: f64, g: f64, b: f64, s: f64, l: f64, sky2: f64) {
    let (ca, sa) = (cos(a), sin(a)); let (wx, wz, tx, tz, ty) = (-sa * w, ca * w, x + ca * lean, z + sa * lean, y + h);
    let (nx, nz, ny, mh) = (ca * 0.35, sa * 0.35, 0.93, 0.55);
    let (mx, mz, my) = (x + ca * lean * 0.35, z + sa * lean * 0.35, y + h * mh);
    let d = 0.62; let f = FOLIAGE;
    o.v(x - wx, y, z - wz, 0.0, nx, ny, nz, 0.0, 150.0, sky2, l, f, r * d, g * d, b * d);
    o.v(x + wx, y, z + wz, 0.0, nx, ny, nz, 0.0, 150.0, sky2, l, f, r * d, g * d, b * d);
    o.v(mx + wx * 0.8, my, mz + wz * 0.8, 0.0, nx, ny, nz, 60.0, 225.0, s, l, f, r * 0.9, g * 0.9, b * 0.9);
    o.v(mx - wx * 0.8, my, mz - wz * 0.8, 0.0, nx, ny, nz, 60.0, 225.0, s, l, f, r * 0.9, g * 0.9, b * 0.9);
    o.v(mx - wx * 0.8, my, mz - wz * 0.8, 0.0, nx, ny, nz, 60.0, 225.0, s, l, f, r * 0.9, g * 0.9, b * 0.9);
    o.v(mx + wx * 0.8, my, mz + wz * 0.8, 0.0, nx, ny, nz, 60.0, 225.0, s, l, f, r * 0.9, g * 0.9, b * 0.9);
    o.v(tx + wx * 0.1, ty, tz + wz * 0.1, 0.0, nx, ny, nz, 170.0, 255.0, s, l, f, r * 1.12, g * 1.12, b * 1.12);
    o.v(tx - wx * 0.1, ty, tz - wz * 0.1, 0.0, nx, ny, nz, 170.0, 255.0, s, l, f, r * 1.12, g * 1.12, b * 1.12);
}
fn stem(o: &mut VBuf, x: f64, y: f64, z: f64, h: f64, s: f64, l: f64) { bx(o, x - 0.016, y, z - 0.016, x + 0.016, y + h, z + 0.016, 0.25, 0.42, 0.13, 0.0, s, l, FOLIAGE, false, 90.0, 255.0) }

#[allow(clippy::too_many_arguments)]
fn leaf_cubes(o: &mut VBuf, b: u8, x: f64, y: f64, z: f64, wx: i32, yi: i32, wz: i32, s: f64, l: f64, lod: u8) {
    let layer = BLOCKS[b as usize].faces[0] as f64; let t = BLOCKS[b as usize].tint;
    let mut r = Rng::new(hash_int(wx, yi, wz, 77)); let zone = hash_int(wx >> 3, yi >> 2, wz >> 3, 7) as f64 / 4294967296.0; let lit = 0.74 + 0.4 * (s / 255.0);
    // Épicéa : un étage net d'un bloc, dessus et côtés distincts (neige sur le dessus quand il y en a).
    if b == B::SPRUCE_LEAVES || b == B::SPRUCE_LEAVES_SNOW {
        let bk = BLOCKS[b as usize]; let (top, side, bot) = (bk.dir_faces[1] as f64, bk.dir_faces[0] as f64, bk.dir_faces[2] as f64); let j = |r: &mut Rng| r.range(-0.05, 0.05);
        let (a, e, c) = (x + j(&mut r), y, z + j(&mut r)); let (a1, e1, c1) = (a + 1.0 + j(&mut r).abs(), e + 1.0, c + 1.0 + j(&mut r).abs()); let k = r.range(0.9, 1.08) * lit;
        let mut quad = |p: [f64; 12], ly: f64, nx: f64, ny: f64, nz: f64, ao: f64| { for i in (0..12).step_by(3) { o.v(p[i], p[i + 1], p[i + 2], ly, nx, ny, nz, 60.0, ao, s, l, LEAF, k, k, k); } };
        quad([a, e1, c1, a1, e1, c1, a1, e1, c, a, e1, c], top, 0.0, 1.0, 0.0, 255.0);
        quad([a, e, c1, a1, e, c1, a1, e1, c1, a, e1, c1], side, 0.0, 0.0, 1.0, 215.0); quad([a1, e, c, a, e, c, a, e1, c, a1, e1, c], side, 0.0, 0.0, -1.0, 215.0);
        quad([a1, e, c1, a1, e, c, a1, e1, c, a1, e1, c1], side, 1.0, 0.0, 0.0, 215.0); quad([a, e, c, a, e, c1, a, e1, c1, a, e1, c], side, -1.0, 0.0, 0.0, 215.0);
        quad([a, e, c, a1, e, c, a1, e, c1, a, e, c1], bot, 0.0, -1.0, 0.0, 150.0); return;
    }
    let n = if lod == 0 { 3 } else { 1 };
    for _ in 0..n {
        let size = if lod == 0 { r.range(0.5, 0.86) } else { r.range(1.0, 1.14) };
        let off = |r: &mut Rng| if lod == 0 { r.range(0.0, 1.0 - size * 0.55) } else { r.range(-0.1, 0.02) };
        let a = x + off(&mut r); let e = y + off(&mut r); let c = z + off(&mut r);
        let k = r.range(0.8, 1.22) * lit; let warm = r.range(0.92, 1.1) * (0.92 + 0.34 * zone);
        let (rr, gg, bb) = (t[0] * k * warm, t[1] * k, t[2] * k * 0.92);
        let (a1, e1, c1) = (a + size, e + size, c + size);
        let mut quad = |p: [f64; 12], nx: f64, ny: f64, nz: f64, ao: f64| { for j in (0..12).step_by(3) { o.v(p[j], p[j + 1], p[j + 2], layer, nx, ny, nz, 110.0, ao, s, l, LEAF, rr, gg, bb); } };
        quad([a, e1, c1, a1, e1, c1, a1, e1, c, a, e1, c], 0.0, 1.0, 0.0, 255.0);
        quad([a, e, c1, a1, e, c1, a1, e1, c1, a, e1, c1], 0.0, 0.0, 1.0, 215.0);
        quad([a1, e, c, a, e, c, a, e1, c, a1, e1, c], 0.0, 0.0, -1.0, 215.0);
        quad([a1, e, c1, a1, e, c, a1, e1, c, a1, e1, c1], 1.0, 0.0, 0.0, 215.0);
        quad([a, e, c, a, e, c1, a, e1, c1, a, e1, c], -1.0, 0.0, 0.0, 215.0);
        quad([a, e, c, a1, e, c, a1, e, c1, a, e, c1], 0.0, -1.0, 0.0, 150.0);
    }
}

/// Anneau de relief lointain : colonnes grossières qui donnent un horizon au-delà des chunks.
pub fn far_mesh(gen: &WorldGen, ccx: i32, ccz: i32) -> (Vec<u8>, usize) {
    const CELLS: usize = 110; let n1 = CELLS + 1; let half = CELLS as f64 / 2.0;
    let mut o = VBuf::new(50000); o.scale = 8.0;
    // Deux anneaux : 880 m en mailles de 16, puis jusqu'à 3,5 km en mailles de 64, un peu plus bas pour ne jamais percer le premier.
    for (step, sink, hole) in [(16.0f64, 0.0f64, 0.0f64), (64.0, 1.6, 880.0 - 64.0)] {
        let mut hs = vec![0f32; n1 * n1]; let mut cols = vec![[0f32; 3]; n1 * n1];
        let (ox, oz) = (ccx as f64 * CX as f64 - half * step, ccz as f64 * CX as f64 - half * step);
        for j in 0..n1 { for i in 0..n1 {
            let (wx, wz) = (ox + i as f64 * step, oz + j as f64 * step); let c = gen.column(wx, wz); let k = i + j * n1;
            let canopy = if c.water != 0.0 || c.h > 86.0 { 0.0 } else if c.forest > 0.35 { 13.0 * (1f64).min((c.forest - 0.35) * 3.0) } else { 0.0 };
            hs[k] = ((if c.water != 0.0 { c.water } else { c.h }) + canopy - sink) as f32;
            let t = (gen.n_deco.n2(wx / 40.0, wz / 40.0) + 1.0) * 0.5;
            let (tg, dp, st, de) = (c.mix.strong(crate::biome::Biome::Taiga), c.mix.strong(crate::biome::Biome::DeepForest), c.mix.strong(crate::biome::Biome::Steppe), c.mix.strong(crate::biome::Biome::Desert));
            cols[k] = if c.mix.strong(crate::biome::Biome::Highlands) > 0.5 && c.water == 0.0 && c.h <= 100.0 { [(0.48 + 0.08 * t) as f32, (0.46 + 0.06 * t) as f32, (0.33 + 0.04 * t) as f32] } else if de > 0.5 && c.water == 0.0 && c.h <= 95.0 { [(0.84 + 0.05 * t) as f32, (0.7 + 0.04 * t) as f32, (0.48 + 0.03 * t) as f32] } else if st > 0.5 && c.water == 0.0 && c.h <= 90.0 && canopy <= 3.0 { [(0.56 + 0.08 * t) as f32, (0.5 + 0.06 * t) as f32, (0.3 + 0.04 * t) as f32] } else if tg > 0.5 || c.h > 100.0 { if c.water != 0.0 { [0.7, 0.8, 0.88] } else if canopy > 3.0 { [0.42, 0.5, 0.48] } else { [0.86, 0.9, 0.95] } } else if dp > 0.5 && canopy > 3.0 { [0.1, 0.2, 0.12] } else if c.water != 0.0 { [0.22, 0.36, 0.42] } else if c.h > 90.0 { [(0.42 + 0.1 * t) as f32, (0.4 + 0.1 * t) as f32, (0.37 + 0.09 * t) as f32] } else if canopy > 3.0 { [(0.2 + 0.1 * t) as f32, (0.34 + 0.1 * t) as f32, 0.09] } else { [(0.42 + 0.1 * t) as f32, (0.5 + 0.05 * t) as f32, 0.2] };
        } }
        for j in 0..CELLS { for i in 0..CELLS {
            let (x0, z0) = ((i as f64 - half) * step, (j as f64 - half) * step); let (x1, z1) = (x0 + step, z0 + step);
            if hole > 0.0 && (x0 + step / 2.0).abs().max((z0 + step / 2.0).abs()) < hole { continue; }
            // Normales et couleurs par sommet : le relief lointain reste doux, sans grandes facettes.
            let at = |ii: isize, jj: isize| hs[ii.clamp(0, CELLS as isize) as usize + jj.clamp(0, CELLS as isize) as usize * n1] as f64;
            for (ci, cj, x, z) in [(i, j + 1, x0, z1), (i + 1, j + 1, x1, z1), (i + 1, j, x1, z0), (i, j, x0, z0)] {
                let (p, q) = (ci as isize, cj as isize); let (mut nx, mut nz, mut ny) = (at(p - 1, q) - at(p + 1, q), at(p, q - 1) - at(p, q + 1), 2.0 * step);
                let l = hypot3(nx, ny, nz); nx /= l; ny /= l; nz /= l; let k = ci + cj * n1; let c = cols[k];
                o.v(x, hs[k] as f64, z, 0.0, nx, ny, nz, 0.0, 255.0, 255.0, 0.0, FLAT, c[0] as f64, c[1] as f64, c[2] as f64);
            }
        } }
    }
    let n = o.n; (o.data, n)
}
