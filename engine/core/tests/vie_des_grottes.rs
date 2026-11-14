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
