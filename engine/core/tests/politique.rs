//! Les peuples entre eux : relations qui bougent chaque jour, guerres (sous -50, jusqu'à la trêve au-dessus de -30),
//! alliances (au-dessus de 40), routes commerciales qui rapprochent et que la guerre coupe ; et ce que cela change pour le
//! joueur : alliés et ennemis ressentent ce qu'on fait pour un peuple, la guerre renchérit les prix.
use alw_core::factions::{Faction, ALL};
use alw_core::politique::{Bond, Politics};
use alw_core::{gen::WorldGen, player::Input, session::*, stream::Preset};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, n: usize) { for _ in 0..n { s.tick(0.05, Input::default(), Actions::default()); } }
fn set(p: &mut Politics, a: Faction, b: Faction, r: f64, war: bool) { let (i, j) = (a as usize, b as usize); p.rel[i][j] = r; p.rel[j][i] = r; p.war[i][j] = war; p.war[j][i] = war; }

#[test]
fn les_relations_des_peuples() {
    let p = Politics::new(SEED); for i in 0..4 { for j in 0..4 { assert_eq!(p.rel[i][j], p.rel[j][i]); } }
    for i in 0..4 { for j in i + 1..4 { println!("{} / {} : {:.0} ({})", ALL[i].name(), ALL[j].name(), p.rel[i][j], p.bond(ALL[i], ALL[j]).label()); } }
    assert!(p.rel[Faction::Royaume as usize][Faction::Ports as usize] > p.rel[Faction::Royaume as usize][Faction::Clans as usize] + 40.0, "le Royaume s'entend mieux avec les ports qu'avec les Clans");
    assert_ne!(Politics::new(SEED).rel, Politics::new(SEED + 1).rel, "chaque monde a ses liens");
    // La guerre éclate sous -50, et dure jusqu'à ce que la relation remonte au-dessus de -30.
    let mut q = Politics::new(SEED); set(&mut q, Faction::Royaume, Faction::Clans, -62.0, false); let said = q.day_passes([[0; 4]; 4]);
    assert_eq!(q.bond(Faction::Royaume, Faction::Clans), Bond::Guerre, "{said:?}"); assert!(said.iter().any(|l| l.starts_with("La guerre")), "{said:?}"); assert!(q.at_war(Faction::Clans) && !q.at_war(Faction::Oasis) || q.at_war(Faction::Oasis));
    set(&mut q, Faction::Royaume, Faction::Clans, -40.0, true); q.day_passes([[0; 4]; 4]); assert_eq!(q.bond(Faction::Royaume, Faction::Clans), Bond::Guerre, "pas encore de trêve à -40");
    set(&mut q, Faction::Royaume, Faction::Clans, -25.0, true); let said = q.day_passes([[0; 4]; 4]); assert_ne!(q.bond(Faction::Royaume, Faction::Clans), Bond::Guerre, "trêve : {said:?}"); assert!(said.iter().any(|l| l.starts_with("Trêve")));
    // Les routes ouvertes rapprochent les peuples : mêmes hasards, une route de plus, la relation monte davantage.
    let (mut a, mut b) = (Politics::new(SEED), Politics::new(SEED)); let mut t = [[0u32; 4]; 4]; t[1][3] = 2; t[3][1] = 2;
    for _ in 0..30 { a.day_passes([[0; 4]; 4]); b.day_passes(t); } println!("Oasis / Clans : sans route {:.1}, avec deux routes {:.1}", a.rel[1][3], b.rel[1][3]); assert!(b.rel[1][3] > a.rel[1][3] + 5.0);
    // Une longue suite de jours : il se passe des choses, et les relations restent dans leurs bornes.
    let mut l = Politics::new(SEED); let mut news = 0; for _ in 0..400 { news += l.day_passes([[0; 4]; 4]).len(); } println!("400 jours : {news} nouvelles ; liens {:?}", (0..4).flat_map(|i| (i + 1..4).map(move |j| (i, j))).map(|(i, j)| l.bond(ALL[i], ALL[j]).label()).collect::<Vec<_>>());
    assert!(news > 60); assert!(l.rel.iter().flatten().all(|r| (-100.0..=100.0).contains(r)));
}

#[test]
fn ce_que_cela_change_pour_le_joueur() {
    let v = WorldGen::new(SEED).villages_near(0.0, 0.0, 2000.0).into_iter().next().unwrap();
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [v.c[0] as f64 + 0.5, v.c[1] as f64 + 1.0, v.c[2] as f64 + 3.5]); ready(&mut s); s.hour = 12.0; idle(&mut s, 40); s.set_creative(false);
    let f = Faction::of_kind(v.kind); assert_eq!(f, Faction::Royaume);
    // Les alliés s'en réjouissent, les ennemis en guerre s'en fâchent.
    set(&mut s.politics, Faction::Royaume, Faction::Ports, 60.0, false); set(&mut s.politics, Faction::Royaume, Faction::Clans, -70.0, true); set(&mut s.politics, Faction::Royaume, Faction::Oasis, 0.0, false);
    s.sway(Faction::Royaume, 20.0); println!("après 20 pour le Royaume : {:?}", ALL.map(|g| s.faction_rep(g)));
    assert_eq!(s.faction_rep(Faction::Royaume), 20); assert_eq!(s.faction_rep(Faction::Ports), 5, "l'allié"); assert_eq!(s.faction_rep(Faction::Clans), -7, "l'ennemi"); assert_eq!(s.faction_rep(Faction::Oasis), 0, "celui qui n'y est pour rien");
    // La guerre renchérit tout dans les villages du peuple qui la mène.
    let war: Vec<u32> = s.offers(&v).iter().map(|o| o.give.1).collect(); set(&mut s.politics, Faction::Royaume, Faction::Clans, 0.0, false); let peace: Vec<u32> = s.offers(&v).iter().map(|o| o.give.1).collect();
    println!("prix en paix {peace:?}, en guerre {war:?}"); assert!(war.iter().zip(&peace).all(|(a, b)| a >= b) && war.iter().zip(&peace).any(|(a, b)| a > b));
    // Les routes commerciales entre villages connus ; la guerre coupe celles qui vont d'un peuple à l'autre.
    for w in WorldGen::new(SEED).villages_near(v.c[0] as f64, v.c[2] as f64, 2500.0) { s.discover(&w); }
    let routes = s.trade_routes(); println!("{} routes, dont {} entre deux peuples", routes.len(), routes.iter().filter(|r| r.fa != r.fb).count()); assert!(routes.len() >= 4 && routes.iter().all(|r| r.open));
    let r = routes.iter().find(|r| r.fa != r.fb).expect("une route entre deux peuples").clone(); set(&mut s.politics, r.fa, r.fb, -80.0, true);
    let cut = s.trade_routes(); assert!(cut.iter().any(|x| x.a == r.a && x.b == r.b && !x.open), "la guerre coupe la route"); assert!(cut.iter().filter(|x| x.fa == x.fb).all(|x| x.open), "pas les autres");
    set(&mut s.politics, r.fa, r.fb, 0.0, false);
    // Les jours passent : les nouvelles arrivent, et tout se garde avec la partie.
    let d0 = s.politics.day; s.pass_hours(24.0 * 12.0); idle(&mut s, 2); println!("jour {} → {} ; nouvelles {:?}", d0, s.politics.day, s.politics.news); assert!(s.politics.day >= d0 + 12 && !s.politics.news.is_empty());
    let t = s.save_text(); let mut l = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 60.0, 0.5]); l.load_text(&t).unwrap();
    for i in 0..4 { for j in 0..4 { assert!((l.politics.rel[i][j] - s.politics.rel[i][j]).abs() < 0.06); assert_eq!(l.politics.war[i][j], s.politics.war[i][j]); } } assert_eq!(l.politics.day, s.politics.day);
}
