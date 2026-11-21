//! Le catalogue des bâtisseurs : chaque nouvelle pièce existe comme bloc (et dans ses quatre sens quand elle s'oriente), se
//! dessine dans son bloc quel que soit son sens, est un objet du catalogue créatif et a une recette en survie ; la lanterne
//! murale éclaire.
use alw_core::{blocks::*, data::def, mesher::{VBuf, VSIZE}, meubles, noise::Rng, survival::recipes};

const TURNED: [&str; 15] = ["balustrade_stone", "balustrade_wood", "iron_railing", "picket_fence", "garden_gate", "flower_box", "awning_red", "awning_blue", "wall_lantern",
    "window_arched", "corbel", "stone_arch", "lattice_roses", "drainpipe", "hanging_sign"];
const PLAIN: [&str; 5] = ["chimney_pot", "stained_glass_blue", "stained_glass_red", "stained_glass_green", "stained_glass_amber"];

#[test]
fn chaque_piece_est_un_bloc_un_objet_et_une_recette() {
    for n in TURNED.iter().chain(PLAIN.iter()) {
        let b = by_name(n).unwrap_or_else(|| panic!("le bloc {n}")); let d = def(n).unwrap_or_else(|| panic!("l'objet {n}")); assert_eq!(d.block, Some(b), "{n} se pose comme son bloc");
        assert!(["batir", "decor"].contains(&d.cat), "{n} dans le catalogue des bâtisseurs ({})", d.cat);
        let r = recipes().iter().find(|r| r.out == *n).unwrap_or_else(|| panic!("la recette de {n}")); for need in &r.need { let _ = need; }
        assert!(r.n >= 1);
    }
    for n in TURNED { for f in 1..4 { assert!(by_name(&format!("{n}@{f}")).is_some(), "{n} s'oriente ({f})"); } }
    assert!(BLOCKS[by_name("wall_lantern").unwrap() as usize].light >= 12, "la lanterne murale éclaire");
    for n in ["balustrade_stone", "iron_railing", "picket_fence", "garden_gate", "window_arched", "stone_arch", "chimney_pot"] { assert!(BLOCKS[by_name(n).unwrap() as usize].solid, "{n} arrête le joueur"); }
}

#[test]
fn chaque_piece_se_dessine_dans_son_bloc() {
    for n in TURNED.iter().chain(["chimney_pot"].iter()) {
        let model = BLOCKS[by_name(n).unwrap() as usize].model.unwrap_or_else(|| panic!("le modèle de {n}"));
        for f in 0..4 { let name = if f == 0 { n.to_string() } else { format!("{n}@{f}") }; if by_name(&name).is_none() { continue; }
            let mut o = VBuf::new(4000); assert!(meubles::draw(&mut o, model, &name, 0.0, 0.0, 0.0, 1.0, 1.0, &mut Rng::new(7)), "{name} se dessine"); assert!(o.n >= 24, "{name} : {} sommets", o.n);
            for v in o.data.chunks_exact(VSIZE) { let g = |i: usize| i16::from_le_bytes([v[i], v[i + 1]]) as f64 / o.scale; for (i, c) in [(0, 'x'), (2, 'y'), (4, 'z')] { assert!((-0.02..=1.02).contains(&g(i)), "{name} déborde de son bloc en {c} : {:.3}", g(i)); } } }
    }
}
