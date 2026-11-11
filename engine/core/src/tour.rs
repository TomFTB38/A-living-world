//! Le travelling de l'écran titre : la caméra survole le monde autour du point de départ, passe au-dessus des villages
//! voisins et traverse des biomes différents, puis revient. Le joueur est mis de côté le temps du survol (position,
//! regard, vol, mode) et retrouvé tel quel ; la sauvegarde garde toujours sa vraie place.
use crate::session::Session;
use std::f64::consts::TAU;

/// Ce que le survol emprunte au joueur, rendu à la fin.
#[derive(Clone, Copy, Debug)]
pub struct Home { pub p: [f64; 3], pub yaw: f64, pub pitch: f64, pub fly: bool, pub creative: bool }
#[derive(Clone, Debug)]
pub struct Tour { pub home: Home, path: Vec<[f64; 2]>, towns: Vec<[f64; 2]>, seg: usize, u: f64, y: f64, yaw: f64 }
/// Vitesse de croisière (blocs par seconde), plus lente au-dessus d'un village ; hauteur de vol au-dessus du relief.
const SPEED: f64 = 9.0; const SLOW: f64 = 4.5; const ABOVE: f64 = 17.0;

fn cr(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2], p3: [f64; 2], t: f64) -> [f64; 2] {
    let (t2, t3) = (t * t, t * t * t);
    let f = |a: f64, b: f64, c: f64, d: f64| 0.5 * (2.0 * b + (c - a) * t + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2 + (3.0 * b - a - 3.0 * c + d) * t3);
    [f(p0[0], p1[0], p2[0], p3[0]), f(p0[1], p1[1], p2[1], p3[1])]
}
impl Session {
    /// La boucle du survol : les villages proches et des lieux de biomes différents, reliés au plus court, retour au départ.
    fn tour_path(&self, from: [f64; 2]) -> (Vec<[f64; 2]>, Vec<[f64; 2]>) {
        let towns: Vec<[f64; 2]> = self.gen.villages_near(from[0], from[1], 1500.0).into_iter().take(3).map(|v| [v.c[0] as f64 + 0.5, v.c[2] as f64 + 0.5]).collect();
        let top = |x: f64, z: f64| { let m = self.gen.climate.at(x, z); (0..crate::biome::COUNT).max_by(|a, b| m.w[*a].total_cmp(&m.w[*b])).unwrap_or(0) };
        let mut seen = vec![top(from[0], from[1])]; let mut wild = Vec::new();
        for r in [380.0, 620.0] { for k in 0..12 { if wild.len() >= 3 { break; } let a = k as f64 * TAU / 12.0 + r * 0.01; let q = [from[0] + r * a.cos(), from[1] + r * a.sin()];
            let c = self.gen.column(q[0], q[1]); if c.water != 0.0 { continue; } let b = top(q[0], q[1]); if !seen.contains(&b) { seen.push(b); wild.push(q); } } }
        if wild.is_empty() { wild.push([from[0] + 300.0, from[1] + 120.0]); }
        // Au plus proche voisin, puis retour.
        let mut left: Vec<[f64; 2]> = towns.iter().chain(wild.iter()).copied().collect(); let mut path = vec![from]; let mut at = from;
        while !left.is_empty() { let i = (0..left.len()).min_by(|a, b| (left[*a][0] - at[0]).hypot(left[*a][1] - at[1]).total_cmp(&(left[*b][0] - at[0]).hypot(left[*b][1] - at[1]))).unwrap(); at = left.swap_remove(i); path.push(at); }
        // Des étapes intermédiaires tous les 120 blocs environ : la courbe reste près de la ligne voulue.
        let mut fine = Vec::new(); let n = path.len();
        for i in 0..n { let (a, b) = (path[i], path[(i + 1) % n]); let d = (b[0] - a[0]).hypot(b[1] - a[1]); let k = (d / 120.0).ceil().max(1.0) as usize;
            for j in 0..k { let t = j as f64 / k as f64; fine.push([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]); } }
        (fine, towns)
    }
    /// Un pas du survol (à appeler à chaque image tant que l'écran titre est affiché).
    pub fn tour_step(&mut self, dt: f64) {
        if self.tour.is_none() {
            let p = &self.player; let home = Home { p: p.p, yaw: p.yaw, pitch: p.pitch, fly: p.fly, creative: self.creative };
            let (path, towns) = self.tour_path([p.p[0], p.p[2]]); let y = p.p[1] + 8.0; let yaw = p.yaw;
            self.tour = Some(Tour { home, path, towns, seg: 0, u: 0.0, y, yaw }); self.creative = true;
        }
        let Some(mut t) = self.tour.take() else { return }; let n = t.path.len();
        let at = |t: &Tour, seg: usize, u: f64| { let g = |k: i64| t.path[((seg as i64 + k).rem_euclid(n as i64)) as usize]; cr(g(-1), g(0), g(1), g(2), u) };
        let here = at(&t, t.seg, t.u);
        let near = t.towns.iter().map(|c| (c[0] - here[0]).hypot(c[1] - here[1])).fold(f64::MAX, f64::min);
        let v = SLOW + (SPEED - SLOW) * ((near - 30.0) / 80.0).clamp(0.0, 1.0);
        let len = { let b = at(&t, t.seg, (t.u + 0.05).min(1.0)); ((b[0] - here[0]).hypot(b[1] - here[1]) / 0.05).max(1.0) };
        t.u += v * dt.min(0.1) / len; while t.u >= 1.0 { t.u -= 1.0; t.seg = (t.seg + 1) % n; }
        let p = at(&t, t.seg, t.u); let ahead = at(&t, t.seg, (t.u + 0.08).min(1.0));
        // Le relief sous soi et un peu devant : on passe au-dessus des collines sans les raser.
        let mut ground: f64 = 0.0; for k in 0..5 { let q = at(&t, t.seg, (t.u + k as f64 * 0.04).min(1.0)); let c = self.gen.column(q[0], q[1]); ground = ground.max(c.h.max(if c.water != 0.0 { c.water } else { 0.0 })); }
        let goal = ground + ABOVE + 6.0 * (1.0 - (near / 60.0).clamp(0.0, 1.0)) * -0.5; t.y += (goal - t.y) * (dt * 0.6).min(1.0);
        // On regarde où l'on va ; près d'un village, on le garde en vue.
        let mut want = (ahead[0] - p[0]).atan2(ahead[1] - p[1]);
        if let Some(c) = t.towns.iter().find(|c| (c[0] - p[0]).hypot(c[1] - p[1]) < 70.0) { let tw = (c[0] - p[0]).atan2(c[1] - p[1]); let w = 0.55 * (1.0 - (c[0] - p[0]).hypot(c[1] - p[1]) / 70.0);
            let d = (tw - want + std::f64::consts::PI).rem_euclid(TAU) - std::f64::consts::PI; want += d * w; }
        let d = (want - t.yaw + std::f64::consts::PI).rem_euclid(TAU) - std::f64::consts::PI; t.yaw += d * (dt * 0.8).min(1.0);
        self.player.p = [p[0], t.y, p[1]]; self.player.v = [0.0; 3]; self.player.fly = true; self.player.yaw = t.yaw; self.player.pitch = -0.28 - 0.1 * (1.0 - (near / 70.0).clamp(0.0, 1.0));
        self.tour = Some(t);
    }
    /// Fin du survol : le joueur retrouve sa place, son regard, son vol et son mode.
    pub fn tour_end(&mut self) {
        let Some(t) = self.tour.take() else { return }; let h = t.home;
        self.player.p = h.p; self.player.yaw = h.yaw; self.player.pitch = h.pitch; self.player.fly = h.fly; self.player.v = [0.0; 3]; self.creative = h.creative; self.dirty = true;
    }
    /// La vraie place du joueur (pendant un survol, celle qu'il avait avant).
    pub fn home(&self) -> Home { self.tour.as_ref().map_or(Home { p: self.player.p, yaw: self.player.yaw, pitch: self.player.pitch, fly: self.player.fly, creative: self.creative }, |t| t.home) }
}
