//! Les structures exportées : format texte, orientation des blocs, et relevé d'un bâtiment de la galerie qui, reposé
//! tourné ou en miroir, redonne exactement le patron d'origine.
use alw_core::{batisse::{Lot, Work}, blocks::{by_name, BLOCKS}, gen::FLAT, patrons::{build, Kind, Look}, player::Input, session::*, stream::Preset, structures::*};
use std::collections::HashMap;
fn ready(s: &mut Session) { for _ in 0..4000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn id(n: &str) -> u16 { by_name(n).unwrap_or_else(|| panic!("{n}")) }
const ROT: [[i32; 2]; 2] = [[0, 1], [-1, 0]]; // +x → +z, +z → -x
const MIR: [[i32; 2]; 2] = [[-1, 0], [0, 1]];

#[test]
fn le_texte_se_relit_a_l_identique() {
    let mut cells = vec![Cell::Keep; 2 * 3 * 2]; cells[1] = Cell::Block(0); cells[2] = Cell::Block(id("planks_stairs_2")); cells[5] = Cell::Block(id("roof_tiles_slope_top_1")); cells[11] = Cell::Block(id("lantern"));
    let st = Structure { name: "essai".into(), size: [2, 3, 2], origin: [-1, -1, 0], cells };
    let t = st.to_text(); let back = parse(&t).unwrap(); assert_eq!(back.len(), 1); assert_eq!(back[0].cells, st.cells, "{t}"); assert_eq!(back[0].origin, st.origin);
    assert_eq!(back[0].to_text(), t);
}

#[test]
fn les_blocs_orientes_tournent_et_se_retournent() {
    for b in BLOCKS.iter().filter(|b| alw_core::shapes::info(b.id).is_some() || b.name.ends_with("_x") || b.name.ends_with("_z")) {
        let mut r = b.id; for _ in 0..4 { r = orient(r, ROT); } assert_eq!(BLOCKS[r as usize].name, b.name, "quatre quarts de tour");
        assert_eq!(BLOCKS[orient(orient(b.id, MIR), MIR) as usize].name, b.name, "deux miroirs");
        assert_eq!(BLOCKS[orient(orient(b.id, ROT), inverse(ROT)) as usize].name, b.name, "tour puis retour");
    }
    assert_eq!(BLOCKS[orient(id("planks_stairs"), ROT) as usize].name, "planks_stairs_3"); assert_eq!(BLOCKS[orient(id("oak_beam_x"), ROT) as usize].name, "oak_beam_z");
}

/// Le patron posé sur un terrain : la dernière pose l'emporte, comme dans le monde.
fn laid(kind: Kind, var: u8, lot: Lot) -> HashMap<[i32; 3], u16> {
    let mut k = Work::default(); build(&mut k, lot, 60, kind, var, Look { roof: "roof_tiles", infill: "plaster" }, 0.4);
    k.blocks.iter().map(|b| ([b.0, b.1, b.2], b.3)).collect()
}

#[test]
fn un_batiment_de_la_galerie_exporte_redonne_son_patron() {
    let mut s = Session::new(FLAT | 3, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 1, [0.5, 56.0, 0.5]); s.set_creative(true); s.third = false;
    let ex = s.exhibits(); assert!(ex.len() >= 12, "la galerie : {}", ex.len());
    let turned = [Lot { o: (100, 40), u: (0, 1), w: (-1, 0) }, Lot { o: (-50, 7), u: (-1, 0), w: (0, -1) }];
    for (name, var) in [("maison", 0u8), ("maison_4", 4)] {
        let e = *ex.iter().find(|e| e.kind == Kind::Maison && e.var == var).unwrap(); assert!(get(name).is_none());
        let before: Vec<_> = turned.iter().map(|l| laid(Kind::Maison, var, *l)).collect();
        let c = e.lot.cell(1, e.y0 + 1, 2); s.player.p = [c[0] as f64 + 0.5, c[1] as f64, c[2] as f64 + 0.5]; ready(&mut s); s.tick(0.0, Input::default(), Actions::default());
        let msg = s.command("/exporter"); assert!(msg.contains(&format!("« {name} »")), "{msg}");
        let (file, text) = s.exports.pop().unwrap(); assert_eq!(file, format!("{name}.txt")); let st = parse(&text).unwrap().remove(0); assert_eq!(st.name, name);
        assert!(st.door().is_some(), "la porte est retrouvée");
        for (l, want) in turned.iter().zip(&before) {
            let got = laid(Kind::Maison, var, *l); let mut bad = Vec::new();
            for (p, b) in &got { if want.get(p).copied().unwrap_or(0) != *b { bad.push(format!("{p:?} : {} au lieu de {}", BLOCKS[*b as usize].name, BLOCKS[want.get(p).copied().unwrap_or(0) as usize].name)); } }
            for (p, b) in want { if *b != 0 && !got.contains_key(p) { bad.push(format!("{p:?} : {} manque", BLOCKS[*b as usize].name)); } }
            assert!(bad.is_empty(), "{name} sur {l:?} : {} écarts\n{}", bad.len(), bad.iter().take(12).cloned().collect::<Vec<_>>().join("\n"));
        }
        // la structure est bien celle qui est posée : une retouche s'y voit, tournée avec le reste
        assert!(find(Kind::Maison, var).is_some_and(|f| f.name == name));
        let mut edited = st.clone(); let gold = id("wool_purple"); let (cx, cz) = (edited.size[0] / 2, edited.size[2] / 2); let top = edited.size[1] - 1;
        let i = (cx + edited.size[0] * (cz + edited.size[2] * top)) as usize; edited.cells[i] = Cell::Block(gold); add(edited);
        assert_eq!(laid(Kind::Maison, var, turned[0]).values().filter(|b| **b == gold).count(), 1, "la retouche apparaît");
        assert!(forget(name));
    }
    // une boîte libre, marquée de deux coins, face au joueur
    s.player.p = [-20.5, 55.0, -20.5]; ready(&mut s); s.player.yaw = 0.0; s.player.pitch = -1.2; s.tick(0.0, Input::default(), Actions::default());
    for (x, z) in [(-21, -18), (-20, -18), (-21, -17)] { s.world.edit(x, 55, z, id("planks")); }
    s.coins = [Some([-21, 55, -18]), Some([-20, 56, -17])]; let msg = s.command("/exporter cabane"); assert!(msg.contains("2 × 2 × 2") && msg.contains("3 blocs"), "{msg}");
    assert!(get("cabane").is_some()); assert!(kind_of("cabane").is_none() && kind_of("maison_3") == Some(Kind::Maison) && kind_of("forge") == Some(Kind::Forge));
}
