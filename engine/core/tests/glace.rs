//! Le Temple de glace : les blocs glissent sur la glace ; on rejoue la solution de chaque bassin, poussée après poussée, comme
//! le joueur la ferait (en se plaçant derrière le bloc, sur la glace ou sur la margelle) ; un bloc coincé revient à sa place
//! quand on sort du bassin ; l'estrade ne se gagne qu'avec le bloc au pied de la brèche ; la porte, la Reine des glaces.
use alw_core::{blocks::*, glace::{self as gl, Rink}, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn world() -> (Session, [i32; 3]) {
    let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [1200.5, y as f64 + 2.0, 1200.5]); ready(&mut s); s.set_creative(false); s.hour = 12.0;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [1200, y, 1200]; s.build_ice_temple(o); let _ = s.sync(); put(&mut s, o, [0, 1, -3]); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
/// Les coordonnées du temple : v = 0 est le dallage.
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] + 1 + q[1], o[2] + q[2]] }
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-24, -3, -32]), at(o, [24, 10, 34])) }
fn chain(s: &Session, o: [i32; 3], u: i32) -> bool { name(s, at(o, [u, 2, gl::CHAINS_W])) == "great_chain" }
/// Pousser le bloc de la case `b` (du bassin `r`) dans la direction `d` : on se tient derrière lui, sur la glace ou sur la
/// margelle ; rend la case où il s'est arrêté.
fn push(s: &mut Session, o: [i32; 3], r: &Rink, b: (i32, i32), d: (i32, i32)) -> (i32, i32) {
    let me = (b.0 - d.0, b.1 - d.1); let inside = me.0 >= 0 && me.1 >= 0 && me.0 < r.size.0 && me.1 < r.size.1; let c = r.cell(me);
    put(s, o, [c[0], if inside { 0 } else { 1 }, c[2]]); run(s, 0.02); put(s, o, [c[0], if inside { 0 } else { 1 }, c[2]]);
    let mut q = b; loop { let n = (q.0 + d.0, q.1 + d.1); if n.0 < 0 || n.1 < 0 || n.0 >= r.size.0 || n.1 >= r.size.1 || name(s, at(o, r.cell(n))) != "air" { break; } q = n; let c = r.cell(q); if name(s, at(o, [c[0], -1, c[2]])) != "ice" { break; } }
    assert!(s.push_at(at(o, r.cell(b))), "un bloc en {b:?}"); run(s, 0.05);
    assert_eq!(name(s, at(o, r.cell(b))), "air", "le bloc a quitté {b:?}"); assert_eq!(name(s, at(o, r.cell(q))), "push_block", "le bloc s'est arrêté en {q:?}"); q
}
fn replay(s: &mut Session, o: [i32; 3], r: &Rink, moves: &[((i32, i32), (i32, i32))]) { for (b, d) in moves { push(s, o, r, *b, *d); } }
const W_SOL: &[((i32, i32), (i32, i32))] = &[((7, 8), (-1, 0)), ((6, 8), (0, -1)), ((6, 6), (-1, 0)), ((4, 6), (0, -1)), ((4, 0), (1, 0)), ((5, 0), (0, 1)), ((5, 1), (-1, 0)), ((1, 1), (0, 1)), ((1, 8), (-1, 0)), ((0, 8), (0, -1)), ((0, 6), (1, 0))];
const E_SOL: &[((i32, i32), (i32, i32))] = &[((1, 4), (0, 1)), ((0, 4), (1, 0)), ((1, 8), (-1, 0)), ((0, 8), (0, -1)), ((0, 6), (1, 0)), ((2, 4), (1, 0)), ((3, 4), (0, -1)), ((3, 3), (1, 0)), ((8, 6), (0, -1)), ((8, 3), (-1, 0)), ((0, 3), (0, 1)), ((0, 4), (1, 0)), ((8, 4), (-1, 0))];
const M_SOL: &[((i32, i32), (i32, i32))] = &[((7, 1), (0, 1)), ((7, 4), (-1, 0)), ((3, 4), (0, -1)), ((3, 3), (-1, 0)), ((0, 3), (0, -1)), ((0, 2), (1, 0)), ((2, 2), (0, -1)), ((2, 0), (1, 0)), ((4, 0), (0, 1))];

#[test]
fn les_blocs_glissent_les_trois_bassins_la_reine_des_glaces() {
    let (mut s, o) = world();
    // du perron, on entre dans la nef ; l'estrade et le Cœur de l'Hiver sont hors d'atteinte
    let r = look(&s, o, [0, 0, -9]); assert!(r.contains(&at(o, [0, 1, 15])), "dans la nef"); assert!(!r.contains(&at(o, [0, 2, 10])), "pas sur l'estrade"); assert!(!r.contains(&at(o, [0, 1, 26])), "le Cœur de l'Hiver est clos");
    for rk in [&gl::WEST, &gl::EAST, &gl::MID] { assert!(r.contains(&at(o, [rk.cell((1, 1))[0], 0, rk.cell((1, 1))[2]])) || rk.pillars.contains(&(1, 1)), "on descend dans le bassin"); }
    // un bloc file sur la glace jusqu'au premier obstacle
    let q = push(&mut s, o, &gl::WEST, (7, 8), (-1, 0)); assert_eq!(q, (6, 8), "arrêté par le pilier (5, 8)"); s.notes.clear();
    let q = push(&mut s, o, &gl::WEST, q, (0, -1)); assert_eq!(q, (6, 6), "il a glissé de deux cases, jusqu'au pilier (6, 5)"); assert!(s.notes.iter().any(|t| t.contains("glisse")));
    // un bloc coincé : on sort du bassin, il revient
    put(&mut s, o, [0, 1, -3]); run(&mut s, 1.5); assert_eq!(name(&s, at(o, gl::WEST.cell((7, 8)))), "push_block", "le bloc est revenu"); assert_eq!(name(&s, at(o, gl::WEST.cell((6, 6)))), "air");
    // le joueur sur la dalle ne compte pas
    let t = gl::WEST.cell(gl::WEST.targets[0]); put(&mut s, o, [t[0], 0, t[2]]); run(&mut s, 0.3); assert!(chain(&s, o, -1), "seul un bloc compte");
    // l'ouest
    replay(&mut s, o, &gl::WEST, W_SOL); run(&mut s, 0.2); assert!(!chain(&s, o, -1) && chain(&s, o, 0) && chain(&s, o, 1), "la chaîne de l'ouest tombe");
    put(&mut s, o, [0, 1, -3]); run(&mut s, 1.5); assert_eq!(name(&s, at(o, t)), "push_block", "résolu, le bloc reste");
    // l'est
    replay(&mut s, o, &gl::EAST, E_SOL); run(&mut s, 0.2); assert!(!chain(&s, o, 0), "la chaîne de l'est tombe");
    // le milieu : le bloc au pied de la brèche, on grimpe, le levier
    assert!(!look(&s, o, [0, 0, 5]).contains(&at(o, [0, 2, 10])));
    replay(&mut s, o, &gl::MID, M_SOL); let n = gl::MID.cell(gl::NOTCH); assert_eq!(name(&s, at(o, n)), "push_block");
    assert!(look(&s, o, [1, 0, 5]).contains(&at(o, [0, 2, 10])), "par le bloc, sur l'estrade"); put(&mut s, o, [0, 2, 10]); assert!(s.lever_at(at(o, gl::LEVER))); run(&mut s, 0.2); assert!(!chain(&s, o, 1), "la chaîne de l'estrade tombe");
    // la porte du sud s'ouvre ; la Reine des glaces
    assert!((-1..=1).all(|u| name(&s, at(o, [u, 2, gl::GATE_W])) == "air")); assert!(look(&s, o, [0, 1, 15]).contains(&at(o, [0, 1, 26])), "au Cœur de l'Hiver");
    let sp = alw_core::hostile::species(gl::BOSS).unwrap(); s.horde.foes.clear(); put(&mut s, o, [0, 1, 23]); run(&mut s, 0.3);
    let i = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("la Reine des glaces se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[i].hp);
    assert!(name(&s, at(o, [0, 2, gl::GATE_W])).starts_with("iron_bars"), "la grille retombe");
    s.horde.foes[i].hp = 1.0; let me = s.player.p; s.horde.strike(i, 99.0, me, "sword", false); run(&mut s, 1.0); assert_eq!(name(&s, at(o, [0, 2, gl::GATE_W])), "air");
    assert!(s.loose.iter().any(|l| l.item == "coeur_glace") || s.inv.iter().flatten().any(|st| st.item == "coeur_glace"), "le Cœur de glace");
    let c = at(o, gl::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin"));
    // la sauvegarde garde les dalles à blocs
    let t = s.save_text(); let mut b = { let y = alw_core::gen::FLAT_Y; let mut b = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [1200.5, y as f64 + 2.0, 1200.5]); ready(&mut b); b };
    b.load_text(&t).unwrap(); assert_eq!(b.mech.wires.iter().map(|w| w.blocks.clone()).collect::<Vec<_>>(), s.mech.wires.iter().map(|w| w.blocks.clone()).collect::<Vec<_>>());
}
