//! Les barques : on la pose sur l'eau, on monte, on rame bien plus vite qu'on ne nage, on vire, la rive l'arrête, on
//! descend sur la berge, on la range ; elle se sauve avec la partie.
use alw_core::{bateaux, blocks::*, player::*, session::*, stream::*};
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
/// Un étang de 60 × 12 blocs, deux de fond, dans le monde plat ; on se tient sur la berge sud, regardant vers le nord.
fn pond() -> (Session, [i32; 3]) {
    let y = alw_core::gen::FLAT_Y; let o = [2000, y, 1960];
    let mut s = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [o[0] as f64 + 0.5, y as f64 + 1.0, o[2] as f64 + 30.5]); ready(&mut s);
    s.set_creative(false); s.hour = 12.0; for _ in 0..6 { s.tick(DT, Input::default(), Actions::default()); } s.wild.all.clear(); s.horde.foes.clear(); s.notes.clear(); s.skip_guide();
    let mut e = Vec::new(); for x in -6..=5 { for z in -30..=29 { for dy in -1..=0 { e.push((o[0] + x, y + dy, o[2] + z, B::WATER)); } for dy in 1..=3 { e.push((o[0] + x, y + dy, o[2] + z, 0)); } } }
    s.world.edit_many(&e); ready(&mut s); (s, o)
}
fn aim(s: &mut Session, from: [f64; 3], to: [f64; 3]) { s.eye = from; let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]]; let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt(); s.look = [d[0] / l, d[1] / l, d[2] / l]; s.target = None; }
fn row(s: &mut Session, i: Input, secs: f64) { for _ in 0..(secs / DT) as usize { s.tick(DT, i, Actions::default()); } }

#[test]
fn ramer_en_barque() {
    let (mut s, o) = pond(); let y = o[1] as f64;
    s.inv = [None; INV_SIZE]; s.sel = 0; s.inv[0] = Some(Stack::new("boat", None));
    // Sur la terre ferme, elle ne se pose pas (et reste dans le sac).
    s.player.p = [o[0] as f64 + 0.5, y + 1.0, o[2] as f64 + 32.5]; aim(&mut s, [o[0] as f64 + 0.5, y + 2.6, o[2] as f64 + 32.5], [o[0] as f64 + 0.5, y + 1.0, o[2] as f64 + 34.5]);
    assert_eq!(s.boat_click(), Some(true)); assert!(s.boats.is_empty() && s.inv[0].is_some(), "pas sur l'herbe"); s.use_t = 0.0;
    // Sur l'eau : elle flotte, au ras de l'eau, face au regard.
    s.player.p = [o[0] as f64 + 0.5, y + 1.0, o[2] as f64 + 31.5]; s.player.yaw = std::f64::consts::PI; // vers -z, le nord
    aim(&mut s, [o[0] as f64 + 0.5, y + 2.6, o[2] as f64 + 31.5], [o[0] as f64 + 0.5, y + 0.5, o[2] as f64 + 27.5]);
    assert_eq!(s.boat_click(), Some(true)); assert_eq!(s.boats.len(), 1, "{:?}", s.notes.last()); assert!(s.inv[0].is_none(), "la barque a quitté le sac");
    let b0 = s.boats[0]; assert!((b0.p[1] - (y + 0.9)).abs() < 1e-9, "au ras de l'eau : {}", b0.p[1]);
    // On monte : on est assis dedans.
    s.use_t = 0.0; aim(&mut s, [o[0] as f64 + 0.5, y + 2.6, o[2] as f64 + 31.5], b0.p); assert_eq!(s.boat_click(), Some(true)); assert_eq!(s.boat, Some(0));
    // On rame quatre secondes vers le nord : bien plus loin qu'à la nage (le nageur fait à peine 3 m/s).
    let p0 = s.boats[0].p; row(&mut s, Input { z: 1.0, ..Default::default() }, 4.0); let p1 = s.boats[0].p; let d = (p1[0] - p0[0]).hypot(p1[2] - p0[2]);
    println!("4 s de rame : {d:.1} m"); assert!(d > 14.0, "la barque avance : {d:.1} m"); assert!(p1[2] < p0[2], "vers le nord");
    assert!((s.player.p[0] - p1[0]).abs() < 1e-9 && (s.player.p[2] - p1[2]).abs() < 1e-9, "le rameur suit sa barque"); assert!(s.player.p[1] > y, "assis au sec, pas dans l'eau");
    // Elle vire : une seconde à droite.
    let yaw = s.boats[0].yaw; row(&mut s, Input { x: 1.0, ..Default::default() }, 1.0); assert!((s.boats[0].yaw - yaw).abs() > 1.2, "elle vire");
    s.boats[0].yaw = std::f64::consts::PI; // de nouveau vers le nord
    // Droit sur la rive nord : elle s'arrête contre elle, sans monter sur l'herbe.
    row(&mut s, Input { z: 1.0, ..Default::default() }, 12.0); let p = s.boats[0].p; assert!(p[2] > o[2] as f64 - 30.0 + bateaux::HALF_L - 0.6, "arrêtée par la rive : z = {:.2}", p[2]);
    assert!((p[1] - (y + 0.9)).abs() < 0.05, "toujours à flot"); assert_eq!(s.boats[0].v, 0.0);
    // On descend : sur la berge.
    row(&mut s, Input { crouch: true, ..Default::default() }, 0.1); assert_eq!(s.boat, None); let q = s.player.p;
    assert!(s.world.get(q[0].floor() as i32, q[1].floor() as i32 - 1, q[2].floor() as i32).is_some_and(|b| is_solid(b) && !is_water(b)), "pied à terre : {q:?}");
    row(&mut s, Input::default(), 1.0); assert!((s.boats[0].p[1] - (y + 0.9)).abs() < 0.05, "la barque vide flotte toujours");
    // Elle se sauve avec la partie.
    let t = s.save_text(); let mut l = Session::new(alw_core::gen::FLAT | 0x123, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 60.0, 0.5]); l.load_text(&t).unwrap();
    assert_eq!(l.boats.len(), 1); assert!((l.boats[0].p[2] - s.boats[0].p[2]).abs() < 1e-9);
    // Accroupi, un clic droit la range dans le sac.
    let bp = s.boats[0].p; s.player.crouch = true; s.use_t = 0.0; aim(&mut s, [bp[0] + 1.5, bp[1] + 1.8, bp[2] + 1.5], bp); assert_eq!(s.boat_click(), Some(true));
    assert!(s.boats.is_empty()); assert!(s.inv.iter().flatten().any(|st| st.item == "boat"), "rangée");
}

#[test]
fn la_barque_se_dessine_et_se_fabrique() {
    let md = alw_core::actors::Models::build(); let r = md.part("boat"); assert!(r.count > 300, "le modèle de la barque : {} indices", r.count);
    let px = (0..56 * 56).filter(|k| alw_core::icons::look("boat", (k % 56) as f64 / 56.0, (k / 56) as f64 / 56.0).is_some()).count(); assert!(px > 400, "l'icône : {px}");
    let rc = alw_core::survival::recipes().iter().find(|r| r.out == "boat").expect("la recette"); assert!(format!("{:?}", rc.at).to_lowercase().contains("etabli") || format!("{:?}", rc.at).to_lowercase().contains("bench"), "{:?}", rc.at);
    let b = bateaux::Boat::new([0.0, 0.0, 0.0], 0.7); assert!(bateaux::boat_ray(&b, [0.0, 3.0, 0.0], [0.0, -1.0, 0.0]).is_some()); assert!(bateaux::boat_ray(&b, [3.0, 3.0, 0.0], [0.0, -1.0, 0.0]).is_none());
    // en long : la pointe est touchée, à 1,4 m dans l'axe de la barque
    assert!(bateaux::boat_ray(&b, [0.7f64.sin() * 1.35, 3.0, 0.7f64.cos() * 1.35], [0.0, -1.0, 0.0]).is_some());
}

#[test]
fn les_rames_nagent() {
    use bateaux::{blade_tip, oar_pose, Boat};
    // La pose : en nageant, la pale plonge pendant la poussée (de l'attaque, devant, au dégagé, derrière) et remonte au
    // retour ; au repos, les rames sont rentrées, pales hors de l'eau, vers l'arrière.
    let mut b = Boat::new([0.0, 10.9, 0.0], 0.0); b.pull = [1.0; 2]; let water = 10.9;
    for k in 0..24 { let ph = k as f64 / 24.0 * std::f64::consts::TAU; b.oar = [ph; 2]; for side in 0..2 { let t = blade_tip(&b, side);
        if ph > 0.35 && ph < std::f64::consts::PI - 0.35 { assert!(t[1] < water - 0.1, "poussée ({ph:.2}) : la pale dans l'eau, {:.2}", t[1]); }
        if ph > std::f64::consts::PI + 0.35 && ph < std::f64::consts::TAU - 0.35 { assert!(t[1] > water - 0.05, "retour ({ph:.2}) : la pale hors de l'eau, {:.2}", t[1]); }
        assert!((t[0] < 0.0) == (side == 0), "chaque rame de son bord"); } }
    b.oar = [0.0; 2]; let front = blade_tip(&b, 1)[2]; b.oar = [std::f64::consts::PI; 2]; let back = blade_tip(&b, 1)[2]; assert!(front > back + 1.0, "la pale va de l'avant ({front:.2}) vers l'arrière ({back:.2})");
    b.pull = [0.0; 2]; for side in 0..2 { let t = blade_tip(&b, side); assert!(t[1] > water && t[2] < 0.0, "au repos : rentrée, vers l'arrière, au sec {t:?}"); }
    assert_eq!(oar_pose(1.3, 0.0), oar_pose(4.0, 0.0), "au repos, le coup n'importe plus");
    // En jeu : les deux rames battent quand on avance, en cadence ; pour virer à droite, la gauche pousse et la droite recule ;
    // lâchées, elles se rentrent.
    let (mut s, o) = pond(); let y = o[1] as f64; s.boats = vec![Boat::new([o[0] as f64 + 0.5, y + 0.9, o[2] as f64], std::f64::consts::PI)]; s.boat = Some(0); s.player.p = [o[0] as f64 + 0.5, y + 0.7, o[2] as f64];
    row(&mut s, Input { z: 1.0, ..Default::default() }, 0.5); let b = s.boats[0]; assert!(b.pull[0] > 0.99 && b.pull[1] > 0.99, "les rames sorties"); assert!((b.oar[0] - b.oar[1]).abs() < 1e-9, "en cadence");
    assert!((b.oar[0] - 0.5 * std::f64::consts::TAU * bateaux::STROKES).abs() < 0.05, "un coup par seconde environ : {:.2}", b.oar[0]); assert!(s.anim.row.is_some(), "le rameur tient ses rames");
    let o0 = s.boats[0].oar; row(&mut s, Input { x: 1.0, ..Default::default() }, 0.2); let o1 = s.boats[0].oar; let fwd = |a: f64, b: f64| (b - a).rem_euclid(std::f64::consts::TAU) < 2.0;
    assert!(fwd(o0[0], o1[0]) && !fwd(o0[1], o1[1]), "à droite : la gauche pousse, la droite recule {o0:?} → {o1:?}");
    row(&mut s, Input::default(), 0.6); assert!(s.boats[0].pull.iter().all(|p| *p < 0.01), "lâchées, elles se rentrent");
    row(&mut s, Input { crouch: true, ..Default::default() }, 0.1); assert!(s.anim.row.is_none(), "hors de la barque, les bras sont libres");
}
