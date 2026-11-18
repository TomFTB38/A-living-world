//! Les camps de brigands grandissent si on les laisse : plus d'hommes au camp, des bandes plus nombreuses et plus souvent, des coups
//! plus forts sur les murailles. Détruit (tous ses gardiens à terre), le camp est dispersé : plus de bandes pendant dix jours.
use alw_core::{camps::{self, Camp}, economie::*, gen::WorldGen, player::{Input, Voxels}, raids, session::*, sites::Lieu, stream::Preset};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, n: usize) { for _ in 0..n { s.tick(0.05, Input::default(), Actions::default()); } }

#[test]
fn un_camp_grandit_s_il_est_laisse() {
    let mut c = Camp::new(0.0); assert_eq!(c.level(), 1);
    c.grow(24.0 * 5.0); assert_eq!(c.level(), 2); c.grow(24.0 * 30.0); assert_eq!(c.level(), 5, "cinq crans au plus");
    // ses gardiens et ses bandes suivent sa puissance
    let pl = WorldGen::new(SEED).places_near(0.0, 0.0, 2600.0).into_iter().find(|p| p.kind == Lieu::Camp).unwrap();
    let g: Vec<(usize, usize)> = (1..=5).map(|l| { let v = camps::garrison(&pl, l); (v.len(), v.iter().filter(|x| x.2).count()) }).collect(); println!("gardiens (hommes, vétérans) par cran : {g:?}");
    assert!(g[0].0 < g[4].0 && g[0].1 < g[4].1, "un camp fort est mieux gardé");
    let b: Vec<u32> = (1..=5).map(|l| raids::band_size(l as f64)).collect(); println!("bandes : {b:?}"); assert!(b[0] == 3 && b[4] == 9);
}

#[test]
fn le_camp_menace_le_village_et_ses_bandes_grossissent() {
    let mut s = Session::new(SEED, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.third = false; idle(&mut s, 5); s.set_creative(true);
    let id = s.econ.as_ref().unwrap().id; let v = WorldGen::new(SEED).site(id.0, id.1).unwrap().v.clone();
    let (pl, p) = s.threat(&v).expect("un camp à moins de mille pas"); println!("{} ; {}", s.threat_line(&v).unwrap(), p);
    // trente jours sans qu'on y aille : il est au plus fort, ses bandes sortent plus souvent et plus nombreuses
    let e = s.econ.clone().unwrap(); let low = raids::raid_chance(&e, false, 0, 0, p); s.pass_hours(24.0 * 30.0); let (_, p) = s.threat(&v).unwrap();
    let high = raids::raid_chance(&e, false, 0, 0, p); println!("puissance {p} ; chance d'une bande par nuit {low:.2} → {high:.2}"); assert!(p >= 5.0 && high > low * 1.5);
    s.player.p = [v.c[0] as f64 + 0.5, v.c[1] as f64 + 30.0, v.c[2] as f64 + 0.5]; s.player.fly = true; ready(&mut s); idle(&mut s, 40); s.hour = 23.0;
    assert!(s.raid_now()); let band: Vec<_> = s.horde.foes.iter().filter(|f| !f.raid.is_empty()).collect(); let vets = band.iter().filter(|f| f.vet).count();
    println!("bande : {} hommes dont {vets} vétérans", band.len()); assert!(band.len() >= 8 && vets >= 3, "une bande d'un camp fort");
    let _ = pl;
}

#[test]
fn detruire_le_camp_le_disperse() {
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.hour = 12.0; s.pass_hours(24.0 * 12.0); s.hour = 12.0;
    let msg = s.command("/lieu brigands"); ready(&mut s); println!("{msg}"); for _ in 0..3 { s.tick(0.05, Input::default(), Actions::default()); }
    let men: Vec<usize> = (0..s.horde.foes.len()).filter(|&i| s.horde.foes[i].camp.is_some()).collect(); let cell = s.horde.foes[men[0]].camp.unwrap();
    println!("{} hommes au camp, puissance {}", men.len(), s.camps[&cell].level());
    // un camp au cran 3 se voit : la troisième tente, la forge, les torches de la palissade
    let pl = WorldGen::new(SEED).places_near(s.player.p[0], s.player.p[2], 60.0).into_iter().find(|p| p.cell == cell).unwrap();
    let count = |s: &Session, n: &str| { let mut k = 0; for x in pl.x - 12..=pl.x + 12 { for z in pl.z - 12..=pl.z + 12 { for y in pl.y..pl.y + 12 { k += s.world.get(x, y, z).is_some_and(|b| alw_core::blocks::BLOCKS[b as usize].name.starts_with(n)) as i32; } } } k };
    println!("au cran 3 : enclume {}, torches {}, râteliers {}", count(&s, "anvil"), count(&s, "torch"), count(&s, "weapon_rack")); assert!(count(&s, "anvil") == 1 && count(&s, "torch") >= 8 && count(&s, "weapon_rack") >= 1, "le camp montre sa puissance"); assert!(men.len() >= 7 && s.notes.iter().any(|n| n.contains("puissance")), "{:?}", s.notes);
    // on les met tous à terre
    for _ in 0..30 { for &i in &men { if s.horde.foes.get(i).is_some_and(|f| f.dead == 0.0) { let p = s.horde.foes[i].p; s.horde.strike(i, 999.0, [p[0] + 1.0, p[1], p[2]], "sword", true); } } s.tick(0.05, Input::default(), Actions::default()); }
    let c = &s.camps[&cell]; println!("camp : dispersé {}, niveau {} ; notes {:?}", c.down(), c.level(), s.notes.last()); assert!(c.down() && c.level() == 0, "le camp est dispersé");
    for _ in 0..3 { s.tick(0.05, Input::default(), Actions::default()); } println!("dispersé : enclume {}, feux éteints {}", count(&s, "anvil"), count(&s, "campfire_out")); assert!(count(&s, "anvil") == 0 && count(&s, "campfire_out") == 1, "le camp abandonné, son feu éteint");
    // dix jours de calme : aucune bande ne peut sortir ; puis il se reforme, petit
    let e = s.econ.clone().unwrap(); assert_eq!(raids::raid_chance(&e, false, 0, 0, 0.0), 0.0);
    s.pass_hours(24.0 * 11.0); let mut c = s.camps[&cell].clone(); c.grow(s.hours); println!("onze jours après : niveau {}", c.level()); assert_eq!(c.level(), 1);
    // la partie s'en souvient
    let t = s.save_text(); assert!(t.lines().any(|l| l.starts_with("camp ")));
}
