//! Le mobilier dont on se sert : s'asseoir (et se lever devant le siège), ranger dans une armoire, poser des objets sur une
//! étagère ou une table (ils restent, se dessinent, se sauvent, reviennent quand on casse le meuble).
use alw_core::{blocks::*, mobilier as mb, player::*, session::*, stream::*};
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn session() -> Session { let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [2000.5, y as f64 + 2.0, 1985.5]); ready(&mut s); s }
fn world() -> (Session, [i32; 3]) { let mut s = session(); s.set_creative(false); s.hour = 12.0; let y = alw_core::gen::FLAT_Y; run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); s.notes.clear(); (s, [2000, y + 1, 1990]) }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.tick(DT, Input::default(), Actions::default()); } }
fn put(s: &mut Session, p: [i32; 3], n: &str) -> Id { let b = by_name(n).unwrap(); s.world.edit(p[0], p[1], p[2], b); b }
fn hand(s: &mut Session, item: &'static str, n: u32) { let sel = s.sel; s.inv[sel] = Some(Stack { item, n: Some(n), wear: 0 }); }
fn count(s: &Session, item: &str) -> u32 { s.inv.iter().flatten().filter(|st| st.item == item).map(|st| st.n.unwrap_or(1)).sum() }

#[test]
fn s_asseoir_et_se_lever() {
    let (mut s, p) = world();
    for (n, f) in [("chair@2", 2usize), ("bench", 0), ("stool", 0), ("armchair@1", 1)] {
        let id = put(&mut s, p, n); s.player.p = [p[0] as f64 + 0.5, p[1] as f64, p[2] as f64 - 1.5]; s.stamina = 10.0;
        assert!(s.furniture_at(p, id), "{n} : on s'assied"); assert_eq!(s.seated, Some(p));
        run(&mut s, 1.0); let q = s.player.p; assert!((q[0] - (p[0] as f64 + 0.5)).abs() < 1e-6 && (q[2] - (p[2] as f64 + 0.5)).abs() < 1e-6, "{n} : on reste assis ({q:?})");
        assert!(q[1] > p[1] as f64 - 0.1 && q[1] < p[1] as f64 + 0.3, "{n} : sur l'assise ({:.2})", q[1] - p[1] as f64); assert!(s.stamina > 35.0, "{n} : on reprend son souffle ({:.0})", s.stamina);
        // un pas : on se lève, devant le siège
        s.tick(DT, Input { z: 1.0, ..Default::default() }, Actions::default()); assert_eq!(s.seated, None, "{n} : on se lève");
        let (fx, fz) = mb::front(id); let c = [s.player.p[0].floor() as i32, s.player.p[1].floor() as i32, s.player.p[2].floor() as i32]; assert_eq!(c, [p[0] + fx, p[1], p[2] + fz], "{n} : devant le siège (sens {f})");
        run(&mut s, 0.3); s.world.edit(p[0], p[1], p[2], 0);
    }
}

#[test]
fn ranger_dans_une_armoire() {
    let (mut s, p) = world(); let id = put(&mut s, p, "cupboard@3"); s.player.p = [p[0] as f64 - 1.5, p[1] as f64, p[2] as f64 + 0.5];
    assert!(s.furniture_at(p, id), "l'armoire s'ouvre"); assert_eq!(s.chest, Some(p)); assert!(s.inv_open);
    s.chests.get_mut(&p).unwrap()[0] = Some(Stack { item: "berries", n: Some(7), wear: 0 }); s.close_chest();
    let t = s.save_text(); let mut b = session(); b.load_text(&t).unwrap(); assert_eq!(b.chests[&p][0].map(|st| (st.item, st.n)), Some(("berries", Some(7))), "le contenu est sauvé");
    for (n, q) in [("barrel", [p[0] + 2, p[1], p[2]]), ("crate", [p[0] + 3, p[1], p[2]])] { let id = put(&mut s, q, n); assert!(s.furniture_at(q, id), "{n} s'ouvre"); assert_eq!(s.chest, Some(q)); s.close_chest(); }
    // casser l'armoire rend ce qu'elle contenait
    let before = count(&s, "berries"); s.world.edit(p[0], p[1], p[2], 0); s.on_broken(p, id); assert_eq!(count(&s, "berries"), before + 7, "les baies reviennent"); assert!(!s.chests.contains_key(&p));
}

#[test]
fn poser_sur_une_etagere_et_une_table() {
    let (mut s, p) = world(); let id = put(&mut s, p, "shelf@1"); s.player.p = [p[0] as f64 - 1.5, p[1] as f64, p[2] as f64 + 0.5];
    hand(&mut s, "berries", 5); for _ in 0..3 { assert!(s.furniture_at(p, id)); } assert_eq!(count(&s, "berries"), 2, "trois baies posées");
    hand(&mut s, "iron_hammer", 1); s.inv[s.sel].as_mut().unwrap().n = None; assert!(s.furniture_at(p, id)); assert!(s.inv[s.sel].is_none(), "la pioche posée");
    let row = s.shelves[&p].clone(); assert_eq!(row.iter().flatten().count(), 4); assert_eq!(row.len(), 6, "six places");
    // chaque place est dans le bloc, du bon côté, quel que soit le sens
    for (k, sl) in mb::slots(id).iter().enumerate() { let q = mb::place(p, id, *sl); for a in 0..3 { assert!(q[a] >= p[a] as f64 && q[a] <= p[a] as f64 + 1.0, "place {k} dans le bloc"); } }
    // pleine : la septième n'a pas de place ; la main vide reprend le dernier posé
    hand(&mut s, "bread", 9); for _ in 0..2 { assert!(s.furniture_at(p, id)); } assert_eq!(count(&s, "bread"), 7); s.notes.clear(); assert!(s.furniture_at(p, id)); assert!(s.notes.iter().any(|t| t.contains("plus de place")));
    let sel = s.sel; s.inv[sel] = None; assert!(s.furniture_at(p, id)); assert_eq!(count(&s, "bread"), 1, "la main vide reprend le dernier posé");
    // la sauvegarde garde ce qui est posé ; les objets se dessinent
    let t = s.save_text(); let mut b = session(); b.load_text(&t).unwrap(); assert_eq!(b.shelves[&p].iter().flatten().map(|st| st.item).collect::<Vec<_>>(), s.shelves[&p].iter().flatten().map(|st| st.item).collect::<Vec<_>>());
    let md = alw_core::actors::Models::build(); assert_eq!(s.shelf_draws(&md).len(), 5, "cinq objets dessinés sur l'étagère");
    // une table : quatre places ; casser la table rend tout
    let q = [p[0] + 3, p[1], p[2]]; let tid = put(&mut s, q, "table"); hand(&mut s, "berries", 2); for _ in 0..2 { assert!(s.furniture_at(q, tid)); } assert_eq!(s.shelves[&q].iter().flatten().count(), 2);
    let before = count(&s, "berries"); s.world.edit(q[0], q[1], q[2], 0); s.on_broken(q, tid); assert_eq!(count(&s, "berries"), before + 2, "la table cassée rend ses baies"); assert!(!s.shelves.contains_key(&q));
}
