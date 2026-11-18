//! Le bestiaire des brigands : archers, épéistes, cavaliers et chef de guerre. Les camps les rassemblent selon leur puissance, et
//! chacun se bat à sa façon — ce n'est plus seulement une affaire de nombre.
use alw_core::{camps, gen::WorldGen, hostile::{self as h, bestiary, Ai, Body}, player::*, session::*, sites::Lieu, stream::*};
use alw_core::session::item;
const SEED: i32 = 20261104;
const DT: f64 = 1.0 / 30.0;
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(4)); } }
fn run(s: &mut Session, sec: f64, mut each: impl FnMut(&Session)) { for i in 0..(sec / DT) as usize { s.tick(DT, Input::default(), Actions::default()); if i % 30 == 0 { let _ = s.sync(); } each(s); } }
/// Le monde plat : rien ne gêne la course (ni arbre, ni relief), et rien n'y naît au hasard.
fn quiet() -> Session {
    let seed = alw_core::gen::FLAT | 0x123; let y = alw_core::gen::FLAT_Y as f64 + 1.0;
    let mut s = Session::new(seed, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [0.5, y, 0.5]); ready(&mut s); s.third = false; s.hour = 23.5; s.wild.all.clear(); s.horde.foes.clear(); s.vit = [100.0; 3]; s
}
fn sp(id: &str) -> usize { h::species(id).expect(id) }

#[test]
fn quatre_nouveaux_brigands_dans_les_donnees() {
    let b = bestiary();
    for id in ["brigand_archer", "brigand_epeiste", "brigand_cavalier", "brigand_chef"] { let s = &b.species[sp(id)];
        assert_eq!(s.weight, 0.0, "{id} ne naît qu'au camp"); for l in &s.loot { assert!(item(l.0).is_some(), "{id} : butin inconnu {}", l.0); } }
    assert!(b.species[sp("brigand_archer")].bow); assert!(b.species[sp("brigand_epeiste")].guard_up && b.species[sp("brigand_epeiste")].armor > b.species[sp("brigand")].armor);
    assert!(b.species[sp("brigand_cavalier")].charge && b.species[sp("brigand_cavalier")].body == Body::Cavalier);
    let c = &b.species[sp("brigand_chef")]; assert!(c.leader && c.rage && c.siege >= 2.5 && c.hp > 2.0 * b.species[sp("brigand")].hp);
}

#[test]
fn la_troupe_change_avec_la_puissance_du_camp() {
    let count = |v: &[&str], id: &str| v.iter().filter(|w| **w == id).count();
    for l in 1..=5u32 { let r = camps::roster(l, 9, false); println!("bande cran {l} : {r:?} —{}", camps::band_words(&r)); }
    assert!(camps::roster(1, 6, true).iter().all(|w| *w == "brigand"), "au cran 1, des brigands errants");
    assert_eq!(count(&camps::roster(2, 7, true), "brigand_archer"), 1);
    let r3 = camps::roster(3, 8, true); assert!(count(&r3, "brigand_epeiste") >= 2 && r3[0] == "brigand_chef" && count(&r3, "brigand_cavalier") == 0);
    let r5 = camps::roster(5, 12, true); assert!(count(&r5, "brigand_cavalier") == 2 && count(&r5, "brigand_archer") == 3 && count(&r5, "brigand_epeiste") == 3 && count(&r5, "brigand_chef") == 1);
    assert_eq!(count(&camps::roster(3, 6, false), "brigand_chef"), 0, "la bande d'un camp moyen sort sans son chef");
    // au camp : le chef dans la tente, l'archer sur la tour de guet
    let pl = WorldGen::new(SEED).places_near(0.0, 0.0, 2600.0).into_iter().find(|p| p.kind == Lieu::Camp).unwrap();
    let g = camps::garrison(&pl, 5); println!("camp au cran 5 : {:?}", g.iter().map(|x| x.0).collect::<Vec<_>>());
    assert_eq!(g[0].0, "brigand_chef"); assert_eq!(g[7].0, "brigand_archer"); assert_eq!(g.len(), 12);
    assert!(camps::band_words(&r5).contains("chef de guerre"));
}

#[test]
fn chacun_se_bat_a_sa_facon() {
    for (id, d, want) in [("brigand_archer", 9.0, "flèche d'un archer brigand"), ("brigand_epeiste", 3.0, "épéiste"), ("brigand_cavalier", 15.0, "cavalier"), ("brigand_chef", 3.0, "chef de guerre")] {
        let mut s = quiet(); let p = s.player.p; let i = s.horde.put(sp(id), [p[0] + d, p[1], p[2]], -1.57, false); let f = &mut s.horde.foes[i]; f.alert = 1.0; f.seen = Some(p); f.state = Ai::Poursuite;
        let hp0 = s.vit[0]; let mut cause = ""; let mut lost = 0.0; let mut far = 0.0f64; let mut charged = false;
        run(&mut s, 14.0, |s| { if s.vit[0] < hp0 && cause.is_empty() { cause = s.cause; lost = hp0 - s.vit[0]; } if let Some(f) = s.horde.foes.first() { far = far.max((f.p[0] - p[0]).hypot(f.p[2] - p[2])); if f.speed > 9.0 { charged = true; } } });
        println!("{id} : « {cause} » (−{lost:.0}), au plus loin à {far:.1} pas, charge {charged}"); assert!(cause.contains(want), "{id} attaque à sa façon : {cause}");
        if id == "brigand_archer" { assert!(s.horde.foes[0].p[0] - p[0] > 3.0, "l'archer garde ses distances"); }
        if id == "brigand_cavalier" { assert!(charged, "le cavalier charge au galop"); }
    }
}

#[test]
fn l_epeiste_avance_bouclier_leve() {
    let mut s = quiet(); let p = s.player.p; let i = s.horde.put(sp("brigand_epeiste"), [p[0] + 9.0, p[1], p[2]], -1.57, false); let f = &mut s.horde.foes[i]; f.alert = 1.0; f.seen = Some(p); f.state = Ai::Poursuite;
    run(&mut s, 0.8, |_| {}); let f = &s.horde.foes[i]; println!("garde {:.2}, à {:.1} pas", f.guard, (f.p[0] - p[0]).hypot(f.p[2] - p[2])); assert!(f.guard > 0.5, "il approche à couvert");
    // une flèche de face : le bouclier l'arrête ; dans le dos, elle porte
    let hp = s.horde.foes[i].hp; let fp = s.horde.foes[i].p; s.horde.strike(i, 10.0, [fp[0] - 6.0, fp[1] + 1.0, fp[2]], "fleche", false); let front = hp - s.horde.foes[i].hp;
    s.horde.foes[i].guard = 1.0; let hp = s.horde.foes[i].hp; s.horde.strike(i, 10.0, [fp[0] + 6.0, fp[1] + 1.0, fp[2]], "fleche", false); let back = hp - s.horde.foes[i].hp;
    println!("flèche de face −{front:.1}, dans le dos −{back:.1}"); assert!(front < back * 0.4, "le bouclier arrête la flèche de face");
}

#[test]
fn le_chef_tient_ses_hommes_et_les_ameute() {
    let mut s = quiet(); let p = s.player.p;
    let c = s.horde.put(sp("brigand_chef"), [p[0] + 3.0, p[1], p[2]], -1.57, false); { let f = &mut s.horde.foes[c]; f.alert = 1.0; f.seen = Some(p); f.state = Ai::Poursuite; }
    let far = s.horde.put(sp("brigand"), [p[0] - 14.0, p[1], p[2] + 8.0], 3.14, false); s.horde.foes[far].state = Ai::Repos;
    let hurt = s.horde.put(sp("brigand"), [p[0] + 5.0, p[1], p[2] + 2.0], -1.57, false); { let f = &mut s.horde.foes[hurt]; f.hp = 3.0; f.state = Ai::Fuite; }
    run(&mut s, 0.5, |_| {});
    println!("le brigand au repos : {:?} ; le blessé : {:?}", s.horde.foes[far].state, s.horde.foes[hurt].state);
    assert!(matches!(s.horde.foes[far].state, Ai::Detection | Ai::Poursuite | Ai::Combat), "le cri du chef ameute le camp");
    assert!(s.horde.foes[hurt].state != Ai::Fuite, "près du chef, on ne fuit pas");
}

#[test]
fn chacun_a_sa_tenue_et_se_dessine() {
    // toutes leurs pièces existent : au repos, en garde, en visée, en pleine charge, en plein coup, à terre
    let md = alw_core::actors::Models::build(); let mut hd = h::Horde::new(3);
    for (k, id) in ["brigand", "brigand_archer", "brigand_epeiste", "brigand_cavalier", "brigand_chef"].iter().enumerate() {
        for pose in 0..5 { let i = hd.put(sp(id), [k as f64 * 3.0, 11.0, pose as f64 * 3.0], 0.0, false); let f = &mut hd.foes[i];
            match pose { 0 => {} 1 => { f.state = Ai::Combat; f.wind = 0.3; f.guard = 1.0; } 2 => { f.state = Ai::Poursuite; f.speed = 11.0; f.phase = 1.3; } 3 => f.dead = 0.6, _ => f.hit = 0.2 } } }
    let n = hd.draws(&md).len(); let per = n / 25; println!("{n} pièces, {per} par brigand en moyenne"); assert!(per >= 10, "des corps articulés");
    let cav = { let mut one = h::Horde::new(4); one.put(sp("brigand_cavalier"), [0.0, 11.0, 0.0], 0.0, false); one.draws(&md).len() };
    let chef = { let mut one = h::Horde::new(4); one.put(sp("brigand_chef"), [0.0, 11.0, 0.0], 0.0, false); one.draws(&md).len() };
    let base = { let mut one = h::Horde::new(4); one.put(sp("brigand_archer"), [0.0, 11.0, 0.0], 0.0, false); one.draws(&md).len() };
    println!("pièces : cavalier {cav}, chef {chef}, archer {base}"); assert!(cav >= base + 6, "le cavalier a son cheval"); assert!(chef > base - 2);
}
