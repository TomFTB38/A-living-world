//! L'écran titre : un survol du monde (villages, biomes) qui rend le joueur tel quel et ne touche pas à la sauvegarde.
use alw_core::{player::*, session::*, stream::*};
#[test]
fn le_survol_part_revient_et_ne_deplace_pas_le_joueur() {
    let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, 90.0, 0.5]);
    let home = s.player.p; let save0 = s.save_text();
    let mut far: f64 = 0.0; let dt = 1.0 / 30.0;
    for _ in 0..(90.0 / dt) as usize { s.tour_step(dt); s.tick(dt, Input::default(), Actions::default()); let p = s.player.p; far = far.max((p[0] - home[0]).hypot(p[2] - home[2]));
        let g = alw_core::gen::WorldGen::new(20261104).column(p[0], p[2]).h; assert!(p[1] > g + 4.0, "la caméra survole le relief : {} au-dessus de {g}", p[1]); }
    assert!(far > 400.0, "elle voyage : {far:.0} blocs");
    assert_eq!(s.save_text().lines().find(|l| l.starts_with("player")), save0.lines().find(|l| l.starts_with("player")), "la sauvegarde garde la vraie place");
    s.tour_end(); assert_eq!(s.player.p, home); assert!(!s.creative && s.tour.is_none());
}
