use alw_core::{blocks::*, player::Voxels, wild::*};
/// Un sol plat de pierre à y = 9, couvert d'une pellicule de neige de 3 dixièmes en y = 10.
struct Flat;
impl Voxels for Flat { fn get(&self, _x: i32, y: i32, _z: i32) -> Option<u16> { Some(if y <= 9 { B::STONE } else if y == 10 { snow_of(3) } else { 0 }) } }

#[test]
fn les_pas_s_impriment_dans_la_neige() {
    let mut w = Wild::new(1); let mut p = [0.5, 10.0, 0.5];
    for _ in 0..60 { p[2] += 0.1; w.update(0.05, &Flat, p, 1.0, 0.0, 0.0, 0.0, true, [0.0; 6], 0.0); }
    println!("{} empreintes", w.prints.len()); assert!(w.prints.len() >= 7 && w.prints.len() <= 10);
    for pr in &w.prints { assert!((pr.p[1] - 10.304).abs() < 1e-6, "posée sur la neige : {}", pr.p[1]); }
    let sides: Vec<bool> = w.prints.iter().map(|pr| pr.p[0] > 0.5).collect(); assert!(sides.windows(2).all(|s| s[0] != s[1]), "pied gauche, pied droit");
    assert_eq!(snow_at(&Flat, 3.0, 10.0, 3.0), Some(10.3));
}

#[test]
fn le_virevoltant_suit_le_vent_et_le_renard_fuit() {
    let mut w = Wild::new(2); w.put(Kind::Tumble, [0.0, 10.0, 0.0], 0.0, false); w.put(Kind::Fox, [3.0, 10.0, 0.0], 0.0, false);
    let p = [0.0, 10.0, -20.0]; let d0 = 3.0f64.hypot(20.0);
    for _ in 0..200 { w.update(0.05, &Flat, p, 1.0, 0.0, 0.0, 0.0, false, [0.0; 6], 0.0); }
    let t = w.all.iter().find(|c| c.kind == Kind::Tumble).expect("virevoltant"); let f = w.all.iter().find(|c| c.kind == Kind::Fox).expect("renard");
    let moved = t.p[0].hypot(t.p[2]); println!("virevoltant à {:?} (vent {:.2}), renard à {:?}", t.p, w.wind, f.p);
    assert!(moved > 15.0, "le vent l'emporte"); assert!((t.p[0] * w.wind.sin() + t.p[2] * w.wind.cos()) / moved > 0.8, "dans le sens du vent");
    assert!(t.p[1] >= 10.0 - 1e-9, "jamais sous le sol");
    // Le renard n'est pas inquiété à 20 m.
    assert!((f.p[0] - 3.0).hypot(f.p[2]) < 15.0 || (f.p[0]).hypot(f.p[2] + 20.0) > d0);
}

/// Sol plat à y = 9, et un mur de 6 blocs de haut à partir de x = 8.
struct Wall;
impl Voxels for Wall { fn get(&self, x: i32, y: i32, _z: i32) -> Option<u16> { Some(if y <= 9 || (x >= 8 && y <= 15) { B::STONE } else { 0 }) } }
#[test]
fn le_virevoltant_ne_grimpe_pas_aux_murs() {
    let mut w = Wild::new(3); w.put(Kind::Tumble, [0.0, 10.0, 0.0], 0.0, false); let p = [0.0, 10.0, -30.0]; let mut top = 0.0f64;
    for _ in 0..1200 { w.wind = 1.5708; w.update(0.02, &Wall, p, 1.0, 0.0, 0.0, 0.0, false, [0.0; 6], 0.0); if let Some(t) = w.all.iter().find(|c| c.kind == Kind::Tumble) { top = top.max(t.p[1]); assert!(t.p[0] < 8.0, "il traverse le mur : {:?}", t.p); } }
    println!("plus haut : {top}"); assert!(top < 11.5, "il s'envole : {top}");
}
