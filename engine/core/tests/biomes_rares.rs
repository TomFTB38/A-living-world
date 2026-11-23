//! Les biomes rares : des taches de cerisiers en fleur et de forêts d'automne au milieu des pays tempérés, avec leurs
//! arbres, leur tapis de pétales ou de feuilles mortes ; on les découvre une fois, pour de l'expérience.
use alw_core::{biome::Biome, blocks::*, gen::*, player::*, session::*, stream::*};
const SEED: i32 = 20261104;
const DT: f64 = 1.0 / 60.0;
fn ready(s: &mut Session) { for _ in 0..20000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
/// Le cœur de la première tache de chaque sorte trouvée en balayant le monde.
fn cores(g: &WorldGen) -> Vec<(Rare, f64, f64)> {
    let mut o: Vec<(Rare, f64, f64)> = Vec::new();
    for i in -90..=90 { for j in -90..=90 { let (x, z) = (i as f64 * 48.0, j as f64 * 48.0); if let Some((k, s)) = g.rare_at(x, z) { if s > 0.99 && !o.iter().any(|q| q.0 == k) && g.column(x, z).water == 0.0 { o.push((k, x, z)); } } } }
    o
}

#[test]
fn des_taches_rares_aux_pays_temperes() {
    let g = WorldGen::new(SEED); let (mut n, mut tot, mut kinds) = (0, 0, std::collections::BTreeSet::new());
    for i in -60..=60 { for j in -60..=60 { let (x, z) = (i as f64 * 64.0, j as f64 * 64.0); tot += 1; if let Some((k, s)) = g.rare_at(x, z) { if s > 0.5 { n += 1; kinds.insert(format!("{k:?}"));
        let m = g.climate.at(x, z); for b in [Biome::Desert, Biome::Steppe, Biome::Taiga, Biome::Mountains] { assert!(m.strong(b) < 0.6, "{k:?} jamais en {b:?}"); } } } } }
    let part = n as f64 / tot as f64; println!("biomes rares : {:.1} % du pays, sortes {kinds:?}", part * 100.0);
    assert!(part > 0.002 && part < 0.06, "rares, mais on en trouve : {:.2} %", part * 100.0); assert_eq!(kinds.len(), 2, "les deux sortes existent");
    assert!(WorldGen::new(FLAT | 3).rare_at(0.0, 0.0).is_none());
}

#[test]
fn leurs_arbres_et_leur_tapis() {
    let g = WorldGen::new(SEED); let c = cores(&g); assert_eq!(c.len(), 2, "un cœur de chaque sorte : {c:?}");
    for (k, x, z) in c {
        let mut count = std::collections::BTreeMap::<&str, usize>::new();
        for dx in -3..=3 { for dz in -3..=3 { let (cx, cz) = ((x as i32).div_euclid(CX as i32) + dx, (z as i32).div_euclid(CX as i32) + dz); let v = g.gen_world(cx, cz); for b in v.iter() { let n = BLOCKS[*b as usize].name; if ["cherry_leaves", "petals", "maple_leaves_red", "maple_leaves_orange", "fallen_leaves", "oak_leaves", "birch_leaves", "beech_leaves"].contains(&n) { *count.entry(n).or_default() += 1; } } } }
        println!("{k:?} à ({x:.0}, {z:.0}) : {count:?}");
        let get = |n: &str| *count.get(n).unwrap_or(&0);
        match k { Rare::Cerisiers => { assert!(get("cherry_leaves") > 1500 && get("petals") > 1000, "des cerisiers et leurs pétales"); assert!(get("oak_leaves") + get("birch_leaves") < get("cherry_leaves") / 4, "presque que des cerisiers"); }
            Rare::Automne => { assert!(get("maple_leaves_red") > 800 && get("maple_leaves_orange") > 600 && get("fallen_leaves") > 1000, "des érables rouges et orangés, des feuilles mortes"); } }
    }
}

#[test]
fn on_les_decouvre() {
    let g = WorldGen::new(SEED); let (k, x, z) = cores(&g)[0];
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [x + 0.5, 140.0, z + 0.5]); ready(&mut s); s.set_creative(false); s.prog.xp = 0.0;
    s.player.p[1] = g.column(x, z).h.floor() + 1.0; for _ in 0..90 { s.tick(DT, Input::default(), Actions::default()); }
    assert!(s.prog.biomes & k.bit() != 0, "découvert"); assert!(s.notes.iter().any(|n| n.contains(k.name())), "annoncé : {:?}", s.notes); let xp = s.prog.xp; assert!(xp > 0.0);
    for _ in 0..60 { s.tick(DT, Input::default(), Actions::default()); } assert_eq!(s.prog.xp, xp, "une seule fois");
    let t = s.save_text(); let mut l = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 120.0, 0.5]); l.load_text(&t).unwrap(); assert!(l.prog.biomes & k.bit() != 0, "sauvé");
    // la carte croquée d'un point de vue les montre de loin : rose ou rousse
    let rgb = |v: u32| [(v >> 24) as u8 as i32, (v >> 16) as u8 as i32, (v >> 8) as u8 as i32]; let c = rgb(alw_core::panoramas::sketch(&g, x, z));
    match k { Rare::Cerisiers => assert!(c[0] > c[1] + 30 && c[2] > c[1], "rose : {c:?}"), Rare::Automne => assert!(c[0] > c[1] + 40 && c[1] > c[2], "roux : {c:?}") }
}
