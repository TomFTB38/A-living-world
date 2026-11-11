//! Monde vivant : loups et sangliers, chasse à l'arc, pêche, cultures de racines.
use alw_core::{blocks::*, player::*, session::*, stream::*, survival::recipes, wild::Kind};
const DT: f64 = 1.0 / 60.0;
fn none() -> Input { Input::default() }
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, sec: f64) { for _ in 0..(sec / DT) as usize { s.tick(DT, none(), Actions::default()); } }
fn look_at(s: &mut Session, q: [f64; 3]) { let e = s.player.eye(); let d = [q[0] - e[0], q[1] - e[1], q[2] - e[2]]; s.player.yaw = d[0].atan2(d[2]); s.player.pitch = (d[1] / d[0].hypot(d[2])).atan(); s.tick(0.0, none(), Actions::default()); }
/// Bander l'arc une seconde (clic droit tenu), puis relâcher.
fn shoot(s: &mut Session) { for _ in 0..62 { s.tick(DT, none(), Actions { guard: true, ..Default::default() }); } s.tick(DT, none(), Actions::default()); }
fn right(s: &mut Session) { s.tick(DT, none(), Actions { mine: false, place: true, ..Default::default() }); }
/// Un terrain plat de pierre, 25 × 25, dégagé ; le joueur au milieu, mains vides.
fn site() -> (Session, i32) {
    let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, 120.0, 0.5]); ready(&mut s); s.third = false; s.inv = [None; INV_SIZE];
    let y = alw_core::gen::WorldGen::new(20261104).column(0.5, 0.5).h.floor() as i32 + 3; let mut e = Vec::new();
    for x in -12..=12 { for z in -12..=12 { e.push((x, y - 1, z, B::STONE)); for h in 0..6 { e.push((x, y + h, z, 0)); } } } s.world.edit_many(&e);
    s.player.p = [0.5, y as f64, 0.5]; idle(&mut s, 0.3); s.wild.all.clear(); s.fauna.deer.clear(); (s, y)
}
fn health_after_wolf(armor: bool) -> f64 {
    let (mut s, y) = site(); s.hour = 23.0; if armor { s.equip[1] = Some(Stack::new("iron_chestplate", None)); }
    s.wild.put(Kind::Wolf, [0.5, y as f64, 7.5], 3.14, false); for _ in 0..(6.0 / DT) as usize { s.tick(DT, none(), Actions::default()); s.wild.all.retain(|c| c.kind == Kind::Wolf); }
    s.vit[0]
}

#[test]
fn loups_la_nuit_et_armure() {
    let bare = health_after_wolf(false); assert!(bare < 85.0, "la nuit, le loup attaque : vie {bare}");
    let armored = health_after_wolf(true); assert!(armored > bare + 5.0, "le plastron amortit les morsures : {armored} contre {bare}");
    // de jour, un loup isolé loin du joueur ne le remarque pas
    let (mut s, y) = site(); s.hour = 12.0; s.wild.put(Kind::Wolf, [0.5, y as f64, 11.5], 0.0, false); idle(&mut s, 3.0); assert_eq!(s.vit[0], 100.0, "de jour, à onze blocs, il ne vient pas");
}

#[test]
fn sanglier_charge_si_on_le_frappe() {
    let (mut s, y) = site(); s.hour = 12.0; s.wild.put(Kind::Boar, [0.5, y as f64, 2.5], 3.14, false); idle(&mut s, 3.0); s.wild.all.retain(|c| c.kind == Kind::Boar);
    assert_eq!(s.vit[0], 100.0, "le sanglier broute sans s'occuper du joueur");
    let b = s.wild.all.iter().position(|c| c.kind == Kind::Boar).expect("sanglier"); let q = s.wild.all[b].p; look_at(&mut s, [q[0], q[1] + 0.4, q[2]]);
    assert!(matches!(s.prey, Some(Prey::Wild(_))), "le sanglier est visé"); s.tick(DT, none(), Actions { mine: true, place: false, ..Default::default() });
    for _ in 0..(4.0 / DT) as usize { s.tick(DT, none(), Actions::default()); s.wild.all.retain(|c| c.kind == Kind::Boar); } assert!(s.vit[0] < 100.0, "frappé, il charge : vie {}", s.vit[0]);
}

#[test]
fn arc_et_fleches() {
    let (mut s, y) = site(); s.hour = 12.0; s.inv[0] = Some(Stack::new("bow", None)); s.add_item("arrow", 3); s.sel = 0;
    // un lièvre à onze blocs : une flèche suffit
    let q = [0.5, y as f64, 11.5]; s.wild.put(Kind::Hare, q, 0.0, false); look_at(&mut s, [q[0], q[1] + 0.25 + 0.7, q[2]]); shoot(&mut s);
    for _ in 0..60 { s.tick(DT, none(), Actions::default()); } s.collect_loose(); assert!(s.count("raw_meat") >= 2, "le lièvre est abattu de loin"); assert_eq!(s.count("arrow"), 2);
    // une flèche manquée se plante dans le sol ; on la ramasse en marchant dessus
    s.wild.all.clear(); idle(&mut s, 0.8); look_at(&mut s, [0.5, y as f64, 6.5]); shoot(&mut s); idle(&mut s, 1.0);
    let a = s.arrows.iter().find(|a| a.stuck).copied().expect("flèche plantée"); assert!((a.p[2] - 6.5).abs() < 1.5, "plantée là où l'on visait : {:?}", a.p); assert_eq!(s.count("arrow"), 1);
    s.player.p = [a.p[0], y as f64, a.p[2]]; idle(&mut s, 0.3); assert_eq!(s.count("arrow"), 2, "flèche ramassée"); assert!(s.arrows.is_empty());
    s.inv = [None; INV_SIZE]; s.inv[0] = Some(Stack::new("bow", None)); s.sel = 0; idle(&mut s, 1.0); shoot(&mut s); assert!(s.notes.iter().any(|n| n == "Plus de flèches"));
}

#[test]
fn peche() {
    let (mut s, y) = site(); let mut e = Vec::new(); for x in -3..=3 { for z in 3..=8 { e.push((x, y - 1, z, B::WATER)); e.push((x, y - 2, z, B::STONE)); } } s.world.edit_many(&e);
    s.inv[0] = Some(Stack::new("fishing_rod", None)); s.sel = 0; look_at(&mut s, [0.5, y as f64 - 0.5, 6.5]); right(&mut s);
    let l = s.line.expect("ligne lancée"); assert!(["trout", "perch", "char"].contains(&l.fish)); assert!(l.bite > s.elapsed + 3.0, "il faut attendre la touche");
    // trop tôt : on ramène la ligne à vide
    idle(&mut s, 0.6); right(&mut s); assert!(s.line.is_none() && s.count(l.fish) == 0, "ferrer trop tôt ne prend rien");
    idle(&mut s, 0.6); look_at(&mut s, [0.5, y as f64 - 0.5, 6.5]); right(&mut s); let l = s.line.expect("relancée");
    while s.elapsed < l.bite + 0.3 { s.tick(DT, none(), Actions::default()); } assert!(s.notes.iter().any(|n| n.starts_with("Ça mord")) || s.line.is_some_and(|l| l.told));
    right(&mut s); assert_eq!(s.count(l.fish), 1, "poisson pris au bon moment");
    // le poisson se cuit au feu
    let i = recipes().iter().position(|r| r.out == "cooked_fish").unwrap(); s.world.edit(2, y, 0, B::CAMPFIRE); s.fires.insert([2, y, 0], 600.0); s.craft(i).unwrap(); assert_eq!(s.count("cooked_fish"), 1);
}

#[test]
fn racines_semees_et_sauvages() {
    let (mut s, y) = site(); let mut e = Vec::new(); for x in -2..=2 { e.push((x, y - 1, 3, B::FARMLAND)); } s.world.edit_many(&e);
    s.inv[0] = Some(Stack::new("carrot", Some(3))); s.inv[1] = Some(Stack::new("potato", Some(3))); s.sel = 0;
    look_at(&mut s, [0.5, y as f64, 3.5]); right(&mut s); idle(&mut s, 0.5); s.sel = 1; look_at(&mut s, [1.5, y as f64, 3.5]); right(&mut s);
    assert_eq!((s.world.get(0, y, 3), s.world.get(1, y, 3)), (Some(B::CARROT_0), Some(B::POTATO_0)), "semées");
    s.pass_hours(200.0); idle(&mut s, 2.5); assert_eq!((s.world.get(0, y, 3), s.world.get(1, y, 3)), (Some(B::CARROT_3), Some(B::POTATO_3)), "mûres");
    let before = s.count("carrot"); s.sel = 5; look_at(&mut s, [0.5, y as f64 + 0.2, 3.5]); let mut k = 0; while s.world.get(0, y, 3) == Some(B::CARROT_3) && k < 600 { s.tick(DT, none(), Actions { mine: true, place: false, ..Default::default() }); k += 1; } s.collect_loose(); assert!(s.count("carrot") >= before + 3, "récolte : plus de carottes qu'on n'en a semé");
    // des racines sauvages poussent dans le monde
    let g = alw_core::gen::WorldGen::new(20261104); let mut n = 0; for cz in -6..6 { for cx in -6..6 { n += g.gen_chunk(cx, cz).iter().filter(|b| **b == B::CARROT_3 || **b == B::POTATO_3).count(); } } assert!(n > 3, "racines sauvages : {n}");
}

#[test]
fn ours_bouquetin_et_lezard() {
    // L'ours défend son coin : à quatre pas, il charge et frappe fort, de jour comme de nuit.
    let (mut s, y) = site(); s.hour = 12.0; s.wild.put(Kind::Bear, [0.5, y as f64, 4.5], 3.14, false); for _ in 0..(4.0 / DT) as usize { s.tick(DT, none(), Actions::default()); s.wild.all.retain(|c| c.kind == Kind::Bear); }
    assert!(s.vit[0] <= 88.0, "l'ours frappe : vie {}", s.vit[0]);
    // Le bouquetin file dès qu'on l'approche ; le lézard détale au dernier moment.
    let (mut s, y) = site(); s.hour = 12.0; s.wild.put(Kind::Ibex, [0.5, y as f64, 6.0], 0.0, false); idle(&mut s, 2.0);
    let d = s.wild.all.iter().find(|c| c.kind == Kind::Ibex).map_or(99.0, |c| (c.p[0] - 0.5).hypot(c.p[2] - 0.5)); assert!(d > 8.0, "le bouquetin s'est éloigné : {d}");
    let (mut s, y) = site(); s.hour = 12.0; s.wild.put(Kind::Lizard, [0.5, y as f64, 1.8], 0.0, false); s.inv[0] = Some(Stack::new("sword", None)); s.sel = 0;
    look_at(&mut s, [0.5, y as f64 + 0.05, 1.8]); assert!(matches!(s.prey, Some(Prey::Wild(_))), "le lézard se vise");
    s.tick(DT, none(), Actions { mine: true, ..Default::default() }); assert!(!s.wild.all.iter().any(|c| c.kind == Kind::Lizard), "un coup d'épée suffit"); s.collect_loose(); assert_eq!(s.count("raw_meat"), 1);
    assert!(alw_core::outdoors::wild_box(Kind::Bear).is_some() && !alw_core::outdoors::wild_loot(Kind::Bear).is_empty());
}
