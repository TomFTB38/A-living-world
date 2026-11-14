//! Taille et bornes des maillages autour d'un point : pour chercher un maillage hors norme.
use alw_core::stream::*;
fn main() {
    let a: Vec<f64> = std::env::args().skip(1).map(|s| s.parse().unwrap()).collect(); let (x, z) = (a[0], a[1]);
    let mut st = Streamer::new(20261104, &Preset { radius: 3, lod1: 2.3, lod2: 4.6 }, 4); let mut all = Vec::new();
    for _ in 0..3000 { let (b, _) = st.update(x, z); all.extend(b); if st.waiting() == 0 && st.loaded() > 0 { break; } std::thread::sleep(std::time::Duration::from_millis(3)); }
    let (mut tot, mut worst) = ([0usize; 3], (0usize, (0, 0), 0u8)); let mut bad = 0;
    for b in &all { let m = &b.mesh; tot[0] += m.n_o; tot[1] += m.n_d; tot[2] += m.n_w; let n = m.n_o + m.n_d + m.n_w; if n > worst.0 { worst = (n, b.key, b.lod); }
        for buf in [&m.opaque, &m.deco, &m.water] { for v in buf.chunks(20) { let l = i16::from_le_bytes([v[6], v[7]]); let nn = [v[8] as i8, v[9] as i8, v[10] as i8]; if l < 0 || (nn == [0, 0, 0]) { bad += 1; } } } }
    println!("{} maillages ; sommets opaques {} déco {} eau {} ; le plus gros {:?} ; sommets suspects {bad}", all.len(), tot[0], tot[1], tot[2], worst);
}
