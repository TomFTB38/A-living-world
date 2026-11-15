//! La vie des grottes : chauves-souris sous les voûtes, ennemis qui naissent dans les salles (ceux de la surface et ceux des
//! profondeurs), jamais sur la colline au-dessus.
use alw_core::{gen::WorldGen, hostile::{self as h, Ctx, Horde}, karst::*, player::{Input, Voxels}, session::*, stream::Preset, wild::Kind};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn in_hall(kind: Under) -> (Session, Hall, WorldGen) {
    let w = WorldGen::new(SEED); let hl = w.nearest_hall_of(0.0, 0.0, 14, Some(kind)).expect("une salle"); let sp = w.hall_spot(&hl, [hl.x, hl.z]).expect("un sol");
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, sp); ready(&mut s); s.third = false; s.player.p = sp;
    for _ in 0..90 { s.tick(1.0 / 30.0, Input::default(), Actions::default()); } (s, hl, w)
}

#[test]
fn les_chauves_souris_peuplent_la_salle() {
    let (mut s, hl, w) = in_hall(Under::Karst);
    assert!(s.cave > 0.6, "sous terre : {:.2}", s.cave); assert_eq!(s.under_kind, Some(Under::Karst), "on se sait dans la salle karstique");
    for _ in 0..30 * 40 { s.vit = [100.0; 3]; s.tick(1.0 / 30.0, Input::default(), Actions::default()); }
    let bats: Vec<_> = s.wild.all.iter().filter(|c| c.kind == Kind::Bat).collect(); assert!(bats.len() >= 2, "des chauves-souris : {}", bats.len());
    for b in &bats {
        let ground = w.column(b.p[0], b.p[2]).h; assert!(b.p[1] < ground - 2.0, "une chauve-souris hors de terre : {:?} (sol {ground})", b.p);
        let cell = s.world.get(b.p[0].floor() as i32, b.p[1].floor() as i32, b.p[2].floor() as i32); assert!(cell.is_some_and(|c| !alw_core::blocks::is_solid(c)), "dans la roche : {:?}", b.p);
    }
    let _ = hl;
}

#[test]
fn les_ennemis_naissent_dans_la_salle() {
    let (s, _, w) = in_hall(Under::Abyss);
    let light = |_: i32, _: i32, _: i32| 0u8; let protect = |_: f64, _: f64| false; let mut hd = Horde::new(11); let feet = s.player.p;
    for _ in 0..30 * 60 { let c = Ctx { eye: [feet[0], feet[1] + 1.6, feet[2]], feet, look: [0.0, -1.0, 0.0], noise: 0.0, lit: false, crouch: true, alive: true, night: 0.0, danger: 2, can_spawn: true, light: &light, protect: &protect, main_biome: 0, cave: 1.0, under: Some("abime") };
        hd.update(1.0 / 30.0, &s.world, &c); if hd.foes.len() >= 4 { break; } }
    assert!(hd.foes.len() >= 2, "en plein jour, sous terre, des ennemis apparaissent : {}", hd.foes.len());
    for f in &hd.foes { let ground = w.column(f.p[0], f.p[2]).h; assert!(f.p[1] < ground - 3.0, "né en surface : {:?} (sol {ground})", f.p); }
    assert!(hd.foes.iter().all(|f| !h::bestiary().species[f.sp].surface), "une créature de surface (harpie, ver, sylvain, noyé) sous terre");
    // les créatures des profondeurs ne sortent jamais à l'air libre ; celles de la surface descendent aussi sous terre
    let bs = h::bestiary(); let deep = bs.species.iter().position(|x| x.id == "abyssale").expect("la bête des profondeurs");
    assert!(!bs.species[deep].caves.is_empty() && bs.species.iter().any(|x| x.id == "rodeur" && x.caves.is_empty()));
    let mut up = Horde::new(5); let flat = Flat;
    for _ in 0..30 * 90 { let c = Ctx { eye: [0.5, 12.6, 0.5], feet: [0.5, 11.0, 0.5], look: [0.0, -1.0, 0.0], noise: 0.0, lit: false, crouch: true, alive: true, night: 1.0, danger: 2, can_spawn: true, light: &light, protect: &protect, main_biome: 0, cave: 0.0, under: None };
        up.update(1.0 / 30.0, &flat, &c); }
    assert!(!up.foes.is_empty()); assert!(up.foes.iter().all(|f| bs.species[f.sp].caves.is_empty()), "une créature des grottes à la surface");
}
struct Flat;
impl Voxels for Flat { fn get(&self, _x: i32, y: i32, _z: i32) -> Option<u16> { Some(if y <= 10 { alw_core::blocks::B::STONE } else { 0 }) } }

// ------------------------------------------------------------------ le premier lot de créatures des profondeurs
use alw_core::hostile::FoeEv;
/// Un sol plat à y = 10 ; de l'eau (un bassin) au-delà de x = 20.
struct Pool;
impl Voxels for Pool { fn get(&self, x: i32, y: i32, _z: i32) -> Option<u16> { use alw_core::blocks::B; Some(if y < 10 { B::STONE } else if y == 10 { if x >= 20 { B::WATER } else { B::STONE } } else { 0 }) } }
fn ctx<'a>(feet: [f64; 3], light: &'a dyn Fn(i32, i32, i32) -> u8, protect: &'a dyn Fn(f64, f64) -> bool) -> Ctx<'a> {
    Ctx { eye: [feet[0], feet[1] + 1.6, feet[2]], feet, look: [0.0, 0.0, 1.0], noise: 0.0, lit: false, crouch: false, alive: true, night: 1.0, danger: 2, can_spawn: false, light, protect, main_biome: 0, cave: 1.0, under: Some("rocheuse") }
}

#[test]
fn les_pieces_des_creatures_existent() {
    let md = alw_core::actors::Models::build();
    for n in ["spdBody", "spdLeg", "glmTorso", "glmHead", "glmArm", "glmLeg", "mgmBody", "mgmHead", "mgmLeg", "ember", "glowbug", "bat", "batWing"] { assert!(md.part(n).count >= 36, "{n} : maillage vide"); }
    // chaque corps se dessine, dans tous ses états, sans pièce manquante
    let mut hd = Horde::new(1); for id in ["araignee", "golem", "magma"] { let sp = h::species(id).unwrap_or_else(|| panic!("{id} manque au bestiaire")); hd.put(sp, [0.5, 11.0, 0.5], 0.0, false); }
    assert!(hd.draws(&md).len() >= 9 + 6 + 8);
}

#[test]
fn le_golem_guette_et_ne_craint_que_le_marteau() {
    let (light, protect) = (|_: i32, _: i32, _: i32| 0u8, |_: f64, _: f64| false); let w = Flat; let sp = h::species("golem").unwrap();
    // à l'affût : immobile tant qu'on reste loin, il se dresse quand on passe près de lui
    let mut hd = Horde::new(2); hd.put(sp, [0.5, 11.0, 12.5], std::f64::consts::PI, false);
    for _ in 0..120 { hd.update(1.0 / 30.0, &w, &ctx([0.5, 11.0, 0.5], &light, &protect)); } assert_eq!(hd.foes[0].state, h::Ai::Embuscade, "de loin, c'est un rocher"); assert!((hd.foes[0].p[2] - 12.5).abs() < 0.01);
    for _ in 0..90 { hd.update(1.0 / 30.0, &w, &ctx([0.5, 11.0, 9.0], &light, &protect)); } assert_ne!(hd.foes[0].state, h::Ai::Embuscade, "de près, il s'éveille");
    // la lame glisse sur la roche, le marteau la brise
    let hit = |weapon: &str| { let mut hd = Horde::new(3); hd.put(sp, [0.5, 11.0, 3.5], 0.0, false); let before = hd.foes[0].hp; hd.strike(0, 10.0, [0.5, 11.0, 0.5], weapon, false); before - hd.foes[0].hp };
    let (sword, hammer) = (hit("epee"), hit("marteau")); assert!(sword < 4.5 && hammer > sword * 1.3, "épée {sword}, marteau {hammer}");
}

#[test]
fn l_araignee_englue_et_le_magma_brule() {
    let (light, protect) = (|_: i32, _: i32, _: i32| 0u8, |_: f64, _: f64| false); let w = Flat;
    // l'araignée : paisible de loin, elle mord de près et sa morsure englue
    let sp = h::species("araignee").unwrap(); let mut hd = Horde::new(4); hd.put(sp, [0.5, 11.0, 12.5], std::f64::consts::PI, false);
    for _ in 0..150 { hd.update(1.0 / 30.0, &w, &ctx([0.5, 11.0, 0.5], &light, &protect)); } assert!(hd.foes[0].p[2] > 6.0, "à douze pas, elle ne charge pas : {:?}", hd.foes[0].p);
    let mut hd = Horde::new(4); hd.put(sp, [0.5, 11.0, 3.0], std::f64::consts::PI, false); let (mut bit, mut snared) = (false, false);
    for _ in 0..30 * 12 { hd.update(1.0 / 30.0, &w, &ctx([0.5, 11.0, 0.5], &light, &protect)); for e in hd.ev.drain(..) { match e { FoeEv::Hit { .. } => bit = true, FoeEv::Snare(t) => { snared = true; assert!(t > 1.0); } _ => {} } } }
    assert!(bit && snared, "morsure {bit}, engluement {snared}");
    // le rampant de magma : il sème des braises en chargeant ; elles brûlent qui marche dessus ; l'eau le tue
    let sp = h::species("magma").unwrap(); let mut hd = Horde::new(5); hd.put(sp, [0.5, 11.0, 14.5], std::f64::consts::PI, false);
    for _ in 0..30 * 4 { hd.update(1.0 / 30.0, &w, &ctx([0.5, 11.0, 0.5], &light, &protect)); hd.ev.clear(); if hd.embers.len() >= 3 { break; } } assert!(hd.embers.len() >= 3, "des braises derrière lui : {}", hd.embers.len());
    let e = hd.embers[0].0; hd.foes.clear(); let mut burnt = false;
    for _ in 0..40 { hd.update(1.0 / 30.0, &w, &ctx(e, &light, &protect)); for ev in hd.ev.drain(..) { if let FoeEv::Hit { cause, .. } = ev { assert!(cause.contains("braises")); burnt = true; } } } assert!(burnt, "la braise ne brûle pas");
    for _ in 0..30 * 6 { hd.update(1.0 / 30.0, &w, &ctx([60.0, 11.0, 0.5], &light, &protect)); } assert!(hd.embers.is_empty(), "les braises s'éteignent");
    let pool = Pool; let mut hd = Horde::new(6); hd.put(sp, [22.5, 10.2, 0.5], 0.0, false); let mut died = false;
    for _ in 0..30 * 6 { hd.update(1.0 / 30.0, &pool, &ctx([0.5, 11.0, 0.5], &light, &protect)); for ev in hd.ev.drain(..) { if matches!(ev, FoeEv::Die(..)) { died = true; } } } assert!(died, "l'eau n'a pas éteint le rampant de magma");
}

#[test]
fn la_toile_ralentit_et_le_scarabee_fait_une_lanterne() {
    use alw_core::blocks::B;
    let (mut s, _, _) = in_hall(Under::Karst); s.wild.all.clear(); let p = s.player.p; let cell = [p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32];
    s.tick(1.0 / 30.0, Input::default(), Actions::default()); assert_eq!(s.player.slow, 0.0);
    s.world.edit(cell[0], cell[1], cell[2], B::COBWEB); s.tick(1.0 / 30.0, Input::default(), Actions::default()); assert!(s.player.slow > 0.4, "pris dans la toile");
    assert!(!alw_core::blocks::is_solid(B::COBWEB) && alw_core::blocks::BLOCKS[B::COBWEB as usize].drop == Some("silk"));
    s.world.edit(cell[0], cell[1], cell[2], 0); s.snare = 2.0; s.tick(1.0 / 30.0, Input::default(), Actions::default()); assert!(s.player.slow > 0.4, "englué par une morsure");
    // le scarabée : il éclaire, on le cueille, et avec du verre il devient un bocal qui luit
    s.snare = 0.0; let before = s.lights(s.player.eye(), 64).len(); s.wild.put(Kind::Glowbug, [p[0] + 1.5, p[1], p[2]], 0.0, false); assert_eq!(s.lights(s.player.eye(), 64).len(), before + 1, "le scarabée éclaire");
    assert_eq!(alw_core::outdoors::wild_loot(Kind::Glowbug), &[("glowbug", 1)]);
    assert!(alw_core::session::item("glow_jar").is_some() && alw_core::session::item("silk").is_some() && alw_core::session::item("ember_gland").is_some());
    assert!(alw_core::blocks::BLOCKS[B::GLOW_JAR as usize].light >= 8 && alw_core::blocks::glow_index(B::GLOW_JAR).is_some());
}
