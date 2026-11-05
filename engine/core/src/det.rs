//! Calculs flottants reproductibles. `f64::sin` dépend de la bibliothèque du système, donc d'une machine à l'autre :
//! un monde doit être identique partout, alors sinus, cosinus, hypoténuse et arrondi sont écrits ici (fdlibm, comme V8).
#![allow(clippy::excessive_precision)]
#[inline] fn hi(x: f64) -> i32 { (x.to_bits() >> 32) as i32 }
#[inline] fn from_hi(h: u32) -> f64 { f64::from_bits((h as u64) << 32) }

fn rem_pio2(x: f64) -> (i32, f64, f64) {
    const INV: f64 = 6.36619772367581382433e-01;
    const P1: f64 = 1.57079632673412561417e+00; const P1T: f64 = 6.07710050650619224932e-11;
    const P2: f64 = 6.07710050630396597660e-11; const P2T: f64 = 2.02226624879595063154e-21;
    const P3: f64 = 2.02226624871116645580e-21; const P3T: f64 = 8.47842766036889956997e-32;
    let hx = hi(x); let ix = hx & 0x7fffffff;
    if ix <= 0x3fe921fb { return (0, x, 0.0); }
    if ix < 0x4002d97c {
        return if hx > 0 {
            let mut z = x - P1;
            let (y0, y1);
            if ix != 0x3ff921fb { y0 = z - P1T; y1 = (z - y0) - P1T; } else { z -= P2; y0 = z - P2T; y1 = (z - y0) - P2T; }
            (1, y0, y1)
        } else {
            let mut z = x + P1;
            let (y0, y1);
            if ix != 0x3ff921fb { y0 = z + P1T; y1 = (z - y0) + P1T; } else { z += P2; y0 = z + P2T; y1 = (z - y0) + P2T; }
            (-1, y0, y1)
        };
    }
    assert!(ix <= 0x413921fb, "angle hors du domaine prévu");
    let t = x.abs(); let n = (t * INV + 0.5) as i32; let f = n as f64;
    let mut r = t - f * P1; let mut w = f * P1T; let j = ix >> 20;
    let mut y0 = r - w; let mut i = j - ((hi(y0) >> 20) & 0x7ff);
    if i > 16 {
        let t2 = r; w = f * P2; r = t2 - w; w = f * P2T - ((t2 - r) - w); y0 = r - w; i = j - ((hi(y0) >> 20) & 0x7ff);
        if i > 49 { let t3 = r; w = f * P3; r = t3 - w; w = f * P3T - ((t3 - r) - w); y0 = r - w; }
    }
    let y1 = (r - y0) - w;
    if hx < 0 { (-n, -y0, -y1) } else { (n, y0, y1) }
}
fn k_sin(x: f64, y: f64, iy: bool) -> f64 {
    const S1: f64 = -1.66666666666666324348e-01; const S2: f64 = 8.33333333332248946124e-03; const S3: f64 = -1.98412698298579493134e-04;
    const S4: f64 = 2.75573137070700676789e-06; const S5: f64 = -2.50507602534068634195e-08; const S6: f64 = 1.58969099521155010221e-10;
    let ix = hi(x) & 0x7fffffff;
    if ix < 0x3e400000 && x as i32 == 0 { return x; }
    let z = x * x; let v = z * x; let r = S2 + z * (S3 + z * (S4 + z * (S5 + z * S6)));
    if !iy { x + v * (S1 + z * r) } else { x - ((z * (0.5 * y - v * r) - y) - v * S1) }
}
fn k_cos(x: f64, y: f64) -> f64 {
    const C1: f64 = 4.16666666666666019037e-02; const C2: f64 = -1.38888888888741095749e-03; const C3: f64 = 2.48015872894767294178e-05;
    const C4: f64 = -2.75573143513906633035e-07; const C5: f64 = 2.08757232129817482790e-09; const C6: f64 = -1.13596475577881948265e-11;
    let ix = hi(x) & 0x7fffffff;
    if ix < 0x3e400000 && x as i32 == 0 { return 1.0; }
    let z = x * x; let r = z * (C1 + z * (C2 + z * (C3 + z * (C4 + z * (C5 + z * C6)))));
    if ix < 0x3fd33333 { return 1.0 - (0.5 * z - (z * r - x * y)); }
    let qx = if ix > 0x3fe90000 { 0.28125 } else { from_hi((ix - 0x00200000) as u32) };
    let iz = 0.5 * z - qx; let a = 1.0 - qx;
    a - (iz - (z * r - x * y))
}
pub fn sin(x: f64) -> f64 {
    if hi(x) & 0x7fffffff <= 0x3fe921fb { return k_sin(x, 0.0, false); }
    let (n, a, b) = rem_pio2(x);
    match n & 3 { 0 => k_sin(a, b, true), 1 => k_cos(a, b), 2 => -k_sin(a, b, true), _ => -k_cos(a, b) }
}
pub fn cos(x: f64) -> f64 {
    if hi(x) & 0x7fffffff <= 0x3fe921fb { return k_cos(x, 0.0); }
    let (n, a, b) = rem_pio2(x);
    match n & 3 { 0 => k_cos(a, b), 1 => -k_sin(a, b, true), 2 => -k_cos(a, b), _ => k_sin(a, b, true) }
}
fn hyp(v: &[f64]) -> f64 {
    let mut m = 0.0f64; for a in v { m = m.max(a.abs()); }
    if m == 0.0 { return 0.0; }
    let (mut sum, mut comp) = (0.0f64, 0.0f64);
    for a in v { let n = a.abs() / m; let s = n * n - comp; let p = sum + s; comp = (p - sum) - s; sum = p; }
    sum.sqrt() * m
}
pub fn hypot2(a: f64, b: f64) -> f64 { hyp(&[a, b]) }
pub fn hypot3(a: f64, b: f64, c: f64) -> f64 { hyp(&[a, b, c]) }
/// Arrondi à l'entier, moitiés vers +∞.
#[inline] pub fn round(x: f64) -> f64 { let c = x.ceil(); if c - 0.5 > x { c - 1.0 } else { c } }
