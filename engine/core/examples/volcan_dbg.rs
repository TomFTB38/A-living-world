//! Rejoue la scène de capture « sous_volcan » sans rendu : placement, 5 s de jeu, état du joueur.
use alw_core::{gen::WorldGen, karst::{Under, UNDERS}, player::*, session::*, stream::*};
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } }
fn main() {
    let key = std::env::args().nth(1).unwrap_or("volcan".into());
    let mut s = Session::new(20261104, &Preset { radius: 3, lod1: 2.3, lod2: 4.6 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s);
    let w = WorldGen::new(20261104); let kind: Under = *UNDERS.iter().find(|k| k.key() == key).unwrap();
    let h = w.nearest_hall_of(0.0, 0.0, 14, Some(kind)).unwrap();
    let target = match (h.fall, h.shaft) { (Some((f, _)), _) => [f[0] as f64 + 0.5, f[1] as f64 - 3.0, f[2] as f64 + 0.5], (None, Some(sh)) => [sh[0], h.floor as f64 + h.h * 0.55, sh[1]], _ => [h.x + h.rx * 0.4, h.floor as f64 + h.h * 0.3, h.z] };
    let (dx, dz) = (target[0] - h.x, target[2] - h.z); let l = dx.hypot(dz).max(1.0); let from = [h.x - dx / l * h.rx * 0.45, h.z - dz / l * h.rz * 0.45];
    let sp = w.hall_spot(&h, from).unwrap(); println!("salle {:.0} {:.0}, place {sp:?}", h.x, h.z);
    s.third = false; s.sel = 8; s.player.p = sp; ready(&mut s);
    for k in 0..50 { s.tick(0.1, Input::default(), Actions::default()); if k % 10 == 0 { println!("t={k} p={:?} vie={:?} mort={:?}", s.player.p, s.vit[0], s.dead.is_some()); } }
    println!("fin : vie {:?}, mort {}, sous terre {:.2}, lumières {}", s.vit[0], s.dead.is_some(), s.cave, s.lights(s.player.eye(), 64).len());
}
