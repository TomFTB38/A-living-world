//! Le monde plat, pour construire : une prairie sans relief, sans villages, et la galerie de tous les bâtiments.
use alw_core::{blocks::*, gen::{is_flat, WorldGen, FLAT, FLAT_Y}};
#[test]
fn monde_plat_et_galerie() {
    let seed = FLAT | 0x123; assert!(is_flat(seed)); assert!(!is_flat(20261104));
    let g = WorldGen::new(seed); assert!(g.flat);
    // loin de la galerie : de l'herbe à la même hauteur partout, rien au-dessus
    for (cx, cz) in [(-20, 9), (30, -40), (100, 100)] { let v = g.gen_chunk(cx, cz); for x in [0usize, 7, 15] { for z in [0usize, 9, 15] {
        assert_eq!(v[idx(x, FLAT_Y as usize, z)], B::GRASS); assert_eq!(v[idx(x, FLAT_Y as usize + 1, z)], 0); assert_eq!(v[idx(x, FLAT_Y as usize - 1, z)], B::DIRT); } } }
    assert!(g.villages_near(0.0, 0.0, 2000.0).is_empty(), "pas de village dans un monde plat");
    // la galerie : des portes, des fenêtres, des pentes de toit, un clocher
    let names: std::collections::HashSet<&str> = g.gallery().iter().map(|b| BLOCKS[b.3 as usize].name).collect();
    for n in ["door", "window", "roof_tiles_slope", "slate_slope", "bell", "anvil", "tanning_vat", "sawhorse", "hay_bale"] { assert!(names.iter().any(|x| x.starts_with(n)), "{n} dans la galerie"); }
    let span = g.gallery().iter().map(|b| b.0).max().unwrap(); assert!(span > 150, "la galerie s'étend sur {span} blocs");
    let v = g.gen_chunk(0, 0); assert!((0..CX).any(|x| (0..CX).any(|z| v[idx(x, FLAT_Y as usize + 1, z)] != 0)), "des bâtiments près de l'arrivée");
}
