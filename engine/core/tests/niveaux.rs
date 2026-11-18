//! Niveaux et compétences de l'explorateur : l'expérience gagnée (combats, découvertes, villages), les niveaux et leurs points,
//! l'arbre de compétences (prix croissant, étages qui s'ouvrent), et ce que chaque compétence change vraiment en jeu.
use alw_core::{blocks::by_name, hostile as h, niveaux::{self as n, Prog}, player::*, session::*, stream::*};
const DT: f64 = 1.0 / 60.0;
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(4)); } }
fn flat() -> Session {
    let seed = alw_core::gen::FLAT | 0x123; let y = alw_core::gen::FLAT_Y as f64 + 1.0;
    let mut s = Session::new(seed, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, y, 0.5]); ready(&mut s); s.third = false; s.hour = 12.0; s.wild.all.clear(); s.horde.foes.clear();
    for _ in 0..30 { s.tick(DT, Input::default(), Actions::default()); } s.vit = [100.0; 3]; s.stamina = 100.0; s.notes.clear(); s
}
fn walk(s: &mut Session, secs: f64, i: Input) { for _ in 0..(secs / DT) as usize { s.vit = [100.0; 3]; s.stamina = 100.0; s.tick(DT, i, Actions::default()); } }
fn idx(id: &str) -> usize { n::SKILLS.iter().position(|s| s.id == id).unwrap() }

#[test]
fn un_niveau_un_point_et_les_rangs_coutent_de_plus_en_plus() {
    let mut p = Prog::default(); assert_eq!(p.level().0, 1); assert_eq!(p.points(), 0);
    p.xp = n::need(1) + n::need(2) + 5.0; assert_eq!(p.level().0, 3); assert_eq!(p.points(), 2, "un point par niveau gagné");
    let total: u32 = (1..=n::RANKS).map(n::cost).sum::<u32>() * n::SKILLS.len() as u32; let xp_full: f64 = (1..=total).map(n::need).sum();
    println!("l'arbre complet : {total} points, soit le niveau {} ({xp_full:.0} XP)", total + 1); assert_eq!(total, 54);
    // ce que rapportent les créatures : un boss bien plus qu'un brigand
    let brig = Session::kill_xp(h::species("brigand").unwrap(), false); let chef = Session::kill_xp(h::species("brigand_chef").unwrap(), false); let cap = Session::kill_xp(h::species("capitaine").unwrap(), true);
    println!("XP : brigand {brig}, chef de guerre {chef}, capitaine vétéran {cap}"); assert!(chef >= 20.0 * brig && cap > chef);
}

#[test]
fn l_arbre_s_ouvre_etage_par_etage() {
    let mut s = flat(); s.prog.xp = (1..=12).map(n::need).sum::<f64>(); assert_eq!(s.prog.points(), 12);
    // le deuxième étage reste fermé tant que la branche n'a pas trois points
    assert!(s.buy_skill(idx("nage")).unwrap_err().contains("S'ouvre"), "Nage fermée");
    s.buy_skill(idx("foulee")).unwrap(); s.buy_skill(idx("foulee")).unwrap(); assert_eq!(s.prog.points(), 9, "rang 1 : 1 point, rang 2 : 2 points");
    s.buy_skill(idx("nage")).unwrap(); assert!(s.buy_skill(idx("escalade")).unwrap_err().contains("S'ouvre"), "Escalade : il faut 8 points dans la branche");
    s.buy_skill(idx("foulee")).unwrap(); s.buy_skill(idx("nage")).unwrap(); assert_eq!(s.prog.branch_spent(0), 9);
    assert!(s.buy_skill(idx("foulee")).unwrap_err().contains("maximum"));
    s.buy_skill(idx("escalade")).unwrap(); assert_eq!(s.prog.points(), 2); s.buy_skill(idx("escalade")).unwrap(); assert_eq!(s.prog.points(), 0); let m = s.buy_skill(idx("frappe")).unwrap_err();
    println!("plus assez de points : « {m} »"); assert!(m.contains("point"));
    // sauvegarde et relecture
    let t = s.save_text(); let mut b = flat(); b.load_text(&t).unwrap(); println!("{}", t.lines().find(|l| l.starts_with("prog")).unwrap());
    assert_eq!(b.prog.ranks, s.prog.ranks); assert_eq!(b.prog.points(), s.prog.points());
}

#[test]
fn foulee_et_portefaix_changent_le_corps() {
    let mut a = flat(); let p0 = a.player.p; walk(&mut a, 2.0, Input { z: 1.0, ..Input::default() }); let d0 = (a.player.p[2] - p0[2]).abs();
    let mut b = flat(); b.prog.ranks[idx("foulee")] = 3; let p0 = b.player.p; walk(&mut b, 2.0, Input { z: 1.0, ..Input::default() }); let d1 = (b.player.p[2] - p0[2]).abs();
    println!("deux secondes de marche : {d0:.2} pas, avec Foulée 3 : {d1:.2} pas"); assert!(d1 > d0 * 1.12);
    let c0 = a.capacity(); a.prog.ranks[idx("portefaix")] = 2; println!("charge portée : {c0:.0} → {:.0}", a.capacity()); assert!(a.capacity() > c0 * 1.3);
}

#[test]
fn l_escalade_monte_un_bloc_de_plus_par_rang() {
    // une falaise de pierre de `h` blocs devant soi ; on avance en tenant Saut cinq secondes : est-on monté dessus ?
    let tops = |rank: u8, h: i32| { let mut s = flat(); s.prog.ranks[idx("escalade")] = rank; let y = s.player.p[1].floor() as i32; let y0 = s.player.p[1];
        for dy in 0..h { for dx in -2..=2 { for dz in 2..40 { s.world.edit(dx, y + dy, dz, by_name("stone").unwrap()); } } } let _ = s.sync();
        walk(&mut s, 5.0, Input { z: 1.0, jump: true, ..Input::default() }); s.player.p[1] - y0 >= h as f64 - 0.05 };
    for rank in 0..=3u8 { let got: Vec<bool> = (2..=5).map(|h| tops(rank, h)).collect(); println!("rang {rank} : falaises de 2, 3, 4, 5 blocs gravies {got:?}");
        for (k, h) in (2..=5).enumerate() { assert_eq!(got[k], rank > 0 && h <= rank as i32 + 1, "rang {rank}, falaise de {h}"); } }
}

#[test]
fn on_gagne_de_l_experience_en_explorant_et_en_combattant() {
    let mut s = flat();
    // un brigand vaincu près de soi
    let p = s.player.p; let i = s.horde.put(h::species("brigand").unwrap(), [p[0], p[1], p[2] + 3.0], 3.14, false); s.horde.foes[i].hp = 1.0; s.horde.strike(i, 50.0, p, "sword", false);
    for _ in 0..30 { s.tick(DT, Input::default(), Actions::default()); } let x1 = s.prog.xp; println!("brigand vaincu : {x1} XP, {:?}", s.notes);
    assert!(x1 >= 8.0 && s.notes.iter().any(|n| n.contains("XP")));
    // un biome jamais foulé : on fait comme si on n'avait connu que les autres
    s.prog.biomes = !0 & !(1u16 << s.mix().main()); s.tick(DT, Input::default(), Actions::default()); println!("nouveau biome : {} XP, {:?}", s.prog.xp, s.notes.last());
    assert!(s.prog.xp >= x1 + n::XP_BIOME);
    // et les niveaux qui montent
    s.earn(500.0, "essai"); println!("{:?}", s.notes.last()); assert!(s.prog.level().0 >= 5 && s.notes.iter().any(|n| n.starts_with("Niveau")));
}
