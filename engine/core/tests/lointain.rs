//! Le relief lointain : il tient la place des chunks pas encore chargés et prolonge la vue jusqu'à près de 8 km.
//! On vérifie qu'il ressemble au monde qui le remplacera (même hauteur, même bloc de surface, arbres là où ils pousseront),
//! qu'il tient dans ses coordonnées, et que le chargement demande puis oublie ses tuiles d'arbres comme il faut.
use alw_core::blocks::{idx, is_leaf, is_opaque, Id, B, CH, CX};
use alw_core::gen::WorldGen;
use alw_core::mesher::{far_mesh, far_trees_mesh, kind, FAR_SCALE, TREE_SCALE, VSIZE};
use alw_core::stream::{Streamer, FAR, HIGH, TREES};

const SEED: i32 = 20261104;
/// Le bloc du dessus d'une colonne réelle (sans les plantes, la neige en pellicule comptant pour de la neige), et sa hauteur.
fn real_top(v: &[Id], x: usize, z: usize) -> Option<(Id, i32)> {
    for y in (1..CH - 1).rev() { let b = v[idx(x, y, z)]; if b == 0 { continue; }
        if is_leaf(b) || alw_core::blocks::BLOCKS[b as usize].name.contains("_log") { return None; } // sous un arbre : on ne compare pas
        if alw_core::blocks::BLOCKS[b as usize].name.starts_with("snow") { return Some((B::SNOW, y as i32)); }
        if is_opaque(b) || alw_core::blocks::is_water(b) { return Some((b, y as i32 + 1)); } }
    None
}

#[test]
fn le_lointain_ressemble_au_monde() {
    let g = WorldGen::new(SEED); let (mut same_b, mut same_h, mut n) = (0, 0, 0);
    // Des chunks épars, dans des biomes variés.
    for k in 0..24 { let (cx, cz) = ((k % 6) * 37 - 90, (k / 6) * 41 - 60); let v = g.gen_world(cx, cz);
        for z in (1..CX).step_by(4) { for x in (1..CX).step_by(4) {
            let Some((b, h)) = real_top(&v, x, z) else { continue }; if alw_core::blocks::is_water(b) { continue; }
            let (wx, wz) = ((cx * CX as i32 + x as i32) as f64 + 0.5, (cz * CX as i32 + z as i32) as f64 + 0.5); let c = g.column(wx, wz);
            if c.water != 0.0 && c.water > c.h { continue; }
            let (fb, _) = g.far_surface(&c, wx, wz); n += 1; if fb == b { same_b += 1; } if (c.h.ceil() as i32 - h).abs() <= 1 { same_h += 1; }
        } } }
    let (pb, ph) = (same_b as f64 / n as f64, same_h as f64 / n as f64);
    println!("{n} colonnes : même bloc de surface {:.0} %, même hauteur à un bloc près {:.0} %", pb * 100.0, ph * 100.0);
    assert!(pb > 0.75, "le lointain doit avoir la couleur du terrain qui le remplacera");
    assert!(ph > 0.85, "et sa hauteur");
}

#[test]
fn les_arbres_lointains_sont_la_ou_poussent_les_vrais() {
    let g = WorldGen::new(SEED); let (mut inside, mut n) = (0, 0);
    let boxes = g.far_trees(-200, -200, 200, 200); println!("{} boîtes d'arbres sur 400 × 400 blocs", boxes.len()); assert!(boxes.len() > 100, "il y a des arbres");
    for cz in -4..4 { for cx in -4..4 { let v = g.gen_world(cx, cz);
        for y in 0..CH { for z in 0..CX { for x in 0..CX { let b = v[idx(x, y, z)]; if !is_leaf(b) { continue; }
            let (wx, wy, wz) = (cx * CX as i32 + x as i32, y as i32, cz * CX as i32 + z as i32); if wx < -190 || wz < -190 || wx >= 190 || wz >= 190 { continue; }
            n += 1; if boxes.iter().any(|(bb, _)| wx >= bb[0] - 1 && wx < bb[3] + 1 && wy >= bb[1] - 1 && wy < bb[4] + 1 && wz >= bb[2] - 1 && wz < bb[5] + 1) { inside += 1; }
        } } } } }
    let p = inside as f64 / n.max(1) as f64; println!("{n} feuilles réelles, {:.0} % dans une couronne lointaine (à un bloc près)", p * 100.0);
    assert!(n > 1000 && p > 0.8, "les couronnes lointaines doivent couvrir le feuillage réel");
    let (_, nv) = far_trees_mesh(&g, 0, 0); assert!(nv > 0 && nv % 4 == 0);
}

#[test]
fn le_relief_lointain_tient_dans_ses_coordonnees() {
    let g = WorldGen::new(SEED); let near = Streamer::near_of(HIGH.radius);
    let t = std::time::Instant::now(); let (d, n) = far_mesh(&g, 3, -2, near); println!("relief lointain : {n} sommets, {:.1} Mo, {:.0} ms", d.len() as f64 / 1e6, t.elapsed().as_secs_f64() * 1e3);
    assert!(n > 0 && n % 4 == 0 && d.len() == n * VSIZE);
    let (mut ext, mut kinds) = (0f64, [0usize; 6]);
    for v in d.chunks(VSIZE) { let p = |i: usize| i16::from_le_bytes([v[i * 2], v[i * 2 + 1]]) as f64 / FAR_SCALE; ext = ext.max(p(0).abs()).max(p(2).abs());
        assert!(p(0).abs() < 8191.0 && p(2).abs() < 8191.0, "aucun sommet ne bute sur la limite des coordonnées"); kinds[v[15] as usize] += 1; }
    println!("porte à {ext:.0} blocs ; sommets par sorte {kinds:?}");
    assert!(ext > 7500.0, "la vue porte à plus de 7 km");
    assert!(kinds[kind::TEX as usize] > 0 && kinds[kind::FAR as usize] > 0, "des colonnes texturées près, un relief lisse au loin");
    let _ = TREE_SCALE;
}

#[test]
fn le_chargement_demande_puis_oublie_les_arbres_lointains() {
    let mut s = Streamer::new(SEED, &HIGH, 4); let (mut far, mut trees, mut chunks) = (0, std::collections::HashSet::new(), 0);
    let t = std::time::Instant::now();
    loop { let (b, _) = s.update(16.0, 16.0); for c in b { match c.lod { FAR => far += 1, TREES => { assert!(c.mesh.n_o > 0); trees.insert(c.key); } _ => chunks += 1 } } if s.waiting() == 0 && far > 0 { break; } assert!(t.elapsed().as_secs() < 240); std::thread::sleep(std::time::Duration::from_millis(10)); }
    println!("{chunks} chunks, {far} horizon, {} tuiles d'arbres en {:.1} s", trees.len(), t.elapsed().as_secs_f64());
    assert!(far == 1 && !trees.is_empty());
    // Aucune tuile tout entière sous les chunks chargés : elles ne servent qu'au-delà.
    let r = (HIGH.radius as f64 - 1.0) * 32.0; let t_ = alw_core::mesher::TREE_TILE as f64;
    for k in &trees { let (x0, z0) = (k.0 as f64 * t_, k.1 as f64 * t_); let far_c = ((x0 - 16.0).abs().max((x0 + t_ - 16.0).abs())).hypot((z0 - 16.0).abs().max((z0 + t_ - 16.0).abs())); assert!(far_c >= r, "tuile {k:?} sous les chunks chargés"); }
    // On s'en va loin : l'horizon se recentre, les tuiles d'avant sont oubliées (un maillage vide), d'autres viennent.
    let (mut forgot, mut new) = (0, 0); let t = std::time::Instant::now();
    loop { let (b, _) = s.update(16.0 + 32.0 * 9.0, 16.0); for c in b { if c.lod == TREES { if c.mesh.n_o == 0 { assert!(trees.contains(&c.key)); forgot += 1; } else { new += 1; } } } if s.waiting() == 0 { break; } assert!(t.elapsed().as_secs() < 240); std::thread::sleep(std::time::Duration::from_millis(10)); }
    println!("après 9 chunks : {forgot} tuiles oubliées, {new} nouvelles");
    assert!(forgot > 0 && new > 0);
}
