//! Les camps de brigands grandissent. Laissé à lui-même, un camp gagne en puissance (d'un cran tous les cinq jours, jusqu'à cinq) ;
//! chaque pillage réussi le renforce, chaque bande perdue l'affaiblit. Sa puissance fait tout : combien d'hommes gardent le camp et
//! combien sont des vétérans, combien de nuits ses bandes sortent, combien d'hommes elles comptent, et la force de leurs coups sur
//! une herse ou une muraille. Le camp qui menace un village est le plus proche à moins de mille pas.
//! Pour l'arrêter, il faut aller le détruire : quand tous ses gardiens sont tombés (le chef compris), il est dispersé — plus aucune
//! bande pendant dix jours, puis il se reforme, petit.
use crate::player::Voxels;
use crate::session::Session;
use crate::sites::{Lieu, Place};

/// L'état d'un camp : sa puissance (0 dispersé, de 1 à 5), quand on l'a mise à jour (heures de jeu), jusqu'à quand il reste dispersé,
/// et combien de ses gardiens sont encore debout depuis que le joueur l'a réveillé.
#[derive(Clone, Debug)]
pub struct Camp { pub power: f64, pub t: f64, pub down_until: f64, pub left: u32, /** le cran que montre le camp (ce qu'on y a bâti) */ pub shown: u32 }
pub const MAX_POWER: f64 = 5.0;
const DAYS_PER_LEVEL: f64 = 5.0; const DOWN_DAYS: f64 = 10.0;
/// Où un camp vise : le plus proche d'un village, à moins de mille pas.
pub const REACH: f64 = 1000.0;

impl Camp {
    pub fn new(t: f64) -> Camp { Camp { power: 1.0, t, down_until: -1.0, left: 0, shown: 1 } }
    /// Le temps passe : la puissance monte ; un camp dispersé se reforme, petit, au bout de dix jours.
    pub fn grow(&mut self, now: f64) {
        if self.down_until > 0.0 { if now < self.down_until { self.t = now; return; } self.down_until = -1.0; self.power = 1.0; self.t = now; return; }
        self.power = (self.power + (now - self.t).max(0.0) / (24.0 * DAYS_PER_LEVEL)).min(MAX_POWER); self.t = now;
    }
    pub fn down(&self) -> bool { self.down_until > 0.0 }
    /// Le cran affiché : 1 à 5, ou 0 s'il est dispersé.
    pub fn level(&self) -> u32 { if self.down() { 0 } else { self.power.floor().clamp(1.0, MAX_POWER) as u32 } }
}
/// Les gardiens du camp, selon sa puissance : de six hommes à douze ; le chef, et d'autres vétérans à partir du cran trois.
pub fn garrison(pl: &Place, level: u32) -> Vec<(&'static str, [f64; 3], bool)> {
    let base = pl.guards(); let n = match level { 0 => 0, 1 => 6, 2 => 7, 3 => 8, 4 => 10, _ => 12 };
    let mut o: Vec<(&'static str, [f64; 3], bool)> = base.iter().take(n.min(base.len())).enumerate().map(|(k, g)| (g.0, g.1, k == Place::CHIEF)).collect();
    // les renforts : autour du feu, en cercle
    let mut k = 0; while o.len() < n { let a = k as f64 * 1.3; o.push(("brigand", [pl.x as f64 + 0.5 + a.cos() * 3.5, pl.y as f64 + 1.05, pl.z as f64 + 0.5 + a.sin() * 3.5], false)); k += 1; }
    for (i, g) in o.iter_mut().enumerate() { if i > 0 && (i as u32) < level.saturating_sub(1) { g.2 = true; } } o
}

impl Session {
    /// Le camp de cette maille, mis à l'heure : il grandit depuis la naissance du monde, qu'on l'ait vu ou non.
    pub(crate) fn camp(&mut self, cell: (i32, i32)) -> &mut Camp { let now = self.hours; let c = self.camps.entry(cell).or_insert_with(|| Camp::new(0.0)); c.grow(now); c }
    /// Le camp qui menace un village (le plus proche à moins de mille pas), et sa puissance (0 s'il est dispersé).
    pub fn threat(&mut self, v: &crate::village::Village) -> Option<(Place, f64)> {
        let pl = self.gen.places_near(v.c[0] as f64, v.c[2] as f64, REACH).into_iter().filter(|p| p.kind == Lieu::Camp).min_by_key(|p| (p.x - v.c[0]).pow(2) + (p.z - v.c[2]).pow(2))?;
        let c = self.camp(pl.cell); let p = if c.down() { 0.0 } else { c.power }; Some((pl, p))
    }
    /// Ce que le village sait de la menace (sans rien changer) : seulement sa force, pas où se cache le camp — à chacun de le
    /// trouver. « Menace : un camp de brigands rôde dans la région · puissance 3/5 ».
    pub fn threat_line(&self, v: &crate::village::Village) -> Option<String> {
        let pl = self.gen.places_near(v.c[0] as f64, v.c[2] as f64, REACH).into_iter().filter(|p| p.kind == Lieu::Camp).min_by_key(|p| (p.x - v.c[0]).pow(2) + (p.z - v.c[2]).pow(2))?;
        let mut c = self.camps.get(&pl.cell).cloned().unwrap_or_else(|| Camp::new(0.0)); c.grow(self.hours);
        Some(if c.down() { format!("Menace : le camp de brigands de la région a été dispersé ; il se reformera dans {:.0} jours", ((c.down_until - self.hours) / 24.0).ceil()) }
            else { format!("Menace : un camp de brigands rôde dans la région · puissance {}/5", c.level()) })
    }
    /// Le camp montre sa puissance : ce qu'il a bâti en grandissant apparaît (ou disparaît, s'il a faibli) ; dispersé, son feu
    /// est éteint. Rien ne change tant que le joueur n'est pas là pour le voir (on le rattrape à son arrivée).
    pub(crate) fn camp_show(&mut self, pl: &crate::sites::Place) {
        let lv = self.camp(pl.cell).level(); let was = self.camps[&pl.cell].shown; if lv == was { return; }
        let f = self.gen.camp_fire(pl); if self.world.get(f[0], f[1], f[2]).is_none() || self.world.get(pl.x + 10, pl.y, pl.z + 10).is_none() || self.world.get(pl.x - 10, pl.y, pl.z - 10).is_none() { return; }
        let mut o = Vec::new();
        for l in (lv + 1..=was).rev() { for b in self.gen.camp_layer(pl, l) { o.push((b.0, b.1, b.2, 0)); } }
        for l in was.max(1) + 1..=lv { o.extend(self.gen.camp_layer(pl, l)); }
        let fire = crate::blocks::by_name(if lv == 0 { "campfire_out" } else { "campfire" }).unwrap_or(0); if lv == 0 || was == 0 { o.push((f[0], f[1], f[2], fire)); }
        self.world.edit_many(&o); if let Some(c) = self.camps.get_mut(&pl.cell) { c.shown = lv; } self.dirty = true;
    }
    /// Un homme du camp est tombé ; quand tous ceux qu'on a réveillés sont à terre, le camp est dispersé.
    pub(crate) fn camp_kill(&mut self, cell: (i32, i32)) {
        let now = self.hours; let Some(c) = self.camps.get_mut(&cell) else { return }; c.left = c.left.saturating_sub(1);
        if c.left == 0 && !c.down() { c.down_until = now + 24.0 * DOWN_DAYS; c.power = 0.0; self.dirty = true;
            self.notes.push("Le camp de brigands est dispersé ! Les villages alentour seront tranquilles un temps.".into()); self.sound.pickup(); }
    }
    /// Une bande revient : un pillage réussi renforce son camp d'un demi-cran, une bande perdue l'affaiblit d'autant.
    pub(crate) fn camp_news(&mut self, cell: Option<(i32, i32)>, won: bool) {
        let Some(cell) = cell else { return }; let c = self.camp(cell); if c.down() { return; } c.power = (c.power + if won { 0.5 } else { -0.5 }).clamp(1.0, MAX_POWER);
    }
    pub(crate) fn camps_save(&self) -> String { self.camps.iter().map(|(k, c)| format!("camp {} {} {:.3} {:.2} {:.2} {}\n", k.0, k.1, c.power, c.t, c.down_until, c.shown)).collect() }
    pub(crate) fn camps_load(&mut self, a: &[&str]) {
        let n = |i: usize| a.get(i).and_then(|x| x.parse::<f64>().ok());
        if let (Some(x), Some(z), Some(p), Some(t), Some(d)) = (n(0), n(1), n(2), n(3), n(4)) { self.camps.insert((x as i32, z as i32), Camp { power: p, t, down_until: d, left: 0, shown: a.get(5).and_then(|x| x.parse().ok()).unwrap_or(1) }); }
    }
}
