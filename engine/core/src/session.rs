//! La partie en cours : joueur, inventaire, visée, casse et pose, sauvegarde. Aucun lien avec la fenêtre ou le rendu.
use crate::actors::{Anim, AnimIn, Draw, Models};
use crate::audio::{pick_mood, Situation, SoundScape};
use crate::blocks::*;
use crate::fauna::{Ev, Fauna};
use crate::felling::{find_tree, Fall, Tree, FELL_TIME};
use crate::flow::Flow;
use crate::gen::WorldGen;
use crate::player::{Input, Player, Voxels};
use crate::stream::{Preset, Streamer};
use crate::weather::Weather;
use std::collections::HashMap;

pub const INV_SIZE: usize = 36; pub const STACK: u32 = 99;
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Item { pub id: &'static str, pub label: &'static str, pub block: Option<u8>, pub tool: Option<&'static str> }
const LABELS: [(&str, &str); 36] = [("grass", "Herbe"), ("dirt", "Terre"), ("path", "Terre de chemin"), ("forest_soil", "Humus"), ("gravel", "Gravier"), ("sand", "Sable"), ("stone", "Pierre"), ("mossy_stone", "Pierre moussue"), ("limestone", "Calcaire"), ("granite", "Granit"),
    ("oak_log", "Bois de chêne"), ("birch_log", "Bois de bouleau"), ("beech_log", "Bois de hêtre"), ("planks", "Planches"), ("fence", "Clôture"), ("oak_leaves", "Feuilles de chêne"), ("birch_leaves", "Feuilles de bouleau"), ("beech_leaves", "Feuilles de hêtre"),
    ("tall_grass", "Herbes hautes"), ("fern", "Fougère"), ("daisy", "Marguerite"), ("lupine", "Lupin"), ("pink_flower", "Fleur rose"), ("yellow_flower", "Fleur jaune"), ("mushroom", "Champignon"), ("shelf_fungus", "Polypore"), ("lily_pad", "Nénuphar"), ("vine", "Liane"), ("flagstone", "Dalle"), ("lantern", "Lanterne"), ("water", "Eau"), ("glowcap", "Champignon lumineux"), ("snow", "Neige"), ("ice", "Glace"), ("spruce_log", "Bois d'épicéa"), ("spruce_leaves", "Aiguilles d'épicéa")];
/// Tous les objets : outils d'abord, puis l'ordre du catalogue.
pub const ORDER: [&str; 39] = ["pickaxe", "axe", "sword", "grass", "dirt", "path", "forest_soil", "gravel", "sand", "snow", "ice", "stone", "mossy_stone", "limestone", "granite", "oak_log", "birch_log", "beech_log", "spruce_log", "planks", "fence", "oak_leaves", "birch_leaves", "beech_leaves", "spruce_leaves", "tall_grass", "fern", "daisy", "lupine", "pink_flower", "yellow_flower", "mushroom", "glowcap", "shelf_fungus", "lily_pad", "vine", "flagstone", "lantern", "water"];
/// Onglets du catalogue créatif ; le premier montre tout.
pub const TABS: [(&str, &[&str]); 7] = [("Tout", &ORDER), ("Terrain", &["grass", "dirt", "path", "forest_soil", "gravel", "sand", "snow", "ice", "water"]), ("Roches", &["stone", "mossy_stone", "limestone", "granite", "flagstone"]), ("Bois", &["oak_log", "birch_log", "beech_log", "spruce_log", "planks", "fence"]), ("Feuillages", &["oak_leaves", "birch_leaves", "beech_leaves", "spruce_leaves", "vine"]), ("Plantes", &["tall_grass", "fern", "daisy", "lupine", "pink_flower", "yellow_flower", "mushroom", "glowcap", "shelf_fungus", "lily_pad"]), ("Objets", &["pickaxe", "axe", "sword", "lantern"])];
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
        if b > 0 && !is_water(b) && !skip(x, y, z, b) { return Some((Target { x, y, z, px, py, pz, id: b }, t)); }
        px = x; py = y; pz = z;
        if tx < ty && tx < tz { x += sx; t = tx; tx += tdx; } else if ty < tz { y += sy; t = ty; ty += tdy; } else { z += sz; t = tz; tz += tdz; }
    }
    None
}
#[derive(Default, Clone, Copy)]
pub struct Actions { pub mine: bool, pub place: bool }
pub struct Session {
    pub world: Streamer, pub player: Player, pub inv: [Option<Stack>; INV_SIZE], pub sel: usize, pub creative: bool, pub target: Option<Target>, pub orient: u8, pub hour: f64, pub vit: [f64; 3], pub seed: i32, pub third: bool, pub cam_dist: f64, pub eye: [f64; 3], pub look: [f64; 3], pub anim: Anim, pub inv_open: bool, pub cursor: Option<Stack>, pub from: Option<usize>,
    mine_key: Option<(i32, i32, i32)>, pub mine_t: f64, cool: f64, axis: Option<char>, pub dirty: bool, pub tab: usize,
    pub deep: f64, pub taiga: f64, glows: HashMap<(i32, i32), Vec<[i32; 3]>>, scanned: std::collections::HashSet<(i32, i32)>, pub fauna: Fauna, pub weather: Weather, pub sound: SoundScape, gen: WorldGen, pub elapsed: f64, hurt_at: f64, built: Vec<f64>, swing_t: f64, dig_t: f64, step_t: f64, water_t: f64, env_t: f64, was_wet: bool, was_ground: bool, fall_v: f64, brook: Option<[f64; 3]>, fall: Option<[f64; 3]>, felling: Option<Tree>, pub falling: Option<Fall>, pub fall_serial: u64, pub flow: Flow,
}
impl Session {
    pub fn new(seed: i32, p: &Preset, threads: usize, at: [f64; 3]) -> Self {
        let mut inv = [None; INV_SIZE];
        for (i, (it, n)) in [("pickaxe", None), ("axe", None), ("sword", None), ("lantern", Some(32)), ("dirt", Some(64)), ("stone", Some(32)), ("planks", Some(16))].into_iter().enumerate() { inv[i] = Some(Stack { item: it, n }); }
        Session { world: Streamer::new(seed, p, threads), player: Player::new(at[0], at[1], at[2]), inv, sel: 0, creative: false, target: None, orient: 0, hour: 16.4, vit: [100.0; 3], seed, third: true, cam_dist: 4.2, eye: [0.0; 3], look: [0.0, 0.0, 1.0], anim: Anim::default(), inv_open: false, cursor: None, from: None, mine_key: None, mine_t: 0.0, cool: 0.0, axis: None, dirty: false, tab: 0, deep: 0.0, taiga: 0.0, glows: HashMap::new(), scanned: Default::default(), fauna: Fauna::new(seed), weather: Weather::new(seed), sound: SoundScape::new(None), gen: WorldGen::new(seed), elapsed: 0.0, hurt_at: -99.0, built: Vec::new(), swing_t: 0.0, dig_t: 0.0, step_t: 0.0, water_t: 0.0, env_t: 0.0, was_wet: false, was_ground: false, fall_v: 0.0, brook: None, fall: None, felling: None, falling: None, fall_serial: 0, flow: Flow::default() }
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
        let (mut k, mut n) = (y - 1, 0); while (g(self, x, k, z) == B::VINE || g(self, x, k, z) == B::ICICLE) && n < 12 { self.world.edit(x, k, z, 0); k -= 1; n += 1; }
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
        let id = placed_variant(self.oriented(block)); let p = self.player.p;
        if is_solid(id) && x as f64 + 1.0 > p[0] - 0.3 && (x as f64) < p[0] + 0.3 && z as f64 + 1.0 > p[2] - 0.3 && (z as f64) < p[2] + 0.3 && y as f64 + 1.0 > p[1] && (y as f64) < p[1] + 1.8 { return false; }
        if !self.world.edit(x, y, z, id) { return false; }
        self.flow.touch(x, y, z);
        if !self.creative { let n = st.n.unwrap_or(0) - 1; self.inv[self.sel] = if n == 0 { None } else { Some(Stack { n: Some(n), ..st }) }; }
        self.sound.place(BLOCKS[id as usize].name); self.built.push(self.elapsed); self.dirty = true; true
    }
    /// L'objet tenu s'il s'agit d'un outil : c'est lui qui donne sa voix au coup.
    fn tool_id(&self) -> Option<&'static str> { self.held().filter(|i| i.block.is_none()).map(|i| i.id) }
    fn mine_step(&mut self, dt: f64, go: bool) {
        let tool = self.tool_id(); self.swing_t -= dt;
        // Dans le vide, on frappe quand même, avec n'importe quoi.
        if go && self.target.is_none() && self.swing_t <= 0.0 { self.swing_t = 0.3; self.sound.swing(tool, 1.0); }
        let Some(t) = self.target.filter(|_| go) else { self.mine_key = None; self.mine_t = 0.0; return };
        let key = (t.x, t.y, t.z); if self.mine_key != Some(key) { self.mine_key = Some(key); self.mine_t = 0.0; self.felling = if tool == Some("axe") { find_tree(&self.world, t.x, t.y, t.z) } else { None }; }
        // La hache sur un arbre : pas de bloc à bloc, on entaille jusqu'à ce que tout l'arbre tombe.
        if self.felling.is_some() {
            self.mine_t += dt; self.dig_t -= dt; if self.dig_t <= 0.0 { self.dig_t = 0.3; self.sound.dig(BLOCKS[t.id as usize].name, tool); }
            if self.mine_t >= FELL_TIME { self.fell(t); self.mine_key = None; self.mine_t = 0.0; }
            return;
        }
        let b = BLOCKS[t.id as usize]; let kind = self.held().and_then(|i| i.tool);
        let rate = if kind.is_some() && kind == Some(b.cat) { 3.0 } else if b.cat == "terrain" { 1.6 } else { 1.0 };
        if self.creative { self.cool -= dt; if self.cool > 0.0 { return; } self.cool = 0.2; self.mine_t = 1e9; }
        self.mine_t += dt * rate; self.dig_t -= dt;
        if self.dig_t <= 0.0 && !self.creative { self.dig_t = 0.3; self.sound.dig(b.name, tool); }
        if self.mine_t >= b.hardness {
            if self.world.edit(t.x, t.y, t.z, 0) { self.flow.touch(t.x, t.y, t.z); self.sound.break_block(b.name, tool); self.drop_attached(t.x, t.y, t.z); if !self.creative { if let Some(d) = b.drop { self.add_item(d, 1); } } self.dirty = true; }
            self.mine_key = None; self.mine_t = 0.0;
        }
    }
    /// Le tronc au-dessus de l'entaille tombe, le feuillage disparaît, le pied de l'arbre reste en place.
    fn fell(&mut self, t: Target) {
        let Some(tree) = self.felling.take() else { return }; let at = |s: &Self, p: (i32, i32, i32)| s.world.get(p.0, p.1, p.2).unwrap_or(0);
        let logs: Vec<((i32, i32, i32), u8)> = tree.logs.iter().filter(|p| p.1 >= t.y).map(|p| (*p, at(self, *p))).collect();
        let leaf_id = tree.leaves.first().map_or(0, |p| at(self, *p)); let drop = BLOCKS[t.id as usize].drop.unwrap_or("oak_log");
        let edits: Vec<_> = logs.iter().map(|(p, _)| (p.0, p.1, p.2, 0)).chain(tree.leaves.iter().map(|p| (p.0, p.1, p.2, 0))).collect(); self.world.edit_many(&edits);
        self.falling = Some(Fall::new(&logs, &tree.leaves, leaf_id, (t.x, t.y, t.z), self.player.p, (t.x * 31 + t.z) as u32)); self.fall_serial += 1;
        for p in logs.iter().map(|l| l.0).chain(tree.leaves.iter().copied()) { self.drop_attached(p.0, p.1, p.2); self.flow.touch(p.0, p.1, p.2); }
        if !self.creative { self.add_item(drop, logs.len().min(99) as u32); }
        self.sound.tree_crack(); self.dirty = true;
    }
    /// Une étape de jeu. Le reste (rendu, son) lit l'état.
    pub fn tick(&mut self, dt: f64, i: Input, a: Actions) {
        let rolled = self.player.rolling(); self.player.update(&self.world, dt, i); if self.player.rolling() && !rolled { self.sound.roll(); }
        let reach = if self.creative { 13.0 } else { 9.5 };
        let (eye, f) = self.camera(dt); self.eye = eye; self.look = f; let (third, head) = (self.third, [self.player.p[0], self.player.p[1] + 1.5, self.player.p[2]]);
        self.target = raycast(&self.world, eye, f, reach, &|x, y, z, _| third && (x as f64 + 0.5 - head[0]) * f[0] + (y as f64 + 0.5 - head[1]) * f[1] + (z as f64 + 0.5 - head[2]) * f[2] < -0.5).map(|t| t.0);
        let open = self.inv_open || self.player.rolling(); self.mine_step(dt, a.mine && !open); if a.place && !open && self.place() { self.anim.trigger(); } // ni coup ni pose en pleine roulade
        let mining = a.mine && !open; self.anim.update(dt, AnimIn { speed: self.player.speed, ground: self.player.ground, vy: self.player.v[1], crouch: self.player.crouch, fly: self.player.fly, mining });
        if !self.creative { let k = if i.sprint { 2.0 } else { 1.0 }; self.vit[1] = (self.vit[1] - dt * 0.012 * k).max(0.0); self.vit[2] = (self.vit[2] - dt * 0.018 * k).max(0.0); }
        if self.player.in_water { self.vit[2] = (self.vit[2] + dt * 8.0).min(100.0); }
        self.elapsed += dt; self.weather.update(dt); let t = self.mix().strong(crate::biome::Biome::DeepForest); self.deep += (t - self.deep) * (dt / 2.5).min(1.0); let t = self.mix().strong(crate::biome::Biome::Taiga); self.taiga += (t - self.taiga) * (dt / 2.5).min(1.0);
        let changes = self.flow.step(dt, &self.world); if !changes.is_empty() { self.world.edit_many(&changes); self.dirty = true; }
        if let Some(mut f) = self.falling.take() { if f.step(dt, &self.world) { self.sound.tree_land(); } if !f.done() { self.falling = Some(f); } }
        if self.world.loaded() > 0 {
            self.fauna.update(dt, &self.world, self.player.p, (self.sun_height() * 3.0).max(0.0));
            for e in std::mem::take(&mut self.fauna.ev) { match e { Ev::Bird(p, v) => self.sound.bird(p, v), Ev::Wings(p) => self.sound.wings(p), Ev::Hoof(p, f) => self.sound.hoof(p, f), Ev::Alarm(p) => self.sound.deer_alarm(p), Ev::Call(p) => self.sound.deer_call(p) } }
        }
        self.audio_step(dt);
    }
    /// Climat et biomes là où se tient le joueur.
    pub fn mix(&self) -> crate::biome::Mix { self.gen.climate.at(self.player.p[0], self.player.p[2]) }
    /// Reçoit les chunks prêts. L'eau d'un chunk neuf peut couler aussitôt : celle qui touche le vide se réveille.
    pub fn sync(&mut self) -> (Vec<crate::stream::Built>, Vec<(i32, i32)>) {
        let p = self.player.p; let (built, gone) = self.world.update(p[0], p[2]);
        for g in &gone { self.scanned.remove(g); self.glows.remove(g); }
        // Champignons lumineux de chaque chunk reçu : leur lumière colorée en dépend.
        for c in &built { if c.lod == crate::stream::FAR { continue; } let n = CX as i32; let mut l = Vec::new(); for (i, b) in c.voxels.iter().enumerate() { if *b == B::GLOWCAP { let i = i as i32; l.push([c.key.0 * n + i % n, i / (n * n), c.key.1 * n + (i / n) % n]); } } if l.is_empty() { self.glows.remove(&c.key); } else { self.glows.insert(c.key, l); } }
        for c in &built { if c.lod != 0 || !self.scanned.insert(c.key) { continue; } let n = CX as i32; let (ox, oz) = (c.key.0 * n, c.key.1 * n); let v = c.voxels.clone();
            for y in 2..CH as i32 - 1 { for z in 0..n { for x in 0..n { if v[idx(x as usize, y as usize, z as usize)] != B::WATER { continue; } let (wx, wz) = (ox + x, oz + z);
                let open = [(1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1), (0, -1, 0)].iter().any(|d| self.world.get(wx + d.0, y + d.1, wz + d.2) == Some(0));
                if open { self.flow.touch(wx, y, wz); }
            } } }
        }
        (built, gone)
    }
    /// Lumières colorées proches : position et couleur des champignons lumineux, les plus près d'abord.
    pub fn lights(&self, e: [f64; 3], max: usize) -> Vec<([f32; 3], [f32; 3])> {
        let mut v: Vec<(f64, [i32; 3])> = self.glows.values().flatten().map(|p| ((p[0] as f64 + 0.5 - e[0]).powi(2) + (p[1] as f64 + 0.3 - e[1]).powi(2) + (p[2] as f64 + 0.5 - e[2]).powi(2), *p)).filter(|q| q.0 < 32.0 * 32.0).collect();
        v.sort_by(|a, b| a.0.total_cmp(&b.0)); v.truncate(max);
        v.into_iter().map(|(_, p)| { let c = crate::mesher::glow_color(p[0], p[1], p[2]); ([p[0] as f32 + 0.5, p[1] as f32 + 0.35, p[2] as f32 + 0.5], [c[0] as f32, c[1] as f32, c[2] as f32]) }).collect()
    }
    /// Hauteur du soleil, négative la nuit.
    fn sun_height(&self) -> f64 { crate::det::sin((self.hour - 6.0) / 12.0 * std::f64::consts::PI) }
    /// Tout ce qui blesse passe par ici : la musique de danger en dépend.
    pub fn hurt(&mut self, n: f64) { if self.player.rolling() { return; } // la roulade esquive
        self.vit[0] = (self.vit[0] - n).max(0.0); self.hurt_at = self.elapsed; self.sound.hurt(); }
    /// La caméra est-elle sous l'eau ?
    pub fn under(&self) -> bool { self.world.get(self.eye[0].floor() as i32, self.eye[1].floor() as i32, self.eye[2].floor() as i32).is_some_and(is_water) }
    /// Sous un toit ou dans une grotte, il ne pleut pas ; le feuillage laisse passer.
    pub fn covered(&self, e: [f64; 3]) -> bool {
        let (x, z, y0) = (e[0].floor() as i32, e[2].floor() as i32, e[1].floor() as i32 + 1);
        (y0..y0 + 24).any(|y| self.world.get(x, y, z).is_some_and(|b| b > 0 && BLOCKS[b as usize].cat != "foliage" && BLOCKS[b as usize].cat != "fluid"))
    }
    /// Eau qui coule autour du joueur : les étangs sont silencieux, seuls ruisseaux et chutes s'entendent.
    fn scan_water(&mut self) {
        let p = self.player.p; let (x0, z0) = (p[0].floor() as i32, p[2].floor() as i32); let (mut bd, mut fd) = (1e9, 1e9); self.brook = None; self.fall = None;
        for dz in (-24..=24).step_by(2) { for dx in (-24..=24).step_by(2) {
            let (x, z) = (x0 + dx, z0 + dz); let c = self.gen.column(x as f64, z as f64); let lvl = c.water; if lvl == 0.0 || c.still { continue; }
            let d = (dx * dx + dz * dz) as f64 + (lvl - p[1]) * (lvl - p[1]);
            if d < bd { bd = d; self.brook = Some([x as f64, lvl, z as f64]); }
            if d < fd { for (a, b) in [(2, 0), (-2, 0), (0, 2), (0, -2)] { let w = self.gen.column((x + a) as f64, (z + b) as f64).water; if w != 0.0 && lvl - w >= 3.0 { fd = d; self.fall = Some([x as f64, lvl - 2.0, z as f64]); break; } } }
        } }
    }
    /// Sons : ambiance, pas selon le bloc, musique selon la situation.
    fn audio_step(&mut self, dt: f64) {
        let (p, yaw) = (self.player.p, self.player.yaw); let eye = [p[0], p[1] + 1.6, p[2]]; self.sound.ear = eye; self.sound.yaw = yaw; let w = self.weather.state;
        self.water_t -= dt; if self.water_t <= 0.0 { self.water_t = 0.5; self.scan_water(); }
        let at = |s: &Self, q: [f64; 3]| s.world.get(q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32);
        let wet = self.player.in_water; if wet && !self.was_wet && self.player.v[1] < -2.0 { self.sound.splash((-self.player.v[1] as f32 / 7.0).min(1.2)); } self.was_wet = wet;
        let below = at(self, [p[0], p[1] - 0.1, p[2]]).unwrap_or(0); let feet = at(self, [p[0], p[1] + 0.1, p[2]]).unwrap_or(0); let name = |b: u8| BLOCKS[b as usize].name; let pl = &self.player;
        if pl.ground && pl.speed > 1.0 {
            self.step_t -= dt * pl.speed;
            if self.step_t <= 0.0 { self.step_t = 2.1; if below > 0 { self.sound.step(name(below), if feet != below && feet > 0 { Some(name(feet)) } else { None }); } }
        } else if wet && pl.speed > 1.0 { self.step_t -= dt * pl.speed; if self.step_t <= 0.0 { self.step_t = 2.6; self.sound.step("water", None); } }
        if pl.ground && !self.was_ground && self.fall_v < -7.0 && !pl.fly && below > 0 { self.sound.land(name(below)); }
        self.was_ground = pl.ground; let vy = pl.v[1]; self.fall_v = if vy < -1.0 { vy } else if pl.ground { self.fall_v * 0.5 } else { self.fall_v };
        self.env_t -= dt;
        if self.env_t <= 0.0 {
            self.env_t = 0.2; let covered = self.covered(eye); let sub = at(self, eye).is_some_and(is_water); let brook = if self.flow.quiet < 2.0 { self.flow.last.or(self.brook) } else { self.brook }; self.sound.env(w.cloud, w.rain * (1.0 - self.taiga), covered, sub, brook, self.fall);
            let now = self.elapsed; self.built.retain(|t| now - t < 25.0);
            self.sound.mood(pick_mood(Situation { hurt: now - self.hurt_at < 12.0, enemy: false, food: self.vit[1], water: self.vit[2], submerged: sub, underground: p[1] < self.gen.column(p[0], p[2]).h - 4.0 && covered, building: self.built.len() >= 3, night: self.sun_height() < 0.0, rain: w.rain > 0.5 }));
        }
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
        let roll = p.rolling().then_some((p.roll_t / crate::player::ROLL_TIME, p.roll_local));
        let mut o = if self.third { self.anim.third(md, p.p, if p.speed > 0.2 && roll.is_none() { p.move_yaw } else { p.yaw }, p.pitch, held, roll) } else { self.anim.first(md, view, held) };
        o.extend(self.fauna.draws(md)); if let Some(f) = &self.falling { o.extend(f.draws(md)); } o
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
            Some(o) => { self.inv[i] = Some(c); match self.from { Some(f) if self.inv[f].is_none() => self.inv[f] = Some(o), _ => { self.cursor = Some(o); self.dirty = true; return; } } }
            None => self.inv[i] = Some(c),
        }
        self.from = None; self.dirty = true;
    }
    /// Clic droit : prend la moitié du tas, le reste reste dans la case.
    pub fn take_half(&mut self, i: usize) {
        let (None, Some(s)) = (self.cursor, self.inv[i]) else { return }; let Some(n) = s.n else { return self.grab_slot(i) };
        let k = n.div_ceil(2); self.cursor = Some(Stack { n: Some(k), ..s }); self.from = Some(i); self.inv[i] = if n == k { None } else { Some(Stack { n: Some(n - k), ..s }) };
    }
    /// Clic droit, objet en main : dépose une seule unité.
    pub fn put_one(&mut self, i: usize) {
        let Some(c) = self.cursor else { return }; let Some(n) = c.n else { return };
        match self.inv[i] { None => self.inv[i] = Some(Stack { n: Some(1), ..c }), Some(o) if o.item == c.item && o.n.is_some_and(|k| k < STACK) => self.inv[i] = Some(Stack { n: o.n.map(|k| k + 1), ..o }), _ => return }
        self.cursor = if n > 1 { Some(Stack { n: Some(n - 1), ..c }) } else { self.from = None; None }; self.dirty = true;
    }
    /// Maj+clic dans le catalogue : l'objet arrive directement dans la barre et il est sélectionné.
    pub fn quick_catalog(&mut self, id: &'static str) {
        let Some(it) = item(id) else { return }; let stack = Stack { item: it.id, n: it.block.map(|_| STACK) };
        let i = self.inv[..9].iter().position(|s| s.is_some_and(|v| v.item == it.id)).or(self.inv[..9].iter().position(|s| s.is_none())).unwrap_or(self.sel);
        self.inv[i] = Some(stack); self.sel = i; self.dirty = true;
    }
    /// Maj+clic sur une case : passe de la barre au sac, ou l'inverse.
    pub fn quick_slot(&mut self, i: usize) {
        let Some(mut s) = self.inv[i] else { return }; let range = if i < 9 { 9..INV_SIZE } else { 0..9 };
        for k in range.clone() { if let (Some(o), Some(n)) = (self.inv[k], s.n) { if o.item == s.item { if let Some(c) = o.n { let m = n.min(STACK - c); self.inv[k] = Some(Stack { n: Some(c + m), ..o }); s.n = Some(n - m); } } } }
        if s.n == Some(0) { self.inv[i] = None; self.dirty = true; return; }
        if let Some(k) = range.clone().find(|k| self.inv[*k].is_none()) { self.inv[k] = Some(s); self.inv[i] = None; } else { self.inv[i] = Some(s); }
        self.dirty = true;
    }
    pub fn progress(&self) -> f64 { match self.target { Some(t) if self.mine_key.is_some() => (self.mine_t / if self.felling.is_some() { FELL_TIME } else { BLOCKS[t.id as usize].hardness.max(0.01) }).min(1.0), _ => 0.0 } }
    pub fn set_creative(&mut self, on: bool) { self.creative = on; if !on { self.player.fly = false; } }
    /// Sauvegarde lisible : l'état du joueur, puis seulement ce qui diffère du monde généré.
    pub fn save_text(&self) -> String {
        let p = &self.player; let mut s = format!("A Living World 1\npalette {}\nseed {}\nplayer {} {} {} {} {}\nhour {}\ncreative {}\nsel {}\nvit {} {} {}\nweather {}\n", BLOCKS.iter().map(|b| b.name).collect::<Vec<_>>().join(" "), self.seed, p.p[0], p.p[1], p.p[2], p.yaw, p.pitch, self.hour, self.creative as u8, self.sel, self.vit[0], self.vit[1], self.vit[2], self.weather.t);
        for (i, it) in self.inv.iter().enumerate() { if let Some(v) = it { s += &format!("inv {} {} {}\n", i, v.item, v.n.map_or(-1, |n| n as i64)); } }
        for (k, e) in self.world.edits() { s += &format!("chunk {} {}", k.0, k.1); let mut l: Vec<_> = e.into_iter().collect(); l.sort(); for (i, id) in l { s += &format!(" {i}:{id}"); } s.push('\n'); }
        s
    }
    pub fn load_text(&mut self, t: &str) -> Result<(), String> {
        let mut lines = t.lines(); if lines.next() != Some("A Living World 1") { return Err("format inconnu".into()); }
        let mut edits: HashMap<(i32, i32), HashMap<u32, u8>> = HashMap::new(); let mut remap: Option<Vec<u8>> = None; let mut inv = [None; INV_SIZE];
        let num = |v: Option<&str>| v.and_then(|x| x.parse::<f64>().ok()).ok_or("nombre illisible");
        for l in lines {
            let mut a = l.split(' ');
            match a.next() {
                Some("player") => { for k in 0..3 { self.player.p[k] = num(a.next())?; } self.player.yaw = num(a.next())?; self.player.pitch = num(a.next())?; }
                Some("hour") => self.hour = num(a.next())?, Some("creative") => self.creative = num(a.next())? != 0.0, Some("sel") => self.sel = (num(a.next())? as usize).min(INV_SIZE - 1),
                Some("weather") => self.weather.t = num(a.next())?,
                Some("vit") => { for k in 0..3 { self.vit[k] = num(a.next())?; } }
                Some("inv") => { let i = num(a.next())? as usize; let id = a.next().unwrap_or(""); let n = num(a.next())?; if let (Some(it), true) = (item(id), i < INV_SIZE) { inv[i] = Some(Stack { item: it.id, n: if n < 0.0 { None } else { Some(n as u32) } }); } }
                // Les blocs sont sauvegardés par numéro ; la palette les retrouve par leur nom si la liste a changé.
                Some("palette") => remap = Some(a.map(|n| by_name(n).unwrap_or(0)).collect()),
                Some("chunk") => { let k = (num(a.next())? as i32, num(a.next())? as i32); let e = edits.entry(k).or_default(); for c in a { if let Some((i, id)) = c.split_once(':') { if let (Ok(i), Ok(id)) = (i.parse::<u32>(), id.parse::<u8>()) { e.insert(i, remap.as_ref().map_or(id, |r| r.get(id as usize).copied().unwrap_or(0))); } } } }
                _ => {}
            }
        }
        self.inv = inv; self.world.set_edits(edits); Ok(())
    }
}
