//! Les brigands qui s'en prennent au village pilote. Certaines nuits, une bande vient piller sa réserve — d'autant plus souvent
//! que le village est riche et mal défendu. La muraille l'arrête ; la herse baissée la nuit ferme la porte ; les gardes montent
//! sur le chemin de ronde et tirent sur elle ; les habitants se calfeutrent. Si la bande atteint l'entrepôt, elle emporte une
//! part des vivres et de la caisse, puis s'enfuit. Le joueur peut prêter main-forte : ce sont des brigands ordinaires, qui se
//! battent avec lui s'ils le voient. Loin du village, l'attaque se règle d'elle-même, selon les défenses.
use crate::blocks::is_opaque;
use crate::economie::{Econ, COIN};
use crate::hostile::{species, Ai};
use crate::player::Voxels;
use crate::session::Session;
use crate::village::Village;

/// Une attaque en cours : la bande (son numéro de groupe dans la horde), d'où elle vient, depuis quand elle est là.
#[derive(Clone, Debug)]
pub struct Raid { pub group: u32, pub from: [f64; 3], pub t: f64, pub size: u32, /** le prochain coup de la bande sur ce qui l'arrête */ pub hit_t: f64 }

/// Ce qui prend des coups : la herse d'une porte, ou un pan de muraille (trois pas de large). Ses points de solidité ; une fois
/// tombé, ce qu'on a abattu (pour le relever à l'identique quand les maçons auront réparé).
#[derive(Clone, Debug)]
pub struct Mark { pub gate: bool, pub at: [i32; 3], pub hp: f64, pub max: f64, pub broken: Vec<(i32, i32, i32, crate::blocks::Id)>, /** quand on l'a frappé pour la dernière fois (heures de jeu) */ pub hit: f64 }
impl Mark { pub fn down(&self) -> bool { self.hp <= 0.0 } }
/// La solidité d'une herse, d'un pan de muraille ; ce que rendent les maçons par heure de jour (et ce qu'il leur faut en pierre).
pub const GATE_HP: f64 = 300.0; pub const WALL_HP: f64 = 600.0; const MEND_PER_HOUR: f64 = 120.0; const MEND_STONE: f64 = 6.0;

/// La chance qu'une bande vienne cette nuit : plus le village a de quoi tenter, plus elle est forte ; ses défenses la réduisent.
pub fn raid_chance(e: &Econ, walls: bool, towers: u32, guards: u32) -> f64 {
    let wealth = e.have(COIN) + e.stock.iter().filter(|(k, _)| crate::economie::FOOD.contains(k)).map(|(_, n)| n).sum::<f64>() * 0.2;
    let lure = (0.12 + wealth / 600.0).min(0.35);
    (lure - if walls { 0.1 } else { 0.0 } - towers as f64 * 0.02 - guards as f64 * 0.03).max(0.04)
}
/// Ce qu'une bande emporte d'une réserve : un tiers des vivres et de la caisse.
pub fn pillage(e: &mut Econ, part: f64) -> Vec<(&'static str, u32)> {
    let keys: Vec<&'static str> = e.stock.keys().copied().filter(|k| *k == COIN || crate::economie::FOOD.contains(k)).collect(); let mut o = Vec::new();
    for k in keys { let n = (e.have(k) * part).floor(); if n >= 1.0 { e.take(k, n); o.push((k, n as u32)); } } o
}
fn said(loot: &[(&'static str, u32)]) -> String {
    if loot.is_empty() { return "rien qui vaille".into(); }
    loot.iter().map(|(k, n)| format!("{} ×{n}", crate::session::item(k).map_or(k.to_string(), |i| i.label.to_lowercase()))).collect::<Vec<_>>().join(", ")
}

/// Une nuit loin du village : une bande vient peut-être (le même tirage qu'en présence du joueur) ; une muraille gardée la tient
/// dehors, sinon elle pille une part de la réserve. Ce que le village en garde : une ligne dans son journal.
pub fn raid_away(e: &mut Econ, c: &crate::economie::Census, day: i64) {
    let guards = if c.walls > 0 { (1 + c.towers / 2).min(3) } else { 0 };
    if crate::noise::hash_float(day as i32, e.id.0, e.id.1, 977) >= raid_chance(e, c.walls > 0, c.towers, guards) { return; }
    let d = (day.max(0)) as u32;
    if c.walls > 0 { e.note(d, "des brigands ont rôdé autour du village cette nuit ; la muraille et les gardes les ont tenus dehors".into()); }
    else { let l = pillage(e, 0.25); e.note(d, format!("des brigands ont pillé la réserve cette nuit : {}", said(&l))); }
}

impl Session {
    /// Le tirage de la nuit, l'arrivée de la bande, le tir des gardes, l'issue de l'attaque.
    pub(crate) fn raid_tick(&mut self, e: &mut Econ, v: &Village, near: bool, dt: f64) {
        let today = (self.hours / 24.0).floor() as i64; let day = (self.hours / 24.0) as u32; let hod = self.hour; let c = e.census(v);
        let guards: Vec<usize> = (0..self.town.folk.len()).filter(|&k| self.town.folk[k].v == v.id && self.town.folk[k].job == 8).collect();
        if e.raid.is_none() && hod >= 22.0 && e.raid_day != today && !self.creative_raids_off() {
            e.raid_day = today; let p = raid_chance(e, c.walls > 0, c.towers, guards.len() as u32);
            if crate::noise::hash_float(today as i32, e.id.0, e.id.1, 977) < p { if near { self.raid_start(e, v); } else {
                // loin du joueur, l'affaire se règle d'elle-même
                if c.walls > 0 && (guards.len() >= 1 || c.towers > 0) { e.note(day, "des brigands ont rôdé autour du village cette nuit ; la muraille et les gardes les ont tenus dehors".into()); }
                else { let l = pillage(e, 0.25); e.note(day, format!("des brigands ont pillé la réserve cette nuit : {}", said(&l))); } } }
        }
        let Some(mut raid) = e.raid.take() else { return };
        raid.t += dt; let alive: Vec<usize> = (0..self.horde.foes.len()).filter(|&i| self.horde.foes[i].group == raid.group && self.horde.foes[i].dead == 0.0).collect();
        if alive.is_empty() {
            // tous tombés (sous les flèches des gardes, ou sous les coups du joueur)
            e.note(day, format!("une bande de {} brigands a été repoussée", raid.size)); self.notes.push(format!("{} : les brigands sont repoussés !", v.name));
            for &k in &guards { self.town.folk[k].work_t = 0.0; } return;
        }
        // au matin, ou après un long siège devant la muraille, la bande renonce et s'en retourne
        let morning = (6.0..20.0).contains(&hod);
        if morning || raid.t > 240.0 {
            for &i in &alive { let f = &mut self.horde.foes[i]; f.raid.clear(); f.home = raid.from; f.kept = false; if f.state != Ai::Combat { f.state = Ai::Retour; } }
            e.note(day, if c.walls > 0 { "des brigands ont tourné autour de la muraille, puis ont renoncé".into() } else { "des brigands ont rôdé autour du village, puis sont repartis".into() });
            self.notes.push(format!("{} : les brigands renoncent et s'éloignent", v.name)); for &k in &guards { self.town.folk[k].work_t = 0.0; } return;
        }
        // les habitants se calfeutrent ; les gardes sont sur la muraille
        for f in self.town.folk.iter_mut().filter(|f| f.v == v.id && f.job != 8) { f.scared = f.scared.max(2.0); }
        // ce qui arrête la bande, elle le frappe : la herse baissée, ou la muraille là où elle bute
        raid.hit_t -= dt; if raid.hit_t <= 0.0 { raid.hit_t = 1.5; self.raid_blows(e, v, &alive, day); }
        // les gardes tirent : une flèche toutes les deux secondes environ sur le pillard le plus proche qu'ils voient
        for &k in &guards {
            let f = &mut self.town.folk[k]; f.work_t -= dt; if f.work_t > 0.0 { continue; }
            let eye = [f.p[0], f.p[1] + 1.5, f.p[2]];
            let target = alive.iter().map(|&i| self.horde.foes[i].p).filter(|q| (q[0] - eye[0]).hypot(q[2] - eye[2]) < 26.0).filter(|q| clear(&self.world, lean(eye, *q), [q[0], q[1] + 1.2, q[2]])).min_by(|a, b| (a[0] - eye[0]).hypot(a[2] - eye[2]).total_cmp(&(b[0] - eye[0]).hypot(b[2] - eye[2])));
            let Some(q) = target else { f.work_t = 0.4; continue };
            // une flèche tendue vers lui, relevée pour sa chute, pas infaillible
            let to = [q[0] + (self.horde.roll() - 0.5) * 0.4, q[1] + 1.1, q[2] + (self.horde.roll() - 0.5) * 0.4]; let d = [to[0] - eye[0], to[1] - eye[1], to[2] - eye[2]];
            let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(0.1); let sp = 30.0; let t = l / sp; let g = crate::outdoors::ARROW_GRAVITY;
            let v0 = [d[0] / t, d[1] / t + 0.5 * g * t, d[2] / t]; let n = (v0[0] * v0[0] + v0[1] * v0[1] + v0[2] * v0[2]).sqrt(); let start = { let p = lean(eye, q); [p[0] + v0[0] / n * 0.3, p[1] + v0[1] / n * 0.3, p[2] + v0[2] / n * 0.3] };
            f.yaw = d[0].atan2(d[2]); f.work_t = 1.8 + self.horde.roll() * 0.6;
            self.arrows.push(crate::outdoors::Arrow { p: start, v: v0, stuck: false, life: 0.0, pow: 1.0, npc: true }); self.sound.swing(Some("sword"), 0.6);
        }
        e.raid = Some(raid);
    }
    /// Une volée de coups : chaque pillard arrêté devant la herse ou au pied de la muraille frappe ; ce qui tombe s'ouvre.
    fn raid_blows(&mut self, e: &mut Econ, v: &Village, alive: &[usize], day: u32) {
        let Some((c, r)) = crate::remparts::ring(e) else { return }; let now = self.hours; let me = self.player.p;
        for &i in alive {
            let f = &self.horde.foes[i]; if f.state == Ai::Combat || f.raid.is_empty() { continue; } let p = f.p; let wp = f.raid[0];
            let d = ((p[0] - c[0] as f64 - 0.5).hypot(p[2] - c[1] as f64 - 0.5), ((wp[0] - c[0] as f64).hypot(wp[2] - c[1] as f64)));
            // devant une porte dont la herse est baissée (et pas encore tombée) ?
            let gate = e.watch.gates.iter().copied().find(|g| (g.0 as f64 + 0.5 - p[0]).hypot(g.2 as f64 + 0.5 - p[2]) < 4.5 && self.portcullis_down(c, *g));
            let (key, is_gate, aim) = if let Some(g) = gate { ([g.0, g.1, g.2], true, [g.0 as f64 + 0.5, p[1], g.2 as f64 + 0.5]) }
                // au pied de la muraille, du dehors, alors qu'il lui faut entrer : le pan le plus proche
                else if d.0 > r && d.0 < r + 4.5 && d.1 < r - 1.0 && !e.watch.gates.iter().any(|g| (g.0 as f64 + 0.5 - p[0]).hypot(g.2 as f64 + 0.5 - p[2]) < 9.0) { let t = (p[2] - c[1] as f64).atan2(p[0] - c[0] as f64); let step = 3.0 / r; let t = (t / step).round() * step;
                    let (x, z) = ((c[0] as f64 + t.cos() * r).floor() as i32, (c[1] as f64 + t.sin() * r).floor() as i32); ([x, p[1] as i32, z], false, [x as f64 + 0.5, p[1], z as f64 + 0.5]) }
                else { continue };
            let k = match e.marks.iter().position(|m| m.gate == is_gate && (m.at[0] - key[0]).abs() <= 1 && (m.at[2] - key[2]).abs() <= 1) { Some(k) => k, None => { let max = if is_gate { GATE_HP } else { WALL_HP }; e.marks.push(Mark { gate: is_gate, at: key, hp: max, max, broken: Vec::new(), hit: now }); e.marks.len() - 1 } };
            if e.marks[k].down() { continue; }
            // le coup : le geste du brigand, le choc, la solidité qui baisse
            let dmg = if self.horde.foes[i].vet { 2.6 } else { 2.0 }; let f = &mut self.horde.foes[i]; f.yaw = (aim[0] - p[0]).atan2(aim[2] - p[2]); f.wind = f.wind.max(0.35);
            let m = &mut e.marks[k]; m.hp -= dmg; m.hit = now; if (aim[0] - me[0]).hypot(aim[2] - me[2]) < 48.0 { self.sound.thump(aim); }
            if m.hp > 0.0 { continue; }
            m.hp = 0.0;
            if is_gate { let g = (key[0], key[1], key[2]); self.portcullis(c, g, false); e.note(day, "la herse a cédé sous les coups des brigands".into()); self.notes.push(format!("{} : la herse cède !", v.name)); }
            else {
                // la brèche : trois pas de muraille abattus jusqu'au sol, gardés pour être relevés
                let mut gone = Vec::new(); let (cx, cz) = (key[0] as f64 + 0.5, key[2] as f64 + 0.5);
                for (x, z, _) in crate::remparts::cells(c, r) { if (x as f64 + 0.5 - cx).hypot(z as f64 + 0.5 - cz) > 1.8 { continue; }
                    let g = self.gen.column(x as f64, z as f64).h.floor() as i32; for y in g - 2..=g + 12 { if let Some(b) = self.world.get(x, y, z) { if b != 0 && !crate::blocks::is_water(b) && y > g - 2 && ["stone_bricks", "cobblestone", "planks", "oak_log", "wall_sconce"].iter().any(|n| crate::blocks::BLOCKS[b as usize].name.starts_with(n)) { gone.push((x, y, z, b)); } } } }
                let ground_y = |s: &Self, x: i32, z: i32| s.gen.column(x as f64, z as f64).h.floor() as i32;
                let open: Vec<(i32, i32, i32, crate::blocks::Id)> = gone.iter().filter(|q| q.1 > ground_y(self, q.0, q.2)).map(|q| (q.0, q.1, q.2, 0)).collect(); self.world.edit_many(&open); self.dirty = true;
                e.marks[k].broken = gone.into_iter().filter(|q| q.1 > ground_y(self, q.0, q.2)).collect();
                e.note(day, "les brigands ont ouvert une brèche dans la muraille".into()); self.notes.push(format!("{} : la muraille est percée !", v.name));
                // la bande passe par la brèche
                let inside = [c[0] as f64 + 0.5 + (cx - c[0] as f64 - 0.5) * (r - 5.0) / r, p[1], c[1] as f64 + 0.5 + (cz - c[1] as f64 - 0.5) * (r - 5.0) / r];
                let store = self.horde.foes[i].raid.last().copied().unwrap_or(inside);
                for &j in alive { let f = &mut self.horde.foes[j]; if !f.raid.is_empty() { f.raid = vec![[cx, p[1], cz], inside, store]; } }
            }
        }
    }
    /// Les maçons du village : de jour, hors des attaques, ils rendent sa solidité à ce qui a pris des coups, pierre de la réserve
    /// à l'appui ; ce qui était tombé se relève une fois tout réparé.
    pub(crate) fn raid_mend(&mut self, e: &mut Econ, day: u32, dt_hours: f64) {
        if e.raid.is_some() || !(7.0..19.0).contains(&self.hour) || e.marks.is_empty() { return; }
        let Some((c, _)) = crate::remparts::ring(e) else { e.marks.clear(); return };
        for k in 0..e.marks.len() {
            let want = (MEND_PER_HOUR * dt_hours).min(e.marks[k].max - e.marks[k].hp); if want <= 0.0 { continue; }
            let stone = e.take("cobblestone", want / MEND_PER_HOUR * MEND_STONE); if stone <= 0.0 { continue; }
            let was = e.marks[k].down(); let m = &mut e.marks[k]; m.hp = (m.hp + stone / MEND_STONE * MEND_PER_HOUR).min(m.max);
            if m.hp >= m.max { let m = e.marks[k].clone(); if was || !m.broken.is_empty() { if !m.broken.is_empty() { self.world.edit_many(&m.broken); self.dirty = true; }
                e.note(day, if m.gate { "les forgerons ont remis la herse en état".into() } else { "les maçons ont refermé la brèche de la muraille".into() }); }
                if m.gate { e.watch.night = None; let _ = c; } }
        }
        e.marks.retain(|m| m.hp < m.max);
    }
    /// La barre de solidité à montrer : ce que la bande frappe en ce moment (ou vient de frapper), près du joueur.
    pub fn siege_bar(&self) -> Option<(String, f64)> {
        let e = self.econ.as_ref()?; let me = self.player.p; let name = self.pilot_village().map(|v| v.name.clone()).unwrap_or_default();
        e.marks.iter().filter(|m| self.hours - m.hit < 0.25 || m.down() && e.raid.is_some()).filter(|m| (m.at[0] as f64 - me[0]).hypot(m.at[2] as f64 - me[2]) < 90.0)
            .min_by(|a, b| (a.hp / a.max).total_cmp(&(b.hp / b.max))).map(|m| (format!("{} de {name}", if m.gate { "Herse" } else { "Muraille" }), m.hp / m.max))
    }
    /// En créatif, pas d'attaque d'elle-même (on la déclenche avec /brigands).
    fn creative_raids_off(&self) -> bool { self.creative }
    /// Une bande arrive : par la porte s'il y en a une (elle vise le passage), sinon droit sur la réserve, du côté le plus vide.
    pub fn raid_start(&mut self, e: &mut Econ, v: &Village) -> bool {
        let Some(sp) = species("brigand") else { return false };
        let store = e.store.first().map(|p| [p[0] as f64 + 0.5, p[1] as f64, p[2] as f64 + 0.5]).unwrap_or([v.c[0] as f64 + 0.5, v.c[1] as f64, v.c[2] as f64 + 0.5]);
        let ring = crate::remparts::ring(e); let reach = ring.map_or(v.r as f64 + 8.0, |r| r.1);
        let gate = (!e.watch.gates.is_empty()).then(|| e.watch.gates[(self.horde.roll() * e.watch.gates.len() as f64) as usize % e.watch.gates.len()]);
        // d'où elle vient, et par où elle passe
        let (dir, path): ([f64; 2], Vec<[f64; 3]>) = match (gate, ring) {
            (Some(g), Some((c, _))) => { let (dx, dz) = (g.0 as f64 + 0.5 - c[0] as f64, g.2 as f64 + 0.5 - c[1] as f64); let l = dx.hypot(dz).max(1.0); let o = [dx / l, dz / l]; let gp = [g.0 as f64 + 0.5, g.1 as f64 + 1.0, g.2 as f64 + 0.5];
                (o, vec![[gp[0] + o[0] * 7.0, gp[1], gp[2] + o[1] * 7.0], [gp[0] - o[0] * 5.0, gp[1], gp[2] - o[1] * 5.0], store]) }
            _ => { let a = self.horde.roll() * std::f64::consts::TAU; ([a.cos(), a.sin()], vec![store]) }
        };
        // le point de départ : à une vingtaine de pas au-dehors (devant la porte, ou du côté tiré au sort), sur un sol chargé
        let mut at = None;
        for k in 0..36 { let d = 18.0 - (k / 3) as f64; let dir = { let a = dir[1].atan2(dir[0]) + (k % 3) as f64 * 0.35 - 0.35; [a.cos(), a.sin()] };
            let (x, z) = if gate.is_some() { (path[0][0] + dir[0] * d, path[0][2] + dir[1] * d) } else { (v.c[0] as f64 + dir[0] * (reach + d), v.c[2] as f64 + dir[1] * (reach + d)) };
            if let Some(y) = crate::fauna::ground(&self.world, x, z, self.gen.column(x, z).h + 4.0) { at = Some([x, y, z]); break; } }
        let Some(from) = at else { return false };
        let size = 3 + (v.houses.len() as u32 / 4).min(3); let mut group = 0;
        for k in 0..size { let o = [from[0] + (k as f64 % 3.0 - 1.0) * 1.6, from[1], from[2] + (k as f64 / 3.0).floor() * 1.6];
            let y = crate::fauna::ground(&self.world, o[0], o[2], from[1] + 3.0).unwrap_or(from[1]);
            let i = self.horde.put(sp, [o[0], y, o[2]], (store[0] - o[0]).atan2(store[2] - o[2]), k == 0); if k == 0 { group = self.horde.foes[i].group; } let f = &mut self.horde.foes[i]; f.group = group; f.raid = path.clone(); f.state = Ai::Patrouille; f.kept = true; }
        let day = (self.hours / 24.0) as u32; e.note(day, format!("une bande de {size} brigands est venue attaquer le village"));
        self.notes.push(format!("Des brigands marchent sur {} !", v.name)); self.sound.fox_scream(from);
        e.raid = Some(Raid { group, from, t: 0.0, size, hit_t: 1.5 }); true
    }
    /// Un pillard a atteint la réserve : la bande emporte sa part, et s'enfuit.
    pub(crate) fn raid_loot(&mut self, group: u32) {
        let Some(v) = self.pilot_village() else { return }; let Some(mut e) = self.econ.take() else { return };
        if let Some(raid) = e.raid.take().filter(|r| r.group == group) {
            let l = pillage(&mut e, 0.33); let day = (self.hours / 24.0) as u32; e.note(day, format!("des brigands ont pillé la réserve : {}", said(&l)));
            self.notes.push(format!("Les brigands pillent la réserve de {} et s'enfuient !", v.name)); self.dirty = true;
            for f in self.horde.foes.iter_mut().filter(|f| f.group == group) { f.raid.clear(); f.home = raid.from; f.kept = false; if f.state != Ai::Combat { f.state = Ai::Retour; } }
        }
        self.econ = Some(e);
    }
    /// Pour les essais et la commande /brigands : une bande tout de suite.
    pub fn raid_now(&mut self) -> bool { let Some(v) = self.pilot_village() else { return false }; let Some(mut e) = self.econ.take() else { return false }; let ok = self.raid_start(&mut e, &v); self.econ = Some(e); ok }
}
/// Penché par-dessus le parapet : le garde tire d'entre deux merlons, à deux pas devant lui vers sa cible.
fn lean(eye: [f64; 3], q: [f64; 3]) -> [f64; 3] { let (dx, dz) = (q[0] - eye[0], q[2] - eye[2]); let l = dx.hypot(dz).max(0.1); [eye[0] + dx / l * 2.0, eye[1] + 0.3, eye[2] + dz / l * 2.0] }
/// La vue entre deux points n'est pas coupée par un bloc plein.
fn clear(w: &impl Voxels, a: [f64; 3], b: [f64; 3]) -> bool {
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]]; let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt(); let n = (l / 0.45).ceil() as i32;
    (1..n).all(|i| { let t = i as f64 / n as f64; w.get((a[0] + d[0] * t).floor() as i32, (a[1] + d[1] * t).floor() as i32, (a[2] + d[2] * t).floor() as i32).is_some_and(|b| !is_opaque(b)) })
}
