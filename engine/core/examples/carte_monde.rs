//! Carte du monde par biomes (un pixel = `pas` blocs) : vérifie la taille des mers et des océans, et la côte.
use alw_core::{biome::*, gen::*};
fn main() {
    let a: Vec<String> = std::env::args().collect(); let pas: f64 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(32.0); let n = 500usize; let g = WorldGen::new(20261104);
    let col = [[60, 120, 50], [140, 175, 80], [30, 80, 35], [70, 100, 80], [180, 160, 90], [225, 205, 140], [130, 120, 95], [235, 235, 240], [70, 140, 200], [25, 55, 130]];
    let mut img = vec![0u8; n * n * 3]; let mut cnt = [0usize; COUNT];
    for j in 0..n { for i in 0..n { let (x, z) = ((i as f64 - n as f64 / 2.0) * pas, (j as f64 - n as f64 / 2.0) * pas); let c = g.column(x, z); let b = c.mix.main(); cnt[b] += 1;
        let mut k = col[b]; if c.water > 0.0 && b < 8 { k = [80, 150, 210]; } if c.mix.sea > 0.0 && c.water == 0.0 && c.h < SEA + 3.0 { k = [240, 220, 160]; }
        let sh = if c.water > 0.0 { 1.0 - (c.water - c.h) / 60.0 } else { 0.75 + (c.h - 50.0) / 160.0 }; for q in 0..3 { img[(i + j * n) * 3 + q] = (k[q] as f64 * sh).clamp(0.0, 255.0) as u8; } } }
    for (b, c) in cnt.iter().enumerate() { println!("{:18} {:5.1} %", NAMES[b], *c as f64 * 100.0 / (n * n) as f64); }
    let mut out = format!("P6 {n} {n} 255\n").into_bytes(); out.extend(img); std::fs::write(&a[1], out).unwrap();
}
