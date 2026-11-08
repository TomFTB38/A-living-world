//! Construction : formes orientées, collisions fidèles à la géométrie, marches sans saut, portes, échelles, dalles.
use alw_core::{blocks::*, player::*, session::*, shapes::{self, Kind}, stream::*, survival::recipes};
const DT: f64 = 1.0 / 60.0;
fn none() -> Input { Input::default() }
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, sec: f64) { for _ in 0..(sec / DT) as usize { s.tick(DT, none(), Actions::default()); } }
fn look_at(s: &mut Session, q: [f64; 3]) { let e = s.player.eye(); let d = [q[0] - e[0], q[1] - e[1], q[2] - e[2]]; s.player.yaw = d[0].atan2(d[2]); s.player.pitch = (d[1] / d[0].hypot(d[2])).atan(); s.tick(0.0, none(), Actions::default()); }
fn walk(s: &mut Session, sec: f64, i: Input) { for _ in 0..(sec / DT) as usize { s.tick(DT, i, Actions::default()); } }
/// Un chantier : une dalle de pierre plate de 9 × 13, dégagée sur six blocs de haut ; le joueur au milieu, regard vers +z.
fn site() -> (Session, [i32; 3]) {
    let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, 120.0, 0.5]); ready(&mut s); s.third = false;
    let g = alw_core::gen::WorldGen::new(20261104); let y = g.column(0.5, 0.5).h.floor() as i32 + 3; let mut e = Vec::new();
    for x in -4..=4 { for z in -6..=6 { e.push((x, y - 1, z, B::STONE)); for h in 0..7 { e.push((x, y + h, z, 0)); } } } s.world.edit_many(&e);
    s.player.p = [0.5, y as f64, -3.5]; s.player.yaw = 0.0; s.player.pitch = 0.0; idle(&mut s, 0.5); assert!(s.player.ground); (s, [0, y, 0])
}

#[test]
fn formes_et_orientations() {
    for b in BLOCKS.iter() { let Some(i) = shapes::info(b.id) else { continue };
        assert_eq!(shapes::variant(i.base, i.facing, i.top, i.open, i.upper), b.id, "{} se retrouve depuis sa base", b.name);
        for bb in shapes::geometry(i) { for k in 0..3 { assert!(bb[k] >= 0.0 && bb[k + 3] <= 1.0 && bb[k] < bb[k + 3], "{} : boîte {bb:?}", b.name); } }
        assert!(item(BLOCKS[i.base as usize].name).is_some(), "{} : l'objet de base existe", b.name); }
    // un quart de tour : la marche haute passe de +z à +x
    let up = |id: u8| shapes::geometry(shapes::info(id).unwrap())[1]; let st = by_name("planks_stairs").unwrap();
    assert_eq!(up(st), [0.0, 0.5, 0.5, 1.0, 1.0, 1.0]); assert_eq!(up(shapes::variant(st, 1, false, false, false)), [0.5, 0.5, 0.0, 1.0, 1.0, 1.0]); assert_eq!(up(shapes::variant(st, 2, false, false, false)), [0.0, 0.5, 0.0, 1.0, 1.0, 0.5]);
    assert_eq!(shapes::facing_of(0.1, 0.9), 0); assert_eq!(shapes::facing_of(0.9, 0.1), 1); assert_eq!(shapes::facing_of(-0.1, -0.9), 2); assert_eq!(shapes::facing_of(-0.9, 0.2), 3);
    assert!(recipes().iter().any(|r| r.out == "door") && recipes().iter().any(|r| r.out == "planks_stairs") && recipes().iter().any(|r| r.out == "window"));
}

#[test]
fn escalier_sans_sauter_et_dalle() {
    let (mut s, [_, y, _]) = site();
    // un escalier de bois qui monte vers +z, puis un palier d'un bloc : on avance sans jamais sauter
    s.world.edit_many(&[(0, y, -1, by_name("planks_stairs").unwrap()), (0, y, 0, B::PLANKS), (0, y, 1, B::PLANKS), (-1, y, -1, B::STONE), (1, y, -1, B::STONE), (-1, y, 0, B::STONE), (1, y, 0, B::STONE)]);
    let mut k = 0; while s.player.p[2] < 0.3 && k < 240 { s.tick(DT, Input { z: 1.0, ..none() }, Actions::default()); k += 1; } assert!(s.player.p[2] > 0.0, "on a franchi l'escalier : z = {:.2}", s.player.p[2]); assert!((s.player.p[1] - (y + 1) as f64).abs() < 0.01, "sur le palier : y = {:.3}", s.player.p[1]);
    // une dalle basse se monte aussi sans sauter, et on se tient à sa surface, à mi-hauteur
    let (mut s, [_, y, _]) = site(); s.world.edit(0, y, -2, by_name("planks_slab").unwrap()); walk(&mut s, 0.5, Input { z: 1.0, ..none() }); idle(&mut s, 0.3);
    assert!((s.player.p[1] - (y as f64 + 0.5)).abs() < 0.01 && s.player.p[2] > -2.6, "posé sur la dalle : y = {:.3}, z = {:.2}", s.player.p[1], s.player.p[2]);
    // tomber sur une dalle : on s'arrête sur sa surface, pas au bloc entier
    let (mut s, [_, y, _]) = site(); s.world.edit(0, y, -4, by_name("stone_bricks_slab").unwrap()); s.player.p = [0.5, y as f64 + 3.0, -3.5]; idle(&mut s, 1.5); assert!((s.player.p[1] - (y as f64 + 0.5)).abs() < 0.01, "chute sur la dalle : {:.3}", s.player.p[1]);
    // une dalle haute laisse passer en dessous ? non : 1,8 m ne tient pas sous 0,5 m… mais on bute dessus en avançant, sans monter
    let (mut s, [_, y, _]) = site(); s.world.edit_many(&[(0, y + 1, -2, by_name("planks_slab_top").unwrap())]); walk(&mut s, 1.0, Input { z: 1.0, ..none() }); assert!(s.player.p[2] < -2.2 && (s.player.p[1] - y as f64).abs() < 0.01, "la dalle haute arrête la tête");
}

#[test]
fn porte_echelle_et_mur_fin() {
    let (mut s, [_, y, _]) = site();
    // porte posée en regardant vers +z, dans une rangée de murs : fermée elle arrête, ouverte on passe
    let mut wall = Vec::new(); for x in -4..=4 { if x != 0 { for h in 0..3 { wall.push((x, y + h, 0, B::STONE)); } } } s.world.edit_many(&wall);
    s.inv[0] = Some(Stack::new("door", Some(1))); s.sel = 0; s.player.p = [0.5, y as f64, -2.5]; idle(&mut s, 0.2);
    look_at(&mut s, [0.5, y as f64, 0.5]); assert_eq!(s.target.map(|t| (t.x, t.y, t.z)), Some((0, y - 1, 0)), "on vise le sol au pied du mur"); s.tick(DT, none(), Actions { mine: false, place: true, ..Default::default() }); idle(&mut s, 0.4);
    let lo = s.world.get(0, y, 0).unwrap(); let hi = s.world.get(0, y + 1, 0).unwrap();
    let (li, ui) = (shapes::info(lo).unwrap_or_else(|| panic!("bas de porte : {}", BLOCKS[lo as usize].name)), shapes::info(hi).expect("haut de porte")); assert!(li.kind == Kind::Door && !li.upper && ui.upper && li.facing == 0 && !li.open);
    s.player.yaw = 0.0; s.player.pitch = 0.0; walk(&mut s, 1.5, Input { z: 1.0, ..none() }); assert!(s.player.p[2] < 0.0, "porte fermée : on bute, z = {:.2}", s.player.p[2]);
    look_at(&mut s, [0.5, y as f64 + 1.0, 0.1]); assert!(s.target.is_some_and(|t| t.x == 0 && t.z == 0), "on vise la porte : {:?}", s.target.map(|t| (t.x, t.y, t.z))); idle(&mut s, 0.5); s.tick(DT, none(), Actions { mine: false, place: true, ..Default::default() });
    assert!(shapes::info(s.world.get(0, y, 0).unwrap()).unwrap().open && shapes::info(s.world.get(0, y + 1, 0).unwrap()).unwrap().open, "les deux moitiés s'ouvrent");
    s.player.yaw = 0.0; s.player.pitch = 0.0; walk(&mut s, 1.5, Input { z: 1.0, ..none() }); assert!(s.player.p[2] > 1.0, "porte ouverte : on passe, z = {:.2}", s.player.p[2]);
    // casser le haut : toute la porte part, une seule porte revient
    s.inv = [None; INV_SIZE]; s.player.p = [0.5, y as f64, -2.5]; s.inv[0] = Some(Stack::new("axe", None)); s.sel = 0; idle(&mut s, 0.3);
    look_at(&mut s, [0.5, y as f64 + 1.5, 0.5]); let mut k = 0; while s.world.get(0, y + 1, 0).is_some_and(|b| b != 0) && k < 2000 { s.stamina = 100.0; s.tick(DT, none(), Actions { mine: true, place: false, ..Default::default() }); k += 1; }
    assert_eq!((s.world.get(0, y, 0), s.world.get(0, y + 1, 0)), (Some(0), Some(0)), "la porte part en entier"); assert_eq!(s.count("door"), 1, "une porte rendue");
    // échelle contre un mur : avancer fait grimper, sans sauter
    let (mut s, [_, y, _]) = site(); let mut e = Vec::new(); for h in 0..5 { e.push((0, y + h, -1, B::STONE)); e.push((0, y + h, -2, by_name("ladder").unwrap())); } s.world.edit_many(&e);
    s.player.p = [0.5, y as f64, -2.5]; walk(&mut s, 1.2, Input { z: 1.0, ..none() }); assert!(s.player.p[1] > y as f64 + 2.5, "on grimpe : y = {:.2}", s.player.p[1] - y as f64);
    let top = s.player.p[1]; walk(&mut s, 1.0, Input { crouch: true, ..none() }); assert!(s.player.p[1] < top - 1.0, "accroupi, on redescend");
    // mur fin et pilier : on s'en approche plus près qu'un bloc plein
    let (mut s, [_, y, _]) = site(); s.world.edit_many(&[(0, y, -1, by_name("stone_wall").unwrap()), (0, y + 1, -1, by_name("stone_wall").unwrap())]); walk(&mut s, 1.0, Input { z: 1.0, ..none() });
    assert!(s.player.p[2] > -1.0 + 0.0 && s.player.p[2] < -0.6, "arrêté par le mur fin, plus près qu'un bloc : z = {:.3}", s.player.p[2]);
}

#[test]
fn pose_rotation_dalles_et_sauvegarde() {
    let (mut s, [_, y, _]) = site(); s.player.p = [0.5, y as f64, -2.5];
    let aim_floor = |s: &mut Session, z: f64| { let e = s.player.eye(); s.player.pitch = -((e[1] - y as f64 - 0.01) / (z - e[2])).atan(); s.tick(0.0, none(), Actions::default()); };
    let put = |s: &mut Session| { s.tick(DT, none(), Actions { mine: false, place: true, ..Default::default() }); idle(s, 0.3); };
    // escalier : dans le sens du regard, puis R tourne d'un quart de tour
    s.inv[0] = Some(Stack::new("planks_stairs", Some(8))); s.sel = 0; aim_floor(&mut s, 0.5); assert_eq!(s.ghost_boxes().len(), 2, "l'aperçu montre les deux marches"); put(&mut s);
    assert_eq!(shapes::info(s.world.get(0, y, 0).unwrap()).unwrap().facing, 0);
    assert_eq!(s.rotate(), "Rotation : 90 °"); look_at(&mut s, [1.5, y as f64, 0.5]); put(&mut s); assert_eq!(shapes::info(s.world.get(1, y, 0).unwrap()).map(|i| i.facing), Some(1), "R : un quart de tour");
    s.inv[0] = Some(Stack::new("planks", Some(8))); let m = s.rotate(); assert!(m.starts_with("Orientation"), "sans bloc de forme, R garde son rôle : {m}"); s.turn = 0;
    // dalle : visée en haut d'une face, dalle haute ; sur le dessus d'une dalle basse de même matière, un bloc plein
    let (mut s, [_, y, _]) = site(); s.player.p = [0.5, y as f64, -2.5]; s.world.edit(0, y, 0, B::STONE); s.inv[0] = Some(Stack::new("planks_slab", Some(8))); s.sel = 0;
    let e = s.player.eye(); s.player.pitch = ((y as f64 + 0.8 - e[1]) / (0.0 - e[2])).atan(); s.tick(0.0, none(), Actions::default()); put(&mut s); assert_eq!(s.world.get(0, y, -1), by_name("planks_slab_top"), "visée haute : dalle haute");
    s.world.edit(0, y, -1, 0); s.player.pitch = ((y as f64 + 0.2 - e[1]) / (0.0 - e[2])).atan(); s.tick(0.0, none(), Actions::default()); put(&mut s); assert_eq!(s.world.get(0, y, -1), by_name("planks_slab"), "visée basse : dalle basse");
    s.player.pitch = ((y as f64 + 0.99 - e[1]) / (-0.5 - e[2])).atan(); s.tick(0.0, none(), Actions::default()); assert_eq!(s.target.map(|t| (t.x, t.y, t.z)), Some((0, y, -1))); put(&mut s);
    assert_eq!(s.world.get(0, y, -1), Some(B::PLANKS), "deux dalles font un bloc"); assert_eq!(s.count("planks_slab"), 5);
    // tout reste après rechargement
    let txt = s.save_text(); let mut o = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, 120.0, 0.5]); o.load_text(&txt).unwrap(); ready(&mut o);
    assert_eq!(o.world.get(0, y, -1), Some(B::PLANKS)); assert_eq!(o.world.get(0, y, 0), Some(B::STONE));
    // les modèles se dessinent
    let m = alw_core::mesher::Mesher::new(); for b in BLOCKS.iter().filter(|b| shapes::info(b.id).is_some()) { let mut v = alw_core::mesher::VBuf::new(400); m.deco_model(&mut v, b.id, 0, 0, 0, 3, 5, 255.0, 0.0, 0); assert!(v.n >= 24, "{} se dessine", b.name); }
}
