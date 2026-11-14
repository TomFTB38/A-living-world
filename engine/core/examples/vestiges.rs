//! Les vestiges souterrains autour de l'origine : combien par biome, et le relief autour d'une salle (diagnostic).
use alw_core::{gen::WorldGen, karst::*};
fn main() {
    let w = WorldGen::new(20261104); let mut per: std::collections::BTreeMap<&str, (usize, usize)> = Default::default();
    for i in -14..=14 { for j in -14..=14 { let Some(h) = w.hall(i, j) else { continue }; let e = per.entry(h.kind.name()).or_default(); e.0 += 1; if let Some(st) = w.hall_site(&h) { e.1 += 1; if std::env::args().nth(1).is_some() { println!("{:?} {} à {} {} {}", st.kind, h.kind.name(), st.x, st.fl, st.z); } } } }
    for (k, (n, s)) in per { println!("{k:28} {n:4} salles, {s:3} vestiges"); }
}
