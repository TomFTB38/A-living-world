//! Les points de vue. Sur certains sommets qui dominent nettement leurs alentours, un belvédère : un dallage de pierre et
//! un mât à drapeau (`gen::view_stamp`). Monter dessus et s'y tenir : la carte se dessine à 250 m à la ronde (le relief,
//! les forêts, les eaux, la neige des cimes), les lieux et les villages alentour sont repérés et marqués sur la carte, et
//! l'on gagne de l'expérience. Les points de vue déjà vus de loin apparaissent sur la carte et la boussole.
//! Vérifié par `tests/panoramas.rs`.
use crate::biome::Biome;
use crate::map::{pack, STEP};
use crate::session::Session;

/// Le rayon que dessine un point de vue, l'expérience qu'il rapporte, et la distance à laquelle la boussole le montre.
pub const VIEW_R: f64 = 250.0; pub const VIEW_XP: f64 = 80.0; pub const VIEW_PIN: f64 = 260.0;

/// Ce qu'on devine d'en haut, de loin : la couleur d'un point de la carte tirée du terrain (biome, forêt, eau, chemins,
/// rochers et neige des cimes), un peu plus pâle que ce qu'on a vu de près.
pub fn sketch(g: &crate::gen::WorldGen, x: f64, z: f64) -> u32 {
    let c = g.column(x, z); let sea = crate::gen::SEA;
    if c.water != 0.0 || c.h < sea { let d = ((sea - c.h) / 14.0).clamp(0.0, 1.0); let f = 1.0 - d * 0.4; return pack([(70.0 * f) as u8, (114.0 * f) as u8, (172.0 * f + 20.0) as u8], c.h.max(sea) as i32); }
    let m = &c.mix; let mut col = [0.0f64; 3]; let mut w = 0.0;
    for (b, rgb) in [(Biome::Temperate, [86.0, 122.0, 58.0]), (Biome::Meadow, [118.0, 150.0, 72.0]), (Biome::DeepForest, [58.0, 92.0, 46.0]), (Biome::Taiga, [70.0, 98.0, 78.0]), (Biome::Steppe, [164.0, 154.0, 96.0]), (Biome::Desert, [216.0, 192.0, 136.0]), (Biome::Highlands, [116.0, 128.0, 88.0]), (Biome::Mountains, [128.0, 126.0, 118.0])] {
        let k = m.of(b); for i in 0..3 { col[i] += rgb[i] * k; } w += k; }
    if w > 0.0 { for v in col.iter_mut() { *v /= w; } } else { col = [100.0, 130.0, 70.0]; }
    let dark = 1.0 - 0.25 * c.forest.clamp(0.0, 1.0); for v in col.iter_mut() { *v *= dark; }
    if c.cliff > 0.4 { col = [126.0, 122.0, 114.0]; }
    if c.h > 150.0 { let k = ((c.h - 150.0) / 20.0).clamp(0.0, 1.0); for (i, s) in [236.0, 240.0, 246.0].iter().enumerate() { col[i] += (s - col[i]) * k; } }
    if let Some((rare, k)) = g.rare_at(x, z) { let t = match rare { crate::gen::Rare::Cerisiers => [226.0, 168.0, 190.0], crate::gen::Rare::Automne => [196.0, 96.0, 44.0] }; for i in 0..3 { col[i] += (t[i] - col[i]) * 0.75 * k; } }
    if c.path > 0.3 { col = [150.0, 124.0, 86.0]; }
    for v in col.iter_mut() { *v += (200.0 - *v) * 0.12; } // le croquis : un peu pâli
    pack([col[0] as u8, col[1] as u8, col[2] as u8], c.h as i32)
}

impl Session {
    /// Debout sur le dallage d'un point de vue qu'on n'a pas encore fait : le panorama.
    pub(crate) fn view_tick(&mut self) {
        if (self.elapsed * 4.0).floor() == ((self.elapsed - 1.0 / 60.0) * 4.0).floor() && self.elapsed > 0.1 { return; } // quatre fois par seconde
        let p = self.player.p; if !self.player.ground || self.dead.is_some() { return; }
        for (cell, top) in self.gen.viewpoints_near(p[0], p[2], 4.0) {
            if self.views.contains(&cell) { continue; }
            let d = (top[0] as f64 + 0.5 - p[0]).hypot(top[2] as f64 + 0.5 - p[2]); if d > 1.9 || p[1] < top[1] as f64 + 0.9 || p[1] > top[1] as f64 + 1.6 { continue; }
            self.sync_view(cell, top);
        }
    }
    /// Le panorama d'un point de vue : la carte dessinée alentour, les lieux et villages repérés, l'expérience.
    pub fn sync_view(&mut self, cell: (i32, i32), top: [i32; 3]) -> usize {
        if !self.views.insert(cell) { return 0; }
        let (cx, cz) = (top[0] as f64 + 0.5, top[2] as f64 + 0.5); let r = (VIEW_R / STEP as f64) as i32; let (px0, pz0) = ((cx / STEP as f64).floor() as i32, (cz / STEP as f64).floor() as i32);
        let mut drawn = 0;
        for dz in -r..=r { for dx in -r..=r { if dx * dx + dz * dz > r * r { continue; } let (px, pz) = (px0 + dx, pz0 + dz); if self.map.get(px, pz) != 0 { continue; }
            let v = sketch(&self.gen, (px * STEP + STEP / 2) as f64, (pz * STEP + STEP / 2) as f64); self.map.set(px, pz, v); drawn += 1; } }
        // les lieux et les villages en vue
        let mut seen: Vec<String> = Vec::new();
        for pl in self.gen.places_near(cx, cz, VIEW_R) { if self.prog.places.insert(pl.cell) { seen.push(pl.kind.name().to_string()); } }
        for v in self.gen.villages_near(cx, cz, VIEW_R) { if !self.town.known.contains(&v.id) { seen.push(v.name.clone()); } self.discover(&v); }
        self.earn(VIEW_XP, "point de vue"); self.sound.pickup(); self.dirty = true;
        self.notes.push("Point de vue ! La carte se dessine à 250 m à la ronde".into());
        if !seen.is_empty() { self.notes.push(format!("En vue : {}", seen.join(", "))); }
        drawn
    }
    /// Les points de vue que la carte connaît (faits, ou aperçus : leur sommet est sur la carte), avec s'ils sont faits.
    pub fn known_views(&self, r: f64) -> Vec<([f64; 3], bool)> {
        let p = self.player.p;
        self.gen.viewpoints_near(p[0], p[2], r).into_iter().filter_map(|(cell, t)| { let done = self.views.contains(&cell);
            (done || self.map.get(t[0].div_euclid(STEP), t[2].div_euclid(STEP)) != 0).then_some(([t[0] as f64 + 0.5, t[1] as f64 + 1.0, t[2] as f64 + 0.5], done)) }).collect()
    }
    /// Les lieux repérés (vus de près ou d'un point de vue), pour la carte : position et nom.
    pub fn known_places(&self, r: f64) -> Vec<([f64; 2], &'static str)> {
        let p = self.player.p; self.gen.places_near(p[0], p[2], r).into_iter().filter(|pl| self.prog.places.contains(&pl.cell)).map(|pl| ([pl.x as f64 + 0.5, pl.z as f64 + 0.5], pl.kind.name())).collect()
    }
}
