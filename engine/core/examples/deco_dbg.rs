//! Sommets de déco par sorte de bloc dans un chunk : qui pèse le plus lourd.
use alw_core::{blocks::*, gen::WorldGen, mesher::{Mesher, VBuf}};
fn main() {
    let a: Vec<i32> = std::env::args().skip(1).map(|s| s.parse().unwrap()).collect(); let w = WorldGen::new(20261104); let v = w.gen_chunk(a[0], a[1]);
    let m = Mesher::new(); let mut per: std::collections::HashMap<&str, (usize, usize)> = Default::default();
    for (i, &b) in v.iter().enumerate() { if b == 0 || !is_deco(b) { continue; } let mut o = VBuf::new(64); m.deco_model(&mut o, b, 5, 5, 5, i as i32, 7, 255.0, 0.0, 0); let e = per.entry(BLOCKS[b as usize].name).or_default(); e.0 += 1; e.1 += o.n; }
    let mut l: Vec<_> = per.into_iter().collect(); l.sort_by_key(|x| std::cmp::Reverse(x.1 .1)); for (n, (c, vv)) in l.iter().take(12) { println!("{n:24} {c:7} blocs {vv:9} sommets"); }
}
