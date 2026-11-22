//! Les outils du bâtisseur, pour construire sans peine :
//! - la pipette (clic molette) : le bloc visé passe en main (en survie, s'il est dans le sac) ;
//! - annuler et rétablir (/annuler, /retablir, depuis le chat) : les dernières poses et casses, une à une ;
//! - entre les deux coins (/coin1, /coin2) : /remplir, /vider, /murs, /sol remplissent la boîte d'un coup (créatif) ;
//! - /copier la boîte, puis /coller là où l'on vise, tournée des quarts de tour de R ; /miroir retourne la copie.
//! Chaque outil qui touche plusieurs blocs ne fait qu'un seul pas d'annulation. Tout est vérifié par `tests/outils.rs`.
use crate::blocks::{by_name, Id, BLOCKS};
use crate::player::Voxels;
use crate::session::{item, Session, Stack};

/// Une case changée : où, ce qu'il y avait, ce qu'il y a.
pub type Change = ([i32; 3], Id, Id);
/// Combien de pas on garde pour annuler.
pub const UNDO_MAX: usize = 64;
/// La plus grande boîte qu'on remplit ou copie d'un coup.
pub const BOX_MAX: i64 = 32 * 32 * 32;

/// Ce qui a été copié : sa taille et ses blocs (x, puis z, puis y), l'air compris.
#[derive(Clone, Debug, Default)]
pub struct Clip { pub size: [i32; 3], pub cells: Vec<Id> }
impl Clip {
    pub fn at(&self, x: i32, y: i32, z: i32) -> Id { self.cells[((y * self.size[2] + z) * self.size[0] + x) as usize] }
}

/// L'objet qui pose ce bloc : son nom, ou celui de sa forme de base (une chaise tournée, une dalle haute, un rondin couché).
pub fn item_of_block(b: Id) -> Option<&'static str> {
    if b == 0 { return None; }
    let n = BLOCKS[b as usize].name; let ok = |s: &str| item(s).filter(|i| i.block.is_some()).map(|i| i.id);
    if let Some(i) = ok(n) { return Some(i); }
    if let Some(si) = crate::shapes::info(b) { if let Some(i) = ok(BLOCKS[si.base as usize].name) { return Some(i); } }
    let mut s = n.split('@').next().unwrap_or(n).to_string();
    for suf in ["_c", "_x", "_z", "_y"] { if let Some(r) = s.strip_suffix(suf) { s = r.to_string(); if let Some(i) = ok(&s) { return Some(i); } } }
    if let Some(i) = ok(&s) { return Some(i); }
    BLOCKS[b as usize].drop.and_then(ok)
}

/// La matrice de `k` quarts de tour (colonnes : images de +x et de +z), retournée en miroir si `flip`.
pub fn turn_matrix(k: u8, flip: bool) -> [[i32; 2]; 2] {
    let mut m = if flip { [[-1, 0], [0, 1]] } else { [[1, 0], [0, 1]] };
    for _ in 0..k % 4 { m = [[-m[0][1], m[0][0]], [-m[1][1], m[1][0]]]; } // chaque image (x, z) devient (-z, x)
    m
}

impl Session {
    /// Clic molette : le bloc visé en main. En créatif, un tas plein ; en survie, la pile du sac qui le pose.
    pub fn pick_block(&mut self) -> String {
        let Some(t) = self.target else { return String::new() };
        let Some(it) = item_of_block(t.id) else { return "Ce bloc ne se ramasse pas".into() };
        let label = item(it).map_or(it, |i| i.label);
        // déjà dans la barre : on la sélectionne
        if let Some(k) = (0..9).find(|k| self.inv[*k].is_some_and(|s| s.item == it)) { self.sel = k; return format!("En main : {label}"); }
        if self.creative {
            let n = if crate::survival::stackable(it) { Some(crate::survival::max_stack(it)) } else { None };
            // une case libre de la barre plutôt que de remplacer ce qu'on tient
            let k = if self.inv[self.sel].is_none() { self.sel } else { (0..9).find(|k| self.inv[*k].is_none()).unwrap_or(self.sel) };
            self.inv[k] = Some(Stack::new(it, n)); self.sel = k; self.seen.insert(it); self.dirty = true; return format!("En main : {label}");
        }
        let Some(k) = (9..self.inv.len()).find(|k| self.inv[*k].is_some_and(|s| s.item == it)) else { return format!("Vous n'avez pas de {label}") };
        self.inv.swap(k, self.sel); self.dirty = true; format!("En main : {label}")
    }

    /// Note un pas d'annulation (les cases vraiment changées seulement). Un nouveau geste efface ce qu'on pouvait rétablir.
    pub(crate) fn record(&mut self, step: Vec<Change>) {
        let step: Vec<Change> = step.into_iter().filter(|c| c.1 != c.2).collect(); if step.is_empty() { return; }
        self.undo.push(step); if self.undo.len() > UNDO_MAX { self.undo.remove(0); } self.redo.clear();
    }
    /// Change plusieurs cases d'un coup (scellées exceptées) ; rend ce qui a changé, pour l'annulation.
    fn apply(&mut self, list: &[([i32; 3], Id)]) -> Vec<Change> {
        let mut ch = Vec::new(); let mut edits = Vec::new();
        for (p, id) in list { let Some(was) = self.world.get(p[0], p[1], p[2]) else { continue }; if was == *id || self.sealed(*p, *id) { continue; } ch.push((*p, was, *id)); edits.push((p[0], p[1], p[2], *id)); }
        self.world.edit_many(&edits);
        for (p, was, now) in &ch { self.flow.touch(p[0], p[1], p[2]); if *was != 0 { self.on_broken(*p, *was); } if *now != 0 { self.on_placed(*p, *now); } }
        for (p, id) in list { if crate::shapes::is_roof(*id) { self.join_roofs_at(*p); } }
        // ce qu'il y a vraiment (un feu de camp se pose éteint)
        for c in ch.iter_mut() { c.2 = self.world.get(c.0[0], c.0[1], c.0[2]).unwrap_or(c.2); }
        if !ch.is_empty() { self.dirty = true; } ch
    }
    /// Revient sur un pas (ou le refait) : chaque case qui n'a pas bougé depuis reprend son état d'alors. En survie, on récupère
    /// ce qu'on avait posé, et il faut avoir en sac ce qu'on remet.
    fn replay(&mut self, step: &[Change], back: bool) -> Result<usize, String> {
        let todo: Vec<([i32; 3], Id, Id)> = step.iter().map(|c| if back { (c.0, c.2, c.1) } else { *c }).filter(|(p, from, _)| self.world.get(p[0], p[1], p[2]) == Some(*from)).collect();
        if todo.is_empty() { return Err("Ces blocs ont changé depuis : rien à refaire".into()); }
        if !self.creative {
            let mut need: std::collections::BTreeMap<&'static str, u32> = Default::default();
            for (_, _, to) in &todo { if let Some(i) = item_of_block(*to) { *need.entry(i).or_default() += 1; } }
            for (i, n) in &need { if self.count(i) < *n { return Err(format!("Il vous manque {} × {}", n - self.count(i), item(i).map_or(*i, |d| d.label))); } }
            for (i, n) in need { self.take(i, n); }
            for (_, from, _) in &todo { if let Some(i) = item_of_block(*from) { let left = self.add_item(i, 1); if left > 0 { let q = self.player.p; self.spill(i, left, q); } } }
        }
        let list: Vec<([i32; 3], Id)> = todo.iter().map(|(p, _, to)| (*p, *to)).collect(); let n = self.apply(&list).len(); Ok(n)
    }
    /// /annuler : le dernier geste de bâtisseur défait.
    pub fn undo_last(&mut self) -> String {
        let Some(step) = self.undo.pop() else { return "Rien à annuler".into() };
        match self.replay(&step, true) { Ok(n) => { self.redo.push(step); self.sound.place("planks"); format!("Annulé : {n} bloc(s)") } Err(e) => { self.undo.push(step); e } }
    }
    /// /retablir : le geste annulé refait.
    pub fn redo_last(&mut self) -> String {
        let Some(step) = self.redo.pop() else { return "Rien à rétablir".into() };
        match self.replay(&step, false) { Ok(n) => { self.undo.push(step); self.sound.place("planks"); format!("Rétabli : {n} bloc(s)") } Err(e) => { self.redo.push(step); e } }
    }

    /// La boîte des deux coins : (min, max).
    pub fn corner_box(&self) -> Option<([i32; 3], [i32; 3])> {
        let [Some(a), Some(b)] = self.coins else { return None };
        Some(([a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])], [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]))
    }
    /// La boîte à montrer, la main vide de bloc : des deux coins, ou du premier au bloc visé tant que le second manque.
    pub fn select_box(&self) -> Option<([i32; 3], [i32; 3])> {
        if self.inv[self.sel].and_then(|s| item(s.item)).is_some_and(|i| i.block.is_some()) { return None; }
        let a = self.coins[0].or(self.coins[1])?; let b = match self.coins { [Some(_), Some(b)] => b, _ => self.target.map(|t| [t.x, t.y, t.z])? };
        Some(([a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])], [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]))
    }
    /// /oublier : plus de boîte.
    pub fn clear_corners(&mut self) -> String { self.coins = [None; 2]; "Boîte oubliée".into() }
    /// Le bloc dont on remplit : celui nommé, sinon celui qu'on tient.
    fn fill_block(&self, name: Option<&str>) -> Result<Id, String> {
        if let Some(n) = name { return crate::data::def(n).and_then(|d| d.block).or_else(|| by_name(n)).ok_or_else(|| format!("Bloc inconnu : « {n} »")); }
        self.inv[self.sel].and_then(|s| item(s.item)).and_then(|i| i.block).ok_or_else(|| "Tenez un bloc en main, ou nommez-le : /remplir stone_bricks".into())
    }
    /// /remplir, /vider, /murs, /sol : la boîte entière, ses quatre murs, ou son fond, d'un bloc (ou d'air).
    pub fn fill_box(&mut self, what: &str, name: Option<&str>) -> String {
        if !self.creative { return "Ces outils demandent le mode créatif (/creatif)".into(); }
        let Some((lo, hi)) = self.corner_box() else { return "Marquez d'abord la boîte : visez un bloc, /coin1, puis l'autre, /coin2".into() };
        let vol = (0..3).map(|k| (hi[k] - lo[k] + 1) as i64).product::<i64>(); if vol > BOX_MAX { return format!("Boîte trop grande : {vol} blocs (32 768 au plus)"); }
        let id = if what == "vider" { 0 } else { match self.fill_block(name) { Ok(b) => b, Err(e) => return e } };
        let mut list = Vec::new();
        for y in lo[1]..=hi[1] { for z in lo[2]..=hi[2] { for x in lo[0]..=hi[0] {
            let edge = x == lo[0] || x == hi[0] || z == lo[2] || z == hi[2];
            let keep = match what { "murs" => edge, "sol" => y == lo[1], _ => true }; if keep { list.push(([x, y, z], id)); } } } }
        let ch = self.apply(&list); let n = ch.len(); self.record(ch);
        if id != 0 { self.sound.place(BLOCKS[id as usize].name); }
        let verb = match what { "vider" => "Vidé", "murs" => "Murs montés", "sol" => "Sol posé", _ => "Rempli" };
        format!("{verb} : {n} bloc(s) — /annuler pour défaire")
    }
    /// /copier : la boîte des deux coins, telle quelle.
    pub fn copy_box(&mut self) -> String {
        let Some((lo, hi)) = self.corner_box() else { return "Marquez d'abord la boîte : /coin1 et /coin2".into() };
        let size = [hi[0] - lo[0] + 1, hi[1] - lo[1] + 1, hi[2] - lo[2] + 1]; let vol = size.iter().map(|v| *v as i64).product::<i64>();
        if vol > BOX_MAX { return format!("Boîte trop grande : {vol} blocs (32 768 au plus)"); }
        let mut cells = Vec::with_capacity(vol as usize);
        for y in 0..size[1] { for z in 0..size[2] { for x in 0..size[0] { let Some(b) = self.world.get(lo[0] + x, lo[1] + y, lo[2] + z) else { return "Une partie de la boîte n'est pas chargée : rapprochez-vous".into() }; cells.push(b); } } }
        let n = cells.iter().filter(|b| **b != 0).count(); self.clip = Some(Clip { size, cells }); self.clip_turn = (0, false);
        format!("Copié : {} × {} × {}, {n} blocs — visez un endroit, R pour tourner, /coller", size[0], size[1], size[2])
    }
    /// /miroir : la copie retournée, gauche contre droite.
    pub fn mirror_clip(&mut self) -> String { if self.clip.is_none() { return "Rien de copié (/copier)".into(); } self.clip_turn.1 = !self.clip_turn.1; if self.clip_turn.1 { "Copie en miroir".into() } else { "Copie à l'endroit".into() } }
    /// Ce que /coller poserait : chaque bloc de la copie (l'air n'efface rien), tourné des quarts de tour de R et du miroir,
    /// le coin de la copie sur la case visée.
    pub fn paste_plan(&self) -> Option<Vec<([i32; 3], Id)>> {
        let (c, t) = (self.clip.as_ref()?, self.target?); let m = turn_matrix(self.turn, self.clip_turn.1);
        let map = |x: i32, z: i32| (m[0][0] * x + m[1][0] * z, m[0][1] * x + m[1][1] * z);
        // le coin bas de la boîte tournée : la plus petite des quatre extrémités
        let (mut ox, mut oz) = (i32::MAX, i32::MAX); for x in [0, c.size[0] - 1] { for z in [0, c.size[2] - 1] { let (a, b) = map(x, z); ox = ox.min(a); oz = oz.min(b); } }
        let mut v = Vec::new();
        for y in 0..c.size[1] { for z in 0..c.size[2] { for x in 0..c.size[0] { let b = c.at(x, y, z); if b == 0 { continue; } let (a, w) = map(x, z);
            v.push(([t.px + a - ox, t.py + y, t.pz + w - oz], crate::structures::orient(b, m))); } } }
        Some(v)
    }
    /// /coller : la copie posée là où l'on vise (créatif).
    pub fn paste(&mut self) -> String {
        if !self.creative { return "Ces outils demandent le mode créatif (/creatif)".into(); }
        if self.clip.is_none() { return "Rien de copié : /coin1, /coin2 puis /copier".into(); }
        let Some(list) = self.paste_plan() else { return "Visez l'endroit où coller".into() };
        let ch = self.apply(&list); let n = ch.len(); self.record(ch); self.sound.place("planks");
        format!("Collé : {n} bloc(s), tourné de {} ° — /annuler pour défaire", self.turn as u32 * 90)
    }
}
