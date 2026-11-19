//! Le Sanctuaire englouti : le premier donjon, bâti avec les mécanismes génériques (`mecanismes.rs`).
//!
//! Sous une ruine de colonnes moussues, un escalier descend à douze blocs sous terre, dans la salle de l'eau noire. De là :
//! 1. **Les trois leviers** — trois leviers sous des bannières d'or (une, deux, trois) : les abaisser dans l'ordre des
//!    bannières lève la grille ; un faux pas relève tout et fait cracher les masques.
//! 2. **Les deux dalles** — deux blocs de pierre à pousser chacun sur sa dalle ; la grille ne tient levée qu'au poids.
//! 3. **Le couloir des pièges** — deux rangées de pics qui sortent et rentrent (on observe, on passe au bon moment), puis
//!    des masques qui crachent des fléchettes au fond du couloir : on pousse un bloc devant soi pour s'en abriter.
//! 4. **La nef** — une salle haute ; on gravit des degrés le long des murs jusqu'à la galerie (une passerelle friable offre
//!    un raccourci risqué au-dessus du bassin), où un levier lève la grille d'en bas.
//! 5. **L'épreuve** — tout à la fois : un bloc sur la dalle désarme les pics et lève la grille des leviers ; deux leviers
//!    dans l'ordre (une bannière bleue, puis deux) ouvrent la dernière porte.
//! 6. **La chambre de l'idole** — le coffre au trésor sur son autel ; un couloir dérobé et son levier ramènent à l'entrée.
//!
//! Le plan est dessiné en coordonnées relatives (`u` en x, `w` en z, `v` en hauteur depuis le sol des salles) et sert deux
//! fois : posé par la génération du monde (`carve`, sur un `Plan`), et rejoué sur un monde déjà là (`Session::build_sanctuary`,
//! pour les essais et les captures). Ses mécanismes (`mechs`) se branchent quand le joueur approche. Tout est vérifié par
//! `tests/sanctuaire.rs`, salle par salle, avec le solveur de chemins `reach`.
use crate::blocks::{by_name, is_solid, is_water, Id};
use crate::mecanismes::{Guard, Hint, Home, Mech, Pulse, Seal, Wire};
use crate::noise::hash_float;
use crate::player::Voxels;
use crate::session::Session;
use crate::shapes::{facing_of, is_ladder, variant};
use crate::sites::{Place, Plan};
use std::collections::{BTreeSet, VecDeque};

/// Les salles sont à `DEPTH` blocs sous le sol de l'entrée (le niveau `v = 0` est leur sol).
pub const DEPTH: i32 = 12;
/// Le coffre au trésor, sur l'autel de la chambre de l'idole.
pub const CHEST: [i32; 3] = [17, 2, -6];

/// Les salles (intérieur : u0, u1, w0, w1, hauteur).
pub const HALL: (i32, i32, i32, i32, i32) = (-5, 5, 2, 12, 5);
pub const R1: (i32, i32, i32, i32, i32) = (-22, -12, 2, 12, 5);
pub const R2: (i32, i32, i32, i32, i32) = (-22, -12, 16, 26, 5);
pub const R3: (i32, i32, i32, i32, i32) = (-10, 9, 20, 22, 4);
pub const R4: (i32, i32, i32, i32, i32) = (12, 22, 16, 26, 9);
pub const R5: (i32, i32, i32, i32, i32) = (12, 22, 2, 12, 5);
pub const R6: (i32, i32, i32, i32, i32) = (12, 22, -10, -2, 6);

/// Salle 1 : les leviers, dans l'ordre où il faut les abaisser (une bannière, deux, trois), les masques, la grille.
pub const R1_LEVERS: [[i32; 3]; 3] = [[-22, 2, 9], [-22, 2, 5], [-22, 2, 7]];
const R1_DARTS: [[i32; 3]; 2] = [[-17, 2, 1], [-13, 2, 13]];
const R1_GATE: [[i32; 3]; 4] = [[-18, 1, 13], [-17, 1, 13], [-18, 2, 13], [-17, 2, 13]];
/// Salle 2 : les dalles, et les blocs à pousser au départ.
pub const R2_PLATES: [[i32; 3]; 2] = [[-20, 0, 18], [-14, 0, 24]];
pub const R2_BLOCKS: [[i32; 3]; 2] = [[-18, 1, 20], [-16, 1, 22]];
const R2_GATE_W: [i32; 3] = [20, 21, 22];
/// Salle 3 : les rangées de pics (deux rythmes), le bloc d'abri, les masques du fond.
pub const R3_SPIKES: [[i32; 2]; 2] = [[-7, -6], [-3, -2]];
pub const R3_BLOCK: [i32; 3] = [0, 1, 21];
pub const R3_DARTS: [[i32; 3]; 3] = [[10, 1, 20], [10, 1, 21], [10, 1, 22]];
/// Le rythme des pics (période, part du cycle sortie) et des fléchettes (période).
pub const SPIKE_PERIOD: f64 = 3.0; pub const SPIKE_UP: f64 = 0.5; pub const DART_PERIOD: f64 = 1.4;
/// Salle 4 : le levier de la galerie, la grille d'en bas.
pub const R4_LEVER: [i32; 3] = [17, 6, 16];
const R4_GATE: [[i32; 3]; 4] = [[13, 1, 15], [14, 1, 15], [13, 2, 15], [14, 2, 15]];
/// Salle 5 : la dalle et son bloc, le champ de pics, la grille des leviers, les leviers dans l'ordre, la porte.
pub const R5_PLATE: [i32; 3] = [20, 0, 10];
pub const R5_BLOCK: [i32; 3] = [18, 1, 10];
pub const R5_SPIKE_W: [i32; 3] = [6, 7, 8];
pub const R5_LEVERS: [[i32; 3]; 2] = [[19, 2, 2], [15, 2, 2]];
const R5_BARS_W: i32 = 4;
const R5_DARTS: [[i32; 3]; 3] = [[23, 1, 6], [23, 1, 7], [23, 1, 8]];
const R5_TRAP: [i32; 3] = [11, 2, 3];
const R5_GATE: [[i32; 3]; 4] = [[17, 1, 1], [18, 1, 1], [17, 2, 1], [18, 2, 1]];
/// Salle 6 : le levier du raccourci, et sa grille dans le mur nord de la salle de l'eau noire.
pub const R6_LEVER: [i32; 3] = [5, 2, 0];
/// La salle secrète (son sol en `SECRET_FLOOR`), la dalle qui l'ouvre, le pan de mur, son coffre.
pub const SECRET: (i32, i32, i32, i32, i32) = (6, 10, 15, 18, 3); pub const SECRET_FLOOR: i32 = 5;
pub const SECRET_PLATE: [i32; 3] = [12, 5, 17];
const SECRET_PANEL: [[i32; 3]; 4] = [[11, 6, 16], [11, 6, 17], [11, 7, 16], [11, 7, 17]];
pub const SECRET_CHEST: [i32; 3] = [8, 6, 17];
/// Le gardien de la chambre de l'idole, et où il se lève.
pub const BOSS: &str = "colosse"; pub const BOSS_AT: [f64; 3] = [17.5, 1.0, -8.5];
const BOSS_DOORS: [[i32; 3]; 4] = [[17, 1, -1], [18, 1, -1], [17, 2, -1], [18, 2, -1]];
const BOSS_DOORS_W: [[i32; 3]; 4] = [[11, 1, -6], [11, 1, -5], [11, 2, -6], [11, 2, -5]];
const R6_GATE: [[i32; 3]; 4] = [[4, 1, 1], [5, 1, 1], [4, 2, 1], [5, 2, 1]];
/// L'étendue scellée (relative) : la pierre ancienne ne se creuse pas.
const SEAL: ([i32; 3], [i32; 3]) = ([-24, -3, -16], [24, DEPTH - 1, 28]);

/// Le chantier, en coordonnées relatives.
struct S<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl S<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    /// La maçonnerie : pierre de taille, moussue par endroits, plus sombre ailleurs.
    fn stone(&self, u: i32, v: i32, w: i32) -> &'static str { let h = hash_float(self.x + u + 977 * v, self.z + w, 4111, 9); if h < 0.24 { "mossy_stone" } else if h < 0.36 { "stone_bricks_dark" } else { "stone_bricks" } }
    /// Un mur, sauf là où on a déjà creusé.
    fn wall(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); if self.p.m.get(&(x, y, z)) == Some(&-1) { return; } let s = self.stone(u, v, w); self.p.n(x, y, z, s); }
    /// Une salle : son intérieur creusé, son sol, ses murs et sa voûte.
    fn room(&mut self, r: (i32, i32, i32, i32, i32)) { self.room_at(r, 0); }
    /// Une salle dont le sol est à la hauteur `base`.
    fn room_at(&mut self, r: (i32, i32, i32, i32, i32), base: i32) {
        let (u0, u1, w0, w1, h) = r;
        for u in u0 - 1..=u1 + 1 { for w in w0 - 1..=w1 + 1 { for v in 0..=h + 1 { if u >= u0 && u <= u1 && w >= w0 && w <= w1 && v >= 1 && v <= h { self.air(u, base + v, w); } else { self.wall(u, base + v, w); } } } }
    }
    /// Un bassin d'eau noire (deux blocs de fond), creusé dans le sol d'une salle.
    fn basin(&mut self, u0: i32, u1: i32, w0: i32, w1: i32) {
        for u in u0 - 1..=u1 + 1 { for w in w0 - 1..=w1 + 1 { let edge = u < u0 || u > u1 || w < w0 || w > w1; for v in -2..=0 { if edge || v == -2 { if v < 0 { let s = self.stone(u, v, w); let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, s); } } else { self.n(u, v, w, "water"); } } } }
    }
    /// Une grille : barreaux en travers (le long de x) ou en long (le long de z).
    fn bars(&mut self, l: &[[i32; 3]], along_z: bool) { for q in l { if along_z { self.t(q[0], q[1], q[2], "iron_bars", 1, 0); } else { self.n(q[0], q[1], q[2], "iron_bars"); } } }
    /// Une colonne : cannelée (base, fûts, chapiteau) pour « stone_pillar », sinon une pile de `n`.
    fn column(&mut self, u: i32, w: i32, v0: i32, v1: i32, n: &'static str) { for v in v0..=v1 { self.n(u, v, w, if n != "stone_pillar" { n } else if v == v0 { "column_base" } else if v == v1 { "column_capital" } else { "column_shaft" }); } }
    fn statue(&mut self, u: i32, v: i32, w: i32, dx: i32, dz: i32) { self.t(u, v, w, "statue_base", dx, dz); self.t(u, v + 1, w, "statue_top", dx, dz); }
    fn tomb(&mut self, u: i32, v: i32, w: i32) { self.t(u, v, w, "tomb_head", 0, 1); self.t(u + 1, v, w, "tomb_foot", 0, 1); }
}

/// Pose le sanctuaire dans le plan : `o` est le sol de l'entrée (le bloc sur lequel on marche) ; les salles sont dessous.
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut s = S { p, x: o[0], f: o[1] - DEPTH, z: o[2] };
    let top = DEPTH; // le sol de la surface, en v
    // L'escalier : douze marches vers le sud, puis un palier jusqu'à la salle de l'eau noire.
    for k in 0..DEPTH { let w = -(DEPTH + 1) + k; let st = DEPTH - 1 - k;
        for u in -1..=1 { for v in st - 3..st { s.wall(u, v, w); } s.t(u, st, w, "stone_bricks_stairs", 0, -1); let open = if k <= 2 { top + 3 } else { st + 3 }; for v in st + 1..=open { s.air(u, v, w); } if k > 2 { s.wall(u, st + 4, w); } }
        for u in [-2, 2] { for v in st - 1..=(st + 4).min(top) { s.wall(u, v, w); } } }
    s.room((-1, 1, -1, 0, 3)); for u in -1..=1 { for v in 1..=3 { s.air(u, v, 1); } }
    // Le portail, en surface : un parvis dallé, deux colonnes cannelées sous un linteau à frise et son fronton, deux gardiens
    // de pierre qui accueillent le visiteur, deux brasiers ; un muret de part et d'autre de la descente.
    for u in -5..=5 { for w in -18..=-14 { s.n(u, top, w, if (u + w) % 3 == 0 { "mossy_stone" } else { "stone_bricks" }); } }
    for u in [-2, 2] { s.column(u, -14, top + 1, top + 5, "stone_pillar"); } for u in -3..=3 { s.t(u, top + 6, -14, "lintel", 0, 1); }
    for u in [-3, -2, 2, 3] { s.n(u, top + 7, -14, "sandstone"); } for u in -1..=1 { s.t(u, top + 7, -14, "wall_relief", 0, 1); s.n(u, top + 8, -14, "sandstone"); } for u in [-2, 2] { s.n(u, top + 8, -14, "sandstone_slab"); } s.n(0, top + 9, -14, "sandstone");
    for u in [-4, 4] { s.statue(u, top + 1, -14, 0, 1); s.n(u, top + 1, -17, "brazier_stand"); }
    for w in -13..=-11 { for u in [-2, 2] { s.n(u, top + 1, w, "stone_bricks_slab"); } }
    s.n(-4, top + 1, -11, "mossy_stone"); s.n(4, top + 1, -12, "stone_bricks_slab"); s.n(-5, top + 1, -9, "column_shaft");
    // La salle de l'eau noire.
    s.room(HALL); s.basin(-2, 2, 6, 9);
    for (u, w) in [(-4, 4), (4, 4), (-4, 10), (4, 10)] { s.column(u, w, 1, HALL.4, "stone_pillar"); }
    // deux gardiens de pierre au pied de l'escalier ; des bas-reliefs aux murs ; un sarcophage contre le mur sud
    for u in [-2, 2] { s.statue(u, 1, 2, 0, -1); } for w in [3, 5, 9, 11] { s.t(6, 4, w, "wall_relief", 1, 0); } for u in [-4, -2, 2, 4] { s.t(u, 4, 13, "wall_relief", 0, 1); }
    for w in [3, 4, 10, 11] { s.t(-6, 4, w, "wall_relief", -1, 0); } s.tomb(-1, 1, 12); s.n(-4, 1, 12, "funeral_urn"); s.n(4, 1, 12, "funeral_urn");
    for (u, w, dx) in [(-5, 4, -1), (5, 4, 1), (-5, 10, -1), (5, 10, 1)] { s.t(u, 3, w, "wall_sconce", dx, 0); }
    s.t(-3, 4, 2, "banner_blue", 0, -1); s.t(3, 4, 2, "banner_blue", 0, -1); s.n(-3, 1, 12, "brazier"); s.n(3, 1, 12, "brazier");
    for (u, w) in [(-5, 2), (5, 12), (-5, 12)] { s.n(u, HALL.4, w, "cobweb"); }
    // vers la salle des leviers, une arche ; le couloir dérobé arrive au nord-est, derrière sa grille
    s.room((-11, -6, 6, 8, 3));
    // 1 — les trois leviers
    s.room(R1);
    for (k, l) in R1_LEVERS.iter().enumerate() { s.t(l[0], l[1], l[2], "lever", -1, 0); for v in 0..=k as i32 { s.t(l[0], 3 + v, l[2], "banner_gold", -1, 0); } }
    for d in R1_DARTS { s.n(d[0], d[1], d[2], "dart_trap"); }
    s.t(-17, 1, 7, "altar", -1, 0); s.n(-17, 2, 7, "candle"); for w in [4, 10] { s.t(-17, 4, w + if w < 7 { -3 } else { 3 }, "wall_relief", 0, if w < 7 { -1 } else { 1 }); } for (u, w, dx, dz) in [(-12, 3, 1, 0), (-12, 11, 1, 0), (-20, 2, 0, -1), (-14, 12, 0, 1)] { s.t(u, 4, w, "wall_sconce", dx, dz); }
    s.room((-18, -17, 13, 15, 2)); s.bars(&R1_GATE, false);
    // 2 — les deux dalles
    s.room(R2);
    for q in R2_PLATES { s.n(q[0], q[1], q[2], "pressure_plate"); } for q in R2_BLOCKS { s.n(q[0], q[1], q[2], "push_block"); }
    for (u, w) in [(-21, 17), (-13, 17), (-21, 25), (-13, 25)] { s.column(u, w, 1, R2.4, "stone_pillar"); }
    for (u, w, dx, dz) in [(-22, 21, -1, 0), (-17, 26, 0, 1), (-17, 16, 0, -1)] { s.t(u, 4, w, "wall_sconce", dx, dz); }
    s.room(R3);
    let g2: Vec<[i32; 3]> = R2_GATE_W.iter().flat_map(|&w| [[-11, 1, w], [-11, 2, w]]).collect(); s.bars(&g2, true);
    // 3 — le couloir des pièges
    for row in R3_SPIKES { for u in row { for w in R3.2..=R3.3 { s.n(u, 1, w, "spikes_down"); } } }
    for u in [-9, -4, 3, 8] { s.t(u, 3, 19 + 1, "wall_sconce", 0, -1); }
    s.room((8, 9, 23, 25, 3)); s.room((10, 11, 24, 25, 3));
    s.n(R3_BLOCK[0], R3_BLOCK[1], R3_BLOCK[2], "push_block"); for d in R3_DARTS { s.n(d[0], d[1], d[2], "dart_trap"); }
    // 4 — la nef
    s.room(R4); s.basin(14, 20, 19, 22);
    for u in R4.0..=R4.1 { for w in 16..=17 { s.n(u, 5, w, "stone_bricks"); } }
    s.t(R4_LEVER[0], R4_LEVER[1], R4_LEVER[2], "lever", 0, -1);
    // les degrés le long du mur sud, puis la corniche le long du mur est, jusqu'à la galerie
    for (k, u) in (13..=16).enumerate() { s.column(u, 26, 1, k as i32 + 1, "stone_bricks"); }
    for u in 17..=21 { s.column(u, 26, 1, 4, "stone_bricks"); } s.column(22, 26, 1, 5, "stone_bricks"); for w in 18..=25 { s.n(22, 5, w, "mossy_stone"); }
    // la passerelle friable : un raccourci au-dessus du bassin
    for w in 18..=25 { s.n(16, 4, w, "crumbling_floor"); }
    for (u, w, dx, dz) in [(12, 20, -1, 0), (22, 20, 1, 0), (17, 26, 0, 1)] { s.t(u, 7, w, "wall_sconce", dx, dz); } for v in 6..=R4.4 { s.n(17, v, 21, "chain"); } s.n(17, 5, 21, "lantern");
    s.room((13, 14, 13, 15, 2)); s.bars(&R4_GATE, false);
    // la salle secrète, derrière le mur ouest de la galerie : une dalle à l'angle de la galerie fait pivoter un pan de mur
    s.room_at(SECRET, SECRET_FLOOR); s.n(SECRET_PLATE[0], SECRET_PLATE[1], SECRET_PLATE[2], "pressure_plate"); for q in SECRET_PANEL { s.n(q[0], q[1], q[2], "mossy_stone"); }
    s.n(SECRET_CHEST[0], SECRET_CHEST[1], SECRET_CHEST[2], "chest"); s.n(9, SECRET_FLOOR + 1, 15, "candle"); s.n(7, SECRET_FLOOR + 1, 18, "candle"); s.n(6, SECRET_FLOOR + 1, 15, "amphora"); s.n(10, SECRET_FLOOR + 1, 18, "bone");
    s.t(8, SECRET_FLOOR + 3, 18, "banner_gold", 0, 1); s.n(6, SECRET_FLOOR + 3, 18, "cobweb");
    // 5 — l'épreuve
    s.room(R5);
    s.n(R5_PLATE[0], R5_PLATE[1], R5_PLATE[2], "pressure_plate"); s.n(R5_BLOCK[0], R5_BLOCK[1], R5_BLOCK[2], "push_block");
    for w in R5_SPIKE_W { for u in R5.0..=R5.1 { s.n(u, 1, w, "spikes"); } } for d in R5_DARTS { s.n(d[0], d[1], d[2], "dart_trap"); }
    for u in R5.0..=R5.1 { for v in 1..=R5.4 { s.n(u, v, R5_BARS_W, "iron_bars"); } }
    s.room((17, 18, -1, 1, 2)); s.bars(&R5_GATE, false);
    for (k, l) in R5_LEVERS.iter().enumerate() { s.t(l[0], l[1], l[2], "lever", 0, -1); for v in 0..=k as i32 { s.t(l[0], 3 + v, l[2], "banner_blue", 0, -1); } }
    s.n(R5_TRAP[0], R5_TRAP[1], R5_TRAP[2], "dart_trap"); for (u, w, dx, dz) in [(12, 10, -1, 0), (22, 11, 1, 0), (12, 3, -1, 0)] { s.t(u, 3, w, "wall_sconce", dx, dz); }
    // 6 — la chambre de l'idole
    s.room(R6); s.basin(13, 13, -9, -3); s.basin(21, 21, -9, -3);
    s.t(CHEST[0], 1, CHEST[2], "altar", 0, -1); s.tomb(14, 1, -10); s.tomb(19, 1, -10); for u in [13, 21] { s.statue(u, 1, -2, 0, 1); } s.n(CHEST[0], CHEST[1], CHEST[2], "chest"); for (u, w) in [(16, -6), (18, -6), (17, -5), (17, -7)] { s.n(u, 1, w, "candle"); }
    for (u, w) in [(15, -9), (19, -9), (15, -3), (19, -3)] { s.column(u, w, 1, R6.4, "stone_pillar"); } for u in [15, 19] { s.t(u, 4, -10, "banner_gold", 0, -1); } s.n(17, R6.4, -6, "chandelier");
    for (u, w) in [(12, -10), (22, -10)] { s.n(u, 1, w, "brazier"); }
    // le couloir dérobé, son levier, sa grille
    s.room((6, 11, -6, -5, 3)); s.room((4, 5, -6, 0, 3)); s.t(R6_LEVER[0], R6_LEVER[1], R6_LEVER[2], "lever", 1, 0); s.bars(&R6_GATE, false);
    s.n(5, 3, -6, "cobweb");
}

/// Les mécanismes du sanctuaire bâti en `o`, leurs noms préfixés de `tag` (un par sanctuaire).
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] - DEPTH; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let ats = |l: &[[i32; 3]]| -> Vec<[i32; 3]> { l.iter().map(|q| at(*q)).collect() };
    let iron = by_name("iron_bars").unwrap_or(0); let iron_z = variant(iron, facing_of(1.0, 0.0), false, false, false);
    let gate = |l: &[[i32; 3]], b: Id| -> Vec<([i32; 3], Id)> { l.iter().map(|q| (at(*q), b)).collect() };
    let box_of = |r: (i32, i32, i32, i32, i32)| (at([r.0, 1, r.2]), at([r.1, r.4, r.3]));
    let name = |n: &str| format!("{tag}{n}");
    let mut m = Mech::default();
    // 1
    m.wires.push(Wire { name: name("r1"), inputs: ats(&R1_LEVERS), seq: true, latch: true, fail_darts: true, darts: ats(&R1_DARTS), doors: gate(&R1_GATE, iron), ..Default::default() });
    // 2
    let g2: Vec<[i32; 3]> = R2_GATE_W.iter().flat_map(|&w| [[-11, 1, w], [-11, 2, w]]).collect();
    m.wires.push(Wire { name: name("r2"), inputs: ats(&R2_PLATES), all: true, doors: gate(&g2, iron_z), ..Default::default() });
    let (a, b) = box_of(R2); m.homes.push(Home { min: a, max: b, homes: ats(&R2_BLOCKS), keep: name("r2") });
    // 3
    for (k, row) in R3_SPIKES.iter().enumerate() { let sp: Vec<[i32; 3]> = row.iter().flat_map(|&u| (R3.2..=R3.3).map(move |w| [u, 1, w])).collect();
        m.pulses.push(Pulse { spikes: ats(&sp), period: SPIKE_PERIOD, up: SPIKE_UP, phase: k as f64 * SPIKE_PERIOD / 2.0, ..Default::default() }); }
    m.pulses.push(Pulse { darts: ats(&R3_DARTS), period: DART_PERIOD, up: 0.0, off_by: name("r4"), ..Default::default() });
    let (a, b) = box_of(R3); m.homes.push(Home { min: a, max: b, homes: vec![at(R3_BLOCK)], keep: String::new() });
    // 4
    m.wires.push(Wire { name: name("r4"), inputs: vec![at(R4_LEVER)], latch: true, doors: gate(&R4_GATE, iron), ..Default::default() });
    // 5
    let field: Vec<[i32; 3]> = R5_SPIKE_W.iter().flat_map(|&w| (R5.0..=R5.1).map(move |u| [u, 1, w])).collect();
    let row: Vec<[i32; 3]> = (R5.0..=R5.1).flat_map(|u| (1..=R5.4).map(move |v| [u, v, R5_BARS_W])).collect();
    m.wires.push(Wire { name: name("r5a"), inputs: vec![at(R5_PLATE)], latch: true, calm: ats(&field), doors: gate(&row, iron), ..Default::default() });
    m.wires.push(Wire { name: name("r5b"), inputs: ats(&R5_LEVERS), seq: true, latch: true, needs: vec![name("r5a")], fail_darts: true, darts: vec![at(R5_TRAP)], doors: gate(&R5_GATE, iron), ..Default::default() });
    m.pulses.push(Pulse { darts: ats(&R5_DARTS), period: 1.6, up: 0.0, off_by: name("r5a"), ..Default::default() });
    let (a, b) = box_of(R5); m.homes.push(Home { min: a, max: b, homes: vec![at(R5_BLOCK)], keep: name("r5a") });
    // 6
    m.wires.push(Wire { name: name("r6"), inputs: vec![at(R6_LEVER)], latch: true, doors: gate(&R6_GATE, iron), ..Default::default() });
    m.chests.push(at(CHEST));
    // la salle secrète : la dalle de la galerie fait pivoter le pan de mur, pour de bon
    let stone = by_name("mossy_stone").unwrap_or(0); m.wires.push(Wire { name: name("secret"), inputs: vec![at(SECRET_PLATE)], latch: true, doors: gate(&SECRET_PANEL, stone), ..Default::default() });
    m.secrets.push(at(SECRET_CHEST));
    // le colosse englouti garde l'idole : il s'éveille quand on entre, les deux issues se ferment
    let mut d = gate(&BOSS_DOORS, iron); d.extend(gate(&BOSS_DOORS_W, iron_z)); let (a, b) = box_of(R6);
    m.guards.push(Guard { min: a, max: b, species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: d, state: 0 });
    // la pierre ancienne
    m.seals.push(Seal { min: at(SEAL.0), max: at(SEAL.1), until: String::new() });
    // les inscriptions
    for (r, t) in [(HALL, "Le Sanctuaire englouti. Sous les voûtes dort une eau noire ; au fond, une idole attend qui saura faire parler les pierres. (Clic droit : abaisser un levier, pousser un bloc.)"),
        (R1, "Trois leviers sous des bannières d'or. Les anciens comptaient : une, puis deux, puis trois."),
        (R2, "Deux dalles dans le sol, deux blocs de pierre : la grille ne se lève que sous le poids."),
        (R3, "Les pics se lèvent et retombent, les masques du fond crachent des fléchettes. Observez le rythme… et poussez la pierre devant vous."),
        (R4, "Une nef noyée d'ombre. Tout là-haut, sur la galerie, un levier."),
        (R5, "L'épreuve : le poids, le piège, l'ordre — une bannière bleue, puis deux."),
        (R6, "La chambre de l'idole. Quelque chose remue sous l'eau noire… Un couloir dérobé ramène à l'entrée.")] { let (a, b) = box_of(r); m.hints.push(Hint { min: a, max: b, text: t.into(), shown: false, xp: 0.0 }); }
    let (a, b) = (at([SECRET.0, SECRET_FLOOR + 1, SECRET.2]), at([SECRET.1, SECRET_FLOOR + SECRET.4, SECRET.3])); m.hints.push(Hint { min: a, max: b, text: "Une salle secrète ! Les bâtisseurs y ont caché leurs offrandes.".into(), shown: false, xp: crate::niveaux::XP_PLACE * 1.5 });
    m
}

/// Le solveur de chemins : les cases où le joueur peut se tenir (les pieds), depuis `from`, sans sauter plus d'un bloc de haut
/// (ni plus d'un trou d'un bloc de large),
/// sans grimper (compétence Escalade) ni casser ou poser quoi que ce soit, dans la boîte `min`..`max`. On marche, on monte
/// d'un bloc si la tête passe, on se laisse tomber (huit blocs au plus), on monte et descend les échelles, on nage. Le sol
/// friable ne compte pas comme un appui (il cède) ; les pics comptent comme passables (ils blessent, sans arrêter).
pub fn reach(w: &impl Voxels, from: [i32; 3], min: [i32; 3], max: [i32; 3]) -> BTreeSet<[i32; 3]> {
    let crumble = by_name("crumbling_floor").unwrap_or(0);
    let get = |p: [i32; 3]| w.get(p[0], p[1], p[2]).unwrap_or(1);
    let pass = |p: [i32; 3]| { let b = get(p); b == 0 || !is_solid(b) };
    let inb = |p: [i32; 3]| (0..3).all(|i| p[i] >= min[i] && p[i] <= max[i]);
    let stand = |p: [i32; 3]| { if !inb(p) || !pass(p) || !pass([p[0], p[1] + 1, p[2]]) { return false; } let below = get([p[0], p[1] - 1, p[2]]); let here = get(p);
        (is_solid(below) && below != crumble) || is_ladder(here) || is_water(here) || is_water(below) };
    let mut seen = BTreeSet::new(); let mut q = VecDeque::new(); if stand(from) { seen.insert(from); q.push_back(from); }
    while let Some(c) = q.pop_front() {
        let mut next = Vec::new(); let up = [c[0], c[1] + 1, c[2]]; let head = [c[0], c[1] + 2, c[2]];
        let climb = is_ladder(get(c)) || is_water(get(c));
        if climb && stand(up) { next.push(up); }
        if climb || is_ladder(get([c[0], c[1] - 1, c[2]])) { let d = [c[0], c[1] - 1, c[2]]; if stand(d) { next.push(d); } }
        for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let n = [c[0] + dx, c[1], c[2] + dz];
            if stand(n) { next.push(n); continue; }
            let n1 = [n[0], n[1] + 1, n[2]]; if pass(head) && stand(n1) { next.push(n1); continue; }
            // un saut par-dessus un trou d'un bloc, au même niveau ou un plus bas (en courant, on franchit deux blocs)
            if pass(n) && pass(n1) && pass(head) && pass([n[0], n[1] + 2, n[2]]) && !stand(n) { let t = [c[0] + 2 * dx, c[1], c[2] + 2 * dz]; let t2 = [t[0], t[1] + 2, t[2]];
                if stand(t) && pass(t2) { next.push(t); } else { let t1 = [t[0], t[1] - 1, t[2]]; if pass(t) && stand(t1) && !stand([n[0], n[1] - 1, n[2]]) { next.push(t1); } } }
            if pass(n) && pass(n1) { for k in 1..=8 { let d = [n[0], n[1] - k, n[2]]; if !inb(d) || !pass(d) { break; } if stand(d) { next.push(d); break; } } }
        }
        for n in next { if seen.insert(n) { q.push_back(n); } }
    }
    seen
}

impl Session {
    /// Bâtit le sanctuaire ici, sur le monde déjà chargé (essais, captures), et branche ses mécanismes. `o` : le sol de l'entrée.
    pub fn build_sanctuary(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("s{}:{}:", o[0], o[2]); if !self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); }
        self.dirty = true;
    }
    /// Un sanctuaire de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn sanctuary_wake(&mut self, pl: &Place) {
        let tag = format!("s{}:{}:", pl.x, pl.z); if self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { return; }
        self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true;
    }
}
