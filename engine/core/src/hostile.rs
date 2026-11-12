//! Les créatures hostiles de la nuit : registre des espèces (lu dans `data/creatures.txt`), apparition dans le noir,
//! comportement (repos, patrouille, soupçon, détection, poursuite, combat, recherche, retour, fuite, embuscade),
//! combat aux attaques annoncées, mort et butin. Un seul système pour toutes les espèces : ce qui les distingue
//! (corps, chiffres, talents, butin) vient des données.
use crate::actors::{b, Draw, Models};
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
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Body { Rodeur, Brigand, Bete, Rampant }
#[derive(Clone, Debug)]
pub struct Species {
    pub id: &'static str, pub name: &'static str, pub tier: u8, pub body: Body, pub size: f64, pub hp: f64, pub dmg: f64, pub walk: f64, pub run: f64,
    pub reach: f64, pub sight: f64, pub hear: f64, pub cd: f64, pub windup: f64, pub armor: f64, pub weak: &'static str,
    /// Biomes où elle vit (indices de `biome::NAMES`) ; vide : partout.
    pub biomes: Vec<usize>, pub weight: f64, pub group: (u32, u32), pub flee: f64,
    pub leap: bool, pub climb: bool, pub ambush: bool, pub bursts: bool, pub flank: bool,
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
    let mut r = Rules { light: 7, ring: (24.0, 46.0), forget: 76.0, cap: 10, cap_near: 6, every: 0.7, distance: vec![(250.0, 1), (750.0, 2), (1500.0, 3), (3000.0, 4)], biome: [2, 1, 2, 2, 3, 3, 4], mountain: (118.0, 5), adapt: vec![(5, 1), (10, 2), (20, 3), (30, 4)], day_secs: 240.0, region: 256.0 };
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
        let mut s = Species { id: f[0], name: f[1], tier: 1, body: Body::Rodeur, size: 1.0, hp: 25.0, dmg: 6.0, walk: 1.0, run: 4.5, reach: 1.5, sight: 16.0, hear: 10.0, cd: 1.6, windup: 0.5, armor: 0.0, weak: "", biomes: Vec::new(), weight: 1.0, group: (1, 1), flee: 0.0, leap: false, climb: false, ambush: false, bursts: false, flank: false, loot: Vec::new() };
        for kv in f[2].split_whitespace() { let Some((k, v)) = kv.split_once('=') else { continue }; let x = v.parse::<f64>().unwrap_or(0.0);
            match k {
                "niveau" => s.tier = x as u8, "corps" => s.body = match v { "brigand" => Body::Brigand, "bete" => Body::Bete, "rampant" => Body::Rampant, _ => Body::Rodeur },
                "taille" => s.size = x, "pv" => s.hp = x, "degats" => s.dmg = x, "marche" => s.walk = x, "course" => s.run = x, "portee" => s.reach = x, "vue" => s.sight = x, "ouie" => s.hear = x,
                "recharge" => s.cd = x, "armement" => s.windup = x, "armure" => s.armor = x.clamp(0.0, 0.8), "faible" => s.weak = v, "poids" => s.weight = x, "fuite" => s.flee = x,
                "groupe" => { let (a, b) = v.split_once('-').unwrap_or((v, v)); s.group = (a.parse().unwrap_or(1), b.parse().unwrap_or(1)); }
                "biomes" => if v != "tous" { s.biomes = v.split(',').filter_map(|n| crate::biome::NAMES.iter().position(|b| b.replace(' ', "_").eq_ignore_ascii_case(n) || *b == n)).collect(); },
                "talents" => for t in v.split(',') { match t { "bond" => s.leap = true, "grimpe" => s.climb = true, "embuscade" => s.ambush = true, "poussees" => s.bursts = true, "contourne" => s.flank = true, _ => {} } },
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
    pub wind: f64, pub hit: f64, pub phase: f64, pub speed: f64, v: [f64; 3], gy: f64,
    /// Où elle croit le joueur, et depuis quand elle ne l'a plus vu ; l'éveil monte avec ce qu'elle voit et entend.
    pub seen: Option<[f64; 3]>, lost: f64, pub alert: f64, home: [f64; 3],
    /// Mort en cours (s) ; variante d'équipement ; vétéran (un peu plus dur, meilleur butin) ; bond ou course en cours.
    pub dead: f64, pub variant: u8, pub vet: bool, pub air: bool, burst: f64, pub climbing: bool, pub group: u32,
}
/// Ce que font les créatures, pour la session (coups, sons, butin).
#[derive(Clone, Debug)] pub enum FoeEv { Hit { from: [f64; 3], dmg: f64, cause: &'static str, push: f64 }, Growl([f64; 3], Body), Swing([f64; 3]), Die([f64; 3], usize, bool) }

pub struct Horde { pub foes: Vec<Foe>, rng: Rng, spawn_t: f64, pub ev: Vec<FoeEv>, groups: u32, pub spawned: u32 }
/// Ce que la session donne à la horde à chaque image.
pub struct Ctx<'a> {
    pub eye: [f64; 3], pub feet: [f64; 3], pub look: [f64; 3], pub noise: f64, pub lit: bool, pub crouch: bool, pub alive: bool,
    pub night: f64, pub danger: i32, pub can_spawn: bool, pub light: &'a dyn Fn(i32, i32, i32) -> u8, pub protect: &'a dyn Fn(f64, f64) -> bool, pub main_biome: usize,
}

fn sees(w: &impl Voxels, a: [f64; 3], b: [f64; 3]) -> bool {
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]]; let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt(); let n = (l / 0.45).ceil() as i32;
    for i in 1..n { let t = i as f64 / n as f64; let q = [a[0] + d[0] * t, a[1] + d[1] * t, a[2] + d[2] * t]; if w.get(q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32).is_none_or(is_opaque) { return false; } } true
}
fn eye_h(b: Body) -> f64 { match b { Body::Rodeur => 1.45, Body::Brigand => 1.62, Body::Bete => 0.8, Body::Rampant => 0.45 } }
/// Boîte de visée et de collision (demi-largeur, hauteur).
pub fn foe_box(b: Body, size: f64) -> (f64, f64) { let (w, h) = match b { Body::Rodeur => (0.36, 1.7), Body::Brigand => (0.36, 1.85), Body::Bete => (0.5, 1.0), Body::Rampant => (0.6, 0.6) }; (w * size, h * size) }

impl Horde {
    pub fn new(seed: i32) -> Self { Horde { foes: Vec::new(), rng: Rng::new((seed ^ 0x6d21) as u32), spawn_t: 2.0, ev: Vec::new(), groups: 0, spawned: 0 } }
    /// Fait apparaître une créature de l'espèce `sp` en `p` (bancs d'essai, outils de développement, vagues).
    pub fn put(&mut self, sp: usize, p: [f64; 3], yaw: f64, vet: bool) -> usize {
        let s = &bestiary().species[sp]; let max = s.hp * if vet { 1.25 } else { 1.0 }; self.groups += 1; let variant = (self.rng.next() * 3.0) as u8;
        self.foes.push(Foe { sp, p, yaw, hp: max, max, state: if s.ambush { Ai::Embuscade } else { Ai::Repos }, t: self.rng.range(1.0, 4.0), cd: 0.5, wind: 0.0, hit: 0.0, phase: 0.0, speed: 0.0, v: [0.0; 3], gy: p[1],
            seen: None, lost: 0.0, alert: 0.0, home: p, dead: 0.0, variant, vet, air: false, burst: 0.0, climbing: false, group: self.groups });
        self.spawned += 1; self.foes.len() - 1
    }
    /// Un coup porté : moins fort sur une armure, plus fort avec l'arme qui la blesse le mieux ; rend vrai si elle meurt.
    pub fn strike(&mut self, i: usize, dmg: f64, from: [f64; 3], weapon: &str, heavy: bool) -> bool {
        let Some(f) = self.foes.get_mut(i) else { return false }; if f.dead > 0.0 { return false; } let s = &bestiary().species[f.sp];
        let mul = if !s.weak.is_empty() && s.weak == weapon { 1.35 } else { 1.0 }; f.hp -= dmg * (1.0 - s.armor) * mul; f.hit = 0.3;
        let (dx, dz) = (f.p[0] - from[0], f.p[2] - from[2]); let d = dx.hypot(dz).max(0.1); let k = if heavy { 6.5 } else { 3.2 } / s.size; f.v = [dx / d * k, 0.0, dz / d * k];
        // Un coup interrompt l'attaque annoncée (le joueur qui frappe le premier garde l'avantage) ; elle sait maintenant où il est.
        if heavy || f.wind > s.windup * 0.5 { f.wind = 0.0; f.cd = f.cd.max(0.6); }
        f.seen = Some(from); f.alert = 1.0; f.lost = 0.0; if matches!(f.state, Ai::Repos | Ai::Patrouille | Ai::Soupcon | Ai::Embuscade | Ai::Retour | Ai::Recherche) { f.state = Ai::Poursuite; }
        if f.hp <= 0.0 { f.dead = 1e-3; f.wind = 0.0; self.ev.push(FoeEv::Die(f.p, f.sp, f.vet)); return true; }
        if s.flee > 0.0 && f.hp < f.max * s.flee { f.state = Ai::Fuite; f.t = 7.0; }
        false
    }
    /// Repoussée (parade, coup lourd) : elle recule et perd son attaque.
    pub fn stagger(&mut self, i: usize, from: [f64; 3], secs: f64) { if let Some(f) = self.foes.get_mut(i) { let (dx, dz) = (f.p[0] - from[0], f.p[2] - from[2]); let d = dx.hypot(dz).max(0.1); f.v = [dx / d * 6.0, 0.0, dz / d * 6.0]; f.wind = 0.0; f.cd = f.cd.max(secs); f.hit = 0.3; } }
    /// La créature vivante la plus proche d'un point (celle qui vient de frapper).
    pub fn nearest(&self, p: [f64; 3]) -> Option<usize> { self.foes.iter().enumerate().filter(|(_, f)| f.dead == 0.0).map(|(i, f)| (i, (f.p[0] - p[0]).hypot(f.p[2] - p[2]))).filter(|x| x.1 < 3.5).min_by(|a, b| a.1.total_cmp(&b.1)).map(|x| x.0) }
    /// Un tirage (butin, vagues).
    pub fn roll(&mut self) -> f64 { self.rng.next() }
    pub fn alive(&self) -> usize { self.foes.iter().filter(|f| f.dead == 0.0).count() }

    /// Choisit l'espèce à faire naître ici : les communes partout, celles du biome en plus ; le danger élève parfois la créature en vétéran.
    fn pick(&mut self, biome: usize, danger: i32) -> Option<(usize, bool)> {
        let bs = bestiary(); let tier_w = |t: u8| match t { 1 => 1.0, 2 => (danger - 2).max(0) as f64 * 0.22, 3 => (danger - 4).max(0) as f64 * 0.12, _ => (danger - 5).max(0) as f64 * 0.03 };
        let c: Vec<(usize, f64)> = bs.species.iter().enumerate().filter(|(_, s)| s.biomes.is_empty() || s.biomes.contains(&biome)).map(|(i, s)| (i, s.weight * tier_w(s.tier))).filter(|x| x.1 > 0.0).collect();
        let sum: f64 = c.iter().map(|x| x.1).sum(); if sum <= 0.0 { return None; } let mut r = self.rng.next() * sum;
        let sp = c.iter().find(|x| { r -= x.1; r <= 0.0 }).unwrap_or(&c[c.len() - 1]).0; let vet = self.rng.next() < ((danger - 2).max(0) as f64 * 0.07).min(0.45); Some((sp, vet))
    }
    /// Une tentative d'apparition, loin des yeux du joueur, dans le noir, sur un sol où le corps tient, hors des lieux protégés.
    fn try_spawn(&mut self, w: &impl Voxels, c: &Ctx) {
        let r = &bestiary().rules; let me = c.feet;
        if self.foes.len() >= r.cap { return; }
        let near = self.foes.iter().filter(|f| hypot2(f.p[0] - me[0], f.p[2] - me[2]) < 24.0).count(); if near >= r.cap_near { return; }
        let Some((sp, vet)) = self.pick(c.main_biome, c.danger) else { return }; let s = &bestiary().species[sp];
        for _ in 0..4 {
            let a = self.rng.range(0.0, std::f64::consts::TAU); let d = self.rng.range(r.ring.0, r.ring.1); let (x, z) = (me[0] + sin(a) * d, me[2] + cos(a) * d);
            if (c.protect)(x, z) { continue; }
            let Some(y) = ground(w, x, z, me[1] + 12.0).or_else(|| ground(w, x, z, me[1] - 8.0)) else { continue }; if (y - me[1]).abs() > 24.0 { continue; }
            let (bx, by, bz) = (x.floor() as i32, y.floor() as i32, z.floor() as i32); if (c.light)(bx, by, bz) >= r.light { continue; }
            // Jamais sous les yeux du joueur : hors de son champ, ou cachée par le relief.
            let (dx, dy, dz) = (x - c.eye[0], y + 1.0 - c.eye[1], z - c.eye[2]); let l = (dx * dx + dy * dy + dz * dz).sqrt().max(0.1);
            let ahead = (dx * c.look[0] + dy * c.look[1] + dz * c.look[2]) / l; if ahead > 0.35 && sees(w, c.eye, [x, y + 1.0, z]) { continue; }
            let yaw = self.rng.range(0.0, 6.283); let tall = (foe_box(s.body, s.size).1.ceil() as i32).max(1); if !fits(w, x, y, z, yaw, tall) { continue; }
            let n = self.rng.range(s.group.0 as f64, s.group.1 as f64 + 0.999).floor() as u32 + if c.danger >= 5 && self.rng.next() < 0.3 { 1 } else { 0 };
            let first = self.put(sp, [x, y, z], yaw, vet); let g = self.foes[first].group;
            for k in 1..n { let o = [x + self.rng.range(-2.5, 2.5), y, z + self.rng.range(-2.5, 2.5)]; if let Some(oy) = ground(w, o[0], o[2], y + 2.0) { if (oy - y).abs() <= 2.0 && fits(w, o[0], oy, o[2], yaw, tall) { let i = self.put(sp, [o[0], oy, o[2]], yaw + k as f64, vet && k == 0); self.foes[i].group = g; } } }
            return;
        }
    }

    pub fn update(&mut self, dt: f64, w: &impl Voxels, c: &Ctx) {
        let r = &bestiary().rules; let me = c.feet;
        if c.can_spawn { self.spawn_t -= dt; if self.spawn_t <= 0.0 { self.spawn_t = r.every * self.rng.range(0.7, 1.3); if c.night > 0.5 || (c.light)(me[0].floor() as i32, me[1].floor() as i32 + 1, me[2].floor() as i32) < 4 { self.try_spawn(w, c); } } }
        let mut all = std::mem::take(&mut self.foes); let n = all.len();
        let groups: Vec<(u32, [f64; 3])> = all.iter().map(|f| (f.group, f.p)).collect();
        for i in 0..n { let f = &mut all[i]; self.think(f, dt, w, c, &groups); }
        // Au lever du jour, celles qui ne se battent pas s'éloignent et s'effacent hors de vue ; les mortes, une fois tombées ; les trop lointaines.
        let day = c.night < 0.2;
        all.retain(|f| { let d = hypot2(f.p[0] - me[0], f.p[2] - me[2]); !(f.dead > 1.6 || d > r.forget || (day && f.state != Ai::Combat && d > 20.0 && f.dead == 0.0) || f.p[1] < -20.0 || w.get(f.p[0].floor() as i32, f.p[1].floor() as i32, f.p[2].floor() as i32).is_none()) });
        self.foes = all;
    }

    fn think(&mut self, f: &mut Foe, dt: f64, w: &impl Voxels, c: &Ctx, groups: &[(u32, [f64; 3])]) {
        let s = &bestiary().species[f.sp]; let r = &mut self.rng; f.hit = (f.hit - dt).max(0.0);
        if f.dead > 0.0 { f.dead += dt; f.speed = 0.0; settle(f, dt, w); return; }
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
                f.yaw = turn(f.yaw, want, dt * 6.0);
                // Le rôdeur traîne, puis accélère d'un coup ; la bête fond ; les autres courent.
                let run = if s.bursts { if f.burst > 0.0 { s.run } else { if r.next() < dt * 0.35 { f.burst = r.range(1.2, 2.0); } s.walk * 1.6 } } else { s.run };
                f.speed = run * if f.vet { 1.08 } else { 1.0 };
                if see && dist < s.reach + 0.05 && dy.abs() < 2.2 { f.state = Combat; f.speed = 0.0; }
                if s.leap && see && f.cd <= 0.0 && !f.air && (3.0..6.5).contains(&dist) && dy.abs() < 1.5 && r.next() < dt * 3.0 { f.air = true; f.v = [to[0] * 9.5, 5.5, to[1] * 9.5]; f.wind = 0.6; f.cd = s.cd + 1.0; self.ev.push(FoeEv::Growl(f.p, s.body)); }
                if !see && f.lost > 4.0 { f.state = Recherche; f.t = 8.0; }
                if !c.alive { f.state = Retour; }
            }
            Combat => {
                f.speed = 0.0; f.yaw = turn(f.yaw, (me[0] - f.p[0]).atan2(me[2] - f.p[2]), dt * if f.wind > 0.0 { 2.0 } else { 7.0 });
                // L'attaque s'annonce (bras levé, gueule ouverte, corps ramassé), puis tombe : esquivable, parable.
                if f.wind <= 0.0 && f.cd <= 0.0 { f.wind = s.windup; self.ev.push(FoeEv::Swing(f.p)); }
                if dist > s.reach + 1.2 && f.wind <= 0.0 { f.state = Poursuite; }
                if !c.alive { f.state = Retour; }
            }
            Recherche => { if let Some(q) = f.seen { if hypot2(q[0] - f.p[0], q[2] - f.p[2]) > 1.5 { f.yaw = turn(f.yaw, (q[0] - f.p[0]).atan2(q[2] - f.p[2]), dt * 4.0); f.speed = s.walk * 1.5; } else { f.speed = 0.0; f.yaw += dt * 1.2; } }
                if see && f.alert > 0.6 { f.state = Poursuite; } else if f.t <= 0.0 { f.state = Retour; f.seen = None; } }
            Retour => { f.speed = s.walk; f.yaw = turn(f.yaw, (f.home[0] - f.p[0]).atan2(f.home[2] - f.p[2]), dt * 3.0); if hypot2(f.p[0] - f.home[0], f.p[2] - f.home[2]) < 2.0 { f.state = Repos; f.t = r.range(2.0, 5.0); } if f.alert > 0.6 && see { f.state = Poursuite; } }
            Fuite => { f.speed = s.run * 0.9; f.yaw = turn(f.yaw, (f.p[0] - me[0]).atan2(f.p[2] - me[2]), dt * 6.0); if f.t <= 0.0 { f.state = Retour; f.home = f.p; } }
        }
        // ---- le coup tombe à la fin de l'annonce, s'il y a contact (distance et angle cohérents avec le geste)
        if f.wind > 0.0 { f.wind -= dt; if f.wind <= 0.0 { f.wind = 0.0; f.cd = s.cd * if f.vet { 0.9 } else { 1.0 };
            let face = sin(f.yaw) * to[0] + cos(f.yaw) * to[1]; let reach = s.reach + if f.air { 0.8 } else { 0.5 };
            if c.alive && dist < reach && face > 0.35 && dy.abs() < 1.9 { let dmg = s.dmg * if f.vet { 1.15 } else { 1.0 }; self.ev.push(FoeEv::Hit { from: f.p, dmg, cause: cause(s.body), push: if s.body == Body::Bete { 4.0 } else { 2.5 } }); } } }
        // ---- le mouvement : marcher sur le relief voxel, contourner, monter et descendre d'un bloc, grimper (rampant), bondir (bête)
        if f.state != Combat && f.state != Detection { f.phase += dt * (2.0 + f.speed * 1.8); } else { f.phase += dt * 1.5; }
        if f.air { f.p[0] += f.v[0] * dt; f.p[2] += f.v[2] * dt; f.v[1] -= 22.0 * dt; f.p[1] += f.v[1] * dt;
            let blocked = w.get(f.p[0].floor() as i32, (f.p[1] + 0.5).floor() as i32, f.p[2].floor() as i32).is_none_or(|b| b != 0 && is_solid(b)); if blocked { f.v[0] = 0.0; f.v[2] = 0.0; }
            if let Some(g) = ground(w, f.p[0], f.p[2], f.p[1] + 1.0) { if f.v[1] < 0.0 && f.p[1] <= g { f.p[1] = g; f.gy = g; f.air = false; f.v = [0.0; 3]; if dist < s.reach + 1.0 { f.state = Combat; } } } else if f.v[1] < -20.0 { f.air = false; }
            return; }
        knock(f, dt, w);
        if f.speed > 0.0 { let step = f.speed * dt; if !walk(f, w, step, s.climb, s.body) && matches!(f.state, Patrouille | Retour) { f.yaw += r.range(1.0, 2.5); } }
        settle(f, dt, w);
    }
    pub fn debug_lines(&self) -> Vec<String> {
        self.foes.iter().take(8).map(|f| { let s = &bestiary().species[f.sp]; format!("  {}{} — {} ({:.0}/{:.0} pv, éveil {:.1})", s.name, if f.vet { " (vétéran)" } else { "" }, if f.dead > 0.0 { "morte" } else { f.state.name() }, f.hp.max(0.0), f.max, f.alert) }).collect()
    }
}
fn cause(b: Body) -> &'static str { match b { Body::Rodeur => "Les griffes d'un rôdeur décharné", Body::Brigand => "L'arme d'un brigand", Body::Bete => "Les crocs d'une bête rôdeuse", Body::Rampant => "Les mandibules d'un rampant nocturne" } }
fn turn(a: f64, b: f64, k: f64) -> f64 { let d = (b - a + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI; a + d.clamp(-k, k) }
fn knock(f: &mut Foe, dt: f64, w: &impl Voxels) {
    if f.v[0] == 0.0 && f.v[2] == 0.0 { return; } let (nx, nz) = (f.p[0] + f.v[0] * dt, f.p[2] + f.v[2] * dt);
    if let Some(y) = ground(w, nx, nz, f.gy) { if (y - f.gy).abs() <= 1.0 { f.p[0] = nx; f.p[2] = nz; f.gy = y; } }
    let k = (1.0 - dt * 7.0).max(0.0); f.v[0] *= k; f.v[2] *= k; if f.v[0].abs() + f.v[2].abs() < 0.05 { f.v[0] = 0.0; f.v[2] = 0.0; }
}
fn settle(f: &mut Foe, dt: f64, w: &impl Voxels) { if let Some(g) = ground(w, f.p[0], f.p[2], f.p[1] + 1.0) { if !f.climbing { f.gy = g; } } f.p[1] += (f.gy - f.p[1]) * (dt * 10.0).min(1.0); }
/// Avancer : tout droit, sinon le cap libre le plus proche ; une marche d'un bloc se franchit ; le rampant grimpe un mur de quatre blocs.
fn walk(f: &mut Foe, w: &impl Voxels, step: f64, climb: bool, body: Body) -> bool {
    let tall = match body { Body::Rampant => 1, Body::Bete => 1, _ => 2 };
    for k in [0.0, 0.45, -0.45, 0.9, -0.9, 1.35, -1.35, 1.8, -1.8] {
        let yaw = f.yaw + k; let (nx, nz) = (f.p[0] + sin(yaw) * step, f.p[2] + cos(yaw) * step);
        if let Some(y) = ground(w, nx, nz, f.gy + 1.0) { if y - f.gy <= 1.05 && f.gy - y <= 3.0 && fits(w, nx, y, nz, yaw, tall) { f.p[0] = nx; f.p[2] = nz; f.gy = y; f.climbing = false; if k != 0.0 { f.yaw += k * 0.5; } return true; } }
        if climb && k == 0.0 { if let Some(y) = ground(w, nx, nz, f.gy + 4.5) { if y - f.gy > 1.0 && y - f.gy <= 4.5 && fits(w, nx, y, nz, yaw, tall) {
            // Le rampant monte le long du mur, puis passe dessus.
            f.climbing = true; f.p[1] += 2.6 * step / f.speed.max(0.1) * f.speed.min(3.0); if f.p[1] >= y { f.p = [nx, y, nz]; f.gy = y; f.climbing = false; } return true; } } }
    }
    false
}

// ----------------------------------------------------------------------------------------------------------- modèles
const SKIN: [f64; 3] = [0.43, 0.46, 0.37]; const SKIN2: [f64; 3] = [0.35, 0.38, 0.3]; const RAG: [f64; 3] = [0.3, 0.24, 0.17]; const RAG2: [f64; 3] = [0.2, 0.16, 0.12]; const RUST: [f64; 3] = [0.46, 0.26, 0.13];
const EYE: [f64; 3] = [1.0, 0.94, 0.55]; const BONE: [f64; 3] = [0.82, 0.78, 0.66]; const HIDE: [f64; 3] = [0.19, 0.17, 0.15]; const HIDE2: [f64; 3] = [0.3, 0.26, 0.2]; const RED_EYE: [f64; 3] = [1.0, 0.42, 0.18];
const CHIT: [f64; 3] = [0.21, 0.16, 0.23]; const CHIT2: [f64; 3] = [0.34, 0.28, 0.35]; const SPOT: [f64; 3] = [0.6, 0.62, 0.48]; const GEYE: [f64; 3] = [0.72, 1.0, 0.6]; const LEATHER: [f64; 3] = [0.36, 0.24, 0.14];
pub fn parts() -> Vec<(&'static str, VBuf)> {
    let mut o: Vec<(&'static str, VBuf)> = Vec::new(); let mut m = |n: &'static str, f: &dyn Fn(&mut VBuf)| { let mut v = VBuf::new(600); f(&mut v); o.push((n, v)); };
    // Rôdeur décharné : un torse maigre, voûté, des côtes sous la peau grise, des haillons, une épaulière rouillée d'un seul côté.
    m("rodTorso", &|q| { b(q, [-0.2, 0.0, -0.12, 0.2, 0.52, 0.12], SKIN); for y in [0.22, 0.3, 0.38] { b(q, [-0.19, y, 0.11, 0.19, y + 0.03, 0.13], SKIN2); }
        b(q, [-0.21, -0.12, -0.13, 0.21, 0.14, 0.13], RAG); b(q, [-0.2, -0.26, -0.11, 0.05, -0.1, 0.12], RAG2); b(q, [0.06, -0.2, -0.1, 0.2, -0.1, 0.1], RAG); b(q, [-0.08, 0.3, -0.135, 0.18, 0.5, -0.11], RAG2);
        b(q, [0.12, 0.42, -0.16, 0.34, 0.6, 0.16], RUST); b(q, [0.14, 0.4, -0.17, 0.32, 0.43, 0.17], [0.32, 0.18, 0.1]); b(q, [-0.07, 0.5, -0.07, 0.07, 0.6, 0.07], SKIN2); });
    // La tête enveloppée de bandes, un seul œil pâle qui luit, la mâchoire nue et longue.
    m("rodHead", &|q| { b(q, [-0.14, 0.0, -0.15, 0.14, 0.3, 0.15], RAG); b(q, [-0.15, 0.18, -0.16, 0.15, 0.24, 0.16], RAG2); b(q, [-0.15, 0.08, -0.16, 0.0, 0.12, 0.16], RAG2);
        b(q, [0.03, 0.12, 0.14, 0.11, 0.2, 0.165], [0.1, 0.09, 0.08]); b(q, [0.05, 0.14, 0.155, 0.09, 0.18, 0.17], EYE); b(q, [-0.1, -0.06, 0.02, 0.1, 0.04, 0.17], SKIN2); for x in [-0.07, -0.02, 0.03, 0.07] { b(q, [x, -0.02, 0.165, x + 0.025, 0.02, 0.175], BONE); }
        b(q, [-0.16, 0.24, -0.1, -0.12, 0.34, 0.0], RAG2); });
    // Bras long et fin, avant-bras bandé, longues mains griffues.
    m("rodArm", &|q| { b(q, [-0.06, -0.42, -0.06, 0.06, 0.04, 0.06], SKIN); b(q, [-0.065, -0.4, -0.065, 0.065, -0.26, 0.065], RAG2); b(q, [-0.055, -0.78, -0.055, 0.055, -0.42, 0.055], SKIN2);
        b(q, [-0.07, -0.72, -0.07, 0.07, -0.56, 0.07], RAG); for x in [-0.045, -0.01, 0.025] { b(q, [x, -0.95, -0.015, x + 0.022, -0.78, 0.04], BONE); } });
    m("rodLeg", &|q| { b(q, [-0.07, -0.42, -0.07, 0.07, 0.0, 0.07], RAG2); b(q, [-0.055, -0.8, -0.055, 0.055, -0.42, 0.055], SKIN2); b(q, [-0.065, -0.66, -0.065, 0.065, -0.5, 0.065], RAG); b(q, [-0.06, -0.84, -0.05, 0.06, -0.78, 0.16], SKIN); });
    // Brigand errant : jaque de cuir, ceinture, capuchon (trois teintes), foulard sur le bas du visage, armes rudimentaires.
    for (i, hood) in [[0.24, 0.27, 0.18], [0.36, 0.17, 0.12], [0.2, 0.2, 0.22]].into_iter().enumerate() {
        let tn: &'static str = ["briTorso0", "briTorso1", "briTorso2"][i]; let hn: &'static str = ["briHead0", "briHead1", "briHead2"][i];
        m(tn, &move |q| { b(q, [-0.24, 0.0, -0.14, 0.24, 0.58, 0.14], LEATHER); b(q, [-0.25, -0.02, -0.15, 0.25, 0.06, 0.15], [0.16, 0.11, 0.07]); b(q, [-0.04, 0.0, 0.15, 0.04, 0.06, 0.16], [0.6, 0.55, 0.4]);
            for y in [0.16, 0.3, 0.44] { b(q, [-0.01, y, 0.14, 0.01, y + 0.08, 0.15], [0.2, 0.13, 0.08]); } b(q, [-0.26, 0.42, -0.18, 0.26, 0.62, 0.17], hood); b(q, [-0.22, -0.24, -0.13, 0.22, -0.02, 0.13], [0.22, 0.2, 0.18]);
            b(q, [0.1, -0.12, 0.12, 0.2, 0.0, 0.16], [0.3, 0.2, 0.12]); });
        m(hn, &move |q| { b(q, [-0.16, 0.0, -0.16, 0.16, 0.3, 0.16], [0.62, 0.48, 0.38]); b(q, [-0.19, 0.08, -0.2, 0.19, 0.4, 0.19], hood); b(q, [-0.17, 0.1, 0.0, 0.17, 0.27, 0.19], [0.05, 0.05, 0.05]);
            b(q, [-0.165, -0.02, 0.06, 0.165, 0.12, 0.175], [hood[0] * 0.7, hood[1] * 0.7, hood[2] * 0.7]); b(q, [-0.1, 0.17, 0.16, -0.03, 0.21, 0.19], [0.85, 0.82, 0.7]); b(q, [0.03, 0.17, 0.16, 0.1, 0.21, 0.19], [0.85, 0.82, 0.7]);
            b(q, [-0.08, 0.18, 0.18, -0.05, 0.205, 0.195], [0.1, 0.07, 0.05]); b(q, [0.05, 0.18, 0.18, 0.08, 0.205, 0.195], [0.1, 0.07, 0.05]); b(q, [-0.06, 0.36, -0.28, 0.06, 0.42, -0.18], hood); });
    }
    m("briArm", &|q| { b(q, [-0.075, -0.42, -0.075, 0.075, 0.04, 0.075], LEATHER); b(q, [-0.07, -0.72, -0.07, 0.07, -0.42, 0.07], [0.62, 0.48, 0.38]); b(q, [-0.08, -0.6, -0.08, 0.08, -0.46, 0.08], [0.22, 0.15, 0.09]); b(q, [-0.065, -0.82, -0.06, 0.065, -0.72, 0.07], [0.58, 0.45, 0.35]); });
    m("briLeg", &|q| { b(q, [-0.085, -0.44, -0.085, 0.085, 0.0, 0.085], [0.22, 0.2, 0.18]); b(q, [-0.08, -0.84, -0.08, 0.08, -0.44, 0.08], [0.24, 0.16, 0.09]); b(q, [-0.085, -0.9, -0.08, 0.085, -0.82, 0.18], [0.18, 0.12, 0.07]); b(q, [-0.09, -0.52, -0.09, 0.09, -0.46, 0.09], [0.3, 0.2, 0.12]); });
    // Les armes, tenues dans la main droite (le long du bras, pointe vers l'avant quand il frappe).
    m("briClub", &|q| { b(q, [-0.03, -0.1, -0.03, 0.03, 0.4, 0.03], [0.36, 0.24, 0.13]); b(q, [-0.065, 0.3, -0.065, 0.065, 0.62, 0.065], [0.42, 0.29, 0.16]); for y in [0.38, 0.5] { b(q, [-0.075, y, -0.02, 0.075, y + 0.03, 0.02], [0.4, 0.4, 0.42]); } });
    m("briSword", &|q| { b(q, [-0.025, -0.12, -0.025, 0.025, 0.06, 0.025], [0.25, 0.16, 0.1]); b(q, [-0.1, 0.06, -0.03, 0.1, 0.1, 0.03], RUST); b(q, [-0.035, 0.1, -0.012, 0.035, 0.78, 0.012], [0.5, 0.42, 0.36]); b(q, [-0.02, 0.4, -0.014, 0.025, 0.6, 0.014], RUST); });
    m("briAxe", &|q| { b(q, [-0.028, -0.1, -0.028, 0.028, 0.62, 0.028], [0.38, 0.26, 0.14]); b(q, [-0.02, 0.44, 0.0, 0.02, 0.64, 0.22], [0.48, 0.44, 0.4]); b(q, [-0.022, 0.48, 0.18, 0.022, 0.6, 0.24], [0.6, 0.58, 0.54]); });
    // Bête rôdeuse : basse, avant massif et arrière fuyant, une crête de plaques d'os sur l'échine, une longue queue à pointe osseuse.
    m("bstBody", &|q| { b(q, [-0.2, -0.05, 0.0, 0.2, 0.34, 0.5], HIDE); b(q, [-0.16, -0.08, -0.5, 0.16, 0.22, 0.02], HIDE); b(q, [-0.21, 0.05, 0.1, 0.21, 0.2, 0.4], HIDE2);
        for (z, h) in [(0.4, 0.44), (0.25, 0.42), (0.1, 0.38), (-0.08, 0.31), (-0.26, 0.27), (-0.42, 0.24)] { b(q, [-0.05, h - 0.12, z - 0.06, 0.05, h, z + 0.06], BONE); }
        for (x, z) in [(-0.21, 0.3), (0.21, 0.15), (-0.17, -0.2), (0.17, -0.35)] { b(q, [x - 0.02, 0.1, z - 0.05, x + 0.02, 0.18, z + 0.05], HIDE2); }
        b(q, [-0.035, 0.06, -0.95, 0.035, 0.12, -0.5], HIDE); b(q, [-0.05, 0.03, -1.08, 0.05, 0.13, -0.95], BONE); });
    // La tête : un crâne long et étroit, coiffé d'un masque d'os, des yeux rouges enfoncés ; la mâchoire s'ouvre à part.
    m("bstHead", &|q| { b(q, [-0.13, -0.08, -0.06, 0.13, 0.14, 0.2], HIDE); b(q, [-0.09, -0.06, 0.2, 0.09, 0.08, 0.46], HIDE2); b(q, [-0.12, 0.06, 0.0, 0.12, 0.17, 0.4], BONE);
        b(q, [-0.1, 0.02, 0.17, -0.06, 0.07, 0.22], RED_EYE); b(q, [0.06, 0.02, 0.17, 0.1, 0.07, 0.22], RED_EYE); for x in [-0.07, 0.05] { b(q, [x, 0.17, -0.04, x + 0.03, 0.3, 0.06], BONE); }
        for x in [-0.07, -0.025, 0.02, 0.055] { b(q, [x, -0.1, 0.4, x + 0.018, -0.05, 0.43], BONE); } });
    m("bstJaw", &|q| { b(q, [-0.08, -0.06, 0.0, 0.08, 0.0, 0.36], HIDE2); for x in [-0.06, -0.015, 0.03] { b(q, [x, 0.0, 0.3, x + 0.018, 0.05, 0.33], BONE); } });
    m("bstFore", &|q| { b(q, [-0.07, -0.34, -0.07, 0.07, 0.04, 0.08], HIDE); b(q, [-0.055, -0.62, -0.05, 0.055, -0.34, 0.06], HIDE2); b(q, [-0.065, -0.66, -0.04, 0.065, -0.6, 0.14], HIDE); for x in [-0.05, 0.0, 0.035] { b(q, [x, -0.68, 0.12, x + 0.018, -0.63, 0.18], BONE); } });
    m("bstHind", &|q| { b(q, [-0.06, -0.24, -0.08, 0.06, 0.04, 0.06], HIDE); b(q, [-0.05, -0.46, -0.1, 0.05, -0.24, -0.02], HIDE2); b(q, [-0.055, -0.5, -0.08, 0.055, -0.44, 0.08], HIDE); });
    // Rampant nocturne : une tête basse à plaques et mandibules, plusieurs yeux, trois anneaux, six pattes coudées.
    m("crwHead", &|q| { b(q, [-0.2, -0.04, -0.12, 0.2, 0.16, 0.16], CHIT); b(q, [-0.22, 0.12, -0.14, 0.22, 0.19, 0.12], CHIT2);
        for (x, y) in [(-0.12, 0.1), (-0.05, 0.13), (0.03, 0.13), (0.1, 0.1), (-0.15, 0.04), (0.13, 0.04)] { b(q, [x, y - 0.02, 0.15, x + 0.035, y + 0.015, 0.175], GEYE); }
        for sx in [-1.0, 1.0] { let (a, c) = if sx < 0.0 { (-0.17, -0.09) } else { (0.09, 0.17) }; b(q, [a, -0.06, 0.14, c, 0.0, 0.3], CHIT2); b(q, [a + if sx < 0.0 { 0.03 } else { -0.01 }, -0.06, 0.28, c + if sx < 0.0 { 0.03 } else { -0.03 }, -0.02, 0.36], BONE); } });
    m("crwSeg", &|q| { b(q, [-0.24, -0.02, -0.2, 0.24, 0.2, 0.2], CHIT); b(q, [-0.25, 0.16, -0.18, 0.25, 0.25, 0.16], CHIT2); b(q, [-0.04, 0.24, -0.12, 0.04, 0.3, 0.1], CHIT2); for x in [-0.15, 0.11] { b(q, [x, 0.25, -0.05, x + 0.05, 0.26, 0.03], SPOT); } });
    m("crwTail", &|q| { b(q, [-0.17, -0.02, -0.34, 0.17, 0.16, 0.02], CHIT); b(q, [-0.1, 0.0, -0.52, 0.1, 0.12, -0.32], CHIT2); b(q, [-0.04, 0.02, -0.62, 0.04, 0.08, -0.5], BONE); });
    m("crwLeg", &|q| { b(q, [0.0, -0.03, -0.03, 0.34, 0.03, 0.03], CHIT2); b(q, [0.3, -0.38, -0.025, 0.36, 0.0, 0.025], CHIT); b(q, [0.31, -0.44, -0.02, 0.35, -0.36, 0.02], BONE); });
    o
}

impl Horde {
    /// Corps articulés, gestes de chaque état : marche, course, détection (tête dressée), attaque annoncée puis coup, blessure, mort.
    pub fn draws(&self, md: &Models) -> Vec<Draw> {
        let mut out = Vec::with_capacity(self.foes.len() * 12);
        for f in &self.foes {
            let s = &bestiary().species[f.sp]; let hurt = (f.hit / 0.3).clamp(0.0, 1.0) as f32; let mut put = |n: &str, m: M4| out.push(Draw { m, r: md.part(n), view: false, local: false, hero: false, dynamic: false, hurt });
            let fall = (f.dead / 0.9).min(1.0); let flinch = crate::wild::flinch(f.hit) as f64;
            // L'annonce : de 0 (rien) à 1 (au sommet du geste) ; puis le coup part d'un coup.
            let wind = if f.wind > 0.0 { (1.0 - f.wind / s.windup.max(0.05)).clamp(0.0, 1.0) } else { 0.0 }; let strike = if f.wind == 0.0 && f.cd > s.cd - 0.25 && f.state == Ai::Combat { 1.0 } else { 0.0 };
            let run = (f.speed / s.run.max(0.1)).clamp(0.0, 1.0); let sw = if f.speed > 0.0 { sin(f.phase) } else { 0.0 };
            let base = mul(&trs(f.p[0], f.p[1], f.p[2], f.yaw, 0.0, s.size * if f.vet { 1.08 } else { 1.0 }), &roll_z(fall * 1.45 * if f.variant % 2 == 0 { 1.0 } else { -1.0 }));
            match s.body {
                Body::Rodeur | Body::Brigand => {
                    let rod = s.body == Body::Rodeur; let hip = if rod { 0.84 } else { 0.9 };
                    let hunch = if rod { 0.42 } else { 0.08 } + run * 0.18 + wind * 0.15 - strike * 0.1 + flinch * 0.6 - if f.state == Ai::Detection { 0.25 } else { 0.0 };
                    let bob = (f.phase * 2.0).sin().abs() * 0.04 * run;
                    let body = mul(&base, &trs(0.0, hip + bob, 0.0, if rod { 0.12 * (f.phase * 0.5).sin() } else { 0.0 }, hunch, 1.0));
                    put(if rod { "rodTorso" } else { ["briTorso0", "briTorso1", "briTorso2"][f.variant as usize % 3] }, body);
                    let look = if f.state == Ai::Detection || f.state == Ai::Soupcon { -0.35 } else { -hunch * 0.6 };
                    put(if rod { "rodHead" } else { ["briHead0", "briHead1", "briHead2"][f.variant as usize % 3] }, mul(&body, &trs(0.0, if rod { 0.56 } else { 0.62 }, if rod { 0.06 } else { 0.0 }, if f.state == Ai::Soupcon { 0.5 * sin(f.phase * 0.7) } else { 0.0 }, look, 1.0)));
                    let leg = sw * (0.45 + 0.35 * run); put(if rod { "rodLeg" } else { "briLeg" }, mul(&base, &trs(-0.12, hip, 0.0, 0.0, leg, 1.0))); put(if rod { "rodLeg" } else { "briLeg" }, mul(&base, &trs(0.12, hip, 0.0, 0.0, -leg, 1.0)));
                    // Bras : ballants et longs (rôdeur), arme au poing (brigand) ; l'attaque lève le bras droit loin en arrière, puis l'abat.
                    let arm_l = -sw * 0.5 * (0.4 + run) - if rod { 0.35 } else { 0.0 } - wind * 0.4; let atk = -2.4 * wind + strike * 0.6;
                    let arm_r = if wind > 0.0 || strike > 0.0 { atk } else { sw * 0.5 * (0.4 + run) - if rod { 0.35 } else { 0.25 } };
                    let sh = if rod { 0.27 } else { 0.32 }; let ay = if rod { 0.48 } else { 0.54 };
                    put(if rod { "rodArm" } else { "briArm" }, mul(&body, &trs(sh, ay, 0.0, 0.0, arm_l - hunch * 0.5, 1.0)));
                    let ar = mul(&body, &trs(-sh, ay, 0.0, 0.0, arm_r - hunch * 0.5, 1.0)); put(if rod { "rodArm" } else { "briArm" }, ar);
                    if !rod { put(["briClub", "briSword", "briAxe"][f.variant as usize % 3], mul(&ar, &trs(0.0, -0.78, 0.04, 0.0, std::f64::consts::FRAC_PI_2 * 0.9, 1.0))); }
                }
                Body::Bete => {
                    let hip = 0.62; let crouch = wind * 0.14 - strike * 0.05 + if f.state == Ai::Embuscade || f.state == Ai::Soupcon { 0.1 } else { 0.0 };
                    let gallop = run > 0.6; let pitch = if f.air { -0.25 } else { 0.0 } + flinch * 0.5;
                    let body = mul(&base, &trs(0.0, hip - crouch + if gallop { (f.phase).sin().abs() * 0.06 } else { 0.0 }, 0.0, 0.0, pitch, 1.0));
                    put("bstBody", body);
                    let head = mul(&body, &trs(0.0, 0.22, 0.5, 0.0, if f.state == Ai::Detection { -0.4 } else { 0.1 + wind * 0.25 - strike * 0.3 }, 1.0)); put("bstHead", head);
                    put("bstJaw", mul(&head, &trs(0.0, -0.06, 0.02, 0.0, 0.15 + (wind + strike) * 0.55 + if f.state == Ai::Detection { 0.4 } else { 0.0 }, 1.0)));
                    let a = if f.air { 0.9 } else { sw * (0.5 + 0.4 * run) }; let b2 = if gallop { a } else { -a };
                    put("bstFore", mul(&body, &trs(-0.15, 0.04, 0.38, 0.0, a, 1.0))); put("bstFore", mul(&body, &trs(0.15, 0.04, 0.38, 0.0, b2, 1.0)));
                    put("bstHind", mul(&body, &trs(-0.12, 0.04, -0.36, 0.0, -b2 * 0.9, 1.0))); put("bstHind", mul(&body, &trs(0.12, 0.04, -0.36, 0.0, -a * 0.9, 1.0)));
                }
                Body::Rampant => {
                    // Trois anneaux qui ondulent à la suite de la tête ; les six pattes vont par trépied (1-4-5 puis 2-3-6).
                    let lift = if f.climbing { 0.0 } else { wind * 0.25 }; let rear = if f.climbing { -1.2 } else { -wind * 0.6 + strike * 0.3 };
                    let root = mul(&base, &trs(0.0, 0.32 + lift, 0.0, 0.0, rear + flinch * 0.4, 1.0)); let wave = |k: f64| 0.18 * sin(f.phase * 0.9 - k) * (0.3 + run);
                    put("crwHead", mul(&root, &trs(0.0, 0.0, 0.42, wave(0.0) * 0.5, if f.state == Ai::Embuscade { 0.2 } else { -0.1 }, 1.0)));
                    let mut seg = root;
                    for k in 0..3 { let m = mul(&seg, &trs(0.0, 0.0, if k == 0 { 0.18 } else { -0.4 }, wave(k as f64 + 1.0), 0.0, 1.0 - k as f64 * 0.08)); put("crwSeg", m);
                        for sx in [-1.0f64, 1.0] { let tri = if (k + if sx > 0.0 { 1 } else { 0 }) % 2 == 0 { 1.0 } else { -1.0 }; let a = sw * 0.45 * tri;
                            let leg = mul(&m, &mul(&trs(0.22 * sx, 0.06, 0.0, a + if sx > 0.0 { 0.0 } else { std::f64::consts::PI }, 0.0, 1.0), &roll_z(0.35 + 0.2 * (sin(f.phase + k as f64) * tri).max(0.0) * run)));
                            put("crwLeg", leg); }
                        seg = m; }
                    put("crwTail", mul(&seg, &trs(0.0, 0.02, -0.18, wave(4.0), 0.0, 1.0)));
                }
            }
        }
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
        let sun = self.sun_height(); let night = ((0.05 - sun) / 0.15).clamp(0.0, 1.0);
        let sp = self.player.v[0].hypot(self.player.v[2]); let mut noise: f64 = if sp > 5.5 { 0.9 } else if sp > 0.5 { 0.45 } else { 0.1 }; if self.mine_t > 0.0 || self.hit_t > 0.0 { noise = noise.max(0.8); } if self.player.crouch { noise *= 0.3; }
        let lit = self.inv[self.sel].is_some_and(|s| matches!(s.item, "torch" | "lantern")) || self.light_at(self.player.p[0].floor() as i32, self.player.p[1].floor() as i32 + 1, self.player.p[2].floor() as i32) >= 10;
        let can_spawn = !self.creative && self.dead.is_none() && self.tour.is_none();
        let mut horde = std::mem::replace(&mut self.horde, Horde::new(0));
        { let gen = &self.gen; let protect = |x: f64, z: f64| gen.villages_near(x, z, 120.0).iter().any(|v| ((v.c[0] as f64 - x).hypot(v.c[2] as f64 - z)) < v.r as f64 + 10.0);
        let me: &Session = self; let light = |x: i32, y: i32, z: i32| me.light_at(x, y, z);
        let ctx = Ctx { eye: self.player.eye(), feet: self.player.p, look: self.look, noise, lit, crouch: self.player.crouch, alive: self.dead.is_none() && !self.creative, night, danger: self.danger.total, can_spawn, light: &light, protect: &protect, main_biome: self.danger.region.2 as usize };
        horde.update(dt, &self.world, &ctx); }
        self.horde = horde;
        for e in std::mem::take(&mut self.horde.ev) {
            match e {
                FoeEv::Hit { from, dmg, cause, push } => { self.sound.thump(from); let n = self.block(from, dmg); if n > 0.0 { self.harm(n.round().max(1.0), cause, true);
                    let (dx, dz) = (self.player.p[0] - from[0], self.player.p[2] - from[2]); let d = dx.hypot(dz).max(0.1); self.player.v[0] += dx / d * push; self.player.v[2] += dz / d * push; self.player.v[1] = self.player.v[1].max(2.5); } }
                FoeEv::Growl(p, b) => match b { Body::Bete => self.sound.fox_scream(p), Body::Rampant => self.sound.rustle(p), _ => self.sound.thump(p) },
                FoeEv::Swing(_) => self.sound.swing(None, 0.8),
                FoeEv::Die(p, sp, vet) => { let s = &bestiary().species[sp]; self.sound.thump(p); self.skills[2] += 2.0; if self.creative { continue; }
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
    /// Outil de développement : fait naître une créature devant soi (l'espèce suivante à chaque appel).
    pub fn debug_spawn(&mut self, k: usize) -> &'static str {
        let n = bestiary().species.len(); if n == 0 { return ""; } let sp = k % n; let f = self.look; let p = self.player.p;
        let (x, z) = (p[0] + f[0] * 6.0, p[2] + f[2] * 6.0); let y = ground(&self.world, x, z, p[1] + 3.0).unwrap_or(p[1]);
        let i = self.horde.put(sp, [x, y, z], (p[0] - x).atan2(p[2] - z), false); if bestiary().species[sp].ambush { self.horde.foes[i].state = Ai::Embuscade; } bestiary().species[sp].name
    }
}
