//! Les lieux de surface : tour de guet en ruine, cimetière abandonné, camp de brigands.
use alw_core::{blocks::*, gen::WorldGen, player::Input, session::*, sites::*, stream::Preset};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..8000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }

#[test]
fn chaque_lieu_existe_est_bati_et_garde_son_coffre() {
    let w = WorldGen::new(SEED); let all = w.places_near(-38.0, 0.0, 2500.0); let mut seen = [0usize; 3];
    let name = |b: u16| BLOCKS[b as usize].name;
    for pl in &all {
        let k = LIEUX.iter().position(|l| *l == pl.kind).unwrap(); seen[k] += 1; if seen[k] > 3 { continue; } // trois de chaque sorte, bloc à bloc
        assert!(((pl.x + 38) as f64).hypot(pl.z as f64) >= 90.0, "trop près du départ"); assert!(w.villages_near(pl.x as f64, pl.z as f64, 100.0).is_empty(), "{:?} dans un village", pl.kind);
        let mut cache: std::collections::HashMap<(i32, i32), Vec<u16>> = Default::default();
        let mut at = |x: i32, y: i32, z: i32| { let v = cache.entry((x.div_euclid(32), z.div_euclid(32))).or_insert_with(|| w.gen_chunk(x.div_euclid(32), z.div_euclid(32))); v[idx(x.rem_euclid(32) as usize, y as usize, z.rem_euclid(32) as usize)] };
        let c = pl.chest(); assert_eq!(at(c[0], c[1], c[2]), B::CHEST, "{:?} en {} {} {} : le coffre est à sa place", pl.kind, pl.x, pl.y, pl.z); assert!(is_solid(at(c[0], c[1] - 1, c[2])), "coffre en l'air");
        assert_eq!(w.place_chest(c), Some(pl.kind)); assert_eq!(w.place_chest([c[0], c[1] + 1, c[2]]), None);
        let mut count = |f: &dyn Fn(&str) -> bool, r: i32, h: i32| { let mut n = 0; for x in pl.x - r..=pl.x + r { for z in pl.z - r..=pl.z + r { for y in pl.y..=pl.y + h { if f(name(at(x, y, z))) { n += 1; } } } } n };
        match pl.kind {
            Lieu::Tour => { assert!(count(&|n| n.starts_with("ladder"), 3, 14) >= 12, "l'échelle monte jusqu'en haut"); assert!(count(&|n| n == "stone_bricks" || n == "cobblestone" || n == "mossy_stone", 3, 14) > 150, "les murs tiennent"); assert_eq!(count(&|n| n.contains("leaves") || n.contains("log"), 2, 14), 0, "pas d'arbre dans la tour"); }
            Lieu::Cimetiere => { assert!(count(&|n| n.starts_with("gravestone") || n.starts_with("grave_cross"), 9, 2) >= 8, "des tombes"); assert!(count(&|n| n == "gravel", 2, 0) >= 6, "l'allée"); }
            Lieu::Camp => { assert_eq!(count(&|n| n == "campfire", 1, 2), 1, "le feu"); assert!(count(&|n| n == "oak_log", 9, 4) > 60, "la palissade"); assert!(count(&|n| n.starts_with("thatch_slope"), 7, 3) >= 30, "les tentes"); assert_eq!(count(&|n| n == "bedroll", 7, 2), 4); }
        }
        // les gardiens ont les pieds sur du plein et la tête à l'air
        for (id, g) in pl.guards() { assert!(alw_core::hostile::species(id).is_some(), "{id}"); let (x, y, z) = (g[0].floor() as i32, g[1].floor() as i32, g[2].floor() as i32); if pl.kind == Lieu::Tour && g[1] > pl.y as f64 + 5.0 { assert!(is_solid(at(x, y - 1, z)), "l'archer du haut n'a pas de plancher"); } assert!(!is_solid(at(x, y + 1, z)), "{id} de {:?} emmuré", pl.kind); }
    }
    println!("lieux : {} tours, {} cimetières, {} camps", seen[0], seen[1], seen[2]); assert!(seen.iter().all(|n| *n >= 10), "chaque sorte de lieu existe : {seen:?}"); assert!(seen[0] < seen[1] + seen[2], "les tours ne prennent pas toute la place");
    assert!(WorldGen::new(alw_core::gen::FLAT).places_near(0.0, 0.0, 800.0).is_empty(), "rien dans le monde plat");
}

#[test]
fn on_y_va_les_gardiens_sortent_le_coffre_est_garni() {
    let w = WorldGen::new(SEED); let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.hour = 12.0; let sp = |id: &str| alw_core::hostile::species(id).unwrap();
    assert!(s.command("/lieu chateau").contains("inconnu"));
    for (key, who, n, loot) in [("tour", "archer", 2, "arrow"), ("brigands", "brigand", 3, "worn_coin"), ("cimetiere", "archer", 1, "worn_coin")] {
        s.horde.foes.clear(); let msg = s.command(&format!("/lieu {key}")); ready(&mut s); let pl = w.places_near(s.player.p[0], s.player.p[2], 40.0).into_iter().next().unwrap_or_else(|| panic!("{key} : {msg}")); assert_eq!(pl.kind.key(), key, "{msg}");
        s.notes.clear(); for _ in 0..3 { s.tick(0.05, Input::default(), Actions::default()); }
        assert!(s.horde.foes.iter().filter(|f| f.sp == sp(who)).count() >= n, "{key} : {n} {who} attendus"); assert!(s.notes.iter().any(|t| t.contains("en vue")), "{key} : annoncé");
        assert_eq!(s.horde.foes.iter().filter(|f| f.sp == sp("fantome")).count(), 0, "pas de fantôme en plein jour");
        let before = s.horde.foes.len(); for _ in 0..40 { s.tick(0.05, Input::default(), Actions::default()); } assert!(s.horde.foes.len() <= before + 2, "{key} : on ne relâche pas des gardiens à chaque image");
        s.open_chest(pl.chest()); let got = &s.chests[&pl.chest()]; assert!(got.iter().flatten().any(|st| st.item == loot), "{key} : {loot} dans le coffre, on a {:?}", got.iter().flatten().map(|st| st.item).collect::<Vec<_>>()); s.close_chest(); s.inv_open = false;
        s.vit = [100.0; 3]; assert!(s.dead.is_none(), "{key} : mort en arrivant");
    }
    // la nuit, au cimetière, les fantômes se lèvent
    s.hour = 0.5; for _ in 0..3 { s.tick(0.05, Input::default(), Actions::default()); } assert_eq!(s.horde.foes.iter().filter(|f| f.sp == sp("fantome")).count(), 2, "deux fantômes la nuit");
}
