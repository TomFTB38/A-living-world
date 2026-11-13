//! Mer et océan : de grands bassins, une côte sans mur d'eau, des fonds vivants (zostère, varech, coraux), des plages, et une faune qui nage en bancs.
use alw_core::{biome::*, blocks::*, gen::*, player::Voxels, wild::*};
const SEED: i32 = 20261104;
fn find(g: &WorldGen, ok: impl Fn(&Column) -> bool) -> (f64, f64) { for r in 20..900 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); if ok(&g.column(x, z)) { return (x, z); } } } panic!("introuvable") }
#[test]
fn des_mers_et_des_oceans_assez_grands() {
    let g = WorldGen::new(SEED); let (mut sea, mut ocean, mut n) = (0, 0, 0);
    for j in -60..60 { for i in -60..60 { let m = g.climate.at(i as f64 * 160.0, j as f64 * 160.0); n += 1; match m.main() { 8 => sea += 1, 9 => ocean += 1, _ => {} } } }
    let (fs, fo) = (sea as f64 / n as f64, ocean as f64 / n as f64); println!("mer {:.1} %, océan {:.1} %", fs * 100.0, fo * 100.0);
    assert!(fs > 0.1 && fo > 0.04 && fs + fo < 0.45);
    // Un grand large : au moins 2 000 blocs d'océan d'un seul tenant, en ligne droite.
    let (x, z) = find(&g, |c| c.mix.strong(Biome::Ocean) > 0.95); let mut best = 0.0f64;
    for k in 0..16 { let a = k as f64 / 16.0 * std::f64::consts::TAU; let mut d = 0.0; while d < 6000.0 && g.climate.at(x + a.cos() * d, z + a.sin() * d).sea > 0.5 { d += 50.0; } let mut e = 0.0; while e < 6000.0 && g.climate.at(x - a.cos() * e, z - a.sin() * e).sea > 0.5 { e += 50.0; } best = best.max(d + e); }
    println!("plus grande traversée : {best} blocs"); assert!(best >= 2000.0);
    assert_eq!(NAMES[Biome::Sea as usize], "Mer"); assert_eq!(NAMES[Biome::Ocean as usize], "Océan");
}
#[test]
fn la_cote_ne_laisse_pas_de_mur_d_eau() {
    // Le long d'une côte, aucune case d'air sous le niveau de la mer ne touche l'eau de mer : l'eau ne tient jamais debout toute seule.
    let g = WorldGen::new(SEED); let (x, z) = find(&g, |c| c.mix.sea > 0.1 && c.mix.sea < 0.3 && c.water == SEA); let (cx, cz) = ((x / CX as f64).floor() as i32, (z / CX as f64).floor() as i32);
    let mut chunks = std::collections::HashMap::new(); for dz in -3..=3 { for dx in -3..=3 { chunks.insert((cx + dx, cz + dz), g.gen_chunk(cx + dx, cz + dz)); } }
    let get = |x: i32, y: i32, z: i32| chunks.get(&(x.div_euclid(CX as i32), z.div_euclid(CX as i32))).map(|v| v[idx(x.rem_euclid(CX as i32) as usize, y as usize, z.rem_euclid(CX as i32) as usize)]);
    let (mut walls, mut wet) = (0, 0);
    for wz in (cz - 2) * CX as i32..(cz + 3) * CX as i32 { for wx in (cx - 2) * CX as i32..(cx + 3) * CX as i32 { let y = SEA as i32; if get(wx, y, wz) != Some(B::WATER) { continue; } wet += 1;
        for (a, b) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { if get(wx + a, y, wz + b) == Some(0) { walls += 1; if walls < 6 { let c = g.column((wx + a) as f64, (wz + b) as f64); let c0 = g.column(wx as f64, wz as f64); println!("mur en {} {} : air h {:.2} sea {:.4} water {} | eau h {:.2} sea {:.4} water {} still {}", wx + a, wz + b, c.h, c.mix.sea, c.water, c0.h, c0.mix.sea, c0.water, c0.still); } } } } }
    println!("{wet} cases d'eau de surface, {walls} bords d'eau contre de l'air"); assert!(wet > 200); assert!(walls * 200 <= wet, "trop de murs d'eau : {walls}");
}
fn count(g: &WorldGen, x: f64, z: f64, r: i32) -> std::collections::HashMap<&'static str, usize> {
    let (cx, cz) = ((x / CX as f64).floor() as i32, (z / CX as f64).floor() as i32); let mut n = std::collections::HashMap::new();
    for dz in -r..=r { for dx in -r..=r { for b in g.gen_chunk(cx + dx, cz + dz) { if b != 0 && b != B::WATER && b != B::STONE { *n.entry(BLOCKS[b as usize].name).or_default() += 1; } } } } n
}
#[test]
fn fonds_marins_et_plages() {
    let g = WorldGen::new(SEED);
    let (x, z) = find(&g, |c| c.mix.sea > 0.6 && c.mix.temp > 0.45 && c.water == SEA && c.water - c.h >= 3.0 && c.water - c.h <= 8.0); let n = count(&g, x, z, 2);
    println!("récif : corail {:?}, gorgone {:?}, zostère {:?}", n.get("coral"), n.get("coral_fan"), n.get("seagrass")); assert!(n.get("coral").copied().unwrap_or(0) > 30 && n.get("coral_fan").copied().unwrap_or(0) > 5);
    let (x, z) = find(&g, |c| c.mix.sea > 0.6 && c.mix.temp < 0.2 && c.water == SEA && c.water - c.h >= 6.0 && c.water - c.h <= 16.0); let n = count(&g, x, z, 3);
    println!("eaux fraîches : varech {:?} / {:?}, zostère {:?}", n.get("kelp"), n.get("kelp_top"), n.get("seagrass")); assert!(n.get("kelp_top").copied().unwrap_or(0) > 10 && n.get("seagrass").copied().unwrap_or(0) > 30);
    let (x, z) = find(&g, |c| c.mix.sea > 0.03 && c.mix.sea < 0.3 && c.water == 0.0 && c.h < SEA + 2.0); let n = count(&g, x, z, 2);
    println!("plage : sable {:?}, coquillages {:?}, oyats {:?}", n.get("sand"), n.get("shell"), n.get("marram")); assert!(n.get("sand").copied().unwrap_or(0) > 200 && n.get("shell").copied().unwrap_or(0) > 3);
    assert!(is_aquatic(B::KELP) && is_wet(B::CORAL) && !is_wet(B::SHELL));
}
/// Une mer plate : fond de sable en y = 9, eau de 10 à 19, de l'air au-dessus.
struct Sea;
impl Voxels for Sea { fn get(&self, _x: i32, y: i32, _z: i32) -> Option<u16> { Some(if y <= 9 { B::SAND } else if y <= 19 { B::WATER } else { 0 }) } }
#[test]
fn les_poissons_nagent_en_banc_et_restent_dans_l_eau() {
    let mut w = Wild::new(5); let p = [0.0, 21.0, 0.0];
    for _ in 0..400 { w.update(0.05, &Sea, p, 1.0, 0.0, 0.0, 0.0, false, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0], 0.0); }
    let fish: Vec<&Critter> = w.all.iter().filter(|c| c.kind == Kind::Fish).collect(); println!("{} maquereaux, {} dauphins, {} goélands", fish.len(), w.all.iter().filter(|c| c.kind == Kind::Dolphin).count(), w.all.iter().filter(|c| c.kind == Kind::Gull).count());
    assert!(fish.len() >= 10, "des bancs apparaissent en mer"); assert!(w.all.iter().all(|c| c.kind != Kind::Fox && c.kind != Kind::Reef), "ni renard ni poisson tropical dans une mer fraîche");
    for f in &fish { assert!(f.p[1] > 10.0 && f.p[1] < 20.0, "dans l'eau : {:?}", f.p); }
    // Un banc reste groupé : chaque poisson est à moins de 6 blocs d'un autre.
    for f in &fish { assert!(fish.iter().any(|o| !std::ptr::eq(*o, *f) && (o.p[0] - f.p[0]).hypot(o.p[2] - f.p[2]) < 6.0)); }
    let mut w = Wild::new(6); for _ in 0..300 { w.update(0.05, &Sea, p, 1.0, 0.0, 0.0, 0.0, false, [0.0, 0.0, 0.0, 1.0, 1.0, 0.0], 0.0); } assert!(w.all.iter().any(|c| c.kind == Kind::Reef), "mer chaude : poissons des récifs");
}
#[test]
fn epaves_grands_coraux_et_ports() {
    let g = WorldGen::new(SEED);
    // Des épaves au fond, avec leur coffre (butin de navire) posé sous l'eau.
    let w = g.wrecks_near(0.0, 0.0, 6000.0); println!("{} épaves", w.len()); assert!(w.len() >= 10); let p = w[0];
    assert!(g.wreck_chest(p)); assert!(!g.wreck_chest([p[0] + 1, p[1], p[2]])); assert!((p[1] as f64) < SEA - 4.0, "l'épave repose au fond");
    let (cx, cz) = (p[0].div_euclid(CX as i32), p[2].div_euclid(CX as i32)); let v = g.gen_chunk(cx, cz); assert_eq!(v[idx(p[0].rem_euclid(CX as i32) as usize, p[1] as usize, p[2].rem_euclid(CX as i32) as usize)], B::CHEST);
    let n = count(&g, p[0] as f64, p[2] as f64, 1); assert!(n.get("wreck_planks").copied().unwrap_or(0) > 60, "une vraie coque : {:?}", n.get("wreck_planks"));
    // Grands coraux : des massifs de blocs de corail dans les mers chaudes, jamais au-dessus de l'eau.
    let (x, z) = find(&g, |c| c.mix.sea > 0.6 && c.mix.temp > 0.45 && c.water == SEA && c.water - c.h >= 4.0); let n = count(&g, x, z, 3);
    let big: usize = n.iter().filter(|(k, _)| k.starts_with("coral_block")).map(|(_, v)| *v).sum(); println!("grands coraux : {big} blocs"); assert!(big > 200);
    // Des ports de pêche, avec leur ponton de planches sur l'eau.
    let ports: Vec<_> = g.villages_near(0.0, 0.0, 6000.0).into_iter().filter(|v| v.kind == "port").collect(); println!("{} ports", ports.len()); assert!(ports.len() >= 3);
    let pv = &ports[0]; let mut planks_over_water = 0; let r = (pv.r / CX as i32) + 2;
    for dz in -r..=r { for dx in -r..=r { let (cx, cz) = (pv.c[0].div_euclid(CX as i32) + dx, pv.c[2].div_euclid(CX as i32) + dz); let v = g.gen_world(cx, cz);
        for z in 0..CX { for x in 0..CX { let y = SEA as usize + 1; if BLOCKS[v[idx(x, y, z)] as usize].name == "spruce_planks" && v[idx(x, y - 1, z)] == B::WATER { planks_over_water += 1; } } } } }
    println!("ponton : {planks_over_water} planches au-dessus de l'eau"); assert!(planks_over_water >= 20);
}
#[test]
fn le_requin_chasse_le_nageur_et_la_baleine_reste_paisible() {
    let mut w = Wild::new(9); let p = [0.0, 14.0, 0.0]; // le joueur nage à mi-profondeur
    w.put_swim(Kind::Shark, [12.0, 14.0, 0.0], 0.0, 7, 0); w.put_swim(Kind::Whale, [-20.0, 14.5, 0.0], 0.0, 8, 0);
    let mut bit = false; for _ in 0..400 { w.update(0.05, &Sea, p, 1.0, 0.0, 0.0, 0.0, false, [0.0; 6], 0.0); for e in std::mem::take(&mut w.ev) { if let Ev::Bite(_, d, cause) = e { bit = true; assert!(d > 0.0 && cause.contains("requin")); } } }
    assert!(bit, "le requin finit par mordre le nageur");
    let mut w = Wild::new(10); w.put_swim(Kind::Shark, [6.0, 14.0, 0.0], 0.0, 7, 0); let dry = [0.0, 21.0, 0.0]; // hors de l'eau : rien à craindre
    for _ in 0..300 { w.update(0.05, &Sea, dry, 1.0, 0.0, 0.0, 0.0, false, [0.0; 6], 0.0); assert!(!w.ev.iter().any(|e| matches!(e, Ev::Bite(..)))); w.ev.clear(); }
    let wh = Wild::new(11); let _ = wh; // la baleine n'a pas de boîte de chasse : on ne la tue pas
    assert!(alw_core::outdoors::wild_box(Kind::Whale).is_none() && alw_core::outdoors::wild_box(Kind::Shark).is_some());
}
