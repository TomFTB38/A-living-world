//! Villages : placement, bâtiments posés dans le monde, journée des villageois, services des artisans, sauvegarde.
use alw_core::{blocks::*, data::db, gen::WorldGen, player::*, session::*, stream::*, village::*};
const SEED: i32 = 20261104;
const DT: f64 = 1.0 / 30.0;
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(5)); } }
fn idle(s: &mut Session, sec: f64) { for _ in 0..(sec / DT) as usize { s.tick(DT, Input::default(), Actions::default()); } }
/// Une partie sur la place du village le plus proche de l'origine, en plein jour.
fn in_village() -> (Session, Village) {
    let v = WorldGen::new(SEED).villages_near(0.0, 0.0, 2000.0).into_iter().find(|v| v.crafts.contains(&"forgeron")).expect("un village de forgeron");
    let at = [v.c[0] as f64 + 0.5, v.c[1] as f64 + 1.0, v.c[2] as f64 + 3.5];
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, at); ready(&mut s); s.third = false; s.inv = [None; INV_SIZE]; s.hour = 12.0;
    idle(&mut s, 2.0); s.wild.all.clear(); (s, v)
}
fn option(s: &Session, start: &str) -> usize { s.dialog().unwrap().options.iter().position(|o| o.0.starts_with(start)).unwrap_or_else(|| panic!("pas de réponse « {start} » : {:?}", s.dialog().unwrap().options)) }
fn smith(s: &Session) -> usize { s.town.folk.iter().position(|f| f.craft == Some("forgeron")).expect("le forgeron") }

#[test]
fn des_villages_partout_et_aucun_artisan_unique() {
    let g = WorldGen::new(SEED); let all = g.villages_near(0.0, 0.0, 4000.0);
    assert!(all.len() >= 18 && all.len() <= 32, "{} villages", all.len()); // des villages plus rares : le voyage compte ; la mer couvre près d'un tiers du monde
    assert!(all.iter().any(|v| v.kind == "port"), "au moins un port de pêche");
    assert!((all[0].c[0] as f64).hypot(all[0].c[2] as f64) < 600.0, "un village à portée du départ : {:?}", all[0].c);
    for c in &db().craftsmen { let n = all.iter().filter(|v| v.crafts.contains(&c.id)).count(); assert!(n >= 2, "{} : {n} village(s) seulement", c.id); }
    for v in &all { assert!(v.crafts.len() >= 2 && v.houses.len() >= v.crafts.len() + 2, "{} : {:?}", v.name, v.crafts); assert!(v.r + 6 <= SPAN); }
    let again = WorldGen::new(SEED).villages_near(0.0, 0.0, 4000.0); assert_eq!(all.iter().map(|v| &v.name).collect::<Vec<_>>(), again.iter().map(|v| &v.name).collect::<Vec<_>>(), "les villages viennent de la graine");
    // Plusieurs types de village, chacun selon son biome.
    let kinds: std::collections::HashSet<_> = all.iter().map(|v| v.kind).collect(); assert!(kinds.len() >= 4, "{kinds:?}");
}

#[test]
fn les_batiments_sont_dans_le_monde() {
    let mut st = alw_core::store::ChunkStore::new(SEED, 64); let v = st.gen.villages_near(0.0, 0.0, 2000.0).remove(0);
    for h in &v.houses {
        // Chaque bâtiment est assis à sa propre hauteur, selon le relief : on lit le niveau de son sol à la porte.
        let fy = h.door[1].floor() as i32; let (dx, dz) = (h.inside[0] - h.door[0], h.inside[2] - h.door[2]); let d = [(h.door[0] + dx * 0.5).floor() as i32, fy, (h.door[2] + dz * 0.5).floor() as i32];
        assert!(BLOCKS[st.block(d[0], d[1], d[2]) as usize].name.starts_with("door"), "{} : porte attendue en {d:?}", v.name);
        assert!(BLOCKS[st.block(d[0], d[1] + 1, d[2]) as usize].name.starts_with("door"), "haut de porte");
        let station = match h.craft { Some("forgeron") | Some("armurier") | Some("maitre_forgeron") => Some("anvil"), Some("tailleur_pierre") => Some("mason_bench"), Some(_) => Some("workbench"), None => None };
        if let Some(name) = station { let (bx, bz) = [(0, 1), (1, 0), (0, -1), (-1, 0)][h.back as usize]; let p = [(h.spot[0] + bx as f64).floor() as i32, fy, (h.spot[2] + bz as f64).floor() as i32]; assert_eq!(BLOCKS[st.block(p[0], p[1], p[2]) as usize].name, name, "poste de {:?}", h.craft); }
        // Dedans, de quoi se tenir debout ; dehors devant la porte, de l'air.
        for p in [h.inside, h.spot, h.door] { assert!(!is_solid(st.block(p[0].floor() as i32, fy, p[2].floor() as i32)), "passage libre en {p:?}"); /* un tapis ne gêne pas */ assert!(is_solid(st.block(p[0].floor() as i32, fy - 1, p[2].floor() as i32)), "sol sous {p:?}"); }
        assert!((fy - v.c[1] - 1).abs() <= 5, "pas plus de cinq blocs d'écart avec la place");
    }
    assert_eq!(BLOCKS[st.block(v.c[0], v.c[1], v.c[2]) as usize].name, "water", "le puits au milieu de la place");
}

#[test]
fn la_journee_des_villageois() {
    let (mut s, v) = in_village(); let n = s.town.folk.iter().filter(|f| f.v == v.id).count();
    assert!(n >= v.houses.len(), "{n} habitants pour {} maisons", v.houses.len());
    assert!(s.town.known.contains(&v.id), "on découvre le village en y entrant");
    // À midi, chaque artisan est à son poste et travaille.
    idle(&mut s, 40.0); for f in s.town.folk.iter().filter(|f| f.v == v.id && f.craft.is_some()) { let h = &v.houses[f.house]; assert!((f.p[0] - h.spot[0]).hypot(f.p[2] - h.spot[2]) < 0.3 && f.working, "{} pas à son poste", f.name); }
    // Le soir sur la place, la nuit chez soi : le chemin passe par la porte.
    // Le soir, une partie du village se retrouve sur la place ; les autres restent chez eux ou passent chez un voisin.
    s.hour = 19.0; idle(&mut s, 60.0); let on_plaza = s.town.folk.iter().filter(|f| f.v == v.id && (f.p[0] - v.c[0] as f64).hypot(f.p[2] - v.c[2] as f64) < 7.0).count();
    assert!(on_plaza >= 1, "personne sur la place le soir");
    // La nuit, chacun rentre : il ouvre sa porte en passant et la referme derrière lui.
    let doors: Vec<[i32; 3]> = v.houses.iter().map(|h| { let (dx, dz) = (h.inside[0] - h.door[0], h.inside[2] - h.door[2]); [(h.door[0] + dx * 0.5).floor() as i32, h.door[1].floor() as i32, (h.door[2] + dz * 0.5).floor() as i32] }).collect();
    let open = |s: &Session| doors.iter().filter(|d| s.world.get(d[0], d[1], d[2]).and_then(alw_core::shapes::info).is_some_and(|i| i.open)).count();
    s.hour = 23.0; let mut seen_open = 0; for _ in 0..(60.0 / DT) as usize { s.tick(DT, Input::default(), Actions::default()); seen_open = seen_open.max(open(&s)); }
    for f in s.town.folk.iter().filter(|f| f.v == v.id) { let h = &v.houses[f.house]; assert!((f.p[0] - h.bed[0]).hypot(f.p[2] - h.bed[2]) < 0.3, "{} n'est pas rentré : en {:?}, lit {:?}, porte {:?}, dedans {:?}, atelier {:?}", f.name, f.p, h.bed, h.door, h.inside, h.craft); }
    assert!(seen_open > 0, "les portes s'ouvrent au passage"); idle(&mut s, 3.0); assert_eq!(open(&s), 0, "et se referment derrière eux");
}

#[test]
fn parler_au_forgeron_et_ses_services() {
    let (mut s, v) = in_village(); idle(&mut s, 40.0); let k = smith(&s);
    // Clic droit sur lui : la conversation s'ouvre.
    let f = s.town.folk[k].p; s.player.p = [f[0] - 1.6 * (s.town.folk[k].yaw).sin(), f[1], f[2] - 1.6 * (s.town.folk[k].yaw).cos()];
    let e = s.player.eye(); let d = [f[0] - e[0], f[1] + 1.5 - e[1], f[2] - e[2]]; s.player.yaw = d[0].atan2(d[2]); s.player.pitch = (d[1] / d[0].hypot(d[2])).atan();
    s.tick(DT, Input::default(), Actions::default()); s.tick(DT, Input::default(), Actions { mine: false, place: true, ..Default::default() });
    assert!(s.town.talk.is_some(), "le clic droit engage la conversation"); assert!(s.town.met.contains(&(v.id, "forgeron")), "l'artisan entre au journal");
    s.close_talk(); s.talk_to(k);
    // Commande : nos matériaux et un paiement.
    s.add_item("iron_ingot", 2); s.add_item("coal", 8); s.choose(option(&s, "Passer commande"));
    let i = option(&s, "Lingot d'acier"); assert!(s.dialog().unwrap().options[i].1, "commande possible : {:?}", s.dialog().unwrap().options[i]);
    let h0 = s.hours; s.choose(i);
    assert_eq!(s.count("steel_ingot"), 1, "{}", s.dialog().unwrap().said); assert_eq!(s.count("iron_ingot"), 0); assert_eq!(s.count("coal"), 0, "deux pour le feu, six pour le prix");
    assert!(s.hours > h0 + 0.9, "le travail prend une heure"); assert_eq!(s.rep(v.id), 4);
    s.choose(option(&s, "Retour"));
    // Sans réputation, pas de transmission ; des cadeaux, et il enseigne (au prix fort).
    let i = option(&s, "Apprendre"); assert!(!s.dialog().unwrap().options[i].1, "réputation trop basse");
    s.add_item("cooked_meat", 6); for _ in 0..6 { s.sel = s.inv.iter().position(|x| x.is_some_and(|x| x.item == "cooked_meat")).unwrap(); s.choose(option(&s, "Offrir")); }
    assert!(s.rep(v.id) >= TEACH_REP, "réputation {}", s.rep(v.id)); s.add_item("coal", 18); s.choose(option(&s, "Apprendre"));
    assert!(s.taught.contains("forgeron"), "{}", s.dialog().unwrap().said); assert_eq!(s.count("coal"), 0);
    // Réparation de l'outil en main, puis amélioration en acier.
    s.inv[0] = Some(Stack { item: "pickaxe", n: None, wear: 120 }); s.sel = 0; s.add_item("bread", 6); s.choose(option(&s, "Réparer"));
    assert_eq!(s.inv[0].unwrap().wear, 0); assert_eq!(s.count("bread"), 3);
    s.add_item("steel_ingot", 2); s.choose(option(&s, "Améliorer"));
    assert_eq!(s.inv[0].unwrap().item, "steel_pickaxe", "{}", s.dialog().unwrap().said); assert_eq!(s.count("steel_ingot"), 0, "la moitié de l'acier d'une pioche neuve"); assert_eq!(s.count("bread"), 0);
    // Une rumeur fait connaître un autre village ; on marque celui-ci.
    let known = s.town.known.len(); s.choose(option(&s, "Des nouvelles")); assert_eq!(s.town.known.len(), known + 1, "{}", s.dialog().unwrap().said);
    s.choose(option(&s, "Marquer")); assert_eq!(s.town.marked, Some(v.id)); assert!(s.marked_village().is_some());
    assert!(s.journal().len() >= 2);
    // Tout cela se sauvegarde.
    let t = s.save_text(); let mut b = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 120.0, 0.5]); b.load_text(&t).unwrap();
    assert_eq!(b.rep(v.id), s.rep(v.id)); assert_eq!(b.town.known, s.town.known); assert_eq!(b.town.met, s.town.met); assert_eq!(b.town.marked, s.town.marked); assert_eq!(b.journal().len(), s.journal().len());
    s.choose(option(&s, "Au revoir")); assert!(s.town.talk.is_none());
}

#[test]
fn le_travail_le_plus_fin_reste_aux_maitres() {
    let c = |id: &str| craftsman(id).unwrap(); let names = |id: &str| orders(c(id)).iter().map(|i| db().recipes[*i].out).collect::<Vec<_>>();
    assert!(!c("maitre_forgeron").services.contains(&"transmission"), "le maître forgeron ne transmet pas");
    assert!(db().errors.is_empty(), "{:?}", db().errors); for k in alw_core::data::VILLAGES { assert!(trades(k).len() >= 6, "{k} : {} échanges", trades(k).len()); }
    assert!(names("maitre_forgeron").contains(&"master_pickaxe") && names("maitre_forgeron").contains(&"steel_pickaxe"), "le maître fait aussi le travail du forgeron");
    assert!(!names("forgeron").contains(&"master_pickaxe")); assert!(names("tailleur_pierre").contains(&"stone_bricks"), "{:?}", names("tailleur_pierre"));
    for cm in &db().craftsmen { if cm.services.contains(&"commande") { assert!(!orders(cm).is_empty(), "{} n'a rien à faire", cm.id); } }
}

#[test]
fn saluer_echanger_et_ranger_son_arme() {
    let (mut s, v) = in_village(); s.hour = 19.0; idle(&mut s, 30.0); let greeted: Vec<String> = s.notes.iter().filter_map(|n| n.split(" : « Bon").next().filter(|_| n.contains(" : « Bon")).map(|x| x.to_string())).collect(); s.notes.clear();
    // Un villageois (pas un artisan) sur la place : on s'approche, il salue.
    // (celui qui est le plus loin de nous et ne nous a pas encore salués : dans un hameau, on se croise vite)
    let me = s.player.p; let k = (0..s.town.folk.len()).filter(|i| s.town.folk[*i].v == v.id && s.town.folk[*i].craft.is_none() && !greeted.contains(&s.town.folk[*i].name)).max_by(|a, b| { let d = |i: usize| (s.town.folk[i].p[0] - me[0]).hypot(s.town.folk[i].p[2] - me[2]); d(*a).total_cmp(&d(*b)) }).expect("un villageois"); let p = s.town.folk[k].p; s.player.p = [p[0] + 1.6, p[1], p[2]]; idle(&mut s, 0.5);
    assert!(s.notes.iter().any(|n| n.contains("Bonsoir") || n.contains("Bonjour")), "il salue : {:?}", s.notes);
    // L'épée à la main : il recule, et ne parle pas tant qu'elle n'est pas rangée.
    s.inv[3] = Some(Stack::new("sword", None)); s.sel = 3; s.notes.clear(); idle(&mut s, 0.3); assert!(s.notes.iter().any(|n| n.contains("Rangez cette arme")), "{:?}", s.notes);
    assert!(s.town.folk[k].scared > 0.0);
    s.talk_to(k); assert_eq!(s.dialog().unwrap().options.len(), 2); s.choose(option(&s, "Ranger")); assert!(!s.armed(), "l'arme est rangée"); assert!(s.dialog().unwrap().options.len() > 2);
    // Échanger : dix baies contre un pain.
    s.add_item("berries", 10); s.choose(option(&s, "Échanger")); let i = option(&s, "Donner 10 myrtilles"); assert!(s.dialog().unwrap().options[i].1);
    let r = s.rep(v.id); s.choose(i); assert_eq!(s.count("bread"), 1); assert_eq!(s.count("berries"), 0); assert_eq!(s.rep(v.id), r + 1); assert!(!s.dialog().unwrap().options[option(&s, "Donner 10 myrtilles")].1, "plus de baies");
    s.choose(option(&s, "Retour")); s.choose(option(&s, "Au revoir")); assert!(s.town.talk.is_none());
}

#[test]
fn les_villageois_contournent_les_blocs() {
    // Des piliers de pierre posés sur la place et devant les portes : personne ne passe au travers.
    let (mut s, v) = in_village(); let y = v.c[1] + 1; let mut e = Vec::new();
    for (dx, dz) in [(3, 0), (-3, 1), (0, 3), (2, -3), (-2, -3), (4, 2)] { for h in 0..2 { e.push((v.c[0] + dx, y + h, v.c[2] + dz, B::STONE)); } }
    for h in &v.houses { let q = [h.ring[0].floor() as i32, y, h.ring[2].floor() as i32]; e.push((q[0], y, q[2], B::STONE)); e.push((q[0], y + 1, q[2], B::STONE)); }
    s.world.edit_many(&e);
    let solid = |s: &Session, p: [f64; 3]| s.world.get(p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32).is_some_and(|b| b == B::STONE);
    for hour in [19.0, 23.0, 12.0] { s.hour = hour; for _ in 0..(40.0 / DT) as usize { s.tick(DT, Input::default(), Actions::default()); for f in &s.town.folk { assert!(!solid(&s, f.p), "{} dans un pilier en {:?}", f.name, f.p); } } }
}
