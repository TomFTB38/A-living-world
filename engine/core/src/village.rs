//! Villages : où ils se trouvent, comment ils sont bâtis, qui y vit et ce que l'on peut y demander.
//! Tout se déduit de la graine : une grille de cases, au plus un village par case, posé sur un terrain plat et sec.
//! Le type du village suit le biome, ses ateliers suivent artisans.txt. Les bâtiments sont posés après la génération
//! du terrain (`gen_world`), si bien que le terrain lui-même reste celui du prototype.
//! Les villageois suivent leur journée : à l'atelier ou aux champs le jour, sur la place le soir, chez eux la nuit.
use crate::actors::{Draw, Models};
use crate::batisse::{raise, style, Lot, Plan, Prop, Work};
use crate::biome::Biome;
use crate::blocks::{Id, by_name, idx, CH, CX};
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
pub const CELL: i32 = 640;
/// Aucun village ne déborde de ce rayon autour de son centre, terrain aplani compris.
pub const SPAN: i32 = 76;
/// Distance où les villageois apparaissent, et où ils s'effacent.
const NEAR: f64 = 150.0; const FAR: f64 = 210.0;
/// Réputation demandée pour qu'un artisan transmette sa technique.
pub const TEACH_REP: i32 = 20;
pub(crate) const BLEND: i32 = 6;

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
    /// Le chemin de la porte jusqu'au bord de la place, le long des rues (sans la porte).
    pub way: Vec<[f64; 3]>,
}
#[derive(Clone, Debug)]
pub struct Village { pub id: (i32, i32), pub kind: &'static str, pub name: String, pub c: [i32; 3], pub r: i32, pub crafts: Vec<&'static str>, pub houses: Vec<House>, /** l'identité du village (civilisation, taille, spécialité, richesse, âge, orientation), pour les villages refondus */ pub ident: Option<crate::bourg::Identity> }
/// Un village tracé, avec les blocs à poser.
pub struct Site { pub v: Village, pub(crate) blocks: Vec<(i32, i32, i32, Id)>, pub(crate) props: Vec<Prop>, pub(crate) pads: Vec<Pad>, pub(crate) paths: Vec<[i32; 3]> }
/// Une assise aplanie : la place (ronde) ou le terrain d'un bâtiment (rectangle), à sa hauteur, fondue dans le relief alentour.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Pad { pub c: [f64; 2], pub half: [f64; 2], pub round: bool, pub h: i32 }
impl Pad { fn dist(&self, x: f64, z: f64) -> f64 { let (dx, dz) = ((x - self.c[0]).abs(), (z - self.c[1]).abs()); if self.round { (dx.hypot(dz) - self.half[0]).max(0.0) } else { (dx - self.half[0]).max(0.0).hypot((dz - self.half[1]).max(0.0)) } } }
/// La hauteur du sol en ce point : le relief naturel, tiré vers chaque assise proche (la plus proche l'emporte), et la distance à l'assise la plus proche.
pub(crate) fn shape(pads: &[Pad], nat: i32, x: f64, z: f64) -> (i32, f64) {
    let mut near: Vec<(f64, i32)> = pads.iter().map(|p| (p.dist(x, z), p.h)).filter(|(d, _)| *d < BLEND as f64).collect();
    near.sort_by(|a, b| b.0.total_cmp(&a.0)); let mut g = nat as f64;
    for (d, h) in &near { let t = d / BLEND as f64; let t = t * t * (3.0 - 2.0 * t); g = *h as f64 + (g - *h as f64) * t; }
    (g.round() as i32, pads.iter().map(|p| p.dist(x, z)).fold(f64::MAX, f64::min))
}
/// Les arbres et les plantes : on les enlève là où l'on bâtit, pour ne laisser ni troncs coupés ni couronnes en l'air.
fn wild(b: Id) -> bool { let k = &crate::blocks::BLOCKS[b as usize]; k.shape == crate::blocks::Shape::Leaf || k.name.contains("_log") || k.name == "cactus" || k.name == "vine" || k.name == "shelf_fungus" || (k.shape == crate::blocks::Shape::Deco && !k.solid && (k.cat == "terrain" || k.cat == "decor") && k.light == 0) }
impl Site { /** Le décor fin du village. */ pub fn props(&self) -> &[Prop] { &self.props } }

pub fn kind_label(k: &str) -> &'static str { match k { "prairie" => "prairie", "taiga" => "taïga", "aride" => "steppe", "desert" => "désert", "montagne" => "montagne", "port" => "port de pêche", _ => "hautes terres" } }
pub(crate) fn id(n: &str) -> Id { by_name(n).unwrap_or_else(|| panic!("bloc inconnu : {n}")) }
fn dir(f: u8) -> (i32, i32) { [(0, 1), (1, 0), (0, -1), (-1, 0)][f as usize % 4] }
const NAME_A: [&str; 16] = ["Haut", "Bel", "Clair", "Roche", "Val", "Mont", "Pré", "Bois", "Fond", "Gué", "Saule", "Brune", "Vieux", "Grand", "Pierre", "Chêne"];
const NAME_B: [&str; 12] = ["val", "fort", "mont", "bourg", "lieu", "ville", "champ", "roche", "font", "court", "mesnil", "lande"];
const FIRST: [&str; 24] = ["Anselme", "Berthe", "Colin", "Denise", "Étienne", "Fanchon", "Gaspard", "Hélène", "Isidore", "Jeanne", "Léon", "Margot", "Nicolas", "Odile", "Pierre", "Rosalie", "Simon", "Thérèse", "Urbain", "Victor", "Agathe", "Basile", "Clémence", "Firmin"];

impl WorldGen {
    /// Le village de cette case de la grille, s'il y en a un.
    pub fn site(&self, gx: i32, gz: i32) -> Option<Rc<Site>> {
        if self.flat { return None; }
        if let Some(s) = self.towns.borrow().get(&(gx, gz)) { return s.clone(); }
        let s = self.trace(gx, gz).map(Rc::new); self.towns.borrow_mut().insert((gx, gz), s.clone()); s
    }
    /// Un chunk complet : le terrain du prototype, puis les villages qui le touchent.
    pub fn gen_world(&self, cx: i32, cz: i32) -> Vec<Id> {
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
        if hash_float(gx, gz, s, 301) > 0.42 { return None; } // un peu plus d'une case sur trois porte un village : ils restent rares, et le voyage compte
        for t in 0..10 {
            let x = ((gx as f64 + 0.22 + 0.56 * hash_float(gx, gz, s, 302 + t * 2)) * CELL as f64) as i32;
            let z = ((gz as f64 + 0.22 + 0.56 * hash_float(gx, gz, s, 303 + t * 2)) * CELL as f64) as i32;
            // Près d'une côte, le village s'installe d'abord à une trentaine de blocs du rivage : il devient un port.
            let mut shores: Vec<(f64, i32)> = (0..16).filter_map(|k| { let a = k as f64 * TAU / 16.0; (4..=60).step_by(4).find(|r| { let c = self.column(x as f64 + a.cos() * *r as f64, z as f64 + a.sin() * *r as f64); c.water == crate::gen::SEA && c.mix.sea > 0.0 }).map(|r| (a, r)) }).collect();
            shores.sort_by_key(|s| s.1);
            for &(a, r) in shores.iter().take(3) { for back in [26.0, 32.0, 40.0, 50.0] { let (x2, z2) = (x + (a.cos() * (r as f64 - back)).round() as i32, z + (a.sin() * (r as f64 - back)).round() as i32); if let Some(y0) = self.flat_at(x2, z2, true) { return Some(self.build(gx, gz, x2, y0, z2)); } } }
            if let Some(y0) = self.flat(x, z) { return Some(self.build(gx, gz, x, y0, z)); }
        }
        None
    }
    /// Hauteur de la place si le terrain est sec et à peu près plat sur trente blocs alentour.
    fn flat(&self, x: i32, z: i32) -> Option<i32> { self.flat_at(x, z, false) }
    /// `coast` : un port, plus serré, se contente d'un terrain moins large et un peu moins plat.
    fn flat_at(&self, x: i32, z: i32, coast: bool) -> Option<i32> {
        let mut tops = Vec::with_capacity(25);
        for (r, n) in if coast { &[(0.0, 1), (9.0, 8), (17.0, 8), (22.0, 8)][..] } else { &[(0.0, 1), (10.0, 8), (19.0, 8), (27.0, 8)][..] } { let (r, n) = (*r, *n); for k in 0..n {
            let a = k as f64 * TAU / n as f64 + r * 0.05; let c = self.column(x as f64 + r * a.cos(), z as f64 + r * a.sin());
            if c.water != 0.0 || c.cliff > 0.45 { return None; } tops.push(c.h.ceil() as i32 - 1);
        } }
        let (lo, hi) = (*tops.iter().min()?, *tops.iter().max()?); let tol = if self.climate.at(x as f64, z as f64).strong(Biome::Mountains) > 0.5 { 15 } else { 10 }; /* en montagne, un vallon un peu pentu suffit : le village l'aplanit */ if hi - lo > tol + if coast { 3 } else { 0 } { return None; }
        tops.sort(); let y0 = tops[tops.len() / 2]; (y0 > 30 && y0 + 20 < CH as i32).then_some(y0)
    }
    fn build(&self, gx: i32, gz: i32, x: i32, y0: i32, z: i32) -> Site {
        let s = self.seed; let h = |k: i32| hash_float(gx, gz, s, 320 + k);
        let mix = self.climate.at(x as f64, z as f64); let top = (0..crate::biome::COUNT).max_by(|a, b| mix.w[*a].total_cmp(&mix.w[*b])).unwrap_or(1);
        // Montagne : un bourg haut perché, ou blotti au pied de sommets qui le dominent.
        let peaks = (0..16).map(|k| { let (a, r) = (k as f64 * TAU / 8.0, if k < 8 { 60.0 } else { 95.0 }); self.column(x as f64 + r * a.cos(), z as f64 + r * a.sin()).h }).fold(0.0, f64::max);
        // Bord de mer : si la mer est à quelques dizaines de blocs, le village devient un port de pêche, tourné vers elle.
        let sea_dir = (0..24).filter_map(|k| { let a = k as f64 * TAU / 24.0; (20..=50).step_by(3).find(|r| { let c = self.column(x as f64 + 0.5 + a.cos() * *r as f64, z as f64 + 0.5 + a.sin() * *r as f64); c.water == crate::gen::SEA && c.mix.sea > 0.0 }).map(|r| (a, r)) }).min_by_key(|(_, r)| *r);
        let kind = if sea_dir.is_some() && mix.strong(Biome::Mountains) < 0.4 { "port" } else if y0 > 76 || peaks - y0 as f64 > 24.0 || mix.strong(Biome::Mountains) > 0.4 { "montagne" } else if mix.strong(Biome::Highlands) > 0.4 { "hautes_terres" } else { match top { t if t == Biome::Taiga as usize => "taiga", t if t == Biome::Steppe as usize => "aride", t if t == Biome::Desert as usize => "desert", t if t == Biome::Highlands as usize => "hautes_terres", _ => "prairie" } };
        // Les ateliers : le forgeron dès que le type le permet, les autres une fois sur trois de moins ; au moins deux, au plus quatre.
        let ck = if kind == "port" { "prairie" } else { kind }; let cand: Vec<&Craftsman> = db().craftsmen.iter().filter(|c| c.villages.contains(&ck)).collect();
        let mut crafts: Vec<&'static str> = cand.iter().enumerate().filter(|(i, c)| c.id == "forgeron" || h(*i as i32) < 0.68).map(|(_, c)| c.id).collect();
        for c in &cand { if crafts.len() >= 2 { break; } if !crafts.contains(&c.id) { crafts.push(c.id); } }
        crafts.truncate(if kind == "port" { 2 } else { 4 }); // un port de pêche reste un petit bourg
        let homes = 2 + (h(10) * 3.0) as usize; let name = format!("{}{}", NAME_A[(h(11) * 16.0) as usize % 16], NAME_B[(h(12) * 12.0) as usize % 12]);
        // La civilisation médiévale passe par le nouveau générateur de plans ; les autres suivront (désert, côte, Nord).
        if crate::bourg::civ_of(kind) == crate::bourg::Civ::Medieval { return crate::bourg::medieval(self, gx, gz, x, y0, z, kind, crafts, name); }
        let mut v = Village { id: (gx, gz), kind, name, c: [x, y0, z], r: 8, crafts: crafts.clone(), houses: Vec::new(), ident: None };
        let nat = |x: i32, z: i32| self.column(x as f64 + 0.5, z as f64 + 0.5).h.ceil() as i32 - 1;
        let mut pads = vec![Pad { c: [x as f64 + 0.5, z as f64 + 0.5], half: [7.5, 7.5], round: true, h: y0 }]; let mut paths: Vec<[i32; 3]> = Vec::new();
        let st = style(kind); let mut plaza = Work::default(); let mut lanes = Work::default(); let mut works = Work::default();
        // La place pavée : un puits couvert au milieu, des lanternes aux quatre coins.
        let pave = st.pave; let water = id("water"); let fence = id("fence"); let lantern = id("lantern"); let cob = id("cobblestone");
        for dx in -6i32..=6 { for dz in -6i32..=6 { if dx * dx + dz * dz <= 40 { plaza.put([x + dx, y0, z + dz], pave); } } }
        for dx in -1..=1 { for dz in -1..=1 { let edge = dx != 0 || dz != 0; for (y, b) in [(y0, if edge { cob } else { water }), (y0 - 1, if edge { cob } else { water }), (y0 - 2, cob)] { plaza.put([x + dx, y, z + dz], b); } if edge { plaza.put([x + dx, y0 + 1, z + dz], cob); } } }
        crate::batisse::well(&mut plaza, [x, y0, z], st.tiles);
        for (dx, dz) in [(4, 4), (-4, 4), (4, -4), (-4, -4)] { plaza.put([x + dx, y0 + 1, z + dz], fence); plaza.put([x + dx, y0 + 2, z + dz], lantern); }
        // Ce que l'on bâtit : les ateliers (la forge en pierre), une auberge, des maisons de ville et des chaumières, une église, un moulin.
        let mut plans: Vec<Plan> = crafts.iter().map(|c| if matches!(*c, "forgeron" | "maitre_forgeron" | "armurier") { Plan::Forge(c) } else { Plan::Shop(c) }).collect();
        for i in 0..homes { plans.push(match i { 0 => Plan::Tavern, i if i % 2 == 1 => Plan::Cottage, _ => Plan::Town }); }
        if h(14) < 0.8 { plans.insert(plans.len() / 2, Plan::Church); }
        if matches!(kind, "prairie" | "aride" | "hautes_terres" | "taiga") && h(15) < 0.75 { plans.push(Plan::Mill); }
        let n = plans.len(); let a0 = h(13) * TAU; let mut taken: Vec<[i32; 4]> = Vec::new(); let mut angles = Vec::new();
        for (i, plan) in plans.iter().copied().enumerate() {
            let (w, d) = plan.size(); let (pw, pd) = if plan == Plan::Mill { (5, 5) } else { (w, d + if plan == Plan::Church { 2 } else { 0 }) };
            let a = match sea_dir.filter(|_| kind == "port") { Some((sa, _)) => sa + std::f64::consts::PI + ((i as f64 + 0.5) / n as f64 - 0.5) * TAU * 0.72 + (h(20 + i as i32) - 0.5) * 0.25, None => a0 + TAU * i as f64 / n as f64 + (h(20 + i as i32) - 0.5) * 0.4 }; // un port tourne ses maisons vers la terre, le dos à la mer
            let (ox, oz) = (a.cos(), a.sin()); let f = crate::shapes::facing_of(ox, oz); let (wx, wz) = dir(f); let (ux, uz) = dir(f + 1);
            let mut rad = 10.0 + n as f64 * 1.0; let mut placed = None;
            for _ in 0..14 {
                let (px, pz) = (x + (ox * (rad + pd as f64 * 0.5)).round() as i32, z + (oz * (rad + pd as f64 * 0.5)).round() as i32);
                let o = (px - (w - 1) / 2 * ux - (d - 1) / 2 * wx, pz - (w - 1) / 2 * uz - (d - 1) / 2 * wz);
                let (lo, hi) = (if plan == Plan::Church { -3 } else if plan == Plan::Mill { -2 } else { -1 }, if plan == Plan::Mill { d + 1 } else { d });
                let cells = [(-1 - (pw - w) / 2, lo), (w + (pw - w) / 2, lo), (-1 - (pw - w) / 2, hi), (w + (pw - w) / 2, hi)].map(|(u, ww)| (o.0 + u * ux + ww * wx, o.1 + u * uz + ww * wz));
                let area = [cells.iter().map(|c| c.0).min().unwrap(), cells.iter().map(|c| c.1).min().unwrap(), cells.iter().map(|c| c.0).max().unwrap(), cells.iter().map(|c| c.1).max().unwrap()];
                let clash = taken.iter().any(|t| area[0] <= t[2] + 2 && area[2] + 2 >= t[0] && area[1] <= t[3] + 2 && area[3] + 2 >= t[1]);
                let far = cells.iter().map(|c| ((c.0 - x) as f64).hypot((c.1 - z) as f64)).fold(0.0, f64::max) + if plan == Plan::Mill { 3.0 } else { 0.0 };
                let plaza_hit = (lo..=hi).any(|ww| (-1..=w).any(|u| { let c = (o.0 + u * ux + ww * wx, o.1 + u * uz + ww * wz); ((c.0 - x) as f64).hypot((c.1 - z) as f64) < 7.5 }));
                // Chaque bâtiment s'assied à la hauteur médiane de son terrain : pas d'eau dessous, pas plus de quatre blocs d'écart avec la place.
                let mut hs = Vec::new(); let mut wet = false;
                for ax in area[0]..=area[2] { for az in area[1]..=area[3] { if (ax + az) % 2 == 0 { let c = self.column(ax as f64 + 0.5, az as f64 + 0.5); wet |= c.water != 0.0; hs.push(c.h.ceil() as i32 - 1); } } }
                hs.sort(); let by = hs.get(hs.len() / 2).copied().unwrap_or(y0);
                if !clash && !plaza_hit && !wet && (by - y0).abs() <= 5 && far + (BLEND as f64) + 8.0 < SPAN as f64 { placed = Some((o, area, far, by)); break; }
                rad += 3.0;
            }
            let Some((o, area, far, by)) = placed else { continue }; taken.push(area);
            pads.push(Pad { c: [(area[0] + area[2] + 1) as f64 * 0.5, (area[1] + area[3] + 1) as f64 * 0.5], half: [(area[2] - area[0] + 1) as f64 * 0.5 + 2.0, (area[3] - area[1] + 1) as f64 * 0.5 + 2.0], round: false, h: by }); v.r = v.r.max(far.ceil() as i32 + 2); angles.push(a);
            let lot = Lot { o, u: (ux, uz), w: (wx, wz) };
            let mid = |u: i32, ww: i32, y: i32| { let c = lot.cell(u, y, ww); [c[0] as f64 + 0.5, y as f64, c[2] as f64 + 0.5] };
            let (du, back) = raise(&mut works, lot, plan, by, st, h(40 + i as i32));
            let front = match plan { Plan::Church => -3, _ => -1 };
            // Une rue pavée de la place à la porte, qui suit le relief.
            let door_out = mid(du, front, by + 1); let dist = (door_out[0] - x as f64 - 0.5).hypot(door_out[2] - z as f64 - 0.5);
            let (ex, ez) = ((door_out[0] - x as f64 - 0.5) / dist, (door_out[2] - z as f64 - 0.5) / dist); let ring = [x as f64 + 0.5 + ex * 6.5, (y0 + 1) as f64, z as f64 + 0.5 + ez * 6.5];
            let mut t = 5.0; while t < dist + 0.6 { let q = (x as f64 + 0.5 + ex * t, z as f64 + 0.5 + ez * t); for (sx, sz) in [(0.0, 0.0), (-ez * 0.7, ex * 0.7)] {
                let (cx, cz) = ((q.0 + sx).floor() as i32, (q.1 + sz).floor() as i32); let gy = shape(&pads, nat(cx, cz), cx as f64 + 0.5, cz as f64 + 0.5).0;
                if !paths.iter().any(|p| p[0] == cx && p[2] == cz) { paths.push([cx, gy, cz]); lanes.put([cx, gy, cz], pave); } } t += 0.5; }
            let (dx0, dz0) = (door_out[0].floor() as i32, door_out[2].floor() as i32); lanes.put([dx0, by, dz0], pave); paths.retain(|p| p[0] != dx0 || p[2] != dz0); paths.push([dx0, by, dz0]);
            if !plan.dwelling() { continue; }
            let craft = plan.craft();
            // Le mobilier : le poste de l'artisan au fond, un lit, un coffre, une lanterne.
            let mut put = |u: i32, ww: i32, b: Id| works.put(lot.cell(u, by + 1, ww), b);
            // Le lit, deux blocs de long, tête au mur du fond ; la lanterne près de la porte, sauf là où l'escalier monte.
            let w = plan.size().0; let fb = crate::shapes::facing_of(wx as f64, wz as f64); put(1, back, variant(id("bed"), fb, false, false, false)); put(1, back - 1, variant(id("bed_foot"), fb, false, false, false));
            match plan { Plan::Town | Plan::Tavern => {} Plan::Shop(_) => put(1, 1, lantern), _ => put(w - 2, 1, lantern) }
            match craft {
                Some("forgeron") | Some("maitre_forgeron") | Some("armurier") => { put(du, back, id("anvil")); put(du + 1, back, id("furnace")); }
                Some("tailleur_pierre") => { put(du, back, id("mason_bench")); put(du + 1, back, cob); }
                Some("tanneur") => { put(du, back, id("workbench")); put(du + 1, back, id("chest")); }
                Some(_) => { put(du, back, id("workbench")); put(du + 1, back, id("planks")); }
                None => { put(w - 2, back, id("chest")); if plan == Plan::Tavern { put(du + 1, 2, id("barrel")); put(du - 1, back - 1, id("barrel")); } }
            }
            let fd = crate::shapes::facing_of(wx as f64, wz as f64);
            v.houses.push(House { craft, area, door: door_out, inside: mid(du, 1, by + 1), spot: mid(du, back - 1, by + 1), bed: mid(2, back, by + 1), ring, back: fd, way: vec![ring] });
        }
        // Les champs, à la lisière : une terre labourée, des rangs de blé, de carottes ou de pommes de terre, une rigole d'eau
        // au milieu, une clôture avec son entrée. Les récoltes mûres sont à prendre.
        let fields = if kind == "desert" { 0 } else { 1 + (h(60) * 2.4) as usize };
        let fence_g = id("fence"); let farmland = id("farmland"); let mut made = 0;
        for k in 0..24 { if made >= fields { break; }
            let a = h(61 + k) * TAU; let rr = v.r as f64 + 5.0; let (fx, fz) = (x + (a.cos() * rr).round() as i32, z + (a.sin() * rr).round() as i32);
            let along_x = a.sin().abs() > a.cos().abs(); let (hx, hz) = if along_x { (4, 3) } else { (3, 4) };
            let area = [fx - hx, fz - hz, fx + hx, fz + hz];
            if taken.iter().any(|t| area[0] <= t[2] + 3 && area[2] + 3 >= t[0] && area[1] <= t[3] + 3 && area[3] + 3 >= t[1]) { continue; }
            if rr + 6.0 + BLEND as f64 + 8.0 >= SPAN as f64 { continue; }
            let mut hs = Vec::new(); let mut wet = false; for ax in area[0]..=area[2] { for az in area[1]..=area[3] { let c = self.column(ax as f64 + 0.5, az as f64 + 0.5); wet |= c.water != 0.0; hs.push(c.h.ceil() as i32 - 1); } }
            hs.sort(); let fy = hs[hs.len() / 2]; if wet || hs[hs.len() - 1] - hs[0] > 6 || (fy - y0).abs() > 5 { continue; }
            taken.push(area); pads.push(Pad { c: [(area[0] + area[2] + 1) as f64 * 0.5, (area[1] + area[3] + 1) as f64 * 0.5], half: [(hx + 1) as f64 + 0.5, (hz + 1) as f64 + 0.5], round: false, h: fy });
            v.r = v.r.max((rr + 6.0).ceil() as i32);
            let crop = ["wheat_3", "carrot_3", "potato_3"][(h(70 + k) * 3.0) as usize % 3]; let crop = id(crop);
            let gate = if (fx - x).abs() > (fz - z).abs() { (if fx > x { area[0] } else { area[2] }, fz) } else { (fx, if fz > z { area[1] } else { area[3] }) };
            for ax in area[0]..=area[2] { for az in area[1]..=area[3] {
                let edge = ax == area[0] || ax == area[2] || az == area[1] || az == area[3];
                if edge { works.put([ax, fy, az], id("dirt")); if (ax, az) != gate { works.put([ax, fy + 1, az], fence_g); } continue; }
                let mid = if along_x { az == fz } else { ax == fx };
                if mid { works.put([ax, fy, az], water); works.put([ax, fy - 1, az], id("dirt")); } else { works.put([ax, fy, az], farmland); works.put([ax, fy + 1, az], crop); }
            } }
            made += 1;
        }
        // Le port, comme un vrai port de pêche : un chemin bordé de murets jusqu'à la grève, une halle aux poissons couverte,
        // un grand ponton en T sur pilotis bordé de cordes et de bittes, un voilier et des barques amarrés, une petite jetée,
        // des barques tirées sur le sable, des séchoirs, des filets, des nasses, des rochers, et une tour de guet sur la côte.
        if let Some((a, _)) = sea_dir.filter(|_| kind == "port") {
            let mark = works.blocks.len();
            let (dx, dz) = (a.cos(), a.sin()); let (sx, sz) = (-dz, dx); let cell = |t: f64, w: f64| ((x as f64 + 0.5 + dx * t + sx * w).floor() as i32, (z as f64 + 0.5 + dz * t + sz * w).floor() as i32);
            let sea = crate::gen::SEA; let deck = sea as i32 + 1; let plank = id("spruce_planks"); let post = id("spruce_log"); let stone = id("fieldstone");
            // Les objets orientés : le garde-corps, le filet, le séchoir et la voile suivent l'axe du ponton (ou lui sont perpendiculaires).
            let along = |n: &str, cross: bool| -> Id { let f = if (dx.abs() > dz.abs()) != cross { 0 } else { 1 }; if f == 0 { id(n) } else { id(&format!("{n}@{f}")) } };
            let ground = |cx: i32, cz: i32| self.column(cx as f64 + 0.5, cz as f64 + 0.5).h.ceil() as i32; // la première case libre au-dessus du sol naturel
            let shore = (7..60).find(|t| { let (cx, cz) = cell(*t as f64, 0.0); self.column(cx as f64 + 0.5, cz as f64 + 0.5).water == sea }).unwrap_or(40) as f64;
            // Le chemin et ses murets de pierre sèche.
            let mut t = 7.0; while t < shore - 1.0 { for w in [0.0, 0.7] { let (cx, cz) = cell(t, w); let gy = shape(&pads, nat(cx, cz), cx as f64 + 0.5, cz as f64 + 0.5).0; if !paths.iter().any(|p| p[0] == cx && p[2] == cz) { paths.push([cx, gy, cz]); lanes.put([cx, gy, cz], pave); } } t += 0.5; }
            let mut t = (shore - 14.0).max(9.0); while t < shore - 3.0 { for w in [-2.5, 3.2] { if (t as i32) % 6 == 0 { continue; } let (cx, cz) = cell(t, w); let g = ground(cx, cz); if self.column(cx as f64 + 0.5, cz as f64 + 0.5).water == 0.0 { works.put([cx, g, cz], stone); } } t += 1.0; }
            // Le grand ponton : cinq de large, des pilotis, des bittes et des cordes sur les bords, une tête en T au bout.
            let len = (14.0 + (h(80) * 5.0).floor()).min(58.0 - shore).max(6.0); let end = shore + len;
            let mut t = shore - 3.0;
            while t <= end { let ti = (t - shore + 3.0) as i32;
                for wi in -2i32..=2 { let w = wi as f64; let (cx, cz) = cell(t, w); works.put([cx, deck, cz], plank); for y in deck + 1..deck + 4 { works.put([cx, y, cz], 0); }
                    if wi.abs() == 2 { if ti % 3 == 0 { let floor = self.column(cx as f64 + 0.5, cz as f64 + 0.5).h.floor() as i32; for y in floor..deck { works.put([cx, y, cz], post); } }
                        if t > shore { works.put([cx, deck + 1, cz], if ti % 5 == 0 { id("mooring_post") } else { along("rope_rail", false) }); } } }
                t += 1.0; }
            for tt in 0i32..4 { let t = end - tt as f64; for wi in -6i32..=6 { let w = wi as f64; let (cx, cz) = cell(t, w); if wi.abs() > 2 { works.put([cx, deck, cz], plank); for y in deck + 1..deck + 4 { works.put([cx, y, cz], 0); } }
                if (wi.abs() == 6 || tt == 0) && (wi + tt) % 2 == 0 { let floor = self.column(cx as f64 + 0.5, cz as f64 + 0.5).h.floor() as i32; for y in floor..deck { works.put([cx, y, cz], post); } }
                if tt == 0 && wi.abs() < 6 { works.put([cx, deck + 1, cz], if wi % 3 == 0 { id("mooring_post") } else { along("rope_rail", true) }); }
                if wi.abs() == 6 && tt > 0 { works.put([cx, deck + 1, cz], along("rope_rail", false)); } } }
            for (tt, w) in [(1.0, -6.0), (1.0, 6.0)] { let (cx, cz) = cell(end - tt, w); works.put([cx, deck + 1, cz], fence); works.put([cx, deck + 2, cz], fence); works.put([cx, deck + 3, cz], lantern); }
            for (i, (tt, w)) in [(2.0, -4.0), (2.0, -3.0), (3.0, -4.0), (2.0, 4.0), (3.0, 3.5), (1.5, 0.0), (3.0, -1.5), (3.0, 1.5)].iter().enumerate() { let (cx, cz) = cell(end - tt, *w); works.put([cx, deck + 1, cz], id(["barrel", "barrel", "fish_crate", "crate", "net_pile", "creel", "buoy", "fish_crate"][i])); }
            { let (cx, cz) = cell(end - 2.0, -4.0); works.put([cx, deck + 2, cz], id("barrel")); }
            { let (cx, cz) = cell(end - 2.0, 5.0); works.put([cx, deck + 1, cz], id("pennant")); }
            // Le voilier amarré au bout : une coque de sept, des bancs, le mât et sa grande voile.
            let hull = |works: &mut crate::batisse::Work, t0: f64, w0: f64, n: i32, wide: i32| { for u in 0..n { let half = if u == n - 1 { 0 } else if u == n - 2 && wide > 1 { wide - 1 } else { wide }; for wi in -half..=half { let (cx, cz) = cell(t0 + u as f64, w0 + wi as f64 * 0.95); works.put([cx, sea as i32, cz], plank);
                if wi.abs() == half || u == 0 || u == n - 1 { works.put([cx, deck, cz], plank); } else if u % 2 == 1 { works.put([cx, deck, cz], along("boat_bench", true)); } else { works.put([cx, deck, cz], 0); } } } };
            hull(&mut works, end - 7.0, 9.0, 8, 1); let (mx, mz) = cell(end - 3.0, 9.0); for y in deck..deck + 7 { works.put([mx, y, mz], fence); }
            for u in [-2.0, -1.0, 1.0] { for y in deck + 2..deck + 6 { let (cx, cz) = cell(end - 3.0 + u, 9.0); works.put([cx, y, cz], along("sail", false)); } }
            { let (cx, cz) = cell(end - 3.0, 9.0); works.put([cx, deck + 7, cz], id("pennant")); }
            // Les barques amarrées le long du ponton, et deux autres tirées sur le sable.
            hull(&mut works, shore + 4.0, -4.2, 5, 1); hull(&mut works, shore + (len * 0.5).floor(), 4.2, 5, 1);
            for (i, w) in [-8.0, -11.5].iter().enumerate() { let t0 = shore - 3.0 - i as f64; let (cx, cz) = cell(t0, *w); let g = ground(cx, cz); if self.column(cx as f64 + 0.5, cz as f64 + 0.5).water == 0.0 && (g - deck).abs() <= 3 {
                for u in 0..5 { let half: i32 = if u == 4 { 0 } else { 1 }; for wi in -half..=half { let (px, pz) = (cx + (sx * wi as f64).round() as i32 + (dx * (u as f64 - 2.0)).round() as i32, cz + (sz * wi as f64).round() as i32 + (dz * (u as f64 - 2.0)).round() as i32);
                    let gg = ground(px, pz); works.put([px, gg, pz], plank); if wi.abs() == half || u == 0 || u == 4 { works.put([px, gg + 1, pz], plank); } } }
                let (ox2, oz2) = cell(t0 + 1.0, *w + 2.0); let g2 = ground(ox2, oz2); works.put([ox2, g2, oz2], id(if i == 0 { "net_pile" } else { "creel" })); } }
            // Une petite jetée sur le côté, pour les barques du matin.
            let jw = -15.0; let js = (0..20).map(|k| shore - 4.0 + k as f64).find(|t| { let (cx, cz) = cell(*t, jw); self.column(cx as f64 + 0.5, cz as f64 + 0.5).water == sea });
            if let Some(js) = js { for k in -2..7 { let t = js + k as f64; for wi in -1..=1 { let (cx, cz) = cell(t, jw + wi as f64); works.put([cx, deck, cz], plank); for y in deck + 1..deck + 3 { works.put([cx, y, cz], 0); } if wi != 0 && k % 3 == 0 { let floor = self.column(cx as f64 + 0.5, cz as f64 + 0.5).h.floor() as i32; for y in floor..deck { works.put([cx, y, cz], post); } } }
                    if k == 6 || k == 3 { let (cx, cz) = cell(t, jw + 1.0); works.put([cx, deck + 1, cz], id("mooring_post")); } }
                hull(&mut works, js + 2.0, jw - 3.0, 5, 1); }
            // La halle aux poissons : quatre poteaux, un toit de bardeaux qui déborde, l'étal, les caisses, les filets pendus.
            let (ht, hw) = (shore - 6.0, 7.5); let mut top = 0; for u in -3..=3 { for w in -2..=2 { let (cx, cz) = cell(ht + u as f64, hw + w as f64); top = top.max(ground(cx, cz)); } }
            if top > 0 && top - deck < 6 { let roof = top + 4;
                for u in -3..=3 { for w in -2..=2 { let (cx, cz) = cell(ht + u as f64, hw + w as f64); let g = ground(cx, cz); for y in g..top { works.put([cx, y, cz], plank); } // le plancher, à niveau
                    let corner = (u == -3 || u == 3) && (w == -2 || w == 2); if corner { for y in top..roof { works.put([cx, y, cz], post); } } }
                    for w in -3..=3 { let (cx, cz) = cell(ht + u as f64, hw + w as f64); works.put([cx, roof, cz], id("shingles")); } }
                for u in [-4, 4] { for w in -3..=3 { let (cx, cz) = cell(ht + u as f64, hw + w as f64); works.put([cx, roof - 1, cz], id("planks_slab_top")); } }
                for u in -1..=1 { let (cx, cz) = cell(ht + u as f64, hw - 1.5); works.put([cx, top, cz], id("planks_slab")); }
                for (u, w, n) in [(-1, 1, "fish_crate"), (0, 1, "fish_crate"), (1, 1, "barrel"), (-2, -1, "creel"), (2, -1, "crate"), (2, 0, "fish_crate")] { let (cx, cz) = cell(ht + u as f64, hw + w as f64); works.put([cx, top, cz], id(n)); }
                for u in [-2, 0, 2] { let (cx, cz) = cell(ht + u as f64, hw + 2.0); works.put([cx, top + 1, cz], along("fishing_net", true)); }
                { let (cx, cz) = cell(ht, hw); works.put([cx, roof - 1, cz], lantern); }
                for (u, w, n) in [(-4, -3, "oars"), (4, -3, "anchor"), (-4, 3, "buoy")] { let (cx, cz) = cell(ht + u as f64, hw + w as f64); works.put([cx, ground(cx, cz), cz], if n == "buoy" { id(n) } else { along(n, false) }); } }
            // Les séchoirs à poissons et les filets, le long du chemin.
            for (i, t) in [shore - 12.0, shore - 16.0, shore - 20.0].iter().enumerate() { if *t < 10.0 { continue; } let w = if i % 2 == 0 { -4.0 } else { 4.5 }; let (cx, cz) = cell(*t, w); if self.column(cx as f64 + 0.5, cz as f64 + 0.5).water == 0.0 { works.put([cx, ground(cx, cz), cz], along("fish_rack", false)); let (nx2, nz2) = cell(*t + 1.0, w); works.put([nx2, ground(nx2, nz2), nz2], along(if i == 1 { "fishing_net" } else { "fish_rack" }, false)); } }
            // Des rochers sur la grève, à l'écart du ponton.
            for k in 0..6 { let (t, w) = (shore - 1.0 + (h(90 + k) * 4.0).floor(), if k % 2 == 0 { -20.0 - h(96 + k) * 6.0 } else { 14.0 + h(96 + k) * 6.0 }); let (cx, cz) = cell(t, w); let g = ground(cx, cz);
                for (ox2, oz2, oy) in [(0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1), (-1, 0, 0)] { if (ox2 + oz2 + oy) as f64 <= h(100 + k) * 3.0 { works.put([cx + ox2, g + oy - 1, cz + oz2], if oy == 1 { id("mossy_stone") } else { id("stone") }); } } }
            // La tour de guet, sur le point le plus haut du rivage, avec son fanion.
            let mut best = None; for w in (-30..=30).step_by(3) { if w > -8 && w < 18 { continue; } for t in [shore - 8.0, shore - 5.0, shore - 3.0] { let (cx, cz) = cell(t, w as f64); let c = self.column(cx as f64 + 0.5, cz as f64 + 0.5); if c.water == 0.0 { let g = ground(cx, cz); if best.is_none_or(|b: (i32, i32, i32)| g > b.0) { best = Some((g, cx, cz)); } } } }
            if let Some((g, tx, tz)) = best { let top = g + 6;
                for ox2 in -1..=1 { for oz2 in -1..=1 { let corner = ox2 != 0 && oz2 != 0; let gg = ground(tx + ox2, tz + oz2); if corner { for y in (gg - 1)..top { works.put([tx + ox2, y, tz + oz2], post); } } works.put([tx + ox2, top, tz + oz2], plank);
                    if ox2 != 0 || oz2 != 0 { works.put([tx + ox2, top + 1, tz + oz2], fence); } } }
                works.put([tx, top + 1, tz], 0); works.put([tx + 1, top + 2, tz + 1], fence); works.put([tx + 1, top + 3, tz + 1], id("pennant")); works.put([tx - 1, top + 2, tz - 1], lantern);
                for y in g..top { works.put([tx, y, tz - 2], id("ladder")); } }
            // Rien au-delà de la portée du village.
            let far = SPAN - 7; let mut tail = works.blocks.split_off(mark); tail.retain(|b| ((b.0 - x) as f64).hypot((b.2 - z) as f64) <= far as f64); works.blocks.extend(tail);
            let reach = works.blocks[mark.min(works.blocks.len())..].iter().map(|b| ((b.0 - x) as f64).hypot((b.2 - z) as f64)).fold(0.0, f64::max); v.r = v.r.max(reach.ceil() as i32 + 1).min(SPAN - 6);
        }
        // Le marché : des étals entre les rues, face au puits, à l'écart des lanternes.
        let stripes = [[[0.62, 0.14, 0.12], [0.9, 0.86, 0.78]], [[0.18, 0.28, 0.5], [0.9, 0.86, 0.78]], [[0.28, 0.42, 0.22], [0.88, 0.82, 0.64]], [[0.7, 0.48, 0.15], [0.38, 0.24, 0.13]]];
        let mut sorted = angles.clone(); sorted.sort_by(|a, b| a.rem_euclid(TAU).total_cmp(&b.rem_euclid(TAU)));
        let mut placed = 0;
        for i in 0..sorted.len() { if placed >= 3 { break; }
            let (a, b2) = (sorted[i].rem_euclid(TAU), sorted[(i + 1) % sorted.len()].rem_euclid(TAU) + if i + 1 == sorted.len() { TAU } else { 0.0 });
            let m = (a + b2) * 0.5; if b2 - a < 0.7 { continue; }
            let diag = (0..4).any(|k| { let q = (m - (0.785 + k as f64 * 1.5708)).rem_euclid(TAU); q < 0.32 || q > TAU - 0.32 }); if diag { continue; }
            let c = [x as f64 + 0.5 + m.cos() * 6.0, (y0 + 1) as f64, z as f64 + 0.5 + m.sin() * 6.0]; let at = [c[0].floor() as i32, y0, c[2].floor() as i32];
            crate::batisse::stall(&mut plaza, at, c, [-m.cos(), 0.0, -m.sin()], stripes[(placed + (h(16) * 4.0) as usize) % 4], h(50 + i as i32)); placed += 1;
        }
        let mut out = plaza.blocks; out.extend(lanes.blocks); out.extend(works.blocks);
        // Le puits et les lanternes, posés en dernier : aucune rue ne les recouvre.
        for dx in -1..=1 { for dz in -1..=1 { let edge = dx != 0 || dz != 0; if edge { out.push((x + dx, y0 + 1, z + dz, cob)); } out.push((x + dx, y0, z + dz, if edge { cob } else { water })); } }
        let mut props = plaza.props; props.extend(works.props);
        Site { v, blocks: out, props, pads, paths }
    }
}
impl Site {
    /// Pose ce village dans un chunk : le sol aplani d'abord (fondu vers le terrain sur les bords), puis les bâtiments.
    fn stamp(&self, gen: &WorldGen, cx: i32, cz: i32, v: &mut [Id]) {
        let (ox, oz) = (cx * CX as i32, cz * CX as i32); let c = self.v.c; let reach = self.v.r + BLEND + 8;
        if c[0] + reach < ox || c[0] - reach >= ox + CX as i32 || c[2] + reach < oz || c[2] - reach >= oz + CX as i32 { return; }
        let dirt = id("dirt"); let ch = CH as i32;
        for z in 0..CX { for x in 0..CX {
            let (wx, wz) = (ox + x as i32, oz + z as i32); let d = ((wx - c[0]) as f64).hypot((wz - c[2]) as f64); if d > reach as f64 { continue; }
            let nat = gen.column(wx as f64 + 0.5, wz as f64 + 0.5).h.ceil() as i32 - 1; if nat < 2 || nat >= ch - 2 { continue; }
            let (goal, dpad) = shape(&self.pads, nat, wx as f64 + 0.5, wz as f64 + 0.5); let goal = goal.clamp(2, ch - 3);
            let dpath = self.paths.iter().map(|p| ((p[0] - wx) as f64).hypot((p[2] - wz) as f64)).fold(f64::MAX, f64::min);
            // Sur une assise et sur ses bords : le sol remonté ou creusé, et rien au-dessus.
            if dpad < BLEND as f64 {
                let surf = match v[idx(x, nat as usize, z)] { 0 => dirt, b if crate::blocks::is_solid(b) => b, _ => dirt }; let sub = match v[idx(x, (nat - 1) as usize, z)] { 0 => dirt, b => b };
                for y in nat.min(goal)..goal { v[idx(x, y as usize, z)] = sub; }
                v[idx(x, goal as usize, z)] = surf;
                for y in goal + 1..(nat.max(goal) + 6).min(ch) { v[idx(x, y as usize, z)] = 0; }
            }
            // Autour, et le long des rues : les arbres et les plantes partent en entier ; le relief, lui, reste.
            if dpad < BLEND as f64 + 6.0 || dpath < 4.5 {
                for y in (goal.min(nat) + 1).max(1)..(nat.max(goal) + 34).min(ch) { let k = idx(x, y as usize, z); if v[k] != 0 && wild(v[k]) { v[k] = 0; } }
            }
        } }
        for &(x, y, z, b) in &self.blocks {
            if x < ox || z < oz || x >= ox + CX as i32 || z >= oz + CX as i32 || y < 0 || y >= ch { continue; }
            v[idx((x - ox) as usize, y as usize, (z - oz) as usize)] = b;
        }
        // Une rue taillée dans une pente : deux blocs de passage dégagés au-dessus des pavés.
        for p in &self.paths { let (x, z) = (p[0] - ox, p[2] - oz); if x < 0 || z < 0 || x >= CX as i32 || z >= CX as i32 { continue; }
            for y in p[1] + 1..(p[1] + 3).min(ch) { let k = idx(x as usize, y as usize, z as usize); if v[k] != 0 && !self.blocks.iter().any(|b| b.0 == p[0] && b.1 == y && b.2 == p[2]) { v[k] = 0; } } }
    }
}

// ---------------------------------------------------------------------------------------------- villageois
#[derive(Clone, Debug)]
pub struct Folk { pub v: (i32, i32), pub house: usize, pub craft: Option<&'static str>, pub name: String, pub p: [f64; 3], pub yaw: f64, pub phase: f64, pub moving: bool, pub working: bool, route: Vec<[f64; 3]>, goal: (u8, u32), look: u8, /** tenue de la région : 0 médiévale, 1 désert, 2 Nord, 3 port */ pub region: u8,
    /// Effrayé par une arme (secondes qui restent), délai avant de resaluer le joueur.
    pub scared: f64, greet_t: f64, stuck: f64 }
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
    /// La nuit au lit. Les artisans à l'atelier, avec une pause à midi. Les autres vont et viennent : chez eux, devant chez un voisin,
    /// le long d'une rue, au marché ; le puits n'est qu'une étape parmi d'autres. Le soir, une partie se retrouve sur la place.
    fn want(&self, f: &Folk, k: usize, hour: f64) -> ((u8, u32), [f64; 3]) {
        let Some(v) = self.village(f.v) else { return ((0, 0), f.p) }; let h = &v.houses[f.house]; let y = v.c[1] as f64 + 1.0;
        let rnd = |salt: u32, q: i32| hash_float(k as i32 * 7 + q, salt as i32, v.id.0, v.id.1);
        let plaza = |salt: u32| { let a = rnd(salt, 1) * TAU; let r = 2.6 + 2.3 * rnd(salt, 2); [v.c[0] as f64 + 0.5 + r * a.cos(), y, v.c[2] as f64 + 0.5 + r * a.sin()] };
        // Un point de la rue qui mène à une maison : entre le bord de la place et sa porte.
        let street = |salt: u32| { let o = &v.houses[(rnd(salt, 3) * v.houses.len() as f64) as usize % v.houses.len()]; if o.way.len() > 2 { return o.way[(rnd(salt, 4) * o.way.len() as f64) as usize % o.way.len()]; } let t = 0.25 + 0.6 * rnd(salt, 4); [o.ring[0] + (o.door[0] - o.ring[0]) * t, o.ring[1] + (o.door[1] - o.ring[1]) * t, o.ring[2] + (o.door[2] - o.ring[2]) * t] };
        let neighbour = |salt: u32| { let o = &v.houses[(rnd(salt, 5) * v.houses.len() as f64) as usize % v.houses.len()]; let side = (rnd(salt, 6) - 0.5) * 1.6; let (ex, ez) = (o.door[0] - o.ring[0], o.door[2] - o.ring[2]); let l = ex.hypot(ez).max(0.1); [o.door[0] - ex / l * 1.2 - ez / l * side, o.door[1], o.door[2] - ez / l * 1.2 + ex / l * side] };
        if !(6.5..21.5).contains(&hour) { return ((0, 0), h.bed); }
        let slot = (hour * 0.9 + rnd(9, 7) * 2.0) as u32;
        if (8.0..18.0).contains(&hour) {
            if f.craft.is_some() { return if (12.5..13.5).contains(&hour) { ((6, 0), h.inside) } else { ((1, 0), h.spot) }; }
            return match (rnd(slot, 8) * 10.0) as u32 { 0..=2 => ((6, slot), h.inside), 3..=4 => ((2, slot), neighbour(slot)), 5..=7 => ((2, slot), street(slot)), 8 => ((2, slot), plaza(slot + 40)), _ => ((2, slot), [h.door[0], h.door[1], h.door[2]]) };
        }
        // Le soir : un sur deux sur la place, les autres devant chez eux ou chez un voisin.
        match (rnd(slot, 10) * 4.0) as u32 { 0 | 1 => ((3, slot), plaza(k as u32 + 3 + slot)), 2 => ((3, slot), neighbour(slot + 90)), _ => ((6, slot), h.inside) }
    }
    fn route(&self, f: &Folk, to: [f64; 3], dest_house: Option<usize>) -> Vec<[f64; 3]> {
        let Some(v) = self.village(f.v) else { return vec![to] }; let mut r = Vec::new();
        if let Some(h) = v.houses.iter().find(|h| inside(h, f.p)) { r.push(h.inside); r.push(h.door); r.extend(h.way.iter().copied()); }
        match dest_house { Some(i) if !inside(&v.houses[i], f.p) => { let h = &v.houses[i]; r.extend(h.way.iter().rev().copied()); r.push(h.door); r.push(h.inside); } _ => {} }
        if let Some(i) = dest_house { if inside(&v.houses[i], f.p) { r.clear(); } }
        r.push(to);
        // Faire le tour du puits plutôt que le traverser.
        let c = [v.c[0] as f64 + 0.5, v.c[2] as f64 + 0.5]; let mut o = Vec::new(); let mut prev = f.p;
        for q in r {
            let (ax, az, bx, bz) = (prev[0] - c[0], prev[2] - c[1], q[0] - c[0], q[2] - c[1]); let (dx, dz) = (bx - ax, bz - az); let l2 = (dx * dx + dz * dz).max(1e-9);
            let t = (-(ax * dx + az * dz) / l2).clamp(0.0, 1.0);
            if (ax + dx * t).hypot(az + dz * t) < 2.8 { let (mut mx, mut mz) = ((ax + bx) * 0.5, (az + bz) * 0.5); if mx.hypot(mz) < 0.1 { (mx, mz) = (-az, ax); } let l = mx.hypot(mz).max(1e-6); o.push([c[0] + mx / l * 3.8, q[1], c[1] + mz / l * 3.8]); }
            o.push(q); prev = q;
        }
        o
    }
    /// La journée des villageois : un pas le long de leur chemin, ou leur travail sur place.
    pub fn update(&mut self, dt: f64, hour: f64, me: [f64; 3], talking: Option<usize>, armed: bool, blocked: &dyn Fn(f64, f64, f64) -> bool, ground: &dyn Fn(f64, f64, f64) -> Option<f64>) {
        for k in 0..self.folk.len() {
            let mut f = self.folk[k].clone(); let near = (f.p[0] - me[0]).hypot(f.p[2] - me[2]); f.greet_t -= dt; f.scared -= dt;
            // Une arme tirée tout près : on recule et on rentre chez soi. Sinon, on salue celui qui passe.
            if armed && near < 2.4 && f.scared <= 0.0 && talking != Some(k) { f.scared = 7.0; self.news.push(format!("{} recule : « Rangez cette arme ! »", f.name)); }
            else if !armed && near < 3.0 && f.greet_t <= 0.0 && f.scared <= 0.0 && talking != Some(k) { f.greet_t = 150.0; self.news.push(format!("{} : « {} ! »", f.name, if hour >= 18.0 || hour < 5.0 { "Bonsoir" } else { "Bonjour" })); }
            let (goal, at) = if f.scared > 0.0 { let Some(v) = self.village(f.v) else { continue }; ((5, 0), v.houses[f.house].inside) } else { self.want(&f, k, hour) };
            let dest = if goal.0 <= 1 || goal.0 == 5 || goal.0 == 6 { Some(f.house) } else { None };
            if goal != f.goal { f.route = self.route(&f, at, dest); f.goal = goal; }
            if talking == Some(k) { f.moving = false; f.working = false; f.yaw = (me[0] - f.p[0]).atan2(me[2] - f.p[2]); self.folk[k] = f; continue; }
            f.working = false;
            if let Some(q) = f.route.first().copied() {
                let (dx, dz) = (q[0] - f.p[0], q[2] - f.p[2]); let d = dx.hypot(dz); let step = if f.scared > 0.0 { 2.6 } else { 1.35 } * dt;
                // Un bloc posé en travers du chemin : on le contourne (de biais, puis de côté), sinon on attend ; coincé trop longtemps, on passe.
                let free = |a: f64| { let (x, z) = (f.p[0] + a.sin() * step.max(0.05) * 3.0, f.p[2] + a.cos() * step.max(0.05) * 3.0); !blocked(x, f.p[1] + 1.0, z) };
                let ahead = dx.atan2(dz);
                if d <= step { f.p = [q[0], f.p[1], q[2]]; f.route.remove(0); f.stuck = 0.0; }
                else if let Some(o) = [0.0, 0.6, -0.6, 1.2, -1.2, 1.6, -1.6].into_iter().find(|o| free(ahead + o)) { let a = ahead + o; f.p = [f.p[0] + a.sin() * step, f.p[1], f.p[2] + a.cos() * step]; f.yaw = a; if o == 0.0 { f.stuck = 0.0; } else { f.stuck += dt; } if f.stuck > 4.0 && !blocked(q[0], q[1], q[2]) { f.p = q; f.route.remove(0); f.stuck = 0.0; } }
                else { f.stuck += dt; if f.stuck > 5.0 && !blocked(q[0], q[1], q[2]) { f.p = q; f.route.remove(0); f.stuck = 0.0; } }
                f.moving = true; f.phase += dt * 7.0;
                // On monte et descend avec le sol (rues en pente, marches, escalier), sans à-coup ; sol inconnu : l'altitude du chemin.
                let gy = ground(f.p[0], f.p[1], f.p[2]).unwrap_or(q[1]); f.p[1] += (gy - f.p[1]).clamp(-dt * 6.0, dt * 6.0);
            } else {
                f.moving = false; if !f.working { f.phase += dt; }
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
            if self.folk.len() >= 100 { break; } // au plus cent villageois simulés à la fois
            let count = if h.craft.is_some() { 1 } else { 1 + (hash_float(v.id.0, v.id.1, i as i32, 91) < 0.6) as usize };
            for j in 0..count {
                let pick = (hash_float(v.id.0 * 7 + i as i32, v.id.1, j as i32, 92) * FIRST.len() as f64) as usize % FIRST.len();
                let look = if let Some(c) = h.craft { db().craftsmen.iter().position(|x| x.id == c).unwrap_or(0) as u8 } else { 6 + (hash_float(i as i32, j as i32, v.id.0, v.id.1) * 3.0) as u8 % 3 };
                self.folk.push(Folk { v: v.id, house: i, craft: if j == 0 { h.craft } else { None }, name: FIRST[pick].to_string(), p: h.spot, yaw: 0.0, phase: n as f64, moving: false, working: false, route: Vec::new(), goal: (9, 0), look, region: match v.kind { "desert" => 1, "montagne" | "taiga" => 2, "port" => 3, _ => 0 }, scared: 0.0, greet_t: 0.0, stuck: 0.0 });
                n += 1;
            }
        }
        // Chacun commence là où sa journée le veut.
        for k in base..self.folk.len() { let (g, at) = self.want(&self.folk[k], k, hour); self.folk[k].p = at; self.folk[k].goal = g; }
    }
    /// Les villageois ont leurs propres gestes : une démarche balancée (épaules et hanches en opposition, tête qui suit),
    /// au repos une respiration, un regard qui fait le tour, une posture à eux (bras ballants, mains dans le dos, bras croisés) ;
    /// en conversation, ils hochent la tête et parlent avec les mains ; au travail, le geste de leur métier, l'outil à la main.
    pub fn draws(&self, md: &Models) -> Vec<Draw> {
        let mut out = Vec::with_capacity(self.folk.len() * 7);
        let talking = self.talk.as_ref().map(|t| t.who);
        for (i, f) in self.folk.iter().enumerate() {
            let base = trs(f.p[0], f.p[1], f.p[2], f.yaw, 0.0, 1.0); let mut put = |n: &str, m| out.push(Draw { m, r: md.part(n), view: false, local: false, hero: false, dynamic: false, hurt: 0.0 });
            let l = f.look.min(8); let seed = f.name.bytes().map(|b| b as usize).sum::<usize>(); let face = seed % 6; let t = f.phase; let k = (seed % 7) as f64;
            #[allow(unused_assignments)] let (mut leg_l, mut leg_r, mut arm_l, mut arm_r, mut yaw_l, mut yaw_r) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let (mut lean, mut bob, mut head_yaw, mut head_pitch) = (0.0, 0.0, 0.0, 0.0); let mut tool: Option<&str> = None;
            if f.moving {
                // La marche : grandes enjambées, bras opposés aux jambes, un léger rebond, le buste un peu en avant.
                let s = t.sin(); let fast = f.scared > 0.0; let amp = if fast { 0.85 } else { 0.55 };
                leg_l = s * amp; leg_r = -s * amp; arm_l = -s * amp * 0.75; arm_r = s * amp * 0.75; bob = (t * 2.0).sin().abs() * 0.035; lean = if fast { 0.18 } else { 0.05 }; head_yaw = (t * 0.5).sin() * 0.08;
            } else if f.working {
                // Le geste du métier, dans son rythme.
                match f.craft {
                    Some("forgeron") | Some("maitre_forgeron") | Some("armurier") => {
                        let c = (t * 0.28).fract(); let up = if c < 0.62 { let e = c / 0.62; e * e * (3.0 - 2.0 * e) } else { 1.0 - ((c - 0.62) / 0.38).powf(0.6) };
                        arm_r = -0.75 - 1.75 * up; arm_l = -0.95; yaw_l = 0.35; lean = 0.12 + 0.06 * (1.0 - up); head_pitch = 0.25; tool = Some("iron_hammer");
                    }
                    Some("tailleur_pierre") => { arm_l = -1.05; yaw_l = 0.3; arm_r = -1.0 - 0.32 * (t * 1.1).sin().abs(); yaw_r = -0.25; lean = 0.2; head_pitch = 0.35; tool = Some("stone_hammer"); }
                    Some("tanneur") => { let sw = (t * 0.32).sin(); arm_l = -1.05 + 0.28 * sw; arm_r = -1.05 + 0.28 * sw; yaw_l = 0.25; yaw_r = -0.25; lean = 0.28 + 0.06 * sw; head_pitch = 0.3; tool = Some("iron_knife"); }
                    Some("maitre_menuisier") => { let sw = (t * 0.45).sin(); arm_r = -1.0 + 0.4 * sw; arm_l = -0.7; yaw_l = 0.3; lean = 0.18 + 0.04 * sw; head_pitch = 0.3; tool = Some("axe"); }
                    _ => { arm_r = -1.2 - 0.9 * (t * 1.4).sin().max(0.0); }
                }
            } else if talking == Some(i) {
                // En conversation : il hoche la tête et accompagne ses mots de la main.
                head_pitch = 0.06 * (t * 2.6).sin(); arm_r = -0.55 - 0.3 * (t * 1.7).sin().max(0.0); yaw_r = -0.25; arm_l = 0.05 * (t * 0.9).sin();
                if seed % 2 == 0 { arm_l = -0.4 - 0.2 * (t * 1.3 + 1.0).sin().max(0.0); yaw_l = 0.2; }
            } else {
                // Au repos : il respire, regarde autour de lui de temps en temps, garde sa posture.
                bob = 0.008 * (t * 0.9).sin(); let look = (t * 0.11 + k).sin(); head_yaw = if look.abs() > 0.55 { look.signum() * (look.abs() - 0.55) * 1.6 } else { 0.0 }; head_pitch = 0.04 * (t * 0.07 + k).sin();
                match seed % 3 { 0 => { arm_l = 0.04 * (t * 0.6).sin(); arm_r = -0.04 * (t * 0.6).sin(); } 1 => { arm_l = 0.38; arm_r = 0.38; yaw_l = -0.3; yaw_r = 0.3; } _ => { arm_l = -0.95; arm_r = -0.95; yaw_l = 0.75; yaw_r = -0.75; } }
                if seed % 5 == 0 { leg_l = 0.06; leg_r = -0.04; }
            }
            let hip = 0.76; let body = mul(&base, &mul(&trs(0.0, hip + bob, 0.0, 0.0, lean, 1.0), &trs(0.0, -hip, 0.0, 0.0, 0.0, 1.0)));
            let rs = if f.region == 0 { String::new() } else { format!("r{}", f.region) }; put(&format!("folk{l}{rs}"), body); put(&format!("folkHead{face}{rs}"), mul(&body, &trs(0.0, 1.4, 0.0, head_yaw, head_pitch, 1.0)));
            put(&format!("folkLeg{l}{rs}"), mul(&base, &trs(-0.125, 0.76 + bob * 0.3, 0.0, 0.0, leg_l, 1.0))); put(&format!("folkLeg{l}{rs}"), mul(&base, &trs(0.125, 0.76 + bob * 0.3, 0.0, 0.0, leg_r, 1.0)));
            let al = mul(&body, &trs(-0.35, 1.34, 0.0, yaw_l, arm_l, 1.0)); let ar = mul(&body, &trs(0.35, 1.34, 0.0, yaw_r, arm_r, 1.0));
            put(&format!("folkArm{l}{rs}"), al); put(&format!("folkArm{l}{rs}"), ar);
            if let Some(h) = tool.and_then(|n| md.held(n)) { out.push(Draw { m: mul(&ar, &mul(&trs(0.0, 0.0, 0.0, 0.0, 0.0, 1.0), &crate::actors::hold_m(h.hold))), r: h.r, view: false, local: h.local, hero: false, dynamic: false, hurt: 0.0 }); }
        }
        out
    }
}
/// La matière d'une pièce de villageois, d'après sa couleur : peau, cheveux, cuir, fer, fourrure, paille ou étoffe.
fn vmat(c: [f64; 3]) -> crate::hostile::Mat {
    use crate::hostile::Mat;
    const SKIN: [[f64; 3]; 8] = [[0.87, 0.69, 0.56], [0.78, 0.6, 0.48], [0.76, 0.58, 0.44], [0.66, 0.47, 0.32], [0.5, 0.35, 0.23], [0.74, 0.56, 0.41], [0.92, 0.76, 0.66], [0.84, 0.64, 0.5]];
    const SKIN2: [[f64; 3]; 4] = [[0.86, 0.68, 0.58], [0.9, 0.72, 0.6], [0.72, 0.52, 0.38], [0.8, 0.6, 0.46]];
    const HAIR: [[f64; 3]; 12] = [[0.32, 0.2, 0.11], [0.13, 0.1, 0.08], [0.72, 0.58, 0.32], [0.08, 0.07, 0.06], [0.14, 0.1, 0.08], [0.1, 0.08, 0.07], [0.82, 0.68, 0.36], [0.62, 0.28, 0.12], [0.48, 0.34, 0.2], [0.3, 0.2, 0.12], [0.15, 0.12, 0.1], [0.6, 0.45, 0.25]];
    const LEATHER: [[f64; 3]; 6] = [[0.22, 0.15, 0.1], [0.4, 0.27, 0.16], [0.2, 0.14, 0.1], [0.16, 0.12, 0.1], [0.5, 0.36, 0.2], [0.66, 0.56, 0.38]];
    const METAL: [[f64; 3]; 6] = [[0.52, 0.53, 0.56], [0.75, 0.66, 0.4], [0.6, 0.6, 0.62], [0.62, 0.58, 0.52], [0.8, 0.66, 0.3], [0.7, 0.7, 0.68]];
    const FUR: [[f64; 3]; 5] = [[0.42, 0.32, 0.22], [0.55, 0.52, 0.48], [0.3, 0.22, 0.15], [0.45, 0.35, 0.25], [0.4, 0.3, 0.2]];
    let near = |l: &[[f64; 3]], ks: &[f64]| l.iter().any(|x| ks.iter().any(|k| (0..3).all(|i| (x[i] * k - c[i]).abs() < 0.004)));
    if near(&SKIN, &[1.0, 0.9]) || near(&SKIN2, &[1.0, 0.9]) { Mat::Human } else if near(&HAIR, &[1.0, 0.8]) { Mat::Hair } else if near(&FUR, &[1.0, 1.2]) { Mat::Fur }
    else if near(&LEATHER, &[1.0]) { Mat::Leather } else if near(&METAL, &[1.0, 0.8]) { Mat::Metal } else if near(&[[0.82, 0.7, 0.42]], &[1.0]) { Mat::Grain }
    else if near(&[[0.95, 0.94, 0.9], [0.14, 0.1, 0.08], [0.55, 0.3, 0.28], [0.5, 0.28, 0.26]], &[1.0]) { Mat::Plain } else { Mat::Cloth }
}
/// Une pièce de villageois : arêtes abattues, matière peinte en pixels.
fn soft(q: &mut VBuf, a: [f64; 6], c: [f64; 3]) { crate::sculpt::rbox(q, a, c, vmat(c), if a[4] - a[1] < 0.2 && a[3] - a[0] < 0.45 { 0.045 } else { 0.06 }) }
/// Le crâne, arrondi : plus étroit au menton et au sommet.
fn skull(q: &mut VBuf, sk: [f64; 3]) { let mut p = crate::sculpt::Pen::new(q, crate::hostile::Mat::Human, 0.045); p.column(&[(0.0, 0.14, 0.15), (0.06, 0.18, 0.18), (0.3, 0.19, 0.19), (0.38, 0.165, 0.17)], 0.0, sk, 0.8); }
/// Les épaules arrondies et le haut des bras, de la couleur du vêtement.
fn shoulders(q: &mut VBuf, c: [f64; 3]) { let mut p = crate::sculpt::Pen::new(q, crate::hostile::Mat::Cloth, 0.05); for sx in [-1.0f64, 1.0] { p.blob([sx * 0.24, 1.33, 0.0], [0.08, 0.075, 0.15], c, c); } }
/// Corps des villageois, un peu plus étoffés que le joueur d'origine. Une tenue par métier (dans l'ordre d'artisans.txt), trois pour les autres :
/// tunique ombrée, col, ceinture et boucle, et l'attribut du métier (tablier de forge, gilet de cuir, ceinture à outils, poussière de pierre).
/// Six têtes : trois chevelures, cheveux courts ou longs, barbe ; yeux avec pupille, sourcils, nez, bouche.
pub fn parts() -> Vec<(&'static str, VBuf)> {
    let b = soft;
    // (tunique, ombre de la tunique, pantalon)
    const OUTFIT: [([f64; 3], [f64; 3], [f64; 3]); 9] = [
        ([0.42, 0.33, 0.25], [0.32, 0.25, 0.19], [0.24, 0.21, 0.19]), ([0.26, 0.22, 0.22], [0.19, 0.16, 0.16], [0.18, 0.16, 0.15]), ([0.36, 0.42, 0.5], [0.27, 0.32, 0.39], [0.22, 0.22, 0.24]),
        ([0.33, 0.46, 0.27], [0.25, 0.35, 0.2], [0.36, 0.3, 0.22]), ([0.56, 0.55, 0.52], [0.44, 0.43, 0.41], [0.3, 0.29, 0.28]), ([0.6, 0.45, 0.27], [0.48, 0.35, 0.2], [0.3, 0.24, 0.18]),
        ([0.3, 0.4, 0.58], [0.22, 0.3, 0.45], [0.34, 0.29, 0.22]), ([0.64, 0.3, 0.24], [0.5, 0.22, 0.18], [0.26, 0.24, 0.22]), ([0.76, 0.7, 0.55], [0.62, 0.56, 0.43], [0.36, 0.32, 0.26])];
    const NAMES: [&str; 9] = ["folk0", "folk1", "folk2", "folk3", "folk4", "folk5", "folk6", "folk7", "folk8"];
    const ARMS: [&str; 9] = ["folkArm0", "folkArm1", "folkArm2", "folkArm3", "folkArm4", "folkArm5", "folkArm6", "folkArm7", "folkArm8"];
    const LEGS: [&str; 9] = ["folkLeg0", "folkLeg1", "folkLeg2", "folkLeg3", "folkLeg4", "folkLeg5", "folkLeg6", "folkLeg7", "folkLeg8"];
    let (skin, skin2, belt, buckle, apron, shoe, linen) = ([0.87, 0.69, 0.56], [0.78, 0.6, 0.48], [0.22, 0.15, 0.1], [0.75, 0.66, 0.4], [0.4, 0.27, 0.16], [0.2, 0.14, 0.1], [0.86, 0.82, 0.72]);
    let mut o = Vec::new();
    for (i, (t, t2, pants)) in OUTFIT.iter().enumerate() {
        let mut body = VBuf::new(200); let q = &mut body;
        b(q, [-0.26, 0.86, -0.16, 0.26, 1.4, 0.16], *t); b(q, [-0.27, 0.68, -0.17, 0.27, 0.88, 0.17], *t2); // buste, puis pans de la tunique
        b(q, [-0.13, 1.33, -0.165, 0.13, 1.41, 0.165], linen); b(q, [-0.035, 1.12, 0.16, 0.035, 1.34, 0.168], *t2); // col, fente
        b(q, [-0.275, 0.86, -0.175, 0.275, 0.93, 0.175], belt); b(q, [-0.05, 0.855, 0.172, 0.05, 0.935, 0.185], buckle);
        match i {
            0..=2 => { b(q, [-0.21, 0.6, 0.17, 0.21, 1.26, 0.2], apron); b(q, [-0.2, 1.26, 0.16, -0.14, 1.4, 0.18], apron); b(q, [0.14, 1.26, 0.16, 0.2, 1.4, 0.18], apron); } // tablier de cuir
            5 => { b(q, [-0.27, 0.92, -0.175, -0.08, 1.38, 0.175], apron); b(q, [0.08, 0.92, -0.175, 0.27, 1.38, 0.175], apron); } // gilet de cuir du tanneur
            3 => { b(q, [0.12, 0.74, 0.16, 0.24, 0.86, 0.22], [0.5, 0.36, 0.2]); b(q, [-0.24, 0.76, 0.17, -0.16, 0.86, 0.2], [0.6, 0.6, 0.62]); } // bourse et outils du menuisier
            4 => { b(q, [-0.2, 1.2, 0.165, 0.2, 1.3, 0.17], [0.7, 0.7, 0.68]); } // poussière de pierre
            _ => { for y in [1.0, 1.1, 1.2] { b(q, [-0.02, y, 0.165, 0.02, y + 0.04, 0.175], *t2); } } // boutons
        }
        shoulders(q, *t); o.push((NAMES[i], body));
        let mut arm = VBuf::new(80); b(&mut arm, [-0.09, -0.42, -0.09, 0.09, 0.04, 0.09], *t); b(&mut arm, [-0.095, -0.44, -0.095, 0.095, -0.38, 0.095], *t2); b(&mut arm, [-0.075, -0.62, -0.08, 0.075, -0.44, 0.08], skin); o.push((ARMS[i], arm));
        let mut leg = VBuf::new(80); b(&mut leg, [-0.11, -0.6, -0.11, 0.11, 0.0, 0.11], *pants); b(&mut leg, [-0.115, -0.76, -0.12, 0.115, -0.6, 0.17], shoe); b(&mut leg, [-0.117, -0.62, -0.122, 0.117, -0.58, 0.122], belt); o.push((LEGS[i], leg));
    }
    const HEADS: [&str; 6] = ["folkHead0", "folkHead1", "folkHead2", "folkHead3", "folkHead4", "folkHead5"];
    let hairs = [[0.32, 0.2, 0.11], [0.13, 0.1, 0.08], [0.72, 0.58, 0.32]];
    for (k, n) in HEADS.iter().enumerate() {
        let hair = hairs[k % 3]; let style = k / 3; let sk = if k % 2 == 0 { skin } else { [0.76, 0.58, 0.44] };
        let mut h = VBuf::new(400); let q = &mut h;
        skull(q, sk); b(q, [-0.2, 0.3, -0.2, 0.2, 0.42, 0.2], hair); b(q, [-0.2, 0.12, -0.2, 0.2, 0.32, -0.1], hair); // crâne, chevelure, nuque
        b(q, [-0.2, 0.2, -0.1, -0.17, 0.33, 0.06], hair); b(q, [0.17, 0.2, -0.1, 0.2, 0.33, 0.06], hair); // tempes
        if style == 1 { b(q, [-0.21, -0.12, -0.21, 0.21, 0.3, -0.12], hair); } // cheveux longs
        for sx in [-1.0, 1.0] { let x0 = if sx < 0.0 { -0.12 } else { 0.05 };
            b(q, [x0, 0.17, 0.19, x0 + 0.07, 0.22, 0.195], [0.95, 0.94, 0.9]); b(q, [x0 + if sx < 0.0 { 0.03 } else { 0.01 }, 0.17, 0.195, x0 + if sx < 0.0 { 0.06 } else { 0.04 }, 0.215, 0.2], [0.14, 0.1, 0.08]); // œil et pupille
            b(q, [x0 - 0.005, 0.245, 0.19, x0 + 0.075, 0.27, 0.2], [hair[0] * 0.8, hair[1] * 0.8, hair[2] * 0.8]); } // sourcils
        b(q, [-0.03, 0.1, 0.19, 0.03, 0.18, 0.225], skin2); b(q, [-0.06, 0.055, 0.19, 0.06, 0.075, 0.198], [0.55, 0.3, 0.28]); // nez, bouche
        if k == 1 || k == 4 { b(q, [-0.16, -0.06, 0.1, 0.16, 0.045, 0.205], hair); b(q, [-0.2, -0.02, -0.02, -0.15, 0.2, 0.18], hair); b(q, [0.15, -0.02, -0.02, 0.2, 0.2, 0.18], hair); b(q, [-0.08, 0.08, 0.19, 0.08, 0.1, 0.205], hair); } // barbe, favoris, moustache
        o.push((n, h));
    }
    o.extend(regional());
    o
}
/// Les tenues des régions : le désert (djellaba, chèche, ceinture de laine, sandales, peaux hâlées), le Nord (laine,
/// manteau de fourrure, jambières croisées, bottes fourrées, tresses, barbes, bonnet ou casque à nasal), le port
/// (marinière rayée, foulard, toile bleue, bonnet de laine ou chapeau de paille). Mêmes pièces, mêmes articulations.
fn regional() -> Vec<(&'static str, VBuf)> {
    let b = soft;
    let leak = |s: String| -> &'static str { Box::leak(s.into_boxed_str()) };
    let dk = |c: [f64; 3], k: f64| [c[0] * k, c[1] * k, c[2] * k];
    let mut o = Vec::new(); let belt = [0.22, 0.15, 0.1]; let leather = [0.4, 0.27, 0.16];
    for region in 1..=3u8 {
        let tones: [[f64; 3]; 3] = match region { 1 => [[0.66, 0.47, 0.32], [0.5, 0.35, 0.23], [0.74, 0.56, 0.41]], 2 => [[0.92, 0.76, 0.66], [0.86, 0.68, 0.58], [0.9, 0.72, 0.6]], _ => [[0.84, 0.64, 0.5], [0.72, 0.52, 0.38], [0.8, 0.6, 0.46]] };
        let cloth: [[f64; 3]; 9] = match region {
            1 => [[0.86, 0.8, 0.66], [0.2, 0.24, 0.45], [0.72, 0.52, 0.28], [0.62, 0.3, 0.2], [0.9, 0.88, 0.82], [0.76, 0.66, 0.48], [0.3, 0.45, 0.62], [0.55, 0.28, 0.18], [0.32, 0.42, 0.3]],
            2 => [[0.42, 0.45, 0.5], [0.3, 0.38, 0.26], [0.55, 0.24, 0.17], [0.42, 0.3, 0.2], [0.78, 0.74, 0.62], [0.36, 0.3, 0.4], [0.24, 0.32, 0.46], [0.5, 0.42, 0.28], [0.6, 0.56, 0.48]],
            _ => [[0.2, 0.25, 0.42], [0.62, 0.2, 0.16], [0.2, 0.25, 0.42], [0.18, 0.3, 0.36], [0.22, 0.24, 0.36], [0.62, 0.2, 0.16], [0.2, 0.25, 0.42], [0.18, 0.3, 0.36], [0.62, 0.2, 0.16]] };
        for (i, t) in cloth.iter().enumerate() { let t2 = dk(*t, 0.78); let sk = tones[i % 3];
            let mut body = VBuf::new(400); let q = &mut body;
            match region {
                1 => { // djellaba : longue robe, capuche dans le dos, ceinture de laine colorée, galon à l'encolure
                    b(q, [-0.27, 0.62, -0.17, 0.27, 1.4, 0.17], *t); b(q, [-0.15, 1.18, -0.23, 0.15, 1.42, -0.16], t2); b(q, [-0.06, 1.0, 0.17, 0.06, 1.38, 0.18], dk(*t, 0.65));
                    let sash = [[0.7, 0.2, 0.16], [0.86, 0.66, 0.2], [0.2, 0.4, 0.5]][i % 3]; b(q, [-0.28, 0.84, -0.18, 0.28, 0.96, 0.18], sash); b(q, [0.14, 0.62, 0.17, 0.2, 0.86, 0.19], sash);
                    if i <= 2 { b(q, [-0.21, 0.6, 0.17, 0.21, 1.26, 0.2], leather); } }
                2 => { // laine et fourrure : tunique, manteau de fourrure sur les épaules, broche, ceinture à grosse boucle
                    b(q, [-0.26, 0.7, -0.16, 0.26, 1.4, 0.16], *t); let fur = [[0.42, 0.32, 0.22], [0.55, 0.52, 0.48], [0.3, 0.22, 0.15]][i % 3];
                    b(q, [-0.31, 1.24, -0.21, 0.31, 1.46, 0.21], fur); b(q, [-0.29, 0.86, -0.21, 0.29, 1.3, -0.17], fur); b(q, [0.12, 1.28, 0.2, 0.2, 1.36, 0.23], [0.8, 0.66, 0.3]);
                    b(q, [-0.275, 0.86, -0.175, 0.275, 0.94, 0.175], belt); b(q, [-0.06, 0.85, 0.172, 0.06, 0.95, 0.19], [0.62, 0.58, 0.52]);
                    for y in [0.74, 0.8] { b(q, [-0.265, y, 0.16, 0.265, y + 0.02, 0.165], t2); } }
                _ => { // marinière : rayures, foulard noué, ceinture de corde
                    for k in 0..7 { let y = 0.72 + k as f64 * 0.1; b(q, [-0.26, y, -0.16, 0.26, y + 0.05, 0.16], [0.9, 0.88, 0.82]); b(q, [-0.262, y + 0.05, -0.162, 0.262, y + 0.1, 0.162], *t); }
                    b(q, [-0.14, 1.3, -0.17, 0.14, 1.42, 0.17], [0.75, 0.16, 0.14]); b(q, [-0.04, 1.14, 0.16, 0.04, 1.3, 0.2], [0.75, 0.16, 0.14]); b(q, [-0.275, 0.84, -0.175, 0.275, 0.9, 0.175], [0.66, 0.56, 0.38]); }
            }
            if region != 2 { shoulders(q, *t); } o.push((leak(format!("folk{i}r{region}")), body));
            let mut arm = VBuf::new(120); let a = &mut arm;
            match region { 1 => { b(a, [-0.11, -0.5, -0.11, 0.11, 0.04, 0.11], *t); b(a, [-0.115, -0.52, -0.115, 0.115, -0.46, 0.115], t2); b(a, [-0.07, -0.64, -0.075, 0.07, -0.5, 0.075], sk); }
                2 => { b(a, [-0.09, -0.42, -0.09, 0.09, 0.04, 0.09], *t); b(a, [-0.1, -0.46, -0.1, 0.1, -0.3, 0.1], leather); b(a, [-0.075, -0.62, -0.08, 0.075, -0.46, 0.08], sk); }
                _ => { for k in 0..4 { let y = -0.3 + k as f64 * 0.08; b(a, [-0.09, y, -0.09, 0.09, y + 0.04, 0.09], [0.9, 0.88, 0.82]); b(a, [-0.091, y + 0.04, -0.091, 0.091, y + 0.08, 0.091], *t); } b(a, [-0.075, -0.62, -0.08, 0.075, -0.3, 0.08], sk); } }
            o.push((leak(format!("folkArm{i}r{region}")), arm));
            let mut leg = VBuf::new(120); let l = &mut leg;
            match region { 1 => { b(l, [-0.13, -0.58, -0.13, 0.13, 0.0, 0.13], *t); b(l, [-0.06, -0.72, -0.07, 0.06, -0.58, 0.07], sk); b(l, [-0.09, -0.76, -0.1, 0.09, -0.72, 0.17], leather); b(l, [-0.09, -0.7, 0.05, 0.09, -0.68, 0.08], leather); } // robe, chevilles nues, sandales
                2 => { let pants = [[0.32, 0.28, 0.24], [0.4, 0.36, 0.3], [0.26, 0.26, 0.28]][i % 3]; b(l, [-0.11, -0.5, -0.11, 0.11, 0.0, 0.11], pants); for y in [-0.46, -0.36, -0.26] { b(l, [-0.115, y, -0.115, 0.115, y + 0.03, 0.115], leather); }
                    let fur = [0.45, 0.35, 0.25]; b(l, [-0.125, -0.76, -0.13, 0.125, -0.5, 0.18], fur); b(l, [-0.13, -0.56, -0.135, 0.13, -0.5, 0.135], dk(fur, 1.2)); } // jambières croisées, bottes fourrées
                _ => { let pants = if i % 2 == 0 { [0.18, 0.22, 0.36] } else { [0.6, 0.48, 0.3] }; b(l, [-0.11, -0.52, -0.11, 0.11, 0.0, 0.11], pants); b(l, [-0.115, -0.58, -0.115, 0.115, -0.5, 0.115], dk(pants, 1.25)); // toile retroussée
                    if i % 3 == 0 { b(l, [-0.08, -0.72, -0.08, 0.08, -0.58, 0.08], sk); b(l, [-0.09, -0.76, -0.1, 0.09, -0.72, 0.16], sk); } else { b(l, [-0.115, -0.76, -0.12, 0.115, -0.58, 0.17], [0.16, 0.12, 0.1]); } } }
            o.push((leak(format!("folkLeg{i}r{region}")), leg));
        }
        // Six têtes par région.
        let hairs: [[f64; 3]; 3] = match region { 1 => [[0.08, 0.07, 0.06], [0.14, 0.1, 0.08], [0.1, 0.08, 0.07]], 2 => [[0.82, 0.68, 0.36], [0.62, 0.28, 0.12], [0.48, 0.34, 0.2]], _ => [[0.3, 0.2, 0.12], [0.15, 0.12, 0.1], [0.6, 0.45, 0.25]] };
        for k in 0..6 { let hair = hairs[k % 3]; let sk = tones[k % 3]; let bearded = k == 1 || k == 4 || (region == 2 && k != 3);
            let mut h = VBuf::new(600); let q = &mut h;
            skull(q, sk);
            for sx in [-1.0, 1.0] { let x0 = if sx < 0.0 { -0.12 } else { 0.05 };
                b(q, [x0, 0.17, 0.19, x0 + 0.07, 0.22, 0.195], [0.95, 0.94, 0.9]); b(q, [x0 + if sx < 0.0 { 0.03 } else { 0.01 }, 0.17, 0.195, x0 + if sx < 0.0 { 0.06 } else { 0.04 }, 0.215, 0.2], [0.14, 0.1, 0.08]);
                b(q, [x0 - 0.005, 0.245, 0.19, x0 + 0.075, 0.27, 0.2], dk(hair, 0.8)); }
            b(q, [-0.03, 0.1, 0.19, 0.03, 0.18, 0.225], dk(sk, 0.9)); b(q, [-0.06, 0.055, 0.19, 0.06, 0.075, 0.198], [0.5, 0.28, 0.26]);
            if bearded { let long = region == 2; b(q, [-0.17, if long { -0.16 } else { -0.06 }, 0.08, 0.17, 0.045, if long { 0.235 } else { 0.21 }], hair); b(q, [-0.2, -0.02, -0.02, -0.15, 0.2, 0.18], hair); b(q, [0.15, -0.02, -0.02, 0.2, 0.2, 0.18], hair); b(q, [-0.08, 0.08, 0.19, 0.08, 0.1, 0.208], hair);
                if long { b(q, [-0.05, -0.24, 0.14, 0.05, -0.15, 0.24], hair); b(q, [-0.04, -0.25, 0.145, 0.04, -0.22, 0.245], [0.6, 0.55, 0.45]); } } // barbe tressée, bague
            match region {
                1 => { // chèche enroulé, ou foulard qui couvre la nuque
                    let tc = [[0.92, 0.9, 0.84], [0.2, 0.26, 0.5], [0.66, 0.24, 0.18]][k % 3];
                    if k < 3 { b(q, [-0.21, 0.3, -0.21, 0.21, 0.42, 0.21], tc); b(q, [-0.2, 0.42, -0.2, 0.2, 0.5, 0.2], dk(tc, 0.92)); b(q, [-0.16, 0.5, -0.16, 0.16, 0.56, 0.16], tc); b(q, [-0.215, 0.36, 0.2, 0.215, 0.39, 0.215], dk(tc, 0.8)); b(q, [0.12, 0.2, -0.22, 0.2, 0.36, -0.14], tc); }
                    else { b(q, [-0.21, 0.28, -0.21, 0.21, 0.44, 0.21], tc); b(q, [-0.21, -0.12, -0.21, 0.21, 0.32, -0.12], tc); b(q, [-0.215, -0.06, -0.12, -0.19, 0.32, 0.12], tc); b(q, [0.19, -0.06, -0.12, 0.215, 0.32, 0.12], tc); } }
                2 => { // longs cheveux, tresses ; un bonnet de fourrure ou un casque de fer à nasal
                    b(q, [-0.2, 0.28, -0.2, 0.2, 0.42, 0.2], hair); b(q, [-0.21, -0.14, -0.21, 0.21, 0.32, -0.12], hair);
                    for sx in [-1.0f64, 1.0] { let x = sx * 0.17; b(q, [x - 0.035, -0.22, 0.0, x + 0.035, 0.2, 0.07], hair); b(q, [x - 0.04, -0.2, 0.0, x + 0.04, -0.17, 0.07], leather); }
                    if k % 3 == 0 { let iron = [0.52, 0.53, 0.56]; b(q, [-0.21, 0.3, -0.21, 0.21, 0.46, 0.21], iron); b(q, [-0.15, 0.46, -0.15, 0.15, 0.52, 0.15], iron); b(q, [-0.215, 0.3, -0.215, 0.215, 0.34, 0.215], dk(iron, 0.8)); b(q, [-0.02, 0.12, 0.2, 0.02, 0.32, 0.225], iron); }
                    else if k % 3 == 1 { let fur = [0.4, 0.3, 0.2]; b(q, [-0.22, 0.32, -0.22, 0.22, 0.5, 0.22], fur); b(q, [-0.23, 0.3, -0.23, 0.23, 0.36, 0.23], dk(fur, 1.2)); } }
                _ => { // bonnet de laine à revers, ou chapeau de paille
                    if k % 2 == 0 { let cap = if k < 3 { [0.7, 0.16, 0.14] } else { [0.2, 0.26, 0.45] }; b(q, [-0.2, 0.3, -0.2, 0.2, 0.48, 0.2], cap); b(q, [-0.21, 0.28, -0.21, 0.21, 0.36, 0.21], dk(cap, 0.8)); b(q, [-0.12, 0.48, -0.12, 0.12, 0.54, 0.12], cap); }
                    else { let straw = [0.82, 0.7, 0.42]; b(q, [-0.2, 0.28, -0.2, 0.2, 0.42, 0.2], hair); b(q, [-0.34, 0.36, -0.34, 0.34, 0.39, 0.34], straw); b(q, [-0.18, 0.39, -0.18, 0.18, 0.5, 0.18], straw); b(q, [-0.185, 0.4, -0.185, 0.185, 0.43, 0.185], [0.5, 0.2, 0.16]); } }
            }
            o.push((leak(format!("folkHead{k}r{region}")), h));
        }
    }
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
        let world = &self.world; let blocked = |x: f64, y: f64, z: f64| { use crate::player::Voxels; [(0.22, 0.0), (-0.22, 0.0), (0.0, 0.22), (0.0, -0.22)].iter().any(|(ox, oz)| (0..2).any(|h| world.get((x + ox).floor() as i32, y.floor() as i32 + h, (z + oz).floor() as i32).is_some_and(|b| b != 0 && crate::blocks::is_solid(b) && crate::shapes::info(b).is_none_or(|i| i.kind != crate::shapes::Kind::Door)))) };
        let ground = |x: f64, y: f64, z: f64| -> Option<f64> { use crate::player::Voxels; let (bx, bz, y0) = (x.floor() as i32, z.floor() as i32, y.round() as i32);
            let hard = |yy: i32| world.get(bx, yy, bz).map(|b| b != 0 && crate::blocks::is_solid(b));
            (y0 - 3..=y0).rev().find_map(|yy| match (hard(yy), hard(yy + 1)) { (Some(true), Some(false)) => Some((yy + 1) as f64), _ => None }) };
        self.town.update(dt, self.hour, me, talking, armed, &blocked, &ground); self.notes.append(&mut self.town.news); self.folk_doors(dt);
        if let Some(t) = &self.town.talk { if self.town.folk.get(t.who).is_none_or(|f| (f.p[0] - me[0]).hypot(f.p[2] - me[2]) > 6.0) { self.town.talk = None; } }
    }
    /// Ce qu'on trouve dans un coffre de village la première fois qu'on l'ouvre : selon le métier de la maison, tiré au sort
    /// une fois pour toutes (la graine et la place du coffre). Un coffre hors des villages est vide.
    pub(crate) fn village_loot(&self, p: [i32; 3]) -> Vec<Option<Stack>> {
        let mut out = vec![None; crate::session::CHEST_SIZE];
        // Le coffre d'une épave : l'or et les pièces d'un navire marchand, du métal, des cordages, de quoi naviguer.
        if self.gen.wreck_chest(p) {
            let mut r = crate::noise::Rng::new((p[0].wrapping_mul(73856093) ^ p[1].wrapping_mul(19349663) ^ p[2].wrapping_mul(83492791) ^ self.seed) as u32); let mut slot = 0;
            for &(id, lo, hi, pr) in &[("worn_coin", 6u32, 18u32, 1.0), ("pearl", 1, 4, 0.7), ("steel_ingot", 1, 3, 0.4), ("iron_ingot", 2, 5, 0.6), ("bronze_ingot", 1, 4, 0.5), ("rope", 2, 6, 0.7), ("rusty_scrap", 2, 6, 0.6), ("glass", 2, 5, 0.35), ("bandage", 1, 3, 0.4), ("shark_tooth", 1, 3, 0.4)] {
                if r.next() >= pr { continue; } let Some(it) = item(id) else { continue }; let n = lo + (r.next() * (hi - lo + 1) as f64) as u32; let one = def(it.id).is_some_and(|d| d.stack <= 1);
                slot = (slot + 1 + (r.next() * 3.0) as usize) % out.len(); while out[slot].is_some() { slot = (slot + 1) % out.len(); } out[slot] = Some(Stack { item: it.id, n: if one { None } else { Some(n.min(hi)) }, wear: 0 }); }
            return out;
        }
        let Some((v, h)) = self.town.here.iter().find_map(|v| v.houses.iter().find(|h| p[0] >= h.area[0] && p[0] <= h.area[2] && p[2] >= h.area[1] && p[2] <= h.area[3] && (p[1] - v.c[1]).abs() < 14).map(|h| (v, h))) else { return out };
        let table: &[(&str, u32, u32, f64)] = match h.craft {
            Some("forgeron") | Some("maitre_forgeron") => &[("coal", 3, 9, 0.9), ("iron_ore", 2, 5, 0.6), ("iron_ingot", 1, 3, 0.5), ("copper_ingot", 1, 3, 0.4), ("iron_hammer", 1, 1, 0.2), ("iron_knife", 1, 1, 0.25), ("charcoal", 2, 6, 0.4), ("bread", 1, 2, 0.5)],
            Some("armurier") => &[("iron_ingot", 1, 3, 0.6), ("leather", 2, 4, 0.6), ("iron_helmet", 1, 1, 0.15), ("wooden_shield", 1, 1, 0.3), ("rope", 1, 3, 0.4), ("coal", 2, 5, 0.6), ("cooked_meat", 1, 2, 0.4)],
            Some("tailleur_pierre") => &[("cobblestone", 6, 16, 0.9), ("stone_bricks", 4, 12, 0.7), ("chiseled_stone", 1, 4, 0.4), ("flint", 2, 5, 0.6), ("stone_hammer", 1, 1, 0.3), ("pebble", 3, 8, 0.5), ("bread", 1, 2, 0.4)],
            Some("tanneur") => &[("leather", 2, 6, 0.9), ("raw_hide", 1, 4, 0.6), ("fur", 1, 3, 0.5), ("rope", 1, 4, 0.6), ("bone_needle", 1, 1, 0.4), ("fiber", 4, 12, 0.7), ("leather_cap", 1, 1, 0.15), ("wool_white", 1, 4, 0.4)],
            Some("maitre_menuisier") => &[("planks", 6, 16, 0.9), ("stick", 4, 12, 0.8), ("rope", 1, 3, 0.4), ("axe", 1, 1, 0.2), ("crate", 1, 2, 0.3), ("barrel", 1, 1, 0.2), ("bread", 1, 2, 0.4)],
            Some(_) => &[("planks", 3, 8, 0.6), ("rope", 1, 3, 0.5), ("bread", 1, 2, 0.5), ("coal", 1, 4, 0.4)],
            None => &[("bread", 1, 3, 0.7), ("berries", 2, 6, 0.5), ("jerky", 1, 3, 0.4), ("carrot", 2, 5, 0.45), ("potato", 2, 5, 0.45), ("seeds", 3, 8, 0.5), ("candle", 1, 2, 0.35), ("torch", 2, 4, 0.5), ("wool_white", 1, 3, 0.3), ("bandage", 1, 2, 0.3), ("bowl", 1, 2, 0.3), ("cheese", 1, 1, 0.25)],
        };
        let mut r = crate::noise::Rng::new((p[0].wrapping_mul(73856093) ^ p[1].wrapping_mul(19349663) ^ p[2].wrapping_mul(83492791) ^ self.seed ^ v.id.0) as u32);
        let mut slot = 0;
        for &(id, lo, hi, pr) in table { if r.next() >= pr { continue; } let Some(it) = item(id) else { continue }; let n = lo + (r.next() * (hi - lo + 1) as f64) as u32;
            let one = def(it.id).is_some_and(|d| d.stack <= 1);
            // Un peu éparpillé dans le coffre, comme on range chez soi.
            slot = (slot + 1 + (r.next() * 3.0) as usize) % out.len(); while out[slot].is_some() { slot = (slot + 1) % out.len(); }
            out[slot] = Some(Stack { item: it.id, n: if one { None } else { Some(n.min(hi)) }, wear: 0 });
        }
        out
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
