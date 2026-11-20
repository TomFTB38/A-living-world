//! Les créatures hostiles de la nuit : registre des espèces (lu dans `data/creatures.txt`), apparition dans le noir,
//! comportement (repos, patrouille, soupçon, détection, poursuite, combat, recherche, retour, fuite, embuscade),
//! combat aux attaques annoncées, mort et butin. Un seul système pour toutes les espèces : ce qui les distingue
//! (corps, chiffres, talents, butin) vient des données.
use crate::actors::{b as flat, Draw, Models};
use crate::blocks::*;
use crate::det::{cos, hypot2, sin};
use crate::fauna::{fits, ground};
use crate::mesher::VBuf;
use crate::noise::Rng;
use crate::player::Voxels;
use crate::view::{mul, roll_z, trs, M4};
use std::sync::OnceLock;

pub const DATA: &str = include_str!("../data/creatures.txt");

// ------------------------------------------------------------------------------------------------------------- données
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Body { Rodeur, Brigand, Bete, Rampant, /** araignée des cavernes : huit pattes */ Araignee, /** golem de cristal : un colosse de roche */ Golem, /** charpente d'homme sculptée, selon la peau : noyé, archer, fantôme, sylvain */ Humain, /** ver des sables */ Ver, /** harpie */ Harpie, /** un brigand monté sur son cheval */ Cavalier }
#[derive(Clone, Debug)]
pub struct Species {
    pub id: &'static str, pub name: &'static str, pub tier: u8, pub body: Body, pub size: f64, pub hp: f64, pub dmg: f64, pub walk: f64, pub run: f64,
    pub reach: f64, pub sight: f64, pub hear: f64, pub cd: f64, pub windup: f64, pub armor: f64, pub weak: &'static str,
    /// Biomes où elle vit (indices de `biome::NAMES`) ; vide : partout.
    pub biomes: Vec<usize>, pub weight: f64, pub group: (u32, u32), pub flee: f64,
    pub leap: bool, pub climb: bool, pub ambush: bool, pub bursts: bool, pub flank: bool,
    /// Créature des grottes : les biomes souterrains où elle vit (« toutes » : partout sous terre) ; vide : une créature de surface, qui descend aussi sous terre.
    pub caves: Vec<&'static str>,
    /// Sa peau (« magma » : le rampant de basalte incandescent) ; `ember` : elle brûle qui l'approche, sème des braises et meurt dans l'eau ; `web` : sa morsure englue.
    pub skin: &'static str, pub ember: bool, pub web: bool,
    /// Elle vole ; spectre : elle traverse les murs et se dissipe au jour ; elle tire à l'arc ; elle chemine sous le sable ; son coup attire au lieu de repousser.
    pub fly: bool, pub ghost: bool, pub bow: bool, pub burrow: bool, pub drag: bool,
    /// Elle ne naît que la nuit ; qu'au bord de l'eau ; jamais sous terre.
    pub night: bool, pub water: bool, pub surface: bool,
    /// Brigands des camps : la force de ses coups contre une herse ou une muraille (1 : un brigand ordinaire) ; il lève son bouclier
    /// en approchant et riposte après un coup paré ; il charge au galop ; il mène les siens (ils ne fuient pas, son cri ameute) ; blessé, il enrage.
    pub siege: f64, pub guard_up: bool, pub charge: bool, pub leader: bool, pub rage: bool,
    /// Un boss : des attaques à lire (taille à parer, frappe écrasante et ruée à esquiver, enchaînement), une garde à briser, deux phases.
    pub boss: bool,
    pub loot: Vec<(&'static str, u32, u32, f64)>,
}
/// Règles d'apparition et de difficulté (voir `data/creatures.txt`).
#[derive(Clone, Debug)]
pub struct Rules {
    pub light: u8, pub ring: (f64, f64), pub forget: f64, pub cap: usize, pub cap_near: usize, pub every: f64,
    pub distance: Vec<(f64, i32)>, pub biome: [i32; crate::biome::COUNT], pub mountain: (f64, i32), pub adapt: Vec<(u32, i32)>, pub day_secs: f64, pub region: f64,
}
pub struct Bestiary { pub species: Vec<Species>, pub rules: Rules }
pub fn bestiary() -> &'static Bestiary { static B: OnceLock<Bestiary> = OnceLock::new(); B.get_or_init(|| parse(DATA)) }
pub fn species(id: &str) -> Option<usize> { bestiary().species.iter().position(|s| s.id == id) }

fn parse(t: &'static str) -> Bestiary {
    let mut r = Rules { light: 7, ring: (24.0, 46.0), forget: 76.0, cap: 10, cap_near: 6, every: 0.7, distance: vec![(250.0, 1), (750.0, 2), (1500.0, 3), (3000.0, 4)], biome: [2, 1, 2, 2, 3, 3, 4, 5, 2, 3], mountain: (118.0, 5), adapt: vec![(5, 1), (10, 2), (20, 3), (30, 4)], day_secs: 240.0, region: 256.0 };
    let mut sp = Vec::new();
    let pairs = |v: &str| -> Vec<(f64, f64)> { v.split_whitespace().filter_map(|p| { let (a, b) = p.split_once(':')?; Some((a.parse().ok()?, b.parse().ok()?)) }).collect() };
    for line in t.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        if let Some(rest) = line.strip_prefix("regle ") {
            let (k, v) = rest.split_once(' ').unwrap_or((rest, "")); let n: Vec<f64> = v.split_whitespace().filter_map(|x| x.parse().ok()).collect();
            match k {
                "lumiere" => r.light = n.first().copied().unwrap_or(7.0) as u8, "cercle" if n.len() >= 2 => r.ring = (n[0], n[1]), "oubli" => r.forget = n.first().copied().unwrap_or(76.0),
                "plafond" if n.len() >= 2 => { r.cap = n[0] as usize; r.cap_near = n[1] as usize; } "rythme" => r.every = n.first().copied().unwrap_or(0.7).max(0.1),
                "distance" => r.distance = pairs(v).into_iter().map(|(a, b)| (a, b as i32)).collect(), "adaptation" => r.adapt = pairs(v).into_iter().map(|(a, b)| (a as u32, b as i32)).collect(),
                "journee" => r.day_secs = n.first().copied().unwrap_or(240.0), "region" => r.region = n.first().copied().unwrap_or(256.0).max(32.0),
                "montagne" => { if let Some((a, b)) = v.trim().split_once('=') { r.mountain = (a.trim().parse().unwrap_or(118.0), b.trim().parse().unwrap_or(5)); } }
                "biome" => { for (i, name) in crate::biome::NAMES.iter().enumerate() { if let Some(p) = v.find(&format!("{name}=")) { let rest = &v[p + name.len() + 1..]; if let Some(d) = rest.split_whitespace().next().and_then(|x| x.parse().ok()) { r.biome[i] = d; } } } }
                _ => {}
            }
            continue;
        }
        let Some(rest) = line.strip_prefix("espece ") else { continue };
        let f: Vec<&'static str> = rest.split('|').map(str::trim).collect(); if f.len() < 3 { continue; }
        let mut s = Species { id: f[0], name: f[1], tier: 1, body: Body::Rodeur, size: 1.0, hp: 25.0, dmg: 6.0, walk: 1.0, run: 4.5, reach: 1.5, sight: 16.0, hear: 10.0, cd: 1.6, windup: 0.5, armor: 0.0, weak: "", biomes: Vec::new(), weight: 1.0, group: (1, 1), flee: 0.0, leap: false, climb: false, ambush: false, bursts: false, flank: false, caves: Vec::new(), skin: "", ember: false, web: false, fly: false, ghost: false, bow: false, burrow: false, drag: false, night: false, water: false, surface: false, siege: 1.0, guard_up: false, charge: false, leader: false, rage: false, boss: false, loot: Vec::new() };
        for kv in f[2].split_whitespace() { let Some((k, v)) = kv.split_once('=') else { continue }; let x = v.parse::<f64>().unwrap_or(0.0);
            match k {
                "niveau" => s.tier = x as u8, "corps" => s.body = match v { "brigand" => Body::Brigand, "bete" => Body::Bete, "rampant" => Body::Rampant, "araignee" => Body::Araignee, "golem" => Body::Golem, "humain" => Body::Humain, "ver" => Body::Ver, "harpie" => Body::Harpie, "cavalier" => Body::Cavalier, _ => Body::Rodeur },
                "taille" => s.size = x, "pv" => s.hp = x, "degats" => s.dmg = x, "marche" => s.walk = x, "course" => s.run = x, "portee" => s.reach = x, "vue" => s.sight = x, "ouie" => s.hear = x,
                "recharge" => s.cd = x, "armement" => s.windup = x, "armure" => s.armor = x.clamp(0.0, 0.8), "faible" => s.weak = v, "poids" => s.weight = x, "fuite" => s.flee = x, "siege" => s.siege = x,
                "groupe" => { let (a, b) = v.split_once('-').unwrap_or((v, v)); s.group = (a.parse().unwrap_or(1), b.parse().unwrap_or(1)); }
                "biomes" => if v != "tous" { s.biomes = v.split(',').filter_map(|n| crate::biome::NAMES.iter().position(|b| b.replace(' ', "_").eq_ignore_ascii_case(n) || *b == n)).collect(); },
                "grottes" => s.caves = v.split(',').collect(), "peau" => s.skin = v, "nuit" => s.night = x > 0.5, "eau" => s.water = x > 0.5, "surface" => s.surface = x > 0.5,
                "talents" => for t in v.split(',') { match t { "bond" => s.leap = true, "grimpe" => s.climb = true, "embuscade" => s.ambush = true, "poussees" => s.bursts = true, "contourne" => s.flank = true, "brasier" => s.ember = true, "toile" => s.web = true, "vol" => s.fly = true, "spectre" => { s.fly = true; s.ghost = true; } "arc" => s.bow = true, "fouit" => s.burrow = true, "entraine" => s.drag = true, "garde" => s.guard_up = true, "charge" => s.charge = true, "meneur" => s.leader = true, "rage" => s.rage = true, "boss" => s.boss = true, _ => {} } },
                "butin" => s.loot = v.split(',').filter_map(|e| { let p: Vec<&'static str> = e.split(':').collect(); if p.len() < 3 { return None; } let (a, b) = p[1].split_once('-').unwrap_or((p[1], p[1])); Some((p[0], a.parse().ok()?, b.parse().ok()?, p[2].parse().ok()?)) }).collect(),
                _ => {}
            }
        }
        sp.push(s);
    }
    Bestiary { species: sp, rules: r }
}

// ------------------------------------------------------------------------------------------------- difficulté, régions
/// Ce que le joueur a vécu dans une région d'un biome : journées validées, présence active du jour en cours.
#[derive(Clone, Copy, Debug, Default)] pub struct Region { pub days: u32, pub secs: f64, pub day: i64 }
/// Le danger en un lieu, décomposé.
#[derive(Clone, Copy, Debug, Default)] pub struct Danger { pub biome: i32, pub distance: i32, pub adapt: i32, pub structure: i32, pub event: i32, pub total: i32, pub region: (i32, i32, u8), pub days: u32 }
pub fn biome_danger(main: usize, altitude: f64) -> i32 { let r = &bestiary().rules; if altitude >= r.mountain.0 { r.mountain.1 } else { r.biome[main.min(r.biome.len() - 1)] } }
pub fn distance_bonus(d: f64) -> i32 { bestiary().rules.distance.iter().filter(|(a, _)| d >= *a).map(|x| x.1).max().unwrap_or(0) }
pub fn adapt_cut(days: u32) -> i32 { bestiary().rules.adapt.iter().filter(|(j, _)| days >= *j).map(|x| x.1).max().unwrap_or(0) }
/// Danger = DangerBiome + max(0, BonusDistance − Adaptation) + BonusStructure + BonusÉvénement.
pub fn danger_of(biome: i32, distance: i32, adapt: i32, structure: i32, event: i32) -> i32 { biome + (distance - adapt.min(distance)).max(0) + structure + event }

// ------------------------------------------------------------------------------------------------------------ créatures
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Ai { Repos, Patrouille, Soupcon, Detection, Poursuite, Combat, Recherche, Retour, Fuite, Embuscade }
impl Ai { pub fn name(self) -> &'static str { match self { Ai::Repos => "repos", Ai::Patrouille => "patrouille", Ai::Soupcon => "soupçon", Ai::Detection => "détection", Ai::Poursuite => "poursuite", Ai::Combat => "combat", Ai::Recherche => "recherche", Ai::Retour => "retour", Ai::Fuite => "fuite", Ai::Embuscade => "embuscade" } } }
#[derive(Clone, Debug)]
pub struct Foe {
    pub sp: usize, pub p: [f64; 3], pub yaw: f64, pub hp: f64, pub max: f64, pub state: Ai, t: f64, cd: f64,
    /// Attaque annoncée en cours : temps restant avant le coup (0 : pas d'attaque).
    pub wind: f64, pub hit: f64, pub phase: f64, pub speed: f64, v: [f64; 3], gy: f64, /** gardien d'un lieu : le jour ne le chasse pas */ pub kept: bool,
    /// Où elle croit le joueur, et depuis quand elle ne l'a plus vu ; l'éveil monte avec ce qu'elle voit et entend.
    pub seen: Option<[f64; 3]>, lost: f64, pub alert: f64, pub(crate) home: [f64; 3],
    /// Mort en cours (s) ; variante d'équipement ; vétéran (un peu plus dur, meilleur butin) ; bond ou course en cours.
    /** bouclier levé (brigand), de 0 à 1 */ pub guard: f64, pub dead: f64, pub variant: u8, pub vet: bool, pub air: bool, burst: f64, pub climbing: bool, pub group: u32,
    /** ver des sables : secondes pendant lesquelles il reste dressé hors du sable (on peut alors le frapper) */ pub up: f64,
    /** une bande qui attaque un village : les points par où elle passe (devant la porte, le passage, la réserve) */ pub raid: Vec<[f64; 3]>,
    /** l'homme d'un camp de brigands (sa maille) : sa mort compte pour la dispersion du camp */ pub camp: Option<(i32, i32)>,
    /** sa marche : la plus courte distance atteinte vers le point suivant, et depuis combien de temps elle n'avance plus */ best: f64, stuck: f64,
    /** combat de boss : sa garde entamée (0 à `POSTURE`), le temps qu'il chancelle, l'attaque en cours, les coups qui restent à son
    enchaînement, sa rage (seconde phase), sa ruée (s), le temps depuis la dernière atteinte à sa garde */
    pub posture: f64, pub broken: f64, pub mv: u8, combo: u8, pub enraged: bool, pub lunge: f64, calm: f64,
}
/// Ce que font les créatures, pour la session (coups, sons, butin).
#[derive(Clone, Debug)] pub enum FoeEv { Hit { from: [f64; 3], dmg: f64, cause: &'static str, push: f64 }, Growl([f64; 3], Body), Swing([f64; 3]), Blocked([f64; 3]), Die([f64; 3], usize, bool, Option<(i32, i32)>), /** une morsure qui englue : ralenti pendant tant de secondes */ Snare(f64), /** un pillard est arrivé à la réserve du village (sa bande) */ Loot([f64; 3], u32),
    /** un coup qu'aucun bouclier n'arrête (seule la roulade l'évite) */ Crush { from: [f64; 3], dmg: f64, cause: &'static str, push: f64 },
    /** sa garde brisée : il chancelle */ Reel([f64; 3]), /** il entre dans sa seconde phase, et appelle ses hommes */ Rage([f64; 3], &'static str), /** le cavalier vide les étriers : son cheval s'enfuit */ Unhorse([f64; 3], f64) }

pub struct Horde { pub foes: Vec<Foe>, rng: Rng, spawn_t: f64, pub ev: Vec<FoeEv>, groups: u32, pub spawned: u32, /** braises semées par les rampants de magma : (position, secondes restantes) */ pub embers: Vec<([f64; 3], f64)>, burn_cd: f64, /** flèches des archers en vol : (position, vitesse, âge, dégâts, ce qui l'a tirée) */ pub shots: Vec<([f64; 3], [f64; 3], f64, f64, &'static str)> }
/// Ce que la session donne à la horde à chaque image.
pub struct Ctx<'a> {
    pub eye: [f64; 3], pub feet: [f64; 3], pub look: [f64; 3], pub noise: f64, pub lit: bool, pub crouch: bool, pub alive: bool,
    pub night: f64, pub danger: i32, pub can_spawn: bool, pub light: &'a dyn Fn(i32, i32, i32) -> u8, pub protect: &'a dyn Fn(f64, f64) -> bool, pub main_biome: usize,
    /// Sous terre (0 à 1), et le biome souterrain où l'on se trouve (sa clé : « karst », « cristal »…).
    pub cave: f64, pub under: Option<&'static str>,
}

fn sees(w: &impl Voxels, a: [f64; 3], b: [f64; 3]) -> bool {
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]]; let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt(); let n = (l / 0.45).ceil() as i32;
    for i in 1..n { let t = i as f64 / n as f64; let q = [a[0] + d[0] * t, a[1] + d[1] * t, a[2] + d[2] * t]; if w.get(q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32).is_none_or(is_opaque) { return false; } } true
}
fn eye_h(b: Body) -> f64 { match b { Body::Rodeur => 1.55, Body::Brigand => 1.62, Body::Bete => 0.8, Body::Rampant => 0.55, Body::Araignee => 0.4, Body::Golem => 1.9, Body::Humain => 1.5, Body::Ver => 0.6, Body::Harpie => 0.3, Body::Cavalier => 2.65 } }
/// Boîte de visée et de collision (demi-largeur, hauteur).
pub fn foe_box(b: Body, size: f64) -> (f64, f64) { let (w, h) = match b { Body::Rodeur => (0.42, 1.85), Body::Brigand => (0.42, 1.85), Body::Bete => (0.5, 1.0), Body::Rampant => (0.55, 0.8), Body::Araignee => (0.55, 0.65), Body::Golem => (0.6, 2.25), Body::Humain => (0.4, 1.8) /* le sylvain, plus massif, garde cette boîte : on le frappe au tronc */, Body::Ver => (0.5, 2.4), Body::Harpie => (0.6, 0.9), Body::Cavalier => (0.62, 2.8) }; (w * size, h * size) }

impl Horde {
    pub fn new(seed: i32) -> Self { Horde { foes: Vec::new(), rng: Rng::new((seed ^ 0x6d21) as u32), spawn_t: 2.0, ev: Vec::new(), groups: 0, spawned: 0, embers: Vec::new(), burn_cd: 0.0, shots: Vec::new() } }
    /// Fait apparaître une créature de l'espèce `sp` en `p` (bancs d'essai, outils de développement, vagues).
    pub fn put(&mut self, sp: usize, p: [f64; 3], yaw: f64, vet: bool) -> usize {
        let s = &bestiary().species[sp]; let max = s.hp * if vet { 1.25 } else { 1.0 }; self.groups += 1; let variant = (self.rng.next() * 3.0) as u8;
        self.foes.push(Foe { sp, p, yaw, hp: max, max, state: if s.ambush { Ai::Embuscade } else { Ai::Repos }, t: self.rng.range(1.0, 4.0), cd: 0.5, wind: 0.0, hit: 0.0, phase: 0.0, speed: 0.0, v: [0.0; 3], gy: p[1],
            seen: None, lost: 0.0, alert: 0.0, home: p, guard: 0.0, dead: 0.0, variant, vet, air: false, burst: 0.0, climbing: false, group: self.groups, up: 0.0, kept: false, raid: Vec::new(), camp: None, best: f64::MAX, stuck: 0.0, posture: 0.0, broken: 0.0, mv: 0, combo: 0, enraged: false, lunge: 0.0, calm: 0.0 });
        self.spawned += 1; self.foes.len() - 1
    }
    /// Un coup porté : moins fort sur une armure, plus fort avec l'arme qui la blesse le mieux ; rend vrai si elle meurt.
    pub fn strike(&mut self, i: usize, dmg: f64, from: [f64; 3], weapon: &str, heavy: bool) -> bool {
        let Some(f) = self.foes.get_mut(i) else { return false }; if f.dead > 0.0 { return false; } let s = &bestiary().species[f.sp];
        if s.burrow && f.wind <= 0.0 && f.up <= 0.0 { f.alert = 1.0; return false; } // sous le sable, on ne frappe que la dune
        let mul = if !s.weak.is_empty() && s.weak == weapon { 1.35 } else { 1.0 };
        // Bouclier levé et coup de face : il arrête l'essentiel (une flèche aussi).
        let face = { let (dx, dz) = (from[0] - f.p[0], from[2] - f.p[2]); let d = dx.hypot(dz).max(0.01); (sin(f.yaw) * dx + cos(f.yaw) * dz) / d };
        let blocked = f.guard > 0.5 && face > 0.3; if blocked { self.ev.push(FoeEv::Blocked(f.p)); if s.guard_up { f.cd = f.cd.min(0.15); } } // l'épéiste pare, et riposte aussitôt
        // Le boss : sa garde encaisse (la vie bouge à peine) tant qu'elle tient ; brisée, il chancelle et chaque coup porte double et demi.
        let boss = if f.broken > 0.0 { BROKEN_MUL } else if !s.boss { 1.0 } else { f.calm = 0.0; f.posture += dmg * if heavy { 1.0 } else { 0.5 }; let guarded = if s.body == Body::Cavalier { MOUNTED_MUL } else if s.bow { 0.7 } else { GUARDED_MUL }; if f.posture >= POSTURE { reel(f, &mut self.ev); BROKEN_MUL } else { guarded } };
        f.hp -= dmg * (1.0 - s.armor) * mul * boss * if blocked { 0.2 } else { 1.0 }; f.hit = if blocked { 0.0 } else { 0.3 };
        let (dx, dz) = (f.p[0] - from[0], f.p[2] - from[2]); let d = dx.hypot(dz).max(0.1); let k = if heavy { 6.5 } else { 3.2 } / s.size / if s.boss { 4.0 } else { 1.0 }; f.v = [dx / d * k, 0.0, dz / d * k];
        // Un coup interrompt l'attaque annoncée (le joueur qui frappe le premier garde l'avantage) ; elle sait maintenant où il est.
        // Le boss, lui, ne se laisse pas interrompre : il faut lire ses attaques.
        if !s.boss && (heavy || f.wind > s.windup * 0.5) { f.wind = 0.0; f.cd = f.cd.max(0.6); }
        f.seen = Some(from); f.alert = 1.0; f.lost = 0.0; if matches!(f.state, Ai::Repos | Ai::Patrouille | Ai::Soupcon | Ai::Embuscade | Ai::Retour | Ai::Recherche) { f.state = Ai::Poursuite; }
        if f.hp <= 0.0 { f.dead = 1e-3; f.wind = 0.0; self.ev.push(FoeEv::Die(f.p, f.sp, f.vet, f.camp)); return true; }
        if s.flee > 0.0 && f.hp < f.max * s.flee { f.state = Ai::Fuite; f.t = 7.0; }
        false
    }
    /// Repoussée (parade, coup lourd) : elle recule et perd son attaque.
    /// La créature `i` ignore-t-elle le joueur (au repos, en ronde, intriguée, de retour) ? Rend alors si c'est un boss.
    pub fn unaware(&self, i: usize) -> Option<bool> { let f = self.foes.get(i)?; (f.dead == 0.0 && matches!(f.state, Ai::Repos | Ai::Patrouille | Ai::Soupcon | Ai::Retour)).then(|| bestiary().species[f.sp].boss) }
    /// Une créature vous traque-t-elle (repéré, poursuivi, combattu) à moins de `r` pas ?
    pub fn hunted(&self, p: [f64; 3], r: f64) -> bool { self.foes.iter().any(|f| f.dead == 0.0 && matches!(f.state, Ai::Detection | Ai::Poursuite | Ai::Combat) && (f.p[0] - p[0]).hypot(f.p[2] - p[2]) < r) }
    /// Le boss ne recule pas : une parade (ou un coup lourd) entame sa garde ; trop entamée, elle se brise et il chancelle.
    pub fn stagger(&mut self, i: usize, from: [f64; 3], secs: f64) { if let Some(f) = self.foes.get_mut(i) { if bestiary().species[f.sp].boss { if f.broken <= 0.0 && f.dead == 0.0 { f.calm = 0.0; f.posture += if secs >= 2.0 { PARRY_POSTURE } else { 12.0 }; f.hit = 0.15; if f.posture >= POSTURE { reel(f, &mut self.ev); } } return; } let (dx, dz) = (f.p[0] - from[0], f.p[2] - from[2]); let d = dx.hypot(dz).max(0.1); f.v = [dx / d * 6.0, 0.0, dz / d * 6.0]; f.wind = 0.0; f.cd = f.cd.max(secs); f.hit = 0.3; } }
    /// La créature vivante la plus proche d'un point (celle qui vient de frapper).
    pub fn nearest(&self, p: [f64; 3]) -> Option<usize> { self.foes.iter().enumerate().filter(|(_, f)| f.dead == 0.0).map(|(i, f)| (i, (f.p[0] - p[0]).hypot(f.p[2] - p[2]))).filter(|x| x.1 < 3.5).min_by(|a, b| a.1.total_cmp(&b.1)).map(|x| x.0) }
    /// Un tirage (butin, vagues).
    pub fn roll(&mut self) -> f64 { self.rng.next() }
    pub fn alive(&self) -> usize { self.foes.iter().filter(|f| f.dead == 0.0).count() }

    /// Choisit l'espèce à faire naître ici : les communes partout, celles du biome en plus ; le danger élève parfois la créature en vétéran.
    fn pick(&mut self, biome: usize, danger: i32, under: Option<&str>, night: bool) -> Option<(usize, bool)> {
        let bs = bestiary(); let tier_w = |t: u8| match t { 1 => 1.0, 2 => (danger - 2).max(0) as f64 * 0.22, 3 => (danger - 4).max(0) as f64 * 0.12, _ => (danger - 5).max(0) as f64 * 0.03 };
        let c: Vec<(usize, f64)> = bs.species.iter().enumerate().filter(|(_, s)| !(s.night && !night && under.is_none()) && !(s.surface && under.is_some())).filter(|(_, s)| if s.caves.is_empty() { s.biomes.is_empty() || s.biomes.contains(&biome) } else { under.is_some_and(|u| s.caves.iter().any(|k| *k == "toutes" || *k == u)) }).map(|(i, s)| (i, s.weight * tier_w(s.tier))).filter(|x| x.1 > 0.0).collect();
        let sum: f64 = c.iter().map(|x| x.1).sum(); if sum <= 0.0 { return None; } let mut r = self.rng.next() * sum;
        let sp = c.iter().find(|x| { r -= x.1; r <= 0.0 }).unwrap_or(&c[c.len() - 1]).0; let vet = self.rng.next() < ((danger - 2).max(0) as f64 * 0.07).min(0.45); Some((sp, vet))
    }
    /// Une tentative d'apparition, loin des yeux du joueur, dans le noir, sur un sol où le corps tient, hors des lieux protégés.
    fn try_spawn(&mut self, w: &impl Voxels, c: &Ctx) {
        let r = &bestiary().rules; let me = c.feet;
        // La nuit, elles sont plus nombreuses : la moitié de plus au plafond, autour du joueur aussi.
        let deep = c.cave > 0.5; let (cap, cap_near) = if c.night > 0.5 || deep { (r.cap * 3 / 2, r.cap_near * 3 / 2) } else { (r.cap, r.cap_near) };
        if self.foes.len() >= cap { return; }
        let near = self.foes.iter().filter(|f| hypot2(f.p[0] - me[0], f.p[2] - me[2]) < 24.0).count(); if near >= cap_near { return; }
        let Some((sp, vet)) = self.pick(c.main_biome, c.danger, if deep { Some(c.under.unwrap_or("rocheuse")) } else { None }, c.night > 0.5) else { return }; let s = &bestiary().species[sp];
        for _ in 0..if c.night > 0.5 || deep { 8 } else { 4 } {
            let a = self.rng.range(0.0, std::f64::consts::TAU); let d = self.rng.range(r.ring.0, r.ring.1); let (x, z) = (me[0] + sin(a) * d, me[2] + cos(a) * d);
            if (c.protect)(x, z) { continue; }
            // sous terre, la créature naît dans la galerie (son sol, sous sa voûte), pas sur la colline au-dessus
            let found = if deep { crate::fauna::cave_floor(w, x, z, me[1]).map(|f| f.0) } else { ground(w, x, z, me[1] + 12.0).or_else(|| ground(w, x, z, me[1] - 8.0)) };
            let Some(y) = found else { continue }; if (y - me[1]).abs() > 24.0 { continue; }
            let (bx, by, bz) = (x.floor() as i32, y.floor() as i32, z.floor() as i32); if (c.light)(bx, by, bz) >= r.light { continue; }
            // le noyé sort de l'eau : il lui faut une rive à quelques pas ; le ver, du sable sous lui
            if s.water && !(-5..=5).any(|dx| (-5..=5).any(|dz| (-2..=0).any(|dy| w.get(bx + dx, by + dy, bz + dz).is_some_and(crate::blocks::is_water)))) { continue; }
            if s.burrow && !sandy(w, x, y, z) { continue; }
            // Jamais sous les yeux du joueur : hors de son champ, ou cachée par le relief.
            let (dx, dy, dz) = (x - c.eye[0], y + 1.0 - c.eye[1], z - c.eye[2]); let l = (dx * dx + dy * dy + dz * dz).sqrt().max(0.1);
            let ahead = (dx * c.look[0] + dy * c.look[1] + dz * c.look[2]) / l; if ahead > 0.35 && sees(w, c.eye, [x, y + 1.0, z]) { continue; }
            let yaw = self.rng.range(0.0, 6.283); let tall = (foe_box(s.body, s.size).1.ceil() as i32).max(1); if !fits(w, x, y, z, yaw, tall) { continue; }
            let n = self.rng.range(s.group.0 as f64, s.group.1 as f64 + 0.999).floor() as u32 + if c.danger >= 5 && self.rng.next() < 0.3 { 1 } else { 0 };
            let lift = if s.ghost { 0.6 } else if s.fly { 7.0 } else { 0.0 }; // les volants naissent en l'air
            let first = self.put(sp, [x, y + lift, z], yaw, vet); let g = self.foes[first].group;
            for k in 1..n { let o = [x + self.rng.range(-2.5, 2.5), y, z + self.rng.range(-2.5, 2.5)]; if let Some(oy) = ground(w, o[0], o[2], y + 2.0) { if (oy - y).abs() <= 2.0 && (c.light)(o[0].floor() as i32, oy.floor() as i32, o[2].floor() as i32) < r.light && fits(w, o[0], oy, o[2], yaw, tall) { let i = self.put(sp, [o[0], oy, o[2]], yaw + k as f64, vet && k == 0); self.foes[i].group = g; } } }
            return;
        }
    }

    pub fn update(&mut self, dt: f64, w: &impl Voxels, c: &Ctx) {
        let r = &bestiary().rules; let me = c.feet;
        if c.can_spawn { self.spawn_t -= dt; if self.spawn_t <= 0.0 { self.spawn_t = r.every * self.rng.range(0.7, 1.3) * if c.night > 0.5 { 0.55 } else if c.cave > 0.5 { 0.8 } else { 1.0 }; if c.night > 0.5 || c.cave > 0.5 || (c.light)(me[0].floor() as i32, me[1].floor() as i32 + 1, me[2].floor() as i32) < 4 { self.try_spawn(w, c); } } }
        let mut all = std::mem::take(&mut self.foes); let n = all.len();
        let groups: Vec<(u32, [f64; 3])> = all.iter().map(|f| (f.group, f.p)).collect();
        // Le chef de guerre : tant qu'il vit, les siens tiennent ; quand il se bat, son cri ameute tous les brigands alentour.
        let leaders: Vec<[f64; 3]> = all.iter().filter(|f| f.dead == 0.0 && bestiary().species[f.sp].leader).map(|f| f.p).collect();
        let cry: Vec<[f64; 3]> = all.iter().filter(|f| f.dead == 0.0 && bestiary().species[f.sp].leader && matches!(f.state, Ai::Poursuite | Ai::Combat)).map(|f| f.p).collect();
        if !cry.is_empty() && c.alive { for f in all.iter_mut() { let s = &bestiary().species[f.sp];
            if f.dead == 0.0 && matches!(s.body, Body::Brigand | Body::Cavalier) && matches!(f.state, Ai::Repos | Ai::Patrouille | Ai::Soupcon | Ai::Retour | Ai::Recherche) && cry.iter().any(|q| hypot2(q[0] - f.p[0], q[2] - f.p[2]) < 22.0) { f.alert = 1.0; f.seen = Some(me); f.lost = 0.0; f.state = Ai::Detection; f.t = 0.3; } } }
        for i in 0..n { let f = &mut all[i]; self.think(f, dt, w, c, &groups, &leaders); }
        // Les flèches des archers : elles volent, retombent, se plantent ; celle qui atteint le joueur le blesse (le bouclier l'arrête de face).
        let mut shots = std::mem::take(&mut self.shots);
        for a in shots.iter_mut() { let steps = 3; for _ in 0..steps { let h = dt / steps as f64; a.1[1] -= 12.0 * h; for k in 0..3 { a.0[k] += a.1[k] * h; }
            if w.get(a.0[0].floor() as i32, a.0[1].floor() as i32, a.0[2].floor() as i32).is_none_or(|b| b != 0 && is_solid(b)) { a.2 = 99.0; break; }
            if c.alive && hypot2(a.0[0] - me[0], a.0[2] - me[2]) < 0.42 && a.0[1] > me[1] - 0.1 && a.0[1] < me[1] + 1.85 { let l = hypot2(a.1[0], a.1[2]).max(0.1);
                let from = [me[0] - a.1[0] / l * 3.0, me[1], me[2] - a.1[2] / l * 3.0]; self.ev.push(if a.4.starts_with(HEAVY_ARROW) { FoeEv::Crush { from, dmg: a.3, cause: a.4, push: 3.0 } } else { FoeEv::Hit { from, dmg: a.3, cause: a.4, push: 1.2 } }); a.2 = 99.0; break; } }
            a.2 += dt; }
        shots.retain(|a| a.2 < 5.0); self.shots = shots;
        // Les braises s'éteignent ; marcher dessus, ou serrer de trop près un rampant de magma, brûle.
        for e in self.embers.iter_mut() { e.1 -= dt; } self.embers.retain(|e| e.1 > 0.0); self.burn_cd -= dt;
        if c.alive && self.burn_cd <= 0.0 {
            let hot = self.embers.iter().any(|e| hypot2(e.0[0] - me[0], e.0[2] - me[2]) < 0.7 && (e.0[1] - me[1]).abs() < 1.0)
                || all.iter().any(|f| f.dead == 0.0 && bestiary().species[f.sp].ember && hypot2(f.p[0] - me[0], f.p[2] - me[2]) < 1.25 && (f.p[1] - me[1]).abs() < 1.5);
            if hot { self.burn_cd = 0.7; self.ev.push(FoeEv::Hit { from: me, dmg: 2.0, cause: "Les braises d'un rampant de magma", push: 0.0 }); }
        }
        // Au lever du jour, celles qui ne se battent pas s'éloignent et s'effacent hors de vue ; les mortes, une fois tombées ; les trop lointaines.
        let day = c.night < 0.2 && c.cave < 0.5; // sous terre, il n'y a pas de lever du jour
        all.retain(|f| { let d = hypot2(f.p[0] - me[0], f.p[2] - me[2]); !(f.dead > 1.6 || d > r.forget && f.raid.is_empty() || (day && !f.kept && f.state != Ai::Combat && d > 20.0 && f.dead == 0.0) || f.p[1] < -20.0 || w.get(f.p[0].floor() as i32, f.p[1].floor() as i32, f.p[2].floor() as i32).is_none()) });
        self.foes = all;
    }

    fn think(&mut self, f: &mut Foe, dt: f64, w: &impl Voxels, c: &Ctx, groups: &[(u32, [f64; 3])], leaders: &[[f64; 3]]) {
        let s = &bestiary().species[f.sp]; let r = &mut self.rng; f.hit = (f.hit - dt).max(0.0);
        if f.dead > 0.0 { f.dead += dt; f.speed = 0.0; settle(f, dt, w); return; }
        if s.fly { self.fly_think(f, dt, w, c); return; } if s.burrow { self.worm_think(f, dt, w, c); return; }
        let me = c.feet; let dist = hypot2(f.p[0] - me[0], f.p[2] - me[2]); let dy = me[1] - f.p[1];
        // Loin du joueur, une créature ne pense qu'une image sur trois.
        if dist > 40.0 && f.state != Ai::Poursuite { f.phase += dt; if r.next() > 0.34 { return; } }
        f.t -= dt; f.cd -= dt; f.burst -= dt;
        // ---- perception : la vue (distance, champ, obstacles, lumière portée par le joueur), puis l'ouïe.
        let fe = [f.p[0], f.p[1] + eye_h(s.body) * s.size, f.p[2]]; let fwd = [sin(f.yaw), cos(f.yaw)]; let to = [(me[0] - f.p[0]) / dist.max(0.01), (me[2] - f.p[2]) / dist.max(0.01)];
        let range = s.sight * if c.lit { 1.35 } else { 1.0 } * if c.crouch { 0.6 } else { 1.0 } * if c.night < 0.3 { 1.2 } else { 1.0 };
        let cone = fwd[0] * to[0] + fwd[1] * to[1] > if f.state == Ai::Poursuite || f.state == Ai::Combat { -0.6 } else { -0.15 };
        let see = c.alive && dist < range && (dist < 3.0 || cone) && sees(w, fe, c.eye);
        let hear = c.alive && c.noise > 0.0 && dist < s.hear * c.noise;
        if see { f.alert = (f.alert + dt * (1.4 + 6.0 / dist.max(1.0))).min(1.0); f.seen = Some(me); f.lost = 0.0; }
        else { f.lost += dt; if hear { f.alert = (f.alert + dt * 0.7).min(1.0); f.seen = Some([me[0] + r.range(-2.0, 2.0), me[1], me[2] + r.range(-2.0, 2.0)]); } else { f.alert = (f.alert - dt * 0.08).max(0.0); } }
        // ---- le boss au combat : sa propre façon de se battre (voir `boss_think`)
        if !s.boss && f.broken > 0.0 { f.broken -= dt; f.speed = 0.0; f.wind = 0.0; f.phase += dt * 1.5; knock(f, dt, w); settle(f, dt, w); return; } // à terre, sonné
        if s.boss && f.dead == 0.0 { if matches!(f.state, Ai::Poursuite | Ai::Combat) && match s.body { Body::Cavalier => Self::rider_think(r, &mut self.ev, f, dt, c, see, dist, dy, to), _ if s.bow => Self::captain_think(r, &mut self.ev, &mut self.shots, f, dt, c, see, dist, dy, to), _ => Self::boss_think(r, &mut self.ev, f, dt, c, see, dist, dy, to) } {
            f.phase += dt * (2.0 + f.speed * 1.8); knock(f, dt, w); if f.speed > 0.0 { let step = f.speed * dt; walk(f, w, step, false, s.body, !f.raid.is_empty()); } settle(f, dt, w); return; }
            f.mv = 0; f.combo = 0; f.lunge = 0.0; if f.broken > 0.0 { f.broken = 0.0; } if !matches!(f.state, Ai::Poursuite | Ai::Combat) { f.wind = 0.0; } }
        // ---- l'état
        use Ai::*;
        match f.state {
            Repos => { f.speed = 0.0; if f.alert > 0.25 { f.state = Soupcon; f.t = 6.0; } else if f.t <= 0.0 { f.state = Patrouille; f.t = r.range(3.0, 7.0); f.yaw += r.range(-1.4, 1.4); } }
            Patrouille => { f.speed = s.walk; if f.alert > 0.25 { f.state = Soupcon; f.t = 6.0; } else if f.t <= 0.0 { f.state = Repos; f.t = r.range(2.0, 6.0); }
                if hypot2(f.p[0] - f.home[0], f.p[2] - f.home[2]) > 14.0 { f.yaw = (f.home[0] - f.p[0]).atan2(f.home[2] - f.p[2]); } }
            Embuscade => { f.speed = 0.0; if see && dist < 9.0 || f.alert > 0.8 || f.hit > 0.0 { f.state = Detection; f.t = 0.25; self.ev.push(FoeEv::Growl(f.p, s.body)); } }
            Soupcon => { if let Some(q) = f.seen { f.yaw = turn(f.yaw, (q[0] - f.p[0]).atan2(q[2] - f.p[2]), dt * 3.0); f.speed = if hypot2(q[0] - f.p[0], q[2] - f.p[2]) > 2.0 { s.walk } else { 0.0 }; }
                if f.alert >= 1.0 { f.state = Detection; f.t = 0.55; self.ev.push(FoeEv::Growl(f.p, s.body)); } else if f.t <= 0.0 && f.alert < 0.2 { f.state = Retour; } }
            Detection => { f.speed = 0.0; if let Some(q) = f.seen { f.yaw = turn(f.yaw, (q[0] - f.p[0]).atan2(q[2] - f.p[2]), dt * 8.0); } if f.t <= 0.0 { f.state = Poursuite; f.t = 20.0; } }
            Poursuite => {
                let goal = f.seen.unwrap_or(me); let mut want = (goal[0] - f.p[0]).atan2(goal[2] - f.p[2]);
                // Les brigands contournent : chacun de leur côté, avant de fondre sur le joueur.
                if s.flank && dist > 3.0 && dist < 9.0 { let side = if f.group % 2 == 0 { 0.85 } else { -0.85 }; let same = groups.iter().filter(|g| g.0 == f.group).count(); if same > 1 { want += side; } }
                // Le cavalier en pleine charge tient son cap : on l'esquive d'un pas de côté.
                let charging = s.charge && f.burst > 0.0;
                f.yaw = turn(f.yaw, want, dt * if charging { 0.9 } else { 6.0 });
                // Le rôdeur traîne, puis accélère d'un coup ; la bête fond ; les autres courent.
                let run = if s.bursts { if f.burst > 0.0 { s.run } else { if r.next() < dt * 0.35 { f.burst = r.range(1.2, 2.0); } s.walk * 1.6 } } else { s.run };
                f.speed = run * if f.vet { 1.08 } else { 1.0 } * if s.rage && f.hp < f.max * 0.5 { 1.15 } else { 1.0 };
                // La charge : de loin, il fond au galop, lance couchée ; au passage, le coup porte fort et renverse ; puis il fait volte-face.
                if s.charge {
                    if charging { f.speed = s.run * 1.3;
                        let face = sin(f.yaw) * to[0] + cos(f.yaw) * to[1];
                        if c.alive && f.cd <= 0.0 && dist < s.reach + 0.7 && dy.abs() < 2.2 && face > 0.2 { self.ev.push(FoeEv::Hit { from: f.p, dmg: s.dmg * 1.6 * r.range(0.9, 1.2) * if f.vet { 1.15 } else { 1.0 }, cause: "La charge d'un cavalier brigand", push: 7.0 }); f.cd = s.cd + 1.2; f.burst = f.burst.min(0.6); }
                    } else if see && f.cd <= 0.0 && (6.0..22.0).contains(&dist) && dy.abs() < 2.5 && r.next() < dt * 2.5 { f.burst = (dist / (s.run * 1.3) + 0.9).min(3.0); self.ev.push(FoeEv::Growl(f.p, s.body)); }
                }
                if !charging && see && dist < s.reach + 0.05 && dy.abs() < 2.2 { f.state = Combat; f.speed = 0.0; }
                if s.leap && see && f.cd <= 0.0 && !f.air && (3.0..6.5).contains(&dist) && dy.abs() < 1.5 && r.next() < dt * 3.0 { f.air = true; f.v = [to[0] * 9.5, 5.5, to[1] * 9.5]; f.wind = 0.6; f.cd = s.cd + 1.0; self.ev.push(FoeEv::Growl(f.p, s.body)); }
                if !see && f.lost > 4.0 { f.state = Recherche; f.t = 8.0; }
                if !c.alive { f.state = Retour; }
            }
            Combat => {
                f.speed = 0.0; f.yaw = turn(f.yaw, (me[0] - f.p[0]).atan2(me[2] - f.p[2]), dt * if f.wind > 0.0 { 2.0 } else { 7.0 });
                // L'attaque s'annonce (bras levé, gueule ouverte, corps ramassé), puis tombe : esquivable, parable.
                if f.wind <= 0.0 && f.cd <= 0.0 { f.wind = s.windup; f.mv = 0; self.ev.push(FoeEv::Swing(f.p));
                    // les vétérans ont leur botte : l'épéiste feinte (un geste à moitié, puis le vrai coup, vite) ; l'archer bande longuement son arc
                    if f.vet && s.guard_up && r.next() < 0.35 { f.mv = MV_FEINTE; } if f.vet && s.bow && r.next() < 0.4 { f.mv = MV_TIR; f.wind = s.windup + WIND_TIR; } }
                // Le brigand au bouclier se couvre entre deux coups.
                if shielded(s, f.variant) { let want = if f.wind <= 0.0 && f.cd > 0.3 { 1.0 } else { 0.0 }; f.guard += (want - f.guard) * (dt * 8.0).min(1.0); }
                if dist > s.reach + 1.2 && f.wind <= 0.0 { f.state = Poursuite; }
                if s.bow && (dist < 3.5 || !see) && f.wind <= 0.0 { if dist < 3.5 { f.state = Fuite; f.t = 2.2; } else { f.state = Poursuite; } } // l'archer rompt quand on le serre, et cherche une ligne de tir
                if !c.alive { f.state = Retour; }
            }
            Recherche => { if let Some(q) = f.seen { if hypot2(q[0] - f.p[0], q[2] - f.p[2]) > 1.5 { f.yaw = turn(f.yaw, (q[0] - f.p[0]).atan2(q[2] - f.p[2]), dt * 4.0); f.speed = s.walk * 1.5; } else { f.speed = 0.0; f.yaw += dt * 1.2; } }
                if see && f.alert > 0.6 { f.state = Poursuite; } else if f.t <= 0.0 { f.state = Retour; f.seen = None; } }
            Retour => { f.speed = s.walk; f.yaw = turn(f.yaw, (f.home[0] - f.p[0]).atan2(f.home[2] - f.p[2]), dt * 3.0); if hypot2(f.p[0] - f.home[0], f.p[2] - f.home[2]) < 2.0 { f.state = Repos; f.t = r.range(2.0, 5.0); } if f.alert > 0.6 && see { f.state = Poursuite; } }
            Fuite => { f.speed = s.run * 0.9; f.yaw = turn(f.yaw, (f.p[0] - me[0]).atan2(f.p[2] - me[2]), dt * 6.0); if f.t <= 0.0 { f.state = Retour; f.home = f.p; } }
        }
        // L'épéiste avance bouclier levé (contre les flèches aussi), plus lentement ; les autres le baissent hors du combat.
        let cover = s.guard_up && f.wind <= 0.0 && ((f.state == Poursuite && see && dist < 12.0) || (!f.raid.is_empty() && f.state != Combat));
        if cover { f.guard += (1.0 - f.guard) * (dt * 6.0).min(1.0); if f.state == Poursuite { f.speed *= 0.75; } } else if f.state != Combat { f.guard = (f.guard - dt * 4.0).max(0.0); }
        // Près de leur chef, les brigands ne fuient pas (l'archer qui rompt pour tirer de plus loin, si).
        if f.state == Fuite && !s.bow && !s.leader && matches!(s.body, Body::Brigand | Body::Cavalier) && leaders.iter().any(|q| hypot2(q[0] - f.p[0], q[2] - f.p[2]) < 20.0) { f.state = Poursuite; f.t = 20.0; }
        // ---- une bande qui attaque un village : tant qu'elle ne se bat pas, elle marche droit sur ses points de passage
        if !f.raid.is_empty() && matches!(f.state, Repos | Patrouille | Retour) {
            let wp = f.raid[0];
            let d = hypot2(wp[0] - f.p[0], wp[2] - f.p[2]);
            if d < 2.2 { f.raid.remove(0); f.best = f64::MAX; f.stuck = 0.0; if f.raid.is_empty() { self.ev.push(FoeEv::Loot(f.p, f.group)); } }
            else { f.state = Patrouille; f.t = 4.0; f.home = f.p; f.yaw = turn(f.yaw, (wp[0] - f.p[0]).atan2(wp[2] - f.p[2]), dt * 4.0); f.speed = s.walk * 1.8;
                // elle n'avance plus (un talus, un fossé, un ruisseau) : elle s'y hisse ou s'y laisse glisser, d'un pas vers son but ; d'un
                // ruisseau, elle remonte sur la berge — jamais par-dessus une muraille, une herse ou un toit
                // toutes les trois secondes : a-t-elle gagné un pas vers son but ?
                if f.best == f64::MAX { f.best = d; } f.stuck += dt; let slow = f.stuck > 3.0 && d > f.best - 1.0; if f.stuck > 3.0 { f.stuck = 0.0; f.best = d; }
                if slow { let wet = (0..3).any(|k| w.get(f.p[0].floor() as i32, f.gy as i32 + k, f.p[2].floor() as i32).is_some_and(crate::blocks::is_water)); let up = if wet { 5.5 } else { 3.1 }; let a0 = (wp[0] - f.p[0]).atan2(wp[2] - f.p[2]); let tall = (foe_box(s.body, s.size).1.ceil() as i32).max(1);
                    for k in [0.0, 0.6, -0.6, 1.2, -1.2] { let (nx, nz) = (f.p[0] + sin(a0 + k) * 1.3, f.p[2] + cos(a0 + k) * 1.3);
                        if let Some(y) = ground(w, nx, nz, f.gy + up + 0.1).or_else(|| ford(w, nx, nz, f.gy + 1.0)) { let built = w.get(nx.floor() as i32, y as i32 - 1, nz.floor() as i32).is_some_and(|b| { let n = BLOCKS[b as usize].name; n.starts_with("iron_bars") || n.starts_with("stone_bricks") || n.starts_with("planks") || n.starts_with("roof_tiles") || n.starts_with("oak_log") || n.starts_with("fence") || n.starts_with("plaster") });
                            if !built && y - f.gy <= up && f.gy - y <= 6.0 && (fits(w, nx, y, nz, a0 + k, tall) || wet_fits(w, nx, y, nz, tall)) { f.p = [nx, y, nz]; f.gy = y; f.best = hypot2(wp[0] - nx, wp[2] - nz); break; } } } } }
        }
        // ---- le coup tombe à la fin de l'annonce, s'il y a contact (distance et angle cohérents avec le geste)
        if f.wind > 0.0 && !s.boss { f.wind -= dt; if f.mv == MV_FEINTE && f.wind < s.windup * 0.45 { f.mv = 0; f.wind = 0.2; } // la feinte : le geste s'arrête net, le vrai coup part
            if f.wind <= 0.0 { f.wind = 0.0; let heavy = f.mv == MV_TIR; f.mv = 0; f.cd = s.cd * if f.vet { 0.9 } else { 1.0 } * if s.rage && f.hp < f.max * 0.5 { 0.6 } else { 1.0 };
            let face = sin(f.yaw) * to[0] + cos(f.yaw) * to[1]; let reach = s.reach + if f.air { 0.8 } else { 0.5 };
            if s.bow { // la flèche part vers où l'on se tient, un peu au-dessus pour la chute ; elle n'est pas infaillible
                if c.alive && see { let from = [f.p[0] + sin(f.yaw) * 0.4, f.p[1] + 1.35 * s.size, f.p[2] + cos(f.yaw) * 0.4]; let aim = [me[0] + r.range(-0.35, 0.35), me[1] + 1.1, me[2] + r.range(-0.35, 0.35)];
                    let dmg = s.dmg * r.range(0.8, 1.2) * if f.vet { 1.15 } else { 1.0 };
                    self.shots.push(if heavy { arrow(from, [me[0], me[1] + 1.1, me[2]], 36.0, dmg * 2.0, if s.body == Body::Brigand { "La flèche lourde d'un archer brigand" } else { "La flèche lourde d'un archer squelette" }) }
                        else { arrow(from, aim, 24.0, dmg, if s.body == Body::Brigand { "La flèche d'un archer brigand" } else { "La flèche d'un archer squelette" }) }); self.ev.push(FoeEv::Swing(f.p)); }
            } else
            if c.alive && dist < reach && face > 0.35 && dy.abs() < 1.9 { let dmg = s.dmg * r.range(0.8, 1.25) * if f.vet { 1.15 } else { 1.0 } * if f.air { 1.4 } else { 1.0 }; self.ev.push(FoeEv::Hit { from: f.p, dmg, cause: cause_of(s), push: if s.drag { -4.5 } else { match s.body { Body::Bete => 4.0, Body::Golem => 6.5, Body::Cavalier => 4.5, _ if s.leader => 5.5, _ => 2.5 } } }); if s.web { self.ev.push(FoeEv::Snare(3.5)); } } } }
        // ---- le rampant de magma : il sème des braises derrière lui, et l'eau l'éteint
        if s.ember {
            if f.speed > 0.5 && self.embers.last().is_none_or(|e| hypot2(e.0[0] - f.p[0], e.0[2] - f.p[2]) > 0.9) && self.embers.len() < 80 { self.embers.push(([f.p[0], f.gy, f.p[2]], 5.0)); }
            if w.get(f.p[0].floor() as i32, (f.p[1] + 0.2).floor() as i32, f.p[2].floor() as i32).is_some_and(crate::blocks::is_water) { f.hp -= 9.0 * dt; f.hit = 0.2; if f.hp <= 0.0 { f.dead = 1e-3; f.wind = 0.0; self.ev.push(FoeEv::Die(f.p, f.sp, f.vet, f.camp)); return; } }
        }
        // ---- le mouvement : marcher sur le relief voxel, contourner, monter et descendre d'un bloc, grimper (rampant), bondir (bête)
        if f.state != Combat && f.state != Detection { f.phase += dt * (2.0 + f.speed * 1.8); } else { f.phase += dt * 1.5; }
        if f.air { f.p[0] += f.v[0] * dt; f.p[2] += f.v[2] * dt; f.v[1] -= 22.0 * dt; f.p[1] += f.v[1] * dt;
            let blocked = w.get(f.p[0].floor() as i32, (f.p[1] + 0.5).floor() as i32, f.p[2].floor() as i32).is_none_or(|b| b != 0 && is_solid(b)); if blocked { f.v[0] = 0.0; f.v[2] = 0.0; }
            if let Some(g) = ground(w, f.p[0], f.p[2], f.p[1] + 1.0) { if f.v[1] < 0.0 && f.p[1] <= g { f.p[1] = g; f.gy = g; f.air = false; f.v = [0.0; 3]; if dist < s.reach + 1.0 { f.state = Combat; } } } else if f.v[1] < -20.0 { f.air = false; }
            return; }
        knock(f, dt, w);
        if f.speed > 0.0 { let step = f.speed * dt; if !walk(f, w, step, s.climb, s.body, !f.raid.is_empty()) && matches!(f.state, Patrouille | Retour) { f.yaw += r.range(1.0, 2.5); } }
        settle(f, dt, w);
    }
    /// Les volants. La harpie tourne au-dessus du joueur, plonge serres en avant, frappe et remonte ; le fantôme dérive droit sur lui,
    /// à travers les murs, et se dissipe quand le jour se lève.
    fn fly_think(&mut self, f: &mut Foe, dt: f64, w: &impl Voxels, c: &Ctx) {
        let s = &bestiary().species[f.sp]; let r = &mut self.rng; f.hit = (f.hit - dt).max(0.0); f.phase += dt * if s.ghost { 2.0 } else { 9.0 };
        if f.dead > 0.0 { f.dead += dt; f.speed = 0.0; if !s.ghost { if let Some(g) = ground(w, f.p[0], f.p[2], f.p[1] + 1.0) { f.p[1] = (f.p[1] - 9.0 * dt).max(g); } } return; }
        if s.ghost && c.night < 0.3 && c.cave < 0.5 { f.dead = 1.7; return; } // le jour le dissout, sans rien laisser
        let me = c.feet; let chest = [me[0], me[1] + if s.ghost { 0.25 } else { 1.0 }, me[2]]; let d = [chest[0] - f.p[0], chest[1] - f.p[1], chest[2] - f.p[2]]; let dist = hypot2(d[0], d[2]); let d3 = (dist * dist + d[1] * d[1]).sqrt();
        f.t -= dt; f.cd -= dt;
        let see = c.alive && d3 < s.sight * if c.lit { 1.3 } else { 1.0 } && (s.ghost || sees(w, [f.p[0], f.p[1] + 0.3, f.p[2]], c.eye));
        if see { f.seen = Some(me); f.lost = 0.0; f.alert = 1.0; } else { f.lost += dt; f.alert = (f.alert - dt * 0.1).max(0.0); }
        use Ai::*;
        if f.state != Poursuite && f.state != Combat && (see || f.hit > 0.0) { f.state = Poursuite; self.ev.push(FoeEv::Growl(f.p, s.body)); }
        if matches!(f.state, Poursuite | Combat) && (f.lost > 6.0 || !c.alive) { f.state = Retour; f.air = false; f.wind = 0.0; }
        // où aller : la cible, et l'allure
        let (goal, speed) = match f.state {
            Poursuite | Combat if s.ghost => { if d3 < s.reach && f.wind <= 0.0 && f.cd <= 0.0 { f.wind = s.windup; self.ev.push(FoeEv::Swing(f.p)); } (if f.cd > 0.3 { [f.p[0] - d[0] / d3.max(0.1) * 3.0, chest[1] + 0.6, f.p[2] - d[2] / d3.max(0.1) * 3.0] } else { chest }, if f.wind > 0.0 { s.walk * 0.4 } else { s.run }) }
            Poursuite | Combat => {
                if f.air { // le piqué : droit sur le joueur ; au contact, les serres ; manqué, elle remonte
                    f.wind -= dt; if d3 < s.reach { if c.alive { self.ev.push(FoeEv::Hit { from: f.p, dmg: s.dmg * r.range(0.85, 1.2), cause: cause(s.body), push: 3.5 }); } f.air = false; f.cd = s.cd; f.wind = 0.0; } else if f.wind <= -2.2 { f.air = false; f.cd = s.cd * 0.6; f.wind = 0.0; }
                    (chest, s.run * 1.5)
                } else { let a = f.phase * 0.07 + f.group as f64; let high = [me[0] + sin(a) * 7.0, me[1] + 6.5, me[2] + cos(a) * 7.0];
                    if f.cd <= 0.0 && see && dist < 11.0 { f.air = true; f.wind = 0.0; self.ev.push(FoeEv::Swing(f.p)); } (high, s.run) }
            }
            _ => { if f.t <= 0.0 { f.t = r.range(3.0, 7.0); f.yaw += r.range(-1.5, 1.5); } if f.state == Retour && hypot2(f.p[0] - f.home[0], f.p[2] - f.home[2]) < 3.0 { f.state = Patrouille; }
                let far = hypot2(f.p[0] - f.home[0], f.p[2] - f.home[2]) > 12.0; let dir = if far || f.state == Retour { (f.home[0] - f.p[0]).atan2(f.home[2] - f.p[2]) } else { f.yaw };
                ([f.p[0] + sin(dir) * 4.0, f.home[1] + 0.4 * sin(f.phase * 0.2), f.p[2] + cos(dir) * 4.0], s.walk) }
        };
        // le coup du fantôme tombe à la fin de son geste, s'il est encore au contact
        if s.ghost && f.wind > 0.0 { f.wind -= dt; if f.wind <= 0.0 { f.wind = 0.0; f.cd = s.cd; if c.alive && d3 < s.reach + 0.6 { self.ev.push(FoeEv::Hit { from: f.p, dmg: s.dmg * r.range(0.85, 1.2), cause: "Le toucher glacé d'un fantôme", push: 1.0 }); } } }
        let g = [goal[0] - f.p[0], goal[1] - f.p[1], goal[2] - f.p[2]]; let gl = (g[0] * g[0] + g[1] * g[1] + g[2] * g[2]).sqrt();
        if gl > 0.05 { f.yaw = turn(f.yaw, if matches!(f.state, Poursuite | Combat) && (s.ghost || f.air) { d[0].atan2(d[2]) } else { g[0].atan2(g[2]) }, dt * 5.0);
            let st = (speed * dt).min(gl); let mut n = [f.p[0] + g[0] / gl * st + f.v[0] * dt, f.p[1] + g[1] / gl * st, f.p[2] + g[2] / gl * st + f.v[2] * dt];
            let solid = |q: [f64; 3]| w.get(q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32).is_none_or(|b| b != 0 && is_solid(b));
            if !s.ghost && (solid(n) || solid([n[0], n[1] - 0.4, n[2]])) { n = [f.p[0], f.p[1] + 4.0 * dt, f.p[2]]; if solid(n) { n = f.p; f.yaw += 2.0 * dt; } if f.air { f.air = false; f.cd = s.cd * 0.5; } } // la roche : elle remonte
            // la lumière repousse le fantôme : il n'entre pas dans une pièce éclairée (lanterne, torche posée) et recule devant qui porte une flamme
            if s.ghost && ((c.light)(n[0].floor() as i32, n[1].floor() as i32, n[2].floor() as i32) >= 9 || (c.lit && d3 < 3.5)) { n = [f.p[0] - d[0] / d3.max(0.1) * st, f.p[1], f.p[2] - d[2] / d3.max(0.1) * st]; f.wind = 0.0; }
            f.p = n; f.speed = speed; }
        let k = (1.0 - dt * 5.0).max(0.0); f.v[0] *= k; f.v[2] *= k; f.gy = f.p[1];
    }
    /// Le ver des sables : il chemine sous la dune vers ce qui marche dessus, jaillit sous les pieds, reste un instant dressé
    /// (c'est là qu'on peut le frapper), puis replonge. Il ne quitte jamais le sable.
    fn worm_think(&mut self, f: &mut Foe, dt: f64, w: &impl Voxels, c: &Ctx) {
        let s = &bestiary().species[f.sp]; let r = &mut self.rng; f.hit = (f.hit - dt).max(0.0); f.phase += dt * 3.0;
        if f.dead > 0.0 { f.dead += dt; f.speed = 0.0; return; }
        let me = c.feet; let dist = hypot2(f.p[0] - me[0], f.p[2] - me[2]); let dy = me[1] - f.p[1]; f.t -= dt; f.cd -= dt;
        if f.wind > 0.0 { f.speed = 0.0; f.wind -= dt; if f.wind <= 0.0 { f.wind = 0.0; f.up = 1.8; f.cd = s.cd; if c.alive && dist < s.reach && dy.abs() < 2.5 { self.ev.push(FoeEv::Hit { from: [f.p[0], f.p[1] - 1.0, f.p[2]], dmg: s.dmg * r.range(0.85, 1.25), cause: cause(s.body), push: 3.0 }); } } return; }
        if f.up > 0.0 { f.up -= dt; f.speed = 0.0; f.yaw = turn(f.yaw, (me[0] - f.p[0]).atan2(me[2] - f.p[2]), dt * 3.0); return; }
        // il sent les pas : pas besoin de voir, il suffit qu'on marche (accroupi, de bien plus près)
        let feel = c.alive && dist < s.hear * if c.crouch { 0.35 } else { 1.0 } && dy.abs() < 6.0;
        if feel { f.seen = Some(me); f.lost = 0.0; if f.state != Ai::Poursuite { f.state = Ai::Poursuite; self.ev.push(FoeEv::Growl(f.p, s.body)); } } else { f.lost += dt; if f.lost > 5.0 && f.state == Ai::Poursuite { f.state = Ai::Retour; } }
        let (want, speed) = match f.state { Ai::Poursuite => { let q = f.seen.unwrap_or(me); ((q[0] - f.p[0]).atan2(q[2] - f.p[2]), s.run) }
            Ai::Retour => { if hypot2(f.p[0] - f.home[0], f.p[2] - f.home[2]) < 2.0 { f.state = Ai::Repos; } ((f.home[0] - f.p[0]).atan2(f.home[2] - f.p[2]), s.walk) }
            _ => { if f.t <= 0.0 { f.t = r.range(3.0, 8.0); f.yaw += r.range(-1.6, 1.6); } (f.yaw, s.walk * 0.6) } };
        f.yaw = turn(f.yaw, want, dt * 4.0); f.speed = speed;
        if f.state == Ai::Poursuite && dist < 1.1 && f.cd <= 0.0 && dy.abs() < 2.0 { f.wind = s.windup; f.speed = 0.0; self.ev.push(FoeEv::Swing(f.p)); return; }
        // sous le sable seulement : au bord de la roche, il longe ou s'arrête
        for k in [0.0, 0.5, -0.5, 1.0, -1.0, 1.5, -1.5] { let yaw = f.yaw + k; let (nx, nz) = (f.p[0] + sin(yaw) * speed * dt, f.p[2] + cos(yaw) * speed * dt);
            if let Some(y) = ground(w, nx, nz, f.gy + 1.0) { if (y - f.gy).abs() <= 1.05 && sandy(w, nx, y, nz) { f.p = [nx, y, nz]; f.gy = y; if k != 0.0 { f.yaw += k * 0.4; } return; } } }
        f.speed = 0.0;
    }
    /// Le combat de boss (le chef de guerre). Quatre attaques, chacune annoncée par un geste qu'on apprend à lire :
    /// la **taille** (balayage à hauteur d'homme) se pare au bouclier, au bon moment ; la **frappe écrasante** (la hache levée
    /// très haut, le corps qui rougeoie) brise toute garde : il faut rouler ; l'**enchaînement** de trois coups au rythme
    /// trompeur (vite, puis retenu) punit qui pare trop tôt ; la **ruée** d'épaule (de loin, le corps qui rougeoie) se fuit
    /// d'une roulade de côté. Après une frappe ou une ruée manquées, il reste un instant à découvert. Taper entame surtout sa
    /// garde ; chaque parade l'entame bien davantage ; brisée, il chancelle trois secondes et chaque coup porte. À mi-vie, il
    /// enrage : il jette son heaume, appelle ses hommes, frappe plus vite. Rend faux s'il quitte le combat.
    fn boss_think(rng: &mut Rng, ev: &mut Vec<FoeEv>, f: &mut Foe, dt: f64, c: &Ctx, see: bool, dist: f64, dy: f64, to: [f64; 2]) -> bool {
        let s = &bestiary().species[f.sp]; let me = c.feet;
        if !c.alive { f.state = Ai::Retour; return false; }
        if !see && f.lost > 6.0 && f.wind <= 0.0 && f.lunge <= 0.0 { f.state = Ai::Recherche; f.t = 8.0; return false; }
        // sa garde se reprend d'elle-même, si on lui laisse le temps
        f.calm += dt; if f.calm > 2.0 && f.broken <= 0.0 { f.posture = (f.posture - POSTURE_REGEN * dt).max(0.0); }
        if f.broken > 0.0 { f.broken -= dt; f.speed = 0.0; f.wind = 0.0; f.state = Ai::Combat; if f.broken <= 0.0 { f.cd = 0.4; } return true; }
        let pace = if f.enraged { 0.7 } else { 1.0 };
        // seconde phase
        if !f.enraged && f.hp < f.max * 0.5 { f.enraged = true; f.wind = 0.0; f.mv = 0; f.combo = 0; f.lunge = 0.0; f.cd = 1.3; f.speed = 0.0; ev.push(FoeEv::Rage(f.p, boss_calls(s))); return true; }
        let face = sin(f.yaw) * to[0] + cos(f.yaw) * to[1];
        // la ruée : tout droit, cap tenu
        if f.lunge > 0.0 { f.lunge -= dt; f.speed = 11.0; f.state = Ai::Poursuite; f.yaw = turn(f.yaw, (me[0] - f.p[0]).atan2(me[2] - f.p[2]), dt * 0.6);
            if dist < 1.7 && dy.abs() < 2.0 && face > 0.2 { ev.push(FoeEv::Crush { from: f.p, dmg: s.dmg * 1.3 * rng.range(0.9, 1.15), cause: boss_cause(s, MV_CHARGE), push: 8.0 }); f.lunge = 0.0; f.mv = 0; f.cd = 1.2 * pace; f.speed = 0.0; }
            else if f.lunge <= 0.0 { f.mv = 0; f.cd = OPENING * pace; f.speed = 0.0; } // manquée : il reprend son souffle
            return true; }
        // une attaque annoncée : il la prépare, puis elle tombe
        if f.wind > 0.0 { f.speed = 0.0; f.state = Ai::Combat;
            let track = match f.mv { MV_ECRASE => 1.2, MV_CHARGE => 3.0, _ => 2.5 }; f.yaw = turn(f.yaw, (me[0] - f.p[0]).atan2(me[2] - f.p[2]), dt * track);
            f.wind -= dt; if f.wind > 0.0 { return true; } f.wind = 0.0;
            let near = |extra: f64| dist < s.reach + extra && face > 0.3 && dy.abs() < 1.9;
            let roll = rng.range(0.85, 1.2);
            match f.mv {
                MV_TAILLE => { if near(0.5) { ev.push(FoeEv::Hit { from: f.p, dmg: s.dmg * roll, cause: boss_cause(s, MV_TAILLE), push: 4.0 }); } f.mv = 0; f.cd = 1.2 * pace; }
                MV_COMBO => { if near(0.4) { ev.push(FoeEv::Hit { from: f.p, dmg: s.dmg * 0.7 * roll, cause: boss_cause(s, MV_COMBO), push: 2.0 }); }
                    f.combo = f.combo.saturating_sub(1); ev.push(FoeEv::Swing(f.p));
                    if f.combo > 0 { f.wind = COMBO_BEATS[(3 - f.combo as usize).min(2)] * pace; f.v = [sin(f.yaw) * 3.0, 0.0, cos(f.yaw) * 3.0]; } else { f.mv = 0; f.cd = 1.5 * pace; } }
                MV_ECRASE => { let hit = near(0.8); if hit { ev.push(FoeEv::Crush { from: f.p, dmg: s.dmg * 1.8 * roll, cause: boss_cause(s, MV_ECRASE), push: 7.0 }); }
                    f.mv = 0; f.cd = if hit { 1.3 } else { OPENING } * pace; } // manquée : la hache plantée en terre, il est à découvert
                MV_CHARGE => { f.lunge = 0.75; }
                _ => { f.mv = 0; }
            }
            return true; }
        // il choisit : de loin, il approche ou se rue ; de près, il taille, enchaîne ou écrase
        f.yaw = turn(f.yaw, (me[0] - f.p[0]).atan2(me[2] - f.p[2]), dt * 5.0);
        if dist > s.reach + 0.4 { f.state = Ai::Poursuite; f.speed = s.run * if f.enraged { 1.15 } else { 1.0 };
            if f.cd <= 0.0 && see && (4.5..12.0).contains(&dist) && dy.abs() < 1.5 && rng.next() < dt * 1.6 { f.mv = MV_CHARGE; f.wind = WIND_CHARGE * pace.max(0.85); f.speed = 0.0; ev.push(FoeEv::Growl(f.p, s.body)); }
            return true; }
        f.state = Ai::Combat; f.speed = 0.0;
        if f.cd <= 0.0 { let r = rng.next();
            let (mv, wind) = if r < 0.4 { (MV_TAILLE, WIND_TAILLE) } else if r < 0.72 { (MV_COMBO, WIND_COMBO) } else { (MV_ECRASE, WIND_ECRASE) };
            f.mv = mv; f.wind = wind * pace; if mv == MV_COMBO { f.combo = 3; } ev.push(FoeEv::Swing(f.p)); }
        true
    }
    /// Le cavalier d'élite. Il tourne autour de sa proie au trot, puis attaque : de loin, la **charge** lance couchée (le
    /// cavalier rougeoie : aucun bouclier ne l'arrête, on l'évite d'un pas ou d'une roulade de côté ; manquée, il file trop loin
    /// et doit faire demi-tour au pas — l'ouverture) ; à mi-distance, le **coup de lance au passage**, qui se pare ; à qui se
    /// colle au cheval, le **cabrage** : les sabots retombent (rougeoie aussi : on s'éloigne d'une roulade). Chaque parade
    /// entame sa garde ; brisée, il vide les étriers et finit à pied, sonné. Rend faux s'il quitte le combat.
    fn rider_think(rng: &mut Rng, ev: &mut Vec<FoeEv>, f: &mut Foe, dt: f64, c: &Ctx, see: bool, dist: f64, dy: f64, to: [f64; 2]) -> bool {
        let s = &bestiary().species[f.sp]; let me = c.feet;
        if !c.alive { f.state = Ai::Retour; return false; }
        if !see && f.lost > 8.0 && f.wind <= 0.0 && f.lunge <= 0.0 { f.state = Ai::Recherche; f.t = 8.0; return false; }
        f.calm += dt; if f.calm > 2.0 { f.posture = (f.posture - POSTURE_REGEN * dt).max(0.0); }
        f.state = Ai::Poursuite; let face = sin(f.yaw) * to[0] + cos(f.yaw) * to[1]; let toward = (me[0] - f.p[0]).atan2(me[2] - f.p[2]);
        // la charge lancée : cap presque tenu ; elle porte au contact, puis il file au-delà avant de tourner bride
        if f.lunge > 0.0 { f.lunge -= dt; f.speed = 13.0; if f.combo == 0 { f.yaw = turn(f.yaw, toward, dt * 0.5); }
            if f.combo < 2 && dist < 1.6 && dy.abs() < 2.2 && face > 0.0 { ev.push(FoeEv::Crush { from: f.p, dmg: s.dmg * 1.7 * rng.range(0.9, 1.15), cause: "La charge d'un cavalier brigand", push: 8.0 }); f.combo = 2; }
            if f.combo == 0 && dist < 2.6 { f.combo = 1; } // il est passé tout près : manquée si rien n'a porté
            if (f.combo > 0 && dist > 5.0) || f.lunge <= 0.0 { let missed = f.combo != 2; f.lunge = 0.0; f.mv = 0; f.combo = 0; f.cd = 1.2; if missed { f.burst = OPENING; } }
            return true; }
        // une attaque annoncée
        if f.wind > 0.0 { f.wind -= dt;
            match f.mv { MV_PASSE => { f.speed = 6.5; f.yaw = turn(f.yaw, toward, dt * 3.0); } MV_CABRE => { f.speed = 0.0; } _ => { f.speed = 0.0; f.yaw = turn(f.yaw, toward, dt * 4.0); } }
            if f.wind > 0.0 { return true; } f.wind = 0.0; let roll = rng.range(0.85, 1.2);
            match f.mv {
                MV_LANCE => { f.lunge = 2.4; f.combo = 0; }
                MV_PASSE => { if dist < s.reach + 0.9 && dy.abs() < 2.2 && face > -0.2 { ev.push(FoeEv::Hit { from: f.p, dmg: s.dmg * roll, cause: "La lance d'un cavalier brigand", push: 3.5 }); } ev.push(FoeEv::Swing(f.p)); f.mv = 0; f.cd = 1.3; if rng.next() < 0.6 { f.combo = 3; } } // puis, souvent, il prend du champ pour charger
                MV_CABRE => { if dist < 2.8 && dy.abs() < 2.2 { ev.push(FoeEv::Crush { from: f.p, dmg: s.dmg * 1.4 * roll, cause: "Les sabots du cheval d'un cavalier brigand", push: 6.5 }); } f.mv = 0; f.cd = 1.5; }
                _ => { f.mv = 0; }
            }
            return true; }
        // après une charge manquée, il a filé trop loin : il tourne bride au pas (c'est le moment de le frapper)
        if f.burst > 0.0 { f.speed = 2.2; f.yaw = turn(f.yaw, toward, dt * 1.2); return true; }
        // il prend du champ : au galop, droit devant lui, loin de vous, avant de faire volte-face pour charger
        if f.combo == 3 { if dist > 13.0 { f.combo = 0; } else { f.speed = s.run; f.yaw = turn(f.yaw, toward + std::f64::consts::PI, dt * 2.0); return true; } }
        // il choisit son attaque
        if f.cd <= 0.0 && see && dy.abs() < 2.0 {
            if dist < 3.0 { f.mv = MV_CABRE; f.wind = WIND_CABRE; f.speed = 0.0; ev.push(FoeEv::Growl(f.p, s.body)); return true; }
            if (3.5..9.0).contains(&dist) && rng.next() < dt * 1.2 { f.mv = MV_PASSE; f.wind = WIND_PASSE + 0.25; return true; } // il fond sur vous au trot, lance levée
            if (10.0..24.0).contains(&dist) && rng.next() < dt * 1.5 { f.mv = MV_LANCE; f.wind = WIND_LANCE; f.speed = 0.0; ev.push(FoeEv::Growl(f.p, s.body)); return true; }
        }
        // sinon, il tourne autour au trot, à sept pas, s'éloigne pour prendre son élan, ou vient au galop s'il est loin
        if dist > 16.0 { f.speed = s.run; f.yaw = turn(f.yaw, toward, dt * 3.0); }
        else { let side = if f.group % 2 == 0 { 1.0 } else { -1.0 }; let a = (f.p[2] - me[2]).atan2(f.p[0] - me[0]) + side * 0.7; let goal = [me[0] + a.cos() * 7.0, me[2] + a.sin() * 7.0];
            f.speed = 6.5; f.yaw = turn(f.yaw, (goal[0] - f.p[0]).atan2(goal[1] - f.p[2]), dt * 2.5); }
        true
    }
    /// Le capitaine squelette, au sommet de sa tour. Il garde ses distances et tourne autour de sa proie ; il tire des **salves**
    /// de trois flèches coup sur coup (le bouclier les arrête), et le **tir chargé** : la corde longuement tendue, il rougeoie, la
    /// flèche lourde traverse tout bouclier — il faut rouler ou se mettre à couvert. Qui le serre de près reçoit un **coup de pied**
    /// (il se pare : chaque parade entame sa garde) avant qu'il ne rompe. Garde brisée, il chancelle. À mi-vie, il siffle ses archers.
    fn captain_think(rng: &mut Rng, ev: &mut Vec<FoeEv>, shots: &mut Vec<([f64; 3], [f64; 3], f64, f64, &'static str)>, f: &mut Foe, dt: f64, c: &Ctx, see: bool, dist: f64, dy: f64, to: [f64; 2]) -> bool {
        let s = &bestiary().species[f.sp]; let me = c.feet;
        if !c.alive { f.state = Ai::Retour; return false; }
        if !see && f.lost > 6.0 && f.wind <= 0.0 { f.state = Ai::Recherche; f.t = 8.0; return false; }
        f.calm += dt; if f.calm > 2.0 && f.broken <= 0.0 { f.posture = (f.posture - POSTURE_REGEN * dt).max(0.0); }
        if f.broken > 0.0 { f.broken -= dt; f.speed = 0.0; f.wind = 0.0; f.state = Ai::Combat; if f.broken <= 0.0 { f.cd = 0.4; } return true; }
        if !f.enraged && f.hp < f.max * 0.5 { f.enraged = true; f.wind = 0.0; f.mv = 0; f.combo = 0; f.cd = 1.0; ev.push(FoeEv::Rage(f.p, boss_calls(s))); return true; }
        let pace = if f.enraged { 0.7 } else { 1.0 }; let toward = (me[0] - f.p[0]).atan2(me[2] - f.p[2]); let face = sin(f.yaw) * to[0] + cos(f.yaw) * to[1];
        let from = [f.p[0] + sin(f.yaw) * 0.4, f.p[1] + 1.4 * s.size, f.p[2] + cos(f.yaw) * 0.4];
        if f.wind > 0.0 { f.speed = 0.0; f.state = Ai::Combat; f.yaw = turn(f.yaw, toward, dt * if f.mv == MV_TIR { 1.5 } else { 4.0 }); f.wind -= dt; if f.wind > 0.0 { return true; } f.wind = 0.0;
            match f.mv {
                MV_TIR => { if see { shots.push(arrow(from, [me[0], me[1] + 1.1, me[2]], 38.0, s.dmg * 2.2, "La flèche lourde d'un capitaine squelette")); } ev.push(FoeEv::Swing(f.p)); f.mv = 0; f.cd = 1.6 * pace; }
                MV_SALVE => { if f.combo == 0 { f.combo = 3; } if see { shots.push(arrow(from, [me[0] + rng.range(-0.4, 0.4), me[1] + 1.1, me[2] + rng.range(-0.4, 0.4)], 26.0, s.dmg * 0.8, "La flèche d'un capitaine squelette")); } ev.push(FoeEv::Swing(f.p));
                    f.combo = f.combo.saturating_sub(1); if f.combo > 0 { f.wind = 0.28 * pace; } else { f.mv = 0; f.cd = 1.4 * pace; } }
                MV_COUP => { if dist < s.reach.min(2.2) + 0.5 && face > 0.3 && dy.abs() < 1.9 { ev.push(FoeEv::Hit { from: f.p, dmg: s.dmg * 0.8, cause: "Le coup de pied d'un capitaine squelette", push: 7.5 }); } f.mv = 0; f.cd = 0.6; f.combo = 4; } // puis il rompt
                _ => { f.mv = 0; }
            }
            return true; }
        // il rompt : quelques pas en arrière, de biais, pour reprendre ses distances
        if f.combo == 4 { if dist > 6.0 || f.cd < -1.5 { f.combo = 0; } else { f.state = Ai::Poursuite; f.speed = s.run; f.yaw = turn(f.yaw, toward + std::f64::consts::PI + 0.5, dt * 10.0); return true; } }
        f.yaw = turn(f.yaw, toward, dt * 5.0);
        if dist < 2.6 && f.cd <= 0.0 { f.mv = MV_COUP; f.wind = WIND_COUP * pace; f.state = Ai::Combat; f.speed = 0.0; ev.push(FoeEv::Swing(f.p)); return true; }
        if f.cd <= 0.0 && see && dist < 20.0 { let r = rng.next(); f.state = Ai::Combat; f.speed = 0.0;
            if r < 0.4 { f.mv = MV_TIR; f.wind = WIND_TIR * pace.max(0.8); } else { f.mv = MV_SALVE; f.wind = WIND_SALVE * pace; f.combo = 3; } ev.push(FoeEv::Swing(f.p)); return true; }
        // il garde ses distances : il s'approche s'il est loin, s'écarte si on le serre, sinon tourne lentement
        f.state = Ai::Poursuite;
        if dist > 14.0 || !see { f.speed = s.run; } else if dist < 5.0 { f.speed = s.walk * 1.6; f.yaw = turn(f.yaw, toward + std::f64::consts::PI, dt * 6.0); }
        else { f.speed = s.walk; let side = if f.group % 2 == 0 { 1.4 } else { -1.4 }; f.yaw = turn(f.yaw, toward + side, dt * 4.0); }
        true
    }
    pub fn debug_lines(&self) -> Vec<String> {
        self.foes.iter().take(8).map(|f| { let s = &bestiary().species[f.sp]; format!("  {}{} — {} ({:.0}/{:.0} pv, éveil {:.1})", s.name, if f.vet { " (vétéran)" } else { "" }, if f.dead > 0.0 { "morte" } else { f.state.name() }, f.hp.max(0.0), f.max, f.alert) }).collect()
    }
}
fn cause(b: Body) -> &'static str { match b { Body::Rodeur => "Les griffes d'un rôdeur décharné", Body::Brigand => "L'arme d'un brigand", Body::Humain => "Le trident d'un noyé", Body::Ver => "La gueule d'un ver des sables", Body::Harpie => "Les serres d'une harpie", Body::Araignee => "Les crochets d'une araignée des cavernes", Body::Golem => "Le poing d'un golem de cristal", Body::Bete => "Les crocs d'une bête rôdeuse", Body::Rampant => "Les mandibules d'un rampant nocturne", Body::Cavalier => "La lance d'un cavalier brigand" } }
/// Ce qui a frappé, selon l'espèce (les brigands des camps ont chacun leur arme).
fn cause_of(s: &Species) -> &'static str { match s.skin { "epeiste" => "L'épée d'un épéiste brigand", "chef" => "La grande hache d'un chef de guerre brigand", _ if s.ember => "Les mandibules brûlantes d'un rampant de magma", _ => cause(s.body) } }
/// Le brigand porte-t-il un bouclier : l'épéiste toujours, le brigand errant deux fois sur trois ; ni l'archer, ni le chef (sa hache se tient à deux mains).
// ---------------------------------------------------------------------------------------------------- combat de boss
/// Les attaques du boss (`Foe::mv`) : taille (à parer), enchaînement de trois coups, frappe écrasante et ruée (à esquiver).
pub const MV_TAILLE: u8 = 1; pub const MV_COMBO: u8 = 3; pub const MV_ECRASE: u8 = 2; pub const MV_CHARGE: u8 = 4;
/// Le temps de chaque annonce (s) : la frappe écrasante est longue et lisible ; l'enchaînement part vite.
pub const WIND_TAILLE: f64 = 0.75; pub const WIND_COMBO: f64 = 0.6; pub const WIND_ECRASE: f64 = 1.15; pub const WIND_CHARGE: f64 = 0.8;
/// Le rythme de l'enchaînement : le deuxième coup suit vite, le troisième se fait attendre.
pub const COMBO_BEATS: [f64; 3] = [0.3, 0.85, 0.6];
/// Sa garde : ce qu'elle encaisse avant de se briser, ce qu'une parade lui retire, ce qu'elle reprend par seconde au repos.
pub const POSTURE: f64 = 100.0; pub const PARRY_POSTURE: f64 = 34.0; pub const POSTURE_REGEN: f64 = 9.0;
/// Les dégâts sur sa vie : à travers sa garde, et quand elle est brisée.
pub const GUARDED_MUL: f64 = 0.35; pub const BROKEN_MUL: f64 = 2.5;
/// Le répit après une frappe ou une ruée manquée (s), et le temps qu'il chancelle, garde brisée.
pub const OPENING: f64 = 2.2; pub const REEL_T: f64 = 3.0;
/// Cette attaque se pare-t-elle (taille, enchaînement) ou faut-il l'esquiver (frappe écrasante, ruée) ?
/// Les attaques du cavalier : la charge lance couchée (à esquiver de côté), le coup de lance au passage (à parer), le cabrage
/// (les sabots qui retombent sur qui s'est collé au cheval : à fuir d'une roulade).
pub const MV_LANCE: u8 = 5; pub const MV_PASSE: u8 = 6; pub const MV_CABRE: u8 = 7;
pub const WIND_LANCE: f64 = 0.9; pub const WIND_PASSE: f64 = 0.45; pub const WIND_CABRE: f64 = 1.0;
/// À cheval, il encaisse moins (on l'atteint mal) ; sa garde brisée, il est désarçonné.
pub const MOUNTED_MUL: f64 = 0.5;
/// Le tir chargé (l'archer vétéran, le capitaine) : la corde longuement tendue, l'archer rougeoie, la flèche lourde traverse le
/// bouclier ; la salve (le capitaine) : trois flèches coup sur coup, que le bouclier arrête ; le coup de pied (le capitaine, à qui
/// le serre de trop près) : il se pare. La feinte (l'épéiste vétéran) : un geste à moitié, puis le vrai coup, vite.
pub const MV_TIR: u8 = 8; pub const MV_COUP: u8 = 9; pub const MV_SALVE: u8 = 10; pub const MV_FEINTE: u8 = 11;
pub const WIND_TIR: f64 = 1.5; pub const WIND_COUP: f64 = 0.5; pub const WIND_SALVE: f64 = 0.7;
pub fn unblockable(mv: u8) -> bool { matches!(mv, MV_ECRASE | MV_CHARGE | MV_LANCE | MV_CABRE | MV_TIR) }
/// Une flèche tirée de `from` vers `aim`, à `sp` pas par seconde (un peu au-dessus, pour la chute).
fn arrow(from: [f64; 3], aim: [f64; 3], sp: f64, dmg: f64, cause: &'static str) -> ([f64; 3], [f64; 3], f64, f64, &'static str) {
    let d = [aim[0] - from[0], aim[1] - from[1], aim[2] - from[2]]; let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(0.1); let t = l / sp; (from, [d[0] / t, d[1] / t + 0.5 * 12.0 * t, d[2] / t], 0.0, dmg, cause)
}
/// Les flèches lourdes traversent le bouclier.
const HEAVY_ARROW: &str = "La flèche lourde";
/// Ce que dit le boss qui frappe : le chef de guerre ou le gardien de la crypte.
fn boss_cause(s: &Species, mv: u8) -> &'static str { match (s.id, mv) {
    ("roi_noye", MV_ECRASE) => "Le trident du Roi noyé, abattu de tout son poids", ("roi_noye", MV_CHARGE) => "La ruée du Roi noyé", ("roi_noye", _) => "Le trident du Roi noyé",
    ("colosse", MV_ECRASE) => "Le trident du colosse englouti, abattu de tout son poids", ("colosse", MV_CHARGE) => "La ruée du colosse englouti", ("colosse", _) => "Le trident du colosse englouti",
    ("gardien_soleil", MV_ECRASE) => "Le poing brûlant du Gardien du Soleil", ("gardien_soleil", MV_CHARGE) => "La ruée du Gardien du Soleil", ("gardien_soleil", _) => "Les griffes du Gardien du Soleil",
    ("reine_glaces", MV_ECRASE) => "La lance de givre de la Reine des glaces, abattue de haut", ("reine_glaces", MV_CHARGE) => "La ruée glacée de la Reine des glaces", ("reine_glaces", _) => "La lance de givre de la Reine des glaces",
    ("maitre_forge", MV_ECRASE) => "Le marteau du Maître de forge, abattu comme sur l'enclume", ("maitre_forge", MV_CHARGE) => "La ruée du Maître de forge", ("maitre_forge", _) => "Le marteau du Maître de forge",
    ("premier_roi", MV_ECRASE) => "L'épée longue du Premier Roi, abattue à deux mains", ("premier_roi", MV_CHARGE) => "La ruée du Premier Roi", ("premier_roi", _) => "L'épée longue du Premier Roi",
    ("gardien", MV_ECRASE) => "Le poing du gardien de la crypte", ("gardien", MV_CHARGE) => "La ruée du gardien de la crypte", ("gardien", _) => "Les griffes du gardien de la crypte",
    (_, MV_TAILLE) => "La taille d'un chef de guerre brigand", (_, MV_ECRASE) => "La frappe écrasante d'un chef de guerre brigand", (_, MV_CHARGE) => "La ruée d'un chef de guerre brigand", _ => "La grande hache d'un chef de guerre brigand" } }
/// Ceux qu'appelle le boss à mi-vie.
fn boss_calls(s: &Species) -> &'static str { match s.id { "gardien" | "gardien_soleil" => "rodeur", "reine_glaces" => "fantome", "capitaine" | "premier_roi" => "archer", "colosse" | "roi_noye" => "noye", _ => "brigand" } }
fn reel(f: &mut Foe, ev: &mut Vec<FoeEv>) { f.broken = REEL_T; f.posture = 0.0; f.wind = 0.0; f.mv = 0; f.combo = 0; f.lunge = 0.0; f.speed = 0.0;
    // le cavalier, garde brisée, vide les étriers : il tombe, sonné, et finit à pied ; son cheval s'enfuit
    if bestiary().species[f.sp].body == Body::Cavalier { if let Some(k) = species("brigand") { let part = f.hp / f.max; let s = &bestiary().species[k]; f.sp = k; f.max = s.hp * if f.vet { 1.25 } else { 1.0 }; f.hp = (f.max * part).max(1.0); f.state = Ai::Combat; f.cd = REEL_T + 0.5; ev.push(FoeEv::Unhorse(f.p, f.yaw)); return; } }
    ev.push(FoeEv::Reel(f.p)); }
fn shielded(s: &Species, variant: u8) -> bool { s.body == Body::Brigand && match s.skin { "epeiste" => true, "" => variant % 3 != 2, _ => false } }
fn turn(a: f64, b: f64, k: f64) -> f64 { let d = (b - a + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI; a + d.clamp(-k, k) }
/// Du sable (ou du gravier) sous ce point : le terrain du ver.
fn sandy(w: &impl Voxels, x: f64, y: f64, z: f64) -> bool { w.get(x.floor() as i32, y.floor() as i32 - 1, z.floor() as i32).is_some_and(|b| b == B::SAND || b == B::GRAVEL) }
fn knock(f: &mut Foe, dt: f64, w: &impl Voxels) {
    if f.v[0] == 0.0 && f.v[2] == 0.0 { return; } let (nx, nz) = (f.p[0] + f.v[0] * dt, f.p[2] + f.v[2] * dt);
    if let Some(y) = ground(w, nx, nz, f.gy) { if (y - f.gy).abs() <= 1.0 { f.p[0] = nx; f.p[2] = nz; f.gy = y; } }
    let k = (1.0 - dt * 7.0).max(0.0); f.v[0] *= k; f.v[2] *= k; if f.v[0].abs() + f.v[2].abs() < 0.05 { f.v[0] = 0.0; f.v[2] = 0.0; }
}
/// Se poser sur le sol sous ses pieds — jamais d'un bond sur ce qui est plus haut qu'une marche (une herse qu'on vient de baisser, un rebord).
fn settle(f: &mut Foe, dt: f64, w: &impl Voxels) { if let Some(g) = ground(w, f.p[0], f.p[2], f.p[1] + 1.0) { if !f.climbing && g <= f.gy + 1.05 { f.gy = g; } } f.p[1] += (f.gy - f.p[1]) * (dt * 10.0).min(1.0); }
/// Avancer : tout droit, sinon le cap libre le plus proche ; une marche d'un bloc se franchit ; le rampant grimpe un mur de quatre blocs.
/// Le corps tient-il là, dans l'eau s'il le faut (la bande qui traverse un ruisseau) : rien de plein sur sa hauteur.
fn wet_fits(w: &impl Voxels, x: f64, y: f64, z: f64, h: i32) -> bool { let (bx, by, bz) = (x.floor() as i32, (y + 0.05).floor() as i32, z.floor() as i32); (0..h).all(|k| w.get(bx, by + k, bz).is_some_and(|b| !is_solid(b))) && w.get(bx, by - 1, bz).is_some_and(is_solid) }
/// Le fond d'un ruisseau (quatre blocs d'eau au plus : on le passe à la nage), s'il y en a un.
fn ford(w: &impl Voxels, x: f64, z: f64, y0: f64) -> Option<f64> {
    let (fx, fz) = (x.floor() as i32, z.floor() as i32); let mut y = y0.floor() as i32 + 2; let mut wet = 0;
    while y > y0 as i32 - 8 { let b = w.get(fx, y, fz)?; if crate::blocks::is_water(b) { wet += 1; if wet > 4 { return None; } } else if is_solid(b) { return Some((y + 1) as f64); } y -= 1; } None
}
fn walk(f: &mut Foe, w: &impl Voxels, step: f64, climb: bool, body: Body, wade: bool) -> bool {
    let tall = match body { Body::Rampant | Body::Bete | Body::Araignee => 1, _ => 2 };
    for k in [0.0, 0.45, -0.45, 0.9, -0.9, 1.35, -1.35, 1.8, -1.8] {
        let yaw = f.yaw + k; let (nx, nz) = (f.p[0] + sin(yaw) * step, f.p[2] + cos(yaw) * step);
        // une bande en marche passe les ruisseaux : elle prend pied au fond, ou nage
        // (sous une voûte basse — le passage d'une porte —, la bande cherche son sol juste sous elle, pas au plafond)
        if let Some(y) = ground(w, nx, nz, f.gy + 1.0).filter(|y| !wade || y - f.gy <= 1.05).or_else(|| if wade { ground(w, nx, nz, f.gy).or_else(|| ford(w, nx, nz, f.gy + 1.0)) } else { None }) { if y - f.gy <= 1.05 && f.gy - y <= 3.0 && fits(w, nx, y.max(f.gy - 1.0), nz, yaw, tall) { f.p[0] = nx; f.p[2] = nz; f.gy = y; f.climbing = false; if k != 0.0 { f.yaw += k * 0.5; } return true; } }
        let lim = 4.5;
        if climb && k == 0.0 { if let Some(y) = ground(w, nx, nz, f.gy + lim) { if y - f.gy > 1.0 && y - f.gy <= lim && fits(w, nx, y, nz, yaw, tall) {
            // Le rampant monte le long du mur, puis passe dessus.
            f.climbing = true; f.p[1] += 2.6 * step / f.speed.max(0.1) * f.speed.min(3.0); if f.p[1] >= y { f.p = [nx, y, nz]; f.gy = y; f.climbing = false; } return true; } } }
    }
    false
}


// ------------------------------------------------------------------------------------------- boîtes « en pixels »
/// Chaque face d'une pièce est découpée en petits pixels (environ 3 cm) dont la teinte suit la matière : trame et
/// effilochures de l'étoffe, grain et coutures du cuir, reflets, rayures et rouille du fer, marbrures de la chair, fissures
/// de l'os, poil de la bête, reflets de la chitine. Les très petites pièces (yeux, dents, rivets) restent unies.
#[derive(Clone, Copy, PartialEq)] pub(crate) enum Mat { /** écorce : sillons verticaux profonds, crêtes claires, crevasses, lichen */ Bark, Plain, Grain, Cloth, Leather, Metal, Flesh, Bone, Fur, Chitin, /** peau lisse de cétacé */ Skin, /** poil ras et luisant (cheval) */ Coat, /** peau de requin, grenue */ Shark, /** peau de baleine semée de bernacles */ Barnacle, /** plumes en rangs d'écailles */ Feather, /** écailles en losanges */ Scale, /** cheveux, crins : des mèches */ Hair, /** peau humaine, douce et unie */ Human, /** laine bouclée */ Wool }
const PIX: f64 = 0.0625;
fn mat_of(c: [f64; 3]) -> Mat {
    if c == EYE || c == GEYE || c == RED_EYE || c == EYEO { return Mat::Plain; }
    if c == CLOTH || c == CLOTH2 || c == TUNIC || c == TUNIC2 || c == CAPE || c == CAPE2 || c == MASK { Mat::Cloth } else if c == HOOD || c == HOOD2 { Mat::Leather } else if c == SKINB { Mat::Flesh } else if c == FURD || c == FURM || c == FURL { Mat::Fur } else if c == LEATH || c == LEATH2 || c == LEATHER { Mat::Leather } else if c == STEEL || c == STEEL2 || c == RUSTM || c == RUST { Mat::Metal }
    else if c == FLESH || c == FLESH2 { Mat::Flesh } else if c == BONE || c == BONE2 { Mat::Bone } else if c == HIDE || c == HIDE2 { Mat::Fur } else if c == CHIT || c == CHIT2 || c == SPOT || c == CHITD { Mat::Chitin } else if c == ABDO || c == ABDO2 { Mat::Flesh } else if c == LEGC || c == LEGD { Mat::Bone } else { Mat::Grain }
}
fn shade(m: Mat, c: [f64; 3], i: i32, j: i32, nu: i32, nv: i32, side: bool, seed: i32) -> [f64; 3] {
    let h = |k: i32| crate::noise::hash_float(i, j, seed, k); let (u, v) = ((i as f64 + 0.5) / nu as f64, (j as f64 + 0.5) / nv as f64);
    let mut f = 0.9 + 0.16 * h(1); let mut out = c;
    match m {
        Mat::Plain => return c,
        Mat::Grain => {}
        Mat::Bark => {
            // Des plaques d'écorce : chaque colonne de pixels est un sillon (sombre) ou une crête (claire), qui se décale tous les
            // quelques pixels de haut ; des crevasses en travers, un fond plus roux au creux des sillons, du lichen çà et là.
            let hh = |a: i32, b: i32, k: i32| crate::noise::hash_float(a, b, seed, k);
            if side { let run = j / 4 + (hh(i, 0, 40) * 3.0) as i32; let g = hh(i, run, 41);
                f = if g < 0.3 { 0.55 + 0.1 * h(1) } else if g > 0.72 { 1.12 + 0.1 * h(1) } else { 0.82 + 0.14 * h(1) };
                if g < 0.3 { out = [c[0] * 1.08, c[1] * 0.9, c[2] * 0.8]; }
                if hh(i / 2, j, 42) < 0.06 { f *= 0.6; } // une crevasse en travers
                if h(43) < 0.035 { out = [c[0] * 0.75 + 0.14, c[1] * 0.75 + 0.2, c[2] * 0.75 + 0.1]; f = 1.0; } // lichen
            } else { f = 0.78 + 0.2 * h(1); if (i + j) % 3 == 0 { f *= 0.86; } }
        }
        Mat::Cloth => { f = 0.93 + 0.1 * h(1); if i % 2 == 0 { f *= 0.95; } if side && v < 0.18 { f *= 0.8 + 0.1 * h(2); } if h(3) < 0.04 { f *= 0.84; } if h(4) < 0.04 { f *= 1.08; } }
        Mat::Leather => { f = 0.84 + 0.22 * h(1); if nu > 3 && (i == 0 || i == nu - 1) && j % 2 == 0 { f *= 1.28; } if h(5) < 0.07 { f *= 0.72; } }
        Mat::Metal => { f = 0.84 + 0.2 * h(1); if v > 0.8 { f *= 1.12; } if h(6) < 0.05 { f *= 0.66; }
            if h(7) < 0.14 { let r = [0.52, 0.3, 0.17]; let k = 0.55 + 0.3 * h(8); out = [c[0] + (r[0] - c[0]) * k, c[1] + (r[1] - c[1]) * k, c[2] + (r[2] - c[2]) * k]; } if (i + j) % 7 == 0 && h(9) < 0.4 { f *= 1.15; } }
        Mat::Flesh => { f = 0.8 + 0.26 * h(1); if h(10) < 0.09 { f *= 0.74; out = [c[0] * 1.08, c[1] * 0.92, c[2] * 0.9]; } if h(11) < 0.04 { f *= 1.15; } }
        Mat::Bone => { f = 0.9 + 0.14 * h(1); if h(12) < 0.07 { f *= 0.6; } }
        Mat::Fur => { f = 0.83 + 0.24 * ((h(1) + crate::noise::hash_float(i, j / 2, seed, 13)) * 0.5); if h(14) < 0.08 { f *= 1.25; } }
        Mat::Coat => { f = 0.93 + 0.07 * crate::noise::hash_float(i, j / 3, seed, 28) + 0.03 * h(1); if v > 0.72 && !side { f *= 1.05; } if h(29) < 0.03 { f *= 0.93; } }
        Mat::Skin => { f = 0.93 + 0.08 * h(1); if h(16) < 0.05 { f *= 1.12; } if h(17) < 0.03 { f *= 0.86; } }
        Mat::Shark => { f = 0.86 + 0.16 * h(1); if (i + 2 * j) % 5 == 0 { f *= 0.94; } if h(18) < 0.05 { f *= 0.78; } }
        Mat::Barnacle => { f = 0.88 + 0.12 * h(1); if h(19) < 0.07 { let k = 0.6 + 0.3 * h(20); out = [0.78 * k + c[0] * 0.2, 0.76 * k + c[1] * 0.2, 0.7 * k + c[2] * 0.2]; } if h(21) < 0.04 { f *= 0.7; } }
        Mat::Feather => { f = 0.86 + 0.14 * h(1); let row = j / 2; if j % 2 == 0 { f *= 0.84; } if (i + row) % 3 == 0 { f *= 1.1; } if h(22) < 0.05 { f *= 1.2; } }
        Mat::Scale => { f = 0.86 + 0.12 * h(1); if (i + j) % 2 == 0 { f *= 0.88; } if (i - j).rem_euclid(4) == 0 { f *= 1.14; } if h(23) < 0.05 { f *= 0.75; } }
        Mat::Hair => { let col = if side { i } else { i + j * 7 }; f = 0.8 + 0.3 * crate::noise::hash_float(col, 0, seed, 24) * (0.85 + 0.15 * h(1)); if h(25) < 0.05 { f *= 1.2; } }
        Mat::Human => { f = 0.95 + 0.06 * h(1); if h(26) < 0.04 { out = [c[0] * 1.03, c[1] * 0.96, c[2] * 0.95]; } }
        Mat::Wool => { f = 0.84 + 0.2 * h(1); if (i + 2 * j) % 3 == 0 { f *= 1.1; } if h(27) < 0.08 { f *= 0.8; } }
        Mat::Chitin => { f = 0.84 + 0.2 * h(1); if u < 0.12 || u > 0.88 || v > 0.88 { f *= 1.2; } if h(15) < 0.06 { out = [c[0] * 1.6, c[1] * 1.6, c[2] * 1.3]; } }
    }
    // L'ombrage du pixel art : un liseré plus sombre au bord de chaque face, le haut des flancs plus clair que le bas, le dessus éclairé.
    if nu >= 3 && nv >= 3 && (i == 0 || i == nu - 1 || j == 0 || j == nv - 1) { f *= 0.84; }
    if side { f *= 0.86 + 0.24 * v; } else { f *= 1.06; }
    [out[0] * f, out[1] * f, out[2] * f]
}
fn b(q: &mut VBuf, a: [f64; 6], c: [f64; 3]) { crate::sculpt::rbox(q, a, c, mat_of(c), PIX) }
/// Une boîte en pixels pour la faune marine : 0 peau lisse (dauphin), 1 peau de requin, 2 baleine à bernacles, 3 uni (yeux, dents).
pub(crate) fn skin(q: &mut VBuf, a: [f64; 6], c: [f64; 3], kind: u8) { bm(q, a, c, match kind { 0 => Mat::Skin, 1 => Mat::Shark, 2 => Mat::Barnacle, _ => Mat::Plain }) }
fn bm(q: &mut VBuf, a: [f64; 6], c: [f64; 3], m: Mat) { paint(q, a, c, m, PIX) }
/// Une boîte en pixels d'une matière, d'une taille de pixel donnée (plus fine pour les petites bêtes).
pub(crate) fn paint(q: &mut VBuf, a: [f64; 6], c: [f64; 3], m: Mat, pix: f64) {
    let pixv = pix;
    let (dx, dy, dz) = (a[3] - a[0], a[4] - a[1], a[5] - a[2]);
    if m == Mat::Plain || dx.max(dy).max(dz) < pixv * 1.6 { flat(q, a, c); return; }
    let seed = ((a[0] * 997.0) as i32) ^ ((a[1] * 613.0) as i32) << 3 ^ ((a[2] * 431.0) as i32) << 6;
    let [x0, y0, z0, x1, y1, z1] = a;
    // Les six faces, dans l'ordre et le sens de `mesher::bx` : coins p0, p1, p3 (le quatrième s'en déduit).
    let faces: [([f64; 3], [f64; 3], [f64; 3], [f64; 3], bool); 6] = [
        ([x0, y1, z1], [x1, y1, z1], [x0, y1, z0], [0.0, 1.0, 0.0], false), ([x0, y0, z0], [x1, y0, z0], [x0, y0, z1], [0.0, -1.0, 0.0], false),
        ([x0, y0, z1], [x1, y0, z1], [x0, y1, z1], [0.0, 0.0, 1.0], true), ([x1, y0, z0], [x0, y0, z0], [x1, y1, z0], [0.0, 0.0, -1.0], true),
        ([x1, y0, z1], [x1, y0, z0], [x1, y1, z1], [1.0, 0.0, 0.0], true), ([x0, y0, z0], [x0, y0, z1], [x0, y1, z0], [-1.0, 0.0, 0.0], true)];
    for (k, (p0, p1, p3, n, side)) in faces.iter().enumerate() {
        let du = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]]; let dv = [p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]];
        let lu = (du[0] * du[0] + du[1] * du[1] + du[2] * du[2]).sqrt(); let lv = (dv[0] * dv[0] + dv[1] * dv[1] + dv[2] * dv[2]).sqrt();
        let (nu, nv) = (((lu / pixv).round() as i32).clamp(1, 24), ((lv / pixv).round() as i32).clamp(1, 24));
        let at = |u: f64, v: f64| [p0[0] + du[0] * u + dv[0] * v, p0[1] + du[1] * u + dv[1] * v, p0[2] + du[2] * u + dv[2] * v];
        for i in 0..nu { for j in 0..nv { let (u0, u1, v0, v1) = (i as f64 / nu as f64, (i + 1) as f64 / nu as f64, j as f64 / nv as f64, (j + 1) as f64 / nv as f64);
            let (a0, a1, a2, a3) = (at(u0, v0), at(u1, v0), at(u1, v1), at(u0, v1)); let col = shade(m, c, i, j, nu, nv, *side, seed.wrapping_add(k as i32 * 131));
            crate::mesher::face(q, [a0[0], a0[1], a0[2], a1[0], a1[1], a1[2], a2[0], a2[1], a2[2], a3[0], a3[1], a3[2]], *n, 0.0, 0.0, 255.0, 255.0, col, 255.0, 0.0, crate::mesher::kind::FLAT); } }
    }
}

// ----------------------------------------------------------------------------------------------------------- modèles
const RUST: [f64; 3] = [0.46, 0.26, 0.13];
const EYE: [f64; 3] = [1.0, 0.86, 0.22]; const BONE: [f64; 3] = [0.82, 0.78, 0.66]; const HIDE: [f64; 3] = [0.19, 0.17, 0.15]; const HIDE2: [f64; 3] = [0.3, 0.26, 0.2]; const RED_EYE: [f64; 3] = [1.0, 0.42, 0.18];
const FLESH: [f64; 3] = [0.6, 0.5, 0.46]; const FLESH2: [f64; 3] = [0.48, 0.39, 0.36]; const BONE2: [f64; 3] = [0.72, 0.64, 0.56]; const CLOTH: [f64; 3] = [0.46, 0.14, 0.14]; const CLOTH2: [f64; 3] = [0.32, 0.1, 0.1];
const LEATH: [f64; 3] = [0.33, 0.21, 0.15]; const LEATH2: [f64; 3] = [0.24, 0.15, 0.11]; const STEEL: [f64; 3] = [0.47, 0.48, 0.52]; const STEEL2: [f64; 3] = [0.36, 0.37, 0.41]; const RUSTM: [f64; 3] = [0.5, 0.3, 0.18];
const TUNIC: [f64; 3] = [0.29, 0.26, 0.24]; const TUNIC2: [f64; 3] = [0.21, 0.19, 0.18]; const HOOD: [f64; 3] = [0.38, 0.25, 0.17]; const HOOD2: [f64; 3] = [0.28, 0.18, 0.13]; const MASK: [f64; 3] = [0.15, 0.14, 0.14];
const SKINB: [f64; 3] = [0.74, 0.56, 0.44]; const CAPE: [f64; 3] = [0.44, 0.13, 0.13]; const CAPE2: [f64; 3] = [0.31, 0.1, 0.1]; const SYMBOL: [f64; 3] = [0.86, 0.82, 0.74]; const WOODH: [f64; 3] = [0.45, 0.3, 0.17];
const FURD: [f64; 3] = [0.25, 0.22, 0.28]; const FURM: [f64; 3] = [0.4, 0.36, 0.43]; const FURL: [f64; 3] = [0.8, 0.7, 0.57];
const ABDO: [f64; 3] = [0.58, 0.15, 0.14]; const ABDO2: [f64; 3] = [0.4, 0.1, 0.11]; const CHITD: [f64; 3] = [0.17, 0.12, 0.2]; const EYEO: [f64; 3] = [1.0, 0.68, 0.12]; const LEGC: [f64; 3] = [0.86, 0.72, 0.5]; const LEGD: [f64; 3] = [0.58, 0.44, 0.3];
const CHIT: [f64; 3] = [0.38, 0.27, 0.42]; const CHIT2: [f64; 3] = [0.5, 0.38, 0.54]; const SPOT: [f64; 3] = [0.6, 0.62, 0.48]; const GEYE: [f64; 3] = [0.72, 1.0, 0.6]; const LEATHER: [f64; 3] = [0.36, 0.24, 0.14];
/// La tenue d'un brigand des camps : ses teintes (veste, cape, symbole du clan), ses épaulières (une de fer, ou deux), sa bandoulière.
struct BriPal { t1: [f64; 3], t2: [f64; 3], c1: [f64; 3], c2: [f64; 3], sy: [f64; 3], steel: bool, belt: bool }
const BRI_ERRANT: BriPal = BriPal { t1: TUNIC, t2: TUNIC2, c1: CAPE, c2: CAPE2, sy: SYMBOL, steel: false, belt: true };
/// L'archer : veste et cape vert de sous-bois, pour se fondre dans les fourrés.
const BRI_ARCHER: BriPal = BriPal { t1: [0.3, 0.31, 0.2], t2: [0.22, 0.23, 0.15], c1: [0.24, 0.32, 0.18], c2: [0.16, 0.22, 0.12], sy: [0.78, 0.74, 0.6], steel: false, belt: true };
/// L'épéiste : une cotte de mailles grise sous un tabard rouge, deux épaulières de fer.
const BRI_EPEISTE: BriPal = BriPal { t1: [0.45, 0.46, 0.49], t2: [0.34, 0.35, 0.38], c1: [0.5, 0.12, 0.12], c2: [0.36, 0.09, 0.09], sy: SYMBOL, steel: true, belt: false };
/// Le cavalier : un cuir fauve de chevaucheur, la cape rouge du clan.
const BRI_CAVALIER: BriPal = BriPal { t1: [0.42, 0.29, 0.18], t2: [0.31, 0.21, 0.13], c1: CAPE, c2: CAPE2, sy: SYMBOL, steel: false, belt: true };
/// Le chef de guerre : cuir noir, grande cape pourpre au symbole d'or, deux lourdes épaulières de fer.
const BRI_CHEF: BriPal = BriPal { t1: [0.15, 0.13, 0.13], t2: [0.1, 0.09, 0.09], c1: [0.5, 0.06, 0.08], c2: [0.34, 0.04, 0.06], sy: [0.9, 0.72, 0.26], steel: true, belt: true };
fn bri_torso(q: &mut VBuf, pal: &BriPal) {
        b(q, [-0.22, 0.06, -0.13, 0.22, 0.62, 0.13], pal.t1); b(q, [-0.2, 0.3, 0.13, -0.04, 0.6, 0.145], pal.t2); b(q, [0.04, 0.3, 0.13, 0.2, 0.6, 0.145], pal.t2);
        b(q, [-0.05, 0.08, 0.13, 0.05, 0.56, 0.14], [0.12, 0.11, 0.1]); for y in [0.2, 0.32, 0.44] { b(q, [-0.06, y, 0.135, 0.06, y + 0.025, 0.15], LEATH2); }
        // Bandoulière de l'épaule droite à la hanche gauche, et sangle du dos.
        if pal.belt { for k in 0..8 { let t = k as f64 / 7.0; let (x, y) = (-0.19 + 0.38 * t, 0.6 - 0.5 * t); b(q, [x - 0.035, y - 0.035, 0.135, x + 0.035, y + 0.035, 0.155], LEATH); b(q, [x - 0.035, y - 0.035, -0.155, x + 0.035, y + 0.035, -0.13], LEATH); } }
        b(q, [0.02, 0.3, 0.15, 0.08, 0.36, 0.165], RUSTM);
        // Ceinture large, boucle, sacoche à gauche, bourse à droite.
        b(q, [-0.235, -0.02, -0.14, 0.235, 0.09, 0.145], LEATH); b(q, [-0.05, -0.01, 0.145, 0.05, 0.08, 0.165], STEEL2); b(q, [-0.03, 0.01, 0.16, 0.03, 0.06, 0.17], LEATH2);
        b(q, [0.13, -0.17, 0.04, 0.26, 0.06, 0.16], LEATH2); b(q, [0.12, -0.0, 0.03, 0.27, 0.07, 0.17], LEATH); b(q, [0.18, -0.06, 0.16, 0.21, -0.02, 0.172], STEEL2); b(q, [-0.22, -0.12, 0.05, -0.14, 0.0, 0.13], LEATH2);
        // Pans de veste effrangés, rouges et bruns, de longueurs inégales.
        for (i, x) in [-0.19, -0.11, -0.03, 0.05, 0.13, 0.2].iter().enumerate() { let low = [-0.3, -0.24, -0.34, -0.26, -0.32, -0.22][i]; let c = if i % 3 == 1 { pal.c2 } else { pal.t2 }; b(q, [*x - 0.04, low, 0.04, *x + 0.04, 0.0, 0.14], c); }
        for z in [-0.1, 0.0] { b(q, [-0.235, -0.26, z, -0.2, 0.0, z + 0.09], pal.t2); b(q, [0.2, -0.24, z, 0.235, 0.0, z + 0.09], pal.c2); }
        // Cape dans le dos, bas en lambeaux, croix du clan.
        b(q, [-0.21, 0.0, -0.17, 0.21, 0.62, -0.14], pal.c1); for (x, low) in [(-0.17, -0.34), (-0.09, -0.24), (0.0, -0.38), (0.09, -0.26), (0.17, -0.32)] { b(q, [x - 0.045, low, -0.172, x + 0.045, 0.02, -0.142], if low < -0.3 { pal.c2 } else { pal.c1 }); }
        for k in 0..7 { let t = k as f64 / 6.0; let (x, y) = (-0.12 + 0.24 * t, 0.5 - 0.3 * t); b(q, [x - 0.03, y - 0.03, -0.185, x + 0.03, y + 0.03, -0.168], pal.sy); b(q, [-x - 0.03, y - 0.03, -0.185, -x + 0.03, y + 0.03, -0.168], pal.sy); }
        // Épaules : fer cabossé à gauche (deux plaques, rivets), cuir épais à droite.
        b(q, [0.14, 0.5, -0.16, 0.34, 0.66, 0.16], STEEL); b(q, [0.15, 0.43, -0.165, 0.35, 0.52, 0.165], STEEL2); b(q, [0.3, 0.58, 0.14, 0.33, 0.62, 0.17], [0.2, 0.2, 0.22]); b(q, [0.2, 0.62, -0.05, 0.28, 0.665, 0.08], RUSTM);
        if pal.steel { b(q, [-0.34, 0.5, -0.16, -0.14, 0.66, 0.16], STEEL); b(q, [-0.35, 0.43, -0.165, -0.15, 0.52, 0.165], STEEL2); b(q, [-0.33, 0.58, 0.14, -0.3, 0.62, 0.17], [0.2, 0.2, 0.22]); }
        else { b(q, [-0.33, 0.5, -0.15, -0.14, 0.64, 0.15], LEATH); b(q, [-0.335, 0.46, -0.155, -0.15, 0.52, 0.155], LEATH2); }
        // Col de foulard rouge.
        b(q, [-0.13, 0.6, -0.11, 0.13, 0.68, 0.12], pal.c2);
}
pub fn parts() -> Vec<(&'static str, VBuf)> {
    let mut o: Vec<(&'static str, VBuf)> = crate::monstres::parts(); let mut m = |n: &'static str, f: &dyn Fn(&mut VBuf)| { let mut v = VBuf::new(600); f(&mut v); o.push((n, v)); };
    // Rôdeur décharné (planche du bestiaire) : chair décharnée gris-rose, côtes saillantes, sangles de cuir croisées, ceinture
    // à boucle et sacoche, haillons rouge sombre en lanières, deux épaulières de fer rouillé, capuche rouge et visage de crâne
    // aux yeux jaunes, avant-bras bandés de cuir, doigts osseux, bottes de cuir sanglées, couteau rouillé. Membres en deux
    // segments (genoux, coudes) pour la posture voûtée et les gestes.
    m("rodTorso", &|q| {
        b(q, [-0.19, 0.06, -0.1, 0.19, 0.6, 0.11], FLESH); b(q, [-0.15, 0.62, -0.08, 0.15, 0.66, 0.08], FLESH2);
        for y in [0.26, 0.33, 0.4, 0.47] { b(q, [-0.17, y, 0.105, -0.03, y + 0.035, 0.125], BONE2); b(q, [0.03, y, 0.105, 0.17, y + 0.035, 0.125], BONE2); }
        b(q, [-0.02, 0.24, 0.105, 0.02, 0.56, 0.12], FLESH2); for y in [0.14, 0.2] { b(q, [-0.15, y, 0.1, 0.15, y + 0.03, 0.115], FLESH2); }
        // Sangles croisées sur la poitrine et le dos.
        for k in 0..7 { let t = k as f64 / 6.0; let (x, y) = (-0.17 + 0.34 * t, 0.58 - 0.46 * t); b(q, [x - 0.035, y - 0.03, -0.125, x + 0.035, y + 0.035, 0.135], LEATH); let x2 = 0.17 - 0.34 * t; b(q, [x2 - 0.03, y - 0.03, 0.118, x2 + 0.03, y + 0.03, 0.137], LEATH2); }
        b(q, [-0.02, 0.36, 0.13, 0.02, 0.4, 0.145], RUSTM);
        // Ceinture, boucle de fer, sacoche sur la hanche droite, petite bourse à gauche.
        b(q, [-0.21, -0.02, -0.125, 0.21, 0.08, 0.13], LEATH); b(q, [-0.05, -0.015, 0.13, 0.05, 0.075, 0.15], RUSTM); b(q, [-0.03, 0.005, 0.145, 0.03, 0.055, 0.155], LEATH2);
        b(q, [-0.25, -0.16, -0.06, -0.15, 0.05, 0.1], LEATH2); b(q, [-0.26, -0.02, -0.07, -0.14, 0.04, 0.11], LEATH); b(q, [0.15, -0.1, 0.03, 0.22, 0.0, 0.11], LEATH2);
        // Haillons rouges en lanières de longueurs inégales, devant, derrière et sur les flancs ; un pan de cape dans le dos.
        for (i, x) in [-0.18, -0.11, -0.04, 0.03, 0.1, 0.17].iter().enumerate() { let low = [-0.34, -0.46, -0.3, -0.42, -0.36, -0.28][i]; let c = if i % 2 == 0 { CLOTH } else { CLOTH2 };
            b(q, [*x - 0.035, low, 0.06, *x + 0.035, 0.0, 0.135], c); b(q, [*x - 0.035, low + 0.06, -0.135, *x + 0.035, 0.0, -0.06], if i % 2 == 0 { CLOTH2 } else { CLOTH }); }
        for z in [-0.07, 0.02] { b(q, [-0.215, -0.3, z, -0.19, 0.0, z + 0.07], CLOTH2); b(q, [0.19, -0.26, z, 0.215, 0.0, z + 0.07], CLOTH); }
        b(q, [-0.16, 0.38, -0.14, 0.16, 0.62, -0.11], CLOTH2); for (x, low) in [(-0.13, 0.16), (-0.065, 0.22), (0.0, 0.12), (0.065, 0.24), (0.13, 0.18)] { b(q, [x - 0.03, low, -0.145, x + 0.03, 0.4, -0.115], if low < 0.2 { CLOTH } else { CLOTH2 }); }
        // Épaulières de fer rouillé, deux plaques superposées de chaque côté, rivets et taches de rouille.
        for sx in [-1.0f64, 1.0] { let (x0, x1) = if sx < 0.0 { (-0.34, -0.13) } else { (0.13, 0.34) };
            b(q, [x0, 0.5, -0.15, x1, 0.66, 0.15], STEEL); b(q, [x0 - 0.01 * sx, 0.44, -0.155, x1 + 0.01 * sx, 0.52, 0.155], STEEL2);
            let rx = if sx < 0.0 { x0 + 0.04 } else { x1 - 0.07 }; b(q, [rx, 0.58, 0.15, rx + 0.04, 0.62, 0.162], RUSTM); b(q, [x0 + 0.05, 0.62, -0.1, x0 + 0.13, 0.665, 0.02], RUSTM);
            b(q, [(x0 + x1) * 0.5 - 0.015, 0.6, 0.15, (x0 + x1) * 0.5 + 0.015, 0.63, 0.165], [0.22, 0.22, 0.24]); }
    });
    // La tête : un visage de crâne émacié, deux yeux jaunes qui luisent au fond des orbites, des dents, sous une capuche
    // rouge profonde dont la pointe retombe vers l'avant et dont le pan couvre la nuque.
    m("rodHead", &|q| {
        b(q, [-0.1, 0.0, -0.08, 0.1, 0.25, 0.1], FLESH); b(q, [-0.09, -0.06, 0.0, 0.09, 0.03, 0.11], FLESH2);
        b(q, [-0.085, 0.12, 0.095, -0.015, 0.18, 0.11], [0.08, 0.06, 0.05]); b(q, [0.015, 0.12, 0.095, 0.085, 0.18, 0.11], [0.08, 0.06, 0.05]);
        b(q, [-0.07, 0.135, 0.105, -0.03, 0.165, 0.118], EYE); b(q, [0.03, 0.135, 0.105, 0.07, 0.165, 0.118], EYE);
        b(q, [-0.015, 0.08, 0.1, 0.015, 0.11, 0.115], [0.1, 0.07, 0.06]); for x in [-0.06, -0.03, 0.0, 0.03] { b(q, [x, 0.03, 0.105, x + 0.022, 0.06, 0.116], BONE); }
        b(q, [-0.07, -0.03, 0.1, 0.07, 0.0, 0.115], [0.4, 0.3, 0.27]);
        // Capuche ample : dessus, joues, arrière, bord avant épais retombant sur le front, ombre intérieure, pointe dressée vers
        // l'arrière, pan qui couvre la nuque en lanières.
        b(q, [-0.18, 0.26, -0.17, 0.18, 0.37, 0.16], CLOTH); b(q, [-0.185, -0.02, -0.17, -0.115, 0.32, 0.15], CLOTH); b(q, [0.115, -0.02, -0.17, 0.185, 0.32, 0.15], CLOTH);
        b(q, [-0.18, -0.1, -0.2, 0.18, 0.34, -0.1], CLOTH2); b(q, [-0.13, 0.22, 0.12, 0.13, 0.29, 0.19], CLOTH2); b(q, [-0.1, 0.19, 0.105, 0.1, 0.22, 0.125], [0.12, 0.05, 0.05]);
        b(q, [-0.125, -0.02, 0.12, -0.105, 0.24, 0.16], CLOTH2); b(q, [0.105, -0.02, 0.12, 0.125, 0.24, 0.16], CLOTH2);
        b(q, [-0.14, 0.36, -0.14, 0.14, 0.43, 0.1], CLOTH); b(q, [-0.1, 0.42, -0.18, 0.1, 0.5, 0.02], CLOTH2); b(q, [-0.06, 0.48, -0.24, 0.06, 0.56, -0.08], CLOTH); b(q, [-0.035, 0.54, -0.3, 0.035, 0.6, -0.18], CLOTH2);
        for (x, low) in [(-0.15, -0.2), (-0.06, -0.26), (0.04, -0.19), (0.13, -0.23)] { b(q, [x - 0.045, low, -0.21, x + 0.045, -0.08, -0.13], CLOTH); }
        b(q, [-0.17, -0.06, -0.06, -0.12, 0.08, 0.12], CLOTH2); b(q, [0.12, -0.06, -0.06, 0.17, 0.08, 0.12], CLOTH2);
    });
    // Haut du bras : décharné, une bande de cuir sous l'épaulière.
    m("rodUpper", &|q| { b(q, [-0.05, -0.34, -0.05, 0.05, 0.02, 0.05], FLESH); b(q, [-0.055, -0.12, -0.055, 0.055, -0.07, 0.055], LEATH2); b(q, [-0.045, -0.36, -0.045, 0.045, -0.31, 0.045], BONE2); });
    // Avant-bras bandé de cuir sanglé, main osseuse aux longs doigts.
    m("rodFore", &|q| { b(q, [-0.045, -0.32, -0.045, 0.045, 0.0, 0.045], FLESH2); b(q, [-0.058, -0.27, -0.058, 0.058, -0.05, 0.058], LEATH); for y in [-0.24, -0.16, -0.09] { b(q, [-0.062, y, -0.062, 0.062, y + 0.025, 0.062], LEATH2); }
        b(q, [-0.05, -0.38, -0.04, 0.05, -0.31, 0.05], FLESH); for (i, x) in [-0.045, -0.015, 0.015].iter().enumerate() { b(q, [*x, -0.48 + i as f64 * 0.01, -0.01, *x + 0.022, -0.37, 0.045], BONE); } b(q, [0.035, -0.42, 0.02, 0.06, -0.35, 0.06], BONE); });
    // Cuisse maigre, chiffon noué au genou.
    m("rodThigh", &|q| { b(q, [-0.055, -0.44, -0.055, 0.055, 0.0, 0.055], FLESH); b(q, [-0.07, -0.08, -0.07, 0.07, 0.02, 0.07], CLOTH2); b(q, [-0.065, -0.47, -0.065, 0.065, -0.38, 0.068], CLOTH); b(q, [-0.02, -0.46, 0.06, 0.02, -0.4, 0.075], BONE2); });
    // Jambe et botte de cuir, sangles, bout relevé.
    m("rodShin", &|q| { b(q, [-0.05, -0.2, -0.05, 0.05, 0.0, 0.05], FLESH2); b(q, [-0.07, -0.44, -0.07, 0.07, -0.16, 0.07], LEATH); for y in [-0.36, -0.25] { b(q, [-0.075, y, -0.075, 0.075, y + 0.03, 0.075], LEATH2); }
        b(q, [-0.075, -0.18, -0.08, 0.075, -0.13, 0.08], LEATH2); b(q, [-0.065, -0.47, -0.06, 0.065, -0.4, 0.17], LEATH); b(q, [-0.07, -0.49, -0.065, 0.07, -0.46, 0.18], [0.16, 0.11, 0.08]); });
    // Couteau rouillé : manche bandé, garde, lame large et ébréchée.
    m("rodKnife", &|q| { b(q, [-0.022, -0.1, -0.022, 0.022, 0.06, 0.022], LEATH2); for y in [-0.07, -0.02, 0.03] { b(q, [-0.025, y, -0.025, 0.025, y + 0.012, 0.025], LEATH); }
        b(q, [-0.07, 0.06, -0.022, 0.07, 0.09, 0.022], RUSTM); b(q, [-0.04, 0.09, -0.01, 0.04, 0.38, 0.01], STEEL); b(q, [-0.025, 0.38, -0.009, 0.025, 0.44, 0.009], STEEL); b(q, [0.0, 0.44, -0.008, 0.015, 0.47, 0.008], STEEL2);
        b(q, [-0.04, 0.16, -0.012, -0.015, 0.22, 0.012], RUSTM); b(q, [0.015, 0.27, -0.012, 0.04, 0.32, 0.012], RUSTM); b(q, [0.03, 0.12, -0.011, 0.045, 0.15, 0.011], [0.06, 0.06, 0.07]); });
    // Brigand errant (planche du bestiaire) : capuche de cuir brun qui tombe en pèlerine sur les épaules, foulard sombre
    // sur le bas du visage, veste épaisse, bandoulières, ceinture à boucle et sacoche, pans de veste effrangés, cape rouge
    // au symbole de clan (une croix claire) dans le dos, épaulière de fer à gauche et de cuir à droite, brassards et bottes
    // sanglées ; hache rudimentaire (ou épée, ou gourdin) et souvent un bouclier de planches cerclé à la croix.
    m("briTorso", &|q| bri_torso(q, &BRI_ERRANT));
    m("briTorsoArc", &|q| { bri_torso(q, &BRI_ARCHER); b(q, [-0.22, 0.1, 0.145, 0.22, 0.16, 0.16], [0.24, 0.23, 0.15]); });
    // L'épéiste : les mailles (des rangs sombres), le tabard rouge par-dessus, ceint d'un baudrier, la croix du clan sur la poitrine.
    m("briTorsoEpe", &|q| { bri_torso(q, &BRI_EPEISTE); let p = &BRI_EPEISTE;
        for y in [0.12, 0.2, 0.28, 0.36, 0.44, 0.52] { b(q, [-0.225, y, -0.135, -0.13, y + 0.018, 0.135], [0.27, 0.28, 0.3]); b(q, [0.13, y, -0.135, 0.225, y + 0.018, 0.135], [0.27, 0.28, 0.3]); }
        b(q, [-0.13, -0.3, 0.14, 0.13, 0.58, 0.162], p.c1); b(q, [-0.13, -0.3, 0.155, -0.1, 0.58, 0.165], p.c2); b(q, [0.1, -0.3, 0.155, 0.13, 0.58, 0.165], p.c2);
        b(q, [-0.025, 0.2, 0.162, 0.025, 0.5, 0.17], p.sy); b(q, [-0.09, 0.37, 0.162, 0.09, 0.42, 0.17], p.sy); b(q, [-0.15, 0.02, 0.16, 0.15, 0.09, 0.175], LEATH); });
    m("briTorsoCav", &|q| { bri_torso(q, &BRI_CAVALIER); b(q, [-0.25, 0.56, -0.17, 0.25, 0.7, 0.15], [0.46, 0.38, 0.28]); for x in [-0.2, -0.08, 0.04, 0.16] { b(q, [x, 0.5, -0.18, x + 0.07, 0.58, 0.16], [0.4, 0.32, 0.23]); } });
    // Le chef : col de fourrure, pointes de fer sur les épaulières, chaînes et trophées à la ceinture.
    m("briTorsoChef", &|q| { bri_torso(q, &BRI_CHEF);
        b(q, [-0.27, 0.58, -0.18, 0.27, 0.74, 0.17], [0.36, 0.3, 0.24]); for x in [-0.24, -0.12, 0.0, 0.12, 0.2] { b(q, [x, 0.54, -0.19, x + 0.07, 0.62, 0.18], [0.28, 0.23, 0.18]); }
        for sx in [-1.0f64, 1.0] { for z in [-0.08, 0.06] { let x = 0.25 * sx; b(q, [x - 0.025, 0.66, z - 0.025, x + 0.025, 0.76, z + 0.025], [0.6, 0.6, 0.62]); b(q, [x - 0.012, 0.76, z - 0.012, x + 0.012, 0.81, z + 0.012], [0.75, 0.75, 0.77]); } }
        for k in 0..6 { let x = -0.2 + k as f64 * 0.08; b(q, [x, 0.05 - (k % 2) as f64 * 0.03, 0.15, x + 0.04, 0.08, 0.17], [0.55, 0.55, 0.58]); }
        b(q, [-0.2, -0.16, 0.15, -0.12, -0.04, 0.2], BONE); b(q, [-0.185, -0.1, 0.2, -0.165, -0.07, 0.205], [0.1, 0.08, 0.08]); b(q, [-0.15, -0.1, 0.2, -0.13, -0.07, 0.205], [0.1, 0.08, 0.08]); });
    m("briHead", &|q| {
        b(q, [-0.12, 0.0, -0.1, 0.12, 0.27, 0.11], SKINB); b(q, [-0.1, 0.17, 0.105, -0.03, 0.2, 0.115], [0.95, 0.93, 0.88]); b(q, [0.03, 0.17, 0.105, 0.1, 0.2, 0.115], [0.95, 0.93, 0.88]);
        b(q, [-0.075, 0.172, 0.11, -0.045, 0.198, 0.12], [0.18, 0.12, 0.08]); b(q, [0.045, 0.172, 0.11, 0.075, 0.198, 0.12], [0.18, 0.12, 0.08]); b(q, [-0.11, 0.205, 0.105, -0.02, 0.225, 0.118], [0.2, 0.13, 0.09]); b(q, [0.02, 0.205, 0.105, 0.11, 0.225, 0.118], [0.2, 0.13, 0.09]);
        // Foulard sombre sur le nez et la bouche, noué derrière.
        b(q, [-0.125, -0.02, -0.105, 0.125, 0.15, 0.135], MASK); b(q, [-0.1, 0.0, 0.135, 0.1, 0.13, 0.145], [0.2, 0.19, 0.18]); b(q, [-0.05, -0.02, -0.14, 0.05, 0.06, -0.1], MASK);
        // Capuche de cuir : dessus, joues, nuque, bord avant, pointe, et pèlerine sur les épaules.
        b(q, [-0.165, 0.25, -0.15, 0.165, 0.35, 0.15], HOOD); b(q, [-0.17, -0.04, -0.15, -0.115, 0.3, 0.14], HOOD); b(q, [0.115, -0.04, -0.15, 0.17, 0.3, 0.14], HOOD); b(q, [-0.165, -0.06, -0.17, 0.165, 0.32, -0.1], HOOD2);
        b(q, [-0.13, 0.24, 0.12, 0.13, 0.3, 0.17], HOOD2); b(q, [-0.125, -0.04, 0.12, -0.11, 0.26, 0.155], HOOD2); b(q, [0.11, -0.04, 0.12, 0.125, 0.26, 0.155], HOOD2);
        b(q, [-0.1, 0.34, -0.12, 0.1, 0.4, 0.06], HOOD); b(q, [-0.05, 0.38, -0.16, 0.05, 0.44, -0.04], HOOD2);
        b(q, [-0.26, -0.12, -0.19, 0.26, -0.03, 0.17], HOOD); for (x, low) in [(-0.22, -0.2), (-0.12, -0.16), (0.0, -0.21), (0.12, -0.15), (0.22, -0.19)] { b(q, [x - 0.05, low, -0.18, x + 0.05, -0.1, 0.16], HOOD2); }
    });
    m("briUpper", &|q| { b(q, [-0.065, -0.34, -0.065, 0.065, 0.02, 0.065], TUNIC); b(q, [-0.07, -0.08, -0.07, 0.07, -0.03, 0.07], LEATH2); b(q, [-0.06, -0.36, -0.06, 0.06, -0.3, 0.06], TUNIC2); });
    m("briFore", &|q| { b(q, [-0.06, -0.3, -0.06, 0.06, 0.0, 0.06], TUNIC2); b(q, [-0.07, -0.29, -0.07, 0.07, -0.08, 0.07], LEATH); for y in [-0.25, -0.17, -0.11] { b(q, [-0.075, y, -0.075, 0.075, y + 0.022, 0.075], LEATH2); }
        b(q, [-0.06, -0.4, -0.05, 0.06, -0.3, 0.07], SKINB); b(q, [-0.06, -0.42, 0.02, 0.06, -0.36, 0.08], [0.66, 0.49, 0.38]); b(q, [0.04, -0.38, 0.04, 0.07, -0.33, 0.08], SKINB); });
    m("briThigh", &|q| { b(q, [-0.08, -0.44, -0.08, 0.08, 0.02, 0.08], TUNIC2); b(q, [0.0, -0.3, 0.075, 0.07, -0.2, 0.09], TUNIC); b(q, [-0.085, -0.46, -0.085, 0.085, -0.38, 0.09], LEATH2); });
    m("briShin", &|q| { b(q, [-0.075, -0.2, -0.075, 0.075, 0.0, 0.075], TUNIC2); b(q, [-0.085, -0.44, -0.08, 0.085, -0.12, 0.085], LEATH); b(q, [-0.095, -0.16, -0.09, 0.095, -0.1, 0.095], LEATH2);
        for y in [-0.36, -0.26] { b(q, [-0.09, y, -0.085, 0.09, y + 0.03, 0.09], LEATH2); b(q, [0.05, y, 0.085, 0.08, y + 0.03, 0.095], STEEL2); } b(q, [-0.085, -0.47, -0.07, 0.085, -0.4, 0.2], LEATH); b(q, [-0.09, -0.49, -0.075, 0.09, -0.46, 0.21], [0.16, 0.11, 0.08]); });
    // Armes, tenues dans le poing droit, le long de l'avant-bras ; bouclier au bras gauche.
    m("briAxe", &|q| { b(q, [-0.026, -0.12, -0.026, 0.026, 0.55, 0.026], WOODH); for y in [-0.08, 0.0] { b(q, [-0.03, y, -0.03, 0.03, y + 0.04, 0.03], LEATH2); }
        b(q, [-0.035, 0.4, -0.06, 0.035, 0.56, 0.05], STEEL2); b(q, [-0.022, 0.36, 0.04, 0.022, 0.6, 0.2], STEEL); b(q, [-0.018, 0.33, 0.18, 0.018, 0.63, 0.23], [0.72, 0.72, 0.74]); b(q, [-0.02, 0.45, -0.12, 0.02, 0.5, -0.05], STEEL2); });
    m("briSword", &|q| { b(q, [-0.025, -0.12, -0.025, 0.025, 0.06, 0.025], LEATH2); b(q, [-0.1, 0.06, -0.03, 0.1, 0.1, 0.03], STEEL2); b(q, [-0.035, 0.1, -0.012, 0.035, 0.78, 0.012], STEEL); b(q, [-0.02, 0.4, -0.014, 0.025, 0.6, 0.014], RUSTM); });
    m("briClub", &|q| { b(q, [-0.03, -0.1, -0.03, 0.03, 0.4, 0.03], WOODH); b(q, [-0.065, 0.3, -0.065, 0.065, 0.62, 0.065], [0.42, 0.29, 0.16]); for y in [0.38, 0.5] { b(q, [-0.075, y, -0.02, 0.075, y + 0.03, 0.02], STEEL2); } });
    m("briShield", &|q| { let rows: [(f64, f64); 6] = [(-0.3, 0.18), (-0.2, 0.26), (-0.08, 0.29), (0.06, 0.29), (0.18, 0.26), (0.28, 0.18)];
        for (i, (y, w)) in rows.iter().enumerate() { b(q, [-0.035, *y - 0.06, -*w, 0.0, *y + 0.07, *w], if i % 2 == 0 { WOODH } else { [0.5, 0.33, 0.18] }); }
        b(q, [-0.05, -0.38, -0.03, -0.03, 0.36, 0.03], STEEL2); b(q, [-0.05, -0.03, -0.3, -0.03, 0.03, 0.3], STEEL2);
        for k in 0..7 { let t = k as f64 / 6.0 - 0.5; b(q, [-0.06, t * 0.5 - 0.035, t * 0.4 - 0.035, -0.035, t * 0.5 + 0.035, t * 0.4 + 0.035], SYMBOL); b(q, [-0.06, t * 0.5 - 0.035, -t * 0.4 - 0.035, -0.035, t * 0.5 + 0.035, -t * 0.4 + 0.035], SYMBOL); }
        b(q, [-0.075, -0.04, -0.04, -0.045, 0.04, 0.04], STEEL); b(q, [0.0, -0.1, -0.03, 0.04, 0.1, 0.03], LEATH2); });
    // Le chapel de fer de l'épéiste : calotte ronde, large bord rabattu, nasal sur le nez, par-dessus la capuche.
    m("briHelm", &|q| { b(q, [-0.18, 0.25, -0.18, 0.18, 0.42, 0.18], STEEL); b(q, [-0.14, 0.42, -0.14, 0.14, 0.47, 0.14], STEEL2); b(q, [-0.02, 0.3, -0.16, 0.02, 0.49, 0.16], STEEL2);
        b(q, [-0.25, 0.24, -0.25, 0.25, 0.28, 0.25], STEEL2); b(q, [-0.26, 0.22, -0.26, 0.26, 0.245, 0.26], [0.3, 0.3, 0.33]); b(q, [-0.022, 0.1, 0.17, 0.022, 0.26, 0.2], STEEL);
        for (x, z) in [(-0.17, 0.0), (0.17, 0.0), (0.0, 0.17), (0.0, -0.17)] { b(q, [x - 0.015, 0.33, z - 0.015, x + 0.015, 0.36, z + 0.015], [0.22, 0.22, 0.24]); } });
    // Le heaume du chef : fermé, une fente noire pour les yeux, des trous de souffle, deux cornes recourbées, un panache rouge.
    m("briHelmChef", &|q| { let (k, k2) = ([0.3, 0.29, 0.31], [0.22, 0.21, 0.23]);
        b(q, [-0.19, -0.04, -0.19, 0.19, 0.42, 0.19], k); b(q, [-0.15, 0.42, -0.15, 0.15, 0.48, 0.15], k2); b(q, [-0.2, 0.2, -0.2, 0.2, 0.24, 0.2], k2);
        b(q, [-0.15, 0.15, 0.19, 0.15, 0.2, 0.2], [0.04, 0.03, 0.03]); b(q, [-0.02, 0.0, 0.19, 0.02, 0.15, 0.205], k2); for (x, y) in [(-0.1, 0.05), (-0.1, 0.09), (0.07, 0.05), (0.07, 0.09)] { b(q, [x, y, 0.19, x + 0.03, y + 0.02, 0.2], [0.04, 0.03, 0.03]); }
        b(q, [-0.2, -0.06, -0.2, 0.2, -0.02, 0.2], [0.9, 0.72, 0.26]);
        for sx in [-1.0f64, 1.0] { for (i, (x, y, w)) in [(0.2, 0.3, 0.05), (0.27, 0.35, 0.045), (0.32, 0.43, 0.04), (0.34, 0.52, 0.032), (0.33, 0.6, 0.024)].iter().enumerate() {
            b(q, [sx * x - *w, *y - *w, -*w, sx * x + *w, *y + *w, *w], if i < 2 { [0.86, 0.8, 0.66] } else { [0.78, 0.71, 0.56] }); } }
        for (z, h) in [(0.08, 0.6), (0.0, 0.66), (-0.08, 0.64), (-0.16, 0.58), (-0.22, 0.5)] { b(q, [-0.03, 0.46, z - 0.045, 0.03, h, z + 0.045], [0.6, 0.08, 0.08]); } });
    // Le carquois de l'archer, en travers du dos, les empennages qui dépassent par-dessus l'épaule.
    m("briQuiver", &|q| { b(q, [0.02, 0.1, -0.29, 0.16, 0.72, -0.18], LEATH); b(q, [0.01, 0.62, -0.3, 0.17, 0.68, -0.17], LEATH2); b(q, [0.01, 0.2, -0.3, 0.17, 0.25, -0.17], LEATH2);
        for (i, (x, z)) in [(0.05, -0.27), (0.1, -0.24), (0.13, -0.21), (0.07, -0.21)].iter().enumerate() { b(q, [*x - 0.012, 0.72, *z - 0.012, *x + 0.012, 0.82, *z + 0.012], WOODH); b(q, [*x - 0.025, 0.8, *z - 0.006, *x + 0.025, 0.9, *z + 0.006], if i % 2 == 0 { [0.88, 0.86, 0.8] } else { [0.6, 0.12, 0.1] }); } });
    // La grande hache du chef, à deux mains : longue hampe cerclée, double fer en croissant, pointe au sommet.
    m("briGreatAxe", &|q| { b(q, [-0.03, -0.45, -0.03, 0.03, 1.05, 0.03], [0.3, 0.2, 0.12]); for y in [-0.38, -0.1, 0.4] { b(q, [-0.035, y, -0.035, 0.035, y + 0.05, 0.035], STEEL2); }
        b(q, [-0.045, 0.78, -0.07, 0.045, 1.0, 0.07], STEEL2); b(q, [-0.02, 1.0, -0.02, 0.02, 1.16, 0.02], STEEL);
        for sz in [-1.0f64, 1.0] { for (z, y0, y1) in [(0.1, 0.76, 1.02), (0.18, 0.7, 1.08), (0.26, 0.64, 1.14), (0.31, 0.62, 1.16)] { b(q, [-0.022, y0, if sz > 0.0 { z - 0.05 } else { -z }, 0.022, y1, if sz > 0.0 { z } else { -z + 0.05 }], if z > 0.28 { [0.74, 0.74, 0.77] } else { STEEL }); } } });
    // La lance du cavalier : longue hampe, fer en feuille, fanion rouge du clan sous la pointe.
    m("briSpear", &|q| { b(q, [-0.025, -0.6, -0.025, 0.025, 1.55, 0.025], WOODH); b(q, [-0.03, -0.1, -0.03, 0.03, 0.08, 0.03], LEATH2);
        b(q, [-0.04, 1.5, -0.012, 0.04, 1.62, 0.012], STEEL2); b(q, [-0.05, 1.62, -0.01, 0.05, 1.76, 0.01], STEEL); b(q, [-0.025, 1.76, -0.008, 0.025, 1.86, 0.008], [0.72, 0.72, 0.74]);
        b(q, [-0.008, 1.22, 0.02, 0.008, 1.46, 0.22], CAPE); b(q, [-0.008, 1.22, 0.22, 0.008, 1.36, 0.32], CAPE2); b(q, [-0.008, 1.36, 0.22, 0.008, 1.46, 0.3], CAPE2); });
    // Bête rôdeuse (planche) : un grand loup noir hérissé, crinière en épis sur la nuque et le garrot, flancs gris,
    // ventre et pattes clairs, yeux jaunes, gueule rouge aux crocs blancs, griffes, queue touffue.
    m("bstBody", &|q| {
        b(q, [-0.2, -0.06, 0.0, 0.2, 0.34, 0.46], FURM); b(q, [-0.16, -0.04, -0.52, 0.16, 0.26, 0.02], FURM); b(q, [-0.15, -0.1, 0.06, 0.15, -0.0, 0.44], FURL); b(q, [-0.12, -0.08, -0.46, 0.12, 0.0, -0.02], FURL);
        b(q, [-0.21, 0.18, -0.02, 0.21, 0.36, 0.47], FURD); b(q, [-0.17, 0.14, -0.53, 0.17, 0.28, 0.02], FURD); b(q, [-0.19, 0.3, 0.32, 0.19, 0.44, 0.58], FURD);
        // Crinière en épis : nuque, garrot, échine.
        for (z, h, w) in [(0.52, 0.52, 0.16), (0.44, 0.5, 0.18), (0.36, 0.47, 0.17), (0.28, 0.43, 0.15), (0.18, 0.41, 0.13), (0.08, 0.38, 0.12), (-0.04, 0.35, 0.11), (-0.16, 0.33, 0.1), (-0.3, 0.31, 0.09), (-0.42, 0.3, 0.08)] {
            b(q, [-w, h - 0.08, z - 0.05, w, h, z + 0.05], FURD); b(q, [-w * 0.55, h - 0.02, z - 0.035, w * 0.55, h + 0.05, z + 0.03], FURD); b(q, [-0.025, h + 0.03, z - 0.02, 0.025, h + 0.08, z + 0.02], FURM); }
        for (x, z) in [(-0.22, 0.38), (0.22, 0.38), (-0.22, 0.24), (0.22, 0.24), (-0.2, 0.1), (0.2, 0.1)] { b(q, [x - 0.025, 0.05, z - 0.06, x + 0.025, 0.3, z + 0.05], FURM); }
        // Jabot clair sous le cou.
        b(q, [-0.13, -0.08, 0.36, 0.13, 0.22, 0.52], FURL); b(q, [-0.08, -0.14, 0.4, 0.08, -0.04, 0.5], FURL);
    });
    m("bstHead", &|q| {
        b(q, [-0.13, -0.06, -0.08, 0.13, 0.16, 0.16], FURD); b(q, [-0.15, -0.08, -0.12, 0.15, 0.1, 0.04], FURM); b(q, [-0.16, -0.12, -0.1, -0.1, 0.06, 0.06], FURL); b(q, [0.1, -0.12, -0.1, 0.16, 0.06, 0.06], FURL);
        b(q, [-0.08, -0.04, 0.16, 0.08, 0.08, 0.36], FURM); b(q, [-0.07, 0.04, 0.16, 0.07, 0.1, 0.32], FURD); b(q, [-0.035, 0.04, 0.34, 0.035, 0.09, 0.38], [0.06, 0.05, 0.05]);
        b(q, [-0.075, -0.05, 0.18, 0.075, -0.0, 0.35], [0.5, 0.12, 0.1]); for x in [-0.06, -0.03, 0.0, 0.03] { b(q, [x, -0.07, 0.3, x + 0.02, -0.02, 0.33], BONE); } b(q, [-0.07, -0.09, 0.24, -0.05, -0.02, 0.27], BONE); b(q, [0.05, -0.09, 0.24, 0.07, -0.02, 0.27], BONE);
        b(q, [-0.1, 0.07, 0.15, -0.04, 0.11, 0.17], EYE); b(q, [0.04, 0.07, 0.15, 0.1, 0.11, 0.17], EYE); b(q, [-0.11, 0.1, 0.13, -0.03, 0.14, 0.18], FURD); b(q, [0.03, 0.1, 0.13, 0.11, 0.14, 0.18], FURD);
        for sx in [-1.0f64, 1.0] { let x = 0.08 * sx; b(q, [x - 0.045, 0.14, -0.04, x + 0.045, 0.3, 0.04], FURD); b(q, [x - 0.025, 0.28, -0.02, x + 0.025, 0.36, 0.02], FURD); b(q, [x - 0.02, 0.16, 0.035, x + 0.02, 0.26, 0.045], [0.45, 0.35, 0.33]); }
    });
    m("bstJaw", &|q| { b(q, [-0.07, -0.06, 0.0, 0.07, 0.0, 0.32], FURM); b(q, [-0.06, 0.0, 0.02, 0.06, 0.015, 0.3], [0.62, 0.16, 0.14]); for x in [-0.05, -0.02, 0.01, 0.035] { b(q, [x, 0.0, 0.27, x + 0.018, 0.05, 0.3], BONE); } b(q, [-0.06, 0.0, 0.2, -0.04, 0.06, 0.23], BONE); b(q, [0.04, 0.0, 0.2, 0.06, 0.06, 0.23], BONE); b(q, [-0.08, -0.1, 0.02, 0.08, -0.04, 0.2], FURL); });
    m("bstFore", &|q| { b(q, [-0.075, -0.3, -0.08, 0.075, 0.06, 0.09], FURD); b(q, [-0.06, -0.56, -0.05, 0.06, -0.28, 0.07], FURL); b(q, [-0.07, -0.62, -0.04, 0.07, -0.55, 0.15], FURL); for x in [-0.055, -0.02, 0.015, 0.045] { b(q, [x, -0.64, 0.13, x + 0.018, -0.58, 0.19], BONE); } b(q, [-0.08, -0.3, 0.06, 0.08, -0.16, 0.1], FURM); });
    m("bstHind", &|q| { b(q, [-0.08, -0.26, -0.12, 0.08, 0.06, 0.08], FURD); b(q, [-0.055, -0.44, -0.12, 0.055, -0.24, -0.02], FURL); b(q, [-0.06, -0.5, -0.1, 0.06, -0.44, 0.1], FURL); for x in [-0.045, -0.01, 0.025] { b(q, [x, -0.52, 0.08, x + 0.018, -0.46, 0.14], BONE); } });
    m("bstTail", &|q| { b(q, [-0.06, -0.05, -0.2, 0.06, 0.07, 0.0], FURD); b(q, [-0.08, -0.1, -0.42, 0.08, 0.06, -0.18], FURM); b(q, [-0.07, -0.16, -0.58, 0.07, 0.02, -0.4], FURM); b(q, [-0.05, -0.2, -0.68, 0.05, -0.04, -0.56], FURL); });
    // Rampant nocturne (plans) : un arthropode trapu à six pattes, carapace bombée violette en plaques, ventre et flancs
    // rouge sang, tête basse rouge sombre à quatre yeux jaune-orangé, crochets d'os, pattes claires coudées et griffues.
    m("crwBody", &|q| {
        b(q, [-0.3, 0.0, -0.46, 0.3, 0.16, 0.34], ABDO); b(q, [-0.24, 0.02, -0.6, 0.24, 0.2, -0.42], ABDO); b(q, [-0.16, 0.05, -0.68, 0.16, 0.16, -0.58], ABDO2);
        for z in [-0.32, -0.12, 0.08, 0.26] { b(q, [-0.31, 0.02, z, 0.31, 0.12, z + 0.04], ABDO2); }
        b(q, [-0.375, 0.12, -0.44, 0.375, 0.28, 0.38], CHIT); b(q, [-0.31, 0.28, -0.38, 0.31, 0.4, 0.33], CHIT); b(q, [-0.22, 0.4, -0.3, 0.22, 0.47, 0.26], CHIT2); b(q, [-0.12, 0.47, -0.2, 0.12, 0.5, 0.16], CHIT2);
        // Les plaques : sillons sombres en travers, rebords clairs, une arête au milieu.
        for z in [-0.26, -0.08, 0.1] { b(q, [-0.38, 0.14, z, 0.38, 0.42, z + 0.035], CHITD); b(q, [-0.3, 0.4, z + 0.035, 0.3, 0.44, z + 0.07], CHIT2); }
        b(q, [-0.04, 0.47, -0.3, 0.04, 0.52, 0.26], CHITD); for x in [-0.385, 0.35] { b(q, [x, 0.12, -0.44, x + 0.035, 0.16, 0.38], CHIT2); }
        for (x, z) in [(-0.2, -0.18), (0.16, 0.0), (-0.1, 0.2), (0.22, -0.34)] { b(q, [x, 0.41, z, x + 0.06, 0.44, z + 0.06], SPOT); }
    });
    m("crwHead", &|q| {
        b(q, [-0.21, -0.02, 0.0, 0.21, 0.22, 0.2], ABDO2); b(q, [-0.22, 0.16, -0.02, 0.22, 0.27, 0.16], CHIT);
        for (x, y) in [(-0.15, 0.1), (0.09, 0.1), (-0.08, 0.15), (0.03, 0.15)] { b(q, [x, y, 0.195, x + 0.06, y + 0.05, 0.215], EYEO); }
        for sx in [-1.0f64, 1.0] { let x = 0.1 * sx; b(q, [x - 0.035, -0.06, 0.16, x + 0.035, 0.04, 0.27], LEGC); b(q, [x - 0.03, -0.14, 0.24, x + 0.03, -0.04, 0.3], LEGC); b(q, [x - 0.025 - 0.02 * sx, -0.18, 0.2, x + 0.025 - 0.02 * sx, -0.12, 0.26], LEGD); }
    });
    // Une patte : le haut part du flanc et monte, le genou sombre, le bas plonge jusqu'au sol, la griffe recourbée.
    m("crwLeg", &|q| { b(q, [0.0, -0.035, -0.04, 0.2, 0.05, 0.04], LEGD); b(q, [0.16, 0.02, -0.045, 0.3, 0.12, 0.045], LEGC); b(q, [0.27, 0.06, -0.045, 0.38, 0.16, 0.045], LEGD);
        b(q, [0.34, -0.06, -0.04, 0.44, 0.12, 0.04], LEGC); b(q, [0.38, -0.2, -0.035, 0.46, -0.04, 0.035], LEGC); b(q, [0.35, -0.27, -0.03, 0.42, -0.18, 0.03], LEGD); b(q, [0.31, -0.29, -0.025, 0.37, -0.24, 0.025], LEGC); });
    o
}

/// Les humanoïdes plus épais : buste élargi, membres plus forts (largeur et profondeur), tête un peu plus grosse.
fn thick(n: &str, m: M4) -> M4 {
    let (sx, sz) = match n { "rodTorso" => (1.22, 1.25), "briTorso" | "briTorsoArc" | "briTorsoEpe" | "briTorsoCav" | "briTorsoChef" | "briQuiver" => (1.18, 1.22), "rodHead" | "briHead" | "briHelm" | "briHelmChef" => (1.1, 1.08), "rodUpper" | "rodFore" | "rodThigh" | "rodShin" => (1.4, 1.35), "briUpper" | "briFore" | "briThigh" | "briShin" => (1.3, 1.28), _ => return m };
    let mut k = m; for r in 0..3 { k[r] *= sx as f32; k[8 + r] *= sz as f32; } k
}
impl Horde {
    /// Corps articulés, gestes de chaque état : marche, course, détection (tête dressée), attaque annoncée puis coup, blessure, mort.
    pub fn draws(&self, md: &Models) -> Vec<Draw> {
        let mut out = Vec::with_capacity(self.foes.len() * 12);
        for f in &self.foes {
            let s = &bestiary().species[f.sp]; let hurt = (f.hit / 0.3).clamp(0.0, 1.0) as f32;
            // le boss rougeoie quand vient une attaque qu'aucun bouclier n'arrête : il faut rouler
            let hurt = if f.dead == 0.0 && ((f.wind > 0.0 && unblockable(f.mv)) || (s.boss && f.lunge > 0.0)) { hurt.max(0.5 + 0.3 * (f.phase * 12.0).sin() as f32) } else { hurt }; let mut put = |n: &str, m: M4| { let m = thick(n, m); out.push(Draw { m, r: md.part(n), view: false, local: false, hero: false, dynamic: false, hurt }) };
            let fall = (f.dead / 0.9).min(1.0); let flinch = crate::wild::flinch(f.hit) as f64;
            // L'annonce : de 0 (rien) à 1 (au sommet du geste) ; puis le coup part d'un coup.
            let total = if s.boss { (match f.mv { MV_TAILLE => WIND_TAILLE, MV_COMBO => WIND_COMBO, MV_ECRASE => WIND_ECRASE, MV_CHARGE => WIND_CHARGE, MV_LANCE => WIND_LANCE, MV_PASSE => WIND_PASSE, MV_CABRE => WIND_CABRE, MV_TIR => WIND_TIR, MV_COUP => WIND_COUP, MV_SALVE => WIND_SALVE, _ => s.windup }) * if f.enraged { 0.7 } else { 1.0 } } else if f.mv == MV_TIR { s.windup + WIND_TIR } else { s.windup };
            let wind = if f.wind > 0.0 { (1.0 - f.wind / total.max(0.05)).clamp(0.0, 1.0) } else { 0.0 }; let strike = if s.boss { if f.wind == 0.0 && f.mv == 0 && f.cd > 0.9 && f.state == Ai::Combat && f.broken <= 0.0 { 1.0 } else { 0.0 } } else if f.wind == 0.0 && f.cd > s.cd - 0.25 && f.state == Ai::Combat { 1.0 } else { 0.0 };
            let run = (f.speed / s.run.max(0.1)).clamp(0.0, 1.0); let sw = if f.speed > 0.0 { sin(f.phase) } else { 0.0 };
            let base = mul(&trs(f.p[0], f.p[1], f.p[2], f.yaw, 0.0, s.size * if f.vet { 1.08 } else { 1.0 }), &roll_z(fall * 1.45 * if f.variant % 2 == 0 { 1.0 } else { -1.0 }));
            match s.body {
                Body::Rodeur => {
                    // Posture voûtée, genoux pliés, tête en avant. Repos : il respire et dodeline ; marche : pas traînants ;
                    // course : buste plongeant, bras rejetés ; détection : il se redresse, tête levée ; attaque : couteau levé
                    // haut derrière la tête, puis taillade en travers ; blessure : il recule ; mort : il s'effondre face contre terre.
                    let t = f.phase; let det = if f.state == Ai::Detection || f.state == Ai::Soupcon { 1.0 } else { 0.0 }; let die = fall;
                    let breathe = if f.speed == 0.0 { 0.02 * (t * 1.3).sin() } else { 0.0 };
                    let stride = (0.32 + 0.38 * run) * sw; let knee0 = 0.34 - 0.2 * det + 0.9 * die; let thigh0 = -0.2 + 0.15 * det - 0.9 * die;
                    let hip = 0.87 + 0.03 * det - 0.62 * die + breathe * 0.5 - (f.phase * 2.0).sin().abs() * 0.03 * run;
                    let lean = 0.24 + 0.28 * run - 0.2 * det + wind * 0.1 - strike * 0.05 - flinch * 0.9 + 1.0 * die;
                    let root = mul(&base, &trs(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)); let _ = roll_z;
                    let body = mul(&trs(f.p[0], f.p[1] + hip * s.size, f.p[2], f.yaw, 0.0, s.size), &trs(0.0, 0.0, 0.0, 0.08 * (t * 0.45).sin() * (1.0 - run), lean, 1.0)); let _ = root;
                    put("rodTorso", body);
                    let head_pitch = 0.12 - lean * 0.4 - 0.3 * det + 0.06 * (t * 0.8).sin() * (1.0 - run) + 0.4 * die; let head_yaw = if f.state == Ai::Soupcon { 0.45 * (t * 0.7).sin() } else { 0.06 * (t * 0.31).sin() };
                    put("rodHead", mul(&body, &trs(0.0, 0.64, 0.08, head_yaw, head_pitch, 1.0)));
                    let legs = trs(f.p[0], f.p[1] + hip * s.size, f.p[2], f.yaw, 0.0, s.size);
                    for (x, st) in [(-0.12, stride), (0.12, -stride)] { let th = mul(&legs, &trs(x, 0.0, 0.0, 0.0, thigh0 + st, 1.0)); put("rodThigh", th);
                        put("rodShin", mul(&th, &trs(0.0, -0.44, 0.0, 0.0, knee0 + (if st > 0.0 { 0.0 } else { -st * 1.1 }), 1.0))); }
                    // Bras : ballants et longs, en avant à cause du dos voûté ; le droit tient le couteau.
                    let dang = -lean * 0.6 - 0.08; let swing_l = -sw * (0.25 + 0.5 * run); let swing_r = sw * (0.25 + 0.5 * run);
                    let (ul, fl) = (dang + swing_l - 0.35 * det - 0.6 * flinch + 1.2 * die, -0.35 - 0.3 * det - 0.4 * run);
                    let (ur, fr) = if wind > 0.0 || strike > 0.0 { (dang - 0.6 - 2.3 * wind + strike * 1.1, -0.9 * wind - 0.2 - strike * 0.3) } else { (dang + swing_r - 0.35 * det - 0.6 * flinch + 1.2 * die, -0.45 - 0.3 * det - 0.4 * run) };
                    let ua = mul(&body, &trs(0.33, 0.56, 0.0, -0.15 * det, ul, 1.0)); put("rodUpper", ua); put("rodFore", mul(&ua, &trs(0.0, -0.34, 0.0, 0.0, fl, 1.0)));
                    let ub = mul(&body, &trs(-0.33, 0.56, 0.0, 0.15 * det + strike * 0.5, ur, 1.0)); put("rodUpper", ub); let fb = mul(&ub, &trs(0.0, -0.34, 0.0, 0.0, fr, 1.0)); put("rodFore", fb);
                    put("rodKnife", mul(&fb, &trs(0.0, -0.4, 0.04, 0.0, 1.45, 1.0)));
                }
                Body::Brigand => {
                    // Debout, solide. Repos : il respire, regarde alentour ; marche, course ; détection : il se redresse, tête levée ;
                    // attaque : hache levée haut puis abattue ; blocage : bouclier levé devant soi ; repli ; mort : il tombe à la renverse.
                    let t = f.phase; let det = if f.state == Ai::Detection || f.state == Ai::Soupcon { 1.0 } else { 0.0 }; let die = fall; let guard = f.guard;
                    let flee = if f.state == Ai::Fuite { 1.0 } else { 0.0 };
                    let stride = (0.34 + 0.4 * run) * sw; let knee0 = 0.12 + 0.18 * run + 0.15 * guard + 0.9 * die; let thigh0 = -0.06 - 0.1 * run - 0.1 * guard - 0.9 * die;
                    let hip = 0.92 - 0.04 * guard - 0.6 * die + if f.speed == 0.0 { 0.012 * (t * 1.2).sin() } else { 0.0 } - (t * 2.0).sin().abs() * 0.03 * run;
                    let lean = 0.06 + 0.26 * run - 0.08 * det + 0.14 * guard + wind * 0.08 - strike * 0.05 - flinch * 0.8 - 1.2 * die;
                    // Le boss : chaque attaque a son geste. La taille : le buste vrillé, la hache rejetée de côté ; la frappe écrasante : la
                    // hache dressée très haut, le corps cambré ; la ruée : l'épaule en avant, tête baissée ; garde brisée : il titube.
                    let bmv = if s.boss && f.dead == 0.0 { if f.lunge > 0.0 { MV_CHARGE } else { f.mv } } else { 0 }; let reel = if f.broken > 0.0 && f.dead == 0.0 { 1.0 } else { 0.0 };
                    let lean = match bmv { MV_ECRASE => lean - 0.3 * wind, MV_CHARGE => 0.55, _ => lean } - 0.3 * reel + 0.12 * reel * (t * 3.0).sin();
                    let twist = if bmv == MV_TAILLE { 0.9 * wind } else if s.boss && strike > 0.0 { -0.5 } else { -0.3 * wind + 0.35 * strike } + 0.2 * reel * (t * 2.3).sin();
                    let hip = hip - 0.14 * reel - if bmv == MV_CHARGE { 0.08 } else { 0.0 };
                    let body = mul(&trs(f.p[0], f.p[1] + hip * s.size, f.p[2], f.yaw, 0.0, s.size), &trs(0.0, 0.0, 0.0, 0.06 * (t * 0.4).sin() * (1.0 - run) + twist, lean, 1.0));
                    // Chacun sa tenue : l'archer en vert, carquois au dos ; l'épéiste en mailles et chapel de fer ; le chef en noir, heaume à cornes.
                    put(match s.skin { "archer" => "briTorsoArc", "epeiste" => "briTorsoEpe", "chef" => "briTorsoChef", _ => "briTorso" }, body);
                    if s.skin == "archer" { put("briQuiver", body); }
                    let hy = if f.state == Ai::Soupcon || (f.speed == 0.0 && f.state == Ai::Repos) { 0.5 * (t * 0.35).sin() } else { 0.0 };
                    let head = mul(&body, &trs(0.0, 0.66, 0.02, hy, -lean * 0.5 - 0.25 * det + 0.3 * die, 1.0)); put("briHead", head);
                    match s.skin { "epeiste" => put("briHelm", head), "chef" if !f.enraged => put("briHelmChef", head), _ => {} }
                    let legs = trs(f.p[0], f.p[1] + hip * s.size, f.p[2], f.yaw, 0.0, s.size);
                    for (x, st) in [(-0.135, stride), (0.135, -stride)] { let th = mul(&legs, &trs(x, 0.0, 0.0, 0.0, thigh0 + st, 1.0)); put("briThigh", th); put("briShin", mul(&th, &trs(0.0, -0.44, 0.0, 0.0, knee0 + if st > 0.0 { 0.0 } else { -st * 1.1 }, 1.0))); }
                    let shield = shielded(s, f.variant); let aiming = s.bow && f.dead == 0.0 && (f.wind > 0.0 || f.state == Ai::Combat); let chef = s.skin == "chef";
                    // Bras gauche : bouclier au repos le long du corps, levé devant soi en garde ; l'arc tendu à bout de bras ; ou la hampe de
                    // la grande hache, à deux mains. Bras droit : l'arme, ou la corde tirée jusqu'à la joue.
                    let (ur, fr) = if aiming { (-1.4 + 0.1 * wind, -1.2 - 0.5 * wind) } else if reel > 0.0 { (0.25, -0.2) } else if bmv == MV_TAILLE && wind > 0.0 { (-1.45, -0.15) } else if bmv == MV_ECRASE && wind > 0.0 { (-0.4 - 3.1 * wind, -0.3 * wind) } else if bmv == MV_CHARGE { (0.15, -0.7) } else if wind > 0.0 || strike > 0.0 { (-0.4 - 2.4 * wind + strike * 1.3, -0.7 * wind - 0.15) } else { (sw * (0.3 + 0.5 * run) - 0.25 - 0.3 * det + 1.2 * die, -0.4 - 0.4 * run) };
                    let (ul, fl, yl) = if aiming { (-1.5, -0.05, -0.15) } else if chef { (ur - 0.1, fr - 0.3, 0.45) } else if guard > 0.0 { (-1.1 * guard - 0.2, -0.9 * guard, 0.6 * guard) } else { (-sw * (0.3 + 0.5 * run) - 0.15 - 0.3 * det + 1.2 * die - 0.3 * flee, -0.35 - 0.4 * run, 0.0) };
                    let ua = mul(&body, &trs(0.36, 0.56, 0.0, -yl, ul, 1.0)); put("briUpper", ua); let fa = mul(&ua, &trs(0.0, -0.34, 0.0, 0.0, fl, 1.0)); put("briFore", fa);
                    if shield { put("briShield", mul(&fa, &trs(0.13, -0.2, 0.0, 0.0, 0.0, 1.0))); }
                    if s.bow { put("arcBow", mul(&fa, &trs(0.0, -0.4, 0.04, 0.0, if aiming { 1.5 } else { 0.3 }, 1.0))); }
                    let ub = mul(&body, &trs(-0.36, 0.56, 0.0, if aiming { 0.3 } else { 0.0 }, ur, 1.0)); put("briUpper", ub); let fb = mul(&ub, &trs(0.0, -0.34, 0.0, 0.0, fr, 1.0)); put("briFore", fb);
                    let weapon = match s.skin { "archer" => None, "epeiste" => Some("briSword"), "chef" => Some("briGreatAxe"), _ => Some(["briAxe", "briSword", "briClub"][f.variant as usize % 3]) };
                    if let Some(wpn) = weapon { put(wpn, mul(&fb, &trs(0.0, -0.38, if chef { 0.0 } else { 0.05 }, 0.0, if reel > 0.0 { 0.4 } else { std::f64::consts::FRAC_PI_2 * 0.92 }, 1.0))); }
                }
                Body::Cavalier => {
                    // Le cheval (les pièces de l'élevage) au pas, au trot ou au galop ; le brigand en selle, les jambes de part et d'autre, la
                    // bride dans la main gauche, la lance dans la droite : dressée au pas, couchée en avant dans la charge, levée puis abattue
                    // pour frapper. Mort : cheval et cavalier basculent sur le flanc.
                    let t = f.phase; let det = if f.state == Ai::Detection || f.state == Ai::Soupcon { 1.0 } else { 0.0 }; let die = fall; let v = (f.variant % 3) as usize;
                    let charging = f.dead == 0.0 && if s.boss { f.lunge > 0.0 || (f.mv == MV_LANCE && f.wind > 0.0) } else { s.charge && f.burst > 0.0 && f.state == Ai::Poursuite };
                    // le cabrage : le cheval se dresse sur ses postérieurs, les antérieurs battant l'air, puis retombe
                    let rear = if f.mv == MV_CABRE && f.wind > 0.0 { (wind * 1.6).min(1.0) } else { 0.0 };
                    let (bob, tilt, legs, nod) = crate::wild::gait(crate::wild::Kind::Horse, f.speed, t, if f.speed > 5.0 { 0.9 } else { 0.6 });
                    let hb = mul(&trs(f.p[0], f.p[1] + bob + 0.55 * rear, f.p[2], f.yaw, tilt - flinch * 0.3 - if rear > 0.0 { 0.75 * rear } else { 0.25 * wind }, 2.5), &roll_z(die * 1.4 * if f.variant % 2 == 0 { 1.0 } else { -1.0 }));
                    let hip = 0.45; put(["horse0", "horse1", "horse2"][v], mul(&hb, &trs(0.0, hip, 0.0, 0.0, 0.0, 1.0)));
                    put(["horseHead0", "horseHead1", "horseHead2"][v], mul(&hb, &trs(0.0, hip + 0.2, 0.38, 0.0, nod - 0.3 * det - 0.2 * wind + 0.3 * die, 1.0)));
                    for (k, ((x, z), a)) in [(-0.1, 0.3), (0.1, 0.3), (-0.1, -0.3), (0.1, -0.3)].into_iter().zip(legs).enumerate() { put(if k < 2 { ["horseLeg0", "horseLeg1", "horseLeg2"][v] } else { ["horseHind0", "horseHind1", "horseHind2"][v] }, mul(&hb, &trs(x, hip, z, 0.0, a + if k < 2 { -0.6 * wind - 0.5 * rear + 0.4 * rear * (t * 9.0).sin() } else { 0.6 * rear }, 1.0))); }
                    put(["horseTail0", "horseTail1", "horseTail2"][v], mul(&hb, &trs(0.0, hip + 0.29, -0.46, 0.3 * (t * 0.4).sin() * (1.0 - run), -(0.2 + 0.6 * run), 1.0)));
                    put("saddle", mul(&hb, &trs(0.0, hip + 0.33, 0.04, 0.0, 0.0, 1.0)));
                    let seat = mul(&hb, &trs(0.0, hip + 0.35, 0.0, 0.0, 0.0, s.size / 2.5));
                    let lean = 0.08 + 0.18 * run + if charging { 0.3 } else { 0.0 } - 0.1 * wind + 0.3 * strike - flinch * 0.6;
                    let body = mul(&seat, &trs(0.0, 0.0, 0.0, 0.0, lean, 1.0)); put("briTorsoCav", body);
                    put("briHead", mul(&body, &trs(0.0, 0.66, 0.02, 0.0, -lean * 0.7 - 0.2 * det, 1.0)));
                    for sx in [-1.0f64, 1.0] { let th = mul(&seat, &mul(&trs(0.2 * sx, 0.02, 0.02, 0.0, -1.3, 1.0), &roll_z(0.55 * sx))); put("briThigh", th);
                        put("briShin", mul(&th, &mul(&trs(0.0, -0.44, 0.0, 0.0, 1.45, 1.0), &roll_z(-0.45 * sx)))); }
                    // la bride, à gauche ; la lance, à droite
                    let ua = mul(&body, &trs(0.36, 0.56, 0.0, -0.2, -0.75 + 0.1 * (t * 0.5).sin(), 1.0)); put("briUpper", ua); put("briFore", mul(&ua, &trs(0.0, -0.34, 0.0, 0.0, -0.7, 1.0)));
                    let (ur, fr, lance) = if charging { (-0.35, -0.15, std::f64::consts::FRAC_PI_2 * 0.97) } else if wind > 0.0 || strike > 0.0 { (-0.4 - 2.4 * wind + strike * 1.3, -0.7 * wind - 0.15, std::f64::consts::FRAC_PI_2 * 0.92) } else { (-0.2, -0.55, 0.45) };
                    let ub = mul(&body, &trs(-0.36, 0.56, 0.0, 0.0, ur, 1.0)); put("briUpper", ub); let fb = mul(&ub, &trs(0.0, -0.34, 0.0, 0.0, fr, 1.0)); put("briFore", fb);
                    put("briSpear", mul(&fb, &trs(0.0, -0.38, 0.05, 0.0, lance, 1.0)));
                }
                Body::Bete => {
                    // Repos : il souffle, la queue balance ; marche ; course (galop) ; repérage : tête et oreilles dressées ;
                    // bond : corps tendu, pattes en avant ; morsure : gueule grande ouverte, tête lancée ; mort : couché sur le flanc.
                    let t = f.phase; let det = if f.state == Ai::Detection || f.state == Ai::Soupcon || f.state == Ai::Embuscade { 1.0 } else { 0.0 }; let die = fall;
                    let gallop = run > 0.55; let crouch = wind * 0.12 + 0.08 * det * if f.state == Ai::Embuscade { 1.0 } else { 0.0 };
                    let pitch = if f.air { -0.3 + 0.5 * (f.v[1] / 6.0).clamp(-1.0, 1.0) * -1.0 } else { 0.0 } + flinch * 0.5;
                    let hip = 0.6 - crouch - 0.38 * die + if gallop { (t).sin().abs() * 0.07 } else { 0.012 * (t * 1.4).sin() };
                    let base2 = mul(&trs(f.p[0], f.p[1] + hip * s.size, f.p[2], f.yaw, 0.0, s.size), &roll_z(die * 1.5));
                    let body = mul(&base2, &trs(0.0, 0.0, 0.0, 0.0, pitch, 1.0)); put("bstBody", body);
                    let head = mul(&body, &trs(0.0, 0.34, 0.62, 0.0, 0.05 - 0.4 * det + wind * 0.25 - strike * 0.35 + 0.3 * die, 1.3)); put("bstHead", head);
                    put("bstJaw", mul(&head, &trs(0.0, -0.06, 0.02, 0.0, 0.08 + (wind + strike) * 0.6 + if f.air { 0.6 } else { 0.0 } + 0.25 * det, 1.0)));
                    put("bstTail", mul(&body, &trs(0.0, 0.22, -0.5, 0.25 * (t * if f.speed > 0.0 { 1.0 } else { 0.5 }).sin(), -0.5 + 0.35 * run - 0.25 * det, 1.0)));
                    let a = if f.air { 1.0 } else { sw * (0.45 + 0.4 * run) }; let b2 = if gallop || f.air { a } else { -a }; let hind = if f.air { -0.9 } else { 0.0 };
                    put("bstFore", mul(&body, &trs(-0.13, 0.04, 0.36, 0.0, -a, 1.0))); put("bstFore", mul(&body, &trs(0.13, 0.04, 0.36, 0.0, -b2, 1.0)));
                    put("bstHind", mul(&body, &trs(-0.12, 0.04, -0.38, 0.0, b2 * 0.9 - hind, 1.0))); put("bstHind", mul(&body, &trs(0.12, 0.04, -0.38, 0.0, a * 0.9 - hind, 1.0)));
                }
                Body::Rampant => {
                    // Corps bas et lourd ; trépied alterné (avant gauche, milieu droit, arrière gauche, puis l'inverse) ;
                    // repérage : il se soulève sur ses pattes ; attaque : il se cabre, crochets en avant ; mort : sur le dos.
                    let t = f.phase; let det = if f.state == Ai::Detection || f.state == Ai::Soupcon { 1.0 } else { 0.0 }; let die = fall;
                    let lift = 0.05 * det + wind * 0.08 - 0.1 * die; let rear = if f.climbing { -1.2 } else { -wind * 0.45 + strike * 0.25 } + flinch * 0.3;
                    let hip = 0.27 + lift + if f.speed > 0.0 { (t * 2.0).sin().abs() * 0.02 } else { 0.006 * (t * 1.6).sin() };
                    let root = mul(&mul(&trs(f.p[0], f.p[1] + hip * s.size, f.p[2], f.yaw, 0.0, s.size), &roll_z(die * 3.0)), &trs(0.0, 0.0, 0.0, 0.0, rear, 1.0));
                    let (pb, ph, pl) = if s.skin == "magma" { ("mgmBody", "mgmHead", "mgmLeg") } else { ("crwBody", "crwHead", "crwLeg") };
                    put(pb, root); put(ph, mul(&root, &trs(0.0, 0.0, 0.36, 0.05 * (t * 0.7).sin(), if f.state == Ai::Embuscade { 0.15 } else { -0.05 - 0.2 * det }, 1.0)));
                    for (k, z) in [0.22, 0.0, -0.24].iter().enumerate() { for sx in [-1.0f64, 1.0] {
                        let tri = if (k + if sx > 0.0 { 1 } else { 0 }) % 2 == 0 { 1.0 } else { -1.0 }; let swing = sw * 0.4 * tri * (0.4 + run); let up = (sw * tri).max(0.0) * 0.12 * run;
                        let spread = [0.45, 0.0, -0.45][k] * sx; let curl = die * 1.2;
                        let leg = mul(&root, &mul(&trs(0.3 * sx, 0.1, *z, (if sx > 0.0 { 0.0 } else { std::f64::consts::PI }) + spread * 0.6 + swing, 0.0, 1.0), &roll_z(up + curl)));
                        put(pl, leg); } }
                }
                Body::Humain if matches!(s.skin, "colosse" | "roinoye" | "gsoleil" | "premroi" | "forgeron") => {
                    // Les boss sculptés : une démarche lourde, l'arme dans la main droite ; ils la lèvent au-dessus de la tête pour
                    // annoncer le coup, puis l'abattent ; la main gauche garde, ou balance.
                    let (pre, hips, sh_x, sh_y, head_y, head_z, leg_x, hunch, weapon) = match s.skin {
                        "colosse" => ("col", 0.92, 0.42, 0.6, 0.66, 0.05, 0.16, 0.22, "colClub"), "roinoye" => ("rny", 0.88, 0.22, 0.55, 0.62, 0.0, 0.1, 0.14, "rnyTrident"),
                        "gsoleil" => ("gso", 0.9, 0.25, 0.56, 0.62, 0.02, 0.11, 0.0, "gsoSpear"), "premroi" => ("prr", 0.88, 0.22, 0.55, 0.62, 0.02, 0.1, 0.06, "prrSword"), _ => ("mfo", 0.84, 0.42, 0.58, 0.66, 0.06, 0.15, 0.1, "mfoHammer") };
                    let n = |k: &str| -> &'static str { match (pre, k) { ("col", "T") => "colTorso", ("col", "H") => "colHead", ("col", "A") => "colArm", ("col", _) => "colLeg", ("rny", "T") => "rnyTorso", ("rny", "H") => "rnyHead", ("rny", "A") => "rnyArm", ("rny", _) => "rnyLeg",
                        ("gso", "T") => "gsoTorso", ("gso", "H") => "gsoHead", ("gso", "A") => "gsoArm", ("gso", _) => "gsoLeg", ("prr", "T") => "prrTorso", ("prr", "H") => "prrHead", ("prr", "A") => "prrArm", ("prr", _) => "prrLeg", (_, "T") => "mfoTorso", (_, "H") => "mfoHead", (_, "A") => "mfoArm", _ => "mfoLeg" } };
                    let t = f.phase; let stride = sw * (0.32 + 0.28 * run); let bob = -(f.phase * 2.0).sin().abs() * 0.03 * (0.4 + run);
                    let lean = hunch + 0.1 * run - wind * 0.25 + strike * 0.45 - flinch * 0.5;
                    let root = mul(&base, &trs(0.0, hips + bob, 0.0, 0.06 * stride, 0.0, 1.0)); let torso = mul(&root, &trs(0.0, 0.0, 0.0, -0.08 * stride, lean, 1.0));
                    put(n("T"), torso); put(n("H"), mul(&torso, &trs(0.0, head_y, head_z, 0.12 * (t * 0.5).sin() * (1.0 - run), -lean * 0.6 + 0.1 * wind, 1.0)));
                    let right = -0.25 - 2.5 * wind + 2.9 * strike + 0.35 * stride; let left = -0.3 * stride - 0.5 * wind - 0.2 * strike;
                    let ar = mul(&torso, &mul(&trs(sh_x, sh_y, 0.0, 0.0, right, 1.0), &roll_z(0.1 + 0.15 * wind))); put(n("A"), ar);
                    put(weapon, mul(&ar, &trs(0.0, -0.62, 0.05, 0.0, -right - lean + 1.25 * strike + 0.15, 1.0)));
                    put(n("A"), mul(&torso, &mul(&trs(-sh_x, sh_y, 0.0, 0.0, left, 1.0), &roll_z(-0.12 - 0.1 * wind))));
                    for sx in [-1.0f64, 1.0] { put(n("L"), mul(&root, &trs(leg_x * sx, 0.0, 0.0, 0.0, sx * stride, 1.0))); }
                }
                Body::Humain if s.skin == "reine" => {
                    // La Reine des glaces glisse sans un pas, la robe ondoyant ; le sceptre dressé dans la main droite ; elle lève la
                    // gauche pour appeler le froid quand elle prépare un coup, puis abat le sceptre ; des éclats tournoient autour d'elle.
                    let t = f.phase; let float = 0.08 + 0.05 * sin(t * 0.8); let lean = 0.06 + 0.1 * run - wind * 0.15 + strike * 0.3 - flinch * 0.4;
                    let root = mul(&base, &trs(0.0, 0.9 + float, 0.0, 0.03 * sin(t * 0.5), 0.0, 1.0)); let torso = mul(&root, &trs(0.0, 0.0, 0.0, 0.0, lean, 1.0));
                    put("rnTorso", torso); put("rnHead", mul(&torso, &trs(0.0, 0.68, 0.01, 0.15 * sin(t * 0.4) * (1.0 - run), -lean * 0.5 + 0.08 * wind, 1.0)));
                    let right = -0.35 - 0.9 * wind + 1.4 * strike; let left = -0.2 + 0.05 * sin(t * 0.7) - 2.2 * wind + 0.6 * strike;
                    let ar = mul(&torso, &mul(&trs(0.19, 0.53, 0.0, 0.0, right, 1.0), &roll_z(0.1))); put("rnArm", ar); put("rnStaff", mul(&ar, &trs(0.0, -0.62, 0.04, 0.0, -right - lean + 0.4 * strike, 1.0)));
                    put("rnArm", mul(&torso, &mul(&trs(-0.19, 0.53, 0.0, 0.0, left, 1.0), &roll_z(-0.12 - 0.5 * wind))));
                    for i in 0..7 { let a = t * (0.5 + 0.9 * wind) + i as f64 * std::f64::consts::TAU / 7.0; let r = 0.95 + 0.15 * sin(t * 0.6 + i as f64) - 0.3 * wind; let h = 0.5 + 0.35 * sin(t * 0.9 + i as f64 * 1.7) + 0.4 * wind;
                        put("rnShard", mul(&base, &trs(cos(a) * r, h + float, sin(a) * r, -a, 0.4 * sin(a * 2.0), 1.0))); }
                }
                Body::Humain => {
                    // La charpente d'homme : le noyé avance bras tendus, l'archer tient son arc à bout de bras et tire la corde, le fantôme
                    // flotte sans jambes, le sylvain à l'affût lève ses bras comme des branches.
                    let (pt, ph, pa, pl) = match s.skin { "archer" => ("arcTorso", "arcHead", "arcArm", "arcLeg"), "fantome" => ("fanTorso", "fanHead", "fanArm", "fanLeg"), "sylvain" => ("sylTorso", "sylHead", "sylArm", "sylLeg"), _ => ("noyTorso", "noyHead", "noyArm", "noyLeg") };
                    let syl = s.skin == "sylvain"; let t = f.phase; let hide = if f.state == Ai::Embuscade { 1.0 } else { 0.0 };
                    let (hips, sh_x, sh_y, head_y, head_z, leg_x) = if syl { (0.75, 0.68, 0.98, 0.98, 0.3, 0.3) } else if s.ghost { (0.86, 0.33, 0.56, 0.6, 0.0, 0.1) } else { (0.86, 0.28, 0.54, 0.6, 0.02, 0.1) };
                    let stride = sw * (0.35 + 0.3 * run); let float = if s.ghost { 0.1 * sin(t * 0.9) + 0.25 } else { -(f.phase * 2.0).sin().abs() * 0.02 * run };
                    let lean = (if s.ghost { 0.2 + 0.2 * run } else if s.drag { 0.22 } else if syl { 0.2 * (1.0 - hide) + 0.1 * run } else { 0.05 + 0.15 * run }) - wind * if s.bow { 0.0 } else { 0.2 } + strike * 0.35 - flinch * 0.5;
                    let root = mul(&base, &trs(0.0, hips + float, 0.0, 0.0, 0.0, 1.0)); let torso = mul(&root, &trs(0.0, 0.0, 0.0, 0.0, lean, 1.0));
                    put(pt, torso); put(ph, mul(&torso, &trs(0.0, head_y, head_z, 0.12 * (t * 0.5).sin() * (1.0 - run) * (1.0 - hide), -lean * 0.5, 1.0)));
                    let aiming = s.bow && (f.wind > 0.0 || f.state == Ai::Combat);
                    for sx in [-1.0f64, 1.0] {
                        let arm = if aiming { if sx > 0.0 { -1.5 } else { -1.2 + 0.5 * wind } } else if s.drag { if sx > 0.0 { -0.55 - 0.9 * wind + 1.2 * strike } else { -1.25 + 0.1 * t.sin() - 0.3 * wind + 0.5 * strike } /* la main du trident, et l'autre qui cherche à saisir */ } else if s.ghost { -0.12 + 0.06 * (t * 0.8 + sx).sin() - 0.5 * wind + 0.5 * strike }
                            else if syl { -2.6 * hide - 0.15 * (1.0 - hide) - 2.3 * wind * (1.0 - hide) + 1.1 * strike - sx * stride * 0.5 * (1.0 - hide) + 0.04 * (t * 0.7 + sx).sin() } else { -sx * stride * 0.8 };
                        let am = mul(&torso, &mul(&trs(sh_x * sx, sh_y, 0.0, 0.0, arm, 1.0), &roll_z(if syl { sx * (0.3 * hide + 0.12) } else if s.ghost { sx * (0.42 + 0.1 * (t * 0.9).sin() + 0.5 * wind) /* les pans du drap pendent sur les côtés, et s'ouvrent quand il frappe */ } else { 0.0 }))); put(pa, am);
                        if s.drag && sx > 0.0 { put("noyTrident", mul(&am, &trs(0.0, -0.62, 0.06, 0.0, -arm - lean + 0.5 * strike, 1.0))); } // le trident reste dressé, et pique quand il frappe
                        if s.bow && sx > 0.0 { put("arcBow", mul(&am, &trs(0.0, -0.62, 0.04, 0.0, if aiming { 1.5 } else { 0.3 }, 1.0))); }
                        if !s.ghost { put(pl, mul(&root, &trs(leg_x * sx, 0.0, 0.0, 0.0, sx * stride * (1.0 - hide) * if syl { 0.5 } else { 1.0 }, 1.0))); }
                    }
                }
                Body::Ver => {
                    // Sous le sable : un sillon qui avance. Il jaillit anneau après anneau, la gueule en haut, se balance, puis replonge.
                    let t = f.phase; let rise = (if f.wind > 0.0 { wind } else if f.up > 0.0 { (f.up / 0.35).min(1.0) } else { 0.0 }) * (1.0 - fall);
                    put("verMound", trs(f.p[0], f.p[1] + if f.speed > 0.0 { 0.03 * (t * 3.0).sin() } else { 0.0 }, f.p[2], f.yaw, 0.0, s.size));
                    if rise > 0.02 { let top = 2.3 * rise - 0.5;
                        for i in 0..6 { let y = top - 0.4 * i as f64; if y < -0.45 { break; } let sway = 0.12 * (t * 0.8 + i as f64 * 0.7).sin() * rise;
                            put(if i == 0 { "verHead" } else { "verSeg" }, trs(f.p[0] + sin(f.yaw) * sway, f.p[1] + y * s.size, f.p[2] + cos(f.yaw) * sway, f.yaw, if i == 0 { 0.25 * rise + 0.2 * strike } else { 0.0 }, s.size)); } }
                }
                Body::Harpie => {
                    // En vol : de grands battements ; en piqué : le corps penché, les ailes à demi repliées, les serres en avant.
                    let dive = if f.air { 1.0 } else { 0.0 }; let flap = if f.dead > 0.0 { 1.2 } else { 0.55 * sin(f.phase) * (1.0 - 0.7 * dive) + 0.5 * dive };
                    let root = mul(&base, &trs(0.0, 0.0, 0.0, 0.0, 0.55 * dive - 0.1, 1.0)); put("hrpBody", root);
                    put("hrpWing", mul(&root, &mul(&trs(0.14, 0.12, 0.0, 0.0, 0.0, 1.0), &roll_z(flap)))); put("hrpWing", mul(&root, &mul(&trs(-0.14, 0.12, 0.0, 0.0, 0.0, 1.0), &roll_z(std::f64::consts::PI - flap))));
                }
                Body::Araignee => {
                    // Huit pattes en deux éventails, qui marchent par paires alternées ; elle se dresse sur l'arrière pour mordre ; morte, pattes recroquevillées.
                    let t = f.phase; let die = fall; let hip = 0.3 - 0.12 * die + if f.speed > 0.0 { (t * 2.0).sin().abs() * 0.015 } else { 0.008 * (t * 1.4).sin() };
                    let rear = -wind * 0.5 + strike * 0.3 + flinch * 0.25;
                    let root = mul(&mul(&trs(f.p[0], f.p[1] + hip * s.size, f.p[2], f.yaw, 0.0, s.size), &roll_z(die * 3.0)), &trs(0.0, 0.0, 0.0, 0.0, rear, 1.0));
                    put("spdBody", root);
                    for (k, z) in [0.3, 0.16, 0.02, -0.12].iter().enumerate() { for sx in [-1.0f64, 1.0] {
                        let alt = if (k + if sx > 0.0 { 1 } else { 0 }) % 2 == 0 { 1.0 } else { -1.0 }; let swing = sw * 0.35 * alt * (0.4 + run); let up = (sw * alt).max(0.0) * 0.14 * (0.3 + run);
                        let spread = [0.85, 0.3, -0.25, -0.8][k] * sx; let reach = if k == 0 { wind * 0.5 } else { 0.0 };
                        let leg = mul(&root, &mul(&trs(0.17 * sx, 0.02, *z, (if sx > 0.0 { 0.0 } else { std::f64::consts::PI }) + spread + swing, 0.0, 1.0), &roll_z(up + reach + die * 1.3)));
                        put("spdLeg", leg); } }
                }
                Body::Golem => {
                    // À l'affût, il est ramassé comme un rocher hérissé de cristaux ; debout, il marche lourdement, lève les deux poings et les abat.
                    let t = f.phase; let hide = if f.state == Ai::Embuscade { 1.0 } else { 0.0 }; let stride = sw * (0.3 + 0.25 * run);
                    let hips = 0.66 * (1.0 - 0.62 * hide) + if f.speed > 0.0 { (t * 2.0).sin().abs() * 0.03 } else { 0.0 };
                    let lean = 0.5 * hide + wind * -0.25 + strike * 0.45 + flinch * 0.2;
                    let root = mul(&base, &trs(0.0, hips, 0.0, 0.0, 0.0, 1.0)); let torso = mul(&root, &trs(0.0, 0.0, 0.0, 0.0, lean, 1.0));
                    put("glmTorso", torso); put("glmHead", mul(&torso, &trs(0.0, 0.88, 0.02, 0.0, 0.35 * hide - 0.15 * wind, 1.0)));
                    for sx in [-1.0f64, 1.0] {
                        let arm = -sx * stride * 0.8 - 2.6 * wind + 0.9 * strike - 0.9 * hide; put("glmArm", mul(&torso, &trs(0.62 * sx, 0.72, 0.0, 0.0, arm, 1.0)));
                        let leg = sx * stride - 1.35 * hide; put("glmLeg", mul(&root, &trs(0.2 * sx, 0.0, 0.0, 0.0, leg, 1.0)));
                    }
                }
            }
        }
        for a in &self.shots { let hz = hypot2(a.1[0], a.1[2]).max(1e-6); out.push(Draw { m: trs(a.0[0], a.0[1], a.0[2], a.1[0].atan2(a.1[2]), -(a.1[1] / hz).atan(), 1.0), r: md.part("arrow"), view: false, local: false, hero: false, dynamic: false, hurt: 0.0 }); }
        // Les braises : elles rétrécissent en s'éteignant.
        for (p, t) in &self.embers { let k = (t / 1.5).min(1.0) * (0.85 + 0.15 * sin(t * 9.0)); out.push(Draw { m: trs(p[0], p[1] + 0.01, p[2], p[0] * 3.1, 0.0, k), r: md.part("ember"), view: false, local: false, hero: false, dynamic: false, hurt: 0.0 }); }
        out
    }
}

// ------------------------------------------------------------------------------------------------------- dans la session
use crate::session::Session;
impl Session {
    /// Le niveau de lumière d'une case (0 à 15) : le ciel s'il est à découvert (faible la nuit), et les lumières posées
    /// à portée (torches, lanternes, feux), qui faiblissent d'un niveau par bloc comme dans le moteur d'éclairage.
    pub fn light_at(&self, x: i32, y: i32, z: i32) -> u8 {
        let day = (self.sun_height() * 3.0 + 0.27).clamp(0.27, 1.0); let mut covered = false;
        for k in 1..48 { if self.world.get(x, y + k, z).is_some_and(is_opaque) { covered = true; break; } }
        let sky = if covered { 0 } else { (15.0 * day).round() as i32 };
        let mut lamp = 0i32;
        let n = CX as i32; let (cx, cz) = (x.div_euclid(n), z.div_euclid(n));
        for dx in -1..=1 { for dz in -1..=1 { for g in self.lamps.get(&(cx + dx, cz + dz)).into_iter().flatten() { let d = (g[0] - x).abs() + (g[1] - y).abs() + (g[2] - z).abs(); if d < 15 { lamp = lamp.max(g[3] - d); } } } }
        sky.max(lamp).clamp(0, 15) as u8
    }
    /// La lumière des seules sources posées (torches, lanternes, feux) en une case, de 0 à 15.
    pub fn lamp_at(&self, x: i32, y: i32, z: i32) -> u8 {
        let n = CX as i32; let (cx, cz) = (x.div_euclid(n), z.div_euclid(n)); let mut lamp = 0i32;
        for dx in -1..=1 { for dz in -1..=1 { for g in self.lamps.get(&(cx + dx, cz + dz)).into_iter().flatten() { let d = (g[0] - x).abs() + (g[1] - y).abs() + (g[2] - z).abs(); if d < 15 { lamp = lamp.max(g[3] - d); } } } }
        lamp.clamp(0, 15) as u8
    }
    /// La région (grille de `region` blocs) et le biome principal du lieu : la clé de l'adaptation.
    pub fn region_key(&self, x: f64, z: f64) -> (i32, i32, u8) { let r = bestiary().rules.region; ((x / r).floor() as i32, (z / r).floor() as i32, self.gen.climate.at(x, z).main() as u8) }
    /// Le danger ici, décomposé (biome, distance, adaptation, structure, événement).
    pub fn danger_here(&self) -> Danger {
        let p = self.player.p; let key = self.region_key(p[0], p[2]); let days = self.regions.get(&key).map_or(0, |r| r.days);
        let biome = biome_danger(key.2 as usize, p[1]); let distance = distance_bonus((p[0] - self.spawn[0]).hypot(p[2] - self.spawn[2])); let adapt = adapt_cut(days).min(distance);
        let event = 0; let structure = 0;
        Danger { biome, distance, adapt, structure, event, total: danger_of(biome, distance, adapt, structure, event), region: key, days }
    }
    /// Une journée validée dans une région : assez de présence active ce jour-là (pas un simple passage).
    fn adapt_step(&mut self, dt: f64) {
        let active = self.player.v[0].hypot(self.player.v[2]) > 0.3 || self.mine_t > 0.0 || self.hit_t > 0.0 || self.inv_open; if !active || self.creative || self.tour.is_some() || self.dead.is_some() { return; }
        let key = self.region_key(self.player.p[0], self.player.p[2]); let today = (self.hours / 24.0).floor() as i64; let need = bestiary().rules.day_secs;
        let r = self.regions.entry(key).or_default(); if r.day != today { if r.secs < 0.0 { r.secs = 0.0; } if r.day < today { r.day = today; r.secs = 0.0; } }
        if r.secs >= 0.0 { r.secs += dt; if r.secs >= need { r.days += 1; r.secs = -1.0; } }
    }
    /// Une image de la vie hostile : adaptation, danger, apparitions, comportements, coups reçus, morts et butin.
    pub(crate) fn hostile_step(&mut self, dt: f64) {
        self.adapt_step(dt); self.danger = self.danger_here();
        self.site_tick();
        let sun = self.sun_height(); let night = ((0.05 - sun) / 0.15).clamp(0.0, 1.0);
        let sp = self.player.v[0].hypot(self.player.v[2]); let mut noise: f64 = if sp > 5.5 { 0.9 } else if sp > 0.5 { 0.45 } else { 0.1 }; if self.mine_t > 0.0 || self.hit_t > 0.0 { noise = noise.max(0.8); } if self.player.crouch { noise *= 0.3; }
        let lit = self.inv[self.sel].is_some_and(|s| matches!(s.item, "torch" | "lantern")) || self.light_at(self.player.p[0].floor() as i32, self.player.p[1].floor() as i32 + 1, self.player.p[2].floor() as i32) >= 10;
        let can_spawn = !self.creative && self.dead.is_none() && self.tour.is_none() && !self.gen.flat;
        let mut horde = std::mem::replace(&mut self.horde, Horde::new(0));
        { let gen = &self.gen; let protect = |x: f64, z: f64| gen.villages_near(x, z, 120.0).iter().any(|v| ((v.c[0] as f64 - x).hypot(v.c[2] as f64 - z)) < v.r as f64 + 10.0);
        let me: &Session = self; let light = |x: i32, y: i32, z: i32| me.light_at(x, y, z);
        let ctx = Ctx { eye: self.player.eye(), feet: self.player.p, look: self.look, noise, lit, crouch: self.player.crouch, alive: self.dead.is_none() && !self.creative, night, danger: self.danger.total, can_spawn, light: &light, protect: &protect, main_biome: self.danger.region.2 as usize, cave: self.cave, under: self.under_kind.map(|k| k.key()) };
        horde.update(dt, &self.world, &ctx); }
        self.horde = horde;
        for e in std::mem::take(&mut self.horde.ev) {
            match e {
                FoeEv::Hit { from, dmg, cause, push } => { self.sound.thump(from); let n = self.block(from, dmg); if n > 0.0 { self.harm(n.round().max(1.0), cause, true);
                    let (dx, dz) = (self.player.p[0] - from[0], self.player.p[2] - from[2]); let d = dx.hypot(dz).max(0.1); self.player.v[0] += dx / d * push; self.player.v[2] += dz / d * push; self.player.v[1] = self.player.v[1].max(2.5); } }
                FoeEv::Growl(p, b) => match b { Body::Bete => self.sound.fox_scream(p), Body::Rampant => self.sound.rustle(p), Body::Cavalier => self.sound.hoof(p, true), _ => self.sound.thump(p) },
                FoeEv::Swing(_) => self.sound.swing(None, 0.8),
                // un coup qu'aucun bouclier n'arrête : seule la roulade l'évite
                FoeEv::Crush { from, dmg, cause, push } => { self.sound.thump(from);
                    // esquivé d'une roulade : l'attaque a porté dans le vide, et il reste à découvert
                    if self.player.rolling() { if let Some(k) = self.horde.nearest(from) { let f = &mut self.horde.foes[k]; if bestiary().species[f.sp].body == Body::Cavalier { f.burst = OPENING; } else { f.cd = f.cd.max(OPENING); } } self.notes.push("Esquivé !".into()); continue; } self.harm(dmg.round().max(1.0), cause, true);
                    if !self.creative && self.dead.is_none() { let (dx, dz) = (self.player.p[0] - from[0], self.player.p[2] - from[2]); let d = dx.hypot(dz).max(0.1); self.player.v[0] += dx / d * push; self.player.v[2] += dz / d * push; self.player.v[1] = self.player.v[1].max(3.0); } }
                FoeEv::Unhorse(p, yaw) => { self.sound.thump(p); self.sound.hoof(p, true); self.notes.push("Le cavalier vide les étriers : il est à terre, frappez !".into());
                    self.wild.put(crate::wild::Kind::Horse, [p[0] + yaw.cos() * 1.5, p[1], p[2] - yaw.sin() * 1.5], yaw, false); let k = self.wild.all.len() - 1; self.wild.strike(k, 0.0, [p[0] - yaw.sin() * 3.0, p[1], p[2] - yaw.cos() * 3.0]); }
                FoeEv::Reel(p) => { self.sound.thump(p); self.notes.push("Sa garde est brisée : il chancelle, frappez !".into()); }
                // la seconde phase : il jette son heaume et appelle deux hommes, qui accourent
                FoeEv::Rage(p, who) => { self.sound.fox_scream(p); self.notes.push(match who { "rodeur" => "Le gardien de la crypte rugit : les morts se lèvent !", "archer" => "Le capitaine siffle : ses archers accourent !", "noye" => "Le colosse englouti gronde : les noyés remontent des eaux noires !", _ => "Le chef de guerre entre en rage et appelle ses hommes !" }.into());
                    if let Some(sp) = species(who) { let me = self.player.p; for k in 0..2 { let a = k as f64 * 2.4 + 1.0; let (x, z) = (p[0] + a.cos() * 6.0, p[2] + a.sin() * 6.0);
                        let y = ground(&self.world, x, z, p[1] + 3.0).unwrap_or(p[1]); let i = self.horde.put(sp, [x, y, z], (me[0] - x).atan2(me[2] - z), true); let f = &mut self.horde.foes[i]; f.kept = true; f.alert = 1.0; f.seen = Some(me); f.state = Ai::Poursuite; } } }
                FoeEv::Loot(_, g) => self.raid_loot(g),
                FoeEv::Snare(t) => { if self.snare <= 0.0 { self.notes.push("Englué dans la soie : vous êtes ralenti".into()); } self.snare = self.snare.max(t); }
                FoeEv::Blocked(p) => { self.sound.thump(p); if (p[0] - self.player.p[0]).hypot(p[2] - self.player.p[2]) < 8.0 { self.notes.push("Bloqué par son bouclier".into()); } }
                FoeEv::Die(p, sp, vet, camp) => { let s = &bestiary().species[sp]; self.sound.thump(p); self.skills[2] += 2.0; if (p[0] - self.player.p[0]).hypot(p[2] - self.player.p[2]) < 32.0 { self.earn(Self::kill_xp(sp, vet), &format!("{} vaincu", s.name)); } /* près de soi : on y a pris part */ if let Some(c) = camp { self.camp_kill(c); } if self.creative { continue; }
                    for &(id, lo, hi, pr) in &s.loot { let roll = self.horde.roll(); let pr = if vet { (pr * 1.3).min(1.0) } else { pr }; if roll < pr { let n = lo + (self.horde.roll() * (hi - lo + 1) as f64) as u32; self.spill(id, n.min(hi).max(lo), [p[0], p[1] + 0.6, p[2]]); } } }
            }
        }
    }
    pub(crate) fn regions_save(&self) -> String { self.regions.iter().filter(|(_, r)| r.days > 0 || r.secs > 0.0).map(|(k, r)| format!("region {} {} {} {} {:.0} {}\n", k.0, k.1, k.2, r.days, r.secs, r.day)).collect() }
    pub(crate) fn region_load(&mut self, a: &[&str]) {
        let n = |i: usize| a.get(i).and_then(|x| x.parse::<f64>().ok()); let (Some(x), Some(z), Some(b), Some(d)) = (n(0), n(1), n(2), n(3)) else { return };
        self.regions.insert((x as i32, z as i32, b as u8), Region { days: d as u32, secs: n(4).unwrap_or(0.0), day: n(5).unwrap_or(0.0) as i64 });
    }
    /// Panneau de développement (F3) : danger, région, adaptation, créatures et leurs états.
    pub fn hostile_report(&self) -> Vec<String> {
        let d = self.danger; let p = self.player.p; let l = self.light_at(p[0].floor() as i32, p[1].floor() as i32 + 1, p[2].floor() as i32);
        let need = bestiary().rules.day_secs; let today = self.regions.get(&d.region).map_or(0.0, |r| if r.secs < 0.0 { need } else { r.secs });
        let mut v = vec![format!("Danger {} = biome {} + max(0, distance {} − adaptation {}) + lieu {} + événement {}", d.total, d.biome, d.distance, d.adapt, d.structure, d.event),
            format!("Région {},{} · {} · {} journée(s) validée(s), aujourd'hui {:.0}/{:.0} s · lumière ici {}", d.region.0, d.region.1, crate::biome::NAMES[d.region.2 as usize], d.days, today, need, l),
            format!("Créatures {} actives / plafond {} · nées depuis le début {}", self.horde.alive(), bestiary().rules.cap, self.horde.spawned)];
        v.extend(self.horde.debug_lines()); v
    }
    /// La barre du boss qui vous combat (en haut de l'écran) : son nom, sa vie, sa garde entamée (0 à 1), et s'il chancelle.
    pub fn boss_bar(&self) -> Option<(&'static str, f64, f64, bool)> {
        let me = self.player.p;
        self.horde.foes.iter().filter(|f| f.dead == 0.0 && bestiary().species[f.sp].boss && matches!(f.state, Ai::Poursuite | Ai::Combat) && (f.p[0] - me[0]).hypot(f.p[2] - me[2]) < 32.0)
            .min_by(|a, b| (a.p[0] - me[0]).hypot(a.p[2] - me[2]).total_cmp(&(b.p[0] - me[0]).hypot(b.p[2] - me[2])))
            .map(|f| (bestiary().species[f.sp].name, (f.hp / f.max).clamp(0.0, 1.0), if f.broken > 0.0 { 1.0 } else { (f.posture / POSTURE).clamp(0.0, 1.0) }, f.broken > 0.0))
    }
    /// Outil de développement : fait naître une créature devant soi (l'espèce suivante à chaque appel).
    pub fn debug_spawn(&mut self, k: usize) -> &'static str {
        let n = bestiary().species.len(); if n == 0 { return ""; } let sp = k % n; let f = self.look; let p = self.player.p;
        let (x, z) = (p[0] + f[0] * 6.0, p[2] + f[2] * 6.0); let y = ground(&self.world, x, z, p[1] + 3.0).unwrap_or(p[1]);
        let i = self.horde.put(sp, [x, y, z], (p[0] - x).atan2(p[2] - z), false); if bestiary().species[sp].ambush { self.horde.foes[i].state = Ai::Embuscade; } bestiary().species[sp].name
    }
}
