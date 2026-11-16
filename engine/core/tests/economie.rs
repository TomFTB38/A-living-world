//! Le village pilote : des stocks réels, une population qui mange, des métiers qui produisent, une réserve qu'on ouvre,
//! une maison bâtie d'un coup quand les matériaux sont réunis, et le temps qui passe en l'absence du joueur.
use alw_core::{blocks::*, economie::*, gen::WorldGen, player::{Input, Voxels}, session::*, stream::Preset};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..9000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, n: usize) { for _ in 0..n { s.tick(0.05, Input::default(), Actions::default()); } }
fn start() -> Session { let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.third = false; idle(&mut s, 5); s }
/// Le village pilote de cette graine.
fn pilot(s: &Session) -> alw_core::village::Village { let id = s.econ.as_ref().expect("un village pilote").id; WorldGen::new(SEED).site(id.0, id.1).unwrap().v.clone() }
fn go(s: &mut Session, v: &alw_core::village::Village) { s.set_creative(true); s.player.p = [v.c[0] as f64 + 0.5, v.c[1] as f64 + 2.0, v.c[2] as f64 + 0.5]; ready(s); idle(s, 40); }

#[test]
fn le_village_produit_mange_et_tient_ses_comptes() {
    let s = start(); let v = pilot(&s); let mut e = s.econ.clone().unwrap(); let c = e.census(&v);
    println!("{} : {:?}\n{}", v.name, c, e.report(&v, 0).join("\n"));
    assert!(v.ident.is_some() && c.pop >= 6 && c.cutters >= 1 && c.miners >= 1, "un vrai village médiéval, avec du monde");
    // dix jours : il mange tous les soirs à sa faim, et ses réserves montent
    let (w0, b0) = (e.have("oak_log") + e.have("planks"), e.have("cobblestone"));
    for h in 0..240u32 { e.step(&c, h % 24, h / 24); }
    assert!(e.fed > 0.99 && e.lean == 0, "nourri : {}", e.fed); assert!(e.have("oak_log") + e.have("planks") > w0 && e.have("cobblestone") > b0, "le bois et la pierre rentrent"); assert!(e.have("bread") > 1.0, "le pain sort du four");
    // la nuit, rien ne se produit
    let snap = e.stock.clone(); for h in [0, 1, 2, 3, 22, 23] { e.step(&c, h, 11); } assert_eq!(snap, e.stock, "on dort");
    // on vide les greniers : le soir même, la disette ; le lendemain, tous les bras sont aux champs
    for k in ["bread", "flour", "wheat", "cooked_meat"] { e.stock.insert(k, 0.0); } e.step(&c, 19, 12); assert!(e.fed < 0.2 && e.lean == 1); assert!(e.log.last().is_some_and(|l| l.1.contains("grenier")), "c'est noté au journal");
    let mut calm = e.clone(); calm.stock.insert("bread", c.pop as f64 * 3.0); e.step(&c, 9, 13); calm.step(&c, 9, 13); assert!(e.have("wheat") > calm.have("wheat") * 1.3, "la disette presse les champs : {} contre {}", e.have("wheat"), calm.have("wheat"));
    // les greniers pleins, on s'arrête de produire
    e.stock.insert("cobblestone", 400.0); e.step(&c, 9, 14); assert_eq!(e.have("cobblestone"), 400.0);
    // la maison : il faut les matériaux, quatre jours de vivres et que personne n'ait eu faim
    let mut e = Econ::found(&v, 0.0); for (k, n) in HOUSE_COST { e.stock.insert(k, n + 5.0); } e.stock.insert("bread", c.pop as f64 * 5.0); let c = e.census(&v); assert!(c.room >= 2);
    e.step(&c, 8, 1); assert_eq!(e.due, 1, "payée"); for (k, _) in HOUSE_COST { assert!(e.have(k) <= 9.0, "{k} dépensé"); }
    let mut poor = Econ::found(&v, 0.0); for (k, n) in HOUSE_COST { poor.stock.insert(k, n + 5.0); } for k in ["bread", "flour", "wheat", "cooked_meat"] { poor.stock.insert(k, 1.0); } poor.step(&c, 8, 1); assert_eq!(poor.due, 0, "pas de chantier le ventre vide");
}

#[test]
fn le_temps_passe_sans_le_joueur_et_se_sauvegarde() {
    let mut s = start(); let v = pilot(&s); let before = s.econ.clone().unwrap();
    assert!(((v.c[0] as f64 + 38.0).hypot(v.c[2] as f64)) > 60.0, "le pilote n'est pas sous nos pieds : on le simule de loin");
    s.pass_hours(24.0 * 5.0); idle(&mut s, 3); let e = s.econ.clone().unwrap();
    assert!((e.t - before.t - 120.0).abs() < 1.5, "cinq jours rattrapés : {} → {}", before.t, e.t); assert!(e.stock != before.stock, "les stocks ont bougé"); assert!(e.fed > 0.9);
    let txt = s.command("/village"); println!("{txt}"); assert!(txt.contains(&v.name) && txt.contains("Stocks") && txt.contains("Vivres"));
    // sauvegarde et reprise : mêmes comptes
    let save = s.save_text(); let mut o = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); o.load_text(&save).unwrap(); let r = o.econ.clone().expect("l'économie est sauvegardée");
    assert_eq!(r.id, e.id); assert!((r.t - e.t).abs() < 1e-9); for (k, n) in &e.stock { assert!((r.have(k) - n).abs() < 1e-6, "{k}"); } assert_eq!(r.log.len(), e.log.len());
}

#[test]
fn la_reserve_s_ouvre_et_la_maison_se_batit_d_un_coup() {
    let mut s = start(); let v = pilot(&s); go(&mut s, &v); let e = s.econ.clone().unwrap();
    assert!(!e.store.is_empty(), "la réserve du village a ses coffres"); let c0 = e.store[0];
    // on ouvre : le coffre montre les stocks, à l'unité près
    s.open_chest(c0); let count = |s: &Session, id: &str| -> u32 { s.econ.as_ref().unwrap().store.iter().filter_map(|c| s.chests.get(c)).flatten().flatten().filter(|st| st.item == id).map(|st| st.n.unwrap_or(1)).sum() };
    for (id, _) in GOODS { assert_eq!(count(&s, id), s.econ.as_ref().unwrap().have(id).floor() as u32, "{id} dans la réserve"); }
    // on y prend dix pains : ils manquent au village, qui le remarque
    let bread = s.econ.as_ref().unwrap().have("bread"); assert!(bread >= 10.0); let rep = s.town.rep.get(&v.id).copied().unwrap_or(0);
    let mut left = 10u32; for c in s.econ.as_ref().unwrap().store.clone() { for st in s.chests.get_mut(&c).unwrap().iter_mut() { if left > 0 { if let Some(x) = st.as_mut().filter(|x| x.item == "bread") { let n = x.n.unwrap(); let t = n.min(left); left -= t; if n == t { *st = None; } else { x.n = Some(n - t); } } } } }
    s.inv_open = false; s.chest = None; s.chest2 = None; idle(&mut s, 3);
    assert!((s.econ.as_ref().unwrap().have("bread") - (bread - 10.0)).abs() < 1.0, "dix pains de moins"); assert_eq!(s.town.rep.get(&v.id).copied().unwrap_or(0), rep - 1, "le village l'a vu");
    // on lui donne de quoi bâtir et manger : au matin, la maison est là, d'un coup, et deux habitants de plus
    let folk = s.town.folk.iter().filter(|f| f.v == v.id).count(); let houses = s.town.here.iter().find(|h| h.id == v.id).unwrap().houses.len();
    { let e = s.econ.as_mut().unwrap(); for (k, n) in HOUSE_COST { e.stock.insert(k, n + 10.0); } e.stock.insert("bread", 200.0); e.fed = 1.0; }
    s.hour = 6.5; s.notes.clear(); s.pass_hours(2.0); idle(&mut s, 5);
    let e = s.econ.clone().unwrap(); assert_eq!((e.extra.len(), e.due), (1, 0), "bâtie : {:?}", e.log); assert!(s.notes.iter().any(|n| n.contains("nouvelle maison")), "{:?}", s.notes);
    assert_eq!(s.town.here.iter().find(|h| h.id == v.id).unwrap().houses.len(), houses + 1); assert_eq!(s.town.folk.iter().filter(|f| f.v == v.id).count(), folk + 2, "deux nouveaux habitants");
    // elle est vraiment dans le monde : des murs, un toit, une porte, à l'écart des autres bâtiments
    let x = e.extra[0]; let mut solid = 0; let mut door = false; for u in -1..10 { for w in -2..9 { for y in x.y..x.y + 9 { let c = x.lot.cell(u, y, w); if let Some(b) = s.world.get(c[0], c[1], c[2]) { if is_solid(b) { solid += 1; } if BLOCKS[b as usize].name.starts_with("door") { door = true; } } } } }
    assert!(solid > 120 && door, "{solid} blocs pleins, porte : {door}");
    // sauvegardée, rechargée : la maison compte toujours, ses habitants aussi
    let save = s.save_text(); let mut o = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); o.load_text(&save).unwrap(); ready(&mut o); idle(&mut o, 40);
    assert_eq!(o.econ.as_ref().unwrap().extra.len(), 1); assert_eq!(o.town.here.iter().find(|h| h.id == v.id).expect("le village est là").houses.len(), houses + 1); assert_eq!(o.town.folk.iter().filter(|f| f.v == v.id).count(), folk + 2);
}

/// Le matin au village pilote : le fermier moissonne et ressème, le bûcheron abat un arbre et replante, le mineur taille la
/// roche ; chacun rapporte son ouvrage à la réserve, où il compte. Rien de tout cela ne sort plus du calcul tant que le village est là.
#[test]
fn les_gens_de_metier_travaillent_pour_de_bon() {
    let mut s = start(); let v = pilot(&s); go(&mut s, &v); s.hour = 8.0; idle(&mut s, 20);
    let jobs: Vec<u8> = s.town.folk.iter().filter(|f| f.v == v.id).map(|f| f.job).collect(); println!("métiers : {jobs:?}");
    assert!(jobs.contains(&1) && jobs.contains(&2), "un fermier, un bûcheron : {jobs:?}");
    let e0 = s.econ.clone().unwrap(); assert!(e0.live[0] && e0.live[1], "champs et bois au geste : {:?}", e0.live); println!("vivant : {:?}", e0.live);
    let (logs0, crops0) = (e0.have("oak_log"), e0.have("wheat") + e0.have("carrot") + e0.have("potato"));
    let (mut felled, mut reaped, mut worked) = (false, false, [false; 4]); let saplings0 = e0.saplings.len();
    for i in 0..(200.0 / 0.05) as usize { s.tick(0.05, Input::default(), Actions::default()); if i % 20 == 0 { s.hour = s.hour.min(11.5); }
        for f in s.town.folk.iter().filter(|f| f.v == v.id) { if f.working && f.task.is_some() { worked[f.job as usize] = true; } if f.job == 1 && !f.carry.is_empty() { reaped = true; } if f.job == 2 && f.carry.iter().any(|c| c.0 == "oak_log") { felled = true; } } }
    let e = s.econ.clone().unwrap(); println!("après : bois {} → {}, cultures {} → {}, plants {}, travail {worked:?}", logs0, e.have("oak_log"), crops0, e.have("wheat") + e.have("carrot") + e.have("potato"), e.saplings.len());
    assert!(worked[1] && worked[2], "on les a vus au travail : {worked:?}"); assert!(reaped, "le fermier a moissonné"); assert!(felled, "le bûcheron a abattu un arbre");
    assert!(e.saplings.len() > saplings0, "et replanté"); let p = e.saplings.last().unwrap().0; assert_eq!(s.world.get(p[0], p[1], p[2]), Some(B::SAPLING));
    assert!(e.have("oak_log") > logs0 || s.town.folk.iter().any(|f| f.job == 2 && !f.carry.is_empty()), "le bois arrive à la réserve (ou est en chemin)");
    // le plant devient un jeune chêne au bout de trois jours
    s.pass_hours(80.0); idle(&mut s, 3); s.econ.as_mut().unwrap().scan_reset(); idle(&mut s, 2); assert!(s.world.get(p[0], p[1], p[2]).is_some_and(is_log), "le chêne a poussé : {:?}", s.world.get(p[0], p[1], p[2]).map(|b| BLOCKS[b as usize].name));
}
