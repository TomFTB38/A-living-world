//! L'élevage et les compagnons : traire, tondre, ramasser les œufs, apprivoiser un loup, le seller, le chevaucher,
//! le retrouver après rechargement. Dans un monde plat, sur un sol parfaitement dégagé.
use alw_core::{gen::{FLAT, FLAT_Y}, player::Input, session::*, stream::Preset, wild::Kind};
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
const AT: [f64; 3] = [-30.5, FLAT_Y as f64 + 1.0, -30.5];
fn session() -> Session { let mut s = Session::new(FLAT | 5, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 1, AT); ready(&mut s); s.set_creative(true); s.third = false; s.hour = 10.0; s.player.p = AT; s.player.yaw = 0.0; s.player.pitch = -0.42; s.tick(0.0, Input::default(), Actions::default()); s }
/// Une bête posée deux pas devant soi, et un objet en main ; un clic droit.
fn beast(s: &mut Session, k: Kind) -> usize { s.wild.all.clear(); s.wild.put(k, [AT[0], AT[1], AT[2] + 2.0], std::f64::consts::PI, false); s.tick(0.0, Input::default(), Actions::default()); 0 }
fn hold(s: &mut Session, id: &str) { s.inv[0] = Some(Stack::new(item(id).unwrap().id, Some(1))); s.sel = 0; }
fn right(s: &mut Session) { s.tick(0.6, Input::default(), Actions::default()); if let Some(c) = s.wild.all.first_mut() { if s.ride.is_none() { c.p = [AT[0], AT[1], AT[2] + 2.0]; } } s.tick(0.0, Input::default(), Actions { place: true, ..Default::default() }); }

#[test]
fn traire_tondre_et_ramasser_les_oeufs() {
    let mut s = session();
    beast(&mut s, Kind::Cow); hold(&mut s, "bucket"); right(&mut s); assert!(s.count("milk_bucket") >= 1, "un seau de lait : {:?}", s.notes);
    beast(&mut s, Kind::Sheep); hold(&mut s, "shears"); right(&mut s); assert!(s.wild.all[0].farm.shorn > 0.0, "le mouton est tondu : {:?}", s.notes);
    assert!(s.loose.iter().any(|l| l.item == "wool_white") || s.count("wool_white") > 0, "de la laine");
    beast(&mut s, Kind::Chicken); s.wild.all[0].farm.lay = 0.05; for _ in 0..10 { s.tick(0.05, Input::default(), Actions::default()); }
    assert!(s.loose.iter().any(|l| l.item == "chicken_egg") || s.count("chicken_egg") > 0, "la poule a pondu");
}

#[test]
fn apprivoiser_seller_et_chevaucher_un_loup() {
    let mut s = session(); s.player.pitch = -0.75; beast(&mut s, Kind::Wolf);
    hold(&mut s, "raw_meat"); right(&mut s); assert!(s.wild.all[0].farm.tame, "le loup est apprivoisé : {:?} {:?} {:?}", s.notes, s.wild.all.iter().map(|c| c.p).collect::<Vec<_>>(), s.look);
    hold(&mut s, "saddle"); right(&mut s); assert!(s.wild.all[0].farm.saddle, "sellé : {:?}", s.notes);
    s.inv[0] = None; right(&mut s); assert!(s.ride.is_some(), "en selle : {:?}", s.notes);
    let p0 = s.wild.all[0].p; for _ in 0..40 { s.tick(0.05, Input { z: 1.0, ..Default::default() }, Actions::default()); }
    let p1 = s.wild.all[0].p; assert!((p1[0] - p0[0]).hypot(p1[2] - p0[2]) > 3.0, "la monture avance : {p0:?} → {p1:?}");
    assert!((s.player.p[1] - (p1[1] + alw_core::elevage::seat(Kind::Wolf))).abs() < 0.05 && (s.player.p[0] - p1[0]).abs() < 0.05, "le cavalier est porté");
    s.tick(0.05, Input { crouch: true, ..Default::default() }, Actions::default()); assert!(s.ride.is_none(), "pied à terre");
    // le compagnon survit à la sauvegarde
    let txt = s.save_text(); let mut o = Session::new(FLAT | 5, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 1, AT); o.load_text(&txt).unwrap();
    assert!(o.wild.all.iter().any(|c| c.kind == Kind::Wolf && c.farm.tame && c.farm.saddle), "le loup sellé est revenu");
}

#[test]
fn le_cheval_galope_au_sprint() {
    // l'allure : au trot, les pattes vont par diagonales ; au galop, les deux avant ensemble, le dos tangue
    let (_, t0, trot, _) = alw_core::wild::gait(Kind::Horse, 4.0, 1.0, 0.6); assert!(t0 == 0.0 && (trot[0] + trot[1]).abs() < 1e-9, "trot : {trot:?}");
    let (_, _, gal, _) = alw_core::wild::gait(Kind::Horse, 10.0, 1.0, 0.6); assert!(gal[0] * gal[1] > 0.0, "galop : les antérieurs ensemble {gal:?}");
    assert!((0..40).any(|k| alw_core::wild::gait(Kind::Horse, 10.0, k as f64 * 0.2, 0.6).1.abs() > 0.1), "le dos tangue");
    let run = |sprint: bool| {
        let mut s = session(); s.player.pitch = -0.6; beast(&mut s, Kind::Horse); { let c = &mut s.wild.all[0].farm; c.tame = true; c.saddle = true; }
        s.inv[0] = None; right(&mut s); assert!(s.ride.is_some(), "en selle : {:?}", s.notes);
        s.set_creative(false); s.stamina = 1.0; // un cavalier épuisé : c'est la monture qui court
        let p0 = s.wild.all[0].p; let mut ys = Vec::new();
        for _ in 0..60 { s.tick(0.05, Input { z: 1.0, sprint, ..Default::default() }, Actions::default()); ys.push(s.player.p[1] - s.wild.all[0].p[1]); }
        let p1 = s.wild.all[0].p; ((p1[0] - p0[0]).hypot(p1[2] - p0[2]), ys)
    };
    let ((trot, _), (gallop, ys)) = (run(false), run(true));
    assert!(gallop > trot * 1.5, "le galop va bien plus vite : {gallop:.1} contre {trot:.1}");
    let (lo, hi) = ys[20..].iter().fold((f64::MAX, f64::MIN), |a, y| (a.0.min(*y), a.1.max(*y))); assert!(hi - lo > 0.08, "le cavalier est secoué par le galop : {lo:.2}..{hi:.2}");
    assert!(alw_core::elevage::seat(Kind::Wolf) > 1.0, "un loup de monte est grand");
}
