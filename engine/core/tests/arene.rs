//! Le Colisée oublié : le tunnel qui descend sur le sable, d'où l'on ne remonte pas ; les trois vagues de l'arène (la grille
//! retombe, les cages s'ouvrent, chaque vague quand la précédente est tombée ; qui sort ou meurt recommence) ; la herse du sud ;
//! le Champion dans l'hypogée ; la sauvegarde.
use alw_core::{arene::{self as ar, DROP}, blocks::*, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn session() -> Session { let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [2000.5, y as f64 + 2.0, 1985.5]); ready(&mut s); s }
/// Une case du Colisée : `v` compté depuis le sol (0 : où l'on se tient dehors).
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] + 1 + q[1], o[2] + q[2]] }
const OUT: [i32; 3] = [0, 0, -27]; const SAND: i32 = 1 - DROP;
fn world() -> (Session, [i32; 3]) {
    let mut s = session(); s.set_creative(false); s.hour = 12.0; let y = alw_core::gen::FLAT_Y;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [2000, y, 2000]; s.build_colosseum(o); let _ = s.sync(); put(&mut s, o, OUT); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-24, ar::CRYPT.4, -30]), at(o, [24, 14, 35])) }
fn arena(s: &Session) -> &alw_core::mecanismes::Arena { &s.mech.arenas[0] }
/// Les adversaires dans l'arène.
fn in_arena(s: &Session, o: [i32; 3]) -> usize { s.horde.foes.iter().filter(|f| f.dead == 0.0 && (f.p[0] - o[0] as f64).abs() < 13.0 && (f.p[2] - o[2] as f64).abs() < 16.0 && f.p[1] < (o[1] + 1) as f64).count() }
/// On abat tout ce qui est dans l'arène.
fn slay(s: &mut Session, o: [i32; 3]) { let me = s.player.p; for i in 0..s.horde.foes.len() { let f = &s.horde.foes[i]; if f.dead == 0.0 && (f.p[0] - o[0] as f64).abs() < 13.0 && (f.p[2] - o[2] as f64).abs() < 16.0 && f.p[1] < (o[1] + 1) as f64 { s.horde.foes[i].hp = 1.0; s.horde.strike(i, 99.0, me, "sword", false); } } run(s, 1.5); }

#[test]
fn plans_vagues_et_empreinte() {
    let w = ar::waves(); assert_eq!(w.len(), 3); assert!(w.iter().all(|v| v.len() >= 3), "des vagues de trois au moins");
    for v in &w { for (id, (u, ww)) in v { assert!(alw_core::hostile::species(id).is_some(), "{id}"); assert!(ar::within(*u, *ww, ar::SAND), "{id} sort sur le sable"); } }
    for u in -25..=25 { for ww in -25..=25 { if let Some(j) = ar::row(u, ww) { assert!(!ar::within(u, ww, ar::PODIUM) && j <= ar::ROWS); } } }
    assert_eq!(ar::floating(), Vec::<[i32; 3]>::new(), "aucune pierre du Colisée ne flotte");
    let (lo, hi) = ar::footprint(); println!("empreinte {lo:?}..{hi:?}"); assert!(lo[0] >= -38 && lo[1] >= -38 && hi[0] <= 38 && hi[1] <= 38, "le Colisée tient dans la portée des empreintes");
}

#[test]
fn tunnel_vagues_herse_et_champion() {
    let (mut s, o) = world();
    // — le tunnel du nord mène au sable ; du sable, on ne remonte ni au tunnel ni aux gradins —
    assert!(look(&s, o, OUT).contains(&at(o, [0, SAND, -12])), "le tunnel descend dans l'arène");
    assert!(!look(&s, o, [0, SAND, 0]).contains(&at(o, [0, SAND, 20])), "la herse du sud est fermée");
    assert!(name(&s, at(o, [0, SAND + 1, ar::SOUTH_GATE])).starts_with("iron_bars")); assert_eq!(name(&s, at(o, [0, SAND + 1, ar::NORTH_GATE])), "air", "l'entrée est ouverte");
    // — on entre : la grille retombe, la première vague sort après un instant —
    put(&mut s, o, [0, SAND, -12]); run(&mut s, 0.3); assert!(name(&s, at(o, [0, SAND + 1, ar::NORTH_GATE])).starts_with("iron_bars"), "la grille retombe"); assert_eq!(arena(&s).state, 1);
    let r = look(&s, o, [0, SAND, 0]); assert!(!r.contains(&at(o, OUT)), "on ne remonte pas du sable"); assert!(!r.iter().any(|c| c[1] > at(o, [0, 0, 0])[1]), "ni sur le podium, ni sur les gradins");
    assert!(s.notes.iter().any(|t| t.contains("3 vagues"))); assert_eq!(in_arena(&s, o), 0, "pas encore"); run(&mut s, 2.0); assert_eq!(in_arena(&s, o), 3, "la première vague : trois rôdeurs");
    // — qui sort recommence : les adversaires disparaissent, la grille se relève —
    put(&mut s, o, OUT); run(&mut s, 0.2); assert_eq!(arena(&s).state, 0); assert_eq!(in_arena(&s, o), 0); assert_eq!(name(&s, at(o, [0, SAND + 1, ar::NORTH_GATE])), "air");
    // — on recommence, et l'on tient : chaque vague sort quand la précédente est tombée —
    put(&mut s, o, [0, SAND, -12]); run(&mut s, 2.5);
    for (k, n) in [(1u8, 3usize), (2, 4), (3, 4)] { assert_eq!(arena(&s).state, k); assert_eq!(in_arena(&s, o), n, "la vague {k}"); slay(&mut s, o); if k < 3 { assert_eq!(in_arena(&s, o), 0, "la vague {k} est tombée, la suivante se fait attendre"); assert!(name(&s, at(o, [0, SAND + 1, ar::NORTH_GATE])).starts_with("iron_bars")); run(&mut s, alw_core::mecanismes::WAVE_PAUSE + 0.5); } }
    assert_eq!(arena(&s).state, 255, "l'arène est conquise"); assert!(s.notes.iter().any(|t| t.contains("conquise")));
    run(&mut s, 0.3); assert_eq!(name(&s, at(o, [0, SAND + 1, ar::NORTH_GATE])), "air", "la grille se relève"); assert_eq!(name(&s, at(o, [0, SAND + 1, ar::SOUTH_GATE])), "air", "la herse du sud aussi");
    let crypt = [0, ar::CRYPT.4 + 1, 26]; assert!(look(&s, o, [0, SAND, 10]).contains(&at(o, crypt)), "l'escalier mène à l'hypogée");
    // — le Champion —
    let sp = alw_core::hostile::species(ar::BOSS).unwrap(); s.horde.foes.clear(); put(&mut s, o, crypt); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("le Champion se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    assert!(name(&s, at(o, [0, SAND + 1, ar::SOUTH_GATE])).starts_with("iron_bars"), "la herse retombe derrière vous");
    s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "spear", false); run(&mut s, 1.0); assert_eq!(name(&s, at(o, [0, SAND + 1, ar::SOUTH_GATE])), "air", "vaincu : la herse se relève");
    assert!(s.loose.iter().any(|l| l.item == "couronne_laurier") || s.inv.iter().flatten().any(|st| st.item == "couronne_laurier"), "la Couronne de laurier");
    let c = at(o, ar::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin"), "le coffre des jeux"); s.close_chest();
    // la sauvegarde garde l'arène conquise
    let t = s.save_text(); let mut b = session(); b.load_text(&t).unwrap();
    let st = |s: &Session| s.mech.arenas.iter().map(|a| (a.name.clone(), a.state, a.min, a.max, a.doors.len(), a.waves.clone())).collect::<Vec<_>>(); assert_eq!(st(&b), st(&s));
}

#[test]
fn qui_meurt_dans_l_arene_recommence() {
    let (mut s, o) = world(); put(&mut s, o, [0, SAND, -12]); run(&mut s, 2.5); assert_eq!(in_arena(&s, o), 3);
    s.vit[0] = 1.0; s.harm(50.0, "essai", false); for _ in 0..30 { s.tick(DT, Input::default(), Actions::default()); }
    assert!(s.dead.is_some(), "mort"); assert_eq!(arena(&s).state, 0, "tout est à refaire"); assert_eq!(in_arena(&s, o), 0); assert_eq!(name(&s, at(o, [0, SAND + 1, ar::NORTH_GATE])), "air");
}
