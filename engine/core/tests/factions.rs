//! Les factions et la réputation : chaque peuple (Royaume, Oasis, Ports, Clans du Nord) garde la mémoire de ce qu'on fait pour
//! ses villages ; la position auprès d'un village mêle sa réputation propre et celle de son peuple, et change les prix, l'accueil,
//! les titres et les présents.
use alw_core::factions::{Faction, Tier, FRIEND, HERO};
use alw_core::hostile::bestiary;
use alw_core::{gen::WorldGen, player::Input, session::*, stream::Preset};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, n: usize) { for _ in 0..n { s.tick(0.05, Input::default(), Actions::default()); } }

fn at_village() -> (Session, alw_core::village::Village) {
    let v = WorldGen::new(SEED).villages_near(0.0, 0.0, 2000.0).into_iter().next().unwrap();
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [v.c[0] as f64 + 0.5, v.c[1] as f64 + 1.0, v.c[2] as f64 + 3.5]); ready(&mut s); s.third = false; s.hour = 12.0; idle(&mut s, 40);
    s.wild.all.clear(); s.horde.foes.clear(); assert!(s.town.here.iter().any(|x| x.id == v.id), "au village"); (s, v)
}

#[test]
fn degres_et_prix() {
    assert_eq!(Tier::of(-30), Tier::Hostile); assert_eq!(Tier::of(-5), Tier::Mefiant); assert_eq!(Tier::of(0), Tier::Neutre); assert_eq!(Tier::of(20), Tier::Apprecie); assert_eq!(Tier::of(50), Tier::Ami); assert_eq!(Tier::of(90), Tier::Heros);
    assert_eq!(Tier::Mefiant.price(4), 6, "méfiant : moitié plus"); assert_eq!(Tier::Neutre.price(4), 4); assert_eq!(Tier::Ami.price(4), 3, "ami : un quart de moins"); assert_eq!(Tier::Heros.price(4), 2, "héros : moitié moins"); assert_eq!(Tier::Ami.price(1), 1, "jamais rien pour rien");
    assert_eq!(Faction::of_kind("plaine"), Faction::Royaume); assert_eq!(Faction::of_kind("desert"), Faction::Oasis); assert_eq!(Faction::of_kind("port"), Faction::Ports); assert_eq!(Faction::of_kind("montagne"), Faction::Clans); assert_eq!(Faction::of_kind("taiga"), Faction::Clans);
}

#[test]
fn la_reputation_d_un_peuple() {
    let (mut s, v) = at_village(); s.set_creative(false); let f = Faction::of_kind(v.kind);
    println!("{} ({}) : peuple {}", v.name, v.kind, f.name()); assert_eq!(s.standing(v.id), 0);
    let base: Vec<u32> = s.offers(&v).iter().map(|o| o.give.1).collect();
    // Ce qu'on fait pour le peuple compte pour chacun de ses villages : la moitié de sa réputation s'ajoute à celle du village.
    s.sway(f, 10.0); assert_eq!(s.standing(v.id), 5);
    // L'ami du peuple : un titre, un présent.
    let coins = s.count(alw_core::economie::COIN); s.sway(f, FRIEND); assert!(s.count(alw_core::economie::COIN) >= coins + 20, "le présent"); assert!(s.notes.iter().any(|n| n.contains(f.title(false))), "le titre : {:?}", s.notes);
    let line = s.faction_lines().into_iter().find(|l| l.0 == f).unwrap(); assert_eq!(line.2, Tier::Ami); assert_eq!(line.3, Some(f.title(false)));
    // Le héros : ses villages sont tous indiqués ; et au village, tout coûte moitié moins (ou un quart, en ami).
    let known = s.town.known.len(); s.sway(f, HERO); assert!(s.town.known.len() > known, "des villages du peuple indiqués : {} → {}", known, s.town.known.len());
    let cheap: Vec<u32> = s.offers(&v).iter().map(|o| o.give.1).collect(); println!("prix : neutre {base:?} → {:?} ({}) {cheap:?}", s.tier(v.id), s.standing(v.id));
    assert!(s.tier(v.id) >= Tier::Ami); assert!(cheap.iter().zip(&base).all(|(a, b)| a <= b) && cheap.iter().zip(&base).any(|(a, b)| a < b), "les prix baissent");
    // Les autres peuples n'en savent rien (sauf ses alliés et ses ennemis : voir tests/politique.rs).
    for g in alw_core::factions::ALL { if g != f && matches!(s.politics.bond(f, g), alw_core::politique::Bond::Paix | alw_core::politique::Bond::Froid) { assert_eq!(s.faction_rep(g), 0); } }
    // Tout se garde avec la partie.
    let t = s.save_text(); let mut l = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, [-38.5, 51.4, 0.5]); l.load_text(&t).unwrap();
    assert_eq!(l.faction_rep(f), s.faction_rep(f)); assert_eq!(l.faction_lines(), s.faction_lines()); let n = l.notes.len(); l.sway(f, 1.0); assert_eq!(l.notes.len(), n, "pas deux fois le même présent");
    // Ennemi : méfiants d'abord (plus cher), puis on vous chasse.
    s.sway(f, -100.0 - 100.0); assert!(s.tier(v.id) <= Tier::Mefiant); let dear: Vec<u32> = s.offers(&v).iter().map(|o| o.give.1).collect(); assert!(dear.iter().zip(&base).any(|(a, b)| a > b), "plus cher : {dear:?}");
    s.town.rep.insert(v.id, -20); assert_eq!(s.tier(v.id), Tier::Hostile);
    let k = s.town.folk.iter().position(|x| x.v == v.id).expect("un villageois"); s.talk_to(k); let d = s.dialog().expect("il répond"); println!("{} : « {} » {:?}", d.title, d.said, d.options);
    assert!(d.said.starts_with("Passez votre chemin") && d.options.len() == 1, "on vous chasse");
}

#[test]
fn abattre_des_brigands_pres_d_un_village() {
    let (mut s, v) = at_village(); let f = Faction::of_kind(v.kind); let sp = bestiary().species.iter().position(|x| x.id == "brigand").unwrap();
    let p = [v.c[0] as f64 + 4.5, v.c[1] as f64 + 1.0, v.c[2] as f64 + 4.5]; s.player.p = [p[0] + 2.0, p[1], p[2]];
    let i = s.horde.put(sp, p, 0.0, false); s.horde.strike(i, 999.0, s.player.p, "sword", true); idle(&mut s, 3);
    println!("{} : réputation {} ; {} {}", v.name, s.rep(v.id), f.name(), s.faction_rep(f)); assert!(s.rep(v.id) >= 1, "le village en sait gré"); assert!(s.factions.rep[f as usize] > 0.0, "et son peuple");
}

#[test]
fn disperser_un_camp_rejouit_les_villages_alentour() {
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.hour = 12.0;
    let msg = s.command("/lieu brigands"); ready(&mut s); println!("{msg}"); for _ in 0..3 { s.tick(0.05, Input::default(), Actions::default()); }
    let men: Vec<usize> = (0..s.horde.foes.len()).filter(|&i| s.horde.foes[i].camp.is_some()).collect(); assert!(!men.is_empty());
    let near: Vec<_> = WorldGen::new(SEED).villages_near(s.player.p[0], s.player.p[2], 1000.0); assert!(!near.is_empty(), "des villages à mille pas");
    for _ in 0..30 { for &i in &men { if s.horde.foes.get(i).is_some_and(|f| f.dead == 0.0) { let p = s.horde.foes[i].p; s.horde.strike(i, 999.0, [p[0] + 1.0, p[1], p[2]], "sword", true); } } s.tick(0.05, Input::default(), Actions::default()); }
    let fs: Vec<Faction> = near.iter().map(|v| Faction::of_kind(v.kind)).collect(); println!("villages alentour : {:?} ; peuples {:?}", near.iter().map(|v| (&v.name, s.rep(v.id))).collect::<Vec<_>>(), alw_core::factions::ALL.map(|g| s.faction_rep(g)));
    assert!(near.iter().all(|v| s.rep(v.id) >= 3), "chaque village alentour en sait gré"); for f in fs { assert!(s.faction_rep(f) >= 6, "{} aussi", f.name()); }
}
