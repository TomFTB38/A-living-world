//! Les outils du bâtisseur : la pipette, annuler et rétablir, remplir / vider / murs / sol entre deux coins, copier et coller
//! (tourné, en miroir), chacun un seul pas d'annulation.
use alw_core::{blocks::*, outils, player::*, session::*, stream::*};
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn world(creative: bool) -> (Session, [i32; 3]) {
    let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [2000.5, y as f64 + 2.0, 1975.5]); ready(&mut s);
    s.set_creative(creative); s.hour = 12.0; for _ in 0..6 { s.tick(DT, Input::default(), Actions::default()); } s.wild.all.clear(); s.horde.foes.clear(); s.notes.clear(); (s, [2000, y + 1, 1990])
}
fn id(n: &str) -> Id { by_name(n).unwrap_or_else(|| panic!("le bloc {n}")) }
fn at(s: &Session, p: [i32; 3]) -> Id { s.world.get(p[0], p[1], p[2]).unwrap() }
/// Viser le dessus du bloc sous `p` : on poserait en `p`.
fn aim(s: &mut Session, p: [i32; 3]) { let b = at(s, [p[0], p[1] - 1, p[2]]); s.target = Some(Target { x: p[0], y: p[1] - 1, z: p[2], px: p[0], py: p[1], pz: p[2], id: b, bx: [0.0, 0.0, 0.0, 1.0, 1.0, 1.0] }); }
fn look_at(s: &mut Session, p: [i32; 3]) { let b = at(s, p); s.target = Some(Target { x: p[0], y: p[1], z: p[2], px: p[0], py: p[1] + 1, pz: p[2], id: b, bx: [0.0, 0.0, 0.0, 1.0, 1.0, 1.0] }); }
fn count(s: &Session, item: &str) -> u32 { s.inv.iter().flatten().filter(|st| st.item == item).map(|st| st.n.unwrap_or(1)).sum() }

#[test]
fn la_pipette_met_le_bloc_vise_en_main() {
    let (mut s, p) = world(true);
    for (n, it) in [("stone_bricks", "stone_bricks"), ("chair@2", "chair"), ("planks", "planks")] {
        s.world.edit(p[0], p[1], p[2], id(n)); look_at(&mut s, p); let m = s.pick_block();
        assert_eq!(s.inv[s.sel].map(|st| st.item), Some(it), "{n} : {m}"); assert!(m.starts_with("En main"), "{m}");
    }
    // les formes tournées, couchées, hautes : l'objet qui les pose
    for n in ["oak_log_x", "oak_log_z_c"] { if by_name(n).is_some() { assert_eq!(outils::item_of_block(id(n)), Some("oak_log"), "{n}"); } }
    for b in 1..BLOCKS.len() as Id { if let Some(i) = alw_core::shapes::info(b) { if let Some(it) = outils::item_of_block(i.base) { assert_eq!(outils::item_of_block(b), Some(it), "{}", BLOCKS[b as usize].name); } } }
    // déjà dans la barre : on la sélectionne sans rien créer
    s.inv[3] = Some(Stack::new("stained_glass", Some(5))); s.world.edit(p[0], p[1], p[2], id("stained_glass")); look_at(&mut s, p); s.pick_block(); assert_eq!(s.sel, 3);
    // en survie : la pile du sac passe en main, et rien ne naît de rien
    let (mut s, p) = world(false); s.inv = [None; INV_SIZE]; s.sel = 0; s.inv[0] = Some(Stack::new("stick", Some(2))); s.inv[20] = Some(Stack::new("cobblestone", Some(12)));
    s.world.edit(p[0], p[1], p[2], id("cobblestone")); look_at(&mut s, p); s.pick_block();
    assert_eq!(s.inv[0].map(|st| (st.item, st.n)), Some(("cobblestone", Some(12)))); assert_eq!(s.inv[20].map(|st| st.item), Some("stick"), "échangées");
    s.world.edit(p[0], p[1], p[2], id("stained_glass")); look_at(&mut s, p); let m = s.pick_block(); assert!(m.starts_with("Vous n'avez pas"), "{m}"); assert_eq!(count(&s, "stained_glass"), 0);
}

#[test]
fn annuler_et_retablir_une_pose_et_une_casse() {
    let (mut s, p) = world(true); s.player.p = [p[0] as f64 + 6.5, p[1] as f64, p[2] as f64 + 6.5];
    s.inv[s.sel] = Some(Stack::new("stone_bricks", Some(64))); aim(&mut s, p); assert!(s.place()); assert_eq!(at(&s, p), id("stone_bricks"));
    assert!(s.undo_last().starts_with("Annulé"), "annuler"); assert_eq!(at(&s, p), 0, "la pose est défaite");
    assert!(s.redo_last().starts_with("Rétabli")); assert_eq!(at(&s, p), id("stone_bricks"), "et refaite");
    assert_eq!(s.command("/annuler"), "Annulé : 1 bloc(s)"); assert_eq!(s.command("/retablir"), "Rétabli : 1 bloc(s)");
    // une casse (au clic, en créatif) s'annule aussi : le bloc revient
    let w = [p[0], p[1] + 1, p[2] + 3]; s.world.edit(w[0], w[1], w[2], id("cobblestone")); s.undo.clear();
    s.player.p = [w[0] as f64 + 0.5, p[1] as f64, w[2] as f64 - 2.5]; s.player.yaw = 0.0; s.player.pitch = -0.05; s.player.fly = true; s.third = false;
    for _ in 0..40 { s.tick(DT, Input::default(), Actions { mine: true, ..Default::default() }); if at(&s, w) == 0 { break; } }
    assert_eq!(at(&s, w), 0, "le pavé est cassé {:?} {:?} {:?}", s.target.map(|t| (t.x, t.y, t.z)), s.eye, s.look); s.tick(DT, Input::default(), Actions::default());
    assert!(s.undo_last().starts_with("Annulé")); assert_eq!(at(&s, w), id("cobblestone"), "la casse est défaite");
    s.player.p = [p[0] as f64 + 6.5, p[1] as f64, p[2] as f64 + 6.5]; s.undo.clear(); s.redo.clear(); s.inv[s.sel] = Some(Stack::new("stone_bricks", Some(64))); aim(&mut s, p); s.world.edit(p[0], p[1], p[2], 0); assert!(s.place());
    // une case changée depuis n'est pas écrasée
    s.world.edit(p[0], p[1], p[2], id("stained_glass")); assert!(s.undo_last().contains("changé"), "rien à refaire"); assert_eq!(at(&s, p), id("stained_glass"));
    // un nouveau geste efface ce qu'on pouvait rétablir ; la mémoire est bornée
    s.undo.clear(); s.redo.clear(); for k in 0..(outils::UNDO_MAX as i32 + 10) { let q = [p[0] + k % 8, p[1] + 2 + k / 64, p[2] + k / 8 % 8]; s.inv[s.sel] = Some(Stack::new("planks", Some(64))); aim(&mut s, q); let _ = s.place(); }
    assert!(s.undo.len() <= outils::UNDO_MAX);
    // en survie : annuler une pose rend le bloc, la rétablir le reprend
    let (mut s, p) = world(false); s.player.p = [p[0] as f64 + 6.5, p[1] as f64, p[2] as f64 + 6.5]; s.inv = [None; INV_SIZE]; s.sel = 0; s.inv[0] = Some(Stack::new("planks", Some(3)));
    aim(&mut s, p); assert!(s.place()); assert_eq!(count(&s, "planks"), 2); let laid = at(&s, p); assert_ne!(laid, 0);
    s.undo_last(); assert_eq!(at(&s, p), 0); assert_eq!(count(&s, "planks"), 3, "la planche revient");
    s.redo_last(); assert_eq!(at(&s, p), laid, "la planche reposée telle quelle"); assert_eq!(count(&s, "planks"), 2, "et repart");
    // sans planche en sac, on ne rétablit pas : rien ne naît de rien
    s.undo_last(); s.inv = [None; INV_SIZE]; let m = s.redo_last(); assert!(m.starts_with("Il vous manque"), "{m}"); assert_eq!(at(&s, p), 0);
}

#[test]
fn remplir_vider_murs_et_sol_entre_deux_coins() {
    let (mut s, p) = world(true);
    let (a, b) = ([p[0], p[1], p[2]], [p[0] + 4, p[1] + 2, p[2] + 3]); // 5 × 3 × 4
    look_at(&mut s, a); s.set_corner(0); look_at(&mut s, b); s.set_corner(1);
    let all = |s: &Session, f: &dyn Fn([i32; 3]) -> bool| { let mut n = 0; for y in a[1]..=b[1] { for z in a[2]..=b[2] { for x in a[0]..=b[0] { if f([x, y, z]) && at(s, [x, y, z]) != 0 { n += 1; } } } } n };
    // la boîte se montre, la main vide : son aperçu couvre les 5 × 3 × 4 cases ; un bloc en main, c'est l'aperçu de la pose
    s.inv[s.sel] = None; assert_eq!(s.select_box(), Some((a, b))); assert_eq!(s.ghost().map(|g| g.0), Some(a)); let g = s.ghost_boxes()[0]; assert!((g[3] - g[0] - 5.04).abs() < 1e-9 && (g[4] - g[1] - 3.04).abs() < 1e-9 && (g[5] - g[2] - 4.04).abs() < 1e-9, "{g:?}");
    s.inv[s.sel] = Some(Stack::new("planks", Some(8))); assert_eq!(s.select_box(), None); s.inv[s.sel] = None;
    let m = s.command("/remplir stone_bricks"); assert_eq!(all(&s, &|_| true), 60, "{m}"); assert!(m.contains("60"), "{m}");
    let m = s.command("/vider"); assert_eq!(all(&s, &|_| true), 0, "{m}");
    s.inv[s.sel] = Some(Stack::new("planks", Some(64))); s.command("/murs"); let edge = |q: [i32; 3]| q[0] == a[0] || q[0] == b[0] || q[2] == a[2] || q[2] == b[2];
    assert_eq!(all(&s, &|q| edge(q)), 42, "les murs, du bloc tenu"); assert_eq!(all(&s, &|q| !edge(q)), 0, "l'intérieur reste vide"); assert_eq!(at(&s, a), id("planks"));
    s.command("/sol cobblestone"); assert_eq!(at(&s, [a[0] + 2, a[1], a[2] + 1]), id("cobblestone"), "le sol"); assert_eq!(at(&s, [a[0] + 2, a[1] + 1, a[2] + 1]), 0);
    // un seul pas d'annulation par outil : le sol, puis les murs, défaits d'un coup chacun
    s.undo_last(); assert_eq!(at(&s, [a[0] + 2, a[1], a[2] + 1]), 0); assert_eq!(at(&s, a), id("planks"), "le sol défait, pas les murs");
    s.undo_last(); assert_eq!(all(&s, &|_| true), 0, "les murs défaits d'un coup");
    assert!(s.command("/remplir pierre_inconnue").starts_with("Bloc inconnu"));
    // en survie, ces outils ne servent pas
    assert_eq!(s.command("/oublier"), "Boîte oubliée"); assert_eq!(s.select_box(), None);
    look_at(&mut s, a); s.set_corner(0); s.set_corner(1);
    s.set_creative(false); assert!(s.command("/remplir stone_bricks").contains("créatif")); assert_eq!(all(&s, &|_| true), 0);
}

#[test]
fn copier_et_coller_tourne_ou_en_miroir() {
    let (mut s, p) = world(true);
    // un petit motif : une pierre, une chaise dos vers +z à l'est, une planche au-dessus de la pierre
    s.world.edit_many(&[(p[0], p[1], p[2], id("stone_bricks")), (p[0] + 1, p[1], p[2], id("chair")), (p[0], p[1] + 1, p[2], id("planks"))]);
    look_at(&mut s, p); s.set_corner(0); look_at(&mut s, [p[0] + 1, p[1] + 1, p[2]]); s.set_corner(1); let m = s.copy_box(); assert!(m.contains("2 × 2 × 1") && m.contains("3 blocs"), "{m}");
    // collé tel quel, plus loin
    let q = [p[0], p[1], p[2] + 6]; aim(&mut s, q); s.turn = 0; s.paste();
    assert_eq!((at(&s, q), at(&s, [q[0] + 1, q[1], q[2]]), at(&s, [q[0], q[1] + 1, q[2]])), (id("stone_bricks"), id("chair"), id("planks")), "collé à l'identique");
    // un quart de tour (R) : ce qui était à l'est passe au sud, et la chaise tourne avec (dos vers -x)
    let q = [p[0] + 6, p[1], p[2]]; aim(&mut s, q); s.turn = 1; let m = s.paste(); assert!(m.contains("90"), "{m}");
    assert_eq!(at(&s, q), id("stone_bricks")); assert_eq!(at(&s, [q[0], q[1], q[2] + 1]), id("chair@3"), "la chaise tournée"); assert_eq!(at(&s, [q[0], q[1] + 1, q[2]]), id("planks"));
    // quatre quarts de tour : on revient au point de départ
    for k in 0..4u8 { let m = outils::turn_matrix(k, false); let back = outils::turn_matrix((4 - k) % 4, false); for f in 0..4 { let n = if f == 0 { "chair".to_string() } else { format!("chair@{f}") };
        assert_eq!(alw_core::structures::orient(alw_core::structures::orient(id(&n), m), back), id(&n), "{n} : {k} quarts de tour aller-retour"); } }
    // en miroir : l'est passe à l'ouest ; l'air de la copie n'efface rien
    s.mirror_clip(); let q = [p[0] + 12, p[1], p[2]]; s.world.edit(q[0], q[1] + 1, q[2], id("stained_glass")); aim(&mut s, q); s.turn = 0; s.paste();
    assert_eq!(at(&s, [q[0], q[1], q[2]]), id("chair"), "la chaise passe à gauche"); assert_eq!(at(&s, [q[0] + 1, q[1], q[2]]), id("stone_bricks")); assert_eq!(at(&s, [q[0] + 1, q[1] + 1, q[2]]), id("planks"));
    assert_eq!(at(&s, [q[0], q[1] + 1, q[2]]), id("stained_glass"), "l'air de la copie n'efface rien");
    // un collage s'annule d'un coup
    s.undo_last(); assert_eq!(at(&s, [q[0] + 1, q[1], q[2]]), 0); assert_eq!(at(&s, [q[0], q[1], q[2]]), 0);
}
