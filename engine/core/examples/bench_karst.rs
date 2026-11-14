//! Mesure : coût du tracé d'une salle karstique, puis temps de génération des chunks qui la traversent, comparé aux autres.
use alw_core::gen::WorldGen;
fn main() {
    let w = WorldGen::new(1); let mut halls = Vec::new(); let t0 = std::time::Instant::now();
    for i in -4..4 { for j in -4..4 { let t = std::time::Instant::now(); if let Some(h) = w.hall(i, j) { halls.push(h); } let e = t.elapsed().as_secs_f64() * 1000.0; if e > 50.0 { println!("case {i},{j} : tracé {e:.0} ms"); } } }
    println!("{} salles sur 64 cases de 144 blocs ; tracé de toutes les cases : {:.0} ms", halls.len(), t0.elapsed().as_secs_f64() * 1000.0);
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for h in halls.iter().take(6) { for dz in -1..=1 { for dx in -1..=1 { let (cx, cz) = ((h.x as i32).div_euclid(32) + dx, (h.z as i32).div_euclid(32) + dz);
        let t0 = std::time::Instant::now(); let _ = w.gen_chunk(cx, cz); a.push(t0.elapsed().as_secs_f64() * 1000.0);
        let t0 = std::time::Instant::now(); let _ = w.gen_chunk(cx + 3, cz + 200); b.push(t0.elapsed().as_secs_f64() * 1000.0); } } }
    let m = |v: &Vec<f64>| v.iter().sum::<f64>() / v.len() as f64; let mx = |v: &Vec<f64>| v.iter().cloned().fold(0.0, f64::max);
    println!("chunks de salle : moyenne {:.1} ms (max {:.1}) ; autres chunks : moyenne {:.1} ms (max {:.1})", m(&a), mx(&a), m(&b), mx(&b));
}
