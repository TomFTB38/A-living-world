//! Les mécanismes des donjons : levier et grille, dalle à pression (sous le pied, sous un bloc poussé), énigme à plusieurs dalles,
//! pics, sol friable, lance-fléchettes, et leur sauvegarde.
use alw_core::{blocks::by_name, mecanismes::{Pulse, Seal, Wire}, player::*, session::*, stream::*};
const DT: f64 = 1.0 / 60.0;
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(4)); } }
fn b(n: &str) -> u16 { by_name(n).unwrap() }
/// Le monde plat : le joueur en (0, y, 0), le sol en y - 1.
fn flat() -> (Session, i32) {
    let seed = alw_core::gen::FLAT | 0x123; let y = alw_core::gen::FLAT_Y as f64 + 1.0;
    let mut s = Session::new(seed, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, y, 0.5]); ready(&mut s); s.third = false; s.hour = 12.0;
    for _ in 0..30 { s.tick(DT, Input::default(), Actions::default()); } s.wild.all.clear(); s.horde.foes.clear(); s.vit = [100.0; 3]; let y = s.player.p[1].floor() as i32; (s, y)
}
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT) as usize { s.tick(DT, Input::default(), Actions::default()); } }
fn at(s: &Session, p: [i32; 3]) -> u16 { use alw_core::player::Voxels; s.world.get(p[0], p[1], p[2]).unwrap_or(9999) }
/// Une grille de deux barreaux de haut en (5, y, 0) et (5, y + 1, 0).
fn gate(s: &mut Session, y: i32) -> Vec<([i32; 3], u16)> { let g = vec![([5, y, 0], b("iron_bars@1")), ([5, y + 1, 0], b("iron_bars@1"))]; for (p, id) in &g { s.world.edit(p[0], p[1], p[2], *id); } g }

#[test]
fn le_levier_leve_et_baisse_la_grille() {
    let (mut s, y) = flat(); let doors = gate(&mut s, y); let lever = [3, y + 1, 1]; s.world.edit(3, y + 1, 2, b("stone")); s.world.edit(lever[0], lever[1], lever[2], b("lever"));
    s.mech.wires.push(Wire { inputs: vec![lever], doors: doors.clone(), ..Default::default() });
    assert!(s.lever_at(lever)); run(&mut s, 0.1); println!("levier abaissé : grille {} {}", at(&s, [5, y, 0]), at(&s, [5, y + 1, 0])); assert_eq!(at(&s, [5, y, 0]), 0, "la grille se lève");
    assert!(s.lever_at(lever)); run(&mut s, 0.1); assert_eq!(at(&s, [5, y, 0]), b("iron_bars@1"), "elle retombe");
}

#[test]
fn la_dalle_s_enfonce_sous_le_pied_ou_sous_un_bloc_pousse() {
    let (mut s, y) = flat(); let doors = gate(&mut s, y); let plate = [0, y - 1, 3]; s.world.edit(plate[0], plate[1], plate[2], b("pressure_plate"));
    s.mech.wires.push(Wire { inputs: vec![plate], doors, ..Default::default() });
    s.player.p = [0.5, y as f64, 3.5]; run(&mut s, 0.2); assert_eq!(at(&s, plate), b("pressure_plate_on")); assert_eq!(at(&s, [5, y, 0]), 0, "sous le pied, la grille s'ouvre");
    s.player.p = [0.5, y as f64, -3.5]; run(&mut s, 0.2); assert_eq!(at(&s, [5, y, 0]), b("iron_bars@1"), "le pied levé, elle se referme");
    // un bloc poussé sur la dalle la tient enfoncée
    s.world.edit(0, y, 2, b("push_block")); s.player.p = [0.5, y as f64, 0.5]; run(&mut s, 0.1); assert!(s.push_at([0, y, 2])); run(&mut s, 0.2);
    println!("bloc poussé : en (0, y, 3) {} ; dalle {} ; grille {}", at(&s, [0, y, 3]) == b("push_block"), at(&s, plate) == b("pressure_plate_on"), at(&s, [5, y, 0]));
    assert_eq!(at(&s, [0, y, 3]), b("push_block")); assert_eq!(at(&s, [5, y, 0]), 0, "la grille reste ouverte sous le bloc");
}

#[test]
fn une_enigme_a_deux_dalles_et_une_grille_qui_reste_ouverte() {
    let (mut s, y) = flat(); let doors = gate(&mut s, y); let (p1, p2) = ([-3, y - 1, 3], [3, y - 1, 3]); s.world.edit(p1[0], p1[1], p1[2], b("pressure_plate")); s.world.edit(p2[0], p2[1], p2[2], b("pressure_plate"));
    s.mech.wires.push(Wire { inputs: vec![p1, p2], all: true, latch: true, doors, ..Default::default() });
    s.player.p = [-2.5, y as f64, 3.5]; run(&mut s, 0.2); assert_eq!(at(&s, [5, y, 0]), b("iron_bars@1"), "une seule dalle : rien");
    s.world.edit(3, y, 3, b("push_block")); run(&mut s, 0.2); assert_eq!(at(&s, [5, y, 0]), 0, "les deux : la grille se lève");
    s.player.p = [0.5, y as f64, -3.5]; run(&mut s, 0.3); assert_eq!(at(&s, [5, y, 0]), 0, "et reste levée");
}

#[test]
fn pics_sol_friable_et_flechettes() {
    let (mut s, y) = flat(); s.world.edit(0, y, 0, b("spikes")); run(&mut s, 1.6); println!("sur les pics : vie {:.0}", s.vit[0]); assert!(s.vit[0] < 95.0);
    let (mut s, y) = flat(); for z in 2..5 { s.world.edit(0, y - 1, z, b("crumbling_floor")); for d in 2..5 { s.world.edit(0, y - d, z, 0); } }
    s.player.p = [0.5, y as f64, 3.5]; run(&mut s, 0.2); let still = s.player.p[1]; run(&mut s, 1.2);
    println!("sur le sol friable : {:.2} puis {:.2}", still - y as f64, s.player.p[1] - y as f64); assert!(still >= y as f64 - 0.01 && s.player.p[1] < y as f64 - 1.5, "il tient, puis s'effondre");
    let (mut s, y) = flat(); let plate = [0, y - 1, 0]; s.world.edit(plate[0], plate[1], plate[2], b("pressure_plate")); s.world.edit(0, y, 8, b("dart_trap"));
    s.mech.wires.push(Wire { inputs: vec![plate], darts: vec![[0, y, 8]], ..Default::default() }); run(&mut s, 1.0); println!("fléchette : vie {:.0}, « {} »", s.vit[0], s.cause); assert!(s.vit[0] < 97.0 && s.cause.contains("fléchette"));
}

/// Trois leviers contre un mur en z = 2, à x = -2, 0, 2.
fn levers(s: &mut Session, y: i32) -> Vec<[i32; 3]> { let v = vec![[-2, y + 1, 1], [0, y + 1, 1], [2, y + 1, 1]]; for p in &v { s.world.edit(p[0], p[1], 2, b("stone")); s.world.edit(p[0], p[1], p[2], b("lever")); } v }

#[test]
fn l_ordre_des_leviers_et_le_faux_pas() {
    let (mut s, y) = flat(); let doors = gate(&mut s, y); let l = levers(&mut s, y); let (a, bb, c) = (l[0], l[1], l[2]);
    // l'ordre : A, puis C, puis B
    s.mech.wires.push(Wire { name: "hall".into(), inputs: vec![a, c, bb], seq: true, latch: true, doors, ..Default::default() }); run(&mut s, 0.1);
    s.lever_at(a); run(&mut s, 0.1); s.lever_at(bb); run(&mut s, 0.1);
    println!("A puis B (faux) : progression {}, leviers {:?}, {:?}", s.mech.wires[0].progress, l.iter().map(|p| at(&s, *p) == b("lever_on")).collect::<Vec<_>>(), s.notes.last());
    assert_eq!(s.mech.wires[0].progress, 0); assert!(l.iter().all(|p| at(&s, *p) == b("lever")), "un faux pas relève tous les leviers"); assert_eq!(at(&s, [5, y, 0]), b("iron_bars@1"));
    s.lever_at(a); run(&mut s, 0.1); s.lever_at(c); run(&mut s, 0.1); assert_eq!(at(&s, [5, y, 0]), b("iron_bars@1"), "pas encore"); s.lever_at(bb); run(&mut s, 0.1);
    assert_eq!(at(&s, [5, y, 0]), 0, "A, C, B : la porte s'ouvre"); let r = s.wires_report(48.0); println!("{r:?}"); assert!(r[0].contains("séquence 3/3") && r[0].contains("enclenché"));
}

#[test]
fn la_porte_a_retardement() {
    let (mut s, y) = flat(); let doors = gate(&mut s, y); let l = levers(&mut s, y);
    s.mech.wires.push(Wire { inputs: vec![l[1]], hold: 3.0, doors, ..Default::default() }); s.lever_at(l[1]); run(&mut s, 0.2);
    assert_eq!(at(&s, [5, y, 0]), 0, "ouverte"); run(&mut s, 2.0); assert_eq!(at(&s, [5, y, 0]), 0, "encore ouverte"); run(&mut s, 1.2);
    println!("après 3,4 s : grille {}, levier relevé {}", at(&s, [5, y, 0]), at(&s, l[1]) == b("lever")); assert_eq!(at(&s, [5, y, 0]), b("iron_bars@1"), "refermée"); assert_eq!(at(&s, l[1]), b("lever"), "le levier se relève");
}

#[test]
fn circuit_inverse_fils_lies_et_piege_desarme() {
    let (mut s, y) = flat(); let plate = [0, y - 1, 3]; s.world.edit(plate[0], plate[1], plate[2], b("pressure_plate"));
    let ga = gate(&mut s, y); let gb = vec![([-5, y, 0], b("iron_bars@1"))]; s.world.edit(-5, y, 0, b("iron_bars@1"));
    // la dalle ouvre A et ferme B (inversé) ; un levier, lié au fil de la dalle, désarme les pics
    let l = levers(&mut s, y); for x in -1..=1 { s.world.edit(x, y, 6, b("spikes")); }
    s.mech.wires.push(Wire { name: "dalle".into(), inputs: vec![plate], doors: ga, ..Default::default() });
    s.mech.wires.push(Wire { name: "inverse".into(), inputs: vec![plate], not: true, doors: gb, ..Default::default() });
    s.mech.wires.push(Wire { name: "desarme".into(), inputs: vec![l[0]], needs: vec!["dalle".into()], calm: (-1..=1).map(|x| [x, y, 6]).collect(), ..Default::default() });
    run(&mut s, 0.2); assert_eq!(at(&s, [5, y, 0]), b("iron_bars@1")); assert_eq!(at(&s, [-5, y, 0]), 0, "au repos, B est ouverte");
    s.lever_at(l[0]); run(&mut s, 0.2); assert_eq!(at(&s, [0, y, 6]), b("spikes"), "le levier seul ne suffit pas");
    s.world.edit(0, y, 3, b("push_block")); run(&mut s, 0.2);
    println!("bloc sur la dalle : A {} B {} pics {}", at(&s, [5, y, 0]), at(&s, [-5, y, 0]), at(&s, [0, y, 6]) == b("spikes_down"));
    assert_eq!(at(&s, [5, y, 0]), 0); assert_eq!(at(&s, [-5, y, 0]), b("iron_bars@1"), "A s'ouvre, B se ferme"); assert_eq!(at(&s, [0, y, 6]), b("spikes_down"), "dalle et levier : pics rentrés");
}

#[test]
fn le_couloir_des_pics_au_rythme() {
    let (mut s, y) = flat(); let row: Vec<[i32; 3]> = (0..3).map(|z| [3, y, z + 4]).collect(); for p in &row { s.world.edit(p[0], p[1], p[2], b("spikes_down")); }
    s.mech.pulses.push(Pulse { spikes: row.clone(), period: 2.0, up: 0.5, off_by: "stop".into(), ..Default::default() });
    let mut seen = Vec::new(); for _ in 0..8 { run(&mut s, 0.5); seen.push(at(&s, row[0]) == b("spikes")); } println!("pics, toutes les 0,5 s : {seen:?}");
    assert!(seen.iter().any(|v| *v) && seen.iter().any(|v| !*v), "ils sortent et rentrent");
    let l = levers(&mut s, y); s.mech.wires.push(Wire { name: "stop".into(), inputs: vec![l[0]], ..Default::default() }); s.lever_at(l[0]);
    let mut after = Vec::new(); for _ in 0..6 { run(&mut s, 0.5); after.push(at(&s, row[0]) == b("spikes")); } assert!(after.iter().all(|v| !*v), "arrêtés par le levier : {after:?}");
}

#[test]
fn le_sceau_de_pierre_ancienne() {
    let (mut s, y) = flat(); s.set_creative(false); let l = levers(&mut s, y); s.world.edit(0, y, 4, b("push_block"));
    s.mech.seals.push(Seal { min: [-4, y - 2, -4], max: [4, y + 4, 6], until: "fin".into() }); s.mech.wires.push(Wire { name: "fin".into(), inputs: vec![l[2]], latch: true, ..Default::default() });
    assert!(s.sealed([0, y + 1, 2], b("stone")), "le mur est scellé"); assert!(!s.sealed(l[0], b("lever")), "le levier reste libre"); assert!(s.sealed([0, y, 4], b("push_block")), "le bloc à pousser se pousse, il ne s'emporte pas");
    assert!(!s.sealed([9, y, 9], b("stone")), "hors de la zone"); s.lever_at(l[2]); run(&mut s, 0.1); assert!(!s.sealed([0, y + 1, 2], b("stone")), "résolu : le sceau se lève");
}

#[test]
fn les_fils_se_sauvegardent() {
    let (mut s, y) = flat(); let doors = gate(&mut s, y);
    s.mech.wires.push(Wire { name: "salle5".into(), inputs: vec![[3, y + 1, 1], [0, y - 1, 3]], all: true, latch: true, doors, darts: vec![[0, y, 8]], spikes: vec![[1, y, 1]], seq: true, not: false, hold: 4.0, needs: vec!["a".into(), "b".into()], calm: vec![[2, y, 2]], fail_darts: true, ..Default::default() });
    s.mech.pulses.push(Pulse { spikes: vec![[3, y, 3]], darts: vec![[4, y, 4]], period: 2.5, up: 0.4, phase: 1.0, off_by: "salle5".into(), ..Default::default() }); s.mech.seals.push(Seal { min: [-3, 0, -3], max: [3, 90, 3], until: "salle5".into() });
    let t = s.save_text(); let (mut b2, _) = flat(); b2.load_text(&t).unwrap();
    let key = |w: &Wire| ((w.name.clone(), w.inputs.clone(), w.all, w.latch, w.seq, w.not, w.hold), (w.needs.clone(), w.doors.clone(), w.darts.clone(), w.spikes.clone(), w.calm.clone(), w.fail_darts));
    assert_eq!(key(&b2.mech.wires[0]), key(&s.mech.wires[0])); assert_eq!(b2.mech.seals, s.mech.seals);
    let (p, q) = (&b2.mech.pulses[0], &s.mech.pulses[0]); assert_eq!((p.period, p.up, p.phase, &p.off_by, &p.spikes, &p.darts), (q.period, q.up, q.phase, &q.off_by, &q.spikes, &q.darts));
}
