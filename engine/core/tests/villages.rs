//! La vie des autres villages : chaque village refondu a son économie. Celle du village où l'on est tourne ; celle d'où l'on vient
//! s'endort, garde ses réserves et ses bâtiments, et rattrape le temps passé (les nuits de brigands comprises) quand on revient.
use alw_core::{economie::*, gen::WorldGen, player::Input, session::*, stream::Preset};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, n: usize) { for _ in 0..n { s.tick(0.05, Input::default(), Actions::default()); } }
fn go(s: &mut Session, v: &alw_core::village::Village) { s.player.p = [v.c[0] as f64 + 0.5, v.c[1] as f64 + 30.0, v.c[2] as f64 + 0.5]; s.player.fly = true; ready(s); idle(s, 40); }

#[test]
fn chaque_village_a_sa_vie() {
    let mut s = Session::new(SEED, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.third = false; idle(&mut s, 5); s.set_creative(true);
    let wg = WorldGen::new(SEED); let pilot = s.econ.as_ref().unwrap().id; let pv = wg.site(pilot.0, pilot.1).unwrap().v.clone();
    // un autre village refondu, le plus proche du pilote
    let other = wg.villages_near(pv.c[0] as f64, pv.c[2] as f64, 1400.0).into_iter().filter(|v| v.id != pilot && v.ident.is_some() && !v.works.is_empty()).min_by_key(|v| (v.c[0] - pv.c[0]).abs() + (v.c[2] - pv.c[2]).abs()).expect("un autre village");
    println!("pilote {} ; autre : {} ({:?}), à {} blocs", pv.name, other.name, other.id, (other.c[0] - pv.c[0]).abs() + (other.c[2] - pv.c[2]).abs());
    go(&mut s, &pv); { let e = s.econ.as_mut().unwrap(); e.stock.insert(COIN, 77.0); } let stock_pilot = s.econ.as_ref().unwrap().have(COIN);
    // on part pour l'autre village : son économie s'éveille, celle du pilote s'endort avec sa caisse
    go(&mut s, &other); let e = s.econ.as_ref().unwrap(); assert_eq!(e.id, other.id, "l'économie du village où l'on est");
    assert!(s.econs.contains_key(&pilot), "le pilote dort"); println!("{}", s.village_report());
    let mayor = s.town.folk.iter().any(|f| f.v == other.id && f.job == 9); assert!(mayor, "ce village a son maire");
    // quelques jours passent là-bas ; il produit, il a son journal
    s.hour = 9.0; for _ in 0..3 { s.pass_hours(24.0); idle(&mut s, 20); } let e = s.econ.as_ref().unwrap(); println!("{} après trois jours : {:?}", other.name, e.log.iter().rev().take(3).collect::<Vec<_>>());
    assert!(e.t >= s.hours - 1.0, "le temps tourne pour lui");
    // la partie garde les deux villages
    let text = s.save_text(); let n = text.lines().filter(|l| l.starts_with("econ ")).count(); println!("{n} économies dans la sauvegarde"); assert!(n >= 2, "les deux villages sont gardés");
    // retour au pilote : il se réveille avec sa caisse, et rattrape les trois jours
    go(&mut s, &pv); let e = s.econ.as_ref().unwrap(); assert_eq!(e.id, pilot); println!("pilote au retour : caisse {} (avant {stock_pilot}), journal {:?}", e.have(COIN), e.log.iter().rev().take(2).collect::<Vec<_>>());
    assert!(e.t >= s.hours - 1.0, "le pilote a rattrapé le temps"); assert!(s.econs.contains_key(&other.id), "l'autre village dort à son tour");
    let mut t = Session::new(SEED, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); t.load_text(&text).unwrap();
    let ids: Vec<(i32, i32)> = t.econ.iter().map(|e| e.id).chain(t.econs.keys().copied()).collect(); println!("rechargé : {ids:?}"); assert!(ids.contains(&pilot) && ids.contains(&other.id), "les deux économies reviennent au chargement");
}
