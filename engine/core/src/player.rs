//! Déplacement à collisions voxel : marche, sprint, saut, nage, vol.
use crate::blocks::{is_solid, B};
const HALF: f64 = 0.3; const GRAV: f64 = 26.0; const JUMP: f64 = 9.2;

/// Accès au monde. `None` : zone pas encore chargée, traitée comme un mur pour ne jamais tomber dans le vide.
pub trait Voxels { fn get(&self, x: i32, y: i32, z: i32) -> Option<u8>; }
#[derive(Default, Clone, Copy)]
pub struct Input { pub x: f64, pub z: f64, pub jump: bool, pub sprint: bool, pub crouch: bool }
pub struct Player { pub p: [f64; 3], pub v: [f64; 3], pub yaw: f64, pub pitch: f64, pub ground: bool, pub crouch: bool, pub in_water: bool, pub speed: f64, pub fly: bool, pub move_yaw: f64 }
impl Player {
    pub fn new(x: f64, y: f64, z: f64) -> Self { Player { p: [x, y, z], v: [0.0; 3], yaw: 0.0, pitch: 0.0, ground: false, crouch: false, in_water: false, speed: 0.0, fly: false, move_yaw: 0.0 } }
    fn hits(&self, w: &impl Voxels, x: f64, y: f64, z: f64) -> bool {
        let h = if self.crouch { 1.45 } else { 1.8 }; let fl = |v: f64| v.floor() as i32;
        for by in fl(y)..=fl(y + h - 0.001) { for bz in fl(z - HALF)..=fl(z + HALF) { for bx in fl(x - HALF)..=fl(x + HALF) {
            if w.get(bx, by, bz).is_none_or(is_solid) { return true; }
        } } }
        false
    }
    pub fn update(&mut self, w: &impl Voxels, dt: f64, i: Input) {
        let fly = self.fly; let sp = if fly { if i.sprint { 22.0 } else { 11.0 } } else if i.sprint { 7.2 } else if i.crouch { 2.0 } else { 4.5 };
        self.crouch = i.crouch && !fly;
        self.in_water = w.get(self.p[0].floor() as i32, (self.p[1] + 0.9).floor() as i32, self.p[2].floor() as i32) == Some(B::WATER);
        let (s, c) = self.yaw.sin_cos();
        let (mut mx, mut mz) = (i.x * c + i.z * s, -i.x * s + i.z * c); let l = (mx * mx + mz * mz).sqrt();
        if l > 1.0 { mx /= l; mz /= l; }
        let accel = if fly { 9.0 } else if self.ground { 14.0 } else if self.in_water { 5.0 } else { 3.5 }; let sw = if self.in_water && !fly { 0.55 } else { 1.0 };
        let k = (accel * dt).min(1.0);
        self.v[0] += (mx * sp * sw - self.v[0]) * k; self.v[2] += (mz * sp * sw - self.v[2]) * k;
        if fly { self.v[1] += ((if i.jump { sp } else { 0.0 }) - (if i.crouch { sp } else { 0.0 }) - self.v[1]) * (9.0 * dt).min(1.0); }
        else if self.in_water { self.v[1] += (if i.jump { 3.5 } else { -1.2 - self.v[1] }) * (6.0 * dt).min(1.0); }
        else { self.v[1] -= GRAV * dt; if i.jump && self.ground { self.v[1] = JUMP; self.ground = false; } }
        if self.v[1] < -40.0 { self.v[1] = -40.0; }
        for a in [0, 2] {
            let mut n = self.p; n[a] += self.v[a] * dt;
            if !self.hits(w, n[0], n[1], n[2]) { self.p[a] = n[a]; }
            else if self.ground && !fly && !self.hits(w, n[0], n[1] + 1.05, n[2]) && !self.hits(w, self.p[0], self.p[1] + 1.05, self.p[2]) { self.p[a] = n[a]; self.p[1] += 1.02; }
            else { self.v[a] = 0.0; }
        }
        let ny = self.p[1] + self.v[1] * dt;
        if !self.hits(w, self.p[0], ny, self.p[2]) { self.p[1] = ny; self.ground = false; }
        else {
            if self.v[1] < 0.0 { self.ground = true; self.p[1] = (ny - 0.001).ceil(); if self.hits(w, self.p[0], self.p[1], self.p[2]) { self.p[1] = ny.floor() + 1.0; } }
            self.v[1] = 0.0;
        }
        self.speed = (self.v[0] * self.v[0] + self.v[2] * self.v[2]).sqrt();
        if l > 0.05 { let d = mx.atan2(mz) - self.move_yaw; let d = d.sin().atan2(d.cos()); self.move_yaw += d * (dt * 11.0).min(1.0); }
    }
    pub fn eye(&self) -> [f64; 3] { [self.p[0], self.p[1] + if self.crouch { 1.32 } else { 1.62 }, self.p[2]] }
    pub fn forward(&self) -> [f64; 3] { let cp = self.pitch.cos(); [self.yaw.sin() * cp, self.pitch.sin(), self.yaw.cos() * cp] }
}
