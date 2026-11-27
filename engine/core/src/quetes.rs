//! Les quêtes des villageois. Dans chaque village, le chef et quelques habitants ont, chaque semaine, une tâche à confier :
//! - apporter des marchandises (ce dont leur peuple a l'usage : du pain et des planches au Royaume, du poisson grillé aux
//!   ports, des fourrures dans le Nord, du cuir aux Oasis…), à remettre en main propre ;
//! - abattre des brigands, des rôdeurs de la nuit ou des araignées qui menacent le village ;
//! - porter une lettre à un village voisin, qu'on vous indique sur la carte : la remettre à n'importe quel habitant.
//! On en mène trois à la fois au plus. Une tâche faite rapporte des pièces, de l'expérience, de la réputation (au village,
//! et un peu à tout son peuple), et aide la fortune du village. On peut y renoncer. Les tâches en cours se voient à
//! l'écran, avec leur avancement. Vérifié par `tests/quetes.rs`.
use crate::factions::Faction;
use crate::noise::hash_float;
use crate::session::{item, Session};
use crate::village::{Folk, Village};

/// Ce qu'on demande.
#[derive(Clone, Debug, PartialEq)]
pub enum Task { Livrer { item: &'static str, n: u32 }, Chasser { group: &'static str, n: u32, got: u32 }, Lettre { to: (i32, i32), to_name: String } }
/// Une tâche : qui la confie (village, maison, nom), ce qu'elle demande, ce qu'elle rapporte, la semaine où on l'a prise.
#[derive(Clone, Debug, PartialEq)]
pub struct Quest { pub v: (i32, i32), pub house: usize, pub giver: String, pub village: String, pub task: Task, pub coins: u32, pub week: i64 }
/// Combien de tâches à la fois.
pub const MAX: usize = 3;
/// Ce qu'une tâche rapporte de réputation au village (et, par lui, à son peuple), et de fortune.
pub const REP: i32 = 4; pub const FORTUNE: f64 = 3.0;

/// Les groupes de créatures qu'on chasse, leur nom, et les espèces qui en sont.
pub fn group_label(g: &str, n: u32) -> String { let s = if n > 1 { "s" } else { "" }; match g { "brigands" => format!("{n} brigand{s}"), "rodeurs" => format!("{n} rôdeur{s} de la nuit"), _ => format!("{n} araignée{s}") } }
pub fn group_of(species: &str) -> Option<&'static str> { if species.starts_with("brigand") { Some("brigands") } else if species.starts_with("rodeu") || species == "rampant" || species == "fouisseur" { Some("rodeurs") } else if species == "araignee" { Some("araignees") } else { None } }
/// Ce que chaque peuple fait demander : (objet, combien).
fn wares(f: Faction) -> &'static [(&'static str, u32)] {
    match f { Faction::Royaume => &[("bread", 5), ("planks", 16), ("iron_ingot", 3), ("wheat", 12), ("leather", 3)], Faction::Oasis => &[("leather", 4), ("bread", 4), ("iron_ingot", 2), ("cobblestone", 16)],
        Faction::Ports => &[("cooked_fish", 5), ("planks", 16), ("leather", 3), ("iron_ingot", 2)], Faction::Clans => &[("fur", 3), ("cooked_meat", 5), ("oak_log", 12), ("iron_ingot", 3)] }
}
impl Task {
    pub fn title(&self) -> String {
        match self { Task::Livrer { item: it, n } => format!("Apporter {n} {}", item(it).map_or(it.to_string(), |x| x.label.to_lowercase())), Task::Chasser { group, n, .. } => format!("Abattre {}", group_label(group, *n)), Task::Lettre { to_name, .. } => format!("Porter une lettre à {to_name}") }
    }
}

impl Session {
    /// Ceux qui ont une tâche à confier : le chef, et à peu près un habitant sur trois (pas les artisans).
    pub fn gives_quests(v: &Village, f: &Folk) -> bool { f.job == 9 || (f.craft.is_none() && v.houses.get(f.house).is_some_and(|h| h.craft.is_none()) && hash_float(v.id.0, v.id.1, f.house as i32, 771) < 0.35) }
    pub fn week(&self) -> i64 { (self.hours / 24.0 / 7.0).floor() as i64 }
    /// La tâche que ce villageois propose cette semaine (toujours la même pour la semaine).
    pub fn quest_offer(&self, v: &Village, f: &Folk) -> Option<Quest> {
        if !Self::gives_quests(v, f) { return None; } let w = self.week(); let h = |k: i32| hash_float(v.id.0 * 31 + f.house as i32, v.id.1, w as i32, 772 + k);
        let fac = Faction::of_kind(v.kind); let pick = h(0);
        let task = if pick < 0.45 { let list = wares(fac); let (it, n) = list[(h(1) * list.len() as f64) as usize % list.len()]; Task::Livrer { item: it, n } }
            else if pick < 0.75 { let g = ["brigands", "rodeurs", "araignees"][(h(2) * 3.0) as usize % 3]; Task::Chasser { group: g, n: if g == "brigands" { 3 } else { 4 }, got: 0 } }
            else { let near: Vec<Village> = self.gen.villages_near(v.c[0] as f64, v.c[2] as f64, 1800.0).into_iter().filter(|w| w.id != v.id).collect(); if near.is_empty() { return None; } let to = &near[(h(3) * near.len() as f64) as usize % near.len()]; Task::Lettre { to: to.id, to_name: to.name.clone() } };
        let coins = match &task { Task::Livrer { n, .. } => 6 + n / 2, Task::Chasser { n, group, .. } => if *group == "brigands" { 6 * n } else { 4 * n }, Task::Lettre { to, .. } => { let w = self.gen.site(to.0, to.1).map(|s| s.v.c).unwrap_or(v.c); 8 + (((w[0] - v.c[0]) as f64).hypot((w[2] - v.c[2]) as f64) / 100.0) as u32 } };
        Some(Quest { v: v.id, house: f.house, giver: f.name.clone(), village: v.name.clone(), task, coins: if f.job == 9 { coins + coins / 2 } else { coins }, week: w })
    }
    fn quest_done(&self, q: &Quest) -> bool { match &q.task { Task::Livrer { item, n } => self.count(item) >= *n, Task::Chasser { n, got, .. } => got >= n, Task::Lettre { .. } => false } }
    fn quest_progress(&self, q: &Quest) -> String { match &q.task { Task::Livrer { item, n } => format!("{}/{n}", self.count(item).min(*n)), Task::Chasser { n, got, .. } => format!("{}/{n}", got.min(n)), Task::Lettre { to_name, .. } => format!("à remettre à {to_name}") } }
    /// Les tâches en cours, pour l'écran : intitulé (qui la confie) et avancement.
    pub fn quest_lines(&self) -> Vec<(String, String, bool)> { self.quests.iter().map(|q| (format!("{} · {} ({})", q.task.title(), q.giver.split(' ').next().unwrap_or(""), q.village), self.quest_progress(q), self.quest_done(q))).collect() }
    /// Les villages où porter une lettre, pour la carte et la boussole : (position, nom).
    pub fn letter_targets(&self) -> Vec<([f64; 2], String)> { self.quests.iter().filter_map(|q| if let Task::Lettre { to, to_name } = &q.task { self.gen.site(to.0, to.1).map(|s| ([s.v.c[0] as f64 + 0.5, s.v.c[2] as f64 + 0.5], to_name.clone())) } else { None }).collect() }
    /// Ce qu'on peut dire à ce villageois au sujet des tâches.
    pub(crate) fn quest_options(&self, v: &Village, f: &Folk) -> Vec<(String, bool)> {
        let mut o = Vec::new();
        for q in &self.quests { if let Task::Lettre { to, .. } = &q.task { if *to == v.id { o.push((format!("Une lettre de {} pour vous", q.village), true)); } } }
        if let Some(q) = self.quests.iter().find(|q| q.v == v.id && q.house == f.house) {
            if self.quest_done(q) { o.push((format!("C'est fait : {}", q.task.title()), true)); } else { o.push((format!("Renoncer à : {}", q.task.title()), true)); }
        } else if let Some(q) = self.quest_offer(v, f).filter(|q| !self.quests_done.contains(&(q.v, q.house, q.week))) {
            o.push((format!("Une tâche ? ({} pièces)", q.coins), self.quests.len() < MAX));
        }
        o
    }
    /// Une réponse au sujet des tâches ; rend ce que dit le villageois, si c'en était une.
    pub(crate) fn quest_choose(&mut self, label: &str, v: &Village, f: &Folk) -> Option<String> {
        if label.starts_with("Une tâche") { let q = self.quest_offer(v, f)?; if let Some(t) = self.town.talk.as_mut() { t.page = crate::village::Page::Quest; }
            let what = match &q.task { Task::Livrer { .. } => format!("J'aurais besoin de quelqu'un pour m'apporter ceci : {}.", q.task.title().replacen("Apporter ", "", 1)), Task::Chasser { group, n, .. } => format!("{} rôdent autour du village. Pourriez-vous les abattre ?", cap(&group_label(group, *n))), Task::Lettre { to_name, .. } => format!("J'ai une lettre pour quelqu'un de {to_name}. Pourriez-vous la porter ?") };
            return Some(format!("{what} Je vous donnerai {} pièces.", q.coins)); }
        if label.starts_with("Accepter") { let q = self.quest_offer(v, f)?; let say = match &q.task {
                Task::Livrer { .. } => format!("Merci ! {} : rapportez-les-moi, je vous paierai {} pièces.", q.task.title(), q.coins),
                Task::Chasser { group, .. } => format!("Merci ! Ils rôdent autour du village : {}. Revenez me voir ensuite.", group_label(group, match q.task { Task::Chasser { n, .. } => n, _ => 0 })),
                Task::Lettre { to, to_name } => { if let Some(s) = self.gen.site(to.0, to.1) { let w = s.v.clone(); self.discover(&w); } format!("Merci ! Portez cette lettre à {to_name} : n'importe quel habitant la prendra. Le village est sur votre carte.") } };
            self.quests.push(q); self.dirty = true; return Some(say); }
        if label.starts_with("Renoncer") { self.quests.retain(|q| !(q.v == v.id && q.house == f.house)); self.dirty = true; return Some("Tant pis. Ce sera pour une autre fois.".into()); }
        if label.starts_with("C'est fait") {
            let i = self.quests.iter().position(|q| q.v == v.id && q.house == f.house)?; let q = self.quests[i].clone(); if !self.quest_done(&q) { return Some("Ce n'est pas encore fait.".into()); }
            if let Task::Livrer { item, n } = q.task { self.take(item, n); } self.quests.remove(i); self.reward(&q, v.id); return Some(format!("Merci du fond du cœur ! Voici {} pièces.", q.coins)); }
        if label.starts_with("Une lettre de") {
            let i = self.quests.iter().position(|q| matches!(&q.task, Task::Lettre { to, .. } if *to == v.id))?; let q = self.quests.remove(i); self.reward(&q, v.id); self.gain_rep(q.v, 2);
            return Some(format!("Une lettre de {} ! Merci d'avoir fait la route : voici {} pièces.", q.village, q.coins)); }
        None
    }
    fn reward(&mut self, q: &Quest, at: (i32, i32)) {
        let left = self.add_item(crate::economie::COIN, q.coins); if left > 0 { self.notes.push("Sac plein".into()); }
        self.earn(crate::niveaux::XP_PLACE, "tâche d'un villageois"); self.gain_rep(at, REP); self.quests_done.insert((q.v, q.house, q.week));
        if let Some(v) = self.town.village(q.v).cloned().or_else(|| self.town.atlas.get(&q.v).cloned()) { let x = (self.fortune(&v) + FORTUNE).min(100.0); self.town.fortune.insert(q.v, x); }
        self.notes.push(format!("Tâche accomplie : {} (+{} pièces)", q.task.title(), q.coins)); self.sound.pickup(); self.dirty = true;
    }
    /// Une créature abattue : elle compte pour les chasses en cours.
    pub(crate) fn quest_kill(&mut self, species: &str) {
        let Some(g) = group_of(species) else { return }; let mut said = Vec::new();
        for q in self.quests.iter_mut() { if let Task::Chasser { group, n, got } = &mut q.task { if *group == g && *got < *n { *got += 1; if *got == *n { said.push(format!("Chasse terminée : retournez voir {} à {}", q.giver.split(' ').next().unwrap_or(""), q.village)); } } } }
        self.notes.extend(said); self.dirty = true;
    }
    pub(crate) fn quests_save(&self) -> String {
        let mut s = String::new();
        for q in &self.quests { let t = match &q.task { Task::Livrer { item, n } => format!("livrer {item} {n}"), Task::Chasser { group, n, got } => format!("chasser {group} {n} {got}"), Task::Lettre { to, .. } => format!("lettre {} {}", to.0, to.1) };
            s += &format!("quete {} {} {} {} {} {}\n", q.v.0, q.v.1, q.house, q.week, q.coins, t); }
        for (v, h, w) in &self.quests_done { s += &format!("quete_faite {} {} {h} {w}\n", v.0, v.1); } s
    }
    pub(crate) fn quests_load(&mut self, key: &str, a: &[&str]) {
        let n = |i: usize| a.get(i).and_then(|x| x.parse::<i64>().ok()); let (Some(x), Some(z), Some(h), Some(w)) = (n(0), n(1), n(2), n(3)) else { return }; let id = (x as i32, z as i32);
        if key == "quete_faite" { self.quests_done.insert((id, h as usize, w)); return; }
        let Some(coins) = n(4) else { return }; let Some(site) = self.gen.site(id.0, id.1) else { return }; let v = site.v.clone();
        let task = match a.get(5).copied() {
            Some("livrer") => { let Some(it) = a.get(6).and_then(|s| crate::session::item(s)).map(|i| i.id) else { return }; Task::Livrer { item: it, n: n(7).unwrap_or(1) as u32 } }
            Some("chasser") => { let Some(g) = a.get(6).and_then(|s| ["brigands", "rodeurs", "araignees"].into_iter().find(|x| x == s)) else { return }; Task::Chasser { group: g, n: n(7).unwrap_or(1) as u32, got: n(8).unwrap_or(0) as u32 } }
            Some("lettre") => { let (Some(tx), Some(tz)) = (n(6), n(7)) else { return }; let to = (tx as i32, tz as i32); let to_name = self.gen.site(to.0, to.1).map_or(String::new(), |s| s.v.name.clone()); Task::Lettre { to, to_name } }
            _ => return,
        };
        let giver = self.town.folk.iter().find(|f| f.v == id && f.house == h as usize).map_or_else(|| crate::village::folk_name(&v, h as usize), |f| f.name.clone());
        self.quests.push(Quest { v: id, house: h as usize, giver, village: v.name.clone(), task, coins: coins as u32, week: w });
    }
}

fn cap(s: &str) -> String { let mut c = s.chars(); c.next().map_or(String::new(), |f| f.to_uppercase().collect::<String>() + c.as_str()) }
