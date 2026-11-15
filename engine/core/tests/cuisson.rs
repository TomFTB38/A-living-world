//! Four et feu de camp : une case pour le matériau, une pour le combustible ; la cuisson avance toute seule.
use alw_core::{blocks::*, data::Station, play::{burns, cook_variants}, player::Input, session::*, stream::Preset};
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn run(s: &mut Session, secs: f64) { let n = (secs * 20.0) as usize; for _ in 0..n { s.tick(0.05, Input::default(), Actions::default()); } }

#[test]
fn ce_qui_cuit_et_ce_qui_brule() {
    let has = |at, src: &str, out: &str| cook_variants(at, src).iter().any(|c| c.out == out);
    assert!(has(Station::Furnace, "iron_ore", "iron_ingot") && has(Station::Furnace, "sand", "glass") && has(Station::Furnace, "clay", "roof_tiles") && has(Station::Fire, "raw_meat", "cooked_meat") && has(Station::Fire, "oak_log", "charcoal"));
    assert!(!has(Station::Fire, "iron_ore", "iron_ingot") && cook_variants(Station::Bench, "raw_meat").is_empty() && cook_variants(Station::Furnace, "copper_ingot").is_empty(), "le bronze, à deux métaux, reste au livre");
    let c = cook_variants(Station::Furnace, "iron_ore").into_iter().find(|c| c.out == "iron_ingot").unwrap(); assert_eq!((c.n, c.need, c.fuel), (1, 2, 2));
    assert!(burns(Station::Furnace, "coal") && burns(Station::Furnace, "charcoal") && !burns(Station::Furnace, "oak_log") && burns(Station::Fire, "oak_log") && !burns(Station::Fire, "stone"));
}

#[test]
fn le_four_fond_tout_seul() {
    let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); for i in 0..INV_SIZE { s.inv[i] = None; }
    s.inv[9] = Some(Stack::new("iron_ore", Some(5))); s.inv[10] = Some(Stack::new("coal", Some(3))); s.inv[11] = Some(Stack::new("stick", Some(4)));
    assert!(!s.accepts(COOK0, s.inv[9].unwrap()), "sans four, la case refuse");
    s.at_station = Some((Station::Furnace, [0, 0, 0])); s.inv_open = true; assert_eq!(s.cooking(), Some(Station::Furnace));
    assert!(s.accepts(COOK0, s.inv[9].unwrap()) && !s.accepts(COOK0, s.inv[11].unwrap()) && s.accepts(FUEL0, s.inv[10].unwrap()) && !s.accepts(FUEL0, s.inv[9].unwrap()));
    s.quick_slot(9); s.quick_slot(10); s.quick_slot(11); assert_eq!(s.cook_in.map(|c| (c.item, c.n)), Some(("iron_ore", Some(5)))); assert_eq!(s.cook_fuel.map(|c| (c.item, c.n)), Some(("coal", Some(3)))); assert_eq!(s.count("stick"), 4);
    assert_eq!(s.cook_block(), None); run(&mut s, 1.0); assert!(s.cook_t > 0.5 && s.count("iron_ingot") == 0, "ça chauffe, rien n'est encore sorti");
    run(&mut s, 3.0); assert_eq!(s.count("iron_ingot"), 1); assert_eq!(s.cook_in.and_then(|c| c.n), Some(3)); assert_eq!(s.cook_fuel.and_then(|c| c.n), Some(1));
    // plus assez de charbon : la fonte s'arrête et le dit
    run(&mut s, 8.0); assert_eq!(s.count("iron_ingot"), 1); assert_eq!(s.cook_block(), Some("Il manque du combustible"));
    // on ferme : tout revient au sac
    s.inv_open = false; run(&mut s, 0.1); assert!(s.cook_in.is_none() && s.cook_fuel.is_none()); assert_eq!((s.count("iron_ore"), s.count("coal")), (3, 1));
}

#[test]
fn le_feu_de_camp_cuit_et_se_nourrit_de_sa_case() {
    let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); for i in 0..INV_SIZE { s.inv[i] = None; }
    let p = s.player.p; let c = [p[0].floor() as i32 + 1, p[1].floor() as i32, p[2].floor() as i32]; s.world.edit(c[0], c[1], c[2], B::CAMPFIRE_OUT);
    s.inv[9] = Some(Stack::new("raw_meat", Some(2))); s.inv[10] = Some(Stack::new("oak_log", Some(4)));
    s.at_station = Some((Station::Fire, c)); s.inv_open = true; s.quick_slot(9); s.quick_slot(10);
    assert_eq!(s.cook_in.map(|c| c.item), Some("raw_meat")); assert_eq!(s.cook_fuel.map(|c| c.item), Some("oak_log"));
    assert_eq!(s.cook_block(), Some("Le feu est éteint")); run(&mut s, 4.0); assert_eq!(s.count("cooked_meat"), 0);
    s.world.edit(c[0], c[1], c[2], B::CAMPFIRE); s.fires.insert(c, 5.0); run(&mut s, 3.0);
    assert_eq!(s.count("cooked_meat"), 1); assert_eq!(s.cook_fuel.and_then(|c| c.n), Some(3), "une bûche est partie au foyer"); assert!(s.fires[&c] > 30.0);
    // on choisit la viande séchée : il en faut trois, il n'en reste qu'une
    s.cook_sel = Some("jerky"); assert_eq!(s.cook_now().map(|c| c.out), Some("jerky")); assert_eq!(s.cook_block(), Some("Pas assez de matériau"));
}
