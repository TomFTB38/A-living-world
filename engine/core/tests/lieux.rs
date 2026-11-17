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
        let cache: std::cell::RefCell<std::collections::HashMap<(i32, i32), Vec<u16>>> = Default::default();
        let at = |x: i32, y: i32, z: i32| { let mut c = cache.borrow_mut(); let v = c.entry((x.div_euclid(32), z.div_euclid(32))).or_insert_with(|| w.gen_chunk(x.div_euclid(32), z.div_euclid(32))); v[idx(x.rem_euclid(32) as usize, y as usize, z.rem_euclid(32) as usize)] };
        let c = pl.chest(); assert!(is_chest_shut(at(c[0], c[1], c[2])), "{:?} en {} {} {} : le coffre est à sa place", pl.kind, pl.x, pl.y, pl.z); assert!(is_solid(at(c[0], c[1] - 1, c[2])), "coffre en l'air");
        assert_eq!(w.place_chest(c), Some(pl.kind)); assert_eq!(w.place_chest([c[0], c[1] + 1, c[2]]), None);
        let count = |f: &dyn Fn(&str) -> bool, r: i32, h: i32| { let mut n = 0; for x in pl.x - r..=pl.x + r { for z in pl.z - r..=pl.z + r { for y in pl.y..=pl.y + h { if f(name(at(x, y, z))) { n += 1; } } } } n };
        let lo = |u: i32, w: i32| { let (a, b) = spin_xz(u, w, pl.rot); (pl.x + a, pl.z + b) };
        match pl.kind {
            Lieu::Tour => {
                assert!(count(&|n| n.contains("_stairs"), 4, 13) >= 12, "les trois volées d'escalier"); assert!(count(&|n| n == "stone_bricks" || n == "cobblestone" || n == "mossy_stone", 4, 14) > 400, "les murs tiennent"); assert_eq!(count(&|n| n.contains("leaves"), 3, 14), 0, "pas d'arbre dans la tour");
                assert_eq!(count(&|n| n == "brazier", 1, 15), 1, "le brasero"); assert!(count(&|n| n == "fence", 5, 14) >= 5, "le hourd");
                // chaque marche a de quoi passer la tête ; la cave est creusée, son échelle descend, son coffre caché y est
                let y = pl.y; for i in 0..4 { for (su, sy, sw) in [(-3, y + 1 + i, -2 + i), (-2 + i, y + 5 + i, 3), (3, y + 9 + i, 2 - i)] { let (sx, sz) = lo(su, sw); assert!(name(at(sx, sy, sz)).contains("_stairs"), "marche manquante en {sx} {sy} {sz}"); for h in 1..=2 { assert!(!is_solid(at(sx, sy + h, sz)), "on se cogne au-dessus de la marche {sx} {sy} {sz}"); } } }
                for dy in 1..=4 { assert!(name({ let (lx, lz) = lo(3, 3); at(lx, y - dy, lz) }).starts_with("ladder"), "échelle de la cave"); assert_eq!({ let (lx, lz) = lo(0, 0); at(lx, y - dy, lz) }, 0, "la cave est creusée"); } assert!(is_chest_shut({ let (lx, lz) = lo(-3, 3); at(lx, y - 4, lz) }), "le coffre caché"); assert!(count(&|n| n.starts_with("iron_bars"), 4, 0) == 0 && name({ let (lx, lz) = lo(-1, -3); at(lx, y - 4, lz) }).starts_with("iron_bars"), "le cachot");
            }
            Lieu::Cimetiere => {
                assert!(count(&|n| n.starts_with("gravestone") || n.starts_with("grave_cross"), 11, 2) >= 14, "des tombes"); assert!(count(&|n| n == "gravel", 9, 0) >= 12, "les allées"); assert!(count(&|n| n.starts_with("slate_slope"), 11, 8) >= 30, "le toit de la chapelle et du porche");
                assert!(count(&|n| n == "stained_glass", 11, 8) >= 10, "les vitraux"); assert!(count(&|n| n.starts_with("bench"), 8, 2) >= 5, "les bancs");
                let y = pl.y; for dy in 1..=4 { assert!(name({ let (lx, lz) = lo(-2, 9); at(lx, y - dy, lz) }).starts_with("ladder"), "échelle de la crypte"); assert_eq!({ let (lx, lz) = lo(0, 4); at(lx, y - dy, lz) }, 0, "la crypte est creusée"); } for h in 1..=2 { assert!(!is_solid({ let (lx, lz) = lo(0, 2); at(lx, y + h, lz) }), "la porte de la chapelle est ouverte"); }
            }
            Lieu::Camp => {
                assert_eq!(count(&|n| n == "campfire", 1, 2), 1, "le feu"); assert!(count(&|n| n == "oak_log", 11, 5) > 150, "la palissade"); assert!(count(&|n| n.starts_with("thatch_slope"), 9, 4) >= 60, "les tentes"); assert_eq!(count(&|n| n == "bedroll", 9, 2), 4);
                assert!(count(&|n| n.starts_with("iron_bars"), 8, 3) >= 14, "la cage"); assert!(count(&|n| n.starts_with("ladder"), 8, 6) >= 5, "l'échelle de la tour de guet"); assert!(count(&|n| n.starts_with("throne"), 8, 2) == 1 && count(&|n| n.starts_with("cart"), 6, 2) == 1, "le siège du chef, la charrette");
                let y = pl.y; for h in 1..=2 { for u in -1..=1 { assert!(!is_solid({ let (lx, lz) = lo(u, -10); at(lx, y + h, lz) }), "le portail est ouvert"); } assert!(!is_solid({ let (lx, lz) = lo(0, 5); at(lx, y + h, lz) }), "on entre dans la tente du chef"); }
            }
        }
        // les gardiens ont les pieds sur du plein et la tête à l'air
        for (id, g) in pl.guards() { assert!(alw_core::hostile::species(id).is_some(), "{id}"); let (x, y, z) = (g[0].floor() as i32, g[1].floor() as i32, g[2].floor() as i32); assert!(is_solid(at(x, y - 1, z)), "{id} de {:?} n'a rien sous les pieds en {x} {y} {z}", pl.kind); assert!(!is_solid(at(x, y + 1, z)), "{id} de {:?} emmuré", pl.kind); }
    }
    println!("lieux : {} tours, {} cimetières, {} camps", seen[0], seen[1], seen[2]); assert!(seen.iter().all(|n| *n >= 10), "chaque sorte de lieu existe : {seen:?}"); assert!(seen[0] * 10 < (seen[0] + seen[1] + seen[2]) * 6, "les tours ne prennent pas toute la place");
    assert!(WorldGen::new(alw_core::gen::FLAT).places_near(0.0, 0.0, 800.0).is_empty(), "rien dans le monde plat");
}

#[test]
fn on_y_va_les_gardiens_sortent_le_coffre_est_garni() {
    let w = WorldGen::new(SEED); let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.hour = 12.0; let sp = |id: &str| alw_core::hostile::species(id).unwrap();
    assert!(s.command("/lieu chateau").contains("inconnu"));
    for (key, who, n, loot) in [("tour", "archer", 5, "arrow"), ("brigands", "brigand", 8, "worn_coin"), ("cimetiere", "archer", 3, "worn_coin")] {
        s.horde.foes.clear(); let msg = s.command(&format!("/lieu {key}")); ready(&mut s); let pl = w.places_near(s.player.p[0], s.player.p[2], 40.0).into_iter().next().unwrap_or_else(|| panic!("{key} : {msg}")); assert_eq!(pl.kind.key(), key, "{msg}");
        s.notes.clear(); for _ in 0..3 { s.tick(0.05, Input::default(), Actions::default()); }
        assert!(s.horde.foes.iter().filter(|f| f.sp == sp(who)).count() >= n, "{key} : {n} {who} attendus, on a {:?}", s.horde.foes.iter().map(|f| (alw_core::hostile::bestiary().species[f.sp].id, f.p.map(|v| v.round()), f.hp)).collect::<Vec<_>>()); assert!(s.notes.iter().any(|t| t.contains("en vue")), "{key} : annoncé");
        assert_eq!(s.horde.foes.iter().filter(|f| f.sp == sp("fantome")).count(), 0, "pas de fantôme en plein jour");
        let before = s.horde.foes.len(); for _ in 0..40 { s.tick(0.05, Input::default(), Actions::default()); } assert!(s.horde.foes.len() <= before + 2, "{key} : on ne relâche pas des gardiens à chaque image");
        s.open_chest(pl.chest()); let got = &s.chests[&pl.chest()]; assert!(got.iter().flatten().any(|st| st.item == loot), "{key} : {loot} dans le coffre, on a {:?}", got.iter().flatten().map(|st| st.item).collect::<Vec<_>>()); s.close_chest(); s.inv_open = false;
        s.vit = [100.0; 3]; assert!(s.dead.is_none(), "{key} : mort en arrivant");
    }
    // la nuit, au cimetière, les fantômes se lèvent
    s.hour = 0.5; for _ in 0..3 { s.tick(0.05, Input::default(), Actions::default()); } assert_eq!(s.horde.foes.iter().filter(|f| f.sp == sp("fantome")).count(), 5, "cinq fantômes la nuit");
}
