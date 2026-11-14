//! Grottes karstiques : rareté, forme, aven, cascade, concrétions ; même résultat quel que soit l'ordre de génération ;
//! on y casse et on y pose comme ailleurs, et la sauvegarde s'en souvient ; l'œil s'habitue à l'obscurité.
use alw_core::{blocks::*, gen::WorldGen, player::{Input, Voxels}, session::*, shapes, stream::Preset};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn halls(w: &WorldGen) -> Vec<alw_core::karst::Hall> { let mut v = Vec::new(); for i in -5..5 { for j in -5..5 { if let Some(h) = w.hall(i, j) { v.push(h); } } } v }

#[test]
fn rares_et_sous_la_terre_ferme() {
    let w = WorldGen::new(SEED); let hs = halls(&w);
    assert!((5..=40).contains(&hs.len()), "des salles, mais rares : {} sur 100 cases", hs.len());
    for h in &hs { let c = w.column(h.x, h.z); assert!(c.water == 0.0 && c.h > h.floor as f64 + h.h + 8.0, "sous terre ferme, sous une voûte épaisse : {h:?}"); }
    assert!(hs.iter().any(|h| h.shaft.is_some()) && hs.iter().any(|h| h.fall.is_some()), "des avens et des cascades");
}

#[test]
fn la_salle_est_la_meme_dans_tous_les_ordres() {
    let w = WorldGen::new(SEED); let h = halls(&w).into_iter().find(|h| h.shaft.is_some() && h.fall.is_some()).expect("une salle complète");
    let (cx, cz) = ((h.x as i32).div_euclid(32), (h.z as i32).div_euclid(32));
    let a: Vec<Vec<Id>> = (-1..=1).flat_map(|dz| (-1..=1).map(move |dx| (dx, dz))).map(|(dx, dz)| w.gen_chunk(cx + dx, cz + dz)).collect();
    let w2 = WorldGen::new(SEED); let mut b: Vec<Vec<Id>> = Vec::new(); for k in (0..9).rev() { b.push(w2.gen_chunk(cx + k % 3 - 1, cz + k / 3 - 1)); } b.reverse();
    assert!(a == b, "chaque chunk ne dépend que de la graine et de sa place");
    // ce qu'il y a dedans
    let all: Vec<Id> = a.concat(); let n = |f: &dyn Fn(Id) -> bool| all.iter().filter(|b| f(**b)).count();
    let drips = n(&|b| BLOCKS[b as usize].name.starts_with("dripstone")); let flow = n(&|b| b == B::FLOWSTONE); let lime = n(&|b| b == B::LIMESTONE);
    println!("pointes {drips}, calcite {flow}, calcaire {lime}"); assert!(drips > 20 && flow > 300 && lime > 1000, "concrétions et calcaire : {drips} {flow} {lime}");
    // le centre est vide sur une bonne hauteur, et l'aven s'ouvre jusqu'au ciel
    let at = |x: i32, y: i32, z: i32| { let (k, l) = (x.div_euclid(32) - cx + 1, z.div_euclid(32) - cz + 1); a[(k + l * 3) as usize][idx(x.rem_euclid(32) as usize, y as usize, z.rem_euclid(32) as usize)] };
    let free = (h.floor..h.floor + h.h as i32).filter(|y| at(h.x as i32, *y, h.z as i32) == 0 || is_water(at(h.x as i32, *y, h.z as i32))).count(); assert!(free >= 8, "de la hauteur au centre : {free}");
    let [sx, sz, _] = h.shaft.unwrap(); let (sx, sz) = (sx as i32, sz as i32);
    if (sx.div_euclid(32) - cx).abs() <= 1 && (sz.div_euclid(32) - cz).abs() <= 1 { assert!((h.floor + 3..CH as i32 - 1).all(|y| !is_opaque(at(sx, y, sz))), "l'aven voit le ciel"); }
    let ([fx, fy, fz], [px, pz]) = h.fall.unwrap(); if (fx.div_euclid(32) - cx).abs() <= 1 && (fz.div_euclid(32) - cz).abs() <= 1 && (px.div_euclid(32) - cx).abs() <= 1 && (pz.div_euclid(32) - cz).abs() <= 1 {
        assert_eq!(at(fx, fy, fz), B::WATER, "la source de la cascade"); assert!((h.floor + 1..fy).filter(|y| is_water(at(px, *y, pz))).count() >= 3, "l'eau tombe"); }
}

#[test]
fn les_pointes_sont_de_vrais_blocs() {
    for k in 0..3 { for d in ["down", "up"] { let b = by_name(&format!("dripstone_{d}_{k}")).unwrap(); let i = shapes::info(b).expect("une forme");
        let g = shapes::geometry(i); let w = g.iter().map(|bb| bb[3] - bb[0]).fold(0.0, f64::max); assert!(w < 0.9 && w > 0.1, "plus fine qu'un bloc");
        assert!(is_solid(b), "on s'y cogne"); assert_eq!(BLOCKS[b as usize].drop, Some("flowstone"), "elle rend de la calcite"); } }
}

#[test]
fn on_y_casse_on_y_pose_et_on_y_voit() {
    let w = WorldGen::new(SEED); let h = halls(&w).into_iter().find(|h| h.fall.is_some()).unwrap(); let spot = w.hall_spot(&h, [h.x, h.z]).expect("un endroit sûr");
    let mut s = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 1, spot); ready(&mut s); s.player.p = spot; s.third = false;
    for _ in 0..80 { s.tick(0.05, Input::default(), Actions::default()); }
    assert!(s.cave > 0.6, "sous terre, l'œil s'habitue : {:.2}", s.cave);
    // une stalagmite devant soi : on la casse à la pioche, elle rend de la calcite ; on repose la calcite
    let (x, y, z) = (spot[0].floor() as i32, spot[1] as i32, spot[2].floor() as i32 + 2); let base = s.world.get(x, y - 1, z).unwrap_or(0);
    s.world.edit(x, y - 1, z, B::LIMESTONE); s.world.edit(x, y, z, B::DRIPSTONE_UP_0); s.world.edit(x, y + 1, z, 0); let _ = base;
    s.inv[0] = Some(Stack::new("pickaxe", None)); s.sel = 0;
    let e = s.player.eye(); let d = [x as f64 + 0.5 - e[0], y as f64 + 0.4 - e[1], z as f64 + 0.5 - e[2]]; s.player.yaw = d[0].atan2(d[2]); s.player.pitch = (d[1] / d[0].hypot(d[2])).atan();
    let mut k = 0; while s.world.get(x, y, z) == Some(B::DRIPSTONE_UP_0) && k < 3000 { s.stamina = 100.0; s.tick(1.0 / 60.0, Input::default(), Actions { mine: true, ..Default::default() }); k += 1; }
    assert_eq!(s.world.get(x, y, z), Some(0), "cassée"); for _ in 0..60 { s.tick(1.0 / 60.0, Input::default(), Actions::default()); }
    assert!(s.count("flowstone") > 0 || s.loose.iter().any(|l| l.item == "flowstone"), "de la calcite : {:?} {:?} {:?}", s.notes, s.inv.iter().flatten().map(|x| x.item).collect::<Vec<_>>(), s.loose.iter().map(|l| l.item).collect::<Vec<_>>());
    s.world.edit(x, y, z, B::FLOWSTONE); let txt = s.save_text();
    let mut o = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 1, spot); o.load_text(&txt).unwrap(); o.player.p = spot; ready(&mut o);
    assert_eq!(o.world.get(x, y, z), Some(B::FLOWSTONE), "la sauvegarde garde la modification");
    // en plein air, l'œil revient
    let c = w.column(h.x + 300.0, h.z); s.player.p = [h.x + 300.5, c.h + 2.0, h.z + 0.5]; ready(&mut s); for _ in 0..120 { s.tick(0.05, Input::default(), Actions::default()); }
    assert!(s.cave < 0.2, "dehors : {:.2}", s.cave);
}
