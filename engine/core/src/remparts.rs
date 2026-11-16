//! Les remparts du village pilote, d'après la planche « Système de fortifications » : la muraille qui en fait le tour, ses
//! portes fortifiées là où passent les chemins, et ses tours de guet ; bâties d'un coup quand le village les a payées et que
//! le joueur est là (comme ses maisons).
//!
//! La muraille (module de la planche : 5 de long, 2 d'épaisseur, 6 de haut, le chemin de ronde à 5) suit un cercle autour de
//! tous les bâtiments, avec de la place pour grandir dedans : pavé en pied, pierre de taille, un poteau de chêne tous les cinq
//! blocs, une poutre couchée en haut sur les deux faces (le chemin de ronde est de bois), des merlons du côté du dehors, une
//! barrière du côté du village, une applique sur un poteau sur deux. Elle épouse le sol, franchit les rivières sur un remblai
//! et s'écarte des maisons qu'elle croiserait.
//! La porte fortifiée (11 × 5, 9 de haut) : un passage voûté de trois de large, la herse relevée, deux tourelles aux toits de
//! tuiles et leurs drapeaux, un étage à pans de bois entre elles, des bannières rouges, des appliques de part et d'autre.
//! La tour de guet (7 × 7 au sol, 12 de haut et son toit) : un pied élargi, un fût de pierre de taille, un étage au niveau du
//! chemin de ronde qu'on traverse, une galerie de bois en encorbellement sous un toit de tuiles en pyramide, une bannière, un
//! drapeau, une échelle du bas jusqu'au guet, la porte vers le village.
use crate::batisse::{Lot, Work};
use crate::blocks::{by_name, is_leaf, is_log, is_solid, is_water, Id, Shape, BLOCKS};
use crate::economie::{Econ, Extra};
use crate::patrons::{Kind, Rec, P};
use crate::player::Voxels;
use crate::session::Session;
use crate::village::Village;

fn id(n: &str) -> Id { by_name(n).unwrap_or(0) }
/// Un bloc naturel : terre, herbe, feuillage, tronc, fleurs — ce que la muraille peut remplacer ou dégager.
fn natural(b: Id) -> bool {
    let n = BLOCKS[b as usize].name; is_leaf(b) || is_log(b) || is_water(b) || BLOCKS[b as usize].shape == Shape::Deco && !n.starts_with("chest") && !n.starts_with("fence") && !n.starts_with("banner") && !n.contains("sconce") && !n.starts_with("lantern")
        || matches!(n, "dirt" | "grass" | "sand" | "gravel" | "snow" | "stone" | "granite" | "limestone" | "sandstone" | "clay" | "podzol" | "coarse_dirt" | "mud" | "forest_soil" | "dry_earth" | "moor_grass" | "grass_deep" | "grass_dry" | "mossy_stone" | "lichen_stone" | "red_sandstone" | "ice"
            | "coal_ore" | "iron_ore" | "copper_ore" | "tin_ore" | "pebbles" | "steppe_grass" | "frost_grass")
        || n.starts_with("grass") || n.starts_with("tall_grass") || n.ends_with("_leaves")
}
/// Un chemin : là où la muraille s'ouvre en porte.
fn road(b: Id) -> bool { matches!(BLOCKS[b as usize].name, "path" | "flagstone") }

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
/// Le repère d'un ouvrage posé sur la muraille au point (x, z) : l'axe local x court le long de la muraille, l'axe local z
/// va du dehors vers le village ; `o` est l'origine du coin (0, 0) d'une emprise de `w` × `d` centrée sur ce point.
fn frame_at(c: [i32; 2], x: i32, z: i32, w: i32, d: i32) -> Lot {
    let (nx, nz) = ((c[0] - x) as f64, (c[1] - z) as f64); let inn: (i32, i32) = if nx.abs() > nz.abs() { (nx.signum() as i32, 0) } else { (0, nz.signum() as i32) };
    let u = (inn.1, -inn.0); let (hw, hd) = (w / 2, d / 2);
    Lot { o: (x - hw * u.0 - hd * inn.0, z - hw * u.1 - hd * inn.1), u, w: inn }
}
/// Les coordonnées locales (le long, vers dedans) d'une colonne dans un repère.
fn local(l: &Lot, x: i32, z: i32) -> (i32, i32) { let (rx, rz) = (x - l.o.0, z - l.o.1); (rx * l.u.0 + rz * l.u.1, rx * l.w.0 + rz * l.w.1) }

impl Session {
    /// Le vrai sol d'une colonne (le plus haut bloc plein et naturel, ou un chemin), au-dessus de l'eau s'il y a une rivière ;
    /// `None` si elle n'est pas chargée. Les blocs des remparts n'y comptent pas : une tour se pose sur le sol, pas sur la muraille.
    fn rampart_ground(&self, x: i32, z: i32) -> Option<i32> {
        let g = self.gen.column(x as f64, z as f64).h.floor() as i32; self.world.get(x, g, z)?;
        let ground = |b: Id| is_solid(b) && !is_leaf(b) && !is_log(b) && BLOCKS[b as usize].shape != Shape::Deco && (natural(b) || road(b));
        let mut y = (g - 6..=g + 8).rev().find(|&y| self.world.get(x, y, z).is_some_and(ground)).unwrap_or(g);
        // une rivière : la muraille la franchit, posée sur un remblai jusqu'à la surface de l'eau
        while self.world.get(x, y + 1, z).is_some_and(is_water) { y += 1; } Some(y)
    }
    /// Les portes de la muraille : une par chemin qui croise le tracé, au milieu de l'endroit où il le croise (x, sol, z).
    pub fn rampart_gates(&self, c: [i32; 2], r: f64) -> Vec<(i32, i32, i32)> {
        let mut roads: Vec<(i32, i32)> = Vec::new();
        for (x, z, _) in cells(c, r) { if let Some(g) = self.rampart_ground(x, z) { if [g, g + 1].iter().any(|&y| self.world.get(x, y, z).is_some_and(road)) { roads.push((x, z)); } } }
        // les cases de chemin voisines font une seule porte ; elle se pose au milieu, sur le cercle
        let mut groups: Vec<Vec<(i32, i32)>> = Vec::new();
        for p in roads { let near: Vec<usize> = (0..groups.len()).filter(|&i| groups[i].iter().any(|q| (q.0 - p.0).abs() <= 3 && (q.1 - p.1).abs() <= 3)).collect();
            let mut g = vec![p]; for &i in near.iter().rev() { g.extend(groups.remove(i)); } groups.push(g); }
        let mut o: Vec<(i32, i32, i32)> = Vec::new();
        for g in groups {
            let (sx, sz) = g.iter().fold((0.0, 0.0), |a, p| { let t = ((p.1 - c[1]) as f64).atan2((p.0 - c[0]) as f64); (a.0 + t.cos(), a.1 + t.sin()) }); let t = sz.atan2(sx);
            let (x, z) = ((c[0] as f64 + t.cos() * r).round() as i32, (c[1] as f64 + t.sin() * r).round() as i32);
            if o.iter().any(|q| (q.0 - x).abs() <= 12 && (q.2 - z).abs() <= 12) { continue; }
            if let Some(y) = self.rampart_ground(x, z) { o.push((x, y, z)); } }
        o
    }
    /// Les blocs de la muraille et de ses portes, et son tracé à garder ; `None` tant que tout son tour n'est pas chargé.
    pub(crate) fn rampart_wall(&self, v: &Village, e: &Econ) -> Option<(Vec<(i32, i32, i32, Id)>, Extra)> {
        let areas: Vec<[i32; 4]> = e.extra.iter().filter(|x| !matches!(x.kind, Kind::Muraille | Kind::TourDefense)).map(crate::economie::area_of).collect();
        let (c, r) = plan_ring(v, &areas); let all = cells(c, r);
        let mut ground = std::collections::HashMap::new(); for &(x, z, _) in &all { ground.insert((x, z), self.rampart_ground(x, z)?); }
        let gates: Vec<(Lot, i32)> = self.rampart_gates(c, r).into_iter().map(|(x, y, z)| (frame_at(c, x, z, 11, 5), y)).collect();
        let in_gate = |x: i32, z: i32| gates.iter().any(|(l, _)| { let (a, b) = local(l, x, z); (0..=10).contains(&a) && (-2..=6).contains(&b) });
        let (cob, brick, fence, sconce) = (id("cobblestone"), id("stone_bricks"), id("fence"), id("wall_sconce")); let mut o = Vec::new();
        for &(x, z, out) in &all {
            let g = ground[&(x, z)]; if in_gate(x, z) { continue; }
            // une maison (ou un mur) sur le tracé : on la laisse, la muraille s'y appuie
            if (g + 1..=g + 6).any(|y| self.world.get(x, y, z).is_some_and(|b| b != 0 && !natural(b))) { continue; }
            // la place le long du tour (un pas par bloc) : les poteaux tous les cinq, les merlons un sur deux
            let t = ((z - c[1]) as f64).atan2((x - c[0]) as f64); let s = (t * r).round() as i32; let along_x = t.sin().abs() > t.cos().abs();
            let beam = id(if along_x { "oak_log_x_c" } else { "oak_log_z_c" });
            for y in g - 1..=g { o.push((x, y, z, cob)); }
            let mut y = g - 2; while self.world.get(x, y, z).is_some_and(is_water) { o.push((x, y, z, cob)); y -= 1; }
            o.push((x, g + 1, z, cob)); let post = s.rem_euclid(5) == 0;
            for y in g + 2..=g + 4 { o.push((x, y, z, if post { id("oak_log_c") } else { brick })); }
            o.push((x, g + 5, z, if post { id("oak_log_c") } else { beam }));
            let top = if out { if s.rem_euclid(2) == 0 { brick } else { 0 } } else { fence }; if top != 0 { o.push((x, g + 6, z, top)); }
            for y in g + if top != 0 { 7 } else { 6 }..=g + 14 { if self.world.get(x, y, z).is_some_and(|b| b != 0 && natural(b)) { o.push((x, y, z, 0)); } }
            // une applique sur un poteau sur deux, sur chaque face, dans l'air devant le mur
            if post && s.rem_euclid(10) == if out { 0 } else { 5 } { let d = if out { 1.0 } else { -1.0 }; let (fx, fz) = ((x as f64 + t.cos() * d * 1.2).round() as i32, (z as f64 + t.sin() * d * 1.2).round() as i32);
                if !all.iter().any(|q| (q.0, q.1) == (fx, fz)) && self.world.get(fx, g + 3, fz).is_some_and(|b| b == 0) { let f = crate::shapes::facing_of(-t.cos() * d, -t.sin() * d); o.push((fx, g + 3, fz, crate::shapes::variant(sconce, f, false, false, false))); } }
        }
        for (l, y) in &gates { let mut k = Work::default(); gatehouse(&mut P { k: &mut k, lot: *l, y0: *y, mirror: false, w: 11, d: 5, h: 0.0, rec: Rec::default() }); o.extend(k.blocks); }
        Some((o, Extra { lot: Lot { o: (c[0], c[1]), u: ((r * 10.0).round() as i32, 0), w: (0, 1) }, y: 0, kind: Kind::Muraille }))
    }
    /// Une tour de guet sur la muraille : la `n`-ième, à intervalles réguliers, écartée des portes.
    pub(crate) fn rampart_tower(&self, e: &Econ, n: usize) -> Option<(Vec<(i32, i32, i32, Id)>, Extra)> {
        let (c, r) = ring(e)?; let gates = self.rampart_gates(c, r);
        let mut a = n as f64 * std::f64::consts::TAU / crate::economie::TOWERS as f64 + 0.55; let at = |a: f64| ((c[0] as f64 + a.cos() * r).round() as i32, (c[1] as f64 + a.sin() * r).round() as i32);
        for _ in 0..80 { let (x, z) = at(a); if !gates.iter().any(|(gx, _, gz)| (gx - x).abs() <= 11 && (gz - z).abs() <= 11) { break; } a += 0.04; }
        // le pied de la tour : celui de la muraille à cet endroit (sous son rang de pavé), sinon le sol
        let (tx, tz) = at(a); let g0 = self.rampart_ground(tx, tz)?; let named = |y: i32| self.world.get(tx, y, tz).map_or("", |b| BLOCKS[b as usize].name);
        let g = (g0 - 4..=g0 + 4).rev().find(|&y| named(y) == "cobblestone" && matches!(named(y + 1), "stone_bricks" | "oak_log_c")).map_or(g0, |y| y - 1); let l = frame_at(c, tx, tz, 7, 7);
        let mut k = Work::default(); watchtower(&mut P { k: &mut k, lot: l, y0: g, mirror: false, w: 7, d: 7, h: 0.0, rec: Rec::default() });
        // le feuillage et les branches qui gêneraient le toit
        let mut o = k.blocks; for dx in -5..=5 { for dz in -5..=5 { for y in g + 9..=g + 22 { let (x, z) = (tx + dx, tz + dz); if self.world.get(x, y, z).is_some_and(|b| b != 0 && natural(b)) && !o.iter().any(|b| (b.0, b.1, b.2) == (x, y, z)) { o.insert(0, (x, y, z, 0)); } } } }
        Some((o, Extra { lot: Lot { o: (tx, tz), u: (1, 0), w: (0, 1) }, y: g, kind: Kind::TourDefense }))
    }
}

/// La tour de guet, dans son repère : 7 × 7, le dehors en z = 0, le village en z = 6, le centre en (3, 3).
fn watchtower(p: &mut P) {
    // dégagée, fondée ; le pied élargi en pavé (deux rangs et un glacis), le fût de pierre de taille
    for x in 0..7 { for z in 0..7 { for y in 1..=11 { p.put(x, y, z, 0); } for y in -3..=0 { p.n(x, y, z, "cobblestone"); } } }
    for x in 0..7 { for z in 0..7 { let ring = |m: i32| (x == m || x == 6 - m || z == m || z == 6 - m) && (m..=6 - m).contains(&x) && (m..=6 - m).contains(&z);
        if ring(0) { for y in 1..=2 { p.n(x, y, z, "cobblestone"); } p.stair(x, 3, z, "stone_bricks_stairs", if x == 0 { 1 } else if x == 6 { -1 } else { 0 }, if z == 0 { 1 } else if z == 6 { -1 } else { 0 }, false); }
        if ring(1) { let corner = (x == 1 || x == 5) && (z == 1 || z == 5); for y in 1..=8 { p.n(x, y, z, if corner || y <= 2 { "cobblestone" } else { "stone_bricks" }); } }
        if (2..=4).contains(&x) && (2..=4).contains(&z) { p.n(x, 0, z, "planks"); p.n(x, 5, z, "planks"); } } }
    // la porte vers le village (on passe le glacis), une applique ; les archères ; la bannière rouge au-dehors
    for y in 1..=3 { p.put(3, y, 6, 0); } p.n(3, 0, 6, "cobblestone"); p.door(3, 1, 5, 0, 1); p.sconce(4, 4, 6, 0, -1);
    for (x, z) in [(3, 1), (1, 3), (5, 3)] { p.put(x, 3, z, 0); } p.put(3, 7, 1, 0); p.put(3, 6, 1, 0); p.win(3, 7, 5, true);
    p.turn(2, 6, 0, "banner_red", 0, 1); p.turn(4, 6, 0, "banner_red", 0, 1);
    // l'étage du chemin de ronde : on le traverse de part en part ; la muraille y est raccordée
    for x in [0, 1, 5, 6] { for z in 2..=4 { for y in 6..=7 { p.put(x, y, z, 0); } if x == 0 || x == 6 { for y in 3..=5 { p.n(x, y, z, "stone_bricks"); } } } }
    // la galerie de guet : un plancher en encorbellement sur des consoles, la barrière, les poteaux d'angle, les lanternes
    for x in 0..7 { for z in 0..7 { p.n(x, 9, z, "planks"); let edge = x == 0 || x == 6 || z == 0 || z == 6; let corner = (x == 0 || x == 6) && (z == 0 || z == 6);
        if corner { for y in 10..=11 { p.log(x, y, z, 'y'); } } else if edge { p.n(x, 10, z, "fence"); } } }
    for (x, z, dx, dz) in [(3, 0, 0, 1), (3, 6, 0, -1), (0, 3, 1, 0), (6, 3, -1, 0)] { p.stair(x, 8, z, "stone_bricks_stairs", dx, dz, true); p.n(x, 11, z, "lantern"); }
    for x in [1, 5] { for z in [1, 5] { p.n(x, 8, z, "cobblestone"); } }
    // l'échelle contre le mur du dehors, du bas jusqu'au guet, et les trémies des planchers
    for y in 1..=9 { p.turn(2, y, 2, "ladder", 0, -1); } p.put(2, 10, 2, 0);
    // le toit de tuiles en pyramide, son épi et le drapeau
    p.pyramid(3, 3, 4, 12, "roof_tiles"); p.n(3, 17, 3, "flag");
}

/// La porte fortifiée, dans son repère : 11 de long sur la muraille, 5 de profondeur, le dehors en z = 0.
fn gatehouse(p: &mut P) {
    for x in 0..11 { for z in 0..5 { for y in 1..=13 { p.put(x, y, z, 0); } let lane = (4..=6).contains(&x); for y in -3..=if lane { -1 } else { 0 } { p.n(x, y, z, "cobblestone"); }
        // le massif de pierre : pavé en pied et aux angles, pierre de taille au-dessus ; le passage de trois, voûté, la herse relevée
        for y in 1..=5 { let quoin = x == 0 || x == 10 || x == 3 || x == 7; p.n(x, y, z, if y == 1 || quoin && y <= 5 && (z == 0 || z == 4) { "cobblestone" } else { "stone_bricks" }); }
        if lane { for y in 1..=3 { p.put(x, y, z, 0); } if x == 5 { p.put(x, 4, z, 0); } } } }
    for x in 4..=6 { p.n(x, 4, 1, "iron_bars"); } p.n(5, 5, 0, "chiseled_stone");
    // l'étage : deux tourelles de pierre aux bouts, une salle à pans de bois entre elles ; le chemin de ronde la traverse
    p.frame((2, 8), (0, 4), (6, 8), 2, "plaster"); p.belt((2, 8), (0, 4), 9);
    for x0 in [0, 8] { for x in x0..=x0 + 2 { for z in 0..5 { let edge = x == x0 || x == x0 + 2 || z == 0 || z == 4; for y in 6..=10 { if edge { p.n(x, y, z, if y == 6 { "cobblestone" } else { "stone_bricks" }); } else { p.put(x, y, z, 0); } } } } }
    for x in 0..11 { for y in 6..=7 { p.put(x, y, 2, 0); } } for x in 0..11 { p.n(x, 5, 2, "planks"); }
    for x in 3..=7 { for z in 1..=3 { p.n(x, 5, z, "planks"); } }
    p.win(5, 7, 0, true); p.win(5, 7, 4, true); p.win(1, 8, 0, true); p.win(9, 8, 0, true); p.win(1, 8, 4, true); p.win(9, 8, 4, true);
    // les toits : à deux pans sur la salle, en pyramide sur les tourelles, leurs drapeaux
    p.roof((3, 7), (0, 4), 10, 0, false, "roof_tiles", Some("plaster")); p.pyramid(1, 2, 2, 11, "roof_tiles"); p.pyramid(9, 2, 2, 11, "roof_tiles"); p.n(1, 14, 2, "flag"); p.n(9, 14, 2, "flag");
    // les bannières rouges sur les tourelles, les appliques de part et d'autre du passage, dehors et côté village
    for x in [1, 9] { p.turn(x, 4, -1, "banner_red", 0, 1); p.turn(x, 4, 5, "banner_red", 0, -1); }
    for x in [3, 7] { p.sconce(x, 3, -1, 0, 1); p.sconce(x, 3, 5, 0, -1); }
}
