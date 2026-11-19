//! Le Sanctuaire englouti, salle par salle : chaque énigme se résout avec les gestes du joueur (leviers, blocs poussés, pas
//! comptés), chaque grille tient tant que l'énigme n'est pas résolue, et le solveur de chemins (`reach`) le vérifie : avant,
//! la salle suivante est hors d'atteinte ; après, on y va. Le couloir des pièges et la nef se parcourent pour de vrai.
use alw_core::{blocks::*, player::*, sanctuaire::{self as sa, DEPTH}, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;
type Room = (i32, i32, i32, i32, i32);

fn ready(s: &mut Session) { for _ in 0..8000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
/// Le monde plat, le sanctuaire bâti sous l'origine ; le joueur dans la salle de l'eau noire.
fn world() -> (Session, [i32; 3]) {
    let seed = alw_core::gen::FLAT | 0x123; let y = alw_core::gen::FLAT_Y;
    let mut s = Session::new(seed, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, y as f64 + 1.0, 0.5]); ready(&mut s); s.set_creative(false); s.hour = 12.0;
    for _ in 0..10 { s.tick(DT, Input::default(), Actions::default()); } s.wild.all.clear(); s.horde.foes.clear();
    let o = [0, y, 0]; s.build_sanctuary(o); let _ = s.sync(); put(&mut s, o, [0, 1, 4]); s.vit = [100.0; 3]; s.notes.clear(); (s, o)
}
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] - DEPTH + q[1], o[2] + q[2]] }
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
/// Le joueur, les pieds dans la case relative `q`.
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT) as usize { s.vit[1] = 100.0; s.vit[2] = 100.0; s.tick(DT, Input::default(), Actions::default()); } }
/// Où peut-on aller depuis la case relative `q` ?
fn reach(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { sa::reach(&s.world, at(o, q), at(o, [-26, -3, -18]), at(o, [26, DEPTH + 4, 30])) }
fn into(set: &BTreeSet<[i32; 3]>, o: [i32; 3], r: Room) -> bool { set.iter().any(|c| { let (u, v, w) = (c[0] - o[0], c[1] - (o[1] - DEPTH), c[2] - o[2]); u >= r.0 && u <= r.1 && w >= r.2 && w <= r.3 && v >= 1 && v <= r.4 }) }
fn bars(s: &Session, o: [i32; 3], q: [i32; 3]) -> bool { name(s, at(o, q)).starts_with("iron_bars") }
/// Pousser le bloc `b` depuis la case `from` (relatives), comme d'un clic droit.
fn push(s: &mut Session, o: [i32; 3], from: [i32; 3], b: [i32; 3]) { put(s, o, from); run(s, 0.05); put(s, o, from); assert!(s.push_at(at(o, b)), "pas de bloc en {b:?}"); run(s, 0.15); }
fn lever(s: &mut Session, o: [i32; 3], l: [i32; 3]) { assert!(s.lever_at(at(o, l)), "pas de levier en {l:?}"); run(s, 0.2); }
/// Marcher jusqu'au centre de la case relative (u, w), en tenant Saut si besoin.
fn go(s: &mut Session, o: [i32; 3], u: i32, w: i32, jump: bool) -> bool {
    let t = [(o[0] + u) as f64 + 0.5, (o[2] + w) as f64 + 0.5];
    for _ in 0..(8.0 / DT) as usize { let p = s.player.p; let d = [t[0] - p[0], t[1] - p[2]]; let l = d[0].hypot(d[1]); if l < 0.2 { s.player.v[0] = 0.0; s.player.v[2] = 0.0; return true; }
        s.player.yaw = d[0].atan2(d[1]); s.vit[1] = 100.0; s.vit[2] = 100.0; s.tick(DT, Input { z: (l / 0.5).clamp(0.35, 1.0), jump, ..Input::default() }, Actions::default()); }
    false
}
fn solve_r1(s: &mut Session, o: [i32; 3]) { for l in sa::R1_LEVERS { lever(s, o, l); } }
fn solve_r2(s: &mut Session, o: [i32; 3]) {
    // le premier bloc : deux fois vers l'ouest, deux fois vers le nord, sur la dalle
    push(s, o, [-17, 1, 20], [-18, 1, 20]); push(s, o, [-18, 1, 20], [-19, 1, 20]); push(s, o, [-20, 1, 21], [-20, 1, 20]); push(s, o, [-20, 1, 20], [-20, 1, 19]);
    // le second : deux fois vers l'est, deux fois vers le sud
    push(s, o, [-17, 1, 22], [-16, 1, 22]); push(s, o, [-16, 1, 22], [-15, 1, 22]); push(s, o, [-14, 1, 21], [-14, 1, 22]); push(s, o, [-14, 1, 22], [-14, 1, 23]);
}

#[test]
fn le_sanctuaire_est_bati_et_ses_salles_se_suivent_fermees() {
    let (s, o) = world();
    for (r, what) in [(sa::HALL, "l'eau noire"), (sa::R1, "salle 1"), (sa::R2, "salle 2"), (sa::R3, "salle 3"), (sa::R4, "salle 4"), (sa::R5, "salle 5"), (sa::R6, "salle 6")] {
        for u in [r.0, r.1] { for w in [r.2, r.3] { let c = at(o, [u, r.4, w]); assert!(!is_solid(s.world.get(c[0], c[1], c[2]).unwrap()), "{what} creusée jusqu'à la voûte ({}, {u} {w})", name(&s, c)); assert!(is_solid(s.world.get(c[0], c[1] + 1, c[2]).unwrap()), "{what} a sa voûte"); } } }
    // depuis la surface, l'escalier descend ; depuis l'eau noire on remonte, on va à la salle des leviers, et pas plus loin
    let top = reach(&s, o, [0, DEPTH + 1, -16]); assert!(into(&top, o, sa::HALL), "on descend l'escalier");
    let r = reach(&s, o, [0, 1, 4]); assert!(r.contains(&at(o, [0, DEPTH + 1, -16])), "on remonte à la surface");
    assert!(into(&r, o, sa::R1)); for (room, n) in [(sa::R2, 2), (sa::R3, 3), (sa::R4, 4), (sa::R5, 5), (sa::R6, 6)] { assert!(!into(&r, o, room), "salle {n} hors d'atteinte au départ"); }
    // le coffre, sur son autel, au fond
    let c = at(o, sa::CHEST); assert!(is_chest_shut(s.world.get(c[0], c[1], c[2]).unwrap())); assert!(name(&s, [c[0], c[1] - 1, c[2]]).starts_with("altar"), "un autel");
    println!("fils : {:?}", s.mech.wires.iter().map(|w| w.name.as_str()).collect::<Vec<_>>()); assert_eq!(s.mech.wires.len(), 7); assert_eq!(s.mech.pulses.len(), 4); assert_eq!(s.mech.hints.len(), 8); assert_eq!(s.mech.guards.len(), 1); assert_eq!(s.mech.homes.len(), 3);
}

#[test]
fn salle_1_les_bannieres_donnent_l_ordre_des_leviers() {
    let (mut s, o) = world(); put(&mut s, o, [-19, 1, 7]); run(&mut s, 0.3); println!("{:?}", s.notes); assert!(s.notes.iter().any(|n| n.contains("Trois leviers")), "l'inscription");
    // l'indice dit vrai : une bannière au-dessus du premier levier de la solution, deux au-dessus du deuxième, trois du troisième
    for (k, l) in sa::R1_LEVERS.iter().enumerate() { let n = (1..=4).filter(|v| name(&s, at(o, [l[0], l[1] + v, l[2]])).starts_with("banner_gold")).count(); assert_eq!(n, k + 1, "bannières au-dessus du levier {l:?}"); }
    // un faux pas : les leviers se relèvent et les masques crachent
    let shots = s.horde.shots.len(); lever(&mut s, o, sa::R1_LEVERS[1]); assert!(s.notes.iter().any(|n| n.contains("déclic")), "{:?}", s.notes); assert!(s.horde.shots.len() > shots || s.vit[0] < 100.0, "des fléchettes");
    assert!(name(&s, at(o, sa::R1_LEVERS[1])).starts_with("lever") && !name(&s, at(o, sa::R1_LEVERS[1])).starts_with("lever_on"), "relevé"); assert!(bars(&s, o, [-18, 1, 13]), "la grille tient");
    assert!(!into(&reach(&s, o, [-19, 1, 7]), o, sa::R2));
    // dans l'ordre : la grille se lève, la salle 2 est là
    solve_r1(&mut s, o); println!("{:?}", s.notes.last()); assert!(!bars(&s, o, [-18, 1, 13]) && !bars(&s, o, [-17, 2, 13]), "la grille est levée");
    let r = reach(&s, o, [-19, 1, 7]); assert!(into(&r, o, sa::R2) && !into(&r, o, sa::R3));
}

#[test]
fn salle_2_deux_blocs_sur_deux_dalles_et_ils_reviennent_si_on_s_en_va() {
    let (mut s, o) = world(); solve_r1(&mut s, o);
    let r = reach(&s, o, [-17, 1, 15]); for c in [[-17, 1, 20], [-20, 1, 21], [-17, 1, 22], [-14, 1, 21]] { assert!(r.contains(&at(o, c)), "on peut se placer en {c:?} pour pousser"); }
    // un bloc à moitié poussé, et l'on s'en va : il revient à sa place
    push(&mut s, o, [-17, 1, 20], [-18, 1, 20]); assert_eq!(name(&s, at(o, [-19, 1, 20])), "push_block"); put(&mut s, o, [0, 1, 4]); run(&mut s, 1.2);
    assert_eq!(name(&s, at(o, sa::R2_BLOCKS[0])), "push_block", "revenu"); assert_eq!(name(&s, at(o, [-19, 1, 20])), "air"); println!("{:?}", s.notes.last());
    // les deux blocs : un seul ne suffit pas
    push(&mut s, o, [-17, 1, 20], [-18, 1, 20]); push(&mut s, o, [-18, 1, 20], [-19, 1, 20]); push(&mut s, o, [-20, 1, 21], [-20, 1, 20]); push(&mut s, o, [-20, 1, 20], [-20, 1, 19]);
    put(&mut s, o, [-17, 1, 18]); run(&mut s, 0.3); assert_eq!(name(&s, at(o, sa::R2_PLATES[0])), "pressure_plate_on"); assert!(bars(&s, o, [-11, 1, 21]), "un seul bloc : la grille tient");
    push(&mut s, o, [-17, 1, 22], [-16, 1, 22]); push(&mut s, o, [-16, 1, 22], [-15, 1, 22]); push(&mut s, o, [-14, 1, 21], [-14, 1, 22]); push(&mut s, o, [-14, 1, 22], [-14, 1, 23]);
    put(&mut s, o, [-17, 1, 18]); run(&mut s, 0.3); assert!(!bars(&s, o, [-11, 1, 21]), "les deux : la grille se lève"); assert!(into(&reach(&s, o, [-17, 1, 18]), o, sa::R3));
    // résolue, on peut partir : les blocs restent sur leurs dalles
    put(&mut s, o, [0, 1, 4]); run(&mut s, 1.2); assert_eq!(name(&s, at(o, [-20, 1, 18])), "push_block"); assert!(!bars(&s, o, [-11, 1, 21]));
}

/// Le couloir des pièges, parcouru pour de vrai : on attend que les pics retombent, puis (si `cover`) on pousse le bloc
/// devant soi jusqu'au fond, et l'on sort entre deux volées. Rend la santé perdue et si l'on est arrivé au bout.
fn corridor(cover: bool) -> (f64, bool) {
    let (mut s, o) = world(); if !cover { let b = at(o, sa::R3_BLOCK); s.world.edit(b[0], b[1], b[2], 0); }
    put(&mut s, o, [-10, 1, 21]); run(&mut s, 0.2);
    let up = |s: &Session, u: i32| name(s, at(o, [u, 1, 21])) == "spikes";
    for (row, stop) in [(sa::R3_SPIKES[0][0], -5), (sa::R3_SPIKES[1][0], -1)] {
        assert!(go(&mut s, o, row - 2, 21, false)); let mut t = 0.0; while !up(&s, row) && t < 4.0 { run(&mut s, DT); t += DT; } while up(&s, row) && t < 8.0 { run(&mut s, DT); t += DT; }
        assert!(go(&mut s, o, stop, 21, false), "passer la rangée {row}"); }
    println!("après les pics : santé {:.0}", s.vit[0]);
    if cover { let mut b = sa::R3_BLOCK; while b[0] < 9 { push(&mut s, o, [b[0] - 1, 1, 21], b); b[0] += 1; assert_eq!(name(&s, at(o, b)), "push_block"); go(&mut s, o, b[0] - 1, 21, false); } }
    else { go(&mut s, o, 8, 21, false); }
    // au fond : on attend une volée, et l'on file au sud
    let n = s.horde.shots.len(); let mut t = 0.0; while s.horde.shots.len() <= n && t < 3.0 { run(&mut s, DT); t += DT; } run(&mut s, 0.12);
    let ok = go(&mut s, o, 8, 22, false) && go(&mut s, o, 8, 24, false) && go(&mut s, o, 10, 24, false); println!("{} : santé {:.0}, {:?}", if cover { "à l'abri du bloc" } else { "à découvert" }, s.vit[0], s.notes);
    (100.0 - s.vit[0], ok)
}
#[test]
fn salle_3_on_observe_le_rythme_et_on_s_abrite_derriere_le_bloc() {
    let (lost, ok) = corridor(true); assert!(ok, "au bout du couloir"); assert!(lost <= 7.5, "presque indemne derrière le bloc : {lost:.0} perdus");
    let (bare, _) = corridor(false); assert!(bare >= lost + 7.0, "à découvert, les fléchettes touchent : {bare:.0} contre {lost:.0}");
}

#[test]
fn salle_4_on_gravit_la_nef_et_le_levier_ouvre_la_grille_d_en_bas() {
    let (mut s, o) = world(); let ledge = [17, 6, 17];
    assert!(reach(&s, o, [11, 1, 24]).contains(&at(o, ledge)), "le solveur trouve la galerie"); assert!(!into(&reach(&s, o, [11, 1, 24]), o, sa::R5), "la grille d'en bas tient");
    // le premier degré ôté, la galerie est hors d'atteinte (le solveur ne saute pas deux blocs)
    { let (mut t, o2) = world(); let p = at(o2, [13, 1, 26]); t.world.edit(p[0], p[1], p[2], 0); assert!(!reach(&t, o2, [11, 1, 24]).contains(&at(o2, ledge)), "sans les degrés, pas de chemin"); }
    // et pour de vrai : on monte les degrés, on suit la corniche, on arrive au levier
    put(&mut s, o, [11, 1, 24]); run(&mut s, 0.1); let mut path = vec![(12, 25)]; for u in 13..=22 { path.push((u, 26)); } for w in (17..=25).rev() { path.push((22, w)); } path.push((18, 17));
    for (u, w) in path { assert!(go(&mut s, o, u, w, w == 26), "coincé avant {u} {w} : {:?}", s.player.p); }
    let f = o[1] - DEPTH; println!("en haut : {:?} (galerie à {})", s.player.p, f + 6); assert!(s.player.p[1] >= (f + 6) as f64 - 0.05, "sur la galerie");
    lever(&mut s, o, sa::R4_LEVER); println!("{:?}", s.notes.last()); assert!(!bars(&s, o, [13, 1, 15]), "la grille d'en bas est levée"); assert!(into(&reach(&s, o, [11, 1, 24]), o, sa::R5));
}

#[test]
fn salle_5_le_poids_le_piege_et_l_ordre() {
    let (mut s, o) = world(); put(&mut s, o, [13, 1, 12]); run(&mut s, 0.3); assert!(s.notes.iter().any(|n| n.contains("L'épreuve")));
    let r = reach(&s, o, [13, 1, 12]); assert!(!r.contains(&at(o, [17, 1, 2])), "les leviers derrière la grille"); assert_eq!(name(&s, at(o, [17, 1, 7])), "spikes", "le piège est armé");
    // tricher (abaisser les leviers d'abord) ne suffit pas : il faut aussi le poids sur la dalle
    for l in sa::R5_LEVERS { lever(&mut s, o, l); } assert!(bars(&s, o, [17, 1, 1]), "sans la dalle, la porte tient");
    for l in sa::R5_LEVERS { s.lever_at(at(o, l)); } run(&mut s, 0.2);
    // le bloc sur la dalle : les pics rentrent, les masques se taisent, la grille des leviers se lève
    push(&mut s, o, [17, 1, 10], [18, 1, 10]); push(&mut s, o, [18, 1, 10], [19, 1, 10]); run(&mut s, 0.3);
    assert_eq!(name(&s, at(o, [17, 1, 7])), "spikes_down", "désarmé"); assert!(!bars(&s, o, [17, 1, 4]), "la grille des leviers"); assert!(reach(&s, o, [13, 1, 12]).contains(&at(o, [17, 1, 2])));
    assert!(bars(&s, o, [17, 1, 1]), "la porte tient encore");
    // dans le désordre : tout se relève ; dans l'ordre des bannières bleues : la porte s'ouvre
    put(&mut s, o, [17, 1, 3]); lever(&mut s, o, sa::R5_LEVERS[1]); assert!(s.notes.iter().any(|n| n.contains("déclic")));
    for (k, l) in sa::R5_LEVERS.iter().enumerate() { let n = (1..=3).filter(|v| name(&s, at(o, [l[0], l[1] + v, l[2]])).starts_with("banner_blue")).count(); assert_eq!(n, k + 1); }
    for l in sa::R5_LEVERS { lever(&mut s, o, l); } assert!(!bars(&s, o, [17, 1, 1]), "la dernière porte"); assert!(into(&reach(&s, o, [17, 1, 3]), o, sa::R6));
}

#[test]
fn salle_6_le_tresor_et_le_raccourci() {
    let (mut s, o) = world(); put(&mut s, o, [17, 1, -3]); run(&mut s, 0.2);
    assert!(!into(&reach(&s, o, [17, 1, -3]), o, sa::HALL), "pas encore de raccourci");
    s.open_chest(at(o, sa::CHEST)); let got: Vec<&str> = s.chests[&at(o, sa::CHEST)].iter().flatten().map(|st| st.item).collect(); println!("le trésor : {got:?}"); assert!(got.contains(&"idole_engloutie")); s.close_chest(); s.inv_open = false;
    put(&mut s, o, [5, 1, -1]); lever(&mut s, o, sa::R6_LEVER); assert!(!bars(&s, o, [4, 1, 1]), "le raccourci s'ouvre");
    assert!(into(&reach(&s, o, [17, 1, -3]), o, sa::HALL), "de l'idole à l'entrée"); assert!(into(&reach(&s, o, [0, 1, 4]), o, sa::R6), "et de l'entrée à l'idole");
}

#[test]
fn de_bout_en_bout_chaque_porte_n_ouvre_qu_apres_son_enigme_et_tout_se_sauvegarde() {
    let (mut s, o) = world(); let from = [0, 1, 4]; let f = o[1] - DEPTH;
    let steps: [(&str, Room); 5] = [("r1", sa::R2), ("r2", sa::R3), ("r4", sa::R5), ("r5", sa::R6), ("r6", sa::HALL)];
    for (k, (what, next)) in steps.iter().enumerate() {
        if *what != "r6" { assert!(!into(&reach(&s, o, from), o, *next), "avant {what}, la suite est fermée"); }
        match *what { "r1" => solve_r1(&mut s, o), "r2" => solve_r2(&mut s, o), "r4" => { put(&mut s, o, [17, 6, 17]); lever(&mut s, o, sa::R4_LEVER); }
            "r5" => { push(&mut s, o, [17, 1, 10], [18, 1, 10]); push(&mut s, o, [18, 1, 10], [19, 1, 10]); put(&mut s, o, [17, 1, 3]); run(&mut s, 0.2); for l in sa::R5_LEVERS { lever(&mut s, o, l); } }
            _ => { put(&mut s, o, [5, 1, -1]); lever(&mut s, o, sa::R6_LEVER); } }
        put(&mut s, o, [0, 1, 4]); run(&mut s, 0.2);
        let r = reach(&s, o, from); assert!(into(&r, o, *next), "après {what} ({k}), on passe : {:?}", s.mech.wires.iter().map(|w| (&w.name, w.on)).collect::<Vec<_>>()); }
    let r = reach(&s, o, from); assert!(r.contains(&at(o, [17, 6, 17])) && r.contains(&at(o, [17, 1, -3])), "tout le sanctuaire, depuis l'entrée");
    let _ = f;
    // la sauvegarde garde les fils, les rythmes, les retours, les inscriptions, le coffre au trésor
    let t = s.save_text(); let (mut b, _) = { let seed = alw_core::gen::FLAT | 0x123; let y = alw_core::gen::FLAT_Y; let mut b = Session::new(seed, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, y as f64 + 1.0, 0.5]); ready(&mut b); (b, ()) };
    b.load_text(&t).unwrap();
    assert_eq!(b.mech.wires.iter().map(|w| (&w.name, w.on)).collect::<Vec<_>>(), s.mech.wires.iter().map(|w| (&w.name, w.on)).collect::<Vec<_>>()); assert_eq!(b.mech.homes, s.mech.homes); assert_eq!(b.mech.hints, s.mech.hints); assert_eq!(b.mech.chests, s.mech.chests); assert_eq!(b.mech.pulses.len(), s.mech.pulses.len()); assert_eq!(b.mech.seals, s.mech.seals);
}

#[test]
fn la_pierre_ancienne_ne_se_creuse_pas() {
    let (s, o) = world(); let id = |n: &str| by_name(n).unwrap();
    assert!(s.sealed(at(o, [-6, 2, 3]), id("stone")), "un mur"); assert!(s.sealed(at(o, [-18, 1, 13]), id("iron_bars")), "une grille"); assert!(s.sealed(at(o, sa::R2_BLOCKS[0]), id("push_block")), "un bloc à pousser");
    assert!(!s.sealed(at(o, sa::R1_LEVERS[0]), id("lever")), "un levier"); assert!(!s.sealed(at(o, [0, 1, 4]), id("torch")), "on peut poser une torche");
    assert!(!s.sealed(at(o, [0, DEPTH + 1, -20]), id("stone")), "la surface reste libre");
}

#[test]
fn la_salle_secrete_s_ouvre_par_la_dalle_de_la_galerie() {
    let (mut s, o) = world(); let inside = (sa::SECRET, sa::SECRET_FLOOR); let gallery = [17, 6, 17];
    let secret = |set: &BTreeSet<[i32; 3]>| set.iter().any(|c| { let (u, v, w) = (c[0] - o[0], c[1] - (o[1] - DEPTH), c[2] - o[2]); u >= inside.0 .0 && u <= inside.0 .1 && w >= inside.0 .2 && w <= inside.0 .3 && v == inside.1 + 1 });
    assert!(!secret(&reach(&s, o, gallery)), "un mur comme un autre");
    // on marche jusqu'à l'angle de la galerie : la dalle s'enfonce, le pan de mur pivote, et reste ouvert
    put(&mut s, o, gallery); run(&mut s, 0.1); assert!(go(&mut s, o, 12, 17, false)); run(&mut s, 0.3); println!("{:?}", s.notes);
    assert!(s.notes.iter().any(|n| n.contains("pan de mur")), "le passage s'ouvre"); assert_eq!(name(&s, at(o, [11, 6, 16])), "air");
    assert!(go(&mut s, o, 14, 17, false)); run(&mut s, 0.5); assert_eq!(name(&s, at(o, [11, 6, 16])), "air", "il reste ouvert"); assert!(secret(&reach(&s, o, gallery)));
    // on entre : l'inscription, de l'expérience ; le coffre caché a ses offrandes
    let xp = s.prog.xp; assert!(go(&mut s, o, 11, 17, false) && go(&mut s, o, 9, 16, false)); run(&mut s, 0.3); println!("{:?}", s.notes.iter().rev().take(2).collect::<Vec<_>>());
    assert!(s.notes.iter().any(|n| n.contains("salle secrète")) && s.prog.xp > xp, "découverte récompensée");
    s.open_chest(at(o, sa::SECRET_CHEST)); let got: Vec<&str> = s.chests[&at(o, sa::SECRET_CHEST)].iter().flatten().map(|st| st.item).collect(); println!("coffre caché : {got:?}"); assert!(got.contains(&"steel_shield"));
}

#[test]
fn le_colosse_englouti_garde_l_idole() {
    let (mut s, o) = world(); let sp = alw_core::hostile::species(sa::BOSS).unwrap();
    let boss = |s: &Session| s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0);
    put(&mut s, o, [17, 1, -3]); run(&mut s, 0.3); println!("{:?}", s.notes);
    let i = boss(&s).expect("le colosse s'éveille"); assert!(s.horde.foes[i].kept); assert!(bars(&s, o, [17, 1, -1]) && bars(&s, o, [11, 1, -6]), "les deux issues se ferment");
    assert!(!into(&reach(&s, o, [17, 1, -3]), o, sa::R5), "on ne fuit pas"); let (hp, at) = (s.horde.foes[i].hp, s.horde.foes[i].p); println!("colosse : {hp:.0} PV en {at:?}"); assert!(hp >= 120.0);
    // on s'en va (ou l'on meurt) : il se rendort, tout se rouvre ; on revient, il se relève
    put(&mut s, o, [0, 1, 4]); run(&mut s, 0.2); assert!(boss(&s).is_none() && !bars(&s, o, [17, 1, -1]), "rendormi, la porte rouverte");
    put(&mut s, o, [17, 1, -3]); run(&mut s, 0.2); let i = boss(&s).expect("il se relève");
    // à mi-vie, les noyés remontent
    let noye = alw_core::hostile::species("noye").unwrap(); s.horde.foes[i].hp = s.horde.foes[i].max * 0.45; s.horde.foes[i].p = at; run(&mut s, 0.6);
    println!("{:?}", s.notes.last()); assert!(s.notes.iter().any(|n| n.contains("noyés remontent")) && s.horde.foes.iter().filter(|f| f.sp == noye).count() >= 2);
    // vaincu : les grilles se relèvent, le harpon tombe, l'expérience d'un boss
    let xp = s.prog.xp; let i = boss(&s).unwrap(); s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "sword", false); run(&mut s, 1.0);
    println!("{:?}", s.notes.iter().rev().take(3).collect::<Vec<_>>()); assert!(s.notes.iter().any(|n| n.contains("Le gardien est vaincu"))); assert!(!bars(&s, o, [17, 1, -1]) && !bars(&s, o, [11, 1, -6]));
    assert!(s.loose.iter().any(|l| l.item == "harpon_englouti") || s.inv.iter().flatten().any(|st| st.item == "harpon_englouti"), "le harpon englouti"); assert!(s.prog.xp - xp >= 300.0, "{} XP", s.prog.xp - xp);
    put(&mut s, o, [0, 1, 4]); run(&mut s, 0.2); put(&mut s, o, [17, 1, -3]); run(&mut s, 0.3); assert!(boss(&s).is_none(), "vaincu pour de bon");
    // et ça se sauvegarde
    let t = s.save_text(); assert!(t.lines().any(|l| l.starts_with("guard") && l.contains("colosse") && l.split_whitespace().nth(5) == Some("2")), "{}", t.lines().filter(|l| l.starts_with("guard")).collect::<Vec<_>>().join("\n"));
}
