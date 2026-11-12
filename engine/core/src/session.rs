//! La partie en cours : joueur, monde, visée, casse et pose, sons, caméra. La survie, l'inventaire, le craft
//! et la sauvegarde sont dans play.rs. Aucun lien avec la fenêtre ou le rendu.
use crate::actors::{Anim, AnimIn, Draw, Models};
use crate::audio::{pick_mood, Situation, SoundScape};
use crate::blocks::*;
use crate::data::def;
use crate::fauna::{Ev, Fauna};
use crate::felling::{Fall, Tree};
use crate::flow::Flow;
use crate::gen::WorldGen;
use crate::player::{Input, Player, Voxels};
use crate::stream::{Preset, Streamer};
use crate::survival::*;
use crate::weather::Weather;
use std::collections::{HashMap, HashSet};

pub const INV_SIZE: usize = 36; pub const STACK: u32 = 99; pub const CHEST_SIZE: usize = 27;
/// Cases de l'écran d'inventaire : le sac (0 à 35), le coffre ouvert (36 à 89, la seconde moitié pour le coffre voisin), l'équipement (90 à 95).
/// Deux coffres côte à côte s'ouvrent ensemble : un grand coffre de deux fois vingt-sept cases.
pub const CHEST_VIEW: usize = 2 * CHEST_SIZE;
pub const CHEST0: usize = INV_SIZE; pub const EQUIP0: usize = INV_SIZE + CHEST_VIEW; pub const SLOTS_ALL: usize = EQUIP0 + 6;
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Item { pub id: &'static str, pub label: &'static str, pub block: Option<crate::blocks::Id>, pub tool: Option<&'static str> }
/// Tous les objets, dans l'ordre du fichier de données (c'est aussi l'ordre des icônes).
pub fn order() -> &'static [&'static str] { &crate::data::db().order }
/// Onglets du catalogue créatif, un par catégorie d'objets.
pub fn tabs() -> &'static [(&'static str, Vec<&'static str>)] { &crate::data::db().tabs }
pub fn item(id: &str) -> Option<Item> { def(id).map(|d| Item { id: d.id, label: d.label, block: d.block, tool: d.tool.map(|t| t.kind) }) }
/// Une pile : l'objet, le nombre (aucun pour un outil, qui ne s'empile pas), l'usure d'un outil.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Stack { pub item: &'static str, pub n: Option<u32>, pub wear: u32 }
impl Stack { pub fn new(item: &'static str, n: Option<u32>) -> Stack { Stack { item, n, wear: 0 } } }
#[derive(Clone, Copy, Debug)]
pub struct Target { pub x: i32, pub y: i32, pub z: i32, pub px: i32, pub py: i32, pub pz: i32, pub id: Id }
/// Parcours de grille : premier bloc ciblable et case d'avant.
pub fn raycast(w: &impl Voxels, o: [f64; 3], d: [f64; 3], max: f64, skip: &dyn Fn(i32, i32, i32, Id) -> bool) -> Option<(Target, f64)> {
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
/// L'animal visé : un cerf ou une bête des biomes froids et secs, par son rang dans sa liste.
#[derive(Clone, Copy, Debug, PartialEq)] pub enum Prey { Deer(usize), Wild(usize), Foe(usize) }
#[derive(Default, Clone, Copy)]
pub struct Actions { pub mine: bool, pub place: bool, pub guard: bool }
pub struct Session {
    pub world: Streamer, pub player: Player, pub inv: [Option<Stack>; INV_SIZE], pub sel: usize, pub creative: bool, pub target: Option<Target>, pub orient: u8, pub hour: f64, pub vit: [f64; 3], pub seed: i32, pub third: bool, pub cam_dist: f64, pub eye: [f64; 3], pub look: [f64; 3], pub anim: Anim, pub inv_open: bool, pub cursor: Option<Stack>, pub from: Option<usize>,
    pub(crate) mine_key: Option<(i32, i32, i32)>, pub mine_t: f64, pub(crate) cool: f64, axis: Option<char>, pub dirty: bool, pub tab: usize, /** catalogue créatif : première rangée affichée */ pub cat_row: usize,
    pub deep: f64, pub taiga: f64, pub steppe: f64, pub desert: f64, pub highlands: f64, /** présence des hautes montagnes autour du joueur, lissée */ pub mountains: f64, /** part de mer autour du joueur, lissée */ pub sea: f64, /** mer chaude (récifs) plutôt que fraîche */ pub sea_warm: f64, pub(crate) glows: HashMap<(i32, i32), Vec<[i32; 4]>>, /** toutes les sources de lumière posées (torches, lanternes, feux…), par chunk : x, y, z, niveau */ pub(crate) lamps: HashMap<(i32, i32), Vec<[i32; 4]>>, scanned: HashSet<(i32, i32)>, pub fauna: Fauna, pub wild: crate::wild::Wild, /** créatures hostiles */ pub horde: crate::hostile::Horde, /** adaptation par région et par biome */ pub regions: HashMap<(i32, i32, u8), crate::hostile::Region>, /** le danger là où l'on est */ pub danger: crate::hostile::Danger, branch_t: f64, pub weather: Weather, pub sound: SoundScape, pub(crate) gen: WorldGen, pub elapsed: f64, pub hurt_at: f64, built: Vec<f64>, pub(crate) swing_t: f64, pub(crate) dig_t: f64, step_t: f64, water_t: f64, env_t: f64, was_wet: bool, was_ground: bool, fall_v: f64, brook: Option<[f64; 3]>, fall: Option<[f64; 3]>, pub(crate) felling: Option<Tree>, pub falling: Option<Fall>, pub fall_serial: u64, pub flow: Flow,
    /// Survie : endurance, souffle et fatigue sur 100, température du corps, mort en cours, sacs laissés, point de réapparition, messages pour l'écran.
    pub stamina: f64, pub air: f64, pub fatigue: f64, pub body: f64, pub exposure: f64, pub winded: bool, pub(crate) spent_t: f64, pub dead: Option<Death>, pub stashes: Vec<Stash>, pub spawn: [f64; 3], pub bed: Option<[i32; 3]>, pub notes: Vec<String>, pub prey: Option<Prey>, pub cause: &'static str,
    pub(crate) low_v: f64, pub(crate) pain_t: f64, pub(crate) use_t: f64, pub(crate) hit_t: f64, pub(crate) mine_need: f64, pub(crate) climate_t: f64,
    /// Heures de jeu écoulées depuis le début ; buissons cueillis et blé semé, avec l'heure où ils l'ont été.
    pub hours: f64, pub regrow: Vec<([i32; 3], f64)>, pub crops: Vec<([i32; 3], f64)>, pub(crate) grow_t: f64,
    /// Équipement porté (tête, torse, jambes, pieds, dos), coffres du monde, coffre ouvert, feux qui brûlent (secondes restantes).
    pub equip: [Option<Stack>; 6], pub chests: HashMap<[i32; 3], Vec<Option<Stack>>>, pub chest: Option<[i32; 3]>, /** le coffre voisin, ouvert avec lui */ pub chest2: Option<[i32; 3]>, pub(crate) lids: Vec<[i32; 3]>, /** l'atelier dont l'interface est ouverte */ pub at_station: Option<(crate::data::Station, [i32; 3])>, /** l'ouvrage en cours à l'atelier */ pub job: Option<crate::play::Job>, /** le survol de l'écran titre */ pub tour: Option<crate::tour::Tour>, pub(crate) lit: Vec<([i32; 3], f64)>, pub fires: HashMap<[i32; 3], f64>,
    /// Profil de survie, compétences pratiquées, objets déjà eus en main (ils révèlent les recettes), recettes favorites.
    /// Objets posés au sol (jetés, ou rendus par un bloc cassé, une bête tuée).
    pub loose: Vec<crate::loose::Loose>,
    pub arrows: Vec<crate::outdoors::Arrow>, pub line: Option<crate::outdoors::Line>, pub turn: u8, pub(crate) ray_t: f64, pub profile: usize, pub skills: [f64; 5], pub seen: HashSet<&'static str>, pub taught: HashSet<&'static str>, pub fav: HashSet<&'static str>, pub last_gain: [f64; 6],
    /// Valeurs affichées des six indicateurs, lissées pour que l'icône se remplisse sans à-coup ; état du livre de recettes.
    pub gauge: [f64; 6], pub book: crate::play::Book,
    /// Villages proches et leurs habitants, réputation, villages connus, conversation en cours.
    pub town: crate::village::Town,
    /// Le terrain déjà vu, et la grande carte.
    pub map: crate::map::Atlas,
    /// Premiers pas : étape du guide et temps passé dessus.
    pub guide: usize, pub(crate) guide_t: f64,
    /// Combat : coup chargé (temps tenu depuis le dernier coup, -1 sans), garde levée (depuis quand, -1 sans).
    pub charge: f64, pub guard_t: f64, guard_cd: f64, /** corde de l'arc tendue depuis (s) */ pub bow_t: f64,
}
impl Session {
    pub fn new(seed: i32, p: &Preset, threads: usize, at: [f64; 3]) -> Self {
        let inv = [None; INV_SIZE]; // on commence les mains vides
        Session { world: Streamer::new(seed, p, threads), player: Player::new(at[0], at[1], at[2]), inv, sel: 0, creative: false, target: None, orient: 0, hour: 16.4, vit: [100.0; 3], seed, third: true, cam_dist: 4.2, eye: [0.0; 3], look: [0.0, 0.0, 1.0], anim: Anim::default(), inv_open: false, cursor: None, from: None, mine_key: None, mine_t: 0.0, cool: 0.0, axis: None, dirty: false, tab: 0, cat_row: 0, deep: 0.0, taiga: 0.0, steppe: 0.0, desert: 0.0, highlands: 0.0, mountains: 0.0, sea: 0.0, sea_warm: 0.0, glows: HashMap::new(), scanned: Default::default(), fauna: Fauna::new(seed), wild: crate::wild::Wild::new(seed), horde: crate::hostile::Horde::new(seed), regions: HashMap::new(), danger: Default::default(), branch_t: 10.0, weather: Weather::new(seed), sound: SoundScape::new(None), gen: WorldGen::new(seed), elapsed: 0.0, hurt_at: -99.0, built: Vec::new(), swing_t: 0.0, dig_t: 0.0, step_t: 0.0, water_t: 0.0, env_t: 0.0, was_wet: false, was_ground: false, fall_v: 0.0, brook: None, fall: None, felling: None, falling: None, fall_serial: 0, flow: Flow::default(),
            stamina: 100.0, air: 100.0, fatigue: 0.0, body: 37.0, exposure: 0.0, winded: false, spent_t: 9.0, dead: None, stashes: Vec::new(), spawn: at, bed: None, notes: Vec::new(), prey: None, cause: "", low_v: 0.0, pain_t: 0.0, use_t: 0.0, hit_t: 0.0, mine_need: 1.0, climate_t: 0.0,
            hours: 0.0, regrow: Vec::new(), crops: Vec::new(), grow_t: 0.0, equip: [None; 6], chests: HashMap::new(), lamps: HashMap::new(), chest2: None, lids: Vec::new(), at_station: None, job: None, tour: None, lit: Vec::new(), chest: None, fires: HashMap::new(), profile: 1, skills: [0.0; 5], bow_t: 0.0, loose: Vec::new(), arrows: Vec::new(), line: None, turn: 0, ray_t: 0.0, taught: HashSet::new(), seen: HashSet::new(), fav: HashSet::new(), last_gain: [-99.0; 6], gauge: [1.0, 1.0, 1.0, 1.0, 0.5, 1.0], book: Default::default(), town: Default::default(), map: Default::default(), guide: 0, guide_t: 0.0, charge: -1.0, guard_t: -1.0, guard_cd: 0.0 }
    }
    /// L'ancien équipement de départ : outils de fer, lanternes et quelques blocs. Pour les bancs d'essai et les captures.
    pub fn give_kit(&mut self) { for (i, (it, n)) in [("pickaxe", None), ("axe", None), ("sword", None), ("lantern", Some(32)), ("dirt", Some(16)), ("stone", Some(16)), ("planks", Some(16))].into_iter().enumerate() { self.inv[i] = Some(Stack::new(it, n)); self.seen.insert(it); } }
    /// Le temps passe : l'heure du jour tourne, le compte des heures avance (baies et blé poussent sur ce compte).
    pub fn pass_hours(&mut self, h: f64) { self.hour = (self.hour + h).rem_euclid(24.0); self.hours += h; }
    pub fn held(&self) -> Option<Item> { self.inv[self.sel].and_then(|s| item(s.item)) }
    /// Ce qui tenait sur le bloc retiré tombe avec lui : plantes au-dessus, lianes en dessous, champignons sur les côtés.
    pub(crate) fn drop_attached(&mut self, x: i32, y: i32, z: i32) {
        let g = |s: &Self, a, b, c| s.world.get(a, b, c).unwrap_or(0);
        let up = g(self, x, y + 1, z);
        if up > 0 && is_deco(up) && up != B::VINE && up != B::SHELF_FUNGUS && up != B::FENCE { self.world.edit(x, y + 1, z, 0); }
        let (mut k, mut n) = (y - 1, 0); while (g(self, x, k, z) == B::VINE || g(self, x, k, z) == B::ICICLE) && n < 12 { self.world.edit(x, k, z, 0); k -= 1; n += 1; }
        for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { if g(self, x + dx, y, z + dz) == B::SHELF_FUNGUS {
            let kept = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|(a, b)| { let c = g(self, x + dx + a, y, z + dz + b); c > 0 && BLOCKS[c as usize].opaque });
            if !kept { self.world.edit(x + dx, y, z + dz, 0); }
        } }
    }
    fn place_spot(t: &Target) -> [i32; 3] { if is_deco(t.id) && emit(t.id) == 0 && !is_solid(t.id) { [t.x, t.y, t.z] } else { [t.px, t.py, t.pz] } }
    /// Ce que le clic droit poserait, et où : une ou deux cases (une porte en prend deux). `None` : rien de possible ici.
    /// La pose et l'aperçu passent tous deux par ici, de sorte que l'aperçu montre exactement ce qui sera posé.
    pub fn plan(&self) -> Option<Vec<([i32; 3], crate::blocks::Id)>> {
        let (t, st) = (self.target?, self.inv[self.sel]?); let block = item(st.item)?.block?;
        if !self.creative && st.n.unwrap_or(0) == 0 { return None; }
        // Une torche posée contre le flanc d'un bloc se fixe au mur, penchée vers soi.
        if block == B::TORCH && t.py == t.y && (t.px != t.x || t.pz != t.z) && is_solid(t.id) {
            let f = crate::shapes::facing_of((t.x - t.px) as f64, (t.z - t.pz) as f64); let id = crate::shapes::variant(by_name("torch_wall")?, f, false, false, false);
            if self.world.get(t.px, t.py, t.pz).is_some_and(|c| c == 0 || !is_solid(c)) { return Some(vec![([t.px, t.py, t.pz], id)]); }
        }
        if let Some(si) = crate::shapes::info(block) { return self.plan_shape(si, &t); }
        let [x, y, z] = Self::place_spot(&t); let cur = self.world.get(x, y, z)?; if is_solid(cur) { return None; }
        let id = if block == B::SNOW_LAYER && snow_level(cur) > 0 { snow_of(snow_level(cur) + 1) } else { let f = self.player.forward(); placed_variant(oriented_block(block, gaze_axis(f[0], f[1], f[2], self.orient, self.axis))) };
        if self.blocks_player([x, y, z], id) { return None; }
        Some(vec![([x, y, z], id)])
    }
    /// Blocs de forme : sens du regard (plus les quarts de tour de R), dalle haute ou basse selon l'endroit visé, deux dalles
    /// qui font un bloc plein, porte sur deux cases.
    fn plan_shape(&self, si: crate::shapes::Info, t: &Target) -> Option<Vec<([i32; 3], crate::blocks::Id)>> {
        use crate::shapes::{facing_of, full_of_slab, info, variant, Kind};
        let f = self.player.forward(); let face = (facing_of(f[0], f[2]) + self.turn) % 4;
        if si.kind == Kind::Slab { if let Some(ti) = info(t.id) { if ti.kind == Kind::Slab && ti.base == si.base && ((!ti.top && t.py > t.y) || (ti.top && t.py < t.y)) { return full_of_slab(si.base).map(|b| vec![([t.x, t.y, t.z], b)]); } } }
        let [x, y, z] = Self::place_spot(t); let cur = self.world.get(x, y, z)?; if is_solid(cur) { return None; }
        let hit_y = self.eye[1] + self.look[1] * self.ray_t - t.y as f64;
        let top = matches!(si.kind, Kind::Slab | Kind::Stairs) && (t.py < t.y || (t.py == t.y && hit_y > 0.5)); // sous un plafond ou sur le haut d'un mur : dalle haute, escalier renversé
        let mut v = vec![([x, y, z], variant(si.base, face, top, false, false))];
        // Un lit : la tête là où l'on vise, le pied vers soi.
        if crate::shapes::is_bed(si.base) { let (dx, dz) = [(0, 1), (1, 0), (0, -1), (-1, 0)][face as usize % 4]; let q = [x - dx, y, z - dz];
            let c = self.world.get(q[0], q[1], q[2])?; if is_solid(c) || is_water(c) { return None; } if !self.world.get(q[0], q[1] - 1, q[2]).is_some_and(is_solid) { return None; }
            v.push((q, variant(crate::blocks::by_name("bed_foot")?, face, false, false, false))); }
        if si.kind == Kind::Door { let up = self.world.get(x, y + 1, z)?; if is_solid(up) || is_water(up) { return None; } v.push(([x, y + 1, z], variant(si.base, face, false, false, true))); }
        if v.iter().any(|(p, id)| self.blocks_player(*p, *id)) { return None; }
        Some(v)
    }
    /// Le bloc posé là rentrerait-il dans le joueur ? (Une dalle au ras du sol, non ; une porte, oui.)
    fn blocks_player(&self, c: [i32; 3], id: Id) -> bool {
        let boxes = crate::shapes::boxes(id).unwrap_or_else(|| if is_solid(id) { vec![[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]] } else { Vec::new() }); let p = self.player.p;
        boxes.iter().any(|b| { let (a, d) = ([c[0] as f64 + b[0], c[1] as f64 + b[1], c[2] as f64 + b[2]], [c[0] as f64 + b[3], c[1] as f64 + b[4], c[2] as f64 + b[5]]); a[0] < p[0] + 0.3 && d[0] > p[0] - 0.3 && a[2] < p[2] + 0.3 && d[2] > p[2] - 0.3 && a[1] < p[1] + 1.8 && d[1] > p[1] + 0.01 })
    }
    pub fn place(&mut self) -> bool {
        let Some(plan) = self.plan() else { return false }; let Some(st) = self.inv[self.sel] else { return false };
        if crate::shapes::info(plan[0].1).is_none() { let f = self.player.forward(); self.axis = Some(gaze_axis(f[0], f[1], f[2], self.orient, self.axis)); }
        let edits: Vec<(i32, i32, i32, Id)> = plan.iter().map(|(p, id)| (p[0], p[1], p[2], *id)).collect();
        if self.world.edit_many(&edits) == 0 { return false; }
        for (p, id) in &plan { self.flow.touch(p[0], p[1], p[2]); self.on_placed(*p, *id); }
        if !self.creative { let n = st.n.unwrap_or(0) - 1; self.inv[self.sel] = if n == 0 { None } else { Some(Stack { n: Some(n), ..st }) }; }
        self.sound.place(BLOCKS[plan[0].1 as usize].name); self.built.push(self.elapsed); self.dirty = true; true
    }
    /// Touche R : un quart de tour pour les blocs de forme, sinon le mode d'orientation des rondins et planches. Le message à afficher.
    pub fn rotate(&mut self) -> String {
        if self.inv[self.sel].and_then(|s| item(s.item)).and_then(|i| i.block).is_some_and(|b| crate::shapes::info(b).is_some()) { self.turn = (self.turn + 1) % 4; return format!("Rotation : {} °", self.turn as u32 * 90); }
        self.orient = (self.orient + 1) % MODES.len() as u8; format!("Orientation : {}", MODES[self.orient as usize])
    }
    /// Une étape de jeu. Le reste (rendu, son) lit l'état.
    pub fn tick(&mut self, dt: f64, i: Input, a: Actions) {
        // Mort, on ne commande plus rien ; essoufflé ou trop chargé, on ne court plus et on ne roule plus.
        let alive = self.dead.is_none(); let i = if alive { self.gate(i) } else { Input::default() };
        let (rolled, was_ground) = (self.player.rolling(), self.player.ground); self.player.update(&self.world, dt, i);
        let roll_start = self.player.rolling() && !rolled; if roll_start { self.sound.roll(); }
        let jumped = was_ground && !self.player.ground && self.player.v[1] > 8.0;
        let reach = if self.creative { 13.0 } else { 9.5 };
        let (eye, f) = self.camera(dt); self.eye = eye; self.look = f; let (third, head) = (self.third, [self.player.p[0], self.player.p[1] + 1.5, self.player.p[2]]);
        let hit = raycast(&self.world, eye, f, reach, &|x, y, z, _| third && (x as f64 + 0.5 - head[0]) * f[0] + (y as f64 + 0.5 - head[1]) * f[1] + (z as f64 + 0.5 - head[2]) * f[2] < -0.5);
        self.target = hit.map(|t| t.0); self.ray_t = hit.map_or(0.0, |t| t.1); // Herbes et fleurs ne cachent pas le gibier : seul un bloc solide fait écran.
        self.prey = if alive { self.aim_prey(eye, f, hit.filter(|t| is_solid(t.0.id)).map_or(f64::INFINITY, |t| t.1)) } else { None };
        let open = self.inv_open || self.player.rolling() || !alive; // ni coup ni pose en pleine roulade
        // Un animal visé passe avant le bloc derrière lui : le clic gauche frappe au lieu de creuser.
        let hunting = a.mine && !open && self.prey.is_some(); self.hit_t -= dt; self.use_t -= dt;
        // Un clic frappe tout de suite ; tenu, le bouton charge un coup lourd qui part de lui-même une fois plein.
        if a.mine && !open { if self.charge < 0.0 { if hunting && self.hit_t <= 0.0 { self.attack(false); self.charge = 0.0; } } else { self.charge += dt; if self.charge >= crate::survival::HEAVY_T { if self.prey.is_some() { self.attack(true); } self.charge = 0.0; } } } else { self.charge = -1.0; }
        // Clic droit tenu, bouclier au bras et rien à poser en main : la garde se lève.
        let bowing = self.inv[self.sel].is_some_and(|s| s.item == "bow"); self.bow_tick(dt, a.guard && !open && bowing);
        let guarding = a.guard && !open && !bowing && self.shield().is_some() && !self.held_places();
        // Baissée, la garde ne se relève qu'après un court instant : on ne pare pas en tapotant.
        self.guard_cd -= dt; if guarding && (self.guard_t >= 0.0 || self.guard_cd <= 0.0) { self.guard_t = self.guard_t.max(0.0) + dt; } else { if self.guard_t >= 0.0 { self.guard_cd = 0.6; } self.guard_t = -1.0; }
        self.mine_step(dt, a.mine && !open && !hunting); if a.place && !open && !self.interact() && self.place() { self.anim.trigger(); }
        self.job_tick(dt); self.lids_tick(dt); self.cool_furnaces(dt);
        let mining = (a.mine && !open) || self.job.is_some_and(|j| j.swings()); self.anim.update(dt, AnimIn { speed: self.player.speed, ground: self.player.ground, vy: self.player.v[1], crouch: self.player.crouch, fly: self.player.fly, mining, guard: self.guard_t >= 0.0 });
        if alive && !self.creative { self.survive(dt, i, jumped, roll_start, mining, was_ground); } else { self.low_v = 0.0; }
        self.outdoors(dt); self.loose_tick(dt); self.town_tick(dt); self.map_tick(dt); self.guide_tick(dt);
        self.elapsed += dt; self.weather.update(dt); let t = self.mix().strong(crate::biome::Biome::DeepForest); self.deep += (t - self.deep) * (dt / 2.5).min(1.0); let t = self.mix().strong(crate::biome::Biome::Taiga); self.taiga += (t - self.taiga) * (dt / 2.5).min(1.0); let t = self.mix().strong(crate::biome::Biome::Steppe); self.steppe += (t - self.steppe) * (dt / 2.5).min(1.0); let t = self.mix().strong(crate::biome::Biome::Desert); self.desert += (t - self.desert) * (dt / 2.5).min(1.0); let t = self.mix().strong(crate::biome::Biome::Highlands); self.highlands += (t - self.highlands) * (dt / 2.5).min(1.0); let m = self.mix(); let t = m.strong(crate::biome::Biome::Mountains); self.mountains += (t - self.mountains) * (dt / 2.5).min(1.0); self.sea += (m.sea - self.sea) * (dt / 2.5).min(1.0); let t = crate::noise::smooth(0.2, 0.45, m.temp); self.sea_warm += (t - self.sea_warm) * (dt / 2.5).min(1.0);
        self.world_step(dt); // baies, blé, feux, dépouilles
        let changes = self.flow.step(dt, &self.world); if !changes.is_empty() { self.world.edit_many(&changes); self.dirty = true; }
        if let Some(mut f) = self.falling.take() { if f.step(dt, &self.world) { self.sound.tree_land(); } if !f.done() { self.falling = Some(f); } }
        if self.world.loaded() > 0 {
            self.fauna.update(dt, &self.world, self.player.p, (self.sun_height() * 3.0).max(0.0));
            for e in std::mem::take(&mut self.fauna.ev) { match e { Ev::Bird(p, v) => self.sound.bird(p, v), Ev::Wings(p) => self.sound.wings(p), Ev::Hoof(p, f) => self.sound.hoof(p, f), Ev::Alarm(p) => self.sound.deer_alarm(p), Ev::Call(p) => self.sound.deer_call(p) } }
            let on = self.player.ground && !self.player.fly; self.wild.update(dt, &self.world, self.player.p, (self.sun_height() * 3.0).max(0.0), self.taiga, self.steppe, (1.0 - self.taiga - self.steppe - self.deep - self.desert).max(0.0) + 0.6 * self.highlands + 0.3 * self.mountains - self.sea, on, [self.deep, self.highlands.max(1.3 * self.mountains), self.desert, self.sea, self.sea_warm]); /* les bouquetins des hautes terres peuplent aussi la montagne, en plus grand nombre */
            for e in std::mem::take(&mut self.wild.ev) { match e { crate::wild::Ev::Bite(p, n, cause) => { self.sound.fox_bark(p); let k = 1.0 - (self.worn().2 / 40.0).min(0.6); let n = self.block(p, n); if n > 0.0 { self.harm((n * k * crate::survival::PROFILES[self.profile].harm).round().max(1.0), cause, true); } }
                crate::wild::Ev::Howl(p) => self.sound.fox_scream(p), crate::wild::Ev::Grunt(p) => self.sound.thump(p), crate::wild::Ev::Caw(p) => self.sound.caw(p), crate::wild::Ev::Cry(p) => self.sound.gull(p), crate::wild::Ev::Splash(p) => self.sound.splash_at(p), crate::wild::Ev::Bark(p) => self.sound.fox_bark(p), crate::wild::Ev::Scream(p) => self.sound.fox_scream(p), crate::wild::Ev::Thump(p) => self.sound.thump(p), crate::wild::Ev::Rustle(p) => self.sound.rustle(p) } }
        }
        if self.world.loaded() > 0 { self.hostile_step(dt); }
        self.audio_step(dt); self.smooth_gauges(dt);
    }
    /// Climat et biomes là où se tient le joueur.
    pub fn mix(&self) -> crate::biome::Mix { self.gen.climate.at(self.player.p[0], self.player.p[2]) }
    /// Reçoit les chunks prêts. L'eau d'un chunk neuf peut couler aussitôt : celle qui touche le vide se réveille.
    pub fn sync(&mut self) -> (Vec<crate::stream::Built>, Vec<(i32, i32)>) {
        let p = self.player.p; let (built, gone) = self.world.update(p[0], p[2]);
        for g in &gone { self.scanned.remove(g); self.glows.remove(g); self.lamps.remove(g); }
        // Champignons lumineux de chaque chunk reçu : leur lumière colorée en dépend.
        for c in &built { if c.lod == crate::stream::FAR { continue; } let n = CX as i32; let mut l = Vec::new(); for (i, b) in c.voxels.iter().enumerate() { if let Some(k) = glow_index(*b) { let i = i as i32; l.push([c.key.0 * n + i % n, i / (n * n), c.key.1 * n + (i / n) % n, k as i32]); } } if l.is_empty() { self.glows.remove(&c.key); } else { self.glows.insert(c.key, l); } }
        // Toutes les lumières posées : elles décident où la nuit peut enfanter ses créatures.
        for c in &built { if c.lod == crate::stream::FAR { continue; } let n = CX as i32; let l: Vec<[i32; 4]> = c.voxels.iter().enumerate().filter(|(_, b)| emit(**b) > 0).map(|(i, b)| { let i = i as i32; [c.key.0 * n + i % n, i / (n * n), c.key.1 * n + (i / n) % n, emit(*b) as i32] }).collect();
            if l.is_empty() { self.lamps.remove(&c.key); } else { self.lamps.insert(c.key, l); } }
        for c in &built { if c.lod != 0 || !self.scanned.insert(c.key) { continue; } let n = CX as i32; let (ox, oz) = (c.key.0 * n, c.key.1 * n); let v = c.voxels.clone();
            for y in 2..CH as i32 - 1 { for z in 0..n { for x in 0..n { if v[idx(x as usize, y as usize, z as usize)] != B::WATER { continue; } let (wx, wz) = (ox + x, oz + z);
                let open = [(1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1), (0, -1, 0)].iter().any(|d| self.world.get(wx + d.0, y + d.1, wz + d.2) == Some(0));
                if open { self.flow.touch(wx, y, wz); }
            } } }
        }
        (built, gone)
    }
    /// Lumières colorées proches : position et couleur des champignons lumineux, les plus près d'abord.
    /// Lumières ponctuelles près de `e` : position et portée (0 : portée courte des champignons), couleur.
    /// La torche ou la lanterne tenue en main éclaire en premier, avec sa flamme qui vacille.
    pub fn lights(&self, e: [f64; 3], max: usize) -> Vec<([f32; 4], [f32; 3])> {
        let mut out = Vec::new();
        if let Some(k) = self.inv[self.sel].and_then(|s| match s.item { "torch" => Some(1.0), "lantern" => Some(1.15), _ => None }) {
            let t = self.elapsed; let fl = (0.9 + 0.06 * (t * 11.0).sin() + 0.04 * (t * 23.7).sin()) as f32 * k as f32; let f = self.look; let p = self.player.eye();
            out.push(([(p[0] + f[0] * 0.4) as f32, (p[1] - 0.25) as f32, (p[2] + f[2] * 0.4) as f32, 11.0], [1.0 * fl, 0.62 * fl, 0.3 * fl]));
        }
        let max = max - out.len();
        let mut v: Vec<(f64, [i32; 4])> = self.glows.values().flatten().map(|p| ((p[0] as f64 + 0.5 - e[0]).powi(2) + (p[1] as f64 + 0.3 - e[1]).powi(2) + (p[2] as f64 + 0.5 - e[2]).powi(2), *p)).filter(|q| q.0 < 32.0 * 32.0).collect();
        v.sort_by(|a, b| a.0.total_cmp(&b.0)); v.truncate(max);
        out.extend(v.into_iter().map(|(_, p)| { let c = crate::mesher::GLOW[p[3] as usize]; ([p[0] as f32 + 0.5, p[1] as f32 + 0.35, p[2] as f32 + 0.5, 0.0], [c[0] as f32, c[1] as f32, c[2] as f32]) })); out
    }
    /// Hauteur du soleil, négative la nuit.
    pub fn sun_height(&self) -> f64 { crate::det::sin((self.hour - 6.0) / 12.0 * std::f64::consts::PI) }
    /// La caméra est-elle sous l'eau ?
    pub fn under(&self) -> bool { self.world.get(self.eye[0].floor() as i32, self.eye[1].floor() as i32, self.eye[2].floor() as i32).is_some_and(is_wet) }
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
        let below = at(self, [p[0], p[1] - 0.1, p[2]]).unwrap_or(0); let feet = at(self, [p[0], p[1] + 0.1, p[2]]).unwrap_or(0); let name = |b: Id| BLOCKS[b as usize].name; let pl = &self.player;
        if pl.ground && pl.speed > 1.0 {
            self.step_t -= dt * pl.speed;
            if self.step_t <= 0.0 { self.step_t = 2.1; if snow_level(feet) > 0 { self.sound.step(name(feet), None); } else if below > 0 { self.sound.step(name(below), if feet != below && feet > 0 { Some(name(feet)) } else { None }); } } // dans la neige, c'est elle qui craque
        } else if wet && pl.speed > 1.0 { self.step_t -= dt * pl.speed; if self.step_t <= 0.0 { self.step_t = 2.6; self.sound.step("water", None); } }
        if pl.ground && !self.was_ground && self.fall_v < -7.0 && !pl.fly && below > 0 { self.sound.land(name(below)); }
        self.was_ground = pl.ground; let vy = pl.v[1]; self.fall_v = if vy < -1.0 { vy } else if pl.ground { self.fall_v * 0.5 } else { self.fall_v };
        // Taïga : le vent passe dans les sapins ; de temps en temps une branche casse sous le poids de la neige.
        self.sound.breeze = (0.13 * self.taiga + 0.07 * self.steppe + 0.16 * self.highlands + 0.2 * self.mountains * (0.5 + self.player.p[1].clamp(70.0, 115.0) / 90.0)) as f32; /* en montagne, le vent forcit avec l'altitude */ self.branch_t -= dt;
        if self.branch_t <= 0.0 { self.branch_t = 14.0 + 30.0 * (crate::noise::hash_float(self.elapsed as i32, 7, self.seed, 3)); if self.taiga > 0.5 { let a = self.elapsed * 2.39; self.sound.branch([p[0] + a.sin() * 22.0, p[1] + 4.0, p[2] + a.cos() * 22.0]); } }
        self.env_t -= dt;
        if self.env_t <= 0.0 {
            self.env_t = 0.2; let covered = self.covered(eye); let sub = at(self, eye).is_some_and(is_wet); let brook = if self.flow.quiet < 2.0 { self.flow.last.or(self.brook) } else { self.brook }; let surf = (self.sea * 2.5).min(1.0) * (0.55 + 0.45 * (self.elapsed * 0.75).sin().abs()) * if sub { 0.25 } else { 1.0 }; /* les vagues : un souffle qui enfle et retombe */ self.sound.env(w.cloud, w.rain * (1.0 - self.taiga), covered, sub, brook, self.fall, surf);
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
        let mut o = if self.third { self.anim.third(md, p.p, if p.speed > 0.2 && roll.is_none() { p.move_yaw } else { p.yaw }, p.pitch, held, roll, self.equip.map(|e| e.map(|st| crate::actors::family(st.item)))) } else { self.anim.first(md, view, held, self.shield().map(|st| crate::actors::family(st.item))) };
        let n0 = o.len(); o.extend(self.fauna.draws(md)); o.extend(self.wild.draws(md)); o.extend(self.horde.draws(md)); o.extend(self.town.draws(md)); o.extend(self.loose_draws(md));
        // Les torches et les feux éclairent aussi les bêtes, les créatures et les villageois : le niveau de lumière posée
        // là où se tient chaque pièce part avec elle (la partie entière paire de `hurt`, lue par le shader).
        for d in o[n0..].iter_mut() { let lv = self.lamp_at(d.m[12].floor() as i32, (d.m[13] + 0.3).floor() as i32, d.m[14].floor() as i32); if lv > 0 { d.hurt += 2.0 * lv as f32; } }
        // Flèches en vol ou plantées, dans l'axe de leur trajectoire ; le bouchon de la ligne, qui plonge quand ça mord.
        for a in &self.arrows { let h = a.v[0].hypot(a.v[2]).max(1e-6); o.push(Draw { m: crate::view::trs(a.p[0], a.p[1], a.p[2], a.v[0].atan2(a.v[2]), -(a.v[1] / h).atan(), 1.0), r: md.part("arrow"), view: false, local: false, hero: false, dynamic: false, hurt: 0.0 }); }
        if let Some(l) = self.line { let dip = if self.elapsed >= l.bite && self.elapsed <= l.bite + crate::outdoors::BITE_WINDOW { -0.12 - 0.05 * (self.elapsed * 18.0).sin() } else { 0.02 * (self.elapsed * 2.0).sin() }; o.push(Draw { m: crate::view::trs(l.spot[0], l.spot[1] + dip, l.spot[2], 0.0, 0.0, 1.0), r: md.part("bobber"), view: false, local: false, hero: false, dynamic: false, hurt: 0.0 }); }
        for (k, s) in self.stashes.iter().enumerate() { o.push(Draw { m: crate::view::trs(s.p[0], s.p[1], s.p[2], k as f64 * 2.4 + 0.6, 0.0, 1.0), r: md.part("stash"), view: false, local: false, hero: false, dynamic: false, hurt: 0.0 }); } if let Some(f) = &self.falling { o.extend(f.draws(md)); } o
    }
    /// Où irait le bloc tenu, et serait-il accepté ?
    /// Aperçu de pose : la case, si la pose est possible, et la forme (au plus deux boîtes, relatives à la case).
    pub fn ghost(&self) -> Option<([i32; 3], bool)> {
        let (t, st) = (self.target?, self.inv[self.sel]?); item(st.item)?.block?;
        match self.plan() { Some(v) => Some((v[0].0, true)), None => Some((Self::place_spot(&t), false)) }
    }
    pub fn ghost_boxes(&self) -> Vec<[f64; 6]> {
        let Some(v) = self.plan() else { return Vec::new() }; let o = v[0].0; let mut out = Vec::new();
        for (p, id) in &v { let d = [(p[0] - o[0]) as f64, (p[1] - o[1]) as f64, (p[2] - o[2]) as f64]; for b in crate::shapes::boxes(*id).unwrap_or_else(|| vec![[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]]) { out.push([b[0] + d[0], b[1] + d[1], b[2] + d[2], b[3] + d[0], b[4] + d[1], b[5] + d[2]]); } }
        out.truncate(2); out
    }
    pub fn progress(&self) -> f64 { match self.target { Some(_) if self.mine_key.is_some() => (self.mine_t / self.mine_need.max(0.01)).min(1.0), _ => 0.0 } }
    pub fn set_creative(&mut self, on: bool) { self.creative = on; if !on { self.player.fly = false; } }
}
