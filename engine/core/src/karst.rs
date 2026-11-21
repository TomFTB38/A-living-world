//! Le monde souterrain : de grandes salles rares, chacune d'un biome souterrain, et leurs accès depuis la surface.
//!
//! Une salle par case de 144 blocs au plus. Son biome dépend de ce qu'il y a au-dessus (climat, humidité, relief, désert,
//! mer), de sa profondeur et d'un tirage de rareté : grotte ordinaire, karstique, verdoyante, fongique, luminescente,
//! cristalline, glacée, volcanique, minérale, désertique, abyssale ou corallienne. Toutes partagent la même architecture :
//! un volume (plancher en gradins et bassins, voûte irrégulière, parois rongées), une roche, des concrétions, un liquide
//! (eau, glace, lave), une cascade, puis l'habillage propre au biome (champignons géants, cristaux, ruines, géodes,
//! gouffre, coraux, végétation). Ajouter un biome, c'est ajouter un `Under` et son `Style` ; le reste suit.
//!
//! Accès : un aven là où la voûte est mince (la lumière du jour y descend), une bouche naturelle en pente depuis la
//! surface, ou une vieille mine boisée, avec ses rails et ses lanternes. Tout est une fonction pure de la graine et des
//! coordonnées : deux chunks voisins tracent la même salle sans se consulter, et les modifications du joueur sont gardées
//! à part, comme ailleurs.
use crate::biome::Biome;
use crate::blocks::*;
use crate::det::{cos, hypot2, sin};
use crate::gen::WorldGen;
use crate::noise::{hash_float, smooth};

/// Côté de la case qui accueille au plus une salle.
pub const CELL: i32 = 144;
/// Pas de la grille des concrétions et des grandes formations.
const STEP: i32 = 5;

/// Les biomes souterrains.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Under { Ordinary, Karst, Verdant, Fungal, Glow, Crystal, Ice, Volcanic, Mineral, Desert, Abyss, Coral }
pub const UNDERS: [Under; 12] = [Under::Ordinary, Under::Karst, Under::Verdant, Under::Fungal, Under::Glow, Under::Crystal, Under::Ice, Under::Volcanic, Under::Mineral, Under::Desert, Under::Abyss, Under::Coral];
impl Under {
    pub fn name(self) -> &'static str {
        match self { Under::Ordinary => "grotte rocheuse", Under::Karst => "grotte karstique", Under::Verdant => "grotte verdoyante", Under::Fungal => "grotte fongique", Under::Glow => "grotte luminescente",
            Under::Crystal => "grotte cristalline", Under::Ice => "grotte de glace", Under::Volcanic => "grotte volcanique", Under::Mineral => "grotte minérale", Under::Desert => "grotte désertique", Under::Abyss => "abîme", Under::Coral => "grotte corallienne" }
    }
    /// Le mot à taper après `/grotte`.
    pub fn key(self) -> &'static str {
        match self { Under::Ordinary => "rocheuse", Under::Karst => "karst", Under::Verdant => "verte", Under::Fungal => "champignons", Under::Glow => "lumineuse", Under::Crystal => "cristal",
            Under::Ice => "glace", Under::Volcanic => "volcan", Under::Mineral => "minerale", Under::Desert => "desert", Under::Abyss => "abime", Under::Coral => "corail" }
    }
}

/// L'allure d'un biome souterrain : roches, concrétions, liquide.
struct Style { wall: Id, wall2: Id, crust: Id, body: Id, points: &'static str, liquid: Id, density: f64 }
fn style(k: Under) -> Style {
    let s = |wall, wall2, crust, body, points, liquid, density| Style { wall, wall2, crust, body, points, liquid, density };
    match k {
        Under::Ordinary => s(B::STONE, B::GRAVEL, B::STONE, B::STONE, "stone", B::WATER, 0.45),
        Under::Karst => s(B::LIMESTONE, B::LIMESTONE, B::FLOWSTONE, B::FLOWSTONE, "dripstone", B::WATER, 1.0),
        Under::Verdant => s(B::LIMESTONE, B::MOSSY_STONE, B::MOSSY_STONE, B::FLOWSTONE, "dripstone", B::WATER, 0.45),
        Under::Fungal => s(B::STONE, B::MYCELIUM, B::MYCELIUM, B::STONE, "stone", B::WATER, 0.25),
        Under::Glow => s(B::ABYSS_ROCK, B::STONE, B::GLOW_MOSS, B::ABYSS_ROCK, "crystal_blue", B::WATER, 0.35),
        Under::Crystal => s(B::ABYSS_ROCK, B::STONE, B::STONE, B::CRYSTAL_VIOLET, "crystal_violet", B::WATER, 0.55),
        Under::Ice => s(B::ICE_BLUE, B::ICE, B::SNOW, B::ICE_BLUE, "ice", B::ICE, 0.9),
        Under::Volcanic => s(B::BASALT, B::OBSIDIAN, B::MAGMA, B::BASALT, "basalt", B::LAVA, 0.5),
        Under::Mineral => s(B::STONE, B::GRANITE, B::GRAVEL, B::STONE, "stone", B::WATER, 0.5),
        Under::Desert => s(B::SANDSTONE, B::RED_SANDSTONE, B::SAND, B::SANDSTONE, "sandstone", 0, 0.4),
        Under::Abyss => s(B::ABYSS_ROCK, B::BASALT, B::ABYSS_ROCK, B::ABYSS_ROCK, "basalt", B::WATER, 0.4),
        Under::Coral => s(B::LIMESTONE, B::SAND, B::SAND, B::LIMESTONE, "dripstone", B::WATER, 0.3),
    }
}
/// Les pointes d'une roche : (vers le bas, vers le haut), de la base à la pointe.
fn points(m: &str) -> ([Id; 3], [Id; 3]) {
    let id = |d: &str, k: i32| if m == "dripstone" { by_name(&format!("dripstone_{d}_{k}")).unwrap() } else { by_name(&format!("{m}_point_{d}_{k}")).unwrap() };
    ([id("down", 0), id("down", 1), id("down", 2)], [id("up", 0), id("up", 1), id("up", 2)])
}

/// L'accès d'une salle depuis la surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Way {
    /// Une bouche naturelle (`mine` faux) ou une vieille mine boisée : de la surface (x, y, z) au bord de la salle, en pente.
    Path { from: [f64; 3], to: [f64; 3], mine: bool },
}

#[derive(Clone, Copy, Debug)]
pub struct Hall {
    /// Centre (x, z), demi-axes et cap de l'ellipse.
    pub x: f64, pub z: f64, pub rx: f64, pub rz: f64, ca: f64, sa: f64,
    /// Niveau du plancher (et du liquide des bassins), hauteur de la voûte au centre.
    pub floor: i32, pub h: f64,
    pub kind: Under,
    /// L'aven : centre (x, z) et rayon.
    pub shaft: Option<[f64; 3]>,
    /// La cascade : la case de la fissure dans la paroi (x, y, z) et la case qui tombe, juste devant.
    pub fall: Option<([i32; 3], [i32; 2])>,
    /// Une bouche ou une mine.
    pub way: Option<Way>,
    key: (i32, i32),
}

/// Ce que la salle est en un point du sol : distance relative au bord, plancher, voûte.
/// Ce que des hommes ont laissé dans une salle : un camp de mineurs abandonné, un autel de cristal, un caveau au coffre.
#[derive(Clone, Copy, Debug, PartialEq)] pub enum Relic { Camp, Altar, Vault }
/// L'emplacement d'un vestige : son centre (x, z) et le niveau de son sol (la première case libre).
#[derive(Clone, Copy, Debug)] pub struct Site { pub kind: Relic, pub x: i32, pub z: i32, pub fl: i32 }
impl Site {
    /// La case du coffre du vestige.
    pub fn chest(&self) -> [i32; 3] { match self.kind { Relic::Camp => [self.x - 2, self.fl, self.z - 2], Relic::Altar => [self.x, self.fl + 1, self.z - 2], Relic::Vault => [self.x, self.fl, self.z + 2] } }
}
#[derive(Clone, Copy)]
struct Cut { q: f64, fl: i32, ce: i32 }

impl Hall {
    fn local(&self, x: f64, z: f64) -> (f64, f64) { let (dx, dz) = (x - self.x, z - self.z); (dx * self.ca + dz * self.sa, -dx * self.sa + dz * self.ca) }
    /// Les bornes de ce que la salle peut toucher (avec l'aven, la cascade, sa bouche ou sa mine).
    fn reach(&self) -> f64 {
        let w = match self.way { Some(Way::Path { from, .. }) => hypot2(from[0] - self.x, from[2] - self.z) + 4.0, None => 0.0 };
        (self.rx.max(self.rz) * 1.3 + 6.0).max(w)
    }
}

impl WorldGen {
    /// La salle de cette case, s'il y en a une.
    pub fn hall(&self, i: i32, j: i32) -> Option<Hall> {
        if let Some(h) = self.halls.borrow().get(&(i, j)) { return *h; }
        let h = self.trace_hall(i, j); self.halls.borrow_mut().insert((i, j), h); h
    }
    /// Les salles qui peuvent toucher ce rectangle.
    pub fn halls_near(&self, x0: i32, z0: i32, x1: i32, z1: i32) -> Vec<Hall> {
        let m = 140; let mut out = Vec::new();
        for i in (x0 - m).div_euclid(CELL)..=(x1 + m).div_euclid(CELL) { for j in (z0 - m).div_euclid(CELL)..=(z1 + m).div_euclid(CELL) {
            if let Some(h) = self.hall(i, j) { let r = h.reach(); if h.x + r >= x0 as f64 && h.x - r <= x1 as f64 && h.z + r >= z0 as f64 && h.z - r <= z1 as f64 { out.push(h); } }
        } }
        out
    }
    /// La salle la plus proche d'un point, d'un biome donné ou non.
    pub fn nearest_hall_of(&self, x: f64, z: f64, cells: i32, kind: Option<Under>) -> Option<Hall> {
        let (ci, cj) = ((x as i32).div_euclid(CELL), (z as i32).div_euclid(CELL)); let mut best: Option<(f64, Hall)> = None;
        for i in ci - cells..=ci + cells { for j in cj - cells..=cj + cells { if let Some(h) = self.hall(i, j) { if kind.is_some_and(|k| k != h.kind) { continue; } let d = hypot2(h.x - x, h.z - z); if best.is_none_or(|b| d < b.0) { best = Some((d, h)); } } } }
        best.map(|b| b.1)
    }
    pub fn nearest_hall(&self, x: f64, z: f64, cells: i32) -> Option<Hall> { self.nearest_hall_of(x, z, cells, None) }
    /// Le biome de la salle où se trouve ce point, s'il est dans une salle (un peu de marge : ses abords en font partie).
    pub fn hall_at(&self, p: [f64; 3]) -> Option<Under> {
        let h = self.nearest_hall(p[0], p[2], 1)?; let (u, v) = h.local(p[0], p[2]);
        ((u / (h.rx + 6.0)).powi(2) + (v / (h.rz + 6.0)).powi(2) <= 1.0 && p[1] > h.floor as f64 - 14.0 && p[1] < h.floor as f64 + h.h + 8.0).then_some(h.kind)
    }

    /// Le vestige d'une salle, s'il y en a un : sa sorte selon le biome, sa place sur un replat sec, loin de l'aven et de la cascade.
    /// Fonction pure de la salle : la génération, le butin du coffre et les commandes retrouvent le même endroit.
    pub fn hall_site(&self, hl: &Hall) -> Option<Site> {
        let r = |k: i32| hash_float(hl.key.0, hl.key.1, self.seed, 460 + k);
        let (kind, p) = match hl.kind { Under::Mineral => (Relic::Camp, 0.75), Under::Ordinary => (Relic::Camp, 0.45), Under::Karst => (Relic::Camp, 0.35), Under::Crystal => (Relic::Altar, 0.7), Under::Glow => (Relic::Altar, 0.35),
            Under::Coral => return None, _ => (Relic::Vault, 0.45) };
        if r(0) > p { return None; }
        let wet = style(hl.kind).liquid != 0;
        for k in 0..28 { let a = r(1) * std::f64::consts::TAU + k as f64 * 2.4; let d = 0.12 + 0.022 * k as f64; let (u, v) = (hl.rx * d * cos(a), hl.rz * d * sin(a));
            let (x, z) = ((hl.x + u * hl.ca - v * hl.sa).floor() as i32, (hl.z + u * hl.sa + v * hl.ca).floor() as i32);
            // un replat : neuf sondages sous le vestige ; on bâtit sur le plus haut (l'assise comble le reste), au sec, sous une voûte assez haute
            let cuts: Vec<Cut> = [(0, 0), (-3, -3), (3, -3), (-3, 3), (3, 3), (0, -3), (0, 3), (-3, 0), (3, 0)].iter().map(|(dx, dz)| self.cut(hl, (x + dx) as f64 + 0.5, (z + dz) as f64 + 0.5)).collect();
            let (lo, fl, ce) = (cuts.iter().map(|c| c.fl).min().unwrap(), cuts.iter().map(|c| c.fl).max().unwrap(), cuts.iter().map(|c| c.ce).min().unwrap());
            if fl - lo > 3 || ce - fl < 6 || cuts.iter().any(|c| c.q > 0.8) || (wet && fl <= hl.floor) { continue; }
            if hl.shaft.is_some_and(|[sx, sz, sr]| hypot2(x as f64 - sx, z as f64 - sz) < sr + 7.0) || hl.fall.is_some_and(|(_, p)| hypot2((x - p[0]) as f64, (z - p[1]) as f64) < 8.0) { continue; }
            let c = Cut { q: cuts[0].q, fl, ce };
            return Some(Site { kind, x, z, fl: c.fl });
        }
        None
    }
    /// Ce coffre est-il celui d'un vestige souterrain ? Sa sorte et le biome de sa salle.
    pub fn cave_chest(&self, p: [i32; 3]) -> Option<(Relic, Under)> {
        let hl = self.nearest_hall(p[0] as f64, p[2] as f64, 1)?; let st = self.hall_site(&hl)?; (st.chest() == p).then_some((st.kind, hl.kind))
    }
    /// Le vestige le plus proche d'une sorte donnée (ou de n'importe laquelle).
    pub fn nearest_site(&self, x: f64, z: f64, cells: i32, kind: Option<Relic>) -> Option<(Hall, Site)> {
        let (ci, cj) = ((x as i32).div_euclid(CELL), (z as i32).div_euclid(CELL)); let mut best: Option<(f64, Hall, Site)> = None;
        for i in ci - cells..=ci + cells { for j in cj - cells..=cj + cells { if let Some(h) = self.hall(i, j) { if let Some(st) = self.hall_site(&h) { if kind.is_some_and(|k| k != st.kind) { continue; }
            let d = hypot2(st.x as f64 - x, st.z as f64 - z); if best.is_none_or(|b| d < b.0) { best = Some((d, h, st)); } } } } }
        best.map(|b| (b.1, b.2))
    }
    fn trace_hall(&self, i: i32, j: i32) -> Option<Hall> {
        if self.flat { return None; }
        let r = |k: i32| hash_float(i, j, self.seed, 300 + k);
        if r(0) > 0.5 { return None; }
        let (x, z) = ((i * CELL) as f64 + 34.0 + (CELL - 68) as f64 * r(1), (j * CELL) as f64 + 34.0 + (CELL - 68) as f64 * r(2));
        let (mut rx, mut rz) = (17.0 + 15.0 * r(3), 15.0 + 12.0 * r(4)); let a = r(5) * std::f64::consts::TAU;
        // Ce qu'il y a au-dessus : la mer (une salle noyée sous le fond), un lac (rien), la terre.
        let c0 = self.column(x, z); let mut smin = f64::MAX; let mut seas = 0;
        for (u, v) in [(0.0, 0.0), (1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (0.7, 0.7), (-0.7, 0.7), (0.7, -0.7), (-0.7, -0.7)] {
            let c = self.column(x + u * rx, z + v * rz); if c.mix.sea > 0.0 { seas += 1; } else if c.water != 0.0 { return None; } smin = smin.min(c.h); }
        if seas > 0 && seas < 9 { return None; } // ni à cheval sur la côte
        let m = c0.mix; let k = r(20); let hilly = m.strong(Biome::Highlands) + m.strong(Biome::Mountains);
        if seas == 9 && k > 0.3 { return None; } // les grottes noyées restent rares
        let kind = if seas == 9 { Under::Coral }
            else if m.strong(Biome::Desert) > 0.4 { Under::Desert }
            else if m.temp < -0.45 || m.strong(Biome::Taiga) > 0.5 || (m.strong(Biome::Mountains) > 0.5 && c0.h > 95.0) { Under::Ice }
            else if k < 0.13 && (m.temp > 0.2 || m.strong(Biome::Steppe) > 0.3 || hilly > 0.3) { Under::Volcanic }
            else if k < 0.23 { Under::Crystal }
            else if k < 0.33 { Under::Abyss }
            else if k < 0.43 || (hilly > 0.3 && k < 0.6) { Under::Mineral }
            else if k < 0.6 && m.wet > -0.1 { Under::Fungal }
            else if k < 0.72 { Under::Glow }
            else if k < 0.84 && m.wet > -0.35 { Under::Verdant }
            else if k < 0.95 && m.strong(Biome::Steppe) < 0.5 { Under::Karst }
            else { Under::Ordinary };
        match kind { Under::Abyss => { rx *= 1.35; rz *= 1.35; } Under::Ordinary => { rx *= 0.7; rz *= 0.7; } Under::Coral => { rx *= 0.85; rz *= 0.85; } _ => {} }
        let floor = match kind { Under::Abyss => 4 + (3.0 * r(6)) as i32, Under::Volcanic => 5 + (5.0 * r(6)) as i32, Under::Crystal => 6 + (6.0 * r(6)) as i32, _ => 8 + (8.0 * r(6)) as i32 };
        // la grotte verdoyante vit près du jour : on la monte sous la surface, pour que l'aven la touche
        let floor = if kind == Under::Verdant { floor.max((smin - 16.0 - 22.0) as i32).min(40) } else { floor };
        let tall = match kind { Under::Abyss => 26.0 + 12.0 * r(7), Under::Fungal => 18.0 + 10.0 * r(7), Under::Ordinary => 9.0 + 6.0 * r(7), _ => 14.0 + 13.0 * r(7) };
        let h = tall.min(smin - 9.0 - floor as f64); if h < 10.0 { return None; }
        let mut hall = Hall { x, z, rx, rz, ca: cos(a), sa: sin(a), floor, h, kind, shaft: None, fall: None, way: None, key: (i, j) };
        let near_town = |px: f64, pz: f64, extra: f64| { let vc = crate::village::CELL; let mut near = false;
            for gx in ((px as i32) - 300).div_euclid(vc)..=((px as i32) + 300).div_euclid(vc) { for gz in ((pz as i32) - 300).div_euclid(vc)..=((pz as i32) + 300).div_euclid(vc) {
                if let Some(s) = self.site(gx, gz) { if hypot2(s.v.c[0] as f64 - px, s.v.c[2] as f64 - pz) < s.v.r as f64 + 40.0 + extra { near = true; } } } } near };
        let town = near_town(x, z, rx.max(rz));
        // L'aven : là où la voûte est mince, loin des villages et des chemins, pas en pleine forêt.
        let p_shaft = match kind { Under::Verdant => 1.0, Under::Desert => 0.9, Under::Karst => 0.8, Under::Ice => 0.6, Under::Abyss | Under::Coral | Under::Volcanic => 0.0, _ => 0.25 };
        let b = r(9) * std::f64::consts::TAU; let (sx, sz) = (x + 0.32 * rx * cos(b), z + 0.32 * rz * sin(b)); let sc = self.column(sx, sz);
        if r(8) < p_shaft && !town && sc.path < 0.1 && (sc.forest < 0.55 || kind == Under::Verdant) && sc.h - (floor as f64 + h) < 36.0 { hall.shaft = Some([sx, sz, 2.4 + 1.5 * r(10) + if kind == Under::Verdant { 1.5 } else { 0.0 }]); }
        if kind == Under::Verdant && hall.shaft.is_none() { hall.kind = Under::Karst; }
        // La cascade (d'eau, ou de lave) : une fissure dans la paroi, et ce qui tombe devant elle jusqu'au bassin.
        if r(11) < 0.85 && !matches!(kind, Under::Coral | Under::Desert | Under::Ice) {
            let b = r(12) * std::f64::consts::TAU; let (dx, dz) = (cos(b), sin(b)); let mut inside = None;
            let mut t = 2.0; while t < 70.0 { let (px, pz) = (x + dx * t, z + dz * t); let c = self.cut(&hall, px, pz); if c.q > 0.9 { break; } inside = Some((px, pz, c)); t += 0.5; }
            if let Some((px, pz, c)) = inside { let (pxi, pzi) = (px.floor() as i32, pz.floor() as i32); let top = (floor + (h * (0.45 + 0.25 * r(13))) as i32).min(c.ce - 2);
                let (sxi, szi) = ((px + dx * 1.2).floor() as i32, (pz + dz * 1.2).floor() as i32);
                if top > floor + 3 && (sxi, szi) != (pxi, pzi) { hall.fall = Some(([sxi, top, szi], [pxi, pzi])); } }
        }
        // La bouche ou la mine : un couloir en pente douce, de la surface jusqu'au bord de la salle.
        let p_mine = match kind { Under::Mineral => 0.85, Under::Ordinary => 0.5, Under::Karst | Under::Crystal => 0.15, _ => 0.0 };
        let p_mouth = match kind { Under::Coral | Under::Abyss => 0.0, _ => 0.45 };
        let mine = r(14) < p_mine; if !town && (mine || r(15) < p_mouth) {
            // une mine file droit, dans l'un des quatre sens ; une bouche part n'importe où
            let b = if mine { (r(16) * 4.0).floor() * std::f64::consts::FRAC_PI_2 } else { r(16) * std::f64::consts::TAU }; let (dx, dz) = (cos(b), sin(b));
            let mut t = 2.0; let mut edge = None; while t < 80.0 { let (px, pz) = (x + dx * t, z + dz * t); let c = self.cut(&hall, px, pz); if c.q > 0.82 { edge = Some((t, c)); break; } t += 0.5; }
            if let Some((te, c)) = edge {
                let to = [x + dx * te, c.fl as f64, z + dz * te];
                // assez long pour descendre en pente douce (une marche tous les deux pas au plus)
                let mut d = 20.0; let mut from = None;
                while d < 120.0 { let (fx, fz) = (to[0] + dx * d, to[2] + dz * d); let col = self.column(fx, fz);
                    if col.water == 0.0 && col.mix.sea == 0.0 && (col.h - to[1]) <= d * 0.48 && col.h - to[1] > 6.0 { from = Some([fx, col.h, fz]); break; } d += 4.0; }
                // pas à travers un donjon enterré (sanctuaire, ruines et leur second étage, grotte, catacombes, forge) : il la couperait
                if let Some(from) = from { if !near_town(from[0], from[2], 10.0) && !self.crosses_dungeon(from, to) { hall.way = Some(Way::Path { from, to, mine }); } }
            }
        }
        Some(hall)
    }
    /// La salle en un point du sol, sans la rugosité des parois.
    fn cut(&self, hl: &Hall, x: f64, z: f64) -> Cut {
        let (u, v) = hl.local(x, z); let n = &self.n_cave; let k = hl.key.0 as f64 * 3.7 + hl.key.1 as f64 * 1.3;
        let q = ((u / hl.rx).powi(2) + (v / hl.rz).powi(2)).sqrt() * (1.0 + 0.2 * n.n2(x / 21.0 + k, z / 21.0 - k));
        // Le plancher : des gradins, des creux où le liquide dort, et il remonte en pente contre les parois.
        let terr = ((n.n2(x / 12.0 + 40.0, z / 12.0 - 17.0) + 0.12) * 4.2).round().clamp(-2.0, 3.0);
        let mut fl = hl.floor as f64 + terr + 6.0 * smooth(0.6, 1.0, q);
        if hl.kind == Under::Desert { fl = hl.floor as f64 + 1.0 + 2.5 * (n.n2(x / 9.0, z / 9.0) + 0.5).max(0.0) + 6.0 * smooth(0.6, 1.0, q); } // des dunes, pas de bassins
        if let Some((_, p)) = hl.fall { let d = hypot2(x - p[0] as f64 - 0.5, z - p[1] as f64 - 0.5); if d < 4.0 { fl = fl.min(hl.floor as f64 - if d < 2.2 { 2.0 } else { 1.0 }); } }
        let ce = hl.floor as f64 + hl.h * (1.0 - q.min(1.0).powf(2.2)).max(0.0).sqrt() * (0.8 + 0.2 * n.n2(x / 10.0 - 9.0, z / 10.0 + 33.0)) + 1.0;
        Cut { q, fl: fl.floor() as i32 + 1, ce: ce.floor() as i32 }
    }

    /// Creuse et habille les salles (et leurs accès) qui touchent ce chunk. `heights` : la surface de chaque colonne.
    pub(crate) fn karst(&self, v: &mut [Id], ox: i32, oz: i32, heights: &[i32]) {
        let halls = self.halls_near(ox, oz, ox + CX as i32 - 1, oz + CX as i32 - 1); if halls.is_empty() { return; }
        let mut c = Carver { v, ox, oz, heights };
        for hl in &halls { self.carve_hall(&mut c, hl); }
        for hl in &halls { if let Some(Way::Path { from, to, mine }) = hl.way { self.carve_path(&mut c, hl, from, to, mine); } }
    }

    fn carve_hall(&self, cv: &mut Carver, hl: &Hall) {
        let n = &self.n_cave; let s = self.seed; let st = style(hl.kind); let wl = hl.floor; let (ox, oz) = (cv.ox, cv.oz);
        let flood = hl.kind == Under::Coral;
        // 1. Le volume, rongé de surplombs et de corniches près des parois et sous la voûte.
        for lz in 0..CX as i32 { for lx in 0..CX as i32 {
            let (x, z) = (ox + lx, oz + lz); let (xf, zf) = (x as f64 + 0.5, z as f64 + 0.5); let c = self.cut(hl, xf, zf); if c.q > 1.15 { continue; }
            let surf = cv.surf(lx, lz); let ce = c.ce.min(surf - 5); if ce <= c.fl { continue; }
            for y in (c.fl - 2).max(2)..=(ce + 3).min(CH as i32 - 2) {
                let yf = y as f64 + 0.5; let edge = c.q > 0.78 || y >= ce - 3 || y <= c.fl;
                let rough = if edge { n.n3(xf / 6.0, yf / 5.0, zf / 6.0) } else { 0.0 };
                let inside = c.q + 0.1 * rough < 1.0 && y >= c.fl + (rough > 0.45) as i32 && (y as f64) < ce as f64 + 2.0 * rough;
                if inside { cv.set(x, y, z, if flood || (y <= wl && c.q < 0.95 && st.liquid != 0) { if flood { B::WATER } else { st.liquid } } else { 0 }); }
            }
            // L'aven : un puits jusqu'au ciel, aux parois irrégulières.
            if let Some([sx, sz, sr]) = hl.shaft { let d = hypot2(xf - sx, zf - sz); let rr = sr * (1.0 + 0.3 * n.n2(xf / 4.0 + 7.0, zf / 4.0 - 3.0));
                if d < rr { for y in (ce - 1).max(c.fl)..CH as i32 - 1 { if y <= wl { continue; } let wob = 0.6 * n.n3(xf / 5.0, y as f64 / 4.0, zf / 5.0); if d < rr + wob { cv.set(x, y, z, 0); } } } }
        } }
        // 2. La roche : celle du biome, avec ses veines ; la croûte sur le sol et le bas des parois.
        for lz in 0..CX as i32 { for lx in 0..CX as i32 {
            let (x, z) = (ox + lx, oz + lz); let (xf, zf) = (x as f64 + 0.5, z as f64 + 0.5); let c = self.cut(hl, xf, zf); if c.q > 1.3 { continue; }
            let surf = cv.surf(lx, lz);
            for y in (c.fl - 4).max(2)..=(c.ce + 5).min(surf - 2) {
                let Some(b) = cv.get(x, y, z) else { continue }; if b != B::STONE && b != B::GRANITE && b != B::DIRT { continue; }
                let wet = n.n3(xf / 7.0 + 11.0, y as f64 / 3.0, zf / 7.0 - 5.0); let up = cv.get(x, y + 1, z); let open_up = up == Some(0) || up.is_some_and(is_water) || up == Some(B::LAVA) || up == Some(B::ICE);
                let vein = n.n3(xf / 11.0 - 3.0, y as f64 / 9.0, zf / 11.0 + 8.0).abs() < 0.06;
                let mut nb = if (open_up && wet > if hl.kind == Under::Glow { 0.34 } else { 0.05 }) || (hl.kind == Under::Karst && wet > 0.42) { st.crust } /* la mousse lumineuse en plaques, pas en tapis */ else if vein || wet > 0.5 { st.wall2 } else { st.wall };
                if hl.kind == Under::Mineral && !open_up { let o = hash_float(x, y * 7 + z, s, 390); let deep = y < 30; // des gisements, pas un semis uniforme
                    let lode = n.n3(xf / 6.0 + 21.0, y as f64 / 5.0, zf / 6.0 - 4.0); if lode > 0.38 { nb = if o < 0.35 { B::COAL_ORE } else if o < 0.65 { B::COPPER_ORE } else if o < 0.85 || !deep { B::TIN_ORE } else { B::IRON_ORE }; } }
                if hl.kind == Under::Volcanic && open_up && wet > 0.35 { nb = B::MAGMA; }
                cv.set(x, y, z, nb);
            }
        } }
        // 3. Les concrétions, sur une grille fixe du monde : stalactites, stalagmites, colonnes, et les grandes formations.
        let site = self.hall_site(hl);
        let (pd, pu) = points(st.points); let r = hl.reach().min(hl.rx.max(hl.rz) * 1.3 + 6.0) as i32;
        for gz in (hl.z as i32 - r).div_euclid(STEP)..=(hl.z as i32 + r).div_euclid(STEP) { for gx in (hl.x as i32 - r).div_euclid(STEP)..=(hl.x as i32 + r).div_euclid(STEP) {
            let h = |k: i32| hash_float(gx, gz, s, 340 + k);
            let (cx, cz) = (gx * STEP + 1 + (3.0 * h(0)) as i32, gz * STEP + 1 + (3.0 * h(1)) as i32);
            if cx + 9 < ox || cz + 9 < oz || cx - 9 >= ox + CX as i32 || cz - 9 >= oz + CX as i32 { continue; }
            let c = self.cut(hl, cx as f64 + 0.5, cz as f64 + 0.5); if c.q > 0.86 { continue; }
            if let Some([sx, sz, sr]) = hl.shaft { if hypot2(cx as f64 - sx, cz as f64 - sz) < sr + 2.5 { continue; } }
            if let Some((_, p)) = hl.fall { if hypot2((cx - p[0]) as f64, (cz - p[1]) as f64) < 3.5 { continue; } }
            let gap = (c.ce - c.fl) as f64; if gap < 5.0 { continue; }
            if site.is_some_and(|st| hypot2((cx - st.x) as f64, (cz - st.z) as f64) < 8.5) { continue; } // la place du vestige reste dégagée
            // les grandes formations propres au biome
            match hl.kind {
                Under::Fungal if h(5) < 0.3 && gap > 9.0 => { self.giant_mushroom(cv, hl, cx, cz, c, h(6), h(7), h(8)); continue; }
                Under::Crystal if h(5) < 0.32 => { self.crystal(cv, cx, cz, c, h(6), h(7), h(8), h(9)); continue; }
                Under::Glow | Under::Abyss if h(5) < 0.07 => { self.crystal(cv, cx, cz, c, h(6), h(7), 0.3, 0.6); continue; }
                Under::Desert if h(5) < 0.14 && c.fl > wl => { self.ruin(cv, cx, cz, c, h(6), h(7)); continue; }
                Under::Coral if h(5) < 0.4 => { self.coral(cv, cx, cz, c, h(6)); continue; }
                _ => {}
            }
            let kind = h(2) / st.density.max(0.05); let big = h(3) * h(3);
            let (down, up, pillar) = (kind < 0.55, (0.55..0.82).contains(&kind), (0.82..0.88).contains(&kind) && gap < 15.0);
            if !(down || up || pillar) { continue; }
            let r0 = if pillar { 1.1 + 1.4 * big } else { 0.55 + 1.7 * big };
            let len = if pillar { gap + 6.0 } else { (2.0 + r0 * 3.2 + 3.0 * h(4)).min(gap * if down { 0.62 } else { 0.45 }) };
            for dz in -3..=3 { for dx in -3..=3 {
                let (x, z) = (cx + dx, cz + dz); let d = hypot2(dx as f64, dz as f64); if d > r0 + 0.6 { continue; } if cv.get(x, 0, z).is_none() { continue; }
                let span: Vec<(i32, f64)> = if pillar { (c.fl - 2..=c.ce + 3).map(|y| (y, ((y - c.fl) as f64 / gap).clamp(0.0, 1.0))).collect() }
                    else if down { (c.ce + 1 - len as i32..=c.ce + 3).map(|y| (y, ((c.ce - y) as f64 / len).max(0.0))).collect() }
                    else { (c.fl - 3..=c.fl - 1 + len as i32).map(|y| (y, ((y - c.fl + 1) as f64 / len).max(0.0))).collect() };
                for (y, t) in span {
                    let rr = if pillar { r0 * (1.0 - 0.32 * sin(std::f64::consts::PI * t)) } else { r0 * (1.0 - t).max(0.0).powf(1.15) };
                    let Some(here) = cv.get(x, y, z) else { continue }; if here != 0 && !is_water(here) && here != B::LAVA { continue; }
                    if d <= rr && rr >= 0.62 { cv.set(x, y, z, st.body); }
                    else if dx == 0 && dz == 0 && !pillar && t < 1.0 { let k = if rr > 0.42 { 0 } else if rr > 0.24 { 1 } else { 2 }; cv.set(x, y, z, if down { pd[k] } else { pu[k] }); }
                }
            } }
        } }
        // 4. La cascade : la source dans sa fissure, et une nappe de deux blocs de large qui tombe jusqu'au bassin.
        if let Some(([sx, sy, sz], [px, pz])) = hl.fall { let liq = if st.liquid == B::LAVA { B::LAVA } else { B::WATER };
            cv.set(sx, sy, sz, liq); cv.set(sx, sy + 1, sz, st.wall);
            let (ax, az) = if (sx - px).abs() >= (sz - pz).abs() { (0, 1) } else { (1, 0) };
            for (qx, qz) in [(px, pz), (px + ax, pz + az)] {
                let c = self.cut(hl, qx as f64 + 0.5, qz as f64 + 0.5); let bottom = c.fl.min(wl);
                for y in bottom..=sy { if let Some(b) = cv.get(qx, y, qz) { if b == 0 || is_water(b) || b == B::LAVA || shape_point(b) { cv.set(qx, y, qz, liq); } } }
                cv.set(qx + (sx - px).signum(), sy, qz + (sz - pz).signum(), liq);
            }
        }
        // 5. Le gouffre de l'abîme : une faille qui plonge jusqu'au fond du monde, de la lave tout en bas.
        if hl.kind == Under::Abyss { let (ga, gb) = (hl.ca, hl.sa);
            for lz in 0..CX as i32 { for lx in 0..CX as i32 { let (x, z) = (ox + lx, oz + lz); let (u, w) = ((x as f64 - hl.x) * ga + (z as f64 - hl.z) * gb, -(x as f64 - hl.x) * gb + (z as f64 - hl.z) * ga);
                let width = 2.2 + 1.2 * n.n2(u / 9.0, 3.0) - (u / (hl.rx * 0.7)).powi(2) * 3.0; if w.abs() > width { continue; }
                let c = self.cut(hl, x as f64 + 0.5, z as f64 + 0.5); for y in 2..c.fl + 1 { cv.set(x, y, z, if y <= 3 { B::LAVA } else { 0 }); } } }
        }
        // 6. L'habillage : la vie, les lumières, la neige, le sable, les géodes.
        for lz in 0..CX as i32 { for lx in 0..CX as i32 {
            let (x, z) = (ox + lx, oz + lz); let (xf, zf) = (x as f64 + 0.5, z as f64 + 0.5); let c = self.cut(hl, xf, zf); if c.q > 1.0 { continue; }
            let lightd = hl.shaft.map_or(99.0, |[sx, sz, sr]| (hypot2(xf - sx, zf - sz) - sr).max(0.0));
            let rr = hash_float(x, z, s, 360); let r2 = hash_float(x, z, s, 361);
            let (gx, gz) = (x.div_euclid(9), z.div_euclid(9)); let lamp = hash_float(gx, gz, s, 370) < 0.32 && x == gx * 9 + 2 + (hash_float(gx, gz, s, 371) * 5.0) as i32 && z == gz * 9 + 2 + (hash_float(gx, gz, s, 372) * 5.0) as i32 && c.q < 0.85;
            // le sol : la première case vide au-dessus d'un bloc plein
            for y in (c.fl - 2).max(2)..(c.ce + 2).min(CH as i32 - 2) {
                let (Some(here), Some(under)) = (cv.get(x, y, z), cv.get(x, y - 1, z)) else { continue };
                let open = here == 0 || (flood && is_water(here)); if !open || !is_opaque(under) || under == B::LAVA { continue; }
                let wet_edge = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|(a, b)| cv.get(x + a, y - 1, z + b).is_some_and(is_water));
                let lit = lightd < 7.0 + 3.0 * n.n2(xf / 3.0, zf / 3.0);
                match hl.kind {
                    Under::Karst | Under::Ordinary | Under::Mineral => {
                        if lamp && lightd > 6.0 && y > wl && hl.kind != Under::Ordinary { cv.set(x, y, z, B::LANTERN); }
                        else if lit { cv.set(x, y - 1, z, B::MOSSY_STONE); if rr < 0.22 { cv.set(x, y, z, B::FERN); } else if rr < 0.3 { cv.set(x, y, z, B::TALL_GRASS); } }
                        else if hl.kind != Under::Karst && rr > 0.986 { cv.set(x, y, z, B::COBWEB); } // les araignées ont tendu leurs toiles
                        else if wet_edge && rr < 0.05 { cv.set(x, y, z, [B::GLOWCAP_BLUE, B::GLOWCAP_CYAN, B::GLOWCAP][(r2 * 3.0) as usize % 3]); }
                        else if rr < 0.006 { cv.set(x, y, z, B::GLOWCAP_VIOLET); }
                    }
                    Under::Verdant => { // un jardin : herbe, fougères, fleurs, nénuphars sur l'eau
                        if under != B::FLOWSTONE && !shape_point(under) { cv.set(x, y - 1, z, if lit || rr < 0.6 { B::GRASS } else { B::MOSSY_STONE }); }
                        let f = rr / (if lit { 1.0 } else { 0.45 }); cv.set(x, y, z, if f < 0.28 { B::TALL_GRASS } else if f < 0.42 { B::FERN } else if f < 0.48 { B::PINK_FLOWER } else if f < 0.52 { B::YELLOW_FLOWER } else if f < 0.54 { B::MUSHROOM } else { 0 });
                    }
                    Under::Fungal => { cv.set(x, y, z, if rr < 0.12 { B::MUSHROOM } else if rr < 0.17 { [B::GLOWCAP_PINK, B::GLOWCAP_VIOLET, B::GLOWCAP_CYAN, B::GLOWCAP][(r2 * 4.0) as usize % 4] } else { 0 }); }
                    Under::Glow => { if n.n2(xf / 5.0 + 3.0, zf / 5.0) > 0.45 || (wet_edge && rr < 0.5) { cv.set(x, y - 1, z, B::GLOW_MOSS); } cv.set(x, y, z, if rr < 0.08 { [B::GLOWCAP_CYAN, B::GLOWCAP_BLUE, B::GLOWCAP_VIOLET][(r2 * 3.0) as usize % 3] } else if rr > 0.97 { B::CRYSTAL_BLUE_POINT_UP_2 } else { 0 }); }
                    Under::Crystal => { if rr < 0.05 { cv.set(x, y, z, [B::CRYSTAL_VIOLET_POINT_UP_2, B::CRYSTAL_PINK_POINT_UP_2, B::CRYSTAL_BLUE_POINT_UP_2, B::CRYSTAL_VIOLET_POINT_UP_1][(r2 * 4.0) as usize % 4]); } }
                    Under::Ice => { if rr < 0.5 { cv.set(x, y, z, by_name(&format!("snow_layer{}", if r2 < 0.5 { String::new() } else { format!("_{}", 2 + (r2 * 4.0) as i32) })).unwrap_or(B::SNOW_LAYER)); } else if rr > 0.985 { cv.set(x, y, z, B::CRYSTAL_WHITE_POINT_UP_1); } }
                    Under::Volcanic => { if rr < 0.01 && y > wl { cv.set(x, y, z, B::OBSIDIAN); } }
                    Under::Desert => { if rr < 0.03 { cv.set(x, y, z, B::FERN_DRY); } }
                    Under::Abyss => { if rr < 0.004 { cv.set(x, y, z, B::GLOWCAP_BLUE); } }
                    Under::Coral => { cv.set(x, y, z, if rr < 0.25 { B::SEAGRASS } else if rr < 0.33 { B::CORAL } else if rr < 0.39 { B::CORAL_FAN } else { B::WATER });
                        if (0.94..0.975).contains(&rr) { let hh = 3 + (r2 * 8.0) as i32; for k in 0..hh { if cv.get(x, y + k, z).is_some_and(is_water) { cv.set(x, y + k, z, if k == hh - 1 { B::KELP_TOP } else { B::KELP }); } } } }
                }
                break;
            }
            // la voûte : lianes autour de l'aven, lianes lumineuses, racines, stalactites de glace serrées
            let hang = |cv: &mut Carver, b: Id, len: i32| { let mut y = c.ce + 2; while y > c.fl && cv.get(x, y, z).is_some_and(|b| b != 0 && !is_water(b)) { y -= 1; } for k in 0..len { if cv.get(x, y - k, z) == Some(0) { cv.set(x, y - k, z, b); } else { break; } } };
            let len = 2 + (hash_float(x, z, s, 362) * 6.0) as i32;
            match hl.kind {
                Under::Verdant if rr > 0.5 => hang(cv, if r2 < 0.15 { B::ROOTS } else { B::VINE }, len + 2),
                Under::Glow if rr > 0.72 => hang(cv, B::GLOW_VINE, len + 3),
                Under::Fungal if rr > 0.94 => hang(cv, B::ROOTS, len / 2 + 1),
                _ if lightd < 5.0 && lightd > 0.0 && rr > 0.55 => hang(cv, B::VINE, len),
                _ => {}
            }
        } }
        // 7. Les géodes de la grotte minérale : des bulles tapissées de cristal, ouvertes par la salle.
        if hl.kind == Under::Mineral { for k in 0..3 { let h = |q: i32| hash_float(hl.key.0 * 3 + k, hl.key.1, s, 395 + q);
            let a = h(0) * std::f64::consts::TAU; let (gx, gz) = (hl.x + hl.rx * 0.95 * cos(a), hl.z + hl.rz * 0.95 * sin(a)); let gy = hl.floor as f64 + 2.0 + h(1) * hl.h * 0.4; let gr = 2.6 + 1.6 * h(2);
            let lining = if h(3) < 0.5 { B::CRYSTAL_VIOLET } else { B::CRYSTAL_WHITE };
            for y in (gy - gr - 1.0) as i32..=(gy + gr + 1.0) as i32 { for z in (gz - gr - 1.0) as i32..=(gz + gr + 1.0) as i32 { for x in (gx - gr - 1.0) as i32..=(gx + gr + 1.0) as i32 {
                let d = ((x as f64 + 0.5 - gx).powi(2) + (y as f64 + 0.5 - gy).powi(2) + (z as f64 + 0.5 - gz).powi(2)).sqrt(); let Some(b) = cv.get(x, y, z) else { continue };
                if d < gr - 1.0 { if b != 0 { cv.set(x, y, z, if hash_float(x, y * 13 + z, s, 399) < 0.12 { B::CRYSTAL_VIOLET_POINT_UP_2 } else { 0 }); } }
                else if d < gr { if b != 0 { cv.set(x, y, z, lining); } } else if d < gr + 0.9 && b != 0 { cv.set(x, y, z, B::FLOWSTONE); }
            } } } } }
        // 8. Le vestige de la salle, en dernier : rien ne pousse dessus.
        if let Some(site) = site { self.relic(cv, hl, site); }
    }

    /// Bâtit le vestige : d'abord son assise (le sol comblé, l'air dégagé), puis ce que les hommes y ont laissé.
    fn relic(&self, cv: &mut Carver, hl: &Hall, st: Site) {
        let (x0, z0, fl, s) = (st.x, st.z, st.fl, self.seed); if x0 + 5 < cv.ox || z0 + 5 < cv.oz || x0 - 5 >= cv.ox + CX as i32 || z0 - 5 >= cv.oz + CX as i32 { return; }
        let wall = style(hl.kind).wall; let h = |x: i32, z: i32, k: i32| hash_float(x, z, s, 470 + k);
        let pad = |cv: &mut Carver, r: i32, top: Option<Id>, clear: i32| { for dz in -r..=r { for dx in -r..=r { let (x, z) = (x0 + dx, z0 + dz);
            for y in fl - 4..fl { if cv.get(x, y, z).is_some_and(|b| !is_opaque(b)) { cv.set(x, y, z, wall); } } if let Some(t) = top { cv.set(x, fl - 1, z, t); }
            for y in fl..fl + clear { if cv.get(x, y, z).is_some() { cv.set(x, y, z, 0); } } } } };
        match st.kind {
            Relic::Camp => { // un foyer froid, un abri de rondins sur le coffre et les tonneaux, des caisses, un tas de charbon, une lanterne sur son poteau
                pad(cv, 3, None, 4);
                for dz in -1..=1 { for dx in -1..=1 { if h(x0 + dx, z0 + dz, 0) < 0.7 { cv.set(x0 + dx, fl - 1, z0 + dz, B::GRAVEL); } } }
                cv.set(x0, fl, z0, B::CAMPFIRE_OUT);
                for x in [x0 - 3, x0 + 1] { for y in fl..fl + 2 { cv.set(x, y, z0 - 3, B::OAK_POST); } } for x in x0 - 3..=x0 + 1 { cv.set(x, fl + 2, z0 - 3, B::OAK_POST_X); for z in z0 - 2..=z0 - 1 { cv.set(x, fl + 2, z, B::PLANKS_SLAB); } }
                let [cx, cy, cz] = st.chest(); cv.set(cx, cy, cz, B::CHEST); cv.set(x0 - 1, fl, z0 - 2, B::BARREL); cv.set(x0, fl, z0 - 2, B::CRATE); if h(x0, z0, 1) < 0.6 { cv.set(x0, fl + 1, z0 - 2, B::CRATE); }
                cv.set(x0 + 2, fl, z0 + 1, B::COAL_HEAP); cv.set(x0 - 2, fl, z0 + 1, B::STOOL); cv.set(x0 + 1, fl, z0 + 2, B::STOOL); cv.set(x0 - 3, fl, z0 + 2, B::SACK);
                for y in fl..fl + 2 { cv.set(x0 + 3, y, z0 + 3, B::OAK_POST); } cv.set(x0 + 3, fl + 2, z0 + 3, B::LANTERN);
            }
            Relic::Altar => { // une estrade de pierre polie, sa marche de dalles, un grand cristal dressé au centre, quatre piliers à lanterne, une offrande
                pad(cv, 4, None, 6); let m = ["crystal_violet", "crystal_blue", "crystal_pink", "crystal_white"][(h(x0, z0, 2) * 4.0) as usize % 4]; let id = |n: &str| by_name(n).unwrap_or(B::STONE);
                for dz in -3..=3i32 { for dx in -3..=3i32 { let edge = dx.abs() == 3 || dz.abs() == 3; cv.set(x0 + dx, fl, z0 + dz, if edge { B::STONE_BRICKS_SLAB } else { B::POLISHED_STONE }); cv.set(x0 + dx, fl - 1, z0 + dz, B::STONE_BRICKS); } }
                cv.set(x0, fl + 1, z0, B::CHISELED_STONE); cv.set(x0, fl + 2, z0, id(m)); cv.set(x0, fl + 3, z0, id(m)); cv.set(x0, fl + 4, z0, id(&format!("{m}_point_up_1")));
                for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { cv.set(x0 + dx, fl + 1, z0 + dz, id(&format!("{m}_point_up_{}", if dz == -1 { 2 } else { 1 }))); }
                for (dx, dz) in [(-2, -2), (2, -2), (-2, 2), (2, 2)] { cv.set(x0 + dx, fl + 1, z0 + dz, B::STONE_PILLAR); cv.set(x0 + dx, fl + 2, z0 + dz, B::STONE_PILLAR); cv.set(x0 + dx, fl + 3, z0 + dz, B::LANTERN); }
                let [cx, cy, cz] = st.chest(); cv.set(cx, cy, cz, B::CHEST);
            }
            Relic::Vault => { // un caveau de maçonnerie à demi ruiné : quatre murs, une porte ouverte, un toit de dalles, le coffre au fond entre deux lanternes
                let (a, b2, cap) = match hl.kind { Under::Desert => (B::SANDSTONE, B::RED_SANDSTONE, B::SANDSTONE), Under::Volcanic => (B::BASALT, B::OBSIDIAN, B::BASALT), Under::Abyss => (B::ABYSS_ROCK, B::CHISELED_STONE, B::STONE_BRICKS), _ => (B::STONE_BRICKS, B::COBBLESTONE, B::STONE_BRICKS) };
                pad(cv, 3, Some(a), 4);
                for dz in -3..=3i32 { for dx in -3..=3i32 { let (x, z) = (x0 + dx, z0 + dz); let edge = dx.abs() == 3 || dz.abs() == 3; let door = dz == -3 && dx == 0;
                    if edge { for y in fl..fl + 3 { let worn = h(x, z, 3 + y - fl); if door && y < fl + 2 { continue; } if y == fl + 2 && worn < 0.22 { continue; } cv.set(x, y, z, if worn > 0.7 { b2 } else { a }); } }
                    else if h(x, z, 8) > 0.18 { cv.set(x, fl + 3, z, if cap == B::STONE_BRICKS { B::STONE_BRICKS_SLAB } else { cap }); } } }
                let [cx, cy, cz] = st.chest(); cv.set(cx, cy, cz, B::CHEST);
                for dx in [-2, 2] { cv.set(x0 + dx, fl, z0 + 2, B::CHISELED_STONE); cv.set(x0 + dx, fl + 1, z0 + 2, B::LANTERN); }
                cv.set(x0 - 2, fl, z0 - 1, B::BARREL); if h(x0, z0, 9) < 0.5 { cv.set(x0 + 2, fl, z0 - 2, B::CRATE); }
            }
        }
    }

    /// Un champignon géant : un pied pâle, un chapeau en dôme (rouge à pois, violet ou bleu), des lamelles qui luisent dessous.
    #[allow(clippy::too_many_arguments)]
    fn giant_mushroom(&self, cv: &mut Carver, hl: &Hall, cx: i32, cz: i32, c: Cut, a: f64, b: f64, cc: f64) {
        let _ = hl; let gap = (c.ce - c.fl) as f64; let stem = (4.0 + a * (gap - 9.0).max(1.0) * 0.8).min(gap - 5.0) as i32; let rad = 2.6 + 3.2 * b; let cap = [B::MUSHROOM_CAP_RED, B::MUSHROOM_CAP_VIOLET, B::MUSHROOM_CAP_BLUE][(cc * 3.0) as usize % 3];
        let top = c.fl + stem; let sr = 0.7 + 0.5 * b;
        for dz in -7..=7 { for dx in -7..=7 { let (x, z) = (cx + dx, cz + dz); let d = hypot2(dx as f64, dz as f64);
            if d <= sr { for y in c.fl - 1..top { if cv.get(x, y, z).is_some_and(|q| q == 0 || is_water(q)) || y == c.fl - 1 { cv.set(x, y, z, B::MUSHROOM_STEM); } } }
            if d < rad { if cv.get(x, top, z).is_some_and(|q| q == 0) && d > sr { cv.set(x, top, z, B::MUSHROOM_GILLS); } }
            for (k, y) in (top + 1..=top + 3).enumerate() { let rr = rad * (1.0 - (k as f64 / 3.2).powi(2)).sqrt() + 0.3; if d < rr && cv.get(x, y, z).is_some_and(|q| q == 0) { cv.set(x, y, z, cap); } }
            if (rad - 0.9..rad + 0.4).contains(&d) && cv.get(x, top, z).is_some_and(|q| q == 0 || q == B::MUSHROOM_GILLS) { cv.set(x, top, z, cap); } // le rebord
        } }
    }
    /// Une grappe de cristaux : de grands prismes inclinés qui s'effilent en pointe, et des aiguilles autour.
    #[allow(clippy::too_many_arguments)]
    fn crystal(&self, cv: &mut Carver, cx: i32, cz: i32, c: Cut, a: f64, b: f64, cc: f64, dd: f64) {
        let col = [B::CRYSTAL_VIOLET, B::CRYSTAL_PINK, B::CRYSTAL_BLUE, B::CRYSTAL_VIOLET, B::CRYSTAL_WHITE][(a * 5.0) as usize % 5];
        let name = BLOCKS[col as usize].name; let (pd, pu) = points(name);
        let hanging = dd > 0.72; let base = if hanging { [cx as f64 + 0.5, c.ce as f64 + 1.0, cz as f64 + 0.5] } else { [cx as f64 + 0.5, c.fl as f64 - 0.5, cz as f64 + 0.5] };
        for k in 0..(2 + (b * 3.0) as i32) { let hk = |q: i32| hash_float(cx * 5 + k, cz, self.seed, 410 + q);
            let tilt = 0.15 + 0.5 * hk(0) * (k > 0) as i32 as f64; let az = hk(1) * std::f64::consts::TAU; let len = (3.0 + 7.0 * cc * hk(2) + 3.0 * (k == 0) as i32 as f64).min((c.ce - c.fl) as f64 * 0.8); let r0 = 0.7 + 1.0 * hk(3) * (k == 0) as i32 as f64 + 0.3 * cc;
            let dir = { let up = if hanging { -1.0 } else { 1.0 }; [tilt * cos(az), up * (1.0 - tilt * 0.5), tilt * sin(az)] }; let nl = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt(); let dir = [dir[0] / nl, dir[1] / nl, dir[2] / nl];
            let ext = (len + r0 + 1.0) as i32;
            for y in (base[1] as i32 - ext)..=(base[1] as i32 + ext) { for z in cz - ext..=cz + ext { for x in cx - ext..=cx + ext {
                let p = [x as f64 + 0.5 - base[0], y as f64 + 0.5 - base[1], z as f64 + 0.5 - base[2]]; let t = p[0] * dir[0] + p[1] * dir[1] + p[2] * dir[2]; if t < -1.0 || t > len { continue; }
                let perp = ((p[0] - dir[0] * t).powi(2) + (p[1] - dir[1] * t).powi(2) + (p[2] - dir[2] * t).powi(2)).sqrt(); let rr = if t < len * 0.62 { r0 } else { r0 * (len - t) / (len * 0.38) };
                let Some(here) = cv.get(x, y, z) else { continue }; let free = here == 0 || is_water(here);
                if perp <= rr.max(0.0) && (free || t < 0.5) { cv.set(x, y, z, col); }
                else if perp < 0.5 && rr < 0.62 && rr > -0.2 && free { let q = if rr > 0.35 { 0 } else if rr > 0.15 { 1 } else { 2 }; cv.set(x, y, z, if dir[1] < 0.0 { pd[q] } else { pu[q] }); }
            } } }
        }
    }
    /// Une ruine ensevelie : des colonnes de grès (certaines brisées), un linteau, des dalles, un brasero.
    fn ruin(&self, cv: &mut Carver, cx: i32, cz: i32, c: Cut, a: f64, b: f64) {
        let hgt = (4 + (a * 4.0) as i32).min(c.ce - c.fl - 2); let axis_x = b < 0.5; let span = 4;
        for k in [-span / 2, span / 2] { let (x, z) = if axis_x { (cx + k, cz) } else { (cx, cz + k) }; let broken = hash_float(x, z, self.seed, 420) < 0.3;
            let top = if broken { hgt / 2 + 1 } else { hgt };
            for y in c.fl - 1..c.fl + top { cv.set(x, y, z, if y == c.fl - 1 || y == c.fl + top - 1 { B::RED_SANDSTONE } else { B::SANDSTONE }); }
            if !broken && b > 0.25 { cv.set(x, c.fl + top, z, B::TORCH); } }
        if a > 0.4 { for k in -span / 2..=span / 2 { let (x, z) = if axis_x { (cx + k, cz) } else { (cx, cz + k) }; if cv.get(x, c.fl + hgt, z) == Some(0) || cv.get(x, c.fl + hgt, z) == Some(B::TORCH) { cv.set(x, c.fl + hgt, z, B::SANDSTONE); } } }
        for dz in -2..=2 { for dx in -2..=2 { if hash_float(cx + dx, cz + dz, self.seed, 421) < 0.6 && cv.get(cx + dx, c.fl, cz + dz) == Some(0) { cv.set(cx + dx, c.fl - 1, cz + dz, B::SANDSTONE); } } }
    }
    /// Un massif de corail : des blocs colorés empilés, des éventails autour.
    fn coral(&self, cv: &mut Carver, cx: i32, cz: i32, c: Cut, a: f64) {
        let cols = [B::CORAL_BLOCK_RED, B::CORAL_BLOCK_ORANGE, B::CORAL_BLOCK_PURPLE, B::CORAL_BLOCK_YELLOW, B::CORAL_BLOCK_BLUE];
        for dz in -2..=2i32 { for dx in -2..=2i32 { let h = hash_float(cx + dx, cz + dz, self.seed, 430); let tall = ((3.0 - hypot2(dx as f64, dz as f64)) * (1.0 + 2.0 * a) * h) as i32;
            for y in c.fl..c.fl + tall.max(0) { if cv.get(cx + dx, y, cz + dz).is_some_and(is_water) { cv.set(cx + dx, y, cz + dz, cols[((h * 50.0) as usize + y as usize) % 5]); } }
            if tall > 0 && cv.get(cx + dx, c.fl + tall, cz + dz).is_some_and(is_water) && h > 0.5 { cv.set(cx + dx, c.fl + tall, cz + dz, B::CORAL_FAN); } } }
    }

    /// Le couloir de `from` à `to` passerait-il à moins de 64 blocs d'un donjon enterré ?
    fn crosses_dungeon(&self, from: [f64; 3], to: [f64; 3]) -> bool {
        let (mx, mz) = ((from[0] + to[0]) / 2.0, (from[2] + to[2]) / 2.0); let half = (to[0] - from[0]).hypot(to[2] - from[2]) / 2.0;
        self.places_near(mx, mz, half + 64.0).iter().any(|p| matches!(p.kind, crate::sites::Lieu::Sanctuaire | crate::sites::Lieu::Ruines | crate::sites::Lieu::Grotte | crate::sites::Lieu::Catacombes | crate::sites::Lieu::Forge | crate::sites::Lieu::Mine | crate::sites::Lieu::Arene | crate::sites::Lieu::Tombeau)
            && (0..=32).any(|k| { let t = k as f64 / 32.0; (from[0] + (to[0] - from[0]) * t - p.x as f64).hypot(from[2] + (to[2] - from[2]) * t - p.z as f64) < 64.0 }))
    }
    /// Une bouche naturelle (un boyau sinueux) ou une vieille mine (droite, boisée, rails et lanternes), en pente douce.
    fn carve_path(&self, cv: &mut Carver, hl: &Hall, from: [f64; 3], to: [f64; 3], mine: bool) {
        let n = &self.n_cave; let s = self.seed; let (ox, oz) = (cv.ox, cv.oz);
        let (dx, dz) = (to[0] - from[0], to[2] - from[2]); let len = dx.hypot(dz); let (ux, uz) = (dx / len, dz / len); let phase = hl.key.0 as f64 * 1.7 + hl.key.1 as f64;
        let wig = |t: f64| if mine { 0.0 } else { 4.0 * sin(t * std::f64::consts::TAU * 1.5 + phase) * (t * (1.0 - t) * 4.0) };
        let floor_at = |t: f64| (from[1] - 1.0 + (to[1] - from[1] + 1.0) * t).round() as i32;
        let along_x = ux.abs() > uz.abs();
        for lz in 0..CX as i32 { for lx in 0..CX as i32 {
            let (x, z) = (ox + lx, oz + lz); let (px, pz) = (x as f64 + 0.5 - from[0], z as f64 + 0.5 - from[2]);
            let t = ((px * ux + pz * uz) / len).clamp(0.0, 1.0); let lat = -px * uz + pz * ux - wig(t);
            let half = if mine { 2.5 } else { 2.3 + 0.8 * n.n2(t * 9.0, phase) }; if lat.abs() > half || (px * ux + pz * uz) < -2.0 || (px * ux + pz * uz) > len + 1.0 { continue; }
            let fy = floor_at(t); let surf = cv.surf(lx, lz);
            if mine {
                let sdist = (t * len).round() as i32; let support = sdist % 4 == 0; let edge = lat.abs() > 1.5;
                for y in fy..=fy + 3 { if y < CH as i32 - 1 { cv.set(x, y, z, 0); } }
                if cv.get(x, fy - 1, z).is_some_and(|b| b == 0 || is_water(b) || b == B::LAVA) { cv.set(x, fy - 1, z, B::PLANKS); } // un plancher là où le couloir croise un vide
                // Un cadre de boisage en rondins fins : deux montants, une traverse, des coudes aux angles, deux jambes de force.
                if support {
                    let inward = crate::shapes::facing_of(lat.signum() * uz, -lat.signum() * ux) as usize; // vers l'axe du couloir
                    const ELBOW: [Id; 4] = [B::OAK_POST_ANGLE_TOP, B::OAK_POST_ANGLE_TOP_1, B::OAK_POST_ANGLE_TOP_2, B::OAK_POST_ANGLE_TOP_3];
                    const BRACE: [Id; 4] = [B::OAK_POST_DIAG, B::OAK_POST_DIAG_1, B::OAK_POST_DIAG_2, B::OAK_POST_DIAG_3];
                    if edge { for y in fy..fy + 3 { cv.set(x, y, z, B::OAK_POST); } cv.set(x, fy + 3, z, ELBOW[inward]); }
                    else { cv.set(x, fy + 3, z, if along_x { B::OAK_POST_Z } else { B::OAK_POST_X }); if lat.abs() > 0.5 { cv.set(x, fy + 2, z, BRACE[inward]); } }
                }
                if lat.abs() < 0.5 && !support { cv.set(x, fy, z, if along_x { B::RAIL_X } else { B::RAIL_Z }); }
                if edge && sdist % 4 == 1 && fy + 4 < surf && hash_float(x, z, self.seed, 481) < 0.3 { cv.set(x, fy + 2, z, B::COBWEB); } // une toile dans l'angle, derrière un cadre
                if lat.abs() < 0.5 && support && sdist % 12 == 0 && fy + 4 < surf { cv.set(x, fy + 2, z, B::LANTERN); }
                // les parois de la mine : le filon qu'on suivait
                if edge && !support { for y in fy..=fy + 3 { let o = hash_float(x, y * 5 + z, s, 440); if o < 0.08 { cv.set(x + if along_x { 0 } else { lat.signum() as i32 }, y, z + if along_x { lat.signum() as i32 } else { 0 }, if o < 0.04 { B::COAL_ORE } else if o < 0.065 { B::COPPER_ORE } else { B::IRON_ORE }); } } }
            } else {
                let mid = fy as f64 + 1.6; let hh = 2.0 + 0.6 * n.n2(t * 7.0 + 3.0, phase);
                for y in fy - 1..=fy + 4 { let e = ((y as f64 + 0.5 - mid) / hh).powi(2) + (lat / half).powi(2); if e < 1.0 + 0.25 * n.n3(x as f64 / 4.0, y as f64 / 4.0, z as f64 / 4.0) && y >= fy && y < CH as i32 - 1 { cv.set(x, y, z, 0); } }
            }
        } }
    }
}

/// Ce qu'on creuse : le chunk, son origine, la surface de ses colonnes.
struct Carver<'a> { v: &'a mut [Id], ox: i32, oz: i32, heights: &'a [i32] }
impl Carver<'_> {
    fn local(&self, x: i32, z: i32) -> Option<(usize, usize)> { let (lx, lz) = (x - self.ox, z - self.oz); if lx >= 0 && lz >= 0 && lx < CX as i32 && lz < CX as i32 { Some((lx as usize, lz as usize)) } else { None } }
    fn set(&mut self, x: i32, y: i32, z: i32, b: Id) { if let Some((lx, lz)) = self.local(x, z) { if y > 1 && y < CH as i32 - 1 { self.v[idx(lx, y as usize, lz)] = b; } } }
    fn get(&self, x: i32, y: i32, z: i32) -> Option<Id> { let (lx, lz) = self.local(x, z)?; if y >= 0 && y < CH as i32 { Some(self.v[idx(lx, y as usize, lz)]) } else { None } }
    fn surf(&self, lx: i32, lz: i32) -> i32 { self.heights[lx as usize + lz as usize * CX] }
}
fn shape_point(b: Id) -> bool { crate::shapes::info(b).is_some_and(|i| i.kind == crate::shapes::Kind::Drip) }

impl WorldGen {
    /// Un endroit sec où se tenir dans la salle, près d'un point visé : pieds sur la roche, trois cases libres au-dessus.
    pub fn hall_spot(&self, hl: &Hall, toward: [f64; 2]) -> Option<[f64; 3]> {
        let mut chunks: std::collections::HashMap<(i32, i32), Vec<Id>> = std::collections::HashMap::new();
        let mut at = |x: i32, y: i32, z: i32| -> Id { let k = (x.div_euclid(CX as i32), z.div_euclid(CX as i32)); let v = chunks.entry(k).or_insert_with(|| self.gen_chunk(k.0, k.1)); if y < 0 || y >= CH as i32 { 0 } else { v[idx(x.rem_euclid(CX as i32) as usize, y as usize, z.rem_euclid(CX as i32) as usize)] } };
        let ok = |b: Id| b == 0 || (hl.kind == Under::Coral && is_water(b)) || (!is_solid(b) && !is_water(b) && b != B::LAVA);
        for ring in 0..16 { for k in 0..(1 + ring * 6) { let a = k as f64 / (1 + ring * 6) as f64 * std::f64::consts::TAU; let r = ring as f64 * 1.5;
            let (x, z) = ((toward[0] + r * cos(a)).floor() as i32, (toward[1] + r * sin(a)).floor() as i32);
            for y in (hl.floor - 3).max(2)..(hl.floor + hl.h as i32).min(CH as i32 - 4) {
                let under = at(x, y - 1, z); if is_opaque(under) && !is_water(under) && under != B::LAVA && under != B::MAGMA && (0..3).all(|d| ok(at(x, y + d, z))) { return Some([x as f64 + 0.5, y as f64, z as f64 + 0.5]); } }
        } }
        None
    }
}

impl crate::session::Session {
    /// `/grotte [biome]` : se rendre dans la salle la plus proche (d'un biome donné : karst, verte, champignons, lumineuse,
    /// cristal, glace, volcan, minerale, desert, abime, corail, rocheuse) ; `/grotte mine` : à l'entrée d'une vieille mine.
    pub fn goto_hall(&mut self, arg: Option<&str>) -> String {
        let p = self.player.p; let arg = arg.map(|a| a.trim().to_lowercase()).filter(|a| !a.is_empty());
        if arg.as_deref() == Some("mine") || arg.as_deref() == Some("bouche") {
            let want_mine = arg.as_deref() == Some("mine"); let (ci, cj) = ((p[0] as i32).div_euclid(CELL), (p[2] as i32).div_euclid(CELL)); let mut best: Option<(f64, Hall, [f64; 3])> = None;
            for i in ci - 12..=ci + 12 { for j in cj - 12..=cj + 12 { if let Some(h) = self.gen.hall(i, j) { if let Some(Way::Path { from, mine, .. }) = h.way { if mine == want_mine { let d = hypot2(from[0] - p[0], from[2] - p[2]); if best.as_ref().is_none_or(|b| d < b.0) { best = Some((d, h, from)); } } } } } }
            let Some((d, h, from)) = best else { return format!("Aucune {} dans les parages", if want_mine { "mine" } else { "bouche de grotte" }) };
            let Some(Way::Path { to, .. }) = h.way else { unreachable!() }; let (ux, uz) = ((to[0] - from[0]), (to[2] - from[2])); let l = ux.hypot(uz);
            let (x, z) = (from[0] - ux / l * 4.0, from[2] - uz / l * 4.0); let y = self.gen.column(x, z).h + 1.5; self.player.p = [x, y, z]; self.player.v = [0.0; 3];
            self.player.yaw = ux.atan2(uz); self.player.pitch = -0.2;
            return format!("{} à {:.0}, {:.0} ({:.0} blocs d'ici) : elle mène à une {}", if want_mine { "Entrée de mine" } else { "Bouche de grotte" }, from[0], from[2], d, h.kind.name());
        }
        if let Some(relic) = match arg.as_deref() { Some("camp") => Some(Relic::Camp), Some("autel") => Some(Relic::Altar), Some("caveau") => Some(Relic::Vault), _ => None } {
            let Some((h, st)) = self.gen.nearest_site(p[0], p[2], 12, Some(relic)) else { return "Aucun vestige de ce genre dans les parages".into() };
            // on arrive à quelques pas, face au vestige
            let (dx, dz) = match relic { Relic::Vault => (0.5, -1.5) /* dans le caveau, face au coffre */, Relic::Altar => (0.5, 4.5), Relic::Camp => (0.5, 3.5) }; self.player.p = [st.x as f64 + dx, st.fl as f64 + 0.2, st.z as f64 + dz]; self.player.v = [0.0; 3];
            self.player.yaw = if dz > 0.0 { std::f64::consts::PI } else { 0.0 }; self.player.pitch = -0.1;
            return format!("{} dans une {}, à {}, {}, {}", match relic { Relic::Camp => "Camp de mineurs abandonné", Relic::Altar => "Autel de cristal", Relic::Vault => "Caveau" }, h.kind.name(), st.x, st.fl, st.z);
        }
        let kind = match &arg { Some(a) => match UNDERS.iter().find(|k| k.key() == a.as_str() || k.name().contains(a.as_str())) { Some(k) => Some(*k), None => return format!("Biome inconnu. Au choix : {}, mine, bouche, camp, autel, caveau", UNDERS.iter().map(|k| k.key()).collect::<Vec<_>>().join(", ")) }, None => None };
        let Some(h) = self.gen.nearest_hall_of(p[0], p[2], 14, kind) else { return "Aucune grande grotte de ce genre dans les parages".into() };
        let Some(spot) = self.gen.hall_spot(&h, [h.x, h.z]) else { return "La grotte la plus proche n'a pas d'endroit sûr où se tenir".into() };
        self.player.p = spot; self.player.v = [0.0; 3];
        format!("{} à {:.0}, {:.0} ({:.0} blocs d'ici){}{}{}", h.kind.name(), h.x, h.z, hypot2(h.x - p[0], h.z - p[2]), if h.shaft.is_some() { " — un aven s'ouvre sur le ciel" } else { "" }, if h.fall.is_some() { " — une cascade" } else { "" },
            match h.way { Some(Way::Path { mine: true, .. }) => " — une vieille mine y descend", Some(_) => " — une bouche s'ouvre en surface", None => "" })
    }
}
