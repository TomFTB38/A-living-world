//! La Tour de l'Horloge : l'horloge de la façade est arrêtée ; les trois rouages couplés barrent la salle tant que leurs encoches
//! ne sont pas en bas ; les aiguilles du cadran, remises à l'heure de la façade, lèvent la grille ; le balancier fauche le pont
//! (et projette qui s'y trouve) ; l'Automate au beffroi. On joue tout, levier après levier, avec le solveur de chemins.
use alw_core::{blocks::*, horloge::{self as ho}, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn session() -> Session { let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [1600.5, y as f64 + 2.0, 1585.5]); ready(&mut s); s }
fn world() -> (Session, [i32; 3]) {
    let mut s = session(); s.set_creative(false); s.hour = 12.0; let y = alw_core::gen::FLAT_Y;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [1600, y, 1600]; s.build_clock_tower(o); let _ = s.sync(); put(&mut s, o, [0, 0, -14]); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
/// Les coordonnées de la tour : v = 0 est le dallage du rez-de-chaussée (la cour est un bloc plus bas).
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] + 1 + q[1], o[2] + q[2]] }
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-13, -1, -18]), at(o, [13, ho::TOP + 2, 14])) }
fn press(s: &mut Session, o: [i32; 3], q: [i32; 3], n: u32) { for _ in 0..n { assert!(s.lever_at(at(o, q)), "un levier en {q:?}"); run(s, 0.1); assert!(!name(s, at(o, q)).starts_with("lever_on"), "le levier se relève"); } }
fn dial(s: &Session, end: &str) -> usize { s.mech.dials.iter().position(|d| d.name.ends_with(end)).unwrap_or_else(|| panic!("le rouage {end}")) }
fn state(s: &Session, end: &str) -> u8 { s.mech.dials[dial(s, end)].state }
/// Le pendule figé sur un cran (sans laisser passer de temps).
fn swing(s: &mut Session, k: u8) { let i = dial(s, "balancier"); s.mech.dials[i].auto = 0.0; s.dial_set(i, k, false); }

#[test]
fn chaque_tour_a_son_heure() {
    let hours: BTreeSet<(u8, u8)> = (0..60).map(|k| ho::stopped([k * 977 - 4000, 60, k * 613])).collect(); assert!(hours.len() > 20, "l'heure change d'une tour à l'autre");
    for k in 0..200 { let o = [k * 331, 60, k * 97 - 9000]; let (h, m) = ho::stopped(o); let (hs, ms) = ho::hands_start(o); assert!(h < 12 && m < 4 && hs < 12 && ms < 4); assert!((hs, ms) != (h, m), "les aiguilles ne sont pas déjà à l'heure");
        assert_ne!(ho::gears_start(o), [0, 0, 0], "les rouages ne sont pas déjà alignés"); }
    // chaque cran d'un engrenage a son encoche, et seule celle d'en bas ouvre le passage
    for s in 0..4u8 { let g = ho::gear(0, s); assert_eq!(g.len(), 28 - 2, "un disque moins l'encoche"); assert_eq!(!g.iter().any(|(c, _)| c[0] == 0 && c[1] <= 2), s == 0, "cran {s}"); }
}

#[test]
fn rouages_cadran_balancier_et_l_automate() {
    let (mut s, o) = world();
    // la cour, la porte ; l'horloge de la façade arrêtée sur son heure
    let (hg, mg) = ho::stopped(o); let r = look(&s, o, [0, 0, -14]); assert!(r.contains(&at(o, [0, 1, -7])), "on entre dans la tour");
    let outside = ho::hand(ho::NORTH - 2, hg, 12, 3.4, false, "brass_dark"); assert!(outside.iter().all(|(c, _)| name(&s, at(o, *c)) == "brass_dark"), "l'aiguille des heures de la façade");
    let outside = ho::hand(ho::NORTH - 3, mg, 4, 4.6, false, "brass"); assert!(outside.iter().all(|(c, _)| name(&s, at(o, *c)) == "brass"), "l'aiguille des minutes de la façade");
    // — les rouages —
    assert!(!r.contains(&at(o, [0, 1, 5])), "les rouages barrent la salle"); let g = ho::gears_start(o); println!("rouages {g:?}, heure {hg} et {} quarts", mg);
    let before = [state(&s, "rouage0"), state(&s, "rouage1"), state(&s, "rouage2")]; assert_eq!(before, g);
    press(&mut s, o, ho::GEAR_LEVERS[0], 1); assert_eq!([state(&s, "rouage0"), state(&s, "rouage1"), state(&s, "rouage2")], [(g[0] + 1) % 4, (g[1] + 1) % 4, g[2]], "le premier levier entraîne les deux premiers rouages");
    assert!(s.notes.iter().any(|t| t.contains("rouages tournent")));
    // le dessin suit : l'encoche du premier rouage est là où dit son cran
    let st = state(&s, "rouage0"); for (c, b) in ho::gear(ho::GEARS[0], st) { assert_eq!(name(&s, at(o, c)), b, "le rouage 0 au cran {st}"); }
    let x1 = (4 - state(&s, "rouage0")) % 4; press(&mut s, o, ho::GEAR_LEVERS[0], x1 as u32); let x2 = (4 - state(&s, "rouage1")) % 4; press(&mut s, o, ho::GEAR_LEVERS[1], x2 as u32);
    let x3 = (4 - state(&s, "rouage2")) % 4; press(&mut s, o, ho::GEAR_LEVERS[2], x3 as u32); assert_eq!([state(&s, "rouage0"), state(&s, "rouage1"), state(&s, "rouage2")], [0, 0, 0]);
    let r = look(&s, o, [0, 1, -7]); assert!(r.contains(&at(o, [0, 1, 5])), "les trois encoches en bas : on passe"); assert!(r.contains(&at(o, [0, 9, 0])), "l'escalier monte au cadran");
    // — le cadran —
    let b = [0, 9, 0]; let r = look(&s, o, b); assert!(!r.contains(&at(o, [-5, 26, 6])), "la grille de l'échelle est fermée"); assert!(name(&s, at(o, ho::GRILLE[0])).starts_with("iron_bars"));
    let (hs, ms) = ho::hands_start(o); assert_eq!((state(&s, "heures"), state(&s, "minutes")), (hs, ms));
    press(&mut s, o, ho::HOUR_LEVER, ((hg as i32 - hs as i32).rem_euclid(12)) as u32); assert!(name(&s, at(o, ho::GRILLE[0])).starts_with("iron_bars"), "l'heure seule ne suffit pas");
    for (c, bk) in ho::hand(ho::NORTH + 2, hg, 12, 3.4, true, "brass_dark") { assert_eq!(name(&s, at(o, c)), bk, "l'aiguille des heures du cadran"); }
    press(&mut s, o, ho::MINUTE_LEVER, ((mg as i32 - ms as i32).rem_euclid(4)) as u32); run(&mut s, 0.2);
    assert!(ho::GRILLE.iter().all(|c| name(&s, at(o, *c)) == "air"), "à l'heure : la grille se lève"); let west = [-5, 26, 6]; assert!(look(&s, o, b).contains(&at(o, west)), "l'échelle mène au palier du balancier");
    // un tour de trop : la grille reste ouverte (l'énigme est résolue pour de bon)
    press(&mut s, o, ho::HOUR_LEVER, 1); run(&mut s, 0.2); assert!(ho::GRILLE.iter().all(|c| name(&s, at(o, *c)) == "air")); press(&mut s, o, ho::HOUR_LEVER, 11);
    // — le balancier — il va et vient tout seul
    let k0 = state(&s, "balancier"); let mut seen = BTreeSet::new(); for _ in 0..40 { run(&mut s, 0.1); seen.insert(state(&s, "balancier")); } assert_eq!(seen.len(), ho::SWING.len(), "le balancier passe par tous ses crans (parti de {k0})");
    let east = [5, 26, 0]; swing(&mut s, 2); assert!(!look(&s, o, west).contains(&at(o, east)), "le balancier barre le pont"); for (c, b) in ho::pendulum(2) { assert_eq!(name(&s, at(o, c)), b); }
    swing(&mut s, 0); assert!(look(&s, o, west).contains(&at(o, east)), "le balancier écarté, on passe"); assert!(ho::pendulum(2).iter().filter(|(c, _)| !ho::pendulum(0).iter().any(|(q, _)| q == c)).all(|(c, _)| name(&s, at(o, *c)) == "air"), "son ancien dessin effacé");
    assert!(look(&s, o, west).contains(&at(o, [0, 22, 0])), "la fosse"); swing(&mut s, 2); assert!(look(&s, o, [0, 22, 0]).contains(&at(o, west)), "de la fosse, l'échelle ramène au palier");
    // il fauche qui se trouve sur le pont
    swing(&mut s, 1); put(&mut s, o, [0, 26, 2]); s.vit = [100.0; 3]; let hp = s.vit[0]; swing(&mut s, 2); assert!(s.vit[0] < hp, "le balancier blesse"); assert!(s.player.v[2] > 3.0, "et projette ({:?})", s.player.v);
    assert!(s.notes.iter().any(|t| t.contains("balancier")));
    // l'échelle du beffroi
    swing(&mut s, 0); assert!(look(&s, o, east).contains(&at(o, [0, 37, 0])), "du palier de l'est, l'échelle monte au beffroi");
    // — l'Automate —
    let sp = alw_core::hostile::species(ho::BOSS).unwrap(); s.horde.foes.clear(); put(&mut s, o, [3, 37, -4]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("l'Automate s'éveille"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "sword", false); run(&mut s, 1.0);
    assert!(s.loose.iter().any(|l| l.item == "coeur_horloge") || s.inv.iter().flatten().any(|st| st.item == "coeur_horloge"), "le Cœur d'horloge");
    let c = at(o, ho::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin")); s.close_chest();
    // la sauvegarde garde les rouages
    let t = s.save_text(); let mut b = session(); b.load_text(&t).unwrap();
    let st = |s: &Session| s.mech.dials.iter().map(|d| (d.name.clone(), d.state, d.goal, d.levers.len(), d.draw.len())).collect::<Vec<_>>(); assert_eq!(st(&b), st(&s));
}
