//! Le Temple du Soleil : une pyramide à degrés, une grande salle sous un oculus, un rayon de soleil à conduire de miroir en
//! miroir jusqu'à trois sceaux, et le sanctuaire de son gardien.
//!
//! Dehors, quatre degrés de grès s'élèvent à seize blocs ; au nord, un portail à colonnes entre deux statues ouvre sur un
//! couloir ; au sud, un grand escalier monte jusqu'à la plate-forme du sommet, où s'ouvre l'oculus. Dedans, la Salle du Soleil
//! (21 × 21, haute de 8) : en plein jour, la lumière de l'oculus tombe sur le disque d'or du centre, qui la renvoie vers l'est.
//! Cinq miroirs d'or tournent sur leur pied (clic droit) ; le rayon file de l'un à l'autre. Trois sceaux de cristal attendent
//! dans les murs — à l'est, au nord, au sud — et chacun demande un autre chemin du rayon. Celui du sud est derrière une grille,
//! qu'ouvre une dalle (un bloc à pousser, dans la même salle). Les trois sceaux embrasés, la porte du sanctuaire se lève ; le
//! Gardien du Soleil s'y dresse devant l'autel. La nuit, le soleil n'entre pas : on attend le jour.
use crate::blocks::{by_name, Id};
use crate::mecanismes::{Beam, Guard, Hint, Home, Mech, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// Les degrés de la pyramide : (demi-côté, du niveau, au niveau), au-dessus du sol (v = 0).
pub const TIERS: [(i32, i32, i32); 4] = [(17, 1, 4), (14, 5, 8), (11, 9, 12), (8, 13, 15)];
/// La Salle du Soleil (intérieur), le sanctuaire.
pub const HALL: (i32, i32) = (10, 8); pub const SANCTUM: (i32, i32, i32, i32, i32) = (-6, 6, 13, 15, 4);
/// Le disque solaire (la source, tournée vers l'est), les miroirs (et leur sens au départ), les sceaux.
pub const DISC: [i32; 3] = [0, 1, 0];
pub const MIRRORS: [([i32; 3], u8); 5] = [([4, 1, 0], 0), ([4, 1, -5], 0), ([4, 1, 5], 0), ([-4, 1, -5], 1), ([-4, 1, 5], 0)];
pub const SEALS: [([i32; 3], (i32, i32)); 3] = [([11, 1, -5], (1, 0)), ([-4, 1, -11], (0, -1)), ([-4, 1, 12], (0, 1))];
/// La grille du sceau du sud, sa dalle, son bloc à pousser.
pub const NICHE: [[i32; 3]; 3] = [[-4, 1, 11], [-4, 2, 11], [-4, 3, 11]];
pub const PLATE: [i32; 3] = [-8, 0, 5]; pub const BLOCK: [i32; 3] = [-8, 1, 2];
/// La porte du sanctuaire, le Gardien du Soleil, le coffre de l'autel.
pub const BOSS: &str = "gardien_soleil"; pub const BOSS_AT: [f64; 3] = [0.5, 1.0, 14.5]; pub const CHEST: [i32; 3] = [0, 2, 15];
fn door() -> Vec<[i32; 3]> { (0..=2).flat_map(|u| (1..=3).map(move |v| [u, v, 11])).collect() }

struct T<'a> { p: &'a mut Plan, x: i32, y: i32, z: i32 }
impl T<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.y + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn column(&mut self, u: i32, w: i32, v0: i32, v1: i32) { for v in v0..=v1 { self.n(u, v, w, if v == v0 { "column_base" } else if v == v1 { "column_capital" } else { "column_shaft" }); } }
    fn statue(&mut self, u: i32, v: i32, w: i32, dx: i32, dz: i32) { self.t(u, v, w, "statue_base", dx, dz); self.t(u, v + 1, w, "statue_top", dx, dz); }
    fn sand(&self, u: i32, v: i32, w: i32) -> &'static str { let h = hash_float(self.x + u + 131 * v, self.z + w, 2027, 6); if h < 0.12 { "limestone_masonry" } else if h < 0.18 { "mossy_stone" } else { "sandstone" } }
}

/// Pose le temple dans le plan : `o` est le sol au centre de la pyramide.
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut t = T { p, x: o[0], y: o[1], z: o[2] };
    // la pyramide : quatre degrés pleins, une corniche de calcaire au bord de chaque terrasse
    for &(r, v0, v1) in &TIERS { for u in -r..=r { for w in -r..=r { let (u, w): (i32, i32) = (u, w); for v in v0..=v1 { let edge = (u.abs() == r || w.abs() == r) && v == v1; let s = if edge { "limestone_masonry" } else { t.sand(u, v, w) }; t.n(u, v, w, s); } } } }
    // le sol de la salle et du parvis, la salle, le sanctuaire, le couloir d'entrée, l'oculus
    for u in -19..=19i32 { for w in -24..=24i32 { if (u.abs() <= 17 && w.abs() <= 17) || (u.abs() <= 8 && w < -17) { t.n(u, 0, w, if (u + w).rem_euclid(2) == 0 { "limestone_masonry" } else { "sandstone" }); } } }
    let (r, h) = HALL; for u in -r..=r { for w in -r..=r { for v in 1..=h { t.air(u, v, w); } } }
    for u in SANCTUM.0..=SANCTUM.1 { for w in SANCTUM.2..=SANCTUM.3 { for v in 1..=SANCTUM.4 { t.air(u, v, w); } t.n(u, SANCTUM.4 + 1, w, "limestone_masonry"); } }
    for u in -1..=1 { for w in -18..=-11 { for v in 1..=3 { t.air(u, v, w); } } }
    for u in -1..=1 { for w in -1..=1 { for v in HALL.1 + 1..=16 { t.air(u, v, w); } } }
    // le grand escalier du sud, de la plaine au sommet
    for k in 1..=15 { let w = 23 - k; for u in -2..=2 { t.t(u, k, w, "sandstone_stairs", 0, -1); for v in k + 1..=k + 3 { t.air(u, v, w); } if w >= 18 { for v in 1..k { t.n(u, v, w, "sandstone"); } } } for u in [-3, 3] { if w >= 18 { for v in 1..=k { t.n(u, v, w, "limestone_masonry"); } } } }
    // le sommet : un garde-fou autour de l'oculus, des brasiers aux angles, huit colonnes tronquées
    for u in -2..=2i32 { for w in -2..=2i32 { if u.abs() == 2 || w.abs() == 2 { t.n(u, 16, w, "limestone_masonry_slab"); } } }
    for (u, w) in [(-7, -7), (7, -7), (-7, 7), (7, 7)] { t.n(u, 16, w, "brazier_stand"); }
    for (u, w) in [(-5, -5), (0, -6), (5, -5), (-6, 0), (6, 0), (-5, 5), (5, 5)] { t.n(u, 16, w, "column_base"); t.n(u, 17, w, "column_shaft"); }
    // le portail du nord : deux colonnes, un linteau, un fronton de grès, deux gardiens de pierre, deux brasiers
    for u in [-3, 3] { t.column(u, -18, 1, 6); } for u in -4..=4 { t.t(u, 7, -18, "lintel", 0, 1); } for u in -3..=3 { t.n(u, 8, -18, "sandstone"); } t.t(0, 8, -18, "wall_relief", 0, 1); for u in -1..=1 { t.n(u, 9, -18, "sandstone"); }
    for u in [-5, 5] { t.statue(u, 1, -18, 0, 1); t.n(u + u.signum() * 2, 1, -18, "brazier_stand"); }
    for v in 1..=4 { for u in [-2, 2] { t.n(u, v, -17, "limestone_masonry"); } } for u in -2..=2 { t.n(u, 4, -17, "limestone_masonry"); }
    // dans la salle : les murs revêtus de calcaire, des bas-reliefs, quatre colonnes, des gardiens de pierre, des brasiers
    for u in -r - 1..=r + 1 { for w in -r - 1..=r + 1 { if u.abs() == r + 1 || w.abs() == r + 1 { for v in 1..=h { let (x, y, z) = t.at(u, v, w); if t.p.m.get(&(x, y, z)) != Some(&-1) { t.n(u, v, w, if v == 1 || v == h { "limestone_masonry" } else { "sandstone" }); } } } } }
    for k in [-8, -2, 2, 8] { t.t(k, 4, -r - 1, "wall_relief", 0, -1); t.t(k, 4, r + 1, "wall_relief", 0, 1); t.t(-r - 1, 4, k, "wall_relief", -1, 0); t.t(r + 1, 4, k, "wall_relief", 1, 0); }
    for (u, w) in [(-8, -8), (8, -8), (-8, 8), (8, 8)] { t.column(u, w, 1, h); }
    for (u, w) in [(-10, -3), (-10, 3), (10, 3)] { t.statue(u, 1, w, u.signum(), 0); } for (u, w) in [(-10, -10), (10, -10), (-10, 10), (10, 10)] { t.n(u, 1, w, "brazier_stand"); }
    for (u, w, dx, dz) in [(-10, 0, -1, 0), (10, 0, 1, 0), (0, -10, 0, -1), (0, 10, 0, 1)] { t.t(u, 5, w, "wall_sconce", dx, dz); }
    // le disque d'or sous l'oculus, tourné vers l'est ; les miroirs ; les sceaux dans les murs, face à la salle
    t.t(DISC[0], DISC[1], DISC[2], "sun_disc", -1, 0); for (q, f) in MIRRORS { let n = if f == 0 { "sun_mirror" } else { ["sun_mirror", "sun_mirror@1", "sun_mirror@2", "sun_mirror@3"][f as usize] }; t.n(q[0], q[1], q[2], n); }
    for (q, back) in SEALS { t.t(q[0], q[1], q[2], "sun_receptor", back.0, back.1); }
    for q in NICHE { t.n(q[0], q[1], q[2], "iron_bars"); } t.n(PLATE[0], PLATE[1], PLATE[2], "pressure_plate"); t.n(BLOCK[0], BLOCK[1], BLOCK[2], "push_block");
    // le sanctuaire : sa grille, l'autel et le coffre, deux gardiens, deux brasiers
    for q in door() { t.n(q[0], q[1], q[2], "iron_bars"); t.air(q[0], q[1], 12); }
    t.t(CHEST[0], 1, CHEST[2], "altar", 0, 1); t.n(CHEST[0], CHEST[1], CHEST[2], "chest"); for u in [-5, 5] { t.statue(u, 1, 15, 0, 1); t.n(u.signum() * 3, 1, 15, "brazier_stand"); }
}

/// Les mécanismes du temple bâti en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let at = |q: [i32; 3]| [o[0] + q[0], o[1] + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}"); let iron = by_name("iron_bars").unwrap_or(0);
    let mut m = Mech::default();
    m.beams.push(Beam { src: at(DISC), dir: (1, 0), day: true, ..Default::default() });
    for (k, (q, _)) in SEALS.iter().enumerate() { m.wires.push(Wire { name: name(&format!("sceau{k}")), inputs: vec![at(*q)], latch: true, ..Default::default() }); }
    m.wires.push(Wire { name: name("niche"), inputs: vec![at(PLATE)], latch: true, doors: NICHE.iter().map(|q| (at(*q), iron)).collect(), ..Default::default() });
    m.homes.push(Home { min: at([-HALL.0, 1, -HALL.0]), max: at([HALL.0, HALL.1, HALL.0]), homes: vec![at(BLOCK)], keep: name("niche") });
    let doors: Vec<([i32; 3], Id)> = door().into_iter().map(|q| (at(q), iron)).collect();
    m.wires.push(Wire { name: name("sanctuaire"), needs: (0..3).map(|k| name(&format!("sceau{k}"))).collect(), latch: true, doors: doors.clone(), ..Default::default() });
    m.guards.push(Guard { min: at([SANCTUM.0, 1, SANCTUM.2]), max: at([SANCTUM.1, SANCTUM.4, SANCTUM.3]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], o[1] as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors, state: 0 });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-18, 0, -18]), max: at([18, 17, 23]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-HALL.0, 1, -HALL.0], [HALL.0, HALL.1, HALL.0], "Le Temple du Soleil. En plein jour, la lumière de l'oculus tombe sur le disque d'or. Trois sceaux de cristal attendent le rayon : tournez les miroirs (clic droit) pour l'y conduire. La nuit, le soleil n'entre pas.");
    hint(&mut m, [-10, 1, 1], [-6, 4, 10], "Derrière une grille, le sceau du sud. Une dalle dans le sol, un bloc de pierre non loin…");
    m
}

impl Session {
    /// Bâtit ici le temple (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol au centre.
    pub fn build_sun_temple(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("t{}:{}:", o[0], o[2]); if !self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    pub(crate) fn sun_temple_wake(&mut self, pl: &Place) { let tag = format!("t{}:{}:", pl.x, pl.z); if self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}
