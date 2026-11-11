//! Scénario de validation du cahier des charges, joué avec les vraies actions du joueur (viser, clic gauche maintenu,
//! clic droit, fabriquer depuis le livre), sans commande de développeur : partir les mains vides, jusqu'au coffre rempli,
//! retrouvé intact après sauvegarde et rechargement.
use alw_core::{blocks::*, felling::find_tree, player::*, session::*, stream::*, survival::*};
const DT: f64 = 1.0 / 60.0;
const SEED: i32 = 20261104;
fn none() -> Input { Input::default() }
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, sec: f64) { for _ in 0..(sec / DT) as usize { s.tick(DT, none(), Actions::default()); } }
/// Le sol sous (x, z) : le premier bloc plein qui ne soit ni bois d'arbre ni feuillage.
fn ground(s: &Session, x: i32, z: i32) -> i32 { let tree = |b: u16| { let n = BLOCKS[b as usize].name; n.contains("log") || n.contains("leaves") || n.contains("branch") || n.contains("hollow") }; let mut y = s.player.p[1] as i32 + 6; while y > 2 && !s.world.get(x, y, z).is_some_and(|b| is_solid(b) && !tree(b)) { y -= 1; } y }
/// Tourne la tête vers un point et met la cible à jour.
fn look(s: &mut Session, q: [f64; 3]) { let e = s.player.eye(); let d = [q[0] - e[0], q[1] - e[1], q[2] - e[2]]; s.player.yaw = d[0].atan2(d[2]); s.player.pitch = (d[1] / d[0].hypot(d[2])).atan(); s.tick(0.0, none(), Actions::default()); }
/// Dégage la vue jusqu'au bloc visé (branches basses, troncs voisins) : le joueur, lui, contournerait.
fn clear_to(s: &mut Session, p: [i32; 3]) { let mine = |b: u16| matches!(b, B::WORKBENCH | B::CAMPFIRE | B::CHEST) || BLOCKS[b as usize].name.starts_with("planks"); /* ce qu'on a construit reste */ for e in [s.player.eye(), s.eye] { let q = [p[0] as f64 + 0.5, p[1] as f64 + 0.5, p[2] as f64 + 0.5]; for k in 1..400 { let t = k as f64 / 400.0; let c = [(e[0] + (q[0] - e[0]) * t).floor() as i32, (e[1] + (q[1] - e[1]) * t).floor() as i32, (e[2] + (q[2] - e[2]) * t).floor() as i32]; for d in [[0, 0, 0], [1, 0, 0], [-1, 0, 0], [0, 0, 1], [0, 0, -1]] { let c = [c[0] + d[0], c[1], c[2] + d[2]]; if c != p && s.world.get(c[0], c[1], c[2]).is_some_and(|b| b != 0 && !mine(b) && !is_water(b) && (!is_solid(b) || c[1] >= p[1])) || (d == [0, 0, 0] && c != p && s.world.get(c[0], c[1], c[2]).is_some_and(|b| b != 0 && !mine(b) && !is_water(b))) { s.world.edit(c[0], c[1], c[2], 0); } } } } }
fn aim(s: &mut Session, p: [i32; 3]) { clear_to(s, p); look(s, [p[0] as f64 + 0.5, p[1] as f64 + 0.35, p[2] as f64 + 0.5]); let t = s.target.unwrap_or_else(|| panic!("rien de visé vers {p:?}")); assert_eq!([t.x, t.y, t.z], p, "on vise {}", BLOCKS[t.id as usize].name); }
/// Clic droit sur un bloc, puis le temps que le geste se termine.
fn right(s: &mut Session, p: [i32; 3]) { aim(s, p); s.tick(DT, none(), Actions { mine: false, place: true, ..Default::default() }); idle(s, 0.8); }
/// Clic gauche maintenu jusqu'à ce que le bloc cède (l'endurance se refait entre deux blocs, comme en jouant).
fn dig(s: &mut Session, p: [i32; 3]) { aim(s, p); let b = s.world.get(p[0], p[1], p[2]); let mut k = 0; while s.world.get(p[0], p[1], p[2]) == b && k < 4000 { s.tick(DT, none(), Actions { mine: true, place: false, ..Default::default() }); if s.winded { idle(s, 2.0); aim(s, p); } k += 1; } assert!(k < 4000, "{p:?} ne cède pas"); idle(s, 0.3); }
/// Pose l'objet de la case `sel` sur le dessus d'un bloc de sol.
fn place_on(s: &mut Session, sel: usize, g: [i32; 3]) -> [i32; 3] { s.sel = sel; clear_to(s, g); look(s, [g[0] as f64 + 0.5, g[1] as f64 + 0.97, g[2] as f64 + 0.5]); s.tick(DT, none(), Actions { mine: false, place: true, ..Default::default() }); idle(s, 0.4); [g[0], g[1] + 1, g[2]] }
fn slot_of(s: &Session, id: &str) -> usize { s.inv.iter().position(|t| t.is_some_and(|t| t.item == id)).unwrap_or_else(|| panic!("pas de {id} dans le sac")) }
fn craft(s: &mut Session, out: &str, times: u32) { let i = recipes().iter().position(|r| r.out == out && r.need.iter().all(|n| s.count(n.id) >= n.n)).unwrap_or_else(|| panic!("aucune recette faisable pour {out}")); for _ in 0..times { s.craft(i).unwrap_or_else(|e| panic!("{out} : {e}")); } }
/// Un emplacement de sol libre à `d` blocs devant le joueur, de niveau avec lui.
fn spot(s: &mut Session, k: i32, l: i32) -> [i32; 3] { let p = s.player.p; let (x, z) = (p[0].floor() as i32 + k, p[2].floor() as i32 + l); let y = p[1].floor() as i32 - 1; if !s.world.get(x, y, z).is_some_and(is_solid) { s.world.edit(x, y, z, B::DIRT); } for h in 1..=4 { s.world.edit(x, y + h, z, 0); } [x, y, z] } // de niveau avec les pieds : le joueur aplanirait

/// Arracher de l'herbe à la main jusqu'à avoir `want` fibres.
fn fiber(s: &mut Session, want: u32) { let mut n = 0; while s.count("fiber") < want { let g = spot(s, 3, [-1, 1, 2][(n % 3) as usize]); s.world.edit(g[0], g[1] + 1, g[2], B::TALL_GRASS); dig(s, [g[0], g[1] + 1, g[2]]); n += 1; assert!(n < 120, "trop peu de fibre"); } }
/// Une prairie plate et sèche, avec un arbre de taille moyenne à moins de vingt blocs.
fn start() -> (Session, [i32; 3]) {
    let g = alw_core::gen::WorldGen::new(SEED);
    for k in 0..300 { let (x, z) = (-38.5 + k as f64 * 5.0, 0.5 + k as f64 * 3.0); let c = g.column(x, z); if c.water > 0.0 || c.bank > 0.0 { continue; }
        let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [x, c.h + 2.0, z]); ready(&mut s); idle(&mut s, 2.0); if !s.player.ground { continue; }
        let p = s.player.p; let (bx, by, bz) = (p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32);
        for r in 3i32..18 { for dz in -r..=r { for dx in -r..=r { if dx.abs().max(dz.abs()) != r { continue; } for y in by - 3..by + 3 { let (x, z) = (bx + dx, bz + dz);
            if s.world.get(x, y, z).is_some_and(is_log) && !s.world.get(x, y - 1, z).is_some_and(is_log) && find_tree(&s.world, x, y, z).is_some_and(|t| t.logs.len() >= 4 && t.logs.len() < 40) { return (s, [x, y, z]); } } } } }
    }
    panic!("aucune prairie avec un arbre");
}

#[test]
fn de_rien_au_coffre_sauvegarde() {
    let (mut s, tree) = start(); s.third = false;
    assert!(s.inv.iter().all(|x| x.is_none()), "1. on part sans rien");
    // 1. Ramasser des branches et des cailloux au clic droit : une branche, un bâton ; un caillou, un caillou.
    for k in 0..6 { let g = spot(&mut s, 2 + k % 2, k as i32 - 3); s.world.edit(g[0], g[1] + 1, g[2], B::TWIGS); right(&mut s, [g[0], g[1] + 1, g[2]]); }
    for k in 0..9 { let g = spot(&mut s, -2 - k % 2, (k % 5) as i32 - 2); s.world.edit(g[0], g[1] + 1, g[2], B::PEBBLES); right(&mut s, [g[0], g[1] + 1, g[2]]); }
    assert_eq!((s.count("stick"), s.count("pebble")), (6, 9), "branches et cailloux ramassés");
    // La fibre : arracher l'herbe à la main jusqu'à en avoir assez.
    fiber(&mut s, 8);
    // 2. Premier outil : la hache de pierre, à la main.
    craft(&mut s, "stone_axe", 1); assert_eq!(s.count("stone_axe"), 1);
    // 3-4. Couper l'arbre : on s'approche, hache en main, clic gauche maintenu ; les rondins vont dans le sac.
    let (gx, gz) = (tree[0] + 2, tree[2]); let gy = ground(&s, gx, gz); for h in 1..=3 { s.world.edit(gx, gy + h, gz, 0); } s.player.p = [gx as f64 + 0.5, gy as f64 + 1.0, gz as f64 + 0.5]; idle(&mut s, 0.5);
    s.sel = slot_of(&s, "stone_axe"); aim(&mut s, tree); let mut k = 0; while s.falling.is_none() && k < 3000 { s.stamina = s.stamina.max(30.0); s.tick(DT, none(), Actions { mine: true, place: false, ..Default::default() }); k += 1; }
    assert!(s.falling.is_some(), "l'arbre tombe"); idle(&mut s, 2.0); let logs = s.count("log"); assert!(logs >= 3, "rondins récoltés : {logs}");
    assert!(s.inv.iter().flatten().find(|t| t.item == "stone_axe").unwrap().wear > 0, "la hache s'use");
    // 5. L'établi : rondins en planches, planches en établi, posé au sol.
    for _ in 0..logs { if s.count("log") > 0 { craft(&mut s, "planks", 1); } }
    let planks = s.count("plank"); assert!(planks >= 12, "planches : {planks}"); craft(&mut s, "workbench", 1);
    let bench = { let g = spot(&mut s, 2, 0); let i = slot_of(&s, "workbench"); place_on(&mut s, i, g) }; assert_eq!(s.world.get(bench[0], bench[1], bench[2]), Some(B::WORKBENCH)); assert!(s.stations()[1], "établi à portée");
    // 6. Une hache plus performante : silex tiré du gravier, corde de fibre, à l'établi.
    let mut n = 0; while s.count("flint") < 2 { let g = spot(&mut s, -2, (n % 5) - 2); s.world.edit(g[0], g[1] + 1, g[2], B::GRAVEL); dig(&mut s, [g[0], g[1] + 1, g[2]]); n += 1; assert!(n < 60, "pas de silex"); }
    craft(&mut s, "rope", 1); craft(&mut s, "flint_axe", 1); assert_eq!(s.count("flint_axe"), 1);
    let speed = |id: &str| alw_core::data::def(id).and_then(|d| d.tool).unwrap().mat.speed; assert!(speed("flint_axe") > speed("stone_axe"), "le silex coupe plus vite");
    // 7. Un petit abri : quatre murs de planches posés un à un.
    let mut walls = 0; for (a, b) in [(-1, 4), (1, 4), (-1, 3), (0, 3), (1, 3)] { if s.count("plank") == 0 { break; } let g = spot(&mut s, a, b); let id = s.inv.iter().position(|t| t.is_some_and(|t| t.item.ends_with("planks"))).unwrap(); let p = place_on(&mut s, id, g); if s.world.get(p[0], p[1], p[2]).is_some_and(is_solid) { walls += 1; } }
    assert!(walls >= 3, "murs posés : {walls}");
    // 8. Le feu de camp : bâtons, cailloux, fibre ; posé, il brûle.
    craft(&mut s, "campfire", 1); let fire = { let g = spot(&mut s, -2, 0); let i = slot_of(&s, "campfire"); place_on(&mut s, i, g) }; assert_eq!(s.world.get(fire[0], fire[1], fire[2]), Some(B::CAMPFIRE_OUT), "posé éteint"); assert!(!s.stations()[2], "un feu éteint ne cuit rien");
    // Il s'allume à l'archet à feu (bâtons et fibre), après quelques essais.
    s.add_item("stick", 2); s.add_item("fiber", 2); craft(&mut s, "fire_drill", 1); s.sel = slot_of(&s, "fire_drill"); let mut k = 0; while s.world.get(fire[0], fire[1], fire[2]) != Some(B::CAMPFIRE) && k < 60 { right(&mut s, fire); k += 1; }
    assert_eq!(s.world.get(fire[0], fire[1], fire[2]), Some(B::CAMPFIRE), "le feu prend"); assert!(s.stations()[2], "feu à portée");
    // 9. Trouver de l'eau et boire, les mains libres.
    let w = spot(&mut s, 0, -2); s.world.edit(w[0], w[1], w[2], B::WATER); s.vit[2] = 40.0; s.sel = s.inv.iter().position(|t| t.is_none()).unwrap(); clear_to(&mut s, w); look(&mut s, [w[0] as f64 + 0.5, w[1] as f64 + 0.6, w[2] as f64 + 0.5]); s.tick(DT, none(), Actions { mine: false, place: true, ..Default::default() }); idle(&mut s, 0.8); assert!(s.vit[2] > 50.0, "on a bu : {}", s.vit[2]);
    // 10. Premier repas : un lièvre chassé, cuit au feu, mangé.
    s.sel = slot_of(&s, "flint_axe"); let g = spot(&mut s, 0, 2); let q = [g[0] as f64 + 0.5, g[1] as f64 + 1.0, g[2] as f64 + 0.5]; s.wild.put(alw_core::wild::Kind::Hare, q, 0.0, false);
    let mut k = 0; while s.count("raw_meat") == 0 && k < 200 { look(&mut s, [q[0], q[1] + 0.25, q[2]]); s.tick(DT, none(), Actions { mine: true, place: false, ..Default::default() }); k += 1; } assert!(s.count("raw_meat") > 0, "le lièvre est chassé");
    craft(&mut s, "cooked_meat", 1); s.vit[1] = 30.0; s.sel = slot_of(&s, "cooked_meat"); let up = [s.player.p[0], s.player.p[1] + 5.0, s.player.p[2] + 1.0]; look(&mut s, up); s.tick(DT, none(), Actions { mine: false, place: true, ..Default::default() }); assert!(s.vit[1] > 55.0, "repas : {}", s.vit[1]);
    // 11-12. Un coffre, posé, ouvert d'un clic droit, rempli comme à la souris.
    while s.count("plank") < 8 { assert!(s.count("log") > 0, "plus de bois"); craft(&mut s, "planks", 1); }
    fiber(&mut s, 3); s.sel = 0; craft(&mut s, "rope", 1); craft(&mut s, "chest", 1); let chest = { let g = spot(&mut s, 1, -2); let i = slot_of(&s, "chest"); place_on(&mut s, i, g) }; assert_eq!(s.world.get(chest[0], chest[1], chest[2]), Some(B::CHEST));
    right(&mut s, chest); assert_eq!(s.chest, Some(chest), "le coffre s'ouvre"); assert!(s.inv_open);
    let mut stored = Vec::new(); for (k, id) in ["pebble", "stick", "fiber"].iter().enumerate() { if let Some(i) = s.inv.iter().position(|t| t.is_some_and(|t| t.item == *id)) { stored.push(s.inv[i].unwrap()); s.grab_slot(i); s.drop_slot(Some(CHEST0 + k)); } }
    assert!(!stored.is_empty()); for (k, st) in stored.iter().enumerate() { assert_eq!(s.slot(CHEST0 + k), Some(*st), "dans le coffre"); }
    s.inv_open = false; s.close_chest();
    // 13-15. Quitter, recharger : personnage, sac, coffre et constructions à l'identique.
    idle(&mut s, 0.5); let txt = s.save_text();
    let mut o = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, 90.0, 0.5]); o.load_text(&txt).unwrap(); ready(&mut o);
    for k in 0..3 { assert!((o.player.p[k] - s.player.p[k]).abs() < 1e-6, "position"); } assert_eq!(o.inv, s.inv, "le sac"); assert_eq!(o.vit, s.vit); assert_eq!(o.chests, s.chests, "le coffre et son contenu");
    for (p, b) in [(bench, B::WORKBENCH), (fire, B::CAMPFIRE), (chest, B::CHEST)] { assert_eq!(o.world.get(p[0], p[1], p[2]), Some(b), "{} retrouvé", BLOCKS[b as usize].name); }
    assert_eq!(o.world.get(tree[0], tree[1], tree[2]), Some(0), "l'arbre abattu ne repousse pas");
    for (k, st) in stored.iter().enumerate() { o.chest = Some(chest); assert_eq!(o.slot(CHEST0 + k), Some(*st)); }
}
