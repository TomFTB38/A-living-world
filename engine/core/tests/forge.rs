//! La Forge sous la lave : le pont qui monte de la lave sous un poids, son verrou ; le bloc qu'on fait passer d'une rive à
//! l'autre jusqu'au plateau du monte-charge, poussé depuis la plate-forme ; la vanne qui retient la lave vingt secondes ; les
//! trois chaînes, la Porte du Foyer, le Maître de forge.
use alw_core::{blocks::*, forge::{self as fo, DEPTH}, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn world() -> (Session, [i32; 3]) {
    let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [1000.5, y as f64 + 1.0, 1000.5]); ready(&mut s); s.set_creative(false); s.hour = 12.0;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [1000, y, 1000]; s.build_forge(o); let _ = s.sync(); put(&mut s, o, [0, 1, 2]); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] - DEPTH + q[1], o[2] + q[2]] }
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-18, -8, -34]), at(o, [18, DEPTH + 4, 38])) }
fn chain(s: &Session, o: [i32; 3], u: i32) -> bool { name(s, at(o, [u, 2, fo::CHAINS_W])) == "great_chain" }
fn bridge_up(s: &Session, o: [i32; 3]) -> bool { (fo::RIVER.0..=fo::RIVER.1).all(|w| name(s, at(o, [0, 0, w])) == "basalt") }
/// Pousser le bloc en `b` d'une case dans la direction `d`, en se plaçant derrière lui (on doit pouvoir s'y tenir) ; rend sa place.
fn push(s: &mut Session, o: [i32; 3], b: [i32; 3], d: (i32, i32)) -> [i32; 3] {
    let me = [b[0] - d.0, b[1], b[2] - d.1]; assert!(look(s, o, me).contains(&at(o, me)) || s.world.get(at(o, me)[0], at(o, me)[1] - 1, at(o, me)[2]).is_some_and(is_solid), "on se tient derrière le bloc en {me:?}");
    put(s, o, me); run(s, 0.05); put(s, o, me); assert!(s.push_at(at(o, b)), "un bloc en {b:?}"); let n = [b[0] + d.0, b[1], b[2] + d.1]; assert_eq!(name(s, at(o, n)), "push_block", "le bloc a glissé en {n:?}"); run(s, 0.1); n
}

#[test]
fn le_pont_le_monte_charge_la_coulee_le_maitre_de_forge() {
    let (mut s, o) = world();
    // de la cour, l'escalier mène à la grande forge ; la lave coupe la salle ; la porte du foyer est close
    let yard = [0, DEPTH + 1, -28]; let r = look(&s, o, yard); assert!(r.contains(&at(o, [0, 1, 2])), "l'escalier mène à la forge"); assert!(look(&s, o, [0, 1, 2]).contains(&at(o, yard)), "et l'on remonte");
    assert!(!r.contains(&at(o, [0, 1, 20])), "la lave coupe la salle"); assert_eq!(name(&s, at(o, [0, 0, 11])), "lava"); assert!(!r.contains(&at(o, [0, 1, 33])));
    // le pont monte sous les pieds de qui se tient sur la dalle, et redescend quand il s'en va
    put(&mut s, o, [3, 1, 8]); run(&mut s, 1.5); assert!(bridge_up(&s, o), "le pont est monté"); assert!(look(&s, o, [0, 1, 8]).contains(&at(o, [0, 1, 20])), "on passerait");
    put(&mut s, o, [5, 1, 3]); run(&mut s, 1.5); assert!(!bridge_up(&s, o) && name(&s, at(o, [0, 0, 11])) == "lava", "le pont est retombé dans la lave");
    // le bloc de pierre sur la dalle : le pont tient ; on passe, on le verrouille
    let mut b = fo::BLOCK; for _ in 0..3 { b = push(&mut s, o, b, (-1, 0)); } for _ in 0..2 { b = push(&mut s, o, b, (0, 1)); } assert_eq!(b, [3, 1, 8]);
    run(&mut s, 1.5); assert!(bridge_up(&s, o)); assert!(look(&s, o, [0, 1, 2]).contains(&at(o, [2, 1, 16])), "sur l'autre rive");
    put(&mut s, o, [2, 1, 16]); assert!(s.lever_at(at(o, fo::LOCK))); run(&mut s, 0.2); assert!(!chain(&s, o, -1) && chain(&s, o, 0) && chain(&s, o, 1), "la chaîne du pont tombe");
    // le bloc quitte la dalle : le pont, verrouillé, reste ; on l'emmène sur l'autre rive, jusqu'au monte-charge
    for _ in 0..3 { b = push(&mut s, o, b, (-1, 0)); } run(&mut s, 1.0); assert!(bridge_up(&s, o), "le verrou tient le pont");
    for _ in 0..10 { b = push(&mut s, o, b, (0, 1)); } for _ in 0..9 { b = push(&mut s, o, b, (1, 0)); } for _ in 0..2 { b = push(&mut s, o, b, (0, 1)); } assert_eq!(b, [9, 1, 20]);
    // chargé sans personne dessus, le monte-charge part seul ; on retire le bloc… puis on le pousse depuis la plate-forme
    assert!(!look(&s, o, [9, 1, 18]).contains(&at(o, [12, 9, 20])), "pas encore de galerie");
    put(&mut s, o, [10, 1, 20]); run(&mut s, 0.1); assert!(s.push_at(at(o, b)), "on pousse depuis la plate-forme"); assert_eq!(name(&s, at(o, fo::TRAY)), "pressure_plate", "le bloc arrive sur le plateau");
    run(&mut s, 4.0); let f = o[1] - DEPTH; println!("{:?} — le joueur à {:.2}", s.notes.iter().rev().take(2).collect::<Vec<_>>(), s.player.p[1] - f as f64);
    assert!((s.player.p[1] - (f + fo::LIFT_UP + 1) as f64).abs() < 0.3, "on est monté avec la plate-forme"); assert!(s.dead.is_none());
    assert!(look(&s, o, [10, fo::LIFT_UP + 1, 20]).contains(&at(o, [15, 9, 25])), "la galerie"); put(&mut s, o, [15, 9, 25]); assert!(s.lever_at(at(o, fo::BELLOWS))); run(&mut s, 0.2); assert!(!chain(&s, o, 0), "la chaîne du soufflet tombe");
    assert!(look(&s, o, [15, 9, 25]).contains(&at(o, [14, 1, 13 + 4])), "l'échelle ramène en bas"); assert!(!look(&s, o, [14, 1, 17]).contains(&at(o, [15, 9, 25])), "mais ne monte pas à la galerie");
    // la coulée : la vanne retire la lave ; on descend, on abaisse le levier du fond, on remonte ; vingt secondes plus tard la lave revient
    assert_eq!(name(&s, at(o, [-12, -3, 21])), "lava"); assert!(!look(&s, o, [-8, 1, 20]).contains(&at(o, [-12, -6, 23])));
    put(&mut s, o, [-8, 1, 20]); assert!(s.lever_at(at(o, fo::VALVE))); run(&mut s, 3.0); assert_eq!(name(&s, at(o, [-12, -3, 21])), "air", "la lave s'est retirée"); assert!(s.notes.iter().any(|t| t.contains("lave se retire")));
    assert!(look(&s, o, [-8, 1, 20]).contains(&at(o, [-12, -6, 23])), "on descend au fond"); put(&mut s, o, [-12, -6, 23]); assert!(s.lever_at(at(o, fo::DEEP))); run(&mut s, 0.2); assert!(!chain(&s, o, 1), "la chaîne de la coulée tombe");
    assert!(look(&s, o, [-12, -6, 23]).contains(&at(o, [-8, 1, 20])), "on remonte à l'échelle"); put(&mut s, o, [-8, 1, 20]);
    run(&mut s, fo::VALVE_T); assert_eq!(name(&s, at(o, [-12, -3, 21])), "lava", "la lave est revenue"); assert!(name(&s, at(o, fo::VALVE)).starts_with("lever") && !name(&s, at(o, fo::VALVE)).starts_with("lever_on"), "la vanne s'est relâchée");
    // les trois chaînes tombées : la Porte du Foyer s'ouvre
    assert!((-1..=1).all(|u| name(&s, at(o, [u, 2, fo::GATE_W])) == "air"), "la grille se lève"); assert!(look(&s, o, [0, 1, 20]).contains(&at(o, [0, 1, 33])), "dans la Fournaise");
    // la sauvegarde garde les plates-formes
    let t = s.save_text(); assert_eq!(t.lines().filter(|l| l.starts_with("lift ")).count(), 2); assert!(t.lines().any(|l| l.starts_with("pool ") && l.ends_with("lava")));
    let mut b2 = { let y = alw_core::gen::FLAT_Y; let mut b = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [1000.5, y as f64 + 1.0, 1000.5]); ready(&mut b); b };
    b2.load_text(&t).unwrap(); assert_eq!(b2.mech.lifts, s.mech.lifts.iter().map(|l| alw_core::mecanismes::Lift { t: 0.0, ..l.clone() }).collect::<Vec<_>>()); assert_eq!(b2.mech.pools[0].liquid, B::LAVA);
    // le Maître de forge
    let sp = alw_core::hostile::species(fo::BOSS).unwrap(); s.horde.foes.clear(); put(&mut s, o, [0, 1, 31]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("le Maître de forge se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    assert!(name(&s, at(o, [0, 2, fo::GATE_W])).starts_with("iron_bars"), "la grille retombe");
    s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "sword", false); run(&mut s, 1.0); assert_eq!(name(&s, at(o, [0, 2, fo::GATE_W])), "air");
    assert!(s.loose.iter().any(|l| l.item == "marteau_forge") || s.inv.iter().flatten().any(|st| st.item == "marteau_forge"), "le marteau du maître");
    let c = at(o, fo::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin"));
}

#[test]
fn la_lave_revient_sur_qui_traine_au_fond() {
    let (mut s, o) = world(); s.vit = [100.0; 3];
    put(&mut s, o, [-8, 1, 20]); assert!(s.lever_at(at(o, fo::VALVE))); run(&mut s, 3.0); put(&mut s, o, [-12, -6, 22]); let hp = s.vit[0];
    for _ in 0..((fo::VALVE_T + 2.0) / DT) as usize { s.tick(DT, Input::default(), Actions::default()); if s.dead.is_some() { break; } }
    assert!(s.dead.is_some() || s.vit[0] < hp - 20.0, "la lave brûle qui est resté au fond ({:.0} PV)", s.vit[0]);
}
