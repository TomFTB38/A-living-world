use alw_core::stream::*;
fn main() { let a: Vec<f64> = std::env::args().skip(1).map(|s| s.parse().unwrap()).collect(); let (x, z) = (a[0], a[1]);
    let mut st = Streamer::new(20261104, &Preset { radius: 4, lod1: 2.3, lod2: 4.6 }, 4); let mut all = Vec::new();
    for _ in 0..5000 { let (b, _) = st.update(x, z); all.extend(b); if st.waiting() == 0 && st.loaded() > 0 { break; } std::thread::sleep(std::time::Duration::from_millis(3)); }
    let vs = alw_core::mesher::VSIZE; let mut bad = 0; let mut maxl = (0i16, (0, 0)); let mut big = (0, (0, 0), 0u8);
    for b in &all { let m = &b.mesh; for (buf, n, w) in [(&m.opaque, m.n_o, "o"), (&m.deco, m.n_d, "d"), (&m.water, m.n_w, "w")] {
        if n > big.0 { big = (n, b.key, b.lod); }
        if buf.len() != n * vs || n % 4 != 0 { println!("{:?} lod {} {w}: n {n} octets {}", b.key, b.lod, buf.len()); bad += 1; }
        for v in buf.chunks(vs) { let l = i16::from_le_bytes([v[6], v[7]]); if l > maxl.0 { maxl = (l, b.key); } }
    } }
    for b in &all { eprintln!("par_chunk {:?} {} {} {} {}", b.key, b.lod, b.mesh.n_o, b.mesh.n_d, b.mesh.n_w); } let tot: usize = all.iter().map(|b| b.mesh.n_o + b.mesh.n_d + b.mesh.n_w).sum(); println!("couche max {:?} ; {} maillages, {bad} bancals, le plus gros {:?}, total {} M sommets, {} Mo", maxl, all.len(), big, tot / 1_000_000, tot * vs / 1_000_000);
}
fn half(_h: u16) -> f32 { 0.0 }
