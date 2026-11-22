//! Les points de vue : des belvédères sur des sommets qui dominent ; s'y tenir dessine la carte à 250 m, repère les lieux
//! et les villages alentour, rapporte de l'expérience, une seule fois.
use alw_core::{blocks::*, gen::*, panoramas, player::*, session::*, stream::*};
const SEED: i32 = 20261104;
const DT: f64 = 1.0 / 60.0;
fn ready(s: &mut Session) { for _ in 0..20000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn block_at(g: &WorldGen, p: [i32; 3]) -> Id { let v = g.gen_world(p[0].div_euclid(CX as i32), p[2].div_euclid(CX as i32)); v[idx(p[0].rem_euclid(CX as i32) as usize, p[1] as usize, p[2].rem_euclid(CX as i32) as usize)] }

#[test]
fn des_belvederes_sur_les_sommets() {
    let g = WorldGen::new(SEED); let all = g.viewpoints_near(0.0, 0.0, 5000.0); println!("{} points de vue à moins de 5 km", all.len()); assert!(all.len() >= 6, "assez de points de vue : {}", all.len());
    for (cell, p) in all.iter().take(6) {
        assert_eq!(block_at(&g, *p), by_name("stone_bricks").unwrap(), "le dallage en {p:?}"); assert_eq!(block_at(&g, [p[0] - 1, p[1], p[2] + 1]), by_name("stone_bricks").unwrap());
        for dy in 1..=3 { assert_eq!(block_at(&g, [p[0] + 1, p[1] + dy, p[2] + 1]), by_name("fence").unwrap(), "le mât"); { let b = block_at(&g, [p[0], p[1] + dy, p[2]]); assert!(!is_solid(b), "de l'air au-dessus du dallage : {}", BLOCKS[b as usize].name); } }
        assert_eq!(block_at(&g, [p[0] + 1, p[1] + 4, p[2] + 1]), by_name("flag").unwrap(), "le drapeau");
        // il domine : le sommet est bien au-dessus du pays alentour
        let mut sum = 0.0; let mut n = 0.0; for a in 0..12 { let t = a as f64 / 12.0 * std::f64::consts::TAU; let c = g.column(p[0] as f64 + 90.0 * t.cos(), p[2] as f64 + 90.0 * t.sin()); sum += c.h.max(SEA); n += 1.0; }
        assert!(p[1] as f64 > sum / n + 8.0, "{cell:?} domine ses alentours : {} contre {:.0}", p[1], sum / n);
    }
    assert!(WorldGen::new(FLAT | 3).viewpoints_near(0.0, 0.0, 5000.0).is_empty(), "le monde plat n'en a pas");
}

#[test]
fn le_panorama_dessine_la_carte() {
    let g = WorldGen::new(SEED); let (cell, top) = g.viewpoints_near(0.0, 0.0, 5000.0)[0];
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [top[0] as f64 - 6.5, top[1] as f64 + 3.0, top[2] as f64 + 0.5]); ready(&mut s);
    s.set_creative(false); s.prog.xp = 0.0; for _ in 0..30 { s.tick(DT, Input::default(), Actions::default()); }
    assert!(s.views.is_empty(), "pas encore"); assert_eq!(s.known_views(300.0).len(), 1, "aperçu : sur la carte, à faire"); assert!(!s.known_views(300.0)[0].1);
    let before = s.map.explored(); let places = s.prog.places.len(); let t0 = std::time::Instant::now();
    s.player.p = [top[0] as f64 + 0.5, top[1] as f64 + 1.0, top[2] as f64 + 0.5]; s.player.v = [0.0; 3]; for _ in 0..30 { s.tick(DT, Input::default(), Actions::default()); }
    let dt = t0.elapsed().as_secs_f64(); assert!(s.views.contains(&cell), "point de vue fait : {:?}", s.notes);
    let drawn = s.map.explored() - before; println!("{drawn} pixels dessinés en {dt:.2} s ; lieux repérés : {} → {} ; {:?}", places, s.prog.places.len(), s.notes.iter().rev().take(2).collect::<Vec<_>>());
    assert!(drawn > 9000, "la carte se dessine à 250 m : {drawn} pixels"); assert!(dt < 3.0, "sans à-coup notable : {dt:.2} s");
    assert!(s.prog.xp >= panoramas::VIEW_XP - 1e-9, "de l'expérience"); assert!(s.notes.iter().any(|n| n.starts_with("Point de vue")));
    // un point à 200 m du sommet est sur la carte, pas un à 400 m
    let px = |x: i32, z: i32| s.map.get(x.div_euclid(4), z.div_euclid(4)); assert!(px(top[0] + 200, top[2]) != 0 && px(top[0] + 400, top[2]) == 0);
    let xp = s.prog.xp; for _ in 0..30 { s.tick(DT, Input::default(), Actions::default()); } assert_eq!(s.prog.xp, xp, "une seule fois"); assert!(s.known_views(300.0)[0].1, "marqué fait");
    let t = s.save_text(); let mut l = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 120.0, 0.5]); l.load_text(&t).unwrap(); assert!(l.views.contains(&cell), "sauvé avec la partie");
}

#[test]
fn le_croquis_a_les_couleurs_du_pays() {
    let g = WorldGen::new(SEED); let rgb = |v: u32| [(v >> 24) as u8 as i32, (v >> 16) as u8 as i32, (v >> 8) as u8 as i32];
    // la mer est bleue, la terre ne l'est pas
    let mut sea = None; let mut land = None; 'f: for i in 0..200 { for j in 0..200 { let (x, z) = (i as f64 * 37.0 - 3700.0, j as f64 * 37.0 - 3700.0); let c = g.column(x, z);
        if c.h < SEA - 6.0 && sea.is_none() { sea = Some((x, z)); } if c.h > SEA + 10.0 && c.water == 0.0 && land.is_none() { land = Some((x, z)); } if sea.is_some() && land.is_some() { break 'f; } } }
    let (sx, sz) = sea.expect("de la mer"); let c = rgb(panoramas::sketch(&g, sx, sz)); assert!(c[2] > c[0] + 40, "la mer en bleu : {c:?}");
    let (lx, lz) = land.unwrap(); let c = rgb(panoramas::sketch(&g, lx, lz)); assert!(c[2] < c[1] + 20, "la terre n'est pas bleue : {c:?}");
}
