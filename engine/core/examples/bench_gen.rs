//! Mesure : temps de génération des chunks (terrain seul), sur une grille autour de l'origine, graine fixe.
use alw_core::gen::WorldGen;
fn main() {
    let seed: i32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(1); let r: i32 = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(6);
    let w = WorldGen::new(seed); let _ = w.gen_chunk(0, 0);
    let mut t = Vec::new(); let mut air = 0usize; let total = std::time::Instant::now();
    for cz in -r..r { for cx in -r..r { let t0 = std::time::Instant::now(); let v = w.gen_chunk(cx, cz); t.push(t0.elapsed().as_secs_f64() * 1000.0);
        air += v.iter().enumerate().filter(|(i, b)| **b == 0 && (i / (32 * 32)) < 45).count(); } }
    t.sort_by(|a, b| a.total_cmp(b)); let n = t.len(); let mean = t.iter().sum::<f64>() / n as f64;
    println!("{n} chunks : moyenne {mean:.2} ms, médiane {:.2} ms, p95 {:.2} ms, max {:.2} ms, total {:.2} s ; vides sous y=45 : {air}", t[n / 2], t[n * 95 / 100], t[n - 1], total.elapsed().as_secs_f64());
}
