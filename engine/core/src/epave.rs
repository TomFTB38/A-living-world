//! L'Épave du navire-fantôme : un grand galion échoué sur un banc de sable, en mer, près du rivage : mâts brisés, voiles en
//! lambeaux, coque crevée sur bâbord. Une marée monte et
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
pub const BOW: i32 = -22; pub const STERN: i32 = 21; pub const HOLD: i32 = 0; pub const GUN: i32 = 5; pub const DECK: i32 = 9; pub const POOP: i32 = 14;
/// La cabine du capitaine sous la dunette (de w à w) ; sa porte (le mur du devant).
pub const CABIN: (i32, i32) = (13, 20); pub const CABIN_DOOR: [[i32; 3]; 4] = [[-1, 10, 12], [0, 10, 12], [-1, 11, 12], [0, 11, 12]];
/// La brèche (u, de w à w) ; l'écoutille de la cale (u, w : deux sur deux) ; la marée : son cycle, sa part de mer haute.
pub const BREACH: (i32, i32, i32) = (-7, -4, -2); pub const HATCH: (i32, i32) = (0, 8); pub const TIDE_PERIOD: f64 = 36.0; pub const TIDE_HIGH: f64 = 0.45;
/// Les rangs de canons (w) ; l'échelle de la proue, de l'entrepont au pont (u, w).
pub const CANNONS: [i32; 6] = [-14, -10, -6, -2, 2, 6]; pub const BOW_LADDER: (i32, i32) = (0, -17);
/// Le grand mât (u = 0) ; la vergue : sa hauteur (le bloc), sa ligne (w), le trou ; l'échelle du mât ; le levier du bout.
pub const MAST_W: i32 = 0; pub const YARD: i32 = 18; pub const YARD_W: i32 = 1; pub const YARD_GAP: i32 = 4; pub const YARD_LEVER: [i32; 3] = [7, 19, 1];
pub const BOSS: &str = "capitaine_fantome"; pub const BOSS_AT: [f64; 3] = [0.5, 10.0, 17.5]; pub const CHEST: [i32; 3] = [0, 10, 19];

/// La demi-largeur de la coque à la hauteur `w` (de la proue à la poupe) et au niveau `v` : une proue en ogive, un maître-couple
/// large, une poupe carrée ; en section, des fonds qui se rentrent vers la quille, un flanc droit, puis un peu de rentrée en haut.
pub fn half(w: i32, v: i32) -> f64 {
    if v < -1 || w < BOW || w > STERN { return 0.0; } let t = (w - BOW) as f64 / (STERN - BOW) as f64;
    let plan = if t < 0.4 { (7.6 * (t / 0.4 * std::f64::consts::FRAC_PI_2).sin().powf(0.6)).max(1.4) } else if t > 0.86 { 7.6 - (t - 0.86) / 0.14 * 1.8 } else { 7.6 };
    plan * match v { -1 => 0.3, 0 => 0.5, 1 => 0.66, 2 => 0.79, 3 => 0.89, 4 => 0.96, 5 | 6 => 1.0, _ => (1.0 - 0.035 * (v - 6) as f64).max(0.8) }
}
/// La paroi de la coque à ce niveau : |u| jusqu'à `shell_r` est de la coque ou du dedans.
pub fn shell_r(w: i32, v: i32) -> i32 { (half(w, v).round() as i32).max(1) }
/// Le dedans de la coque : à l'abri de la paroi de tous les côtés (devant, derrière, dessous).
pub fn interior(u: i32, w: i32, v: i32) -> bool {
    if w <= BOW || w >= STERN || v < 0 { return false; } let below = if v > DECK + 1 { f64::MAX } else { half(w, v - 1) };
    (u.abs() as f64) < [half(w, v), below, half(w - 1, v), half(w + 1, v)].into_iter().fold(f64::MAX, f64::min).round() - 0.5
}
/// La paroi au niveau de l'entrepont (là où sont les canons).
pub fn beam(w: i32) -> i32 { shell_r(w, GUN + 1) }
/// Le haut du bordage : il se relève vers la proue ; la dunette, à la poupe.
pub fn top(w: i32) -> i32 { if w >= STERN - 1 { POOP + 3 } else if w >= CABIN.0 { POOP + 1 } else if w <= FORE { DECK + 4 } else if w <= FORE + 3 { DECK + 2 } else { DECK + 1 } }
/// Le gaillard d'avant : un pont surélevé à la proue, jusqu'à `FORE` (w).
pub const FORE: i32 = -16;
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
    // le banc de sable où le navire s'est échoué, dans l'eau : une île basse, ses hauts-fonds tout autour ; des épaves éparses
    for u in -16..=16i32 { for w in -40..=38i32 { let e = (u as f64 / 12.0).powi(2) + (w as f64 / 31.0).powi(2);
        if e <= 1.0 { for v in -9..=-1 { g.n(u, v, w, "sand"); } for v in 0..=6 { g.air(u, v, w); } let r = g.h(u, 0, w, 1); if !(u.abs() <= 9 && w >= BOW - 1 && w <= STERN + 2) { if r < 0.05 { g.n(u, 0, w, "shell"); } else if r > 0.97 { g.n(u, 0, w, "wreck_planks"); } else if r > 0.955 { g.n(u, 0, w, "dead_bush"); } } }
        else if e <= 1.35 { for v in -9..=-2 { g.n(u, v, w, "sand"); } if g.h(u, 0, w, 2) < 0.08 { g.n(u, -1, w, "kelp"); } } } }
    for (u, w) in [(-10, -16), (9, 8), (-9, 24)] { g.n(u, 0, w, "barrel"); g.n(u + 1, 0, w, "crate"); } g.n(8, 0, -26, "anchor"); g.n(-10, 0, 2, "net_pile"); for (u, w) in [(10, -2), (-6, 28), (5, -30)] { g.n(u, 0, w, "mossy_stone"); g.n(u + 1, 0, w, "stone"); }
    // la coque : la quille, des fonds sombres, une préceinte rouge, des bordés d'épave, des œuvres mortes peintes en bleu ; des
    // planches arrachées sur tribord ; dedans, la cale, l'entrepont, le pont, la dunette
    for w in BOW..=STERN { for v in -1..=top(w) { let r = shell_r(w, v); for u in -r..=r {
        let floor = v == HOLD || v == GUN || v == DECK || (v == POOP && w >= CABIN.0) || (v == DECK + 3 && w <= FORE);
        if interior(u, w, v) { if floor { g.n(u, v, w, "planks"); } else { g.air(u, v, w); } continue; }
        if v > DECK && u.abs() < r && w > BOW && w < STERN && !(w >= CABIN.0 && v <= POOP) && !(w <= FORE && v == DECK + 3) { g.air(u, v, w); continue; }
        let hole = u > 0 && (2..=4).contains(&v) && g.h(u, v, w, 3) < 0.06 && w > BOW + 4 && w < STERN - 3;
        if hole { g.air(u, v, w); continue; }
        // les couleurs : des fonds goudronnés, une préceinte rouge, la batterie en bois d'épave, un liseré d'or, des œuvres mortes
        // bleues (les châteaux, en panneaux bleus entre des pilastres dorés), un plat-bord doré
        let castle = (w >= CABIN.0 || w <= FORE) && v > DECK;
        g.n(u, v, w, if v <= 4 { if g.h(u, v, w, 5) < 0.1 { "wreck_planks" } else { "planks_dark" } } else if v == 5 { "planks_red" } else if v == DECK - 1 || v == top(w) { "brass" } else if castle && (u.abs() == r && w.rem_euclid(3) == 0 || w == STERN && u.rem_euclid(3) == 0) { "brass" } else if v >= DECK { "planks_blue" } else if g.h(u, v, w, 4) < 0.15 { "planks_dark" } else { "wreck_planks" }); } } }
    // la brèche de bâbord, sa rampe de sable ; du varech, du corail
    let (_, b0, b1) = BREACH; for w in b0..=b1 { for v in 1..=3 { for u in -9..0 { if !interior(u, w, v) && -u <= shell_r(w, v) { g.air(u, v, w); } } } for u in -9..=-(shell_r(w, 0) + 1) { g.n(u, 0, w, "sand"); } }
    for (u, w) in [(-3, -8), (3, 3), (-2, 12), (2, -15), (2, -4)] { g.n(u, HOLD + 1, w, "kelp"); } for (u, w) in [(-3, 0), (3, -10), (3, 14)] { g.n(u, HOLD + 1, w, "coral"); }
    // la cale : des caisses, des tonneaux, des chaînes, des os ; l'écoutille de l'entrepont
    for (u, w) in [(3, -6), (3, -5), (-3, 6), (3, 10), (-2, -14), (2, 16)] { g.n(u, HOLD + 1, w, if (u + w) % 2 == 0 { "crate" } else { "barrel" }); }
    for (u, w) in [(-4, 3), (2, -9), (-1, 15)] { g.n(u, HOLD + 1, w, "bone_pile"); } for v in HOLD + 1..GUN { g.n(-3, v, -10, "great_chain"); }
    for du in 0..=1 { for dw in 0..=1 { g.air(HATCH.0 + du, GUN, HATCH.1 + dw); } }
    // l'entrepont : les canons sur leurs affûts, aux sabords des deux flancs ; l'échelle de la proue
    for v in GUN + 1..=DECK { g.t(BOW_LADDER.0, v, BOW_LADDER.1, "ladder", 0, -1); } g.n(BOW_LADDER.0, GUN + 1, BOW_LADDER.1 - 1, "planks_dark"); for v in GUN + 2..DECK { g.n(BOW_LADDER.0, v, BOW_LADDER.1 - 1, "planks_dark"); }
    for (u, w) in [(-4, -12), (4, 0), (-4, 10)] { g.n(u, GUN + 1, w, "lantern"); }
    // les mâts : chacun ses vergues, ses voiles grises en lambeaux, gonflées par un vent qui ne souffle plus ; la misaine avec sa
    // hune ; le grand mât, sa vergue basse (une planche manque, la grand-voile roulée dessous), sa hune, son hunier, son perroquet,
    // le pavillon noir ; l'artimon et sa voile latine
    let rag = |g: &G, u: i32, v: i32, w: i32| g.h(u, v, w, 7) < 0.16;
    let sheet = |g: &mut G, mw: i32, half: i32, v0: i32, v1: i32, skip0: bool| { for u in -half..=half { for v in v0..=v1 { if (skip0 && u == 0) || rag(g, u, v, mw) || (v == v0 && (u + mw).rem_euclid(3) == 0) { continue; }
        let belly = ((1.0 - (u as f64 / (half as f64 + 1.0)).powi(2)) * ((v - v0) as f64 / (v1 - v0 + 1) as f64 * std::f64::consts::PI).sin() * 2.2).round() as i32; g.n(u, v, mw + 1 + belly, "sail"); } } };
    let fm = -13; for v in HOLD + 1..DECK + 19 { g.n(0, v, fm, "spruce_log"); }
    for (vy, hw) in [(DECK + 10, 6), (DECK + 17, 4)] { for u in -hw..=hw { if u != 0 { g.n(u, vy, fm, "spruce_log_x"); } } } sheet(&mut g, fm, 5, DECK + 5, DECK + 9, false); sheet(&mut g, fm, 3, DECK + 13, DECK + 16, false);
    for du in -1..=1 { for dw in -1..=1 { if (du, dw) != (0, 0) { g.n(du, DECK + 11, fm + dw, "planks_dark"); } } }
    for v in HOLD + 1..YARD + 14 { g.n(0, v, MAST_W, "spruce_log"); }
    for u in -7..=7 { if u != 0 && u != YARD_GAP { g.n(u, YARD, YARD_W, "spruce_log_x"); } } g.t(YARD_LEVER[0], YARD_LEVER[1], YARD_LEVER[2], "lever", 0, 1);
    for v in DECK + 1..=YARD { g.t(0, v, YARD_W, "ladder", 0, -1); }
    for du in -1..=1 { for dw in -1..=1 { if (du, dw) != (0, 0) { g.n(du, YARD + 5, MAST_W + dw, "planks_dark"); g.n(du, YARD + 6, MAST_W + dw, if du == 0 || dw == 0 { "rope_rail" } else { "planks_dark" }); } } }
    for (vy, hw) in [(YARD + 9, 5), (YARD + 13, 3)] { for u in -hw..=hw { if u != 0 { g.n(u, vy, YARD_W, "spruce_log_x"); } } } sheet(&mut g, YARD_W, 4, YARD + 7, YARD + 8, true); sheet(&mut g, YARD_W, 2, YARD + 10, YARD + 12, true);
    for (du, dv) in [(1, 0), (2, 0), (3, 0), (1, -1), (2, -1), (3, -1)] { g.n(du, YARD + 14 + dv, MAST_W, "wool_black"); } g.n(0, YARD + 14, MAST_W, "lantern");
    g.draw(&sail(0));
    for v in POOP + 1..POOP + 12 { g.n(0, v, 17, "spruce_log"); } for k in 0..12 { let (v, w) = (POOP + 2 + k * 10 / 12, 22 - k); g.n(0, v, w, "spruce_log_z"); }
    for k in 0..12 { let (vt, w) = (POOP + 2 + k * 10 / 12, 22 - k); for v in POOP + 3..vt { if !rag(&g, 1, v, w) && w != 17 { g.n(1, v, w, "sail"); } } }
    // les haubans : des chaînes tendues du bordage aux hunes
    let line = |g: &mut G, a: [i32; 3], b: [i32; 3]| { let n = (b[0] - a[0]).abs().max((b[1] - a[1]).abs()).max((b[2] - a[2]).abs()).max(1); for k in 0..=n { let t = k as f64 / n as f64; let c = [a[0] + ((b[0] - a[0]) as f64 * t).round() as i32, a[1] + ((b[1] - a[1]) as f64 * t).round() as i32, a[2] + ((b[2] - a[2]) as f64 * t).round() as i32]; if c[0] != 0 && (c[1] > top(c[2]) || c[0].abs() > shell_r(c[2], c[1])) { g.n(c[0], c[1], c[2], "chain"); } } };
    for (mw, mv) in [(fm, DECK + 10), (MAST_W, YARD + 4), (17, POOP + 7)] { for s in [-1, 1] { for dw in [-3, -1, 1] { let w = mw + dw; let v0 = top(w) + 1; let b = shell_r(w, v0 - 1); line(&mut g, [s * b, v0, w], [s, mv, mw]); } } }
    // les étais : du beaupré à la misaine, de la misaine au grand mât, du grand mât à l'artimon
    line(&mut g, [1, DECK + 7, BOW - 8], [1, DECK + 17, fm]); line(&mut g, [1, DECK + 17, fm], [1, YARD + 9, MAST_W]); line(&mut g, [1, YARD + 9, MAST_W], [1, POOP + 10, 17]);
    // le bastingage du pont, le gaillard d'avant, la dunette et son escalier, la barre ; la cabine (sa porte barrée)
    for w in FORE + 1..CABIN.0 { let b = shell_r(w, DECK + 1); for s in [-1, 1] { if (w + s).rem_euclid(3) != 0 { g.n(s * (b - 1), DECK + 1, w, "rope_rail"); } } }
    for u in -3..=3i32 { if u.abs() >= 2 { g.n(u, DECK + 4, FORE, "rope_rail"); } }
    for k in 0..5 { for u in [-5, 5] { g.t(u, DECK + 1 + k, CABIN.0 - 5 + k, "planks_stairs", 0, 1); for v in DECK + 1..=DECK + k { g.n(u, v, CABIN.0 - 5 + k, "planks"); } } }
    for u in -6..=6 { for v in DECK + 1..POOP { if interior(u, CABIN.0 - 1, v) || u.abs() < shell_r(CABIN.0 - 1, v) { g.n(u, v, CABIN.0 - 1, if v == DECK + 4 { "brass" } else if (u + 7).rem_euclid(3) == 0 { "planks_dark" } else { "planks_blue" }); } } } for c in CABIN_DOOR { g.n(c[0], c[1], c[2], "great_chain"); }
    for u in [-3, 3] { g.n(u, DECK + 1, CABIN.0 - 2, "lantern"); }
    g.n(0, POOP + 1, 19, "spinning_wheel"); for (u, w) in [(-4, 14), (4, 14), (-3, 19), (3, 19)] { g.n(u, DECK + 1, w, "lantern"); } g.n(CHEST[0], CHEST[1], CHEST[2], "chest"); g.n(2, DECK + 1, 18, "barrel"); g.n(-2, DECK + 1, 18, "fish_crate");
    // la poupe : le tableau et ses deux rangs de fenêtres, le balcon, la couronne dorée et ses trois fanaux ; les bouteilles de poupe
    for u in -4..=4i32 { for v in [DECK + 1, DECK + 2, DECK + 4] { if u.abs() % 2 == 1 { g.n(u, v, STERN, "stained_glass"); } } g.n(u, DECK, STERN + 1, "planks_dark"); g.n(u, DECK + 1, STERN + 1, if u.abs() == 4 || u % 2 == 0 { "rope_rail" } else { "air" }); }
    for u in -3..=3 { g.n(u, POOP + 4, STERN, if u.abs() == 3 { "lantern" } else { "brass" }); } g.n(0, POOP + 5, STERN, "brass"); g.n(0, POOP + 6, STERN, "lantern");
    for s in [-1, 1] { for w in 15..=19 { let b = shell_r(w, DECK + 2); for v in DECK..=POOP + 1 { g.n(s * (b + 1), v, w, if v == DECK || v == POOP + 1 { "brass" } else if (w == 16 || w == 18) && v < POOP { "stained_glass" } else { "planks_blue" }); } } }
    // la proue : la figure dorée, le beaupré, les ancres aux bossoirs ; les sabords et leurs canons ; le gouvernail
    for k in 0..12 { let c = [0, DECK + 2 + k / 2, BOW - k]; g.n(c[0], c[1], c[2], "spruce_log_z"); } for k in 0..5 { for v in DECK + 3..DECK + 3 + k { if !rag(&g, 1, v, BOW - 7 + k) { g.n(1, v, BOW - 7 + k, "sail"); } } }
    for v in DECK - 3..=DECK + 2 { g.n(0, v, BOW - 1, "brass"); } g.n(0, DECK + 3, BOW - 2, "brass"); g.n(0, DECK + 1, BOW - 2, "brass");
    for s in [-1, 1] { g.n(s * (shell_r(BOW + 4, DECK - 1) + 1), DECK - 1, BOW + 4, "anchor"); }
    for w in CANNONS { for s in [-1, 1] { let b = beam(w); g.n(s * b, GUN + 1, w, "cannon"); g.air(s * b, GUN + 2, w); g.n(s * (b - 1), GUN + 1, w + 1, "planks_dark"); g.n(s * (b - 1), GUN + 1, w - 1, "planks_dark"); g.n(s * b, GUN + 3, w, "planks_red"); } }
    for w in (-12..=10).step_by(4) { for s in [-1, 1] { let b = shell_r(w + 2, DECK + 1); g.n(s * b, DECK + 1, w + 2, "planks_red"); } }
    for v in -1..=DECK - 2 { g.n(0, v, STERN + 1, "planks_dark"); }
}

/// Les mécanismes de l'épave bâtie en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] + 1; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let ids = |l: Vec<([i32; 3], &'static str)>| l.into_iter().map(|(c, b)| (at(c), by_name(b).unwrap_or(0))).collect::<Vec<_>>();
    let mut m = Mech::default();
    m.tides.push(Tide { name: name("maree"), period: TIDE_PERIOD, high: TIDE_HIGH, near: at([0, 0, 0]), ..Default::default() });
    // la cale : toutes ses cases libres, du plancher jusque sous l'entrepont
    let cols: Vec<[i32; 3]> = (BOW + 1..STERN).flat_map(|w| (-8..=8).map(move |u| (u, w))).filter_map(|(u, w)| (HOLD + 1..GUN).find(|v| interior(u, w, *v)).map(|v| { let c = at([u, v, w]); [c[0], c[2], c[1]] })).collect();
    m.pools.push(Pool { cols, top: f + GUN - 1, by: name("maree"), level: f + HOLD, ..Default::default() });
    for (k, w) in CANNONS.iter().enumerate() { m.pulses.push(Pulse { darts: [-1, 1].iter().map(|s| at([s * beam(*w), GUN + 1, *w])).collect(), period: 3.2, up: 0.0, phase: k as f64 * 0.55, ..Default::default() }); }
    m.dials.push(Dial { name: name("voile"), n: 2, state: 0, goal: 1, draw: (0..2).map(|k| ids(sail(k))).collect(), levers: vec![(at(YARD_LEVER), 1)], ..Default::default() });
    let chain = by_name("great_chain").unwrap_or(0);
    m.wires.push(Wire { name: name("cabine"), needs: vec![name("voile")], latch: true, doors: CABIN_DOOR.iter().map(|c| (at(*c), chain)).collect(), ..Default::default() });
    m.guards.push(Guard { min: at([-6, DECK + 1, CABIN.0]), max: at([6, POOP - 1, CABIN.1]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: CABIN_DOOR.iter().map(|c| (at(*c), chain)).collect(), state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-10, -1, BOW - 10]), max: at([10, YARD + 11, STERN + 2]), until: String::new() });
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
