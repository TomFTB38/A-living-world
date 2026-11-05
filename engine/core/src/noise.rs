//! Hachage et bruits déterministes : une même seed produit exactement le même monde, quel que soit l'ordre de chargement.
pub fn hash_int(a: i32, b: i32, c: i32, d: i32) -> u32 {
    let m = |x: i32, k: u32| (x as u32).wrapping_mul(k);
    let mut h = m(a, 0x27d4eb2d) ^ m(b, 0x165667b1) ^ m(c, 0x9e3779b1) ^ m(d, 0x85ebca6b);
    h = (h ^ (h >> 15)).wrapping_mul(0x2c1b3c6d);
    h = (h ^ (h >> 12)).wrapping_mul(0x297a2d39);
    h ^ (h >> 15)
}
#[inline] pub fn hash_float(a: i32, b: i32, c: i32, d: i32) -> f64 { hash_int(a, b, c, d) as f64 / 4294967296.0 }

/// Générateur séquentiel (mulberry32) pour la croissance des arbres.
pub struct Rng { s: u32 }
impl Rng {
    pub fn new(seed: u32) -> Self { Rng { s: seed } }
    pub fn next(&mut self) -> f64 {
        self.s = self.s.wrapping_add(0x6d2b79f5);
        let mut t = self.s;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
        (t ^ (t >> 14)) as f64 / 4294967296.0
    }
    pub fn range(&mut self, a: f64, b: f64) -> f64 { a + (b - a) * self.next() }
    pub fn int(&mut self, a: i32, b: i32) -> i32 { self.range(a as f64, (b + 1) as f64).floor() as i32 }
}

const G3: [f64; 36] = [1.,1.,0.,-1.,1.,0.,1.,-1.,0.,-1.,-1.,0.,1.,0.,1.,-1.,0.,1.,1.,0.,-1.,-1.,0.,-1.,0.,1.,1.,0.,-1.,1.,0.,1.,-1.,0.,-1.,-1.];

pub struct Noise { p: [u8; 512], f2: f64, g2: f64 }
impl Noise {
    pub fn new(seed: i32) -> Self {
        let mut p = [0u8; 256];
        for (i, v) in p.iter_mut().enumerate() { *v = i as u8; }
        let mut r = Rng::new(seed as u32);
        for i in (1..256usize).rev() { let j = (r.next() * (i + 1) as f64).floor() as usize; p.swap(i, j); }
        let mut q = [0u8; 512];
        for i in 0..512 { q[i] = p[i & 255]; }
        let s3 = 3f64.sqrt();
        Noise { p: q, f2: 0.5 * (s3 - 1.0), g2: (3.0 - s3) / 6.0 }
    }
    #[inline(always)] fn pp(&self, i: i64) -> i64 { self.p[i as usize] as i64 }
    /// Simplex 2D, environ [-1, 1].
    pub fn n2(&self, x: f64, y: f64) -> f64 {
        let (f2, g2) = (self.f2, self.g2);
        let s = (x + y) * f2; let i = (x + s).floor(); let j = (y + s).floor(); let t = (i + j) * g2;
        let (x0, y0) = (x - i + t, y - j + t);
        let (i1, j1): (i64, i64) = if x0 > y0 { (1, 0) } else { (0, 1) };
        let (x1, y1) = (x0 - i1 as f64 + g2, y0 - j1 as f64 + g2);
        let (x2, y2) = (x0 - 1.0 + 2.0 * g2, y0 - 1.0 + 2.0 * g2);
        let (ii, jj) = ((i as i64) & 255, (j as i64) & 255);
        let mut n = 0.0;
        let mut tt = 0.5 - x0 * x0 - y0 * y0;
        if tt > 0.0 { let g = (self.pp(ii + self.pp(jj)) % 12 * 3) as usize; tt *= tt; n += tt * tt * (G3[g] * x0 + G3[g + 1] * y0); }
        tt = 0.5 - x1 * x1 - y1 * y1;
        if tt > 0.0 { let g = (self.pp(ii + i1 + self.pp(jj + j1)) % 12 * 3) as usize; tt *= tt; n += tt * tt * (G3[g] * x1 + G3[g + 1] * y1); }
        tt = 0.5 - x2 * x2 - y2 * y2;
        if tt > 0.0 { let g = (self.pp(ii + 1 + self.pp(jj + 1)) % 12 * 3) as usize; tt *= tt; n += tt * tt * (G3[g] * x2 + G3[g + 1] * y2); }
        70.0 * n
    }
    /// Perlin 3D, environ [-1, 1].
    pub fn n3(&self, x: f64, y: f64, z: f64) -> f64 {
        let (xf, yf, zf) = (x.floor(), y.floor(), z.floor());
        let (fx, fy, fz) = (x - xf, y - yf, z - zf);
        let (xi, yi, zi) = ((xf as i64) & 255, (yf as i64) & 255, (zf as i64) & 255);
        let fade = |t: f64| t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
        let (u, v, w) = (fade(fx), fade(fy), fade(fz));
        let gr = |h: i64, a: f64, b: f64, c: f64| { let h = (h % 12 * 3) as usize; G3[h] * a + G3[h + 1] * b + G3[h + 2] * c };
        let a = self.pp(xi) + yi; let aa = self.pp(a) + zi; let ab = self.pp(a + 1) + zi;
        let b = self.pp(xi + 1) + yi; let ba = self.pp(b) + zi; let bb = self.pp(b + 1) + zi;
        let l = |t: f64, a: f64, b: f64| a + t * (b - a);
        l(w,
          l(v, l(u, gr(self.pp(aa), fx, fy, fz), gr(self.pp(ba), fx - 1.0, fy, fz)), l(u, gr(self.pp(ab), fx, fy - 1.0, fz), gr(self.pp(bb), fx - 1.0, fy - 1.0, fz))),
          l(v, l(u, gr(self.pp(aa + 1), fx, fy, fz - 1.0), gr(self.pp(ba + 1), fx - 1.0, fy, fz - 1.0)), l(u, gr(self.pp(ab + 1), fx, fy - 1.0, fz - 1.0), gr(self.pp(bb + 1), fx - 1.0, fy - 1.0, fz - 1.0)))) * 1.1
    }
    pub fn fbm2(&self, x: f64, y: f64, oct: usize, lac: f64, gain: f64) -> f64 {
        let (mut a, mut f, mut s, mut n) = (1.0, 1.0, 0.0, 0.0);
        for i in 0..oct { s += a * self.n2(x * f + i as f64 * 17.3, y * f - i as f64 * 9.1); n += a; a *= gain; f *= lac; }
        s / n
    }
    pub fn fbm(&self, x: f64, y: f64, oct: usize) -> f64 { self.fbm2(x, y, oct, 2.03, 0.5) }
}

#[inline] pub fn clamp(v: f64, a: f64, b: f64) -> f64 { if v < a { a } else if v > b { b } else { v } }
#[inline] pub fn lerp(a: f64, b: f64, t: f64) -> f64 { a + (b - a) * t }
#[inline] pub fn smooth(a: f64, b: f64, x: f64) -> f64 { let t = clamp((x - a) / (b - a), 0.0, 1.0); t * t * (3.0 - 2.0 * t) }
