//! Toits qui se lient (arêtiers, noues), toits renversés, et boîtes fidèles au dessin (visée, contour, collisions).
use alw_core::{blocks::*, player::Voxels, shapes::{self, Kind}};
use std::collections::HashMap;

struct W(HashMap<(i32, i32, i32), Id>);
impl Voxels for W { fn get(&self, x: i32, y: i32, z: i32) -> Option<Id> { Some(*self.0.get(&(x, y, z)).unwrap_or(&0)) } }
fn slope(f: u8, top: bool) -> Id { shapes::variant(by_name("roof_tiles_slope").unwrap(), f, top, false, false) }
fn rot(d: (i32, i32), r: u8) -> (i32, i32) { let mut d = d; for _ in 0..r { d = (d.1, -d.0); } d } // un quart de tour : +z devient +x

#[test]
fn les_pans_perpendiculaires_font_arêtier_et_noue() {
    for top in [false, true] { for r in 0..4u8 {
        let f = |k: u8| (k + r) % 4;
        // angle sortant : la case d'angle (pente vers +z), la pente d'au-dessus monte vers +x
        let up = rot((0, 1), r); let j = shapes::roof_join(slope(f(0), top), |dx, dz| if (dx, dz) == up { slope(f(1), top) } else { 0 });
        let i = shapes::info(j).unwrap(); assert_eq!((i.kind, i.facing, i.top), (Kind::Corner, f(0), top), "arêtier, rotation {r}, renversé {top}");
        // l'autre pan posé en premier donne le même arêtier
        let up = rot((1, 0), r); let j2 = shapes::roof_join(slope(f(1), top), |dx, dz| if (dx, dz) == up { slope(f(0), top) } else { 0 }); assert_eq!(j2, j);
        // angle rentrant : la pente d'en dessous monte vers +x
        let down = rot((0, -1), r); let v = shapes::roof_join(slope(f(0), top), |dx, dz| if (dx, dz) == down { slope(f(1), top) } else { 0 });
        let i = shapes::info(v).unwrap(); assert_eq!((i.kind, i.facing, i.top), (Kind::Valley, f(0), top), "noue, rotation {r}");
        // seule, ou entre deux pentes parallèles, une pente reste une pente
        assert_eq!(shapes::roof_join(slope(f(0), top), |dx, dz| if dx != 0 && dz == 0 { slope(f(0), top) } else { 0 }), slope(f(0), top));
    } }
    // l'arêtier raccorde ses deux pans sans vide : sur chaque face partagée, la même hauteur que la pente voisine
    let c = shapes::geometry(shapes::info(by_name("roof_tiles_corner").unwrap()).unwrap()); let s0 = shapes::geometry(shapes::info(slope(0, false)).unwrap());
    let h_at = |g: &[shapes::Bx], x: f64, z: f64| g.iter().filter(|b| b[0] <= x && x <= b[3] && b[2] <= z && z <= b[5]).map(|b| b[4]).fold(0.0, f64::max);
    for k in 1..8 { let z = k as f64 / 8.0 - 0.01; assert!((h_at(&c, 0.999, z) - h_at(&s0, 0.001, z)).abs() < 0.13, "arêtier contre sa pente en z = {z}"); }
}

#[test]
fn tous_les_toits_se_renversent() {
    for b in BLOCKS.iter() { let Some(i) = shapes::info(b.id) else { continue }; if !shapes::is_roof(b.id) || i.top { continue; }
        let t = shapes::variant(i.base, i.facing, true, false, false); assert_ne!(t, b.id, "{} n'a pas de version renversée", b.name);
        let (g, gt) = (shapes::geometry(i), shapes::geometry(shapes::info(t).unwrap()));
        // renversé : la même forme retournée (le volume est le même, le haut devient le bas)
        let vol = |v: &[shapes::Bx]| v.iter().map(|b| (b[3] - b[0]) * (b[4] - b[1]) * (b[5] - b[2])).sum::<f64>(); assert!((vol(&g) - vol(&gt)).abs() < 1e-9, "{}", b.name);
        assert!(gt.iter().any(|b| b[4] > 0.999) && g.iter().any(|b| b[1] < 0.001), "{} : posé au sol, renversé au plafond", b.name);
    }
}

#[test]
fn les_marches_des_pentes_collent_au_dessin() {
    // pente vers +z : la surface est y = z ; aucune marche ne dépasse d'un huitième au-dessus, aucune ne laisse un trou de plus
    let g = shapes::geometry(shapes::info(slope(0, false)).unwrap());
    for k in 0..64 { let z = (k as f64 + 0.5) / 64.0; let top = g.iter().filter(|b| b[2] <= z && z <= b[5]).map(|b| b[4]).fold(0.0, f64::max); assert!(top >= z - 1e-9 && top - z <= 0.125 + 1e-9, "z = {z} : marche à {top}"); }
}

#[test]
fn on_vise_et_on_bute_la_ou_le_bloc_est_dessiné() {
    let mut w = W(HashMap::new()); let post = B::OAK_POST; w.0.insert((0, 0, 0), post); w.0.insert((0, 0, 3), B::STONE);
    // un rayon qui passe à côté de la poutre (dans sa case, hors de sa section) la manque et touche la pierre derrière
    let hit = alw_core::session::raycast(&w, [0.1, 0.5, -2.0], [0.0, 0.0, 1.0], 10.0, &|_, _, _, _| false).unwrap().0; assert_eq!((hit.x, hit.y, hit.z), (0, 0, 3), "à côté de la poutre");
    let hit = alw_core::session::raycast(&w, [0.5, 0.5, -2.0], [0.0, 0.0, 1.0], 10.0, &|_, _, _, _| false).unwrap().0; assert_eq!((hit.x, hit.z), (0, 0)); assert!((hit.bx[3] - hit.bx[0] - 0.4).abs() < 1e-9, "contour : la section de la poutre, {:?}", hit.bx);
    // une dalle basse : on passe au-dessus
    let mut w = W(HashMap::new()); w.0.insert((0, 0, 0), by_name("planks_slab").unwrap()); w.0.insert((0, 0, 3), B::STONE);
    assert_eq!(alw_core::session::raycast(&w, [0.5, 0.8, -2.0], [0.0, 0.0, 1.0], 10.0, &|_, _, _, _| false).unwrap().0.z, 3);
    // un toit se vise sur sa pente : au-dessus de la surface inclinée on passe, en dessous on le touche
    let mut w = W(HashMap::new()); w.0.insert((0, 0, 0), slope(0, false)); w.0.insert((0, 0, 3), B::STONE);
    let ray = |y: f64| alw_core::session::raycast(&w, [0.5, y, -2.0], [0.0, 0.0, 1.0], 10.0, &|_, _, _, _| false).unwrap().0.z; assert_eq!(ray(0.9), 0, "sous le faîte, la tranche haute"); 
    let down = alw_core::session::raycast(&w, [0.5, 3.0, 0.2], [0.0, -1.0, 0.0], 10.0, &|_, _, _, _| false).unwrap(); assert_eq!(down.0.y, 0); assert!((down.1 - 2.8).abs() < 0.01, "sur la pente à z = 0,2 : touchée à 0,2 de haut, t = {}", down.1);
    // la clôture : seule, un poteau ; contre un mur, ses lisses vont jusqu'à lui
    let mut w = W(HashMap::new()); w.0.insert((0, 0, 0), B::FENCE); let alone = shapes::cell_boxes(&w, 0, 0, 0, B::FENCE, true).unwrap(); assert_eq!(alone.len(), 1);
    w.0.insert((1, 0, 0), B::STONE); let v = shapes::cell_boxes(&w, 0, 0, 0, B::FENCE, true).unwrap(); assert!(v.iter().any(|b| b[3] >= 1.0), "lisses jusqu'au mur");
    // tout bloc de décor solide a une boîte dans sa case, tirée de son dessin
    for b in BLOCKS.iter().filter(|b| b.shape == Shape::Deco && b.solid && shapes::info(b.id).is_none()) {
        if let Some(v) = shapes::own_boxes(b.id, true) { for bb in v { assert!(bb[0] < bb[3] && bb[1] < bb[4] && bb[2] < bb[5] && bb.iter().all(|c| (0.0..=1.0).contains(c)), "{} : {bb:?}", b.name); } } }
}
