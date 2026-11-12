//! Hautes montagnes : des pics sous le plafond du monde, neige et glace aux sommets, alpages et épicéas en bas, bouquetins.
use alw_core::{biome::*, blocks::*, gen::WorldGen};
const SEED: i32 = 20261104;
fn coeur(g: &WorldGen) -> (f64, f64) { for r in 8..900 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); if g.column(x, z).mix.strong(Biome::Mountains) > 0.97 { return (x, z); } } } panic!("pas de hautes montagnes") }
#[test]
fn les_montagnes_dressent_des_pics() {
    let g = WorldGen::new(SEED); let (x, z) = coeur(&g); let (mut lo, mut hi) = (f64::MAX, 0.0f64);
    for i in -40..40 { for j in -40..40 { let h = g.column(x + i as f64 * 5.0, z + j as f64 * 5.0).h; lo = lo.min(h); hi = hi.max(h); } }
    println!("altitudes {lo:.0} à {hi:.0}"); assert!(hi > 104.0, "pas de vrai pic : {hi}"); assert!(hi < CH as f64 - 8.0, "les sommets touchent le plafond"); assert!(hi - lo > 35.0, "relief trop plat : {lo}..{hi}");
}
#[test]
fn neige_glace_alpages_et_epiceas() {
    let g = WorldGen::new(SEED); let (x, z) = coeur(&g); let (cx, cz) = ((x / CX as f64).floor() as i32, (z / CX as f64).floor() as i32);
    let mut n = std::collections::HashMap::<&str, usize>::new();
    for dz in -4..=4 { for dx in -4..=4 { let v = g.gen_chunk(cx + dx, cz + dz); for b in v { let k = match b { b if b == B::SNOW => "neige", b if b == B::ICE => "glace", b if b == B::GRASS => "alpage", b if b == B::GRAVEL => "eboulis", b if b == B::SPRUCE_LOG => "epicea", b if b == B::LICHEN_STONE => "lichen", b if is_water(b) => "eau", _ => continue }; *n.entry(k).or_default() += 1; } } }
    println!("{n:?}"); for k in ["neige", "glace", "alpage", "eboulis", "epicea"] { assert!(n.get(k).copied().unwrap_or(0) > 20, "il manque {k} : {n:?}"); }
}
#[test]
fn le_biome_porte_son_nom_et_son_danger() {
    assert_eq!(NAMES[Biome::Mountains as usize], "Hautes montagnes"); assert_eq!(alw_core::hostile::biome_danger(Biome::Mountains as usize, 80.0), 5);
}
