//! Refonte des villages, phases 1 et 2 : l'identité d'un village et le générateur de plans.
//!
//! Un village a une civilisation (médiévale, désertique, côtière, nordique), une taille (hameau, petit village,
//! village développé, bourg), une spécialité (cultures, élevage, forge, artisanat, commerce), une richesse, un âge et
//! une orientation (stabilité, autosuffisance, commerce, défense, tradition, prospérité, expansion). Tout se déduit de
//! la graine : deux villages ne sont jamais identiques, et un village reste le même d'un chargement à l'autre.
//!
//! Le plan se trace dans l'ordre du prompt maître : (1) le site, déjà jugé sec et assez plat par `village` ;
//! (2) l'identité ; (3) les routes, qui partent de la place et suivent le relief (pentes douces, pas d'eau, de légères
//! courbes), puis les ruelles qui s'en détachent ; (4) les bâtiments essentiels près du centre (auberge, forge,
//! ateliers, chapelle), (5) les maisons le long des rues, les granges et les champs en lisière, les métiers odorants
//! à l'écart ; (6) la validation : pas de chevauchement, pas de bâtiment sur la route ou la place, un terrain assez
//! plat et sec sous chaque bâtiment, une porte reliée à la rue par un chemin, rien hors de la portée du village.
//! Seule la civilisation médiévale passe pour l'instant par ce générateur.
use crate::batisse::{Lot, Work};
use crate::edifice::{Arch, Cond, Made, Spec};
use crate::gen::WorldGen;
use crate::noise::hash_float;
use crate::village::{id, shape, House, Pad, Site, Village, BLEND, SPAN};
use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::{PI, TAU};

#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum Civ { Medieval, Desert, Coastal, Nordic }
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub enum Size { Hamlet, Small, Developed, Borough }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum Focus { Farming, Livestock, Smithing, Crafts, Trade }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum Outlook { Stability, SelfSufficiency, Trade, Defense, Tradition, Prosperity, Expansion }
/// L'identité d'un village.
#[derive(Clone, Copy, Debug)]
pub struct Identity { pub civ: Civ, pub size: Size, pub focus: Focus, pub outlook: Outlook, /** 0 pauvre, 1 riche */ pub wealth: f64, /** ans depuis la fondation */ pub age: u32 }
impl Size {
    /// Nombre de bâtiments visé (configurable ici).
    pub fn range(self) -> (usize, usize) { match self { Size::Hamlet => (4, 7), Size::Small => (8, 16), Size::Developed => (17, 30), Size::Borough => (30, 55) } }
    pub fn label(self) -> &'static str { match self { Size::Hamlet => "hameau", Size::Small => "petit village", Size::Developed => "village", Size::Borough => "bourg" } }
}
impl Identity {
    /// Une phrase pour le journal : « bourg commerçant, prospère, fondé il y a 240 ans ».
    pub fn describe(&self) -> String {
        let f = match self.focus { Focus::Farming => "agricole", Focus::Livestock => "d'éleveurs", Focus::Smithing => "de forgerons", Focus::Crafts => "d'artisans", Focus::Trade => "marchand" };
        let w = if self.wealth > 0.7 { "prospère" } else if self.wealth < 0.3 { "modeste" } else { "aisé" };
        let o = match self.outlook { Outlook::Stability => "paisible", Outlook::SelfSufficiency => "qui vit de peu", Outlook::Trade => "tourné vers le commerce", Outlook::Defense => "sur ses gardes", Outlook::Tradition => "attaché à ses traditions", Outlook::Prosperity => "qui fait fortune", Outlook::Expansion => "qui s'agrandit" };
        format!("{} {f}, {w}, {o}, fondé il y a {} ans", self.size.label(), self.age)
    }
}
/// La civilisation d'un type de village.
pub fn civ_of(kind: &str) -> Civ { match kind { "desert" => Civ::Desert, "port" => Civ::Coastal, "taiga" | "montagne" => Civ::Nordic, _ => Civ::Medieval } }
/// L'identité d'un village, tirée de sa graine et de son biome.
pub fn identity(h: &dyn Fn(i32) -> f64, kind: &str) -> Identity {
    let r = h(1); let size = if r < 0.3 { Size::Hamlet } else if r < 0.7 { Size::Small } else if r < 0.92 { Size::Developed } else { Size::Borough };
    let wealth = (0.1 + 0.8 * h(2) + if size == Size::Borough { 0.15 } else { 0.0 } - if size == Size::Hamlet { 0.15 } else { 0.0 }).clamp(0.05, 0.95);
    let age = 40 + (h(3) * 360.0) as u32;
    let f = h(4); let focus = match kind { "hautes_terres" => if f < 0.5 { Focus::Livestock } else if f < 0.75 { Focus::Farming } else { Focus::Crafts }, "aride" => if f < 0.4 { Focus::Trade } else if f < 0.7 { Focus::Livestock } else { Focus::Farming },
        _ => if f < 0.4 { Focus::Farming } else if f < 0.55 { Focus::Livestock } else if f < 0.7 { Focus::Smithing } else if f < 0.85 { Focus::Crafts } else { Focus::Trade } };
    let o = h(5); let outlook = if size == Size::Hamlet && o < 0.45 { Outlook::SelfSufficiency } else if focus == Focus::Trade && o < 0.6 { Outlook::Trade } else if age > 280 && o < 0.5 { Outlook::Tradition } else if wealth > 0.7 && o < 0.55 { Outlook::Prosperity }
        else { [Outlook::Stability, Outlook::Expansion, Outlook::Defense, Outlook::Stability, Outlook::Tradition, Outlook::Expansion][(h(6) * 6.0) as usize % 6] };
    Identity { civ: civ_of(kind), size, focus, outlook, wealth, age }
}

fn dir(f: u8) -> (i32, i32) { [(0, 1), (1, 0), (0, -1), (-1, 0)][f as usize % 4] }
/// Une route : ses points (tous les deux blocs), la route d'où elle part (et à quel point), sa demi-largeur.
struct Road { pts: Vec<[f64; 2]>, parent: Option<(usize, usize)>, half: f64 }
/// Ce que le village demande : une fiche, et où elle se plaît (0 au centre, 1 au milieu, 2 en lisière, 3 à l'écart).
struct Want { spec: Spec, pref: u8 }
struct Placed { spec: Spec, lot: Lot, by: i32, body: [i32; 4], made: Made, road: usize, at: usize, link: Vec<(i32, i32)> }

/// La porte attendue d'une fiche : côté et case le long du mur (pour aligner la façade sur la rue avant de bâtir).
fn door_of(sp: &Spec) -> (u8, i32) { match sp.arch { Arch::Forge => (0, sp.w - 2), Arch::Mill => (0, 1), Arch::TimberHouse if sp.door_side == 1 => (1, sp.d / 2), _ => (0, sp.w / 2) } }

/// Trace un village médiéval : identité, routes, bâtiments, champs, décor ; rend le site à poser dans le monde.
#[allow(clippy::too_many_arguments)]
pub(crate) fn medieval(g: &WorldGen, gx: i32, gz: i32, x: i32, y0: i32, z: i32, kind: &'static str, crafts: Vec<&'static str>, name: String) -> Site {
    let s = g.seed; let h = |k: i32| hash_float(gx, gz, s, 900 + k); let ident = identity(&h, kind);
    let cache: RefCell<HashMap<(i32, i32), (i32, bool)>> = RefCell::new(HashMap::new());
    let col = |cx: i32, cz: i32| -> (i32, bool) { if let Some(v) = cache.borrow().get(&(cx, cz)) { return *v; } let c = g.column(cx as f64 + 0.5, cz as f64 + 0.5); let v = (c.h.ceil() as i32 - 1, c.water != 0.0 || c.cliff > 0.6); cache.borrow_mut().insert((cx, cz), v); v };
    let (fx, fz) = (x as f64 + 0.5, z as f64 + 0.5); let reach = (SPAN - BLEND - 5) as f64;
    // ---- la place
    let pr = match ident.size { Size::Hamlet => 3.6, Size::Small => 5.6, Size::Developed => 6.6, Size::Borough => 7.6 };
    // ---- les routes : d'abord les grands axes, puis les ruelles
    let (n_main, len) = match ident.size { Size::Hamlet => (2, 26.0), Size::Small => (2 + (h(10) < 0.5) as usize, 40.0), Size::Developed => (3, 52.0), Size::Borough => (4, 60.0) };
    let a0 = h(11) * TAU; let mut roads: Vec<Road> = Vec::new();
    let trace = |start: [f64; 2], a_aim: f64, len: f64, seed: i32| -> Vec<[f64; 2]> {
        let mut p = start; let mut a = a_aim; let mut pts = vec![p]; let mut walked = 0.0; let phase = hash_float(seed, 3, s, 911) * TAU;
        while walked < len {
            let wob = 0.32 * (walked * 0.07 + phase).sin(); let hp = col(p[0].floor() as i32, p[1].floor() as i32).0; let mut best: Option<(f64, [f64; 2], f64)> = None;
            for da in [-0.45, -0.22, 0.0, 0.22, 0.45] { let na = a + da; let q = [p[0] + na.cos() * 2.0, p[1] + na.sin() * 2.0];
                if (q[0] - fx).hypot(q[1] - fz) > reach - 2.0 { continue; } let (hq, wet) = col(q[0].floor() as i32, q[1].floor() as i32); if wet { continue; }
                let dh = (hq - hp).abs() as f64; if dh > 2.0 { continue; }
                let cost = dh * 2.2 + ((na - a_aim - wob + PI).rem_euclid(TAU) - PI).abs() * 1.4 + da.abs() * 0.6;
                if best.is_none_or(|b| cost < b.0) { best = Some((cost, q, na)); } }
            let Some((_, q, na)) = best else { break }; p = q; a = na; pts.push(p); walked += 2.0;
        }
        pts
    };
    for i in 0..n_main { let a = a0 + TAU * i as f64 / n_main as f64 + (h(12 + i as i32) - 0.5) * 0.5; let st = [fx + a.cos() * (pr - 0.4), fz + a.sin() * (pr - 0.4)];
        let pts = trace(st, a, len * (0.75 + 0.35 * h(20 + i as i32)), i as i32 + gx * 31 + gz * 17); if pts.len() >= 4 { roads.push(Road { pts, parent: None, half: if ident.size >= Size::Developed { 1.3 } else { 1.0 } }); } }
    if ident.size >= Size::Developed { let mains = roads.len();
        for r in 0..mains { for k in 0..if ident.size == Size::Borough { 2 } else { 1 } { let n = roads[r].pts.len(); if n < 12 { continue; }
            let at = ((0.35 + 0.3 * h(40 + (r * 2 + k) as i32)) * n as f64) as usize; let (p, q) = (roads[r].pts[at], roads[r].pts[at - 1]); let ta = (p[1] - q[1]).atan2(p[0] - q[0]);
            let side = if (k + r) % 2 == 0 { 1.0 } else { -1.0 }; let a = ta + side * (PI * 0.5 + (h(50 + r as i32) - 0.5) * 0.5);
            let pts = trace(p, a, 16.0 + 12.0 * h(60 + (r * 2 + k) as i32), 100 + (r * 2 + k) as i32 + gx * 7); if pts.len() >= 4 { roads.push(Road { pts, parent: Some((r, at)), half: 0.8 }); } } } }
    // Les cases de route et de place.
    let mut road_cells: HashMap<(i32, i32), (usize, bool)> = HashMap::new(); // (route, grand axe)
    for (ri, r) in roads.iter().enumerate() { for w in r.pts.windows(2) { let n = 8; for k in 0..=n { let t = k as f64 / n as f64; let (px, pz) = (w[0][0] + (w[1][0] - w[0][0]) * t, w[0][1] + (w[1][1] - w[0][1]) * t);
        for dx in -2..=2 { for dz in -2..=2 { let (cx, cz) = (px.floor() as i32 + dx, pz.floor() as i32 + dz); if (cx as f64 + 0.5 - px).hypot(cz as f64 + 0.5 - pz) <= r.half + 0.15 { road_cells.entry((cx, cz)).or_insert((ri, r.parent.is_none())); } } } } } }
    let in_plaza = |cx: i32, cz: i32| (cx as f64 + 0.5 - fx).hypot(cz as f64 + 0.5 - fz) <= pr + 0.6;
    // ---- ce que le village demande, du plus essentiel au plus accessoire
    let mut wants: Vec<Want> = Vec::new(); let mut salt = 200; let mut nh = || { salt += 1; h(salt) };
    let cond = |r: f64| -> Cond { let aged = ident.age as f64 / 400.0; if r < 0.05 * (1.0 - ident.wealth) { Cond::Damaged } else if r < 0.05 + aged * 0.45 { Cond::Old } else if r < 0.12 + aged * 0.45 { Cond::Repaired } else if r < 0.6 { Cond::Kept } else { Cond::New } };
    let mk = |arch: Arch, hh: f64, craft: Option<&'static str>, c: Cond| Spec::new(arch, (hh * 97.0) as u8, hh, (ident.wealth + (hh - 0.5) * 0.3).clamp(0.0, 1.0), craft, c);
    if ident.size >= Size::Small || ident.focus == Focus::Trade { let hh = nh(); wants.push(Want { spec: mk(Arch::Inn, hh, None, cond(nh())), pref: 0 }); }
    for c in &crafts { let hh = nh(); let arch = if matches!(*c, "forgeron" | "maitre_forgeron" | "armurier") { Arch::Forge } else { Arch::Workshop }; let pref = if *c == "tanneur" { 3 } else if arch == Arch::Forge { 0 } else { 1 };
        wants.push(Want { spec: mk(arch, hh, Some(c), Cond::Kept), pref }); }
    if ident.size >= Size::Developed || (ident.size == Size::Small && (ident.outlook == Outlook::Tradition || h(70) < 0.35)) { wants.push(Want { spec: mk(Arch::Chapel, nh(), None, Cond::Kept), pref: 0 }); }
    let (lo, hi) = ident.size.range(); let target = lo + ((hi - lo + 1) as f64 * h(71)) as usize;
    let barns = match ident.focus { Focus::Farming | Focus::Livestock => 2 + (ident.size >= Size::Developed) as usize * 2, _ => 1 + (ident.size >= Size::Developed) as usize }.min(target / 3).max(1);
    let mills = (ident.size >= Size::Small && h(72) < 0.6) as usize;
    let homes = target.saturating_sub(wants.len() + barns + mills).max(2);
    for i in 0..homes { let hh = nh(); let centre = i < homes / 3; let timber = hh < if centre { 0.35 + 0.5 * ident.wealth } else { 0.15 + 0.3 * ident.wealth };
        wants.push(Want { spec: mk(if timber { Arch::TimberHouse } else { Arch::Cottage }, nh(), None, cond(nh())), pref: if centre { 0 } else if timber { 1 } else { 2 } }); }
    for _ in 0..barns { wants.push(Want { spec: mk(Arch::Barn, nh(), None, cond(nh())), pref: 3 }); }
    for _ in 0..mills { wants.push(Want { spec: mk(Arch::Mill, nh(), None, Cond::Kept), pref: 3 }); }
    // L'histoire du village : une maison en chantier quand il s'agrandit, une ruine quand il décline.
    if ident.outlook == Outlook::Expansion || (ident.outlook == Outlook::Prosperity && h(73) < 0.5) { wants.push(Want { spec: mk(if h(74) < 0.5 { Arch::Cottage } else { Arch::TimberHouse }, nh(), None, Cond::Building), pref: 2 }); }
    if ident.size >= Size::Small && ident.wealth < 0.45 && ident.outlook != Outlook::Expansion && h(75) < 0.6 { wants.push(Want { spec: mk(Arch::Cottage, nh(), None, Cond::Abandoned), pref: 3 }); }
    // ---- le placement : pour chaque demande, l'emplacement libre le mieux venu le long des rues
    let total: f64 = roads.iter().map(|r| r.pts.len() as f64 * 2.0).fold(0.0, f64::max) + pr;
    let mut placed: Vec<Placed> = Vec::new(); let mut taken: Vec<[i32; 4]> = Vec::new();
    let mut cands: Vec<(usize, usize, f64)> = Vec::new(); // (route, point, côté)
    for (ri, r) in roads.iter().enumerate() { for k in 1..r.pts.len() - 1 { for side in [-1.0, 1.0] { cands.push((ri, k, side)); } } }
    let world = |lot: &Lot, u: i32, ww: i32| { let c = lot.cell(u, 0, ww); (c[0], c[2]) };
    let rect = |lot: &Lot, e: [i32; 4]| { let (a, b) = (world(lot, e[0], e[1]), world(lot, e[2], e[3])); [a.0.min(b.0), a.1.min(b.1), a.0.max(b.0), a.1.max(b.1)] };
    let mut why = [0usize; 7];
    for (wi, want) in wants.iter().enumerate() {
        let sp = want.spec; let ext = sp.extent(); let (ds, dt) = door_of(&sp);
        let mut order: Vec<(f64, usize)> = cands.iter().enumerate().map(|(i, c)| { let p = roads[c.0].pts[c.1]; let dc = (p[0] - fx).hypot(p[1] - fz);
            let base = match want.pref { 0 => dc, 1 => (dc - total * 0.5).abs(), 2 => (total * 0.85 - dc).abs(), _ => total - dc + if roads[c.0].parent.is_some() { -4.0 } else { 0.0 } };
            (base + hash_float(i as i32, wi as i32, s, 913) * 3.0, i) }).collect();
        order.sort_by(|a, b| a.0.total_cmp(&b.0));
        'cand: for (_, ci) in order {
            let (ri, k, side) = cands[ci]; let r = &roads[ri]; let p = r.pts[k]; let (q0, q1) = (r.pts[k - 1], r.pts[k + 1]);
            let t = [q1[0] - q0[0], q1[1] - q0[1]]; let n = [-t[1] * side, t[0] * side]; let fo = crate::shapes::facing_of(n[0], n[1]);
            let (uf, wf) = if ds == 0 { (fo + 1, fo) } else { (fo + 2, fo + 1) }; let (ux, uz) = dir(uf); let (wx, wz) = dir(wf); let (nx, nz) = dir(fo);
            let front = if ds == 0 { -ext[1] - 1 } else { ext[2] - sp.w }; let off0 = r.half.ceil() as i32 + 2 + front;
            'back: for setback in 0..5 { let off = off0 + setback;
            let (px, pz) = (p[0].floor() as i32 + nx * off, p[1].floor() as i32 + nz * off);
            let o = if ds == 0 { (px - dt * ux, pz - dt * uz) } else { (px - (sp.w - 1) * ux - dt * wx, pz - (sp.w - 1) * uz - dt * wz) };
            let lot = Lot { o, u: (ux, uz), w: (wx, wz) }; let area = rect(&lot, ext); let body = rect(&lot, [0, 0, sp.w - 1, sp.d - 1]);
            for (cx, cz) in [(area[0], area[1]), (area[2], area[1]), (area[0], area[3]), (area[2], area[3])] { if ((cx - x) as f64).hypot((cz - z) as f64) > reach { why[0] += 1; continue 'back; } }
            if taken.iter().any(|t| area[0] <= t[2] + 1 && area[2] + 1 >= t[0] && area[1] <= t[3] + 1 && area[3] + 1 >= t[1]) { why[1] += 1; continue 'back; }
            for cx in area[0]..=area[2] { for cz in area[1]..=area[3] { if road_cells.contains_key(&(cx, cz)) || in_plaza(cx, cz) { why[2] += 1; continue 'back; } } }
            let mut hs = Vec::new(); for cx in area[0]..=area[2] { for cz in area[1]..=area[3] { let (hh, wet) = col(cx, cz); if wet { why[3] += 1; continue 'back; } if cx >= body[0] && cx <= body[2] && cz >= body[1] && cz <= body[3] { hs.push(hh); } } }
            hs.sort(); let by = hs[hs.len() / 2]; let tol = if sp.arch == Arch::Barn { 5 } else { 4 }; if hs[hs.len() - 1] - hs[0] > tol + 1 || hs[hs.len() - 1] - by > tol || by - hs[0] > tol || (by - y0).abs() > 5 { why[4] += 1; continue 'back; }
            // La porte rejoint la rue : un chemin droit, qui ne traverse aucun autre bâtiment.
            let door_out = if ds == 0 { world(&lot, dt, -1) } else { world(&lot, sp.w, dt) }; let mut link = Vec::new(); let mut c = door_out; let mut ok = false;
            for _ in 0..off + 3 { if road_cells.contains_key(&c) || in_plaza(c.0, c.1) { ok = true; break; } if taken.iter().any(|t| c.0 >= t[0] && c.0 <= t[2] && c.1 >= t[1] && c.1 <= t[3]) { break; } link.push(c); c = (c.0 - nx, c.1 - nz); }
            if !ok && sp.arch != Arch::Mill { why[5] += 1; continue 'back; }
            if ok && (col(c.0, c.1).0 - by).abs() > 1 + link.len() as i32 / 2 { why[6] += 1; continue 'back; } // la rue est à portée de marche de la porte
            taken.push(area); placed.push(Placed { spec: sp, lot, by, body, made: Made::default(), road: ri, at: k, link }); break 'cand; }
        }
    }
    if std::env::var("ALW_BOURG").is_ok() { eprintln!("{gx},{gz} {:?} routes {} ({:?}) demandes {} posés {} refus {:?}", ident.size, roads.len(), roads.iter().map(|r| r.pts.len()).collect::<Vec<_>>(), wants.len(), placed.len(), why); }
    // ---- les assises, puis les bâtiments
    let mut pads = vec![Pad { c: [fx, fz], half: [pr + 0.5, pr + 0.5], round: true, h: y0 }];
    for p in &placed { let e = p.spec.extent(); let a = rect(&p.lot, e); pads.push(Pad { c: [(a[0] + a[2] + 1) as f64 * 0.5, (a[1] + a[3] + 1) as f64 * 0.5], half: [(a[2] - a[0] + 1) as f64 * 0.5 + 1.0, (a[3] - a[1] + 1) as f64 * 0.5 + 1.0], round: false, h: p.by }); }
    let nat = |cx: i32, cz: i32| col(cx, cz).0; let gy = |pads: &[Pad], cx: i32, cz: i32| shape(pads, nat(cx, cz), cx as f64 + 0.5, cz as f64 + 0.5).0;
    let mut ground = Work::default(); let mut works = Work::default(); let mut paths: Vec<[i32; 3]> = Vec::new();
    let pave = id(if ident.wealth > 0.6 { "setts" } else { "cobblestone" }); let trail = id("dirt_path"); let cob = id("cobblestone"); let grass = id("grass");
    // Les rues : pavées autour de la place dans un village aisé, en terre battue ailleurs, avec des plaques de pavés usés.
    // Les hauteurs des rues, adoucies : jamais plus d'un bloc d'écart entre deux cases voisines (on remblaie le creux).
    let mut rh: HashMap<(i32, i32), i32> = road_cells.keys().map(|&(cx, cz)| ((cx, cz), gy(&pads, cx, cz))).collect();
    for p in &placed { for &(cx, cz) in &p.link { rh.insert((cx, cz), gy(&pads, cx, cz)); } }
    for _ in 0..6 { let keys: Vec<(i32, i32)> = rh.keys().copied().collect(); for (cx, cz) in keys { let hc = rh[&(cx, cz)]; for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { if let Some(hn) = rh.get_mut(&(cx + dx, cz + dz)) { if *hn < hc - 1 { *hn = hc - 1; } } } } }
    let mut cells: Vec<(&(i32, i32), &(usize, bool))> = road_cells.iter().collect(); cells.sort();
    for (&(cx, cz), &(_, main)) in cells { let y = rh[&(cx, cz)]; for f in gy(&pads, cx, cz)..y { ground.put([cx, f, cz], id("dirt")); } let dc = ((cx - x) as f64).hypot((cz - z) as f64);
        let b = if main && ident.size >= Size::Small && dc < pr + 6.0 + ident.wealth * 10.0 { pave } else if hash_float(cx, cz, s, 915) < 0.12 && main { cob } else if hash_float(cx, cz, s, 916) < 0.06 { grass } else { trail };
        ground.put([cx, y, cz], b); paths.push([cx, y, cz]); }
    // La place : pavée (en terre battue au hameau), le puits au milieu, des lanternes, les étals du marché.
    let plaza_mat = if ident.size == Size::Hamlet { trail } else { pave };
    let r2 = (pr + 0.6).ceil() as i32; for dx in -r2..=r2 { for dz in -r2..=r2 { if in_plaza(x + dx, z + dz) { ground.put([x + dx, y0, z + dz], plaza_mat); } } }
    let water = id("water"); for dx in -1..=1 { for dz in -1..=1 { let edge = dx != 0 || dz != 0; for (y, b) in [(y0, if edge { cob } else { water }), (y0 - 1, if edge { cob } else { water }), (y0 - 2, cob)] { ground.put([x + dx, y, z + dz], b); } if edge { ground.put([x + dx, y0 + 1, z + dz], cob); } } }
    crate::batisse::well(&mut ground, [x, y0, z], id("roof_tiles"));
    if ident.size >= Size::Small { for (dx, dz) in [(3, 3), (-3, 3), (3, -3), (-3, -3)] { let (cx, cz) = (x + dx + dx.signum(), z + dz + dz.signum()); if !road_cells.contains_key(&(cx, cz)) { ground.put([cx, y0 + 1, cz], id("fence")); ground.put([cx, y0 + 2, cz], id("lantern")); } } }
    let stalls = match (ident.size, ident.focus) { (Size::Hamlet, _) => 0, (_, Focus::Trade) => 4, (Size::Borough, _) => 4, (Size::Developed, _) => 2, _ => 1 };
    let stripes = [[[0.62, 0.14, 0.12], [0.9, 0.86, 0.78]], [[0.18, 0.28, 0.5], [0.9, 0.86, 0.78]], [[0.28, 0.42, 0.22], [0.88, 0.82, 0.64]], [[0.7, 0.48, 0.15], [0.38, 0.24, 0.13]]];
    let road_angles: Vec<f64> = roads.iter().filter(|r| r.parent.is_none()).map(|r| (r.pts[1][1] - fz).atan2(r.pts[1][0] - fx)).collect(); let mut made_stalls = 0;
    for k in 0..12 { if made_stalls >= stalls { break; } let a = h(80) * TAU + k as f64 * TAU / 12.0; if road_angles.iter().any(|ra| ((a - ra + PI).rem_euclid(TAU) - PI).abs() < 0.55) { continue; }
        let c = [fx + a.cos() * (pr - 1.4), (y0 + 1) as f64, fz + a.sin() * (pr - 1.4)]; let at = [c[0].floor() as i32, y0, c[2].floor() as i32];
        crate::batisse::stall(&mut ground, at, c, [-a.cos(), 0.0, -a.sin()], stripes[(made_stalls + (h(81) * 4.0) as usize) % 4], h(82 + k)); made_stalls += 1; }
    // Les bâtiments, et leurs chemins jusqu'à la rue.
    for p in placed.iter_mut() { p.made = crate::edifice::raise(&mut works, p.lot, &p.spec, p.by);
        for &(cx, cz) in &p.link { let y = rh[&(cx, cz)]; for f in gy(&pads, cx, cz)..y { ground.put([cx, f, cz], id("dirt")); } ground.put([cx, y, cz], trail); paths.push([cx, y, cz]); } }
    // Le décor des rues : des lanternes sur poteau le long des grands axes, des bancs près de la place.
    if ident.size >= Size::Small { for r in roads.iter().filter(|r| r.parent.is_none()) { for k in (4..r.pts.len()).step_by(6) { let p = r.pts[k]; let q = r.pts[k - 1]; let t = [p[0] - q[0], p[1] - q[1]]; let l = t[0].hypot(t[1]).max(1e-6); let side = if k % 12 == 4 { 1.0 } else { -1.0 };
        let (cx, cz) = ((p[0] - t[1] / l * side * (r.half + 1.2)).floor() as i32, (p[1] + t[0] / l * side * (r.half + 1.2)).floor() as i32);
        if road_cells.contains_key(&(cx, cz)) || taken.iter().any(|t| cx >= t[0] - 1 && cx <= t[2] + 1 && cz >= t[1] - 1 && cz <= t[3] + 1) || col(cx, cz).1 { continue; }
        let y = gy(&pads, cx, cz); works.put([cx, y + 1, cz], id("fence")); works.put([cx, y + 2, cz], id("fence")); works.put([cx, y + 3, cz], id("lantern")); } } }
    // ---- les champs, derrière les granges et en lisière
    let fields = match ident.focus { Focus::Farming => 4, Focus::Livestock => 2, _ => 2 } + (ident.size >= Size::Developed) as usize;
    let mut made_fields = 0; let farmland = id("farmland"); let fence = id("fence");
    let anchors: Vec<(f64, f64)> = placed.iter().filter(|p| p.spec.arch == Arch::Barn).map(|p| { let c = p.lot.p(p.spec.w as f64 * 0.5, 0.0, p.spec.d as f64 + 6.0); (c[0], c[2]) }).chain((0..12).map(|k| { let a = h(90) * TAU + k as f64 * 0.52; (fx + a.cos() * (total * 0.8), fz + a.sin() * (total * 0.8)) })).collect();
    for (k, (ax, az)) in anchors.iter().enumerate() { if made_fields >= fields { break; }
        let along_x = h(91 + k as i32) < 0.5; let (hx, hz) = if along_x { (4, 3) } else { (3, 4) }; let (cx, cz) = (ax.floor() as i32, az.floor() as i32); let area = [cx - hx, cz - hz, cx + hx, cz + hz];
        if [(area[0], area[1]), (area[2], area[3]), (area[0], area[3]), (area[2], area[1])].iter().any(|c| ((c.0 - x) as f64).hypot((c.1 - z) as f64) > reach) { continue; }
        if taken.iter().any(|t| area[0] <= t[2] + 2 && area[2] + 2 >= t[0] && area[1] <= t[3] + 2 && area[3] + 2 >= t[1]) { continue; }
        let mut hs = Vec::new(); let mut bad = false; for a in area[0]..=area[2] { for b in area[1]..=area[3] { let (hh, wet) = col(a, b); bad |= wet || road_cells.contains_key(&(a, b)) || in_plaza(a, b); hs.push(hh); } }
        if bad { continue; } hs.sort(); let fy = hs[hs.len() / 2]; if hs[hs.len() - 1] - hs[0] > 5 || (fy - y0).abs() > 6 { continue; }
        taken.push(area); pads.push(Pad { c: [(area[0] + area[2] + 1) as f64 * 0.5, (area[1] + area[3] + 1) as f64 * 0.5], half: [(hx + 1) as f64 + 0.5, (hz + 1) as f64 + 0.5], round: false, h: fy });
        let crop = id(["wheat_3", "carrot_3", "potato_3", "wheat_3"][(h(95 + k as i32) * 4.0) as usize % 4]); let (gxx, gzz) = if along_x { (cx, area[1]) } else { (area[0], cz) };
        for a in area[0]..=area[2] { for b in area[1]..=area[3] { let edge = a == area[0] || a == area[2] || b == area[1] || b == area[3];
            if edge { works.put([a, fy, b], id("dirt")); if (a, b) != (gxx, gzz) { works.put([a, fy + 1, b], fence); } continue; }
            let mid = if along_x { b == cz } else { a == cx }; if mid { works.put([a, fy, b], water); works.put([a, fy - 1, b], id("dirt")); } else { works.put([a, fy, b], farmland); works.put([a, fy + 1, b], crop); } } }
        made_fields += 1; }
    // ---- le village : ses maisons habitées, leurs chemins jusqu'à la place
    let mut v = Village { id: (gx, gz), kind, name, c: [x, y0, z], r: 8, crafts: Vec::new(), houses: Vec::new(), ident: Some(ident) };
    for p in &placed { for (cx, cz) in [(p.body[0] - 2, p.body[1] - 2), (p.body[2] + 2, p.body[3] + 2)] { v.r = v.r.max(((cx - x) as f64).hypot((cz - z) as f64).ceil() as i32 + 2); } }
    for t in &taken { for (cx, cz) in [(t[0], t[1]), (t[2], t[3]), (t[0], t[3]), (t[2], t[1])] { v.r = v.r.max(((cx - x) as f64).hypot((cz - z) as f64).ceil() as i32 + 1); } }
    v.r = v.r.min(SPAN - 6);
    for p in &placed {
        if !p.spec.dwelling() { continue; } let Some((ds, dt)) = p.made.door else { continue };
        let lot = p.lot; let fy = (p.by + 1) as f64; let mid = |u: i32, ww: i32| { let c = lot.cell(u, 0, ww); [c[0] as f64 + 0.5, fy, c[2] as f64 + 0.5] };
        let (door, inside) = if ds == 0 { (mid(dt, -1), mid(dt, 1)) } else { (mid(p.spec.w, dt), mid(p.spec.w - 2, dt)) };
        let up = |c: (i32, i32)| [c.0 as f64 + 0.5, (rh.get(&c).copied().unwrap_or_else(|| gy(&pads, c.0, c.1)) + 1) as f64, c.1 as f64 + 0.5];
        let mut way: Vec<[f64; 3]> = p.link.iter().skip(1).map(|c| up(*c)).collect();
        let push_road = |ri: usize, from: usize, way: &mut Vec<[f64; 3]>| { let r = &roads[ri]; for k in (0..=from).rev() { let q = r.pts[k]; way.push(up((q[0].floor() as i32, q[1].floor() as i32))); } };
        push_road(p.road, p.at, &mut way); if let Some((pr_, pk)) = roads[p.road].parent { push_road(pr_, pk, &mut way); }
        let ring = *way.last().unwrap_or(&door);
        let (spot, back) = match p.made.spot { Some((sp, st)) => { let a = lot.cell(sp.0, 0, sp.1); let b = lot.cell(st.0, 0, st.1); let d = (b[0] - a[0], b[2] - a[2]); (mid(sp.0, sp.1), [(0, 1), (1, 0), (0, -1), (-1, 0)].iter().position(|x| *x == d).unwrap_or(0) as u8) } None => (inside, crate::shapes::facing_of(lot.w.0 as f64, lot.w.1 as f64)) };
        let bed = p.made.bed.map_or(inside, |b| mid(b.0, b.1));
        if let Some(c) = p.spec.craft { if !v.crafts.contains(&c) { v.crafts.push(c); } }
        v.houses.push(House { craft: if p.made.spot.is_some() { p.spec.craft } else { None }, area: p.body, door, inside, spot, bed, ring, back, way });
    }
    // Les artisans d'abord, comme dans les anciens villages.
    v.houses.sort_by_key(|h| h.craft.is_none());
    let mut blocks = ground.blocks; blocks.extend(works.blocks); let mut props = ground.props; props.extend(works.props);
    for dx in -1..=1 { for dz in -1..=1 { let edge = dx != 0 || dz != 0; if edge { blocks.push((x + dx, y0 + 1, z + dz, cob)); } blocks.push((x + dx, y0, z + dz, if edge { cob } else { water })); } }
    Site { v, blocks, props, pads, paths }
}
