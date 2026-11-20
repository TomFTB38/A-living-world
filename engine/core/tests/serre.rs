//! La Serre empoisonnée : les spores brûlent tant que les deux leviers d'aération (en haut de deux champignons géants) ne sont
//! pas abaissés ; le bloc poussé sur la dalle d'arrosage fait pousser la liane jusqu'à la canopée ; le pont de ronces pousse,
//! tient huit secondes et se flétrit ; la Mère-Racine sur la terrasse du nord. On joue tout avec le solveur de chemins.
use alw_core::{blocks::*, serre::{self as se}, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn session() -> Session { let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [2400.5, y as f64 + 2.0, 2385.5]); ready(&mut s); s }
fn world() -> (Session, [i32; 3]) {
    let mut s = session(); s.set_creative(false); s.hour = 12.0; let y = alw_core::gen::FLAT_Y;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [2400, y, 2400]; s.build_greenhouse(o); let _ = s.sync(); put(&mut s, o, [0, 0, -19]); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] + 1 + q[1], o[2] + q[2]] }
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
/// Sans soigner le joueur à chaque pas : pour compter les dégâts.
fn breathe(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-15, -1, -22]), at(o, [15, 24, 21])) }
fn dial(s: &Session, end: &str) -> usize { s.mech.dials.iter().position(|d| d.name.ends_with(end)).unwrap_or_else(|| panic!("{end}")) }
fn state(s: &Session, end: &str) -> u8 { s.mech.dials[dial(s, end)].state }
/// Pousser le bloc de `b` dans la direction `d` : on se tient derrière lui.
fn push(s: &mut Session, o: [i32; 3], b: [i32; 3], d: (i32, i32)) -> [i32; 3] { put(s, o, [b[0] - d.0, 1, b[2] - d.1]); run(s, 0.02); assert!(s.push_at(at(o, b)), "un bloc en {b:?}"); run(s, 0.05); let n = [b[0] + d.0, 1, b[2] + d.1]; assert_eq!(name(s, at(o, n)), "push_block", "poussé en {n:?}"); n }

#[test]
fn la_liane_et_le_pont_poussent_dans_l_ordre() {
    let st = se::liana_steps(); assert_eq!(st.len() as i32, se::DECK); assert_eq!(st.last().unwrap(), &[se::TRUNK.0, se::DECK, se::CANOPY - 1], "la dernière marche, au bord de la canopée");
    for w in st.windows(2) { assert_eq!((w[1][0] - w[0][0]).abs() + (w[1][2] - w[0][2]).abs(), 1, "marche après marche"); assert_eq!(w[1][1], w[0][1] + 1); }
    assert_eq!(se::liana(3).len(), 3); assert_eq!(se::bridge(se::bridge_len()).last().unwrap().0, [se::BRIDGE_U, se::DECK, se::BRIDGE.1]);
}

#[test]
fn spores_liane_ronces_et_la_mere_racine() {
    let (mut s, o) = world();
    let r = look(&s, o, [0, 0, -19]); assert!(r.contains(&at(o, [0, 1, -15])), "on entre"); assert!(!r.contains(&at(o, [0, 1, 0])), "la grille de la nef est fermée");
    // — les spores — elles brûlent ; les deux leviers sont au sommet des champignons, qu'on gravit de chapeau en chapeau
    put(&mut s, o, [0, 1, -10]); s.vit = [100.0; 3]; breathe(&mut s, 2.6); let hp = s.vit[0]; assert!(hp <= 100.0 - 2.0 * (se::SPORES - 1.5) + 0.01 && hp > 85.0, "les spores, plus fortes que la guérison ({hp})"); assert!(s.notes.iter().any(|t| t.contains("spores")));
    let r = look(&s, o, [0, 1, -15]); assert!(r.contains(&at(o, [6, 7, -8])), "au sommet du champignon de l'est"); assert!(r.contains(&at(o, [-6, 6, -12])), "au sommet du champignon de l'ouest");
    for v in se::VENTS { put(&mut s, o, [v[0] - v[0].signum(), v[1], v[2]]); assert!(s.lever_at(at(o, v))); run(&mut s, 0.1); }
    run(&mut s, 0.2); assert_eq!(name(&s, at(o, [0, 1, se::MID])), "air", "l'air est assaini : la grille se lève");
    put(&mut s, o, [0, 1, -10]); s.vit = [100.0; 3]; breathe(&mut s, 2.5); assert!(s.vit[0] >= 99.9, "plus de spores ({})", s.vit[0]);
    // — la liane — le bloc sur la dalle d'arrosage
    let r = look(&s, o, [0, 1, -10]); assert!(r.contains(&at(o, [0, 1, 0])), "dans la nef"); let top = [0, se::DECK + 1, 12]; assert!(!r.contains(&at(o, top)), "la canopée est hors d'atteinte");
    let mut b = se::BLOCK; for _ in 0..4 { b = push(&mut s, o, b, (1, 0)); } for _ in 0..5 { b = push(&mut s, o, b, (0, 1)); } assert_eq!(b, se::PLATE);
    put(&mut s, o, [0, 1, 0]); run(&mut s, 0.3); assert!(s.notes.iter().any(|t| t.contains("liane jaillit")), "{:?}", s.notes); run(&mut s, 3.5); assert_eq!(state(&s, "liane"), se::DECK as u8, "la liane a tout poussé");
    for (c, bk) in se::liana(se::DECK as u8) { assert_eq!(name(&s, at(o, c)), bk); } assert!(look(&s, o, [0, 1, 0]).contains(&at(o, top)), "la liane mène à la canopée");
    // on sort de la nef : le bloc reste sur sa dalle (la liane tient)
    put(&mut s, o, [0, 1, -10]); run(&mut s, 1.5); assert_eq!(name(&s, at(o, se::PLATE)), "push_block"); assert_eq!(state(&s, "liane"), se::DECK as u8);
    // — le pont de ronces — il pousse, tient, se flétrit
    let north = [0, se::DECK + 1, -9]; put(&mut s, o, top); assert!(!look(&s, o, top).contains(&at(o, north)), "la terrasse du nord est hors d'atteinte");
    let l = se::BRIDGE_LEVERS[0]; put(&mut s, o, [l[0] + 1, l[1], l[2]]); assert!(s.lever_at(at(o, l))); run(&mut s, 2.0); assert_eq!(state(&s, "pont"), se::bridge_len(), "le pont a poussé");
    assert!(look(&s, o, top).contains(&at(o, north)), "le pont mène à la terrasse"); run(&mut s, 4.0); assert_eq!(state(&s, "pont"), se::bridge_len(), "il tient encore");
    run(&mut s, 4.5); assert_eq!(state(&s, "pont"), 0, "il s'est flétri"); assert!(s.notes.iter().any(|t| t.contains("flétrit"))); assert!(!look(&s, o, top).contains(&at(o, north)));
    // on rappelle le pont depuis la terrasse (pour rentrer)
    let l2 = se::BRIDGE_LEVERS[1]; put(&mut s, o, [l2[0] + 1, l2[1], l2[2]]); assert!(s.lever_at(at(o, l2))); run(&mut s, 2.0); assert!(look(&s, o, north).contains(&at(o, top)), "le levier du nord rappelle le pont");
    // — la Mère-Racine —
    let sp = alw_core::hostile::species(se::BOSS).unwrap(); put(&mut s, o, [0, se::DECK + 1, -7]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("la Mère-Racine s'éveille"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "axe", false); run(&mut s, 1.0);
    assert!(s.loose.iter().any(|l| l.item == "graine_mere") || s.inv.iter().flatten().any(|st| st.item == "graine_mere"), "la Graine-Mère");
    let c = at(o, se::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin")); s.close_chest();
    // la sauvegarde garde la liane, le pont, les spores
    let t = s.save_text(); let mut b = session(); b.load_text(&t).unwrap();
    let st = |s: &Session| (s.mech.dials.iter().map(|d| (d.name.clone(), d.state, d.grow.clone(), d.draw.len())).collect::<Vec<_>>(), s.mech.miasmas.iter().map(|m| (m.min, m.max, m.until.clone())).collect::<Vec<_>>()); assert_eq!(st(&b), st(&s));
}
