//! Les feuilles qui tombent : dans une forêt d'automne, des feuilles rouges, orangées et dorées se détachent des érables et
//! des hêtres ; sous les cerisiers en fleur, ce sont des pétales roses. Elles tombent en tourbillonnant, poussées par le
//! vent, se posent au sol, y restent un moment puis disparaissent. Elles ne naissent que sous un vrai feuillage de la
//! bonne sorte, autour du joueur (les biomes rares surtout, et çà et là un bouleau doré dans une forêt ordinaire). Vérifié par `tests/feuilles.rs`.
use crate::actors::{b, Draw, Models};
use crate::blocks::{is_solid, B};
use crate::mesher::VBuf;
use crate::noise::Rng;
use crate::player::Voxels;
use crate::session::Session;

/// Combien en même temps, au plus ; combien il en naît par seconde sous un feuillage ; à quelle distance du joueur.
pub const MAX: usize = 400; pub const RATE: f64 = 80.0; pub const NEAR: f64 = 22.0;
/// Le temps qu'une feuille reste posée au sol avant de s'effacer.
pub const REST: f64 = 3.5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sorte { Rouge, Orange, Or, Petale }
#[derive(Clone, Copy, Debug)]
pub struct Feuille { pub p: [f64; 3], pub kind: Sorte, /** sa vitesse de chute, sa dérive, sa phase de tourbillon */ pub fall: f64, pub drift: [f64; 2], pub phase: f64, pub spin: f64, /** posée depuis */ pub rest: Option<f64> }

/// La sorte de feuille que lâche ce feuillage, s'il en lâche.
pub fn from_leaves(id: crate::blocks::Id) -> Option<Sorte> {
    if id == B::MAPLE_LEAVES_RED { Some(Sorte::Rouge) } else if id == B::MAPLE_LEAVES_ORANGE { Some(Sorte::Orange) } else if id == B::BIRCH_LEAVES_GOLD { Some(Sorte::Or) } else if id == B::CHERRY_LEAVES { Some(Sorte::Petale) } else { None }
}

impl Session {
    /// Faire naître, tomber, poser et s'effacer les feuilles.
    pub(crate) fn leaves_tick(&mut self, dt: f64) {
        let p = self.player.p; let t = self.elapsed;
        // naître : des points au hasard autour du joueur ; sous chacun, on cherche un feuillage qui en lâche, et la feuille part de dessous
        if self.leaves_fall.len() < MAX { // partout où il y a de ces feuillages (les biomes rares, et çà et là un bouleau doré)
            let mut r = Rng::new(((t * 1000.0) as u32) ^ 0x5eed); let n = (RATE * dt + r.next()).floor() as usize;
            for _ in 0..n { if self.leaves_fall.len() >= MAX { break; } let (a, d) = (r.next() * std::f64::consts::TAU, NEAR * r.next().sqrt()); let (x, z) = (p[0] + a.cos() * d, p[2] + a.sin() * d);
                let (xi, zi) = (x.floor() as i32, z.floor() as i32);
                // de haut en bas, à travers le houppier (feuilles, branches, troncs), jusqu'au dessous du feuillage
                for y in ((p[1] - 6.0) as i32..(p[1] + 26.0) as i32).rev() { let Some(id) = self.world.get(xi, y, zi) else { break };
                    if let Some(kind) = from_leaves(id) { if self.world.get(xi, y - 1, zi) == Some(0) {
                        let fall = match kind { Sorte::Petale => r.range(0.45, 0.7), _ => r.range(0.6, 1.0) };
                        self.leaves_fall.push(Feuille { p: [x, y as f64 - 0.05, z], kind, fall, drift: [r.range(-0.3, 0.3), r.range(-0.3, 0.3)], phase: r.next() * 6.3, spin: r.range(-3.0, 3.0), rest: None }); break; }
                        continue; }
                    if id != 0 && !crate::blocks::is_leaf(id) && !crate::blocks::is_log(id) && !crate::blocks::is_branch(id) { break; } } } }
        // tomber : en tourbillonnant (un balancement de feuille morte), poussées par le vent ; se poser sur ce qui est solide
        let w = &self.world; let wind = [0.35 * (t * 0.13).sin(), 0.25 * (t * 0.09).cos()];
        for f in self.leaves_fall.iter_mut() {
            if let Some(r) = f.rest.as_mut() { *r += dt; continue; }
            f.phase += dt * (2.2 + f.fall); let sway = f.phase.sin() * 0.55;
            let q = [f.p[0] + (f.drift[0] + wind[0] + sway * f.phase.cos() * 0.6) * dt, f.p[1] - f.fall * (0.75 + 0.35 * (2.0 * f.phase).cos().abs()) * dt, f.p[2] + (f.drift[1] + wind[1] + sway * 0.4) * dt];
            let below = w.get(q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32);
            match below { Some(id) if is_solid(id) || crate::blocks::is_water(id) => { f.p = [q[0], q[1].floor() + 1.01, q[2]]; f.rest = Some(0.0); } // posée (sur l'eau, elle flotte)
                None => { f.rest = Some(REST); } // hors du monde chargé : effacée
                _ => { f.p = q; } }
        }
        let (px, pz) = (p[0], p[2]); self.leaves_fall.retain(|f| f.rest.is_none_or(|r| r < REST) && (f.p[0] - px).hypot(f.p[2] - pz) < NEAR + 12.0);
    }
    /// Les feuilles en l'air et au sol, à dessiner : chacune tourne sur elle-même et bascule en tombant, à plat une fois posée.
    pub fn leaves_draws(&self, md: &Models) -> Vec<Draw> {
        use crate::view::{mul, roll_z, trs};
        self.leaves_fall.iter().map(|f| { let part = match f.kind { Sorte::Rouge => "leaf_red", Sorte::Orange => "leaf_orange", Sorte::Or => "leaf_gold", Sorte::Petale => "petal" };
            let m = match f.rest { None => mul(&trs(f.p[0], f.p[1], f.p[2], f.phase * f.spin * 0.4, 0.7 * f.phase.sin(), 1.6), &roll_z(0.5 * (f.phase * 1.3).cos())),
                Some(r) => { let s = (1.0 - ((r - (REST - 1.0)).max(0.0))).clamp(0.05, 1.0); trs(f.p[0], f.p[1] - 0.01, f.p[2], f.spin, 0.0, 1.6 * s) } }; // un peu plus grandes que nature, pour qu'on les voie tomber
            Draw { m, r: md.part(part), view: false, local: false, hero: false, dynamic: false, hurt: 0.0 } }).collect()
    }
}

/// Les modèles : une feuille d'érable (un limbe à pointes et sa tige), une feuille de hêtre dorée, un pétale ; à plat,
/// d'une douzaine de centimètres.
pub fn parts() -> Vec<(&'static str, VBuf)> {
    let maple = |c: [f64; 3]| { let mut o = VBuf::new(200); let dk = [c[0] * 0.75, c[1] * 0.75, c[2] * 0.75];
        b(&mut o, [-0.045, -0.004, -0.04, 0.045, 0.004, 0.04], c); b(&mut o, [-0.015, -0.004, 0.035, 0.015, 0.004, 0.075], c); // le limbe et sa pointe
        b(&mut o, [-0.075, -0.004, -0.01, -0.04, 0.004, 0.03], c); b(&mut o, [0.04, -0.004, -0.01, 0.075, 0.004, 0.03], c); // les deux lobes
        b(&mut o, [-0.006, -0.005, -0.075, 0.006, 0.005, -0.035], dk); o }; // la tige
    let mut gold = VBuf::new(200); b(&mut gold, [-0.035, -0.004, -0.05, 0.035, 0.004, 0.05], [0.9, 0.7, 0.2]); b(&mut gold, [-0.02, -0.004, 0.05, 0.02, 0.004, 0.07], [0.9, 0.7, 0.2]); b(&mut gold, [-0.005, -0.005, -0.07, 0.005, 0.005, -0.05], [0.5, 0.36, 0.14]);
    let mut petal = VBuf::new(100); b(&mut petal, [-0.025, -0.003, -0.03, 0.025, 0.003, 0.03], [0.98, 0.7, 0.82]); b(&mut petal, [-0.012, -0.003, 0.03, 0.012, 0.003, 0.04], [0.98, 0.8, 0.88]);
    vec![("leaf_red", maple([0.82, 0.16, 0.07])), ("leaf_orange", maple([0.95, 0.5, 0.1])), ("leaf_gold", gold), ("petal", petal)]
}
