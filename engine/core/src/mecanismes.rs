//! Les mécanismes des donjons : un système de signaux générique, réutilisable d'un donjon à l'autre.
//!
//! **Entrées** : le levier (manœuvré au clic droit) et la dalle à pression (enfoncée sous le joueur, une créature, ou un bloc à
//! pousser qu'on y a amené).
//!
//! **Le fil** (`Wire`, nommé) est le contrôleur logique. Il lit ses entrées et l'état d'autres fils, et commande des effets.
//! - Opérateurs : OU (une entrée suffit), ET (`all` : toutes), NON (`not` : inversé), ET d'autres fils (`needs` : des fils
//!   nommés qui doivent être enclenchés), SÉQUENCE (`seq` : les leviers dans l'ordre ; un faux pas relève tous les leviers,
//!   et peut déclencher les fléchettes : `fail_darts`).
//! - Durée : `hold` (s) : une fois enclenché, il le reste ce temps, puis se relâche et relève ses leviers (la porte à
//!   retardement) ; `latch` : une fois enclenché, il le reste pour de bon (l'énigme résolue).
//! - Effets : des grilles (ou des pans de mur : le passage secret) qui s'ouvrent, des pics qui sortent (`spikes`) ou qui
//!   rentrent (`calm` : le piège désarmé), des lance-fléchettes qui tirent (`darts`).
//!
//! **Le rythme** (`Pulse`) : des pics qui sortent et rentrent à intervalle régulier, des lance-fléchettes qui tirent à chaque
//! cycle, que l'on observe avant de passer ; un fil peut l'arrêter (`off_by`) ou le mettre en marche (`on_by`).
//!
//! **Le sceau** (`Seal`) : une zone de pierre ancienne qu'on ne peut ni creuser ni bâtir tant qu'un fil n'est pas enclenché
//! (ou jamais) ; les blocs faits pour être manipulés (leviers, dalles, blocs à pousser, coffres, torches) restent libres. Le
//! mode créatif passe outre.
//!
//! À part : les pics sortis blessent qui marche dessus, le sol friable cède un instant après qu'on y a mis le pied.
//! Tout se sauvegarde ; la commande « /fils » montre les fils proches, leurs entrées et leur état (outil de mise au point).
use crate::blocks::{by_name, Id, BLOCKS};
use crate::player::Voxels;
use crate::session::Session;
use std::collections::BTreeMap;

/// Un fil : ses entrées, sa logique, ce qu'il commande, son état.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Wire {
    /// son nom (pour être lu par d'autres fils, les rythmes et les sceaux)
    pub name: String,
    /// leviers ou dalles à pression
    pub inputs: Vec<[i32; 3]>,
    /// toutes les entrées doivent être actives (sinon une seule suffit)
    pub all: bool,
    /// les leviers, dans l'ordre de la liste
    pub seq: bool,
    /// inversé : enclenché quand ses entrées ne le sont pas
    pub not: bool,
    /// d'autres fils, qui doivent être enclenchés aussi
    pub needs: Vec<String>,
    /// les cases d'une grille (ou d'un pan de mur) et le bloc qui les ferme
    pub doors: Vec<([i32; 3], Id)>,
    /// les lance-fléchettes qui tirent quand le fil s'enclenche
    pub darts: Vec<[i32; 3]>,
    /// les pics qui sortent quand le fil s'enclenche (et rentrent quand il se relâche)
    pub spikes: Vec<[i32; 3]>,
    /// les pics qui rentrent quand le fil s'enclenche (le piège désarmé)
    pub calm: Vec<[i32; 3]>,
    /// une séquence ratée fait tirer les fléchettes
    pub fail_darts: bool,
    /// une fois enclenché, il le reste
    pub latch: bool,
    /// une fois enclenché, il le reste ce temps (s), puis relève ses leviers
    pub hold: f64,
    pub on: bool,
    /// où en est la séquence
    pub progress: usize,
    /** état interne : le temps qui reste, les entrées au pas précédent, la logique avant durée et inversion */ pub timer: f64, pub last: Vec<bool>, pub raw: bool,
    /// des statues qui doivent regarder dans une direction (leur orientation, 0 à 3) : toutes, en plus des entrées
    pub faces: Vec<([i32; 3], u8)>,
}
/// Une citerne : des colonnes d'eau (x, z, fond) qui se remplissent jusqu'à `top` tant que le fil `by` est enclenché, et se
/// vident quand il se relâche, une couche à la fois.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pool { pub cols: Vec<[i32; 3]>, pub top: i32, pub by: String, /** le haut de l'eau (sous le fond : vide) */ pub level: i32, pub t: f64 }
/// Un rythme : des pics qui sortent un temps à chaque cycle, des fléchettes qui partent au début de chaque cycle.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pulse { pub spikes: Vec<[i32; 3]>, pub darts: Vec<[i32; 3]>, /** s */ pub period: f64, /** part du cycle où les pics sont sortis */ pub up: f64, /** décalage (s) */ pub phase: f64, /** un fil enclenché l'arrête */ pub off_by: String, /** seul un fil enclenché le met en marche */ pub on_by: String, pub was_up: Option<bool>, pub cycle: i64 }
/// Un sceau : de `min` à `max`, on ne creuse ni ne bâtit, tant que le fil `until` n'est pas enclenché (vide : pour toujours).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Seal { pub min: [i32; 3], pub max: [i32; 3], pub until: String }
/// Un retour : tant que le fil `keep` n'est pas enclenché, dès que le joueur a quitté la salle (`min`..`max`, à une case près),
/// ses blocs à pousser reviennent à leur place (`homes`) : on ne se coince jamais pour de bon.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Home { pub min: [i32; 3], pub max: [i32; 3], pub homes: Vec<[i32; 3]>, pub keep: String }
/// Une inscription, lue une fois en entrant dans la salle (`min`..`max`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Hint { pub min: [i32; 3], pub max: [i32; 3], pub text: String, pub shown: bool, /** l'expérience d'une découverte (une salle secrète) */ pub xp: f64 }
/// Le gardien d'une salle : il s'éveille quand le joueur y entre (`min`..`max`), les grilles (`doors`) se referment derrière
/// lui ; vaincu, elles se relèvent pour de bon. Si le joueur s'en va (ou meurt), le gardien se rendort et tout se rouvre.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Guard { pub min: [i32; 3], pub max: [i32; 3], pub species: String, pub at: [f64; 3], pub doors: Vec<([i32; 3], Id)>, /** 0 endormi, 1 au combat, 2 vaincu */ pub state: u8 }
/// Tous les mécanismes du monde.
#[derive(Clone, Debug, Default)]
pub struct Mech { pub wires: Vec<Wire>, pub pulses: Vec<Pulse>, pub seals: Vec<Seal>, pub homes: Vec<Home>, pub hints: Vec<Hint>, pub guards: Vec<Guard>, pub pools: Vec<Pool>, /** les coffres au trésor d'un donjon, et ceux de ses salles secrètes */ pub chests: Vec<[i32; 3]>, pub secrets: Vec<[i32; 3]>, /** les coffres au trésor des donjons assemblés */ pub vaults: Vec<[i32; 3]>, crumble: BTreeMap<[i32; 3], f64>, hurt_t: f64, clock: f64, home_t: f64 }

/// Les dégâts des pics (par atteinte), d'une fléchette, et le temps que tient le sol friable.
pub const SPIKE_DMG: f64 = 5.0; pub const DART_DMG: f64 = 7.0; pub const CRUMBLE_T: f64 = 0.6;
fn id(n: &str) -> Id { by_name(n).unwrap_or(0) }
fn name(b: Id) -> &'static str { BLOCKS[b as usize].name }
/// Le nom sans son sens (« lever_on@2 » → « lever_on ») et le sens.
fn split(n: &str) -> (&str, &str) { match n.split_once('@') { Some((a, f)) => (a, f), None => (n, "") } }
fn with_face(base: &str, f: &str) -> Id { id(&if f.is_empty() { base.to_string() } else { format!("{base}@{f}") }) }
/// Ce qu'on peut manipuler dans une zone scellée.
fn free(b: Id) -> bool { matches!(split(name(b)).0, "lever" | "lever_on" | "pressure_plate" | "pressure_plate_on" | "chest" | "torch" | "torch_wall" | "lantern") || crate::blocks::is_chest(b) }

impl Mech {
    pub fn wire(&self, n: &str) -> Option<&Wire> { self.wires.iter().find(|w| w.name == n) }
    fn is_on(&self, n: &str) -> bool { self.wire(n).is_some_and(|w| w.on) }
    /// Ajoute les mécanismes d'un donjon.
    pub fn absorb(&mut self, o: Mech) { self.wires.extend(o.wires); self.pulses.extend(o.pulses); self.seals.extend(o.seals); self.homes.extend(o.homes); self.hints.extend(o.hints); self.guards.extend(o.guards); self.pools.extend(o.pools); self.chests.extend(o.chests); self.secrets.extend(o.secrets); self.vaults.extend(o.vaults); }
}
fn inside(p: [i32; 3], min: [i32; 3], max: [i32; 3], m: i32) -> bool { (0..3).all(|i| p[i] >= min[i] - m && p[i] <= max[i] + m) }

impl Session {
    /// Un levier est-il abaissé, une dalle enfoncée ?
    fn input_on(&self, p: [i32; 3]) -> bool { self.world.get(p[0], p[1], p[2]).is_some_and(|b| { let n = split(name(b)).0; n == "lever_on" || n == "pressure_plate_on" }) }
    /// Quelque chose pèse-t-il sur la dalle `p` : le joueur, une créature, un bloc à pousser ?
    fn weighs(&self, p: [i32; 3]) -> bool {
        let on = |q: [f64; 3]| q[0].floor() as i32 == p[0] && q[2].floor() as i32 == p[2] && ((q[1] - (p[1] + 1) as f64).abs() < 0.3);
        (self.dead.is_none() && !self.player.fly && on(self.player.p)) || self.horde.foes.iter().any(|f| f.dead == 0.0 && on(f.p)) || self.wild.all.iter().any(|c| on(c.p))
            || self.world.get(p[0], p[1] + 1, p[2]) == Some(id("push_block"))
    }
    /// Relève (remet au repos) les leviers d'une liste.
    fn levers_up(&mut self, list: &[[i32; 3]]) { for p in list { if let Some(b) = self.world.get(p[0], p[1], p[2]) { let (n, f) = split(name(b)); if n == "lever_on" { self.world.edit(p[0], p[1], p[2], with_face("lever", f)); } } } }
    /// Les mécanismes, à chaque pas de temps.
    pub(crate) fn mech_tick(&mut self, dt: f64) {
        self.mech.clock += dt;
        // les dalles des fils s'enfoncent ou remontent
        let plates: Vec<[i32; 3]> = self.mech.wires.iter().flat_map(|w| w.inputs.iter().copied()).filter(|p| self.world.get(p[0], p[1], p[2]).is_some_and(|b| split(name(b)).0.starts_with("pressure_plate"))).collect();
        for p in plates { let down = self.weighs(p); if down != self.input_on(p) { self.world.edit(p[0], p[1], p[2], id(if down { "pressure_plate_on" } else { "pressure_plate" })); self.sound.place("stone"); } }
        // les fils lisent leurs entrées
        for k in 0..self.mech.wires.len() {
            let w = &self.mech.wires[k]; if w.inputs.is_empty() && w.needs.is_empty() && w.faces.is_empty() { continue; }
            let faces_ok = w.faces.iter().all(|(p, f)| self.facing_at(*p) == Some(*f));
            let cur: Vec<bool> = w.inputs.iter().map(|p| self.input_on(*p)).collect();
            // la séquence : chaque levier abaissé doit être le suivant ; sinon tout se relève
            let mut fail = false; let mut progress = w.progress;
            // (sans mémoire du pas précédent — un fil tout juste branché ou relu —, les leviers sont supposés relevés ; ceux déjà
            // comptés dans la séquence ne sont pas un faux pas)
            if w.seq && !(w.latch && w.on) { let last = if w.last.len() == cur.len() { w.last.clone() } else { vec![false; cur.len()] };
                for i in 0..cur.len() { if cur[i] && !last[i] { if i == progress { progress += 1; } else if i > progress { fail = true; } } if !cur[i] && last[i] && i < progress { progress = i; } } }
            let mut raw = if w.inputs.is_empty() { faces_ok } else if !faces_ok { false } else if w.seq { !fail && progress == w.inputs.len() } else if w.all { cur.iter().all(|c| *c) } else { cur.iter().any(|c| *c) };
            raw = raw && w.needs.iter().all(|n| self.mech.is_on(n));
            let (inputs, fail_darts, darts) = (w.inputs.clone(), w.fail_darts, w.darts.clone());
            let w = &mut self.mech.wires[k]; w.last = cur; w.progress = if fail { 0 } else { progress };
            if fail { if w.fail_darts { /* tirées plus bas */ } self.levers_up(&inputs); self.notes.push("Un déclic sourd : les leviers se relèvent".into()); self.sound.place("iron_bars"); if fail_darts { for p in &darts { self.dart(*p); } } if let Some(w) = self.mech.wires.get_mut(k) { w.last = vec![false; inputs.len()]; } }
            // la durée : enclenché, il le reste `hold` secondes, puis se relâche et relève ses leviers
            let w = &mut self.mech.wires[k]; let rising = raw && !w.raw; w.raw = raw;
            let mut act = if w.hold > 0.0 { if rising { w.timer = w.hold; } let was = w.timer > 0.0; w.timer = (w.timer - dt).max(0.0); if was && w.timer <= 0.0 { let l = w.inputs.clone(); self.levers_up(&l); self.notes.push("Clac : le mécanisme se relâche".into()); } self.mech.wires[k].timer > 0.0 } else { raw };
            let w = &self.mech.wires[k]; if w.not { act = !act; }
            if act == w.on || (w.latch && w.on) { continue; }
            let w = self.mech.wires[k].clone(); self.mech.wires[k].on = act; self.dirty = true;
            for (p, closed) in &w.doors { let now = self.world.get(p[0], p[1], p[2]); if act && now == Some(*closed) { self.world.edit(p[0], p[1], p[2], 0); } else if !act && now == Some(0) { self.world.edit(p[0], p[1], p[2], *closed); } }
            if !w.doors.is_empty() { self.sound.place("iron_bars"); if act { self.notes.push(if w.doors.iter().all(|(_, b)| split(name(*b)).0 == "iron_bars") { "Une grille se lève" } else if w.doors.iter().all(|(_, b)| name(*b) == "chain") { "Une chaîne se détache et tombe dans un fracas" } else { "Un pan de mur pivote : un passage !" }.into()); } }
            for p in &w.spikes { self.world.edit(p[0], p[1], p[2], id(if act { "spikes" } else { "spikes_down" })); }
            for p in &w.calm { self.world.edit(p[0], p[1], p[2], id(if act { "spikes_down" } else { "spikes" })); }
            if act { for p in &w.darts { self.dart(*p); } if w.hold > 0.0 { self.notes.push(format!("Vite : {:.0} secondes avant que ça ne retombe", w.hold)); } }
        }
        // les citernes : une couche d'eau de plus (ou de moins) toutes les 0,3 s
        for k in 0..self.mech.pools.len() { let pl = &self.mech.pools[k]; let bottom = pl.cols.iter().map(|c| c[2]).min().unwrap_or(0);
            let target = if self.mech.is_on(&pl.by) { pl.top } else { bottom - 1 }; if pl.level == target { continue; }
            let t = pl.t - dt; if t > 0.0 { self.mech.pools[k].t = t; continue; } let pl = self.mech.pools[k].clone(); let water = crate::blocks::B::WATER;
            let (y, put, next) = if target > pl.level { (pl.level + 1, water, pl.level + 1) } else { (pl.level, 0, pl.level - 1) };
            let e: Vec<(i32, i32, i32, Id)> = pl.cols.iter().filter(|c| c[2] <= y).filter(|c| self.world.get(c[0], y, c[1]) == Some(if put == 0 { water } else { 0 })).map(|c| (c[0], y, c[1], put)).collect();
            self.world.edit_many(&e); self.mech.pools[k].level = next; self.mech.pools[k].t = 0.3; self.dirty = true;
            if self.near(&pl.cols.iter().map(|c| [c[0], y, c[1]]).take(1).collect::<Vec<_>>(), 24.0) && (pl.level == bottom - 1 || next == bottom - 1) { self.sound.place("water"); self.notes.push(if put != 0 { "L'eau monte en grondant".into() } else { "L'eau se retire".into() }); } }
        // les rythmes
        let clock = self.mech.clock;
        for k in 0..self.mech.pulses.len() {
            let pu = &self.mech.pulses[k]; let live = (pu.off_by.is_empty() || !self.mech.is_on(&pu.off_by)) && (pu.on_by.is_empty() || self.mech.is_on(&pu.on_by));
            let t = (clock + pu.phase).max(0.0); let cycle = (t / pu.period.max(0.1)).floor() as i64; let up = live && (t % pu.period.max(0.1)) < pu.up * pu.period;
            let (spikes, darts, fire) = (pu.spikes.clone(), pu.darts.clone(), live && cycle != pu.cycle);
            if pu.was_up != Some(up) { for p in &spikes { self.world.edit(p[0], p[1], p[2], id(if up { "spikes" } else { "spikes_down" })); } if up && !spikes.is_empty() && self.near(&spikes, 24.0) { self.sound.place("iron_bars"); } }
            if fire && self.near(&darts, 32.0) { for p in &darts { self.dart(*p); } }
            let pu = &mut self.mech.pulses[k]; pu.was_up = Some(up); pu.cycle = cycle;
        }
        // les inscriptions, lues en entrant
        let me = [self.player.p[0].floor() as i32, (self.player.p[1] + 0.1).floor() as i32, self.player.p[2].floor() as i32];
        for k in 0..self.mech.hints.len() { let h = &self.mech.hints[k]; if !h.shown && self.dead.is_none() && inside(me, h.min, h.max, 0) { let (t, xp) = (h.text.clone(), h.xp); self.mech.hints[k].shown = true; self.dirty = true; self.notes.push(t); if xp > 0.0 { self.earn(xp, "salle secrète découverte"); } } }
        // les gardiens des salles
        for k in 0..self.mech.guards.len() { let g = self.mech.guards[k].clone(); let Some(sp) = crate::hostile::species(&g.species) else { continue };
            let here = self.dead.is_none() && inside(me, g.min, g.max, 0);
            let alive = |s: &Session| s.horde.foes.iter().any(|f| f.sp == sp && f.dead == 0.0 && inside([f.p[0].floor() as i32, f.p[1].floor() as i32, f.p[2].floor() as i32], g.min, g.max, 2));
            let doors = |s: &mut Session, shut: bool| { for (p, b) in &g.doors { let now = s.world.get(p[0], p[1], p[2]); if shut && now == Some(0) { s.world.edit(p[0], p[1], p[2], *b); } else if !shut && now == Some(*b) { s.world.edit(p[0], p[1], p[2], 0); } } s.sound.place("iron_bars"); };
            match g.state {
                0 if here => { let me_f = self.player.p; let i = self.horde.put(sp, g.at, (me_f[0] - g.at[0]).atan2(me_f[2] - g.at[2]), true); let f = &mut self.horde.foes[i]; f.kept = true; f.hp *= 2.0; f.max = f.hp; f.alert = 1.0; f.seen = Some(me_f); f.state = crate::hostile::Ai::Poursuite;
                    doors(self, true); self.mech.guards[k].state = 1; self.dirty = true; self.notes.push(format!("Le gardien s'éveille : {} ! Les grilles retombent derrière vous", crate::hostile::bestiary().species[sp].name)); }
                1 => { let a = alive(self);
                    if !a && here { doors(self, false); self.mech.guards[k].state = 2; self.dirty = true; self.notes.push("Le gardien est vaincu : les grilles se relèvent".into()); }
                    else if !here { let (lo, hi) = (g.min, g.max); self.horde.foes.retain(|f| !(f.sp == sp && inside([f.p[0].floor() as i32, f.p[1].floor() as i32, f.p[2].floor() as i32], lo, hi, 2))); doors(self, false); self.mech.guards[k].state = 0; self.dirty = true; } }
                _ => {}
            } }
        // les blocs à pousser d'une salle qu'on a quittée reviennent à leur place
        self.mech.home_t -= dt;
        if self.mech.home_t <= 0.0 { self.mech.home_t = 0.5; let pb = id("push_block");
            for k in 0..self.mech.homes.len() { let h = self.mech.homes[k].clone(); if inside(me, h.min, h.max, 1) || (!h.keep.is_empty() && self.mech.is_on(&h.keep)) || !self.near(&h.homes, 64.0) { continue; }
                let mut found = Vec::new(); let mut loaded = true;
                'scan: for x in h.min[0]..=h.max[0] { for y in h.min[1]..=h.max[1] { for z in h.min[2]..=h.max[2] { match self.world.get(x, y, z) { None => { loaded = false; break 'scan; } Some(b) if b == pb => found.push([x, y, z]), _ => {} } } } }
                if !loaded || h.homes.iter().all(|p| found.contains(p)) { continue; }
                let mut e: Vec<(i32, i32, i32, Id)> = found.iter().filter(|p| !h.homes.contains(p)).map(|p| (p[0], p[1], p[2], 0)).collect();
                for p in &h.homes { if !found.contains(p) && self.world.get(p[0], p[1], p[2]) == Some(0) { e.push((p[0], p[1], p[2], pb)); } }
                self.world.edit_many(&e); self.dirty = true; if self.near(&h.homes, 28.0) { self.notes.push("Un grondement sourd : les blocs de pierre ont repris leur place".into()); self.sound.place("stone"); }
            } }
        // les pics sortis blessent ; le sol friable cède
        if self.dead.is_some() || self.creative || self.player.fly { return; }
        let f = [self.player.p[0].floor() as i32, (self.player.p[1] + 0.05).floor() as i32, self.player.p[2].floor() as i32];
        self.mech.hurt_t -= dt; if self.world.get(f[0], f[1], f[2]) == Some(id("spikes")) && self.mech.hurt_t <= 0.0 { self.mech.hurt_t = 0.7; self.harm(SPIKE_DMG, "Des pics dans un donjon", false); }
        let below = [f[0], f[1] - 1, f[2]];
        if self.player.ground && self.world.get(below[0], below[1], below[2]) == Some(id("crumbling_floor")) && !self.mech.crumble.contains_key(&below) { self.mech.crumble.insert(below, CRUMBLE_T); self.sound.place("gravel"); }
        let mut fall = Vec::new(); for (p, t) in self.mech.crumble.iter_mut() { *t -= dt; if *t <= 0.0 { fall.push(*p); } }
        for p in fall { self.mech.crumble.remove(&p); if self.world.get(p[0], p[1], p[2]) == Some(id("crumbling_floor")) { self.world.edit(p[0], p[1], p[2], 0); self.sound.place("gravel"); self.dirty = true; } }
    }
    fn near(&self, l: &[[i32; 3]], r: f64) -> bool { l.iter().any(|p| (p[0] as f64 + 0.5 - self.player.p[0]).hypot(p[2] as f64 + 0.5 - self.player.p[2]) < r) }
    /// Une case scellée : on n'y creuse ni n'y bâtit (sauf ce qui est fait pour être manipulé, et en créatif).
    pub fn sealed(&self, p: [i32; 3], b: Id) -> bool {
        if self.creative || free(b) { return false; }
        self.mech.seals.iter().any(|s| (0..3).all(|i| p[i] >= s.min[i] && p[i] <= s.max[i]) && (s.until.is_empty() || !self.mech.is_on(&s.until)))
    }
    /// Un lance-fléchettes tire, par sa face libre la plus proche du joueur.
    fn dart(&mut self, p: [i32; 3]) {
        let me = self.player.p; let mut best: Option<([f64; 3], f64)> = None;
        for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { if self.world.get(p[0] + dx, p[1], p[2] + dz) != Some(0) { continue; }
            let d = (me[0] - (p[0] as f64 + 0.5)) * dx as f64 + (me[2] - (p[2] as f64 + 0.5)) * dz as f64; if best.is_none_or(|b| d > b.1) { best = Some(([dx as f64, 0.0, dz as f64], d)); } }
        let Some((dir, _)) = best else { return };
        let from = [p[0] as f64 + 0.5 + dir[0] * 0.6, p[1] as f64 + 0.5, p[2] as f64 + 0.5 + dir[2] * 0.6];
        self.horde.shots.push((from, [dir[0] * 24.0, 0.6, dir[2] * 24.0], 0.0, DART_DMG, "Une fléchette piégée")); self.sound.swing(None, 0.6);
    }
    /// Clic droit sur un levier : il bascule. Rend vrai si c'en était un.
    /// L'orientation d'une statue (0 à 3), si c'en est une.
    pub fn facing_at(&self, p: [i32; 3]) -> Option<u8> { let b = self.world.get(p[0], p[1], p[2])?; let (n, f) = split(name(b)); if !n.starts_with("statue_") { return None; } Some(f.parse().unwrap_or(0)) }
    /// Faire pivoter d'un quart de tour la statue en `p` (ses deux blocs).
    pub fn turn_statue_at(&mut self, p: [i32; 3]) -> bool {
        let Some(f) = self.facing_at(p) else { return false }; let b = split(name(self.world.get(p[0], p[1], p[2]).unwrap())).0;
        let base = if b == "statue_top" { [p[0], p[1] - 1, p[2]] } else { p }; let g = (f + 1) % 4; let face = if g == 0 { String::new() } else { g.to_string() };
        let e = [(base[0], base[1], base[2], with_face("statue_base", &face)), (base[0], base[1] + 1, base[2], with_face("statue_top", &face))]; self.world.edit_many(&e);
        self.sound.place("stone"); self.dirty = true; true
    }
    pub fn lever_at(&mut self, p: [i32; 3]) -> bool {
        let Some(b) = self.world.get(p[0], p[1], p[2]) else { return false }; let (n, f) = split(name(b));
        let to = match n { "lever" => "lever_on", "lever_on" => "lever", _ => return false };
        self.world.edit(p[0], p[1], p[2], with_face(to, f)); self.sound.place("lever"); self.dirty = true; true
    }
    /// Clic droit sur un bloc à pousser : il glisse d'une case, droit devant soi (et tombe s'il n'a plus de sol). Rend vrai si
    /// c'en était un.
    pub fn push_at(&mut self, p: [i32; 3]) -> bool {
        if self.world.get(p[0], p[1], p[2]) != Some(id("push_block")) { return false; }
        let (dx, dz) = { let (ex, ez) = (p[0] as f64 + 0.5 - self.player.p[0], p[2] as f64 + 0.5 - self.player.p[2]); if ex.abs() >= ez.abs() { (ex.signum() as i32, 0) } else { (0, ez.signum() as i32) } };
        let mut q = [p[0] + dx, p[1], p[2] + dz]; if self.world.get(q[0], q[1], q[2]) != Some(0) { self.notes.push("Le bloc ne bouge pas : quelque chose le bloque".into()); return true; }
        for _ in 0..8 { if self.world.get(q[0], q[1] - 1, q[2]) == Some(0) { q[1] -= 1; } else { break; } }
        self.world.edit(p[0], p[1], p[2], 0); self.world.edit(q[0], q[1], q[2], id("push_block")); self.sound.place("stone"); self.dirty = true; true
    }
    /// L'outil de mise au point : les fils à moins de `r` blocs, leur logique, leurs entrées, leur état.
    pub fn wires_report(&self, r: f64) -> Vec<String> {
        let me = self.player.p; let mut o = Vec::new();
        for w in &self.mech.wires { if !w.inputs.iter().chain(w.doors.iter().map(|(p, _)| p)).any(|p| (p[0] as f64 - me[0]).hypot(p[2] as f64 - me[2]) < r) { continue; }
            let logic = if w.seq { format!("séquence {}/{}", w.progress, w.inputs.len()) } else if w.all { "ET".into() } else { "OU".into() };
            let ins: String = w.inputs.iter().map(|p| if self.input_on(*p) { '●' } else { '○' }).collect();
            o.push(format!("{} · {} {}{}{} · {} · {}", if w.name.is_empty() { "fil" } else { &w.name }, logic, ins, if w.not { " · NON" } else { "" }, if w.needs.is_empty() { String::new() } else { format!(" · avec {}", w.needs.join(", ")) },
                if w.on { "enclenché" } else { "au repos" }, if w.hold > 0.0 { format!("{:.0} s", w.timer) } else if w.latch { "verrou".into() } else { String::new() })); }
        if o.is_empty() { o.push("Aucun mécanisme à proximité".into()); } o
    }
    pub(crate) fn mech_save(&self) -> String {
        let pts = |v: &[[i32; 3]]| if v.is_empty() { "-".to_string() } else { v.iter().map(|p| format!("{},{},{}", p[0], p[1], p[2])).collect::<Vec<_>>().join(";") };
        let word = |s: &str| if s.is_empty() { "-".to_string() } else { s.to_string() };
        let mut o: String = self.mech.wires.iter().map(|w| { let doors = if w.doors.is_empty() { "-".to_string() } else { w.doors.iter().map(|(p, b)| format!("{},{},{},{}", p[0], p[1], p[2], name(*b))).collect::<Vec<_>>().join(";") };
            format!("wire {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n", w.all as u8, w.latch as u8, w.on as u8, pts(&w.inputs), doors, pts(&w.darts), pts(&w.spikes),
                word(&w.name), w.seq as u8, w.not as u8, w.hold, if w.needs.is_empty() { "-".into() } else { w.needs.join(";") }, pts(&w.calm), w.fail_darts as u8, w.progress, w.timer, if w.faces.is_empty() { "-".to_string() } else { w.faces.iter().map(|(p, f)| format!("{},{},{},{f}", p[0], p[1], p[2])).collect::<Vec<_>>().join(";") }) }).collect();
        for p in &self.mech.pulses { o += &format!("pulse {} {} {} {} {} {} {}\n", p.period, p.up, p.phase, word(&p.off_by), word(&p.on_by), pts(&p.spikes), pts(&p.darts)); }
        for s in &self.mech.seals { o += &format!("seal {} {} {}\n", pts(&[s.min]), pts(&[s.max]), word(&s.until)); }
        for h in &self.mech.homes { o += &format!("home {} {} {} {}\n", pts(&[h.min]), pts(&[h.max]), word(&h.keep), pts(&h.homes)); }
        for h in &self.mech.hints { o += &format!("hint {} {} {} {} {}\n", pts(&[h.min]), pts(&[h.max]), h.shown as u8, h.xp, h.text); }
        for g in &self.mech.guards { let doors = if g.doors.is_empty() { "-".to_string() } else { g.doors.iter().map(|(p, b)| format!("{},{},{},{}", p[0], p[1], p[2], name(*b))).collect::<Vec<_>>().join(";") };
            o += &format!("guard {} {} {} {},{},{} {} {}\n", pts(&[g.min]), pts(&[g.max]), g.species, g.at[0], g.at[1], g.at[2], g.state, doors); }
        for p in &self.mech.pools { o += &format!("pool {} {} {} {}\n", p.top, p.level, word(&p.by), pts(&p.cols)); }
        if !self.mech.chests.is_empty() { o += &format!("mchest {}\n", pts(&self.mech.chests)); }
        if !self.mech.secrets.is_empty() { o += &format!("msecret {}\n", pts(&self.mech.secrets)); }
        if !self.mech.vaults.is_empty() { o += &format!("mvault {}\n", pts(&self.mech.vaults)); }
        o
    }
    pub(crate) fn mech_load(&mut self, rest: &str) {
        let a: Vec<&str> = rest.split_whitespace().collect(); if a.len() < 7 { return; }
        let doors = if a[4] == "-" { Vec::new() } else { a[4].split(';').filter_map(|c| { let v: Vec<&str> = c.splitn(4, ',').collect(); if v.len() < 4 { return None; } Some(([v[0].parse().ok()?, v[1].parse().ok()?, v[2].parse().ok()?], by_name(v[3])?)) }).collect() };
        let opt = |i: usize| a.get(i).copied().filter(|s| *s != "-").unwrap_or("");
        self.mech.wires.push(Wire { all: a[0] == "1", latch: a[1] == "1", on: a[2] == "1", inputs: pts(a[3]), doors, darts: pts(a[5]), spikes: pts(a[6]),
            name: opt(7).to_string(), seq: opt(8) == "1", not: opt(9) == "1", hold: opt(10).parse().unwrap_or(0.0), needs: if opt(11).is_empty() { Vec::new() } else { opt(11).split(';').map(|s| s.to_string()).collect() },
            calm: pts(a.get(12).copied().unwrap_or("-")), fail_darts: opt(13) == "1", progress: opt(14).parse().unwrap_or(0), timer: opt(15).parse().unwrap_or(0.0), raw: a[2] == "1",
            faces: opt(16).split(';').filter_map(|c| { let v: Vec<&str> = c.split(',').collect(); if v.len() < 4 { return None; } Some(([v[0].parse().ok()?, v[1].parse().ok()?, v[2].parse().ok()?], v[3].parse().ok()?)) }).collect(), ..Default::default() });
    }
    pub(crate) fn pulse_load(&mut self, rest: &str) {
        let a: Vec<&str> = rest.split_whitespace().collect(); if a.len() < 7 { return; } let w = |s: &str| if s == "-" { String::new() } else { s.to_string() };
        self.mech.pulses.push(Pulse { period: a[0].parse().unwrap_or(2.0), up: a[1].parse().unwrap_or(0.5), phase: a[2].parse().unwrap_or(0.0), off_by: w(a[3]), on_by: w(a[4]), spikes: pts(a[5]), darts: pts(a[6]), ..Default::default() });
    }
    pub(crate) fn seal_load(&mut self, rest: &str) {
        let a: Vec<&str> = rest.split_whitespace().collect(); if a.len() < 3 { return; } let (m, n) = (pts(a[0]), pts(a[1])); if m.is_empty() || n.is_empty() { return; }
        self.mech.seals.push(Seal { min: m[0], max: n[0], until: if a[2] == "-" { String::new() } else { a[2].to_string() } });
    }
}
impl Session {
    pub(crate) fn home_load(&mut self, rest: &str) {
        let a: Vec<&str> = rest.split_whitespace().collect(); if a.len() < 4 { return; } let (m, n) = (pts(a[0]), pts(a[1])); if m.is_empty() || n.is_empty() { return; }
        self.mech.homes.push(Home { min: m[0], max: n[0], keep: if a[2] == "-" { String::new() } else { a[2].to_string() }, homes: pts(a[3]) });
    }
    pub(crate) fn hint_load(&mut self, rest: &str) {
        let mut a = rest.splitn(5, ' '); let (Some(m), Some(n), Some(sh), Some(xp), Some(t)) = (a.next(), a.next(), a.next(), a.next(), a.next()) else { return }; let (m, n) = (pts(m), pts(n)); if m.is_empty() || n.is_empty() { return; }
        self.mech.hints.push(Hint { min: m[0], max: n[0], shown: sh == "1", xp: xp.parse().unwrap_or(0.0), text: t.to_string() });
    }
    pub(crate) fn guard_load(&mut self, rest: &str) {
        let a: Vec<&str> = rest.split_whitespace().collect(); if a.len() < 6 { return; } let (m, n) = (pts(a[0]), pts(a[1])); if m.is_empty() || n.is_empty() { return; }
        let at: Vec<f64> = a[3].split(',').filter_map(|v| v.parse().ok()).collect(); if at.len() != 3 { return; }
        let doors = if a[5] == "-" { Vec::new() } else { a[5].split(';').filter_map(|c| { let v: Vec<&str> = c.splitn(4, ',').collect(); if v.len() < 4 { return None; } Some(([v[0].parse().ok()?, v[1].parse().ok()?, v[2].parse().ok()?], by_name(v[3])?)) }).collect() };
        self.mech.guards.push(Guard { min: m[0], max: n[0], species: a[2].to_string(), at: [at[0], at[1], at[2]], state: a[4].parse().unwrap_or(0), doors });
    }
    pub(crate) fn mchest_load(&mut self, rest: &str) { self.mech.chests.extend(pts(rest.trim())); }
    pub(crate) fn msecret_load(&mut self, rest: &str) { self.mech.secrets.extend(pts(rest.trim())); }
    pub(crate) fn pool_load(&mut self, rest: &str) {
        let a: Vec<&str> = rest.split_whitespace().collect(); if a.len() < 4 { return; }
        self.mech.pools.push(Pool { top: a[0].parse().unwrap_or(0), level: a[1].parse().unwrap_or(0), by: if a[2] == "-" { String::new() } else { a[2].to_string() }, cols: pts(a[3]), t: 0.0 });
    }
    pub(crate) fn mvault_load(&mut self, rest: &str) { self.mech.vaults.extend(pts(rest.trim())); }
}
fn pts(s: &str) -> Vec<[i32; 3]> { if s == "-" { return Vec::new(); } s.split(';').filter_map(|c| { let v: Vec<i32> = c.split(',').filter_map(|x| x.parse().ok()).collect(); (v.len() == 3).then(|| [v[0], v[1], v[2]]) }).collect() }
