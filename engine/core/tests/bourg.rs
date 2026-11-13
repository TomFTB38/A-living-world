//! Validation du générateur de villages médiévaux (phases 1 et 2 de la refonte) : identité, bâtiments posés sans
//! chevauchement, portes, intérieurs accessibles, chemins praticables jusqu'à la place, villages tous différents,
//! et rien qui ne change d'un tracé à l'autre.
use alw_core::{blocks::*, bourg::Civ, gen::WorldGen, store::ChunkStore};
const SEED: i32 = 20261104;

fn medievaux(g: &WorldGen) -> Vec<alw_core::village::Village> { g.villages_near(0.0, 0.0, 3000.0).into_iter().filter(|v| v.ident.is_some_and(|i| i.civ == Civ::Medieval)).collect() }

#[test]
fn des_villages_medievaux_tous_differents() {
    let g = WorldGen::new(SEED); let all = medievaux(&g);
    assert!(all.len() >= 6, "{} villages médiévaux", all.len());
    let sizes: std::collections::HashSet<_> = all.iter().map(|v| v.ident.unwrap().size).collect(); assert!(sizes.len() >= 2, "plusieurs tailles : {sizes:?}");
    // Deux villages ne se ressemblent jamais : identité, nombre de maisons ou plan diffèrent.
    for (i, a) in all.iter().enumerate() { for b in &all[i + 1..] {
        let same = a.ident.unwrap().describe() == b.ident.unwrap().describe() && a.houses.len() == b.houses.len() && a.r == b.r; assert!(!same, "{} et {} identiques", a.name, b.name); } }
    // Le même monde redonne exactement les mêmes villages.
    let again = medievaux(&WorldGen::new(SEED));
    for (a, b) in all.iter().zip(&again) { assert_eq!(a.houses.len(), b.houses.len()); for (h, k) in a.houses.iter().zip(&b.houses) { assert_eq!(h.door, k.door); assert_eq!(h.way.len(), k.way.len()); } }
    for v in &all { assert!(v.houses.len() >= 2, "{} : {} maisons", v.name, v.houses.len()); assert!(v.crafts.len() >= 1, "{} sans artisan", v.name); }
}

#[test]
fn batiments_valides_et_chemins_praticables() {
    let all = medievaux(&WorldGen::new(SEED)); let mut st = ChunkStore::new(SEED, 96);
    for v in all.iter().take(4) {
        // Pas de chevauchement entre bâtiments habités.
        for (i, a) in v.houses.iter().enumerate() { for b in &v.houses[i + 1..] { let (p, q) = (a.area, b.area); assert!(p[2] < q[0] || q[2] < p[0] || p[3] < q[1] || q[3] < p[1], "{} : bâtiments qui se chevauchent {p:?} {q:?}", v.name); } }
        for h in &v.houses {
            let fy = h.door[1].floor() as i32; let (dx, dz) = (h.inside[0] - h.door[0], h.inside[2] - h.door[2]); let d = [(h.door[0] + dx * 0.5).floor() as i32, fy, (h.door[2] + dz * 0.5).floor() as i32];
            assert!(BLOCKS[st.block(d[0], d[1], d[2]) as usize].name.starts_with("door"), "{} : porte attendue en {d:?}", v.name);
            // Dedans : on tient debout près de la porte, au poste et près du lit ; le sol porte.
            for p in [h.inside, h.spot, h.bed] { let (x, z) = (p[0].floor() as i32, p[2].floor() as i32); let y = p[1].floor() as i32;
                assert!(!is_solid(st.block(x, y, z)) && !is_solid(st.block(x, y + 1, z)), "{} : passage bouché en {p:?} ({})", v.name, BLOCKS[st.block(x, y, z) as usize].name);
                assert!(is_solid(st.block(x, y - 1, z)) || BLOCKS[st.block(x, y - 1, z) as usize].name.contains("slab"), "{} : pas de sol sous {p:?}", v.name); }
            // Le chemin jusqu'à la place : des pas d'au plus deux blocs, de l'air pour passer, du sol dessous.
            let mut prev = h.door; let mut bad = 0;
            for p in &h.way { let (x, y, z) = (p[0].floor() as i32, p[1].floor() as i32, p[2].floor() as i32);
                if (p[1] - prev[1]).abs() > 2.0 { bad += 1; eprintln!("marche {prev:?} -> {p:?}"); } if is_solid(st.block(x, y, z)) || !is_solid(st.block(x, y - 1, z)) { bad += 1; eprintln!("case {p:?} : {} sur {}", BLOCKS[st.block(x, y, z) as usize].name, BLOCKS[st.block(x, y - 1, z) as usize].name); } prev = *p; }
            assert!(bad * 10 <= h.way.len().max(1), "{} : chemin difficile depuis {:?} ({bad} sur {})", v.name, h.door, h.way.len());
            assert!((h.way.last().unwrap()[0] - v.c[0] as f64 - 0.5).hypot(h.way.last().unwrap()[2] - v.c[2] as f64 - 0.5) < 9.5, "{} : le chemin finit loin de la place", v.name);
        }
        assert_eq!(BLOCKS[st.block(v.c[0], v.c[1], v.c[2]) as usize].name, "water", "le puits au milieu de la place");
    }
}
