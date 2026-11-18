//! Le chef de guerre, un combat de boss : ses attaques s'annoncent, la taille se pare, la frappe écrasante ne se pare pas (il faut
//! rouler), les parades brisent sa garde et il chancelle, taper sa garde ne fait presque rien, et à mi-vie il enrage.
use alw_core::{hostile::{self as h, Ai}, player::*, session::*, stream::*};
const DT: f64 = 1.0 / 60.0;
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(4)); } }
/// Le monde plat, de nuit, un bouclier au bras, le chef de guerre à deux pas, face à nous.
fn duel() -> (Session, usize) {
    let seed = alw_core::gen::FLAT | 0x123; let y = alw_core::gen::FLAT_Y as f64 + 1.0;
    let mut s = Session::new(seed, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, y, 0.5]); ready(&mut s); s.third = false; s.hour = 23.5; s.wild.all.clear(); s.horde.foes.clear();
    for _ in 0..30 { s.tick(DT, Input::default(), Actions::default()); } s.horde.foes.clear(); s.vit = [100.0; 3]; s.stamina = 100.0; s.equip[5] = Some(Stack::new("iron_shield", None)); s.player.yaw = 0.0; s.player.pitch = 0.0;
    let p = s.player.p; let i = s.horde.put(h::species("brigand_chef").unwrap(), [p[0], p[1], p[2] + 2.0], std::f64::consts::PI, false);
    let f = &mut s.horde.foes[i]; f.alert = 1.0; f.seen = Some(p); f.state = Ai::Combat; (s, i)
}
/// Le chef annonce l'attaque `mv` ; elle tombe dans `left` secondes. On joue `secs` secondes avec la garde (ou la roulade au premier instant).
fn incoming(mv: u8, left: f64, guard: bool, roll: bool, secs: f64) -> Session {
    let (mut s, i) = duel(); { let f = &mut s.horde.foes[i]; f.mv = mv; f.wind = left; }
    for k in 0..(secs / DT) as usize { s.stamina = s.stamina.max(60.0); s.tick(DT, Input { roll: roll && k == 0, ..Input::default() }, Actions { guard, ..Default::default() }); if s.horde.foes[0].mv == 0 && s.horde.foes[0].wind == 0.0 && k > 30 { break; } }
    s
}

#[test]
fn la_taille_se_pare_au_bon_moment() {
    let s = incoming(h::MV_TAILLE, 0.25, true, false, 0.6); let f = &s.horde.foes[0];
    println!("taille parée : vie {:.0}, garde du chef {:.0}, notes {:?}", s.vit[0], f.posture, s.notes); assert!(s.notes.iter().any(|n| n.contains("Parade")) && s.vit[0] >= 99.0 && f.posture >= h::PARRY_POSTURE - 1.0);
    let s = incoming(h::MV_TAILLE, 0.25, false, false, 0.6); println!("taille sans garde : vie {:.0}", s.vit[0]); assert!(s.vit[0] < 95.0, "sans garde, elle porte");
}

#[test]
fn la_frappe_ecrasante_ne_se_pare_pas_elle_s_esquive() {
    let s = incoming(h::MV_ECRASE, 0.25, true, false, 0.6); println!("frappe écrasante sur le bouclier : vie {:.0}, {:?}", s.vit[0], s.notes);
    assert!(s.vit[0] < 85.0 && !s.notes.iter().any(|n| n.contains("Parade")), "le bouclier ne l'arrête pas");
    let s = incoming(h::MV_ECRASE, 0.15, false, true, 0.6); println!("frappe écrasante esquivée d'une roulade : vie {:.0}", s.vit[0]); assert!(s.vit[0] >= 99.0, "la roulade l'évite");
}

#[test]
fn les_parades_brisent_sa_garde() {
    let (mut s, i) = duel(); let me = s.player.p;
    // taper sa garde : la vie bouge à peine
    let hp0 = s.horde.foes[i].hp; s.horde.strike(i, 10.0, me, "sword", false); let guarded = hp0 - s.horde.foes[i].hp;
    // trois parades : la garde se brise, il chancelle
    for _ in 0..3 { s.horde.stagger(i, me, 2.2); } assert!(s.horde.foes[i].broken > 0.0, "garde brisée");
    s.tick(DT, Input::default(), Actions::default()); assert!(s.notes.iter().any(|n| n.contains("chancelle")), "{:?}", s.notes);
    let hp1 = s.horde.foes[i].hp; s.horde.strike(i, 10.0, me, "sword", false); let open = hp1 - s.horde.foes[i].hp;
    println!("un coup d'épée : {guarded:.1} sur sa garde, {open:.1} quand il chancelle"); assert!(open > guarded * 5.0);
    let bar = s.boss_bar().expect("la barre du boss"); println!("barre : {bar:?}"); assert!(bar.3);
    // il se reprend au bout de trois secondes
    for _ in 0..(3.5 / DT) as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); } assert!(s.horde.foes[i].broken <= 0.0);
}

#[test]
fn a_mi_vie_il_enrage_et_appelle_ses_hommes() {
    let (mut s, i) = duel(); s.horde.foes[i].hp = s.horde.foes[i].max * 0.45;
    for _ in 0..10 { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); }
    let men = s.horde.foes.iter().filter(|f| f.dead == 0.0 && f.sp == h::species("brigand").unwrap()).count();
    println!("enragé {}, renforts {men}, {:?}", s.horde.foes[i].enraged, s.notes.last()); assert!(s.horde.foes[i].enraged && men == 2);
}

#[test]
fn il_enchaine_ses_attaques() {
    // laissé faire une minute (sans qu'on meure), il emploie ses quatre attaques
    let (mut s, i) = duel(); let mut seen = std::collections::BTreeSet::new(); let p0 = s.player.p;
    for k in 0..(60.0 / DT) as usize { s.vit = [100.0; 3]; if k % 300 == 0 { s.player.p = [p0[0], p0[1], p0[2] + if (k / 300) % 2 == 0 { 0.0 } else { -8.0 }]; } s.tick(DT, Input::default(), Actions::default()); let f = &s.horde.foes[i]; if f.mv != 0 { seen.insert(f.mv); } }
    println!("attaques vues : {seen:?}"); assert_eq!(seen.len(), 4, "taille, écrasante, enchaînement, ruée");
}

/// Le cavalier d'élite à `d` pas devant soi, face à nous.
fn rider(d: f64) -> (Session, usize) {
    let (mut s, _) = duel(); s.horde.foes.clear(); let p = s.player.p;
    let i = s.horde.put(h::species("brigand_cavalier").unwrap(), [p[0], p[1], p[2] + d], std::f64::consts::PI, false); let f = &mut s.horde.foes[i]; f.alert = 1.0; f.seen = Some(p); f.state = Ai::Poursuite; (s, i)
}
fn play(s: &mut Session, secs: f64, guard: bool, roll_at: Option<usize>) { for k in 0..(secs / DT) as usize { s.stamina = s.stamina.max(60.0); s.tick(DT, Input { roll: roll_at == Some(k), x: if roll_at == Some(k) { 1.0 } else { 0.0 }, ..Input::default() }, Actions { guard, ..Default::default() }); } }

#[test]
fn la_charge_du_cavalier_s_esquive_de_cote() {
    // bouclier levé : la charge passe au travers
    let (mut s, i) = rider(10.0); { let f = &mut s.horde.foes[i]; f.mv = h::MV_LANCE; f.wind = 0.1; } play(&mut s, 1.5, true, None);
    println!("charge sur le bouclier : vie {:.0}, {:?}", s.vit[0], s.notes); assert!(s.vit[0] < 95.0 && !s.notes.iter().any(|n| n.contains("Parade")));
    // une roulade de côté quand il arrive : il passe, file trop loin, et doit tourner bride au pas
    let (mut s, i) = rider(10.0); { let f = &mut s.horde.foes[i]; f.mv = h::MV_LANCE; f.wind = 0.1; }
    let mut at = None; for k in 0..90 { s.tick(DT, Input::default(), Actions::default()); let f = &s.horde.foes[i]; if (f.p[2] - s.player.p[2]).abs() < 4.5 { at = Some(k); break; } }
    assert!(at.is_some(), "il arrive au galop"); play(&mut s, 1.6, false, Some(0)); let f = &s.horde.foes[i];
    println!("roulade de côté : vie {:.0}, le cavalier à {:.1} pas, au pas {}", s.vit[0], (f.p[0] - s.player.p[0]).hypot(f.p[2] - s.player.p[2]), f.speed < 3.0);
    assert!(s.vit[0] >= 99.0, "esquivée"); assert!(f.speed < 3.0, "manquée : il tourne bride, au pas — l'ouverture");
}

#[test]
fn le_coup_de_lance_au_passage_se_pare() {
    let (mut s, i) = rider(2.6); { let f = &mut s.horde.foes[i]; f.mv = h::MV_PASSE; f.wind = 0.2; } play(&mut s, 0.5, true, None);
    println!("coup au passage paré : vie {:.0}, garde du cavalier {:.0}", s.vit[0], s.horde.foes[i].posture); assert!(s.notes.iter().any(|n| n.contains("Parade")) && s.vit[0] >= 99.0);
}

#[test]
fn garde_brisee_le_cavalier_vide_les_etriers() {
    let (mut s, i) = rider(3.0); let me = s.player.p; let horses = s.wild.all.len();
    let hp0 = s.horde.foes[i].hp; s.horde.strike(i, 10.0, me, "sword", false); println!("à cheval, un coup d'épée : −{:.1}", hp0 - s.horde.foes[i].hp);
    for _ in 0..3 { s.horde.stagger(i, me, 2.2); } s.tick(DT, Input::default(), Actions::default());
    let f = &s.horde.foes[i]; println!("après trois parades : {} ({:?}), à terre {:.1} s, chevaux {} → {}, {:?}", h::bestiary().species[f.sp].name, f.state, f.broken, horses, s.wild.all.len(), s.notes.last());
    assert_eq!(f.sp, h::species("brigand").unwrap(), "il finit à pied"); assert!(f.broken > 0.0 && s.wild.all.len() == horses + 1 && s.notes.iter().any(|n| n.contains("étriers")));
    let hp1 = s.horde.foes[i].hp; s.horde.strike(i, 10.0, me, "sword", false); println!("à terre, un coup d'épée : −{:.1}", hp1 - s.horde.foes[i].hp); assert!(hp1 - s.horde.foes[i].hp > 15.0);
}

#[test]
fn le_cavalier_tourne_et_varie_ses_attaques() {
    let (mut s, i) = rider(12.0); let mut seen = std::collections::BTreeSet::new(); let p0 = s.player.p;
    for k in 0..(60.0 / DT) as usize { s.vit = [100.0; 3]; if k % 400 == 0 { s.player.p = p0; } s.tick(DT, Input::default(), Actions::default()); let f = &s.horde.foes[i]; if f.mv != 0 { seen.insert(f.mv); } }
    println!("attaques du cavalier : {seen:?}"); assert!(seen.contains(&h::MV_LANCE) && seen.contains(&h::MV_PASSE), "charge et coup au passage");
}

fn foe(id: &str, d: f64, vet: bool) -> (Session, usize) {
    let (mut s, _) = duel(); s.horde.foes.clear(); let p = s.player.p;
    let i = s.horde.put(h::species(id).unwrap(), [p[0], p[1], p[2] + d], std::f64::consts::PI, vet); let f = &mut s.horde.foes[i]; f.alert = 1.0; f.seen = Some(p); f.state = Ai::Combat; (s, i)
}

#[test]
fn le_capitaine_salve_au_bouclier_tir_charge_a_esquiver() {
    let (mut s, i) = foe("capitaine", 9.0, false); { let f = &mut s.horde.foes[i]; f.mv = h::MV_SALVE; f.wind = 0.1; } play(&mut s, 2.0, true, None); let salve = 100.0 - s.vit[0];
    let (mut s, i) = foe("capitaine", 9.0, false); { let f = &mut s.horde.foes[i]; f.mv = h::MV_TIR; f.wind = 0.1; } play(&mut s, 1.2, true, None); let tir = 100.0 - s.vit[0];
    println!("au bouclier : la salve −{salve:.0}, le tir chargé −{tir:.0} ({:?})", s.cause); assert!(tir > 10.0 && salve < tir, "le tir chargé traverse le bouclier"); assert!(s.cause.contains("capitaine"));
    // de près, le coup de pied, puis il rompt
    let (mut s, i) = foe("capitaine", 1.8, false); play(&mut s, 2.5, false, None); let d = (s.horde.foes[i].p[2] - s.player.p[2]).hypot(s.horde.foes[i].p[0] - s.player.p[0]);
    println!("serré de près : « {} », il s'est écarté à {d:.1} pas", s.cause); assert!(s.cause.contains("pied") && d > 3.0);
}

#[test]
fn le_gardien_de_la_crypte_se_bat_en_boss() {
    let (mut s, i) = foe("gardien", 2.0, false); let mut seen = std::collections::BTreeSet::new();
    for _ in 0..(25.0 / DT) as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); let f = &s.horde.foes[i]; if f.mv != 0 { seen.insert(f.mv); } }
    println!("gardien : attaques {seen:?}, « {} »", s.cause); assert!(seen.len() >= 3 && s.cause.contains("gardien"));
    let (mut s, i) = foe("gardien", 2.0, false); s.horde.foes[i].hp = s.horde.foes[i].max * 0.4; play(&mut s, 0.3, false, None);
    let dead = s.horde.foes.iter().filter(|f| f.sp == h::species("rodeur").unwrap()).count(); println!("à mi-vie : {dead} rôdeurs se lèvent, {:?}", s.notes.last()); assert_eq!(dead, 2);
}

#[test]
fn les_veterans_ont_leur_botte() {
    // l'archer vétéran bande parfois longuement son arc : sa flèche lourde traverse le bouclier
    let (mut s, i) = foe("brigand_archer", 9.0, true); let mut charged = false; for _ in 0..(20.0 / DT) as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); if s.horde.foes[i].mv == h::MV_TIR { charged = true; } }
    // l'épéiste vétéran feinte : le geste s'arrête net, puis le vrai coup part vite
    let (mut s, i) = foe("brigand_epeiste", 1.8, true); let mut feint = false; for _ in 0..(20.0 / DT) as usize { s.vit = [100.0; 3]; s.tick(DT, Input::default(), Actions::default()); if s.horde.foes[i].mv == h::MV_FEINTE { feint = true; } }
    println!("tir chargé {charged}, feinte {feint}"); assert!(charged && feint);
}
