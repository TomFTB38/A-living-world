//! Premiers pas : chaque conseil s'efface quand on l'a suivi, les rappels passent seuls, tout se sauvegarde.
use alw_core::{player::*, session::*, stream::*};
const DT: f64 = 1.0 / 30.0;
fn step(s: &mut Session) -> usize { s.tick(DT, Input::default(), Actions::default()); s.hint().map_or(99, |h| h.0) }
#[test]
fn le_guide_suit_le_joueur() {
    let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 120.0, 0.5]); s.inv = [None; INV_SIZE]; s.seen.clear(); s.guide = 0;
    assert_eq!(step(&mut s), 1); assert!(s.hint().unwrap().2.contains("branches"));
    s.add_item("stick", 1); assert_eq!(step(&mut s), 1, "il faut aussi un caillou"); s.add_item("pebble", 1); assert_eq!(step(&mut s), 2);
    s.add_item("fiber", 1); assert_eq!(step(&mut s), 3); s.add_item("stone_knife", 1); assert_eq!(step(&mut s), 4);
    s.add_item("oak_log", 1); assert_eq!(step(&mut s), 5); s.add_item("workbench", 1); assert_eq!(step(&mut s), 6); s.add_item("campfire", 1); assert_eq!(step(&mut s), 7);
    for _ in 0..(46.0 / DT) as usize { s.tick(DT, Input::default(), Actions::default()); } assert_eq!(s.hint().map(|h| h.0), Some(8), "le rappel passe seul");
    assert!(s.hint().unwrap().2.contains("village"), "{}", s.hint().unwrap().2);
    let t = s.save_text(); let mut b = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 120.0, 0.5]); b.load_text(&t).unwrap(); assert_eq!(b.guide, s.guide);
    s.set_creative(true); assert!(s.hint().is_none(), "pas de conseils en créatif"); s.set_creative(false);
    s.skip_guide(); assert!(s.hint().is_none());
}
