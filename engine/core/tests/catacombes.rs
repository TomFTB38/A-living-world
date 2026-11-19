//! Les Catacombes des Rois : on descend du mausolée, on se perd dans l'ossuaire, on lit les épitaphes, on franchit le charnier,
//! on réveille les rois dans l'ordre de leurs règnes, la Porte des Morts s'ouvre, le Premier Roi se lève. Et le passage secret.
use alw_core::{blocks::*, catacombes::{self as ca, DEPTH}, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn world() -> (Session, [i32; 3]) {
    let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [800.5, y as f64 + 1.0, 800.5]); ready(&mut s); s.set_creative(false); s.hour = 12.0;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [800, y, 800]; s.build_catacombs(o); let _ = s.sync(); put(&mut s, o, [0, 1, 15]); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] - DEPTH + q[1], o[2] + q[2]] }
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-37, -4, -35]), at(o, [37, DEPTH + 4, 38])) }
fn press(s: &mut Session, o: [i32; 3], q: [i32; 3]) { assert!(s.lever_at(at(o, q)), "un levier en {q:?}"); run(s, 0.15); }
fn gate_shut(s: &Session, o: [i32; 3]) -> bool { (-1..=1).all(|u| (1..=3).all(|v| name(s, at(o, [u, v, ca::GATE_W])) == "great_chain")) }

#[test]
fn chaque_ossuaire_a_ses_chapelles_et_son_levier() {
    for k in 0..300 { let o = [k * 977 - 40_000, 60, k * 613 + 1234];
        for side in [-1, 1] { let w = ca::wing(o, side); assert_eq!(w.open.len() as i32, ca::NX * ca::NZ - 1, "un labyrinthe parfait"); assert_eq!(w.chapels.len(), 2, "deux chapelles");
            assert_eq!(w.lever.is_some(), side > 0, "le levier secret, à l'est seulement"); for (c, _) in &w.chapels { assert!(*c != ca::ENTRY && *c != ca::EXIT); } }
        let r = ca::reign(o); let mut seen = r.to_vec(); seen.sort(); assert_eq!(seen, vec![0, 1, 2, 3], "un ordre des règnes");
    }
    let orders: BTreeSet<[usize; 4]> = (0..50).map(|k| ca::reign([k * 311, 0, k * 97])).collect(); assert!(orders.len() > 8, "l'ordre change d'une catacombe à l'autre");
}

#[test]
fn on_descend_on_lit_les_epitaphes_on_reveille_les_rois_dans_l_ordre() {
    let (mut s, o) = world();
    // du haut de l'allée, l'escalier du mausolée mène au vestibule ; de là, les deux galeries mènent à la crypte
    let top = [0, DEPTH + 1, -20]; assert_eq!(name(&s, at(o, [-2, DEPTH + 1, -8])), "column_base", "le portique du mausolée");
    let r = look(&s, o, top); println!("{} cases atteintes depuis l'allée", r.len());
    assert!(r.contains(&at(o, [0, 1, 15])), "l'escalier mène au vestibule"); assert!(look(&s, o, [0, 1, 15]).contains(&at(o, top)), "et l'on remonte");
    for side in [-1, 1] { assert!(r.contains(&at(o, [8 * side, 1, -22])), "la galerie {side} mène à la crypte"); }
    assert!(r.contains(&at(o, [-7, 1, -29])) && r.contains(&at(o, [6, 1, -30])), "au pied des tombeaux"); assert!(!r.contains(&at(o, [0, 1, 30])), "le tombeau du Premier Roi est clos"); assert!(!r.contains(&at(o, [-33, 1, 1])), "la chambre secrète aussi");
    for k in 0..4 { let ((u, w), _) = ca::chapel(o, k); assert!(r.contains(&at(o, [u, 1, w])) || r.contains(&at(o, [u + 1, 1, w])) || r.contains(&at(o, [u, 1, w + 1])) || r.contains(&at(o, [u + 1, 1, w + 1])), "la chapelle {k}"); }
    let sl = ca::secret_lever(o); assert!(name(&s, at(o, sl)).starts_with("lever"), "le levier secret"); assert!(r.contains(&at(o, [sl[0], 1, sl[2]])), "on l'atteint");
    // les épitaphes disent l'ordre des règnes
    let rank = ca::reign(o); let ord = ["en premier", "en deuxième", "en troisième", "en dernier"];
    for k in 0..4 { let ((u, w), _) = ca::chapel(o, k); s.notes.clear(); put(&mut s, o, [u, 1, w]); run(&mut s, 0.1); let n = s.notes.iter().find(|t| t.contains("Ici repose")).unwrap_or_else(|| panic!("l'épitaphe {k} : {:?}", s.notes)); assert!(n.contains(ca::KINGS[k].0) && n.contains(ord[rank[k]]), "{n}"); }
    // le charnier : la passerelle cède sous les pieds ; on remonte par l'échelle
    put(&mut s, o, [8, 1, -27]); run(&mut s, 1.2); assert_eq!(name(&s, at(o, [8, 0, -27])), "air", "la passerelle a cédé"); assert!(s.player.p[1] < (o[1] - DEPTH) as f64, "on est tombé dans le charnier");
    assert!(look(&s, o, [8, -2, -27]).contains(&at(o, [7, 1, -30])), "l'échelle ramène au pied des tombeaux");
    // un faux pas : les leviers se relèvent, les fléchettes partent
    let wrong = (0..4).find(|k| rank[*k] == 1).unwrap(); put(&mut s, o, [0, 1, -30]); s.horde.shots.clear(); press(&mut s, o, ca::lever(wrong));
    assert!(name(&s, at(o, ca::lever(wrong))).starts_with("lever") && !name(&s, at(o, ca::lever(wrong))).starts_with("lever_on"), "le levier s'est relevé"); assert!(!s.horde.shots.is_empty(), "les fléchettes"); assert!(gate_shut(&s, o));
    // dans l'ordre : la Porte des Morts s'ouvre
    let mut order: Vec<usize> = (0..4).collect(); order.sort_by_key(|k| rank[*k]); for (i, k) in order.iter().enumerate() { press(&mut s, o, ca::lever(*k)); assert_eq!(gate_shut(&s, o), i < 3, "après le levier {i}"); }
    println!("{:?}", s.notes.iter().rev().take(2).collect::<Vec<_>>()); let r = look(&s, o, [0, 1, 15]); assert!(r.contains(&at(o, [0, 1, 30])), "par le Couloir des Lames, au tombeau du Premier Roi");
    // le Couloir des Lames : chaque rang de pics jaillit à son tour
    let mut up = [false; 3]; put(&mut s, o, [0, 1, 17]); for _ in 0..(3.0 / DT) as usize { run(&mut s, DT); for (k, w) in ca::SPIKE_ROWS.iter().enumerate() { if name(&s, at(o, [0, 1, *w])) == "spikes" { up[k] = true; } } } assert_eq!(up, [true; 3], "les pics en vague");
    // le Premier Roi
    let sp = alw_core::hostile::species(ca::BOSS).unwrap(); s.horde.foes.clear(); put(&mut s, o, [0, 1, 29]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("le Premier Roi se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    assert!(name(&s, at(o, [0, 2, ca::TOMB_DOOR_W])).starts_with("iron_bars"), "la grille retombe");
    s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "sword", false); run(&mut s, 1.0); assert_eq!(name(&s, at(o, [0, 2, ca::TOMB_DOOR_W])), "air");
    assert!(s.loose.iter().any(|l| l.item == "sceptre_des_rois") || s.inv.iter().flatten().any(|st| st.item == "sceptre_des_rois"), "le Sceptre des Rois");
    let c = at(o, ca::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin")); s.close_chest();
}

#[test]
fn le_passage_secret() {
    let (mut s, o) = world();
    let pane = at(o, [-29, 2, 1]); assert!(name(&s, pane).starts_with("ossuary_wall"), "un pan du mur de crânes"); assert!(!look(&s, o, [0, 1, 15]).contains(&at(o, [-33, 1, 1])));
    let sl = ca::secret_lever(o); put(&mut s, o, [sl[0], 1, sl[2]]); press(&mut s, o, sl); assert_eq!(name(&s, pane), "air", "le pan de mur pivote");
    assert!(s.notes.iter().any(|t| t.contains("pan de mur")), "{:?}", s.notes); assert!(look(&s, o, [0, 1, 15]).contains(&at(o, [-33, 1, 1])), "la chambre secrète");
    let xp = s.prog.xp; put(&mut s, o, [-33, 1, 1]); run(&mut s, 0.1); assert!(s.prog.xp > xp, "une découverte"); let c = at(o, ca::SECRET_CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().count() > 0, "un trésor oublié");
}
