//! Base des objets et des recettes. Tout vient des fichiers de `engine/core/data` : on ajoute un objet, un outil,
//! un vêtement ou une recette en ajoutant une ligne, sans toucher au code. Les erreurs de lecture sont rassemblées
//! dans `Db::errors` (un test les refuse), jamais ignorées en silence.
use crate::blocks::by_name;
use std::collections::HashMap;
use std::sync::OnceLock;

pub const OBJETS: &str = include_str!("../data/objets.txt");
pub const RECETTES: &str = include_str!("../data/recettes.txt");
pub const ARTISANS: &str = include_str!("../data/artisans.txt");
pub const TRADES: &str = include_str!("../data/echanges.txt");
/// Un échange proposé par les villageois d'un type de village (« tous » : partout).
#[derive(Clone, Debug)] pub struct Trade { pub kind: &'static str, pub give: (&'static str, u32), pub get: (&'static str, u32) }
/// Les types de villages, d'après les biomes.
pub const VILLAGES: [&str; 7] = ["prairie", "taiga", "aride", "desert", "montagne", "hautes_terres", "port"];

/// Emplacements d'équipement, dans l'ordre de l'écran.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub enum Slot { Head, Torso, Legs, Feet, Back, Hand }
pub const SLOTS: [(Slot, &str); 6] = [(Slot::Head, "Tête"), (Slot::Torso, "Torse"), (Slot::Legs, "Jambes"), (Slot::Feet, "Pieds"), (Slot::Back, "Dos"), (Slot::Hand, "Main gauche")];
/// Matière d'un outil : palier (ce qu'elle peut extraire), vitesse de travail, usure, force des coups.
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Mat { pub name: &'static str, pub tier: u8, pub speed: f64, pub dur: u32, pub force: f64 }
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Tool { pub kind: &'static str, pub mat: Mat }
impl Tool {
    /// La catégorie de blocs que cet outil travaille vite.
    pub fn works(&self, cat: &str) -> bool { match self.kind { "hache" => cat == "wood", "pioche" => cat == "rock", "pelle" => cat == "terrain", "faucille" | "couteau" => cat == "foliage" || cat == "decor", _ => false } }
    /// Dégâts et temps entre deux coups.
    pub fn strike(&self) -> (f64, f64) {
        let (d, w) = match self.kind { "epee" => (6.0, 0.45), "lance" => (6.0, 0.65), "hache" => (5.0, 0.7), "couteau" => (4.0, 0.4), "pioche" => (4.0, 0.75), "marteau" => (4.5, 0.8), "faucille" => (3.0, 0.45), _ => (3.0, 0.6) };
        (d * self.mat.force, w)
    }
    /// Portée du coup : la lance tient la proie à distance.
    pub fn reach(&self) -> f64 { if self.kind == "lance" { 5.0 } else { 3.6 } }
    /// Voix du coup : celle de l'outil d'acier de même forme.
    pub fn voice(&self) -> &'static str { match self.kind { "hache" => "axe", "pioche" | "pelle" | "houe" | "marteau" => "pickaxe", _ => "sword" } }
}
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Food { pub hunger: f64, pub thirst: f64, pub health: f64 }
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Wear { pub slot: Slot, pub warm: f64, pub cool: f64, pub armor: f64, pub bag: f64 }
/// Un objet : tout ce que l'interface, le sac, le craft et la survie ont besoin d'en savoir.
#[derive(Clone, Debug)] pub struct Def {
    pub id: &'static str, pub label: &'static str, pub cat: &'static str, pub mass: f64, pub stack: u32, pub desc: &'static str, pub block: Option<crate::blocks::Id>,
    pub tool: Option<Tool>, pub food: Option<Food>, pub keep: f64, pub risk: f64, pub wear: Option<Wear>, pub fuel: f64, pub drink: Option<(f64, bool)>, pub heal: f64,
    /// Nombre d'usages avant qu'il ne s'use (arc, aiguille, gourde : une gorgée par usage). 0 : ne s'use pas ainsi.
    pub uses: u32,
}
#[derive(Clone, Copy, Debug, PartialEq)] pub enum Station { Hand, Bench, Fire, Furnace, Anvil, Mason }
impl Station {
    pub fn label(self) -> &'static str { match self { Station::Hand => "À la main", Station::Bench => "Près d'un établi", Station::Fire => "Près d'un feu de camp allumé", Station::Furnace => "Près d'un four", Station::Anvil => "Près d'une enclume", Station::Mason => "Près d'un banc de taille" } }
    pub fn name(self) -> &'static str { match self { Station::Hand => "main", Station::Bench => "établi", Station::Fire => "feu de camp", Station::Furnace => "four", Station::Anvil => "enclume", Station::Mason => "banc de taille" } }
    pub const ALL: [Station; 6] = [Station::Hand, Station::Bench, Station::Fire, Station::Furnace, Station::Anvil, Station::Mason];
}
/// Un ingrédient : un objet ou un groupe, combien, et s'il est seulement nécessaire (un outil qu'on garde).
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Need { pub id: &'static str, pub n: u32, pub keep: bool }
#[derive(Clone, Debug)] pub struct Recipe { pub out: &'static str, pub n: u32, pub need: Vec<Need>, pub at: Station, pub cat: &'static str, pub artisan: Option<&'static str> }
/// Un métier de village : le palier le plus haut qu'il maîtrise, les villages où il vit, ce qu'il propose.
#[derive(Clone, Debug)] pub struct Craftsman { pub id: &'static str, pub name: &'static str, pub tier: u8, pub villages: Vec<&'static str>, pub services: Vec<&'static str>, pub price: Vec<(&'static str, u32)>, pub works: Vec<(Station, &'static str)> }
pub struct Db { pub items: Vec<Def>, index: HashMap<&'static str, usize>, pub groups: Vec<(&'static str, &'static str, Vec<&'static str>)>, pub recipes: Vec<Recipe>, pub errors: Vec<String>, pub order: Vec<&'static str>, pub tabs: Vec<(&'static str, Vec<&'static str>)> , pub craftsmen: Vec<Craftsman>, pub trades: Vec<Trade> }
/// Onglets du catalogue créatif : clé de catégorie et libellé court.
pub const CATS: [(&str, &str); 13] = [("terrain", "Terrain"), ("roches", "Roches"), ("bois", "Bois"), ("plantes", "Plantes"), ("batir", "Bâtir"), ("mobilier", "Mobilier"), ("decor", "Décor"), ("ateliers", "Ateliers"), ("outils", "Outils"), ("matieres", "Matières"), ("vivres", "Vivres"), ("habits", "Habits"), ("oeufs", "Œufs")];

impl Db {
    pub fn get(&self, id: &str) -> Option<&Def> { self.index.get(id).map(|i| &self.items[*i]) }
    /// Les objets acceptés pour un ingrédient : tout le groupe, ou l'objet lui-même.
    pub fn members(&self, need: &str) -> Vec<&'static str> { self.groups.iter().find(|g| g.0 == need).map_or_else(|| self.get(need).map(|d| vec![d.id]).unwrap_or_default(), |g| g.2.clone()) }
    pub fn group_label(&self, need: &str) -> Option<&'static str> { self.groups.iter().find(|g| g.0 == need).map(|g| g.1) }
    fn parse() -> Db {
        let mut db = Db { items: Vec::new(), index: HashMap::new(), groups: Vec::new(), recipes: Vec::new(), errors: Vec::new(), order: Vec::new(), tabs: Vec::new(), craftsmen: Vec::new(), trades: Vec::new() };
        let mut mats: Vec<Mat> = Vec::new();
        for (ln, line) in OBJETS.lines().enumerate() {
            let l = line.trim(); if l.is_empty() || l.starts_with('#') { continue; }
            if let Some(m) = l.strip_prefix("matiere ") { let p: Vec<&str> = m.split_whitespace().collect(); if p.len() == 5 { if let (Ok(t), Ok(s), Ok(d), Ok(f)) = (p[1].parse(), p[2].parse(), p[3].parse(), p[4].parse()) { mats.push(Mat { name: p[0], tier: t, speed: s, dur: d, force: f }); continue; } } db.errors.push(format!("objets.txt:{} matière illisible", ln + 1)); continue; }
            let f: Vec<&'static str> = l.split('|').map(str::trim).collect();
            if f.len() != 7 { db.errors.push(format!("objets.txt:{} : 7 champs attendus, {} trouvés", ln + 1, f.len())); continue; }
            let (Ok(mass), Ok(stack)) = (f[3].parse::<f64>(), f[4].parse::<u32>()) else { db.errors.push(format!("objets.txt:{} masse ou pile illisible", ln + 1)); continue };
            let mut d = Def { id: f[0], label: f[1], cat: f[2], mass, stack, desc: f[5], block: by_name(f[0]), tool: None, food: None, keep: 0.0, risk: 0.0, wear: None, fuel: 0.0, drink: None, uses: 0, heal: 0.0 };
            let (mut kind, mut mat, mut slot, mut warm, mut cool, mut armor, mut bag) = (None, None, None, 0.0, 0.0, 0.0, 0.0);
            for p in f[6].split_whitespace() {
                let Some((k, v)) = p.split_once('=') else { db.errors.push(format!("objets.txt:{} propriété « {p} » sans valeur", ln + 1)); continue };
                let num = |v: &str| v.parse::<f64>().ok(); let mut bad = false;
                match k {
                    "outil" => kind = Some(v), "mat" => mat = mats.iter().find(|m| m.name == v).copied(), "porte" => slot = match v { "tete" => Some(Slot::Head), "torse" => Some(Slot::Torso), "jambes" => Some(Slot::Legs), "pieds" => Some(Slot::Feet), "dos" => Some(Slot::Back), "main" => Some(Slot::Hand), _ => { bad = true; None } },
                    "chaud" => warm = num(v).unwrap_or_else(|| { bad = true; 0.0 }), "frais" => cool = num(v).unwrap_or_else(|| { bad = true; 0.0 }), "armure" => armor = num(v).unwrap_or_else(|| { bad = true; 0.0 }), "sac" => bag = num(v).unwrap_or_else(|| { bad = true; 0.0 }),
                    "aliment" => { let n: Vec<f64> = v.split(':').filter_map(num).collect(); if n.len() == 3 { d.food = Some(Food { hunger: n[0], thirst: n[1], health: n[2] }); } else { bad = true; } }
                    "usages" => d.uses = num(v).map_or_else(|| { bad = true; 0 }, |x| x as u32),
                    "boisson" => { let n: Vec<f64> = v.split(':').filter_map(num).collect(); if n.len() == 2 { d.drink = Some((n[0], n[1] > 0.5)); } else { bad = true; } }
                    "garde" => d.keep = num(v).unwrap_or_else(|| { bad = true; 0.0 }), "risque" => d.risk = num(v).unwrap_or_else(|| { bad = true; 0.0 }), "combustible" => d.fuel = num(v).unwrap_or_else(|| { bad = true; 0.0 }), "soin" => d.heal = num(v).unwrap_or_else(|| { bad = true; 0.0 }),
                    _ => bad = true,
                }
                if bad { db.errors.push(format!("objets.txt:{} propriété « {p} » inconnue ou illisible", ln + 1)); }
            }
            match (kind, mat) { (Some(k), Some(m)) => d.tool = Some(Tool { kind: k, mat: m }), (None, None) => {}, _ => db.errors.push(format!("objets.txt:{} outil sans matière connue", ln + 1)) }
            if let Some(s) = slot { d.wear = Some(Wear { slot: s, warm, cool, armor, bag }); }
            if !CATS.iter().any(|c| c.0 == d.cat) { db.errors.push(format!("objets.txt:{} catégorie « {} » inconnue", ln + 1, d.cat)); }
            if db.index.insert(d.id, db.items.len()).is_some() { db.errors.push(format!("objets.txt:{} « {} » défini deux fois", ln + 1, d.id)); }
            db.items.push(d);
        }
        for (ln, line) in RECETTES.lines().enumerate() {
            let l = line.trim(); if l.is_empty() || l.starts_with('#') { continue; }
            if let Some(g) = l.strip_prefix("groupe ") { let p: Vec<&'static str> = g.split('|').map(str::trim).collect(); if p.len() == 3 { db.groups.push((p[0], p[1], p[2].split_whitespace().collect())); } else { db.errors.push(format!("recettes.txt:{} groupe illisible", ln + 1)); } continue; }
            let f: Vec<&'static str> = l.split('|').map(str::trim).collect();
            if f.len() != 4 && f.len() != 5 { db.errors.push(format!("recettes.txt:{} : 4 ou 5 champs attendus", ln + 1)); continue; }
            let artisan = match f.get(4).map(|x| x.strip_prefix("artisan=")) { None => None, Some(Some(a)) => Some(a), Some(None) => { db.errors.push(format!("recettes.txt:{} cinquième champ : artisan=<métier>", ln + 1)); continue } };
            let mut o = f[0].split_whitespace(); let (Some(out), Some(Ok(n))) = (o.next(), o.next().map(str::parse::<u32>)) else { db.errors.push(format!("recettes.txt:{} sortie illisible", ln + 1)); continue };
            let at = match f[1] { "main" => Station::Hand, "etabli" => Station::Bench, "feu" => Station::Fire, "four" => Station::Furnace, "enclume" => Station::Anvil, "taille" => Station::Mason, a => { db.errors.push(format!("recettes.txt:{} atelier « {a} » inconnu", ln + 1)); continue } };
            let mut need = Vec::new();
            for part in f[2].split(',') { let mut w = part.split_whitespace(); let (Some(id), Some(Ok(k))) = (w.next(), w.next().map(str::parse::<u32>)) else { db.errors.push(format!("recettes.txt:{} ingrédient « {} » illisible", ln + 1, part.trim())); continue }; let keep = id.starts_with('!'); need.push(Need { id: id.trim_start_matches('!'), n: k, keep }); }
            db.recipes.push(Recipe { out, n, need, at, cat: f[3], artisan });
        }
        for (ln, line) in TRADES.lines().enumerate() {
            let line = line.trim(); if line.is_empty() || line.starts_with('#') { continue; }
            let f: Vec<&str> = line.split('|').map(str::trim).collect(); let pair = |t: &'static str| { let mut w = t.split_whitespace(); Some((w.next()?, w.next()?.parse::<u32>().ok()?)) };
            match (f.len(), f.get(1).and_then(|t| pair(t)), f.get(2).and_then(|t| pair(t))) { (3, Some(give), Some(get)) => db.trades.push(Trade { kind: f[0], give, get }), _ => db.errors.push(format!("echanges.txt:{} illisible", ln + 1)) }
        }
        for (ln, line) in ARTISANS.lines().enumerate() {
            let l = line.trim(); if l.is_empty() || l.starts_with('#') { continue; } let f: Vec<&'static str> = l.split('|').map(str::trim).collect();
            let Some(Ok(tier)) = f.get(2).map(|t| t.parse::<u8>()) else { db.errors.push(format!("artisans.txt:{} palier illisible", ln + 1)); continue };
            if f.len() != 7 { db.errors.push(format!("artisans.txt:{} : 7 champs attendus", ln + 1)); continue; }
            let mut price = Vec::new();
            for part in f[5].split(',') { let mut w = part.split_whitespace(); match (w.next(), w.next().map(str::parse::<u32>)) { (Some(id), Some(Ok(n))) => price.push((id, n)), _ => db.errors.push(format!("artisans.txt:{} prix « {} » illisible", ln + 1, part.trim())) } }
            let mut works = Vec::new();
            for wk in f[6].split_whitespace() { let (st, cat) = wk.split_once('/').unwrap_or((wk, "")); match station_of(st) { Some(at) => works.push((at, cat)), None => db.errors.push(format!("artisans.txt:{} atelier « {st} »", ln + 1)) } }
            for v in f[3].split_whitespace() { if !VILLAGES.contains(&v) { db.errors.push(format!("artisans.txt:{} village « {v} » inconnu", ln + 1)); } }
            db.craftsmen.push(Craftsman { id: f[0], name: f[1], tier, villages: f[3].split_whitespace().collect(), services: f[4].split_whitespace().collect(), price, works });
        }
        for t in &db.trades { for id in [t.give.0, t.get.0] { if !db.index.contains_key(id) { db.errors.push(format!("echanges.txt : objet « {id} » inconnu")); } } if t.kind != "tous" && !VILLAGES.contains(&t.kind) { db.errors.push(format!("echanges.txt : type de village « {} »", t.kind)); } }
        for c in &db.craftsmen { for (id, _) in &c.price { if !db.index.contains_key(id) { db.errors.push(format!("artisans.txt : prix de {} : objet « {id} » inconnu", c.id)); } } }
        for r in &db.recipes { if let Some(a) = r.artisan { if !db.craftsmen.iter().any(|c| c.id == a) { db.errors.push(format!("recette de {} : métier « {a} » inconnu", r.out)); } } }
        // Tout ce qu'une recette nomme doit exister.
        for r in &db.recipes { if db.get(r.out).is_none() { db.errors.push(format!("recette : sortie « {} » inconnue", r.out)); } for nd in &r.need { if db.members(nd.id).is_empty() { db.errors.push(format!("recette de {} : ingrédient « {} » inconnu", r.out, nd.id)); } } }
        for g in &db.groups { for m in &g.2 { if db.get(m).is_none() { db.errors.push(format!("groupe {} : « {m} » inconnu", g.0)); } } }
        db.order = db.items.iter().map(|d| d.id).collect();
        db.tabs = CATS.iter().map(|(k, label)| (*label, db.items.iter().filter(|d| d.cat == *k).map(|d| d.id).collect())).collect();
        db
    }
}
pub fn db() -> &'static Db { static DB: OnceLock<Db> = OnceLock::new(); DB.get_or_init(Db::parse) }
/// L'atelier nommé dans les fichiers de données.
pub fn station_of(n: &str) -> Option<Station> { Some(match n { "main" => Station::Hand, "etabli" => Station::Bench, "feu" => Station::Fire, "four" => Station::Furnace, "enclume" => Station::Anvil, "taille" => Station::Mason, _ => return None }) }
pub fn def(id: &str) -> Option<&'static Def> { db().get(id) }
