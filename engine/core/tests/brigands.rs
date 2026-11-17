//! Les brigands : une bande marche de nuit sur le village pilote. Sans muraille, elle atteint l'entrepôt et emporte une part
//! des vivres et de la caisse ; derrière la muraille et la herse baissée, elle reste dehors, les gardes lui tirent dessus du
//! chemin de ronde, et la réserve est intacte.
use alw_core::{economie::*, gen::WorldGen, patrons::Kind, player::Input, raids, remparts, session::*, stream::Preset};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, n: usize) { for _ in 0..n { s.tick(0.05, Input::default(), Actions::default()); } }
fn start() -> (Session, alw_core::village::Village) {
    let mut s = Session::new(SEED, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.third = false; idle(&mut s, 5);
    let id = s.econ.as_ref().unwrap().id; let v = WorldGen::new(SEED).site(id.0, id.1).unwrap().v.clone();
    // en créatif, les brigands ne voient pas le joueur : l'affaire se joue entre eux et le village
    s.set_creative(true); s.player.p = [v.c[0] as f64 + 0.5, v.c[1] as f64 + 30.0, v.c[2] as f64 + 0.5]; s.player.fly = true; ready(&mut s); idle(&mut s, 40); (s, v)
}
fn band(s: &Session) -> usize { s.horde.foes.iter().filter(|f| f.dead == 0.0 && !f.raid.is_empty()).count() }

#[test]
fn sans_muraille_la_bande_pille_la_reserve() {
    let (mut s, _v) = start(); s.hour = 10.0; idle(&mut s, 20);
    { let e = s.econ.as_mut().unwrap(); e.stock.insert(COIN, 60.0); e.stock.insert("bread", 30.0); }
    let (coin0, bread0) = { let e = s.econ.as_ref().unwrap(); (e.have(COIN), e.have("bread")) };
    s.hour = 23.0; assert!(s.raid_now(), "une bande arrive"); println!("bande : {} brigands", band(&s));
    let mut looted = false; for i in 0..240 { idle(&mut s, 20); s.hour = 23.0; let e = s.econ.as_ref().unwrap();
        if e.log.iter().any(|l| l.1.contains("pillé")) { looted = true; println!("pillage au bout de {} s : {:?}", i, e.log.last()); break; } }
    let e = s.econ.as_ref().unwrap(); println!("caisse {coin0} → {}, pain {bread0} → {}", e.have(COIN), e.have("bread"));
    assert!(looted && e.have(COIN) < coin0 && e.have("bread") < bread0, "la réserve est pillée");
    assert!(e.raid.is_none(), "la bande s'enfuit");
}

#[test]
fn la_muraille_et_les_gardes_tiennent_la_bande_dehors() {
    let (mut s, v) = start(); s.hour = 10.0;
    { let e = s.econ.as_mut().unwrap(); e.project = Some((Kind::Muraille, true)); for (k, n) in [("cobblestone", 600.0), ("planks", 200.0), ("oak_log", 100.0), ("iron_ingot", 10.0), (COIN, 120.0)] { e.stock.insert(k, n); } }
    idle(&mut s, 10); { let e = s.econ.as_mut().unwrap(); e.project = Some((Kind::TourDefense, true)); } idle(&mut s, 60); idle(&mut s, 120);
    let e = s.econ.clone().unwrap(); assert!(remparts::ring(&e).is_some(), "la muraille"); println!("{}", s.village_report());
    // la nuit tombe : la herse se baisse, les gardes montent ; une bande arrive par la porte
    s.hour = 22.5; idle(&mut s, 40); s.hour = 22.5;
    let c = e.census(&v); println!("portes {:?} ronde {}", e.watch.gates, e.watch.patrol.len()); println!("chance d'une bande cette nuit : {:.2}", raids::raid_chance(&e, true, c.towers, 1));
    let coin0 = s.econ.as_ref().unwrap().have(COIN); assert!(s.raid_now(), "une bande arrive"); let n0 = band(&s);
    let mut hurt = 0.0; let mut shots = 0;
    for _ in 0..400 {
        idle(&mut s, 20); s.hour = 23.0; shots += s.arrows.iter().filter(|a| a.npc && !a.stuck).count();
        hurt = s.horde.foes.iter().filter(|f| f.max > 0.0).map(|f| f.max - f.hp.max(0.0)).sum::<f64>().max(hurt); if s.econ.as_ref().unwrap().raid.is_none() { break; } }
    let e = s.econ.as_ref().unwrap(); println!("bande de {n0} : {:.0} pv perdus sous les flèches, {} vols de flèche vus ; journal {:?}", hurt, shots, e.log.iter().rev().take(2).collect::<Vec<_>>());
    assert!(!e.log.iter().any(|l| l.1.contains("pillé")), "rien de pillé"); assert_eq!(e.have(COIN), coin0, "la caisse intacte");
    assert!(hurt > 0.0 && shots > 0, "les gardes ont tiré sur la bande");
    assert!(e.raid.is_none(), "l'attaque est finie : repoussée ou abandonnée");
}
