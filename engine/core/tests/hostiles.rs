//! Créatures hostiles : données, danger, apparition la nuit dans le noir (et pas sous la lumière), perception à travers
//! les murs, combat, mort et butin, adaptation par région, sauvegarde.
use alw_core::{blocks::B, gen::WorldGen, hostile::{self as h, bestiary, biome_danger, distance_bonus, adapt_cut, danger_of, Ai}, player::*, session::*, stream::*};
use alw_core::session::item;
fn species(id: &str) -> Option<usize> { h::species(id) }
const SEED: i32 = 20261104;
const DT: f64 = 1.0 / 30.0;
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(4)); } }
fn run(s: &mut Session, sec: f64, mut each: impl FnMut(&Session)) { for i in 0..(sec / DT) as usize { s.tick(DT, Input::default(), Actions::default()); if i % 30 == 0 { let _ = s.sync(); } each(s); } }
fn at(x: f64, z: f64, hour: f64) -> Session {
    let g = WorldGen::new(SEED); let y = g.column(x, z).h.ceil() + 0.05;
    let mut s = Session::new(SEED, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [x, y, z]); ready(&mut s); s.third = false; s.hour = hour; s.wild.all.clear(); s.vit = [100.0; 3]; s
}
fn quiet_spot() -> (f64, f64) {
    // Un point sec, hors des villages, près du départ.
    let g = WorldGen::new(SEED);
    for k in 0..200 { let (x, z) = (40.0 + k as f64 * 13.0, -30.0 - k as f64 * 7.0); if g.column(x, z).water == 0.0 && g.villages_near(x, z, 200.0).iter().all(|v| (v.c[0] as f64 - x).hypot(v.c[2] as f64 - z) > v.r as f64 + 60.0) { return (x, z); } }
    panic!("pas de lieu tranquille")
}

#[test]
fn le_bestiaire_vient_des_donnees() {
    let b = bestiary(); assert!(b.species.len() >= 4);
    for id in ["rodeur", "brigand", "rodeuse", "rampant"] { let s = &b.species[species(id).expect(id)]; assert_eq!(s.tier, 1, "{id} est commun"); assert!(s.biomes.is_empty(), "{id} vit partout");
        for l in &s.loot { assert!(item(l.0).is_some(), "{id} : butin inconnu {}", l.0); } }
    let bodies: std::collections::HashSet<String> = b.species.iter().map(|s| format!("{:?}", s.body)).collect(); assert_eq!(bodies.len(), 4, "quatre corps différents");
    assert_eq!(b.rules.light, 7); assert_eq!(b.rules.distance.len(), 4); assert_eq!(b.rules.biome[5], 3, "désert : 3"); assert_eq!(b.rules.biome[1], 1, "prairie : 1"); assert_eq!(b.rules.biome[6], 4);
}

#[test]
fn la_formule_du_danger() {
    assert_eq!(distance_bonus(100.0), 0); assert_eq!(distance_bonus(300.0), 1); assert_eq!(distance_bonus(2000.0), 3); assert_eq!(distance_bonus(5000.0), 4);
    assert_eq!(adapt_cut(0), 0); assert_eq!(adapt_cut(5), 1); assert_eq!(adapt_cut(12), 2); assert_eq!(adapt_cut(40), 4);
    // L'adaptation n'enlève que le bonus de distance : le danger du biome reste.
    assert_eq!(danger_of(3, 4, 0, 0, 0), 7); assert_eq!(danger_of(3, 4, 2, 0, 0), 5); assert_eq!(danger_of(3, 1, 4, 0, 0), 3, "jamais sous le danger du biome"); assert_eq!(danger_of(3, 0, 0, 1, 2), 6);
    assert_eq!(biome_danger(1, 60.0), 1); assert_eq!(biome_danger(6, 60.0), 4); assert_eq!(biome_danger(1, 140.0), 5, "haute montagne");
}

#[test]
fn la_nuit_dans_le_noir_et_pas_de_jour() {
    let (x, z) = quiet_spot();
    let mut s = at(x, z, 12.0); run(&mut s, 30.0, |_| {}); assert_eq!(s.horde.spawned, 0, "rien de jour à découvert");
    let mut s = at(x, z, 23.5); let mut first: Vec<f64> = Vec::new(); let mut seen = 0;
    run(&mut s, 50.0, |s| { while seen < s.horde.foes.len() { let f = &s.horde.foes[seen]; first.push((f.p[0] - s.player.p[0]).hypot(f.p[2] - s.player.p[2])); assert!(s.light_at(f.p[0].floor() as i32, f.p[1].floor() as i32, f.p[2].floor() as i32) < 7, "née dans le noir"); seen += 1; } });
    assert!(s.horde.spawned >= 2, "la nuit, des créatures apparaissent : {}", s.horde.spawned);
    assert!(first.iter().all(|d| *d > 20.0), "jamais sous le nez : {first:?}");
    assert!(s.horde.foes.len() <= bestiary().rules.cap * 3 / 2 + 3, "la nuit, la moitié de plus au plafond");
}

#[test]
fn la_lumiere_empeche_les_apparitions() {
    let (x, z) = quiet_spot(); let mut s = at(x, z, 23.5); let p = s.player.p; let g = WorldGen::new(SEED); let mut edits = Vec::new();
    // Des torches tous les 4 blocs sur tout le cercle d'apparition.
    for i in -13..=13 { for j in -13..=13 { let (tx, tz) = (p[0] + i as f64 * 4.0, p[2] + j as f64 * 4.0); let h = g.column(tx, tz).h.ceil() as i32; edits.push((tx.floor() as i32, h, tz.floor() as i32, B::TORCH)); } }
    s.world.edit_many(&edits); for _ in 0..200 { let _ = s.sync(); std::thread::sleep(std::time::Duration::from_millis(5)); }
    let lit = (0..40).filter(|k| { let a = *k as f64 * 0.157; let (tx, tz) = (p[0] + a.cos() * 30.0, p[2] + a.sin() * 30.0); let h = g.column(tx, tz).h.ceil() as i32; s.light_at(tx.floor() as i32, h + 1, tz.floor() as i32) >= 7 }).count();
    assert!(lit >= 36, "le cercle est éclairé : {lit}/40");
    let mut seen = 0; let mut where_: Vec<String> = Vec::new(); run(&mut s, 40.0, |s| { while seen < s.horde.foes.len() { let f = &s.horde.foes[seen]; where_.push(format!("{:.0},{:.1},{:.0} l={} d={:.0}", f.p[0] - p[0], f.p[1] - p[1], f.p[2] - p[2], s.light_at(f.p[0].floor() as i32, f.p[1].floor() as i32, f.p[2].floor() as i32), (f.p[0]-p[0]).hypot(f.p[2]-p[2]))); seen += 1; } });
    // Il reste quelques recoins sombres entre les torches (creux, dessous de rochers) : de rares apparitions, jamais en pleine lumière.
    assert!(s.horde.spawned <= 14, "presque rien sous la lumière : {} {:?}", s.horde.spawned, where_); assert!(where_.iter().all(|w| w.split("l=").nth(1).and_then(|x| x.split(' ').next()).and_then(|x| x.parse::<u8>().ok()).is_some_and(|l| l < 8)), "{where_:?}");
}

#[test]
fn chaque_biome_a_ses_creatures_la_nuit() {
    let g = WorldGen::new(SEED); let mut found = [None; alw_core::biome::COUNT];
    'f: for r in (300..7000).step_by(300) { for k in 0..24 { let a = k as f64 * 0.2618; let (x, z) = (r as f64 * a.cos(), r as f64 * a.sin()); let m = g.climate.at(x, z);
        let b = m.main(); if found[b].is_none() && m.w[b] > 0.7 && g.column(x, z).water == 0.0 && g.villages_near(x, z, 200.0).iter().all(|v| (v.c[0] as f64 - x).hypot(v.c[2] as f64 - z) > v.r as f64 + 60.0) { found[b] = Some((x, z)); }
        if found.iter().all(|f| f.is_some()) { break 'f; } } }
    let mut done = 0;
    for (b, spot) in found.iter().enumerate() { let Some((x, z)) = spot else { continue }; let mut s = at(*x, *z, 23.5); run(&mut s, 45.0, |_| {});
        assert!(s.horde.spawned >= 1, "{} : aucune créature la nuit", alw_core::biome::NAMES[b]); assert!(s.danger.biome == bestiary().rules.biome[b] || s.danger.biome == bestiary().rules.mountain.1, "{} : danger du biome", alw_core::biome::NAMES[b]); done += 1; }
    assert!(done >= 5, "{done} biomes essayés");
}

#[test]
fn un_mur_cache_le_joueur() {
    let (x, z) = quiet_spot(); let mut s = at(x, z, 23.5); s.creative = false; let p = s.player.p; let (bx, by, bz) = (p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32);
    // Une cabane de pierre fermée autour du joueur.
    let mut e = Vec::new(); for dx in -2i32..=2 { for dz in -2i32..=2 { for dy in 0..4 { let edge = dx.abs() == 2 || dz.abs() == 2 || dy == 3; if edge { e.push((bx + dx, by + dy, bz + dz, B::STONE)); } } } } s.world.edit_many(&e); let _ = s.sync();
    s.player.crouch = true; let i = s.horde.put(species("rodeur").unwrap(), [p[0] + 6.0, p[1], p[2]], -1.57, false);
    run(&mut s, 8.0, |_| {}); let f = &s.horde.foes[i.min(s.horde.foes.len() - 1)]; assert!(!matches!(f.state, Ai::Poursuite | Ai::Combat), "à travers les murs, elle ne sait pas : {:?}", f.state);
    // Les murs tombent : elle le voit.
    let open: Vec<_> = e.iter().map(|q| (q.0, q.1, q.2, 0)).collect(); s.world.edit_many(&open); s.player.crouch = false; let _ = s.sync();
    s.horde.foes.clear(); s.hour = 12.0; s.horde.put(species("rodeur").unwrap(), [p[0] + 6.0, p[1], p[2]], -1.57, false); run(&mut s, 4.0, |_| {}); // au grand jour : aucune autre ne naît, et l'on voit loin
    let f = s.horde.foes.iter().min_by(|a, b| (a.p[0] - p[0] - 6.0).hypot(a.p[2] - p[2]).total_cmp(&(b.p[0] - p[0] - 6.0).hypot(b.p[2] - p[2]))).unwrap(); // (d'autres ont pu naître dans la nuit)
    assert!(matches!(f.state, Ai::Poursuite | Ai::Combat | Ai::Detection), "à découvert, elle le voit : {:?}", f.state);
}

#[test]
fn combat_mort_et_butin() {
    let (x, z) = quiet_spot();
    for id in ["rodeur", "brigand", "rodeuse", "rampant"] {
        let mut s = at(x, z, 23.5); let p = s.player.p; let i = s.horde.put(species(id).unwrap(), [p[0] + 3.0, p[1], p[2]], -1.57, true); s.horde.foes[i].alert = 1.0; s.horde.foes[i].seen = Some(p);
        s.horde.foes[i].state = Ai::Poursuite; let hp0 = s.vit[0]; let mut cause = ""; run(&mut s, 12.0, |s| { if s.vit[0] < hp0 && cause.is_empty() { cause = s.cause; } });
        assert!(!cause.is_empty(), "{id} : elle attaque"); assert!(cause.contains(match id { "rodeur" => "rôdeur", "brigand" => "brigand", "rodeuse" => "bête", _ => "rampant" }), "{id} : {cause}");
        // On la tue ; elle tombe, puis son butin est au sol.
        let me = s.player.p; let k = s.horde.foes.iter().position(|f| f.dead == 0.0).unwrap(); let mut dead = false; for _ in 0..40 { if s.horde.strike(k, 12.0, me, "epee", false) { dead = true; break; } }
        assert!(dead, "{id} meurt"); run(&mut s, 2.5, |_| {}); assert!(!s.horde.foes.iter().any(|f| f.dead > 0.0), "{id} : le corps disparaît");
        let loot: Vec<&str> = s.loose.iter().map(|l| l.item).collect(); assert!(!loot.is_empty(), "{id} : du butin au sol");
        let sp = &bestiary().species[species(id).unwrap()]; assert!(loot.iter().all(|i| sp.loot.iter().any(|l| l.0 == *i)), "{id} : butin de sa table {loot:?}");
    }
}

#[test]
fn l_attaque_s_annonce_et_le_bouclier_la_pare() {
    let (x, z) = quiet_spot(); let mut s = at(x, z, 23.5); let p = s.player.p; let i = s.horde.put(species("brigand").unwrap(), [p[0], p[1], p[2] + 1.6], 3.14, false);
    s.horde.foes[i].state = Ai::Combat; s.horde.foes[i].alert = 1.0; s.horde.foes[i].seen = Some(p);
    let mut wound = false; for _ in 0..60 { s.tick(DT, Input::default(), Actions::default()); if s.horde.foes[i].wind > 0.0 { wound = true; break; } } assert!(wound, "l'attaque s'annonce avant de frapper");
    assert_eq!(s.vit[0], 100.0, "pas encore de coup pendant l'annonce");
}

#[test]
fn l_adaptation_se_gagne_par_region() {
    let (x, z) = quiet_spot(); let mut s = at(x, z, 10.0); s.inv_open = true; let need = bestiary().rules.day_secs;
    let key = s.region_key(s.player.p[0], s.player.p[2]); run(&mut s, need * 0.5, |_| {}); assert_eq!(s.regions.get(&key).map_or(0, |r| r.days), 0, "une demi-présence ne compte pas");
    run(&mut s, need * 0.6, |_| {}); assert_eq!(s.regions[&key].days, 1, "une journée validée");
    run(&mut s, need * 1.2, |_| {}); assert_eq!(s.regions[&key].days, 1, "une seule par journée de jeu");
    // Une autre région, même biome : rien n'est acquis.
    let far = s.region_key(s.player.p[0] + 2000.0, s.player.p[2]); assert!(!s.regions.contains_key(&far));
    // Loin du départ, dix journées retirent deux du bonus de distance, jamais le danger du biome.
    s.spawn = [s.player.p[0] - 2000.0, 0.0, s.player.p[2]]; s.regions.get_mut(&key).unwrap().days = 10; let d = s.danger_here(); assert_eq!(d.distance, 3); assert_eq!(d.adapt, 2); assert_eq!(d.total, d.biome + 1);
    s.regions.get_mut(&key).unwrap().days = 40; let d = s.danger_here(); assert_eq!(d.total, d.biome, "au plus, le bonus de distance disparaît");
    // La sauvegarde garde l'adaptation.
    let t = s.save_text(); let mut b = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, s.player.p); b.load_text(&t).unwrap(); assert_eq!(b.regions.get(&key).map(|r| r.days), Some(40));
}
