//! Une partie d'essai jouée « à la souris » : on ouvre les ateliers d'un clic droit sur leur bloc, on prend et on dépose
//! les objets comme le fait l'écran (appui : grab_slot ; relâché : drop_slot), on taille, on fond, on referme.
use alw_core::{blocks::*, data::Station, player::{Input, Voxels}, session::*, stream::Preset};
const DT: f64 = 0.05;
fn ready(s: &mut Session) { for _ in 0..6000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn idle(s: &mut Session, sec: f64) { for _ in 0..(sec / DT) as usize { s.tick(DT, Input::default(), Actions::default()); } }
/// Pose un bloc d'atelier devant soi, à hauteur des pieds, la vue dégagée ; le vise.
fn station(s: &mut Session, b: u16) -> [i32; 3] {
    let p = s.player.p; let c = [p[0].floor() as i32 + 2, p[1].floor() as i32, p[2].floor() as i32];
    let mut e = Vec::new(); for dx in 0..=3 { for dy in 0..=2 { for dz in -1..=1 { if (dx, dy, dz) != (2, 0, 0) { e.push((p[0].floor() as i32 + dx, c[1] + dy, c[2] + dz, B::AIR)); } } } } e.push((c[0], c[1], c[2], b)); s.world.edit_many(&e);
    let eye = s.player.eye(); let d = [c[0] as f64 + 0.5 - eye[0], c[1] as f64 + 0.6 - eye[1], c[2] as f64 + 0.5 - eye[2]]; s.player.yaw = d[0].atan2(d[2]); s.player.pitch = (d[1] / d[0].hypot(d[2])).atan(); s.tick(0.0, Input::default(), Actions::default());
    let t = s.target.expect("rien de visé"); assert_eq!([t.x, t.y, t.z], c, "on vise {}", BLOCKS[t.id as usize].name); c }
fn right_click(s: &mut Session) { s.tick(DT, Input::default(), Actions { mine: false, place: true, ..Default::default() }); }
/// Un glisser-déposer : appui sur une case, relâché sur une autre.
fn drag(s: &mut Session, from: usize, to: usize) { s.grab_slot(from); s.drop_slot(Some(to)); }
fn start() -> Session { let mut s = Session::new(20261104, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.third = false; idle(&mut s, 1.0); for i in 0..INV_SIZE { s.inv[i] = None; } s }

#[test]
fn a_l_atelier_de_taille_a_la_souris() {
    let mut s = start(); s.inv[9] = Some(Stack::new("stone", Some(10))); s.inv[10] = Some(Stack::new("raw_meat", Some(2))); s.inv[11] = Some(Stack::new("planks", Some(5)));
    station(&mut s, B::MASON_BENCH); right_click(&mut s);
    assert!(s.inv_open && s.at_station.map(|a| a.0) == Some(Station::Mason) && s.cutting(), "le clic droit ouvre l'atelier, en mode taille");
    // la viande ne se taille pas : glissée dans la case, elle retourne d'où elle vient
    drag(&mut s, 10, CUT0); assert!(s.cut_in.is_none() && s.cursor.is_none()); assert_eq!(s.inv[10].map(|c| (c.item, c.n)), Some(("raw_meat", Some(2))));
    // la pierre, glissée : elle y est ; on taille un escalier, puis toutes les dalles possibles
    drag(&mut s, 9, CUT0); assert_eq!(s.cut_in.map(|c| (c.item, c.n)), Some(("stone", Some(10)))); assert!(s.inv[9].is_none());
    assert!(s.cut("stone_stairs", false).is_ok()); assert_eq!((s.count("stone_stairs"), s.cut_in.and_then(|c| c.n)), (1, Some(9)));
    assert!(s.cut("stone_slab", true).is_ok()); assert_eq!(s.count("stone_slab"), 18); assert!(s.cut_in.is_none(), "le tas y est passé");
    assert!(s.cut("stone_slab", false).is_err(), "case vide : on le dit");
    // une dalle ne se retaille pas ; on pose des planches, puis du calcaire par-dessus : les planches reviennent au sac
    let i = s.inv.iter().position(|t| t.is_some_and(|t| t.item == "stone_slab")).unwrap(); drag(&mut s, i, CUT0); assert!(s.cut_in.is_none() && s.cursor.is_none()); assert_eq!(s.count("stone_slab"), 18);
    drag(&mut s, 11, CUT0); assert_eq!(s.cut_in.map(|c| c.item), Some("planks")); s.inv[20] = Some(Stack::new("limestone", Some(3))); drag(&mut s, 20, CUT0);
    assert_eq!(s.cut_in.map(|c| c.item), Some("limestone"), "on remplace le bloc posé"); assert_eq!(s.count("planks"), 5); assert!(s.cursor.is_none(), "rien ne reste collé à la souris");
    // on passe aux recettes : le bloc posé revient au sac ; rien n'est perdu
    s.cut_mode = false; idle(&mut s, 0.1); assert!(s.cut_in.is_none()); assert_eq!(s.count("limestone"), 3);
    // on reprend un bloc en main et on ferme l'inventaire sans le reposer : il n'est pas perdu non plus
    s.cut_mode = true; let i = s.inv.iter().position(|t| t.is_some_and(|t| t.item == "planks")).unwrap(); drag(&mut s, i, CUT0); assert!(s.cut_in.is_some()); s.grab_slot(CUT0); assert!(s.cursor.is_some()); s.drop_slot(None); s.inv_open = false; idle(&mut s, 0.2);
    assert!(s.at_station.is_none() && s.cut_in.is_none() && s.cursor.is_none()); assert_eq!(s.count("planks"), 5);
}

#[test]
fn au_four_a_la_souris() {
    let mut s = start(); s.inv[9] = Some(Stack::new("iron_ore", Some(4))); s.inv[10] = Some(Stack::new("coal", Some(4))); s.inv[11] = Some(Stack::new("sand", Some(4)));
    station(&mut s, B::FURNACE); right_click(&mut s); assert_eq!(s.cooking(), Some(Station::Furnace), "le clic droit ouvre le four, cases visibles");
    // le charbon refuse la case du matériau, le minerai celle du combustible
    drag(&mut s, 10, COOK0); assert!(s.cook_in.is_none()); drag(&mut s, 9, FUEL0); assert!(s.cook_fuel.is_none()); assert_eq!((s.count("coal"), s.count("iron_ore")), (4, 4));
    drag(&mut s, 9, COOK0); drag(&mut s, 10, FUEL0); assert_eq!(s.cook_block(), None); idle(&mut s, 3.5); assert_eq!(s.count("iron_ingot"), 1);
    // en pleine fonte, on change de matériau : le minerai revient au sac, la flèche repart de zéro, le sable devient verre
    idle(&mut s, 1.0); assert!(s.cook_t > 0.0); drag(&mut s, 11, COOK0); assert_eq!(s.cook_in.map(|c| c.item), Some("sand")); assert_eq!(s.cook_t, 0.0); assert_eq!(s.count("iron_ore"), 2); assert!(s.cursor.is_none());
    idle(&mut s, 3.5); assert_eq!(s.count("glass"), 2); assert_eq!(s.cook_fuel.and_then(|c| c.n), Some(1));
    // on s'éloigne du four sans fermer : tout revient au sac
    s.inv_open = false; idle(&mut s, 0.2); assert_eq!((s.count("coal"), s.count("iron_ore")), (1, 2)); assert!(s.cook_in.is_none() && s.cook_fuel.is_none());
}

/// Chaque espèce hostile lâchée face au joueur, de nuit, dans le vrai monde ; il s'avance d'un pas ou deux : trente secondes sans que rien ne déraille
/// (position finie, pas de chute sous le monde, pas de téléportation), et le relevé de ce qu'elle a fait.
#[test]
fn chaque_monstre_trente_secondes_dans_le_vrai_monde() {
    let ids: Vec<&str> = alw_core::hostile::bestiary().species.iter().map(|s| s.id).collect(); let mut inert = Vec::new();
    for (k, id) in ids.iter().enumerate() {
        let mut s = start(); s.hour = 23.5; let p = s.player.p; let at = [p[0] + 6.0, p[1] + 0.2, p[2] + 2.0]; let i = s.horde.put(k, at, (p[0] - at[0]).atan2(p[2] - at[2]), false); s.player.yaw = (at[0] - p[0]).atan2(at[2] - p[2]); /* la bête regarde le joueur, qui fait quelques pas bruyants vers elle */ let hp0 = s.vit[0]; let (mut moved, mut closest, mut hurt) = (0.0f64, 99.0f64, false);
        for t in 0..600 { s.tick(DT, if t < 30 { Input { z: 1.0, ..Default::default() } } else { Input::default() }, Actions::default()); if s.dead.is_some() { hurt = true; break; } if s.vit[0] < hp0 - 0.5 { hurt = true; }
            let Some(f) = s.horde.foes.get(i) else { break }; let q = s.player.p; assert!(f.p.iter().all(|v| v.is_finite()), "{id} : position folle {:?}", f.p); assert!(f.p[1] > 2.0, "{id} est tombé sous le monde : {:?}", f.p);
            let d = (f.p[0] - q[0]).hypot(f.p[2] - q[2]); assert!(d < 70.0, "{id} s'est envolé à {d:.0} blocs"); closest = closest.min(d); moved = moved.max((f.p[0] - at[0]).hypot(f.p[2] - at[2])); }
        println!("{id:>10} : déplacé de {moved:.1}, au plus près à {closest:.1}, joueur blessé : {hurt}"); if moved < 0.5 && !hurt { inert.push(*id); }
    }
    // le sylvain guette immobile et le noyé attend dans l'eau : eux seuls ont le droit de ne rien faire ici
    inert.retain(|id| !matches!(*id, "sylvain" | "noye" | "ver")); assert!(inert.is_empty(), "restés inertes : {inert:?}");
}
