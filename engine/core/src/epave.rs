//! L'Épave du navire-fantôme : un grand galion échoué dans le sable, mâts brisés, coque crevée sur bâbord. Une marée monte et
//! descend dans sa cale. Trois épreuves, un boss :
//! - **la cale et la marée** : on entre par la brèche, à marée basse. L'écoutille qui mène à l'entrepont est trop haute ; il
//!   faut attendre que la marée remplisse la cale et nager jusqu'à elle (le nouveau mécanisme : la marée, un fil qui
//!   s'enclenche tout seul, à intervalles réguliers).
//! - **l'entrepont des canons** : de part et d'autre, des canons fantômes tirent leurs boulets en travers du pont, rang après
//!   rang ; on file entre deux salves, à l'abri des mâts, jusqu'à l'échelle de la proue.
//! - **la grand-vergue** : sur le pont, on grimpe au grand mât, puis on s'avance sur la vergue (elle a perdu une planche : un
//!   saut) jusqu'au levier du bout. La grand-voile se déploie, le vent la gonfle, la coque craque : la porte de la cabine
//!   cède.
//! - **le Capitaine** attend dans sa cabine, sous la dunette, devant son coffre.
//! Tout est vérifié par `tests/epave.rs`.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Dial, Guard, Hint, Mech, Pool, Pulse, Seal, Tide, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// La coque : de la proue (w) à la poupe ; les ponts : le sol de la cale, l'entrepont, le pont, la dunette (le bloc).
pub const BOW: i32 = -22; pub const STERN: i32 = 21; pub const HOLD: i32 = 0; pub const GUN: i32 = 5; pub const DECK: i32 = 9; pub const POOP: i32 = 13;
/// La cabine du capitaine sous la dunette (de w à w) ; sa porte (le mur du devant).
pub const CABIN: (i32, i32) = (13, 20); pub const CABIN_DOOR: [[i32; 3]; 4] = [[-1, 10, 12], [0, 10, 12], [-1, 11, 12], [0, 11, 12]];
/// La brèche (u, de w à w) ; l'écoutille de la cale (u, w : deux sur deux) ; la marée : son cycle, sa part de mer haute.
pub const BREACH: (i32, i32, i32) = (-7, -4, -2); pub const HATCH: (i32, i32) = (0, 8); pub const TIDE_PERIOD: f64 = 36.0; pub const TIDE_HIGH: f64 = 0.45;
/// Les rangs de canons (w) ; l'échelle de la proue, de l'entrepont au pont (u, w).
pub const CANNONS: [i32; 6] = [-14, -10, -6, -2, 2, 6]; pub const BOW_LADDER: (i32, i32) = (0, -17);
/// Le grand mât (u = 0) ; la vergue : sa hauteur (le bloc), sa ligne (w), le trou ; l'échelle du mât ; le levier du bout.
pub const MAST_W: i32 = 0; pub const YARD: i32 = 18; pub const YARD_W: i32 = 1; pub const YARD_GAP: i32 = 4; pub const YARD_LEVER: [i32; 3] = [7, 19, 1];
pub const BOSS: &str = "capitaine_fantome"; pub const BOSS_AT: [f64; 3] = [0.5, 10.0, 17.5]; pub const CHEST: [i32; 3] = [0, 10, 19];

/// La demi-largeur de la coque (sa paroi) à la hauteur de `w` : fine à la proue, carrée à la poupe.
pub fn beam(w: i32) -> i32 { if w < -12 { 2 + (w - BOW) / 2 } else if w > 14 { 7 - (w - 14) * 2 / 7 } else { 7 } }
/// La grand-voile : roulée sous la vergue (0), ou déployée (1).
pub fn sail(k: u8) -> Vec<([i32; 3], &'static str)> {
    let mut o = Vec::new(); for u in -6..=6i32 { if k == 0 { o.push(([u, YARD - 1, YARD_W + 1], "sail")); } else { for v in DECK + 3..YARD { let belly = u.abs() < 4 && v > DECK + 4 && v < YARD - 2; o.push(([u, v, YARD_W + 1 + belly as i32], "sail")); } } } o
}

struct G<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl G<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn h(&self, u: i32, v: i32, w: i32, k: i32) -> f64 { hash_float(self.x + u + 331 * v, self.z + w, 5309, k) }
    fn draw(&mut self, l: &[([i32; 3], &'static str)]) { for (c, b) in l { self.n(c[0], c[1], c[2], b); } }
}

/// Pose l'épave dans le plan : `o` est le sol au centre (le plancher de la cale est un bloc plus haut).
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut g = G { p, x: o[0], f: o[1] + 1, z: o[2] };
    // la grève : du sable, des coquillages, des épaves éparses, une ancre plantée, des tonneaux
    for u in -14..=14i32 { for w in -30..=28i32 { g.n(u, -1, w, "sand"); for v in 0..=4 { g.air(u, v, w); } let r = g.h(u, 0, w, 1); if u.abs() > 9 || w < BOW - 2 || w > STERN + 2 { if r < 0.05 { g.n(u, 0, w, "shell"); } else if r > 0.97 { g.n(u, 0, w, "wreck_planks"); } else if r > 0.955 { g.n(u, 0, w, "kelp"); } } } }
    for (u, w) in [(-11, -18), (10, 8), (-10, 24)] { g.n(u, 0, w, "barrel"); g.n(u + 1, 0, w, "crate"); } g.n(9, 0, -26, "anchor"); g.n(-12, 0, 2, "net_pile");
    // la coque : la quille, les flancs de bois d'épave (des bordés plus sombres), la cale, l'entrepont, le pont ; tout le dedans vide
    for w in BOW..=STERN { let b = beam(w); let bp = beam((w - 1).max(BOW)).max(beam((w + 1).min(STERN)));
        for u in -b.max(bp)..=b.max(bp) { let side = u.abs() >= b || w == BOW || w == STERN; let top = if w >= CABIN.0 { POOP + 1 } else { DECK + 1 };
            for v in -1..=top { let shell = side && u.abs() <= b.max(bp); let floor = v == -1 || v == HOLD || v == GUN || v == DECK || (v == POOP && w >= CABIN.0);
                if u.abs() > b && !shell { continue; }
                if shell { g.n(u, v, w, if v % 4 == 3 { "planks_dark" } else { "wreck_planks" }); } else if floor { g.n(u, v, w, if v == -1 { "wreck_planks" } else { "planks" }); } else { g.air(u, v, w); } } } }
    // la brèche de bâbord, la rampe de sable ; quelques planches arrachées, du varech
    let (bu, b0, b1) = BREACH; for w in b0..=b1 { for v in 1..=3 { g.air(bu, v, w); g.air(bu - 1, v, w); } g.n(bu - 1, 0, w, "sand"); } g.n(bu - 1, 1, b0 - 1, "wreck_planks");
    for (u, w) in [(-5, -8), (4, 3), (-2, 12), (5, -15), (2, -4)] { g.n(u, HOLD + 1, w, "kelp"); } for (u, w) in [(-4, 0), (3, -10), (5, 14)] { g.n(u, HOLD + 1, w, "coral"); }
    // la cale : des caisses, des tonneaux, des chaînes, des os ; l'écoutille de l'entrepont
    for (u, w) in [(5, -6), (5, -5), (-5, 6), (4, 10), (-3, -14), (3, 16)] { g.n(u, HOLD + 1, w, if (u + w) % 2 == 0 { "crate" } else { "barrel" }); }
    for (u, w) in [(-4, 3), (2, -9), (-1, 15)] { g.n(u, HOLD + 1, w, "bone_pile"); } for v in HOLD + 1..GUN { g.n(-6, v, -10, "great_chain"); }
    for du in 0..=1 { for dw in 0..=1 { g.air(HATCH.0 + du, GUN, HATCH.1 + dw); } }
    // l'entrepont : les canons sur leurs affûts, aux sabords des deux flancs ; l'échelle de la proue
    for w in CANNONS { for s in [-1, 1] { let b = beam(w); g.n(s * b, GUN + 1, w, "cannon"); g.air(s * b, GUN + 2, w); g.n(s * (b - 1), GUN + 1, w + 1, "planks_dark"); g.n(s * (b - 1), GUN + 1, w - 1, "planks_dark"); } }
    for v in GUN + 1..=DECK { g.t(BOW_LADDER.0, v, BOW_LADDER.1, "ladder", 0, -1); } g.n(BOW_LADDER.0, GUN + 1, BOW_LADDER.1 - 1, "planks_dark"); for v in GUN + 2..DECK { g.n(BOW_LADDER.0, v, BOW_LADDER.1 - 1, "planks_dark"); }
    for (u, w) in [(-4, -12), (4, 0), (-4, 10)] { g.n(u, GUN + 1, w, "lantern"); }
    // les mâts : le mât de misaine, brisé ; le grand mât, sa vergue (une planche manque), son échelle, sa hune ; le mât d'artimon
    for v in HOLD + 1..DECK + 9 { g.n(0, v, -12, "spruce_log"); } for v in HOLD + 1..YARD + 9 { g.n(0, v, MAST_W, "spruce_log"); } for v in POOP + 1..POOP + 8 { g.n(0, v, 17, "spruce_log"); }
    for u in -7..=7 { if u != 0 && u != YARD_GAP { g.n(u, YARD, YARD_W, "spruce_log_x"); } } g.t(YARD_LEVER[0], YARD_LEVER[1], YARD_LEVER[2], "lever", 0, 1);
    for v in DECK + 1..=YARD { g.t(0, v, YARD_W, "ladder", 0, -1); }
    for du in -1..=1 { for dw in -1..=1 { if (du, dw) != (0, 0) { g.n(du, YARD + 5, MAST_W + dw, "planks_dark"); g.n(du, YARD + 6, MAST_W + dw, if du == 0 || dw == 0 { "rope_rail" } else { "planks_dark" }); } } }
    for v in DECK + 3..DECK + 8 { g.n(0, v, -12, "spruce_log"); } g.n(1, DECK + 8, -11, "spruce_log_z"); g.n(1, DECK + 8, -10, "spruce_log_z");
    g.draw(&sail(0));
    // le bastingage du pont, la dunette et son escalier, la barre ; la cabine (sa porte barrée), ses fenêtres de poupe
    for w in BOW..CABIN.0 { let b = beam(w); for s in [-1, 1] { if (w + s).rem_euclid(3) != 0 { g.n(s * (b - 1), DECK + 1, w, "rope_rail"); } } }
    for k in 0..4 { for u in [-5, 5] { g.t(u, DECK + 1 + k, CABIN.0 - 4 + k, "planks_stairs", 0, 1); for v in DECK + 1..=DECK + k { g.n(u, v, CABIN.0 - 4 + k, "planks"); } } }
    for u in -6..=6 { for v in DECK + 1..POOP { g.n(u, v, CABIN.0 - 1, if v == DECK + 3 { "planks_dark" } else { "wreck_planks" }); } } for c in CABIN_DOOR { g.n(c[0], c[1], c[2], "great_chain"); }
    for u in -4..=4 { if u % 2 == 0 { g.n(u, DECK + 2, STERN, "stained_glass"); } } g.n(0, POOP + 1, 19, "spinning_wheel"); g.n(0, POOP + 1, STERN - 1, "lantern");
    for (u, w) in [(-4, 14), (4, 14), (-4, 19), (4, 19)] { g.n(u, DECK + 1, w, "lantern"); } g.n(CHEST[0], CHEST[1], CHEST[2], "chest"); g.n(3, DECK + 1, 18, "barrel"); g.n(-3, DECK + 1, 18, "fish_crate");
    // le beaupré, à la proue ; le gouvernail, à la poupe
    for k in 0..6 { g.n(0, DECK + 1 + k / 2, BOW - 1 - k, "spruce_log_z"); } for v in -1..=DECK - 2 { g.n(0, v, STERN + 1, "planks_dark"); }
}

/// Les mécanismes de l'épave bâtie en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] + 1; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let ids = |l: Vec<([i32; 3], &'static str)>| l.into_iter().map(|(c, b)| (at(c), by_name(b).unwrap_or(0))).collect::<Vec<_>>();
    let mut m = Mech::default();
    m.tides.push(Tide { name: name("maree"), period: TIDE_PERIOD, high: TIDE_HIGH, near: at([0, 0, 0]), ..Default::default() });
    // la cale : toutes ses cases libres, du plancher jusque sous l'entrepont
    let cols: Vec<[i32; 3]> = (BOW + 1..STERN).flat_map(|w| (-beam(w) + 1..beam(w)).map(move |u| (u, w))).map(|(u, w)| { let c = at([u, 0, w]); [c[0], c[2], f + HOLD + 1] }).collect();
    m.pools.push(Pool { cols, top: f + GUN - 1, by: name("maree"), level: f + HOLD, ..Default::default() });
    for (k, w) in CANNONS.iter().enumerate() { m.pulses.push(Pulse { darts: [-1, 1].iter().map(|s| at([s * beam(*w), GUN + 1, *w])).collect(), period: 3.2, up: 0.0, phase: k as f64 * 0.55, ..Default::default() }); }
    m.dials.push(Dial { name: name("voile"), n: 2, state: 0, goal: 1, draw: (0..2).map(|k| ids(sail(k))).collect(), levers: vec![(at(YARD_LEVER), 1)], ..Default::default() });
    let chain = by_name("great_chain").unwrap_or(0);
    m.wires.push(Wire { name: name("cabine"), needs: vec![name("voile")], latch: true, doors: CABIN_DOOR.iter().map(|c| (at(*c), chain)).collect(), ..Default::default() });
    m.guards.push(Guard { min: at([-6, DECK + 1, CABIN.0]), max: at([6, POOP - 1, CABIN.1]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: CABIN_DOOR.iter().map(|c| (at(*c), chain)).collect(), state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-9, -1, BOW - 8]), max: at([9, YARD + 9, STERN + 1]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-6, 1, BREACH.1 - 2], [-4, 3, BREACH.2 + 2], "L'Épave du navire-fantôme. La cale est sèche, mais le varech pend jusqu'aux barrots : la marée monte jusqu'ici. L'écoutille est bien haute…");
    hint(&mut m, [-2, GUN + 1, HATCH.1 - 1], [2, GUN + 3, HATCH.1 + 2], "L'entrepont. Les canons fantômes tirent en travers, rang après rang. Filez entre deux salves jusqu'à l'échelle de la proue.");
    hint(&mut m, [-3, DECK + 1, BOW_LADDER.1 - 1], [3, DECK + 3, BOW_LADDER.1 + 1], "Le pont. La porte de la cabine est bloquée. Là-haut, au bout de la grand-vergue, un levier libère la grand-voile.");
    m
}

impl Session {
    /// Bâtit ici l'épave (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol au centre.
    pub fn build_wreck(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("n{}:{}:", o[0], o[2]); if !self.mech.dials.iter().any(|d| d.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Une épave de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn wreck_wake(&mut self, pl: &Place) { let tag = format!("n{}:{}:", pl.x, pl.z); if self.mech.dials.iter().any(|d| d.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}
