//! La carte : ce que le joueur a vu, et seulement cela. Un pixel couvre 4 × 4 blocs ; il retient la couleur du dessus
//! (la moyenne de la texture du bloc) et sa hauteur, d'où l'ombrage du relief à l'affichage. Les pixels se découvrent
//! autour du joueur, dans les chunks chargés, et se rangent par régions de 64 × 64 qui se sauvegardent avec la partie.
use crate::blocks::{is_solid, is_water, BLOCKS, CH};
use crate::player::Voxels;
use crate::session::Session;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Blocs par pixel de carte, pixels par côté de région, rayon de découverte en blocs.
pub const STEP: i32 = 4; pub const REGION: i32 = 64; pub const REVEAL: i32 = 60;
/// Échelles de la grande carte : blocs par case dessinée.
pub const ZOOMS: [i32; 4] = [4, 8, 16, 32];
const BUDGET: usize = 140;

#[derive(Default)]
pub struct Atlas {
    /// Pixel : rouge, vert, bleu (bit faible forcé à 1 : 0 veut dire inexploré), hauteur du dessus.
    pub regions: HashMap<(i32, i32), Vec<u32>>,
    /// Grande carte ouverte, et son animation (0 fermée, 1 dépliée) ; échelle choisie (indice dans ZOOMS).
    pub open: bool, pub anim: f64, pub zoom: usize,
    /// Décalage de la grande carte (en blocs) quand on la fait glisser à la souris ; recentrée à chaque ouverture.
    pub pan: [f64; 2],
    last: Option<(i32, i32)>, pending: bool,
}
/// Couleur moyenne du dessus de chaque bloc, tirée des textures du jeu.
pub fn palette() -> &'static [[u8; 3]; BLOCKS.len()] {
    static P: OnceLock<[[u8; 3]; BLOCKS.len()]> = OnceLock::new();
    P.get_or_init(|| {
        let t = crate::textures::build_textures(8); let n = 64usize; let mut p = [[0u8; 3]; BLOCKS.len()];
        for b in BLOCKS.iter() {
            let src = if crate::shapes::info(b.id).is_some() { crate::shapes::material(b.id) } else { b.id };
            let l = BLOCKS[src as usize].faces[0] as usize; if (l + 1) * n * 4 > t.albedo.len() { continue; }
            let mut c = [0u32; 3]; for px in t.albedo[l * n * 4..(l + 1) * n * 4].chunks(4) { for k in 0..3 { c[k] += px[k] as u32; } }
            let tint = BLOCKS[src as usize].tint; p[b.id as usize] = [0, 1, 2].map(|k| ((c[k] / n as u32) as f64 * tint[k]).min(255.0) as u8);
        }
        p
    })
}
fn pack(c: [u8; 3], h: i32) -> u32 { (c[0] as u32) << 24 | (c[1] as u32) << 16 | ((c[2] | 1) as u32) << 8 | h.clamp(0, 255) as u32 }
/// Un pixel découvert : sa couleur et la hauteur du dessus.
pub fn unpack(v: u32) -> Option<([u8; 3], u8)> { (v != 0).then_some(([(v >> 24) as u8, (v >> 16) as u8, (v >> 8) as u8], v as u8)) }

impl Atlas {
    /// Le pixel de carte (px, pz), en unités de STEP blocs.
    pub fn get(&self, px: i32, pz: i32) -> u32 {
        let (r, l) = ((px.div_euclid(REGION), pz.div_euclid(REGION)), (px.rem_euclid(REGION) + pz.rem_euclid(REGION) * REGION) as usize);
        self.regions.get(&r).map_or(0, |v| v[l])
    }
    fn set(&mut self, px: i32, pz: i32, v: u32) {
        let (r, l) = ((px.div_euclid(REGION), pz.div_euclid(REGION)), (px.rem_euclid(REGION) + pz.rem_euclid(REGION) * REGION) as usize);
        self.regions.entry(r).or_insert_with(|| vec![0; (REGION * REGION) as usize])[l] = v;
    }
    /// Part de la surface vue, en pixels découverts.
    pub fn explored(&self) -> usize { self.regions.values().map(|v| v.iter().filter(|p| **p != 0).count()).sum() }
    pub fn save(&self) -> String {
        let mut keys: Vec<_> = self.regions.keys().copied().collect(); keys.sort(); let mut s = String::new();
        for k in keys {
            let v = &self.regions[&k]; s += &format!("carte {} {}", k.0, k.1); let mut i = 0;
            while i < v.len() { let mut n = 1; while i + n < v.len() && v[i + n] == v[i] { n += 1; } s += &if n > 1 { format!(" {n}*{:x}", v[i]) } else { format!(" {:x}", v[i]) }; i += n; }
            s.push('\n');
        }
        s
    }
    pub fn load(&mut self, a: &[&str]) {
        let (Some(rx), Some(rz)) = (a.first().and_then(|v| v.parse().ok()), a.get(1).and_then(|v| v.parse().ok())) else { return };
        let mut v = Vec::with_capacity((REGION * REGION) as usize);
        for t in &a[2..] { let (n, x) = t.split_once('*').map_or((1, *t), |(n, x)| (n.parse().unwrap_or(1), x)); let x = u32::from_str_radix(x, 16).unwrap_or(0); for _ in 0..n { v.push(x); } }
        v.resize((REGION * REGION) as usize, 0); self.regions.insert((rx, rz), v);
    }
}

impl Session {
    /// Ouvre ou ferme la grande carte.
    pub fn toggle_map(&mut self) { self.map.open = !self.map.open; if self.map.open { self.inv_open = false; self.map.pan = [0.0; 2]; } }
    /// Faire glisser la grande carte : `d` en pixels à l'écran, `px` la taille d'une case à l'écran. La carte suit la souris.
    pub fn pan_map(&mut self, d: [f64; 2], px: f64) { let cell = ZOOMS[self.map.zoom] as f64; let k = cell / px.max(0.1); self.map.pan[0] -= d[0] * k; self.map.pan[1] -= d[1] * k;
        let lim = 4000.0; self.map.pan = self.map.pan.map(|v| v.clamp(-lim, lim)); }
    /// Recentrer la grande carte sur soi.
    pub fn center_map(&mut self) { self.map.pan = [0.0; 2]; }
    pub fn zoom_map(&mut self, d: i32) { self.map.zoom = (self.map.zoom as i32 + d).clamp(0, ZOOMS.len() as i32 - 1) as usize; }
    /// L'animation de la carte, et la découverte du terrain autour du joueur.
    pub(crate) fn map_tick(&mut self, dt: f64) {
        let goal = if self.map.open { 1.0 } else { 0.0 }; let k = dt * 1.8; self.map.anim = if self.map.anim < goal { (self.map.anim + k).min(goal) } else { (self.map.anim - k * 1.4).max(goal) };
        let p = self.player.p; let here = ((p[0] / STEP as f64).floor() as i32, (p[2] / STEP as f64).floor() as i32);
        if self.map.last != Some(here) { self.map.last = Some(here); self.map.pending = true; }
        if !self.map.pending { return; }
        let r = REVEAL / STEP; let mut done = 0; let mut left = false;
        'scan: for dz in -r..=r { for dx in -r..=r {
            if dx * dx + dz * dz > r * r { continue; } let (px, pz) = (here.0 + dx, here.1 + dz); if self.map.get(px, pz) != 0 { continue; }
            if done >= BUDGET { left = true; break 'scan; }
            if let Some(v) = self.survey(px * STEP + STEP / 2, pz * STEP + STEP / 2, p[1] as i32) { self.map.set(px, pz, v); done += 1; }
        } }
        self.map.pending = left; if done > 0 { self.dirty = true; }
    }
    /// Ce qu'on voit d'en haut en (x, z) : le premier bloc plein ou d'eau, sous le ciel. `None` si le chunk n'est pas chargé.
    fn survey(&self, x: i32, z: i32, near: i32) -> Option<u32> {
        let pal = palette(); let top = (near + 80).min(CH as i32 - 1);
        self.world.get(x, top, z)?;
        let mut y = top;
        while y > 0 { let b = self.world.get(x, y, z)?; if b != 0 && (is_solid(b) || is_water(b)) { break; } y -= 1; }
        let b = self.world.get(x, y, z)?;
        if is_water(b) {
            // L'eau s'assombrit avec la profondeur.
            let mut d = 0; while d < 12 && self.world.get(x, y - d - 1, z).is_some_and(is_water) { d += 1; }
            let f = 1.0 - d as f64 * 0.045; return Some(pack([(58.0 * f) as u8, (104.0 * f) as u8, (168.0 * f + 20.0) as u8], y));
        }
        Some(pack(pal[b as usize], y))
    }
}
