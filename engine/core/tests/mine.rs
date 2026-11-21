//! La Mine des nains : l'escalier de la cour à la gare ; le wagonnet et ses deux aiguillages (une voie vers l'îlot, une vers le
//! butoir, une seule par le pont) ; les marteaux-pilons du couloir et leurs runes ; la porte runique ; le Golem de minerai.
//! On joue tout avec le solveur de chemins, et en vrai : le wagonnet emporte le joueur.
use alw_core::{blocks::*, mine::{self as mi, DEPTH}, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn session() -> Session { let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [2000.5, y as f64 + 2.0, 1985.5]); ready(&mut s); s }
/// Une case de la mine : `v` compté depuis le sol de la gare.
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] - DEPTH + q[1], o[2] + q[2]] }
const COURT: [i32; 3] = [0, DEPTH + 1, -35];
fn world() -> (Session, [i32; 3]) {
    let mut s = session(); s.set_creative(false); s.hour = 12.0; let y = alw_core::gen::FLAT_Y;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [2000, y, 2000]; s.build_dwarf_mine(o); let _ = s.sync(); put(&mut s, o, COURT); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-20, mi::CHASM.2 - 1, -40]), at(o, [20, DEPTH + 10, 36])) }
fn cell(s: &Session) -> [i32; 3] { let p = s.player.p; [p[0].floor() as i32, (p[1] + 0.05).floor() as i32, p[2].floor() as i32] }
fn dial(s: &Session, end: &str) -> usize { s.mech.dials.iter().position(|d| d.name.ends_with(end)).unwrap_or_else(|| panic!("le rouage {end}")) }
fn on(s: &Session, end: &str) -> bool { let w = s.mech.wires.iter().find(|w| w.name.ends_with(end)).unwrap(); w.on }
/// On monte dans le wagonnet (où qu'il soit), on y reste jusqu'à ce qu'il s'arrête au bout.
fn ride(s: &mut Session, o: [i32; 3]) -> [i32; 3] {
    let c = &s.mech.carts[0]; let here = c.routes[c.route][c.pos]; s.player.p = [here[0] as f64 + 0.5, here[1] as f64 + 1.0, here[2] as f64 + 0.5]; s.player.v = [0.0; 3];
    let mut moved = false; for _ in 0..1200 { run(s, 0.05); let c = &s.mech.carts[0]; moved |= c.state == 1 || c.state == 3; if moved && (c.state == 2 || (c.state == 0 && c.t < 0.0)) { break; } } assert!(moved, "le wagonnet part");
    let c = cell(s); [c[0] - o[0], c[1] - (o[1] - DEPTH) - 1, c[2] - o[2]]
}

#[test]
fn plans_voies_runes_et_empreinte() {
    let r = mi::routes(); assert_eq!(r.len(), 4);
    for v in &r { assert_eq!(v[0], mi::STATION, "chaque voie part du quai"); for p in v.windows(2) { assert_eq!((p[1][0] - p[0][0]).abs() + (p[1][2] - p[0][2]).abs(), 1, "un pas à la fois"); } }
    assert_eq!(*r[0].last().unwrap(), [-10, 1, 0], "sans aiguillage : vers l'îlot"); assert_eq!(*r[1].last().unwrap(), [16, 1, mi::JUNCTION_W], "l'aiguillage du quai seul : le butoir");
    assert_eq!(r[2], r[0], "celui de l'îlot seul : l'îlot encore"); assert_eq!(*r[3].last().unwrap(), [10, 1, mi::FAR.0 + 1], "les deux : par le pont, de l'autre côté");
    let many: BTreeSet<[u8; 3]> = (0..60).map(|k| mi::runes([k * 977, 60, k * 613 - 4000])).collect(); assert!(many.len() >= 12, "les runes changent d'une mine à l'autre");
    for k in 0..200 { let o = [k * 331, 60, k * 97 - 9000]; let (g, w) = (mi::runes(o), mi::wheels_start(o)); assert!((0..3).all(|i| g[i] != w[i]), "aucune roue n'est déjà sur sa rune"); }
    for r in 0..4 { assert!(mi::glyph(r, (0, 3), 0).len() >= 4, "une rune se lit"); for c in mi::glyph(r, (0, 3), 0) { assert!(mi::glyph(r, (0, 3), 0).contains(&[-c[0], c[1], c[2]]), "symétrique : on ne la lit jamais à l'envers"); } }
    let (lo, hi) = mi::footprint(); println!("empreinte {lo:?}..{hi:?}");
    assert!(lo[0] >= -38 && lo[1] >= -38 && hi[0] <= 38 && hi[1] <= 38, "la mine tient dans la portée des empreintes ({lo:?}..{hi:?})");
}

#[test]
fn escalier_wagonnet_marteaux_porte_et_golem() {
    let (mut s, o) = world(); let rn = mi::runes(o); println!("runes {rn:?}, roues {:?}", mi::wheels_start(o));
    // — de la cour à la gare —
    let r = look(&s, o, COURT); assert!(r.contains(&at(o, [8, 1, -18])), "l'escalier mène à la gare"); assert!(r.contains(&at(o, [1, 1, -14])), "jusqu'au quai");
    assert!(!r.contains(&at(o, [-10, 1, -1])), "pas d'îlot à pied"); assert!(!r.contains(&at(o, [10, 1, 7])), "ni d'autre rive"); assert!(!r.contains(&at(o, [0, 1, 12])), "ni de couloir");
    assert!(look(&s, o, [0, mi::CHASM.2 + 1, 0]).contains(&at(o, [1, 1, -14])), "l'échelle en remonte");
    assert_eq!(name(&s, at(o, mi::STATION)), "minecart", "le wagonnet à quai");
    // — sans aiguillage, il mène à l'îlot —
    let end = ride(&mut s, o); assert_eq!(end, [-10, 1, 0], "le wagonnet vous emporte jusqu'à l'îlot"); assert_eq!(name(&s, at(o, [-10, 1, 0])), "minecart"); assert!(name(&s, at(o, mi::STATION)).starts_with("rail"), "le quai est vide");
    assert!(s.notes.iter().any(|t| t.contains("s'ébranle")) && s.notes.iter().any(|t| t.contains("butoir")));
    assert!(!look(&s, o, [-11, 1, 0]).contains(&at(o, [1, 1, -14])), "de l'îlot, pas de retour à pied");
    // sur l'îlot : le second aiguillage ; on remonte, il revient à quai
    put(&mut s, o, [-11, 1, 0]); assert!(s.lever_at(at(o, mi::SWITCH_B))); run(&mut s, 0.1); assert!(on(&s, "aiguillageB"));
    let back = ride(&mut s, o); assert_eq!(back, mi::STATION, "il ramène à quai"); assert_eq!(s.mech.carts[0].state, 0);
    // il attend : le temps d'abaisser le levier du quai sans descendre
    assert!(s.lever_at(at(o, mi::SWITCH_A))); run(&mut s, 0.1); assert!(on(&s, "aiguillageA")); assert_eq!(s.mech.carts[0].state, 0, "pas encore reparti");
    let end = ride(&mut s, o); assert_eq!(end, [10, 1, mi::FAR.0 + 1], "les deux aiguillages : par le pont, de l'autre côté du gouffre");
    // l'autre voie : le butoir (on revient, on relève l'aiguillage de l'îlot... qui est de l'autre côté : on refait le chemin en test)
    let i = dial(&s, "marteau0"); let mut seen = BTreeSet::new(); for _ in 0..40 { run(&mut s, 0.05); seen.insert(s.mech.dials[i].state); } assert_eq!(seen.len(), 3, "le marteau monte et descend");
    for k in 0..3 { let d = dial(&s, &format!("marteau{k}")); s.mech.dials[d].auto = 0.0; s.dial_set(d, 0, false); }
    let far = [10, 1, mi::FAR.0 + 1]; let r = look(&s, o, [9, 1, 7]); assert!(r.contains(&at(o, [0, 1, mi::CORRIDOR.0])), "de l'autre rive, le couloir des marteaux");
    assert!(r.contains(&at(o, [0, 1, mi::ANTE.0 + 1])), "les marteaux levés, on passe dessous"); assert!(!r.contains(&at(o, [0, 1, 28])), "la porte runique est fermée");
    // — les marteaux-pilons — ils tombent seuls, en cadence, et écrasent qui passe dessous
    s.dial_set(i, 2, false); assert!(!look(&s, o, [0, 1, 10]).contains(&at(o, [0, 1, 14])), "un marteau baissé barre le couloir"); for (c, b) in mi::hammer(mi::HAMMERS[0], 2) { assert_eq!(name(&s, at(o, c)), b); }
    s.dial_set(i, 0, false); put(&mut s, o, [0, 1, mi::HAMMERS[0]]); s.player.v = [0.0, 0.0, 2.0]; s.vit = [100.0; 3]; let hp = s.vit[0]; s.dial_set(i, 2, false);
    assert!(s.vit[0] < hp, "le marteau écrase"); assert!(s.player.v[2] < -3.0, "et rejette en arrière ({:?})", s.player.v); assert!(s.notes.iter().any(|t| t.contains("marteau-pilon")));
    s.dial_set(i, 0, false); run(&mut s, 0.5);
    // les runes du mur
    for (k, w) in mi::HAMMERS.iter().enumerate() { for c in mi::glyph(rn[k], (0, 3), 0) { assert_eq!(name(&s, at(o, [-2, c[1], w - 2 + c[0]])), "crystal_violet", "la rune {k}"); } }
    // — la porte runique — chaque roue tourne d'un cran par levier ; les trois sur les runes du couloir : les chaînes tombent
    let door = [0, 2, mi::DOOR_W]; assert_eq!(name(&s, at(o, door)), "great_chain");
    put(&mut s, o, [0, 1, mi::ANTE.0]); for k in 0..3 { let d = dial(&s, &format!("roue{k}")); let n = (rn[k] as i32 - s.mech.dials[d].state as i32).rem_euclid(4);
        for _ in 0..n { assert!(s.lever_at(at(o, mi::WHEEL_LEVERS[k]))); run(&mut s, 0.15); } assert_eq!(s.mech.dials[d].state, rn[k]);
        for c in mi::glyph(rn[k], mi::WHEELS[k], mi::DOOR_W - 1) { assert_eq!(name(&s, at(o, c)), "crystal_blue", "la roue {k} montre sa rune"); }
        if k < 2 { assert_eq!(name(&s, at(o, door)), "great_chain", "pas encore"); } }
    run(&mut s, 0.2); assert_eq!(name(&s, at(o, door)), "air", "les chaînes tombent"); assert!(look(&s, o, [0, 1, mi::ANTE.0]).contains(&at(o, [0, 1, 28])), "on entre dans la salle du trône");
    // — le Golem de minerai —
    let sp = alw_core::hostile::species(mi::BOSS).unwrap(); s.horde.foes.clear(); put(&mut s, o, [0, 1, 28]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("le Golem se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    assert_eq!(name(&s, at(o, door)), "great_chain", "les chaînes retombent derrière vous");
    s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "hammer", false); run(&mut s, 1.0); assert_eq!(name(&s, at(o, door)), "air", "vaincu : la porte se rouvre");
    assert!(s.loose.iter().any(|l| l.item == "pierre_coeur") || s.inv.iter().flatten().any(|st| st.item == "pierre_coeur"), "la Pierre-Cœur");
    let c = at(o, mi::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin"), "le trésor des rois nains"); s.close_chest();
    // — le butoir : on revient au quai, relève l'aiguillage de l'îlot (en test, directement) : la voie de l'est s'arrête net
    let _ = far; put(&mut s, o, [10, 1, mi::FAR.0 + 1]); let back = ride(&mut s, o); assert_eq!(back, mi::STATION);
    assert!(s.lever_at(at(o, mi::SWITCH_B))); run(&mut s, 0.1); assert!(!on(&s, "aiguillageB") && on(&s, "aiguillageA"));
    let end = ride(&mut s, o); assert_eq!(end, [16, 1, mi::JUNCTION_W], "l'aiguillage du quai seul : contre le butoir"); assert!(!look(&s, o, [15, 1, mi::JUNCTION_W]).contains(&at(o, [10, 1, 7])), "et pas de l'autre côté");
    // la sauvegarde garde le wagonnet et les rouages
    let t = s.save_text(); let mut b = session(); b.load_text(&t).unwrap();
    let st = |s: &Session| (s.mech.carts.iter().map(|c| (c.name.clone(), c.pos, c.route, c.routes.len(), c.switches.clone())).collect::<Vec<_>>(), s.mech.dials.iter().map(|d| (d.name.clone(), d.state, d.goal)).collect::<Vec<_>>()); assert_eq!(st(&b), st(&s));
}
