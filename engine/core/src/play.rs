//! Le jeu de survie : inventaire et équipement, craft, récolte et outils, chasse, interactions du clic droit,
//! besoins du corps (faim, soif, endurance, souffle, chaleur, fatigue), sommeil, coffres, feux, cultures,
//! mort et dépouille, sauvegarde. Tout ce qui est contenu (objets, recettes) vient de data.rs.
use crate::blocks::*;
use crate::data::{def, Station, SLOTS};
use crate::felling::{find_tree, Fall};
use crate::noise::hash_float;
use crate::player::{Input, Player, Voxels};
use crate::session::*;
use crate::survival::*;
use std::collections::HashMap;

impl Session {
    // ------------------------------------------------------------------ inventaire
    /// Range des objets dans le sac ; renvoie ce qui n'a pas trouvé de place.
    pub fn add_item(&mut self, id: &str, n: u32) -> u32 {
        let Some(it) = item(id) else { return n }; let mut left = n; let max = max_stack(it.id); self.seen.insert(it.id);
        if max <= 1 { for s in self.inv.iter_mut() { if left == 0 { break; } if s.is_none() { *s = Some(Stack::new(it.id, None)); left -= 1; } } return left; }
        for s in self.inv.iter_mut().flatten() { if left > 0 && s.item == it.id { if let Some(c) = s.n { if c < max { let m = left.min(max - c); s.n = Some(c + m); left -= m; } } } }
        for s in self.inv.iter_mut() { if left == 0 { break; } if s.is_none() { let m = left.min(max); *s = Some(Stack::new(it.id, Some(m))); left -= m; } }
        left
    }
    /// Une pile entière (avec son usure) dans le sac ; ce qui reste.
    fn add_stack(&mut self, st: Stack) -> Option<Stack> {
        if st.n.is_none() { if let Some(s) = self.inv.iter_mut().find(|s| s.is_none()) { *s = Some(st); self.seen.insert(st.item); return None; } return Some(st); }
        let left = self.add_item(st.item, st.n.unwrap_or(1)); if left > 0 { Some(Stack { n: Some(left), ..st }) } else { None }
    }
    /// Case de l'écran : sac, coffre ouvert ou équipement.
    pub fn slot(&self, i: usize) -> Option<Stack> {
        if i < INV_SIZE { self.inv[i] } else if i < EQUIP0 { self.chest.and_then(|c| self.chests.get(&c)).and_then(|v| v.get(i - CHEST0).copied().flatten()) } else { self.equip.get(i - EQUIP0).copied().flatten() }
    }
    fn set_slot(&mut self, i: usize, v: Option<Stack>) {
        if i < INV_SIZE { self.inv[i] = v; } else if i < EQUIP0 { if let Some(c) = self.chest { if let Some(s) = self.chests.get_mut(&c).and_then(|x| x.get_mut(i - CHEST0)) { *s = v; } } } else if let Some(s) = self.equip.get_mut(i - EQUIP0) { *s = v; }
        if let Some(s) = v { self.seen.insert(s.item); } self.dirty = true;
    }
    /// Une case d'équipement n'accepte que ce qui se porte à cet endroit.
    pub fn accepts(&self, i: usize, st: Stack) -> bool { if i < EQUIP0 { return i < INV_SIZE || self.chest.is_some(); } def(st.item).and_then(|d| d.wear).is_some_and(|w| w.slot == SLOTS[i - EQUIP0].0) }
    /// Inventaire : prend ce qu'il y a dans une case. En créatif, le catalogue donne un tas plein.
    pub fn grab_slot(&mut self, i: usize) { if self.cursor.is_none() { if let Some(s) = self.slot(i) { self.set_slot(i, None); self.cursor = Some(s); self.from = Some(i); } } }
    pub fn grab_catalog(&mut self, id: &'static str) { if let Some(it) = item(id) { self.cursor = Some(Stack::new(it.id, if stackable(it.id) { Some(max_stack(it.id)) } else { None })); self.from = None; } }
    /// Dépose : réunit deux tas du même objet, sinon échange les cases ; hors de toute case, l'objet retourne d'où il vient.
    pub fn drop_slot(&mut self, i: Option<usize>) {
        let Some(c) = self.cursor.take() else { return };
        let back = |s: &mut Self, c: Stack| { match s.from.take() { Some(f) if s.slot(f).is_none() && s.accepts(f, c) => s.set_slot(f, Some(c)), _ => { if let Some(r) = s.add_stack(c) { s.cursor = Some(r); } } } };
        let Some(i) = i else { back(self, c); return };
        if !self.accepts(i, c) { back(self, c); self.dirty = true; return; }
        let max = max_stack(c.item);
        match self.slot(i) {
            Some(o) if o.item == c.item && o.n.is_some() && c.n.is_some() => { let t = o.n.unwrap() + c.n.unwrap(); self.set_slot(i, Some(Stack { n: Some(t.min(max)), ..o })); if t > max { back(self, Stack { n: Some(t - max), ..o }); } }
            Some(o) => { self.set_slot(i, Some(c)); match self.from { Some(f) if self.slot(f).is_none() && self.accepts(f, o) => self.set_slot(f, Some(o)), _ => { self.cursor = Some(o); self.dirty = true; return; } } }
            None => self.set_slot(i, Some(c)),
        }
        self.from = None; self.dirty = true;
    }
    /// Clic droit : prend la moitié du tas, le reste reste dans la case.
    pub fn take_half(&mut self, i: usize) {
        let (None, Some(s)) = (self.cursor, self.slot(i)) else { return }; let Some(n) = s.n else { return self.grab_slot(i) };
        let k = n.div_ceil(2); self.cursor = Some(Stack { n: Some(k), ..s }); self.from = Some(i); self.set_slot(i, if n == k { None } else { Some(Stack { n: Some(n - k), ..s }) });
    }
    /// Clic droit, objet en main : dépose une seule unité.
    pub fn put_one(&mut self, i: usize) {
        let Some(c) = self.cursor else { return }; let Some(n) = c.n else { return }; if !self.accepts(i, c) { return; }
        match self.slot(i) { None => self.set_slot(i, Some(Stack { n: Some(1), ..c })), Some(o) if o.item == c.item && o.n.is_some_and(|k| k < max_stack(c.item)) => self.set_slot(i, Some(Stack { n: o.n.map(|k| k + 1), ..o })), _ => return }
        self.cursor = if n > 1 { Some(Stack { n: Some(n - 1), ..c }) } else { self.from = None; None }; self.dirty = true;
    }
    /// Maj+clic dans le catalogue : l'objet arrive directement dans la barre et il est sélectionné.
    pub fn quick_catalog(&mut self, id: &'static str) {
        let Some(it) = item(id) else { return }; let stack = Stack::new(it.id, if stackable(it.id) { Some(max_stack(it.id)) } else { None });
        let i = self.inv[..9].iter().position(|s| s.is_some_and(|v| v.item == it.id)).or(self.inv[..9].iter().position(|s| s.is_none())).unwrap_or(self.sel);
        self.inv[i] = Some(stack); self.sel = i; self.seen.insert(it.id); self.dirty = true;
    }
    /// Maj+clic sur une case : du coffre vers le sac et inversement ; un vêtement va se porter ; sinon de la barre au sac.
    pub fn quick_slot(&mut self, i: usize) {
        let Some(s) = self.slot(i) else { return };
        let mut range: Vec<usize> = if i >= EQUIP0 { (9..INV_SIZE).chain(0..9).collect() } else if i >= CHEST0 { (0..INV_SIZE).collect() } else if self.chest.is_some() { (CHEST0..EQUIP0).collect() } else if i < 9 { (9..INV_SIZE).collect() } else { (0..9).collect() };
        if i < CHEST0 && self.chest.is_none() { if let Some(w) = def(s.item).and_then(|d| d.wear) { let e = EQUIP0 + SLOTS.iter().position(|x| x.0 == w.slot).unwrap_or(0); if self.slot(e).is_none() { range = vec![e]; } } }
        let mut s = s; let max = max_stack(s.item);
        for k in range.clone() { if let (Some(o), Some(n)) = (self.slot(k), s.n) { if o.item == s.item { if let Some(c) = o.n { let m = n.min(max.saturating_sub(c)); self.set_slot(k, Some(Stack { n: Some(c + m), ..o })); s.n = Some(n - m); } } } }
        if s.n == Some(0) { self.set_slot(i, None); return; }
        if let Some(k) = range.into_iter().find(|k| self.slot(*k).is_none() && self.accepts(*k, s)) { self.set_slot(k, Some(s)); self.set_slot(i, None); } else { self.set_slot(i, Some(s)); }
    }
    /// Combien d'un ingrédient (ou d'un groupe : bois, pierre…) dans le sac.
    pub fn count(&self, need: &str) -> u32 { let m = members(need); self.inv.iter().flatten().filter(|s| m.contains(&s.item)).map(|s| s.n.unwrap_or(1)).sum() }
    pub(crate) fn take(&mut self, need: &str, mut k: u32) {
        let m = members(need);
        for s in self.inv.iter_mut() { if k == 0 { break; } let Some(st) = *s else { continue }; if !m.contains(&st.item) { continue; }
            let n = st.n.unwrap_or(1); let used = n.min(k); k -= used; *s = if n == used { None } else { Some(Stack { n: Some(n - used), ..st }) }; }
    }
    /// Masse portée (sac et équipement) et charge supportable.
    pub fn carried(&self) -> f64 { self.inv.iter().chain(self.equip.iter()).flatten().map(|s| def(s.item).map_or(1.0, |d| d.mass) * s.n.unwrap_or(1) as f64).sum::<f64>() + self.cursor.map_or(0.0, |s| def(s.item).map_or(1.0, |d| d.mass) * s.n.unwrap_or(1) as f64) }
    pub fn capacity(&self) -> f64 { CARRY + self.worn().4 }
    /// Ce que l'équipement apporte : chaleur, fraîcheur, armure, (rien), charge en plus.
    pub fn worn(&self) -> (f64, f64, f64, f64, f64) { let mut w = (0.0, 0.0, 0.0, 0.0, 0.0); for s in self.equip.iter().flatten() { if let Some(x) = def(s.item).and_then(|d| d.wear) { w.0 += x.warm; w.1 += x.cool; w.2 += x.armor; w.4 += x.bag; } } w }

    // ------------------------------------------------------------------ fabrication
    /// Les ateliers à portée de main (quatre blocs), dans l'ordre de Station::ALL ; un feu éteint ne cuit rien.
    pub fn stations(&self) -> [bool; 6] {
        let p = self.player.p; let (bx, by, bz) = (p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32); let mut o = [true, false, false, false, false, false];
        for y in by - 2..=by + 3 { for z in bz - 4..=bz + 4 { for x in bx - 4..=bx + 4 { match self.world.get(x, y, z) { Some(B::WORKBENCH) => o[1] = true, Some(B::CAMPFIRE) => o[2] = true, Some(B::FURNACE) => o[3] = true, Some(B::ANVIL) => o[4] = true, Some(B::MASON_BENCH) => o[5] = true, _ => {} } } } }
        o
    }
    pub fn station_ok(st: &[bool; 6], at: Station) -> bool { st[Station::ALL.iter().position(|s| *s == at).unwrap_or(0)] }
    /// Une recette est connue dès qu'on a eu en main l'un de ses ingrédients ou ce qu'elle produit.
    pub fn known(&self, r: &Recipe) -> bool { self.creative || self.seen.contains(r.out) || r.need.iter().any(|n| members(n.id).iter().any(|m| self.seen.contains(m))) }
    pub fn can_craft(&self, r: &Recipe, st: &[bool; 6]) -> bool { r.artisan.is_none_or(|a| self.creative || self.taught.contains(a)) && Self::station_ok(st, r.at) && r.need.iter().all(|n| self.count(n.id) >= n.n) }
    /// Fabrique la recette `i` une fois. L'erreur dit ce qui manque.
    pub fn craft(&mut self, i: usize) -> Result<(), &'static str> {
        let r = recipes().get(i).ok_or("Recette inconnue")?.clone(); let st = self.stations();
        if !Self::station_ok(&st, r.at) { return Err(match r.at { Station::Bench => "Il faut un établi à portée", Station::Fire => "Il faut un feu de camp allumé à portée", Station::Furnace => "Il faut un four à portée", Station::Anvil => "Il faut une enclume à portée", Station::Mason => "Il faut un banc de taille à portée", Station::Hand => "" }); }
        // Au-delà du fer : un artisan de village doit avoir enseigné la technique.
        if let Some(a) = r.artisan { if !self.creative && !self.taught.contains(a) { return Err("Demande un artisan de village"); } }
        if !r.need.iter().all(|n| self.count(n.id) >= n.n) { return Err("Il manque des ingrédients"); }
        let before = self.inv; for n in r.need.iter().filter(|n| !n.keep) { self.take(n.id, n.n); }
        if self.add_item(r.out, r.n) > 0 { self.inv = before; return Err("L'inventaire est plein"); }
        // Les outils qu'on garde (marteau, couteau, aiguille…) s'usent à chaque ouvrage.
        for n in r.need.iter().filter(|n| n.keep) { let m = members(n.id); if let Some(i) = self.inv.iter().position(|s| s.is_some_and(|s| m.contains(&s.item))) { self.wear_slot(i, 1); } }
        let sk = if r.at == Station::Fire { 4 } else { 3 }; self.skills[sk] += 1.0;
        self.sound.place(item(r.out).and_then(|i| i.block).map_or("planks", |b| BLOCKS[b as usize].name)); self.dirty = true; Ok(())
    }

    // ------------------------------------------------------------------ outils et récolte
    /// L'objet tenu s'il ne se pose pas : c'est lui qui donne sa voix au coup.
    pub(crate) fn tool_id(&self) -> Option<&'static str> { self.held().filter(|i| i.block.is_none()).map(|i| i.id) }
    fn tool(&self) -> Option<crate::data::Tool> { self.tool_id().and_then(def).and_then(|d| d.tool) }
    /// L'outil tenu s'use ; à bout, il se brise.
    fn wear_tool(&mut self, n: u32) { self.wear_slot(self.sel, n); }
    /// Combien d'usages tient un objet : un outil selon sa matière, un arc, une aiguille ou une gourde selon « usages ».
    fn max_uses(&self, item: &str) -> Option<u32> {
        let d = def(item)?; if let Some(t) = d.tool { return Some((t.mat.dur as f64 / PROFILES[self.profile].wear).round().max(1.0) as u32); }
        (d.uses > 0).then(|| if d.drink.is_some() { d.uses } else { (d.uses as f64 / PROFILES[self.profile].wear).round().max(1.0) as u32 })
    }
    /// L'objet de la case `i` s'use de `n` ; à bout, il se brise (une gourde, elle, se vide).
    pub(crate) fn wear_slot(&mut self, i: usize, n: u32) {
        if self.creative { return; } let Some(st) = self.inv[i] else { return }; let Some(max) = self.max_uses(st.item) else { return }; let w = st.wear + n;
        if w >= max {
            if def(st.item).is_some_and(|d| d.drink.is_some()) { self.inv[i] = Some(Stack::new("waterskin", None)); self.notes.push("La gourde est vide".into()); }
            else { self.inv[i] = None; self.notes.push(format!("{} s'est brisé", item(st.item).map_or(st.item, |i| i.label))); self.sound.break_block("stone", Some("pickaxe")); }
        } else { self.inv[i] = Some(Stack { wear: w, ..st }); }
        self.dirty = true;
    }
    /// Usure d'un outil ou d'un bouclier, de 0 (neuf) à 1 (brisé), pour l'affichage.
    pub fn wear_of(&self, st: &Stack) -> Option<f64> {
        let d = def(st.item)?; if d.tool.is_some() || d.uses > 0 { return self.max_uses(st.item).map(|m| st.wear as f64 / m as f64); }
        d.wear.filter(|w| w.slot == crate::data::Slot::Hand).map(|w| st.wear as f64 / shield_life(w.armor, self.profile) as f64)
    }
    /// Un coup encaissé use le bouclier ; à bout, il se fend.
    fn wear_equip(&mut self, i: usize, n: u32) {
        if self.creative { return; } let Some(st) = self.equip[i] else { return }; let Some(w) = def(st.item).and_then(|d| d.wear) else { return };
        let left = st.wear + n; if left >= shield_life(w.armor, self.profile) { self.equip[i] = None; self.notes.push(format!("{} s'est fendu", item(st.item).map_or(st.item, |x| x.label))); self.sound.break_block("wood", Some("axe")); } else { self.equip[i] = Some(Stack { wear: left, ..st }); }
        self.dirty = true;
    }
    pub(crate) fn mine_step(&mut self, dt: f64, go: bool) {
        let tool = self.tool(); let v = voice(self.tool_id()); self.swing_t -= dt;
        // Dans le vide, on frappe quand même, avec n'importe quoi ; chaque coup coûte un peu d'endurance.
        if go && self.target.is_none() && self.swing_t <= 0.0 { self.swing_t = 0.3; self.sound.swing(v, 1.0); if !self.creative { self.spend(STAMINA_SWING); } }
        let Some(t) = self.target.filter(|_| go) else { self.mine_key = None; self.mine_t = 0.0; return };
        let key = (t.x, t.y, t.z);
        if self.mine_key != Some(key) { self.mine_key = Some(key); self.mine_t = 0.0; self.felling = if tool.is_some_and(|t| t.kind == "hache") { find_tree(&self.world, t.x, t.y, t.z) } else { None };
            // Un arbre immense exige du métal : la hache de pierre ou de silex le travaille bloc à bloc.
            if let (Some(tr), Some(tl)) = (&self.felling, tool) { if tr.logs.len() > 60 && tl.mat.tier < 2 { self.felling = None; self.notes.push("Trop gros pour une hache de pierre : il faut du métal, ou le couper bloc à bloc".into()); } } }
        let tired = if self.winded && !self.creative { 0.5 } else { 1.0 }; // essoufflé, les coups portent moitié moins
        let skill = |s: &Self, k: usize| skill_bonus(s.skills[k]) * PROFILES[s.profile].harvest;
        // La hache sur un arbre : on entaille jusqu'à ce que tout l'arbre tombe ; plus il est grand et plus la hache est faible, plus c'est long.
        if let (Some(tr), Some(tl)) = (&self.felling, tool) {
            self.mine_need = (3.0 + tr.logs.len() as f64 * 0.12) * 3.2 / tl.mat.speed;
            self.mine_t += dt * tired * skill(self, 0); self.dig_t -= dt; let mut struck = false; if self.dig_t <= 0.0 { self.dig_t = 0.3; self.sound.dig(BLOCKS[t.id as usize].name, v); struck = true; }
            if struck && !self.creative { self.spend(STAMINA_SWING); }
            if self.mine_t >= self.mine_need { let n = self.felling.as_ref().map_or(0, |f| f.logs.len()); self.fell(t); self.mine_key = None; self.mine_t = 0.0; self.wear_tool(1 + n as u32 / 8); self.skills[0] += 3.0; }
            return;
        }
        let b = BLOCKS[t.id as usize]; let fit = tool.filter(|tl| tl.works(b.cat));
        let rate = fit.map_or(bare_rate(b.cat), |k| k.mat.speed) * tired * skill(self, if b.cat == "wood" { 0 } else { 1 }); self.mine_need = b.hardness * HARD;
        if self.creative { self.cool -= dt; if self.cool > 0.0 { return; } self.cool = 0.2; self.mine_t = 1e9; }
        self.mine_t += dt * rate; self.dig_t -= dt;
        if self.dig_t <= 0.0 && !self.creative { self.dig_t = 0.3; self.sound.dig(b.name, v); if self.mine_need > 0.25 { self.spend(STAMINA_SWING); } }
        if self.mine_t >= self.mine_need {
            if self.world.edit(t.x, t.y, t.z, 0) { self.flow.touch(t.x, t.y, t.z); self.sound.break_block(b.name, v); self.drop_attached(t.x, t.y, t.z); self.on_broken([t.x, t.y, t.z], t.id); if !self.creative { self.drops(b, t, fit); } self.dirty = true;
                if tool.is_some() && self.mine_need > 0.25 { self.wear_tool(1); } if b.cat == "rock" || b.cat == "wood" { self.skills[if b.cat == "wood" { 0 } else { 1 }] += 1.0; } }
            self.mine_key = None; self.mine_t = 0.0;
        }
    }
    /// Ce que rend un bloc cassé : son objet (la roche, seulement à la bonne pioche), les baies du buisson, parfois fibre, graines ou silex.
    fn drops(&mut self, b: Block, t: Target, fit: Option<crate::data::Tool>) {
        if needs_tool(b.cat) && fit.is_none() { self.notes.push("Sans pioche, la pierre s'effrite sans rien donner".into()); return; }
        if b.name.ends_with("_ore") { if let Some(tl) = fit { if tl.mat.tier < ore_tier(b.name) { self.notes.push(format!("Cette pioche est trop tendre pour ce minerai : il faut au moins du cuivre")); return; } } }
        // Le gravier rend son silex à la place de lui-même, pas en plus.
        if b.name == "gravel" { let flint = hash_float(t.x * 7, t.y, t.z * 13 + (self.elapsed * 61.0) as i32, 77) < 0.2; self.add_item(if flint { "flint" } else { "gravel" }, 1); return; }
        if let Some(d) = b.drop { if d != "air" { self.add_item(d, 1); } } if let Some((d, n)) = bonus_drop(b.name) { self.add_item(d, n); }
        let kind = fit.map(|t| t.kind).or(self.tool().map(|t| t.kind));
        for (k, (d, p)) in grass_drops(b.name).iter().enumerate() {
            let p = if *d == "fiber" && matches!(kind, Some("couteau") | Some("faucille")) { 1.0 } else if *d == "seeds" && kind == Some("faucille") { 0.6 } else { *p }; // au couteau, l'herbe donne toujours sa fibre
            if hash_float(t.x * 7 + k as i32, t.y, t.z * 13 + (self.elapsed * 61.0) as i32, 77) < p { self.add_item(d, 1); }
        }
        if b.name == "wheat_3" && kind == Some("faucille") { self.add_item("wheat", 1); }
    }
    /// Le tronc au-dessus de l'entaille tombe, le feuillage disparaît, le pied de l'arbre reste en place ; des branches tombent avec.
    pub(crate) fn fell(&mut self, t: Target) {
        let Some(tree) = self.felling.take() else { return }; let at = |s: &Self, p: (i32, i32, i32)| s.world.get(p.0, p.1, p.2).unwrap_or(0);
        let logs: Vec<((i32, i32, i32), crate::blocks::Id)> = tree.logs.iter().filter(|p| p.1 >= t.y).map(|p| (*p, log_of(at(self, *p)))).collect();
        let leaf_id = tree.leaves.first().map_or(0, |p| at(self, *p)); let drop = BLOCKS[t.id as usize].drop.unwrap_or("oak_log");
        let edits: Vec<_> = logs.iter().map(|(p, _)| (p.0, p.1, p.2, 0)).chain(tree.leaves.iter().map(|p| (p.0, p.1, p.2, 0))).collect(); self.world.edit_many(&edits);
        self.falling = Some(Fall::new(&logs, &tree.leaves, leaf_id, (t.x, t.y, t.z), self.player.p, (t.x * 31 + t.z) as u32)); self.fall_serial += 1;
        for p in logs.iter().map(|l| l.0).chain(tree.leaves.iter().copied()) { self.drop_attached(p.0, p.1, p.2); self.flow.touch(p.0, p.1, p.2); }
        if !self.creative { self.add_item(drop, logs.len().min(99) as u32); self.add_item("stick", (tree.leaves.len() / 40).clamp(1, 8) as u32); }
        self.sound.tree_crack(); self.dirty = true;
    }

    // ------------------------------------------------------------------ chasse
    /// L'animal sous le réticule, s'il est plus près que le bloc visé et à portée de l'arme.
    pub(crate) fn aim_prey(&self, o: [f64; 3], d: [f64; 3], block_t: f64) -> Option<crate::session::Prey> {
        use crate::session::Prey;
        let head = self.player.eye(); let reach = self.tool().map_or(REACH, |t| t.reach()); let mut best: Option<(f64, Prey)> = None;
        let mut consider = |t: Option<f64>, p: Prey| { if let Some(t) = t { let q = [o[0] + d[0] * t, o[1] + d[1] * t, o[2] + d[2] * t]; let r = ((q[0] - head[0]).powi(2) + (q[1] - head[1]).powi(2) + (q[2] - head[2]).powi(2)).sqrt();
            if t < block_t && r <= reach && best.is_none_or(|b| t < b.0) { best = Some((t, p)); } } };
        for (k, dr) in self.fauna.deer.iter().enumerate() { consider(ray_box(o, d, dr.p, 0.55, 1.6), Prey::Deer(k)); }
        for (k, c) in self.wild.all.iter().enumerate() { let Some((hw, h)) = crate::outdoors::wild_box(c.kind) else { continue }; consider(ray_box(o, d, c.p, hw, h), Prey::Wild(k)); }
        best.map(|b| b.1)
    }
    /// Un coup porté à l'animal visé, selon l'arme ; épuisé, on frappe moitié moins fort. Au couteau, le dépeçage rend davantage.
    /// Le bouclier porté au bras, s'il y en a un.
    pub fn shield(&self) -> Option<Stack> { self.equip[SLOTS.iter().position(|s| s.0 == crate::data::Slot::Hand).unwrap_or(5)] }
    /// L'objet en main se pose-t-il (un bloc) ? Alors le clic droit pose au lieu de lever la garde.
    pub fn held_places(&self) -> bool { self.inv[self.sel].is_some_and(|st| crate::blocks::by_name(st.item).is_some()) }
    /// Une attaque venue de `from` contre la garde : parée (rien), bloquée en partie, ou entière. Rend les dégâts qui passent.
    pub(crate) fn block(&mut self, from: [f64; 3], n: f64) -> f64 {
        let Some(sh) = self.shield() else { return n }; if self.guard_t < 0.0 { return n; }
        let (dx, dz) = (from[0] - self.player.p[0], from[2] - self.player.p[2]); let d = dx.hypot(dz).max(0.01);
        if (dx * self.look[0] + dz * self.look[2]) / d / self.look[0].hypot(self.look[2]).max(0.01) < 0.35 { return n; } // de côté ou de dos : la garde ne couvre pas
        let armor = def(sh.item).and_then(|d| d.wear).map_or(0.0, |w| w.armor); self.wear_equip(SLOTS.iter().position(|s| s.0 == crate::data::Slot::Hand).unwrap_or(5), 1);
        if self.guard_t <= PARRY_T { if let Some(k) = self.wild.nearest(from) { self.wild.stagger(k, self.player.p, 2.0); } self.notes.push("Parade !".into()); self.sound.thump(from); self.skills[2] += 0.5; return 0.0; }
        let stop = block_share(armor); let cost = n * stop * 0.8; let ok = self.creative || self.stamina >= cost; if !self.creative { self.spend(cost); }
        self.sound.thump(from); n * (1.0 - if ok { stop } else { stop * 0.4 })
    }
    pub(crate) fn attack(&mut self, heavy: bool) {
        use crate::session::Prey;
        let Some(prey) = self.prey.take() else { return }; let tool = self.tool_id(); let (mut dmg, wait) = weapon(tool); self.hit_t = wait;
        dmg *= skill_bonus(self.skills[2]); let cost = if heavy { STAMINA_HIT * HEAVY_COST } else { STAMINA_HIT }; if heavy { dmg *= HEAVY_MUL; }
        if !self.creative { if self.stamina < cost { dmg *= 0.5; } self.spend(cost); }
        self.sound.swing(voice(tool), 1.0); let me = self.player.p; let knife = self.tool().is_some_and(|t| t.kind == "couteau");
        let (loot, extra): (&[(&str, u32)], &[(&str, u32)]) = match prey {
            Prey::Deer(k) => { let at = self.fauna.deer[k].p; self.sound.thump(at); let close = (at[0] - me[0]).hypot(at[2] - me[2]) < 2.4;
                if self.fauna.strike(k, dmg, me) { (DEER_LOOT, DEER_KNIFE) } else { if close && hash_float((self.elapsed * 977.0) as i32, k as i32, self.seed, 13) < 0.3 { self.harm(6.0, "La ruade d'un cerf", true); } (&[], &[]) } }
            Prey::Wild(k) => { let at = self.wild.all[k].p; self.sound.thump(at); if heavy { self.wild.stagger(k, me, 1.2); } match self.wild.strike(k, dmg, me) { Some(crate::wild::Kind::Fox) => (FOX_LOOT, FOX_KNIFE), Some(crate::wild::Kind::Hare) => (HARE_LOOT, HARE_KNIFE), Some(crate::wild::Kind::Wolf) => (WOLF_LOOT, &[("fur", 1)]), Some(crate::wild::Kind::Boar) => (BOAR_LOOT, &[("raw_meat", 1), ("leather", 1)]), Some(crate::wild::Kind::Bear) => (BEAR_LOOT, &[("raw_hide", 1)]), Some(crate::wild::Kind::Ibex) => (IBEX_LOOT, &[("raw_meat", 1)]), Some(crate::wild::Kind::Lizard) => (LIZARD_LOOT, &[]), _ => (&[], &[]) } }
        };
        self.wear_tool(1);
        if !loot.is_empty() && !self.creative {
            let mut got: Vec<(&str, u32)> = loot.to_vec(); if knife { for (id, n) in extra { if let Some(g) = got.iter_mut().find(|g| g.0 == *id) { g.1 += n; } else { got.push((id, *n)); } } }
            let mut said = Vec::new(); for (id, n) in got { self.add_item(id, n); if let Some(i) = item(id) { said.push(format!("+{n} {}", i.label)); } } self.notes.push(said.join(", ")); self.skills[2] += 2.0; self.dirty = true;
        }
    }

    // ------------------------------------------------------------------ clic droit
    /// Tout ce que fait le clic droit avant de poser un bloc : ramasser, cueillir, ouvrir un coffre, dormir, nourrir un feu,
    /// labourer, semer, remplir une gourde, manger, boire, se soigner. Vrai si le clic a servi.
    pub(crate) fn interact(&mut self) -> bool {
        if self.outdoor_use() { return true; } // l'arc tire, la canne lance ou ferre
        if self.greet() { return true; } // on parle aux villageois
        let held = self.inv[self.sel]; let hd = held.and_then(|s| def(s.item));
        if let Some(t) = self.target {
            let h = self.player.eye(); let near = ((t.x as f64 + 0.5 - h[0]).powi(2) + (t.y as f64 + 0.5 - h[1]).powi(2) + (t.z as f64 + 0.5 - h[2]).powi(2)).sqrt() <= 4.5;
            let name = BLOCKS[t.id as usize].name; let p = [t.x, t.y, t.z];
            if near {
                if self.use_t > 0.0 && (pickup(name).is_some() || harvest(name).is_some()) { return true; }
                if let Some((id, n)) = pickup(name) { if self.world.edit(t.x, t.y, t.z, 0) { self.add_item(id, n); self.sound.place(name); self.anim.trigger(); self.use_t = 0.3; self.dirty = true; } return true; }
                if let Some((id, n, bare)) = harvest(name) { if let Some(b) = by_name(bare) { if self.world.edit(t.x, t.y, t.z, b) { self.add_item(id, n); self.regrow.push((p, self.hours)); self.sound.place(name); self.anim.trigger(); self.use_t = 0.4; self.dirty = true; } } return true; }
                // Porte et trappe : clic droit pour ouvrir ou fermer (accroupi, on pose contre elles).
                if let Some(si) = crate::shapes::info(t.id).filter(|i| matches!(i.kind, crate::shapes::Kind::Door | crate::shapes::Kind::Trapdoor | crate::shapes::Kind::Shutter)) { if !self.player.crouch {
                    if self.use_t > 0.0 { return true; } self.toggle(p, si); self.use_t = 0.25; return true; } }
                if t.id == B::CHEST && !self.player.crouch { self.chests.entry(p).or_insert_with(|| vec![None; CHEST_SIZE]); self.chest = Some(p); self.inv_open = true; self.sound.place("chest"); return true; }
                if (t.id == B::BED || t.id == B::BEDROLL) && !self.player.crouch { self.sleep(p, t.id); return true; }
                // Nourrir un feu : bois, bâtons, charbon ; un feu éteint se rallume.
                // Allumer : la torche prend à coup sûr, le silex et l'archet à feu au bout de quelques essais ; sans bois, rien ne prend.
                if t.id == B::CAMPFIRE_OUT && held.is_some_and(|s| matches!(s.item, "torch" | "flint" | "fire_drill")) {
                    if self.use_t > 0.0 { return true; } let it = held.unwrap().item; self.use_t = if it == "torch" { 0.3 } else { 0.55 }; self.anim.trigger();
                    if self.fires.get(&p).is_none_or(|f| *f <= 0.0) { self.notes.push("Il n'y a plus de bois : ajoutez-en avant d'allumer".into()); return true; }
                    let chance = match it { "torch" => 1.0, "flint" => 0.4, _ => 0.3 } * (1.0 - 0.5 * self.weather.state.rain.min(1.0));
                    self.sound.place("flint"); if it == "fire_drill" { self.wear_slot(self.sel, 1); } if !self.creative && hash_float((self.elapsed * 397.0) as i32, p[0] ^ p[2], self.seed, 23) >= chance { self.notes.push(if it == "torch" { "Le feu ne prend pas sous cette pluie" } else { "Des étincelles... encore" }.into()); return true; }
                    self.world.edit(t.x, t.y, t.z, B::CAMPFIRE); self.sound.place("campfire"); self.notes.push("Le feu prend".into()); self.dirty = true; return true;
                }
                if (t.id == B::CAMPFIRE || t.id == B::CAMPFIRE_OUT) && hd.is_some_and(|d| d.fuel > 0.0 && d.block.is_none_or(|b| !is_deco(b))) {
                    if self.use_t > 0.0 { return true; } let d = hd.unwrap(); let add = d.fuel * FUEL_SCALE; *self.fires.entry(p).or_insert(0.0) += add;
                    if t.id == B::CAMPFIRE_OUT { self.notes.push("Bois ajouté : allumez le feu (silex, torche ou archet à feu)".into()); }
                    self.consume_held(); self.use_t = 0.3; self.sound.place("campfire"); return true;
                }
                // La houe laboure l'herbe et la terre ; les graines se sèment sur la terre labourée.
                if hd.and_then(|d| d.tool).is_some_and(|tl| tl.kind == "houe") && matches!(t.id, B::GRASS | B::GRASS_DEEP | B::DIRT | B::FOREST_SOIL | B::MOOR_GRASS | B::STEPPE_GRASS) && self.world.get(t.x, t.y + 1, t.z).is_some_and(|b| b == 0 || (is_deco(b) && !is_solid(b))) {
                    if self.use_t > 0.0 { return true; } self.world.edit(t.x, t.y + 1, t.z, 0); self.world.edit(t.x, t.y, t.z, B::FARMLAND); self.wear_tool(1); self.use_t = 0.3; self.sound.dig("dirt", Some("pickaxe")); self.anim.trigger(); self.dirty = true; return true;
                }
                // Semer : graines de blé, carotte ou pomme de terre, sur une terre labourée.
                if let Some(first) = held.and_then(|s| sow(s.item)).filter(|_| t.id == B::FARMLAND && self.world.get(t.x, t.y + 1, t.z) == Some(0)) {
                    if self.use_t > 0.0 { return true; } self.world.edit(t.x, t.y + 1, t.z, first); self.crops.push(([t.x, t.y + 1, t.z], self.hours)); self.consume_held(); self.use_t = 0.25; self.sound.place("tall_grass"); self.anim.trigger(); return true;
                }
            }
        }
        if self.use_t > 0.0 { return hd.is_some_and(|d| d.food.is_some() || d.drink.is_some() || d.heal > 0.0 || d.id == "waterskin"); }
        // Manger ce qu'on tient. Accroupi, le champignon se pose au lieu d'être mangé.
        if let (Some(st), Some(f)) = (held, hd.and_then(|d| d.food)) {
            let block = hd.is_some_and(|d| d.block.is_some()); if block && self.player.crouch { return false; }
            if self.vit[1] >= 99.0 && f.health >= 0.0 { if block { return false; } self.use_t = 1.5; self.notes.push("Vous n'avez plus faim".into()); return true; }
            let k = skill_bonus(self.skills[4]); self.vit[1] = (self.vit[1] + f.hunger * k).min(100.0); self.vit[2] = (self.vit[2] + f.thirst).clamp(0.0, 100.0);
            let risk = hd.map_or(0.0, |d| d.risk); if f.health < 0.0 || (risk > 0.0 && hash_float((self.elapsed * 331.0) as i32, 5, self.seed, 17) < risk) { self.harm(f.health.abs().max(3.0), "Une nourriture avariée", false); self.notes.push(format!("{} ne vous réussit pas", item(st.item).map_or("Ce repas", |i| i.label))); } else { self.vit[0] = (self.vit[0] + f.health).min(100.0); }
            self.consume_held(); self.use_t = 0.7; self.anim.trigger(); self.sound.place("mushroom"); self.gain(1); return true;
        }
        // Boire à la gourde : l'eau brute peut rendre malade ; la gourde se vide.
        if let (Some(_), Some((q, safe))) = (held, hd.and_then(|d| d.drink)) {
            self.vit[2] = (self.vit[2] + q).min(100.0); if !safe && hash_float((self.elapsed * 211.0) as i32, 9, self.seed, 19) < 0.3 { self.harm(5.0, "Une eau non bouillie", false); self.notes.push("Cette eau n'était pas saine : faites-la bouillir au feu".into()); }
            self.wear_slot(self.sel, 1); self.use_t = 0.7; self.sound.splash(0.2); self.anim.trigger(); self.gain(2); self.dirty = true; return true;
        }
        if let Some(d) = hd.filter(|d| d.heal > 0.0) { if self.vit[0] >= 100.0 { return true; } self.vit[0] = (self.vit[0] + d.heal).min(100.0); self.consume_held(); self.use_t = 1.0; self.gain(0); self.notes.push("Blessure pansée".into()); return true; }
        if hd.is_some_and(|d| d.block.is_some()) || self.creative { return false; }
        // Les mains libres ou un outil : on boit dans la mare ou le ruisseau visés ; la gourde vide s'y remplit.
        let (e, f) = (self.player.eye(), self.look); let mut t = 0.0;
        while t < 3.5 { let q = [e[0] + f[0] * t, e[1] + f[1] * t, e[2] + f[2] * t]; match self.world.get(q[0].floor() as i32, q[1].floor() as i32, q[2].floor() as i32) {
            Some(b) if is_water(b) => {
                if held.is_some_and(|s| s.item == "waterskin") { self.inv[self.sel] = Some(Stack::new("waterskin_raw", None)); self.sound.splash(0.3); self.use_t = 0.6; self.notes.push("Gourde remplie : faites-la bouillir au feu de camp pour boire sans risque".into()); self.dirty = true; return true; }
                // Une eau dormante peut rendre malade ; celle qui court est sûre.
                let still = b == B::WATER && self.gen.column(q[0], q[2]).still;
                self.vit[2] = (self.vit[2] + 15.0).min(100.0); self.use_t = 0.6; self.sound.splash(0.25); self.anim.trigger(); self.gain(2);
                if still && hash_float((self.elapsed * 97.0) as i32, 3, self.seed, 23) < 0.12 { self.harm(4.0, "Une eau stagnante", false); self.notes.push("Cette eau dormante n'était pas saine".into()); }
                return true; }
            Some(b) if is_solid(b) => return false, None => return false, _ => {} } t += 0.1; }
        false
    }
    fn consume_held(&mut self) { if self.creative { return; } if let Some(st) = self.inv[self.sel] { let n = st.n.unwrap_or(1).saturating_sub(1); self.inv[self.sel] = if n == 0 { None } else { Some(Stack { n: Some(n), ..st }) }; self.dirty = true; } }
    /// Un besoin vient de changer : son icône se montre un moment (0 vie, 1 faim, 2 soif, 3 endurance, 4 chaleur, 5 fatigue).
    pub fn gain(&mut self, k: usize) { self.last_gain[k] = self.elapsed; }
    /// Ce qui se passe quand un bloc est posé : un feu commence à brûler, un coffre s'ouvre à vide.
    pub(crate) fn on_placed(&mut self, p: [i32; 3], id: Id) {
        // Un feu de camp se pose éteint, avec sa première brassée de bois : il faut l'allumer.
        if id == B::CAMPFIRE { self.world.edit(p[0], p[1], p[2], B::CAMPFIRE_OUT); self.fires.insert(p, FIRE_START); self.notes.push("Feu de camp posé : allumez-le d'un clic droit avec un silex, une torche ou un archet à feu".into()); }
        if id == B::CHEST { self.chests.insert(p, vec![None; CHEST_SIZE]); }
    }
    /// Un bloc vient d'être cassé : un coffre rend son contenu, un feu s'éteint pour de bon, une culture est arrachée.
    /// Ouvre ou ferme une porte (ses deux moitiés) ou une trappe (qui s'ouvre du côté opposé au joueur).
    fn toggle(&mut self, p: [i32; 3], si: crate::shapes::Info) {
        use crate::shapes::{facing_of, variant, Kind};
        let open = !si.open; let mut edits = Vec::new();
        if si.kind == Kind::Door {
            let low = if si.upper { [p[0], p[1] - 1, p[2]] } else { p }; edits.push((low[0], low[1], low[2], variant(si.base, si.facing, false, open, false)));
            if self.world.get(low[0], low[1] + 1, low[2]).and_then(crate::shapes::info).is_some_and(|i| i.kind == Kind::Door) { edits.push((low[0], low[1] + 1, low[2], variant(si.base, si.facing, false, open, true))); }
        } else if si.kind == Kind::Shutter { edits.push((p[0], p[1], p[2], variant(si.base, si.facing, false, open, false))); } else { let f = self.player.forward(); edits.push((p[0], p[1], p[2], variant(si.base, (facing_of(f[0], f[2]) + 2) % 4, false, open, false))); }
        self.world.edit_many(&edits); self.sound.place("planks"); self.anim.trigger(); self.dirty = true;
    }
    pub(crate) fn on_broken(&mut self, p: [i32; 3], id: Id) {
        // Une porte se casse en entier : l'autre moitié part avec, et la porte revient une seule fois dans le sac.
        if let Some(si) = crate::shapes::info(id).filter(|i| i.kind == crate::shapes::Kind::Door) {
            let other = if si.upper { [p[0], p[1] - 1, p[2]] } else { [p[0], p[1] + 1, p[2]] };
            if self.world.get(other[0], other[1], other[2]).and_then(crate::shapes::info).is_some_and(|i| i.kind == crate::shapes::Kind::Door) { self.world.edit(other[0], other[1], other[2], 0); }
            if si.upper && !self.creative { self.add_item("door", 1); }
        }
        if id == B::CHEST { if let Some(items) = self.chests.remove(&p) { let mut rest = Vec::new(); for st in items.into_iter().flatten() { if let Some(r) = self.add_stack(st) { rest.push(r); } } if !rest.is_empty() { self.stashes.push(Stash { p: [p[0] as f64 + 0.5, p[1] as f64, p[2] as f64 + 0.5], items: rest, born: self.hours }); self.notes.push("Le sac est plein : le reste du coffre attend dans un sac posé là".into()); } } if self.chest == Some(p) { self.chest = None; } }
        if id == B::CAMPFIRE || id == B::CAMPFIRE_OUT { self.fires.remove(&p); }
        self.crops.retain(|c| c.0 != p);
        if self.bed == Some(p) { self.bed = None; self.notes.push("Votre lit est détruit : vous réapparaîtrez au point de départ".into()); }
    }
    /// Fermer le coffre (ou l'inventaire).
    pub fn close_chest(&mut self) { self.chest = None; }
    /// Un artisan transmet sa technique : ses recettes deviennent faisables (aux ateliers voulus). Pour les villages.
    pub fn learn(&mut self, craftsman: &str) -> bool { match crate::data::db().craftsmen.iter().find(|c| c.id == craftsman) { Some(c) if c.services.contains(&"transmission") => { self.taught.insert(c.id); self.dirty = true; true } _ => false } }

    // ------------------------------------------------------------------ sommeil
    /// Dormir jusqu'au matin : la fatigue tombe selon la qualité du lieu (lit, abri, chaleur), on guérit, le temps passe ; on réapparaîtra ici.
    pub fn sleep(&mut self, p: [i32; 3], id: Id) {
        if self.creative { return; }
        if self.vit[1] < 10.0 || self.vit[2] < 10.0 { self.notes.push("Trop faim ou trop soif pour dormir".into()); return; }
        if self.sun_height() > 0.0 && self.fatigue < 25.0 { self.notes.push("Vous n'avez pas sommeil".into()); self.bed = Some(p); self.spawn = [p[0] as f64 + 0.5, p[1] as f64 + 0.6, p[2] as f64 + 0.5]; return; }
        if self.exposure.abs() > 12.0 { self.notes.push(if self.exposure < 0.0 { "Il fait trop froid pour dormir : un feu, un abri ou des vêtements" } else { "Il fait trop chaud pour dormir" }.into()); return; }
        let wake = 7.0; let h = (wake - self.hour).rem_euclid(24.0).clamp(1.0, 10.0); let mut q = if id == B::BED { 1.0 } else { 0.7 };
        if self.covered(self.player.eye()) { q += 0.15; } if self.exposure.abs() > 4.0 { q -= 0.25; }
        self.pass_hours(h); self.fatigue = (self.fatigue - h * 14.0 * q).max(0.0); self.vit[0] = (self.vit[0] + h * 4.0 * q).min(100.0);
        self.vit[1] = (self.vit[1] - h * 1.2 * PROFILES[self.profile].needs).max(0.0); self.vit[2] = (self.vit[2] - h * 1.6 * PROFILES[self.profile].needs).max(0.0); self.stamina = 100.0;
        self.bed = Some(p); self.spawn = [p[0] as f64 + 0.5, p[1] as f64 + 0.6, p[2] as f64 + 0.5]; self.gain(5);
        self.notes.push(format!("Vous avez dormi {} h{}", h.round(), if q >= 1.0 { " : bien reposé" } else if q < 0.6 { ", mal" } else { "" })); self.dirty = true;
    }

    // ------------------------------------------------------------------ le corps
    /// Un coup : la roulade l'esquive.
    pub fn hurt(&mut self, n: f64) { self.harm(n, "Un coup", true); }
    /// Tout ce qui blesse passe par ici : la musique de danger en dépend. `dodge` : un coup qu'une roulade évite et que l'armure amortit.
    pub fn harm(&mut self, n: f64, cause: &'static str, dodge: bool) {
        if self.creative || self.dead.is_some() || (dodge && self.player.rolling()) { return; }
        let n = n * PROFILES[self.profile].harm * if dodge { armor_cut(self.worn().2) } else { 1.0 };
        self.vit[0] = (self.vit[0] - n).max(0.0); self.hurt_at = self.elapsed; self.gain(0); self.sound.hurt(); self.cause = cause;
        if self.vit[0] <= 0.0 { self.die(cause); }
    }
    /// Rougeur de l'écran juste après une blessure, de 1 à 0.
    pub fn flash(&self) -> f64 { (1.0 - (self.elapsed - self.hurt_at) / 0.6).clamp(0.0, 1.0) }
    /// La mort : tout ce que l'on portait, équipement compris, reste dans un sac, là où l'on est tombé.
    fn die(&mut self, cause: &'static str) {
        let p = self.player.p; let mut items: Vec<Stack> = self.inv.iter_mut().chain(self.equip.iter_mut()).filter_map(|s| s.take()).collect(); if let Some(c) = self.cursor.take() { items.push(c); }
        if !items.is_empty() { self.stashes.push(Stash { p, items, born: self.hours }); }
        self.dead = Some(Death { cause, at: p }); self.inv_open = false; self.chest = None; self.from = None; self.mine_key = None; self.mine_t = 0.0; self.felling = None; self.prey = None; self.dirty = true;
    }
    /// Retour au lit, s'il existe encore, sinon au point de départ, en pleine forme ; le sac attend là où l'on est mort.
    pub fn respawn(&mut self) {
        if self.dead.take().is_none() { return; }
        if let Some(b) = self.bed { if !self.world.get(b[0], b[1], b[2]).is_none_or(|x| x == B::BED || x == B::BEDROLL) { self.bed = None; } }
        let s = self.spawn; let (yaw, pitch) = (self.player.yaw, self.player.pitch); self.player = Player::new(s[0], s[1], s[2]); self.player.yaw = yaw; self.player.pitch = pitch;
        self.vit = [100.0, 80.0, 80.0]; self.stamina = 100.0; self.air = 100.0; self.body = 37.0; self.fatigue = self.fatigue.min(30.0); self.winded = false; self.low_v = 0.0; self.pain_t = 1.0; self.hurt_at = -99.0; self.dirty = true;
        if !self.stashes.is_empty() { self.notes.push("Vos affaires vous attendent là où vous êtes tombé".into()); }
    }
    pub(crate) fn gate(&self, mut i: Input) -> Input {
        if self.creative { return i; }
        if self.winded || self.stamina <= 0.0 { i.sprint = false; } if self.winded || self.stamina < STAMINA_ROLL { i.roll = false; }
        // Trop chargé, on avance plus lentement ; bien trop chargé, on ne court ni ne saute plus.
        let over = self.carried() / self.capacity(); if over > 1.0 { let k = if over > 1.5 { 0.45 } else { 0.75 }; i.x *= k; i.z *= k; if over > 1.5 { i.sprint = false; i.jump = false; } }
        i
    }
    pub(crate) fn spend(&mut self, n: f64) { let k = if self.carried() > self.capacity() { 1.5 } else { 1.0 }; self.stamina = (self.stamina - n * k).max(0.0); self.spent_t = 0.0; if n > 2.0 { self.gain(3); } }
    /// Ce qui pique ou brûle au contact : un cactus frôlé, un feu de camp sous les pieds.
    fn touching(&self) -> Option<Id> {
        let p = self.player.p; let fl = |v: f64| v.floor() as i32; let get = |x, y, z| self.world.get(x, y, z).unwrap_or(0);
        for y in fl(p[1] + 0.05)..=fl(p[1] + 0.5) { for z in fl(p[2] - 0.22)..=fl(p[2] + 0.22) { for x in fl(p[0] - 0.22)..=fl(p[0] + 0.22) { if get(x, y, z) == B::CAMPFIRE { return Some(B::CAMPFIRE); } } } }
        for y in fl(p[1] + 0.05)..=fl(p[1] + 1.7) { for z in fl(p[2] - 0.38)..=fl(p[2] + 0.38) { for x in fl(p[0] - 0.38)..=fl(p[0] + 0.38) { if get(x, y, z) == B::CACTUS { return Some(B::CACTUS); } } } }
        None
    }
    /// Chaleur d'un feu proche : un feu de camp, un four ou une torche, plus forte de près.
    fn fire_heat(&self) -> f64 {
        let p = self.player.p; let (bx, by, bz) = (p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32); let mut best = 0.0f64;
        for y in by - 2..=by + 2 { for z in bz - 5..=bz + 5 { for x in bx - 5..=bx + 5 { let b = self.world.get(x, y, z).unwrap_or(0);
            let k = if b == B::CAMPFIRE { 20.0 } else if b == B::FURNACE { 14.0 } else if b == B::TORCH { 4.0 } else { continue };
            let d = ((x as f64 + 0.5 - p[0]).powi(2) + (z as f64 + 0.5 - p[2]).powi(2)).sqrt(); best = best.max(k * (1.0 - d / 6.0).max(0.0)); } } }
        best
    }
    /// Température ressentie là où l'on se tient (avant vêtements).
    pub fn air_here(&self) -> f64 {
        let m = self.mix(); const BASE: [f64; 7] = [15.0, 18.0, 13.0, -4.0, 20.0, 30.0, 6.0]; const AMP: [f64; 7] = [6.0, 7.0, 4.0, 5.0, 9.0, 15.0, 6.0];
        let (mut b, mut a) = (0.0, 0.0); for i in 0..7 { b += m.w[i] * BASE[i]; a += m.w[i] * AMP[i]; } let w = self.weather.state;
        air_temp(Climate { biome: b, day_amp: a, altitude: self.player.p[1], sun: self.sun_height(), cloud: w.cloud, rain: w.rain, covered: self.covered(self.player.eye()), in_water: self.player.in_water, fire: self.fire_heat() })
    }
    /// Le corps : faim, soif, fatigue, endurance, souffle, chaleur, chutes, blessures qui reviennent, guérison, sacs à reprendre.
    pub(crate) fn survive(&mut self, dt: f64, i: Input, jumped: bool, roll_start: bool, working: bool, was_ground: bool) {
        let pr = PROFILES[self.profile]; let swim = self.player.in_water && self.player.speed > 0.8;
        let sprint = i.sprint && self.player.speed > 5.0 && !self.player.fly; let effort = if sprint { 2.5 } else if working || swim { 1.5 } else { 1.0 };
        // Chaleur : le corps suit lentement l'inconfort ; le froid creuse la faim, la chaleur la soif.
        self.climate_t -= dt; if self.climate_t <= 0.0 { self.climate_t = 0.5; let (warm, cool, ..) = self.worn(); let was = self.exposure; self.exposure = exposure(self.air_here(), warm, cool) * pr.climate; if (self.exposure.abs() > 3.0) != (was.abs() > 3.0) { self.gain(4); } }
        let target = 37.0 + (self.exposure * 0.25).clamp(-6.0, 4.0); self.body += (target - self.body) * (dt / 40.0).min(1.0);
        let cold = (37.0 - self.body).max(0.0); let hot = (self.body - 37.0).max(0.0);
        self.vit[1] = (self.vit[1] - dt * HUNGER_RATE * effort * pr.needs * (1.0 + 0.3 * cold)).max(0.0);
        self.vit[2] = (self.vit[2] - dt * THIRST_RATE * effort * pr.needs * (1.0 + self.desert * 0.5 + 0.5 * hot)).max(0.0);
        if self.player.in_water && !swim { self.vit[2] = (self.vit[2] + dt * 8.0).min(100.0); }
        self.fatigue = (self.fatigue + dt * FATIGUE_RATE * pr.fatigue * if sprint || working { 1.5 } else { 1.0 }).min(100.0);
        self.spent_t += dt; if sprint { self.spend(STAMINA_SPRINT * dt); } if swim { self.spend(STAMINA_SWIM * dt); } if jumped { self.spend(STAMINA_JUMP); } if roll_start { self.spend(STAMINA_ROLL); }
        // Affamé ou épuisé, on s'épuise vite ; fatigué ou transi, l'endurance revient lentement.
        let cap = if self.vit[1] < 15.0 || self.fatigue > 90.0 { 50.0 } else { 100.0 };
        if self.spent_t > STAMINA_WAIT { let k = if self.vit[2] < 15.0 { 0.5 } else { 1.0 } * if self.fatigue > 70.0 { 0.6 } else { 1.0 } * if cold > 1.5 { 0.7 } else { 1.0 }; self.stamina += dt * STAMINA_REGEN * k; } self.stamina = self.stamina.min(cap);
        if self.stamina <= 0.0 { self.winded = true; } if self.winded && self.stamina >= WINDED_UNTIL { self.winded = false; }
        let h = self.player.eye(); let sub = self.world.get(h[0].floor() as i32, h[1].floor() as i32, h[2].floor() as i32).is_some_and(is_water);
        self.air = if sub { (self.air - dt * 100.0 / AIR_TIME).max(0.0) } else { (self.air + dt * 60.0).min(100.0) };
        // Chute : la vitesse la plus forte de la descente, lue au moment de toucher le sol ; l'eau amortit tout.
        if self.player.ground && !was_ground && !self.player.in_water && !self.player.fly { let v = -self.low_v; if v > SAFE_FALL { self.harm(((v - SAFE_FALL) * 4.0).round(), "Une chute", false); } }
        if self.player.ground || self.player.in_water || self.player.fly { self.low_v = 0.0; } else { self.low_v = self.low_v.min(self.player.v[1]); }
        self.pain_t -= dt;
        if self.pain_t <= 0.0 {
            let pain = if sub && self.air <= 0.0 { Some((8.0, "La noyade", 1.0)) } else if let Some(b) = self.touching() { if b == B::CACTUS { Some((3.0, "Les épines d'un cactus", 0.7)) } else { Some((5.0, "Les flammes", 0.7)) } }
                else if self.body < 33.5 { Some((2.0, "Le froid", 2.0)) } else if self.body > 40.0 { Some((2.0, "La chaleur", 2.0)) }
                else if self.vit[2] <= 0.0 { Some((2.0, "La soif", 3.0)) } else if self.vit[1] <= 0.0 { Some((2.0, "La faim", 4.0)) } else if self.fatigue >= 100.0 { Some((1.0, "L'épuisement", 6.0)) } else { None };
            if let Some((n, c, every)) = pain { self.pain_t = every; self.harm(n, c, false); }
        }
        if self.dead.is_some() { return; }
        // Bien nourri, désaltéré et pas transi, on guérit ; c'est le ventre qui paie.
        if self.vit[0] < 100.0 && self.vit[1] >= 60.0 && self.vit[2] >= 50.0 && cold < 2.0 { let r = if self.vit[1] >= 90.0 && self.vit[2] >= 80.0 { 1.5 } else { 0.6 }; self.vit[0] = (self.vit[0] + dt * r).min(100.0); self.vit[1] = (self.vit[1] - dt * 0.15 * r).max(0.0); }
        let p = self.player.p; let mut back = false;
        for k in (0..self.stashes.len()).rev() {
            let q = self.stashes[k].p; if (q[0] - p[0]).hypot(q[2] - p[2]) > 1.8 || (q[1] - p[1]).abs() > 2.5 { continue; }
            let items = std::mem::take(&mut self.stashes[k].items); let mut rest = Vec::new();
            for st in items { if let Some(w) = def(st.item).and_then(|d| d.wear) { let e = SLOTS.iter().position(|s| s.0 == w.slot).unwrap_or(0); if self.equip[e].is_none() { self.equip[e] = Some(st); continue; } } if let Some(r) = self.add_stack(st) { rest.push(r); } }
            back = true; if rest.is_empty() { self.stashes.remove(k); } else { self.stashes[k].items = rest; }
        }
        if back { self.notes.push(if self.stashes.iter().any(|s| (s.p[0] - p[0]).hypot(s.p[2] - p[2]) <= 1.8) { "Affaires récupérées, mais le sac n'est pas vide : l'inventaire est plein".into() } else { "Affaires récupérées".into() }); self.sound.place("leather"); self.dirty = true; }
    }
    /// Le monde qui vit sans nous : baies et blé qui repoussent, feux qui se consument, dépouilles qui finissent par disparaître.
    pub(crate) fn world_step(&mut self, dt: f64) {
        // Les feux brûlent leur combustible ; à court, ils s'éteignent.
        if !self.fires.is_empty() { let mut out = Vec::new(); let lit: Vec<[i32; 3]> = self.fires.keys().filter(|p| self.world.get(p[0], p[1], p[2]) == Some(B::CAMPFIRE)).copied().collect(); for p in lit { let t = self.fires.get_mut(&p).unwrap(); *t -= dt; if *t <= 0.0 { out.push(p); } }
            for p in out { self.fires.remove(&p); if self.world.get(p[0], p[1], p[2]) == Some(B::CAMPFIRE) { self.world.edit(p[0], p[1], p[2], B::CAMPFIRE_OUT); self.dirty = true; } } }
        self.grow_t -= dt; if self.grow_t > 0.0 { return; } self.grow_t = 2.0; let now = self.hours;
        if !self.regrow.is_empty() { let mut keep = Vec::new();
            for (p, at) in std::mem::take(&mut self.regrow) { let cur = self.world.get(p[0], p[1], p[2]); let full = cur.and_then(|b| BLOCKS[b as usize].name.strip_suffix("_picked")).and_then(by_name);
                match (cur, full) { (None, _) => keep.push((p, at)), (Some(_), Some(f)) => { if now - at >= REGROW_HOURS { self.world.edit(p[0], p[1], p[2], f); self.dirty = true; } else { keep.push((p, at)); } } _ => {} } }
            self.regrow = keep; }
        // Le blé passe d'un stade au suivant ; sur une terre près de l'eau, il pousse plus vite.
        if !self.crops.is_empty() { let mut keep = Vec::new();
            for (p, at) in std::mem::take(&mut self.crops) { let Some(cur) = self.world.get(p[0], p[1], p[2]) else { keep.push((p, at)); continue }; let Some(first) = crop_first(cur) else { continue };
                let wet = (-4..=4).any(|dz: i32| (-4..=4).any(|dx: i32| self.world.get(p[0] + dx, p[1] - 1, p[2] + dz).is_some_and(is_water)));
                let stage = (((now - at) / CROP_STAGE_HOURS) * if wet { 1.5 } else { 1.0 }).floor().min(3.0) as Id; let want = first + stage;
                if want != cur { self.world.edit(p[0], p[1], p[2], want); self.dirty = true; } if stage < 3 { keep.push((p, at)); } }
            self.crops = keep; }
        let days = PROFILES[self.profile].stash_days; let before = self.stashes.len(); self.stashes.retain(|s| now - s.born < days * 24.0); if self.stashes.len() < before { self.notes.push("Une dépouille oubliée a disparu".into()); }
    }

    /// Valeurs des six besoins, de 0 (danger) à 1 (bien) ; la chaleur vaut 0,5 quand on est à l'aise.
    pub fn needs(&self) -> [f64; 6] { [self.vit[0] / 100.0, self.vit[1] / 100.0, self.vit[2] / 100.0, self.stamina / 100.0, (0.5 + self.exposure / 24.0).clamp(0.0, 1.0), 1.0 - self.fatigue / 100.0] }
    pub(crate) fn smooth_gauges(&mut self, dt: f64) { let n = self.needs(); for k in 0..6 { self.gauge[k] += (n[k] - self.gauge[k]) * (dt * 5.0).min(1.0); } }

    // ------------------------------------------------------------------ sauvegarde
    /// Sauvegarde lisible : l'état du joueur, puis seulement ce qui diffère du monde généré.
    pub fn save_text(&self) -> String {
        let p = &self.player; let st = |v: &Stack| format!("{} {} {}", v.item, v.n.map_or(-1, |n| n as i64), v.wear);
        let mut s = format!("A Living World 1\npalette {}\nseed {}\nplayer {} {} {} {} {}\nhour {}\ncreative {}\nsel {}\nvit {} {} {}\nweather {}\n", BLOCKS.iter().map(|b| b.name).collect::<Vec<_>>().join(" "), self.seed, p.p[0], p.p[1], p.p[2], p.yaw, p.pitch, self.hour, self.creative as u8, self.sel, self.vit[0], self.vit[1], self.vit[2], self.weather.t);
        s += &format!("body {} {} {} {}\nprofile {}\nskills {}\n", self.stamina, self.fatigue, self.body, self.air, self.profile, self.skills.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
        for (i, it) in self.inv.iter().enumerate() { if let Some(v) = it { s += &format!("inv {} {}\n", i, st(v)); } }
        for (i, it) in self.equip.iter().enumerate() { if let Some(v) = it { s += &format!("equip {} {}\n", i, st(v)); } }
        s += &format!("spawn {} {} {}\nhours {}\n", self.spawn[0], self.spawn[1], self.spawn[2], self.hours);
        if let Some(b) = self.bed { s += &format!("bed {} {} {}\n", b[0], b[1], b[2]); }
        let mut seen: Vec<&str> = self.seen.iter().copied().collect(); seen.sort(); s += &format!("seen {}\n", seen.join(" "));
        let mut taught: Vec<&str> = self.taught.iter().copied().collect(); taught.sort(); if !taught.is_empty() { s += &format!("taught {}\n", taught.join(" ")); }
        s += &self.town_save(); s += &self.map.save(); s += &format!("guide {}\n", self.guide);
        let mut fav: Vec<&str> = self.fav.iter().copied().collect(); fav.sort(); if !fav.is_empty() { s += &format!("fav {}\n", fav.join(" ")); }
        for (p, at) in &self.regrow { s += &format!("regrow {} {} {} {}\n", p[0], p[1], p[2], at); }
        for (p, at) in &self.crops { s += &format!("crop {} {} {} {}\n", p[0], p[1], p[2], at); }
        let mut fires: Vec<_> = self.fires.iter().collect(); fires.sort_by_key(|f| *f.0); for (p, t) in fires { s += &format!("fire {} {} {} {}\n", p[0], p[1], p[2], t); }
        let mut chests: Vec<_> = self.chests.iter().collect(); chests.sort_by_key(|c| *c.0);
        for (p, items) in chests { s += &format!("chest {} {} {}", p[0], p[1], p[2]); for (i, v) in items.iter().enumerate() { if let Some(v) = v { s += &format!(" {i}:{}:{}:{}", v.item, v.n.map_or(-1, |n| n as i64), v.wear); } } s.push('\n'); }
        for stv in &self.stashes { s += &format!("stash {} {} {} {}", stv.p[0], stv.p[1], stv.p[2], stv.born); for v in &stv.items { s += &format!(" {}:{}:{}", v.item, v.n.map_or(-1, |n| n as i64), v.wear); } s.push('\n'); }
        for (k, e) in self.world.edits() { s += &format!("chunk {} {}", k.0, k.1); let mut l: Vec<_> = e.into_iter().collect(); l.sort(); for (i, id) in l { s += &format!(" {i}:{id}"); } s.push('\n'); }
        s
    }
    pub fn load_text(&mut self, t: &str) -> Result<(), String> {
        let mut lines = t.lines(); if lines.next() != Some("A Living World 1") { return Err("format inconnu".into()); }
        let mut edits: HashMap<(i32, i32), HashMap<u32, Id>> = HashMap::new(); let mut remap: Option<Vec<Id>> = None; let mut inv = [None; INV_SIZE]; let mut stashes = Vec::new(); let mut stash_v1 = false;
        let num = |v: Option<&str>| v.and_then(|x| x.parse::<f64>().ok()).ok_or("nombre illisible");
        let stack = |id: &str, n: f64, w: f64| item(id).map(|it| Stack { item: it.id, n: if n < 0.0 { None } else { Some(n as u32) }, wear: w.max(0.0) as u32 });
        for l in lines {
            let mut a = l.split(' ');
            match a.next() {
                Some("player") => { for k in 0..3 { self.player.p[k] = num(a.next())?; } self.player.yaw = num(a.next())?; self.player.pitch = num(a.next())?; }
                Some("hour") => self.hour = num(a.next())?, Some("creative") => self.creative = num(a.next())? != 0.0, Some("sel") => self.sel = (num(a.next())? as usize).min(INV_SIZE - 1),
                Some("weather") => self.weather.t = num(a.next())?,
                Some("vit") => { for k in 0..3 { self.vit[k] = num(a.next())?; } }
                Some("body") => { self.stamina = num(a.next())?; self.fatigue = num(a.next())?; self.body = num(a.next())?; self.air = num(a.next())?; }
                Some("profile") => self.profile = (num(a.next())? as usize).min(PROFILES.len() - 1),
                Some("skills") => { for k in 0..5 { self.skills[k] = num(a.next()).unwrap_or(0.0); } }
                Some("inv") => { let i = num(a.next())? as usize; let id = a.next().unwrap_or(""); let n = num(a.next())?; let w = num(a.next()).unwrap_or(0.0); if let (Some(s), true) = (stack(id, n, w), i < INV_SIZE) { inv[i] = Some(s); } }
                Some("equip") => { let i = num(a.next())? as usize; let id = a.next().unwrap_or(""); let n = num(a.next())?; let w = num(a.next()).unwrap_or(0.0); if let (Some(s), true) = (stack(id, n, w), i < 5) { self.equip[i] = Some(s); } }
                Some("hours") => self.hours = num(a.next())?,
                Some("bed") => self.bed = Some([num(a.next())? as i32, num(a.next())? as i32, num(a.next())? as i32]),
                Some("seen") => { for id in a { if let Some(d) = def(id) { self.seen.insert(d.id); } } }
                Some("taught") => { for id in a.by_ref() { if let Some(c) = crate::data::db().craftsmen.iter().find(|c| c.id == id) { self.taught.insert(c.id); } } }
                Some("guide") => { self.guide = a.next().and_then(|v| v.parse().ok()).unwrap_or(0); }
                Some("carte") => { let v: Vec<&str> = a.collect(); self.map.load(&v); }
                Some(k @ ("rep" | "known" | "met" | "marked")) => { let v: Vec<&str> = a.collect(); self.town_load(k, &v); }
                Some("fav") => { for id in a { if let Some(d) = def(id) { self.fav.insert(d.id); } } }
                Some("regrow") => { let p = [num(a.next())? as i32, num(a.next())? as i32, num(a.next())? as i32]; self.regrow.push((p, num(a.next())?)); }
                Some("crop") => { let p = [num(a.next())? as i32, num(a.next())? as i32, num(a.next())? as i32]; self.crops.push((p, num(a.next())?)); }
                Some("fire") => { let p = [num(a.next())? as i32, num(a.next())? as i32, num(a.next())? as i32]; self.fires.insert(p, num(a.next())?); }
                Some("chest") => { let p = [num(a.next())? as i32, num(a.next())? as i32, num(a.next())? as i32]; let mut v = vec![None; CHEST_SIZE];
                    for c in a { let f: Vec<&str> = c.split(':').collect(); if f.len() == 4 { if let (Ok(i), Ok(n), Ok(w)) = (f[0].parse::<usize>(), f[2].parse::<f64>(), f[3].parse::<f64>()) { if i < CHEST_SIZE { v[i] = stack(f[1], n, w); } } } }
                    self.chests.insert(p, v); }
                Some("spawn") => { for k in 0..3 { self.spawn[k] = num(a.next())?; } }
                Some("stash") => { let p = [num(a.next())?, num(a.next())?, num(a.next())?]; let rest: Vec<&str> = a.collect();
                    // Ancien format (sans heure de la mort) : la dépouille repart de maintenant.
                    let (born, list) = match rest.first().and_then(|x| x.parse::<f64>().ok()) { Some(b) if !rest[0].contains(':') => (b, &rest[1..]), _ => { stash_v1 = true; (f64::NAN, &rest[..]) } };
                    let items: Vec<Stack> = list.iter().filter_map(|c| { let f: Vec<&str> = c.split(':').collect(); let n: f64 = f.get(1)?.parse().ok()?; let w: f64 = f.get(2).and_then(|x| x.parse().ok()).unwrap_or(0.0); stack(f[0], n, w) }).collect(); if !items.is_empty() { stashes.push(Stash { p, items, born }); } }
                // Les blocs sont sauvegardés par numéro ; la palette les retrouve par leur nom si la liste a changé.
                Some("palette") => remap = Some(a.map(|n| by_name(n).unwrap_or(0)).collect()),
                Some("chunk") => { let k = (num(a.next())? as i32, num(a.next())? as i32); let e = edits.entry(k).or_default(); for c in a { if let Some((i, id)) = c.split_once(':') { if let (Ok(i), Ok(id)) = (i.parse::<u32>(), id.parse::<Id>()) { e.insert(i, remap.as_ref().map_or(id, |r| r.get(id as usize).copied().unwrap_or(0))); } } } }
                _ => {}
            }
        }
        if stash_v1 { for s in &mut stashes { if s.born.is_nan() { s.born = self.hours; } } }
        for s in inv.iter().chain(self.equip.iter()).flatten() { self.seen.insert(s.item); }
        self.inv = inv; self.stashes = stashes; self.world.set_edits(edits);
        // Une partie sauvée au moment de la mort reprend vivante, au point de départ.
        if self.vit[0] <= 0.0 { self.dead = Some(Death { cause: "", at: self.player.p }); self.respawn(); }
        Ok(())
    }
}
/// État du livre de recettes : catégorie (0 = tout), page, recette choisie, recherche en cours de saisie.
#[derive(Default, Clone, Debug)] pub struct Book { pub cat: usize, pub page: usize, pub sel: Option<usize>, pub search: String, pub typing: bool }
/// Liste des recettes du livre, dans l'ordre : favorites d'abord, puis faisables, puis le reste ; seulement celles qu'on connaît.
pub fn book(s: &Session, cat: Option<&str>, search: &str) -> Vec<usize> {
    let st = s.stations(); let q = search.to_lowercase();
    let mut v: Vec<usize> = recipes().iter().enumerate().filter(|(_, r)| s.known(r) && cat.is_none_or(|c| r.cat == c) && (q.is_empty() || item(r.out).is_some_and(|i| i.label.to_lowercase().contains(&q)))).map(|(i, _)| i).collect();
    v.sort_by_key(|i| { let r = &recipes()[*i]; (!s.fav.contains(r.out), !s.can_craft(r, &st), *i) }); v
}
/// Toutes les recettes, connues ou non (pour le compte « à découvrir »).
pub fn unknown(s: &Session) -> usize { recipes().iter().filter(|r| !s.known(r)).count() }
pub const BOOK_CATS: [(&str, &str); 8] = [("outils", "Outils"), ("matieres", "Matières"), ("batir", "Bâtir"), ("mobilier", "Mobilier"), ("decor", "Décor"), ("ateliers", "Ateliers"), ("vivres", "Vivres"), ("habits", "Habits")];

/// Ce qu'on sème : le premier stade de la culture.
pub fn sow(item: &str) -> Option<Id> { match item { "seeds" => Some(B::WHEAT_0), "carrot" => Some(B::CARROT_0), "potato" => Some(B::POTATO_0), _ => None } }
/// Le premier stade de la culture dont `b` est un stade (les quatre stades se suivent dans le registre).
pub fn crop_first(b: Id) -> Option<Id> { [B::WHEAT_0, B::CARROT_0, B::POTATO_0].into_iter().find(|f| (*f..=*f + 3).contains(&b)) }
