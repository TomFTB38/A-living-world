//! L'Épave du navire-fantôme : la marée monte et descend dans la cale (on n'atteint l'écoutille qu'à la nage, mer haute) ; les
//! canons fantômes tirent leurs boulets en travers de l'entrepont ; la grand-vergue et son levier déploient la voile et libèrent
//! la porte de la cabine ; le Capitaine. On joue tout avec le solveur de chemins.
use alw_core::{blocks::*, epave::{self as ep}, player::*, sanctuaire::reach, session::*, stream::*};
use std::collections::BTreeSet;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn session() -> Session { let y = alw_core::gen::FLAT_Y; let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [2800.5, y as f64 + 2.0, 2785.5]); ready(&mut s); s }
fn world() -> (Session, [i32; 3]) {
    let mut s = session(); s.set_creative(false); s.hour = 12.0; let y = alw_core::gen::FLAT_Y;
    run(&mut s, 0.1); s.wild.all.clear(); s.horde.foes.clear(); let o = [2800, y, 2800]; s.build_wreck(o); let _ = s.sync(); put(&mut s, o, [-10, 0, -3]); run(&mut s, 0.1); s.notes.clear(); (s, o)
}
fn at(o: [i32; 3], q: [i32; 3]) -> [i32; 3] { [o[0] + q[0], o[1] + 1 + q[1], o[2] + q[2]] }
fn name(s: &Session, p: [i32; 3]) -> &'static str { BLOCKS[s.world.get(p[0], p[1], p[2]).unwrap_or(0) as usize].name }
fn put(s: &mut Session, o: [i32; 3], q: [i32; 3]) { let c = at(o, q); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; s.player.v = [0.0; 3]; }
fn run(s: &mut Session, secs: f64) { for _ in 0..(secs / DT).ceil() as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } }
fn look(s: &Session, o: [i32; 3], q: [i32; 3]) -> BTreeSet<[i32; 3]> { reach(&s.world, at(o, q), at(o, [-15, -1, -31]), at(o, [15, 30, 29])) }
fn water(s: &Session, o: [i32; 3], v: i32) -> bool { name(s, at(o, [2, v, -6])) == "water" }

#[test]
fn la_coque_est_close() {
    assert!(ep::beam(ep::BOW + 1) <= 2, "une proue fine"); assert!((7..=8).contains(&ep::beam(0)), "un maître-couple large"); assert!((5..=6).contains(&ep::beam(ep::STERN)), "une poupe carrée");
    assert!(ep::shell_r(0, -1) < ep::shell_r(0, 0) && ep::shell_r(0, 0) < ep::shell_r(0, 3), "des fonds qui se rentrent vers la quille");
    // étanche : chaque case du dedans ne touche que du dedans ou la coque (jusqu'au pont ; au-dessus, il est à ciel ouvert)
    for w in ep::BOW..=ep::STERN { for v in 0..=ep::DECK { for u in -9..=9 { if !ep::interior(u, w, v) { continue; }
        for (du, dv, dw) in [(1, 0, 0), (-1, 0, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)] { let (nu, nv, nw) = (u + du, v + dv, w + dw); assert!(ep::interior(nu, nw, nv) || (nw >= ep::BOW && nw <= ep::STERN && nv >= -1 && nu.abs() <= ep::shell_r(nw, nv)), "une fuite en {u} {v} {w} vers {nu} {nv} {nw}"); } } } }
    for k in 0..2u8 { assert!(ep::sail(k).iter().all(|(c, _)| c[1] > ep::DECK + 1 || k == 0), "la voile ne barre pas le pont"); }
}

#[test]
fn maree_canons_vergue_et_le_capitaine() {
    let (mut s, o) = world();
    // — la cale, à marée basse —
    let hold = [0, 1, -6]; let gun = [3, ep::GUN + 1, ep::HATCH.1]; let r = look(&s, o, [-10, 0, -3]); assert!(r.contains(&at(o, hold)), "par la brèche, dans la cale"); assert!(!r.contains(&at(o, gun)), "l'écoutille est trop haute");
    assert!(!water(&s, o, 1), "la cale est sèche"); let i = s.mech.tides.iter().position(|t| t.name.ends_with("maree")).unwrap(); assert!(!s.mech.tides[i].up());
    // la marée monte : la cale se remplit, on nage jusqu'à l'écoutille
    put(&mut s, o, hold); let wait = ep::TIDE_PERIOD * (1.0 - ep::TIDE_HIGH) + 2.0; run(&mut s, wait); assert!(s.mech.tides[i].up(), "la mer est haute"); assert!(s.notes.iter().any(|t| t.contains("marée monte")), "{:?}", s.notes);
    for v in 1..ep::GUN { assert!(water(&s, o, v), "l'eau monte jusqu'à {v}"); } assert!(!water(&s, o, ep::GUN + 1));
    assert!(look(&s, o, hold).contains(&at(o, gun)), "à la nage, jusqu'à l'entrepont");
    // elle redescend
    run(&mut s, ep::TIDE_PERIOD * ep::TIDE_HIGH + 2.0); assert!(!water(&s, o, 1), "la mer s'est retirée"); assert!(s.notes.iter().any(|t| t.contains("se retire")));
    // — l'entrepont — les canons tirent leurs boulets en travers ; l'échelle de la proue mène au pont
    let deck = [2, ep::DECK + 1, -10]; assert!(look(&s, o, gun).contains(&at(o, deck)), "de l'entrepont, l'échelle de la proue mène au pont"); assert!(!look(&s, o, gun).contains(&at(o, [0, ep::DECK + 1, 15])), "la cabine est barrée");
    put(&mut s, o, [0, ep::GUN + 1, ep::CANNONS[2]]); s.horde.shots.clear(); s.vit = [100.0; 3]; let mut hit = false; let mut balls = false;
    for _ in 0..(4.0 / DT) as usize { s.tick(DT, Input::default(), Actions::default()); balls |= s.horde.shots.iter().any(|sh| sh.4.contains("boulet")); if s.vit[0] < 95.0 { hit = true; } s.vit[1] = 100.0; s.vit[2] = 100.0; if hit { break; } put(&mut s, o, [0, ep::GUN + 1, ep::CANNONS[2]]); }
    assert!(balls, "les canons tirent des boulets"); assert!(hit, "un boulet frappe qui se tient dans la ligne de tir");
    // — la vergue — on grimpe au mât, on saute le trou, le levier du bout ; la voile se déploie, la porte cède
    let tip = [ep::YARD_LEVER[0] - 1, ep::YARD + 1, ep::YARD_W]; let r = look(&s, o, deck); assert!(r.contains(&at(o, tip)), "au bout de la vergue"); assert!(r.contains(&at(o, [ep::YARD_GAP + 1, ep::YARD + 1, ep::YARD_W])), "par-dessus le trou");
    assert!(ep::CABIN_DOOR.iter().all(|c| name(&s, at(o, *c)) == "great_chain")); put(&mut s, o, tip); assert!(s.lever_at(at(o, ep::YARD_LEVER))); run(&mut s, 0.2);
    for (c, b) in ep::sail(1) { assert_eq!(name(&s, at(o, c)), b, "la voile déployée"); } assert!(ep::CABIN_DOOR.iter().all(|c| name(&s, at(o, *c)) == "air"), "la porte de la cabine cède");
    assert!(look(&s, o, deck).contains(&at(o, [0, ep::DECK + 1, 15])), "dans la cabine");
    for v in ep::DECK + 1..ep::POOP { assert_eq!(name(&s, at(o, [0, v, 16])), "air", "une cabine haute de quatre blocs : le Capitaine y tient debout"); }
    // — le Capitaine —
    let sp = alw_core::hostile::species(ep::BOSS).unwrap(); s.horde.foes.clear(); put(&mut s, o, [-3, ep::DECK + 1, 14]); run(&mut s, 0.3);
    let k = s.horde.foes.iter().position(|f| f.sp == sp && f.dead == 0.0).expect("le Capitaine se lève"); println!("{:?} — {:.0} PV", s.notes.last(), s.horde.foes[k].hp);
    assert_eq!(name(&s, at(o, ep::CABIN_DOOR[0])), "great_chain", "la porte se referme");
    s.horde.foes[k].hp = 1.0; let me = s.player.p; s.horde.strike(k, 99.0, me, "sword", false); run(&mut s, 1.0); assert_eq!(name(&s, at(o, ep::CABIN_DOOR[0])), "air");
    assert!(s.loose.iter().any(|l| l.item == "boussole_capitaine") || s.inv.iter().flatten().any(|st| st.item == "boussole_capitaine"), "la Boussole du Capitaine");
    let c = at(o, ep::CHEST); s.open_chest(c); assert!(s.chests[&c].iter().flatten().any(|st| st.item == "worn_coin")); s.close_chest();
    // la sauvegarde garde la marée
    let t = s.save_text(); let mut b = session(); b.load_text(&t).unwrap(); assert_eq!(b.mech.tides.len(), 1); assert!((b.mech.tides[0].t - s.mech.tides[0].t).abs() < 1e-6); assert_eq!(b.mech.tides[0].name, s.mech.tides[0].name);
}
