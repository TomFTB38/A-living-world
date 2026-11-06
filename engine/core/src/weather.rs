//! Météo déterministe par tranches de temps, transitions lentes, et son effet sur le ciel.
use crate::noise::hash_float;
use crate::view::Sky;
pub const SLOT: f64 = 210.0; // secondes de jeu par épisode
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Kind { Clair, Nuageux, Pluie, Brouillard }
pub const KINDS: [Kind; 4] = [Kind::Clair, Kind::Nuageux, Kind::Pluie, Kind::Brouillard];
impl Kind { pub fn name(self) -> &'static str { match self { Kind::Clair => "clair", Kind::Nuageux => "nuageux", Kind::Pluie => "pluie", Kind::Brouillard => "brouillard" } } }
#[derive(Clone, Copy, PartialEq, Debug)] pub struct State { pub cloud: f64, pub rain: f64, pub fog: f64 }
pub fn target(k: Kind) -> State { match k { Kind::Clair => State { cloud: 0.42, rain: 0.0, fog: 0.0 }, Kind::Nuageux => State { cloud: 0.8, rain: 0.0, fog: 0.1 }, Kind::Pluie => State { cloud: 1.0, rain: 1.0, fog: 0.35 }, Kind::Brouillard => State { cloud: 0.7, rain: 0.0, fog: 1.0 } } }
/// On découvre toujours le monde sous le soleil.
pub fn kind_at(seed: i32, t: f64) -> Kind {
    if t < SLOT { return Kind::Clair; }
    let r = hash_float(seed, (t / SLOT).floor() as i32, 17, 3);
    if r < 0.5 { Kind::Clair } else if r < 0.74 { Kind::Nuageux } else if r < 0.88 { Kind::Pluie } else { Kind::Brouillard }
}
pub struct Weather { seed: i32, pub t: f64, pub forced: Option<Kind>, pub state: State, pub kind: Kind }
impl Weather {
    pub fn new(seed: i32) -> Self { Weather { seed, t: 0.0, forced: None, state: target(Kind::Clair), kind: Kind::Clair } }
    pub fn update(&mut self, dt: f64) {
        self.t += dt; self.kind = self.forced.unwrap_or_else(|| kind_at(self.seed, self.t));
        let (g, k) = (target(self.kind), (dt / 14.0).min(1.0));
        self.state.cloud += (g.cloud - self.state.cloud) * k; self.state.rain += (g.rain - self.state.rain) * k; self.state.fog += (g.fog - self.state.fog) * k;
    }
    /// Change de temps sans transition, pour les captures.
    pub fn snap(&mut self, k: Kind) { self.forced = Some(k); self.kind = k; self.state = target(k); }
}
/// Ce que le rendu reçoit en plus du ciel.
#[derive(Clone, Copy, Debug)] pub struct Env { pub cloud: f32, pub fog_k: f32, pub rain: f32, pub mist: f32, pub covered: bool, pub bright: f32, pub under: bool, pub flies: f32 }
/// La météo agit sur l'astre, le ciel et la densité de la brume.
pub fn apply(s: &mut Sky, w: State, covered: bool) -> Env {
    let (r, f, c) = (w.rain as f32, w.fog as f32, w.cloud as f32);
    let dim = 1.0 - 0.72 * r.max(f * 0.75).max((c - 0.42) * 0.7); let k = (r * 0.85 + f * 0.6 + (c - 0.42).max(0.0) * 0.8).min(1.0); let day = 0.25 + 0.75 * ((s.dir[1] + 0.1) * 3.0).min(1.0);
    for i in 0..3 { s.col[i] *= dim; }
    let grey = |c: [f32; 3], g: [f32; 3]| [c[0] + (g[0] * day - c[0]) * k, c[1] + (g[1] * day - c[1]) * k, c[2] + (g[2] * day - c[2]) * k];
    s.top = grey(s.top, [0.42, 0.47, 0.54]); s.hor = grey(s.hor, [0.72, 0.76, 0.8]);
    Env { cloud: c, fog_k: 1.0 + 1.6 * r + 5.5 * f, rain: r, mist: 0.0045 * r + 0.024 * f, covered, bright: 1.0, under: false, flies: 0.0 }
}
