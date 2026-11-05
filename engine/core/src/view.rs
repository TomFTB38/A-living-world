//! Caméra, projection et astre. Matrices en colonnes, profondeur inversée : proche = 1, infini = 0.
pub type M4 = [f32; 16];
pub fn mul(a: &M4, b: &M4) -> M4 { let mut o = [0.0; 16]; for c in 0..4 { for r in 0..4 { let mut s = 0.0; for k in 0..4 { s += a[r + k * 4] * b[k + c * 4]; } o[r + c * 4] = s; } } o }
pub fn norm(v: [f64; 3]) -> [f64; 3] { let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt(); [v[0] / l, v[1] / l, v[2] / l] }
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
pub fn perspective(fovy: f64, aspect: f64, near: f64) -> M4 { let f = (1.0 / (fovy * 0.5).tan()) as f32; let mut m = [0.0; 16]; m[0] = f / aspect as f32; m[5] = f; m[11] = -1.0; m[14] = near as f32; m }
pub fn look_dir(eye: [f64; 3], f: [f64; 3]) -> M4 {
    let s = norm(cross(f, [0.0, 1.0, 0.0])); let u = cross(s, f);
    let c = |v: f64| v as f32;
    [c(s[0]), c(u[0]), c(-f[0]), 0.0, c(s[1]), c(u[1]), c(-f[1]), 0.0, c(s[2]), c(u[2]), c(-f[2]), 0.0, c(-dot(s, eye)), c(-dot(u, eye)), c(dot(f, eye)), 1.0]
}
pub fn invert(m: &M4) -> M4 {
    let a: Vec<f64> = m.iter().map(|v| *v as f64).collect(); let mut o = [0.0f64; 16];
    o[0] = a[5]*a[10]*a[15]-a[5]*a[11]*a[14]-a[9]*a[6]*a[15]+a[9]*a[7]*a[14]+a[13]*a[6]*a[11]-a[13]*a[7]*a[10];
    o[4] = -a[4]*a[10]*a[15]+a[4]*a[11]*a[14]+a[8]*a[6]*a[15]-a[8]*a[7]*a[14]-a[12]*a[6]*a[11]+a[12]*a[7]*a[10];
    o[8] = a[4]*a[9]*a[15]-a[4]*a[11]*a[13]-a[8]*a[5]*a[15]+a[8]*a[7]*a[13]+a[12]*a[5]*a[11]-a[12]*a[7]*a[9];
    o[12] = -a[4]*a[9]*a[14]+a[4]*a[10]*a[13]+a[8]*a[5]*a[14]-a[8]*a[6]*a[13]-a[12]*a[5]*a[10]+a[12]*a[6]*a[9];
    o[1] = -a[1]*a[10]*a[15]+a[1]*a[11]*a[14]+a[9]*a[2]*a[15]-a[9]*a[3]*a[14]-a[13]*a[2]*a[11]+a[13]*a[3]*a[10];
    o[5] = a[0]*a[10]*a[15]-a[0]*a[11]*a[14]-a[8]*a[2]*a[15]+a[8]*a[3]*a[14]+a[12]*a[2]*a[11]-a[12]*a[3]*a[10];
    o[9] = -a[0]*a[9]*a[15]+a[0]*a[11]*a[13]+a[8]*a[1]*a[15]-a[8]*a[3]*a[13]-a[12]*a[1]*a[11]+a[12]*a[3]*a[9];
    o[13] = a[0]*a[9]*a[14]-a[0]*a[10]*a[13]-a[8]*a[1]*a[14]+a[8]*a[2]*a[13]+a[12]*a[1]*a[10]-a[12]*a[2]*a[9];
    o[2] = a[1]*a[6]*a[15]-a[1]*a[7]*a[14]-a[5]*a[2]*a[15]+a[5]*a[3]*a[14]+a[13]*a[2]*a[7]-a[13]*a[3]*a[6];
    o[6] = -a[0]*a[6]*a[15]+a[0]*a[7]*a[14]+a[4]*a[2]*a[15]-a[4]*a[3]*a[14]-a[12]*a[2]*a[7]+a[12]*a[3]*a[6];
    o[10] = a[0]*a[5]*a[15]-a[0]*a[7]*a[13]-a[4]*a[1]*a[15]+a[4]*a[3]*a[13]+a[12]*a[1]*a[7]-a[12]*a[3]*a[5];
    o[14] = -a[0]*a[5]*a[14]+a[0]*a[6]*a[13]+a[4]*a[1]*a[14]-a[4]*a[2]*a[13]-a[12]*a[1]*a[6]+a[12]*a[2]*a[5];
    o[3] = -a[1]*a[6]*a[11]+a[1]*a[7]*a[10]+a[5]*a[2]*a[11]-a[5]*a[3]*a[10]-a[9]*a[2]*a[7]+a[9]*a[3]*a[6];
    o[7] = a[0]*a[6]*a[11]-a[0]*a[7]*a[10]-a[4]*a[2]*a[11]+a[4]*a[3]*a[10]+a[8]*a[2]*a[7]-a[8]*a[3]*a[6];
    o[11] = -a[0]*a[5]*a[11]+a[0]*a[7]*a[9]+a[4]*a[1]*a[11]-a[4]*a[3]*a[9]-a[8]*a[1]*a[7]+a[8]*a[3]*a[5];
    o[15] = a[0]*a[5]*a[10]-a[0]*a[6]*a[9]-a[4]*a[1]*a[10]+a[4]*a[2]*a[9]+a[8]*a[1]*a[6]-a[8]*a[2]*a[5];
    let d = a[0]*o[0]+a[1]*o[4]+a[2]*o[8]+a[3]*o[12]; let mut r = [0.0f32; 16]; for i in 0..16 { r[i] = (o[i] / d) as f32; } r
}
/// Un pavé est-il au moins en partie dans le champ de la caméra ? Plans gauche, droit, bas, haut extraits de vp.
pub fn visible(vp: &M4, min: [f32; 3], max: [f32; 3]) -> bool {
    let r = |i: usize| [vp[i], vp[4 + i], vp[8 + i], vp[12 + i]]; let (x, y, w) = (r(0), r(1), r(3));
    for k in 0..4 {
        let p = match k { 0 => [w[0] + x[0], w[1] + x[1], w[2] + x[2], w[3] + x[3]], 1 => [w[0] - x[0], w[1] - x[1], w[2] - x[2], w[3] - x[3]], 2 => [w[0] + y[0], w[1] + y[1], w[2] + y[2], w[3] + y[3]], _ => [w[0] - y[0], w[1] - y[1], w[2] - y[2], w[3] - y[3]] };
        let q = [if p[0] > 0.0 { max[0] } else { min[0] }, if p[1] > 0.0 { max[1] } else { min[1] }, if p[2] > 0.0 { max[2] } else { min[2] }];
        if p[0] * q[0] + p[1] * q[1] + p[2] * q[2] + p[3] < 0.0 { return false; }
    }
    true
}
#[derive(Clone, Copy, Debug)]
pub struct Sky { pub dir: [f32; 3], pub col: [f32; 3], pub top: [f32; 3], pub hor: [f32; 3], pub ground: [f32; 3], pub night: f32, pub exposure: f32 }
/// Soleil et ciel selon l'heure, comme le prototype. La lune prend le relais : jamais de lumière par-dessous.
pub fn sky_at(hour: f64) -> Sky {
    let a = ((hour - 6.0) / 12.0) * std::f64::consts::PI; let el = a.sin();
    let mx = |x: [f64; 3], y: [f64; 3], t: f64| [x[0] + (y[0] - x[0]) * t, x[1] + (y[1] - x[1]) * t, x[2] + (y[2] - x[2]) * t];
    let cl = |v: f64| v.clamp(0.0, 1.0);
    let d = norm([a.cos() * 0.82, el * 0.9 + 0.02, 0.38 + 0.2 * (1.0 - el)]);
    let k = cl((d[1] + 0.06) / 0.5); let st = k * k * (3.0 - 2.0 * k); let day = cl((d[1] + 0.1) / 0.25); let night = 1.0 - cl((d[1] + 0.12) / 0.18); let moon = cl((-d[1] - 0.04) / 0.12);
    let moon_side = d[1] < -0.04;
    let dir = if moon_side { norm([-d[0], -d[1] + 0.05, d[2]]) } else { d };
    let col = if moon_side { [0.13 * moon, 0.18 * moon, 0.3 * moon] } else { let c = mx([4.2, 1.9, 0.75], [6.2, 5.3, 3.7], st); let f = cl((d[1] + 0.04) / 0.14); [c[0] * f, c[1] * f, c[2] * f] };
    let f = |c: [f64; 3]| [c[0] as f32, c[1] as f32, c[2] as f32];
    Sky { dir: f(dir), col: f(col), top: f(mx(mx([0.012, 0.02, 0.05], [0.19, 0.3, 0.62], day), [0.22, 0.46, 1.05], st)), hor: f(mx(mx([0.03, 0.045, 0.09], [1.05, 0.62, 0.42], day), [0.82, 1.0, 1.25], st)), ground: f(mx([0.02, 0.022, 0.03], [0.3, 0.3, 0.15], day)), night: night as f32, exposure: (0.6 + 0.32 * night) as f32 }
}
