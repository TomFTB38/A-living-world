//! Les douze biomes souterrains : chacun existe et se reconnaît à ses blocs ; chaque salle se génère à l'identique dans
//! n'importe quel ordre ; les mines et les bouches mènent de la surface à la salle par une pente praticable ; la lave brûle.
use alw_core::{blocks::*, gen::WorldGen, karst::*, player::{Input, Voxels}, session::*, stream::Preset};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn around(w: &WorldGen, h: &Hall) -> Vec<Id> { let (cx, cz) = ((h.x as i32).div_euclid(32), (h.z as i32).div_euclid(32)); let mut v = Vec::new(); for dz in -1..=1 { for dx in -1..=1 { v.extend(w.gen_chunk(cx + dx, cz + dz)); } } v }

#[test]
fn chaque_biome_existe_et_a_ses_blocs() {
    let w = WorldGen::new(SEED);
    let sign: [(Under, &[&str]); 12] = [(Under::Ordinary, &["stone_point"]), (Under::Karst, &["flowstone", "dripstone"]), (Under::Verdant, &["vine", "fern", "mossy_stone"]), (Under::Fungal, &["mushroom_stem", "mushroom_cap", "mycelium"]),
        (Under::Glow, &["glow_vine", "glow_moss"]), (Under::Crystal, &["crystal_"]), (Under::Ice, &["ice_blue", "ice_point"]), (Under::Volcanic, &["basalt", "lava"]), (Under::Mineral, &["_ore"]),
        (Under::Desert, &["sandstone"]), (Under::Abyss, &["abyss_rock"]), (Under::Coral, &["coral"])];
    for (k, names) in sign {
        let h = w.nearest_hall_of(0.0, 0.0, 14, Some(k)).unwrap_or_else(|| panic!("aucune {}", k.name())); let v = around(&w, &h);
        for n in names { let c = v.iter().filter(|b| BLOCKS[**b as usize].name.contains(n)).count(); assert!(c > 0, "{} : pas de « {n} »", k.name()); }
        println!("{} à {:.0} {:.0} : sol {} voûte {:.0}", k.name(), h.x, h.z, h.floor, h.h);
    }
}

#[test]
fn les_salles_ne_dependent_pas_de_l_ordre() {
    let w = WorldGen::new(SEED);
    for k in [Under::Volcanic, Under::Fungal, Under::Crystal, Under::Abyss] {
        let h = w.nearest_hall_of(0.0, 0.0, 14, Some(k)).unwrap(); let (cx, cz) = ((h.x as i32).div_euclid(32), (h.z as i32).div_euclid(32));
        let a: Vec<Vec<Id>> = (0..9).map(|i| w.gen_chunk(cx + i % 3 - 1, cz + i / 3 - 1)).collect();
        let w2 = WorldGen::new(SEED); let mut b: Vec<Vec<Id>> = (0..9).rev().map(|i| w2.gen_chunk(cx + i % 3 - 1, cz + i / 3 - 1)).collect(); b.reverse();
        assert!(a == b, "{} : même résultat quel que soit l'ordre", k.name());
    }
}

#[test]
fn les_mines_et_les_bouches_menent_a_la_salle() {
    let w = WorldGen::new(SEED); let (mut mines, mut mouths) = (0, 0);
    for i in -10..10 { for j in -10..10 { let Some(h) = w.hall(i, j) else { continue }; let Some(Way::Path { from, to, mine }) = h.way else { continue };
        if mine { mines += 1; } else { mouths += 1; } if mines > 3 && mouths > 3 { break; }
        let mut chunks = std::collections::HashMap::new(); let mut at = |x: i32, y: i32, z: i32| -> Id { let k = (x.div_euclid(32), z.div_euclid(32)); let v = chunks.entry(k).or_insert_with(|| w.gen_chunk(k.0, k.1)); v[idx(x.rem_euclid(32) as usize, y as usize, z.rem_euclid(32) as usize)] };
        // le long de l'axe : deux cases libres pour passer, un appui sous les pieds, des marches d'un bloc au plus
        let len = (to[0] - from[0]).hypot(to[2] - from[2]); let steps = (len * 2.0) as i32; let mut last: Option<i32> = None; let mut blocked = 0;
        let (ux, uz) = ((to[0] - from[0]) / len, (to[2] - from[2]) / len);
        for k in 4..steps - 2 { let t = k as f64 / steps as f64;
            let fy = (from[1] - 1.0 + (to[1] - from[1] + 1.0) * t).round() as i32;
            // une bouche serpente : on cherche le passage en travers, sur la largeur où elle peut aller
            let lats: Vec<f64> = if mine { vec![0.0] } else { (-12..=12).map(|l| l as f64 * 0.5).collect() };
            let ok = lats.iter().any(|l| { let (x, z) = ((from[0] + (to[0] - from[0]) * t - uz * l).floor() as i32, (from[2] + (to[2] - from[2]) * t + ux * l).floor() as i32);
                (fy - 1..=fy + 1).any(|y| { let (a, b) = (at(x, y, z), at(x, y + 1, z)); !is_solid(a) && a != B::LAVA && !is_solid(b) && b != B::LAVA }) });
            if !ok { blocked += 1; }
            if let Some(l) = last { assert!((fy - l).abs() <= 1, "une marche trop haute"); } last = Some(fy); }
        assert!(blocked as f64 <= steps as f64 * 0.05, "{} : le couloir est dégagé ({blocked} cases bouchées sur {steps})", if mine { "mine" } else { "bouche" });
        if mine { let n = (0..steps).filter(|k| { let t = *k as f64 / steps as f64; let (x, z) = ((from[0] + (to[0] - from[0]) * t).floor() as i32, (from[2] + (to[2] - from[2]) * t).floor() as i32);
            (-2..6).any(|dy| { let b = at(x, (from[1] + (to[1] - from[1]) * t) as i32 + dy, z); b == B::RAIL_X || b == B::RAIL_Z || b == B::OAK_POST_X || b == B::OAK_POST_Z }) }).count(); assert!(n > 5, "la mine a ses rails et son boisage"); }
    } }
    assert!(mines > 0 && mouths > 0, "des mines ({mines}) et des bouches ({mouths})");
}

#[test]
fn la_lave_brule() {
    let w = WorldGen::new(SEED); let h = w.nearest_hall_of(0.0, 0.0, 14, Some(Under::Volcanic)).unwrap(); let spot = w.hall_spot(&h, [h.x, h.z]).expect("un endroit sûr");
    let mut s = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 1, spot); ready(&mut s); s.player.p = spot;
    let (x, y, z) = (spot[0].floor() as i32, spot[1] as i32, spot[2].floor() as i32); s.world.edit(x, y, z, B::LAVA); let hp = s.vit[0];
    for _ in 0..30 { s.tick(0.05, Input::default(), Actions::default()); } assert!(s.vit[0] < hp - 10.0, "la lave brûle : {hp} → {}", s.vit[0]);
}

#[test]
fn les_vestiges_sont_batis_et_leurs_coffres_garnis() {
    use alw_core::karst::Relic;
    let w = WorldGen::new(SEED); let mut seen = [0usize; 3];
    for i in -14..=14 { for j in -14..=14 { let Some(h) = w.hall(i, j) else { continue }; let Some(st) = w.hall_site(&h) else { continue };
        let k = match st.kind { Relic::Camp => 0, Relic::Altar => 1, Relic::Vault => 2 }; seen[k] += 1; if seen[k] > 3 { continue; } // on en vérifie trois de chaque sorte
        let at = |x: i32, y: i32, z: i32| { let v = w.gen_chunk(x.div_euclid(32), z.div_euclid(32)); v[idx(x.rem_euclid(32) as usize, y as usize, z.rem_euclid(32) as usize)] };
        let c = st.chest(); assert_eq!(at(c[0], c[1], c[2]), B::CHEST, "{:?} de la salle {} : le coffre est à sa place {c:?}", st.kind, h.kind.name());
        assert_eq!(w.cave_chest(c), Some((st.kind, h.kind))); assert_eq!(w.cave_chest([c[0], c[1] + 1, c[2]]), None);
        // posé sur du plein, de l'air pour s'y tenir devant, et sous terre
        assert!(is_solid(at(c[0], c[1] - 1, c[2])), "coffre en l'air"); assert!(w.column(st.x as f64, st.z as f64).h > st.fl as f64 + 4.0, "vestige à ciel ouvert");
        let front = match st.kind { Relic::Vault => (st.x, st.fl, st.z), Relic::Altar => (st.x + 1, st.fl + 1, st.z + 2), Relic::Camp => (st.x + 1, st.fl, st.z + 1) };
        assert!((0..2).all(|d| !is_solid(at(front.0, front.1 + d, front.2))) && is_solid(at(front.0, front.1 - 1, front.2)), "{:?} : pas de place pour s'y tenir", st.kind);
        let sign = match st.kind { Relic::Camp => B::CAMPFIRE_OUT, Relic::Altar => B::CHISELED_STONE, Relic::Vault => B::LANTERN };
        let mut found = false; for dz in -4..=4 { for dx in -4..=4 { for dy in 0..5 { if at(st.x + dx, st.fl + dy, st.z + dz) == sign { found = true; } } } } assert!(found, "{:?} sans sa marque", st.kind);
    } }
    println!("vestiges : {} camps, {} autels, {} caveaux", seen[0], seen[1], seen[2]); assert!(seen.iter().all(|n| *n >= 2), "chaque sorte de vestige existe : {seen:?}");
    // le coffre d'un camp contient quelque chose, toujours la même chose
    let (h, st) = w.nearest_site(0.0, 0.0, 14, Some(Relic::Camp)).unwrap(); let sp = w.hall_spot(&h, [st.x as f64, st.z as f64]).unwrap();
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, sp); ready(&mut s); s.open_chest(st.chest());
    let n = s.chests[&st.chest()].iter().flatten().count(); assert!(n >= 2, "coffre de camp presque vide : {n}");
}
