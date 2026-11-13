//! Les objets sans dessin à eux (ceux qui retombent sur le petit paquet).
use alw_core::{data::def, session::order};
fn main() {
    let mut n = 0; let mut list = Vec::new();
    for id in order() { let Some(d) = def(id) else { continue }; if d.block.is_some() || d.tool.is_some() || d.wear.is_some() { continue; }
        let fallback = (0..=20).all(|i| (0..=20).all(|j| { let (x, y) = (i as f64 / 20.0, j as f64 / 20.0); let a = alw_core::icons::look(id, x, y); let inside = (x - 0.5).abs() < 0.22 && (y - 0.5).abs() < 0.22; match a { Some((c, _)) => inside && c == [0.6, 0.55, 0.45], None => !inside } }));
        if fallback { n += 1; list.push(*id); } }
    println!("{n} objets sans dessin : {}", list.join(" "));
}
