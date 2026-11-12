//! Taille du tampon des modèles (sommets), et les pièces les plus lourdes, pour surveiller le poids des bêtes sculptées.
fn main() {
    let m = alw_core::actors::Models::build(); println!("{} sommets, {} Mo", m.data.len() / 20, m.data.len() / 1_000_000);
    let mut v: Vec<(String, u32)> = m.parts.iter().map(|(k, r)| (k.to_string(), r.count / 6 * 4)).collect(); v.sort_by(|a, b| b.1.cmp(&a.1));
    let tot = |p: &str| v.iter().filter(|x| x.0.starts_with(p)).map(|x| x.1 as u64).sum::<u64>();
    for p in ["folk", "folkHead", "folkArm", "folkLeg", "deer", "bear", "wolf", "boar", "fox", "hare", "ibex", "dolphin", "shark", "whale", "rodeur", "brigand"] { println!("{p:10} {}", tot(p)); }
    for (k, n) in v.iter().filter(|x| !x.0.starts_with("folk")).take(30) { println!("{k:20} {n}"); }
}
