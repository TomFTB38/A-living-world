use alw_core::{blocks::*, felling::find_tree, player::Voxels};
use std::collections::HashMap;
struct W(HashMap<(i32, i32, i32), u8>);
impl Voxels for W { fn get(&self, x: i32, y: i32, z: i32) -> Option<u8> { Some(if y < 10 { B::DIRT } else { *self.0.get(&(x, y, z)).unwrap_or(&0) }) } }
#[test]
fn abattre_un_arbre_n_emporte_pas_son_voisin() {
    // Deux chênes à 6 blocs l'un de l'autre, reliés par une grosse branche et un feuillage commun.
    let mut m = HashMap::new(); for y in 10..22 { m.insert((0, y, 0), B::OAK_LOG); m.insert((6, y, 0), B::OAK_LOG); } for x in 1..6 { m.insert((x, 18, 0), B::OAK_LOG_X); }
    for x in -3..10 { for z in -3..4 { for y in 22..25 { m.insert((x, y, z), B::OAK_LEAVES); } } }
    let t = find_tree(&W(m), 0, 11, 0).expect("un arbre"); let theirs = t.logs.iter().filter(|p| p.0 == 6 && p.1 < 18).count();
    println!("{} bûches, dont {theirs} du tronc voisin", t.logs.len()); assert_eq!(theirs, 0, "le tronc voisin reste debout");
}
