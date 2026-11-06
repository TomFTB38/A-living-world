//! Eau qui coule : un petit automate cellulaire. Une source garde le niveau 8 ; chaque case voisine perd un niveau,
//! l'eau descend d'abord et ne s'étale que sur un support. Seules les cases proches d'un changement sont calculées.
use crate::blocks::*;
use crate::player::Voxels;
use std::collections::HashSet;
pub const STEP: f64 = 0.16; // secondes entre deux avancées
const PER_STEP: usize = 700; const MAX_CELLS: usize = 400000;
type P = (i32, i32, i32);
const SIDES: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
#[derive(Default)] pub struct Flow { dirty: HashSet<P>, t: f64, cells: usize, pub last: Option<[f64; 3]>, pub quiet: f64 }
fn replaceable(b: u8) -> bool { b == 0 || (is_deco(b) && !is_solid(b) && b != B::LANTERN) || (is_water(b) && b != B::WATER) }
impl Flow {
    /// Un bloc a changé : lui et ses voisins doivent être recalculés.
    pub fn touch(&mut self, x: i32, y: i32, z: i32) { for p in [(x, y, z), (x + 1, y, z), (x - 1, y, z), (x, y, z + 1), (x, y, z - 1), (x, y + 1, z), (x, y - 1, z)] { self.dirty.insert(p); } }
    pub fn busy(&self) -> bool { !self.dirty.is_empty() }
    /// Niveau que cette case devrait avoir.
    fn want(w: &impl Voxels, p: P) -> u8 {
        let (x, y, z) = p;
        if w.get(x, y + 1, z).is_some_and(is_water) { return 7; } // l'eau qui tombe reste haute : elle s'étalera en bas
        let mut best = 0;
        for (dx, dz) in SIDES {
            let Some(n) = w.get(x + dx, y, z + dz) else { continue }; let l = water_level(n); if l <= 1 { continue; }
            // L'eau sans support tombe au lieu de s'étaler ; une source alimente toujours ses voisins.
            let under = w.get(x + dx, y - 1, z + dz).unwrap_or(1); if l < 8 && replaceable(under) { continue; }
            best = best.max(l - 1);
        }
        best
    }
    /// Une avancée : rend les blocs à poser. L'appelant les applique d'un seul coup.
    pub fn step(&mut self, dt: f64, w: &impl Voxels) -> Vec<(i32, i32, i32, u8)> {
        self.t += dt; self.quiet += dt; let mut out = Vec::new(); if self.t < STEP || self.dirty.is_empty() { return out; } self.t = 0.0;
        let list: Vec<P> = self.dirty.iter().copied().take(PER_STEP).collect(); for p in &list { self.dirty.remove(p); }
        for p in list {
            let Some(b) = w.get(p.0, p.1, p.2) else { continue }; if b == B::WATER || !replaceable(b) { continue; }
            let (now, want) = (water_level(b), Self::want(w, p)); if now == want { continue; }
            if want > now { if self.cells >= MAX_CELLS { continue; } self.cells += 1; } else { self.cells = self.cells.saturating_sub(1); }
            out.push((p.0, p.1, p.2, if want == 0 { 0 } else { flow_block(want) })); self.last = Some([p.0 as f64, p.1 as f64, p.2 as f64]); self.quiet = 0.0;
        }
        for q in &out { self.touch(q.0, q.1, q.2); }
        out
    }
}
