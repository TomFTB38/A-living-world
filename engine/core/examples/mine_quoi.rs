//! Ce que voient les captures « mine_entree » et « mine_galerie » : rejoue leur placement et lance des rayons.
use alw_core::{blocks::*, gen::WorldGen, karst::Way, player::{Input, Voxels}, session::*, stream::Preset, view};
fn main() { std::thread::Builder::new().stack_size(8 << 20).spawn(run).unwrap().join().unwrap(); }
fn run() {
    let scene = std::env::args().nth(1).unwrap_or("mine_entree".into()); let w = WorldGen::new(20261104); let want_mine = scene != "bouche"; let mut best: Option<(f64, [f64; 3], [f64; 3])> = None;
    for i in -12..=12 { for j in -12..=12 { if let Some(h) = w.hall(i, j) { if let Some(Way::Path { from, to, mine }) = h.way { if mine == want_mine { let d = from[0].hypot(from[2]); if best.is_none_or(|b| d < b.0) { best = Some((d, from, to)); } } } } } }
    let (_, from, to) = best.unwrap(); let (ux, uz) = (to[0] - from[0], to[2] - from[2]); let l = ux.hypot(uz); let (ux, uz) = (ux / l, uz / l);
    let (p, t) = if scene == "mine_galerie" { let a = 0.35; let fy = (from[1] - 1.0 + (to[1] - from[1] + 1.0) * a).round(); ([from[0] + ux * l * a + 0.5, fy, from[2] + uz * l * a + 0.5], [from[0] + ux * l * 0.6, fy - 4.0, from[2] + uz * l * 0.6]) }
        else { let (x, z) = (from[0] - ux * 7.0 - uz * 3.0, from[2] - uz * 7.0 + ux * 3.0); ([x, w.column(x, z).h + 1.0, z], [from[0] + ux * 4.0, from[1] - 1.0, from[2] + uz * 4.0]) };
    println!("{from:?} → {to:?} ; joueur {p:?}");
    let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 3.5, lod2: 4.5 }, 1, p);
    for _ in 0..8000 { let q = s.player.p; s.world.update(q[0], q[2]); if s.world.waiting() == 0 && s.world.loaded() > 10 { break; } std::thread::sleep(std::time::Duration::from_millis(3)); }
    s.player.p = p; s.third = false; for _ in 0..30 { s.tick(0.1, Input::default(), Actions::default()); }
    let e = s.player.eye(); println!("œil {e:?} (le joueur est tombé à {:?})", s.player.p); let f = view::norm([t[0] - e[0], t[1] - e[1], t[2] - e[2]]);
    let r = view::norm([-f[2], 0.0, f[0]]); let u = [r[1] * f[2] - r[2] * f[1], r[2] * f[0] - r[0] * f[2], r[0] * f[1] - r[1] * f[0]]; let th = (35f64).to_radians().tan();
    for j in 0..7 { let mut line = String::new(); for i in 0..9 { let (sx, sy) = (((i as f64 + 0.5) / 9.0 * 2.0 - 1.0) * th * 1.77, (1.0 - (j as f64 + 0.5) / 7.0 * 2.0) * th);
        let d = view::norm([f[0] + r[0] * sx + u[0] * sy, f[1] + r[1] * sx + u[1] * sy, f[2] + r[2] * sx + u[2] * sy]); let mut hit = "ciel".to_string(); let mut tt = 0.0;
        while tt < 60.0 { let q = [e[0] + d[0] * tt, e[1] + d[1] * tt, e[2] + d[2] * tt]; if let Some(b) = s.world.get(q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32) { if b != 0 { hit = format!("{}@{:.0}", BLOCKS[b as usize].name, tt); break; } } tt += 0.05; }
        line += &format!("{hit:>18} "); } println!("{line}"); }
}
