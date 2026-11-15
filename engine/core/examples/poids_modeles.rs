//! Combien pèsent les modèles animés : total, et les plus lourds.
fn main() { let m = alw_core::actors::Models::build(); let mut v: Vec<(&str, u32)> = m.parts.iter().map(|(k, r)| (*k, r.count / 6)).collect(); v.sort_by_key(|x| std::cmp::Reverse(x.1));
    println!("total {} quads, {} Mo", m.n / 4, m.data.len() / 1_000_000); for (k, n) in v.iter().take(25) { println!("{n:>8} {k}"); } }
