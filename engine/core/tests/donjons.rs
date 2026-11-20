//! La bibliothèque de salles et les donjons assemblés : chaque plan tiré tient sur sa grille, chaque donjon se parcourt de
//! bout en bout avec les solutions de ses salles, et chaque grille ne s'ouvre qu'après son épreuve (vérifié par le solveur).
use alw_core::{blocks::*, donjons::{self as dj, Act, Kind}, player::*, sanctuaire::{reach, DEPTH}, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..8000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn name_is(b: u16, n: &str) -> bool { BLOCKS[b as usize].name == n }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn put(s: &mut Session, c: [i32; 3]) { s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }

#[test]
fn chaque_plan_tient_sur_sa_grille_et_les_donjons_varient() {
    let mut seen = BTreeSet::new();
    for seed in 0..300u32 {
        let l = dj::layout(seed); assert_eq!(l.rooms.len(), dj::ROOMS); assert_eq!(dj::layout(seed).rooms, l.rooms, "même graine, même donjon");
        assert_eq!(l.rooms[0].kind, Kind::Entree); assert_eq!(l.rooms[dj::ROOMS - 2].kind, Kind::Gardien); assert_eq!(l.rooms[dj::ROOMS - 1].kind, Kind::Tresor);
        assert_eq!(l.rooms[4].kind, Kind::Descente); assert_eq!(l.rooms[4].exit.map(|d| alw_core::sites::spin_xz(d.0, d.1, (4 - l.rooms[4].q) % 4)), Some((1, 0)), "{seed} : la descente sort à droite");
        for (i, r) in l.rooms.iter().enumerate() { assert_eq!(r.y, if i >= 4 { -dj::LEVEL } else { 0 }, "{seed} : l'étage de la salle {i}"); }
        let mid: Vec<Kind> = l.rooms[1..dj::ROOMS - 2].iter().map(|r| r.kind).filter(|k| *k != Kind::Descente).collect(); assert_eq!(mid.len(), 6); assert!(mid.iter().all(|k| dj::TRIALS.contains(k))); for (i, k) in mid.iter().enumerate() { assert!(!mid[i + 1..].contains(k), "{seed} : deux fois {k:?}"); }
        let lim = (dj::GRID - 1) * dj::PITCH / 2;
        for (i, r) in l.rooms.iter().enumerate() {
            assert!(r.c.0.abs() <= lim && r.c.1.abs() <= lim); assert!(l.rooms[..i].iter().all(|o| (o.c, o.y) != (r.c, r.y)), "{seed} : deux salles au même endroit");
            if let Some(n) = l.rooms.get(i + 1) { let d = ((n.c.0 - r.c.0) / dj::PITCH, (n.c.1 - r.c.1) / dj::PITCH); assert_eq!(Some(d), r.exit); assert_eq!(d.0.abs() + d.1.abs(), 1, "salles voisines"); } else { assert_eq!(r.exit, None); }
            // l'épreuve combinée ne sort que tout droit
            if r.kind == Kind::Epreuve { let back = alw_core::sites::spin_xz(0, 1, r.q); assert_eq!(r.exit, Some(back), "{seed} : l'épreuve sort en face"); }
        }
        seen.insert(format!("{:?}", l.rooms.iter().map(|r| (r.kind, r.c)).collect::<Vec<_>>()));
    }
    println!("{} donjons différents sur 300 graines", seen.len()); assert!(seen.len() > 250);
    for k in dj::TRIALS { assert!((0..300u32).any(|sd| dj::layout(sd).rooms.iter().any(|r| r.kind == k)), "{k:?} sort parfois"); }
    let l = dj::layout(7); println!("graine 7 : {:?} ; gardien {}", l.rooms.iter().map(|r| r.kind.name()).collect::<Vec<_>>(), l.boss);
}

/// Bâtit le donjon de la graine `seed` sur le monde plat et le parcourt : de la surface au trésor.
fn play(seed: u32) {
    let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [300.5, y as f64 + 1.0, 300.5]); ready(&mut s); s.set_creative(false); s.hour = 12.0;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [300, y, 300]; let (l, sol) = s.build_dungeon(o, seed); let _ = s.sync();
    let f = y - DEPTH; let w = |c: [i32; 3]| [o[0] + c[0], f + c[1], o[2] + c[2]];
    let look = |s: &Session, from: [i32; 3]| reach(&s.world, from, [o[0] - 30, f - dj::LEVEL - 3, o[2] - 30], [o[0] + 30, y + 6, o[2] + 30]);
    println!("graine {seed} : {:?}", l.rooms.iter().map(|r| r.kind.name()).collect::<Vec<_>>());
    // du parvis du portail, on descend le grand escalier jusqu'à la salle d'entrée, et l'on en remonte
    let top = w(l.rooms[0].porch()); assert!(look(&s, top).contains(&w(l.rooms[0].spot())), "{seed} : le grand escalier descend jusqu'à l'entrée"); assert!(look(&s, w(l.rooms[0].spot())).contains(&top), "{seed} : et remonte");
    for k in 0..l.rooms.len() - 1 {
        let (room, next) = (l.rooms[k], l.rooms[k + 1]); let from = w(room.spot()); let to = w(next.spot()); put(&mut s, from); run(&mut s, 0.2);
        let gated = room.kind.gated() && room.kind != Kind::Gardien;
        if room.kind == Kind::Labyrinthe { let walls = (-4..=4).flat_map(|u| (-4..=4).map(move |v| (u, v))).filter(|&(u, v)| { let c = w(room.at([u, 1, v])); is_solid(s.world.get(c[0], c[1], c[2]).unwrap()) }).count();
            assert!(walls >= 16, "{seed} : le labyrinthe a ses murs ({walls})"); }
        if gated { assert!(!look(&s, from).contains(&to), "{seed} : {} — la grille tient", room.kind.name()); }
        for a in &sol[k] { match *a {
            Act::Lever(p) => { assert!(s.lever_at(p), "{seed} : levier en {p:?}"); run(&mut s, 0.2); }
            Act::Push { from: f2, block } => { assert!(look(&s, from).contains(&f2), "{seed} : {} — on peut se placer en {f2:?} pour pousser", room.kind.name()); put(&mut s, f2); run(&mut s, 0.05); put(&mut s, f2); assert!(s.push_at(block), "{seed} : bloc en {block:?}"); run(&mut s, 0.2); }
            Act::Reach(p) => { assert!(look(&s, from).contains(&p), "{seed} : {} — on atteint {p:?}", room.kind.name()); put(&mut s, p); run(&mut s, 0.1); }
            Act::Slay => {
                let sp = alw_core::hostile::species(l.boss).unwrap(); let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).unwrap_or_else(|| panic!("{seed} : le gardien s'éveille ({:?})", s.notes));
                assert!(!look(&s, from).contains(&to), "{seed} : pendant le combat, la sortie est close"); s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "sword", false); run(&mut s, 0.6); }
        } }
        assert!(look(&s, from).contains(&to), "{seed} : après {}, on passe ({:?})", room.kind.name(), s.notes.iter().rev().take(3).collect::<Vec<_>>());
    }
    // le trésor
    let t = l.rooms.last().unwrap(); let c = w(t.at([0, 2, 2])); assert!(is_chest_shut(s.world.get(c[0], c[1], c[2]).unwrap())); s.open_chest(c);
    assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin"), "{seed} : le trésor"); println!("{:?}", s.notes);
}

#[test]
fn chaque_donjon_se_parcourt_de_bout_en_bout() { for seed in [1, 2, 3, 7, 11, 42] { play(seed); } }
#[test]
fn chaque_donjon_se_parcourt_de_bout_en_bout_encore() { for seed in [100, 271, 314, 999, 2026, 31337] { play(seed); } }
#[test]
fn quarante_donjons_de_suite() { for seed in 1000..1040 { play(seed); } }
