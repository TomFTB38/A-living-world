//! Répartition des biomes souterrains et de leurs accès sur une grande région (graine du jeu), et la plus proche de chaque.
use alw_core::{gen::WorldGen, karst::*};
fn main() {
    let seed: i32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(20261104); let r: i32 = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(14);
    let w = WorldGen::new(seed); let mut n = std::collections::BTreeMap::new(); let (mut sh, mut mine, mut mouth, mut fall, mut tot) = (0, 0, 0, 0, 0); let t0 = std::time::Instant::now();
    for i in -r..r { for j in -r..r { if let Some(h) = w.hall(i, j) { tot += 1; *n.entry(h.kind.key()).or_insert(0) += 1; if h.shaft.is_some() { sh += 1; } if h.fall.is_some() { fall += 1; }
        if let Some(Way::Path { mine: m, .. }) = h.way { if m { mine += 1; } else { mouth += 1; } } } } }
    println!("{tot} salles sur {} cases ({:.0} ms) : avens {sh}, cascades {fall}, bouches {mouth}, mines {mine}", 4 * r * r, t0.elapsed().as_secs_f64() * 1000.0);
    for (k, v) in &n { println!("  {k:12} {v}"); }
    for k in UNDERS { match w.nearest_hall_of(0.0, 0.0, r, Some(k)) { Some(h) => println!("{:12} la plus proche à {:5.0} {:5.0} ({:.0} blocs) sol {} haut {:.0}", k.key(), h.x, h.z, h.x.hypot(h.z), h.floor, h.h), None => println!("{:12} aucune", k.key()) } }
}
