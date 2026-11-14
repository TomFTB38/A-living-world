//! Coupe verticale et plan d'une salle karstique : pour juger la forme sans lancer le jeu.
use alw_core::{blocks::*, gen::WorldGen};
fn main() {
    let a: Vec<String> = std::env::args().collect(); let seed: i32 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
    let w = WorldGen::new(seed); let h = w.nearest_hall(0.0, 0.0, 6).expect("aucune salle"); println!("salle {:.0} {:.0} sol {} haut {:.0} r {:.0}×{:.0} aven {:?} cascade {:?}", h.x, h.z, h.floor, h.h, h.rx, h.rz, h.shaft, h.fall);
    let (x0, z0) = (h.x as i32 - 48, h.z as i32 - 48); let mut chunks = std::collections::HashMap::new();
    let mut get = |x: i32, y: i32, z: i32| -> Id { let k = (x.div_euclid(32), z.div_euclid(32)); let v = chunks.entry(k).or_insert_with(|| w.gen_chunk(k.0, k.1)); v[idx(x.rem_euclid(32) as usize, y as usize, z.rem_euclid(32) as usize)] };
    let col = |b: Id| -> [u8; 3] { if b == 0 { [20, 20, 26] } else if is_water(b) { [40, 140, 170] } else if b == B::FLOWSTONE { [230, 215, 180] } else if BLOCKS[b as usize].name.starts_with("dripstone") { [255, 240, 120] } else if b == B::LIMESTONE { [170, 160, 140] } else if b == B::MOSSY_STONE || b == B::FERN || b == B::VINE { [70, 140, 50] } else if BLOCKS[b as usize].name.starts_with("glowcap") { [200, 80, 255] } else { [100, 96, 92] } };
    let (wd, ht) = (96usize, 100usize); let mut img = vec![[0u8; 3]; wd * ht * 2 + wd * 96];
    // coupe en x (z = centre) puis en z (x = centre), puis plan au niveau du sol + 3
    for (k, along_x) in [(0, true), (1, false)] { for i in 0..wd { for y in 0..ht { let (x, z) = if along_x { (x0 + i as i32, h.z as i32) } else { (h.x as i32, z0 + i as i32) }; img[(k * ht + (ht - 1 - y)) * wd + i] = col(get(x, y as i32, z)); } } }
    for j in 0..96 { for i in 0..wd { img[(2 * ht + j) * wd + i] = col(get(x0 + i as i32, h.floor + 3, z0 + j as i32)); } }
    let (wi, hi) = (wd, ht * 2 + 96); let mut out = format!("P6 {wi} {hi} 255\n").into_bytes(); for p in &img { out.extend_from_slice(p); } std::fs::write(&a[1], out).unwrap();
}
