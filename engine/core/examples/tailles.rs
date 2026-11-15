//! Ce que l'atelier de taille tire d'un bloc : `tailles stone planks oak_log …`.
fn main() { for a in std::env::args().skip(1) { let v = alw_core::play::cut_variants(&a); println!("{a} ({}) : {}", v.len(), v.iter().map(|c| format!("{}×{}←{}", c.out, c.n, c.need)).collect::<Vec<_>>().join(", ")); } }
