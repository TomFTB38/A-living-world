//! Planche de contrôle des matériaux nommés : chaque variante en 128 px, répétée 2 × 2 pour voir les raccords.
use alw_core::blocks::{MATERIALS, VARIANTS};
fn main() {
    let a: Vec<String> = std::env::args().collect(); let names = &a[2..]; let t = alw_core::textures::build_textures(128); let s = 128usize; let v = VARIANTS as usize;
    let (w, h) = (s * 2 * v, s * 2 * names.len()); let mut img = vec![0u8; w * h * 3];
    for (r, n) in names.iter().enumerate() { let m = MATERIALS.iter().position(|x| x == n).expect("matériau"); for k in 0..v { let l = m * v + k;
        for y in 0..s * 2 { for x in 0..s * 2 { let p = l * s * s * 4 + ((x % s) + (y % s) * s) * 4; let q = ((r * s * 2 + y) * w + k * s * 2 + x) * 3; img[q..q + 3].copy_from_slice(&t.albedo[p..p + 3]); } } } }
    let mut out = format!("P6 {w} {h} 255\n").into_bytes(); out.extend_from_slice(&img); std::fs::write(&a[1], out).unwrap();
}
