//! L'atelier de taille : on y pose un bloc, on en tire ses formes et ses déclinaisons.
use alw_core::{blocks::*, play::{cut_variants, Cut}, player::Input, session::*, stream::Preset};
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }

#[test]
fn chaque_matiere_a_ses_tailles() {
    let has = |src: &str, out: &str| cut_variants(src).iter().any(|c| c.out == out);
    for (src, outs) in [("stone", vec!["stone_bricks", "cobblestone", "chiseled_stone"]), ("stone_bricks", vec!["stone_bricks_slab", "stone_bricks_stairs", "stone_pillar", "stone_wall", "stone_bricks_vslab"]),
        ("planks", vec!["planks_slab", "planks_stairs", "planks_slope", "planks_corner", "planks_valley", "planks_ridge", "planks_peak", "parquet"]), ("oak_log", vec!["planks", "oak_post", "oak_post_diag", "oak_post_angle"]),
        ("roof_tiles", vec!["roof_tiles_slope", "roof_tiles_corner", "roof_tiles_valley", "roof_tiles_ridge", "roof_tiles_peak", "roof_tiles_slab"]), ("spruce_log", vec!["spruce_post"])] {
        for o in outs { assert!(has(src, o), "{src} ne donne pas {o} : {:?}", cut_variants(src)); } }
    // ni meubles, ni portes : seulement des ouvrages de construction ; rien pour ce qui n'est pas un bloc ; jamais le bloc lui-même
    for bad in ["bed", "door", "workbench", "round_table", "chair", "ladder"] { assert!(!has("planks", bad), "les planches ne se taillent pas en {bad}"); }
    assert!(cut_variants("stick").is_empty() && cut_variants("iron_ingot").is_empty() && cut_variants("n_existe_pas").is_empty());
    for b in BLOCKS.iter() { for c in cut_variants(b.name) { assert!(c.out != b.name && c.n >= 1 && c.need >= 1 && by_name(c.out).is_some() && item(c.out).is_some(), "{} → {c:?}", b.name); } }
    assert!(cut_variants("oak_log").contains(&Cut { out: "oak_post", n: 3, need: 1 }), "un rondin donne trois poutres");
}

#[test]
fn on_pose_un_bloc_et_on_le_taille() {
    let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); for i in 0..INV_SIZE { s.inv[i] = None; }
    s.inv[9] = Some(Stack::new("stone_bricks", Some(20))); s.inv[10] = Some(Stack::new("stick", Some(5)));
    // sans atelier : la case refuse, et tailler échoue
    assert!(!s.accepts(CUT0, s.inv[9].unwrap())); assert!(s.cut("stone_bricks_slab", false).is_err());
    s.at_station = Some((alw_core::data::Station::Mason, [0, 0, 0])); s.inv_open = true; assert!(s.cutting());
    assert!(s.accepts(CUT0, s.inv[9].unwrap()) && !s.accepts(CUT0, s.inv[10].unwrap()), "la pierre se pose, pas le bâton");
    s.quick_slot(9); assert_eq!(s.cut_in.map(|c| (c.item, c.n)), Some(("stone_bricks", Some(20)))); assert!(s.inv[9].is_none(), "Maj+clic pose le bloc dans l'atelier");
    s.quick_slot(10); assert_eq!(s.count("stick"), 5); assert_eq!(s.cut_in.map(|c| c.item), Some("stone_bricks"), "le bâton ne va pas dans l'atelier");
    // une taille : trois briques font six dalles
    assert_eq!(s.cut("stone_bricks_slab", false), Ok(6)); assert_eq!(s.count("stone_bricks_slab"), 6); assert_eq!(s.cut_in.and_then(|c| c.n), Some(17));
    assert!(s.cut("planks_slab", false).is_err(), "une taille qui n'est pas de cette matière");
    // tout le tas : dix-sept briques, six par quatre escaliers → deux fois, il en reste cinq
    assert_eq!(s.cut("stone_bricks_stairs", true), Ok(8)); assert_eq!(s.cut_in.and_then(|c| c.n), Some(5)); assert_eq!(s.count("stone_bricks_stairs"), 8);
    assert!(s.cut("stone_bricks_stairs", false).is_err(), "pas assez pour une taille de plus");
    // dans les recettes de l'atelier, on ne taille plus ; en le quittant, le reste revient au sac
    s.cut_mode = false; assert!(!s.cutting() && s.cut("stone_bricks_slab", false).is_err()); s.cut_mode = true;
    s.inv_open = false; s.tick(1.0 / 30.0, Input::default(), Actions::default()); assert!(s.cut_in.is_none() && s.at_station.is_none()); assert_eq!(s.count("stone_bricks"), 5, "le reste revient au sac");
}
