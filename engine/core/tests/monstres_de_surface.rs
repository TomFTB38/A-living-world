//! Les monstres de surface : sylvain, noyé, archer, fantôme, ver des sables, harpie.
use alw_core::{blocks::B, hostile::{self as h, Ai, Ctx, FoeEv, Horde}, player::Voxels};
/// Sol plat à y = 10 (les pieds à 11), dans la matière voulue ; un mur de pierre de 6 de haut en z = 6 si demandé.
struct Land { top: u16, wall: bool }
impl Voxels for Land { fn get(&self, _x: i32, y: i32, z: i32) -> Option<u16> { Some(if y < 10 { B::STONE } else if y == 10 { self.top } else if self.wall && z == 6 && y < 17 { B::STONE } else { 0 }) } }
const DT: f64 = 1.0 / 30.0;
fn ctx<'a>(feet: [f64; 3], night: f64, light: &'a dyn Fn(i32, i32, i32) -> u8, protect: &'a dyn Fn(f64, f64) -> bool) -> Ctx<'a> {
    Ctx { eye: [feet[0], feet[1] + 1.6, feet[2]], feet, look: [0.0, 0.0, 1.0], noise: 1.0, lit: false, crouch: false, alive: true, night, danger: 2, can_spawn: false, light, protect, main_biome: 0, cave: 0.0, under: None }
}
fn sp(id: &str) -> usize { h::species(id).unwrap_or_else(|| panic!("{id} manque au bestiaire")) }
const ME: [f64; 3] = [0.5, 11.0, 0.5];

#[test]
fn le_bestiaire_de_surface_est_complet_et_se_dessine() {
    let b = h::bestiary(); let md = alw_core::actors::Models::build(); let mut hd = Horde::new(1);
    for id in ["sylvain", "noye", "archer", "fantome", "ver", "harpie"] { let k = sp(id); let s = &b.species[k]; assert!(!s.loot.is_empty() && s.hp > 0.0, "{id}"); let i = hd.put(k, [0.5, 11.0, 5.5], 0.0, false); hd.foes[i].up = 1.0; }
    assert!(b.species[sp("harpie")].fly && b.species[sp("fantome")].ghost && b.species[sp("fantome")].fly && b.species[sp("archer")].bow && b.species[sp("ver")].burrow && b.species[sp("noye")].drag && b.species[sp("noye")].water && b.species[sp("sylvain")].ambush);
    assert!(hd.draws(&md).len() >= 6 * 3, "chaque créature a ses pièces"); hd.shots.push(([0.0, 12.0, 0.0], [0.0, 0.0, 9.0], 0.0, 3.0, "")); let n = hd.draws(&md).len(); hd.shots.clear(); assert_eq!(hd.draws(&md).len(), n - 1, "une flèche en vol se dessine");
}

#[test]
fn l_archer_tire_de_loin_et_ne_vient_pas_au_contact() {
    let (light, protect) = (|_: i32, _: i32, _: i32| 0u8, |_: f64, _: f64| false); let w = Land { top: B::STONE, wall: false };
    let mut hd = Horde::new(2); hd.put(sp("archer"), [0.5, 11.0, 11.5], std::f64::consts::PI, false); let (mut shot, mut hit, mut closest) = (0usize, false, f64::MAX);
    for _ in 0..30 * 25 { hd.update(DT, &w, &ctx(ME, 1.0, &light, &protect)); shot = shot.max(hd.shots.len()); closest = closest.min((hd.foes[0].p[2] - 0.5).abs());
        for e in hd.ev.drain(..) { if let FoeEv::Hit { cause, from, .. } = e { assert!(cause.contains("flèche"), "{cause}"); assert!(from[2] > 0.5, "le coup vient du côté de l'archer : {from:?}"); hit = true; } } }
    assert!(shot >= 1, "aucune flèche tirée"); assert!(hit, "aucune flèche n'a touché un joueur immobile en vingt-cinq secondes"); assert!(closest > 3.0, "l'archer est venu au contact ({closest:.1})");
    // une flèche s'arrête dans un mur
    let wall = Land { top: B::STONE, wall: true }; let mut hd = Horde::new(2); hd.shots.push(([0.5, 12.0, 10.0], [0.0, 0.0, -20.0], 0.0, 5.0, ""));
    for _ in 0..40 { hd.update(DT, &wall, &ctx(ME, 1.0, &light, &protect)); } assert!(hd.shots.is_empty() && hd.ev.iter().all(|e| !matches!(e, FoeEv::Hit { .. })), "la flèche a traversé le mur");
}

#[test]
fn le_fantome_traverse_les_murs_et_se_dissipe_au_jour() {
    let (light, protect) = (|_: i32, _: i32, _: i32| 0u8, |_: f64, _: f64| false); let wall = Land { top: B::STONE, wall: true };
    let mut hd = Horde::new(3); hd.put(sp("fantome"), [0.5, 11.6, 12.5], std::f64::consts::PI, false); let mut hit = false;
    for _ in 0..30 * 20 { hd.update(DT, &wall, &ctx(ME, 1.0, &light, &protect)); for e in hd.ev.drain(..) { if let FoeEv::Hit { cause, .. } = e { assert!(cause.contains("fantôme")); hit = true; } } if hit { break; } }
    assert!(hit, "le mur a arrêté le fantôme : {:?}", hd.foes.first().map(|f| f.p));
    let mut died = false; for _ in 0..90 { hd.update(DT, &wall, &ctx(ME, 0.0, &light, &protect)); died |= hd.ev.drain(..).any(|e| matches!(e, FoeEv::Die(..))); } assert!(hd.foes.is_empty() && !died, "au jour, il se dissipe sans butin");
}

#[test]
fn la_harpie_plonge_et_remonte() {
    let (light, protect) = (|_: i32, _: i32, _: i32| 0u8, |_: f64, _: f64| false); let w = Land { top: B::STONE, wall: false };
    let mut hd = Horde::new(4); hd.put(sp("harpie"), [0.5, 18.0, 14.5], std::f64::consts::PI, false); let (mut hit, mut low, mut high_after) = (false, f64::MAX, 0.0f64);
    for _ in 0..30 * 30 { hd.update(DT, &w, &ctx(ME, 1.0, &light, &protect)); let y = hd.foes[0].p[1]; low = low.min(y); if hit { high_after = high_after.max(y); }
        for e in hd.ev.drain(..) { if let FoeEv::Hit { cause, .. } = e { assert!(cause.contains("harpie")); hit = true; } } assert!(y > 10.9, "la harpie est passée sous le sol"); }
    assert!(hit, "elle n'a jamais frappé (au plus bas : {low:.1})"); assert!(high_after > 15.0, "après son coup elle remonte : {high_after:.1}");
    // on peut l'abattre : elle tombe au sol
    let k = sp("harpie"); let mut hd = Horde::new(4); hd.put(k, [0.5, 17.0, 4.5], 0.0, false); assert!(hd.strike(0, 99.0, ME, "fleche", false));
    for _ in 0..45 { hd.update(DT, &w, &ctx(ME, 1.0, &light, &protect)); if hd.foes.is_empty() { break; } assert!(hd.foes[0].p[1] <= 17.0); }
}

#[test]
fn le_ver_jaillit_du_sable_et_ne_suit_pas_sur_la_roche() {
    let (light, protect) = (|_: i32, _: i32, _: i32| 0u8, |_: f64, _: f64| false); let sand = Land { top: B::SAND, wall: false }; let k = sp("ver");
    let mut hd = Horde::new(5); hd.put(k, [0.5, 11.0, 12.5], 0.0, false);
    assert!(!hd.strike(0, 50.0, ME, "lance", false) && hd.foes[0].hp == hd.foes[0].max, "sous le sable, on ne peut pas le blesser");
    let (mut hit, mut seen_up) = (false, false);
    for _ in 0..30 * 20 { hd.update(DT, &sand, &ctx(ME, 0.0, &light, &protect)); for e in hd.ev.drain(..) { if let FoeEv::Hit { cause, .. } = e { assert!(cause.contains("ver")); hit = true; } }
        if hd.foes[0].up > 0.0 { seen_up = true; let before = hd.foes[0].hp; hd.strike(0, 5.0, ME, "lance", false); assert!(hd.foes[0].hp < before, "dressé, il se frappe"); break; } }
    assert!(hit && seen_up, "il est venu sous le joueur, a frappé, puis est resté dressé (coup {hit}, dressé {seen_up})");
    // sur la roche, il ne peut pas approcher
    let rock = Land { top: B::STONE, wall: false }; let mut hd = Horde::new(5); hd.put(k, [0.5, 11.0, 12.5], 0.0, false);
    for _ in 0..30 * 8 { hd.update(DT, &rock, &ctx(ME, 0.0, &light, &protect)); } assert!((hd.foes[0].p[2] - 12.5).abs() < 0.01 && hd.ev.iter().all(|e| !matches!(e, FoeEv::Hit { .. })), "le ver a traversé la roche");
}

#[test]
fn le_noye_attire_et_le_sylvain_guette() {
    let (light, protect) = (|_: i32, _: i32, _: i32| 0u8, |_: f64, _: f64| false); let w = Land { top: B::STONE, wall: false };
    let mut hd = Horde::new(6); hd.put(sp("noye"), [0.5, 11.0, 3.5], std::f64::consts::PI, false); let mut pull = None;
    for _ in 0..30 * 12 { hd.update(DT, &w, &ctx(ME, 1.0, &light, &protect)); for e in hd.ev.drain(..) { if let FoeEv::Hit { push, .. } = e { pull = Some(push); } } if pull.is_some() { break; } }
    assert!(pull.is_some_and(|p| p < 0.0), "le coup du noyé tire vers lui : {pull:?}");
    // le sylvain : un arbre parmi les arbres tant qu'on reste loin ; la hache le blesse mieux que l'épée
    let k = sp("sylvain"); let mut hd = Horde::new(7); hd.put(k, [0.5, 11.0, 14.5], std::f64::consts::PI, false);
    for _ in 0..120 { hd.update(DT, &w, &ctx(ME, 1.0, &light, &protect)); } assert_eq!(hd.foes[0].state, Ai::Embuscade); assert!((hd.foes[0].p[2] - 14.5).abs() < 0.01);
    for _ in 0..60 { hd.update(DT, &w, &ctx([0.5, 11.0, 10.5], 1.0, &light, &protect)); } assert_ne!(hd.foes[0].state, Ai::Embuscade, "de près, il s'anime");
    let cut = |weapon: &str| { let mut hd = Horde::new(8); hd.put(k, [0.5, 11.0, 3.5], 0.0, false); let b = hd.foes[0].hp; hd.strike(0, 10.0, ME, weapon, false); b - hd.foes[0].hp }; assert!(cut("hache") > cut("epee") * 1.3);
}

#[test]
fn la_lumiere_tient_le_fantome_a_distance() {
    // une pièce éclairée autour du joueur (lumière forte à moins de cinq pas) : le fantôme tourne autour sans entrer
    let (light, protect) = (|x: i32, _: i32, z: i32| if ((x * x + z * z) as f64).sqrt() < 5.0 { 13u8 } else { 0 }, |_: f64, _: f64| false); let w = Land { top: B::STONE, wall: false };
    let mut hd = Horde::new(9); hd.put(sp("fantome"), [0.5, 11.6, 14.5], std::f64::consts::PI, false);
    for _ in 0..30 * 15 { hd.update(DT, &w, &ctx(ME, 1.0, &light, &protect)); assert!(hd.ev.drain(..).all(|e| !matches!(e, FoeEv::Hit { .. })), "il a frappé dans la lumière"); }
    let f = &hd.foes[0]; assert!((f.p[0] - 0.5).hypot(f.p[2] - 0.5) > 3.5, "il est entré dans la lumière : {:?}", f.p);
}
