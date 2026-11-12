//! Planche éclairée des matériaux (albédo × lumière rasante lue dans la carte de normales), à la résolution du jeu.
use alw_core::blocks::{MATERIALS, VARIANTS};
fn main() {
    let a: Vec<String> = std::env::args().collect(); let names = &a[2..]; let s = 256usize; let t = alw_core::textures::build_textures(s); let v = VARIANTS as usize; let show = 160usize;
    let (w, h) = (show * names.len(), show); let mut img = vec![0u8; w * h * 3]; let l = [-0.55f64, -0.45, 0.7]; let ln = (l[0] * l[0] + l[1] * l[1] + l[2] * l[2]).sqrt();
    for (r, n) in names.iter().enumerate() { let m = MATERIALS.iter().position(|x| x == n).expect("matériau"); let lay = m * v;
        for y in 0..show { for x in 0..show { let p = lay * s * s * 4 + (x + y * s) * 4; let nx = t.nrm[p] as f64 / 127.5 - 1.0; let ny = t.nrm[p + 1] as f64 / 127.5 - 1.0; let nz = (1.0 - nx * nx - ny * ny).max(0.0).sqrt();
            let d = ((nx * l[0] + ny * l[1] + nz * l[2]) / ln).max(0.0); let k = 0.35 + 0.9 * d; let q = (y * w + r * show + x) * 3; for c in 0..3 { img[q + c] = (t.albedo[p + c] as f64 * k).min(255.0) as u8; } } } }
    let mut out = format!("P6 {w} {h} 255\n").into_bytes(); out.extend_from_slice(&img); std::fs::write(&a[1], out).unwrap();
}
