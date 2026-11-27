//! L'atlas des icônes d'objets (dans le jeu, `gpu::ICON_COLS` × `ICON_COLS` cases) doit les contenir toutes : un objet de
//! trop, et la carte graphique refusait la passe entière — plus aucune icône dans le sac ni la barre.
#[test]
fn toutes_les_icones_tiennent_dans_l_atlas() {
    let n = alw_core::session::order().len(); println!("{n} objets, 900 cases");
    assert!(n <= 900, "agrandir l'atlas des icônes (game/src/gpu.rs : ICON_COLS)");
}
