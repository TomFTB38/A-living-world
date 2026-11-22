//! La machine volante : posée au sol, on y monte, elle roule, décolle, monte en battant des ailes (au prix du souffle),
//! plane en descendant doucement, vire, pique, décroche trop lente, se pose seule, s'arrête contre un mur ; on en descend
//! une fois posé, on la range ; elle se sauve avec la partie.
use alw_core::{machine::{self, Flyer}, player::*, session::*, stream::*};
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn field(creative: bool) -> (Session, f64) {
    let y = alw_core::gen::FLAT_Y as f64 + 1.0; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 4, lod1: 1.5, lod2: 2.5 }, 4, [2000.5, y, 1975.5]); ready(&mut s);
    s.set_creative(creative); s.hour = 12.0; for _ in 0..6 { s.tick(DT, Input::default(), Actions::default()); } s.wild.all.clear(); s.horde.foes.clear(); s.notes.clear(); s.skip_guide(); (s, y)
}
fn go(s: &mut Session, i: Input, secs: f64) { for _ in 0..(secs / DT) as usize { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); s.tick(DT, i, Actions::default()); } }
fn fw() -> Input { Input { z: 1.0, ..Default::default() } }

#[test]
fn voler_planer_se_poser() {
    let (mut s, y) = field(false);
    // Posée au sol, devant soi, face au regard.
    s.inv = [None; INV_SIZE]; s.sel = 0; s.inv[0] = Some(Stack::new("flying_machine", None)); s.player.yaw = std::f64::consts::PI;
    let (x, z) = (2000, 1970); s.target = Some(Target { x, y: y as i32 - 1, z, px: x, py: y as i32, pz: z, id: 1, bx: [0.0, 0.0, 0.0, 1.0, 1.0, 1.0] });
    assert_eq!(s.flyer_click(), Some(true)); assert_eq!(s.flyers.len(), 1, "{:?}", s.notes.last()); assert!(s.inv[0].is_none()); assert!((s.flyers[0].p[1] - y).abs() < 1e-9 && s.flyers[0].ground);
    // On monte (clic sur une aile).
    s.use_t = 0.0; s.eye = [x as f64 + 2.5, y + 3.0, z as f64 + 0.7]; s.look = [0.0, -1.0, 0.0]; s.target = None; assert_eq!(s.flyer_click(), Some(true)); assert_eq!(s.flyer, Some(0));
    // Au sol : sauter sans élan ne décolle pas ; on roule, on prend de la vitesse, on décolle.
    go(&mut s, Input { jump: true, ..Default::default() }, 0.5); assert!(s.flyers[0].ground, "sans élan, on reste au sol");
    go(&mut s, fw(), 2.0); assert!(s.flyers[0].ground && s.flyers[0].v > machine::TAKEOFF, "elle roule : {:.1} m/s", s.flyers[0].v);
    let st0 = s.stamina; go(&mut s, Input { z: 1.0, jump: true, ..Default::default() }, 4.0); let f = s.flyers[0]; println!("après 4 s de battements : {:.1} m au-dessus du sol, {:.1} m/s, souffle {:.0} → {:.0}", f.p[1] - y, f.v, st0, s.stamina);
    assert!(!f.ground && f.p[1] > y + 6.0, "elle monte : {:.1} m", f.p[1] - y); assert!(s.stamina < st0 - 40.0, "battre des ailes essouffle : {:.0} → {:.0}", st0, s.stamina);
    assert!((s.player.p[1] - (f.p[1] + 0.42)).abs() < 1e-9, "le pilote est à bord"); assert!(machine::wing_angle(&f).abs() <= 0.55);
    // Sans battre : on plane, en descendant doucement, en gardant de la vitesse ; accroupi en l'air, on reste à bord.
    let h0 = s.flyers[0].p[1]; let x0 = s.flyers[0].p[2]; go(&mut s, fw(), 2.0); let f = s.flyers[0]; let sink = (h0 - f.p[1]) / 2.0; let run = (x0 - f.p[2]).abs() / 2.0;
    println!("plané : {sink:.2} m/s de chute pour {run:.1} m/s d'avance"); assert!(sink > 0.3 && sink < 2.0 && run > 8.0, "un vrai plané");
    go(&mut s, Input { crouch: true, ..Default::default() }, 0.2); assert_eq!(s.flyer, Some(0), "on ne saute pas en l'air");
    // Virer.
    let yaw = s.flyers[0].yaw; go(&mut s, Input { x: 1.0, z: 1.0, ..Default::default() }, 1.0); assert!((s.flyers[0].yaw - yaw).abs() > 1.0, "elle vire");
    // Reculer fait piquer, puis elle se pose seule au sol.
    go(&mut s, Input { z: -1.0, ..Default::default() }, 1.0); assert!(s.flyers[0].vy < -2.0, "elle pique : {:.1}", s.flyers[0].vy);
    for _ in 0..30 { if s.flyers[0].ground { break; } go(&mut s, Input { z: -1.0, ..Default::default() }, 0.5); }
    let f = s.flyers[0]; assert!(f.ground && (f.p[1] - y).abs() < 1e-9, "posée sur le sol : {:.2}", f.p[1] - y); assert!(s.dead.is_none() && s.vit[0] > 99.0, "sans dommage");
    go(&mut s, Input::default(), 3.0); assert!(s.flyers[0].v < 0.5, "elle s'arrête en roulant");
    // On descend, une fois posé ; on la range.
    go(&mut s, Input { crouch: true, ..Default::default() }, 0.1); assert_eq!(s.flyer, None); let p = s.player.p; assert!((p[1] - y).abs() < 0.2, "à pied, au sol : {p:?}");
    let t = s.save_text(); let mut l = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 60.0, 0.5]); l.load_text(&t).unwrap(); assert_eq!(l.flyers.len(), 1, "sauvée avec la partie");
    let f = s.flyers[0]; s.player.crouch = true; s.use_t = 0.0; s.eye = [f.p[0], f.p[1] + 3.5, f.p[2]]; s.look = [0.0, -1.0, 0.0]; s.target = None; assert_eq!(s.flyer_click(), Some(true));
    assert!(s.flyers.is_empty() && s.inv.iter().flatten().any(|st| st.item == "flying_machine"), "rangée");
}

#[test]
fn decrochage_mur_et_souffle() {
    let (mut s, y) = field(false);
    // Trop lente en l'air, elle décroche et tombe vite.
    let mut f = Flyer::new([2000.5, y + 30.0, 1960.5], 0.0); f.ground = false; f.v = 2.0; s.flyers = vec![f]; s.flyer = Some(0); s.player.p = [2000.5, y + 30.42, 1960.5];
    go(&mut s, Input::default(), 0.6); assert!(s.flyers[0].vy < -2.0, "décrochage : {:.1}", s.flyers[0].vy);
    // Sans souffle, battre ne fait plus monter.
    let mut f = Flyer::new([2000.5, y + 30.0, 1960.5], 0.0); f.ground = false; f.v = 9.0; s.flyers = vec![f]; s.player.p = [2000.5, y + 30.42, 1960.5]; s.stamina = 0.0; s.winded = true;
    let h = s.flyers[0].p[1]; go(&mut s, Input { z: 1.0, jump: true, ..Default::default() }, 1.0); assert!(s.flyers[0].p[1] < h, "essoufflé : on plane seulement");
    // Un mur arrête la machine net.
    let wall: Vec<_> = (-6..=6).flat_map(|dx| (0..6).map(move |dy| (2000 + dx, y as i32 + dy, 1990, alw_core::blocks::by_name("stone_bricks").unwrap()))).collect(); s.world.edit_many(&wall);
    let mut f = Flyer::new([2000.5, y + 1.0, 1984.5], 0.0); f.ground = false; f.v = 10.0; s.flyers = vec![f]; s.player.p = [2000.5, y + 1.42, 1984.5]; s.stamina = 100.0; s.winded = false;
    go(&mut s, Input { z: 1.0, jump: true, ..Default::default() }, 2.0); assert!(s.flyers[0].p[2] < 1990.0 - machine::HALF_L + 0.3, "arrêtée par le mur : z = {:.2}", s.flyers[0].p[2]);
    // En créatif, battre ne coûte rien.
    let (mut c, y) = field(true); let mut f = Flyer::new([2000.5, y + 20.0, 1960.5], 0.0); f.ground = false; f.v = 9.0; c.flyers = vec![f]; c.flyer = Some(0); c.player.p = [2000.5, y + 20.42, 1960.5];
    go(&mut c, Input { z: 1.0, jump: true, ..Default::default() }, 1.0); assert!(c.flyers[0].p[1] > y + 20.5);
}

#[test]
fn la_machine_se_dessine_et_se_fabrique() {
    let md = alw_core::actors::Models::build(); for p in ["flyer", "wing_l", "wing_r"] { assert!(md.part(p).count > 300, "{p} : {}", md.part(p).count); }
    let px = (0..56 * 56).filter(|k| alw_core::icons::look("flying_machine", (k % 56) as f64 / 56.0, (k / 56) as f64 / 56.0).is_some()).count(); assert!(px > 500, "l'icône : {px}");
    assert!(alw_core::survival::recipes().iter().any(|r| r.out == "flying_machine"));
    let f = Flyer::new([0.0, 0.0, 0.0], 0.4); assert!(machine::flyer_ray(&f, [2.5 * 0.4f64.cos(), 5.0, -2.5 * 0.4f64.sin()], [0.0, -1.0, 0.0]).is_some(), "le bout d'une aile se vise"); assert!(machine::flyer_ray(&f, [6.0, 5.0, 0.0], [0.0, -1.0, 0.0]).is_none());
}
