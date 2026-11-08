//! Chasse à l'arc et pêche à la ligne. Les flèches sont de vrais projectiles (vitesse, gravité) qui touchent le gibier
//! ou se plantent dans le décor, d'où on les ramasse ; la ligne attend une touche, qu'il faut saisir au bon moment.
//! Le poisson dépend de l'eau : courante (truite), dormante (perche), froide du nord et des hauteurs (omble).
use crate::blocks::*;
use crate::noise::hash_float;
use crate::player::Voxels;
use crate::session::*;
use crate::survival::*;

#[derive(Clone, Copy, Debug, PartialEq)] pub struct Arrow { pub p: [f64; 3], pub v: [f64; 3], pub stuck: bool, pub life: f64 }
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Line { pub spot: [f64; 3], pub bite: f64, pub told: bool, pub fish: &'static str }
pub const ARROW_SPEED: f64 = 34.0; const ARROW_GRAVITY: f64 = 12.0; const ARROW_DMG: f64 = 9.0;
/// Une touche se saisit dans cette fenêtre (secondes), sinon le poisson file.
pub const BITE_WINDOW: f64 = 1.8;

impl Session {
    fn rand(&self, salt: i32) -> f64 { hash_float((self.elapsed * 613.0) as i32, salt, self.seed, 41) }
    /// Clic droit avec l'arc ou la canne. Vrai si le clic a servi.
    pub(crate) fn outdoor_use(&mut self) -> bool {
        match self.inv[self.sel].map(|s| s.item) {
            Some("bow") => {
                if self.use_t > 0.0 { return true; }
                if self.count("arrow") == 0 { self.notes.push("Plus de flèches".into()); self.use_t = 0.8; return true; }
                if !self.creative { self.take_one("arrow"); self.spend(4.0); }
                let (e, f) = (self.eye, self.look); self.arrows.push(Arrow { p: [e[0] + f[0] * 0.6, e[1] + f[1] * 0.6, e[2] + f[2] * 0.6], v: [f[0] * ARROW_SPEED, f[1] * ARROW_SPEED, f[2] * ARROW_SPEED], stuck: false, life: 0.0 });
                self.sound.swing(Some("sword"), 1.3); self.anim.trigger(); self.use_t = 0.7; true
            }
            Some("fishing_rod") => {
                if self.use_t > 0.0 { return true; } self.use_t = 0.5; self.anim.trigger();
                if let Some(l) = self.line.take() {
                    let now = self.elapsed;
                    if now >= l.bite && now <= l.bite + BITE_WINDOW { self.add_item(l.fish, 1); self.notes.push(format!("+1 {}", item(l.fish).map_or(l.fish, |i| i.label))); self.sound.splash(0.4); self.skills[2] += 1.0; }
                    else { self.notes.push("Ligne ramenée".into()); }
                    return true;
                }
                // On lance vers l'eau visée, jusqu'à quatorze blocs.
                let (e, f) = (self.eye, self.look); let mut t = 0.5;
                while t < 14.0 { let q = [e[0] + f[0] * t, e[1] + f[1] * t, e[2] + f[2] * t]; let c = [q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32];
                    match self.world.get(c[0], c[1], c[2]) {
                        Some(b) if is_water(b) => {
                            let col = self.gen.column(q[0], q[2]); let cold = self.taiga > 0.4 || self.highlands > 0.4; let running = b != B::WATER || !col.still;
                            let fish = if cold { "char" } else if running { "trout" } else { "perch" };
                            let wait = (5.0 + 9.0 * self.rand(3)) / (1.0 + self.skills[2] * 0.01).min(1.5);
                            self.line = Some(Line { spot: [q[0], c[1] as f64 + 0.92, q[2]], bite: self.elapsed + wait, told: false, fish }); self.sound.splash(0.15); return true;
                        }
                        Some(b) if b != 0 && is_solid(b) => break, None => break, _ => {}
                    } t += 0.25; }
                self.notes.push("Visez une étendue d'eau".into()); true
            }
            _ => false,
        }
    }
    fn take_one(&mut self, id: &str) { if let Some(s) = self.inv.iter_mut().find(|s| s.is_some_and(|s| s.item == id)) { let st = s.unwrap(); let n = st.n.unwrap_or(1) - 1; *s = if n == 0 { None } else { Some(Stack { n: Some(n), ..st }) }; self.dirty = true; } }
    /// Vol des flèches, touches, flèches plantées à ramasser ; la ligne qui attend une touche.
    pub(crate) fn outdoors(&mut self, dt: f64) {
        let me = self.player.p; let mut arrows = std::mem::take(&mut self.arrows);
        for a in arrows.iter_mut() {
            a.life += dt;
            if a.stuck { if (a.p[0] - me[0]).hypot(a.p[2] - me[2]) < 1.3 && (a.p[1] - me[1] - 0.5).abs() < 1.6 { if self.add_item("arrow", 1) == 0 { a.life = 1e9; } } continue; }
            a.v[1] -= ARROW_GRAVITY * dt; let step = [a.v[0] * dt, a.v[1] * dt, a.v[2] * dt]; let len = (step[0] * step[0] + step[1] * step[1] + step[2] * step[2]).sqrt(); let d = [step[0] / len, step[1] / len, step[2] / len];
            // Le gibier d'abord : la boîte la plus proche le long du trajet de cette image.
            let mut best: Option<(f64, crate::session::Prey)> = None;
            for (k, dr) in self.fauna.deer.iter().enumerate() { if let Some(t) = ray_box(a.p, d, dr.p, 0.55, 1.6) { if t <= len && best.is_none_or(|b| t < b.0) { best = Some((t, Prey::Deer(k))); } } }
            for (k, c) in self.wild.all.iter().enumerate() { if let Some((hw, h)) = wild_box(c.kind) { if let Some(t) = ray_box(a.p, d, c.p, hw, h) { if t <= len && best.is_none_or(|b| t < b.0) { best = Some((t, Prey::Wild(k))); } } } }
            if let Some((_, prey)) = best { self.arrow_hit(prey, a.p); a.life = 1e9; continue; }
            // Puis le décor : la flèche se plante là où elle touche.
            let mut t = 0.0; let mut hit = false;
            while t < len { let q = [a.p[0] + d[0] * t, a.p[1] + d[1] * t, a.p[2] + d[2] * t]; if self.world.get(q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32).is_none_or(|b| b != 0 && is_solid(b)) { a.p = q; a.stuck = true; a.life = 0.0; hit = true; break; } t += 0.1; }
            if !hit { a.p = [a.p[0] + step[0], a.p[1] + step[1], a.p[2] + step[2]]; if a.life > 6.0 { a.life = 1e9; } } else { self.sound.thump(a.p); }
        }
        arrows.retain(|a| a.life < if a.stuck { 600.0 } else { 6.0 }); self.arrows = arrows;
        if let Some(mut l) = self.line {
            let far = (l.spot[0] - me[0]).hypot(l.spot[2] - me[2]) > 20.0 || self.inv[self.sel].map(|s| s.item) != Some("fishing_rod");
            if far { self.line = None; } else {
                let now = self.elapsed;
                if !l.told && now >= l.bite { l.told = true; self.notes.push("Ça mord ! Clic droit".into()); self.sound.splash(0.35); }
                if now > l.bite + BITE_WINDOW { l.told = false; l.bite = now + 4.0 + 7.0 * self.rand(9); self.notes.push("Le poisson s'est échappé".into()); }
                self.line = Some(l);
            }
        }
    }
    fn arrow_hit(&mut self, prey: Prey, from: [f64; 3]) {
        let loot: &[(&str, u32)] = match prey {
            Prey::Deer(k) => { let at = self.fauna.deer[k].p; self.sound.thump(at); if self.fauna.strike(k, ARROW_DMG, from) { DEER_LOOT } else { &[] } }
            Prey::Wild(k) => { let at = self.wild.all[k].p; self.sound.thump(at); self.wild.strike(k, ARROW_DMG, from).map_or(&[], wild_loot) }
        };
        if !loot.is_empty() && !self.creative { let mut said = Vec::new(); for (id, n) in loot { self.add_item(id, *n); if let Some(i) = item(id) { said.push(format!("+{n} {}", i.label)); } } self.notes.push(said.join(", ")); self.skills[2] += 2.0; self.dirty = true; }
    }
}
/// Boîte de visée d'une bête (demi-largeur, hauteur) ; les oiseaux et virevoltants ne se chassent pas.
pub fn wild_box(k: crate::wild::Kind) -> Option<(f64, f64)> { use crate::wild::Kind::*; match k { Fox => Some((0.35, 0.7)), Hare => Some((0.28, 0.5)), Wolf => Some((0.45, 1.0)), Boar => Some((0.5, 0.85)), _ => None } }
pub fn wild_loot(k: crate::wild::Kind) -> &'static [(&'static str, u32)] { use crate::wild::Kind::*; match k { Fox => FOX_LOOT, Hare => HARE_LOOT, Wolf => WOLF_LOOT, Boar => BOAR_LOOT, _ => &[] } }
