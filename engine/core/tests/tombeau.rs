//! Le Tombeau du Pharaon : l'escalier, les trois dalles de la salle hypostyle (deux blocs à pousser, et soi), le Scarabée d'or et
//! le rocher roulant (on y échappe en courant, ou dans une niche ; en marchant, il vous rattrape), le mur fragile qu'il fracasse,
//! le Gardien chacal ; la sauvegarde.
use alw_core::{blocks::*, mecanismes as me, player::*, sanctuaire::reach, session::*, stream::*, tombeau::{self as tb, DEPTH}};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn session() -> Session { let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [2000.5, y as f64 + 2.0, 1985.5]); ready(&mut s); s }
/// Une case du tombeau : `v` compté depuis le sol des salles.
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] - DEPTH + q[1], o[2] + q[2]] }
const COURT: [i32; 3] = [0, DEPTH + 1, -35];
fn world() -> (Session, [i32; 3]) {
    let mut s = session(); s.set_creative(false); s.hour = 12.0; let y = alw_core::gen::FLAT_Y;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [2000, y, 2000]; s.build_tomb(o); let _ = s.sync(); put(&mut s, o, COURT); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-24, -1, -36]), at(o, [33, DEPTH + 6, 1])) }
/// Pousse le bloc qui est en `b` d'une case, depuis la case `from` (où l'on se tient).
fn push(s: &mut Session, o: [i32; 3], from: [i32; 3], b: [i32; 3]) { put(s, o, from); assert!(s.push_at(at(o, b)), "un bloc à pousser en {b:?}"); run(s, 0.05); }
/// On prend l'idole depuis la case devant le socle.
fn take_idol(s: &mut Session, o: [i32; 3]) { put(s, o, [tb::IDOL[0] - 1, 1, tb::LANE]); assert!(s.idol_at(at(o, tb::IDOL)), "l'idole"); }
/// On ouvre la porte de l'est (les deux blocs sur leurs dalles, et soi sur la troisième).
fn open_door(s: &mut Session, o: [i32; 3]) {
    for (k, b) in tb::BLOCKS.iter().enumerate() { let (pl, mut u) = (tb::PLATES[k], b[0]); while u < pl[0] { push(s, o, [u - 1, 1, b[2]], [u, 1, b[2]]); u += 1; }
        let d = (pl[2] - b[2]).signum(); let mut w = b[2]; while w != pl[2] { push(s, o, [pl[0], 1, w - d], [pl[0], 1, w]); w += d; } }
    put(s, o, [tb::PLATES[2][0], 1, tb::PLATES[2][2]]); run(s, 0.3);
}
/// Le joueur avance (sprint ou pas) vers l'ouest pendant `secs`.
fn flee(s: &mut Session, secs: f64, sprint: bool) -> bool { let mut hit = false; s.player.yaw = -std::f64::consts::FRAC_PI_2; for _ in 0..(secs / DT).ceil() as usize { s.vit[1] = 100.0; s.vit[2] = 100.0; s.stamina = 100.0; s.winded = false; let n = s.notes.len(); s.tick(DT, Input { x: 0.0, z: 1.0, sprint, ..Default::default() }, Actions::default()); hit |= s.notes[n..].iter().any(|t| t.contains("écrase")); s.player.yaw = -std::f64::consts::FRAC_PI_2; } hit }

#[test]
fn plans_course_et_empreinte() {
    let p = tb::path(); for q in p.windows(2) { assert_eq!((q[1][0] - q[0][0]).abs() + (q[1][1] - q[0][1]).abs() + (q[1][2] - q[0][2]).abs(), 1, "un pas à la fois"); }
    assert_eq!(*p.last().unwrap(), [tb::WEAK_U + 2, 2, tb::LANE], "il finit contre le mur fragile");
    for u0 in tb::NICHES { for q in p.iter().flat_map(|c| me::boulder_cells(*c)) { assert!(q[2] > tb::LANE - 2, "les niches sont hors de sa course ({u0})"); } }
    for (k, b) in tb::BLOCKS.iter().enumerate() { for q in p.iter().flat_map(|c| me::boulder_cells(*c)) { assert!(q != *b && q != [tb::PLATES[k][0], 1, tb::PLATES[k][2]], "ni les blocs ni leurs dalles"); } }
    // le temps qu'il met jusqu'à la porte : à pied on n'y arrive pas avant lui, en courant si
    let t = me::BOULDER_DELAY + (p.iter().position(|c| c[0] == tb::DOOR_U).unwrap()) as f64 * me::BOULDER_STEP; let run = (tb::IDOL[0] - 1 - tb::DOOR_U) as f64; println!("{t:.2} s ; à pied {:.2} s, en courant {:.2} s", run / 4.5, run / 7.2);
    assert!(run / 4.5 > t && run / 7.2 + 0.6 < t, "on échappe au rocher en courant, pas en marchant");
    let (lo, hi) = tb::footprint(); println!("empreinte {lo:?}..{hi:?}"); assert!(lo[0] >= -38 && lo[1] >= -38 && hi[0] <= 38 && hi[1] <= 38, "le tombeau tient dans la portée des empreintes");
}

#[test]
fn dalles_idole_rocher_mur_et_gardien() {
    let (mut s, o) = world();
    let r = look(&s, o, COURT); assert!(r.contains(&at(o, [0, 1, -15])), "l'escalier mène à la salle hypostyle");
    assert!(!r.contains(&at(o, [15, 1, tb::LANE])), "la porte de l'est est close"); assert!(!r.contains(&at(o, [-15, 1, tb::LANE])), "la chambre funéraire est murée");
    // — les dalles : deux blocs, et soi —
    for (k, b) in tb::BLOCKS.iter().enumerate() { let (pl, mut u) = (tb::PLATES[k], b[0]); while u < pl[0] { push(&mut s, o, [u - 1, 1, b[2]], [u, 1, b[2]]); u += 1; }
        let d = (pl[2] - b[2]).signum(); let mut w = b[2]; while w != pl[2] { push(&mut s, o, [pl[0], 1, w - d], [pl[0], 1, w]); w += d; }
        assert_eq!(name(&s, at(o, [pl[0], 1, pl[2]])), "push_block"); assert_eq!(name(&s, at(o, pl)), "pressure_plate_on", "le bloc {k} pèse sur sa dalle"); }
    assert_eq!(name(&s, at(o, [tb::DOOR_U, 2, tb::LANE])), "red_sandstone", "deux dalles ne suffisent pas");
    put(&mut s, o, [tb::PLATES[2][0], 1, tb::PLATES[2][2]]); run(&mut s, 0.3); assert_eq!(name(&s, at(o, [tb::DOOR_U, 2, tb::LANE])), "air", "les trois : la porte s'abaisse");
    put(&mut s, o, [0, 1, -15]); run(&mut s, 0.3); assert_eq!(name(&s, at(o, [tb::DOOR_U, 2, tb::LANE])), "air", "et reste ouverte");
    assert!(look(&s, o, [0, 1, -15]).contains(&at(o, [tb::IDOL[0] - 1, 1, tb::LANE])), "jusqu'à l'idole");
    // — l'idole : on la prend ; on se jette dans une niche : le rocher passe sans vous toucher —
    take_idol(&mut s, o); assert!(s.inv.iter().flatten().any(|st| st.item == "scarabee_or"), "le Scarabée d'or"); assert_eq!(name(&s, at(o, tb::IDOL)), "air"); assert!(s.notes.iter().any(|t| t.contains("courez")));
    put(&mut s, o, [tb::NICHES[0], 1, tb::LANE - 3]); let mut seen = false; let mut hit = false;
    for _ in 0..(8.0 / DT) as usize { let n = s.notes.len(); s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); hit |= s.notes[n..].iter().any(|t| t.contains("écrase")); seen |= name(&s, at(o, [tb::NICHES[0], 2, tb::LANE])) == "boulder"; if s.mech.boulders[0].state == 3 { break; } }
    assert!(seen, "le rocher passe devant la niche"); assert!(!hit, "sans vous toucher"); assert_eq!(s.mech.boulders[0].state, 3, "il est allé au bout");
    // — le mur fragile s'est effondré ; le rocher, brisé, ne bouche rien —
    for q in tb::weak_wall() { assert_eq!(name(&s, at(o, q)), "air", "le mur fragile"); } assert!(tb::path().iter().flat_map(|c| me::boulder_cells(*c)).all(|q| name(&s, at(o, q)) != "boulder"), "plus de rocher");
    assert!(s.notes.iter().any(|t| t.contains("fracasse"))); for c in tb::COLUMNS { assert_eq!(name(&s, at(o, [c.0, 3, c.1])), "column_shaft", "les colonnes tiennent"); }
    assert!(look(&s, o, [0, 1, -15]).contains(&at(o, [-14, 1, -12])), "la chambre funéraire est ouverte");
    // — le Gardien chacal —
    let sp = alw_core::hostile::species(tb::BOSS).unwrap(); s.horde.foes.clear(); put(&mut s, o, [-14, 1, -12]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("le Gardien chacal se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    s.horde.foes[i].hp = 1.0; let me_p = s.player.p; s.horde.strike(i, 99.0, me_p, "sword", false); run(&mut s, 1.0);
    assert!(s.loose.iter().any(|l| l.item == "masque_funeraire") || s.inv.iter().flatten().any(|st| st.item == "masque_funeraire"), "le Masque funéraire");
    let c = at(o, tb::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin"), "le trésor du Pharaon"); s.close_chest();
    // la sauvegarde garde le rocher et la porte
    let t = s.save_text(); let mut b = session(); b.load_text(&t).unwrap(); assert_eq!(b.mech.boulders, s.mech.boulders.iter().map(|x| me::Boulder { t: b.mech.boulders[0].t, ..x.clone() }).collect::<Vec<_>>());
}

#[test]
fn en_courant_on_echappe_en_marchant_il_vous_rattrape() {
    for (sprint, caught) in [(false, true), (true, false)] {
        let (mut s, o) = world(); open_door(&mut s, o); take_idol(&mut s, o); let hit = flee(&mut s, if sprint { 2.6 } else { 4.5 }, sprint);
        // une fois dans la salle hypostyle, on s'écarte de l'allée
        if sprint && !hit { let p = s.player.p; println!("sorti du couloir en u = {:.1}", p[0] - o[0] as f64); assert!(p[0] < (o[0] + tb::DOOR_U) as f64, "dans la salle hypostyle"); s.player.p[2] = p[2] - 4.0; }
        let mut late = false; for _ in 0..(5.0 / DT) as usize { let n = s.notes.len(); s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); late |= s.notes[n..].iter().any(|t| t.contains("écrase")); if s.mech.boulders[0].state == 3 { break; } }
        println!("{} : {:?}", if sprint { "en courant" } else { "en marchant" }, s.player.p); assert_eq!(hit || late, caught, "sprint {sprint}");
    }
}
