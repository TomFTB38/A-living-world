//! Grottes karstiques : de grandes salles rares, creusées dans le calcaire sous les terres.
//!
//! Une salle par case de 144 blocs au plus (moins d'une sur deux en pratique) : un plancher en gradins où l'eau dort en
//! bassins, une voûte irrégulière, des parois rongées en surplombs et en corniches. Des stalactites pendent de la voûte,
//! des stalagmites montent du sol et sortent parfois de l'eau, quelques colonnes relient les deux. Une cascade tombe d'une
//! fissure de la paroi dans un bassin. Là où la voûte est mince, un aven s'ouvre sur le ciel : la lumière du jour y
//! descend, la mousse, les fougères et les lianes s'y installent.
//!
//! Tout est une fonction pure de la graine et des coordonnées : deux chunks voisins tracent la même salle, sans jamais
//! se consulter, et la salle se régénère à l'identique (les modifications du joueur sont gardées à part, comme ailleurs).
use crate::biome::Biome;
use crate::blocks::*;
use crate::det::{cos, hypot2, sin};
use crate::gen::WorldGen;
use crate::noise::{hash_float, smooth};

/// Côté de la case qui accueille au plus une salle.
pub const CELL: i32 = 144;
/// Pas de la grille des concrétions.
const STEP: i32 = 5;

#[derive(Clone, Copy, Debug)]
pub struct Hall {
    /// Centre (x, z), demi-axes et cap de l'ellipse.
    pub x: f64, pub z: f64, pub rx: f64, pub rz: f64, ca: f64, sa: f64,
    /// Niveau du plancher (et de l'eau des bassins), hauteur de la voûte au centre.
    pub floor: i32, pub h: f64,
    /// L'aven : centre (x, z) et rayon.
    pub shaft: Option<[f64; 3]>,
    /// La cascade : la case de la fissure dans la paroi (x, y, z) et la case d'eau qui tombe, juste devant.
    pub fall: Option<([i32; 3], [i32; 2])>,
    key: (i32, i32),
}

/// Ce que la salle est en un point du sol : distance relative au bord, plancher, voûte.
#[derive(Clone, Copy)]
struct Cut { q: f64, fl: i32, ce: i32 }

impl Hall {
    fn local(&self, x: f64, z: f64) -> (f64, f64) { let (dx, dz) = (x - self.x, z - self.z); (dx * self.ca + dz * self.sa, -dx * self.sa + dz * self.ca) }
    /// Les bornes de ce que la salle peut toucher (avec l'aven et la cascade).
    fn reach(&self) -> f64 { self.rx.max(self.rz) * 1.3 + 4.0 }
}

impl WorldGen {
    /// La salle de cette case, s'il y en a une.
    pub fn hall(&self, i: i32, j: i32) -> Option<Hall> {
        if let Some(h) = self.halls.borrow().get(&(i, j)) { return *h; }
        let h = self.trace_hall(i, j); self.halls.borrow_mut().insert((i, j), h); h
    }
    /// Les salles qui peuvent toucher ce rectangle.
    pub fn halls_near(&self, x0: i32, z0: i32, x1: i32, z1: i32) -> Vec<Hall> {
        let m = 60; let mut out = Vec::new();
        for i in (x0 - m).div_euclid(CELL)..=(x1 + m).div_euclid(CELL) { for j in (z0 - m).div_euclid(CELL)..=(z1 + m).div_euclid(CELL) {
            if let Some(h) = self.hall(i, j) { let r = h.reach(); if h.x + r >= x0 as f64 && h.x - r <= x1 as f64 && h.z + r >= z0 as f64 && h.z - r <= z1 as f64 { out.push(h); } }
        } }
        out
    }
    /// La salle la plus proche d'un point (pour s'y rendre, ou la montrer).
    pub fn nearest_hall(&self, x: f64, z: f64, cells: i32) -> Option<Hall> {
        let (ci, cj) = ((x as i32).div_euclid(CELL), (z as i32).div_euclid(CELL)); let mut best: Option<(f64, Hall)> = None;
        for i in ci - cells..=ci + cells { for j in cj - cells..=cj + cells { if let Some(h) = self.hall(i, j) { let d = hypot2(h.x - x, h.z - z); if best.is_none_or(|b| d < b.0) { best = Some((d, h)); } } } }
        best.map(|b| b.1)
    }
    fn trace_hall(&self, i: i32, j: i32) -> Option<Hall> {
        if self.flat { return None; }
        let r = |k: i32| hash_float(i, j, self.seed, 300 + k);
        if r(0) > 0.45 { return None; }
        let (x, z) = ((i * CELL) as f64 + 34.0 + (CELL - 68) as f64 * r(1), (j * CELL) as f64 + 34.0 + (CELL - 68) as f64 * r(2));
        let (rx, rz) = (17.0 + 15.0 * r(3), 15.0 + 12.0 * r(4)); let a = r(5) * std::f64::consts::TAU;
        // Sous des terres : ni mer, ni lac, ni désert de sable ; la voûte garde au moins huit blocs de roche.
        let mut smin = f64::MAX;
        for (u, v) in [(0.0, 0.0), (1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (0.7, 0.7), (-0.7, 0.7), (0.7, -0.7), (-0.7, -0.7)] {
            let c = self.column(x + u * rx, z + v * rz); if c.water != 0.0 || c.mix.sea > 0.0 || c.mix.strong(Biome::Desert) > 0.4 { return None; } smin = smin.min(c.h); }
        let floor = 8 + (8.0 * r(6)) as i32; let h = (14.0 + 13.0 * r(7)).min(smin - 9.0 - floor as f64); if h < 11.0 { return None; }
        let mut hall = Hall { x, z, rx, rz, ca: cos(a), sa: sin(a), floor, h, shaft: None, fall: None, key: (i, j) };
        // L'aven : là où la voûte est mince, loin des villages et des chemins, pas en pleine forêt.
        let b = r(9) * std::f64::consts::TAU; let (sx, sz) = (x + 0.32 * rx * cos(b), z + 0.32 * rz * sin(b)); let sc = self.column(sx, sz);
        let near_town = { let vc = crate::village::CELL; let mut near = false;
            for gx in ((x as i32) - 300).div_euclid(vc)..=((x as i32) + 300).div_euclid(vc) { for gz in ((z as i32) - 300).div_euclid(vc)..=((z as i32) + 300).div_euclid(vc) {
                if let Some(s) = self.site(gx, gz) { if hypot2(s.v.c[0] as f64 - x, s.v.c[2] as f64 - z) < s.v.r as f64 + 60.0 + rx.max(rz) { near = true; } } } } near };
        if r(8) < 0.8 && !near_town && sc.path < 0.1 && sc.forest < 0.55 && sc.h - (floor as f64 + h) < 36.0 { hall.shaft = Some([sx, sz, 2.4 + 1.5 * r(10)]); }
        // La cascade : une fissure dans la paroi, aux deux tiers de la hauteur, et l'eau qui tombe devant elle jusqu'au bassin.
        if r(11) < 0.9 {
            let b = r(12) * std::f64::consts::TAU; let (dx, dz) = (cos(b), sin(b)); let mut inside = None;
            let mut t = 2.0; while t < 60.0 { let (px, pz) = (x + dx * t, z + dz * t); let c = self.cut(&hall, px, pz); if c.q > 0.9 { break; } inside = Some((px, pz, c)); t += 0.5; }
            if let Some((px, pz, c)) = inside { let (pxi, pzi) = (px.floor() as i32, pz.floor() as i32); let top = (floor + (h * (0.45 + 0.25 * r(13))) as i32).min(c.ce - 2);
                let (sxi, szi) = ((px + dx * 1.2).floor() as i32, (pz + dz * 1.2).floor() as i32);
                if top > floor + 3 && (sxi, szi) != (pxi, pzi) { hall.fall = Some(([sxi, top, szi], [pxi, pzi])); } }
        }
        Some(hall)
    }
    /// La salle en un point du sol, sans la rugosité des parois.
    fn cut(&self, hl: &Hall, x: f64, z: f64) -> Cut {
        let (u, v) = hl.local(x, z); let n = &self.n_cave; let k = hl.key.0 as f64 * 3.7 + hl.key.1 as f64 * 1.3;
        let q = ((u / hl.rx).powi(2) + (v / hl.rz).powi(2)).sqrt() * (1.0 + 0.2 * n.n2(x / 21.0 + k, z / 21.0 - k));
        // Le plancher : des gradins, des creux où l'eau dort, et il remonte en pente contre les parois.
        let terr = ((n.n2(x / 12.0 + 40.0, z / 12.0 - 17.0) + 0.12) * 4.2).round().clamp(-2.0, 3.0);
        let mut fl = hl.floor as f64 + terr + 6.0 * smooth(0.6, 1.0, q);
        if let Some((_, p)) = hl.fall { let d = hypot2(x - p[0] as f64 - 0.5, z - p[1] as f64 - 0.5); if d < 4.0 { fl = fl.min(hl.floor as f64 - if d < 2.2 { 2.0 } else { 1.0 }); } } // le bassin de la cascade
        let ce = hl.floor as f64 + hl.h * (1.0 - q.min(1.0).powf(2.2)).max(0.0).sqrt() * (0.8 + 0.2 * n.n2(x / 10.0 - 9.0, z / 10.0 + 33.0)) + 1.0;
        Cut { q, fl: fl.floor() as i32 + 1, ce: ce.floor() as i32 }
    }
    /// Creuse et habille les salles qui touchent ce chunk. `heights` : la surface de chaque colonne (la voûte reste dessous).
    pub(crate) fn karst(&self, v: &mut [Id], ox: i32, oz: i32, heights: &[i32]) {
        let halls = self.halls_near(ox, oz, ox + CX as i32 - 1, oz + CX as i32 - 1); if halls.is_empty() { return; }
        let n = &self.n_cave; let s = self.seed;
        let set = |v: &mut [Id], x: i32, y: i32, z: i32, b: Id| { let (lx, lz) = (x - ox, z - oz); if lx >= 0 && lz >= 0 && lx < CX as i32 && lz < CX as i32 && y > 1 && y < CH as i32 - 1 { v[idx(lx as usize, y as usize, lz as usize)] = b; } };
        let get = |v: &[Id], x: i32, y: i32, z: i32| -> Option<Id> { let (lx, lz) = (x - ox, z - oz); if lx >= 0 && lz >= 0 && lx < CX as i32 && lz < CX as i32 && y >= 0 && y < CH as i32 { Some(v[idx(lx as usize, y as usize, lz as usize)]) } else { None } };
        for hl in &halls {
            let wl = hl.floor; // l'eau des bassins
            // 1. La salle : le volume, rongé de surplombs et de corniches près des parois et sous la voûte.
            for lz in 0..CX as i32 { for lx in 0..CX as i32 {
                let (x, z) = (ox + lx, oz + lz); let (xf, zf) = (x as f64 + 0.5, z as f64 + 0.5); let c = self.cut(hl, xf, zf); if c.q > 1.15 { continue; }
                let surf = heights[lx as usize + lz as usize * CX]; let ce = c.ce.min(surf - 5); if ce <= c.fl { continue; }
                for y in (c.fl - 2).max(2)..=(ce + 3).min(CH as i32 - 2) {
                    let yf = y as f64 + 0.5; let edge = c.q > 0.78 || y >= ce - 3 || y <= c.fl;
                    let rough = if edge { n.n3(xf / 6.0, yf / 5.0, zf / 6.0) } else { 0.0 };
                    let inside = c.q + 0.1 * rough < 1.0 && y >= c.fl + (rough > 0.45) as i32 && (y as f64) < ce as f64 + 2.0 * rough;
                    if inside { set(v, x, y, z, if y <= wl && c.q < 0.95 { B::WATER } else { 0 }); }
                }
                // L'aven : un puits jusqu'au ciel, aux parois irrégulières.
                if let Some([sx, sz, sr]) = hl.shaft { let d = hypot2(xf - sx, zf - sz); let rr = sr * (1.0 + 0.3 * n.n2(xf / 4.0 + 7.0, zf / 4.0 - 3.0));
                    if d < rr { for y in (ce - 1).max(c.fl)..CH as i32 - 1 { if y <= wl { continue; } let wob = 0.6 * n.n3(xf / 5.0, y as f64 / 4.0, zf / 5.0); if d < rr + wob { set(v, x, y, z, 0); } } } }
            } }
            // 2. Le calcaire : la roche de la salle devient calcaire, la calcite coule sur le sol et le bas des parois.
            for lz in 0..CX as i32 { for lx in 0..CX as i32 {
                let (x, z) = (ox + lx, oz + lz); let (xf, zf) = (x as f64 + 0.5, z as f64 + 0.5); let c = self.cut(hl, xf, zf); if c.q > 1.3 { continue; }
                let surf = heights[lx as usize + lz as usize * CX];
                for y in (c.fl - 4).max(2)..=(c.ce + 5).min(surf - 2) {
                    let Some(b) = get(v, x, y, z) else { continue }; if b != B::STONE && b != B::GRANITE { continue; }
                    let wet = n.n3(xf / 7.0 + 11.0, y as f64 / 3.0, zf / 7.0 - 5.0); let open_up = get(v, x, y + 1, z) == Some(0) || get(v, x, y + 1, z) == Some(B::WATER);
                    set(v, x, y, z, if (open_up && wet > 0.05) || wet > 0.42 { B::FLOWSTONE } else { B::LIMESTONE });
                }
            } }
            // 3. Les concrétions, sur une grille fixe du monde : stalactites, stalagmites (même dans l'eau), colonnes.
            let r = hl.reach() as i32;
            for gz in (hl.z as i32 - r).div_euclid(STEP)..=(hl.z as i32 + r).div_euclid(STEP) { for gx in (hl.x as i32 - r).div_euclid(STEP)..=(hl.x as i32 + r).div_euclid(STEP) {
                let h = |k: i32| hash_float(gx, gz, s, 340 + k);
                let (cx, cz) = (gx * STEP + 1 + (3.0 * h(0)) as i32, gz * STEP + 1 + (3.0 * h(1)) as i32);
                if cx + 4 < ox || cz + 4 < oz || cx - 4 >= ox + CX as i32 || cz - 4 >= oz + CX as i32 { continue; }
                let c = self.cut(hl, cx as f64 + 0.5, cz as f64 + 0.5); if c.q > 0.86 { continue; }
                if let Some([sx, sz, sr]) = hl.shaft { if hypot2(cx as f64 - sx, cz as f64 - sz) < sr + 2.5 { continue; } } // l'aven reste dégagé
                if let Some((_, p)) = hl.fall { if hypot2((cx - p[0]) as f64, (cz - p[1]) as f64) < 3.5 { continue; } }
                let gap = (c.ce - c.fl) as f64; if gap < 5.0 { continue; }
                let kind = h(2); let big = h(3) * h(3);
                let (down, up, pillar) = (kind < 0.55, (0.55..0.82).contains(&kind), (0.82..0.88).contains(&kind) && gap < 15.0);
                if !(down || up || pillar) { continue; }
                let r0 = if pillar { 1.1 + 1.4 * big } else { 0.55 + 1.7 * big };
                let len = if pillar { gap + 6.0 } else { (2.0 + r0 * 3.2 + 3.0 * h(4)).min(gap * if down { 0.62 } else { 0.45 }) };
                for dz in -3..=3 { for dx in -3..=3 {
                    let (x, z) = (cx + dx, cz + dz); let d = hypot2(dx as f64, dz as f64); if d > r0 + 0.6 { continue; } if get(v, x, 0, z).is_none() { continue; }
                    // de la base (t = 0) vers la pointe (t = 1) ; au-delà de la base, la roche comble les creux de la voûte ou du sol
                    let span: Vec<(i32, f64)> = if pillar { (c.fl - 2..=c.ce + 3).map(|y| (y, ((y - c.fl) as f64 / gap).clamp(0.0, 1.0))).collect() }
                        else if down { (c.ce + 1 - len as i32..=c.ce + 3).map(|y| (y, ((c.ce - y) as f64 / len).max(0.0))).collect() }
                        else { (c.fl - 3..=c.fl - 1 + len as i32).map(|y| (y, ((y - c.fl + 1) as f64 / len).max(0.0))).collect() };
                    for (y, t) in span {
                        let rr = if pillar { r0 * (1.0 - 0.32 * sin(std::f64::consts::PI * t)) } else { r0 * (1.0 - t).max(0.0).powf(1.15) };
                        let Some(here) = get(v, x, y, z) else { continue }; if here != 0 && here != B::WATER { continue; }
                        if d <= rr && rr >= 0.62 { set(v, x, y, z, B::FLOWSTONE); }
                        else if dx == 0 && dz == 0 && !pillar && t < 1.0 { let k = if rr > 0.42 { 0 } else if rr > 0.24 { 1 } else { 2 };
                            set(v, x, y, z, if down { [B::DRIPSTONE_DOWN_0, B::DRIPSTONE_DOWN_1, B::DRIPSTONE_DOWN_2][k] } else { [B::DRIPSTONE_UP_0, B::DRIPSTONE_UP_1, B::DRIPSTONE_UP_2][k] }); }
                    }
                } }
            } }
            // 4. La cascade : la source dans sa fissure, et une nappe de deux blocs de large qui tombe jusqu'au bassin.
            if let Some(([sx, sy, sz], [px, pz])) = hl.fall {
                set(v, sx, sy, sz, B::WATER); set(v, sx, sy + 1, sz, B::LIMESTONE);
                let (ax, az) = if (sx - px).abs() >= (sz - pz).abs() { (0, 1) } else { (1, 0) }; // le côté de la nappe, en travers de la paroi
                for (qx, qz) in [(px, pz), (px + ax, pz + az)] {
                    let c = self.cut(hl, qx as f64 + 0.5, qz as f64 + 0.5); let bottom = c.fl.min(wl);
                    for y in bottom..=sy { if let Some(b) = get(v, qx, y, qz) { if b == 0 || b == B::WATER || BLOCKS[b as usize].name.starts_with("dripstone") { set(v, qx, y, qz, B::WATER); } } }
                    set(v, qx + (sx - px).signum(), sy, qz + (sz - pz).signum(), B::WATER);
                }
            }
            // 5. La vie autour de l'aven et au bord de l'eau : mousse, fougères, lianes ; quelques champignons lumineux dans l'ombre.
            for lz in 0..CX as i32 { for lx in 0..CX as i32 {
                let (x, z) = (ox + lx, oz + lz); let (xf, zf) = (x as f64 + 0.5, z as f64 + 0.5); let c = self.cut(hl, xf, zf); if c.q > 1.0 { continue; }
                let lightd = hl.shaft.map_or(99.0, |[sx, sz, sr]| (hypot2(xf - sx, zf - sz) - sr).max(0.0));
                let rr = hash_float(x, z, s, 360);
                // les lanternes d'anciens explorateurs, posées çà et là sur le chemin : une par case de neuf blocs au plus
                let (gx, gz) = (x.div_euclid(9), z.div_euclid(9)); let lamp = hash_float(gx, gz, s, 370) < 0.32 && x == gx * 9 + 2 + (hash_float(gx, gz, s, 371) * 5.0) as i32 && z == gz * 9 + 2 + (hash_float(gx, gz, s, 372) * 5.0) as i32 && c.q < 0.85;
                // le sol : la première case vide au-dessus d'un bloc plein, dans la tranche de la salle
                for y in (c.fl - 2).max(2)..(c.ce + 2).min(CH as i32 - 2) {
                    let (Some(here), Some(under)) = (get(v, x, y, z), get(v, x, y - 1, z)) else { continue }; if here != 0 || !is_opaque(under) { continue; }
                    let wet_edge = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|(a, b)| get(v, x + a, y - 1, z + b) == Some(B::WATER));
                    if lamp && lightd > 6.0 && y > wl { set(v, x, y, z, B::LANTERN); }
                    else if lightd < 7.0 + 3.0 * n.n2(xf / 3.0, zf / 3.0) { set(v, x, y - 1, z, B::MOSSY_STONE);
                        if rr < 0.22 { set(v, x, y, z, B::FERN); } else if rr < 0.3 { set(v, x, y, z, B::TALL_GRASS); } }
                    else if wet_edge && rr < 0.05 { set(v, x, y, z, [B::GLOWCAP_BLUE, B::GLOWCAP_CYAN, B::GLOWCAP][(hash_float(x, z, s, 361) * 3.0) as usize % 3]); }
                    else if rr < 0.006 { set(v, x, y, z, B::GLOWCAP_VIOLET); }
                    break;
                }
                // la voûte autour de l'aven : des lianes qui pendent dans la lumière
                if lightd < 5.0 && lightd > 0.0 && rr > 0.55 { let mut y = c.ce + 2; while y > c.fl && get(v, x, y, z).is_some_and(|b| b != 0) { y -= 1; }
                    let len = 2 + (hash_float(x, z, s, 362) * 6.0) as i32; for k in 0..len { if get(v, x, y - k, z) == Some(0) { set(v, x, y - k, z, B::VINE); } else { break; } } }
            } }
        }
    }
}

impl WorldGen {
    /// Un endroit sec où se tenir dans la salle, près d'un point visé : pieds sur la roche, trois cases libres au-dessus.
    pub fn hall_spot(&self, hl: &Hall, toward: [f64; 2]) -> Option<[f64; 3]> {
        let mut chunks: std::collections::HashMap<(i32, i32), Vec<Id>> = std::collections::HashMap::new();
        let mut at = |x: i32, y: i32, z: i32| -> Id { let k = (x.div_euclid(CX as i32), z.div_euclid(CX as i32)); let v = chunks.entry(k).or_insert_with(|| self.gen_chunk(k.0, k.1)); if y < 0 || y >= CH as i32 { 0 } else { v[idx(x.rem_euclid(CX as i32) as usize, y as usize, z.rem_euclid(CX as i32) as usize)] } };
        for ring in 0..14 { for k in 0..(1 + ring * 6) { let a = k as f64 / (1 + ring * 6) as f64 * std::f64::consts::TAU; let r = ring as f64 * 1.5;
            let (x, z) = ((toward[0] + r * cos(a)).floor() as i32, (toward[1] + r * sin(a)).floor() as i32);
            for y in (hl.floor - 3).max(2)..(hl.floor + hl.h as i32).min(CH as i32 - 4) {
                let under = at(x, y - 1, z); if is_opaque(under) && !is_water(under) && (0..3).all(|d| at(x, y + d, z) == 0) { return Some([x as f64 + 0.5, y as f64, z as f64 + 0.5]); } }
        } }
        None
    }
}

impl crate::session::Session {
    /// `/grotte` : se rendre dans la salle karstique la plus proche.
    pub fn goto_hall(&mut self) -> String {
        let p = self.player.p; let Some(h) = self.gen.nearest_hall(p[0], p[2], 10) else { return "Aucune grande grotte dans les parages".into() };
        let Some(spot) = self.gen.hall_spot(&h, [h.x, h.z]) else { return "La grotte la plus proche n'a pas d'endroit sûr où se tenir".into() };
        self.player.p = spot; self.player.v = [0.0; 3];
        format!("Grotte karstique à {:.0}, {:.0} ({:.0} blocs d'ici){}{}", h.x, h.z, hypot2(h.x - p[0], h.z - p[2]), if h.shaft.is_some() { " — un aven s'ouvre sur le ciel" } else { "" }, if h.fall.is_some() { " — une cascade" } else { "" })
    }
}
