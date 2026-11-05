//! La partie en cours : joueur, inventaire, visée, casse et pose, sauvegarde. Aucun lien avec la fenêtre ou le rendu.
use crate::actors::{Anim, AnimIn, Draw, Models};
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
/// Tous les objets : outils d'abord, puis l'ordre du catalogue.
pub const ORDER: [&str; 34] = ["pickaxe", "axe", "sword", "grass", "dirt", "path", "forest_soil", "gravel", "sand", "stone", "mossy_stone", "limestone", "granite", "oak_log", "birch_log", "beech_log", "planks", "fence", "oak_leaves", "birch_leaves", "beech_leaves", "tall_grass", "fern", "daisy", "lupine", "pink_flower", "yellow_flower", "mushroom", "shelf_fungus", "lily_pad", "vine", "flagstone", "lantern", "water"];
pub fn item(id: &str) -> Option<Item> {
    match id { "pickaxe" => return Some(Item { id: "pickaxe", label: "Pioche", block: None, tool: Some("rock") }), "axe" => return Some(Item { id: "axe", label: "Hache", block: None, tool: Some("wood") }), "sword" => return Some(Item { id: "sword", label: "Épée", block: None, tool: None }), _ => {} }
    LABELS.iter().find(|l| l.0 == id).map(|l| Item { id: l.0, label: l.1, block: by_name(l.0), tool: None })
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Stack { pub item: &'static str, pub n: Option<u32> }
#[derive(Clone, Copy, Debug)]
pub struct Target { pub x: i32, pub y: i32, pub z: i32, pub px: i32, pub py: i32, pub pz: i32, pub id: u8 }
/// Parcours de grille : premier bloc ciblable et case d'avant.
pub fn raycast(w: &impl Voxels, o: [f64; 3], d: [f64; 3], max: f64, skip: &dyn Fn(i32, i32, i32, u8) -> bool) -> Option<(Target, f64)> {
    let (mut x, mut y, mut z) = (o[0].floor() as i32, o[1].floor() as i32, o[2].floor() as i32);
    let sg = |v: f64| if v > 0.0 { 1 } else if v < 0.0 { -1 } else { 0 };
    let (sx, sy, sz) = (sg(d[0]), sg(d[1]), sg(d[2])); let (tdx, tdy, tdz) = ((1.0 / d[0]).abs(), (1.0 / d[1]).abs(), (1.0 / d[2]).abs());
    let side = |s: i32, c: i32, o: f64| if s > 0 { c as f64 + 1.0 - o } else { o - c as f64 };
    let (mut tx, mut ty, mut tz) = (side(sx, x, o[0]) * tdx, side(sy, y, o[1]) * tdy, side(sz, z, o[2]) * tdz);
    let (mut px, mut py, mut pz, mut t) = (x, y, z, 0.0);
    for _ in 0..80 {
        if t > max { break; }
        let b = w.get(x, y, z)?;
        if b > 0 && b != B::WATER && !skip(x, y, z, b) { return Some((Target { x, y, z, px, py, pz, id: b }, t)); }
        px = x; py = y; pz = z;
        if tx < ty && tx < tz { x += sx; t = tx; tx += tdx; } else if ty < tz { y += sy; t = ty; ty += tdy; } else { z += sz; t = tz; tz += tdz; }
    }
    None
}
#[derive(Default, Clone, Copy)]
pub struct Actions { pub mine: bool, pub place: bool }
pub struct Session {
    pub world: Streamer, pub player: Player, pub inv: [Option<Stack>; INV_SIZE], pub sel: usize, pub creative: bool, pub target: Option<Target>, pub orient: u8, pub hour: f64, pub vit: [f64; 3], pub seed: i32, pub third: bool, pub cam_dist: f64, pub eye: [f64; 3], pub look: [f64; 3], pub anim: Anim, pub inv_open: bool, pub cursor: Option<Stack>, pub from: Option<usize>,
    mine_key: Option<(i32, i32, i32)>, pub mine_t: f64, cool: f64, axis: Option<char>, pub dirty: bool,
}
impl Session {
    pub fn new(seed: i32, p: &Preset, threads: usize, at: [f64; 3]) -> Self {
        let mut inv = [None; INV_SIZE];
        for (i, (it, n)) in [("pickaxe", None), ("axe", None), ("sword", None), ("lantern", Some(32)), ("dirt", Some(64)), ("stone", Some(32)), ("planks", Some(16))].into_iter().enumerate() { inv[i] = Some(Stack { item: it, n }); }
        Session { world: Streamer::new(seed, p, threads), player: Player::new(at[0], at[1], at[2]), inv, sel: 0, creative: false, target: None, orient: 0, hour: 16.4, vit: [100.0; 3], seed, third: true, cam_dist: 4.2, eye: [0.0; 3], look: [0.0, 0.0, 1.0], anim: Anim::default(), inv_open: false, cursor: None, from: None, mine_key: None, mine_t: 0.0, cool: 0.0, axis: None, dirty: false }
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
        let (eye, f) = self.camera(dt); self.eye = eye; self.look = f; let (third, head) = (self.third, [self.player.p[0], self.player.p[1] + 1.5, self.player.p[2]]);
        self.target = raycast(&self.world, eye, f, reach, &|x, y, z, _| third && (x as f64 + 0.5 - head[0]) * f[0] + (y as f64 + 0.5 - head[1]) * f[1] + (z as f64 + 0.5 - head[2]) * f[2] < -0.5).map(|t| t.0);
        let open = self.inv_open; self.mine_step(dt, a.mine && !open); if a.place && !open && self.place() { self.anim.trigger(); }
        let mining = a.mine && !open; self.anim.update(dt, AnimIn { speed: self.player.speed, ground: self.player.ground, vy: self.player.v[1], crouch: self.player.crouch, fly: self.player.fly, mining });
        if !self.creative { let k = if i.sprint { 2.0 } else { 1.0 }; self.vit[1] = (self.vit[1] - dt * 0.012 * k).max(0.0); self.vit[2] = (self.vit[2] - dt * 0.018 * k).max(0.0); }
        if self.player.in_water { self.vit[2] = (self.vit[2] + dt * 8.0).min(100.0); }
    }
    /// Caméra : tête du joueur, ou épaule retenue par le terrain pour garder le héros au tiers gauche.
    pub fn camera(&mut self, dt: f64) -> ([f64; 3], [f64; 3]) {
        let (head, f, yaw) = (self.player.eye(), self.player.forward(), self.player.yaw);
        if !self.third { return (head, f); }
        let (right, back) = ([yaw.cos(), 0.0, -yaw.sin()], [-f[0], -f[1], -f[2]]); let anchor = [head[0] - right[0] * 0.8, head[1] + 0.42, head[2] - right[2] * 0.8]; let mut d = 4.2;
        if let Some((_, t)) = raycast(&self.world, anchor, back, d + 0.3, &|_, _, _, b| is_deco(b) || is_leaf(b)) { d = (t - 0.3).max(0.5); }
        let k = if d < self.cam_dist { 0.5 } else { 0.08 }; self.cam_dist += (d - self.cam_dist) * if dt > 0.0 { k } else { 1.0 };
        ([anchor[0] + back[0] * self.cam_dist, anchor[1] + back[1] * self.cam_dist, anchor[2] + back[2] * self.cam_dist], f)
    }
    /// Ce que le rendu doit dessiner du personnage.
    pub fn draws(&self, md: &Models, view: &crate::view::M4) -> Vec<Draw> {
        let held = self.inv[self.sel].and_then(|s| md.held(s.item)); let p = &self.player;
        if self.third { self.anim.third(md, p.p, if p.speed > 0.2 { p.move_yaw } else { p.yaw }, p.pitch, held) } else { self.anim.first(md, view, held) }
    }
    /// Où irait le bloc tenu, et serait-il accepté ?
    pub fn ghost(&self) -> Option<([i32; 3], bool)> {
        let (t, st) = (self.target?, self.inv[self.sel]?); let id = item(st.item)?.block?; let [x, y, z] = Self::place_spot(&t); let cur = self.world.get(x, y, z)?; let p = self.player.p;
        let hit = is_solid(id) && x as f64 + 1.0 > p[0] - 0.3 && (x as f64) < p[0] + 0.3 && z as f64 + 1.0 > p[2] - 0.3 && (z as f64) < p[2] + 0.3 && y as f64 + 1.0 > p[1] && (y as f64) < p[1] + 1.8;
        Some(([x, y, z], (self.creative || st.n.unwrap_or(0) > 0) && !is_solid(cur) && !hit))
    }
    /// Inventaire : prend ce qu'il y a dans une case. En créatif, le catalogue donne un tas plein.
    pub fn grab_slot(&mut self, i: usize) { if self.cursor.is_none() { if let Some(s) = self.inv[i].take() { self.cursor = Some(s); self.from = Some(i); } } }
    pub fn grab_catalog(&mut self, id: &'static str) { if let Some(it) = item(id) { self.cursor = Some(Stack { item: it.id, n: if it.block.is_some() { Some(STACK) } else { None } }); self.from = None; } }
    /// Dépose : réunit deux tas du même objet, sinon échange les cases.
    pub fn drop_slot(&mut self, i: Option<usize>) {
        let Some(c) = self.cursor.take() else { return };
        let Some(i) = i else { if let Some(f) = self.from.take() { if self.inv[f].is_none() { self.inv[f] = Some(c); } else { self.add_item(c.item, c.n.unwrap_or(1)); } } return };
        match self.inv[i] {
            Some(o) if o.item == c.item && o.n.is_some() && c.n.is_some() => { let t = o.n.unwrap() + c.n.unwrap(); self.inv[i] = Some(Stack { n: Some(t.min(STACK)), ..o }); if t > STACK { let rest = Some(Stack { n: Some(t - STACK), ..o }); if let Some(f) = self.from { self.inv[f] = rest; } else { self.add_item(o.item, t - STACK); } } }
            Some(o) => { self.inv[i] = Some(c); if let Some(f) = self.from { self.inv[f] = Some(o); } else { self.cursor = Some(o); } }
            None => self.inv[i] = Some(c),
        }
        self.from = None; self.dirty = true;
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
