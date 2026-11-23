//! Les feuilles qui tombent : sous les érables d'une forêt d'automne, des feuilles rouges et orangées ; sous les cerisiers,
//! des pétales ; elles tombent, se posent, s'effacent ; ailleurs, rien.
use alw_core::{feuilles::{self, Sorte}, gen::*, player::*, session::*, stream::*};
const SEED: i32 = 20261104;
const DT: f64 = 1.0 / 60.0;
fn ready(s: &mut Session) { for _ in 0..20000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn core(g: &WorldGen, want: Rare) -> (f64, f64) {
    for i in -90..=90 { for j in -90..=90 { let (x, z) = (i as f64 * 48.0, j as f64 * 48.0); if let Some((k, s)) = g.rare_at(x, z) { let c = g.column(x, z); if k == want && s > 0.99 && c.water == 0.0 && c.h > SEA + 3.0 { return (x, z); } } } }
    panic!("pas de {want:?}")
}
fn run(want: Rare) -> Session {
    let g = WorldGen::new(SEED); let (x, z) = core(&g, want);
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [x + 0.5, 140.0, z + 0.5]); ready(&mut s); s.set_creative(true); s.player.fly = true;
    s.player.p = [x + 0.5, g.column(x, z).h.floor() + 2.0, z + 0.5]; s.player.v = [0.0; 3]; s
}

#[test]
fn feuilles_d_automne() {
    let mut s = run(Rare::Automne); let p0 = s.player.p;
    for _ in 0..180 { s.player.p = p0; s.tick(DT, Input::default(), Actions::default()); }
    let n = s.leaves_fall.len(); let kinds: std::collections::BTreeSet<String> = s.leaves_fall.iter().map(|f| format!("{:?}", f.kind)).collect();
    println!("après 3 s : {n} feuilles, {kinds:?}"); assert!(n > 25, "des feuilles tombent : {n}"); assert!(kinds.contains("Rouge") || kinds.contains("Orange"), "des feuilles d'érable");
    assert!(s.leaves_fall.iter().all(|f| f.kind != Sorte::Petale), "pas de pétales sous les érables");
    // elles tombent, en tourbillonnant, et se posent
    let y0: Vec<f64> = s.leaves_fall.iter().map(|f| f.p[1]).collect(); for _ in 0..60 { s.player.p = p0; s.tick(DT, Input::default(), Actions::default()); }
    assert!(s.leaves_fall.iter().zip(&y0).filter(|(f, y)| f.rest.is_none() && f.p[1] < **y - 0.3).count() > 5, "elles descendent");
    for _ in 0..600 { s.player.p = p0; s.tick(DT, Input::default(), Actions::default()); }
    let landed = s.leaves_fall.iter().filter(|f| f.rest.is_some()).count(); println!("{landed} posées sur {}", s.leaves_fall.len()); assert!(landed > 10, "elles se posent au sol");
    for f in s.leaves_fall.iter().filter(|f| f.rest.is_some()) { let below = s.world.get(f.p[0].floor() as i32, (f.p[1] - 0.5).floor() as i32, f.p[2].floor() as i32).unwrap_or(0); assert!(alw_core::blocks::is_solid(below) || alw_core::blocks::is_water(below), "posée sur du solide"); }
    assert!(s.leaves_fall.len() <= feuilles::MAX);
    let md = alw_core::actors::Models::build(); assert_eq!(s.leaves_draws(&md).len(), s.leaves_fall.len(), "chacune se dessine"); for p in ["leaf_red", "leaf_orange", "leaf_gold", "petal"] { assert!(md.part(p).count >= 36, "{p}"); }
}

#[test]
fn petales_de_cerisier_et_rien_ailleurs() {
    let mut s = run(Rare::Cerisiers); let p0 = s.player.p; for _ in 0..180 { s.player.p = p0; s.tick(DT, Input::default(), Actions::default()); }
    println!("{} pétales", s.leaves_fall.len()); assert!(s.leaves_fall.len() > 15 && s.leaves_fall.iter().all(|f| f.kind == Sorte::Petale), "des pétales");
    // dans le monde plat, ni feuilles ni pétales
    let mut f = Session::new(FLAT | 9, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, [0.5, FLAT_Y as f64 + 1.0, 0.5]); ready(&mut f); for _ in 0..120 { f.tick(DT, Input::default(), Actions::default()); } assert!(f.leaves_fall.is_empty());
}
