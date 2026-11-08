//! Survie : ce que le corps demande (faim, soif, souffle, endurance), ce qui le blesse, ce qu'on mange,
//! et les recettes de fabrication. Que des règles et des nombres : la partie (session.rs) les applique.
use crate::actors::b;
use crate::mesher::VBuf;

/// Objets qui ne sont pas des blocs : identifiant, nom affiché.
pub const EXTRA: [(&str, &str); 12] = [("stone_knife", "Couteau en pierre"), ("stone_axe", "Hache en pierre"), ("stone_pickaxe", "Pioche en pierre"), ("pebble", "Caillou"), ("fiber", "Fibre végétale"), ("seeds", "Graines"), ("stick", "Bâton"), ("berries", "Myrtilles"), ("rowan_berries", "Baies de sorbier"), ("raw_meat", "Viande crue"), ("cooked_meat", "Viande cuite"), ("leather", "Cuir")];
/// Les outils ne s'empilent pas : un par case.
pub fn stackable(id: &str) -> bool { !matches!(id, "pickaxe" | "axe" | "sword" | "stone_knife" | "stone_axe" | "stone_pickaxe") }
/// Outils : catégorie de blocs qu'ils attaquent, et vitesse sur cette catégorie (la pierre taillée va moins vite que l'acier).
pub fn tool_kind(id: &str) -> Option<(&'static str, f64)> { match id { "pickaxe" => Some(("rock", 3.0)), "axe" => Some(("wood", 3.0)), "stone_pickaxe" => Some(("rock", 1.8)), "stone_axe" => Some(("wood", 1.8)), _ => None } }
pub fn is_axe(id: Option<&str>) -> bool { matches!(id, Some("axe") | Some("stone_axe")) }
/// Le son d'un outil de pierre est celui de l'outil d'acier de même forme.
pub fn voice(id: Option<&'static str>) -> Option<&'static str> { match id { Some("stone_axe") => Some("axe"), Some("stone_pickaxe") => Some("pickaxe"), Some("stone_knife") => Some("sword"), o => o } }
/// Tout casse plus lentement qu'au prototype : la dureté du registre est multipliée par ce facteur.
pub const HARD: f64 = 4.0;
/// Vitesse de casse à mains nues ou avec un outil qui ne convient pas, selon la catégorie du bloc.
pub fn bare_rate(cat: &str) -> f64 { match cat { "rock" => 0.25, "wood" => 0.45, _ => 1.0 } }
/// À mains nues, la roche s'effrite sans rien donner : il faut une pioche.
pub fn needs_tool(cat: &str) -> bool { cat == "rock" }
/// Ce que l'herbe arrachée laisse parfois : fibre (toujours au couteau), graines.
pub fn grass_drops(block: &str) -> &'static [(&'static str, f64)] { match block { "tall_grass" | "feather_grass" | "frost_grass" | "cotton_grass" | "fern" | "fern_dry" => &[("fiber", 0.4), ("seeds", 0.15)], _ => &[] } }
/// Ramassé d'un clic droit : le bloc disparaît et l'objet va dans le sac.
pub fn pickup(block: &str) -> Option<(&'static str, u32)> { match block { "twigs" => Some(("stick", 1)), "pebbles" => Some(("pebble", 1)), _ => None } }
/// Buissons qui se cueillent au clic droit : les baies, et le buisson dépouillé qui les remplace pour une journée.
pub fn harvest(block: &str) -> Option<(&'static str, u32, &'static str)> { match block { "blueberry" => Some(("berries", 3, "blueberry_picked")), "rowan" => Some(("rowan_berries", 3, "rowan_picked")), _ => None } }
pub const REGROW_HOURS: f64 = 48.0; // deux jours de jeu

// Rythmes. Une journée dure 1500 s : sans manger, on tient un peu plus d'une journée et demie ; sans boire, un peu plus d'une.
pub const HUNGER_RATE: f64 = 0.04; pub const THIRST_RATE: f64 = 0.06;
pub const STAMINA_SPRINT: f64 = 12.0; pub const STAMINA_JUMP: f64 = 5.0; pub const STAMINA_ROLL: f64 = 15.0; pub const STAMINA_HIT: f64 = 4.0;
/// L'endurance revient vite, mais seulement après une seconde de répit.
pub const STAMINA_REGEN: f64 = 22.0; pub const STAMINA_WAIT: f64 = 1.0; pub const WINDED_UNTIL: f64 = 30.0;
/// Quinze secondes de souffle sous l'eau, puis la noyade.
pub const AIR_TIME: f64 = 15.0;
/// Vitesse d'arrivée au sol sans dommage : une chute d'environ trois blocs et demi.
pub const SAFE_FALL: f64 = 13.5;

/// Ce que rapporte un aliment : faim, soif, santé (négative pour ce qui rend malade).
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Food { pub hunger: f64, pub thirst: f64, pub health: f64 }
pub fn food(id: &str) -> Option<Food> {
    let f = |hunger, thirst, health| Some(Food { hunger, thirst, health });
    match id { "berries" => f(9.0, 5.0, 0.0), "rowan_berries" => f(6.0, 2.0, 0.0), "mushroom" => f(7.0, 0.0, 0.0), "raw_meat" => f(10.0, 0.0, -4.0), "cooked_meat" => f(35.0, 0.0, 4.0), _ => None }
}
/// Ce qu'on récolte en plus du bloc : les baies tombent du buisson cueilli.
pub fn bonus_drop(block: &str) -> Option<(&'static str, u32)> { match block { "blueberry" => Some(("berries", 3)), "rowan" => Some(("rowan_berries", 3)), "mushroom" => Some(("mushroom", 1)), _ => None } }

/// Groupes d'ingrédients : n'importe quel bois, n'importe quelle roche, n'importe quel champignon lumineux.
pub const GROUPS: [(&str, &str, &[&str]); 3] = [
    ("log", "Bois", &["oak_log", "birch_log", "beech_log", "spruce_log", "acacia_log", "palm_log", "dead_log"]),
    ("rock", "Pierre ou caillou", &["pebble", "stone", "mossy_stone", "limestone", "granite", "lichen_stone", "sandstone", "red_sandstone"]),
    ("glow", "Champignon lumineux", &["glowcap", "glowcap_blue", "glowcap_pink", "glowcap_violet", "glowcap_cyan"]),
];
/// Les objets acceptés pour un ingrédient : le groupe entier, ou l'objet lui-même.
pub fn members(need: &'static str) -> Vec<&'static str> { GROUPS.iter().find(|g| g.0 == need).map_or(vec![need], |g| g.2.to_vec()) }
pub fn group_label(need: &str) -> Option<&'static str> { GROUPS.iter().find(|g| g.0 == need).map(|g| g.1) }

#[derive(Clone, Copy, Debug, PartialEq)] pub enum Station { Hand, Bench, Fire }
impl Station { pub fn label(self) -> &'static str { match self { Station::Hand => "À la main", Station::Bench => "Près d'un établi", Station::Fire => "Près d'un feu de camp" } } }
#[derive(Clone, Copy, Debug)] pub struct Recipe { pub out: &'static str, pub n: u32, pub need: &'static [(&'static str, u32)], pub at: Station }
/// Le départ : branches, cailloux et fibre d'herbe donnent les outils de pierre, à la main ; la hache donne le bois, le bois l'établi.
pub const RECIPES: [Recipe; 14] = [
    Recipe { out: "stone_knife", n: 1, need: &[("pebble", 2), ("stick", 1), ("fiber", 1)], at: Station::Hand },
    Recipe { out: "stone_axe", n: 1, need: &[("pebble", 3), ("stick", 2), ("fiber", 3)], at: Station::Hand },
    Recipe { out: "stone_pickaxe", n: 1, need: &[("pebble", 3), ("stick", 2), ("fiber", 3)], at: Station::Hand },
    Recipe { out: "planks", n: 4, need: &[("log", 1)], at: Station::Hand },
    Recipe { out: "stick", n: 2, need: &[("planks", 1)], at: Station::Hand },
    Recipe { out: "workbench", n: 1, need: &[("planks", 4)], at: Station::Hand },
    Recipe { out: "campfire", n: 1, need: &[("stick", 3), ("rock", 3)], at: Station::Hand },
    Recipe { out: "cooked_meat", n: 1, need: &[("raw_meat", 1)], at: Station::Fire },
    Recipe { out: "pickaxe", n: 1, need: &[("rock", 3), ("stick", 2), ("fiber", 2)], at: Station::Bench },
    Recipe { out: "axe", n: 1, need: &[("rock", 3), ("stick", 2), ("fiber", 2)], at: Station::Bench },
    Recipe { out: "sword", n: 1, need: &[("rock", 2), ("stick", 1), ("leather", 1), ("fiber", 1)], at: Station::Bench },
    Recipe { out: "fence", n: 3, need: &[("planks", 2), ("stick", 2)], at: Station::Bench },
    Recipe { out: "flagstone", n: 4, need: &[("rock", 2)], at: Station::Bench },
    Recipe { out: "lantern", n: 2, need: &[("stick", 1), ("glow", 1)], at: Station::Bench },
];

/// Gibier : points de vie, ce qu'il laisse, et la boîte qui sert à le viser (demi-largeur, hauteur).
pub const DEER_HP: f64 = 14.0; pub const FOX_HP: f64 = 8.0; pub const HARE_HP: f64 = 4.0;
pub const DEER_LOOT: &[(&str, u32)] = &[("raw_meat", 3), ("leather", 2)]; pub const FOX_LOOT: &[(&str, u32)] = &[("leather", 1)]; pub const HARE_LOOT: &[(&str, u32)] = &[("raw_meat", 2)];
/// Coup selon ce qu'on tient, et temps avant le suivant.
pub fn weapon(id: Option<&str>) -> (f64, f64) { match id { Some("sword") => (7.0, 0.45), Some("axe") => (5.0, 0.7), Some("pickaxe") => (4.0, 0.75), Some("stone_knife") => (4.0, 0.4), Some("stone_axe") => (4.0, 0.75), Some("stone_pickaxe") => (3.0, 0.8), _ => (2.0, 0.4) } }
/// Chaque coup, dans le vide, dans un bloc ou sur un animal, coûte un peu d'endurance.
pub const STAMINA_SWING: f64 = 1.5;
pub const REACH: f64 = 3.6;
/// Rayon contre boîte posée au sol, centrée en x et z : distance d'entrée, s'il la touche.
pub fn ray_box(o: [f64; 3], d: [f64; 3], c: [f64; 3], hw: f64, h: f64) -> Option<f64> {
    let (lo, hi) = ([c[0] - hw, c[1], c[2] - hw], [c[0] + hw, c[1] + h, c[2] + hw]); let (mut t0, mut t1) = (0.0f64, f64::INFINITY);
    for k in 0..3 {
        if d[k].abs() < 1e-9 { if o[k] < lo[k] || o[k] > hi[k] { return None; } continue; }
        let (a, b) = ((lo[k] - o[k]) / d[k], (hi[k] - o[k]) / d[k]); let (a, b) = if a < b { (a, b) } else { (b, a) };
        t0 = t0.max(a); t1 = t1.min(b); if t0 > t1 { return None; }
    }
    Some(t0)
}

/// Pourquoi on est mort, et où sont restées les affaires.
#[derive(Clone, Debug, PartialEq)] pub struct Death { pub cause: &'static str, pub at: [f64; 3] }
#[derive(Clone, Debug, PartialEq)] pub struct Stash { pub p: [f64; 3], pub items: Vec<crate::session::Stack> }
/// Le sac laissé à la mort : cuir, rabat, sangle.
pub fn parts() -> Vec<(&'static str, VBuf)> {
    let mut o = VBuf::new(60); let (bag, flap, strap) = ([0.42, 0.27, 0.15], [0.52, 0.35, 0.2], [0.26, 0.16, 0.09]);
    b(&mut o, [-0.22, 0.0, -0.15, 0.22, 0.36, 0.15], bag); b(&mut o, [-0.23, 0.26, -0.16, 0.23, 0.4, 0.16], flap); b(&mut o, [-0.04, 0.12, 0.15, 0.04, 0.32, 0.18], strap); b(&mut o, [-0.025, 0.4, -0.03, 0.025, 0.5, 0.03], strap);
    vec![("stash", o)]
}
