//! Rejoue hors du jeu la scène de capture « karst_aven » (salle à aven la plus proche), avec la pile de 1 Mo du fil principal sous Windows.
use alw_core::{gen::WorldGen, player::Input, session::*, stream::Preset};
fn main() { std::thread::Builder::new().stack_size(1 << 20).spawn(run).unwrap().join().unwrap(); }
fn run() {
    let w = WorldGen::new(20261104); let mut best: Option<(f64, alw_core::karst::Hall)> = None;
    for i in -8..=8 { for j in -8..=8 { if let Some(h) = w.hall(i, j) { if h.shaft.is_none() || h.fall.is_none() { continue; } let d = h.x.hypot(h.z); if best.is_none_or(|b| d < b.0) { best = Some((d, h)); } } } }
    let h = best.unwrap().1; let sh = h.shaft.unwrap(); let target = [sh[0], 0.0, sh[1]]; let (dx, dz) = (target[0] - h.x, target[2] - h.z); let l = dx.hypot(dz).max(1.0);
    let from = [h.x - dx / l * h.rx * 0.5, h.z - dz / l * h.rz * 0.5]; let sp = w.hall_spot(&h, from).unwrap(); println!("spot {sp:?}");
    let mut s = Session::new(20261104, &Preset { radius: 4, lod1: 5.5, lod2: 6.5 }, 1, sp);
    for _ in 0..8000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 20 { break; } std::thread::sleep(std::time::Duration::from_millis(3)); }
    s.player.p = sp; for _ in 0..50 { s.tick(0.1, Input::default(), Actions::default()); }
    println!("lumières {}", s.lights(s.eye, 48).len());
    let md = alw_core::actors::Models::build(); let _ = md; println!("ok");
}
