//! Le destin des villages : chacun a sa fortune, qui monte ou descend jour après jour ; les villages qui s'appauvrissent se
//! vident, ceux qui prospèrent deviennent des villes.
//!
//! La fortune (de -100 à 100) bouge chaque jour pour chaque village connu : les routes commerciales ouvertes l'enrichissent ;
//! la guerre que mène son peuple l'appauvrit, une alliance l'aide un peu ; un camp de brigands puissant tout près le saigne ;
//! la réputation du joueur y compte (on y vient, on y commerce) ; un don au chef du village l'aide d'un coup ; et un peu de
//! hasard. Elle revient doucement vers ce que permet la richesse du village.
//!
//! Cinq étapes : en ruine (≤ -60), en déclin (≤ -20), stable, prospère (≥ 35), ville (≥ 70, pour un village qui n'est pas un
//! hameau). Ce qui se voit, quand on est au village :
//! - en déclin, deux maisons sur cinq sont abandonnées (portes arrachées, vitres brisées, toiles d'araignée, herbes folles au
//!   seuil, plus personne dedans) ; en ruine, quatre sur cinq, ateliers compris. Le marché s'amenuise (la moitié des
//!   marchandises en déclin, plus rien en ruine) ;
//! - prospère, les prix baissent d'un dixième ; ville, d'un cinquième, et le village s'entoure d'un rempart de pierre crénelé,
//!   avec ses tours à lanternes, ouvert là où passent les rues.
//! Quand la fortune remonte, les maisons abandonnées se relèvent telles qu'elles étaient et leurs habitants reviennent.
//! Vérifié par `tests/destin.rs`.
use crate::batisse::id;
use crate::blocks::{Id, BLOCKS};
use crate::factions::Faction;
use crate::noise::hash_float;
use crate::player::Voxels;
use crate::politique::Bond;
use crate::session::Session;
use crate::village::Village;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage { Ruine, Declin, Stable, Prospere, Ville }
impl Stage {
    pub fn of(f: f64, hamlet: bool) -> Stage { if f <= -60.0 { Stage::Ruine } else if f <= -20.0 { Stage::Declin } else if f < 35.0 { Stage::Stable } else if f < 70.0 || hamlet { Stage::Prospere } else { Stage::Ville } }
    pub fn label(self) -> &'static str { match self { Stage::Ruine => "en ruine", Stage::Declin => "en déclin", Stage::Stable => "stable", Stage::Prospere => "prospère", Stage::Ville => "ville" } }
    /// La part des maisons abandonnées.
    pub fn empty(self) -> f64 { match self { Stage::Ruine => 0.8, Stage::Declin => 0.4, _ => 0.0 } }
    /// Ce que coûtent les marchandises (prospère, un dixième de moins ; ville, un cinquième).
    pub fn price(self, n: u32) -> u32 { match self { Stage::Prospere => ((n * 9) / 10).max(1), Stage::Ville => ((n * 4) / 5).max(1), _ => n }.min(n) }
}
/// Ce que coûte un don au chef du village, et ce qu'il rapporte à sa fortune.
pub const GIFT: u32 = 10; pub const GIFT_FORTUNE: f64 = 8.0;

/// La fortune où un village se tient de lui-même : selon sa richesse.
pub fn rest(v: &Village) -> f64 { v.ident.map_or(5.0, |i| -10.0 + i.wealth * 40.0) }
fn hamlet(v: &Village) -> bool { v.ident.is_none_or(|i| i.size == crate::bourg::Size::Hamlet) }

impl Session {
    /// La fortune d'un village (au départ, celle que permet sa richesse).
    pub fn fortune(&self, v: &Village) -> f64 { self.town.fortune.get(&v.id).copied().unwrap_or_else(|| rest(v)) }
    pub fn stage(&self, v: &Village) -> Stage { Stage::of(self.fortune(v), hamlet(v)) }
    /// Les maisons abandonnées d'un village à son étape : toujours les mêmes, dans un ordre tiré du village.
    pub fn empty_houses(v: &Village, st: Stage) -> Vec<usize> {
        let mut order: Vec<usize> = (0..v.houses.len()).filter(|&i| st == Stage::Ruine || v.houses[i].craft.is_none()).collect();
        order.sort_by(|a, b| hash_float(v.id.0, v.id.1, *a as i32, 731).total_cmp(&hash_float(v.id.0, v.id.1, *b as i32, 731)));
        let n = (v.houses.len() as f64 * st.empty()).round() as usize; order.truncate(n.min(v.houses.len().saturating_sub(1))); order
    }
    /// Une journée pour chaque village connu : la fortune bouge ; un changement d'étape se raconte.
    pub(crate) fn fate_day(&mut self, days: i64) {
        let vs: Vec<Village> = self.town.atlas.values().filter(|v| self.town.known.contains(&v.id) || self.town.here.iter().any(|h| h.id == v.id)).cloned().collect();
        let routes = self.trade_routes();
        for v in vs {
            let f = Faction::of_kind(v.kind); let open = routes.iter().filter(|r| r.open && (r.a == v.id || r.b == v.id)).count() as f64;
            let war = self.politics.at_war(f); let ally = crate::factions::ALL.iter().any(|&g| g != f && self.politics.bond(f, g) == Bond::Allies);
            let threat = self.threat(&v).map_or(0.0, |t| t.1); let standing = self.standing(v.id) as f64;
            let before = self.stage(&v); let mut x = self.fortune(&v);
            for d in 0..days.min(60) {
                let luck = hash_float(v.id.0, v.id.1, (self.politics.day - d) as i32, 733) * 2.0 - 1.0;
                x += 0.12 * open - if war { 0.8 } else { 0.0 } + if ally { 0.3 } else { 0.0 } - 0.2 * threat + standing / 70.0 + luck * 0.8 + (rest(&v) - x) * 0.02; x = x.clamp(-100.0, 100.0);
            }
            self.town.fortune.insert(v.id, x); let now = self.stage(&v);
            if now != before && self.town.known.contains(&v.id) { self.notes.push(match (now > before, now) {
                (true, Stage::Ville) => format!("{} devient une ville ! On y élève un rempart", v.name), (true, Stage::Prospere) => format!("{} prospère", v.name), (true, _) => format!("{} se relève : des familles reviennent", v.name),
                (false, Stage::Ruine) => format!("{} tombe en ruine : il n'y reste presque personne", v.name), (false, Stage::Declin) => format!("{} décline : des familles s'en vont", v.name), (false, _) => format!("{} perd de sa splendeur", v.name) }); }
        }
    }
    /// Au village : ce que son étape change dans le monde (maisons vidées ou relevées, rempart), une fois le terrain chargé.
    pub(crate) fn fate_tick(&mut self) {
        if self.elapsed - self.town.fate_t < 0.5 && self.elapsed >= self.town.fate_t { return; } self.town.fate_t = self.elapsed; // deux fois par seconde
        let here: Vec<Village> = self.town.here.clone();
        for v in here {
            let st = self.stage(&v); let want = Self::empty_houses(&v, st); let had = self.town.empty.get(&v.id).cloned().unwrap_or_default();
            if self.world.get(v.c[0], v.c[1], v.c[2]).is_none() { continue; }
            // les maisons qu'on quitte ; celles où l'on revient
            let gone: Vec<usize> = want.iter().copied().filter(|i| !had.contains(i)).collect(); let back: Vec<usize> = had.iter().copied().filter(|i| !want.contains(i)).collect();
            if gone.is_empty() && back.is_empty() && (st != Stage::Ville || self.town.walled.contains(&v.id)) { continue; }
            if ![&gone, &back].iter().all(|l| l.iter().all(|&i| { let a = v.houses[i].area; [(a[0], a[1]), (a[2], a[3])].iter().all(|c| self.world.get(c.0, v.c[1], c.1).is_some()) })) { continue; }
            for &i in &gone { self.abandon(&v, i); } for &i in &back { self.resettle(&v, i); }
            self.town.empty.insert(v.id, want.clone()); self.town.folk.retain(|f| f.v != v.id || !want.contains(&f.house));
            if !back.is_empty() { self.town.respawn(&v, self.hour, &back); }
            if st == Stage::Ville && !self.town.walled.contains(&v.id) && self.rampart(&v) { self.town.walled.push(v.id); }
            self.dirty = true;
        }
    }
    /// Une maison qu'on abandonne : les portes arrachées, les vitres brisées, des toiles d'araignée, des herbes au seuil.
    fn abandon(&mut self, v: &Village, i: usize) {
        let h = &v.houses[i]; let a = h.area; let y0 = v.c[1] - 6; let mut e: Vec<(i32, i32, i32, Id)> = Vec::new(); let web = id("cobweb"); let weed = crate::blocks::by_name("tall_grass").unwrap_or(0);
        for x in a[0]..=a[2] { for z in a[1]..=a[3] { for y in y0..y0 + 24 { let Some(b) = self.world.get(x, y, z) else { continue }; let n = BLOCKS[b as usize].name;
            if n.starts_with("door") || n.starts_with("window") || n.starts_with("shutter") || n.contains("lantern") || n.starts_with("chandelier") { e.push((x, y, z, 0)); }
            else if b == 0 && hash_float(x, y * 7 + z, v.id.0, 735) < 0.04 && self.world.get(x, y + 1, z).is_some_and(|u| u != 0) { e.push((x, y, z, web)); } } } }
        let d = h.door; let (dx, dz) = (d[0].floor() as i32, d[2].floor() as i32); for (ox, oz) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] { let y = d[1].floor() as i32; if weed != 0 && self.world.get(dx + ox, y, dz + oz) == Some(0) { e.push((dx + ox, y, dz + oz, weed)); } }
        self.world.edit_many(&e);
    }
    /// Une maison où l'on revient : on la relève telle que le village l'avait bâtie.
    fn resettle(&mut self, v: &Village, i: usize) {
        let a = v.houses[i].area; let Some(site) = self.gen.site(v.id.0, v.id.1) else { return };
        let e: Vec<(i32, i32, i32, Id)> = site.blocks.iter().copied().filter(|b| b.0 >= a[0] - 1 && b.0 <= a[2] + 1 && b.2 >= a[1] - 1 && b.2 <= a[3] + 1).collect();
        let web = id("cobweb"); let mut clear: Vec<(i32, i32, i32, Id)> = Vec::new();
        for x in a[0] - 1..=a[2] + 1 { for z in a[1] - 1..=a[3] + 1 { for y in v.c[1] - 6..v.c[1] + 18 { let b = self.world.get(x, y, z); if b == Some(web) || (b == crate::blocks::by_name("tall_grass") && !e.iter().any(|q| q.0 == x && q.1 == y && q.2 == z)) { clear.push((x, y, z, 0)); } } } }
        clear.extend(e); self.world.edit_many(&clear);
    }
    /// Le rempart d'une ville : un mur de pierre crénelé en cercle autour des maisons, une tour à lanterne de loin en loin,
    /// ouvert là où passent les rues. Rend faux si le terrain n'est pas encore là.
    fn rampart(&mut self, v: &Village) -> bool {
        let rr = v.houses.iter().flat_map(|h| [(h.area[0], h.area[1]), (h.area[2], h.area[3])]).map(|(x, z)| ((x - v.c[0]) as f64).hypot((z - v.c[2]) as f64)).fold(10.0, f64::max) + 4.0;
        // ce que le village a bâti (rues, place, maisons, champs) : le rempart s'ouvre là
        let built: std::collections::HashSet<(i32, i32)> = self.gen.site(v.id.0, v.id.1).map(|s| s.blocks.iter().filter(|b| b.3 != 0).map(|b| (b.0, b.2)).collect()).unwrap_or_default();
        let (cx, cz) = (v.c[0] as f64 + 0.5, v.c[2] as f64 + 0.5); let mut e: Vec<(i32, i32, i32, Id)> = Vec::new(); let mut done = std::collections::HashSet::new();
        let (wall, tower, lamp) = (id("stone_bricks"), id("cobblestone"), id("lantern")); let n = (std::f64::consts::TAU * (rr + 1.0)).ceil() as i32;
        let ground = |s: &Session, x: i32, z: i32| -> Option<i32> { (v.c[1] - 20..v.c[1] + 30).rev().find(|&y| s.world.get(x, y, z).is_some_and(|b| b != 0 && BLOCKS[b as usize].solid)) };
        for i in 0..n { let a = i as f64 / n as f64 * std::f64::consts::TAU; let is_tower = i % 24 == 0;
            for (ring, outer) in [(rr, false), (rr + 1.0, true)] { let (x, z) = ((cx + a.cos() * ring).floor() as i32, (cz + a.sin() * ring).floor() as i32); if !done.insert((x, z)) { continue; }
                if self.world.get(x, v.c[1], z).is_none() { return false; } if built.contains(&(x, z)) { continue; }
                let Some(g) = ground(self, x, z) else { continue }; let gb = self.world.get(x, g, z).unwrap_or(0); let gn = BLOCKS[gb as usize].name;
                if gn.contains("water") || gn.contains("log") || gn.contains("leaves") || gn.contains("planks") { continue; }
                if (1..=4).any(|k| self.world.get(x, g + k, z).is_some_and(|b| b != 0 && BLOCKS[b as usize].solid && !BLOCKS[b as usize].name.contains("leaves"))) { continue; }
                for k in 1..=4 { e.push((x, g + k, z, wall)); } if outer && i % 2 == 0 { e.push((x, g + 5, z, wall)); } } // le chemin de ronde, les merlons dehors
            if is_tower { // une tour de trois sur trois, plus haute, sa lanterne au sommet
                let (x, z) = ((cx + a.cos() * (rr + 0.5)).floor() as i32, (cz + a.sin() * (rr + 0.5)).floor() as i32);
                if (-1..=1).all(|dx| (-1..=1).all(|dz| !built.contains(&(x + dx, z + dz)))) { if let Some(g) = ground(self, x, z) {
                    for dx in -1..=1 { for dz in -1..=1 { for k in 1..=7 { e.push((x + dx, g + k, z + dz, tower)); } if (dx + dz) % 2 == 0 { e.push((x + dx, g + 8, z + dz, tower)); } } } e.push((x, g + 8, z, lamp)); } } } }
        if e.is_empty() { return false; } self.world.edit_many(&e); true
    }
}
