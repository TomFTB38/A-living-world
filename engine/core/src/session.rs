//! La partie en cours : joueur, inventaire, visée, casse et pose, sauvegarde. Aucun lien avec la fenêtre ou le rendu.
use crate::blocks::*;
use crate::player::{Input, Player, Voxels};
use crate::stream::{Preset, Streamer};
use std::collections::HashMap;

pub const INV_SIZE: usize = 36; pub const STACK: u32 = 99;
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Item { pub id: &'static str, pub label: &'static str, pub block: Option<u8>, pub tool: Option<&'static str> }
const LABELS: [(&str, &str); 31] = [("grass", "Herbe"), ("dirt", "Terre"), ("path", "Terre de chemin"), ("forest_soil", "Humus"), ("gravel", "Gravier"), ("sand", "Sable"), ("stone", "Pierre"), ("mossy_stone", "Pierre moussue"), ("limestone", "Calcaire"), ("granite", "Granit"),
    ("oak_log", "Bois de chêne"), ("birch_log", "Bois de bouleau"), ("beech_log", "Bois de hêtre"), ("planks", "Planches"), ("fence", "Clôture"), ("oak_leaves", "Feuilles de chêne"), ("birch_leaves", "Feuilles de bouleau"), ("beech_leaves", "Feuilles de hêtre"),
    ("tall_grass", "Herbes hautes"), ("fern", "Fougère"), ("daisy", "Marguerite"), ("lupine", "Lupin"), ("pink_flower", "Fleur rose"), ("yellow_flower", "Fleur jaune"), ("mushroom", "Champignon"), ("shelf_fungus", "Polypore"), ("lily_pad", "Nénuphar"), ("vine", "Liane"), ("flagstone", "Dalle"), ("lantern", "Lanterne"), ("water", "Eau")];
pub fn item(id: &str) -> Option<Item> {
    match id { "pickaxe" => return Some(Item { id: "pickaxe", label: "Pioche", block: None, tool: Some("rock") }), "axe" => return Some(Item { id: "axe", label: "Hache", block: None, tool: Some("wood") }), "sword" => return Some(Item { id: "sword", label: "Épée", block: None, tool: None }), _ => {} }
    LABELS.iter().find(|l| l.0 == id).map(|l| Item { id: l.0, label: l.1, block: by_name(l.0), tool: None })
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Stack { pub item: &'static str, pub n: Option<u32> }
#[derive(Clone, Copy, Debug)]
pub struct Target { pub x: i32, pub y: i32, pub z: i32, pub px: i32, pub py: i32, pub pz: i32, pub id: u8 }
/// Parcours de grille : premier bloc ciblable et case d'avant.
pub fn raycast(w: &impl Voxels, o: [f64; 3], d: [f64; 3], max: f64) -> Option<Target> {
    let (mut x, mut y, mut z) = (o[0].floor() as i32, o[1].floor() as i32, o[2].floor() as i32);
    let sg = |v: f64| if v > 0.0 { 1 } else if v < 0.0 { -1 } else { 0 };
    let (sx, sy, sz) = (sg(d[0]), sg(d[1]), sg(d[2])); let (tdx, tdy, tdz) = ((1.0 / d[0]).abs(), (1.0 / d[1]).abs(), (1.0 / d[2]).abs());
    let side = |s: i32, c: i32, o: f64| if s > 0 { c as f64 + 1.0 - o } else { o - c as f64 };
    let (mut tx, mut ty, mut tz) = (side(sx, x, o[0]) * tdx, side(sy, y, o[1]) * tdy, side(sz, z, o[2]) * tdz);
    let (mut px, mut py, mut pz, mut t) = (x, y, z, 0.0);
    for _ in 0..80 {
        if t > max { break; }
        let b = w.get(x, y, z)?;
        if b > 0 && b != B::WATER { return Some(Target { x, y, z, px, py, pz, id: b }); }
        px = x; py = y; pz = z;
        if tx < ty && tx < tz { x += sx; t = tx; tx += tdx; } else if ty < tz { y += sy; t = ty; ty += tdy; } else { z += sz; t = tz; tz += tdz; }
    }
    None
}
#[derive(Default, Clone, Copy)]
pub struct Actions { pub mine: bool, pub place: bool }
pub struct Session {
    pub world: Streamer, pub player: Player, pub inv: [Option<Stack>; INV_SIZE], pub sel: usize, pub creative: bool, pub target: Option<Target>, pub orient: u8, pub hour: f64, pub vit: [f64; 3], pub seed: i32,
    mine_key: Option<(i32, i32, i32)>, pub mine_t: f64, cool: f64, axis: Option<char>, pub dirty: bool,
}
impl Session {
    pub fn new(seed: i32, p: &Preset, threads: usize, at: [f64; 3]) -> Self {
        let mut inv = [None; INV_SIZE];
        for (i, (it, n)) in [("pickaxe", None), ("axe", None), ("sword", None), ("lantern", Some(32)), ("dirt", Some(64)), ("stone", Some(32)), ("planks", Some(16))].into_iter().enumerate() { inv[i] = Some(Stack { item: it, n }); }
        Session { world: Streamer::new(seed, p, threads), player: Player::new(at[0], at[1], at[2]), inv, sel: 0, creative: false, target: None, orient: 0, hour: 16.4, vit: [100.0; 3], seed, mine_key: None, mine_t: 0.0, cool: 0.0, axis: None, dirty: false }
    }
    pub fn held(&self) -> Option<Item> { self.inv[self.sel].and_then(|s| item(s.item)) }
    pub fn add_item(&mut self, id: &str, n: u32) {
        let Some(it) = item(id) else { return }; let mut left = n;
        for s in self.inv.iter_mut().flatten() { if left > 0 && s.item == it.id { if let Some(c) = s.n { if c < STACK { let m = left.min(STACK - c); s.n = Some(c + m); left -= m; } } } }
        for s in self.inv.iter_mut() { if left == 0 { break; } if s.is_none() { let m = left.min(STACK); *s = Some(Stack { item: it.id, n: Some(m) }); left -= m; } }
    }
    /// Ce qui tenait sur le bloc retiré tombe avec lui : plantes au-dessus, lianes en dessous, champignons sur les côtés.
    fn drop_attached(&mut self, x: i32, y: i32, z: i32) {
        let g = |s: &Self, a, b, c| s.world.get(a, b, c).unwrap_or(0);
        let up = g(self, x, y + 1, z);
        if up > 0 && is_deco(up) && up != B::VINE && up != B::SHELF_FUNGUS && up != B::FENCE { self.world.edit(x, y + 1, z, 0); }
        let (mut k, mut n) = (y - 1, 0); while g(self, x, k, z) == B::VINE && n < 12 { self.world.edit(x, k, z, 0); k -= 1; n += 1; }
        for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { if g(self, x + dx, y, z + dz) == B::SHELF_FUNGUS {
            let kept = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|(a, b)| { let c = g(self, x + dx + a, y, z + dz + b); c > 0 && BLOCKS[c as usize].opaque });
            if !kept { self.world.edit(x + dx, y, z + dz, 0); }
        } }
    }
    fn place_spot(t: &Target) -> [i32; 3] { if is_deco(t.id) && t.id != B::LANTERN && !is_solid(t.id) { [t.x, t.y, t.z] } else { [t.px, t.py, t.pz] } }
    fn oriented(&mut self, b: u8) -> u8 { let f = self.player.forward(); let a = gaze_axis(f[0], f[1], f[2], self.orient, self.axis); self.axis = Some(a); oriented_block(b, a) }
    pub fn place(&mut self) -> bool {
        let (Some(t), Some(st)) = (self.target, self.inv[self.sel]) else { return false };
        let Some(block) = item(st.item).and_then(|i| i.block) else { return false };
        if !self.creative && st.n.unwrap_or(0) == 0 { return false; }
        let [x, y, z] = Self::place_spot(&t); let Some(cur) = self.world.get(x, y, z) else { return false }; if is_solid(cur) { return false; }
        let id = self.oriented(block); let p = self.player.p;
        if is_solid(id) && x as f64 + 1.0 > p[0] - 0.3 && (x as f64) < p[0] + 0.3 && z as f64 + 1.0 > p[2] - 0.3 && (z as f64) < p[2] + 0.3 && y as f64 + 1.0 > p[1] && (y as f64) < p[1] + 1.8 { return false; }
        if !self.world.edit(x, y, z, id) { return false; }
        if !self.creative { let n = st.n.unwrap_or(0) - 1; self.inv[self.sel] = if n == 0 { None } else { Some(Stack { n: Some(n), ..st }) }; }
        self.dirty = true; true
    }
    fn mine_step(&mut self, dt: f64, go: bool) {
        let Some(t) = self.target.filter(|_| go) else { self.mine_key = None; self.mine_t = 0.0; return };
        let key = (t.x, t.y, t.z); if self.mine_key != Some(key) { self.mine_key = Some(key); self.mine_t = 0.0; }
        let b = BLOCKS[t.id as usize]; let tool = self.held().and_then(|i| i.tool);
        let rate = if tool.is_some() && tool == Some(b.cat) { 3.0 } else if b.cat == "terrain" { 1.6 } else { 1.0 };
        if self.creative { self.cool -= dt; if self.cool > 0.0 { return; } self.cool = 0.2; self.mine_t = 1e9; }
        self.mine_t += dt * rate;
        if self.mine_t >= b.hardness {
            if self.world.edit(t.x, t.y, t.z, 0) { self.drop_attached(t.x, t.y, t.z); if !self.creative { if let Some(d) = b.drop { self.add_item(d, 1); } } self.dirty = true; }
            self.mine_key = None; self.mine_t = 0.0;
        }
    }
    /// Une étape de jeu. Le reste (rendu, son) lit l'état.
    pub fn tick(&mut self, dt: f64, i: Input, a: Actions) {
        self.player.update(&self.world, dt, i);
        let reach = if self.creative { 13.0 } else { 9.5 };
        self.target = raycast(&self.world, self.player.eye(), self.player.forward(), reach);
        self.mine_step(dt, a.mine); if a.place { self.place(); }
        if !self.creative { let k = if i.sprint { 2.0 } else { 1.0 }; self.vit[1] = (self.vit[1] - dt * 0.012 * k).max(0.0); self.vit[2] = (self.vit[2] - dt * 0.018 * k).max(0.0); }
        if self.player.in_water { self.vit[2] = (self.vit[2] + dt * 8.0).min(100.0); }
    }
    pub fn progress(&self) -> f64 { match self.target { Some(t) if self.mine_key.is_some() => (self.mine_t / BLOCKS[t.id as usize].hardness.max(0.01)).min(1.0), _ => 0.0 } }
    pub fn set_creative(&mut self, on: bool) { self.creative = on; if !on { self.player.fly = false; } }
    /// Sauvegarde lisible : l'état du joueur, puis seulement ce qui diffère du monde généré.
    pub fn save_text(&self) -> String {
        let p = &self.player; let mut s = format!("A Living World 1\nseed {}\nplayer {} {} {} {} {}\nhour {}\ncreative {}\nsel {}\nvit {} {} {}\n", self.seed, p.p[0], p.p[1], p.p[2], p.yaw, p.pitch, self.hour, self.creative as u8, self.sel, self.vit[0], self.vit[1], self.vit[2]);
        for (i, it) in self.inv.iter().enumerate() { if let Some(v) = it { s += &format!("inv {} {} {}\n", i, v.item, v.n.map_or(-1, |n| n as i64)); } }
        for (k, e) in self.world.edits() { s += &format!("chunk {} {}", k.0, k.1); let mut l: Vec<_> = e.into_iter().collect(); l.sort(); for (i, id) in l { s += &format!(" {i}:{id}"); } s.push('\n'); }
        s
    }
    pub fn load_text(&mut self, t: &str) -> Result<(), String> {
        let mut lines = t.lines(); if lines.next() != Some("A Living World 1") { return Err("format inconnu".into()); }
        let mut edits: HashMap<(i32, i32), HashMap<u32, u8>> = HashMap::new(); let mut inv = [None; INV_SIZE];
        let num = |v: Option<&str>| v.and_then(|x| x.parse::<f64>().ok()).ok_or("nombre illisible");
        for l in lines {
            let mut a = l.split(' ');
            match a.next() {
                Some("player") => { for k in 0..3 { self.player.p[k] = num(a.next())?; } self.player.yaw = num(a.next())?; self.player.pitch = num(a.next())?; }
                Some("hour") => self.hour = num(a.next())?, Some("creative") => self.creative = num(a.next())? != 0.0, Some("sel") => self.sel = (num(a.next())? as usize).min(INV_SIZE - 1),
                Some("vit") => { for k in 0..3 { self.vit[k] = num(a.next())?; } }
                Some("inv") => { let i = num(a.next())? as usize; let id = a.next().unwrap_or(""); let n = num(a.next())?; if let (Some(it), true) = (item(id), i < INV_SIZE) { inv[i] = Some(Stack { item: it.id, n: if n < 0.0 { None } else { Some(n as u32) } }); } }
                Some("chunk") => { let k = (num(a.next())? as i32, num(a.next())? as i32); let e = edits.entry(k).or_default(); for c in a { if let Some((i, id)) = c.split_once(':') { if let (Ok(i), Ok(id)) = (i.parse(), id.parse()) { e.insert(i, id); } } } }
                _ => {}
            }
        }
        self.inv = inv; self.world.set_edits(edits); Ok(())
    }
}
