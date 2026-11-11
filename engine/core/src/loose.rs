//! Objets au sol : ce qu'on jette, ce que rendent les blocs cassés et les bêtes tuées. Ils sautent un peu en tombant,
//! roulent, se posent, et vont dans le sac quand on passe dessus. Ils restent dix minutes de jeu, puis disparaissent.
use crate::blocks::is_solid;
use crate::player::Voxels;
use crate::session::{item, Session, Stack};

/// Un tas d'objets posé ou en l'air.
#[derive(Clone, Copy, Debug)]
pub struct Loose { pub item: &'static str, pub n: u32, pub wear: u32, pub p: [f64; 3], pub v: [f64; 3], pub yaw: f64, pub age: f64, pub wait: f64, pub rest: bool }
/// Durée de vie au sol, en secondes ; distance de ramassage ; attente avant de pouvoir ramasser ce qu'on vient de jeter.
pub const LIFE: f64 = 600.0; const REACH: f64 = 1.5; const THROWN_WAIT: f64 = 1.4;

impl Session {
    fn rnd(&self, k: i32) -> f64 { crate::noise::hash_float((self.elapsed * 733.0) as i32 + k * 31, self.loose.len() as i32, self.seed, 59) }
    /// Fait tomber `n` objets en `p`, avec un petit saut au hasard (un bloc qui se casse, une bête qui tombe).
    pub fn spill(&mut self, id: &str, n: u32, p: [f64; 3]) { self.spill_worn(id, n, 0, p); }
    pub(crate) fn spill_worn(&mut self, id: &str, n: u32, wear: u32, p: [f64; 3]) {
        let Some(it) = item(id) else { return }; if n == 0 { return; } self.seen.insert(it.id);
        let (a, b, c) = (self.rnd(1), self.rnd(2), self.rnd(3)); let ang = a * std::f64::consts::TAU;
        self.loose.push(Loose { item: it.id, n, wear, p, v: [ang.cos() * 1.6 * b, 3.2 + 1.5 * c, ang.sin() * 1.6 * b], yaw: a * 6.28, age: 0.0, wait: 0.35, rest: false });
        self.dirty = true;
    }
    /// Jette l'objet tenu devant soi (un seul, ou toute la pile).
    pub fn throw_held(&mut self, all: bool) {
        let Some(st) = self.inv[self.sel] else { return }; let n = if all { st.n.unwrap_or(1) } else { 1 };
        let left = st.n.map(|k| k - n.min(k)); self.inv[self.sel] = if left.is_none_or(|k| k == 0) { None } else { Some(Stack { n: left, ..st }) };
        let (e, f) = (self.player.eye(), self.look); let p = [e[0] + f[0] * 0.5, e[1] - 0.25 + f[1] * 0.5, e[2] + f[2] * 0.5];
        self.loose.push(Loose { item: st.item, n, wear: st.wear, p, v: [f[0] * 5.0, f[1] * 5.0 + 2.2, f[2] * 5.0], yaw: self.player.yaw, age: 0.0, wait: THROWN_WAIT, rest: false });
        self.sound.swing(None, 0.6); self.anim.trigger(); self.dirty = true;
    }
    /// Lâche hors de l'inventaire ce qu'on tient au curseur : il tombe devant soi.
    pub fn throw_cursor(&mut self) {
        let Some(st) = self.cursor.take() else { return }; if self.creative { self.dirty = true; return; } self.from = None;
        let (e, f) = (self.player.eye(), self.look); let p = [e[0] + f[0] * 0.5, e[1] - 0.25 + f[1] * 0.5, e[2] + f[2] * 0.5];
        self.loose.push(Loose { item: st.item, n: st.n.unwrap_or(1), wear: st.wear, p, v: [f[0] * 4.0, f[1] * 4.0 + 2.0, f[2] * 4.0], yaw: self.player.yaw, age: 0.0, wait: THROWN_WAIT, rest: false }); self.dirty = true;
    }
    /// Chute, rebonds, repos ; ramassage au passage ; les tas voisins du même objet se rejoignent.
    pub(crate) fn loose_tick(&mut self, dt: f64) {
        if self.loose.is_empty() { return; }
        let me = self.player.p; let alive = self.dead.is_none(); let mut list = std::mem::take(&mut self.loose); let mut got: Vec<(&'static str, u32)> = Vec::new();
        for l in list.iter_mut() {
            l.age += dt; l.wait -= dt;
            let solid = |w: &crate::stream::Streamer, x: f64, y: f64, z: f64| w.get(x.floor() as i32, y.floor() as i32, z.floor() as i32).is_none_or(|b| b != 0 && is_solid(b));
            if !l.rest || !solid(&self.world, l.p[0], l.p[1] - 0.05, l.p[2]) {
                l.rest = false; l.v[1] -= 20.0 * dt; l.v[1] = l.v[1].max(-30.0);
                for a in [0usize, 2] { let mut q = l.p; q[a] += l.v[a] * dt; if solid(&self.world, q[0], q[1] + 0.05, q[2]) { l.v[a] *= -0.3; } else { l.p = q; } }
                let ny = l.p[1] + l.v[1] * dt;
                if solid(&self.world, l.p[0], ny, l.p[2]) {
                    if l.v[1] < 0.0 { l.p[1] = ny.floor() + 1.0; if l.v[1] < -4.0 { l.v[1] *= -0.25; } else { l.v = [0.0; 3]; l.rest = true; } l.v[0] *= 0.55; l.v[2] *= 0.55; } else { l.v[1] = 0.0; }
                } else { l.p[1] = ny; }
                // Dans l'eau, il coule doucement.
                if self.world.get(l.p[0].floor() as i32, l.p[1].floor() as i32, l.p[2].floor() as i32).is_some_and(crate::blocks::is_water) { l.v[1] = l.v[1].max(-1.2); l.v[0] *= 0.9; l.v[2] *= 0.9; }
            }
            // Attiré puis pris quand on s'en approche.
            let (dx, dy, dz) = (me[0] - l.p[0], me[1] + 0.6 - l.p[1], me[2] - l.p[2]); let d = (dx * dx + dz * dz).sqrt();
            if alive && l.wait <= 0.0 && d < REACH && dy.abs() < 2.0 && l.n > 0 {
                if d < 0.6 {
                    let before = l.n; let left = if crate::data::def(l.item).is_some_and(|x| x.stack <= 1) { if self.inv.iter().any(|s| s.is_none()) { if let Some(slot) = self.inv.iter_mut().find(|s| s.is_none()) { *slot = Some(Stack { item: l.item, n: None, wear: l.wear }); } 0 } else { l.n } } else { self.add_item(l.item, l.n) };
                    if left < before { got.push((l.item, before - left)); } l.n = left;
                } else { let k = (dt * 9.0).min(1.0); l.p[0] += dx * k * 0.5; l.p[2] += dz * k * 0.5; l.p[1] += dy.clamp(-0.5, 0.5) * k * 0.3; l.rest = false; }
            }
        }
        list.retain(|l| l.n > 0 && l.age < LIFE && l.p[1] > -10.0);
        // Deux tas proches du même objet n'en font plus qu'un (sans dépasser la pile).
        let mut i = 0; while i < list.len() { let mut j = i + 1; while j < list.len() {
            let (a, b) = (list[i], list[j]); let max = crate::data::def(a.item).map_or(64, |d| d.stack);
            if a.item == b.item && a.wear == b.wear && a.rest && b.rest && (a.p[0] - b.p[0]).hypot(a.p[2] - b.p[2]) < 0.7 && (a.p[1] - b.p[1]).abs() < 0.3 && a.n + b.n <= max && max > 1 { list[i].n += b.n; list[i].age = a.age.min(b.age); list.swap_remove(j); } else { j += 1; }
        } i += 1; }
        self.loose = list;
        if !got.is_empty() { self.sound.pickup(); for (id, n) in got { self.notes.push(format!("+{n} {}", item(id).map_or(id, |i| i.label))); } self.dirty = true; }
    }
    pub(crate) fn loose_save(&self) -> String { self.loose.iter().map(|l| format!("sol {} {} {} {:.2} {:.2} {:.2} {:.1}\n", l.item, l.n, l.wear, l.p[0], l.p[1], l.p[2], l.age)).collect() }
    pub(crate) fn loose_load(&mut self, a: &[&str]) {
        let (Some(it), Some(n), Some(w)) = (a.first().and_then(|x| item(x)), a.get(1).and_then(|x| x.parse().ok()), a.get(2).and_then(|x| x.parse().ok())) else { return };
        let f = |i: usize| a.get(i).and_then(|x| x.parse::<f64>().ok()).unwrap_or(0.0);
        self.loose.push(Loose { item: it.id, n, wear: w, p: [f(3), f(4), f(5)], v: [0.0; 3], yaw: f(3) * 7.3 + f(5) * 3.1, age: f(6), wait: 0.0, rest: false });
    }
    /// Ramasse d'un coup tout ce qui traîne au sol (bancs d'essai).
    pub fn collect_loose(&mut self) { for l in std::mem::take(&mut self.loose) { self.add_item(l.item, l.n); } }
    /// Les objets au sol, en petit : un bloc en cube, un outil couché, un meuble en réduction ; ils oscillent à peine.
    pub fn loose_draws(&self, md: &crate::actors::Models) -> Vec<crate::actors::Draw> {
        use crate::view::{mul, trs};
        let mut o = Vec::new();
        for l in &self.loose { let Some(h) = md.held(l.item) else { continue };
            let bob = if l.rest { 0.03 * (self.elapsed * 2.0 + l.yaw).sin() + 0.03 } else { 0.0 };
            let copies = if l.n >= 8 { 3 } else if l.n >= 2 { 2 } else { 1 };
            for c in 0..copies { let off = [0.12 * c as f64 * (l.yaw + c as f64).cos(), 0.0, 0.12 * c as f64 * (l.yaw + c as f64).sin()];
                let base = trs(l.p[0] + off[0], l.p[1] + bob + c as f64 * 0.02, l.p[2] + off[2], l.yaw + c as f64 * 0.9, 0.0, 1.0);
                let m = match h.hold { crate::actors::Hold::Tool => mul(&base, &trs(0.0, 0.05, 0.0, 0.0, -std::f64::consts::FRAC_PI_2, 0.62)), _ => mul(&base, &mul(&trs(0.0, 0.0, 0.0, 0.0, 0.0, 0.28), &trs(-0.5, 0.0, -0.5, 0.0, 0.0, 1.0))) };
                o.push(crate::actors::Draw { m, r: h.r, view: false, local: h.local, hero: false, dynamic: false, hurt: 0.0 });
            }
        }
        o
    }
}
#[allow(dead_code)] fn _v<V: Voxels>(_: &V) {}
