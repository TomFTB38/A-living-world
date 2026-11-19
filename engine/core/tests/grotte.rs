//! La Grotte du Roi noyé : on descend l'escalier du puits, on résout les trois énigmes de la caverne (la citerne, la paroi,
//! les rois de pierre) dans l'ordre qu'on veut, chaque chaîne tombe, la grille du temple se lève, le Roi noyé se lève.
//! La citerne se remplit vraiment, la paroi se grimpe vraiment (avec ses deux brèches à sauter).
use alw_core::{blocks::*, grotte::{self as gr, DEPTH}, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn world() -> (Session, [i32; 3]) {
    let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [400.5, y as f64 + 1.0, 400.5]); ready(&mut s); s.set_creative(false); s.hour = 12.0;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [400, y, 400]; s.build_grotto(o); let _ = s.sync(); put(&mut s, o, [-4, 1, 0]); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] - DEPTH + q[1], o[2] + q[2]] }
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit[1] = 100.0; s.vit[2] = 100.0; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-34, -10, -14]), at(o, [34, DEPTH + 4, 38])) }
fn chain_up(s: &Session, o: [i32; 3], u: i32) -> bool { name(s, at(o, [u, 3, gr::CHAINS_W])) == "great_chain" }
fn in_cella(set: &BTreeSet<[i32; 3]>, o: [i32; 3]) -> bool { set.contains(&at(o, [0, 2, 17])) }
/// Marcher jusqu'au centre de la case relative (u, w), en tenant Saut si besoin.
fn go(s: &mut Session, o: [i32; 3], u: i32, w: i32, jump: bool) -> bool {
    let t = [(o[0] + u) as f64 + 0.5, (o[2] + w) as f64 + 0.5];
    for _ in 0..(8.0 / DT) as usize { let p = s.player.p; let d = [t[0] - p[0], t[1] - p[2]]; let l = d[0].hypot(d[1]); if l < 0.2 { s.player.v[0] = 0.0; s.player.v[2] = 0.0; return true; }
        s.player.yaw = d[0].atan2(d[1]); s.vit = [100.0; 3]; s.stamina = 100.0; s.tick(DT, Input { z: (l / 0.5).clamp(0.35, 1.0), jump, ..Input::default() }, Actions::default()); }
    false
}
/// Les rois : on fait pivoter chacun jusqu'à ce qu'il regarde son temple (rend le nombre de quarts de tour).
fn face_kings(s: &mut Session, o: [i32; 3]) -> usize {
    let want: Vec<([i32; 3], u8)> = s.mech.wires.iter().find(|w| w.name.ends_with("regard")).unwrap().faces.clone(); let mut n = 0;
    for (p, f) in want { while s.facing_at(p) != Some(f) { assert!(s.turn_statue_at([p[0], p[1] + 1, p[2]]), "une statue en {p:?}"); n += 1; assert!(n < 20); } } run(s, 0.2); let _ = o; n
}

#[test]
fn on_descend_le_puits_et_le_temple_est_ferme() {
    let (s, o) = world();
    // de l'esplanade, l'escalier en spirale descend au fond ; on en remonte
    let porch = [-9, DEPTH + 1, -5]; assert!(name(&s, at(o, [-9, DEPTH, -5])).contains("stone"), "le dallage de l'esplanade");
    let down = look(&s, o, porch); println!("{} cases atteintes depuis l'esplanade", down.len());
    assert!(down.contains(&at(o, [-4, 1, 0])), "le fond du puits"); assert!(look(&s, o, [-4, 1, 0]).contains(&at(o, porch)), "et l'on remonte");
    for (i, (u, w, top, ..)) in gr::spiral().iter().enumerate() { assert!(name(&s, at(o, [*u, *top, *w])).starts_with("stone_bricks"), "marche {i}"); }
    // le temple : sur l'île, on va jusqu'au portique (à la nage), pas dans la cella ; trois chaînes, une grille
    let r = look(&s, o, [-4, 1, 0]); assert!(r.contains(&at(o, [-2, 2, 12])), "le portique, à la nage"); assert!(!in_cella(&r, o), "la cella est close");
    for u in -1..=1 { assert!(chain_up(&s, o, u)); assert!(name(&s, at(o, [u, 3, gr::DOOR_W])).starts_with("iron_bars")); }
    // la caverne est vaste et la voûte reste sous terre
    let mut air = 0; for u in (-30..=30).step_by(2) { for w in (-10..=34).step_by(2) { for v in 1..=24 { if s.world.get(at(o, [u, v, w])[0], at(o, [u, v, w])[1], at(o, [u, v, w])[2]) == Some(0) { air += 1; } } } }
    println!("volume d'air (1 case sur 8) : {air}"); assert!(air > 3000);
    let c = at(o, gr::CHEST); assert!(is_chest_shut(s.world.get(c[0], c[1], c[2]).unwrap()));
}

#[test]
fn la_citerne_se_remplit_et_l_on_remonte_a_la_nage() {
    let (mut s, o) = world(); let rim = [-25, 1, 1]; let bottom = [-25, -7, 3];
    assert!(look(&s, o, rim).contains(&at(o, bottom)), "on se laisse tomber au fond"); put(&mut s, o, bottom); run(&mut s, 0.3);
    println!("{:?}", s.notes); assert!(s.notes.iter().any(|n| n.contains("citerne")));
    assert!(!look(&s, o, bottom).contains(&at(o, rim)), "à sec, on ne remonte pas");
    // la chaîne de l'eau
    assert!(s.lever_at(at(o, gr::LEVER_WATER))); run(&mut s, 0.3); assert!(!chain_up(&s, o, -1), "la chaîne de l'eau tombe"); assert!(chain_up(&s, o, 0) && chain_up(&s, o, 1));
    // la vanne : l'eau monte, couche après couche ; on nage jusqu'au bord
    assert!(s.lever_at(at(o, gr::VALVE))); run(&mut s, 1.0); let mid = name(&s, at(o, [-25, -4, 3])); run(&mut s, 3.0);
    println!("à mi-chemin : {mid} ; au bout de 4 s : {} ; {:?}", name(&s, at(o, [-25, 0, 3])), s.notes.last()); assert_eq!(name(&s, at(o, [-25, 0, 3])), "water", "pleine");
    assert!(look(&s, o, bottom).contains(&at(o, rim)), "le solveur remonte à la nage");
    let mut ok = false; for _ in 0..(10.0 / DT) as usize { let p = s.player.p; let t = [(o[0] - 25) as f64 + 0.5, (o[2] + 1) as f64 + 0.5]; let d = [t[0] - p[0], t[1] - p[2]]; s.player.yaw = d[0].atan2(d[1]); s.vit = [100.0; 3];
        s.tick(DT, Input { z: 1.0, jump: true, ..Input::default() }, Actions::default()); if s.player.ground && s.player.p[1] >= at(o, rim)[1] as f64 - 0.05 && (s.player.p[2] - t[1]).abs() < 0.6 { ok = true; break; } }
    println!("le joueur : {:?} (bord à {})", s.player.p, at(o, rim)[1]); assert!(ok, "on sort de la citerne à la nage");
    // la vanne refermée, la citerne se vide
    assert!(s.lever_at(at(o, gr::VALVE))); run(&mut s, 4.0); assert_eq!(name(&s, at(o, [-25, -6, 3])), "air", "vidée");
}

#[test]
fn la_paroi_se_grimpe_corniche_apres_corniche() {
    let (mut s, o) = world(); let nook = [27, 16, 10];
    assert!(look(&s, o, [24, 1, 2]).contains(&at(o, nook)), "le solveur trouve la niche");
    // sans la corniche d'après la première brèche, plus de chemin
    { let (mut t, o2) = world(); let p = at(o2, [26, 3, 7]); t.world.edit(p[0], p[1], p[2], 0); assert!(!look(&t, o2, [24, 1, 2]).contains(&at(o2, nook)), "la brèche ne se franchit pas sans appui"); }
    // et pour de vrai : corniche après corniche, en tenant Saut
    put(&mut s, o, [24, 1, 2]); run(&mut s, 0.1);
    for &(u, w, top) in gr::LEDGES.iter() { assert!(go(&mut s, o, u, w, true), "coincé avant la corniche {u} {w} : {:?}", s.player.p); let y = s.player.p[1] - at(o, [0, 0, 0])[1] as f64; assert!(y >= top as f64 + 0.9, "tombé avant {u} {w} ({y:.2})"); }
    assert!(go(&mut s, o, 26, 10, true) && go(&mut s, o, 27, 10, false)); println!("dans la niche : {:?}", s.player.p); assert!(s.player.p[1] >= at(o, nook)[1] as f64 - 0.05);
    assert!(s.lever_at(at(o, gr::LEVER_WALL))); run(&mut s, 0.3); assert!(!chain_up(&s, o, 0), "la chaîne de la paroi tombe");
}

#[test]
fn les_rois_regardent_leur_temple_dans_l_ordre_qu_on_veut_et_le_roi_noye_se_leve() {
    let (mut s, o) = world();
    // dans le désordre : les rois d'abord
    let n = face_kings(&mut s, o); println!("{n} quarts de tour pour les trois rois"); assert!(n >= 3); assert!(!chain_up(&s, o, 1), "la chaîne du regard tombe");
    assert!(!in_cella(&look(&s, o, [-4, 1, 0]), o), "deux chaînes encore");
    assert!(s.lever_at(at(o, gr::LEVER_WALL))); run(&mut s, 0.2); assert!(!in_cella(&look(&s, o, [-4, 1, 0]), o), "une chaîne encore");
    assert!(s.lever_at(at(o, gr::LEVER_WATER))); run(&mut s, 0.3); println!("{:?}", s.notes.iter().rev().take(2).collect::<Vec<_>>());
    for u in -1..=1 { assert!(!chain_up(&s, o, u)); assert_eq!(name(&s, at(o, [u, 3, gr::DOOR_W])), "air", "la grille se lève"); }
    assert!(in_cella(&look(&s, o, [-4, 1, 0]), o), "le temple s'ouvre");
    // le Roi noyé
    let sp = alw_core::hostile::species(gr::BOSS).unwrap(); put(&mut s, o, [0, 2, 15]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("le Roi noyé se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    assert!(name(&s, at(o, [0, 3, gr::DOOR_W])).starts_with("iron_bars"), "la grille retombe derrière vous");
    let xp = s.prog.xp; s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "sword", false); run(&mut s, 1.0);
    assert_eq!(name(&s, at(o, [0, 3, gr::DOOR_W])), "air", "vaincu : la grille se relève"); assert!(s.prog.xp - xp >= 300.0);
    assert!(s.loose.iter().any(|l| l.item == "couronne_noyee") || s.inv.iter().flatten().any(|st| st.item == "couronne_noyee"), "la couronne du Roi noyé");
    let c = at(o, gr::CHEST); s.open_chest(c); println!("le trésor : {:?}", s.chests[&c].iter().flatten().map(|st| st.item).collect::<Vec<_>>()); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin"));
    // tout se sauvegarde : la citerne, les rois, les fils
    let t = s.save_text(); assert!(t.lines().any(|l| l.starts_with("pool ")));
    let faces = s.mech.wires.iter().find(|w| w.name.ends_with("regard")).unwrap().faces.iter().map(|(p, f)| format!("{},{},{},{f}", p[0], p[1], p[2])).collect::<Vec<_>>().join(";");
    assert!(t.lines().any(|l| l.starts_with("wire") && l.split_whitespace().nth(17) == Some(faces.as_str())), "les regards attendus des rois sont sauvegardés");
}
