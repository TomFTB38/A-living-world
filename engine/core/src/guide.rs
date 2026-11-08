//! Premiers pas : une suite de conseils qui s'efface d'elle-même à mesure que le joueur les suit.
//! Chaque étape se valide sur ce qu'il a réellement obtenu (objets vus), ou au bout d'un moment pour les simples rappels.
use crate::session::Session;

/// Une étape : texte, condition de fin, et durée d'affichage au-delà de laquelle elle passe seule (rappels).
struct Step { text: fn(&Session) -> String, done: fn(&Session) -> bool, max: f64 }
fn has(s: &Session, ids: &[&str]) -> bool { ids.iter().any(|i| s.seen.contains(i)) }
fn log(s: &Session) -> bool { s.seen.iter().any(|i| i.ends_with("_log")) }
const STEPS: [Step; 8] = [
    Step { text: |_| "Clic droit sur les branches et les cailloux au sol pour les ramasser".into(), done: |s| has(s, &["stick"]) && has(s, &["pebble"]), max: f64::INFINITY },
    Step { text: |_| "Cassez l'herbe haute (clic gauche) : elle donne parfois de la fibre".into(), done: |s| has(s, &["fiber"]), max: f64::INFINITY },
    Step { text: |_| "E : le livre de recettes. Fabriquez un couteau de pierre".into(), done: |s| has(s, &["stone_knife", "stone_axe", "flint_axe"]), max: f64::INFINITY },
    Step { text: |_| "Une hache de pierre, puis abattez un arbre en frappant le tronc".into(), done: log, max: f64::INFINITY },
    Step { text: |_| "Des rondins : des planches, puis un établi à poser (clic droit)".into(), done: |s| has(s, &["workbench"]), max: f64::INFINITY },
    Step { text: |_| "Avant la nuit : un feu de camp, pour la chaleur et la cuisson".into(), done: |s| has(s, &["campfire"]), max: f64::INFINITY },
    Step { text: |_| "Boire : clic droit sur l'eau (mieux vaut la faire bouillir). Manger : baies, gibier cuit".into(), done: |_| false, max: 45.0 },
    Step { text: |s| match s.nearest_village(s.player.p[0], s.player.p[2]) {
        Some(v) => format!("M : la carte · J : le journal. Un village, {}, se trouve à {}", v.name, crate::village::where_is(s.player.p, &v)),
        None => "M : la carte · J : le journal des villages".into() }, done: |s| !s.town.known.is_empty() && s.guide_t > 8.0, max: 60.0 },
];
pub const STEP_COUNT: usize = STEPS.len();

impl Session {
    /// Le conseil du moment : (numéro, total, texte). Rien en mode créatif ni une fois le guide terminé.
    pub fn hint(&self) -> Option<(usize, usize, String)> { if self.creative || self.guide >= STEPS.len() { return None; } Some((self.guide + 1, STEPS.len(), (STEPS[self.guide].text)(self))) }
    pub(crate) fn guide_tick(&mut self, dt: f64) {
        if self.creative { return; }
        self.guide_t += dt;
        while self.guide < STEPS.len() && ((STEPS[self.guide].done)(self) || self.guide_t > STEPS[self.guide].max) { self.guide += 1; self.guide_t = 0.0; self.dirty = true; }
    }
    /// Passe le guide (réglage « conseils » coupé, ou joueur qui connaît déjà le jeu).
    pub fn skip_guide(&mut self) { self.guide = STEPS.len(); self.dirty = true; }
}
