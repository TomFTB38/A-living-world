//! Les quêtes des villageois : chaque semaine, le chef et quelques habitants ont une tâche à confier (apporter des
//! marchandises, abattre des créatures, porter une lettre à un village voisin) ; on en mène trois au plus ; une tâche faite
//! rapporte pièces, expérience, réputation et fortune au village ; on peut y renoncer ; tout se garde avec la partie.
use alw_core::hostile::bestiary;
use alw_core::quetes::{Task, MAX};
use alw_core::{gen::WorldGen, player::Input, session::*, stream::Preset, village::Village};
const SEED: i32 = 20261104;
fn ready(s: &mut Session) { for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, n: usize) { for _ in 0..n { s.tick(0.05, Input::default(), Actions::default()); } }
fn go(s: &mut Session, v: &Village) { s.player.p = [v.c[0] as f64 + 0.5, v.c[1] as f64 + 1.0, v.c[2] as f64 + 3.5]; ready(s); idle(s, 40); s.wild.all.clear(); s.horde.foes.clear(); }
fn at_village() -> (Session, Village) {
    let v = WorldGen::new(SEED).villages_near(0.0, 0.0, 2000.0).into_iter().next().unwrap();
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [v.c[0] as f64 + 0.5, v.c[1] as f64 + 1.0, v.c[2] as f64 + 3.5]); ready(&mut s); s.hour = 12.0; idle(&mut s, 40); s.set_creative(false); s.wild.all.clear(); s.horde.foes.clear(); (s, v)
}
/// Parle au villageois `k` et choisit l'option qui commence par `start` ; rend ce qu'il répond.
fn answer(s: &mut Session, k: usize, start: &str) -> Option<String> { s.talk_to(k); let d = s.dialog()?; let i = d.options.iter().position(|o| o.0.starts_with(start))?; s.choose(i);
    if start == "Une tâche" { let d = s.dialog()?; println!("  proposé : « {} » {:?}", d.said, d.options); let i = d.options.iter().position(|o| o.0 == "Accepter" && o.1)?; s.choose(i); } let said = s.dialog().map(|d| d.said); s.town.talk = None; said }

#[test]
fn les_taches_des_villageois() {
    let (mut s, v) = at_village();
    let givers: Vec<usize> = (0..s.town.folk.len()).filter(|&k| s.town.folk[k].v == v.id && Session::gives_quests(&v, &s.town.folk[k])).collect(); assert!(!givers.is_empty(), "quelqu'un a une tâche à confier");
    let others = (0..s.town.folk.len()).filter(|&k| s.town.folk[k].v == v.id && !Session::gives_quests(&v, &s.town.folk[k])).count(); println!("{} : {} donneurs de tâches, {} autres", v.name, givers.len(), others);
    // Les tâches changent d'une semaine à l'autre ; on trouve les trois sortes.
    let mut seen = [None, None, None]; let h0 = s.hours;
    for w in 0..30 { s.hours = h0 + w as f64 * 168.0; for &k in &givers { if let Some(q) = s.quest_offer(&v, &s.town.folk[k]) { let i = match q.task { Task::Livrer { .. } => 0, Task::Chasser { .. } => 1, Task::Lettre { .. } => 2 }; if seen[i].is_none() { println!("semaine {w}, {} : {} ({} pièces)", q.giver, q.task.title(), q.coins); seen[i] = Some((w, k)); } } } }
    assert!(seen.iter().all(|x| x.is_some()), "trois sortes de tâches");
    // Apporter des marchandises.
    let (w, k) = seen[0].unwrap(); s.hours = h0 + w as f64 * 168.0; let said = answer(&mut s, k, "Une tâche").expect("on accepte"); println!("« {said} »"); assert_eq!(s.quests.len(), 1);
    let Task::Livrer { item, n } = s.quests[0].task.clone() else { panic!() }; assert!(answer(&mut s, k, "C'est fait").is_none(), "pas avant d'avoir les marchandises");
    s.add_item(item, n + 2); let coins = s.count(alw_core::economie::COIN); let rep = s.rep(v.id); let xp = s.prog.xp; let fortune = s.fortune(&v);
    let said = answer(&mut s, k, "C'est fait").expect("on rend la tâche"); println!("« {said} »");
    assert!(s.quests.is_empty()); assert_eq!(s.count(item), 2, "les marchandises remises"); assert!(s.count(alw_core::economie::COIN) > coins && s.rep(v.id) >= rep + 4 && s.prog.xp > xp && s.fortune(&v) > fortune, "pièces, réputation, expérience, fortune");
    s.talk_to(k); assert!(!s.dialog().unwrap().options.iter().any(|o| o.0.starts_with("Une tâche")), "une seule tâche par semaine et par personne"); s.town.talk = None;
    // Abattre des créatures : chaque mort compte.
    let (w, k) = seen[1].unwrap(); s.hours = h0 + w as f64 * 168.0; answer(&mut s, k, "Une tâche").unwrap(); let Task::Chasser { group, n, .. } = s.quests[0].task.clone() else { panic!() };
    let sp = bestiary().species.iter().position(|x| alw_core::quetes::group_of(x.id) == Some(group)).unwrap(); let p = s.player.p;
    for _ in 0..n { let i = s.horde.put(sp, [p[0] + 3.0, p[1], p[2]], 0.0, false); s.horde.strike(i, 9999.0, p, "sword", true); idle(&mut s, 3); }
    println!("chasse : {:?} ; {:?}", s.quest_lines(), s.notes.last()); assert!(matches!(s.quests[0].task, Task::Chasser { got, n, .. } if got == n)); assert!(answer(&mut s, k, "C'est fait").is_some()); assert!(s.quests.is_empty());
    // Porter une lettre : le village voisin s'inscrit sur la carte ; n'importe quel habitant la prend.
    let (w, k) = seen[2].unwrap(); s.hours = h0 + w as f64 * 168.0; answer(&mut s, k, "Une tâche").unwrap(); let Task::Lettre { to, to_name } = s.quests[0].task.clone() else { panic!() };
    assert!(s.town.known.contains(&to), "le village de destination sur la carte"); assert_eq!(s.letter_targets().len(), 1); let rep0 = s.rep(v.id);
    // On la garde en partant : tout se sauve.
    let t = s.save_text(); let mut l = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 60.0, 0.5]); l.load_text(&t).unwrap(); assert_eq!(l.quests.len(), 1); assert_eq!(l.quests[0].task, s.quests[0].task); assert_eq!(l.quests_done, s.quests_done);
    let dest = WorldGen::new(SEED).site(to.0, to.1).unwrap().v.clone(); go(&mut s, &dest); let k2 = s.town.folk.iter().position(|f| f.v == to).expect("un habitant");
    let said = answer(&mut s, k2, "Une lettre de").expect("la lettre remise"); println!("à {to_name} : « {said} »"); assert!(s.quests.is_empty() && s.rep(v.id) >= rep0 + 2 && s.rep(to) >= 4, "les deux villages en savent gré");
}

#[test]
fn trois_a_la_fois_et_on_peut_renoncer() {
    let (mut s, _) = at_village(); let vs: Vec<Village> = WorldGen::new(SEED).villages_near(0.0, 0.0, 2000.0).into_iter().take(4).collect();
    let mut blocked = false;
    for v in &vs { go(&mut s, v); for k in 0..s.town.folk.len() { if s.town.folk[k].v != v.id || !Session::gives_quests(v, &s.town.folk[k]) || s.quest_offer(v, &s.town.folk[k]).is_none() { continue; }
        if s.quests.len() == MAX { s.talk_to(k); let d = s.dialog().unwrap(); blocked |= d.options.iter().any(|o| o.0.starts_with("Une tâche") && !o.1); s.town.talk = None; continue; }
        let _ = answer(&mut s, k, "Une tâche"); } if blocked { break; } }
    println!("{} tâches : {:?}", s.quests.len(), s.quest_lines()); assert_eq!(s.quests.len(), MAX); assert!(blocked, "pas une quatrième");
    let q = s.quests.last().unwrap().clone(); let k = s.town.folk.iter().position(|f| f.v == q.v && f.house == q.house).unwrap(); answer(&mut s, k, "Renoncer").expect("on renonce"); assert_eq!(s.quests.len(), MAX - 1);
}
