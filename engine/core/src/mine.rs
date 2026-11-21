//! La Mine des nains : une porte de pierre et un chevalement en surface ; un long escalier taillé descend, dix-huit blocs plus
//! bas, dans les salles des nains. Trois épreuves, un boss :
//! - **la gare des wagonnets** : une immense salle coupée d'un gouffre. Un wagonnet attend à quai ; on monte dedans, il part sur
//!   la voie que désignent deux aiguillages (le nouveau mécanisme : les wagonnets, qui emportent le joueur). Le premier se
//!   manœuvre au quai ; le second sur un îlot au milieu du gouffre, où mène l'une des voies. Une seule mène de l'autre côté.
//! - **le couloir des marteaux-pilons** : trois marteaux tombent du plafond en cadence, et écrasent qui passe dessous. Sur le mur,
//!   près de chacun, une rune.
//! - **la porte runique** : trois roues de runes et leurs leviers ; les trois runes du couloir, dans l'ordre où on les a croisées,
//!   de gauche à droite : les chaînes tombent.
//! - **le Golem de minerai** se lève dans la salle du trône, devant le coffre des rois nains.
//! Les runes changent d'une mine à l'autre. Tout est vérifié par `tests/mine.rs`.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Cart, Dial, Guard, Hint, Mech, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// Le sol des salles est à `DEPTH` blocs sous le sol de la cour.
pub const DEPTH: i32 = 18;
/// La gare (intérieur u0, u1, w0, w1, voûte) ; le gouffre (w0, w1, son fond) ; l'îlot (u0, u1, w0, w1) ; le quai d'en face (w0, w1).
pub const HALL: (i32, i32, i32, i32, i32) = (-18, 18, -17, 8, 10); pub const CHASM: (i32, i32, i32) = (-6, 5, -9); pub const ISLE: (i32, i32, i32, i32) = (-13, -8, -1, 0); pub const FAR: (i32, i32) = (6, 8);
/// Le quai du wagonnet, la jonction, les aiguillages (A au quai, B sur l'îlot), l'échelle du gouffre (u, w).
pub const STATION: [i32; 3] = [0, 1, -13]; pub const JUNCTION_W: i32 = -8; pub const SWITCH_A: [i32; 3] = [3, 1, -13]; pub const SWITCH_B: [i32; 3] = [-12, 1, 0]; pub const CHASM_LADDER: (i32, i32) = (5, -6);
/// Le couloir (w0, w1) ; les marteaux (w) ; les runes du mur ; l'antichambre (w0, w1) ; le mur de la porte (w) ; la salle du trône.
pub const CORRIDOR: (i32, i32) = (9, 19); pub const HAMMERS: [i32; 3] = [12, 15, 18]; pub const ANTE: (i32, i32) = (20, 24); pub const DOOR_W: i32 = 25; pub const THRONE: (i32, i32, i32, i32, i32) = (-10, 10, 26, 34, 9);
/// Les roues de runes (le centre u, v de chacune, de gauche à droite pour qui regarde la porte : de l'est à l'ouest) et leurs leviers.
pub const WHEELS: [(i32, i32); 3] = [(5, 3), (0, 6), (-5, 3)]; pub const WHEEL_LEVERS: [[i32; 3]; 3] = [[5, 1, 23], [2, 1, 23], [-5, 1, 23]];
pub const BOSS: &str = "golem_minerai"; pub const BOSS_AT: [f64; 3] = [0.5, 1.0, 30.5]; pub const CHEST: [i32; 3] = [3, 1, 34];
/// Les paliers de l'escalier (u0, u1, w0, w1, sol) : sous la volée du sud, au bout de la volée du nord, le boyau vers la gare.
pub const LANDINGS: [(i32, i32, i32, i32, i32); 3] = [(-1, 5, -22, -20, 8), (3, 9, -32, -30, 0), (7, 9, -29, -18, 0)];
/// Les marches (w, dessus, u de la première des trois, sens) : la volée du sud (de la cour au premier palier), puis celle du nord.
pub fn flights() -> Vec<(i32, i32, i32, i32)> { (-31..=-23).map(|w| (w, -14 - w, -1, -1)).chain((-29..=-23).map(|w| (w, 30 + w, 3, 1))).collect() }
/// Les quatre runes (rangée du haut d'abord ; symétriques, pour qu'on ne les lise jamais à l'envers).
pub const RUNES: [[u8; 3]; 4] = [[0b010, 0b111, 0b010], [0b101, 0b010, 0b101], [0b111, 0b101, 0b111], [0b111, 0b101, 0b101]];

/// Les runes du couloir (et donc celles qu'attend la porte) ; où sont les roues au départ.
pub fn runes(o: [i32; 3]) -> [u8; 3] { let r = |k| (hash_float(o[0], o[2], 6203, k) * 4.0) as u8 % 4; [r(1), r(2), r(3)] }
pub fn wheels_start(o: [i32; 3]) -> [u8; 3] { let g = runes(o); let r = |k| 1 + (hash_float(o[0], o[2], 6203, 10 + k) * 3.0) as u8 % 3; [(g[0] + r(0)) % 4, (g[1] + r(1)) % 4, (g[2] + r(2)) % 4] }
/// Les cases allumées d'une rune, centrée en (cu, cv), dans le plan w ; `east_first` : la colonne de gauche est à l'est (pour qui
/// regarde vers le sud).
pub fn glyph(rune: u8, c: (i32, i32), w: i32) -> Vec<[i32; 3]> { let p = RUNES[rune as usize % 4]; (0..3).flat_map(|r| (0..3).map(move |k| (r, k))).filter(|(r, k)| p[*r] >> (2 - k) & 1 == 1).map(|(r, k)| [c.0 + 1 - k as i32, c.1 + 1 - r as i32, w]).collect() }
/// Les trois voies : vers l'îlot (aiguillage A relâché), vers le butoir de l'est (A seul), par le pont de l'est (A et B).
pub fn routes() -> Vec<Vec<[i32; 3]>> {
    let mut start = vec![]; for w in STATION[2]..=JUNCTION_W { start.push([0, 1, w]); }
    let west = { let mut r = start.clone(); for u in 1..=10 { r.push([-u, 1, JUNCTION_W]); } for w in JUNCTION_W + 1..=0 { r.push([-10, 1, w]); } r };
    let dead = { let mut r = start.clone(); for u in 1..=16 { r.push([u, 1, JUNCTION_W]); } r };
    let bridge = { let mut r = start.clone(); for u in 1..=10 { r.push([u, 1, JUNCTION_W]); } for w in JUNCTION_W + 1..=FAR.0 + 1 { r.push([10, 1, w]); } r };
    vec![west.clone(), dead, west, bridge]
}
/// Un marteau-pilon au cran `k` (0 en haut, 2 écrasé au sol), dans le couloir à `w`.
pub fn hammer(w: i32, k: u8) -> Vec<([i32; 3], &'static str)> { let (lo, hi) = [(4, 5), (3, 4), (1, 2)][k as usize % 3]; (-1..=1).flat_map(|u| (lo..=hi).map(move |v| ([u, v, w], if v == lo { "iron_block" } else { "stone_bricks_dark" }))).collect() }

struct G<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl G<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn h(&self, u: i32, v: i32, w: i32, k: i32) -> f64 { hash_float(self.x + u + 409 * v, self.z + w, 7121, k) }
    /// La roche des nains : de la pierre, des veines de minerai, quelques cristaux.
    fn rock(&self, u: i32, v: i32, w: i32) -> &'static str { let h = self.h(u, v, w, 1); if h < 0.03 { "iron_ore" } else if h < 0.05 { "copper_ore" } else if h < 0.06 { "crystal_blue" } else if h < 0.2 { "cobblestone" } else { "stone" } }
    fn column(&mut self, u: i32, w: i32, v0: i32, v1: i32) { for v in v0..=v1 { self.n(u, v, w, if v == v0 { "column_base" } else if v == v1 { "column_capital" } else { "column_shaft" }); } }
}

/// Pose la mine dans le plan : `o` est le sol de la cour (au-dessus de la gare).
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut g = G { p, x: o[0], f: o[1] - DEPTH, z: o[2] }; let d = DEPTH;
    // un bloc de roche, d'un bout à l'autre, pour que les salles soient closes quoi qu'il y ait sous terre
    for u in -20..=20 { for w in -18..=36 { for v in CHASM.2 - 1..=HALL.4 + 1 { let r = g.rock(u, v, w); g.n(u, v, w, r); } } }
    // — LA GARE — le dallage, la voûte, les piliers ; le gouffre et son fond ; l'îlot ; le quai d'en face ; l'échelle du gouffre
    let (u0, u1, w0, w1, top) = HALL; for u in u0..=u1 { for w in w0..=w1 { let vault = top - ((u as f64 / 19.0).powi(2) * 3.0).round() as i32;
        let chasm = w >= CHASM.0 && w <= CHASM.1; let isle = u >= ISLE.0 && u <= ISLE.1 && w >= ISLE.2 && w <= ISLE.3;
        g.n(u, if chasm && !isle { CHASM.2 } else { 0 }, w, if (u + w).rem_euclid(5) == 0 { "stone_bricks_dark" } else { "stone_bricks" });
        let low = if chasm && !isle { CHASM.2 + 1 } else { 1 }; for v in low..=vault { g.air(u, v, w); } } }
    for (u, w) in [(-15, -14), (15, -14), (-15, -10), (15, -10), (-15, 6), (15, 6)] { g.column(u, w, 1, HALL.4 - 2); g.n(u, HALL.4 - 1, w, "stone_bricks_dark"); }
    for (u, w) in [(-17, -16), (17, -16), (-17, 7), (17, 7), (-6, -16), (6, -16)] { g.n(u, 1, w, "lantern"); } for (u, w) in [(-13, -1), (-8, -1)] { g.n(u, 1, w, "lantern"); } g.n(9, 1, 7, "lantern");
    for v in CHASM.2 + 1..=0 { g.t(CHASM_LADDER.0, v, CHASM_LADDER.1, "ladder", 0, -1); }
    // les voies : des rails partout où passent les wagonnets, des chaînes qui pendent de la voûte au-dessus des ponts ; le butoir ;
    // le wagonnet à quai ; les deux aiguillages
    let rs = routes(); for r in &rs { for (i, c) in r.iter().enumerate() { let q = if i + 1 < r.len() { r[i + 1] } else { r[i - 1] }; g.n(c[0], c[1], c[2], if q[0] != c[0] { "rail_x" } else { "rail_z" });
        if c[2] >= CHASM.0 && c[2] <= CHASM.1 && !(c[0] >= ISLE.0 && c[0] <= ISLE.1) && c[2].rem_euclid(3) == 0 { for v in 5..=HALL.4 - 2 { g.n(c[0], v, c[2], "chain"); } } } }
    g.n(17, 1, JUNCTION_W, "oak_beam"); g.n(17, 2, JUNCTION_W, "oak_beam"); g.n(STATION[0], STATION[1], STATION[2], "minecart");
    for l in [SWITCH_A, SWITCH_B] { g.t(l[0], l[1], l[2], "lever", 0, -1); } g.n(-1, 1, STATION[2] - 1, "coal_ore"); g.n(1, 1, STATION[2] - 2, "crate");
    for (u, w) in [(-4, -15), (5, -16), (-14, -12)] { g.n(u, 1, w, "minecart"); g.n(u, 2, w, if g.h(u, 2, w, 3) < 0.5 { "coal_ore" } else { "iron_ore" }); }
    // — LE COULOIR DES MARTEAUX — un boyau de trois de large ; les marteaux, levés ; les runes du mur de l'ouest
    let (c0, c1) = CORRIDOR; for w in c0..=c1 { for u in -1..=1 { g.n(u, 0, w, "stone_bricks_dark"); for v in 1..=5 { g.air(u, v, w); } } } for w in FAR.0..=FAR.1 { for u in -1..=1 { g.n(u, 0, w, "stone_bricks"); } }
    let rn = runes(o); for (k, w) in HAMMERS.iter().enumerate() { for (c, b) in hammer(*w, 0) { g.n(c[0], c[1], c[2], b); } for v in 6..=8 { for u in -1..=1 { g.n(u, v, *w, "air"); } } g.n(0, 8, *w, "iron_block");
        for dw in -3..=-1 { for v in 2..=4 { g.n(-2, v, w + dw, "stone_bricks_dark"); } } for c in glyph(rn[k], (0, 3), 0) { g.n(-2, c[1], w - 2 + (c[0]), "crystal_violet"); } g.n(2, 2, w - 2, "lantern"); }
    // — L'ANTICHAMBRE ET LA PORTE RUNIQUE —
    let (a0, a1) = ANTE; for u in -7..=7 { for w in a0..=a1 { g.n(u, 0, w, "stone_bricks_dark"); for v in 1..=7 { g.air(u, v, w); } } }
    for u in -7..=7 { for v in 1..=7 { g.n(u, v, DOOR_W, if u.abs() == 7 || v == 7 { "stone_bricks_dark" } else { "stone_bricks" }); } } for u in -1..=1 { for v in 1..=3 { g.n(u, v, DOOR_W, "great_chain"); } }
    let ws = wheels_start(o); for (k, wc) in WHEELS.iter().enumerate() { for du in -2..=2i32 { for dv in -2..=2i32 { if du.abs() == 2 || dv.abs() == 2 { g.n(wc.0 + du, wc.1 + dv, DOOR_W - 1, "stone_bricks_dark"); } } } for c in glyph(ws[k], *wc, DOOR_W - 1) { g.n(c[0], c[1], c[2], "crystal_blue"); } }
    for l in WHEEL_LEVERS { g.t(l[0], l[1], l[2], "lever", 0, 1); }
    // — LA SALLE DU TRÔNE — des piliers, des statues de rois nains, le trône, le coffre
    let (t0, t1, tw0, tw1, tv) = THRONE; for u in t0..=t1 { for w in tw0..=tw1 { g.n(u, 0, w, if (u + w).rem_euclid(2) == 0 { "stone_bricks_dark" } else { "stone_bricks" }); for v in 1..=tv { g.air(u, v, w); } } }
    for u in [-6, 6] { for w in [28, 32] { g.column(u, w, 1, tv - 1); } } for u in [-9, 9] { for w in [29, 33] { g.n(u, 1, w, "statue_base"); g.n(u, 2, w, "statue_top"); } }
    for u in -1..=1 { g.n(u, 1, tw1, "stone_bricks_dark"); } g.t(0, 1, tw1 - 1, "stone_bricks_stairs", 0, 1); g.n(0, 2, tw1, "stone_bricks_dark"); g.n(0, 3, tw1, "brass");
    g.n(CHEST[0], CHEST[1], CHEST[2], "chest"); for (u, w) in [(-9, 27), (9, 27), (-9, 34), (9, 34)] { g.n(u, 1, w, "lantern"); }
    // — L'ESCALIER — de la cour à la gare, en trois volées : on descend vers le sud, on revient vers le nord, puis un boyau plat
    // mène à la gare. Tout est taillé dans un bloc de roche, pour qu'aucune grotte ne s'y ouvre.
    for u in -2..=10 { for w in -33..=-18 { for v in -1..d { let r = g.rock(u, v, w); g.n(u, v, w, r); } } }
    for (u0, u1, w0, w1, floor) in LANDINGS { for u in u0..=u1 { for w in w0..=w1 { g.n(u, floor, w, "stone_bricks_dark"); for v in floor + 1..=floor + 4 { g.air(u, v, w); } } } }
    for (w, top, u0, face) in flights() { for u in u0..=u0 + 2 { g.t(u, top, w, "stone_bricks_stairs", 0, face); for v in top + 1..=top + 4 { g.air(u, v, w); } } }
    for (u, v, w) in [(2, 14, -27), (2, 10, -23), (6, 4, -26), (6, 2, -21), (10, 2, -31), (10, 2, -20)] { g.n(u, v, w, "lantern"); }
    // — EN SURFACE — la porte des nains : un portail de pierre sombre sous un fronton, deux statues ; le chevalement et son wagonnet
    let s = d; for u in -4..=4i32 { for w in -32..=-27 { for v in s + 1..=s + 6 { let wall = u.abs() == 4 || w == -32 || w == -27 || v == s + 6; if wall { g.n(u, v, w, if v == s + 6 || u.abs() == 4 && (w == -32 || w == -27) { "stone_bricks_dark" } else { "stone_bricks" }); } else { g.air(u, v, w); } } if !(u.abs() <= 1 && (-31..=-28).contains(&w)) { g.n(u, s, w, "stone_bricks_dark"); } } }
    for u in -3..=3 { g.n(u, s, -33, "stone_bricks_dark"); } for u in -1..=1 { for v in s + 1..=s + 3 { g.air(u, v, -32); } } for u in -2..=2 { g.n(u, s + 4, -33, "stone_bricks_dark"); } for u in -1..=1 { g.n(u, s + 5, -33, "stone_bricks_dark"); } g.n(0, s + 6, -33, "brass");
    for u in [-3, 3] { g.n(u, s + 1, -33, "statue_base"); g.n(u, s + 2, -33, "statue_top"); } g.n(-2, s + 1, -31, "lantern"); g.n(2, s + 1, -31, "lantern");
    for (u, w) in [(7, -32), (11, -32), (7, -28), (11, -28)] { for v in s + 1..=s + 11 { g.n(u, v, w, "oak_beam"); } } for u in 7..=11 { g.n(u, s + 11, -32, "oak_beam_x"); g.n(u, s + 11, -28, "oak_beam_x"); } for w in -32..=-28 { g.n(9, s + 12, w, "oak_beam_z"); } g.n(9, s + 13, -30, "spinning_wheel");
    for v in s + 3..=s + 10 { g.n(9, v, -30, "chain"); } for w in -27..=-23 { g.t(9, s + 1, w, "rail_z", 0, 1); } g.n(9, s + 1, -25, "minecart"); g.n(9, s + 2, -25, "coal_ore"); g.n(8, s + 1, -30, "crate"); g.n(10, s + 1, -30, "crate");
}

/// Les mécanismes de la mine bâtie en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] - DEPTH; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let ids = |l: Vec<([i32; 3], &'static str)>| l.into_iter().map(|(c, b)| (at(c), by_name(b).unwrap_or(0))).collect::<Vec<_>>();
    let mut m = Mech::default();
    m.wires.push(Wire { name: name("aiguillageA"), inputs: vec![at(SWITCH_A)], ..Default::default() }); m.wires.push(Wire { name: name("aiguillageB"), inputs: vec![at(SWITCH_B)], ..Default::default() });
    m.carts.push(Cart { name: name("wagonnet"), routes: routes().into_iter().map(|r| r.into_iter().map(at).collect()).collect(), switches: vec![name("aiguillageA"), name("aiguillageB")], ..Default::default() });
    for (k, w) in HAMMERS.iter().enumerate() { m.dials.push(Dial { name: name(&format!("marteau{k}")), n: 3, state: 0, goal: 255, draw: (0..3).map(|s| ids(hammer(*w, s))).collect(), auto: 0.3, swing: true, hit: 9.0, t: k as f64 * 0.37, ..Default::default() }); }
    let (rn, ws, crystal) = (runes(o), wheels_start(o), by_name("crystal_blue").unwrap_or(0));
    for (k, wc) in WHEELS.iter().enumerate() { m.dials.push(Dial { name: name(&format!("roue{k}")), n: 4, state: ws[k], goal: rn[k], draw: (0..4).map(|s| glyph(s, *wc, DOOR_W - 1).into_iter().map(|c| (at(c), crystal)).collect()).collect(), levers: vec![(at(WHEEL_LEVERS[k]), 1)], ..Default::default() }); }
    let chain = by_name("great_chain").unwrap_or(0); let door: Vec<([i32; 3], Id)> = (-1..=1).flat_map(|u| (1..=3).map(move |v| [u, v, DOOR_W])).map(|c| (at(c), chain)).collect();
    m.wires.push(Wire { name: name("porte"), needs: (0..3).map(|k| name(&format!("roue{k}"))).collect(), latch: true, doors: door.clone(), ..Default::default() });
    let (t0, t1, tw0, tw1, tv) = THRONE;
    m.guards.push(Guard { min: at([t0 + 1, 1, tw0 + 1]), max: at([t1 - 1, tv - 1, tw1]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: door, state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-20, CHASM.2 - 1, -40]), max: at([20, DEPTH + 14, 36]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-3, 1, -17], [9, 3, -12], "La Mine des nains. Un wagonnet attend à quai : montez dedans, il part sur la voie que désignent les aiguillages. Le levier du quai en manœuvre un ; l'autre est ailleurs.");
    hint(&mut m, [ISLE.0, 1, ISLE.2], [ISLE.1, 3, ISLE.3], "L'îlot du gouffre, et le second aiguillage. Pour repartir, restez dans le wagonnet : il revient à quai.");
    hint(&mut m, [-1, 1, CORRIDOR.0], [1, 3, CORRIDOR.0 + 1], "Le couloir des marteaux-pilons. Passez entre deux coups ; et lisez les runes du mur, dans l'ordre où vous les croisez.");
    hint(&mut m, [-7, 1, ANTE.0], [7, 3, ANTE.1], "La porte runique. Tournez les trois roues sur les runes du couloir, de gauche à droite.");
    m
}

impl Session {
    /// Bâtit ici la mine (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol de la cour.
    pub fn build_dwarf_mine(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("m{}:{}:", o[0], o[2]); if !self.mech.carts.iter().any(|c| c.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Une mine de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn dwarf_mine_wake(&mut self, pl: &Place) { let tag = format!("m{}:{}:", pl.x, pl.z); if self.mech.carts.iter().any(|c| c.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}

/// L'étendue de la mine, de part et d'autre de son centre (x, z) : elle doit tenir dans la portée des empreintes.
pub fn footprint() -> ([i32; 2], [i32; 2]) { let mut p = Plan::new(); carve(&mut p, [0, 80, 0]); p.m.keys().fold(([i32::MAX; 2], [i32::MIN; 2]), |(lo, hi), &(x, _, z)| ([lo[0].min(x), lo[1].min(z)], [hi[0].max(x), hi[1].max(z)])) }
