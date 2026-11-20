//! L'Observatoire céleste : les neuf étoiles (chaque levier bascule son étoile et ses voisines) à accorder au plafond ; la planète
//! qui emporte le joueur d'un palier à l'autre au-dessus de la fosse ; l'Astronome dans la coupole ; la grande lunette à tourner
//! vers l'étoile d'or. On joue tout avec le solveur de chemins.
use alw_core::{blocks::*, observatoire::{self as ob}, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn session() -> Session { let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [2000.5, y as f64 + 2.0, 1985.5]); ready(&mut s); s }
fn world() -> (Session, [i32; 3]) {
    let mut s = session(); s.set_creative(false); s.hour = 12.0; let y = alw_core::gen::FLAT_Y;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [2000, y, 2000]; s.build_observatory(o); let _ = s.sync(); put(&mut s, o, [0, 0, -19]); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] + 1 + q[1], o[2] + q[2]] }
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-14, ob::PIT - 1, -22]), at(o, [14, 20, 17])) }
fn lever(s: &mut Session, o: [i32; 3], q: [i32; 3]) { assert!(s.lever_at(at(o, q)), "un levier en {q:?}"); run(s, 0.1); assert!(!name(s, at(o, q)).starts_with("lever_on"), "le levier se relève"); }
fn dial(s: &Session, end: &str) -> usize { s.mech.dials.iter().position(|d| d.name.ends_with(end)).unwrap_or_else(|| panic!("le rouage {end}")) }
fn state(s: &Session, end: &str) -> u8 { s.mech.dials[dial(s, end)].state }
fn star_on(s: &Session, o: [i32; 3], c: usize, r: usize) -> bool { name(s, at(o, ob::star_cell(c, r))) == "star_lit" }
/// Les leviers à abaisser (une fois chacun) pour passer de `now` à la constellation : on essaie les 512 façons.
fn solve(now: [[bool; 3]; 3], sky: usize) -> Vec<(usize, usize)> {
    (0..512u32).map(|m| (0..9).filter(|k| m >> k & 1 == 1).map(|k| (k % 3, k / 3)).collect::<Vec<_>>()).filter(|ps| { let mut g = now; for (c, r) in ps { for (cc, rr) in ob::flips(*c, *r) { g[cc][rr] = !g[cc][rr]; } } (0..3).all(|c| (0..3).all(|r| g[c][r] == ob::lit(sky, c, r))) }).min_by_key(|ps| ps.len()).expect("toujours une solution")
}

#[test]
fn chaque_ciel_se_resout_et_la_planete_reste_dans_la_fosse() {
    for k in 0..ob::SKIES.len() { let p = solve([[false; 3]; 3], k); assert!(p.len() >= 3, "{} : {} leviers", ob::SKIES[k].0, p.len()); }
    let skies: BTreeSet<usize> = (0..60).map(|k| ob::sky([k * 977, 60, k * 613 - 4000])).collect(); assert!(skies.len() >= 5, "la constellation change d'un observatoire à l'autre");
    for k in 0..200 { let o = [k * 331, 60, k * 97 - 9000]; assert_ne!(ob::star(o), ob::scope_start(o), "la lunette ne vise pas déjà l'étoile"); }
    for k in 0..ob::ORBIT_N { let p = ob::planet(k); assert!(p[..9].iter().all(|(c, _)| c[1] == 0), "d'abord le dessus");
        assert!(p.iter().all(|(c, _)| c[2] > ob::LEDGE_N[1] && c[2] < ob::LEDGE_S[0] && c[0].abs() < ob::HALF - 1), "la planète ne heurte ni les paliers ni l'escalier ({k})");
        let n = ob::planet((k + 1) % ob::ORBIT_N); let d = (n[4].0[0] - p[4].0[0]).abs() + (n[4].0[2] - p[4].0[2]).abs(); assert!(d <= 3, "un pas à la fois"); }
    for k in 0..8 { assert!(ob::scope(k).iter().all(|(c, _)| { let d = ((c[0] * c[0] + (c[2] - ob::DOME_W).pow(2)) as f64).sqrt(); d <= ob::dome_r(c[1]) }), "la lunette tient sous la coupole ({k})"); }
}

#[test]
fn etoiles_planetaire_astronome_et_lunette() {
    let (mut s, o) = world(); let sk = ob::sky(o); println!("{}", ob::SKIES[sk].0);
    let r = look(&s, o, [0, 0, -19]); assert!(r.contains(&at(o, [0, 1, -13])), "on entre dans la rotonde"); assert!(!r.contains(&at(o, [0, 1, -1])), "la grille du planétaire est fermée");
    for c in 0..3 { for rr in 0..3 { let q = ob::star_cell(c, rr); assert!(!star_on(&s, o, c, rr)); assert_eq!(name(&s, at(o, [q[0], ob::CEILING, q[2]])) == "star_lit", ob::lit(sk, c, rr), "le plafond"); } }
    // un levier bascule son étoile et ses voisines
    lever(&mut s, o, [ob::STAR_U[1], 2, ob::STAR_W[0]]); for c in 0..3 { for rr in 0..3 { assert_eq!(star_on(&s, o, c, rr), ob::flips(1, 0).contains(&(c, rr)), "après le levier du nord ({c}, {rr})"); } }
    let now = [[0, 1, 2].map(|rr| star_on(&s, o, 0, rr)), [0, 1, 2].map(|rr| star_on(&s, o, 1, rr)), [0, 1, 2].map(|rr| star_on(&s, o, 2, rr))]; let p = solve(now, sk); println!("{} leviers", p.len());
    for (k, (c, rr)) in p.iter().enumerate() { let q = ob::star_cell(*c, *rr); lever(&mut s, o, [q[0], 2, q[2]]); if k + 1 < p.len() { assert_eq!(name(&s, at(o, [0, 1, ob::MID])), "iron_bars"); } }
    run(&mut s, 0.2); assert_eq!(name(&s, at(o, [0, 1, ob::MID])), "air", "le ciel accordé : la grille se lève");
    let north = [0, 1, -1]; assert!(look(&s, o, [0, 1, -13]).contains(&at(o, north)), "au palier du planétaire");
    // la planète : jamais de passage direct ; on monte dessus quand elle passe au bord
    let i = dial(&s, "planete"); s.mech.dials[i].auto = 0.0; let south = [0, 1, 12];
    for k in 0..ob::ORBIT_N { s.dial_set(i, k, false); assert!(!look(&s, o, north).contains(&at(o, south)), "pas de passage au cran {k}"); }
    s.dial_set(i, 0, false); assert!(look(&s, o, north).contains(&at(o, [0, 1, 1])), "on monte sur la planète");
    put(&mut s, o, [0, 1, 1]); for k in 1..=ob::ORBIT_N / 2 { s.dial_set(i, k, false); } let c = ob::planet(ob::ORBIT_N / 2)[4].0; let p = s.player.p;
    assert_eq!([p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32], at(o, [c[0], 1, c[2]]), "elle vous emporte jusqu'au sud"); assert!(look(&s, o, [c[0], 1, c[2]]).contains(&at(o, south)), "on descend au palier du sud");
    // en vrai, elle tourne seule et emporte qui s'y tient
    s.dial_set(i, 0, false); s.mech.dials[i].auto = 0.45; put(&mut s, o, [0, 1, 1]); let p0 = s.player.p; run(&mut s, 1.0); let p = s.player.p; let feet = [p[0].floor() as i32, (p[1] + 0.05).floor() as i32 - 1, p[2].floor() as i32];
    assert!((p[0] - p0[0]).abs() + (p[2] - p0[2]).abs() > 1.5, "emporté ({p0:?} -> {p:?})"); assert!(name(&s, feet) == "copper_roof" || name(&s, feet) == "brass", "toujours debout sur la planète ({})", name(&s, feet));
    assert!(look(&s, o, [0, 1, 12]).contains(&at(o, [0, ob::GALLERY + 1, -2])), "du palier du sud, l'escalier et la galerie mènent à la coupole"); assert!(!look(&s, o, north).contains(&at(o, [6, ob::GALLERY + 1, 0])), "pas depuis le palier du nord");
    // — l'Astronome —
    let sp = alw_core::hostile::species(ob::BOSS).unwrap(); s.horde.foes.clear(); put(&mut s, o, [-3, 9, -9]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("l'Astronome se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    assert!(name(&s, at(o, ob::DOME_DOOR[0])).starts_with("iron_bars"), "la porte de la coupole retombe");
    s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "sword", false); run(&mut s, 1.0); assert_eq!(name(&s, at(o, ob::DOME_DOOR[0])), "air");
    assert!(s.loose.iter().any(|l| l.item == "astrolabe") || s.inv.iter().flatten().any(|st| st.item == "astrolabe"), "l'Astrolabe");
    // — la lunette — vers l'étoile d'or de la rose des vents
    let (gold, k0) = (ob::star(o), ob::scope_start(o)); assert_eq!(state(&s, "lunette"), k0); assert!(name(&s, at(o, ob::VAULT_BARS[0])).starts_with("iron_bars"));
    let n = (gold as i32 - k0 as i32).rem_euclid(8) as u32; for _ in 0..n - 1 { lever(&mut s, o, ob::SCOPE_LEVER); } assert!(name(&s, at(o, ob::VAULT_BARS[0])).starts_with("iron_bars"), "pas encore");
    lever(&mut s, o, ob::SCOPE_LEVER); run(&mut s, 0.2); for (c, b) in ob::scope(gold) { assert_eq!(name(&s, at(o, c)), b, "la lunette dessinée vers l'étoile"); }
    assert!(ob::VAULT_BARS.iter().all(|c| name(&s, at(o, *c)) == "air"), "la niche s'ouvre"); let c = at(o, ob::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin")); s.close_chest();
    // la sauvegarde garde les rouages et la planète qui emporte
    let t = s.save_text(); let mut b = session(); b.load_text(&t).unwrap();
    let st = |s: &Session| s.mech.dials.iter().map(|d| (d.name.clone(), d.state, d.goal, d.carry, d.levers.len(), d.draw.len())).collect::<Vec<_>>(); assert_eq!(st(&b), st(&s));
}
