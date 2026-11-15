//! Les lieux de surface autour du départ : combien de chaque sorte, et où (diagnostic).
use alw_core::gen::WorldGen;
fn main() { let w = WorldGen::new(20261104); let v = w.places_near(-38.0, 0.0, 2500.0); let mut n = std::collections::BTreeMap::new(); for p in &v { *n.entry(p.kind.name()).or_insert(0) += 1; }
    println!("{} lieux à moins de 2500 blocs : {n:?}", v.len()); for p in v.iter().take(12) { println!("{:>24} à {:>6} {:>4} {:>6}  ({:.0} blocs)", p.kind.name(), p.x, p.y, p.z, ((p.x + 38) as f64).hypot(p.z as f64)); } }
