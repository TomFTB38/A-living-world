//! Cartes au trésor et trésors cachés. Le monde sème des cachettes (`gen::cache_stamp`) : un cairn de pierres sèches sur
//! la terre ferme, et deux blocs sous lui un coffre enfoui. On peut tomber dessus en chemin ; une carte au trésor (trouvée
//! dans les coffres des lieux, des épaves, des cachettes elles-mêmes) en révèle une : la croix s'inscrit sur la carte, la
//! boussole et la mini-carte. Ouvrir le coffre la première fois rapporte de l'expérience. Vérifié par `tests/tresors.rs`.
use crate::session::Session;

/// Une carte ne mène ni à deux pas, ni au bout du monde.
pub const MAP_NEAR: f64 = 120.0; pub const MAP_FAR: f64 = 900.0;
/// L'expérience d'un trésor déterré.
pub const TREASURE_XP: f64 = 60.0;

/// La direction d'un point vu d'ici, en toutes lettres (le nord est vers -z, comme sur la boussole).
pub fn direction(dx: f64, dz: f64) -> &'static str {
    let a = dx.atan2(-dz).to_degrees().rem_euclid(360.0); // 0 : nord, 90 : est
    ["au nord", "au nord-est", "à l'est", "au sud-est", "au sud", "au sud-ouest", "à l'ouest", "au nord-ouest"][((a + 22.5) / 45.0) as usize % 8]
}

impl Session {
    /// Clic droit avec une carte au trésor en main : la cachette la plus proche qu'on n'a ni ouverte ni déjà marquée, entre
    /// 120 et 900 blocs, s'inscrit sur la carte ; la carte est lue (consommée). Rend vrai si elle l'a été.
    pub fn read_treasure_map(&mut self) -> bool {
        if self.use_t > 0.0 { return true; }
        let me = self.player.p; let marks = &self.treasure_marks; let chests = &self.chests;
        let found = self.gen.caches_near(me[0], me[2], MAP_FAR).into_iter().find(|p| (p[0] as f64 - me[0]).hypot(p[2] as f64 - me[2]) >= MAP_NEAR && !marks.contains(p) && !chests.contains_key(p));
        let Some(p) = found else { self.notes.push("Les traits de la carte ne correspondent à rien d'ici : essayez-la ailleurs".into()); self.use_t = 0.5; return true };
        self.consume_held(); self.treasure_marks.push(p); self.use_t = 0.5; self.dirty = true; self.sound.pickup();
        let (dx, dz) = (p[0] as f64 + 0.5 - me[0], p[2] as f64 + 0.5 - me[2]);
        self.notes.push(format!("La carte révèle un trésor enfoui à {:.0} m {} : cherchez le cairn, creusez dessous", dx.hypot(dz), direction(dx, dz)));
        true
    }
    /// Un coffre qu'on ouvre la première fois : si c'est celui d'une cachette, le trésor est trouvé.
    pub(crate) fn treasure_found(&mut self, p: [i32; 3]) {
        if self.chests.contains_key(&p) || !self.gen.cache_chest(p) { return; }
        let marked = self.treasure_marks.contains(&p); self.treasure_marks.retain(|q| *q != p);
        self.notes.push(if marked { "Le trésor de la carte !".into() } else { "Un trésor enfoui !".to_string() }); self.earn(TREASURE_XP, "trésor déterré"); self.sound.pickup();
    }
    /// Ce que contiendrait le coffre de ce point à sa première ouverture (bancs d'essai).
    pub fn loot_of(&self, p: [i32; 3]) -> Vec<Option<crate::session::Stack>> { self.village_loot(p) }
    /// Les trésors marqués qui restent à trouver (la boussole montre le plus proche).
    pub fn treasure_targets(&self) -> Vec<[f64; 3]> { self.treasure_marks.iter().filter(|p| !self.chests.contains_key(*p)).map(|p| [p[0] as f64 + 0.5, p[1] as f64, p[2] as f64 + 0.5]).collect() }
}
