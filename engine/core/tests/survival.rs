//! Survie : besoins, endurance, dégâts, mort et sac, nourriture, chasse, fabrication.
use alw_core::{blocks::*, player::*, session::*, stream::*, survival::*};
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn dry() -> [f64; 3] { let g = alw_core::gen::WorldGen::new(20261104); for k in 0..400 { let x = -38.5 + k as f64 * 3.0; let z = 0.5 + k as f64; let c = g.column(x, z); let flat = (0..8).all(|a| { let t = a as f64 * 0.785; let o = g.column(x + t.cos() * 8.0, z + t.sin() * 8.0); o.water == 0.0 && (o.h - c.h).abs() < 2.0 }); if c.water == 0.0 && c.bank == 0.0 && flat { return [x, c.h + 2.0, z]; } } panic!("aucun terrain plat") }
const DT: f64 = 1.0 / 60.0;
fn none() -> Input { Input::default() }
fn idle(s: &mut Session, sec: f64) { for _ in 0..(sec / DT) as usize { s.tick(DT, none(), Actions::default()); } }
fn session() -> Session { let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, dry()); s.give_kit(); ready(&mut s); idle(&mut s, 3.0); assert!(s.player.ground); s.vit = [100.0; 3]; s }
fn ground_y(s: &Session, x: f64, z: f64) -> f64 { let mut y = s.player.p[1] as i32 + 3; while y > 2 && !s.world.get(x.floor() as i32, y, z.floor() as i32).is_some_and(is_solid) { y -= 1; } y as f64 + 1.0 }

#[test]
fn fabriquer_de_l_arbre_a_la_pioche() {
    let mut s = session(); s.inv = [None; INV_SIZE]; s.add_item("oak_log", 1); s.add_item("birch_log", 1);
    let i = |id: &str| RECIPES.iter().position(|r| r.out == id).unwrap();
    assert_eq!(s.count("log"), 2, "deux essences, un seul ingrédient « bois »");
    s.craft(i("planks")).unwrap(); s.craft(i("planks")).unwrap(); assert_eq!(s.count("planks"), 8); assert_eq!(s.count("log"), 0);
    assert_eq!(s.craft(i("planks")), Err("Il manque des ingrédients"));
    s.craft(i("stick")).unwrap(); assert_eq!(s.count("stick"), 2); s.craft(i("workbench")).unwrap(); assert_eq!(s.count("workbench"), 1); assert_eq!(s.count("planks"), 3);
    s.add_item("stone", 2); s.add_item("granite", 1); s.add_item("fiber", 2);
    assert_eq!(s.craft(i("pickaxe")), Err("Il faut un établi à portée"));
    let p = s.player.p; let (x, z) = (p[0].floor() as i32 + 2, p[2].floor() as i32); let y = ground_y(&s, x as f64, z as f64) as i32; assert!(s.world.edit(x, y, z, B::WORKBENCH));
    assert!(s.stations().0); s.craft(i("pickaxe")).unwrap(); assert_eq!(s.count("rock"), 0, "pierre et granite comptent tous deux");
    assert_eq!(s.inv.iter().flatten().find(|t| t.item == "pickaxe").unwrap().n, None, "un outil ne s'empile pas");
    s.add_item("axe", 2); assert_eq!(s.inv.iter().flatten().filter(|t| t.item == "axe").count(), 2);
    // inventaire plein : rien n'est perdu
    s.inv = [Some(Stack { item: "dirt", n: Some(99) }); INV_SIZE]; s.inv[0] = Some(Stack { item: "oak_log", n: Some(1) }); s.inv[1] = Some(Stack { item: "planks", n: Some(99) }); let before = s.inv;
    assert!(s.craft(i("planks")).is_ok(), "le bois libère sa case pour les planches"); s.inv = before; s.inv[0] = Some(Stack { item: "oak_log", n: Some(2) });
    assert_eq!(s.craft(i("planks")), Err("L'inventaire est plein")); assert_eq!(s.inv, { let mut b = before; b[0] = Some(Stack { item: "oak_log", n: Some(2) }); b });
    for r in RECIPES { assert!(item(r.out).is_some(), "{}", r.out); for (k, _) in r.need { for m in members(k) { assert!(item(m).is_some(), "{m}"); } } }
    for id in ORDER { assert!(item(id).is_some(), "{id}"); }
}

#[test]
fn manger_et_boire() {
    let mut s = session(); s.inv = [None; INV_SIZE]; s.vit = [80.0, 40.0, 30.0];
    s.add_item("berries", 2); s.sel = 0; s.tick(DT, none(), Actions { mine: false, place: true });
    assert!(s.vit[1] > 48.0 && s.vit[2] > 34.0, "myrtilles : {:?}", s.vit); assert_eq!(s.inv[0].unwrap().n, Some(1));
    s.tick(DT, none(), Actions { mine: false, place: true }); assert_eq!(s.inv[0].unwrap().n, Some(1), "on ne mange pas deux fois d'affilée");
    s.inv[0] = Some(Stack { item: "raw_meat", n: Some(1) }); idle(&mut s, 1.0); let h = s.vit[0]; s.tick(DT, none(), Actions { mine: false, place: true }); assert!(s.vit[0] < h, "la viande crue rend malade"); assert!(s.inv[0].is_none());
    s.inv[0] = Some(Stack { item: "cooked_meat", n: Some(1) }); idle(&mut s, 1.0); let f = s.vit[1]; s.tick(DT, none(), Actions { mine: false, place: true }); assert!(s.vit[1] > f + 30.0);
    // Boire : une flaque devant soi, les mains vides
    let p = s.player.p; let (x, z) = (p[0].floor() as i32, p[2].floor() as i32 + 1); let y = p[1].floor() as i32 - 1; s.world.edit(x, y, z, B::WATER);
    s.inv[0] = None; s.player.yaw = 0.0; s.player.pitch = -0.9; s.vit[2] = 20.0; idle(&mut s, 1.0); s.player.pitch = -0.9; s.tick(DT, none(), Actions { mine: false, place: true });
    assert!(s.vit[2] > 30.0, "gorgée : {}", s.vit[2]);
    assert_eq!(food("berries").unwrap().hunger > 0.0, true); assert!(bonus_drop("blueberry").is_some());
}

#[test]
fn chute_noyade_faim() {
    let mut s = session(); s.third = false; let p = s.player.p;
    // petite chute : rien ; grande chute : des dégâts proportionnés
    s.player.p[1] += 2.5; idle(&mut s, 2.0); assert_eq!(s.vit[0], 100.0, "deux blocs et demi ne blessent pas");
    s.player.p = [p[0], p[1] + 14.0, p[2]]; idle(&mut s, 3.0); let h = s.vit[0]; assert!(h < 90.0 && h > 20.0, "quatorze blocs : {h}"); assert_eq!(s.cause, "Une chute");
    // noyade : la tête sous l'eau, plus de souffle
    s.vit = [100.0; 3]; let p = s.player.p; let (x, y, z) = (p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32); s.world.edit_many(&[(x, y, z, B::WATER), (x, y + 1, z, B::WATER), (x, y + 2, z, B::WATER)]);
    idle(&mut s, 3.0); assert!(s.air < 85.0, "le souffle baisse : {}", s.air); assert_eq!(s.vit[0], 100.0);
    s.air = 0.0; idle(&mut s, 2.1); assert!(s.vit[0] < 100.0, "noyade"); assert_eq!(s.cause, "La noyade");
    s.world.edit_many(&[(x, y, z, 0), (x, y + 1, z, 0), (x, y + 2, z, 0)]); idle(&mut s, 3.0); assert!(s.air > 99.0, "on reprend son souffle");
    // la faim tue lentement ; bien nourri, on guérit
    s.vit = [50.0, 0.0, 100.0]; idle(&mut s, 9.0); assert!(s.vit[0] < 50.0 && s.cause == "La faim");
    s.vit = [50.0, 100.0, 100.0]; idle(&mut s, 5.0); assert!(s.vit[0] > 55.0, "guérison : {}", s.vit[0]); assert!(s.vit[1] < 100.0, "la guérison coûte");
}

#[test]
fn endurance() {
    let mut s = session(); let run = Input { z: 1.0, sprint: true, ..none() };
    for _ in 0..900 { s.player.yaw += 0.03; s.tick(DT, run, Actions::default()); if s.winded { break; } } // en rond, pour rester dans la zone chargée assert!(s.winded, "dix secondes de course épuisent : {}", s.stamina);
    for _ in 0..30 { s.player.yaw += 0.03; s.tick(DT, run, Actions::default()); } assert!(s.player.speed < 5.0, "essoufflé, on marche : {}", s.player.speed);
    s.tick(DT, Input { roll: true, ..none() }, Actions::default()); assert!(!s.player.rolling(), "ni roulade");
    idle(&mut s, 3.0); assert!(!s.winded && s.stamina > 30.0); idle(&mut s, 4.0); assert_eq!(s.stamina, 100.0);
    s.vit[1] = 10.0; idle(&mut s, 0.1); assert!(s.stamina <= 50.0, "affamé, l'endurance plafonne");
}

#[test]
fn mort_sac_et_reapparition() {
    let mut s = session(); let n0 = s.inv.iter().flatten().count(); assert!(n0 > 0); let at = s.player.p;
    s.harm(250.0, "Un essai", false); let d = s.dead.clone().expect("mort"); assert_eq!(d.cause, "Un essai");
    assert!(s.inv.iter().all(|x| x.is_none())); assert_eq!(s.stashes.len(), 1); assert_eq!(s.stashes[0].items.len(), n0);
    let p = s.player.p; s.tick(DT, Input { z: 1.0, jump: true, ..none() }, Actions { mine: true, place: true }); assert!((s.player.p[0] - p[0]).abs() < 0.01 && (s.player.p[2] - p[2]).abs() < 0.01, "mort, on ne bouge plus");
    let md = alw_core::actors::Models::build(); assert!(s.draws(&md, &alw_core::view::identity()).len() > 1, "le sac se dessine");
    // la sauvegarde garde le sac
    let txt = s.save_text(); let mut o = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, 90.0, 0.5]); o.load_text(&txt).unwrap(); assert_eq!(o.stashes, s.stashes); assert!(o.dead.is_none(), "on recharge vivant"); assert_eq!(o.vit[0], 100.0);
    s.spawn = [at[0] + 20.0, at[1] + 2.0, at[2]]; s.respawn(); assert!(s.dead.is_none()); assert_eq!(s.vit, [100.0; 3]); assert!((s.player.p[0] - at[0] - 20.0).abs() < 0.01); assert!(!s.notes.is_empty());
    ready(&mut s); idle(&mut s, 2.0); assert_eq!(s.stashes.len(), 1, "le sac attend");
    s.player.p = [at[0] + 0.5, at[1] + 0.5, at[2]]; idle(&mut s, 1.0); assert!(s.stashes.is_empty(), "sac repris"); assert_eq!(s.inv.iter().flatten().count(), n0);
}

#[test]
fn chasser() {
    let mut s = session(); s.third = false; s.player.yaw = 0.0; s.player.pitch = 0.0; s.inv[0] = Some(Stack { item: "sword", n: None }); s.sel = 0;
    let p = s.player.p; let q = [p[0], ground_y(&s, p[0], p[2] + 2.0), p[2] + 2.0]; s.player.pitch = ((q[1] + 0.25 - s.player.eye()[1]) / 2.0).atan();
    s.wild.put(alw_core::wild::Kind::Hare, q, 0.0, false); s.tick(0.0, none(), Actions::default()); assert_eq!(s.prey, Some(Prey::Wild(s.wild.all.len() - 1)), "le lièvre est visé");
    let meat = s.count("raw_meat"); s.tick(DT, none(), Actions { mine: true, place: false });
    assert!(s.wild.all.iter().all(|c| c.kind != alw_core::wild::Kind::Hare || (c.p[0] - q[0]).abs() > 0.01), "un coup d'épée abat le lièvre"); assert_eq!(s.count("raw_meat"), meat + 2);
    // le cerf encaisse un coup et s'enfuit ; le poing ne l'abat pas
    idle(&mut s, 0.5); s.sel = 8; let q = [p[0], ground_y(&s, p[0], p[2] + 2.5), p[2] + 2.5]; s.fauna.put(q, 0.0); s.player.pitch = ((q[1] + 0.9 - s.player.eye()[1]) / 2.5).atan();
    s.tick(0.0, none(), Actions::default()); assert!(matches!(s.prey, Some(Prey::Deer(_))), "le cerf est visé"); let st = s.stamina;
    s.tick(DT, none(), Actions { mine: true, place: false }); let d = s.fauna.deer.last().unwrap(); assert_eq!(d.hp, DEER_HP - 2.0); assert_eq!(d.state, alw_core::fauna::DeerState::Flee); assert!(s.stamina < st, "frapper fatigue");
    assert_eq!(ray_box([0.0, 1.0, -5.0], [0.0, 0.0, 1.0], [0.0, 0.0, 0.0], 0.5, 2.0), Some(4.5)); assert_eq!(ray_box([3.0, 1.0, -5.0], [0.0, 0.0, 1.0], [0.0, 0.0, 0.0], 0.5, 2.0), None);
}

#[test]
fn feu_de_camp_et_etabli() {
    let m = alw_core::mesher::Mesher::new();
    for b in [B::CAMPFIRE, B::WORKBENCH] { let mut o = alw_core::mesher::VBuf::new(200); m.deco_model(&mut o, b, 0, 0, 0, 3, 5, 255.0, 0.0, 0); assert!(o.n > 40, "{} a un modèle", BLOCKS[b as usize].name); }
    assert!(emit(B::CAMPFIRE) > 10, "le feu éclaire"); assert!(is_solid(B::WORKBENCH) && !is_solid(B::CAMPFIRE), "l'établi arrête, le feu se traverse");
    // marcher dans le feu brûle ; à côté, on peut cuisiner sans se brûler
    let mut s = session(); let p = s.player.p; let (x, y, z) = (p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32);
    s.world.edit(x + 1, y, z, B::CAMPFIRE); s.player.p = [x as f64 + 0.5, p[1], z as f64 + 0.5]; idle(&mut s, 2.0); assert_eq!(s.vit[0], 100.0, "à côté du feu");
    assert!(s.stations().1); s.inv = [None; INV_SIZE]; s.add_item("raw_meat", 1); let i = RECIPES.iter().position(|r| r.out == "cooked_meat").unwrap(); s.craft(i).unwrap(); assert_eq!(s.count("cooked_meat"), 1);
    s.player.p = [x as f64 + 1.5, p[1], z as f64 + 0.5]; idle(&mut s, 1.0); assert!(s.vit[0] < 100.0 && s.cause == "Les flammes", "dans le feu");
}

#[test]
fn depart_mains_vides_et_ramassage() {
    let o = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, dry()); assert!(o.inv.iter().all(|x| x.is_none()), "on commence sans rien");
    let mut s = session(); s.inv = [None; INV_SIZE]; s.third = false; s.player.pitch = -0.7; s.player.yaw = 0.0; idle(&mut s, 0.2);
    let put = |s: &mut Session, b: u8| -> Target { s.player.pitch = -0.7; s.tick(0.0, none(), Actions::default()); let t = s.target.expect("sol visé"); let (x, y, z) = (t.x, t.y + 1, t.z); s.world.edit(x, y, z, b); s.tick(0.0, none(), Actions::default()); let t = s.target.unwrap(); assert_eq!(t.id, b, "on vise ce qu'on vient de poser"); t };
    let t = put(&mut s, B::TWIGS); s.tick(DT, none(), Actions { mine: false, place: true }); assert_eq!(s.world.get(t.x, t.y, t.z), Some(0)); assert_eq!(s.count("stick"), 1, "une branche, un bâton");
    idle(&mut s, 0.5); let t = put(&mut s, B::PEBBLES); s.tick(DT, none(), Actions { mine: false, place: true }); assert_eq!(s.world.get(t.x, t.y, t.z), Some(0)); assert_eq!(s.count("pebble"), 1, "un caillou");
    // myrtillier : cueilli au clic droit, dépouillé un jour, puis ses baies reviennent
    idle(&mut s, 0.5); let t = put(&mut s, B::BLUEBERRY); s.tick(DT, none(), Actions { mine: false, place: true }); assert_eq!(s.count("berries"), 3); assert_eq!(s.world.get(t.x, t.y, t.z), Some(B::BLUEBERRY_PICKED));
    idle(&mut s, 1.0); s.tick(DT, none(), Actions { mine: false, place: true }); assert_eq!(s.count("berries"), 3, "rien à cueillir sur un buisson dépouillé");
    s.pass_hours(30.0); idle(&mut s, 2.5); assert_eq!(s.world.get(t.x, t.y, t.z), Some(B::BLUEBERRY_PICKED), "pas encore");
    let txt = s.save_text(); assert!(txt.contains("regrow "));
    s.pass_hours(19.0); idle(&mut s, 2.5); assert_eq!(s.world.get(t.x, t.y, t.z), Some(B::BLUEBERRY), "deux jours plus tard, les baies sont revenues"); assert!(s.regrow.is_empty());
}

#[test]
fn durete_outils_de_pierre_fibre() {
    let mut s = session(); s.inv = [None; INV_SIZE]; s.third = false;
    // outils de pierre : à la main, avec branches, cailloux et fibre
    let i = |id: &str| RECIPES.iter().position(|r| r.out == id).unwrap();
    s.add_item("stick", 5); s.add_item("pebble", 8); s.add_item("fiber", 7); for t in ["stone_knife", "stone_axe", "stone_pickaxe"] { s.craft(i(t)).unwrap(); }
    assert_eq!(s.count("fiber"), 0); assert_eq!(s.count("pebble"), 0); assert_eq!(s.count("stick"), 0);
    let time = |s: &mut Session, b: u8, sel: Option<usize>| -> (f64, bool) {
        s.player.pitch = -0.7; s.player.yaw = 0.0; s.tick(0.0, none(), Actions::default()); let g = s.target.unwrap(); let (x, y, z) = (g.x, g.y + 1, g.z); s.world.edit(x, y, z, b); s.stamina = 100.0;
        s.sel = sel.unwrap_or(30); let before = s.inv.iter().flatten().map(|t| t.n.unwrap_or(1)).sum::<u32>(); let mut k = 0;
        while s.world.get(x, y, z) == Some(b) && k < 2000 { s.stamina = 100.0; s.tick(DT, none(), Actions { mine: true, place: false }); k += 1; }
        let after = s.inv.iter().flatten().map(|t| t.n.unwrap_or(1)).sum::<u32>(); idle(s, 0.4); (k as f64 * DT, after > before)
    };
    let pick = s.inv.iter().position(|t| t.is_some_and(|t| t.item == "stone_pickaxe")); let axe = s.inv.iter().position(|t| t.is_some_and(|t| t.item == "stone_axe")); let knife = s.inv.iter().position(|t| t.is_some_and(|t| t.item == "stone_knife"));
    let (hand_stone, got) = time(&mut s, B::STONE, None); assert!(hand_stone > 15.0 && !got, "la pierre à mains nues : {hand_stone:.1} s, rien");
    let (pick_stone, got) = time(&mut s, B::STONE, pick); assert!(pick_stone < 3.4 && pick_stone > 2.4 && got, "à la pioche de pierre : {pick_stone:.1} s");
    let (hand_dirt, _) = time(&mut s, B::DIRT, None); assert!(hand_dirt > 1.5 && hand_dirt < 2.2, "la terre à la main : {hand_dirt:.1} s");
    let (hand_wood, got) = time(&mut s, B::PLANKS, None); let (axe_wood, _) = time(&mut s, B::PLANKS, axe); assert!(got && hand_wood > 6.0 && axe_wood < hand_wood / 3.0, "bois : {hand_wood:.1} s à la main, {axe_wood:.1} s à la hache");
    // l'herbe donne parfois fibre et graines, toujours de la fibre au couteau
    let f0 = s.count("fiber"); for _ in 0..6 { time(&mut s, B::TALL_GRASS, knife); } assert_eq!(s.count("fiber"), f0 + 6);
    let (f1, g1) = (s.count("fiber"), s.count("seeds")); for _ in 0..40 { time(&mut s, B::TALL_GRASS, None); } let (df, dg) = (s.count("fiber") - f1, s.count("seeds") - g1); assert!(df > 6 && df < 30 && dg > 0 && dg < 15, "fibre {df}, graines {dg} sur 40 touffes");
    // frapper dans le vide fatigue
    s.player.fly = true; s.player.p[1] += 30.0; s.player.pitch = 0.6; s.stamina = 100.0; for _ in 0..30 { s.tick(DT, none(), Actions { mine: true, place: false }); } assert!(s.target.is_none() && s.stamina < 100.0, "coups dans le vide : {}", s.stamina);
}
