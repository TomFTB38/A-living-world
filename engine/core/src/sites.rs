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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)] pub enum Lieu { Tour, Cimetiere, Camp }
impl Lieu {
    pub fn name(self) -> &'static str { match self { Lieu::Tour => "Tour de guet en ruine", Lieu::Cimetiere => "Cimetière abandonné", Lieu::Camp => "Camp de brigands" } }
    pub fn key(self) -> &'static str { match self { Lieu::Tour => "tour", Lieu::Cimetiere => "cimetiere", Lieu::Camp => "brigands" } }
    /// Demi-emprise (le long de x, le long de z) et dénivelé toléré.
    fn fit(self) -> (i32, i32, f64) { match self { Lieu::Tour => (4, 4, 4.0), Lieu::Cimetiere => (10, 8, 3.6), Lieu::Camp => (9, 9, 4.0) } }
}
pub const LIEUX: [Lieu; 3] = [Lieu::Tour, Lieu::Cimetiere, Lieu::Camp];
/// Un lieu décidé : son centre, le niveau de son sol (le bloc sur lequel on marche), sa maille.
#[derive(Clone, Copy, Debug, PartialEq)] pub struct Place { pub kind: Lieu, pub x: i32, pub y: i32, pub z: i32, pub cell: (i32, i32) }
impl Place {
    /// Le coffre du lieu.
    pub fn chest(&self) -> [i32; 3] { match self.kind { Lieu::Tour => [self.x - 2, self.y + TOP + 1, self.z - 2], Lieu::Cimetiere => [self.x, self.y + 1, self.z + 5], Lieu::Camp => [self.x, self.y + 1, self.z + 5] } }
    /// Où lâcher les gardiens : espèce, position.
    pub fn guards(&self) -> Vec<(&'static str, [f64; 3])> {
        let at = |dx: f64, dy: f64, dz: f64| [self.x as f64 + 0.5 + dx, self.y as f64 + 1.05 + dy, self.z as f64 + 0.5 + dz];
        match self.kind {
            Lieu::Tour => vec![("archer", at(-1.0, TOP as f64, -1.0)), ("archer", at(0.0, 0.0, -6.0))],
            Lieu::Cimetiere => vec![("fantome", at(-5.0, 0.0, -2.0)), ("fantome", at(5.0, 0.0, 1.0)), ("archer", at(0.0, 0.0, 1.0))],
            Lieu::Camp => vec![("brigand", at(-2.0, 0.0, -1.5)), ("brigand", at(2.5, 0.0, 1.0)), ("brigand", at(0.0, 0.0, 3.0))],
        }
    }
    /// Où se tenir pour le regarder (commande, captures) : position et cap.
    pub fn view(&self) -> ([f64; 3], f64) { let d = match self.kind { Lieu::Tour => 13.0, Lieu::Cimetiere => 15.0, Lieu::Camp => 16.0 }; ([self.x as f64 + 0.5, self.y as f64 + 1.2, self.z as f64 + 0.5 - d], 0.0) }
}
/// Hauteur du dernier plancher de la tour au-dessus du sol.
const TOP: i32 = 13;

/// Le chantier d'un lieu : ce qu'on pose (ou dégage : -1), case par case.
struct Plan { m: BTreeMap<(i32, i32, i32), i32>, ids: std::collections::HashMap<&'static str, Id> }
impl Plan {
    fn put(&mut self, x: i32, y: i32, z: i32, b: Id) { self.m.insert((x, y, z), b as i32); }
    fn air(&mut self, x: i32, y: i32, z: i32) { self.m.insert((x, y, z), -1); }
    fn id(&mut self, n: &'static str) -> Id { *self.ids.entry(n).or_insert_with(|| by_name(n).unwrap_or_else(|| panic!("bloc inconnu : {n}"))) }
    fn n(&mut self, x: i32, y: i32, z: i32, n: &'static str) { let b = self.id(n); self.put(x, y, z, b); }
    /// Un bloc orienté : `(dx, dz)` est le côté haut d'une pente, le mur d'une échelle, le dos d'un meuble.
    fn turn(&mut self, x: i32, y: i32, z: i32, n: &'static str, dx: i32, dz: i32) { let b = self.id(n); self.put(x, y, z, variant(b, facing_of(dx as f64, dz as f64), false, false, false)); }
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
        let first = (hash_float(gx, gz, s, 174) * 3.0) as usize; let mut r = Rng::new(hash_int(gx, gz, s, 172));
        for k in 0..3 { let kind = LIEUX[(first + k) % 3]; if k > 0 && kind == Lieu::Tour && hash_float(gx, gz, s, 177) > 0.3 { continue; } /* la tour ne bouche pas tous les trous */ for _ in 0..if kind == Lieu::Tour { 8 } else { 36 } {
            let x = ((gx as f64 + r.range(0.15, 0.85)) * SITE_CELL as f64).floor() as i32; let z = ((gz as f64 + r.range(0.15, 0.85)) * SITE_CELL as f64).floor() as i32;
            if ((x + 38) as f64).hypot(z as f64) < 90.0 { continue; } // pas sur le pas de la porte
            if let Some(y) = self.place_fits(kind, x, z) { if !self.villages_near(x as f64, z as f64, 120.0).is_empty() { continue; } return Some(Place { kind, x, y, z, cell: (gx, gz) }); } } }
        None
    }
    /// Le terrain porte-t-il ce lieu ici ? Rend le niveau du sol.
    fn place_fits(&self, kind: Lieu, x: i32, z: i32) -> Option<i32> {
        let (hx, hz, tol) = kind.fit(); let c0 = self.column(x as f64, z as f64);
        match kind { Lieu::Camp => if c0.forest < 0.2 && c0.mix.of(crate::biome::Biome::Steppe) < 0.5 { return None; }, Lieu::Cimetiere => if c0.forest > 0.75 { return None; }, Lieu::Tour => {} }
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
    /// Le coffre d'un lieu de surface est-il ici ?
    pub fn place_chest(&self, p: [i32; 3]) -> Option<Lieu> {
        let (gx, gz) = (p[0].div_euclid(SITE_CELL), p[2].div_euclid(SITE_CELL)); self.place(gx, gz).filter(|pl| pl.chest() == p).map(|pl| pl.kind)
    }
    /// Dégage la place et assoit le sol : comble les creux, rase les bosses, enlève arbres et rochers au-dessus.
    fn pad(&self, p: &mut Plan, pl: &Place, inside: impl Fn(i32, i32) -> bool, hx: i32, hz: i32, up: i32, floor: Option<Id>) {
        for dz in -hz - 2..=hz + 2 { for dx in -hx - 2..=hx + 2 {
            let (x, z) = (pl.x + dx, pl.z + dz); let core = dx.abs() <= hx && dz.abs() <= hz && inside(dx, dz); let h = self.column(x as f64, z as f64).h.floor() as i32;
            if core { for y in h + 1..pl.y { p.put(x, y, z, B::DIRT); } for y in pl.y + 1..=h.max(pl.y) { p.air(x, y, z); } p.put(x, pl.y, z, floor.unwrap_or(B::GRASS)); for y in pl.y + 1..=pl.y + up { p.air(x, y, z); } }
            else { for y in h.max(pl.y) + 1..=pl.y + up { p.air(x, y, z); } } // la marge : on n'y touche pas au sol, on y coupe ce qui dépasse
        } }
    }
    pub(crate) fn site_stamp(&self, gx: i32, gz: i32) -> Option<Stamp> {
        let pl = self.place(gx, gz)?; let mut r = Rng::new(hash_int(gx, gz, self.seed, 175)); let mut p = Plan { m: BTreeMap::new(), ids: Default::default() };
        match pl.kind { Lieu::Tour => self.build_tower(&mut p, &pl, &mut r), Lieu::Cimetiere => self.build_cemetery(&mut p, &pl, &mut r), Lieu::Camp => self.build_camp(&mut p, &pl, &mut r) }
        Some(Stamp { data: p.m.into_iter().map(|((a, b, c), id)| [a, b, c, id]).collect(), force: true, ground: false })
    }
    /// La tour de guet : sept blocs de côté, trois niveaux, un chemin de ronde crénelé ; un angle s'est effondré.
    fn build_tower(&self, p: &mut Plan, pl: &Place, r: &mut Rng) {
        let (x0, y0, z0) = (pl.x, pl.y, pl.z); let cob = p.id("cobblestone"); self.pad(p, pl, |_, _| true, 4, 4, TOP + 5, Some(cob));
        let (bx, bz) = (3, 3); // l'angle effondré (à l'opposé du coffre)
        let walls = ["stone_bricks", "stone_bricks", "stone_bricks", "cobblestone", "cobblestone", "mossy_stone"];
        // jusqu'où monte encore le mur à cet endroit : entier loin de l'angle, mangé près de lui
        let height = |u: i32, w: i32, r: &mut Rng| -> i32 { let d = ((u - bx) as f64).hypot((w - bz) as f64); if d > 5.0 { TOP + 1 } else { (TOP as f64 - (5.0 - d) * 1.9 + r.range(-0.6, 0.6)).round() as i32 } };
        for w in -3..=3i32 { for u in -3..=3i32 { let ring = u.abs() == 3 || w.abs() == 3; let top = height(u, w, r);
            if ring { for y in 1..=top.min(TOP + 1) {
                    let slit = (y == 6 || y == 10) && (u == 0 || w == 0); let door = w == -3 && u == 0 && y <= 2; let merlon = y == TOP + 1 && (u + w).rem_euclid(2) == 1; if slit || door || merlon { continue; }
                    let n = walls[r.int(0, walls.len() as i32 - 1) as usize]; p.n(x0 + u, y0 + y, z0 + w, n); } }
            else { for fl in [5, 9, TOP] { if fl <= top && !(u == -2 && w == 2) { p.n(x0 + u, y0 + fl, z0 + w, if fl == TOP { "stone_bricks" } else { "planks" }); } } }
        } }
        // l'échelle, contre le mur du fond, du sol au chemin de ronde
        for y in 1..=TOP { p.turn(x0 - 2, y0 + y, z0 + 2, "ladder", 0, 1); }
        // ce qui reste dedans : un tonneau, une paillasse, des toiles, le coffre là-haut
        p.n(x0 + 2, y0 + 1, z0 - 2, "barrel"); p.n(x0 + 2, y0 + 6, z0 - 2, "bedroll"); p.n(x0 - 2, y0 + 4, z0 - 2, "cobweb"); p.n(x0 + 2, y0 + 8, z0 - 2, "cobweb"); p.n(x0 - 2, y0 + 12, z0 - 2, "cobweb");
        p.n(x0 + 1, y0 + 1, z0 + 1, "campfire_out"); p.n(x0 - 2, y0 + TOP + 1, z0 - 2, "chest");
        // les éboulis au pied de l'angle tombé
        for _ in 0..14 { let (u, w) = (bx + r.int(-1, 4), bz + r.int(-1, 4)); if u.abs() <= 3 && w.abs() <= 3 { continue; } let (x, z) = (x0 + u, z0 + w); let h = self.column(x as f64, z as f64).h.floor() as i32; let n = walls[r.int(2, 5) as usize]; p.n(x, h.max(y0) + 1, z, n); }
    }
    /// Le cimetière : un muret éboulé, une allée de gravier, des rangs de tombes, un arbre mort, un caveau au fond.
    fn build_cemetery(&self, p: &mut Plan, pl: &Place, r: &mut Rng) {
        let (x0, y0, z0) = (pl.x, pl.y, pl.z); let (hx, hz) = (9, 7); self.pad(p, pl, |_, _| true, hx, hz, 9, None);
        for w in -hz..=hz { for u in -hx..=hx { let edge = u.abs() == hx || w.abs() == hz; if !edge { continue; } let gate = w == -hz && u.abs() <= 1; if gate { continue; }
            let post = (u.abs() == hx && w.abs() == hz) || (w == -hz && u.abs() == 2); if post { for y in 1..=3 { p.n(x0 + u, y0 + y, z0 + w, "stone_bricks"); } continue; }
            if r.next() < 0.14 { continue; } p.n(x0 + u, y0 + 1, z0 + w, if r.next() < 0.4 { "mossy_stone" } else { "cobblestone" }); if r.next() < 0.3 { p.n(x0 + u, y0 + 2, z0 + w, "iron_bars"); } } }
        for w in -hz..=2 { p.n(x0, y0, z0 + w, "gravel"); if r.next() < 0.5 { p.n(x0 + if r.next() < 0.5 { 1 } else { -1 }, y0, z0 + w, "gravel"); } }
        // les tombes, la tête au fond, le tertre vers l'entrée… ou l'inverse : personne ne les a alignées depuis longtemps
        for &w in &[-5, -2, 1] { for &u in &[-7, -5, -3, 3, 5, 7] { let k = r.next(); if k < 0.16 { continue; }
            let (dx, dz) = if r.next() < 0.75 { (0, -1) } else { (0, 1) }; p.turn(x0 + u, y0 + 1, z0 + w, if k < 0.72 { "gravestone" } else { "grave_cross" }, dx, dz); } }
        for _ in 0..16 { let (u, w) = (r.int(-hx + 1, hx - 1), r.int(-hz + 1, 2)); if u.abs() <= 1 || p.m.get(&(x0 + u, y0 + 1, z0 + w)).is_some_and(|b| *b > 0) { continue; } let n = match r.int(0, 5) { 0 | 1 => "dead_bush", 2 => "bone", 3 => "mushroom", _ => "tall_grass" }; p.n(x0 + u, y0 + 1, z0 + w, n); }
        for y in 1..=4 { p.n(x0 - 8, y0 + y, z0 + 5, "dead_log"); } p.n(x0 - 7, y0 + 4, z0 + 5, "dead_branch"); p.n(x0 - 8, y0 + 5, z0 + 5, "dead_branch");
        // le caveau : cinq sur quatre, bas, la porte béante vers l'allée
        for w in 3..=6 { for u in -2..=2i32 { let wall = u.abs() == 2 || w == 3 || w == 6; p.n(x0 + u, y0, z0 + w, "stone_bricks");
            for y in 1..=3 { if wall && !(w == 3 && u == 0 && y <= 2) { p.n(x0 + u, y0 + y, z0 + w, if r.next() < 0.25 { "mossy_stone" } else { "stone_bricks" }); } }
            p.n(x0 + u, y0 + 4, z0 + w, if wall { "stone_bricks_slab" } else { "stone_bricks" }); } }
        p.n(x0, y0 + 1, z0 + 5, "chest"); p.n(x0 - 1, y0 + 3, z0 + 5, "cobweb"); p.n(x0 + 1, y0 + 3, z0 + 4, "cobweb"); p.n(x0 + 1, y0 + 1, z0 + 5, "bone");
    }
    /// Le camp de brigands : une palissade ronde, deux tentes de chaume, le feu au milieu, la réserve au fond.
    fn build_camp(&self, p: &mut Plan, pl: &Place, r: &mut Rng) {
        let (x0, y0, z0) = (pl.x, pl.y, pl.z); let d = |u: i32, w: i32| (u as f64).hypot(w as f64); self.pad(p, pl, |u, w| d(u, w) < 9.0, 9, 9, 8, None);
        for w in -9..=9i32 { for u in -9..=9i32 { let dd = d(u, w); if !(7.5..8.5).contains(&dd) { continue; } if w < 0 && u.abs() <= 1 { continue; } if r.next() < 0.07 { continue; }
            let h = 2 + (hash_int(x0 + u, z0 + w, self.seed, 176) % 2) as i32; for y in 1..=h { p.n(x0 + u, y0 + y, z0 + w, "oak_log"); } } }
        // le sol battu autour du feu
        for w in -2..=2i32 { for u in -2..=2i32 { if d(u, w) < 2.6 && r.next() < 0.8 { p.n(x0 + u, y0, z0 + w, "path"); } } }
        p.n(x0, y0 + 1, z0, "campfire"); p.n(x0 - 2, y0 + 1, z0, "dead_log_z"); p.n(x0 + 2, y0 + 1, z0 - 1, "dead_log_z"); p.n(x0, y0 + 1, z0 + 2, "dead_log_x");
        // deux tentes : des pans de chaume appuyés l'un sur l'autre, une paillasse dessous
        for &u0 in &[-6, 3] { for w in -1..=2 {
            p.turn(x0 + u0, y0 + 1, z0 + w, "thatch_slope", 1, 0); p.turn(x0 + u0 + 3, y0 + 1, z0 + w, "thatch_slope", -1, 0); p.turn(x0 + u0 + 1, y0 + 2, z0 + w, "thatch_slope", 1, 0); p.turn(x0 + u0 + 2, y0 + 2, z0 + w, "thatch_slope", -1, 0); }
            p.n(x0 + u0 + 1, y0 + 1, z0 + 2, "thatch"); p.n(x0 + u0 + 2, y0 + 1, z0 + 2, "thatch"); p.n(x0 + u0 + 1, y0 + 1, z0, "bedroll"); p.n(x0 + u0 + 2, y0 + 1, z0 + 1, "bedroll"); }
        // la réserve : le coffre du butin, des tonneaux, une lanterne sur un pieu
        p.n(x0, y0 + 1, z0 + 5, "chest"); p.n(x0 + 1, y0 + 1, z0 + 5, "barrel"); p.n(x0 - 1, y0 + 1, z0 + 6, "barrel"); p.n(x0 - 1, y0 + 2, z0 + 6, "barrel"); p.n(x0 + 2, y0 + 1, z0 + 6, "fence"); p.n(x0 + 2, y0 + 2, z0 + 6, "lantern");
        // les pieux de l'entrée
        for u in [-2, 2] { for y in 1..=4 { p.n(x0 + u, y0 + y, z0 - 8, "oak_log"); } }
    }
}

impl crate::session::Session {
    /// Les gardiens d'un lieu sortent quand le joueur approche (une fois, puis de nouveau après un quart d'heure d'absence) ;
    /// les fantômes du cimetière ne se lèvent que la nuit.
    pub(crate) fn site_tick(&mut self) {
        if self.dead.is_some() { return; } let me = self.player.p; let night = self.sun_height() < 0.0;
        for pl in self.gen.places_near(me[0], me[2], 44.0) {
            for ghosts in [false, true] {
                if ghosts && !night { continue; } let key = (pl.cell, ghosts); if self.sites_awake.get(&key).is_some_and(|t| self.elapsed - t < 900.0) { self.sites_awake.insert(key, self.elapsed); continue; }
                let who: Vec<_> = pl.guards().into_iter().filter(|g| (g.0 == "fantome") == ghosts).collect(); if who.is_empty() { continue; }
                self.sites_awake.insert(key, self.elapsed); let yaw = (me[0] - pl.x as f64).atan2(me[2] - pl.z as f64);
                for (id, at) in who { if let Some(sp) = crate::hostile::species(id) { self.horde.put(sp, at, yaw, false); } }
                if !ghosts { self.notes.push(format!("{} en vue", pl.kind.name())); }
            }
        }
    }
    /// `/lieu [tour|cimetiere|brigands]` : va au lieu de surface le plus proche (de ce genre).
    pub fn goto_place(&mut self, arg: Option<&str>) -> String {
        let kind = match arg { None => None, Some(a) => match LIEUX.iter().find(|k| k.key().starts_with(a) || a.starts_with(k.key()) || (a == "camp" && **k == Lieu::Camp)) { Some(k) => Some(*k), None => return "Lieu inconnu. Au choix : tour, cimetiere, brigands".into() } };
        let p = self.player.p; let Some(pl) = self.gen.places_near(p[0], p[2], 2600.0).into_iter().find(|pl| kind.is_none_or(|k| k == pl.kind)) else { return "Aucun lieu de ce genre dans les parages".into() };
        let (at, yaw) = pl.view(); let y = self.gen.column(at[0], at[2]).h.max(pl.y as f64) + 1.5; self.player.p = [at[0], y, at[2]]; self.player.v = [0.0; 3]; self.player.yaw = yaw; self.player.pitch = 0.05;
        format!("{} à {}, {}, {}", pl.kind.name(), pl.x, pl.y, pl.z)
    }
}
