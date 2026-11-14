//! Poutres fines (droites, en diagonale, en angle) et clôtures qui se lient à leurs voisins.
use alw_core::{blocks::*, mesher::{Mesher, VBuf}, shapes::{self, Kind}};

#[test]
fn chaque_poutre_a_sa_forme_et_retrouve_ses_variantes() {
    for sp in shapes::WOODS {
        for (base, kind, n) in [(format!("{sp}_post"), Kind::Post, 3u8), (format!("{sp}_post_diag"), Kind::Brace, 4), (format!("{sp}_post_angle"), Kind::Elbow, 4)] {
            let b = by_name(&base).unwrap_or_else(|| panic!("{base} manque")); let i = shapes::info(b).expect("forme");
            assert_eq!(i.kind, kind, "{base}"); assert_eq!(shapes::material(b), by_name(&format!("{sp}_log")).unwrap(), "{base} : matière du rondin");
            for top in [false, true] { for f in 0..n {
                let v = shapes::variant(b, f, top, false, false); let vi = shapes::info(v).unwrap();
                assert_eq!((vi.kind, vi.facing, vi.base), (kind, f, b), "{base} {f} {top} → {}", BLOCKS[v as usize].name);
                if kind == Kind::Elbow { assert_eq!(vi.top, top); }
                let g = shapes::geometry(vi); assert!(!g.is_empty());
                for bb in &g { assert!(bb.iter().all(|c| (-1e-9..=1.0 + 1e-9).contains(c)), "{} déborde : {bb:?}", BLOCKS[v as usize].name);
                    // plus fine qu'un bloc : jamais plus de 0,4 dans deux directions sur trois
                    let thin = (0..3).filter(|k| bb[k + 3] - bb[*k] <= 0.5 + 1e-9).count(); assert!(thin >= 2, "{} : section {bb:?}", BLOCKS[v as usize].name); }
            } }
        }
    }
    // la mooring_post du port n'est pas une poutre
    assert!(shapes::info(by_name("mooring_post").unwrap()).is_none_or(|i| i.kind != Kind::Post));
}

#[test]
fn la_diagonale_monte_vers_son_sens() {
    // sens 0 (+z) : basse côté -z, haute côté +z ; sens 1 (+x) : pareil le long de x
    for (name, ax) in [("oak_post_diag", 2usize), ("oak_post_diag_1", 0)] {
        let g = shapes::geometry(shapes::info(by_name(name).unwrap()).unwrap());
        let lo = g.iter().min_by(|a, b| a[ax].partial_cmp(&b[ax]).unwrap()).unwrap(); let hi = g.iter().max_by(|a, b| a[ax].partial_cmp(&b[ax]).unwrap()).unwrap();
        assert!(lo[1] < 0.05 && hi[4] > 0.95 && lo[4] < hi[1] + 0.5, "{name} : {g:?}");
    }
}

fn verts(m: &Mesher, id: Id) -> usize { let mut o = VBuf::new(400); m.deco_model(&mut o, id, 5, 5, 5, 85, 85, 255.0, 0.0, 0); o.n }

#[test]
fn les_poutres_se_dessinent_dans_leur_bois() {
    let m = Mesher::new();
    for n in ["oak_post", "oak_post_x", "spruce_post_z", "oak_post_diag_2", "birch_post_angle_top_3", "acacia_post_angle_1"] {
        assert!(verts(&m, by_name(n).unwrap()) >= 24, "{n} sans géométrie");
    }
}

#[test]
fn la_cloture_se_lie_de_tous_les_cotes() {
    let mut m = Mesher::new(); m.set(5, 5, 5, B::FENCE); let alone = verts(&m, B::FENCE); let rail = alone / 2; // le poteau et son chapeau : deux boîtes, comme deux lisses
    for (dx, dz, nb) in [(-1, 0, B::STONE), (0, -1, B::STONE), (1, 0, B::STONE), (0, 1, B::STONE), (-1, 0, B::FENCE), (0, -1, B::OAK_POST), (1, 0, B::OAK_LOG)] {
        let mut m = Mesher::new(); m.set(5, 5, 5, B::FENCE); m.set(5 + dx, 5, 5 + dz, nb);
        assert_eq!(verts(&m, B::FENCE), alone + 2 * rail, "clôture contre {} en ({dx}, {dz}) : deux lisses", BLOCKS[nb as usize].name);
    }
    // entre deux murs : quatre lisses, et toujours son poteau
    let mut m = Mesher::new(); for (x, z) in [(4, 5), (6, 5)] { m.set(x, 5, z, B::STONE); } m.set(5, 5, 5, B::FENCE); assert_eq!(verts(&m, B::FENCE), alone + 4 * rail);
}
