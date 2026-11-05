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
    let heros = |s: &Session| s.draws(&md, &v).into_iter().filter(|d| d.hero).count(); s.third = true; let d = s.draws(&md, &v); assert_eq!(heros(&s), 7, "corps et outil"); for k in &d { assert!(k.m[12].is_finite() && k.r.count > 0); }
    s.third = false; assert_eq!(heros(&s), 2, "bras et outil"); s.sel = 8; assert_eq!(heros(&s), 1, "poing"); s.sel = 5; assert_eq!(heros(&s), 1, "bloc seul");
    for _ in 0..30 { s.anim.update(1.0 / 60.0, alw_core::actors::AnimIn { speed: 6.0, ground: true, mining: true, ..Default::default() }); } assert!(s.anim.swing > 0.0);
    // glisser-déposer : échange, réunion, retour à la case d'origine
    s.inv = [None; INV_SIZE]; s.add_item("dirt", 10); s.add_item("stone", 5); s.grab_slot(0); s.drop_slot(Some(1)); assert_eq!(s.inv[1].unwrap().item, "dirt"); assert_eq!(s.inv[0].unwrap().item, "stone");
    s.add_item("stone", 3); s.grab_slot(1); s.drop_slot(None); assert_eq!(s.inv[1].unwrap().item, "dirt");
    s.inv[2] = Some(Stack { item: "dirt", n: Some(95) }); s.grab_slot(1); s.drop_slot(Some(2)); assert_eq!(s.inv[2].unwrap().n, Some(99)); assert_eq!(s.inv[1].unwrap().n, Some(6));
    s.grab_catalog("lantern"); s.drop_slot(Some(9)); assert_eq!(s.inv[9].unwrap().n, Some(99));
}

#[test]
fn categories_et_gestes_de_l_inventaire() {
    let mut all: Vec<&str> = TABS[1..].iter().flat_map(|t| t.1.iter().copied()).collect(); all.sort(); let mut o = ORDER.to_vec(); o.sort(); assert_eq!(all, o, "chaque objet a une catégorie, une seule");
    let mut s = session(); s.inv = [None; INV_SIZE]; s.add_item("dirt", 51);
    s.take_half(0); assert_eq!(s.cursor.unwrap().n, Some(26)); assert_eq!(s.inv[0].unwrap().n, Some(25));
    s.put_one(1); assert_eq!(s.inv[1].unwrap().n, Some(1)); assert_eq!(s.cursor.unwrap().n, Some(25));
    s.drop_slot(None); assert_eq!(s.inv[0].unwrap().n, Some(50), "le reste revient à sa case"); assert!(s.cursor.is_none());
    s.quick_slot(0); assert!(s.inv[0].is_none()); assert_eq!(s.inv[9].unwrap().n, Some(50)); s.quick_slot(9); assert_eq!(s.inv[1].unwrap().n, Some(51), "réuni avec le tas de la barre");
    s.quick_catalog("granite"); assert_eq!(s.held().unwrap().id, "granite"); s.quick_catalog("sword"); assert_eq!(s.held().unwrap().id, "sword");
    s.grab_slot(1); s.drop_slot(Some(s.sel)); assert_eq!(s.held().unwrap().id, "dirt"); assert_eq!(s.inv[1].unwrap().item, "sword", "échange des cases");
}
#[test]
fn sons_et_ambiance_du_jeu() {
    use alw_core::audio::*; use alw_core::weather::*; use std::sync::mpsc::channel;
    let (tx, rx) = channel(); let mut s = session(); s.sound = SoundScape::new(Some(tx)); s.sel = 0; let drain = |r: &std::sync::mpsc::Receiver<Cmd>| { let (mut p, mut e, mut sets) = (0, 0, Vec::new()); while let Ok(c) = r.try_recv() { match c { Cmd::Play { l, .. } => { p += 1; sets.push(l.set); } Cmd::Env { .. } => e += 1, _ => {} } } (p, e, sets) };
    for _ in 0..60 { s.tick(1.0 / 60.0, Input { z: 1.0, ..Default::default() }, Actions::default()); } let (p, e, sets) = drain(&rx); println!("marche {p} sons {sets:?}, {e} ambiances"); assert!(p >= 2, "des pas"); assert!(e >= 4);
    s.player.pitch = 1.5; s.tick(0.0, Input::default(), Actions::default()); assert!(s.target.is_none(), "rien en face"); for _ in 0..30 { s.tick(1.0 / 60.0, Input::default(), Actions { mine: true, place: false }); } let (p, _, sets) = drain(&rx); println!("vide {sets:?}"); assert!(p >= 1 && sets.contains(&"whoosh"), "frapper le vide avec la pioche");
    aim_down(&mut s); for _ in 0..240 { s.tick(1.0 / 60.0, Input::default(), Actions { mine: true, place: false }); } let (p, _, sets) = drain(&rx); println!("casse {p} {sets:?}"); assert!(p >= 3, "coups puis casse");
    assert!(!s.covered([0.5, 200.0, 0.5])); s.weather.snap(Kind::Pluie); let mut k = view_sky(); let e = apply(&mut k, s.weather.state, false); assert!(e.rain == 1.0 && e.fog_k > 2.0 && e.mist > 0.0); let c = alw_core::view::sky_at(16.4); assert!(k.col[0] < c.col[0] * 0.4, "la pluie atténue le soleil");
}
fn view_sky() -> alw_core::view::Sky { alw_core::view::sky_at(16.4) }
#[test]
fn la_faune_vit_dans_le_monde_reel() {
    let mut s = session(); let (mut deer, mut bird) = (0, 0);
    for k in 0..3600 { s.tick(1.0 / 60.0, Input::default(), Actions::default()); deer = deer.max(s.fauna.deer.len()); bird = bird.max(s.fauna.birds.len()); if k % 120 == 0 { let p = s.player.p; s.world.update(p[0], p[2]); } }
    println!("{deer} cerfs, {bird} oiseaux au plus"); assert!(deer + bird > 0, "des animaux apparaissent");
    let md = alw_core::actors::Models::build(); let d = s.draws(&md, &alw_core::view::look_dir([0.0; 3], [0.0, 0.0, 1.0])); assert!(d.iter().filter(|x| !x.hero).count() == s.fauna.deer.len() * 6 + s.fauna.birds.len() * 3); assert!(d.len() <= 96);
    for id in ORDER { let m = md.icon_matrix(id); let h = md.held(id).unwrap(); let (a, b) = (h.r.first as usize / 6 * 4, (h.r.first + h.r.count) as usize / 6 * 4);
        for v in md.data[a * 20..b * 20].chunks_exact(20) { let p = [0, 2, 4].map(|k| i16::from_le_bytes([v[k], v[k + 1]]) as f32 / 128.0); let q = alw_core::view::mul(&m, &alw_core::view::trs(p[0] as f64, p[1] as f64, p[2] as f64, 0.0, 0.0, 1.0)); assert!(q[12].abs() <= 0.93 && q[13].abs() <= 0.93 && q[14] >= -0.001 && q[14] <= 1.001, "{id} sort de son icône : {:?}", &q[12..15]); } }
}
