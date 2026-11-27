//! Le destin des villages : une fortune qui bouge chaque jour (routes, guerre, alliances, brigands, réputation, dons, hasard) ;
//! en déclin ou en ruine, des maisons se vident (portes arrachées, vitres brisées, plus personne) et le marché s'amenuise ;
//! prospère ou ville, les prix baissent et la ville s'entoure d'un rempart ; quand la fortune remonte, on revient.
use alw_core::destin::Stage;
use alw_core::factions::Faction;
use alw_core::{blocks::BLOCKS, gen::WorldGen, player::{Input, Voxels}, session::*, stream::Preset, village::Village};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, n: usize) { for _ in 0..n { s.tick(0.05, Input::default(), Actions::default()); } }
fn at_village() -> (Session, Village) { at(|_| true) }
fn at(ok: impl Fn(&Village) -> bool) -> (Session, Village) {
    let v = WorldGen::new(SEED).villages_near(0.0, 0.0, 2000.0).into_iter().find(|v| ok(v)).unwrap();
    let mut s = Session::new(SEED, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [v.c[0] as f64 + 0.5, v.c[1] as f64 + 1.0, v.c[2] as f64 + 3.5]); ready(&mut s); s.hour = 12.0; idle(&mut s, 40);
    s.set_creative(false); s.wild.all.clear(); s.horde.foes.clear(); (s, v)
}
fn count(s: &Session, v: &Village, i: usize, f: impl Fn(&str) -> bool) -> usize { let a = v.houses[i].area; let mut k = 0; for x in a[0]..=a[2] { for z in a[1]..=a[3] { for y in v.c[1] - 6..v.c[1] + 18 { if s.world.get(x, y, z).is_some_and(|b| f(BLOCKS[b as usize].name)) { k += 1; } } } } k }

#[test]
fn les_etapes() {
    assert_eq!(Stage::of(-70.0, false), Stage::Ruine); assert_eq!(Stage::of(-30.0, false), Stage::Declin); assert_eq!(Stage::of(0.0, false), Stage::Stable); assert_eq!(Stage::of(50.0, false), Stage::Prospere); assert_eq!(Stage::of(80.0, false), Stage::Ville);
    assert_eq!(Stage::of(80.0, true), Stage::Prospere, "un hameau ne devient pas une ville"); assert_eq!(Stage::Ville.price(10), 8); assert_eq!(Stage::Prospere.price(10), 9); assert_eq!(Stage::Stable.price(10), 10); assert_eq!(Stage::Ville.price(1), 1);
    let v = WorldGen::new(SEED).villages_near(0.0, 0.0, 2000.0).into_iter().next().unwrap();
    let d = Session::empty_houses(&v, Stage::Declin); let r = Session::empty_houses(&v, Stage::Ruine); println!("{} : {} bâtiments ; en déclin {d:?} ; en ruine {r:?}", v.name, v.houses.len());
    assert!(d.iter().all(|i| r.contains(i)), "la ruine prolonge le déclin"); assert!(d.iter().all(|&i| v.houses[i].craft.is_none()), "en déclin, les artisans restent"); assert!(r.len() > d.len() && r.len() < v.houses.len());
}

#[test]
fn un_village_decline_puis_se_releve() {
    let (mut s, v) = at_village(); let folk0 = s.town.folk.iter().filter(|f| f.v == v.id).count(); let trade0 = s.offers(&v).len(); println!("{} : {} habitants, {} marchandises, fortune {:.0} ({})", v.name, folk0, trade0, s.fortune(&v), s.stage(&v).label());
    // La guerre longue et les brigands l'appauvrissent : il décline.
    let f = Faction::of_kind(v.kind); for g in alw_core::factions::ALL { if g != f { let (i, j) = (f as usize, g as usize); s.politics.rel[i][j] = -80.0; s.politics.rel[j][i] = -80.0; s.politics.war[i][j] = true; s.politics.war[j][i] = true; } }
    s.town.fortune.insert(v.id, -15.0); s.pass_hours(24.0 * 20.0); idle(&mut s, 30); ready(&mut s); idle(&mut s, 10);
    println!("après vingt jours de guerre : fortune {:.0} ({}) ; notes {:?}", s.fortune(&v), s.stage(&v).label(), s.notes.iter().filter(|n| n.contains(&v.name)).collect::<Vec<_>>());
    assert!(s.stage(&v) <= Stage::Declin, "il décline");
    let st = s.stage(&v); let gone = Session::empty_houses(&v, st); assert!(!gone.is_empty()); assert_eq!(s.town.empty.get(&v.id), Some(&gone), "les maisons sont vidées");
    let i = gone[0]; println!("maison {i} : portes {}, fenêtres {}, toiles {}", count(&s, &v, i, |n| n.starts_with("door")), count(&s, &v, i, |n| n.starts_with("window")), count(&s, &v, i, |n| n == "cobweb"));
    assert_eq!(count(&s, &v, i, |n| n.starts_with("door") || n.starts_with("window")), 0, "portes arrachées, vitres brisées"); assert!(count(&s, &v, i, |n| n == "cobweb") > 0, "des toiles d'araignée");
    let folk = s.town.folk.iter().filter(|f| f.v == v.id).count(); assert!(folk < folk0 && s.town.folk.iter().filter(|f| f.v == v.id).all(|f| !gone.contains(&f.house)), "plus personne dans les maisons vides ({folk0} → {folk})");
    assert!(s.offers(&v).len() < trade0, "le marché s'amenuise");
    // Tout se garde avec la partie.
    let t = s.save_text(); let mut l = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 60.0, 0.5]); l.load_text(&t).unwrap(); assert_eq!(l.town.empty.get(&v.id), Some(&gone)); assert!((l.fortune(&v) - s.fortune(&v)).abs() < 0.06);
    // La paix revient, le joueur fait des dons : le village se relève, les maisons sont rebâties, les familles reviennent.
    for g in alw_core::factions::ALL { if g != f { let (i, j) = (f as usize, g as usize); s.politics.rel[i][j] = 0.0; s.politics.rel[j][i] = 0.0; s.politics.war[i][j] = false; s.politics.war[j][i] = false; } }
    s.town.fortune.insert(v.id, 10.0); idle(&mut s, 30); println!("relève : {:?} ; ici {} ; centre {:?}", s.town.empty.get(&v.id), s.town.here.iter().any(|h| h.id == v.id), s.world.get(v.c[0], v.c[1], v.c[2])); assert_eq!(s.stage(&v), Stage::Stable); assert!(s.town.empty.get(&v.id).is_none_or(|e| e.is_empty()), "plus de maison vide");
    println!("relevée : portes {}, fenêtres {}, toiles {} ; {} habitants", count(&s, &v, i, |n| n.starts_with("door")), count(&s, &v, i, |n| n.starts_with("window")), count(&s, &v, i, |n| n == "cobweb"), s.town.folk.iter().filter(|f| f.v == v.id).count());
    assert!(count(&s, &v, i, |n| n.starts_with("door")) > 0 && count(&s, &v, i, |n| n == "cobweb") == 0, "la maison relevée"); assert_eq!(s.town.folk.iter().filter(|f| f.v == v.id).count(), folk0, "tout le monde est revenu");
}

#[test]
fn un_village_devient_une_ville() {
    let (mut s, v) = at(|v| v.ident.is_some_and(|i| i.size != alw_core::bourg::Size::Hamlet) && v.kind != "port"); println!("{} ({:?})", v.name, v.ident.map(|i| i.size));
    let base: Vec<u32> = s.offers(&v).iter().map(|o| o.give.1).collect();
    // Un don au chef du village : sa fortune monte d'un coup.
    let k = s.town.folk.iter().position(|f| f.v == v.id && f.job == 9).expect("le chef du village"); s.add_item(alw_core::economie::COIN, 30); let f0 = s.fortune(&v);
    s.talk_to(k); let d = s.dialog().unwrap(); let i = d.options.iter().position(|o| o.0.starts_with("Faire un don")).expect("l'option du don"); s.choose(i); println!("« {} »", s.dialog().unwrap().said);
    assert!((s.fortune(&v) - f0 - alw_core::destin::GIFT_FORTUNE).abs() < 1e-6); s.town.talk = None;
    // Prospère, puis ville : les prix baissent, le rempart se dresse.
    s.town.fortune.insert(v.id, 90.0); idle(&mut s, 30); assert_eq!(s.stage(&v), Stage::Ville); assert!(s.town.walled.contains(&v.id), "le rempart est élevé");
    let cheap: Vec<u32> = s.offers(&v).iter().map(|o| o.give.1).collect(); assert!(cheap.iter().zip(&base).all(|(a, b)| a <= b) && cheap.iter().zip(&base).any(|(a, b)| a < b), "les prix baissent : {base:?} → {cheap:?}");
    let mut wall = 0; for x in v.c[0] - 70..=v.c[0] + 70 { for z in v.c[2] - 70..=v.c[2] + 70 { for y in v.c[1] - 10..v.c[1] + 20 { if s.world.get(x, y, z) == alw_core::blocks::by_name("stone_bricks") { wall += 1; } } } }
    println!("rempart : {wall} pierres de taille"); assert!(wall > 150, "un vrai rempart");
}
