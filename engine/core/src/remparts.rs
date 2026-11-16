//! Les remparts du village pilote : la muraille qui en fait le tour et ses tours de défense, bâties d'un coup quand le
//! village les a payées et que le joueur est là (comme ses maisons).
//!
//! La muraille suit un cercle autour de tous ses bâtiments, avec de la place pour grandir dedans : deux blocs d'épaisseur,
//! pavé en pied, pierre de taille au-dessus, un chemin de ronde à quatre blocs, des merlons du côté du dehors. Elle épouse
//! le sol, s'écarte des maisons qu'elle croiserait, et s'ouvre en porte (un linteau, deux lanternes) là où passe un chemin.
//! Une tour se pose à cheval sur la muraille : cinq sur cinq, dix de haut, un étage au niveau du chemin de ronde (on la
//! traverse), une échelle, des archères, un toit-terrasse crénelé et son drapeau, la porte vers le village.
use crate::batisse::Lot;
use crate::blocks::{by_name, is_leaf, is_log, is_solid, is_water, Id, Shape, BLOCKS};
use crate::economie::{Econ, Extra};
use crate::patrons::Kind;
use crate::player::Voxels;
use crate::session::Session;
use crate::shapes::{facing_of, variant};
use crate::village::Village;

fn id(n: &str) -> Id { by_name(n).unwrap_or(0) }
/// Un bloc naturel : terre, herbe, feuillage, tronc, fleurs — ce que la muraille peut remplacer ou dégager.
fn natural(b: Id) -> bool {
    let n = BLOCKS[b as usize].name; is_leaf(b) || is_log(b) || BLOCKS[b as usize].shape == Shape::Deco && !n.starts_with("chest") || matches!(n, "dirt" | "grass" | "sand" | "gravel" | "snow" | "stone" | "granite" | "limestone" | "sandstone" | "clay" | "podzol" | "coarse_dirt" | "mud")
        || n.starts_with("grass") || n.starts_with("tall_grass") || n.ends_with("_leaves")
}

/// Le tracé de la muraille gardé dans l'économie : son centre, son rayon (au dixième).
pub fn ring(e: &Econ) -> Option<([i32; 2], f64)> { e.extra.iter().find(|x| x.kind == Kind::Muraille).map(|x| ([x.lot.o.0, x.lot.o.1], x.lot.u.0 as f64 / 10.0)) }
/// Où passerait la muraille : un cercle autour de tout ce que le village a bâti, avec vingt blocs de marge pour grandir.
pub fn plan_ring(v: &Village, areas: &[[i32; 4]]) -> ([i32; 2], f64) {
    let c = [v.c[0], v.c[2]]; let mut r = v.r as f64 * 0.7;
    for a in v.works.iter().map(|w| w.1).chain(v.houses.iter().map(|h| h.area)).chain(areas.iter().copied()) { for (x, z) in [(a[0], a[1]), (a[2], a[1]), (a[0], a[3]), (a[2], a[3])] { r = r.max(((x - c[0]) as f64).hypot((z - c[1]) as f64)); } }
    (c, ((r + 20.0).clamp(30.0, 80.0) * 10.0).round() / 10.0)
}
/// Les colonnes de la muraille : deux d'épaisseur ; vrai pour la rangée du dehors.
pub fn cells(c: [i32; 2], r: f64) -> Vec<(i32, i32, bool)> {
    let k = r.ceil() as i32 + 2; let mut o = Vec::new();
    for x in c[0] - k..=c[0] + k { for z in c[1] - k..=c[1] + k { let d = ((x - c[0]) as f64).hypot((z - c[1]) as f64); if d >= r - 1.0 && d < r + 1.0 { o.push((x, z, d >= r)); } } } o
}

impl Session {
    /// Le vrai sol d'une colonne (le plus haut bloc plein qui n'est ni feuillage, ni tronc, ni décor), si elle est chargée.
    fn rampart_ground(&self, x: i32, z: i32) -> Option<i32> {
        let g = self.gen.column(x as f64, z as f64).h.floor() as i32; self.world.get(x, g, z)?;
        let ground = |b: Id| is_solid(b) && !is_leaf(b) && !is_log(b) && BLOCKS[b as usize].shape != Shape::Deco;
        let mut y = (g - 6..=g + 8).rev().find(|&y| self.world.get(x, y, z).is_some_and(ground)).unwrap_or(g);
        // une rivière : la muraille la franchit, posée sur un remblai jusqu'à la surface de l'eau
        while self.world.get(x, y + 1, z).is_some_and(is_water) { y += 1; } Some(y)
    }
    /// Les blocs de la muraille, et son tracé à garder ; `None` tant que tout son tour n'est pas chargé.
    pub(crate) fn rampart_wall(&self, v: &Village, e: &Econ) -> Option<(Vec<(i32, i32, i32, Id)>, Extra)> {
        let areas: Vec<[i32; 4]> = e.extra.iter().filter(|x| !matches!(x.kind, Kind::Muraille | Kind::TourDefense)).map(|x| crate::economie::area_of(x)).collect();
        let (c, r) = plan_ring(v, &areas); let all = cells(c, r);
        let mut ground = std::collections::HashMap::new(); for &(x, z, _) in &all { ground.insert((x, z), self.rampart_ground(x, z)?); }
        // les portes : là où un chemin (ou une rue pavée) croise le tracé, sur cinq de large
        let road = |x: i32, z: i32| ground.get(&(x, z)).and_then(|g| self.world.get(x, *g, z)).is_some_and(|b| matches!(BLOCKS[b as usize].name, "path" | "gravel" | "cobblestone" | "flagstone"));
        let roads: Vec<(i32, i32)> = all.iter().filter(|(x, z, _)| road(*x, *z)).map(|(x, z, _)| (*x, *z)).collect();
        let gate = |x: i32, z: i32| roads.iter().any(|(a, b)| (a - x).abs() <= 2 && (b - z).abs() <= 2);
        let (cob, brick, lin) = (id("cobblestone"), id("stone_bricks"), id("oak_log_c")); let mut o = Vec::new();
        for &(x, z, out) in &all {
            let g = ground[&(x, z)];
            // une maison (ou un mur) sur le tracé : on la laisse, la muraille s'y appuie
            if (g + 1..=g + 5).any(|y| self.world.get(x, y, z).is_some_and(|b| b != 0 && !natural(b))) { continue; }
            for y in g - 1..=g { o.push((x, y, z, cob)); }
            let mut y = g - 2; while self.world.get(x, y, z).is_some_and(is_water) { o.push((x, y, z, cob)); y -= 1; }
            if gate(x, z) { for y in g + 1..=g + 3 { o.push((x, y, z, 0)); } o.push((x, g + 4, z, if out { brick } else { lin })); }
            else { o.push((x, g + 1, z, cob)); for y in g + 2..=g + 4 { o.push((x, y, z, brick)); } }
            // les merlons du dehors, un sur deux ; le ciel dégagé au-dessus (branches, feuillage)
            let merlon = out && (x + z).rem_euclid(2) == 0; if merlon { o.push((x, g + 5, z, brick)); }
            for y in g + if merlon { 6 } else { 5 }..=g + 14 { if self.world.get(x, y, z).is_some_and(|b| b != 0 && natural(b)) { o.push((x, y, z, 0)); } }
        }
        // deux lanternes à chaque porte, côté village
        let lamp = id("lantern"); for &(a, b) in roads.iter().step_by(3) { let (dx, dz) = ((c[0] - a) as f64, (c[1] - b) as f64); let l = dx.hypot(dz).max(1.0); let (ix, iz) = ((a as f64 + dx / l * 2.0).round() as i32, (b as f64 + dz / l * 2.0).round() as i32); if let Some(g) = ground.get(&(a, b)) { if self.world.get(ix, g + 4, iz).is_some_and(|x| x == 0) { o.push((ix, g + 4, iz, lamp)); } } }
        Some((o, Extra { lot: Lot { o: (c[0], c[1]), u: ((r * 10.0).round() as i32, 0), w: (0, 1) }, y: 0, kind: Kind::Muraille }))
    }
    /// Une tour sur la muraille : la `n`-ième, à intervalles réguliers, écartée des portes.
    pub(crate) fn rampart_tower(&self, e: &Econ, n: usize) -> Option<(Vec<(i32, i32, i32, Id)>, Extra)> {
        let (c, r) = ring(e)?; let gates: Vec<(i32, i32)> = cells(c, r).into_iter().filter(|(x, z, _)| self.rampart_ground(*x, *z).and_then(|g| self.world.get(*x, g + 2, *z)).is_some_and(|b| b == 0)).map(|(x, z, _)| (x, z)).collect();
        let mut a = n as f64 * std::f64::consts::TAU / crate::economie::TOWERS as f64 + 0.55; let at = |a: f64| ((c[0] as f64 + a.cos() * r).round() as i32, (c[1] as f64 + a.sin() * r).round() as i32);
        for _ in 0..60 { let (x, z) = at(a); if !gates.iter().any(|(gx, gz)| (gx - x).abs() <= 5 && (gz - z).abs() <= 5) { break; } a += 0.04; }
        let (tx, tz) = at(a); let g = self.rampart_ground(tx, tz)?; let mut o = Vec::new();
        let (cob, brick, plank) = (id("cobblestone"), id("stone_bricks"), id("planks"));
        // la porte regarde le village ; l'axe du chemin de ronde, le long de la muraille
        let (nx, nz) = ((c[0] - tx) as f64, (c[1] - tz) as f64); let door: (i32, i32) = if nx.abs() > nz.abs() { (nx.signum() as i32, 0) } else { (0, nz.signum() as i32) }; let along = (door.1.abs(), door.0.abs());
        for dx in -2..=2i32 { for dz in -2..=2i32 { let (x, z) = (tx + dx, tz + dz); let edge = dx.abs() == 2 || dz.abs() == 2; let corner = dx.abs() == 2 && dz.abs() == 2;
            for y in g - 2..=g { o.push((x, y, z, cob)); }
            for y in g + 1..=g + 9 { o.push((x, y, z, if !edge { 0 } else if corner || y == g + 1 { cob } else { brick })); }
            if !edge { o.push((x, g + 4, z, plank)); o.push((x, g + 9, z, plank)); }
            if edge && !corner && (dx + dz).rem_euclid(2) == 0 { o.push((x, g + 10, z, brick)); } else if corner { o.push((x, g + 10, z, brick)); o.push((x, g + 11, z, brick)); }
            for y in g + 10..=g + 16 { if !(edge && (corner || (dx + dz).rem_euclid(2) == 0) && y <= g + 11) && self.world.get(x, y, z).is_some_and(|b| b != 0 && natural(b)) { o.push((x, y, z, 0)); } } } }
        // la porte (deux blocs), les archères au milieu de chaque face, le passage du chemin de ronde de part et d'autre
        let (ddx, ddz) = (door.0 * 2, door.1 * 2); let f = facing_of(-door.0 as f64, -door.1 as f64); let d = id("door");
        o.push((tx + ddx, g + 1, tz + ddz, variant(d, f, false, false, false))); o.push((tx + ddx, g + 2, tz + ddz, variant(d, f, false, false, true)));
        for (fx, fz) in [(2, 0), (-2, 0), (0, 2), (0, -2)] { if (fx, fz) != (ddx, ddz) { o.push((tx + fx, g + 3, tz + fz, 0)); } o.push((tx + fx, g + 7, tz + fz, 0)); }
        for s in [-2, 2] { for y in g + 5..=g + 6 { o.push((tx + along.0 * s, y, tz + along.1 * s, 0)); } }
        // l'échelle contre le mur du fond, les trous des planchers ; le drapeau, deux lanternes, une applique à la porte
        let (lx, lz) = (tx - door.0, tz - door.1); let lf = facing_of(door.0 as f64, door.1 as f64); let ladder = variant(id("ladder"), lf, false, false, false);
        for y in g + 1..=g + 9 { o.push((lx, y, lz, ladder)); }
        o.push((tx, g + 10, tz, id("flag"))); o.push((tx + along.0, g + 5, tz + along.1, id("lantern"))); o.push((tx - along.0, g + 10, tz - along.1, id("lantern")));
        o.push((tx + door.0 * 3, g + 3, tz + door.1 * 3, variant(id("wall_sconce"), facing_of(-door.0 as f64, -door.1 as f64), false, false, false)));
        Some((o, Extra { lot: Lot { o: (tx, tz), u: (1, 0), w: (0, 1) }, y: g, kind: Kind::TourDefense }))
    }
}
