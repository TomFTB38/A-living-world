//! Combat : coup lourd en tenant le bouton, garde au bouclier, parade au bon moment, bouclier qui s'use.
use alw_core::{blocks::*, player::*, session::*, stream::*, survival::*, wild::Kind};
const DT: f64 = 1.0 / 60.0;
fn none() -> Input { Input::default() }
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn look_at(s: &mut Session, q: [f64; 3]) { let e = s.player.eye(); let d = [q[0] - e[0], q[1] - e[1], q[2] - e[2]]; s.player.yaw = d[0].atan2(d[2]); s.player.pitch = (d[1] / d[0].hypot(d[2])).atan(); }
fn site() -> (Session, i32) {
    let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, 120.0, 0.5]); ready(&mut s); s.third = false; s.inv = [None; INV_SIZE];
    let y = alw_core::gen::WorldGen::new(20261104).column(0.5, 0.5).h.floor() as i32 + 3; let mut e = Vec::new();
    for x in -12..=12 { for z in -12..=12 { e.push((x, y - 1, z, B::STONE)); for h in 0..6 { e.push((x, y + h, z, 0)); } } } s.world.edit_many(&e);
    s.player.p = [0.5, y as f64, 0.5]; for _ in 0..20 { s.tick(DT, none(), Actions::default()); } s.wild.all.clear(); s.fauna.deer.clear(); (s, y)
}
fn wolf(k: &Kind) -> bool { *k == Kind::Wolf }
/// Six secondes face à un loup, la nuit : `guard` dit comment on tient le clic droit à chaque instant.
fn fight(shield: bool, guard: &dyn Fn(f64) -> bool) -> (f64, Session) {
    let (mut s, y) = site(); s.hour = 23.0; s.stamina = 100.0; if shield { s.equip[5] = Some(Stack::new("iron_shield", None)); }
    s.wild.put(Kind::Wolf, [0.5, y as f64, 6.5], 3.14, false); let mut t = 0.0;
    while t < 8.0 { if let Some(w) = s.wild.all.iter().find(|c| wolf(&c.kind)) { let p = w.p; look_at(&mut s, [p[0], p[1] + 0.5, p[2]]); } s.stamina = s.stamina.max(60.0); s.tick(DT, none(), Actions { guard: guard(t), ..Default::default() }); s.wild.all.retain(|c| wolf(&c.kind)); t += DT; }
    (s.vit[0], s)
}

#[test]
fn coup_lourd() {
    let (mut s, y) = site(); s.hour = 12.0; s.wild.put(Kind::Boar, [0.5, y as f64, 2.4], 3.14, false); s.wild.all.retain(|c| c.kind == Kind::Boar);
    let hp = |s: &Session| s.wild.all.iter().find(|c| c.kind == Kind::Boar).map_or(0.0, |c| c.hp);
    // La bête est tenue en place devant soi : on mesure les coups, pas la poursuite.
    let aim = move |s: &mut Session| { if let Some(b) = s.wild.all.iter_mut().find(|c| c.kind == Kind::Boar) { b.p = [0.5, y as f64, 2.4]; } look_at(s, [0.5, y as f64 + 0.4, 2.4]); };
    aim(&mut s); s.tick(0.0, none(), Actions::default()); assert!(s.prey.is_some(), "le sanglier est visé"); let h0 = hp(&s); let st0 = s.stamina;
    s.tick(DT, none(), Actions { mine: true, ..Default::default() }); let light = h0 - hp(&s); assert!(light > 0.5, "un coup tout de suite au clic : {light}"); let cost_light = st0 - s.stamina;
    // On garde le bouton : rien de plus jusqu'à la charge pleine, puis un coup bien plus fort.
    let (h1, st1) = (hp(&s), s.stamina); let mut t = 0.0;
    while t < HEAVY_T - 0.1 { aim(&mut s); s.tick(DT, none(), Actions { mine: true, ..Default::default() }); t += DT; } assert_eq!(hp(&s), h1, "pas de coup pendant la charge"); assert!(s.charge > 0.5);
    while t < HEAVY_T + 0.1 { aim(&mut s); s.tick(DT, none(), Actions { mine: true, ..Default::default() }); t += DT; }
    let heavy = h1 - hp(&s); assert!(heavy > light * 1.8, "coup lourd {heavy} contre {light}"); assert!(st1 - s.stamina > cost_light * 1.8, "le coup lourd coûte plus d'endurance");
    s.tick(DT, none(), Actions::default()); assert!(s.charge < 0.0, "relâcher annule la charge");
}

#[test]
fn garde_et_parade() {
    let (bare, _) = fight(false, &|_| false); assert!(bare < 85.0, "sans bouclier, le loup mord : {bare}");
    let (held, s) = fight(true, &|_| true); assert!(held > bare + 8.0, "la garde tenue arrête une bonne part : {held} contre {bare}");
    assert!(s.equip[5].is_some_and(|st| st.wear > 0), "le bouclier s'use en encaissant");
    // Lever la garde au bon moment pare tout ; on ne peut pas la relever en tapotant sans cesse.
    let (_, s) = fight(true, &|t| t % 0.9 < 0.4); assert!(s.notes.iter().any(|n| n.contains("Parade")), "une parade au moins : {:?}", s.notes);
    // Rien à parer de dos.
    let (mut s, y) = site(); s.hour = 23.0; s.equip[5] = Some(Stack::new("iron_shield", None)); s.wild.put(Kind::Wolf, [0.5, y as f64, 4.5], 3.14, false);
    s.player.yaw = 3.14; for _ in 0..(4.0 / DT) as usize { s.player.yaw = 3.14; s.player.pitch = 0.0; s.tick(DT, none(), Actions { guard: true, ..Default::default() }); s.wild.all.retain(|c| wolf(&c.kind)); }
    assert!(s.vit[0] < 95.0 && !s.notes.iter().any(|n| n.contains("Parade")), "de dos, la garde ne protège pas : {}", s.vit[0]);
}

#[test]
fn boucliers_a_fabriquer() {
    for id in ["wooden_shield", "iron_shield", "steel_shield"] { assert!(recipes().iter().any(|r| r.out == id), "{id}"); let d = alw_core::data::def(id).unwrap(); assert_eq!(d.wear.unwrap().slot, alw_core::data::Slot::Hand); }
    assert!(recipes().iter().find(|r| r.out == "steel_shield").unwrap().artisan.is_some(), "l'acier passe par l'armurier");
    assert!(block_share(11.0) > block_share(3.0));
}

#[test]
fn les_betes_frappees_restent_dans_le_monde() {
    // Un mur juste derrière la bête : les coups la font reculer, jamais à travers lui ni dans le sol.
    let (mut s, y) = site(); s.hour = 12.0; let wall: Vec<_> = (-4..=4).flat_map(|x| (0..3).map(move |h| (x, y + h, 4, B::STONE))).collect(); s.world.edit_many(&wall);
    for kind in [Kind::Boar, Kind::Wolf, Kind::Bear, Kind::Ibex, Kind::Fox] {
        s.wild.all.clear(); s.wild.put(kind, [0.5, y as f64, 2.8], 0.0, false);
        for k in 0..40 {
            if !s.wild.all.is_empty() { if k % 2 == 0 { s.wild.stagger(0, [0.5, y as f64, 0.5], 1.0); } else { s.wild.all[0].hp = 999.0; let _ = s.wild.strike(0, 0.1, [0.5, y as f64, 0.5]); } }
            for _ in 0..6 { s.tick(DT, none(), Actions::default()); s.wild.all.retain(|c| c.kind == kind); }
            for c in s.wild.all.iter().filter(|c| c.p[0].abs() < 11.0 && c.p[2].abs() < 11.0) { let b = s.world.get(c.p[0].floor() as i32, (c.p[1] + 0.2).floor() as i32, c.p[2].floor() as i32); assert!(b.is_some_and(|b| !is_solid(b)), "{kind:?} dans un bloc en {:?}", c.p); assert!(c.p[2] < 4.0, "{kind:?} a traversé le mur : {:?}", c.p); assert!((c.p[1] - y as f64).abs() < 1.1, "{kind:?} hors du sol : {:?}", c.p); }
        }
    }
    assert_eq!(alw_core::wild::flinch(0.3), 0.0); assert!(alw_core::wild::flinch(0.15) > 0.25); assert_eq!(alw_core::wild::flinch(0.0), 0.0);
}
