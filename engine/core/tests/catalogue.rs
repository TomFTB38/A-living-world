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

#[test]
fn les_barrieres_se_raccordent_aux_angles() {
    use alw_core::shapes::{cell_boxes, railing_joins}; use alw_core::player::Voxels;
    struct W(std::collections::HashMap<(i32, i32, i32), Id>); impl Voxels for W { fn get(&self, x: i32, y: i32, z: i32) -> Option<Id> { Some(*self.0.get(&(x, y, z)).unwrap_or(&0)) } }
    for n in meubles::RAILINGS {
        let b = by_name(n).unwrap();
        // un angle : une voisine à l'est, une au sud ; la pièce d'angle a ses deux bras, et rien vers l'ouest ni le nord
        let w = W([((1, 0, 0), b), ((0, 0, 1), by_name(&format!("{n}@1")).unwrap())].into_iter().collect());
        let bx = cell_boxes(&w, 0, 0, 0, b, true).expect("des boîtes"); let reach = |f: &dyn Fn(&[f64; 6]) -> bool| bx.iter().any(f);
        assert!(reach(&|q| q[3] >= 1.0), "{n} : un bras vers l'est"); assert!(reach(&|q| q[5] >= 1.0), "{n} : un bras vers le sud");
        assert!(!reach(&|q| q[0] <= 0.0), "{n} : rien vers l'ouest"); assert!(!reach(&|q| q[2] <= 0.0), "{n} : rien vers le nord");
        // les quatre bras, toutes les combinaisons : le dessin reste dans son bloc
        for m in 0..16u8 { let sides = [m & 1 != 0, m & 2 != 0, m & 4 != 0, m & 8 != 0]; let mut o = VBuf::new(4000); meubles::railing(&mut o, n, 0.0, 0.0, 0.0, 1.0, 1.0, meubles::railing_sides(sides, 0));
            assert!(o.n > 0); for v in o.data.chunks_exact(VSIZE) { let g = |i: usize| i16::from_le_bytes([v[i], v[i + 1]]) as f64 / o.scale; for i in [0, 2, 4] { assert!((-0.02..=1.1).contains(&g(i)), "{n} {sides:?} déborde : {:.3}", g(i)); } } }
        // seule, elle court selon son sens ; au bout d'une ligne, elle a sa longueur entière
        assert_eq!(meubles::railing_sides([false; 4], 1), [false, false, true, true]); assert_eq!(meubles::railing_sides([true, false, false, false], 0), [true, true, false, false]);
        assert!(railing_joins(by_name("fence").unwrap()) && railing_joins(by_name("garden_gate").unwrap()) && railing_joins(by_name("stone_bricks").unwrap()), "la clôture, le portillon, un mur la tiennent");
    }
}
