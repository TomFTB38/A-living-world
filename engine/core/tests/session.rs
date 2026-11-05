use alw_core::{blocks::*, player::*, session::*, stream::*};
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn session() -> Session { let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 90.0, 0.5]); ready(&mut s); for _ in 0..500 { s.tick(1.0 / 60.0, Input::default(), Actions::default()); } assert!(s.player.ground); s }
fn aim_down(s: &mut Session) -> Target { s.player.pitch = -1.5; s.tick(0.0, Input::default(), Actions::default()); s.target.expect("visée du sol") }
#[test]
fn casser_poser_sauver() {
    let mut s = session(); let t = aim_down(&mut s); let id0 = t.id; println!("visé {} à {:?}", BLOCKS[t.id as usize].name, (t.x, t.y, t.z));
    s.sel = 0; for _ in 0..200 { s.tick(1.0 / 60.0, Input::default(), Actions { mine: true, place: false }); if s.world.get(t.x, t.y, t.z) != Some(id0) { break; } }
    assert_ne!(s.world.get(t.x, t.y, t.z), Some(id0), "le bloc a disparu");
    ready(&mut s); assert_eq!(s.world.get(t.x, t.y, t.z), Some(0), "le remaillage n'efface pas la modification");
    // rupture de stock et pose
    s.sel = 5; let n0 = s.inv[5].unwrap().n.unwrap();
    for _ in 0..300 { s.tick(1.0 / 60.0, Input::default(), Actions::default()); }
    s.player.pitch = -1.5; s.player.p[0] += 2.0; for _ in 0..30 { s.tick(1.0 / 60.0, Input::default(), Actions::default()); }
    let mut ok = false; for k in 0..40 { for y in 0..8 { s.player.yaw = y as f64 * 0.8; s.player.pitch = -1.4 + k as f64 * 0.05; s.tick(0.0, Input::default(), Actions::default()); if s.place() { ok = true; break; } } if ok { break; } }
    assert!(ok, "pose : cible {:?}, joueur {:?}", s.target, s.player.p); assert_eq!(s.inv[5].unwrap().n, Some(n0 - 1)); ready(&mut s);
    // sauvegarde : un monde neuf retrouve la modification
    let txt = s.save_text(); let mut o = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 90.0, 0.5]); o.load_text(&txt).unwrap(); ready(&mut o);
    assert_eq!(o.world.get(t.x, t.y, t.z), Some(0), "modification retrouvée après rechargement"); assert_eq!(o.inv[5], s.inv[5]);
}
#[test]
fn inventaire() {
    let mut s = session(); s.inv = [None; INV_SIZE]; s.add_item("dirt", 150); assert_eq!(s.inv[0].unwrap().n, Some(99)); assert_eq!(s.inv[1].unwrap().n, Some(51));
    s.add_item("inconnu", 1); assert!(s.inv[2].is_none());
    for b in BLOCKS.iter() { if let Some(d) = b.drop { if b.name != "air" { assert!(item(d).is_some(), "objet pour {d}"); } } }
    assert_eq!(gaze_axis(0.1, -0.9, 0.2, 0, None), 'y'); assert_eq!(gaze_axis(0.9, -0.2, 0.1, 0, None), 'x'); assert_eq!(gaze_axis(0.9, 0.0, 0.1, 3, None), 'z');
}

#[test]
fn personnage_et_inventaire_ecran() {
    let mut s = session(); let md = alw_core::actors::Models::build(); assert!(md.n > 1000);
    for n in ORDER { assert!(md.held(n).is_some(), "{n}"); }
    let v = alw_core::view::identity();
    s.third = true; let d = s.draws(&md, &v); assert_eq!(d.len(), 7, "corps et outil"); for k in &d { assert!(k.m[12].is_finite() && k.r.count > 0); }
    s.third = false; assert_eq!(s.draws(&md, &v).len(), 2, "bras et outil"); s.sel = 8; assert_eq!(s.draws(&md, &v).len(), 1, "poing"); s.sel = 5; assert_eq!(s.draws(&md, &v).len(), 1, "bloc seul");
    for _ in 0..30 { s.anim.update(1.0 / 60.0, alw_core::actors::AnimIn { speed: 6.0, ground: true, mining: true, ..Default::default() }); } assert!(s.anim.swing > 0.0);
    // glisser-déposer : échange, réunion, retour à la case d'origine
    s.inv = [None; INV_SIZE]; s.add_item("dirt", 10); s.add_item("stone", 5); s.grab_slot(0); s.drop_slot(Some(1)); assert_eq!(s.inv[1].unwrap().item, "dirt"); assert_eq!(s.inv[0].unwrap().item, "stone");
    s.add_item("stone", 3); s.grab_slot(1); s.drop_slot(None); assert_eq!(s.inv[1].unwrap().item, "dirt");
    s.inv[2] = Some(Stack { item: "dirt", n: Some(95) }); s.grab_slot(1); s.drop_slot(Some(2)); assert_eq!(s.inv[2].unwrap().n, Some(99)); assert_eq!(s.inv[1].unwrap().n, Some(6));
    s.grab_catalog("lantern"); s.drop_slot(Some(9)); assert_eq!(s.inv[9].unwrap().n, Some(99));
}
