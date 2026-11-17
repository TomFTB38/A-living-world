//! Les brigands : une bande marche de nuit sur le village pilote. Sans muraille, elle atteint l'entrepôt et emporte une part
//! des vivres et de la caisse ; derrière la muraille et la herse baissée, elle reste dehors, les gardes lui tirent dessus du
//! chemin de ronde, et la réserve est intacte.
use alw_core::{economie::*, gen::WorldGen, patrons::Kind, player::{Input, Voxels}, raids, remparts, session::*, stream::Preset};
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

fn walled(guards_towers: u32) -> (Session, alw_core::village::Village) {
    let (mut s, v) = start(); s.hour = 10.0;
    { let e = s.econ.as_mut().unwrap(); e.project = Some((Kind::Muraille, true)); for (k, n) in [("cobblestone", 900.0), ("planks", 300.0), ("oak_log", 150.0), ("iron_ingot", 10.0), (COIN, 200.0)] { e.stock.insert(k, n); } }
    idle(&mut s, 10); for _ in 0..guards_towers { { let e = s.econ.as_mut().unwrap(); e.project = Some((Kind::TourDefense, true)); } idle(&mut s, 30); } idle(&mut s, 120);
    assert!(remparts::ring(s.econ.as_ref().unwrap()).is_some(), "la muraille"); (s, v)
}
fn siege(s: &mut Session, secs: usize) -> (f64, Option<(String, f64)>) {
    let mut hurt = 0.0; let mut bar = None;
    for _ in 0..secs { idle(s, 20); s.hour = 23.0; if let Some(b) = s.siege_bar() { bar = Some(b); }
        hurt = s.horde.foes.iter().filter(|f| f.max > 0.0).map(|f| f.max - f.hp.max(0.0)).sum::<f64>().max(hurt); if s.econ.as_ref().unwrap().raid.is_none() { break; } }
    (hurt, bar)
}

#[test]
fn la_bande_frappe_la_herse_et_les_gardes_tirent() {
    let (mut s, _v) = walled(1); s.hour = 22.5; idle(&mut s, 40); s.hour = 22.5;
    assert!(s.raid_now(), "une bande arrive"); let (hurt, bar) = siege(&mut s, 400);
    let e = s.econ.as_ref().unwrap(); let gate = e.marks.iter().find(|m| m.gate).map(|m| m.hp);
    println!("herse : {gate:?} ; barre vue : {bar:?} ; {hurt:.0} pv perdus sous les flèches ; journal {:?}", e.log.iter().rev().take(3).collect::<Vec<_>>());
    assert!(gate.is_some_and(|hp| hp < raids::GATE_HP), "la herse a pris des coups"); assert!(bar.is_some_and(|b| b.0.starts_with("Herse")), "la barre de la herse");
    assert!(hurt > 0.0, "les gardes ont tiré sur la bande"); assert!(e.raid.is_none(), "l'attaque est finie");
}

#[test]
fn une_breche_dans_la_muraille_puis_les_macons_la_referment() {
    let (mut s, v) = walled(0); s.hour = 22.5; idle(&mut s, 40); s.hour = 22.5;
    // sans porte, la bande vient droit sur la réserve et bute sur la muraille
    { let e = s.econ.as_mut().unwrap(); e.watch.gates.clear(); }
    assert!(s.raid_now(), "une bande arrive");
    let mut breach = None;
    for _ in 0..400 { idle(&mut s, 20); s.hour = 23.0; s.econ.as_mut().unwrap().watch.gates.clear();
        let e = s.econ.as_mut().unwrap(); if let Some(m) = e.marks.iter_mut().find(|m| !m.gate) { if breach.is_none() && m.hp > 20.0 && m.hp < raids::WALL_HP { m.hp = 20.0; } if m.down() && breach.is_none() { breach = Some(m.clone()); } }
        if e.raid.is_none() { break; } }
    let m = breach.expect("une brèche"); println!("brèche en {:?} : {} blocs abattus ; journal {:?}", m.at, m.broken.len(), s.econ.as_ref().unwrap().log.iter().rev().take(3).collect::<Vec<_>>());
    assert!(m.broken.len() > 20, "trois pas de muraille abattus"); assert!(m.broken.iter().all(|b| s.world.get(b.0, b.1, b.2) == Some(0)), "la brèche est ouverte");
    // le lendemain, les maçons réparent avec la pierre de la réserve, et la muraille se relève
    for _ in 0..60 { s.econ.as_mut().unwrap().raid = None; s.hour = 10.0; s.pass_hours(0.2); idle(&mut s, 5); }
    let e = s.econ.as_ref().unwrap(); println!("après : {:?} ; journal {:?}", e.marks.iter().map(|m| m.hp).collect::<Vec<_>>(), e.log.last());
    assert!(e.marks.is_empty(), "tout est réparé"); assert!(m.broken.iter().all(|b| s.world.get(b.0, b.1, b.2) == Some(b.3)), "la muraille est relevée"); let _ = v;
}
