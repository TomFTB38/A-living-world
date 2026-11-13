//! Aucun trou dans les bâtiments des patrons. Cherche les trous des bâtiments. On part de l'air des combles (sous les toits) et de l'intérieur fermé, et l'on
//! cherche par où il s'échappe vers une case à ciel ouvert : chaque sortie est un trou dans un mur, un pignon ou un toit.
use alw_core::{batisse::{Lot, Work}, blocks::BLOCKS, edifice::{raise, Arch, Cond, Spec}};
use std::collections::{HashMap, HashSet, VecDeque};
#[test]
fn aucun_trou_dans_les_batiments() {
    // archétype, hauteur des combles, cases de l'intérieur fermé (rez-de-chaussée)
    let all: [(&str, Arch, &[(i32, i32, i32, i32)]); 12] = [("villageois", Arch::House, &[(3, 1, 2, 1), (3, 5, 3, 4)]), ("forge", Arch::Forge, &[(3, 6, 4, 5)]), ("auberge", Arch::Inn, &[(6, 1, 2, 1), (6, 5, 4, 5), (6, 9, 5, 8)]), ("moulin", Arch::Mill, &[(4, 1, 2, 1), (4, 6, 4, 5), (4, 10, 4, 10)]), ("tannerie", Arch::Tannery, &[(3, 6, 4, 5), (10, 6, 4, 5), (3, 1, 2, 1)]), ("eglise", Arch::Chapel, &[(5, 1, 7, 1), (5, 10, 8, 9), (5, 7, 15, 7)]), ("grande_mairie", Arch::TownHall, &[(9, 1, 6, 1), (9, 6, 6, 6), (9, 12, 7, 10), (2, 11, 6, 10), (16, 11, 6, 10)]), ("petite_mairie", Arch::Hall, &[(4, 1, 3, 1), (4, 6, 3, 5)]), ("atelier", Arch::Woodshop, &[(3, 1, 2, 1), (3, 6, 4, 5)]), ("ferme", Arch::Farm, &[(5, 1, 2, 1), (5, 6, 4, 5), (11, 5, 5, 4)]), ("etable", Arch::Fold, &[(6, 6, 4, 4)]), ("ecurie", Arch::Stable, &[(6, 6, 4, 4)])];
    let want: Vec<String> = Vec::new(); let mut total = 0;
    for (name, arch, rooms) in all { if !want.is_empty() && !want.iter().any(|w| w == name) { continue; }
        for var in 0..arch.variants() {
            let sp = Spec::new(arch, var, 0.4, 0.5, match arch { Arch::Forge => Some("forgeron"), Arch::Tannery => Some("tanneur"), Arch::Woodshop => Some("maitre_menuisier"), _ => None }, Cond::Kept);
            let mut k = Work::default(); let lot = Lot { o: (0, 0), u: (1, 0), w: (0, 1) }; raise(&mut k, lot, &sp, 0);
            let mut m: HashMap<(i32, i32, i32), u16> = HashMap::new(); for &(x, y, z, b) in &k.blocks { m.insert((x, y, z), b); }
            let top = m.keys().map(|c| c.1).max().unwrap();
            let get = |x: i32, y: i32, z: i32| -> u16 { if y <= 0 && !m.contains_key(&(x, y, z)) { 1 } else { *m.get(&(x, y, z)).unwrap_or(&0) } };
            let covered = |x: i32, y: i32, z: i32| (y + 1..=top).any(|yy| get(x, yy, z) != 0);
            let mirror = var & if arch == Arch::House { 4 } else { 1 } != 0; let mx = |x: i32| if mirror { sp.w - 1 - x } else { x };
            let mut holes: Vec<String> = Vec::new();
            // 1) les combles : l'air couvert au-dessus de `ys`, au-dessus du corps ; 2) chaque pièce fermée
            let runs: Vec<(Vec<(i32, i32, i32)>, i32)> = rooms.iter().map(|r| (vec![(mx(r.0), r.1, r.2)], r.3)).collect();
            for (seeds, ymin) in runs { let mut par: HashMap<(i32, i32, i32), (i32, i32, i32)> = HashMap::new(); let mut seen: HashSet<(i32, i32, i32)> = seeds.iter().copied().collect(); let mut q: VecDeque<_> = seeds.into_iter().collect();
                while let Some((x, y, z)) = q.pop_front() { for (dx, dy, dz) in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)] { let c = (x + dx, y + dy, z + dz);
                    if c.1 < ymin || seen.contains(&c) || get(c.0, c.1, c.2) != 0 { continue; }
                    if !covered(c.0, c.1, c.2) { if std::env::var("CHEMIN").is_ok() { let mut p = (x, y, z); let mut path = vec![]; while let Some(&q) = par.get(&p) { path.push(p); p = q; } println!("chemin {:?}", path.iter().rev().map(|p| (mx(p.0), p.1, p.2)).collect::<Vec<_>>()); std::process::exit(0); } let n = |p: (i32, i32, i32)| BLOCKS[get(p.0, p.1, p.2) as usize].name; holes.push(format!("({},{},{})→({},{},{}) [{} | {}]", mx(x), y, z, mx(c.0), c.1, c.2, n((x, y + 1, z)), n((x - dx, y, z - dz)))); continue; }
                    seen.insert(c); par.insert(c, (x, y, z)); q.push_back(c); } } }
            holes.sort(); holes.dedup(); total += holes.len();
            println!("{name} v{var} : {} trous {}", holes.len(), holes.iter().take(14).cloned().collect::<Vec<_>>().join(" "));
        } }
    assert_eq!(total, 0, "des trous dans les bâtiments (voir ci-dessus ; `cargo run --example trous` pour le détail)");
}
