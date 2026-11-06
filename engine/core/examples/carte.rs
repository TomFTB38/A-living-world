//! Carte des biomes, vue de dessus, pour juger formes et transitions. a-living-world n'en dépend pas.
use alw_core::biome::*; use std::io::Write;
fn main() {
    const COL: [[f64; 3]; COUNT] = [[60., 130., 60.], [170., 215., 90.], [20., 70., 45.], [190., 215., 235.], [205., 165., 80.], [240., 215., 150.]];
    for seed in [20261104, 7] { let c = Climate::new(seed); let (n, st) = (900, 14.0); let mut px = vec![0u8; n * n * 3]; let mut part = [0usize; COUNT];
        for j in 0..n { for i in 0..n { let (x, z) = ((i as f64 - 450.0) * st, (j as f64 - 450.0) * st); let m = c.at(x, z); part[m.main()] += 1; let mut o = [0.0; 3]; for b in 0..COUNT { for k in 0..3 { o[k] += COL[b][k] * m.w[b]; } } if x.hypot(z) < 40.0 { o = [255.0, 0.0, 0.0]; } for k in 0..3 { px[(i + j * n) * 3 + k] = o[k] as u8; } } }
        let mut f = std::fs::File::create(format!("/tmp/claude-0/s/carte{}.ppm", if seed == 7 { 7 } else { 0 })).unwrap(); write!(f, "P6 {n} {n} 255\n").unwrap(); f.write_all(&px).unwrap();
        for b in 0..COUNT { println!("{seed} {:<16} {:.1} %", NAMES[b], part[b] as f64 / (n * n) as f64 * 100.0); } }
}
