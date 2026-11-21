//! La Tour de l'Archimage : des salles closes de toutes parts, que seuls les portails relient ; le cadran du laboratoire qui
//! change la couleur du portail du nord ; le grimoire sans lequel le portail d'or vacille ; le cachot qui se referme et ses deux
//! vagues ; l'Archimage dans son Sanctuaire ; la sauvegarde.
use alw_core::{archimage::{self as am, DEPTH}, blocks::*, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn session() -> Session { let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [2000.5, y as f64 + 2.0, 1985.5]); ready(&mut s); s }
/// Une case de la tour : `v` compté depuis le sol des salles.
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] - DEPTH + q[1], o[2] + q[2]] }
const COURT: [i32; 3] = [0, DEPTH + 1, -9];
fn world() -> (Session, [i32; 3]) {
    let mut s = session(); s.set_creative(false); s.hour = 12.0; let y = alw_core::gen::FLAT_Y;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [2000, y, 2000]; s.build_mage_tower(o); let _ = s.sync(); put(&mut s, o, COURT); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-32, -1, -32]), at(o, [32, DEPTH + 10, 34])) }
fn cell(s: &Session, o: [i32; 3]) -> [i32; 3] { let p = s.player.p; [p[0].floor() as i32 - o[0], (p[1] + 0.05).floor() as i32 - (o[1] - DEPTH), p[2].floor() as i32 - o[2]] }
/// On se tient sur une dalle, le temps qu'elle vous prenne ; rend où l'on est.
fn step_on(s: &mut Session, o: [i32; 3], pad: [i32; 3]) -> [i32; 3] { put(s, o, [pad[0], pad[1] + 1, pad[2]]); run(s, alw_core::mecanismes::PORTAL_T + 0.2); cell(s, o) }
fn dial(s: &Session) -> u8 { s.mech.dials.iter().find(|d| d.name.ends_with("cadran")).unwrap().state }

#[test]
fn plans_salles_closes_et_empreinte() {
    let ar = am::arrivals(); let rooms = [am::HALL, am::LIBRARY, am::LAB, am::CELL, am::SANCTUM];
    for (k, r) in rooms.iter().enumerate() { let a = ar[k]; assert!(a[0] >= r.0 && a[0] <= r.1 && a[2] >= r.2 && a[2] <= r.3, "on arrive dans la salle {k}"); }
    for (i, a) in rooms.iter().enumerate() { for b in rooms.iter().skip(i + 1) { let gap = (b.0 - a.1).max(a.0 - b.1).max(b.2 - a.3).max(a.2 - b.3); assert!(gap >= 8, "les salles sont loin l'une de l'autre ({gap})"); } }
    let (lo, hi) = am::footprint(); println!("empreinte {lo:?}..{hi:?}"); assert!(lo[0] >= -38 && lo[1] >= -38 && hi[0] <= 38 && hi[1] <= 38, "la tour tient dans la portée des empreintes");
}

#[test]
fn portails_cadran_grimoire_cachot_et_archimage() {
    let (mut s, o) = world(); let ar = am::arrivals();
    // — à pied, on ne va nulle part : chaque salle est close —
    assert!(look(&s, o, COURT).contains(&at(o, [0, DEPTH + 1, 0])), "on entre dans la tour"); assert!(!look(&s, o, COURT).iter().any(|c| c[1] < o[1]), "rien ne descend");
    // — la dalle de la tour : au hall —
    assert_eq!(step_on(&mut s, o, am::TOWER_PAD), ar[0], "la dalle de la tour mène au hall"); assert!(s.notes.iter().any(|t| t.contains("emporte")));
    let r = look(&s, o, ar[0]); for k in 1..5 { assert!(!r.contains(&at(o, ar[k])), "le hall est clos ({k})"); }
    // — le bleu : la bibliothèque, et retour ; le blanc : la surface, et retour —
    assert_eq!(step_on(&mut s, o, am::HALL_W), ar[1], "le bleu mène à la bibliothèque"); assert_eq!(step_on(&mut s, o, am::LIB_PAD), ar[0], "et en revient");
    assert_eq!(step_on(&mut s, o, am::HALL_S), ar[5], "le blanc mène à la surface"); assert_eq!(step_on(&mut s, o, am::TOWER_PAD), ar[0]);
    // — le rose : le cachot ; les barreaux tombent, deux vagues ; vaincues, on repart —
    assert_eq!(step_on(&mut s, o, am::HALL_E), ar[3], "le rose mène au cachot"); run(&mut s, 0.3); assert!(name(&s, at(o, [am::CELL_PAD[0], 1, am::CELL_PAD[2]])).starts_with("iron_bars"), "les barreaux tombent sur la dalle");
    for wave in [2usize, 2] { run(&mut s, 3.5); let me = s.player.p; let live: Vec<usize> = (0..s.horde.foes.len()).filter(|i| s.horde.foes[*i].dead == 0.0).collect(); assert_eq!(live.len(), wave, "une vague de {wave}");
        for i in live { s.horde.foes[i].hp = 1.0; s.horde.strike(i, 99.0, me, "sword", false); } run(&mut s, 1.0); }
    run(&mut s, 0.5); assert_eq!(name(&s, at(o, [am::CELL_PAD[0], 1, am::CELL_PAD[2]])), "air", "les barreaux se relèvent"); assert_eq!(step_on(&mut s, o, am::CELL_PAD), ar[0]);
    // — le nord : violet, le laboratoire ; le cadran, deux crans : l'or —
    assert_eq!(name(&s, at(o, am::HALL_N)), "crystal_violet"); assert_eq!(step_on(&mut s, o, am::HALL_N), ar[2], "le violet mène au laboratoire");
    for k in 1..=2 { assert!(s.lever_at(at(o, am::DIAL_LEVER))); run(&mut s, 0.2); assert_eq!(dial(&s), k); assert_eq!(name(&s, at(o, am::HALL_N)), am::NORTH_COLORS[k as usize], "la dalle du nord change de couleur"); }
    assert_eq!(step_on(&mut s, o, am::LAB_PAD), ar[0]);
    // — l'or, sans la formule : il vacille, on reste —
    s.notes.clear(); assert_eq!(step_on(&mut s, o, am::HALL_N), [am::HALL_N[0], 1, am::HALL_N[2]], "sans la formule, on reste"); assert!(s.notes.iter().any(|t| t.contains("vacille")));
    // — la formule du grimoire, à la bibliothèque —
    assert_eq!(step_on(&mut s, o, am::HALL_W), ar[1]); assert!(s.lever_at(at(o, am::LECTERN))); run(&mut s, 0.2); assert_eq!(step_on(&mut s, o, am::LIB_PAD), ar[0]);
    assert_eq!(step_on(&mut s, o, am::HALL_N), ar[4], "avec elle, l'or mène au Sanctuaire");
    // — l'Archimage —
    let sp = alw_core::hostile::species(am::BOSS).unwrap(); put(&mut s, o, [0, 1, 23]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("l'Archimage se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "sword", false); run(&mut s, 1.0);
    assert!(s.loose.iter().any(|l| l.item == "orbe_arcanes") || s.inv.iter().flatten().any(|st| st.item == "orbe_arcanes"), "l'Orbe des arcanes");
    let c = at(o, am::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin"), "le trésor de l'Archimage"); s.close_chest();
    assert_eq!(step_on(&mut s, o, am::SANCTUM_PAD), ar[0], "le Sanctuaire ramène au hall");
    // la sauvegarde garde les portails, le cadran, la formule
    let t = s.save_text(); let mut b = session(); b.load_text(&t).unwrap(); assert_eq!(b.mech.portals, s.mech.portals); assert_eq!(b.mech.dials.iter().map(|d| d.state).collect::<Vec<_>>(), s.mech.dials.iter().map(|d| d.state).collect::<Vec<_>>());
}
