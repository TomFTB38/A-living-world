//! Le Temple du Soleil : le rayon de l'oculus suit les miroirs qu'on tourne, embrase un sceau à la fois, s'éteint la nuit ;
//! le sceau du sud demande d'abord la dalle (un bloc poussé) ; les trois sceaux embrasés ouvrent le sanctuaire.
use alw_core::{blocks::*, player::*, sanctuaire::reach, session::*, soleil as so, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn world() -> (Session, [i32; 3]) {
    let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [600.5, y as f64 + 1.0, 600.5]); ready(&mut s); s.set_creative(false); s.hour = 12.0;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [600, y, 600]; s.build_sun_temple(o); let _ = s.sync(); put(&mut s, o, [0, 1, -6]); run(&mut s, 0.5); s.notes.clear(); (s, o)
}
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] + q[1], o[2] + q[2]] }
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-26, -2, -30]), at(o, [26, 22, 30])) }
fn lit(s: &Session, o: [i32; 3], k: usize) -> bool { name(s, at(o, so::SEALS[k].0)).starts_with("sun_receptor_on") }
fn beam(s: &Session, o: [i32; 3], q: [i32; 3]) -> bool { name(s, at(o, q)).starts_with("light_beam") }
/// Tourner le miroir `k` jusqu'à ce que sa diagonale ait la parité voulue (0 : « / », 1 : « \ »).
fn set(s: &mut Session, o: [i32; 3], k: usize, parity: u8) { let p = at(o, so::MIRRORS[k].0); while s.facing_at(p).unwrap() % 2 != parity { assert!(s.turn_statue_at(p)); } run(s, 0.3); }

#[test]
fn le_temple_se_visite_et_le_rayon_suit_les_miroirs() {
    let (mut s, o) = world();
    // dehors, le portail du nord mène à la salle ; l'escalier du sud mène au sommet ; le sanctuaire est clos
    let r = look(&s, o, [0, 1, -22]); assert!(r.contains(&at(o, [0, 1, -6])), "par le portail, dans la salle"); assert!(!r.contains(&at(o, [0, 1, 14])), "le sanctuaire est clos");
    assert!(look(&s, o, [0, 1, 25]).contains(&at(o, [0, 16, -4])), "l'escalier du sud mène au sommet");
    for (q, _) in so::MIRRORS { let n = [[1, 0, 0], [-1, 0, 0], [0, 0, 1], [0, 0, -1]].iter().any(|d| r.contains(&at(o, [q[0] + d[0], q[1], q[2] + d[2]]))); assert!(n, "on peut s'approcher du miroir {q:?}"); }
    // au départ : le rayon part du disque vers l'est, aucun sceau n'est embrasé
    assert!(beam(&s, o, [2, 1, 0]), "le rayon, entre le disque et le premier miroir"); for k in 0..3 { assert!(!lit(&s, o, k)); }
    // la nuit, il s'éteint
    s.hour = 23.0; run(&mut s, 0.4); assert!(!beam(&s, o, [2, 1, 0]), "la nuit, pas de soleil"); s.hour = 12.0; run(&mut s, 0.4); assert!(beam(&s, o, [2, 1, 0]));
    // 1. le sceau de l'est : le premier miroir renvoie au nord, le second à l'est
    set(&mut s, o, 0, 1); set(&mut s, o, 1, 1); println!("{:?}", s.notes.last());
    assert!(lit(&s, o, 0), "le sceau de l'est s'embrase"); assert!(beam(&s, o, [8, 1, -5]), "le rayon file vers l'est"); assert!(!beam(&s, o, [4, 1, 3]), "plus rien au sud du premier miroir");
    // 2. le sceau du nord : le second miroir renvoie à l'ouest, le quatrième au nord ; celui de l'est s'éteint, son fil reste enclenché
    set(&mut s, o, 1, 0); set(&mut s, o, 3, 0); assert!(lit(&s, o, 1), "le sceau du nord"); assert!(!lit(&s, o, 0), "l'est s'éteint");
    let on = |s: &Session, k: usize| s.mech.wires.iter().find(|w| w.name.ends_with(&format!("sceau{k}"))).unwrap().on; assert!(on(&s, 0) && on(&s, 1) && !on(&s, 2));
    // 3. le sceau du sud : par le sud, l'est… la grille l'arrête ; le bloc sur la dalle, elle se lève
    set(&mut s, o, 0, 0); set(&mut s, o, 2, 1); set(&mut s, o, 4, 1); assert!(beam(&s, o, [-4, 1, 9]), "le rayon descend vers le sud"); assert!(!lit(&s, o, 2), "la grille l'arrête");
    for k in 0..3 { let b = [so::BLOCK[0], 1, so::BLOCK[2] + k]; assert!(look(&s, o, [0, 1, -6]).contains(&at(o, [b[0], 1, b[2] - 1])), "on se place pour pousser"); put(&mut s, o, [b[0], 1, b[2] - 1]); run(&mut s, 0.05); put(&mut s, o, [b[0], 1, b[2] - 1]); assert!(s.push_at(at(o, b))); run(&mut s, 0.2); }
    run(&mut s, 0.5); println!("{:?}", s.notes.iter().rev().take(3).collect::<Vec<_>>()); assert!(lit(&s, o, 2), "le sceau du sud s'embrase");
    // les trois sceaux : le sanctuaire s'ouvre ; le Gardien du Soleil
    for q in [[0, 1, 11], [1, 2, 11], [2, 3, 11]] { assert_eq!(name(&s, at(o, q)), "air", "la grille du sanctuaire"); }
    assert!(look(&s, o, [0, 1, -6]).contains(&at(o, [0, 1, 14])), "on entre au sanctuaire");
    let sp = alw_core::hostile::species(so::BOSS).unwrap(); put(&mut s, o, [1, 1, 13]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("le Gardien du Soleil se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    assert!(name(&s, at(o, [1, 2, 11])).starts_with("iron_bars"), "la grille retombe");
    s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "sword", false); run(&mut s, 1.0); assert_eq!(name(&s, at(o, [1, 2, 11])), "air");
    assert!(s.loose.iter().any(|l| l.item == "disque_solaire") || s.inv.iter().flatten().any(|st| st.item == "disque_solaire"), "le Disque du Soleil");
    let c = at(o, so::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin"));
    // la sauvegarde garde le rayon
    let t = s.save_text(); let l = t.lines().find(|l| l.starts_with("beam ")).expect("le rayon sauvegardé"); println!("{}", &l[..l.len().min(120)]);
    let mut b = { let y = alw_core::gen::FLAT_Y; let mut b = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [600.5, y as f64 + 1.0, 600.5]); ready(&mut b); b };
    b.load_text(&t).unwrap(); assert_eq!(b.mech.beams, s.mech.beams.iter().map(|x| alw_core::mecanismes::Beam { t: 0.0, ..x.clone() }).collect::<Vec<_>>());
}
