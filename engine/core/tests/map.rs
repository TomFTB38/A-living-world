//! Carte : on découvre le terrain autour de soi, dans sa couleur ; la carte se sauvegarde ; la grande carte se déplie.
use alw_core::{blocks::*, map::*, player::*, session::*, stream::*};
const SEED: i32 = 20261104;
const DT: f64 = 1.0 / 30.0;
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(5)); } }
fn idle(s: &mut Session, sec: f64) { for _ in 0..(sec / DT) as usize { s.tick(DT, Input::default(), Actions::default()); } }

#[test]
fn on_decouvre_ce_qu_on_voit() {
    let mut s = Session::new(SEED, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [0.5, 120.0, 0.5]); ready(&mut s); idle(&mut s, 1.0);
    let r = REVEAL / STEP; let disc = (std::f64::consts::PI * (r * r) as f64) as usize;
    let n = s.map.explored(); assert!(n * 10 >= disc * 9, "{n} pixels découverts sur {disc}");
    assert_eq!(s.map.get(400, 400), 0, "rien de vu au loin");
    // Le pixel sous les pieds a la couleur du bloc du dessus et sa hauteur.
    let (px, pz) = ((s.player.p[0] / STEP as f64).floor() as i32, (s.player.p[2] / STEP as f64).floor() as i32); let (x, z) = (px * STEP + STEP / 2, pz * STEP + STEP / 2);
    let (c, h) = unpack(s.map.get(px, pz)).expect("découvert");
    let mut y = CH as i32 - 1; while y > 0 && !s.world.get(x, y, z).is_some_and(|b| b != 0 && (is_solid(b) || is_water(b))) { y -= 1; }
    let b = s.world.get(x, y, z).unwrap(); assert_eq!(h as i32, y);
    if !is_water(b) { let p = palette()[b as usize]; assert_eq!([c[0], c[1], c[2] | 1], [p[0], p[1], p[2] | 1], "couleur de {}", BLOCKS[b as usize].name); }
    // Marcher plus loin découvre du terrain neuf.
    s.player.fly = true; for _ in 0..40 { s.player.p[0] += 3.0; let p = s.player.p; s.world.update(p[0], p[2]); s.tick(DT, Input::default(), Actions::default()); } ready(&mut s); idle(&mut s, 1.0);
    assert!(s.map.explored() > n + disc / 2, "{} après la marche", s.map.explored());
    // Tout revient au chargement.
    let t = s.save_text(); let mut b = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 120.0, 0.5]); b.load_text(&t).unwrap();
    assert_eq!(b.map.regions, s.map.regions);
}

#[test]
fn la_carte_se_deplie() {
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 120.0, 0.5]);
    s.toggle_map(); assert!(s.map.open); idle(&mut s, 0.2); assert!(s.map.anim > 0.2 && s.map.anim < 0.9, "en train de se déplier : {}", s.map.anim);
    idle(&mut s, 0.6); assert_eq!(s.map.anim, 1.0); s.zoom_map(1); s.zoom_map(9); assert_eq!(s.map.zoom, ZOOMS.len() - 1); s.zoom_map(-9); assert_eq!(s.map.zoom, 0);
    s.toggle_map(); idle(&mut s, 0.6); assert_eq!(s.map.anim, 0.0);
}
