//! Le Temple de glace : un sanctuaire de glace et de marbre, sous un toit à degrés, entre deux tours que coiffent des aiguilles
//! de cristal. Dans la grande nef, trois bassins gelés, trois énigmes de blocs qui glissent : un bloc poussé sur la glace file
//! jusqu'à ce que quelque chose l'arrête (un pilier de glace, un autre bloc, le bord du bassin, une dalle de cristal).
//! - **le bassin de l'ouest** : amener le bloc sur la dalle de cristal ;
//! - **le bassin de l'est** : amener deux blocs sur deux dalles — l'un sert d'appui à l'autre ;
//! - **le bassin du milieu** : au sud, une estrade ceinte d'un parapet ; une seule brèche. Il faut y amener le bloc, juste au
//!   pied de la brèche, pour y grimper — le levier est là-haut.
//! Si un bloc se coince, on sort du bassin : la glace reprend sa forme (les blocs reviennent à leur place).
//! Les trois chaînes de la porte du sud tombées, la grille se lève : au Cœur de l'Hiver, la Reine des glaces se lève de son trône.
//! Tout est vérifié par `tests/glace.rs` (les solutions sont rejouées bloc à bloc) avec le solveur de chemins.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Guard, Hint, Home, Mech, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// Un bassin gelé : son coin (u, w), sa largeur, sa profondeur, ses piliers de glace, ses blocs, ses dalles de cristal (en
/// coordonnées du bassin : x vers +u, y vers +w).
pub struct Rink { pub at: (i32, i32), pub size: (i32, i32), pub pillars: &'static [(i32, i32)], pub blocks: &'static [(i32, i32)], pub targets: &'static [(i32, i32)] }
impl Rink { pub fn cell(&self, c: (i32, i32)) -> [i32; 3] { [self.at.0 + c.0, 0, self.at.1 + c.1] } }
pub const WEST: Rink = Rink { at: (-17, 0), size: (9, 9), pillars: &[(0, 1), (6, 0), (7, 7), (6, 5), (5, 8), (7, 2), (3, 6), (8, 6), (0, 5), (3, 5), (5, 2)], blocks: &[(7, 8)], targets: &[(2, 6)] };
pub const EAST: Rink = Rink { at: (9, 0), size: (9, 9), pillars: &[(0, 1), (0, 5), (2, 0), (2, 7), (3, 0), (3, 2), (4, 4), (5, 7), (7, 7), (8, 2), (8, 8)], blocks: &[(1, 4), (0, 4)], targets: &[(2, 4), (7, 4)] };
/// Le bassin du milieu : la case au pied de la brèche de l'estrade est (4, 6).
pub const MID: Rink = Rink { at: (-4, 2), size: (9, 7), pillars: &[(0, 0), (0, 1), (2, 4), (3, 2), (5, 0), (6, 1), (7, 5), (8, 0)], blocks: &[(7, 1)], targets: &[] };
pub const NOTCH: (i32, i32) = (4, 6);
/// L'estrade (w0, w1), son levier ; la porte du sud : ses chaînes (w), sa grille (w).
pub const DAIS: (i32, i32) = (9, 11); pub const LEVER: [i32; 3] = [0, 2, 11]; pub const CHAINS_W: i32 = 20; pub const GATE_W: i32 = 21;
/// La nef (intérieur u0, u1, w0, w1) ; le Cœur de l'Hiver (centre, rayon) ; la Reine des glaces ; le coffre.
pub const NAVE: (i32, i32, i32, i32) = (-20, 20, -6, 20); pub const HEART: (i32, i32, f64) = (0, 27, 5.5);
pub const BOSS: &str = "reine_glaces"; pub const BOSS_AT: [f64; 3] = [0.5, 1.0, 26.5]; pub const CHEST: [i32; 3] = [0, 2, 32];
/// La voûte de la nef ; le toit à degrés.
pub fn vault(u: i32) -> i32 { 7 + (0.35 * (20 - u.abs()) as f64).round() as i32 }
fn roof(u: i32) -> i32 { match u.abs() { 0..=4 => 16, 5..=8 => 15, 9..=12 => 14, 13..=16 => 13, _ => 12 } }

struct G<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl G<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn h(&self, u: i32, v: i32, w: i32, k: i32) -> f64 { hash_float(self.x + u + 337 * v, self.z + w, 6421, k) }
    fn column(&mut self, u: i32, w: i32, v0: i32, v1: i32) { for v in v0..=v1 { self.n(u, v, w, if v == v0 { "column_base" } else if v == v1 { "column_capital" } else { "column_shaft" }); } }
    fn statue(&mut self, u: i32, v: i32, w: i32, back: (i32, i32)) { self.t(u, v, w, "statue_base", back.0, back.1); self.t(u, v + 1, w, "statue_top", back.0, back.1); }
    /// Une aiguille de glace : un fût de glace bleue, une pointe de cristal.
    fn spire(&mut self, u: i32, w: i32, v0: i32, v1: i32) { for v in v0..=v1 { self.n(u, v, w, "ice_blue"); } self.n(u, v1 + 1, w, "ice_point_up_2"); }
}

/// Pose le temple dans le plan : `o` est le sol au centre de la nef (le dallage est un bloc plus haut).
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut g = G { p, x: o[0], f: o[1] + 1, z: o[2] };
    let (u0, u1, w0, w1) = NAVE;
    // le soubassement de marbre, le dallage en damier ; les murs de glace à bandeaux de marbre ; la voûte et le toit à degrés
    for u in u0 - 1..=u1 + 1 { for w in w0 - 1..=w1 + 1 { let wall = u < u0 || u > u1 || w < w0 || w > w1;
        g.n(u, -1, w, "marble"); g.n(u, 0, w, if wall { "marble" } else if (u + w).rem_euclid(2) == 0 { "marble" } else { "marble_black" });
        let top = if wall { 0 } else { vault(u) }; for v in 1..=top { g.air(u, v, w); }
        for v in top + 1..=roof(u) { let band = v == 1 || v == 6 || v == 11 || v == roof(u); g.n(u, v, w, if band { "marble" } else if wall && (v == 2 || v == 10) { "ice_blue" } else { "ice" }); }
        if u.abs() == 0 && w.rem_euclid(3) == 0 { g.n(0, roof(0) + 1, w, "ice_point_up_1"); } } }
    // des glaçons pendus à la voûte
    for u in u0..=u1 { for w in w0..=w1 { if g.h(u, 9, w, 2) < 0.07 { let k = (g.h(u, 8, w, 3) * 3.0) as usize % 3; g.n(u, vault(u), w, ["ice_point_down_0", "ice_point_down_1", "ice_point_down_2"][k]); } } }
    // des pilastres de marbre et de hautes fenêtres de vitrail sur les flancs ; des contreforts surmontés d'aiguilles
    for w in (w0..=w1).step_by(4) { for side in [-1, 1] { let u = (u1 + 1) * side; for v in 1..=11 { g.n(u, v, w, "marble"); } g.spire(u + side, w, 1, 9); } }
    for w in (w0 + 2..=w1).step_by(4) { for side in [-1, 1] { for v in 3..=9 { g.n((u1 + 1) * side, v, w, "stained_glass"); } } }
    // la façade du nord : un grand portail en arc brisé, une rose de vitrail ; deux tours à aiguilles de cristal ; le perron
    for u in -2..=2i32 { for v in 1..=5 { g.air(u, v, w0 - 1); } } for (u, v) in [(-1, 6), (0, 6), (1, 6), (0, 7)] { g.air(u, v, w0 - 1); } for (u, v) in [(-3, 1), (-3, 2), (-3, 3), (-3, 4), (-3, 5), (-2, 6), (-1, 7), (0, 8)] { for s in [-1, 1] { g.n(u * s, v, w0 - 1, "marble"); } }
    for u in -2..=2i32 { for v in 9..=13 { let (du, dv) = (u as f64, v as f64 - 11.0); if du * du + dv * dv <= 5.0 { g.n(u, v, w0 - 1, "stained_glass"); } } }
    for side in [-1, 1] { for u in 17..=21 { for w in w0 - 4..=w0 - 1 { let edge = u == 17 || u == 21 || w == w0 - 4 || w == w0 - 1; for v in -1..=20 { g.n(u * side, v, w, if v <= 0 { "marble" } else if !edge && v >= 1 { "ice" } else if (u == 17 || u == 21) && (w == w0 - 4 || w == w0 - 1) { "marble" } else if v % 5 == 0 { "marble" } else { "ice" }); } } }
        for (u, w) in [(17, w0 - 4), (21, w0 - 4), (17, w0 - 1), (21, w0 - 1)] { g.spire(u * side, w, 21, 23); } for u in 18..=20 { for w in w0 - 3..=w0 - 2 { g.n(u * side, 21, w, "crystal_blue"); g.n(u * side, 22, w, "ice_blue"); } } g.spire(19 * side, w0 - 3, 23, 27); g.n(19 * side, 15, w0 - 4, "stained_glass"); g.n(19 * side, 16, w0 - 4, "stained_glass"); }
    for u in -5..=5 { g.t(u, 0, w0 - 2, "marble_stairs", 0, 1); g.n(u, -1, w0 - 2, "marble"); }
    // l'allée des obélisques de glace devant le temple
    for w in [-26, -21, -16, -11] { for side in [-1, 1] { g.n(5 * side, -1, w, "marble"); g.spire(5 * side, w, 0, 3); } }
    // les trois bassins gelés : de la glace un bloc plus bas, des piliers de glace, les blocs, les dalles de cristal ; une margelle blanche
    for r in [&WEST, &EAST, &MID] { for x in -1..=r.size.0 { for y in -1..=r.size.1 { let (u, w) = (r.at.0 + x, r.at.1 + y); let inside = x >= 0 && y >= 0 && x < r.size.0 && y < r.size.1;
            if inside { g.air(u, 0, w); g.n(u, -1, w, "ice"); g.n(u, -2, w, "marble"); } else { g.n(u, 0, w, "marble"); } } }
        for &(x, y) in r.pillars { let (u, w) = (r.at.0 + x, r.at.1 + y); g.n(u, 0, w, "ice_blue"); g.n(u, 1, w, "ice_blue"); g.n(u, 2, w, ["ice_point_up_0", "ice_point_up_1", "ice_point_up_2"][((x * 7 + y * 3) % 3) as usize]); }
        for &(x, y) in r.blocks { g.n(r.at.0 + x, 0, r.at.1 + y, "push_block"); } for &(x, y) in r.targets { g.n(r.at.0 + x, -1, r.at.1 + y, "crystal_blue"); } }
    // l'estrade : un bloc de marbre, un parapet de glace et sa brèche, des murs autour ; le levier ; deux statues de reines
    let (d0, d1) = DAIS; for u in -5..=5i32 { for w in d0..=d1 + 1 { let side = u.abs() == 5 || w == d1 + 1;
        for v in 0..=1 { g.n(u, v, w, "marble"); } for v in 2..=4 { if side { g.n(u, v, w, if v == 4 { "marble" } else { "ice_blue" }); } else { g.air(u, v, w); } } } }
    for u in -4..=4 { if u != MID.at.0 + NOTCH.0 { for v in 2..=3 { g.n(u, v, d0, "ice_blue"); } } } g.t(LEVER[0], LEVER[1], LEVER[2], "lever", 0, 1); for u in [-3, 3] { g.statue(u, 2, d1, (0, 1)); }
    // des colonnes le long de la nef, des lustres, des grappes de cristaux dans les angles
    for w in [-4, 0, 4, 8, 13, 17] { for side in [-1, 1] { g.column(7 * side, w, 1, vault(7) - 1); } }
    for (u, w) in [(0, -3), (-12, 13), (12, 13), (0, 16)] { g.n(u, vault(u), w, "chandelier"); }
    for (u, w) in [(-19i32, -5i32), (19, -5), (-19, 19), (19, 19), (-19, 10), (19, 10)] { for (du, dw, k) in [(0, 0, 2), (1, 0, 1), (0, 1, 0)] { let uu = u + du * -u.signum(); g.n(uu, 1, w + dw * if w > 0 { -1 } else { 1 }, ["crystal_blue_point_up_0", "crystal_blue_point_up_1", "crystal_blue_point_up_2"][k]); } }
    for (u, w) in [(-3, -5), (3, -5), (-10, 19), (10, 19)] { g.n(u, 1, w, "glow_jar"); }
    // la porte du sud : un cadre de marbre, trois chaînes devant une grille
    for u in -1..=1 { for v in 1..=3 { g.n(u, v, CHAINS_W, "great_chain"); g.n(u, v, GATE_W, "iron_bars"); } } for u in [-2, 2] { g.column(u, CHAINS_W, 1, 4); g.n(u, 5, CHAINS_W, "glow_jar"); } for u in -2..=2 { g.n(u, 4, GATE_W, "marble"); }
    // le Cœur de l'Hiver : une salle ronde sous une coupole de glace, des piliers de glace, l'autel de la reine et son trésor
    let (hu, hw, hr) = HEART; for u in hu - 7..=hu + 7 { for w in GATE_W..=hw + 7 { let r = ((u - hu) as f64).hypot((w - hw) as f64);
        g.n(u, -1, w, "marble"); if r >= hr || w <= GATE_W { if w > GATE_W || u.abs() > 1 { for v in 0..=8 { g.n(u, v, w, if v == 0 || v == 8 { "marble" } else if v % 3 == 0 { "ice_blue" } else { "ice" }); } } continue; }
        let top = 3 + (6.0 * (1.0 - (r / hr).powi(2)).max(0.0).sqrt()).round() as i32; g.n(u, 0, w, if r < 2.0 { "crystal_blue" } else if (u + w).rem_euclid(2) == 0 { "marble" } else { "marble_black" }); for v in 1..=top { g.air(u, v, w); } for v in top + 1..=top + 2 { g.n(u, v, w, "ice_blue"); } } }
    for (u, w) in [(-3, 25), (3, 25), (-3, 30), (3, 30)] { g.spire(u, w, 1, 4); } g.t(CHEST[0], 1, CHEST[2], "altar", 0, 1); g.n(CHEST[0], CHEST[1], CHEST[2], "chest"); for u in [-2, 2] { g.n(u, 1, 32, "glow_jar"); }
    g.n(0, 9, hw, "chandelier");
}

/// Les mécanismes du temple bâti en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] + 1; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let chain = by_name("great_chain").unwrap_or(0); let iron = by_name("iron_bars").unwrap_or(0);
    let chain_of = |u: i32| (1..=3).map(|v| (at([u, v, CHAINS_W]), chain)).collect::<Vec<_>>();
    let mut m = Mech::default();
    for (k, r) in [&WEST, &EAST].iter().enumerate() { let wn = name(if k == 0 { "ouest" } else { "est" });
        m.wires.push(Wire { name: wn.clone(), blocks: r.targets.iter().map(|c| at(r.cell(*c))).collect(), latch: true, doors: chain_of(k as i32 - 1), ..Default::default() }); }
    m.wires.push(Wire { name: name("estrade"), inputs: vec![at(LEVER)], latch: true, doors: chain_of(1), ..Default::default() });
    for (r, keep) in [(&WEST, "ouest"), (&EAST, "est"), (&MID, "estrade")] { m.homes.push(Home { min: at([r.at.0, 0, r.at.1]), max: at([r.at.0 + r.size.0 - 1, 1, r.at.1 + r.size.1 - 1 + if keep == "estrade" { 3 } else { 0 }]), homes: r.blocks.iter().map(|c| at(r.cell(*c))).collect(), keep: name(keep) }); }
    let gate: Vec<([i32; 3], Id)> = (-1..=1).flat_map(|u| (1..=3).map(move |v| [u, v, GATE_W])).map(|q| (at(q), iron)).collect();
    m.wires.push(Wire { name: name("coeur"), needs: vec![name("ouest"), name("est"), name("estrade")], latch: true, doors: gate.clone(), ..Default::default() });
    let (hu, hw, hr) = HEART; let r = hr as i32;
    m.guards.push(Guard { min: at([hu - r, 1, GATE_W + 1]), max: at([hu + r, 8, hw + r]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: gate, state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-24, -3, -12]), max: at([24, 28, 35]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-6, 1, -6], [6, 3, -3], "Le Temple de glace. La porte du sud est tenue par trois chaînes, une par bassin gelé. Sur la glace, un bloc poussé glisse jusqu'à ce que quelque chose l'arrête. Si un bloc se coince, sortez du bassin : la glace reprendra sa forme.");
    hint(&mut m, [-18, 0, -1], [-8, 1, 9], "Le bassin de l'ouest : un bloc, une dalle de cristal. Un bloc s'arrête aussi sur le cristal.");
    hint(&mut m, [8, 0, -1], [18, 1, 9], "Le bassin de l'est : deux blocs, deux dalles. Un bloc peut servir d'appui à l'autre.");
    hint(&mut m, [-5, 0, 1], [5, 1, 8], "Le bassin du milieu. Au sud, l'estrade et son levier, derrière un parapet de glace : une seule brèche. Il faudrait de quoi y grimper, juste au pied…");
    m
}

impl Session {
    /// Bâtit ici le temple (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol au centre de la nef.
    pub fn build_ice_temple(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("i{}:{}:", o[0], o[2]); if !self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Un temple de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn ice_temple_wake(&mut self, pl: &Place) { let tag = format!("i{}:{}:", pl.x, pl.z); if self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}
