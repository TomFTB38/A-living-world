//! Les villages médiévaux autour de l'origine : identité, maisons, ateliers, rayon ; et le temps de tracé.
fn main() {
    let seed: i32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(20261104); let t = std::time::Instant::now();
    let g = alw_core::gen::WorldGen::new(seed); let all = g.villages_near(0.0, 0.0, 4000.0);
    for v in &all { let d = v.ident.map(|i| i.describe()).unwrap_or_else(|| "(ancien générateur)".into());
        println!("{:<12} {:<13} {:>5},{:<5} r{:<3} maisons {:<3} artisans {:?} — {d}", v.name, v.kind, v.c[0], v.c[2], v.r, v.houses.len(), v.crafts); }
    println!("{} villages en {:.1} s", all.len(), t.elapsed().as_secs_f64());
}
