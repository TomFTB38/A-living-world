//! Les villages médiévaux : nom, taille, centre (ALW_BOURG=1 pour le détail des bâtiments).
use alw_core::{bourg::Civ, gen::WorldGen};
fn main() {
    let g = WorldGen::new(20261104);
    for v in g.villages_near(0.0, 0.0, 3000.0).into_iter().filter(|v| v.ident.is_some_and(|i| i.civ == Civ::Medieval)) { println!("VILLAGE {} {:?} c {:?} r {} {} maisons", v.name, v.ident.unwrap().size, v.c, v.r, v.houses.len()); }
}
