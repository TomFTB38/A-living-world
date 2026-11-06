//! Maillage : faces fusionnées, occlusion par sommet, lumière voxel (ciel et blocs), géométrie enrichie.
use crate::blocks::*;
use crate::det::{cos, hypot3, round, sin};
use crate::gen::WorldGen;
use crate::noise::{hash_int, Rng};

pub const VSIZE: usize = 20;
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
                        self.m1[pos] = (face << 8) | ao | (if is_leaf(b) { 1 << 24 } else { 0 }) | (grain << 25) | (tone(b) << 28);
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
        let r_ = &mut r;
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
            "fence" => {
                let wood = |o: &mut VBuf, a: f64, e: f64, c: f64, a1: f64, e1: f64, c1: f64, k: f64| bx3(o, a, e, c, a1, e1, c1, 0.46 * k, 0.31 * k, 0.18 * k, 0.0, s, l);
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
            if m == "fern" {
                let a = r_.range(0.0, 3.0); blade(o, cx, y, cz, a, 0.5, 0.6, 0.3, 0.23, 0.46, 0.13, s, l, s);
                let a = r_.range(3.0, 6.0); blade(o, cx, y, cz, a, 0.5, 0.6, 0.3, 0.26, 0.5, 0.14, s, l, s);
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
            "fern" => {
                let n = r_.int(5, 8); let h0 = r_.range(0.55, 1.0); let t = r_.range(0.85, 1.15); let start = r_.range(0.0, 6.28);
                for i in 0..n {
                    let a = start + (i as f64 / n as f64) * 6.28 + r_.range(-0.25, 0.25); let (ca, sa) = (cos(a), sin(a)); let ll = r_.range(0.55, 0.95);
                    let seg = 8;
                    for k in 1..=seg {
                        let f = k as f64 / seg as f64; let dist = ll * f; let hh = h0 * sin(f * 2.1) * (1.0 - f * 0.25); let lw = 0.15 * (1.0 - f * 0.86);
                        let (ax, az, ay) = (cx + ca * dist, cz + sa * dist, y + hh);
                        let f0 = (k - 1) as f64 / seg as f64; let (bxx, bz, by) = (cx + ca * ll * f0, cz + sa * ll * f0, y + h0 * sin(f0 * 2.1) * (1.0 - f0 * 0.25));
                        let (px2, pz2, w) = (-sa * lw, ca * lw, 40.0 + 170.0 * f);
                        let g = 0.42 * t + 0.05 * (k & 1) as f64; let re = 0.3 * t; let bl = 0.17 * t;
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
            "glowcap" => {
                // Champignon lumineux : un pied pâle, un chapeau qui éclaire le sous-bois.
                let n = r_.int(2, 4);
                for _ in 0..n {
                    let a = x + r_.range(0.25, 0.75); let c = z + r_.range(0.25, 0.75); let h = r_.range(0.2, 0.5); let w = r_.range(0.08, 0.17);
                    bx3(o, a - 0.03, y, c - 0.03, a + 0.03, y + h, c + 0.03, 0.82, 0.86, 0.9, 0.0, s, l);
                    // couleurs mystiques : vert, bleu, rose, violet, turquoise
                    const PAL: [[f64; 3]; 5] = [[0.3, 0.85, 0.3], [0.18, 0.42, 0.95], [0.95, 0.32, 0.7], [0.62, 0.3, 0.95], [0.15, 0.75, 0.78]]; let col = PAL[r_.int(0, 4) as usize];
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
            cols[k] = if c.water != 0.0 { [0.22, 0.36, 0.42] } else if c.h > 90.0 { [(0.42 + 0.1 * t) as f32, (0.4 + 0.1 * t) as f32, (0.37 + 0.09 * t) as f32] } else if canopy > 3.0 { [(0.2 + 0.1 * t) as f32, (0.34 + 0.1 * t) as f32, 0.09] } else { [(0.42 + 0.1 * t) as f32, (0.5 + 0.05 * t) as f32, 0.2] };
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
