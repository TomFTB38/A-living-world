//! Quels blocs voit-on sur la capture « karst_salle » ? Rejoue la scène et lance un rayon par case d'une grille sur l'image.
use alw_core::{blocks::*, gen::WorldGen, player::{Input, Voxels}, session::*, stream::Preset, view};
fn main() { std::thread::Builder::new().stack_size(8 << 20).spawn(run).unwrap().join().unwrap(); }
fn run() {
    let fovy: f64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(70.0);
    let w = WorldGen::new(20261104); let mut best: Option<(f64, alw_core::karst::Hall)> = None;
    for i in -8..=8 { for j in -8..=8 { if let Some(h) = w.hall(i, j) { if h.fall.is_none() { continue; } let d = h.x.hypot(h.z); if best.is_none_or(|b| d < b.0) { best = Some((d, h)); } } } }
    let h = best.unwrap().1; let (fall, _) = h.fall.unwrap(); let target = [fall[0] as f64 + 0.5, fall[1] as f64 - 3.0, fall[2] as f64 + 0.5];
    let (dx, dz) = (target[0] - h.x, target[2] - h.z); let l = dx.hypot(dz).max(1.0); let from = [h.x - dx / l * h.rx * 0.5, h.z - dz / l * h.rz * 0.5];
    let sp = w.hall_spot(&h, from).unwrap(); let mut s = Session::new(20261104, &Preset { radius: 3, lod1: 4.5, lod2: 5.5 }, 1, sp);
    for _ in 0..8000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 20 { break; } std::thread::sleep(std::time::Duration::from_millis(3)); }
    s.player.p = sp; s.third = false; for _ in 0..50 { s.tick(0.1, Input::default(), Actions::default()); }
    let e = s.player.eye(); let f = view::norm([target[0] - e[0], target[1] - e[1], target[2] - e[2]]);
    let r = view::norm([-f[2], 0.0, f[0]]); let u = [r[1] * f[2] - r[2] * f[1], r[2] * f[0] - r[0] * f[2], r[0] * f[1] - r[1] * f[0]];
    let (th, aspect) = ((fovy.to_radians() / 2.0).tan(), 16.0 / 9.0);
    println!("œil {e:?} regard {f:?}");
    let (cols, rows) = (16, 9);
    for j in 0..rows { let mut line = String::new(); for i in 0..cols {
        let (sx, sy) = (((i as f64 + 0.5) / cols as f64 * 2.0 - 1.0) * th * aspect, (1.0 - (j as f64 + 0.5) / rows as f64 * 2.0) * th);
        let mut d = [f[0] + r[0] * sx + u[0] * sy, f[1] + r[1] * sx + u[1] * sy, f[2] + r[2] * sx + u[2] * sy]; for k in &mut d { *k = *k; } let d = view::norm(d);
        let mut hit = "—".to_string(); let mut t = 0.0; while t < 80.0 { let p = [e[0] + d[0] * t, e[1] + d[1] * t, e[2] + d[2] * t]; if let Some(b) = s.world.get(p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32) { if b != 0 { hit = format!("{}@{:.0}", BLOCKS[b as usize].name, t); break; } } t += 0.05; }
        line += &format!("{hit:>22} "); } println!("{line}"); }
}
