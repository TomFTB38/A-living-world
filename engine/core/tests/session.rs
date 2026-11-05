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

#[test]
fn arbre_abattu_a_la_hache() {
    use alw_core::felling::find_tree;
    let mut s = session(); s.player.fly = true; let p0 = s.player.p; let (mut spot, mut sz) = (None, 0);
    for dx in -28..=28 { for dz in -28..=28 { for y in (p0[1] as i32 - 8)..(p0[1] as i32 + 14) { let (x, z) = (p0[0] as i32 + dx, p0[2] as i32 + dz); if s.world.get(x, y, z).is_some_and(is_log) && s.world.get(x, y - 1, z).is_some_and(|b| !is_log(b)) { if let Some(t) = find_tree(&s.world, x, y, z) { if t.logs.len() > sz { sz = t.logs.len(); spot = Some((x, y, z, t)); } } } } } }
    let real = spot.expect("un arbre près du départ"); println!("arbre du monde : {} bûches, {} feuilles", real.3.logs.len(), real.3.leaves.len()); assert!(real.3.leaves.len() >= 6);
    // un arbre d'essai en plein ciel : la visée ne dépend pas du relief
    let (x, y, z) = (p0[0] as i32 + 4, 120, p0[2] as i32); s.world.edit(x, y - 1, z, B::STONE);
    for k in 0..8 { s.world.edit(x, y + k, z, B::OAK_LOG); } for dx in -2..=2 { for dz in -2..=2 { for k in 7..10 { if (dx, dz) != (0, 0) || k > 7 { s.world.edit(x + dx, y + k, z + dz, B::OAK_LEAVES); } } } }
    let tree = find_tree(&s.world, x, y + 1, z).expect("arbre d'essai"); ready(&mut s);
    s.player.p = [x as f64 + 4.5, y as f64, z as f64 + 0.5]; let cut = (x, y + 1, z); let e = s.player.eye(); let d = [cut.0 as f64 + 0.5 - e[0], cut.1 as f64 + 0.5 - e[1], cut.2 as f64 + 0.5 - e[2]]; let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt(); s.third = false; s.player.yaw = d[0].atan2(d[2]); s.player.pitch = (d[1] / l).asin();
    s.inv = [None; INV_SIZE]; s.inv[1] = Some(Stack { item: "axe", n: None }); s.sel = 1; s.tick(0.0, Input::default(), Actions::default()); let t = s.target.expect("le tronc est visé"); assert!(is_log(t.id), "visé {}", BLOCKS[t.id as usize].name);
    let a = Actions { mine: true, place: false }; for _ in 0..(9.5 * 60.0) as i32 { s.tick(1.0 / 60.0, Input::default(), a); } assert!(s.falling.is_none(), "encore debout après 9,5 s"); assert!(s.progress() > 0.9); assert_eq!(s.world.get(t.x, t.y, t.z), Some(t.id));
    for _ in 0..60 { s.tick(1.0 / 60.0, Input::default(), a); } assert!(s.falling.is_some(), "tombe après 10 s"); assert_eq!(s.world.get(t.x, t.y, t.z), Some(0));
    for (px, py, pz) in tree.leaves.iter().copied() { assert!(s.world.get(px, py, pz).is_none_or(|b| !is_leaf(b)), "feuillage disparu"); }
    assert!(tree.logs.iter().filter(|p| p.1 < t.y).all(|p| s.world.get(p.0, p.1, p.2).is_some_and(is_log)), "le pied reste"); let got = s.inv.iter().flatten().filter(|i| i.item.ends_with("_log")).map(|i| i.n.unwrap()).sum::<u32>(); assert!(got > 0); println!("récolté {got}");
    let md = alw_core::actors::Models::build(); let v = alw_core::view::look_dir(s.eye, s.look); assert!(s.draws(&md, &v).iter().any(|d| d.dynamic)); let ended = (0..600).any(|_| { s.tick(1.0 / 60.0, Input::default(), Actions::default()); s.falling.is_none() }); assert!(ended, "la chute se termine");
    // les bûches posées à la main ne sont pas un arbre
    s.world.edit(x + 6, y + 30, z, B::OAK_LOG); assert!(find_tree(&s.world, x + 6, y + 30, z).is_none());
}
#[test]
fn l_eau_posee_coule() {
    use alw_core::flow::*; use std::collections::HashMap;
    struct W(HashMap<(i32, i32, i32), u8>); impl Voxels for W { fn get(&self, x: i32, y: i32, z: i32) -> Option<u8> { if y < 10 { return Some(B::STONE); } Some(*self.0.get(&(x, y, z)).unwrap_or(&0)) } }
    let mut w = W(HashMap::new()); w.0.insert((0, 14, 0), B::WATER); let mut f = Flow::default(); f.touch(0, 14, 0); let run = |w: &mut W, f: &mut Flow, n: usize| for _ in 0..n { for (x, y, z, b) in f.step(STEP, &*w) { if b == 0 { w.0.remove(&(x, y, z)); } else { w.0.insert((x, y, z), b); } } };
    run(&mut w, &mut f, 80); assert!(!f.busy(), "l'eau se stabilise");
    assert_eq!(w.0.get(&(0, 13, 0)).copied().map(water_level), Some(7), "elle tombe"); assert_eq!(w.0.get(&(0, 10, 0)).copied().map(water_level), Some(7));
    assert_eq!(w.0.get(&(3, 10, 0)).copied().map(water_level), Some(5), "elle s'étale en perdant un niveau par case"); assert_eq!(w.0.get(&(7, 10, 0)).copied().map(water_level), Some(1)); assert!(!w.0.contains_key(&(8, 10, 0)), "et s'arrête");
    assert_eq!(w.0.get(&(1, 14, 0)).copied().map(water_level), Some(7), "la source alimente ses côtés");
    w.0.remove(&(0, 14, 0)); f.touch(0, 14, 0); run(&mut w, &mut f, 200); assert!(w.0.is_empty(), "sans source tout se vide : {:?}", w.0.len());
}
#[test]
fn l_eau_coule_sur_le_vrai_relief() {
    let mut s = Session::new(20261104, &Preset { radius: 4, lod1: 2.3, lod2: 4.6 }, 4, [-38.5, 52.0, 0.5]); s.player.fly = true; ready(&mut s);
    let (x, z) = (-33, -4); let h = alw_core::gen::WorldGen::new(20261104).column(x as f64, z as f64).h as i32; let y = h + 2; assert!(s.world.edit(x, y, z, B::WATER)); s.flow.touch(x, y, z);
    let pump = |s: &mut Session| { for _ in 0..4000 { let p = s.player.p; let (b, _) = s.world.update(p[0], p[2]); for c in &b { assert!(c.mesh.n_w % 4 == 0); } if s.world.waiting() == 0 { return; } std::thread::sleep(std::time::Duration::from_millis(1)); } panic!("remaillage bloqué"); };
    let mut n = 0; for _ in 0..240 { s.tick(1.0 / 60.0, Input::default(), Actions::default()); pump(&mut s); n += 1; }
    let mut cells = 0; for dx in -30..30 { for dz in -30..30 { for y in 30..80 { if s.world.get(x + dx, y, z + dz).is_some_and(|b| is_water(b) && b != B::WATER) { cells += 1; } } } } println!("{n} étapes, {cells} cases d'eau courante"); assert!(cells > 5);
}
