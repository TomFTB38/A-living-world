//! Les créatures montent et descendent les marches d'un bloc : une hostile qui poursuit le joueur sur des terrasses,
//! un renard qui file sur un escalier de terre.
use alw_core::{blocks::B, hostile::{self as h, Ctx, Horde}, player::Voxels, wild::{Kind, Wild}};
/// Des terrasses : le sol monte d'un bloc tous les quatre blocs vers +x, jusqu'à x = 24.
struct Steps;
impl Voxels for Steps { fn get(&self, x: i32, y: i32, _z: i32) -> Option<u16> { let top = 10 + (x.clamp(0, 24) / 4); Some(if y <= top { B::STONE } else { 0 }) } }
fn top(x: f64) -> f64 { (10 + (x.floor() as i32).clamp(0, 24) / 4) as f64 + 1.0 }

#[test]
fn une_hostile_gravit_les_terrasses_et_redescend() {
    let w = Steps; let mut hd = Horde::new(7); let sp = (0..h::bestiary().species.len()).find(|i| h::bestiary().species[*i].id == "brigand").unwrap_or(0);
    hd.put(sp, [1.5, top(1.5), 0.5], std::f64::consts::FRAC_PI_2, false);
    let light = |_: i32, _: i32, _: i32| 0u8; let protect = |_: f64, _: f64| false;
    let mut feet = [26.5, top(26.5), 0.5];
    for i in 0..(70.0 * 30.0) as usize { if i == 450 { feet = [1.5, top(1.5), 4.5]; }
        let c = Ctx { eye: [feet[0], feet[1] + 1.6, feet[2]], feet, look: [1.0, 0.0, 0.0], noise: 1.0, lit: false, crouch: false, alive: true, night: 1.0, danger: 2, can_spawn: false, light: &light, protect: &protect, main_biome: 0 };
        hd.update(1.0 / 30.0, &w, &c); if i == 449 { let f = &hd.foes[0]; assert!(f.p[0] > 16.0 && f.p[1] > top(1.5) + 3.0, "elle n'a pas monté : {:?}", f.p); } }
    let f = &hd.foes[0]; assert!(f.p[0] < 8.0 && f.p[1] < top(1.5) + 2.5, "elle n'est pas redescendue : {:?}", f.p);
}

#[test]
fn un_renard_monte_et_descend_les_marches() {
    let w = Steps; let mut wd = Wild::new(3); wd.put(Kind::Fox, [4.5, top(4.5), 0.5], std::f64::consts::FRAC_PI_2, false);
    let (mut lo, mut hi) = (f64::MAX, f64::MIN);
    for _ in 0..(60.0 * 30.0) as usize { wd.update(1.0 / 30.0, &w, [12.0, 20.0, 40.0], 0.5, 0.0, 0.0, 0.5, true, [0.0; 6]); if let Some(c) = wd.all.first() { lo = lo.min(c.p[1]); hi = hi.max(c.p[1]); } }
    assert!(hi - lo >= 1.9, "le renard reste à sa hauteur : de {lo} à {hi}");
}
