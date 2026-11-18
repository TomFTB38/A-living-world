//! Discrétion : accroupi, on se sait caché ou repéré ; frapper par surprise une créature qui ne vous a pas vu porte triple, et un
//! coup porté accroupi, à pas de loup, l'abat net (sauf un boss, qui encaisse double).
use alw_core::{hostile::{self as h, Ai}, player::*, session::*, stream::*, survival::*};
const DT: f64 = 1.0 / 60.0;
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(4)); } }
fn look_at(s: &mut Session, q: [f64; 3]) { let e = s.player.eye(); let d = [q[0] - e[0], q[1] - e[1], q[2] - e[2]]; s.player.yaw = d[0].atan2(d[2]); s.player.pitch = (d[1] / d[0].hypot(d[2])).atan(); }
/// Le monde plat, de jour, une créature `id` à deux pas devant soi, de dos, dans l'état `st`.
fn behind(id: &str, st: Ai) -> (Session, usize) {
    let seed = alw_core::gen::FLAT | 0x123; let y = alw_core::gen::FLAT_Y as f64 + 1.0;
    let mut s = Session::new(seed, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, y, 0.5]); ready(&mut s); s.third = false; s.hour = 12.0;
    for _ in 0..30 { s.tick(DT, Input::default(), Actions::default()); } s.wild.all.clear(); s.horde.foes.clear(); s.inv[0] = Some(Stack::new("sword", None)); s.sel = 0; s.stamina = 100.0;
    let p = s.player.p; let i = s.horde.put(h::species(id).unwrap(), [p[0], p[1], p[2] + 2.2], 0.0, false); s.horde.foes[i].state = st; if st == Ai::Combat { s.horde.foes[i].alert = 1.0; s.horde.foes[i].seen = Some(p); }
    look_at(&mut s, [p[0], p[1] + 1.0, p[2] + 2.2]); s.tick(0.0, Input::default(), Actions::default()); s.notes.clear(); (s, i)
}
/// Un coup, debout ou accroupi : ce qu'il a retiré.
fn hit(s: &mut Session, i: usize, crouch: bool) -> f64 {
    let hp0 = s.horde.foes[i].hp; let p = s.horde.foes[i].p; let i0 = Input { crouch, ..Input::default() };
    for _ in 0..10 { s.tick(DT, i0, Actions::default()); } s.horde.foes[i].p = p; look_at(s, [p[0], p[1] + 1.0, p[2]]); s.tick(DT, i0, Actions { mine: true, ..Default::default() }); hp0 - s.horde.foes[i].hp
}

#[test]
fn le_coup_en_traitre_porte_triple() {
    let (mut s, i) = behind("brigand_epeiste", Ai::Combat); let aware = hit(&mut s, i, false);
    let (mut b, j) = behind("brigand_epeiste", Ai::Repos); let sneak = hit(&mut b, j, false);
    println!("un coup d'épée : {aware:.1} sur un épéiste en garde, {sneak:.1} en traître ({:?})", b.notes); assert!(sneak > aware * 2.5 && b.notes.iter().any(|n| n.contains("traître")));
}

#[test]
fn accroupi_dans_le_dos_on_abat_net_sauf_un_boss() {
    let (mut s, i) = behind("brigand_epeiste", Ai::Patrouille); hit(&mut s, i, true); println!("épéiste en ronde, frappé accroupi dans le dos : {:?}", s.notes);
    assert!(s.horde.foes[i].dead > 0.0 || s.horde.foes[i].hp <= 0.0, "abattu net"); assert!(s.notes.iter().any(|n| n.contains("Assassinat")));
    let (mut b, j) = behind("brigand_chef", Ai::Repos); let hp0 = b.horde.foes[j].hp; hit(&mut b, j, true); println!("chef de guerre, frappé accroupi dans le dos : {:.0} → {:.0}, {:?}", hp0, b.horde.foes[j].hp, b.notes);
    assert!(b.horde.foes[j].dead == 0.0 && b.horde.foes[j].hp > 0.0, "un boss n'est pas abattu net"); assert!(b.notes.iter().any(|n| n.contains("×2")));
}

#[test]
fn accroupi_on_se_sait_cache_ou_repere() {
    let (mut s, i) = behind("brigand", Ai::Repos); assert_eq!(s.stealth(), None, "debout : rien à dire");
    s.tick(DT, Input { crouch: true, ..Input::default() }, Actions::default()); assert_eq!(s.stealth(), Some(true), "accroupi, personne ne vous traque");
    s.horde.foes[i].state = Ai::Poursuite; assert_eq!(s.stealth(), Some(false), "un brigand vous poursuit : repéré");
}
