//! Ateliers et coffres : l'interface de chaque atelier, l'ouvrage qui prend le temps de son geste, le grand coffre.
use alw_core::{blocks::*, data::Station, player::*, session::*, stream::*, survival::recipes};
const DT: f64 = 1.0 / 60.0;
fn none() -> Input { Input::default() }
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, sec: f64) { for _ in 0..(sec / DT) as usize { s.tick(DT, none(), Actions::default()); } }
fn look_at(s: &mut Session, q: [f64; 3]) { let e = s.player.eye(); let d = [q[0] - e[0], q[1] - e[1], q[2] - e[2]]; s.player.yaw = d[0].atan2(d[2]); s.player.pitch = (d[1] / d[0].hypot(d[2])).atan(); s.tick(0.0, none(), Actions::default()); }
/// Un chantier : une dalle de pierre plate de 9 × 13, dégagée sur six blocs de haut ; le joueur au milieu, regard vers +z.
fn site() -> (Session, [i32; 3]) {
    let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, 120.0, 0.5]); ready(&mut s); s.third = false;
    let g = alw_core::gen::WorldGen::new(20261104); let y = g.column(0.5, 0.5).h.floor() as i32 + 3; let mut e = Vec::new();
    for x in -4..=4 { for z in -6..=6 { e.push((x, y - 1, z, B::STONE)); for h in 0..7 { e.push((x, y + h, z, 0)); } } } s.world.edit_many(&e);
    s.player.p = [0.5, y as f64, -3.5]; s.player.yaw = 0.0; s.player.pitch = 0.0; idle(&mut s, 0.5); assert!(s.player.ground); (s, [0, y, 0])
}


fn click(s: &mut Session) { s.tick(DT, none(), Actions { mine: false, place: true, guard: false }); s.tick(DT, none(), Actions::default()); }

#[test]
fn deux_coffres_cote_a_cote_font_un_grand_coffre() {
    let (mut s, [x, y, z]) = site(); s.player.p[2] = z as f64 - 1.0; s.world.edit_many(&[(x, y, z + 1, B::CHEST), (x + 1, y, z + 1, B::CHEST)]);
    s.chests.insert([x, y, z + 1], vec![None; CHEST_SIZE]); s.chests.insert([x + 1, y, z + 1], vec![None; CHEST_SIZE]);
    look_at(&mut s, [x as f64 + 0.5, y as f64 + 0.4, z as f64 + 1.5]); click(&mut s);
    assert!(s.chest.is_some() && s.chest2.is_some(), "les deux coffres s'ouvrent ensemble"); assert_eq!(s.chest_cells(), 2 * CHEST_SIZE);
    assert_eq!(s.world.get(x, y, z + 1), Some(B::CHEST_OPEN), "le couvercle se lève"); assert_eq!(s.world.get(x + 1, y, z + 1), Some(B::CHEST_OPEN));
    // La seconde moitié de l'écran est le coffre voisin.
    s.inv[0] = Some(Stack::new("pebble", Some(5))); s.grab_slot(0); s.drop_slot(Some(CHEST0 + CHEST_SIZE + 2));
    assert!(s.chests[&[x + 1, y, z + 1]][2].is_some_and(|st| st.item == "pebble"), "rangé dans le coffre de droite");
    s.inv_open = false; s.tick(DT, none(), Actions::default());
    assert_eq!(s.world.get(x, y, z + 1), Some(B::CHEST), "le couvercle retombe"); assert!(s.chest.is_none());
}

#[test]
fn chaque_atelier_a_son_interface_et_son_geste() {
    let (mut s, [x, y, z]) = site(); s.player.p[2] = z as f64 - 0.5; s.world.edit(x, y, z + 2, B::ANVIL); s.world.edit(x + 2, y, z + 2, B::FURNACE);
    look_at(&mut s, [x as f64 + 0.5, y as f64 + 0.5, z as f64 + 2.5]); click(&mut s);
    assert_eq!(s.at_station.map(|a| a.0), Some(Station::Anvil), "l'enclume ouvre son interface"); assert!(s.inv_open);
    let list = alw_core::play::book(&s, None, ""); assert!(list.iter().all(|i| recipes()[*i].at == Station::Anvil), "seulement les ouvrages de l'enclume");
    // Un ouvrage au four : il prend son temps, le four s'embrase, l'objet arrive au bout.
    let r = recipes().iter().position(|r| r.at == Station::Furnace && r.artisan.is_none()).unwrap(); let rc = &recipes()[r];
    for n in &rc.need { let id = alw_core::survival::members(n.id)[0]; s.add_item(id, n.n * 2); }
    let before = s.count(rc.out); assert_eq!(s.order_craft(r, false), Ok(0)); assert!(s.job.is_some());
    for _ in 0..30 { s.tick(DT, none(), Actions::default()); } assert_eq!(s.count(rc.out), before, "pas encore"); assert_eq!(s.world.get(x + 2, y, z + 2), Some(B::FURNACE_LIT), "le four s'embrase");
    for _ in 0..200 { s.tick(DT, none(), Actions::default()); } assert_eq!(s.count(rc.out), before + rc.n, "l'ouvrage est fait"); assert!(s.job.is_none());
    for _ in 0..(10.0 / DT) as usize { s.tick(DT, none(), Actions::default()); } assert_eq!(s.world.get(x + 2, y, z + 2), Some(B::FURNACE), "puis il refroidit");
    // Refermer en cours de route interrompt l'ouvrage.
    s.order_craft(r, false).unwrap(); s.inv_open = false; s.tick(DT, none(), Actions::default()); assert!(s.job.is_none() && s.at_station.is_none());
}
