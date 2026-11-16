//! Les remparts du village pilote : la muraille, choisie chez le maire et payée (pièces comprises), se pose d'un coup autour du
//! village, avec des portes sur ses chemins ; les tours viennent dessus ; les maisons suivantes se bâtissent dedans.
//! Et la caisse : le jour de marché, des marchands de passage achètent le surplus.
use alw_core::{blocks::*, economie::*, gen::WorldGen, patrons::Kind, player::{Input, Voxels}, remparts, session::*, stream::Preset};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, n: usize) { for _ in 0..n { s.tick(0.05, Input::default(), Actions::default()); } }
fn name(s: &Session, x: i32, y: i32, z: i32) -> &'static str { s.world.get(x, y, z).map_or("?", |b| BLOCKS[b as usize].name) }

#[test]
fn la_muraille_et_ses_tours() {
    let mut s = Session::new(SEED, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.third = false; idle(&mut s, 5);
    let id = s.econ.as_ref().unwrap().id; let v = WorldGen::new(SEED).site(id.0, id.1).unwrap().v.clone();
    s.set_creative(true); s.player.p = [v.c[0] as f64 + 0.5, v.c[1] as f64 + 2.0, v.c[2] as f64 + 0.5]; ready(&mut s); idle(&mut s, 40); s.hour = 10.0;
    // chez le maire : la muraille est proposée, avec son prix en pièces
    let e = s.econ.clone().unwrap(); let c = e.census(&v); assert!(e.options(&c).contains(&Kind::Muraille), "{:?}", e.options(&c)); assert!(!e.options(&c).contains(&Kind::TourDefense), "pas de tour sans muraille");
    assert!(project(Kind::Muraille).cost.iter().any(|x| x.0 == COIN), "les maçons se paient");
    { let e = s.econ.as_mut().unwrap(); e.project = Some((Kind::Muraille, true)); for (k, n) in [("cobblestone", 380.0), ("planks", 120.0), ("oak_log", 60.0), ("iron_ingot", 10.0), (COIN, 60.0)] { e.stock.insert(k, n); } }
    idle(&mut s, 10); let e = s.econ.clone().unwrap(); println!("{:?}", e.log.last());
    let (c0, r) = remparts::ring(&e).expect("la muraille est debout"); assert_eq!(e.have(COIN), 30.0, "trente pièces aux maçons");
    println!("centre {c0:?} rayon {r} (village r {})", v.r);
    // elle fait le tour : de la pierre à hauteur d'homme presque partout, des portes là où passent les chemins
    let cells = remparts::cells(c0, r); let mut open = 0; let mut posts = 0; let wg = WorldGen::new(SEED);
    let built = |n: &str| matches!(n, "stone_bricks" | "cobblestone" | "planks" | "plaster" | "fence" | "iron_bars" | "chiseled_stone") || n.starts_with("oak_log") || n.starts_with("roof_tiles");
    for &(x, z, _) in &cells { let h = wg.column(x as f64, z as f64).h as i32; let n = (h - 8..h + 20).filter(|&y| built(name(&s, x, y, z))).count(); if n < 4 { open += 1; if open < 6 { println!("autre {x},{z} : {n} h{h} {:?}", (h - 4..h + 8).map(|y| name(&s, x, y, z)).collect::<Vec<_>>()); } }
        if (h - 8..h + 20).any(|y| name(&s, x, y, z) == "oak_log_c") { posts += 1; } }
    let gates = s.rampart_gates(c0, r); println!("{} colonnes : {open} sans muraille ; {} portes fortifiées {:?} ; {posts} colonnes à poteau", cells.len(), gates.len(), gates);
    assert_eq!(open, 0, "la muraille fait tout le tour, sans trou, rivière comprise"); assert!(!gates.is_empty(), "au moins une porte"); assert!(posts > 20, "les poteaux de chêne");
    for &(x, y, z) in &gates { let mut bars = false; let mut flags = 0; let mut air = 0; for dx in -6..=6 { for dz in -6..=6 { for yy in y..y + 16 { let n = name(&s, x + dx, yy, z + dz); bars |= n == "iron_bars"; flags += (n == "flag") as i32; } } }
        for yy in y + 1..=y + 3 { air += (name(&s, x, yy, z) == "air") as i32; } println!("porte en {x},{y},{z} : herse {bars}, {flags} drapeaux, passage {air}/3"); assert!(bars && flags >= 2 && air == 3, "la porte fortifiée"); }
    // tout le village est dedans
    for h in &v.houses { let m = [(h.area[0] + h.area[2]) / 2, (h.area[1] + h.area[3]) / 2]; assert!(((m[0] - c0[0]) as f64).hypot((m[1] - c0[1]) as f64) < r - 3.0, "{:?} dedans", h.area); }
    // les tours, maintenant proposées ; la première se pose sur la muraille
    let c = e.census(&v); assert!(e.options(&c).contains(&Kind::TourDefense));
    // chez le maire : neuf réponses au plus par page ; la tour se trouve, au besoin sur la page suivante
    let k = s.town.folk.iter().position(|f| f.v == v.id && f.job == 9).expect("un maire"); s.talk_to(k);
    let i = s.dialog().unwrap().options.iter().position(|o| o.0.starts_with("Les projets")).unwrap(); s.choose(i);
    let mut seen = Vec::new(); for _ in 0..4 { let d = s.dialog().unwrap(); assert!(d.options.len() <= 9, "{} réponses", d.options.len()); seen.extend(d.options.iter().map(|o| o.0.clone()));
        match d.options.iter().position(|o| o.0.starts_with("Autres projets")) { Some(n) => s.choose(n), None => break } }
    println!("{} projets : {}", e.options(&c).len(), seen.iter().map(|x| x.split(" · ").next().unwrap()).collect::<Vec<_>>().join(", "));
    assert!(seen.iter().any(|o| o.contains("Tour de guet")), "la tour se trouve"); s.close_talk();
    { let e = s.econ.as_mut().unwrap(); e.project = Some((Kind::TourDefense, true)); }
    idle(&mut s, 10); let e = s.econ.clone().unwrap(); let t = e.extra.iter().find(|x| x.kind == Kind::TourDefense).copied().expect("une tour");
    let (tx, tz, g) = (t.lot.o.0, t.lot.o.1, t.y); let d = ((tx - c0[0]) as f64).hypot((tz - c0[1]) as f64); assert!((d - r).abs() < 1.5, "sur la muraille : {d} / {r}");
    let mut walls = 0; let mut ladder = false; let mut door = false; let mut flag = false; let mut tiles = 0;
    for dx in -4..=4 { for dz in -4..=4 { for y in g + 12..=g + 17 { tiles += name(&s, tx + dx, y, tz + dz).starts_with("roof_tiles") as i32; } } }
    for dx in -3..=3 { for dz in -3..=3 { for y in g + 1..=g + 17 { let n = name(&s, tx + dx, y, tz + dz); if n == "stone_bricks" || n == "cobblestone" { walls += 1; } ladder |= n.starts_with("ladder"); door |= n.starts_with("door"); flag |= n == "flag"; } } }
    println!("tour en {tx},{g},{tz} : {walls} blocs de pierre, échelle {ladder}, porte {door}, drapeau {flag}, {tiles} tuiles"); assert!(walls > 90 && ladder && door && flag && tiles > 40);
    assert!(s.village_report().contains("Remparts : la muraille, 1 tour"), "{}", s.village_report());
    // sauvegardée, rechargée : le tracé est gardé
    let save = s.save_text(); let mut o = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); o.load_text(&save).unwrap(); assert_eq!(remparts::ring(o.econ.as_ref().unwrap()), Some((c0, r)));
    // la maison suivante se bâtit dedans
    { let e = s.econ.as_mut().unwrap(); e.project = Some((Kind::Maison, true)); for (k, n) in [("cobblestone", 80.0), ("planks", 80.0), ("oak_log", 30.0)] { e.stock.insert(k, n); } }
    idle(&mut s, 10); let e = s.econ.clone().unwrap(); if let Some(h) = e.extra.iter().rev().find(|x| x.kind == Kind::Maison) { let a = area_of(h); for (x, z) in [(a[0], a[1]), (a[2], a[3])] { assert!(((x - c0[0]) as f64).hypot((z - c0[1]) as f64) < r - 5.0, "la maison est dedans"); } } else { println!("pas de terrain pour une maison (le village est plein) : {:?}", e.due); }
}

/// Le jour de marché : une fois la semaine, des marchands de passage achètent le surplus ; le reste de la semaine, rien.
#[test]
fn le_jour_de_marche_remplit_la_caisse() {
    let g = WorldGen::new(SEED); let v = g.villages_near(-38.0, 0.0, 1400.0).into_iter().find(|v| v.ident.is_some() && !v.works.is_empty() && ambition(v) > 0).unwrap();
    let mut e = Econ::found(&v, 0.0); e.extra.push(Extra { lot: alw_core::batisse::Lot { o: (0, 0), u: (1, 0), w: (0, 1) }, y: 0, kind: Kind::Entrepot }); let c = e.census(&v);
    for (k, n) in [("bread", 300.0), ("wheat", 200.0), ("cobblestone", 150.0)] { e.stock.insert(k, n); } let coins = e.have(COIN);
    for d in 0..3 { e.step(&c, 12, d); } assert_eq!(e.have(COIN), coins, "pas de marché ces jours-là");
    e.step(&c, 12, 3); println!("{:?}", e.log.last()); assert!(e.have(COIN) > coins, "la caisse se remplit"); assert!(e.have("bread") < 300.0 && e.have("wheat") < 200.0, "le surplus est parti");
    assert!(e.log.last().is_some_and(|l| l.1.contains("jour de marché")));
    // ce qu'il garde pour ses vivres et son chantier ne part pas
    let k = e.keep("cobblestone", &c); assert!(e.have("cobblestone") >= k.min(150.0), "{} gardés sur {k}", e.have("cobblestone"));
}
