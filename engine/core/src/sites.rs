//! Les lieux de surface : la tour de guet en ruine (des archers squelettes y veillent), le cimetière abandonné (les
//! fantômes y sortent la nuit) et le camp de brigands (palissade, tentes, feu). Un lieu au plus par maille de
//! `SITE_CELL` blocs, loin des villages et du point de départ, sur une terre assez plate pour le porter.
//!
//! Chaque lieu est un tampon posé à la génération (comme les épaves) : il dégage sa place, assoit son sol, puis se bâtit.
//! Son coffre a son butin (voir `village_loot`) ; ses gardiens sont lâchés quand le joueur approche (voir `Session::site_tick`).
use crate::blocks::{by_name, Id, B};
use crate::gen::{Stamp, WorldGen, SEA, SITE_CELL};
use crate::noise::{hash_float, hash_int, Rng};
use crate::shapes::{facing_of, variant};
use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)] pub enum Lieu { Tour, Cimetiere, Camp, Sanctuaire, Ruines, Grotte, Soleil, Catacombes, Forge, Glace, Horloge, Observatoire, Serre }
impl Lieu {
    pub fn name(self) -> &'static str { match self { Lieu::Tour => "Tour de guet en ruine", Lieu::Cimetiere => "Cimetière abandonné", Lieu::Camp => "Camp de brigands", Lieu::Sanctuaire => "Sanctuaire englouti", Lieu::Ruines => "Ruines englouties", Lieu::Grotte => "Grotte du Roi noyé", Lieu::Soleil => "Temple du Soleil", Lieu::Catacombes => "Catacombes des Rois", Lieu::Forge => "Forge sous la lave", Lieu::Glace => "Temple de glace", Lieu::Horloge => "Tour de l'Horloge", Lieu::Observatoire => "Observatoire céleste", Lieu::Serre => "Serre empoisonnée" } }
    pub fn key(self) -> &'static str { match self { Lieu::Tour => "tour", Lieu::Cimetiere => "cimetiere", Lieu::Camp => "brigands", Lieu::Sanctuaire => "sanctuaire", Lieu::Ruines => "ruines", Lieu::Grotte => "grotte", Lieu::Soleil => "soleil", Lieu::Catacombes => "catacombes", Lieu::Forge => "forge", Lieu::Glace => "glace", Lieu::Horloge => "horloge", Lieu::Observatoire => "observatoire", Lieu::Serre => "serre" } }
    /// Demi-emprise (le long de x, le long de z) et dénivelé toléré.
    /// (Le sanctuaire est enterré : on vérifie seulement que le terrain le couvre, voir `place_fits`.)
    fn fit(self) -> (i32, i32, f64) { match self { Lieu::Tour => (5, 5, 4.0), Lieu::Cimetiere => (12, 11, 3.6), Lieu::Camp => (11, 11, 4.0), Lieu::Sanctuaire => (24, 28, 9.0), Lieu::Ruines => (29, 29, 9.0), Lieu::Grotte => (31, 35, 10.0), Lieu::Soleil => (19, 24, 8.0), Lieu::Catacombes => (12, 34, 9.0), Lieu::Forge => (10, 20, 9.0), Lieu::Glace => (22, 30, 12.0), Lieu::Horloge => (12, 17, 6.0), Lieu::Observatoire => (13, 21, 6.0), Lieu::Serre => (14, 21, 6.0) } }
}
pub const LIEUX: [Lieu; 13] = [Lieu::Tour, Lieu::Cimetiere, Lieu::Camp, Lieu::Sanctuaire, Lieu::Ruines, Lieu::Grotte, Lieu::Soleil, Lieu::Catacombes, Lieu::Forge, Lieu::Glace, Lieu::Horloge, Lieu::Observatoire, Lieu::Serre];
/// Un lieu décidé : son centre, le niveau de son sol (le bloc sur lequel on marche), sa maille.
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Place { pub kind: Lieu, pub x: i32, pub y: i32, pub z: i32, pub cell: (i32, i32), /** quarts de tour : l'entrée regarde -z, -x, +z ou +x */ pub rot: u8 }
/// Un décalage dessiné « droit » (l'entrée vers -z), tourné de `q` quarts de tour : +z devient +x.
pub fn spin_xz(u: i32, w: i32, q: u8) -> (i32, i32) { let (mut u, mut w) = (u, w); for _ in 0..q % 4 { (u, w) = (w, -u); } (u, w) }
fn spin_f(u: f64, w: f64, q: u8) -> (f64, f64) { let (mut u, mut w) = (u, w); for _ in 0..q % 4 { (u, w) = (w, -u); } (u, w) }
/// Le même bloc, tourné : escaliers, pentes, échelles, meubles, faîtages, poutres et troncs couchés.
pub fn spin_block(id: Id, q: u8) -> Id {
    use crate::shapes::{info, Kind}; let q = q % 4; if q == 0 { return id; }
    match info(id) {
        Some(i) => match i.kind { Kind::Ridge => variant(i.base, (i.facing + q) % 2, i.top, i.open, i.upper), Kind::Post => if q % 2 == 1 && i.facing > 0 { variant(i.base, 3 - i.facing.min(2), i.top, i.open, i.upper) } else { id }, _ => variant(i.base, (i.facing + q) % 4, i.top, i.open, i.upper) },
        None => { if q % 2 == 0 { return id; } let n = crate::blocks::BLOCKS[id as usize].name; let other = if let Some(b) = n.strip_suffix("_x") { format!("{b}_z") } else if let Some(b) = n.strip_suffix("_z") { format!("{b}_x") } else { return id }; by_name(&other).unwrap_or(id) }
    }
}
impl Place {
    /// Le coffre du lieu.
    pub fn chest(&self) -> [i32; 3] { let (u, dy, w) = match self.kind { Lieu::Tour => (-3, TOP + 1, 3), Lieu::Cimetiere => (0, -4, 3), Lieu::Camp => (0, 1, 7), Lieu::Sanctuaire => (crate::sanctuaire::CHEST[0], crate::sanctuaire::CHEST[1] - crate::sanctuaire::DEPTH, crate::sanctuaire::CHEST[2]),
            Lieu::Ruines => { let c = self.ruins().rooms.last().unwrap().at([0, 2, 2]); (c[0], c[1] - crate::sanctuaire::DEPTH, c[2]) }
            Lieu::Grotte => (crate::grotte::CHEST[0], crate::grotte::CHEST[1] - crate::grotte::DEPTH, crate::grotte::CHEST[2]),
            Lieu::Soleil => (crate::soleil::CHEST[0], crate::soleil::CHEST[1], crate::soleil::CHEST[2]),
            Lieu::Catacombes => (crate::catacombes::CHEST[0], crate::catacombes::CHEST[1] - crate::catacombes::DEPTH, crate::catacombes::CHEST[2]),
            Lieu::Forge => (crate::forge::CHEST[0], crate::forge::CHEST[1] - crate::forge::DEPTH, crate::forge::CHEST[2]),
            Lieu::Glace => (crate::glace::CHEST[0], crate::glace::CHEST[1] + 1, crate::glace::CHEST[2]),
            Lieu::Horloge => (crate::horloge::CHEST[0], crate::horloge::CHEST[1] + 1, crate::horloge::CHEST[2]),
            Lieu::Observatoire => (crate::observatoire::CHEST[0], crate::observatoire::CHEST[1] + 1, crate::observatoire::CHEST[2]),
            Lieu::Serre => (crate::serre::CHEST[0], crate::serre::CHEST[1] + 1, crate::serre::CHEST[2]) }; let (u, w) = spin_xz(u, w, self.rot); [self.x + u, self.y + dy, self.z + w] }
    /// Où lâcher les gardiens : espèce, position.
    pub fn guards(&self) -> Vec<(&'static str, [f64; 3])> {
        let at = |dx: f64, dy: f64, dz: f64| { let (dx, dz) = spin_f(dx, dz, self.rot); [self.x as f64 + 0.5 + dx, self.y as f64 + 1.05 + dy, self.z as f64 + 0.5 + dz] };
        match self.kind {
            // le capitaine sur le chemin de ronde ; un guetteur à chaque étage ; la garde au pied de la porte ; un rôdeur à la cave
            Lieu::Tour => vec![("capitaine", at(1.0, TOP as f64, 1.0)), ("archer", at(-1.0, TOP as f64, 2.0)), ("archer", at(0.0, 8.0, 1.0)), ("archer", at(1.0, 4.0, -1.0)), ("rodeur", at(-1.0, 0.0, -2.0)),
                ("archer", at(0.0, 0.0, -5.0)), ("rodeur", at(-2.0, 0.0, -5.0)), ("rodeur", at(2.0, 0.0, -5.0)), ("rodeur", at(0.0, -5.0, 1.0))],
            // le gardien de la crypte ; des archers au porche, dans l'allée, sur le parvis ; un rôdeur dans la nef, un autre en bas ; cinq fantômes la nuit
            Lieu::Cimetiere => vec![("gardien", at(0.0, -5.0, 7.0)), ("rodeur", at(0.0, -5.0, 4.0)), ("archer", at(2.0, 0.0, -6.0)), ("archer", at(-2.0, 0.0, -8.0)), ("archer", at(0.0, 0.0, 1.0)), ("rodeur", at(0.0, 0.0, 5.0)),
                ("fantome", at(-6.0, 0.0, -3.0)), ("fantome", at(6.0, 0.0, -6.0)), ("fantome", at(0.0, 0.0, -1.0)), ("fantome", at(-8.0, 0.0, 4.0)), ("fantome", at(8.0, 0.0, 2.0))],
            // le chef dans sa tente ; deux hommes au portail, trois au feu, un près de la cage, le guetteur sur sa tour de bois
            Lieu::Camp => vec![("brigand", at(0.0, 0.0, 5.0)), ("brigand", at(-1.0, 0.0, -8.0)), ("brigand", at(1.0, 0.0, -8.0)), ("brigand", at(-2.0, 0.0, -1.5)), ("brigand", at(2.5, 0.0, 2.0)), ("brigand", at(-1.0, 0.0, 3.0)),
                ("brigand", at(-4.0, 0.0, 3.0)), ("brigand", at(6.0, 5.0, -6.0))],
            // des noyés montent la garde dans la salle de l'eau noire
            Lieu::Sanctuaire => { let d = -(crate::sanctuaire::DEPTH as f64); vec![("noye", at(0.0, d, 11.0)), ("noye", at(-3.0, d, 7.0)), ("noye", at(3.0, d, 7.0))] }
            // deux rôdeurs devant le portail du temple
            Lieu::Soleil => vec![("rodeur", at(-7.0, 0.0, -22.0)), ("rodeur", at(7.0, 0.0, -22.0))],
            // deux archers squelettes sur l'allée des rois, un rôdeur dans le vestibule ; des fantômes entre les tombes, la nuit
            Lieu::Catacombes => { let d = -(crate::catacombes::DEPTH as f64); vec![("archer", at(-1.0, 0.0, -20.0)), ("archer", at(1.0, 0.0, -14.0)), ("rodeur", at(0.0, d, 15.0)), ("fantome", at(-8.0, 0.0, -22.0)), ("fantome", at(8.0, 0.0, -16.0))] }
            // deux brigands dans la cour de l'atelier, un rôdeur au bord du fleuve de lave
            Lieu::Forge => { let d = -(crate::forge::DEPTH as f64); vec![("brigand", at(-3.0, 0.0, -28.0)), ("brigand", at(3.0, 0.0, -27.0)), ("rodeur", at(-6.0, d, 4.0))] }
            // deux rôdeurs devant le temple de glace, un dans la nef ; des fantômes entre les obélisques, la nuit
            // deux brigands dans la cour de la tour de l'horloge, un rôdeur devant la porte
            // deux rôdeurs dans la cour de l'observatoire
            // deux rôdeurs dans le jardin en friche de la serre
            Lieu::Serre => vec![("rodeur", at(-11.0, 0.0, -19.0)), ("rodeur", at(11.0, 0.0, 18.0))],
            Lieu::Observatoire => vec![("rodeur", at(-9.0, 0.0, -18.0)), ("rodeur", at(9.0, 0.0, -18.0))],
            Lieu::Horloge => vec![("brigand", at(-8.0, 0.0, -13.0)), ("brigand", at(8.0, 0.0, -13.0)), ("rodeur", at(0.0, 0.0, -14.0))],
            Lieu::Glace => vec![("rodeur", at(-6.0, 0.0, -14.0)), ("rodeur", at(6.0, 0.0, -14.0)), ("rodeur", at(0.0, 1.0, -3.0)), ("fantome", at(-9.0, 0.0, -20.0)), ("fantome", at(9.0, 0.0, -20.0))],
            // des noyés rôdent sur les rives du lac
            Lieu::Grotte => { let d = -(crate::grotte::DEPTH as f64); vec![("noye", at(-8.0, d, 3.0)), ("noye", at(8.0, d, 3.0)), ("noye", at(-14.0, d, 30.0))] }
            // deux rôdeurs dans la salle d'entrée des ruines
            Lieu::Ruines => { let e = self.ruins().rooms[0]; [[3, 0, -3], [-3, 0, 3]].iter().map(|l| { let c = e.at(*l); ("rodeur", [self.x as f64 + c[0] as f64 + 0.5, (self.y - crate::sanctuaire::DEPTH) as f64 + 1.05, self.z as f64 + c[2] as f64 + 0.5]) }).collect() }
        }
    }
    /// Le premier gardien de la liste est le chef du lieu : un vétéran, deux fois plus endurant.
    pub const CHIEF: usize = 0;
    /// Où se tenir pour en voir l'intérieur : le seuil de la tour, la nef de la chapelle, le portail du camp.
    /// Le plan des ruines englouties de ce lieu.
    pub fn ruins(&self) -> crate::donjons::Layout { crate::donjons::layout(crate::donjons::seed_at(self.x, self.z)) }
    pub fn inside(&self) -> ([f64; 3], f64) { if self.kind == Lieu::Grotte { return ([self.x as f64 - 3.5, (self.y - crate::grotte::DEPTH) as f64 + 1.1, self.z as f64 + 0.5], 0.0); } if self.kind == Lieu::Ruines { let s = self.ruins().rooms[0].spot(); return ([self.x as f64 + s[0] as f64 + 0.5, (self.y - crate::sanctuaire::DEPTH) as f64 + 1.1, self.z as f64 + s[2] as f64 + 0.5], 0.0); } let (dz, dy) = match self.kind { Lieu::Tour => (-3.0, 0.0), Lieu::Cimetiere => (3.0, 0.0), Lieu::Camp => (-7.0, 0.0), Lieu::Sanctuaire | Lieu::Ruines | Lieu::Grotte => (4.0, -(crate::sanctuaire::DEPTH as f64)), Lieu::Soleil => (-6.0, 0.0), Lieu::Catacombes => (16.0, -(crate::catacombes::DEPTH as f64)), Lieu::Forge => (2.0, -(crate::forge::DEPTH as f64)), Lieu::Glace => (-3.0, 1.0), Lieu::Horloge => (-7.0, 1.0), Lieu::Observatoire => (-13.0, 1.0), Lieu::Serre => (-15.0, 1.0) }; let (dx, dz) = spin_f(0.0, dz, self.rot); ([self.x as f64 + 0.5 + dx, self.y as f64 + 1.1 + dy, self.z as f64 + 0.5 + dz], self.rot as f64 * std::f64::consts::FRAC_PI_2) }
    /// Où se tenir pour le regarder (commande, captures) : position et cap.
    pub fn view(&self) -> ([f64; 3], f64) { if self.kind == Lieu::Grotte { return ([self.x as f64 + 0.5, self.y as f64 + 1.2, self.z as f64 - 15.5], 0.0); } if self.kind == Lieu::Ruines { let s = self.ruins().rooms[0].at([-5, 0, -11]); let t = self.ruins().rooms[0].at([-5, 0, -5]); return ([self.x as f64 + s[0] as f64 + 0.5, self.y as f64 + 1.2, self.z as f64 + s[2] as f64 + 0.5], ((t[0] - s[0]) as f64).atan2((t[2] - s[2]) as f64)); } let d = match self.kind { Lieu::Tour => 15.5, Lieu::Cimetiere => 21.0, Lieu::Camp => 21.0, Lieu::Sanctuaire => 24.0, Lieu::Ruines => 20.0, Lieu::Grotte => 16.0, Lieu::Soleil => 27.0, Lieu::Catacombes => 28.0, Lieu::Forge => 35.0, Lieu::Glace => 26.0, Lieu::Horloge => 30.0, Lieu::Observatoire => 32.0, Lieu::Serre => 32.0 }; /* dans la clairière : on n'arrive pas dans un tronc */ let (dx, dz) = spin_f(0.0, -d, self.rot); ([self.x as f64 + 0.5 + dx, self.y as f64 + 1.2, self.z as f64 + 0.5 + dz], self.rot as f64 * std::f64::consts::FRAC_PI_2) }
}
/// Hauteur du dernier plancher de la tour au-dessus du sol.
const TOP: i32 = 12;

/// Le chantier d'un lieu : ce qu'on pose (ou dégage : -1), case par case.
pub(crate) struct Plan { pub(crate) m: BTreeMap<(i32, i32, i32), i32>, pub(crate) ids: std::collections::HashMap<&'static str, Id> }
impl Plan {
    pub(crate) fn new() -> Plan { Plan { m: BTreeMap::new(), ids: Default::default() } }
    pub(crate) fn put(&mut self, x: i32, y: i32, z: i32, b: Id) { self.m.insert((x, y, z), b as i32); }
    pub(crate) fn air(&mut self, x: i32, y: i32, z: i32) { self.m.insert((x, y, z), -1); }
    /// La peau du terrain, reposée à ce niveau.
    fn skin(&mut self, x: i32, y: i32, z: i32) { self.m.insert((x, y, z), -2); }
    pub(crate) fn id(&mut self, n: &'static str) -> Id { *self.ids.entry(n).or_insert_with(|| by_name(n).unwrap_or_else(|| panic!("bloc inconnu : {n}"))) }
    pub(crate) fn n(&mut self, x: i32, y: i32, z: i32, n: &'static str) { let b = self.id(n); self.put(x, y, z, b); }
    /// Un bloc orienté : `(dx, dz)` est le côté haut d'une pente, le mur d'une échelle, le dos d'un meuble.
    pub(crate) fn turn(&mut self, x: i32, y: i32, z: i32, n: &'static str, dx: i32, dz: i32) { let b = self.id(n); self.put(x, y, z, variant(b, facing_of(dx as f64, dz as f64), false, false, false)); }
}

impl WorldGen {
    /// Le lieu de cette maille, s'il y en a un (décidé une fois, retenu).
    pub fn place(&self, gx: i32, gz: i32) -> Option<Place> {
        if let Some(p) = self.places.borrow().get(&(gx, gz)) { return *p; }
        let p = self.place_find(gx, gz); let mut m = self.places.borrow_mut(); if m.len() > 4000 { m.clear(); } m.insert((gx, gz), p); p
    }
    /// On tire d'abord le genre du lieu, puis on lui cherche une place dans la maille : plusieurs essais, sinon le genre suivant.
    fn place_find(&self, gx: i32, gz: i32) -> Option<Place> {
        if self.flat { return None; } let s = self.seed; if hash_float(gx, gz, s, 171) > 0.6 { return None; }
        let first = (hash_float(gx, gz, s, 174) * LIEUX.len() as f64) as usize; let mut r = Rng::new(hash_int(gx, gz, s, 172));
        for k in 0..LIEUX.len() { let kind = LIEUX[(first + k) % LIEUX.len()]; if k > 0 && kind == Lieu::Tour && hash_float(gx, gz, s, 177) > 0.3 { continue; } /* la tour ne bouche pas tous les trous */ for _ in 0..if kind == Lieu::Tour { 8 } else { 70 } {
            let (x, z) = { let (a, b) = (r.range(0.15, 0.85), r.range(0.15, 0.85)); /* les plus grands lieux (catacombes, forge) restent au cœur de leur maille : loin de ceux d'à côté */ let k = |t: f64| if matches!(kind, Lieu::Catacombes | Lieu::Forge | Lieu::Glace) { 0.5 + (t - 0.5) * 0.257 } else { t }; (((gx as f64 + k(a)) * SITE_CELL as f64).floor() as i32, ((gz as f64 + k(b)) * SITE_CELL as f64).floor() as i32) };
            if ((x + 38) as f64).hypot(z as f64) < 90.0 { continue; } // pas sur le pas de la porte
            let rot = (r.next() * 4.0) as u8 % 4; let rot = if matches!(kind, Lieu::Sanctuaire | Lieu::Ruines | Lieu::Grotte | Lieu::Soleil | Lieu::Catacombes | Lieu::Forge | Lieu::Glace | Lieu::Horloge | Lieu::Observatoire | Lieu::Serre) { 0 } else { rot }; /* le sanctuaire est bâti droit */ if let Some(y) = self.place_fits(kind, x, z, rot) { if !self.villages_near(x as f64, z as f64, 120.0).is_empty() { continue; } return Some(Place { kind, x, y, z, cell: (gx, gz), rot }); } } }
        None
    }
    /// Le terrain porte-t-il ce lieu ici ? Rend le niveau du sol.
    fn place_fits(&self, kind: Lieu, x: i32, z: i32, rot: u8) -> Option<i32> {
        let (hx, hz, tol) = kind.fit(); let (hx, hz) = if rot % 2 == 1 { (hz, hx) } else { (hx, hz) }; let c0 = self.column(x as f64, z as f64);
        match kind { Lieu::Camp => if c0.forest < 0.2 && c0.mix.of(crate::biome::Biome::Steppe) < 0.5 { return None; }, Lieu::Cimetiere => if c0.forest > 0.75 { return None; }, Lieu::Tour | Lieu::Sanctuaire | Lieu::Ruines | Lieu::Grotte | Lieu::Soleil | Lieu::Catacombes | Lieu::Forge | Lieu::Glace | Lieu::Horloge | Lieu::Observatoire | Lieu::Serre => {} }
        // les catacombes : sous terre, seule la mer les arrête (leurs murs les gardent du reste) ; le niveau est le sol le plus bas,
        // pour que tout reste sous terre ; en surface, l'allée et le mausolée veulent une terre sèche, sans falaise, assez plate
        // le temple de glace : une grande emprise ; on ne veut ni mer, ni rivière, ni route, et une pente modérée (le terrain est
        // ensuite aplani autour de lui, falaises comprises)
        // la tour de l'horloge : haute de près de soixante blocs ; elle ne doit pas crever le ciel du monde
        if matches!(kind, Lieu::Glace | Lieu::Horloge | Lieu::Observatoire | Lieu::Serre) { let (ax, az0, az1, top) = match kind { Lieu::Glace => (24, -30, 34, 118.0), Lieu::Horloge => (12, -17, 13, 66.0), Lieu::Serre => (14, -21, 20, 100.0), _ => (13, -21, 16, 104.0) }; let (mut lo, mut hi, mut sum, mut n) = (f64::MAX, f64::MIN, 0.0, 0.0); let mut dz = az0; while dz <= az1 { let mut dx = -ax; while dx <= ax {
                let c = self.column((x + dx) as f64, (z + dz) as f64); if c.mix.sea > 0.02 || c.wet || c.h < SEA + 1.5 || c.h > top || c.path > 0.3 { return None; } lo = lo.min(c.h); hi = hi.max(c.h); sum += c.h; n += 1.0; dx += 3; } dz += 3; }
            return (hi - lo <= tol).then(|| (sum / n).floor() as i32); }
        let under = match kind { Lieu::Catacombes => Some((36, -36, 38, 14, -34, 10)), Lieu::Forge => Some((20, -34, 38, 12, -34, -12)), _ => None };
        if let Some((ax, az0, az1, kx, kz0, kz1)) = under { let mut lo = f64::MAX; let (mut a, mut b) = (f64::MAX, f64::MIN); let mut dz = az0; while dz <= az1 { let mut dx = -ax; while dx <= ax {
                let c = self.column((x + dx) as f64, (z + dz) as f64); if c.mix.sea > 0.02 || c.h < SEA + 1.5 || c.h > 118.0 { return None; } lo = lo.min(c.h);
                if dx.abs() <= kx && (kz0..=kz1).contains(&dz) { if c.wet || c.cliff > 0.5 || c.path > 0.3 { return None; } a = a.min(c.h); b = b.max(c.h); } dx += 4; } dz += 4; }
            return (b - a <= tol && a - lo <= tol).then(|| lo.floor() as i32); }
        // le sanctuaire : ses salles s'étendent de -24 à +24 en x, de -18 à +28 en z ; le sol le plus bas sert de niveau, pour qu'elles restent sous terre
        if matches!(kind, Lieu::Sanctuaire | Lieu::Ruines | Lieu::Grotte | Lieu::Catacombes) { let (z0, z1, xr) = match kind { Lieu::Ruines => (-29, 29, 29), Lieu::Grotte => (-12, 36, 32), Lieu::Catacombes => (-36, 38, 36), _ => (-18, 28, 24) }; let (mut lo, mut hi) = (f64::MAX, f64::MIN); let mut dz = z0; while dz <= z1 { let mut dx = -xr; while dx <= xr {
                let c = self.column((x + dx) as f64, (z + dz) as f64); if c.mix.sea > 0.02 || c.wet || c.h < SEA + 1.5 || c.h > 118.0 || c.cliff > 0.5 || (kind == Lieu::Sanctuaire && dz < -6 && c.path > 0.3) { return None; } lo = lo.min(c.h); hi = hi.max(c.h); dx += 4; } dz += 4; }
            return (hi - lo <= tol).then(|| lo.floor() as i32); }
        let (mut lo, mut hi, mut sum, mut n) = (f64::MAX, f64::MIN, 0.0, 0.0);
        let mut dz = -hz; while dz <= hz { let mut dx = -hx; while dx <= hx {
            let c = self.column((x + dx) as f64, (z + dz) as f64); if c.mix.sea > 0.02 || c.wet || c.h < SEA + 1.5 || c.h > 118.0 || c.path > 0.3 || c.cliff > 0.5 { return None; }
            lo = lo.min(c.h); hi = hi.max(c.h); sum += c.h; n += 1.0; dx += 2; } dz += 2; }
        (hi - lo <= tol).then(|| (sum / n).floor() as i32)
    }
    /// Les lieux à moins de `r` blocs, du plus proche au plus lointain.
    pub fn places_near(&self, x: f64, z: f64, r: f64) -> Vec<Place> {
        let f = |v: f64| (v / SITE_CELL as f64).floor() as i32; let mut o = Vec::new();
        for gx in f(x - r)..=f(x + r) { for gz in f(z - r)..=f(z + r) { if let Some(p) = self.place(gx, gz) { if ((p.x as f64 - x).hypot(p.z as f64 - z)) <= r { o.push(p); } } } }
        o.sort_by(|a, b| (a.x as f64 - x).hypot(a.z as f64 - z).total_cmp(&(b.x as f64 - x).hypot(b.z as f64 - z))); o
    }
    /// Un arbre qui pousserait ici serait-il coupé par la place d'un lieu ? On ne le fait pas pousser : sinon son tronc
    /// tombe sous la pioche du chantier et sa couronne, hors de la place, reste à flotter. Une clairière de quelques blocs en plus.
    pub(crate) fn site_claims(&self, x: f64, z: f64) -> bool {
        if self.flat { return false; } let f = |v: f64| (v / SITE_CELL as f64).floor() as i32; let (gx, gz) = (f(x), f(z));
        for a in gx - 1..=gx + 1 { for b in gz - 1..=gz + 1 { if let Some(p) = self.place(a, b) { if p.kind == Lieu::Sanctuaire { if (x - p.x as f64).abs() <= 16.0 && z - p.z as f64 >= -29.0 && z - p.z as f64 <= 3.0 { return true; } continue; }
                if p.kind == Lieu::Grotte { if (x - p.x as f64).abs() <= 17.0 && (z - p.z as f64).abs() <= 17.0 { return true; } continue; }
                if p.kind == Lieu::Ruines { let e = p.ruins().rooms[0].at([-5, 0, -5]); if (x - (p.x + e[0]) as f64).abs() <= 9.0 && (z - (p.z + e[2]) as f64).abs() <= 9.0 { return true; } continue; }
            let (hx, hz, _) = p.kind.fit(); let r = hx.max(hz) as f64 + 6.0 + 5.0; if (x - p.x as f64).abs() <= r && (z - p.z as f64).abs() <= r { return true; } } } }
        false
    }
    /// Le coffre d'un lieu de surface est-il ici ?
    pub fn place_chest(&self, p: [i32; 3]) -> Option<Lieu> {
        let (gx, gz) = (p[0].div_euclid(SITE_CELL), p[2].div_euclid(SITE_CELL)); self.place(gx, gz).filter(|pl| pl.chest() == p).map(|pl| pl.kind)
    }
    /// La colonne du monde qui correspond à une case du plan (dessiné droit autour du centre du lieu en chantier).
    fn site_xz(&self, x: i32, z: i32) -> (i32, i32) { let (cx, cz, q) = self.site_rot.get(); let (u, w) = spin_xz(x - cx, z - cz, q); (cx + u, cz + w) }
    /// Dégage la place et assoit le sol : comble les creux, rase les bosses, enlève les arbres jusqu'à la cime ; autour, sur
    /// six blocs, le terrain est raccordé en pente douce et garde sa peau (herbe, sable, neige).
    fn pad(&self, p: &mut Plan, pl: &Place, inside: impl Fn(i32, i32) -> bool, hx: i32, hz: i32, up: i32, floor: Option<Id>) {
        const SKIRT: i32 = 6; let up = up.max(48); // jusqu'au-dessus des plus grands arbres : pas de feuillage qui flotte sur un tronc coupé
        // à combien de cases est-on du bord de la place ? (0 : dedans)
        let core = |dx: i32, dz: i32| dx.abs() <= hx && dz.abs() <= hz && inside(dx, dz);
        let out = |dx: i32, dz: i32| -> i32 { if core(dx, dz) { return 0; } for t in 1..=SKIRT { for a in -t..=t { for b in -t..=t { if a.abs().max(b.abs()) == t && core(dx + a, dz + b) { return t; } } } } SKIRT + 1 };
        for dz in -hz - SKIRT..=hz + SKIRT { for dx in -hx - SKIRT..=hx + SKIRT {
            let (x, z) = (pl.x + dx, pl.z + dz); let h = self.gy(x, z); let t = out(dx, dz); if t > SKIRT { continue; }
            let level = if t == 0 { pl.y } else { pl.y + ((h - pl.y) as f64 * t as f64 / (SKIRT + 1) as f64).round() as i32 };
            for y in h + 1..level { p.put(x, y, z, B::DIRT); } for y in level + 1..=h.max(level) { p.air(x, y, z); }
            if t == 0 { match floor { Some(f) => p.put(x, level, z, f), None => p.skin(x, level, z) } } else if level != h { p.skin(x, level, z); }
            for y in h.max(level) + 1..=pl.y + up { p.air(x, y, z); }
        } }
    }
    /// Ce qu'un camp de brigands ajoute en grandissant, cran par cran (posé tourné comme le camp) :
    /// 2 — une troisième tente, un râtelier d'armes près du feu, des caisses et des sacs de butin ;
    /// 3 — une forge (enclume, meule), des torches tout autour de la palissade, une passerelle de guet sur le portail ;
    /// 4 — un second feu et ses bancs, deux hautes bannières rouges, d'autres râteliers ;
    /// 5 — le grand étendard du chef derrière sa tente, des ossements au pied de la cage et devant le portail.
    pub fn camp_layer(&self, pl: &Place, level: u32) -> Vec<(i32, i32, i32, Id)> {
        let (x0, y0, z0) = (pl.x, pl.y, pl.z); let mut p = Plan { m: BTreeMap::new(), ids: Default::default() };
        match level {
            2 => { for w in -5..=-2 { p.turn(x0 - 7, y0 + 1, z0 + w, "thatch_slope", 1, 0); p.turn(x0 - 4, y0 + 1, z0 + w, "thatch_slope", -1, 0); p.turn(x0 - 6, y0 + 2, z0 + w, "thatch_slope", 1, 0); p.turn(x0 - 5, y0 + 2, z0 + w, "thatch_slope", -1, 0); }
                p.n(x0 - 6, y0 + 1, z0 - 4, "bedroll"); p.n(x0 - 5, y0 + 1, z0 - 3, "bedroll"); p.turn(x0 + 2, y0 + 1, z0 + 2, "weapon_rack", -1, 0);
                for (u, w, n) in [(7, 6, "crate"), (7, 5, "sack"), (3, 6, "sack"), (-3, 7, "crate")] { p.n(x0 + u, y0 + 1, z0 + w, n); } }
            3 => { p.n(x0 + 3, y0 + 1, z0 - 3, "anvil"); p.n(x0 + 4, y0 + 1, z0 - 3, "grindstone"); p.n(x0 + 3, y0 + 1, z0 - 4, "barrel");
                for k in 0..10 { let a = k as f64 / 10.0 * std::f64::consts::TAU + 0.3; let (u, w) = ((a.cos() * 9.0).round() as i32, (a.sin() * 9.0).round() as i32); if w < -7 && u.abs() <= 3 { continue; } p.n(x0 + u, y0 + 1, z0 + w, "fence"); p.n(x0 + u, y0 + 2, z0 + w, "torch"); }
                for u in -2..=2 { p.n(x0 + u, y0 + 6, z0 - 9, "planks"); p.n(x0 + u, y0 + 7, z0 - 9, if u.abs() == 2 { "oak_post" } else { "fence" }); } p.n(x0 - 2, y0 + 8, z0 - 9, "torch"); p.n(x0 + 2, y0 + 8, z0 - 9, "torch"); }
            4 => { p.n(x0 - 5, y0 + 1, z0 - 7, "campfire"); p.n(x0 - 7, y0 + 1, z0 - 7, "dead_log_z"); p.n(x0 - 3, y0 + 1, z0 - 7, "dead_log_z");
                for (u, w) in [(-3, -8), (3, -6)] { for y in 1..=7 { p.n(x0 + u, y0 + y, z0 + w, "oak_post"); } p.turn(x0 + u, y0 + 6, z0 + w - 1, "banner_red", 0, 1); p.turn(x0 + u, y0 + 5, z0 + w - 1, "banner_red", 0, 1); }
                p.turn(x0 - 2, y0 + 1, z0 + 3, "weapon_rack", 0, -1); p.turn(x0 + 8, y0 + 1, z0 + 3, "weapon_rack", -1, 0); }
            5 => { for y in 1..=10 { p.n(x0, y0 + y, z0 + 9, "oak_post"); } for y in [8, 9] { p.turn(x0, y0 + y, z0 + 8, "banner_red", 0, -1); p.turn(x0 - 1, y0 + y, z0 + 9, "banner_gold", -1, 0); p.turn(x0 + 1, y0 + y, z0 + 9, "banner_gold", 1, 0); }
                for (u, w) in [(-4, 4), (-4, 6), (-5, 3), (-3, -9), (3, -9), (-1, -8), (1, -8)] { p.n(x0 + u, y0 + 1, z0 + w, "bone"); } }
            _ => {}
        }
        p.m.into_iter().map(|((a, b, c), id)| { let (u, w) = spin_xz(a - pl.x, c - pl.z, pl.rot); (pl.x + u, b, pl.z + w, if id > 0 { spin_block(id as Id, pl.rot) } else { 0 }) }).collect()
    }
    /// Le feu du camp (éteint quand il est dispersé).
    pub fn camp_fire(&self, pl: &Place) -> [i32; 3] { [pl.x, pl.y + 1, pl.z] }
    pub(crate) fn site_stamp(&self, gx: i32, gz: i32) -> Option<Stamp> {
        let pl = self.place(gx, gz)?; self.site_rot.set((pl.x, pl.z, pl.rot)); let mut r = Rng::new(hash_int(gx, gz, self.seed, 175)); let mut p = Plan { m: BTreeMap::new(), ids: Default::default() };
        match pl.kind { Lieu::Tour => self.build_tower(&mut p, &pl, &mut r), Lieu::Cimetiere => self.build_cemetery(&mut p, &pl, &mut r), Lieu::Camp => self.build_camp(&mut p, &pl, &mut r), Lieu::Sanctuaire => { self.pad(&mut p, &pl, |_, dz| (-18..=-8).contains(&dz), 5, 18, 8, None); crate::sanctuaire::carve(&mut p, [pl.x, pl.y, pl.z]); }
            Lieu::Grotte => crate::grotte::carve(&mut p, [pl.x, pl.y, pl.z]),
            Lieu::Soleil => { self.pad(&mut p, &pl, |_, _| true, 19, 24, 20, None); crate::soleil::carve(&mut p, [pl.x, pl.y, pl.z]); }
            Lieu::Serre => { self.pad(&mut p, &pl, |_, _| true, 14, 21, 26, None); crate::serre::carve(&mut p, [pl.x, pl.y, pl.z]); }
            Lieu::Observatoire => { self.pad(&mut p, &pl, |_, _| true, 13, 21, 24, None); crate::observatoire::carve(&mut p, [pl.x, pl.y, pl.z]); }
            Lieu::Horloge => { self.pad(&mut p, &pl, |_, _| true, 12, 17, 60, None); crate::horloge::carve(&mut p, [pl.x, pl.y, pl.z]); }
            Lieu::Glace => { self.pad(&mut p, &pl, |_, dz| dz >= -30, 24, 32, 30, Some(B::SNOW)); crate::glace::carve(&mut p, [pl.x, pl.y, pl.z]); }
            Lieu::Forge => { self.pad(&mut p, &pl, |_, dz| dz <= -10, 12, 32, 16, None); crate::forge::carve(&mut p, [pl.x, pl.y, pl.z]); }
            Lieu::Catacombes => { self.pad(&mut p, &pl, |_, dz| dz <= 9, 12, 32, 14, None); crate::catacombes::carve(&mut p, [pl.x, pl.y, pl.z]); }
            Lieu::Ruines => { let g = |x: i32, z: i32| self.column(x as f64, z as f64).h.floor() as i32; let _ = crate::donjons::carve(&mut p, [pl.x, pl.y, pl.z], &pl.ruins(), "", &g); } }
        // le plan est dessiné droit : on le tourne en le posant (les blocs orientés avec lui) ; rangé par colonne, de bas en haut
        let mut data: Vec<[i32; 4]> = p.m.into_iter().map(|((a, b, c), id)| { let (u, w) = spin_xz(a - pl.x, c - pl.z, pl.rot); [pl.x + u, b, pl.z + w, if id > 0 { spin_block(id as Id, pl.rot) as i32 } else { id }] }).collect(); data.sort_by_key(|d| (d[0], d[2], d[1]));
        Some(Stamp { data, force: true, ground: false })
    }
    /// Le sol naturel d'une colonne (le bloc sur lequel on marche).
    fn gy(&self, x: i32, z: i32) -> i32 { let (x, z) = self.site_xz(x, z); self.column(x as f64, z as f64).h.floor() as i32 }
    /// La tour de guet : neuf blocs de côté sur contreforts, une cave à cachot, trois étages meublés reliés par des volées
    /// d'escalier le long des murs, un chemin de ronde crénelé avec son brasero et son hourd de bois ; un angle s'est
    /// effondré, et une courtine en ruine s'en va vers le nord, percée d'une arche.
    fn build_tower(&self, p: &mut Plan, pl: &Place, r: &mut Rng) {
        let (x0, y0, z0) = (pl.x, pl.y, pl.z); self.pad(p, pl, |_, _| true, 5, 5, TOP + 6, None);
        let walls = ["stone_bricks", "stone_bricks", "stone_bricks", "stone_bricks", "cobblestone", "cobblestone", "mossy_stone"];
        let (bx, bz) = (-4, -4); // l'angle effondré
        let reach = |u: i32, w: i32, r: &mut Rng| -> i32 { let d = ((u - bx) as f64).hypot((w - bz) as f64); if d > 5.2 { 99 } else { (7.6 + d * 1.25 + r.range(-0.7, 0.7)).round() as i32 } };
        // la cave : creusée sous la tour, murée
        for w in -4..=4i32 { for u in -4..=4i32 { let ring = u.abs() == 4 || w.abs() == 4; p.n(x0 + u, y0 - 5, z0 + w, "stone_bricks_dark"); for y in -4..=-1 { if ring { let n = walls[r.int(0, 6) as usize]; p.n(x0 + u, y0 + y, z0 + w, n); } else { p.air(x0 + u, y0 + y, z0 + w); } } } }
        // les murs, les planchers
        for w in -4..=4i32 { for u in -4..=4i32 { let ring = u.abs() == 4 || w.abs() == 4; let top = reach(u, w, r);
            if ring { for y in 1..=(TOP + 2).min(top) {
                    let slit = matches!(y, 6 | 7 | 10 | 11) && (u == 0 || w == 0); let door = w == -4 && u == 0 && y <= 2; let crenel = y == TOP + 2 && (u + w).rem_euclid(2) == 1; if slit || door || crenel { continue; }
                    let n = if y == 4 || y == 8 || y == TOP { "stone_bricks" } else { walls[r.int(0, 6) as usize] }; p.n(x0 + u, y0 + y, z0 + w, n); } }
            else { p.n(x0 + u, y0, z0 + w, "stone_bricks"); for fl in [4, 8, TOP] { if fl <= top { p.n(x0 + u, y0 + fl, z0 + w, if fl == TOP { "stone_bricks" } else { "planks" }); } } }
        } }
        // les contreforts : deux à chaque angle, coiffés d'un glacis
        for (su, sw) in [(1, 1), (1, -1), (-1, 1)] { for (u, w, dx, dz) in [(5 * su, 4 * sw, -su, 0), (4 * su, 5 * sw, 0, -sw)] { for y in 1..=3 { p.n(x0 + u, y0 + y, z0 + w, "stone_bricks"); } p.turn(x0 + u, y0 + 4, z0 + w, "stone_bricks_stairs", dx, dz); } }
        // les trois volées : le long du mur ouest, puis nord, puis est ; le plancher s'ouvre au-dessus de chacune
        for i in 0..4 { p.turn(x0 - 3, y0 + 1 + i, z0 - 2 + i, "planks_stairs", 0, 1); p.turn(x0 - 2 + i, y0 + 5 + i, z0 + 3, "planks_stairs", 1, 0); p.turn(x0 + 3, y0 + 9 + i, z0 + 2 - i, "stone_bricks_stairs", 0, -1); }
        for i in 0..3 { p.air(x0 - 3, y0 + 4, z0 - 2 + i); p.air(x0 - 2 + i, y0 + 8, z0 + 3); p.air(x0 + 3, y0 + TOP, z0 + 2 - i); }
        // la trappe de la cave : une échelle contre le mur est
        for y in -4..=0 { p.turn(x0 + 3, y0 + y, z0 + 3, "ladder", 1, 0); }
        // la cave : le cachot aux barreaux tordus, la réserve, le coffre caché derrière les sacs
        for w in -3..=-1 { for y in -4..=-2 { if !(w == -2 && y <= -3) { p.n(x0 - 1, y0 + y, z0 + w, "iron_bars"); } } } for u in -3..=-2 { for y in -4..=-2 { p.n(x0 + u, y0 + y, z0, "iron_bars"); } }
        p.n(x0 - 3, y0 - 4, z0 - 3, "bone"); p.n(x0 - 2, y0 - 4, z0 - 2, "bedroll"); p.n(x0 - 3, y0 - 4, z0 - 1, "bucket"); p.turn(x0 - 3, y0 - 3, z0 - 3, "chain", 0, 1);
        for (u, w, n) in [(3, -3, "barrel"), (2, -3, "barrel"), (3, -2, "barrel"), (1, -3, "crate"), (-2, 3, "sack"), (-1, 3, "sack"), (-3, 2, "sack"), (1, 2, "candelabra"), (0, -3, "amphora")] { p.n(x0 + u, y0 - 4, z0 + w, n); }
        p.n(x0 + 3, y0 - 3, z0 - 3, "barrel"); p.n(x0 - 3, y0 - 4, z0 + 3, "chest"); p.n(x0 + 3, y0 - 1, z0 - 3, "cobweb"); p.n(x0 - 3, y0 - 1, z0 - 3, "cobweb"); p.n(x0, y0 - 1, z0 + 3, "cobweb");
        // le rez-de-chaussée : l'âtre, la table de garde
        for u in [-1, 1] { for y in 1..=2 { p.n(x0 + u, y0 + y, z0 + 3, "cobblestone"); } } for u in -1..=1 { p.n(x0 + u, y0 + 3, z0 + 3, "cobblestone"); } p.n(x0, y0 + 1, z0 + 3, "campfire_out");
        for (u, w, n) in [(1, 0, "table"), (0, 0, "stool"), (2, 0, "stool"), (1, -1, "stool"), (3, -3, "sack"), (3, -2, "barrel"), (2, 3, "log_pile"), (-2, 3, "log_pile"), (2, -3, "bucket")] { p.n(x0 + u, y0 + 1, z0 + w, n); }
        p.turn(x0 + 3, y0 + 1, z0, "tool_rack", 1, 0); p.n(x0 + 3, y0 + 3, z0 - 3, "cobweb");
        // le premier : le dortoir
        for (u, w, n) in [(-1, -3, "bedroll"), (1, -3, "bedroll"), (3, -1, "bedroll"), (0, 0, "carpet_red"), (1, 0, "carpet_red"), (0, 1, "carpet_red"), (1, 1, "carpet_red"), (3, 1, "candle"), (-1, 1, "stool")] { p.n(x0 + u, y0 + 5, z0 + w, n); }
        p.turn(x0 + 3, y0 + 5, z0 - 3, "cupboard", 0, -1); p.turn(x0 + 3, y0 + 5, z0 + 2, "chair", 1, 0); p.turn(x0 + 3, y0 + 6, z0 + 1, "wall_shelf", 1, 0); p.n(x0 - 3, y0 + 7, z0 - 3, "cobweb");
        // le second : la salle d'armes
        p.turn(x0 - 1, y0 + 9, z0 - 3, "tool_rack", 0, -1); p.turn(x0, y0 + 9, z0 - 3, "tool_rack", 0, -1); p.turn(x0 - 3, y0 + 9, z0, "hides", -1, 0);
        for (u, w, n) in [(-1, 0, "table"), (-2, 0, "books_pile"), (0, 0, "stool"), (1, -2, "candelabra"), (-3, 3, "amphora"), (-2, -3, "crate"), (1, -3, "bottles"), (-3, -2, "sack")] { if reach(u, w, r) >= 9 { p.n(x0 + u, y0 + 9, z0 + w, n); } }
        // le chemin de ronde : le brasero sur son socle, le coffre, la bannière, le hourd de bois à l'est sur ses corbeaux
        p.n(x0, y0 + TOP + 1, z0, "chiseled_stone"); p.n(x0, y0 + TOP + 2, z0, "brazier"); p.n(x0 - 3, y0 + TOP + 1, z0 + 3, "chest"); p.n(x0 + 4, y0 + TOP + 3, z0 + 4, "flag"); p.n(x0 + 2, y0 + TOP + 1, z0 - 2, "barrel");
        for w in -3..=3 { p.n(x0 + 5, y0 + TOP, z0 + w, "planks"); p.n(x0 + 5, y0 + TOP + 1, z0 + w, "fence"); if w % 3 == 0 { let b = p.id("stone_bricks_stairs"); p.put(x0 + 5, y0 + TOP - 1, z0 + w, variant(b, facing_of(-1.0, 0.0), true, false, false)); } }
        for w in [-2, 1] { p.air(x0 + 4, y0 + TOP + 1, z0 + w); } // deux portes du rempart vers le hourd
        // les éboulis au pied de l'angle tombé
        for _ in 0..22 { let (u, w) = (bx + r.int(-4, 1), bz + r.int(-4, 1)); if u.abs() <= 4 && w.abs() <= 4 { continue; } let (x, z) = (x0 + u, z0 + w); let n = walls[r.int(3, 6) as usize]; let g = self.gy(x, z).max(if u.abs() <= 5 && w.abs() <= 5 { y0 } else { 0 }); p.n(x, g + 1, z, n); if r.next() < 0.25 { p.n(x, g + 2, z, n); } }
        // la courtine : elle part de l'angle nord-est, épouse le terrain, s'abaisse en s'éloignant ; une arche la traverse
        for k in 0..13 { let (x, z) = (x0 + 4, z0 + 5 + k); let g = self.gy(x, z); let h = (6.0 - k as f64 * 0.38 + r.range(-0.8, 0.8)).round().max(1.0) as i32; let arch = k == 5 || k == 6;
            for y in 1..=h { if arch && y <= 3 { p.air(x, g + y, z); continue; } let n = walls[r.int(0, 6) as usize]; p.n(x, g + y, z, n); } if h >= 5 && k % 2 == 0 { p.n(x, g + h + 1, z, "stone_bricks"); } for y in h + 2..=9 { p.air(x, g + y, z); } }
        // une charrette abandonnée et un feu froid au pied du mur
        let (cx, cz) = (x0 + 2, z0 + 9); let g = self.gy(cx, cz); p.turn(cx, g + 1, cz, "cart", 1, 0); let g2 = self.gy(cx - 1, cz + 3); p.n(cx - 1, g2 + 1, cz + 3, "campfire_out");
    }
    /// Le cimetière : un porche couvert, une allée, des rangs de tombes où brûlent quelques bougies, deux tombeaux, des arbres
    /// morts ; au fond, une chapelle au toit à demi tombé (bancs, autel, vitraux) et, sous elle, la crypte aux sarcophages.
    fn build_cemetery(&self, p: &mut Plan, pl: &Place, r: &mut Rng) {
        let (x0, y0, z0) = (pl.x, pl.y, pl.z); let (hx, hz) = (11, 10); self.pad(p, pl, |_, _| true, hx, hz, 11, None);
        // le mur d'enceinte, éboulé par endroits, ses grilles
        for w in -hz..=hz { for u in -hx..=hx { if !(u.abs() == hx || w.abs() == hz) || (w == -hz && u.abs() <= 2) || (w == hz && u.abs() <= 3) { continue; }
            if u.abs() == hx && w.abs() == hz { for y in 1..=3 { p.n(x0 + u, y0 + y, z0 + w, "stone_bricks"); } p.n(x0 + u, y0 + 4, z0 + w, "stone_bricks_slab"); continue; }
            if r.next() < 0.12 { continue; } p.n(x0 + u, y0 + 1, z0 + w, if r.next() < 0.4 { "mossy_stone" } else { "cobblestone" }); if (u + w).rem_euclid(4) == 0 { p.n(x0 + u, y0 + 2, z0 + w, "cobblestone"); } else if r.next() < 0.55 { p.n(x0 + u, y0 + 2, z0 + w, "iron_bars"); } } }
        // le porche : deux poteaux, un petit toit d'ardoise en bâtière
        for u in [-2, 2] { for y in 1..=3 { p.n(x0 + u, y0 + y, z0 - hz, "oak_log"); } } for u in -2..=2 { p.n(x0 + u, y0 + 4, z0 - hz, "oak_log_x"); p.turn(x0 + u, y0 + 4, z0 - hz - 1, "slate_slope", 0, 1); p.turn(x0 + u, y0 + 4, z0 - hz + 1, "slate_slope", 0, -1); p.n(x0 + u, y0 + 5, z0 - hz, "slate_ridge"); }
        p.n(x0 - 1, y0 + 3, z0 - hz, "lantern");
        // l'allée
        for w in -hz..=1 { p.n(x0, y0, z0 + w, "gravel"); if r.next() < 0.55 { p.n(x0 + if r.next() < 0.5 { 1 } else { -1 }, y0, z0 + w, "gravel"); } } for u in -8..=8 { if r.next() < 0.8 { p.n(x0 + u, y0, z0 - 2, "gravel"); } }
        // les tombes
        let mut graves: Vec<(i32, i32)> = Vec::new(); for &w in &[-8, -5, 0] { for &u in &[-9, -7, -5, -3, 3, 5, 7, 9] { graves.push((u, w)); } } for &w in &[3, 6] { for &u in &[-9, -7, 7, 9] { graves.push((u, w)); } }
        for (u, w) in graves { let k = r.next(); if k < 0.14 { continue; } let (dx, dz) = if r.next() < 0.8 { (0, 1) } else { (0, -1) };
            p.turn(x0 + u, y0 + 1, z0 + w, if k < 0.7 { "gravestone" } else { "grave_cross" }, dx, dz); if r.next() < 0.2 { p.n(x0 + u + 1, y0 + 1, z0 + w, "candle"); } else if r.next() < 0.2 { p.n(x0 + u + 1, y0 + 1, z0 + w, "pink_flower"); } }
        // deux tombeaux de pierre, grille en façade
        for &(u, w) in &[(-7, 8), (7, 8)] { for a in -1..=1i32 { for b in -1..=1i32 { for y in 1..=2 { if a == 0 && b == -1 { if y == 1 { p.n(x0 + u, y0 + 1, z0 + w - 1, "iron_bars"); } else { p.n(x0 + u, y0 + 2, z0 + w - 1, "chiseled_stone"); } } else if a.abs() == 1 || b.abs() == 1 { p.n(x0 + u + a, y0 + y, z0 + w + b, "stone_bricks"); } } p.n(x0 + u + a, y0 + 3, z0 + w + b, "stone_bricks_slab"); } } p.n(x0 + u, y0 + 1, z0 + w, "bone"); p.n(x0 + u, y0 + 4, z0 + w, "cross"); }
        // l'herbe folle, les os, deux arbres morts
        for _ in 0..40 { let (u, w) = (r.int(-hx + 1, hx - 1), r.int(-hz + 1, 1)); if u.abs() <= 1 || p.m.contains_key(&(x0 + u, y0 + 1, z0 + w)) && p.m[&(x0 + u, y0 + 1, z0 + w)] > 0 { continue; } let n = match r.int(0, 7) { 0 | 1 => "dead_bush", 2 => "bone", 3 => "mushroom", 4 => "fern", _ => "tall_grass" }; p.n(x0 + u, y0 + 1, z0 + w, n); }
        for &(u, w, h) in &[(-10, 2, 5), (10, -4, 4)] { for y in 1..=h { p.n(x0 + u, y0 + y, z0 + w, "dead_log"); } p.n(x0 + u + if u < 0 { 1 } else { -1 }, y0 + h, z0 + w, "dead_branch"); p.n(x0 + u, y0 + h + 1, z0 + w, "dead_branch"); p.n(x0 + u, y0 + h - 1, z0 + w + 1, "dead_branch"); }
        // la chapelle : sept sur neuf, pignons, toit d'ardoise dont tout un pan est tombé
        let (c0, c1) = (2, 10); // de la façade au chevet
        for w in c0..=c1 { for u in -3..=3i32 { let wall = u.abs() == 3 || w == c0 || w == c1; p.n(x0 + u, y0, z0 + w, "stone_bricks");
            if wall { for y in 1..=4 { let door = w == c0 && u == 0 && y <= 2; let glass = y >= 2 && y <= 3 && ((u.abs() == 3 && matches!(w, 4 | 6 | 8)) || (w == c1 && u == 0)); if door { continue; }
                    p.n(x0 + u, y0 + y, z0 + w, if glass { "stained_glass" } else if r.next() < 0.18 { "mossy_stone" } else { "stone_bricks" }); } }
            // les pignons montent sous le toit
            if w == c0 || w == c1 { for y in 5..=7 { if u.abs() <= 7 - y { p.n(x0 + u, y0 + y, z0 + w, if y == 6 && u == 0 { "stained_glass" } else { "stone_bricks" }); } } }
            // le toit : deux versants, le faîte ; le versant est s'est effondré vers le chevet
            for (y, a) in [(5, 3), (6, 2), (7, 1)] { if u.abs() == a { let fallen = u > 0 && w >= 6 && r.next() < 0.85 || r.next() < 0.06; if !fallen { p.turn(x0 + u, y0 + y, z0 + w, "slate_slope", -u.signum(), 0); } } }
            if u == 0 && !(w >= 7 && r.next() < 0.6) { p.n(x0, y0 + 8, z0 + w, "slate_ridge_1"); }
        } }
        for u in [-4, 4] { for w in [c0, c1] { for y in 1..=3 { p.n(x0 + u, y0 + y, z0 + w, "stone_bricks"); } p.turn(x0 + u, y0 + 4, z0 + w, "stone_bricks_stairs", -u.signum(), 0); } } // contreforts
        // des ardoises tombées dans l'herbe, côté est
        for _ in 0..6 { let (u, w) = (r.int(4, 6), r.int(5, 9)); p.n(x0 + u, y0 + 1, z0 + w, if r.next() < 0.5 { "slate_slab" } else { "cobblestone" }); }
        // dedans : l'allée de tapis, les bancs, l'autel, la croix, les chandeliers
        for w in 3..=7 { p.n(x0, y0 + 1, z0 + w, "carpet_red"); } for &w in &[4, 6] { for &u in &[-2, -1, 1, 2] { if r.next() < 0.85 { p.turn(x0 + u, y0 + 1, z0 + w, "bench", 0, -1); } } }
        p.n(x0, y0 + 1, z0 + 8, "table"); p.n(x0 - 1, y0 + 1, z0 + 8, "candelabra"); p.n(x0 + 1, y0 + 1, z0 + 8, "candelabra"); p.n(x0, y0 + 1, z0 + 9, "cross"); p.n(x0 + 2, y0 + 1, z0 + 9, "books_pile"); p.n(x0 + 2, y0 + 4, z0 + 3, "cobweb"); p.n(x0 - 2, y0 + 4, z0 + 9, "cobweb");
        // la crypte, sous la nef : on y descend par une échelle derrière l'autel
        for w in c0..=c1 { for u in -3..=3i32 { let ring = u.abs() == 3 || w == c0 || w == c1; p.n(x0 + u, y0 - 5, z0 + w, "stone_bricks_dark"); for y in -4..=-1 { if ring { p.n(x0 + u, y0 + y, z0 + w, if r.next() < 0.3 { "mossy_stone" } else { "stone_bricks" }); } else { p.air(x0 + u, y0 + y, z0 + w); } } } }
        for y in -4..=0 { p.turn(x0 - 2, y0 + y, z0 + 9, "ladder", -1, 0); }
        for &w in &[5, 7] { for &u in &[-1, 1] { for y in -4..=-1 { p.n(x0 + u, y0 + y, z0 + w, "stone_pillar"); } } }
        for &(u, w) in &[(2, 3), (2, 6), (-2, 5)] { for k in 0..2 { p.n(x0 + u, y0 - 4, z0 + w + k, "stone_bricks"); p.n(x0 + u, y0 - 3, z0 + w + k, "stone_bricks_slab"); } } // les sarcophages
        p.n(x0, y0 - 4, z0 + 6, "chiseled_stone"); p.n(x0, y0 - 3, z0 + 6, "stone_bricks_slab"); p.n(x0, y0 - 4, z0 + 3, "chest");
        for (u, w, n) in [(-2, 3, "candle"), (2, 9, "candle"), (-1, 3, "bone"), (1, 9, "bone"), (2, 5, "amphora"), (-2, 7, "bone"), (0, 8, "candelabra")] { p.n(x0 + u, y0 - 4, z0 + w, n); }
        for (u, w) in [(2, 3), (-2, 3), (2, 9), (0, 5)] { p.n(x0 + u, y0 - 1, z0 + w, "cobweb"); }
    }
    /// Le camp de brigands : une palissade de pieux, un portail, une tour de guet de bois, la grande tente du chef et deux
    /// petites, le feu et sa marmite, une cage, la charrette volée et ses caisses.
    fn build_camp(&self, p: &mut Plan, pl: &Place, r: &mut Rng) {
        let (x0, y0, z0) = (pl.x, pl.y, pl.z); let d = |u: i32, w: i32| (u as f64).hypot(w as f64); self.pad(p, pl, |u, w| d(u, w) < 11.2, 11, 11, 10, None);
        // la palissade : des rondins serrés, taillés en pointe
        for w in -11..=11i32 { for u in -11..=11i32 { let dd = d(u, w); if !(9.5..10.5).contains(&dd) { continue; } if w < 0 && u.abs() <= 1 { continue; } if r.next() < 0.05 { continue; }
            let h = 3 + (hash_int(x0 + u, z0 + w, self.seed, 176) % 2) as i32; for y in 1..=h { p.n(x0 + u, y0 + y, z0 + w, "oak_log"); } p.n(x0 + u, y0 + h + 1, z0 + w, "oak_post"); } }
        // le portail : deux grands pieux, une traverse, une bannière
        for u in [-2, 2] { for y in 1..=5 { p.n(x0 + u, y0 + y, z0 - 10, "oak_log"); } } for u in -1..=1 { p.n(x0 + u, y0 + 5, z0 - 10, "oak_log_x"); } p.n(x0 + 2, y0 + 6, z0 - 10, "flag"); p.n(x0, y0 + 4, z0 - 10, "lantern");
        // le sol battu : du portail au feu, autour du feu
        for w in -10..=0 { for u in -1..=1 { if r.next() < 0.75 { p.n(x0 + u, y0, z0 + w, "path"); } } } for w in -3..=3i32 { for u in -3..=3i32 { if d(u, w) < 3.4 && r.next() < 0.8 { p.n(x0 + u, y0, z0 + w, "path"); } } }
        // le feu, la marmite, les troncs où l'on s'assoit
        p.n(x0, y0 + 1, z0, "campfire"); p.n(x0 + 1, y0 + 1, z0 + 1, "cauldron"); p.n(x0 - 2, y0 + 1, z0, "dead_log_z"); p.n(x0 - 2, y0 + 1, z0 + 1, "dead_log_z"); p.n(x0 + 2, y0 + 1, z0 - 1, "dead_log_z"); p.n(x0, y0 + 1, z0 - 2, "dead_log_x"); p.n(x0 + 1, y0 + 1, z0 - 2, "dead_log_x"); p.n(x0 - 1, y0 + 1, z0 + 2, "bottles");
        // deux petites tentes : des pans de chaume appuyés l'un sur l'autre, des paillasses dessous
        for &u0 in &[-8, 5] { for w in -1..=2 { p.turn(x0 + u0, y0 + 1, z0 + w, "thatch_slope", 1, 0); p.turn(x0 + u0 + 3, y0 + 1, z0 + w, "thatch_slope", -1, 0); p.turn(x0 + u0 + 1, y0 + 2, z0 + w, "thatch_slope", 1, 0); p.turn(x0 + u0 + 2, y0 + 2, z0 + w, "thatch_slope", -1, 0); }
            p.n(x0 + u0 + 1, y0 + 1, z0 + 2, "thatch"); p.n(x0 + u0 + 2, y0 + 1, z0 + 2, "thatch"); p.n(x0 + u0 + 1, y0 + 1, z0, "bedroll"); p.n(x0 + u0 + 2, y0 + 1, z0 + 1, "bedroll"); }
        // la grande tente du chef, au fond : trois rangs de pans, un fond de laine rouge, le siège, la table, le coffre
        for w in 4..=8 { for (y, a) in [(1, 3), (2, 2), (3, 1)] { p.turn(x0 - a, y0 + y, z0 + w, "thatch_slope", 1, 0); p.turn(x0 + a - 1, y0 + y, z0 + w, "thatch_slope", -1, 0); } }
        for (y, a) in [(1, 2), (2, 1)] { for u in -a..a { p.n(x0 + u, y0 + y, z0 + 8, "wool_red"); } } for w in 4..=8 { for u in -2..=1 { p.n(x0 + u, y0, z0 + w, "planks"); } }
        p.n(x0, y0 + 1, z0 + 7, "chest"); p.turn(x0 - 1, y0 + 1, z0 + 7, "throne", 0, 1); p.n(x0 - 2, y0 + 1, z0 + 5, "sack"); p.n(x0 + 1, y0 + 1, z0 + 6, "candle"); for (u, w) in [(-1, 5), (0, 5), (-1, 6), (0, 6)] { p.n(x0 + u, y0 + 1, z0 + w, "carpet_red"); }
        // la tour de guet de bois : quatre montants, une plate-forme à garde-corps, un toit de chaume, une échelle
        let (tx, tz) = (6, -6); for (a, b) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] { for y in 1..=8 { p.n(x0 + tx + a, y0 + y, z0 + tz + b, if y <= 5 { "oak_log" } else { "oak_post" }); } }
        for a in -1..=1i32 { for b in -1..=1i32 { p.n(x0 + tx + a, y0 + 5, z0 + tz + b, "planks"); p.n(x0 + tx + a, y0 + 9, z0 + tz + b, "thatch"); if (a != 0 || b != 0) && a.abs() + b.abs() == 1 && !(a == 0 && b == 1) { p.n(x0 + tx + a, y0 + 6, z0 + tz + b, "fence"); } } }
        for y in 1..=5 { p.turn(x0 + tx, y0 + y, z0 + tz + 2, "ladder", 0, -1); }
        // la cage : des barreaux, un toit de planches, ce qu'il reste du dernier prisonnier
        for a in -1..=1i32 { for b in -1..=1i32 { let (u, w) = (-6 + a, 5 + b); if a.abs() == 1 || b.abs() == 1 { for y in 1..=2 { p.n(x0 + u, y0 + y, z0 + w, "iron_bars"); } } p.n(x0 + u, y0 + 3, z0 + w, "planks_slab"); } } p.n(x0 - 6, y0 + 1, z0 + 5, "bone");
        // la charrette volée, les caisses, les sacs, les tonneaux, le bois
        p.turn(x0 + 4, y0 + 1, z0 + 4, "cart", 1, 0); for (u, w, y, n) in [(6, 4, 1, "crate"), (6, 5, 1, "crate"), (6, 4, 2, "crate"), (5, 6, 1, "sack"), (4, 6, 1, "sack"), (3, 7, 1, "barrel"), (4, 7, 1, "barrel"), (4, 7, 2, "barrel"), (-4, -6, 1, "log_pile"), (-5, -6, 1, "log_pile"), (-5, -6, 2, "log_pile"), (-3, -7, 1, "sawhorse"), (-7, -4, 1, "amphora")] { p.n(x0 + u, y0 + y, z0 + w, n); }
        for u in [-3, 3] { p.n(x0 + u, y0 + 1, z0 - 8, "fence"); p.n(x0 + u, y0 + 2, z0 - 8, "lantern"); }
    }
}

impl crate::session::Session {
    /// Les gardiens d'un lieu sortent quand le joueur approche (une fois, puis de nouveau après un quart d'heure d'absence) ;
    /// les fantômes du cimetière ne se lèvent que la nuit.
    pub(crate) fn site_tick(&mut self) {
        if self.dead.is_some() { return; } let me = self.player.p; let night = self.sun_height() < 0.0;
        // un camp de brigands qu'on voit : il montre ce qu'il a bâti en grandissant
        for pl in self.gen.places_near(me[0], me[2], 90.0).into_iter().filter(|p| p.kind == Lieu::Camp) { self.camp_show(&pl); }
        // un sanctuaire qu'on approche : ses mécanismes se mettent en place (une fois)
        for pl in self.gen.places_near(me[0], me[2], 56.0).into_iter().filter(|p| matches!(p.kind, Lieu::Sanctuaire | Lieu::Ruines | Lieu::Grotte | Lieu::Soleil | Lieu::Catacombes | Lieu::Forge | Lieu::Glace | Lieu::Horloge | Lieu::Observatoire | Lieu::Serre)) { match pl.kind { Lieu::Serre => self.greenhouse_wake(&pl), Lieu::Observatoire => self.observatory_wake(&pl), Lieu::Horloge => self.clock_tower_wake(&pl), Lieu::Glace => self.ice_temple_wake(&pl), Lieu::Forge => self.forge_wake(&pl), Lieu::Ruines => self.dungeon_wake(&pl), Lieu::Grotte => self.grotto_wake(&pl), Lieu::Soleil => self.sun_temple_wake(&pl), Lieu::Catacombes => self.catacombs_wake(&pl), _ => self.sanctuary_wake(&pl) } }
        for pl in self.gen.places_near(me[0], me[2], 44.0) {
            for ghosts in [false, true] {
                if ghosts && !night { continue; } let key = (pl.cell, ghosts); if self.sites_awake.get(&key).is_some_and(|t| self.elapsed - t < 900.0) { self.sites_awake.insert(key, self.elapsed); continue; }
                // un camp de brigands : ses gardiens selon sa puissance (personne s'il est dispersé), chacun marqué de son camp
                let level = if pl.kind == Lieu::Camp && !ghosts { Some(self.camp(pl.cell).level()) } else { None };
                let who: Vec<(&'static str, [f64; 3], bool)> = match level { Some(l) => crate::camps::garrison(&pl, l), None => pl.guards().into_iter().filter(|g| (g.0 == "fantome") == ghosts).enumerate().map(|(k, g)| (g.0, g.1, !ghosts && k == Place::CHIEF)).collect() };
                if who.is_empty() { if level == Some(0) { self.sites_awake.insert(key, self.elapsed); self.notes.push("Un camp de brigands abandonné : les cendres sont froides".into()); } continue; }
                self.sites_awake.insert(key, self.elapsed); let yaw = (me[0] - pl.x as f64).atan2(me[2] - pl.z as f64); let n = who.len() as u32;
                for (id, at, vet) in who { if let Some(sp) = crate::hostile::species(id) { let i = self.horde.put(sp, at, yaw, vet); let f = &mut self.horde.foes[i]; f.kept = true; if level.is_some() { f.camp = Some(pl.cell); } if vet && level.is_none_or(|_| f.hp > 0.0) { f.hp *= 2.0; } } }
                if let Some(l) = level { self.camp(pl.cell).left = n; self.notes.push(format!("{} en vue · puissance {l}/5", pl.kind.name())); self.found_place(pl.cell, pl.kind.name()); continue; }
                if !ghosts { self.notes.push(format!("{} en vue", pl.kind.name())); self.found_place(pl.cell, pl.kind.name()); }
            }
        }
    }
    /// `/lieu [tour|cimetiere|brigands|sanctuaire|ruines|grotte|soleil]` : va au lieu de surface le plus proche (de ce genre).
    pub fn goto_place(&mut self, arg: Option<&str>) -> String {
        let inside = arg.is_some_and(|a| a.ends_with('!')); let arg = arg.map(|a| a.trim_end_matches('!'));
        let kind = match arg { None => None, Some(a) => match LIEUX.iter().find(|k| k.key().starts_with(a) || a.starts_with(k.key()) || (a == "camp" && **k == Lieu::Camp)) { Some(k) => Some(*k), None => return "Lieu inconnu. Au choix : tour, cimetiere, brigands, sanctuaire, ruines, grotte, soleil, catacombes, forge, glace, horloge, observatoire, serre".into() } };
        let p = self.player.p; let near = |r: f64| self.gen.places_near(p[0], p[2], r).into_iter().find(|pl| kind.is_none_or(|k| k == pl.kind));
        let Some(pl) = near(2600.0).or_else(|| near(5200.0)) else { return "Aucun lieu de ce genre dans les parages".into() };
        let (at, yaw) = if inside { pl.inside() } else { pl.view() }; let y = if inside { at[1] } else { self.gen.column(at[0], at[2]).h.max(pl.y as f64) + 1.5 }; self.player.p = [at[0], y, at[2]]; self.player.v = [0.0; 3]; self.player.yaw = yaw; self.player.pitch = if !inside && pl.kind == Lieu::Tour { 0.28 } else { 0.05 };
        format!("{} à {}, {}, {}", pl.kind.name(), pl.x, pl.y, pl.z)
    }
}
