//! Le dromadaire : sauvage dans le désert, il s'apprivoise (blé, cactus, carotte), se selle et se monte ; il va l'amble,
//! porte haut son cavalier, plus vite au pas de course ; il baraque à la demande et revient avec la partie.
use alw_core::{gen::{FLAT, FLAT_Y}, player::Input, session::*, stream::Preset, wild::{self, Kind, Wild}};
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
const AT: [f64; 3] = [-30.5, FLAT_Y as f64 + 1.0, -30.5];
fn session() -> Session { let mut s = Session::new(FLAT | 5, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 1, AT); ready(&mut s); s.set_creative(true); s.third = false; s.hour = 10.0; s.player.p = AT; s.player.yaw = 0.0; s.player.pitch = 0.0; s.tick(0.0, Input::default(), Actions::default()); s }
fn camel(s: &mut Session) { s.wild.all.clear(); s.wild.put(Kind::Camel, [AT[0], AT[1], AT[2] + 2.4], std::f64::consts::PI, false); s.tick(0.0, Input::default(), Actions::default()); }
fn hold(s: &mut Session, id: &str) { s.inv[0] = Some(Stack::new(item(id).unwrap().id, Some(1))); s.sel = 0; }
fn right(s: &mut Session) { s.tick(0.6, Input::default(), Actions::default()); if let Some(c) = s.wild.all.first_mut() { if s.ride.is_none() { c.p = [AT[0], AT[1], AT[2] + 2.4]; } } s.tick(0.0, Input::default(), Actions { place: true, ..Default::default() }); }

#[test]
fn apprivoiser_seller_et_monter_un_dromadaire() {
    let mut s = session(); camel(&mut s);
    hold(&mut s, "cactus"); right(&mut s); assert!(s.wild.all[0].farm.tame, "apprivoisé au cactus : {:?}", s.notes);
    hold(&mut s, "saddle"); right(&mut s); assert!(s.wild.all[0].farm.saddle, "sellé : {:?}", s.notes);
    s.inv[0] = None; right(&mut s); assert!(s.ride.is_some(), "en selle : {:?}", s.notes);
    let seat = alw_core::elevage::seat(Kind::Camel); assert!(seat > alw_core::elevage::seat(Kind::Horse) + 0.2, "on est juché plus haut qu'à cheval : {seat:.2}");
    let run = |s: &mut Session, sprint: bool| { let p0 = s.wild.all[0].p; for _ in 0..60 { s.tick(0.05, Input { z: 1.0, sprint, ..Default::default() }, Actions::default()); } let p1 = s.wild.all[0].p; (p1[0] - p0[0]).hypot(p1[2] - p0[2]) };
    let walk = run(&mut s, false); let fast = run(&mut s, true); println!("3 s au pas : {walk:.1} m ; au pas de course : {fast:.1} m");
    assert!(walk > 8.0 && fast > walk * 1.4, "il avance, plus vite en courant");
    let c = &s.wild.all[0]; assert!((s.player.p[1] - (c.p[1] + seat)).abs() < 0.2, "le cavalier est porté sur la bosse");
    s.tick(0.05, Input { crouch: true, ..Default::default() }, Actions::default()); assert!(s.ride.is_none(), "pied à terre");
    let txt = s.save_text(); let mut o = Session::new(FLAT | 5, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 1, AT); o.load_text(&txt).unwrap();
    assert!(o.wild.all.iter().any(|c| c.kind == Kind::Camel && c.farm.tame && c.farm.saddle), "le dromadaire sellé revient avec la partie");
    // sans selle, un clic à main nue le fait baraquer (il attend), un autre le relève
    let mut s = session(); camel(&mut s); s.wild.all[0].farm.tame = true; s.inv[0] = None; right(&mut s); assert!(s.wild.all[0].farm.sit, "il baraque : {:?}", s.notes);
    right(&mut s); assert!(!s.wild.all[0].farm.sit, "il se relève : {:?}", s.notes);
}

#[test]
fn l_amble_et_le_modele() {
    // l'amble : les deux pattes d'un même côté vont ensemble (avant gauche avec arrière gauche)
    for k in 0..20 { let (_, _, l, _) = wild::gait(Kind::Camel, 4.0, k as f64 * 0.3, 0.6); assert!((l[0] - l[2]).abs() < 1e-9 && (l[1] - l[3]).abs() < 1e-9 && (l[0] + l[1]).abs() < 1e-9, "l'amble : {l:?}"); }
    let md = alw_core::actors::Models::build();
    for v in 0..3 { for p in ["camel", "camelHead", "camelLeg", "camelHind"] { let n = format!("{p}{v}"); assert!(md.part(&n).count > 600, "{n} : {}", md.part(&n).count); } }
}

#[test]
fn des_dromadaires_au_desert() {
    // Dans un désert plat, la faune sauvage fait venir des dromadaires ; ailleurs, aucun.
    let s = session(); let mut w = Wild::new(7); let mut d = Wild::new(7);
    for _ in 0..1200 { w.update(0.05, &s.world, AT, 1.0, 0.0, 0.0, 0.0, true, [0.0, 0.0, 1.0, 0.0, 1.0, 0.0], 0.0); d.update(0.05, &s.world, AT, 1.0, 0.0, 0.0, 1.0, true, [0.0; 6], 0.0); }
    let n = w.all.iter().filter(|c| c.kind == Kind::Camel).count(); println!("{n} dromadaires au désert"); assert!(n >= 1, "des dromadaires au désert");
    assert_eq!(d.all.iter().filter(|c| c.kind == Kind::Camel).count(), 0, "pas dans la prairie");
}
