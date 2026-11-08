//! Survie : ce que le corps demande (faim, soif, souffle, endurance, chaleur, sommeil), ce qui le blesse,
//! ce que donnent la récolte et la chasse. Que des règles et des nombres : la partie (session.rs, play.rs) les applique.
//! Les objets et les recettes sont des données : voir data.rs et engine/core/data.
use crate::actors::b;
use crate::data::{db, def};
pub use crate::data::{Food, Recipe, Station};
use crate::mesher::VBuf;

/// Les recettes, dans l'ordre du fichier.
pub fn recipes() -> &'static [Recipe] { &db().recipes }
/// Les objets acceptés pour un ingrédient : le groupe entier, ou l'objet lui-même.
pub fn members(need: &str) -> Vec<&'static str> { db().members(need) }
pub fn group_label(need: &str) -> Option<&'static str> { db().group_label(need) }
pub fn food(id: &str) -> Option<Food> { def(id).and_then(|d| d.food) }
/// Un outil ne s'empile pas : une pile de taille 1.
pub fn stackable(id: &str) -> bool { def(id).is_some_and(|d| d.stack > 1) }
pub fn max_stack(id: &str) -> u32 { def(id).map_or(64, |d| d.stack.max(1)) }
pub fn is_axe(id: Option<&str>) -> bool { id.and_then(def).and_then(|d| d.tool).is_some_and(|t| t.kind == "hache") }
/// La voix d'un coup : celle de l'outil d'acier de même forme, ou rien (le poing).
pub fn voice(id: Option<&'static str>) -> Option<&'static str> { id.and_then(def).and_then(|d| d.tool).map(|t| t.voice()) }
/// Coup selon ce qu'on tient, et temps avant le suivant.
pub fn weapon(id: Option<&str>) -> (f64, f64) { id.and_then(def).and_then(|d| d.tool).map_or((2.0, 0.4), |t| t.strike()) }

/// Tout casse plus lentement qu'au prototype : la dureté du registre est multipliée par ce facteur.
pub const HARD: f64 = 4.0;
/// Vitesse de casse à mains nues ou avec un outil qui ne convient pas, selon la catégorie du bloc.
pub fn bare_rate(cat: &str) -> f64 { match cat { "rock" => 0.25, "wood" => 0.45, _ => 1.0 } }
/// À mains nues, la roche s'effrite sans rien donner : il faut une pioche.
pub fn needs_tool(cat: &str) -> bool { cat == "rock" }
/// Palier de pioche qu'exige un minerai : le fer veut au moins du cuivre.
pub fn ore_tier(block: &str) -> u8 { match block { "iron_ore" => 2, _ => 1 } }
/// Ce que l'herbe arrachée laisse parfois : fibre (toujours au couteau ou à la faucille), graines.
pub fn grass_drops(block: &str) -> &'static [(&'static str, f64)] { match block { "tall_grass" | "feather_grass" | "frost_grass" | "cotton_grass" | "fern" | "fern_dry" | "heather" | "moor_grass" => &[("fiber", 0.4), ("seeds", 0.15)], "gravel" => &[("flint", 0.2)], _ => &[] } }
/// Ce qu'on récolte en plus du bloc : les baies tombent du buisson, le blé mûr donne aussi ses graines.
pub fn bonus_drop(block: &str) -> Option<(&'static str, u32)> { match block { "blueberry" => Some(("berries", 3)), "rowan" => Some(("rowan_berries", 3)), "mushroom" => Some(("mushroom", 1)), "wheat_3" => Some(("seeds", 2)), "carrot_3" => Some(("carrot", 2)), "potato_3" => Some(("potato", 3)), _ => None } }
/// Ramassé d'un clic droit : le bloc disparaît et l'objet va dans le sac.
pub fn pickup(block: &str) -> Option<(&'static str, u32)> { match block { "twigs" => Some(("stick", 1)), "pebbles" => Some(("pebble", 1)), _ => None } }
/// Buissons qui se cueillent au clic droit : les baies, et le buisson dépouillé qui les remplace pour deux jours.
pub fn harvest(block: &str) -> Option<(&'static str, u32, &'static str)> { match block { "blueberry" => Some(("berries", 3, "blueberry_picked")), "rowan" => Some(("rowan_berries", 3, "rowan_picked")), _ => None } }
pub const REGROW_HOURS: f64 = 48.0; // deux jours de jeu
/// Le blé passe un stade toutes les douze heures : semé un matin, mûr le surlendemain soir.
pub const CROP_STAGE_HOURS: f64 = 12.0;

// Rythmes. Une journée dure 1500 s : sans manger, on tient un peu plus d'une journée et demie ; sans boire, un peu plus d'une.
pub const HUNGER_RATE: f64 = 0.04; pub const THIRST_RATE: f64 = 0.06;
/// La fatigue monte lentement : il faut dormir à peu près toutes les deux nuits.
pub const FATIGUE_RATE: f64 = 0.03;
pub const STAMINA_SPRINT: f64 = 12.0; pub const STAMINA_JUMP: f64 = 5.0; pub const STAMINA_ROLL: f64 = 15.0; pub const STAMINA_HIT: f64 = 4.0; pub const STAMINA_SWIM: f64 = 4.0;
/// L'endurance revient vite, mais seulement après une seconde de répit.
pub const STAMINA_REGEN: f64 = 22.0; pub const STAMINA_WAIT: f64 = 1.0; pub const WINDED_UNTIL: f64 = 30.0;
/// Chaque coup, dans le vide, dans un bloc ou sur un animal, coûte un peu d'endurance.
pub const STAMINA_SWING: f64 = 1.5;
/// Coup lourd : temps de charge, dégâts multipliés, endurance multipliée.
pub const HEAVY_T: f64 = 0.75; pub const HEAVY_MUL: f64 = 2.2; pub const HEAVY_COST: f64 = 2.5;
/// Garde : une parade dans ce délai après avoir levé le bouclier pare tout et fait reculer la bête.
pub const PARRY_T: f64 = 0.3;
/// Part d'une attaque de face qu'arrête un bouclier, selon son armure.
pub fn block_share(armor: f64) -> f64 { (0.35 + armor / 40.0).min(0.85) }
/// Combien de coups encaisse un bouclier avant de se fendre.
pub fn shield_life(armor: f64, profile: usize) -> u32 { ((40.0 + armor * 15.0) / PROFILES[profile].wear).round() as u32 }
/// Quinze secondes de souffle sous l'eau, puis la noyade.
pub const AIR_TIME: f64 = 15.0;
/// Vitesse d'arrivée au sol sans dommage : une chute d'environ trois blocs et demi.
pub const SAFE_FALL: f64 = 13.5;
/// Charge portée sans gêne (kg) ; un sac à dos en ajoute.
/// Charge portée sans gêne : de quoi construire (une quarantaine de pierres) avant de ralentir ; le sac à dos en ajoute.
pub const CARRY: f64 = 110.0;
/// Un feu de camp neuf brûle dix minutes ; le combustible des objets compte dix fois sa valeur.
pub const FIRE_START: f64 = 600.0; pub const FUEL_SCALE: f64 = 10.0;

/// Profils de survie : Exploration, Standard, Exigeant.
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Profile { pub name: &'static str, pub needs: f64, pub fatigue: f64, pub harm: f64, pub wear: f64, pub harvest: f64, pub climate: f64, pub stash_days: f64 }
pub const PROFILES: [Profile; 3] = [
    Profile { name: "Exploration", needs: 0.6, fatigue: 0.6, harm: 0.7, wear: 0.6, harvest: 1.25, climate: 0.6, stash_days: 10.0 },
    Profile { name: "Standard", needs: 1.0, fatigue: 1.0, harm: 1.0, wear: 1.0, harvest: 1.0, climate: 1.0, stash_days: 5.0 },
    Profile { name: "Exigeant", needs: 1.3, fatigue: 1.3, harm: 1.3, wear: 1.35, harvest: 0.85, climate: 1.4, stash_days: 3.0 },
];

/// Compétences : on s'améliore un peu en pratiquant, jusqu'à quinze pour cent, sans corvée de répétition.
pub const SKILLS: [&str; 5] = ["Bûcheronnage", "Extraction", "Chasse", "Artisanat", "Cuisine"];
pub fn skill_bonus(n: f64) -> f64 { 1.0 + 0.15 * (n / 150.0).min(1.0) }

/// Température de l'air ressentie, en degrés, avant vêtements : biome, altitude, heure, météo, abri, eau, feu.
#[derive(Clone, Copy, Debug, Default)] pub struct Climate { pub biome: f64, pub day_amp: f64, pub altitude: f64, pub sun: f64, pub cloud: f64, pub rain: f64, pub covered: bool, pub in_water: bool, pub fire: f64 }
pub fn air_temp(c: Climate) -> f64 {
    let mut t = c.biome - 0.3 * (c.altitude - 70.0).max(0.0) + c.day_amp * c.sun - 2.0 * c.cloud - 5.0 * c.rain;
    if c.covered { t += (17.0 - t) * 0.35; } // un abri tempère
    if c.in_water { t -= 8.0; }
    t + c.fire
}
/// Inconfort thermique : négatif quand on a froid, positif quand on a chaud, nul entre 10 et 26 degrés ressentis.
/// Les vêtements chauds aident contre le froid mais pèsent dans la chaleur ; le frais (chapeau) aide au soleil.
pub fn exposure(air: f64, warm: f64, cool: f64) -> f64 {
    let felt = air + 1.8 * warm; if felt < 10.0 { return felt - 10.0; }
    let hot = air + 0.6 * warm.max(0.0) - 2.0 * cool; if hot > 26.0 { hot - 26.0 } else { 0.0 }
}

/// Gibier : points de vie, ce qu'il laisse (et en plus, au couteau), la boîte qui sert à le viser.
pub const WOLF_HP: f64 = 16.0; pub const BOAR_HP: f64 = 22.0;
pub const BEAR_HP: f64 = 42.0; pub const IBEX_HP: f64 = 12.0; pub const LIZARD_HP: f64 = 3.0;
pub const BEAR_LOOT: &[(&str, u32)] = &[("fur", 3), ("raw_meat", 5), ("raw_hide", 2)]; pub const IBEX_LOOT: &[(&str, u32)] = &[("raw_meat", 2), ("raw_hide", 1)]; pub const LIZARD_LOOT: &[(&str, u32)] = &[("raw_meat", 1)];
pub const WOLF_LOOT: &[(&str, u32)] = &[("fur", 2), ("raw_meat", 1), ("bone", 1)]; pub const BOAR_LOOT: &[(&str, u32)] = &[("raw_meat", 4), ("leather", 1)];
pub const DEER_HP: f64 = 14.0; pub const FOX_HP: f64 = 8.0; pub const HARE_HP: f64 = 4.0;
pub const DEER_LOOT: &[(&str, u32)] = &[("raw_meat", 3), ("raw_hide", 2), ("bone", 1)]; pub const FOX_LOOT: &[(&str, u32)] = &[("fur", 1), ("raw_meat", 1)]; pub const HARE_LOOT: &[(&str, u32)] = &[("raw_meat", 2)];
pub const DEER_KNIFE: &[(&str, u32)] = &[("raw_meat", 1), ("raw_hide", 1)]; pub const FOX_KNIFE: &[(&str, u32)] = &[("fur", 1)]; pub const HARE_KNIFE: &[(&str, u32)] = &[("fur", 1)];
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
/// L'armure retient une part des coups : 20 points en arrêtent la moitié.
pub fn armor_cut(armor: f64) -> f64 { 1.0 - armor / (armor + 20.0) }

/// Pourquoi on est mort, et où sont restées les affaires (avec l'heure de la mort : la dépouille finit par disparaître).
#[derive(Clone, Debug, PartialEq)] pub struct Death { pub cause: &'static str, pub at: [f64; 3] }
#[derive(Clone, Debug, PartialEq)] pub struct Stash { pub p: [f64; 3], pub items: Vec<crate::session::Stack>, pub born: f64 }
/// Le sac laissé à la mort : cuir, rabat, sangle.
pub fn parts() -> Vec<(&'static str, VBuf)> {
    let mut o = VBuf::new(60); let (bag, flap, strap) = ([0.42, 0.27, 0.15], [0.52, 0.35, 0.2], [0.26, 0.16, 0.09]);
    b(&mut o, [-0.22, 0.0, -0.15, 0.22, 0.36, 0.15], bag); b(&mut o, [-0.23, 0.26, -0.16, 0.23, 0.4, 0.16], flap); b(&mut o, [-0.04, 0.12, 0.15, 0.04, 0.32, 0.18], strap); b(&mut o, [-0.025, 0.4, -0.03, 0.025, 0.5, 0.03], strap);
    // Flèche : hampe le long de +z, pointe de silex devant, empennage de fibre derrière. Bouchon rouge et blanc.
    let mut arrow = VBuf::new(60); b(&mut arrow, [-0.012, -0.012, -0.35, 0.012, 0.012, 0.3], [0.55, 0.4, 0.22]); b(&mut arrow, [-0.025, -0.02, 0.3, 0.025, 0.02, 0.4], [0.3, 0.32, 0.36]); b(&mut arrow, [-0.045, -0.004, -0.35, 0.045, 0.004, -0.22], [0.72, 0.66, 0.38]); b(&mut arrow, [-0.004, -0.045, -0.35, 0.004, 0.045, -0.22], [0.72, 0.66, 0.38]);
    let mut bobber = VBuf::new(60); b(&mut bobber, [-0.05, 0.0, -0.05, 0.05, 0.06, 0.05], [0.9, 0.12, 0.08]); b(&mut bobber, [-0.05, -0.06, -0.05, 0.05, 0.0, 0.05], [0.95, 0.94, 0.9]); b(&mut bobber, [-0.01, 0.06, -0.01, 0.01, 0.14, 0.01], [0.2, 0.2, 0.2]);
    vec![("stash", o), ("arrow", arrow), ("bobber", bobber)]
}
