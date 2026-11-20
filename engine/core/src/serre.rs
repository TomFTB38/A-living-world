//! La Serre empoisonnée : une immense serre à l'ancienne, verrières sur un socle de brique, voûte de nervures blanches dont les
//! carreaux sont tombés, au milieu d'un jardin retourné à la friche. Dedans, trois épreuves et un boss :
//! - **la salle des spores** : des champignons géants y crachent des spores qui brûlent la gorge (on perd de la vie tant qu'on
//!   y respire). Deux leviers d'aération, chacun au sommet d'un champignon géant qu'on gravit de chapeau en chapeau ; les deux
//!   abaissés, les spores retombent et la grille de la nef se lève. Il faut faire vite.
//! - **la liane** : au milieu de la nef, une souche. Poussez le bloc de pierre jusqu'à la dalle d'arrosage : la liane jaillit et
//!   grimpe en spirale autour de son tronc jusqu'à la canopée (le nouveau mécanisme : ce qui pousse tant qu'on l'arrose, et se
//!   flétrit sinon).
//! - **le pont de ronces** : de la canopée, un levier fait pousser un pont de ronces au-dessus de la nef, jusqu'à la terrasse
//!   du nord ; huit secondes, puis il se flétrit. Il faut courir.
//! - **la Mère-Racine** attend sur la terrasse, devant le coffre.
//! Tout est vérifié par `tests/serre.rs`.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Dial, Guard, Hint, Home, Mech, Miasma, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// La serre : demi-largeur (u, murs compris), murs du nord et du sud ; la cloison entre la salle des spores et la nef ; le haut
/// des verrières ; le sol des terrasses (la canopée au sud, la terrasse du nord au-dessus de la salle des spores).
pub const HALF: i32 = 10; pub const NORTH: i32 = -16; pub const SOUTH: i32 = 16; pub const MID: i32 = -5; pub const WALL_TOP: i32 = 16; pub const DECK: i32 = 12;
/// La canopée du sud commence à `CANOPY` (w) ; les leviers d'aération ; les spores (une bouffée par seconde).
pub const CANOPY: i32 = 9; pub const VENTS: [[i32; 3]; 2] = [[7, 7, -8], [-7, 6, -12]]; pub const SPORES: f64 = 4.0;
/// Les deux escaliers de champignons (pied, chapeau) et les deux grands chapeaux (centre, hauteur du dessus).
pub const STEPS_E: [(i32, i32, i32); 5] = [(8, -14, 1), (8, -13, 2), (8, -12, 3), (8, -11, 4), (8, -10, 5)]; pub const CAP_E: (i32, i32, i32) = (7, -8, 6);
pub const STEPS_W: [(i32, i32, i32); 4] = [(-8, -7, 1), (-8, -8, 2), (-8, -9, 3), (-8, -10, 4)]; pub const CAP_W: (i32, i32, i32) = (-7, -12, 5);
/// La liane : son tronc (u, w) ; le bloc à pousser, sa dalle d'arrosage.
pub const TRUNK: (i32, i32) = (0, 7); pub const BLOCK: [i32; 3] = [-6, 1, -1]; pub const PLATE: [i32; 3] = [-2, 1, 4];
/// Le pont de ronces : sa ligne (u), ses cases (w, de la canopée vers le nord) ; ses deux leviers ; combien de temps il tient.
pub const BRIDGE_U: i32 = -6; pub const BRIDGE: (i32, i32) = (8, -4); pub const BRIDGE_LEVERS: [[i32; 3]; 2] = [[-5, 13, 10], [-5, 13, -6]]; pub const BRIDGE_HOLD: f64 = 8.0;
pub const BOSS: &str = "mere_racine"; pub const BOSS_AT: [f64; 3] = [0.5, 13.0, -10.5]; pub const CHEST: [i32; 3] = [0, 13, -14];

/// Les marches de la liane, dans l'ordre où elles poussent : une spirale autour du tronc, d'une marche par bloc de hauteur.
pub fn liana_steps() -> Vec<[i32; 3]> {
    let ring = [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];
    (0..DECK).map(|k| { let (du, dw) = ring[((k + 2) % 8) as usize]; [TRUNK.0 + du, k + 1, TRUNK.1 + dw] }).collect()
}
/// La liane quand `k` marches ont poussé.
pub fn liana(k: u8) -> Vec<([i32; 3], &'static str)> { liana_steps().into_iter().take(k as usize).map(|c| (c, "liana")).collect() }
/// Le pont de ronces quand `k` cases ont poussé (de la canopée vers le nord).
pub fn bridge(k: u8) -> Vec<([i32; 3], &'static str)> { (0..k as i32).map(|i| ([BRIDGE_U, DECK, BRIDGE.0 - i], "liana")).collect() }
pub fn bridge_len() -> u8 { (BRIDGE.0 - BRIDGE.1 + 1) as u8 }

struct G<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl G<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn h(&self, u: i32, v: i32, w: i32, k: i32) -> f64 { hash_float(self.x + u + 331 * v, self.z + w, 6007, k) }
    /// Un champignon : un pied de `h` blocs, un chapeau carré de demi-largeur `r` (ou d'un bloc).
    fn mushroom(&mut self, u: i32, w: i32, h: i32, r: i32, cap: &'static str) { for v in 1..h { self.n(u, v, w, "mushroom_stem"); } for du in -r..=r { for dw in -r..=r { if r <= 1 || du.abs() + dw.abs() < 2 * r { self.n(u + du, h, w + dw, cap); } } } if r > 0 { self.n(u, h - 1, w, "mushroom_gills"); } }
}

/// Pose la serre dans le plan : `o` est le sol au centre (le dallage est un bloc plus haut).
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut g = G { p, x: o[0], f: o[1] + 1, z: o[2] };
    // le jardin retourné à la friche : une allée dallée, des haies, des fleurs, des buissons morts
    for u in -14..=14i32 { for w in -21..=20i32 { let path = u.abs() <= 1 && w < NORTH; g.n(u, -1, w, if path { "stone_bricks" } else if g.h(u, 0, w, 1) < 0.15 { "mossy_stone" } else { "grass" }); for v in 0..=4 { g.air(u, v, w); }
        if !path && (u.abs() > HALF + 1 || w < NORTH - 1 || w > SOUTH + 1) { let r = g.h(u, 0, w, 2); if (u.abs() == 14 || w == 20 || w == -21) && !(u.abs() <= 1) { g.n(u, 0, w, "oak_leaves"); g.n(u, 1, w, "oak_leaves"); } else if r < 0.12 { g.n(u, 0, w, if r < 0.04 { "pink_flower" } else if r < 0.08 { "yellow_flower" } else { "fern" }); } else if r > 0.97 { g.n(u, 0, w, "dead_bush"); } } } }
    // le socle de brique, les verrières sur leurs montants de fer, la grande voûte de nervures
    for u in -HALF..=HALF { for w in NORTH..=SOUTH { let edge = u.abs() == HALF || w == NORTH || w == SOUTH;
        for v in 0..=WALL_TOP + 9 { if edge && v <= WALL_TOP { let post = (if u.abs() == HALF { w } else { u }).rem_euclid(4) == 0 || u.abs() == HALF && (w == NORTH || w == SOUTH);
                if v <= 2 { g.n(u, v, w, "clay_bricks"); } else if post || v == WALL_TOP || v == 8 { g.n(u, v, w, "marble"); } else if u.abs() == HALF { g.t(u, v, w, "window", 1, 0); } else { g.t(u, v, w, "window", 0, 1); } }
            else if v == 0 { g.n(u, v, w, if g.h(u, 0, w, 3) < 0.2 { "mossy_stone" } else { "dirt" }); } else { g.air(u, v, w); } } } }
    let arch = |u: i32| (WALL_TOP as f64 + (100.0 - (u * u) as f64).max(0.0).sqrt() * 0.8).round() as i32;
    for w in (NORTH..=SOUTH).filter(|w| (w - NORTH) % 4 == 0) { for u in -HALF..=HALF { let lo = if u.abs() == HALF { arch(u) } else { arch(u + u.signum()).min(arch(u)) }; for v in lo..=arch(u) { g.n(u, v, w, "marble"); } } }
    for u in [-7, 0, 7] { let top = (WALL_TOP as f64 + (100.0 - (u * u) as f64).sqrt() * 0.8).round() as i32; for w in NORTH..=SOUTH { g.n(u, top, w, "marble"); if g.h(u, top, w, 4) < 0.3 { g.n(u, top - 1, w, if g.h(u, top, w, 5) < 0.5 { "glow_vine" } else { "vine" }); } } }
    // la porte du nord : un arc de fer, des marches
    for u in -1..=1 { for v in 1..=3 { g.air(u, v, NORTH); } } for u in -2..=2 { g.n(u, 4, NORTH, "iron_block"); } for u in -2..=2 { g.n(u, 0, NORTH - 1, "stone_bricks"); }
    // — LA SALLE DES SPORES — le plafond moussu (la terrasse du nord), la cloison et sa grille ; les champignons géants
    for u in -HALF + 1..HALF { for w in NORTH + 1..=MID { g.n(u, DECK, w, "mossy_stone"); } } for u in -HALF + 1..HALF { for v in 1..DECK { g.n(u, v, MID, if v <= 2 { "clay_bricks" } else { "mossy_stone" }); } }
    for u in -1..=1 { for v in 1..=3 { g.n(u, v, MID, "iron_bars"); } }
    for (u, w, h) in STEPS_E.iter().chain(STEPS_W.iter()) { g.mushroom(*u, *w, *h, 0, "mushroom_cap_red"); }
    let (cu, cw, ch) = CAP_E; g.mushroom(cu, cw, ch, 1, "mushroom_cap_red"); let (cu, cw, ch) = CAP_W; g.mushroom(cu, cw, ch, 1, "mushroom_cap_violet");
    for (u, w, h, r) in [(-2, -10, 9, 2), (3, -13, 7, 1), (-4, -7, 4, 1), (2, -7, 3, 0), (-3, -14, 3, 0)] { g.mushroom(u, w, h, r, if r == 2 { "mushroom_cap_violet" } else { "mushroom_cap_blue" }); }
    for u in -HALF + 1..HALF { for w in NORTH + 1..MID { if g.h(u, 0, w, 6) < 0.25 { g.n(u, 0, w, "glow_moss"); } } }
    for v in [VENTS[0], VENTS[1]] { g.t(v[0], v[1], v[2], "lever", 0, 1); }
    // — LA NEF — le bassin, les jardinières, la souche, le tronc de la liane, la dalle d'arrosage, le bloc à pousser
    for u in 4..=8 { for w in -3..=2 { g.n(u, -1, w, "mossy_stone"); g.n(u, 0, w, "water"); if g.h(u, 1, w, 7) < 0.3 { g.n(u, 1, w, "lily_pad"); } } }
    for w in [-3, 3] { for u in [-9, 9] { g.n(u, 1, w, "oak_leaves"); g.n(u, 2, w, "fern"); } }
    for v in 1..=DECK + 2 { g.n(TRUNK.0, v, TRUNK.1, "liana"); } for du in -2..=2i32 { for dw in -2..=2i32 { if du.abs() + dw.abs() < 4 { g.n(TRUNK.0 + du, DECK + 3, TRUNK.1 + dw, "oak_leaves"); if du.abs() + dw.abs() < 2 { g.n(TRUNK.0 + du, DECK + 4, TRUNK.1 + dw, "oak_leaves"); } } } }
    g.n(PLATE[0], 0, PLATE[2], "glow_moss"); for (du, dw) in [(-1, 1), (1, 1)] { g.n(PLATE[0] + du, 1, PLATE[2] + dw, "roots"); } g.n(PLATE[0], 1, PLATE[2] + 1, "liana"); g.n(BLOCK[0], BLOCK[1], BLOCK[2], "push_block");
    // — LA CANOPÉE — un plancher sur colonnes au sud, son garde-fou ; le levier du pont de ronces
    for u in -HALF + 1..HALF { for w in CANOPY..SOUTH { g.n(u, DECK, w, "planks"); } } for u in [-6, -2, 2, 6] { for v in 1..DECK { g.n(u, v, CANOPY, "iron_block"); } }
    for u in -HALF + 1..HALF { if u != 0 && u != 1 && u != BRIDGE_U { g.n(u, DECK + 1, CANOPY, "fence"); } }
    for l in BRIDGE_LEVERS { g.t(l[0], l[1], l[2], "lever", 0, 1); }
    for (u, w) in [(-8, 14), (8, 14), (-8, -14), (8, -7)] { g.n(u, DECK + 1, w, "lantern"); }
    // — LA TERRASSE DU NORD — le coffre sur son autel, des ronces
    g.n(CHEST[0], CHEST[1], CHEST[2], "chest");
    for (u, w) in [(-8, -15), (8, -15), (-4, -13), (5, -12)] { g.n(u, DECK + 1, w, "dead_bush"); }
}

/// Les mécanismes de la serre bâtie en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] + 1; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let ids = |l: Vec<([i32; 3], &'static str)>| l.into_iter().map(|(c, b)| (at(c), by_name(b).unwrap_or(0))).collect::<Vec<_>>();
    let mut m = Mech::default(); let iron = by_name("iron_bars").unwrap_or(0);
    m.wires.push(Wire { name: name("aeration"), inputs: VENTS.iter().map(|v| at(*v)).collect(), all: true, latch: true, doors: (-1..=1).flat_map(|u| (1..=3).map(move |v| [u, v, MID])).map(|c| (at(c), iron)).collect(), ..Default::default() });
    m.miasmas.push(Miasma { min: at([-HALF + 1, 1, NORTH + 2]), max: at([HALF - 1, DECK - 1, MID - 1]), dps: SPORES, until: name("aeration"), ..Default::default() });
    m.wires.push(Wire { name: name("arrosage"), blocks: vec![at(PLATE)], ..Default::default() });
    m.dials.push(Dial { name: name("liane"), n: DECK as u8 + 1, state: 0, goal: 255, draw: (0..=DECK as u8).map(|k| ids(liana(k))).collect(), auto: 0.25, grow: name("arrosage"), ..Default::default() });
    m.homes.push(Home { min: at([-HALF + 1, 1, MID + 1]), max: at([HALF - 1, DECK - 1, CANOPY - 1]), homes: vec![at(BLOCK)], keep: name("arrosage") });
    m.wires.push(Wire { name: name("ronces"), inputs: BRIDGE_LEVERS.iter().map(|l| at(*l)).collect(), hold: BRIDGE_HOLD, ..Default::default() });
    m.dials.push(Dial { name: name("pont"), n: bridge_len() + 1, state: 0, goal: 255, draw: (0..=bridge_len()).map(|k| ids(bridge(k))).collect(), auto: 0.12, grow: name("ronces"), ..Default::default() });
    m.guards.push(Guard { min: at([-HALF + 1, DECK + 1, NORTH + 1]), max: at([HALF - 1, DECK + 8, MID - 1]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: Vec::new(), state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-HALF, -1, NORTH - 1]), max: at([HALF, WALL_TOP + 9, SOUTH]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-HALF + 1, 1, NORTH + 1], [HALF - 1, 3, NORTH + 1], "La Serre empoisonnée. Les champignons géants crachent leurs spores. Deux leviers d'aération, tout en haut de deux chapeaux : abaissez-les vite, avant que les spores ne vous aient.");
    hint(&mut m, [-HALF + 1, 1, MID + 1], [HALF - 1, 3, MID + 2], "La nef. Au pied de la souche, une dalle de mousse attend qu'on l'arrose : un bloc de pierre posé dessus y fera monter l'eau.");
    hint(&mut m, [-HALF + 1, DECK + 1, CANOPY], [HALF - 1, DECK + 3, SOUTH - 1], "La canopée. Le levier fait pousser un pont de ronces jusqu'à la terrasse du nord ; il se flétrit vite.");
    m
}

impl Session {
    /// Bâtit ici la serre (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol au centre.
    pub fn build_greenhouse(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("e{}:{}:", o[0], o[2]); if !self.mech.dials.iter().any(|d| d.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Une serre de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn greenhouse_wake(&mut self, pl: &Place) { let tag = format!("e{}:{}:", pl.x, pl.z); if self.mech.dials.iter().any(|d| d.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}
