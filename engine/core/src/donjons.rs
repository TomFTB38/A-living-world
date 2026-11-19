//! La bibliothèque de salles, et les donjons assemblés au hasard : les « ruines englouties ».
//!
//! Chaque salle de la bibliothèque est dessinée une fois, dans son repère : 11 × 11 blocs à l'intérieur, l'entrée au sud
//! (`z = -6`), la sortie au nord, à l'est ou à l'ouest selon le tour que prend le donjon. Posée, elle est tournée pour que
//! son entrée regarde d'où l'on vient. Chaque salle sait aussi se résoudre (`Act`) : les tests rejouent ces solutions, et le
//! solveur de chemins (`sanctuaire::reach`) vérifie qu'avant, la salle suivante est hors d'atteinte, et après, qu'on y va.
//!
//! Le générateur trace un chemin sur une grille de 4 × 4 mailles de 14 blocs (sept salles, sans se recouper), puis distribue
//! les salles : l'entrée (un puits à échelle descend de la surface), quatre épreuves tirées de la bibliothèque, le gardien,
//! le trésor. Une même graine donne toujours le même donjon.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Guard, Hint, Home, Mech, Pulse, Seal, Wire};
use crate::noise::{hash_float, hash_int, Rng};
use crate::sanctuaire::DEPTH;
use crate::session::Session;
use crate::shapes::{facing_of, variant};
use crate::sites::{spin_xz, Plan};

/// Les salles de la bibliothèque.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind { Entree, Leviers, Dalles, Pics, Nef, Epreuve, Gardien, Tresor }
impl Kind {
    pub fn name(self) -> &'static str { match self { Kind::Entree => "l'entrée", Kind::Leviers => "les leviers", Kind::Dalles => "les dalles", Kind::Pics => "les pics", Kind::Nef => "la nef", Kind::Epreuve => "l'épreuve", Kind::Gardien => "le gardien", Kind::Tresor => "le trésor" } }
    /// La hauteur sous voûte.
    pub fn h(self) -> i32 { match self { Kind::Entree => DEPTH - 1, Kind::Nef => 9, Kind::Gardien => 6, _ => 5 } }
    /// Une grille ferme-t-elle la sortie tant que l'épreuve n'est pas résolue ?
    pub fn gated(self) -> bool { matches!(self, Kind::Leviers | Kind::Dalles | Kind::Nef | Kind::Epreuve | Kind::Gardien) }
    /// Les sorties possibles, dans le repère de la salle : en face (0, 1), à droite (1, 0) ou à gauche (-1, 0).
    fn exits(self) -> &'static [(i32, i32)] { match self { Kind::Epreuve => &[(0, 1)], _ => &[(0, 1), (1, 0), (-1, 0)] } }
}
/// Les épreuves qu'on tire pour le milieu du donjon.
pub const TRIALS: [Kind; 5] = [Kind::Leviers, Kind::Dalles, Kind::Pics, Kind::Nef, Kind::Epreuve];
/// Le pas de la grille, le nombre de mailles par côté, le nombre de salles.
pub const PITCH: i32 = 14; pub const GRID: i32 = 4; pub const ROOMS: usize = 7;

/// Une salle posée : son genre, son centre (relatif au donjon), son quart de tour, sa sortie (dans le monde).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Room { pub kind: Kind, pub c: (i32, i32), pub q: u8, pub exit: Option<(i32, i32)> }
impl Room {
    /// Un point de la salle, de son repère au donjon (`v` depuis le sol des salles).
    pub fn at(&self, l: [i32; 3]) -> [i32; 3] { let (u, w) = spin_xz(l[0], l[2], self.q); [self.c.0 + u, l[1], self.c.1 + w] }
    /// Une direction de la salle, dans le donjon.
    fn dir(&self, d: (i32, i32)) -> (i32, i32) { spin_xz(d.0, d.1, self.q) }
    /// La sortie, dans le repère de la salle.
    fn exit_local(&self) -> Option<(i32, i32)> { self.exit.map(|d| spin_xz(d.0, d.1, (4 - self.q) % 4)) }
    /// Où l'on se tient en entrant.
    pub fn spot(&self) -> [i32; 3] { self.at(if self.kind == Kind::Entree { [-2, 1, -4] } else { [0, 1, -4] }) }
    /// Sur le parvis du portail de l'entrée, quand le sol est plat au-dessus (le haut de l'escalier est alors en `z = 6`).
    pub fn porch(&self) -> [i32; 3] { self.at([-5, DEPTH + 1, 9]) }
}
/// Un donjon : ses salles dans l'ordre, et ses tirages.
#[derive(Clone, Debug)]
pub struct Layout { pub rooms: Vec<Room>, pub seed: u32, pub boss: &'static str }

/// Le donjon de cette graine.
pub fn layout(seed: u32) -> Layout {
    let mut r = Rng::new(seed ^ 0x5eed_d07e);
    // le chemin : une marche sur la grille qui ne se recoupe pas
    let path = 'walk: loop {
        let mut p = vec![(r.int(0, GRID - 1), r.int(0, GRID - 1))];
        while p.len() < ROOMS { let (x, z) = *p.last().unwrap(); let mut n: Vec<(i32, i32)> = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().map(|d| (x + d.0, z + d.1)).filter(|c| c.0 >= 0 && c.0 < GRID && c.1 >= 0 && c.1 < GRID && !p.contains(c)).collect();
            if n.is_empty() { continue 'walk; } let k = (r.next() * n.len() as f64) as usize % n.len(); p.push(n.swap_remove(k)); }
        break p;
    };
    let center = |c: (i32, i32)| (-(GRID - 1) * PITCH / 2 + c.0 * PITCH, -(GRID - 1) * PITCH / 2 + c.1 * PITCH);
    let step = |a: (i32, i32), b: (i32, i32)| (b.0 - a.0, b.1 - a.1);
    // le quart de tour qui amène (0, 1) sur la direction `d`
    let q_of = |d: (i32, i32)| (0..4u8).find(|&q| spin_xz(0, 1, q) == d).unwrap();
    // les épreuves, mélangées, chacune là où sa sortie le permet
    let mut pool: Vec<Kind> = TRIALS.to_vec(); for i in (1..pool.len()).rev() { let j = (r.next() * (i + 1) as f64) as usize % (i + 1); pool.swap(i, j); }
    let mut rooms = Vec::new();
    for (i, &c) in path.iter().enumerate() {
        let d_in = if i == 0 { step(c, path[1]) } else { step(path[i - 1], c) }; let q = q_of(d_in); let exit = path.get(i + 1).map(|n| step(c, *n));
        let local = exit.map(|d| spin_xz(d.0, d.1, (4 - q) % 4));
        let kind = if i == 0 { Kind::Entree } else if i == ROOMS - 1 { Kind::Tresor } else if i == ROOMS - 2 { Kind::Gardien } else {
            let k = pool.iter().position(|k| local.is_none_or(|l| k.exits().contains(&l))).unwrap_or(0); pool.remove(k) };
        rooms.push(Room { kind, c: center(c), q, exit });
    }
    let boss = if r.next() < 0.5 { "colosse" } else { "gardien" };
    Layout { rooms, seed, boss }
}

/// Un geste qui résout (une partie d') une salle : abaisser un levier, pousser un bloc depuis une case, aller jusqu'à une
/// case (qu'on doit pouvoir atteindre), vaincre le gardien.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Act { Lever([i32; 3]), Push { from: [i32; 3], block: [i32; 3] }, Reach([i32; 3]), Slay }

/// Ce que contient une salle une fois tirée : les leviers dans l'ordre, les blocs et leurs dalles…
struct Draw { r: Rng }
impl Draw { fn pick<T: Copy>(&mut self, l: &[T]) -> T { l[(self.r.next() * l.len() as f64) as usize % l.len()] } }

/// Le chantier d'une salle : on dessine dans son repère, on pose dans le donjon.
struct B<'a> { p: &'a mut Plan, o: [i32; 3], room: Room }
impl B<'_> {
    fn w(&self, l: [i32; 3]) -> (i32, i32, i32) { let a = self.room.at(l); (self.o[0] + a[0], self.o[1] - DEPTH + a[1], self.o[2] + a[2]) }
    fn stone(&self, x: i32, y: i32, z: i32) -> &'static str { let h = hash_float(x + 977 * y, z, 4111, 9); if h < 0.24 { "mossy_stone" } else if h < 0.36 { "stone_bricks_dark" } else { "stone_bricks" } }
    fn n(&mut self, l: [i32; 3], n: &'static str) { let (x, y, z) = self.w(l); self.p.n(x, y, z, n); }
    /// Un bloc orienté : `back` est son dos (le mur d'un levier, d'une bannière), dans le repère de la salle.
    fn t(&mut self, l: [i32; 3], n: &'static str, back: (i32, i32)) { let (x, y, z) = self.w(l); let d = self.room.dir(back); self.p.turn(x, y, z, n, d.0, d.1); }
    fn air(&mut self, l: [i32; 3]) { let (x, y, z) = self.w(l); self.p.air(x, y, z); }
    fn wall(&mut self, l: [i32; 3]) { let (x, y, z) = self.w(l); if self.p.m.get(&(x, y, z)) == Some(&-1) { return; } let s = self.stone(x, y, z); self.p.n(x, y, z, s); }
    /// Les barreaux d'une porte percée dans le mur de côté `side` (repère de la salle).
    fn bars_id(&self, side: (i32, i32)) -> Id { let d = self.room.dir(side); variant(by_name("iron_bars").unwrap_or(0), if d.0 != 0 { facing_of(1.0, 0.0) } else { 0 }, false, false, false) }
    /// Une colonne cannelée de `v0` à `v1` : sa base, ses fûts, son chapiteau.
    fn column(&mut self, u: i32, w: i32, v0: i32, v1: i32) { for v in v0..=v1 { self.n([u, v, w], if v == v0 { "column_base" } else if v == v1 { "column_capital" } else { "column_shaft" }); } }
    /// Une statue de gardien (deux blocs), le dos vers `back`.
    fn statue(&mut self, u: i32, v: i32, w: i32, back: (i32, i32)) { self.t([u, v, w], "statue_base", back); self.t([u, v + 1, w], "statue_top", back); }
    /// Un sarcophage : la tête en (u, w), les pieds en (u + 1, w).
    fn tomb(&mut self, u: i32, v: i32, w: i32) { self.t([u, v, w], "tomb_head", (0, 1)); self.t([u + 1, v, w], "tomb_foot", (0, 1)); }
    fn door(&self, side: (i32, i32), v: i32) -> Vec<[i32; 3]> { let perp = (side.1.abs(), side.0.abs()); (-1..=1).map(|k| [side.0 * 6 + perp.0 * k, v, side.1 * 6 + perp.1 * k]).collect() }
}

/// Les mécanismes et la solution d'une salle.
struct Out { m: Mech, acts: Vec<Act> }

/// Pose une salle et rend ses mécanismes et sa solution (en coordonnées du monde). `name` préfixe ses fils.
fn build_room(p: &mut Plan, o: [i32; 3], room: Room, k: usize, name: &str, boss: &'static str, seed: u32, ground: &dyn Fn(i32, i32) -> i32) -> Out {
    let mut b = B { p, o, room }; let h = room.kind.h(); let mut d = Draw { r: Rng::new(hash_int(room.c.0, room.c.1, seed as i32, 0x51) ^ k as u32) };
    let mut m = Mech::default(); let mut acts = Vec::new();
    let at = |b: &B, l: [i32; 3]| { let (x, y, z) = b.w(l); [x, y, z] };
    // la coque : sol, murs, voûte ; l'intérieur creusé
    for u in -6..=6i32 { for w in -6..=6i32 { for v in 0..=h + 1 { if u.abs() <= 5 && w.abs() <= 5 && v >= 1 && v <= h { b.air([u, v, w]); } else { b.wall([u, v, w]); } } } }
    for (u, w, back) in [(5, 4, (1, 0)), (-5, 4, (-1, 0)), (5, -4, (1, 0)), (-5, -4, (-1, 0))] { b.t([u, 3, w], "wall_sconce", back); }
    // des bas-reliefs dans les murs, de part et d'autre des portes
    for side in [(0i32, 1i32), (1, 0), (-1, 0), (0, -1)] { let perp = (side.1.abs(), side.0.abs()); for k in [-3, 3] { b.t([side.0 * 6 + perp.0 * k, 3, side.1 * 6 + perp.1 * k], "wall_relief", side); } }
    let exit = room.exit_local(); let gate = |b: &B| exit.map(|s| { let id = b.bars_id(s); (1..=3).flat_map(|v| b.door(s, v)).map(|l| (at(b, l), id)).collect::<Vec<_>>() }).unwrap_or_default();
    let wire = |m: &mut Mech, w: Wire| m.wires.push(Wire { name: format!("{name}{}", w.name), ..w });
    let (lo, hi) = (at(&b, [-5, 1, -5]), at(&b, [5, h, 5])); let (lo, hi) = ([lo[0].min(hi[0]), lo[1], lo[2].min(hi[2])], [lo[0].max(hi[0]), hi[1], lo[2].max(hi[2])]);
    let hint = |m: &mut Mech, t: &str| m.hints.push(Hint { min: lo, max: hi, text: t.into(), shown: false, xp: 0.0 });
    match room.kind {
        Kind::Entree => {
            // Le grand escalier : deux de large contre le mur ouest, du jour (au nord) jusqu'au sol d'une salle haute de onze blocs.
            // Là où le sol est plus haut, il se prolonge dans une tranchée murée jusqu'à la surface.
            let rel = |b: &B, u: i32, w: i32| { let (x, _, z) = b.w([u, 0, w]); ground(x, z) - (b.o[1] - DEPTH) };
            let mut zt = 6; while zt < 13 && 6 + zt < rel(&b, -5, zt + 1).max(rel(&b, -4, zt + 1)) { zt += 1; }
            for z in -5..=zt { let top = 6 + z;
                for u in [-5, -4] { b.t([u, top, z], "stone_bricks_stairs", (0, 1)); if z.abs() >= 2 && z <= 5 { for v in 1..top { b.n([u, v, z], "stone_bricks"); } }
                    let open = if z >= 3 { rel(&b, u, z).max(top + 3) + 2 } else { top + 3 }; for v in top + 1..=open { b.air([u, v, z]); } }
                if z > 5 { for u in [-6, -3] { for v in top - 1..=rel(&b, u, z) { b.wall([u, v, z]); } } } }
            // la salle : deux colonnes cannelées, deux gardiens de pierre, un sarcophage, des brasiers, un lustre ; un bassin
            b.column(2, -3, 1, h); b.column(2, 3, 1, h); b.statue(5, 1, -4, (1, 0)); b.statue(5, 1, 4, (1, 0)); b.tomb(0, 1, -5); b.n([3, 1, -5], "funeral_urn");
            b.n([3, 1, 5], "brazier_stand"); b.n([-2, 1, 5], "brazier_stand"); for v in h - 2..=h { b.n([0, v, 0], "chain"); } b.n([0, h - 3, 0], "chandelier");
            for (u, w, back) in [(5, 0, (1, 0)), (-1, 5, (0, 1)), (0, -5, (0, -1))] { b.t([u, 7, w], "wall_sconce", back); }
            for side in [(1i32, 0i32), (0, -1)] { let perp = (side.1.abs(), side.0.abs()); for k in [-4, -2, 2, 4] { b.t([side.0 * 6 + perp.0 * k, 7, side.1 * 6 + perp.1 * k], "wall_relief", side); } }
            for u in -1..=1 { for w in 0..=2 { b.n([u, 0, w], "water"); b.n([u, -1, w], "stone_bricks"); } }
            // Le portail, au sommet de l'escalier : un parvis dallé, deux colonnes, un linteau à frise sous un fronton, deux statues
            // qui accueillent le visiteur, deux brasiers ; un muret borde la trémie au-dessus des marches.
            let g = 6 + zt; let zp = zt + 1;
            for u in -8..=-1 { for w in zp..=zp + 3 { b.n([u, g, w], if (u + w) % 3 == 0 { "mossy_stone" } else { "stone_bricks" }); for v in g - 3..g { b.wall([u, v, w]); } for v in g + 1..=g + 9 { b.air([u, v, w]); } } }
            for u in [-6, -3] { b.column(u, zp, g + 1, g + 5); } for u in -7..=-2 { b.t([u, g + 6, zp], "lintel", (0, -1)); }
            b.t([-6, g + 7, zp], "stone_bricks_stairs", (1, 0)); b.n([-5, g + 7, zp], "stone_bricks"); b.n([-4, g + 7, zp], "stone_bricks"); b.t([-3, g + 7, zp], "stone_bricks_stairs", (-1, 0));
            b.t([-5, g + 8, zp], "stone_bricks_stairs", (1, 0)); b.t([-4, g + 8, zp], "stone_bricks_stairs", (-1, 0));
            for u in [-8, -1] { b.statue(u, g + 1, zp, (0, -1)); b.n([u, g + 1, zp + 3], "brazier_stand"); }
            for w in 3..zp { for u in [-6, -3] { let gg = rel(&b, u, w).max(if w <= 5 { DEPTH } else { 0 }); b.n([u, gg + 1, w], "stone_bricks_slab"); } }
            hint(&mut m, "Des ruines englouties. Les salles s'enchaînent sous la terre ; au bout, un trésor et son gardien. (Clic droit : abaisser un levier, pousser un bloc.)");
        }
        Kind::Leviers => {
            // trois leviers contre le mur d'entrée, sous une, deux, trois bannières : l'ordre est celui des bannières
            let mut spots = vec![-5, -4, -3, 3, 4, 5]; for i in (1..spots.len()).rev() { let j = (d.r.next() * (i + 1) as f64) as usize % (i + 1); spots.swap(i, j); } spots.truncate(3);
            let color = d.pick(&["banner_gold", "banner_blue", "banner_red"]);
            let levers: Vec<[i32; 3]> = spots.iter().map(|&u| [u, 2, -5]).collect();
            for (n, l) in levers.iter().enumerate() { b.t(*l, "lever", (0, -1)); for v in 0..=n as i32 { b.t([l[0], 3 + v, -5], color, (0, -1)); } }
            for u in [-2, 2] { b.n([u, 2, -6], "dart_trap"); } b.t([0, 1, 1], "altar", (0, 1)); b.n([0, 2, 1], "candle"); b.n([5, 1, 5], "funeral_urn"); b.n([-5, 1, 5], "funeral_urn");
            let g = gate(&b); wire(&mut m, Wire { name: "lev".into(), inputs: levers.iter().map(|l| at(&b, *l)).collect(), seq: true, latch: true, fail_darts: true, darts: vec![at(&b, [-2, 2, -6]), at(&b, [2, 2, -6])], doors: g, ..Default::default() });
            for l in &levers { acts.push(Act::Lever(at(&b, *l))); }
            hint(&mut m, "Des leviers sous des bannières : une, puis deux, puis trois.");
        }
        Kind::Dalles => {
            // deux blocs, chacun à pousser droit devant jusqu'à sa dalle
            let cols = [d.pick(&[-4, -3]), d.pick(&[3, 4])]; let mut homes = Vec::new(); let mut plates = Vec::new();
            for &u in &cols { let k = d.r.int(2, 4); let (bl, pl) = ([u, 1, -2], [u, 0, -2 + k]); b.n(bl, "push_block"); b.n(pl, "pressure_plate"); homes.push(at(&b, bl)); plates.push(at(&b, pl));
                for s in 0..k { acts.push(Act::Push { from: at(&b, [u, 1, -3 + s]), block: at(&b, [u, 1, -2 + s]) }); } }
            for (u, w) in [(-5, 5), (5, 5)] { b.column(u, w, 1, h); }
            let g = gate(&b); wire(&mut m, Wire { name: "dal".into(), inputs: plates, all: true, doors: g, ..Default::default() });
            m.homes.push(Home { min: lo, max: hi, homes, keep: format!("{name}dal") });
            hint(&mut m, "Deux dalles, deux blocs de pierre : la grille ne se lève que sous le poids.");
        }
        Kind::Pics => {
            // deux rangées de pics qui sortent tour à tour ; une bande de sol friable dans un coin
            let period = d.pick(&[2.6, 3.0, 3.4]);
            for (n, w) in [-2, 1].into_iter().enumerate() { let row: Vec<[i32; 3]> = (-5..=5).map(|u| [u, 1, w]).collect(); for l in &row { b.n(*l, "spikes_down"); }
                m.pulses.push(Pulse { spikes: row.iter().map(|l| at(&b, *l)).collect(), period, up: 0.5, phase: n as f64 * period / 2.0, ..Default::default() }); }
            for u in -5..=-4 { for w in 3..=4 { b.n([u, 0, w], "crumbling_floor"); b.air([u, -1, w]); b.wall([u, -2, w]); } }
            for (u, w) in [(5, 5), (5, -5), (-5, -5)] { b.n([u, 1, w], "funeral_urn"); }
            hint(&mut m, "Les pics se lèvent et retombent : observez le rythme.");
        }
        Kind::Nef => {
            // des degrés le long du mur d'entrée, une corniche le long du mur ouest, une galerie au fond ; un levier là-haut
            for (n, u) in (-5..=-2).rev().enumerate() { let _ = n; let top = -1 - u; for v in 1..=top { b.n([u, v, -5], "stone_bricks"); } }
            for w in -4..=3 { b.n([-5, 5, w], "mossy_stone"); } for u in -5..=5 { for w in 4..=5 { b.n([u, 5, w], "stone_bricks"); } }
            b.t([2, 6, 5], "lever", (0, 1)); b.column(5, -5, 1, h); for u in [-3, 4] { b.n([u, 6, 5], "brazier_stand"); } b.n([0, h, 0], "chain"); b.n([0, h - 1, 0], "chain"); b.n([0, h - 2, 0], "lantern");
            for u in -1..=1 { for w in -1..=1 { b.n([u, 0, w], "water"); b.n([u, -1, w], "stone_bricks"); } }
            let g = gate(&b); wire(&mut m, Wire { name: "nef".into(), inputs: vec![at(&b, [2, 6, 5])], latch: true, doors: g, ..Default::default() });
            acts.push(Act::Reach(at(&b, [2, 6, 4]))); acts.push(Act::Lever(at(&b, [2, 6, 5])));
            hint(&mut m, "Une salle haute comme une nef. Là-haut, sur la galerie, un levier.");
        }
        Kind::Epreuve => {
            // un bloc sur la dalle désarme les pics et lève la grille des leviers ; deux leviers dans l'ordre des bannières
            b.n([1, 1, -3], "push_block"); b.n([3, 0, -3], "pressure_plate"); let field: Vec<[i32; 3]> = (-5..=5).flat_map(|u| [[u, 1, -1], [u, 1, 0]]).collect(); for l in &field { b.n(*l, "spikes"); }
            let row: Vec<[i32; 3]> = (-5..=5).flat_map(|u| (1..=h).map(move |v| [u, v, 2])).collect(); let row_id = b.bars_id((0, 1)); for l in &row { let (x, y, z) = b.w(*l); b.p.put(x, y, z, row_id); }
            let sw = d.r.next() < 0.5; let levers = if sw { [[3, 2, 5], [-3, 2, 5]] } else { [[-3, 2, 5], [3, 2, 5]] };
            for (n, l) in levers.iter().enumerate() { b.t(*l, "lever", (0, 1)); for v in 0..=n as i32 { b.t([l[0], 3 + v, 5], "banner_blue", (0, 1)); } } b.n([-5, 2, 6], "dart_trap");
            wire(&mut m, Wire { name: "epa".into(), inputs: vec![at(&b, [3, 0, -3])], latch: true, calm: field.iter().map(|l| at(&b, *l)).collect(), doors: row.iter().map(|l| (at(&b, *l), row_id)).collect(), ..Default::default() });
            let g = gate(&b); wire(&mut m, Wire { name: "epb".into(), inputs: levers.iter().map(|l| at(&b, *l)).collect(), seq: true, latch: true, needs: vec![format!("{name}epa")], fail_darts: true, darts: vec![at(&b, [-5, 2, 6])], doors: g, ..Default::default() });
            m.homes.push(Home { min: lo, max: hi, homes: vec![at(&b, [1, 1, -3])], keep: format!("{name}epa") });
            acts.push(Act::Push { from: at(&b, [0, 1, -3]), block: at(&b, [1, 1, -3]) }); acts.push(Act::Push { from: at(&b, [1, 1, -3]), block: at(&b, [2, 1, -3]) });
            acts.push(Act::Reach(at(&b, [0, 1, 4]))); for l in &levers { acts.push(Act::Lever(at(&b, *l))); }
            hint(&mut m, "L'épreuve : le poids, le piège, l'ordre.");
        }
        Kind::Gardien => {
            for (u, w) in [(-3, -3), (3, -3), (-3, 3), (3, 3)] { b.column(u, w, 1, h); } b.n([0, h, 0], "chandelier"); for u in [-4, 4] { b.statue(u, 1, 5, (0, 1)); b.n([u, 1, -5], "brazier_stand"); }
            let entry = { let id = b.bars_id((0, -1)); (1..=3).flat_map(|v| b.door((0, -1), v)).map(|l| (at(&b, l), id)).collect::<Vec<_>>() };
            let mut doors = gate(&b); doors.extend(entry); let a = at(&b, [0, 1, 3]);
            m.guards.push(Guard { min: lo, max: hi, species: boss.into(), at: [a[0] as f64 + 0.5, a[1] as f64, a[2] as f64 + 0.5], doors: doors.iter().map(|(p, id)| ([p[0], p[1], p[2]], *id)).collect(), state: 0 });
            // la sortie s'ouvre quand le gardien tombe : un fil sans entrée ne s'enclenche pas, on se sert des grilles du gardien
            acts.push(Act::Slay);
        }
        Kind::Tresor => {
            b.t([0, 1, 2], "altar", (0, 1)); b.n([0, 2, 2], "chest"); for (u, w) in [(-1, 2), (1, 2), (0, 1)] { b.n([u, 1, w], "candle"); }
            for u in [-3, 3] { b.t([u, 4, 5], "banner_gold", (0, 1)); b.column(u, 4, 1, h); } b.n([0, h, 2], "chandelier");
            // les tombeaux des anciens, des urnes, deux gardiens de pierre
            b.tomb(-5, 1, -2); b.tomb(3, 1, -2); for u in [-2, 2] { b.n([u, 1, 4], "funeral_urn"); } for u in [-5, 5] { b.statue(u, 1, 2, (u.signum(), 0)); }
            m.vaults.push(at(&b, [0, 2, 2])); hint(&mut m, "La salle du trésor.");
        }
    }
    // la sortie du gardien : ses grilles sont celles du gardien ; on les pose fermées seulement pendant le combat
    if room.kind.gated() && room.kind != Kind::Gardien { if let Some(s) = exit { let id = b.bars_id(s); for v in 1..=3 { for l in b.door(s, v) { let (x, y, z) = b.w(l); b.p.put(x, y, z, id); } } } }
    Out { m, acts }
}

/// Pose le donjon dans le plan (`o` : le sol de la surface au-dessus du centre) et rend ses mécanismes et la solution de
/// chaque salle. `ground` : la hauteur du sol en (x, z), pour que le puits débouche au jour.
pub(crate) fn carve(p: &mut Plan, o: [i32; 3], l: &Layout, tag: &str, ground: &dyn Fn(i32, i32) -> i32) -> (Mech, Vec<Vec<Act>>) {
    let mut m = Mech::default(); let mut sol = Vec::new();
    for (k, room) in l.rooms.iter().enumerate() { let out = build_room(p, o, *room, k, &format!("{tag}{k}:"), l.boss, l.seed, ground); m.absorb(out.m); sol.push(out.acts); }
    // les couloirs : trois blocs de large entre deux salles voisines (les grilles des sorties sont déjà posées)
    let f = o[1] - DEPTH;
    for room in &l.rooms { let Some(d) = room.exit else { continue }; let perp = (d.1.abs(), d.0.abs());
        for t in 6..=8 { for k in -2..=2 { for v in 0..=4 { let (x, z) = (o[0] + room.c.0 + d.0 * t + perp.0 * k, o[2] + room.c.1 + d.1 * t + perp.1 * k); let y = f + v;
            let inner = k.abs() <= 1 && (1..=3).contains(&v);
            if inner { if t != 6 || !room.kind.gated() || room.kind == Kind::Gardien { p.air(x, y, z); } } else if p.m.get(&(x, y, z)) != Some(&-1) { p.n(x, y, z, "stone_bricks"); } } } } }
    let r = (GRID * PITCH) / 2 + 1; m.seals.push(Seal { min: [o[0] - r, f - 3, o[2] - r], max: [o[0] + r, o[1] - 1, o[2] + r], until: String::new() });
    (m, sol)
}

/// La graine du donjon d'un lieu : de sa position.
pub fn seed_at(x: i32, z: i32) -> u32 { hash_int(x, z, 0x0d0e, 77) }

impl Session {
    /// Bâtit ici le donjon de cette graine (monde déjà chargé : essais, captures), branche ses mécanismes, et rend son plan
    /// et la solution de chaque salle.
    pub fn build_dungeon(&mut self, o: [i32; 3], seed: u32) -> (Layout, Vec<Vec<Act>>) {
        let l = layout(seed); let mut p = Plan::new(); let tag = format!("d{}:{}:", o[0], o[2]); let y0 = o[1]; let (m, sol) = carve(&mut p, o, &l, &tag, &|_, _| y0);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); if !self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { self.mech.absorb(m); } self.dirty = true; (l, sol)
    }
    /// Des ruines englouties de la carte, qu'on approche : leurs mécanismes se branchent, une fois.
    pub(crate) fn dungeon_wake(&mut self, pl: &crate::sites::Place) {
        let tag = format!("d{}:{}:", pl.x, pl.z); if self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { return; }
        let l = layout(seed_at(pl.x, pl.z)); let mut p = Plan::new(); let g = |x: i32, z: i32| self.gen.column(x as f64, z as f64).h.floor() as i32; let (m, _) = carve(&mut p, [pl.x, pl.y, pl.z], &l, &tag, &g); self.mech.absorb(m); self.dirty = true;
    }
}
