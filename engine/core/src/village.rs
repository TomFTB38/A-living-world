//! Villages : où ils se trouvent, comment ils sont bâtis, qui y vit et ce que l'on peut y demander.
//! Tout se déduit de la graine : une grille de cases, au plus un village par case, posé sur un terrain plat et sec.
//! Le type du village suit le biome, ses ateliers suivent artisans.txt. Les bâtiments sont posés après la génération
//! du terrain (`gen_world`), si bien que le terrain lui-même reste celui du prototype.
//! Les villageois suivent leur journée : à l'atelier ou aux champs le jour, sur la place le soir, chez eux la nuit.
use crate::actors::{b, Draw, Models};
use crate::biome::Biome;
use crate::blocks::{by_name, idx, CH, CX};
use crate::data::{db, def, Craftsman, Recipe, Station};
use crate::gen::WorldGen;
use crate::mesher::VBuf;
use crate::noise::hash_float;
use crate::session::{item, Session, Stack};
use crate::shapes::variant;
use crate::view::{mul, trs};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::f64::consts::{PI, TAU};
use std::rc::Rc;

/// Taille d'une case de la grille des villages, en blocs.
pub const CELL: i32 = 384;
/// Aucun village ne déborde de ce rayon autour de son centre, terrain aplani compris.
pub const SPAN: i32 = 64;
/// Distance où les villageois apparaissent, et où ils s'effacent.
const NEAR: f64 = 150.0; const FAR: f64 = 210.0;
/// Réputation demandée pour qu'un artisan transmette sa technique.
pub const TEACH_REP: i32 = 20;
const BLEND: i32 = 6;

#[derive(Clone, Debug)]
pub struct House {
    /// Le métier de l'atelier, ou `None` pour une maison.
    pub craft: Option<&'static str>,
    /// Coins (inclus) au sol, bords de toit compris : [x0, z0, x1, z1].
    pub area: [i32; 4],
    /// Devant la porte dehors, juste derrière la porte dedans, à l'établi, près du lit, et au bord de la place.
    pub door: [f64; 3], pub inside: [f64; 3], pub spot: [f64; 3], pub bed: [f64; 3], pub ring: [f64; 3],
    /// Sens vers le fond du bâtiment (0 : +z, 1 : +x, 2 : -z, 3 : -x) : l'artisan regarde son poste.
    pub back: u8,
}
#[derive(Clone, Debug)]
pub struct Village { pub id: (i32, i32), pub kind: &'static str, pub name: String, pub c: [i32; 3], pub r: i32, pub crafts: Vec<&'static str>, pub houses: Vec<House> }
/// Un village tracé, avec les blocs à poser.
pub struct Site { pub v: Village, blocks: Vec<(i32, i32, i32, u8)> }

pub fn kind_label(k: &str) -> &'static str { match k { "prairie" => "prairie", "taiga" => "taïga", "aride" => "steppe", "desert" => "désert", "montagne" => "montagne", _ => "hautes terres" } }
fn id(n: &str) -> u8 { by_name(n).unwrap_or_else(|| panic!("bloc inconnu : {n}")) }
fn dir(f: u8) -> (i32, i32) { [(0, 1), (1, 0), (0, -1), (-1, 0)][f as usize % 4] }
const NAME_A: [&str; 16] = ["Haut", "Bel", "Clair", "Roche", "Val", "Mont", "Pré", "Bois", "Fond", "Gué", "Saule", "Brune", "Vieux", "Grand", "Pierre", "Chêne"];
const NAME_B: [&str; 12] = ["val", "fort", "mont", "bourg", "lieu", "ville", "champ", "roche", "font", "court", "mesnil", "lande"];
const FIRST: [&str; 24] = ["Anselme", "Berthe", "Colin", "Denise", "Étienne", "Fanchon", "Gaspard", "Hélène", "Isidore", "Jeanne", "Léon", "Margot", "Nicolas", "Odile", "Pierre", "Rosalie", "Simon", "Thérèse", "Urbain", "Victor", "Agathe", "Basile", "Clémence", "Firmin"];

struct Mats { wall: u8, corner: u8, floor: u8, roof: u8, ridge: u8, found: u8 }
fn mats(kind: &str) -> Mats {
    let m = |w: &str, c: &str, f: &str, r: &str, g: &str, o: &str| Mats { wall: id(w), corner: id(c), floor: id(f), roof: id(r), ridge: id(g), found: id(o) };
    match kind {
        "taiga" => m("spruce_planks", "spruce_log", "spruce_planks", "planks_stairs", "planks", "cobblestone"),
        "aride" => m("cobblestone", "acacia_log", "planks", "thatch_stairs", "thatch", "cobblestone"),
        "desert" => m("sandstone", "red_sandstone", "sandstone", "thatch_stairs", "thatch", "sandstone"),
        "montagne" | "hautes_terres" => m("stone_bricks", "spruce_log", "planks", "cobblestone_stairs", "cobblestone", "cobblestone"),
        _ => m("planks", "oak_log", "planks", "thatch_stairs", "thatch", "cobblestone"),
    }
}

impl WorldGen {
    /// Le village de cette case de la grille, s'il y en a un.
    pub fn site(&self, gx: i32, gz: i32) -> Option<Rc<Site>> {
        if let Some(s) = self.towns.borrow().get(&(gx, gz)) { return s.clone(); }
        let s = self.trace(gx, gz).map(Rc::new); self.towns.borrow_mut().insert((gx, gz), s.clone()); s
    }
    /// Un chunk complet : le terrain du prototype, puis les villages qui le touchent.
    pub fn gen_world(&self, cx: i32, cz: i32) -> Vec<u8> {
        let mut v = self.gen_chunk(cx, cz); let (ox, oz) = (cx * CX as i32, cz * CX as i32);
        for gx in (ox - SPAN).div_euclid(CELL)..=(ox + CX as i32 + SPAN).div_euclid(CELL) { for gz in (oz - SPAN).div_euclid(CELL)..=(oz + CX as i32 + SPAN).div_euclid(CELL) {
            if let Some(s) = self.site(gx, gz) { s.stamp(self, cx, cz, &mut v); }
        } }
        v
    }
    /// Les villages dont le centre est à moins de `r` blocs de ce point, du plus proche au plus lointain.
    pub fn villages_near(&self, x: f64, z: f64, r: f64) -> Vec<Village> {
        let k = (r / CELL as f64).ceil() as i32 + 1; let (gx, gz) = ((x / CELL as f64).floor() as i32, (z / CELL as f64).floor() as i32);
        let mut o: Vec<Village> = Vec::new();
        for a in gx - k..=gx + k { for b in gz - k..=gz + k { if let Some(s) = self.site(a, b) { if (s.v.c[0] as f64 - x).hypot(s.v.c[2] as f64 - z) <= r { o.push(s.v.clone()); } } } }
        o.sort_by(|a, b| (a.c[0] as f64 - x).hypot(a.c[2] as f64 - z).total_cmp(&(b.c[0] as f64 - x).hypot(b.c[2] as f64 - z))); o
    }
    fn trace(&self, gx: i32, gz: i32) -> Option<Site> {
        let s = self.seed;
        if hash_float(gx, gz, s, 301) > 0.7 { return None; }
        for t in 0..10 {
            let x = ((gx as f64 + 0.22 + 0.56 * hash_float(gx, gz, s, 302 + t * 2)) * CELL as f64) as i32;
            let z = ((gz as f64 + 0.22 + 0.56 * hash_float(gx, gz, s, 303 + t * 2)) * CELL as f64) as i32;
            if let Some(y0) = self.flat(x, z) { return Some(self.build(gx, gz, x, y0, z)); }
        }
        None
    }
    /// Hauteur de la place si le terrain est sec et à peu près plat sur trente blocs alentour.
    fn flat(&self, x: i32, z: i32) -> Option<i32> {
        let mut tops = Vec::with_capacity(25);
        for (r, n) in [(0.0, 1), (10.0, 8), (19.0, 8), (27.0, 8)] { for k in 0..n {
            let a = k as f64 * TAU / n as f64 + r * 0.05; let c = self.column(x as f64 + r * a.cos(), z as f64 + r * a.sin());
            if c.water != 0.0 || c.cliff > 0.45 { return None; } tops.push(c.h.ceil() as i32 - 1);
        } }
        let (lo, hi) = (*tops.iter().min()?, *tops.iter().max()?); if hi - lo > 10 { return None; }
        tops.sort(); let y0 = tops[tops.len() / 2]; (y0 > 30 && y0 + 20 < CH as i32).then_some(y0)
    }
    fn build(&self, gx: i32, gz: i32, x: i32, y0: i32, z: i32) -> Site {
        let s = self.seed; let h = |k: i32| hash_float(gx, gz, s, 320 + k);
        let mix = self.climate.at(x as f64, z as f64); let top = (0..crate::biome::COUNT).max_by(|a, b| mix.w[*a].total_cmp(&mix.w[*b])).unwrap_or(1);
        // Montagne : un bourg haut perché, ou blotti au pied de sommets qui le dominent.
        let peaks = (0..16).map(|k| { let (a, r) = (k as f64 * TAU / 8.0, if k < 8 { 60.0 } else { 95.0 }); self.column(x as f64 + r * a.cos(), z as f64 + r * a.sin()).h }).fold(0.0, f64::max);
        let kind = if y0 > 76 || peaks - y0 as f64 > 24.0 { "montagne" } else if mix.strong(Biome::Highlands) > 0.4 { "hautes_terres" } else { match top { t if t == Biome::Taiga as usize => "taiga", t if t == Biome::Steppe as usize => "aride", t if t == Biome::Desert as usize => "desert", t if t == Biome::Highlands as usize => "hautes_terres", _ => "prairie" } };
        // Les ateliers : le forgeron dès que le type le permet, les autres une fois sur trois de moins ; au moins deux, au plus quatre.
        let cand: Vec<&Craftsman> = db().craftsmen.iter().filter(|c| c.villages.contains(&kind)).collect();
        let mut crafts: Vec<&'static str> = cand.iter().enumerate().filter(|(i, c)| c.id == "forgeron" || h(*i as i32) < 0.68).map(|(_, c)| c.id).collect();
        for c in &cand { if crafts.len() >= 2 { break; } if !crafts.contains(&c.id) { crafts.push(c.id); } }
        crafts.truncate(4);
        let homes = 2 + (h(10) * 3.0) as usize; let name = format!("{}{}", NAME_A[(h(11) * 16.0) as usize % 16], NAME_B[(h(12) * 12.0) as usize % 12]);
        let mut v = Village { id: (gx, gz), kind, name, c: [x, y0, z], r: 8, crafts: crafts.clone(), houses: Vec::new() };
        let m = mats(kind); let mut out: Vec<(i32, i32, i32, u8)> = Vec::new();
        // La place : un puits au milieu, des lanternes aux quatre coins.
        let path = id("path"); let cob = id("cobblestone"); let water = id("water"); let fence = id("fence"); let lantern = id("lantern");
        for dx in -5i32..=5 { for dz in -5i32..=5 { if dx * dx + dz * dz <= 26 { out.push((x + dx, y0, z + dz, path)); } } }
        for dx in -1..=1 { for dz in -1..=1 { let edge = dx != 0 || dz != 0; out.push((x + dx, y0, z + dz, if edge { cob } else { water })); out.push((x + dx, y0 - 1, z + dz, if edge { cob } else { water })); out.push((x + dx, y0 - 2, z + dz, cob)); if edge { out.push((x + dx, y0 + 1, z + dz, cob)); } } }
        for (dx, dz) in [(4, 4), (-4, 4), (4, -4), (-4, -4)] { out.push((x + dx, y0 + 1, z + dz, fence)); out.push((x + dx, y0 + 2, z + dz, lantern)); }
        let n = crafts.len() + homes; let a0 = h(13) * TAU; let mut taken: Vec<[i32; 4]> = Vec::new();
        for i in 0..n {
            let craft = crafts.get(i).copied(); let (w, d) = if craft.is_some() { (7, 7) } else { (5, 6) };
            let a = a0 + TAU * i as f64 / n as f64 + (h(20 + i as i32) - 0.5) * 0.4;
            let (ox, oz) = (a.cos(), a.sin()); let f = crate::shapes::facing_of(ox, oz); let (wx, wz) = dir(f); let (ux, uz) = dir(f + 1);
            let mut rad = 10.0 + n as f64 * 1.1; let mut placed = None;
            for _ in 0..8 {
                let (px, pz) = (x + (ox * (rad + d as f64 * 0.5)).round() as i32, z + (oz * (rad + d as f64 * 0.5)).round() as i32);
                let o = (px - (w - 1) / 2 * ux - (d - 1) / 2 * wx, pz - (w - 1) / 2 * uz - (d - 1) / 2 * wz);
                let cells = [(-1, -1), (w, -1), (-1, d), (w, d)].map(|(u, ww)| (o.0 + u * ux + ww * wx, o.1 + u * uz + ww * wz));
                let area = [cells.iter().map(|c| c.0).min().unwrap(), cells.iter().map(|c| c.1).min().unwrap(), cells.iter().map(|c| c.0).max().unwrap(), cells.iter().map(|c| c.1).max().unwrap()];
                let clash = taken.iter().any(|t| area[0] <= t[2] + 2 && area[2] + 2 >= t[0] && area[1] <= t[3] + 2 && area[3] + 2 >= t[1]);
                let far = cells.iter().map(|c| ((c.0 - x) as f64).hypot((c.1 - z) as f64)).fold(0.0, f64::max);
                let plaza = (0..=d).any(|ww| (0..=w).any(|u| { let c = (o.0 + u * ux + ww * wx, o.1 + u * uz + ww * wz); ((c.0 - x) as f64).hypot((c.1 - z) as f64) < 7.0 }));
                if !clash && !plaza && far + (BLEND as f64) + 2.0 < SPAN as f64 { placed = Some((o, area, far)); break; }
                rad += 3.0;
            }
            let Some((o, area, far)) = placed else { continue }; taken.push(area); v.r = v.r.max(far.ceil() as i32 + 2);
            let at = |u: i32, ww: i32| (o.0 + u * ux + ww * wx, o.1 + u * uz + ww * wz);
            let mid = |u: i32, ww: i32, y: i32| { let c = at(u, ww); [c.0 as f64 + 0.5, y as f64, c.1 as f64 + 0.5] };
            let mut put = |u: i32, y: i32, ww: i32, b: u8| { let c = at(u, ww); out.push((c.0, y, c.1, b)); };
            // Fondations, plancher, murs avec poteaux d'angle, intérieur vide jusque sous le faîte.
            for u in -1..=w { for ww in -1..=d { for y in y0 + 1..y0 + 4 + w / 2 + 2 { put(u, y, ww, 0); } } }
            for u in 0..w { for ww in 0..d { for y in y0 - 3..y0 { put(u, y, ww, m.found); } put(u, y0, ww, m.floor); } }
            for y in y0 + 1..=y0 + 3 { for u in 0..w { for ww in 0..d {
                let edge = u == 0 || ww == 0 || u == w - 1 || ww == d - 1; if !edge { continue; }
                let corner = (u == 0 || u == w - 1) && (ww == 0 || ww == d - 1); put(u, y, ww, if corner { m.corner } else { m.wall });
            } } }
            // Toit à deux pans, faîte du devant vers le fond ; pignons pleins.
            let yr = y0 + 4; let (mut k, mut lu, mut ru) = (0, -1, w);
            while lu <= ru {
                if lu == ru { for ww in -1..=d { put(lu, yr + k, ww, m.ridge); } break; }
                for ww in -1..=d { put(lu, yr + k, ww, variant(m.roof, (1 + f) % 4, false, false, false)); put(ru, yr + k, ww, variant(m.roof, (3 + f) % 4, false, false, false)); }
                for u in lu + 1..ru { put(u, yr + k, 0, m.wall); put(u, yr + k, d - 1, m.wall); }
                k += 1; lu += 1; ru -= 1;
            }
            // Porte au milieu de la façade, fenêtres sur les côtés et au fond.
            let du = w / 2; let door = id("door"); let window = id("window");
            put(du, y0 + 1, 0, variant(door, f, false, false, false)); put(du, y0 + 2, 0, variant(door, f, false, false, true));
            put(0, y0 + 2, d / 2, variant(window, (1 + f) % 4, false, false, false)); put(w - 1, y0 + 2, d / 2, variant(window, (1 + f) % 4, false, false, false));
            put(du, y0 + 2, d - 1, variant(window, f, false, false, false));
            if craft.is_some() { put(du - 2, y0 + 2, 0, variant(window, f, false, false, false)); put(du + 2, y0 + 2, 0, variant(window, f, false, false, false)); }
            // Le mobilier : le poste de l'artisan au fond, un lit, un coffre, une lanterne.
            let back = d - 2; put(1, y0 + 1, back, id("bed")); put(w - 2, y0 + 1, 1, lantern);
            match craft {
                Some("forgeron") | Some("maitre_forgeron") | Some("armurier") => { put(du, y0 + 1, back, id("anvil")); put(du + 1, y0 + 1, back, id("furnace")); }
                Some("tailleur_pierre") => { put(du, y0 + 1, back, id("mason_bench")); put(du + 1, y0 + 1, back, cob); }
                Some("tanneur") => { put(du, y0 + 1, back, id("workbench")); put(du + 1, y0 + 1, back, id("chest")); }
                Some(_) => { put(du, y0 + 1, back, id("workbench")); put(du + 1, y0 + 1, back, id("planks")); }
                None => { put(w - 2, y0 + 1, back, id("chest")); }
            }
            // Un chemin de terre de la place à la porte.
            let door_out = mid(du, -1, y0 + 1); let dist = (door_out[0] - x as f64 - 0.5).hypot(door_out[2] - z as f64 - 0.5);
            let (ex, ez) = ((door_out[0] - x as f64 - 0.5) / dist, (door_out[2] - z as f64 - 0.5) / dist); let ring = [x as f64 + 0.5 + ex * 6.5, (y0 + 1) as f64, z as f64 + 0.5 + ez * 6.5];
            let mut t = 5.0; while t < dist + 0.6 { let q = (x as f64 + 0.5 + ex * t, z as f64 + 0.5 + ez * t); out.push((q.0.floor() as i32, y0, q.1.floor() as i32, path)); t += 0.5; }
            out.push((door_out[0].floor() as i32, y0, door_out[2].floor() as i32, path));
            v.houses.push(House { craft, area, door: door_out, inside: mid(du, 1, y0 + 1), spot: mid(du, back - 1, y0 + 1), bed: mid(2, back, y0 + 1), ring, back: f });
        }
        // Les blocs du sol des maisons ne doivent pas recevoir de chemin : on repose les planchers en dernier.
        Site { v, blocks: out }
    }
}
impl Site {
    /// Pose ce village dans un chunk : le sol aplani d'abord (fondu vers le terrain sur les bords), puis les bâtiments.
    fn stamp(&self, gen: &WorldGen, cx: i32, cz: i32, v: &mut [u8]) {
        let (ox, oz) = (cx * CX as i32, cz * CX as i32); let c = self.v.c; let reach = self.v.r + BLEND;
        if c[0] + reach < ox || c[0] - reach >= ox + CX as i32 || c[2] + reach < oz || c[2] - reach >= oz + CX as i32 { return; }
        let dirt = id("dirt");
        for z in 0..CX { for x in 0..CX {
            let (wx, wz) = (ox + x as i32, oz + z as i32); let d = ((wx - c[0]) as f64).hypot((wz - c[2]) as f64); if d > reach as f64 { continue; }
            let nat = gen.column(wx as f64, wz as f64).h.ceil() as i32 - 1; if nat < 2 || nat >= CH as i32 - 2 { continue; }
            let t = ((d - self.v.r as f64) / BLEND as f64).clamp(0.0, 1.0); let goal = (c[1] as f64 + (nat - c[1]) as f64 * t).round() as i32;
            let surf = match v[idx(x, nat as usize, z)] { 0 => dirt, b => b }; let sub = match v[idx(x, (nat - 1) as usize, z)] { 0 => dirt, b => b };
            for y in nat.min(goal)..goal { v[idx(x, y as usize, z)] = sub; }
            v[idx(x, goal as usize, z)] = surf;
            for y in goal + 1..(nat.max(goal) + 22).min(CH as i32) { v[idx(x, y as usize, z)] = 0; }
        } }
        for &(x, y, z, b) in &self.blocks {
            if x < ox || z < oz || x >= ox + CX as i32 || z >= oz + CX as i32 || y < 0 || y >= CH as i32 { continue; }
            v[idx((x - ox) as usize, y as usize, (z - oz) as usize)] = b;
        }
    }
}

// ---------------------------------------------------------------------------------------------- villageois
#[derive(Clone, Debug)]
pub struct Folk { pub v: (i32, i32), pub house: usize, pub craft: Option<&'static str>, pub name: String, pub p: [f64; 3], pub yaw: f64, pub phase: f64, pub moving: bool, pub working: bool, route: Vec<[f64; 3]>, goal: (u8, u32), look: u8,
    /// Effrayé par une arme (secondes qui restent), délai avant de resaluer le joueur.
    pub scared: f64, greet_t: f64 }
#[derive(Clone, Copy, Debug, PartialEq)] pub enum Page { Main, Order(usize), Trade }
#[derive(Clone, Debug)] pub struct Talk { pub who: usize, pub page: Page, pub said: String }
/// Les villages autour du joueur, leurs habitants, et ce que le joueur en sait.
#[derive(Default)]
pub struct Town {
    pub here: Vec<Village>, pub folk: Vec<Folk>, pub talk: Option<Talk>, pub journal: bool,
    /// Réputation par village, villages connus (vus ou entendus) avec leur description, artisans rencontrés, village marqué.
    pub rep: HashMap<(i32, i32), i32>, pub atlas: BTreeMap<(i32, i32), Village>, pub known: Vec<(i32, i32)>, pub met: HashSet<((i32, i32), &'static str)>, pub marked: Option<(i32, i32)>,
    scan_t: f64,
    /// Ce que disent les villageois en passant (pour les notes), portes qu'ils ont ouvertes (et quand les refermer).
    pub(crate) news: Vec<String>, doors: Vec<([i32; 3], f64)>,
}
/// Une fenêtre de dialogue : qui parle, ce qu'il dit, et les réponses possibles (texte, possible maintenant).
#[derive(Clone, Debug)] pub struct Dialog { pub title: String, pub sub: String, pub said: String, pub options: Vec<(String, bool)> }

/// Les échanges que proposent les villageois de ce type de village.
pub fn trades(kind: &str) -> Vec<&'static crate::data::Trade> { db().trades.iter().filter(|t| t.kind == "tous" || t.kind == kind).collect() }
fn label(id: &str) -> String { item(id).map_or(id.to_string(), |i| i.label.to_lowercase()) }
pub fn craftsman(id: &str) -> Option<&'static Craftsman> { db().craftsmen.iter().find(|c| c.id == id) }
/// Ce qu'un artisan sait faire sur commande : les recettes de son métier (un maître fait aussi celles du métier qu'il porte dans son nom)
/// et les recettes ordinaires de ses travaux.
pub fn orders(c: &Craftsman) -> Vec<usize> {
    // Son propre métier d'abord, puis les ouvrages ordinaires, du plus avancé (le bas du fichier) au plus simple.
    let rs = &db().recipes; let mut own: Vec<usize> = (0..rs.len()).filter(|i| rs[*i].artisan.is_some_and(|a| c.id.ends_with(a))).collect();
    own.extend((0..rs.len()).rev().filter(|i| rs[*i].artisan.is_none() && rs[*i].at != Station::Hand && c.works.iter().any(|(st, cat)| *st == rs[*i].at && (cat.is_empty() || *cat == rs[*i].cat)))); own
}
fn dir_word(dx: f64, dz: f64) -> &'static str {
    // Le nord est vers -z, l'est vers +x.
    let a = dx.atan2(-dz); let k = ((a / (PI / 4.0)).round() as i32).rem_euclid(8) as usize; ["nord", "nord-est", "est", "sud-est", "sud", "sud-ouest", "ouest", "nord-ouest"][k]
}
pub fn where_is(from: [f64; 3], v: &Village) -> String { let (dx, dz) = (v.c[0] as f64 + 0.5 - from[0], v.c[2] as f64 + 0.5 - from[2]); format!("{:.0} m au {}", dx.hypot(dz), dir_word(dx, dz)) }
fn inside(h: &House, p: [f64; 3]) -> bool { p[0] > h.area[0] as f64 + 1.0 && p[0] < h.area[2] as f64 && p[2] > h.area[1] as f64 + 1.0 && p[2] < h.area[3] as f64 }

impl Town {
    fn village(&self, id: (i32, i32)) -> Option<&Village> { self.here.iter().find(|v| v.id == id).or_else(|| self.atlas.get(&id)) }
    /// Où ce villageois veut être à cette heure : (lieu, numéro qui change quand il doit repartir) et la position.
    fn want(&self, f: &Folk, k: usize, hour: f64) -> ((u8, u32), [f64; 3]) {
        let Some(v) = self.village(f.v) else { return ((0, 0), f.p) }; let h = &v.houses[f.house]; let y = v.c[1] as f64 + 1.0;
        let plaza = |salt: u32| { let a = hash_float(k as i32, salt as i32, v.id.0, v.id.1) * TAU; let r = 2.6 + 3.2 * hash_float(salt as i32, k as i32, v.id.1, 7); [v.c[0] as f64 + 0.5 + r * a.cos(), y, v.c[2] as f64 + 0.5 + r * a.sin()] };
        if !(6.5..21.5).contains(&hour) { return ((0, 0), h.bed); }
        if (8.0..18.0).contains(&hour) { return if f.craft.is_some() { ((1, 0), h.spot) } else { let s = (hour * 0.75) as u32; ((2, s), plaza(s + 40)) }; }
        ((3, 0), plaza(k as u32 + 3))
    }
    fn route(&self, f: &Folk, to: [f64; 3], dest_house: Option<usize>) -> Vec<[f64; 3]> {
        let Some(v) = self.village(f.v) else { return vec![to] }; let mut r = Vec::new();
        if let Some(h) = v.houses.iter().find(|h| inside(h, f.p)) { r.push(h.inside); r.push(h.door); r.push(h.ring); }
        match dest_house { Some(i) if !inside(&v.houses[i], f.p) => { let h = &v.houses[i]; r.push(h.ring); r.push(h.door); r.push(h.inside); } _ => {} }
        if let Some(i) = dest_house { if inside(&v.houses[i], f.p) { r.clear(); } }
        r.push(to);
        // Faire le tour du puits plutôt que le traverser.
        let c = [v.c[0] as f64 + 0.5, v.c[2] as f64 + 0.5]; let mut o = Vec::new(); let mut prev = f.p;
        for q in r {
            let (ax, az, bx, bz) = (prev[0] - c[0], prev[2] - c[1], q[0] - c[0], q[2] - c[1]); let (dx, dz) = (bx - ax, bz - az); let l2 = (dx * dx + dz * dz).max(1e-9);
            let t = (-(ax * dx + az * dz) / l2).clamp(0.0, 1.0);
            if (ax + dx * t).hypot(az + dz * t) < 2.4 { let (mut mx, mut mz) = ((ax + bx) * 0.5, (az + bz) * 0.5); if mx.hypot(mz) < 0.1 { (mx, mz) = (-az, ax); } let l = mx.hypot(mz).max(1e-6); o.push([c[0] + mx / l * 3.4, q[1], c[1] + mz / l * 3.4]); }
            o.push(q); prev = q;
        }
        o
    }
    /// La journée des villageois : un pas le long de leur chemin, ou leur travail sur place.
    pub fn update(&mut self, dt: f64, hour: f64, me: [f64; 3], talking: Option<usize>, armed: bool) {
        for k in 0..self.folk.len() {
            let mut f = self.folk[k].clone(); let near = (f.p[0] - me[0]).hypot(f.p[2] - me[2]); f.greet_t -= dt; f.scared -= dt;
            // Une arme tirée tout près : on recule et on rentre chez soi. Sinon, on salue celui qui passe.
            if armed && near < 2.4 && f.scared <= 0.0 && talking != Some(k) { f.scared = 7.0; self.news.push(format!("{} recule : « Rangez cette arme ! »", f.name)); }
            else if !armed && near < 3.0 && f.greet_t <= 0.0 && f.scared <= 0.0 && talking != Some(k) { f.greet_t = 150.0; self.news.push(format!("{} : « {} ! »", f.name, if hour >= 18.0 || hour < 5.0 { "Bonsoir" } else { "Bonjour" })); }
            let (goal, at) = if f.scared > 0.0 { let Some(v) = self.village(f.v) else { continue }; ((5, 0), v.houses[f.house].inside) } else { self.want(&f, k, hour) };
            let dest = if goal.0 <= 1 || goal.0 == 5 { Some(f.house) } else { None };
            if goal != f.goal { f.route = self.route(&f, at, dest); f.goal = goal; }
            if talking == Some(k) { f.moving = false; f.working = false; f.yaw = (me[0] - f.p[0]).atan2(me[2] - f.p[2]); self.folk[k] = f; continue; }
            f.working = false;
            if let Some(q) = f.route.first().copied() {
                let (dx, dz) = (q[0] - f.p[0], q[2] - f.p[2]); let d = dx.hypot(dz); let step = if f.scared > 0.0 { 2.6 } else { 1.35 } * dt;
                if d <= step { f.p = q; f.route.remove(0); } else { f.p = [f.p[0] + dx / d * step, q[1], f.p[2] + dz / d * step]; f.yaw = dx.atan2(dz); }
                f.moving = true; f.phase += dt * 7.0;
            } else {
                f.moving = false;
                if goal.0 == 1 { let Some(v) = self.village(f.v) else { continue }; let (bx, bz) = dir(v.houses[f.house].back); f.yaw = (bx as f64).atan2(bz as f64); f.working = true; f.phase += dt * 5.0; }
                // Arrêté près du joueur, on se tourne vers lui.
                if !f.working && near < 4.0 && f.scared <= 0.0 { f.yaw = (me[0] - f.p[0]).atan2(me[2] - f.p[2]); }
            }
            self.folk[k] = f;
        }
    }
    fn spawn(&mut self, v: &Village, hour: f64) {
        let mut n = 0u32; let base = self.folk.len();
        for (i, h) in v.houses.iter().enumerate() {
            let count = if h.craft.is_some() { 1 } else { 1 + (hash_float(v.id.0, v.id.1, i as i32, 91) < 0.6) as usize };
            for j in 0..count {
                let pick = (hash_float(v.id.0 * 7 + i as i32, v.id.1, j as i32, 92) * FIRST.len() as f64) as usize % FIRST.len();
                let look = if let Some(c) = h.craft { db().craftsmen.iter().position(|x| x.id == c).unwrap_or(0) as u8 } else { 6 + (hash_float(i as i32, j as i32, v.id.0, v.id.1) * 3.0) as u8 % 3 };
                self.folk.push(Folk { v: v.id, house: i, craft: if j == 0 { h.craft } else { None }, name: FIRST[pick].to_string(), p: h.spot, yaw: 0.0, phase: n as f64, moving: false, working: false, route: Vec::new(), goal: (9, 0), look, scared: 0.0, greet_t: 0.0 });
                n += 1;
            }
        }
        // Chacun commence là où sa journée le veut.
        for k in base..self.folk.len() { let (g, at) = self.want(&self.folk[k], k, hour); self.folk[k].p = at; self.folk[k].goal = g; }
    }
    pub fn draws(&self, md: &Models) -> Vec<Draw> {
        let mut out = Vec::with_capacity(self.folk.len() * 6);
        for f in &self.folk {
            let base = trs(f.p[0], f.p[1], f.p[2], f.yaw, 0.0, 1.0); let mut put = |n: &str, m| out.push(Draw { m, r: md.part(n), view: false, local: false, hero: false, dynamic: false });
            let s = if f.moving { f.phase.sin() * 0.6 } else { 0.0 };
            put(&format!("folk{}", f.look.min(8)), base); put(&format!("folkHead{}", f.look % 3), mul(&base, &trs(0.0, 1.36, 0.0, 0.0, 0.0, 1.0)));
            put("folkLeg", mul(&base, &trs(-0.1, 0.74, 0.0, 0.0, s, 1.0))); put("folkLeg", mul(&base, &trs(0.1, 0.74, 0.0, 0.0, -s, 1.0)));
            let work = if f.working { -1.2 - 0.9 * (f.phase * 1.4).sin().max(0.0) } else { -s };
            put(&format!("folkArm{}", f.look.min(8)), mul(&base, &trs(-0.29, 1.32, 0.0, 0.0, s, 1.0))); put(&format!("folkArm{}", f.look.min(8)), mul(&base, &trs(0.29, 1.32, 0.0, 0.0, work, 1.0)));
        }
        out
    }
}
/// Corps des villageois : une tunique par métier (dans l'ordre d'artisans.txt), trois pour les autres ; trois chevelures.
pub fn parts() -> Vec<(&'static str, VBuf)> {
    const TUNIC: [[f64; 3]; 9] = [[0.36, 0.27, 0.2], [0.2, 0.17, 0.17], [0.38, 0.42, 0.48], [0.3, 0.42, 0.24], [0.52, 0.52, 0.5], [0.62, 0.45, 0.24], [0.28, 0.36, 0.55], [0.6, 0.25, 0.2], [0.72, 0.66, 0.5]];
    const NAMES: [&str; 9] = ["folk0", "folk1", "folk2", "folk3", "folk4", "folk5", "folk6", "folk7", "folk8"];
    const ARMS: [&str; 9] = ["folkArm0", "folkArm1", "folkArm2", "folkArm3", "folkArm4", "folkArm5", "folkArm6", "folkArm7", "folkArm8"];
    let (skin, pants, belt, apron) = ([0.86, 0.68, 0.55], [0.3, 0.26, 0.22], [0.2, 0.14, 0.09], [0.42, 0.3, 0.2]);
    let mut o = Vec::new();
    for (i, t) in TUNIC.iter().enumerate() {
        let mut body = VBuf::new(60); b(&mut body, [-0.22, 0.74, -0.13, 0.22, 1.36, 0.13], *t); b(&mut body, [-0.225, 0.86, -0.135, 0.225, 0.92, 0.135], belt);
        if i < 3 { b(&mut body, [-0.18, 0.62, 0.13, 0.18, 1.2, 0.16], apron); } // tablier de cuir des forgerons et de l'armurier
        o.push((NAMES[i], body));
        let mut arm = VBuf::new(60); b(&mut arm, [-0.07, -0.46, -0.07, 0.07, 0.04, 0.07], *t); b(&mut arm, [-0.065, -0.6, -0.065, 0.065, -0.46, 0.065], skin); o.push((ARMS[i], arm));
    }
    for (n, hair) in [("folkHead0", [0.3, 0.2, 0.12]), ("folkHead1", [0.12, 0.1, 0.08]), ("folkHead2", [0.7, 0.58, 0.34])] {
        let mut h = VBuf::new(60); b(&mut h, [-0.16, 0.0, -0.16, 0.16, 0.32, 0.16], skin); b(&mut h, [-0.17, 0.24, -0.17, 0.17, 0.36, 0.17], hair); b(&mut h, [-0.17, 0.08, -0.17, 0.17, 0.26, -0.08], hair);
        b(&mut h, [-0.09, 0.15, 0.16, -0.04, 0.2, 0.17], [0.1, 0.1, 0.12]); b(&mut h, [0.04, 0.15, 0.16, 0.09, 0.2, 0.17], [0.1, 0.1, 0.12]); o.push((n, h));
    }
    let mut leg = VBuf::new(60); b(&mut leg, [-0.08, -0.62, -0.08, 0.08, 0.0, 0.08], pants); b(&mut leg, [-0.085, -0.74, -0.09, 0.085, -0.62, 0.11], belt); o.push(("folkLeg", leg));
    o
}

// ---------------------------------------------------------------------------------------------- la partie
impl Session {
    /// Les villages proches se peuplent, les lointains s'effacent ; on découvre ceux qu'on approche.
    pub(crate) fn town_tick(&mut self, dt: f64) {
        let me = self.player.p; self.town.scan_t -= dt;
        if self.town.scan_t <= 0.0 {
            self.town.scan_t = 1.5;
            let gone: Vec<(i32, i32)> = self.town.here.iter().filter(|v| (v.c[0] as f64 - me[0]).hypot(v.c[2] as f64 - me[2]) > FAR).map(|v| v.id).collect();
            if !gone.is_empty() { self.town.here.retain(|v| !gone.contains(&v.id)); self.town.folk.retain(|f| !gone.contains(&f.v)); self.town.talk = None; }
            for v in self.gen.villages_near(me[0], me[2], NEAR) {
                if self.town.here.iter().any(|h| h.id == v.id) { continue; }
                self.town.spawn(&v, self.hour); self.town.here.push(v);
            }
            for v in self.town.here.clone() { if (v.c[0] as f64 - me[0]).hypot(v.c[2] as f64 - me[2]) < v.r as f64 + 16.0 && !self.town.known.contains(&v.id) {
                self.notes.push(format!("Vous arrivez à {}", v.name)); self.discover(&v);
            } }
        }
        let talking = self.town.talk.as_ref().map(|t| t.who); let armed = self.armed();
        self.town.update(dt, self.hour, me, talking, armed); self.notes.append(&mut self.town.news); self.folk_doors(dt);
        if let Some(t) = &self.town.talk { if self.town.folk.get(t.who).is_none_or(|f| (f.p[0] - me[0]).hypot(f.p[2] - me[2]) > 6.0) { self.town.talk = None; } }
    }
    /// Une arme en main : épée, lance ou arc.
    pub fn armed(&self) -> bool { self.inv[self.sel].is_some_and(|st| st.item == "bow" || def(st.item).and_then(|d| d.tool).is_some_and(|t| t.kind == "epee" || t.kind == "lance")) }
    /// Les villageois ouvrent les portes devant eux et les referment derrière eux.
    fn folk_doors(&mut self, dt: f64) {
        use crate::player::Voxels; use crate::shapes::{info, Kind};
        let mut open = Vec::new();
        for f in &self.town.folk { if !f.moving { continue; } let Some(q) = f.route.first() else { continue }; let (dx, dz) = (q[0] - f.p[0], q[2] - f.p[2]); let d = dx.hypot(dz).max(1e-6);
            for t in [0.0, 0.7] { let c = [(f.p[0] + dx / d * t).floor() as i32, f.p[1].floor() as i32, (f.p[2] + dz / d * t).floor() as i32]; if let Some(i) = self.world.get(c[0], c[1], c[2]).and_then(info).filter(|i| i.kind == Kind::Door) { if !i.open { open.push((c, i)); } else if let Some(e) = self.town.doors.iter_mut().find(|e| e.0 == c) { e.1 = 1.5; } } } }
        for (c, i) in open { self.set_door(c, i, true); self.town.doors.retain(|e| e.0 != c); self.town.doors.push((c, 1.5)); }
        let me = self.player.p; let mut close = Vec::new();
        for e in self.town.doors.iter_mut() { e.1 -= dt; let busy = self.town.folk.iter().any(|f| (f.p[0] - e.0[0] as f64 - 0.5).hypot(f.p[2] - e.0[2] as f64 - 0.5) < 1.2) || (me[0] - e.0[0] as f64 - 0.5).hypot(me[2] - e.0[2] as f64 - 0.5) < 1.1; if busy { e.1 = e.1.max(0.5); } if e.1 <= 0.0 { close.push(e.0); } }
        self.town.doors.retain(|e| e.1 > 0.0);
        for c in close { if let Some(i) = self.world.get(c[0], c[1], c[2]).and_then(info).filter(|i| i.kind == Kind::Door && i.open) { self.set_door(c, i, false); } }
    }
    fn set_door(&mut self, c: [i32; 3], i: crate::shapes::Info, open: bool) {
        use crate::player::Voxels; let low = if i.upper { [c[0], c[1] - 1, c[2]] } else { c }; let mut e = vec![(low[0], low[1], low[2], variant(i.base, i.facing, false, open, false))];
        if self.world.get(low[0], low[1] + 1, low[2]).and_then(crate::shapes::info).is_some_and(|u| u.kind == crate::shapes::Kind::Door) { e.push((low[0], low[1] + 1, low[2], variant(i.base, i.facing, false, open, true))); }
        self.world.edit_many(&e); self.sound.place("planks");
    }
    fn discover(&mut self, v: &Village) { if !self.town.known.contains(&v.id) { self.town.known.push(v.id); } self.town.atlas.insert(v.id, v.clone()); self.dirty = true; }
    /// Le villageois sous le réticule, à portée de voix et devant le bloc visé.
    pub fn aim_folk(&self) -> Option<usize> {
        let (o, d) = (self.eye, self.look); let block_t = if self.target.is_some() { self.ray_t } else { f64::INFINITY }; let head = self.player.eye();
        let mut best: Option<(f64, usize)> = None;
        for (k, f) in self.town.folk.iter().enumerate() {
            if let Some(t) = crate::survival::ray_box(o, d, f.p, 0.32, 1.75) { let q = [o[0] + d[0] * t, o[1] + d[1] * t, o[2] + d[2] * t]; let r = ((q[0] - head[0]).powi(2) + (q[1] - head[1]).powi(2) + (q[2] - head[2]).powi(2)).sqrt();
                if t < block_t + 0.3 && r <= 4.0 && best.is_none_or(|b| t < b.0) { best = Some((t, k)); } }
        }
        best.map(|b| b.1)
    }
    /// Clic droit sur un villageois : on engage la conversation.
    pub(crate) fn greet(&mut self) -> bool {
        let Some(k) = self.aim_folk() else { return false }; self.talk_to(k); true
    }
    pub fn talk_to(&mut self, k: usize) {
        let Some(f) = self.town.folk.get(k).cloned() else { return };
        if let Some(v) = self.town.village(f.v).cloned() { self.discover(&v); }
        if let Some(c) = f.craft { self.town.met.insert((f.v, c)); }
        let said = match f.craft { Some(c) => format!("Bonjour ! Je suis {}, {}. Que puis-je pour vous ?", f.name, craftsman(c).map_or(c, |x| x.name).to_lowercase()), None => format!("Bonjour, je m'appelle {}. Bienvenue chez nous.", f.name) };
        self.town.talk = Some(Talk { who: k, page: Page::Main, said }); self.inv_open = false; self.dirty = true;
    }
    pub fn close_talk(&mut self) { self.town.talk = None; }
    pub fn rep(&self, v: (i32, i32)) -> i32 { self.town.rep.get(&v).copied().unwrap_or(0) }
    fn gain_rep(&mut self, v: (i32, i32), n: i32) { let r = self.town.rep.entry(v).or_insert(0); *r = (*r + n).clamp(-50, 100); self.dirty = true; }
    /// Le paiement qu'on peut fournir pour un service : la première chose acceptée que l'on a en assez grande quantité.
    pub fn price(&self, c: &Craftsman, times: u32) -> Result<(&'static str, u32), String> {
        if self.creative { return Ok(("", 0)); }
        c.price.iter().find(|(id, n)| self.count(id) >= n * times).map(|(id, n)| (*id, n * times)).ok_or_else(|| format!("Il demande {}", c.price.iter().map(|(id, n)| format!("{} {}", n * times, item(id).map_or(*id, |i| i.label).to_lowercase())).collect::<Vec<_>>().join(" ou ")))
    }
    fn pay(&mut self, c: &Craftsman, times: u32) -> Result<(), String> { let (id, n) = self.price(c, times)?; if n > 0 { self.take(id, n); } Ok(()) }
    /// Ce que coûte une commande en matériaux : les ingrédients, sauf les outils (l'artisan a les siens).
    pub fn order_needs(r: &Recipe) -> Vec<(&'static str, u32)> { r.need.iter().filter(|n| !n.keep).map(|n| (n.id, n.n)).collect() }
    fn has_all(&self, needs: &[(&'static str, u32)]) -> bool { self.creative || needs.iter().all(|(id, n)| self.count(id) >= *n) }
    /// L'objet en main, et la recette de cet artisan qui en fait la version supérieure (même outil ou même pièce d'habit).
    pub fn upgrade_of(&self, c: &Craftsman) -> Option<(usize, Stack)> {
        let st = self.inv[self.sel]?; let d = def(st.item)?;
        let better = |r: &Recipe| { let o = def(r.out); match (d.tool, o.and_then(|o| o.tool), d.wear, o.and_then(|o| o.wear)) { (Some(a), Some(b), _, _) => a.kind == b.kind && b.mat.tier > a.mat.tier, (_, _, Some(a), Some(b)) => a.slot == b.slot && b.armor > a.armor, _ => false } };
        orders(c).into_iter().filter(|i| better(&db().recipes[*i])).min_by_key(|i| def(db().recipes[*i].out).and_then(|o| o.tool).map_or(0, |t| t.mat.tier)).map(|i| (i, st))
    }
    /// Une amélioration coûte la moitié des matériaux, sans le manche : on garde l'objet de départ.
    pub fn upgrade_needs(r: &Recipe) -> Vec<(&'static str, u32)> { Self::order_needs(r).into_iter().filter(|(id, _)| *id != "stick").map(|(id, n)| (id, n.div_ceil(2))).collect() }
    fn needs_text(n: &[(&'static str, u32)]) -> String { n.iter().map(|(id, k)| format!("{} {}", k, crate::survival::group_label(id).or(item(id).map(|i| i.label)).unwrap_or(id).to_lowercase())).collect::<Vec<_>>().join(", ") }

    /// La fenêtre de dialogue ouverte, avec ses réponses.
    pub fn dialog(&self) -> Option<Dialog> {
        let t = self.town.talk.as_ref()?; let f = self.town.folk.get(t.who)?; let v = self.town.village(f.v)?;
        let cm = f.craft.and_then(craftsman); let title = match cm { Some(c) => format!("{} · {}", f.name, c.name), None => format!("{} · villageois", f.name) };
        let sub = format!("{} · village de {} · réputation {}", v.name, kind_label(v.kind), self.rep(v.id)); let mut o: Vec<(String, bool)> = Vec::new();
        if self.armed() { o.push(("Ranger mon arme".into(), true)); o.push(("Au revoir".into(), true)); return Some(Dialog { title, sub, said: "Rangez d'abord votre arme, je vous prie.".into(), options: o }); }
        match (t.page, cm) {
            (Page::Trade, _) => {
                for tr in trades(v.kind) { let have = self.creative || self.count(tr.give.0) >= tr.give.1; o.push((format!("Donner {} {} contre {} {}", tr.give.1, label(tr.give.0), tr.get.1, label(tr.get.0)), have)); }
                o.push(("Retour".into(), true));
            }
            (Page::Order(page), Some(c)) => {
                let list = orders(c);
                for i in list.iter().skip(page * 7).take(7) { let r = &db().recipes[*i]; let n = Self::order_needs(r); o.push((format!("{} x{} · {}", item(r.out).map_or(r.out, |x| x.label), r.n, Self::needs_text(&n)), self.has_all(&n) && self.price(c, 1).is_ok())); }
                if list.len() > (page + 1) * 7 { o.push(("Autres travaux...".into(), true)); }
                o.push(("Retour".into(), true));
            }
            _ => {
                if let Some(c) = cm {
                    let s = |x: &str| c.services.contains(&x);
                    if s("commande") { o.push(("Passer commande (avec vos matériaux)".into(), true)); }
                    if s("amelioration") { match self.upgrade_of(c) { Some((i, st)) => { let r = &db().recipes[i]; o.push((format!("Améliorer {} en {} · {}", item(st.item).map_or(st.item, |x| x.label).to_lowercase(), item(r.out).map_or(r.out, |x| x.label).to_lowercase(), Self::needs_text(&Self::upgrade_needs(r))), self.has_all(&Self::upgrade_needs(r)) && self.price(c, 1).is_ok())); } None => o.push(("Améliorer l'objet en main (rien à améliorer)".into(), false)) } }
                    if s("reparation") { let st = self.inv[self.sel].filter(|st| st.wear > 0 && def(st.item).is_some_and(|d| d.tool.is_some() || d.wear.is_some())); o.push((match st { Some(st) => format!("Réparer {}", item(st.item).map_or(st.item, |x| x.label).to_lowercase()), None => "Réparer l'objet en main (rien à réparer)".into() }, st.is_some() && self.price(c, 1).is_ok())); }
                    if s("transmission") { let done = self.taught.contains(c.id); o.push((if done { "Technique déjà apprise".into() } else { format!("Apprendre la technique (réputation {TEACH_REP}, prix triple)") }, !done && (self.creative || self.rep(v.id) >= TEACH_REP) && self.price(c, 3).is_ok())); }
                }
                if cm.is_none() { o.push(("Échanger des marchandises".into(), !trades(v.kind).is_empty())); }
                let food = self.inv[self.sel].filter(|st| crate::survival::food(st.item).is_some());
                o.push((match food { Some(st) => format!("Offrir {}", item(st.item).map_or(st.item, |x| x.label).to_lowercase()), None => "Offrir de quoi manger (rien en main)".into() }, food.is_some()));
                o.push(("Des nouvelles d'autres villages ?".into(), true));
                o.push((if self.town.marked == Some(v.id) { "Ne plus marquer ce village".into() } else { "Marquer ce village sur ma route".into() }, true));
                o.push(("Au revoir".into(), true));
            }
        }
        Some(Dialog { title, sub, said: t.said.clone(), options: o })
    }
    /// Choisir la réponse `i` (à partir de 0) de la fenêtre de dialogue.
    pub fn choose(&mut self, i: usize) {
        let Some(d) = self.dialog() else { return }; let Some((label, ok)) = d.options.get(i).cloned() else { return };
        let Some(t) = self.town.talk.clone() else { return }; let f = self.town.folk[t.who].clone(); let cm = f.craft.and_then(craftsman); let vid = f.v;
        let say = |s: &mut Session, m: String| { if let Some(t) = s.town.talk.as_mut() { t.said = m; } };
        if !ok { if let Some(c) = cm { if let Err(e) = self.price(c, if label.starts_with("Apprendre") { 3 } else { 1 }) { say(self, format!("{e}, en paiement.")); return; } } say(self, "Je ne peux rien faire pour ça.".into()); return; }
        if label.starts_with("Ranger mon arme") {
            // On prend en main une case vide de la barre, sinon une case sans arme.
            let calm = |s: &Session, i: usize| s.inv[i].is_none_or(|st| st.item != "bow" && def(st.item).and_then(|d| d.tool).is_none_or(|t| t.kind != "epee" && t.kind != "lance"));
            if let Some(i) = (0..9).find(|i| self.inv[*i].is_none()).or((0..9).find(|i| calm(self, *i))) { self.sel = i; }
            let m = if f.craft.is_some() { "Merci. Que puis-je pour vous ?" } else { "Merci, c'est mieux ainsi." }; say(self, m.into()); return;
        }
        if label.starts_with("Au revoir") { self.town.talk = None; return; }
        match t.page {
            Page::Trade => {
                let Some(v) = self.town.village(vid).cloned() else { return }; let list = trades(v.kind);
                if let Some(tr) = list.get(i) { if !self.creative { self.take(tr.give.0, tr.give.1); } let left = self.add_item(tr.get.0, tr.get.1); if left > 0 { self.notes.push("Sac plein".into()); } self.seen.insert(tr.get.0); self.gain_rep(vid, 1); say(self, "Marché conclu !".into()); }
                else if let Some(t) = self.town.talk.as_mut() { t.page = Page::Main; t.said = "Autre chose ?".into(); }
            }
            Page::Order(page) => {
                let Some(c) = cm else { return }; let list = orders(c); let shown: Vec<usize> = list.iter().skip(page * 7).take(7).copied().collect();
                if i < shown.len() {
                    let r = db().recipes[shown[i]].clone(); let needs = Self::order_needs(&r);
                    if let Err(e) = self.pay(c, 1) { say(self, e); return; }
                    if !self.creative { for (id, n) in &needs { self.take(id, *n); } }
                    let left = self.add_item(r.out, r.n); if left > 0 { self.notes.push("Sac plein : la commande est posée à vos pieds".into()); }
                    self.seen.insert(r.out); self.gain_rep(vid, 4); self.pass_hours(1.0); self.sound.place("anvil");
                    say(self, format!("Voilà : {}. Une heure de travail, et du solide.", item(r.out).map_or(r.out, |x| x.label).to_lowercase()));
                } else if label.starts_with("Autres") { self.town.talk.as_mut().unwrap().page = Page::Order(page + 1); }
                else if let Some(t) = self.town.talk.as_mut() { t.page = Page::Main; }
            }
            Page::Main => {
                if label.starts_with("Échanger") { if let Some(t) = self.town.talk.as_mut() { t.page = Page::Trade; t.said = "Voyons ce que vous avez. Voilà ce que je peux vous donner.".into(); } }
                else if label.starts_with("Passer commande") { if let Some(t) = self.town.talk.as_mut() { t.page = Page::Order(0); t.said = "Dites-moi ce qu'il vous faut. Fournissez la matière, je fournis le savoir-faire.".into(); } }
                else if label.starts_with("Améliorer") {
                    let Some(c) = cm else { return }; let Some((ri, st)) = self.upgrade_of(c) else { return }; let r = db().recipes[ri].clone(); let needs = Self::upgrade_needs(&r);
                    if let Err(e) = self.pay(c, 1) { say(self, e); return; }
                    if !self.creative { for (id, n) in &needs { self.take(id, *n); } }
                    self.inv[self.sel] = Some(Stack::new(r.out, if crate::survival::stackable(r.out) { Some(1) } else { None })); self.seen.insert(r.out); self.gain_rep(vid, 4); self.pass_hours(1.0); self.dirty = true;
                    say(self, format!("Votre {} est devenu {}.", item(st.item).map_or(st.item, |x| x.label).to_lowercase(), item(r.out).map_or(r.out, |x| x.label).to_lowercase()));
                }
                else if label.starts_with("Réparer") { let Some(c) = cm else { return }; if let Err(e) = self.pay(c, 1) { say(self, e); return; } if let Some(st) = self.inv[self.sel].as_mut() { st.wear = 0; } self.gain_rep(vid, 2); self.dirty = true; say(self, "Comme neuf. Prenez-en soin.".into()); }
                else if label.starts_with("Apprendre") { let Some(c) = cm else { return }; if let Err(e) = self.pay(c, 3) { say(self, e); return; } self.learn(c.id); self.gain_rep(vid, 3); say(self, format!("Je vous montre. Vous pourrez faire mes ouvrages vous-même, à {}.", c.works.first().map_or("l'atelier", |w| w.0.name()))); }
                else if label.starts_with("Offrir") { self.consume_held_one(); self.gain_rep(vid, 3); say(self, "Merci, c'est bien aimable !".into()); }
                else if label.starts_with("Des nouvelles") { let m = self.rumor(vid); say(self, m); }
                else if label.starts_with("Marquer") { self.town.marked = Some(vid); self.dirty = true; say(self, "Bon retour, et revenez nous voir.".into()); }
                else if label.starts_with("Ne plus marquer") { self.town.marked = None; self.dirty = true; say(self, "Comme vous voudrez.".into()); }
                else { self.town.talk = None; }
            }
        }
    }
    fn consume_held_one(&mut self) { if let Some(st) = self.inv[self.sel] { let n = st.n.unwrap_or(1).saturating_sub(1); self.inv[self.sel] = if n == 0 { None } else { Some(Stack { n: Some(n), ..st }) }; self.dirty = true; } }
    /// Une rumeur : le village inconnu le plus proche, de préférence avec un artisan qui manque ici.
    fn rumor(&mut self, from: (i32, i32)) -> String {
        let Some(here) = self.town.village(from).cloned() else { return "Je ne sais pas.".into() };
        let mut all = self.gen.villages_near(here.c[0] as f64, here.c[2] as f64, CELL as f64 * 3.5); all.retain(|v| v.id != from && !self.town.known.contains(&v.id));
        let pick = all.iter().find(|v| v.crafts.iter().any(|c| !here.crafts.contains(c))).or(all.first()).cloned();
        let Some(v) = pick else { return "Je ne connais pas d'autre village que ceux dont vous avez entendu parler.".into() };
        self.discover(&v); let who: Vec<String> = v.crafts.iter().filter_map(|c| craftsman(c)).map(|c| c.name.to_lowercase()).collect();
        format!("On parle de {}, {} d'ici. Il y a là-bas {}.", v.name, where_is([here.c[0] as f64, 0.0, here.c[2] as f64], &v), who.join(", "))
    }
    /// Le village marqué : son nom et la position de sa place, pour le repère à l'écran.
    pub fn marked_village(&self) -> Option<&Village> { self.town.marked.and_then(|id| self.town.village(id)) }
    /// Le journal : les villages connus, du plus proche au plus lointain, avec leurs artisans (rencontrés ou non).
    pub fn journal(&self) -> Vec<(Village, String, Vec<(&'static str, bool)>)> {
        let me = self.player.p; let mut l: Vec<Village> = self.town.known.iter().filter_map(|id| self.town.village(*id).cloned()).collect();
        l.sort_by(|a, b| (a.c[0] as f64 - me[0]).hypot(a.c[2] as f64 - me[2]).total_cmp(&(b.c[0] as f64 - me[0]).hypot(b.c[2] as f64 - me[2])));
        l.into_iter().map(|v| { let w = where_is(me, &v); let c = v.crafts.iter().map(|c| (*c, self.town.met.contains(&(v.id, *c)))).collect(); (v, w, c) }).collect()
    }
    /// Marquer le n-ième village du journal (ou démarquer s'il l'était).
    pub fn mark_nth(&mut self, n: usize) { if let Some((v, _, _)) = self.journal().get(n) { self.town.marked = if self.town.marked == Some(v.id) { None } else { Some(v.id) }; self.dirty = true; } }
    /// Le village le plus proche d'un point (pour les captures et les essais).
    pub fn nearest_village(&self, x: f64, z: f64) -> Option<Village> { self.gen.villages_near(x, z, CELL as f64 * 4.0).into_iter().next() }

    pub(crate) fn town_save(&self) -> String {
        let mut s = String::new();
        let mut rep: Vec<_> = self.town.rep.iter().collect(); rep.sort(); for (id, n) in rep { s += &format!("rep {} {} {n}\n", id.0, id.1); }
        for id in &self.town.known { s += &format!("known {} {}\n", id.0, id.1); }
        let mut met: Vec<_> = self.town.met.iter().collect(); met.sort(); for (id, c) in met { s += &format!("met {} {} {c}\n", id.0, id.1); }
        if let Some(id) = self.town.marked { s += &format!("marked {} {}\n", id.0, id.1); }
        s
    }
    /// Une ligne de sauvegarde du village ; faux si elle n'en est pas une.
    pub(crate) fn town_load(&mut self, key: &str, a: &[&str]) -> bool {
        let n = |i: usize| a.get(i).and_then(|v| v.parse::<i32>().ok()); let Some(id) = n(0).zip(n(1)) else { return matches!(key, "rep" | "known" | "met" | "marked") };
        match key {
            "rep" => { self.town.rep.insert(id, n(2).unwrap_or(0)); }
            "known" => { if let Some(s) = self.gen.site(id.0, id.1) { if !self.town.known.contains(&id) { self.town.known.push(id); } self.town.atlas.insert(id, s.v.clone()); } }
            "met" => { if let Some(c) = a.get(2).and_then(|c| craftsman(c)) { self.town.met.insert((id, c.id)); } }
            "marked" => { self.town.marked = Some(id); }
            _ => return false,
        }
        true
    }
}
