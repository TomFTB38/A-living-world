//! Les lieux de surface : tour de guet en ruine, cimetière abandonné, camp de brigands.
use alw_core::{blocks::*, gen::WorldGen, player::Input, session::*, sites::*, stream::Preset};
const SEED: i32 = 20261104;
fn w_ground(w: &WorldGen, x: i32, z: i32) -> i32 { w.column(x as f64, z as f64).h.floor() as i32 }
fn ready(s: &mut Session) { for _ in 0..8000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }

#[test]
fn chaque_lieu_existe_est_bati_et_garde_son_coffre() {
    let w = WorldGen::new(SEED); let all = w.places_near(-38.0, 0.0, 2800.0); let mut seen = [0usize; 9];
    let name = |b: u16| BLOCKS[b as usize].name;
    for pl in &all {
        let k = LIEUX.iter().position(|l| *l == pl.kind).unwrap(); seen[k] += 1; if seen[k] > 3 { continue; } // trois de chaque sorte, bloc à bloc
        assert!(((pl.x + 38) as f64).hypot(pl.z as f64) >= 90.0, "trop près du départ"); assert!(w.villages_near(pl.x as f64, pl.z as f64, 100.0).is_empty(), "{:?} dans un village", pl.kind);
        let cache: std::cell::RefCell<std::collections::HashMap<(i32, i32), Vec<u16>>> = Default::default();
        let at = |x: i32, y: i32, z: i32| { let mut c = cache.borrow_mut(); let v = c.entry((x.div_euclid(32), z.div_euclid(32))).or_insert_with(|| w.gen_chunk(x.div_euclid(32), z.div_euclid(32))); v[idx(x.rem_euclid(32) as usize, y as usize, z.rem_euclid(32) as usize)] };
        let c = pl.chest(); assert!(is_chest_shut(at(c[0], c[1], c[2])), "{:?} en {} {} {} : le coffre est à sa place", pl.kind, pl.x, pl.y, pl.z); assert!(is_solid(at(c[0], c[1] - 1, c[2])), "coffre en l'air");
        assert_eq!(w.place_chest(c), Some(pl.kind)); assert_eq!(w.place_chest([c[0], c[1] + 1, c[2]]), None);
        let count = |f: &dyn Fn(&str) -> bool, r: i32, h: i32| { let mut n = 0; for x in pl.x - r..=pl.x + r { for z in pl.z - r..=pl.z + r { for y in pl.y..=pl.y + h { if f(name(at(x, y, z))) { n += 1; } } } } n };
        let lo = |u: i32, w: i32| { let (a, b) = spin_xz(u, w, pl.rot); (pl.x + a, pl.z + b) };
        match pl.kind {
            Lieu::Tour => {
                assert!(count(&|n| n.contains("_stairs"), 4, 13) >= 12, "les trois volées d'escalier"); assert!(count(&|n| n == "stone_bricks" || n == "cobblestone" || n == "mossy_stone", 4, 14) > 400, "les murs tiennent"); assert_eq!(count(&|n| n.contains("leaves"), 3, 14), 0, "pas d'arbre dans la tour");
                assert_eq!(count(&|n| n == "brazier", 1, 15), 1, "le brasero"); assert!(count(&|n| n == "fence", 5, 14) >= 5, "le hourd");
                // chaque marche a de quoi passer la tête ; la cave est creusée, son échelle descend, son coffre caché y est
                let y = pl.y; for i in 0..4 { for (su, sy, sw) in [(-3, y + 1 + i, -2 + i), (-2 + i, y + 5 + i, 3), (3, y + 9 + i, 2 - i)] { let (sx, sz) = lo(su, sw); assert!(name(at(sx, sy, sz)).contains("_stairs"), "marche manquante en {sx} {sy} {sz}"); for h in 1..=2 { assert!(!is_solid(at(sx, sy + h, sz)), "on se cogne au-dessus de la marche {sx} {sy} {sz}"); } } }
                for dy in 1..=4 { assert!(name({ let (lx, lz) = lo(3, 3); at(lx, y - dy, lz) }).starts_with("ladder"), "échelle de la cave"); assert_eq!({ let (lx, lz) = lo(0, 0); at(lx, y - dy, lz) }, 0, "la cave est creusée"); } assert!(is_chest_shut({ let (lx, lz) = lo(-3, 3); at(lx, y - 4, lz) }), "le coffre caché"); assert!(count(&|n| n.starts_with("iron_bars"), 4, 0) == 0 && name({ let (lx, lz) = lo(-1, -3); at(lx, y - 4, lz) }).starts_with("iron_bars"), "le cachot");
            }
            Lieu::Cimetiere => {
                assert!(count(&|n| n.starts_with("gravestone") || n.starts_with("grave_cross"), 11, 2) >= 14, "des tombes"); assert!(count(&|n| n == "gravel", 9, 0) >= 12, "les allées"); assert!(count(&|n| n.starts_with("slate_slope"), 11, 8) >= 30, "le toit de la chapelle et du porche");
                assert!(count(&|n| n == "stained_glass", 11, 8) >= 10, "les vitraux"); assert!(count(&|n| n.starts_with("bench"), 8, 2) >= 5, "les bancs");
                let y = pl.y; for dy in 1..=4 { assert!(name({ let (lx, lz) = lo(-2, 9); at(lx, y - dy, lz) }).starts_with("ladder"), "échelle de la crypte"); assert_eq!({ let (lx, lz) = lo(0, 4); at(lx, y - dy, lz) }, 0, "la crypte est creusée"); } for h in 1..=2 { assert!(!is_solid({ let (lx, lz) = lo(0, 2); at(lx, y + h, lz) }), "la porte de la chapelle est ouverte"); }
            }
            Lieu::Sanctuaire => {
                // l'escalier descend sous la ruine, les salles sont creusées, les grilles fermées, les leviers en place ; la voûte reste sous terre
                use alw_core::sanctuaire::{self as sa, DEPTH}; let f = pl.y - DEPTH; let rel = |u: i32, v: i32, w: i32| at(pl.x + u, f + v, pl.z + w);
                for u in [-2, 2] { assert_eq!(name(rel(u, DEPTH + 1, -14)), "column_base", "le portail en surface"); assert_eq!(name(rel(u, DEPTH + 5, -14)), "column_capital"); } assert!(name(rel(0, DEPTH + 6, -14)).starts_with("lintel"), "son linteau"); assert!(name(rel(-4, DEPTH + 2, -14)).starts_with("statue_top"), "ses gardiens"); for k in 0..DEPTH - 1 { assert!(name(rel(0, DEPTH - 1 - k, -(DEPTH + 1) + k)).starts_with("stone_bricks_stairs"), "marche {k}"); }
                for l in sa::R1_LEVERS { assert!(name(rel(l[0], l[1], l[2])).starts_with("lever"), "levier de la salle 1"); } assert!(name(rel(-18, 1, 13)).starts_with("iron_bars"), "la grille de la salle 1");
                for q in sa::R2_PLATES { assert_eq!(name(rel(q[0], q[1], q[2])), "pressure_plate"); } assert_eq!(name(rel(sa::R4_LEVER[0], sa::R4_LEVER[1], sa::R4_LEVER[2])).split('@').next(), Some("lever"));
                for (u, w) in [(0, 4), (-15, 7), (-17, 21), (0, 21), (17, 24), (17, 7), (17, -4)] { assert_eq!(at(pl.x + u, f + 2, pl.z + w), 0, "salle creusée en {u} {w}"); }
                for (u, ww) in [(-20, 20), (0, 25), (20, 20), (17, -8)] { let g = w_ground(&w, pl.x + u, pl.z + ww); assert!(g >= f + sa::R4.4 + 2, "la voûte reste sous terre en {u} {ww} : sol {g}, voûte {}", f + sa::R4.4 + 1); }
            }
            Lieu::Soleil => {
                // la pyramide à degrés, le portail du nord, la salle et son disque d'or, les miroirs, les sceaux, l'oculus
                use alw_core::soleil as so; let rel = |u: i32, v: i32, w: i32| at(pl.x + u, pl.y + v, pl.z + w);
                assert_eq!(name(rel(-3, 1, -18)), "column_base", "le portail"); assert!(name(rel(0, 1, 0)).starts_with("sun_disc"), "le disque d'or");
                for (q, _) in so::MIRRORS { assert!(name(rel(q[0], q[1], q[2])).starts_with("sun_mirror")); } for (q, _) in so::SEALS { assert!(name(rel(q[0], q[1], q[2])).starts_with("sun_receptor")); }
                assert_eq!(rel(0, 4, -6), 0, "la salle"); assert_eq!(rel(0, 12, 0), 0, "l'oculus"); assert!(is_solid(rel(12, 6, 0)), "le deuxième degré"); assert!(name(rel(0, 8, 15)).starts_with("sandstone_stairs"), "le grand escalier");
            }
            Lieu::Forge => {
                // l'atelier et sa cheminée, l'escalier, la grande forge et son fleuve de lave, le pont au fond, la porte du foyer
                use alw_core::forge::{self as fo, DEPTH}; let f = pl.y - DEPTH; let rel = |u: i32, v: i32, w: i32| at(pl.x + u, f + v, pl.z + w);
                assert_eq!(name(rel(3, DEPTH + 14, -17)), "campfire", "la fumée de la cheminée"); for k in 0..DEPTH { assert!(name(rel(0, DEPTH - 1 - k, -19 + k)).starts_with("stone_bricks_stairs"), "marche {k}"); }
                assert_eq!(rel(0, 5, 5), 0, "la grande forge"); assert_eq!(name(rel(5, 0, 11)), "lava", "le fleuve de lave"); assert_eq!(name(rel(0, fo::BRIDGE_LOW, 11)), "basalt", "le pont au fond");
                assert_eq!(name(rel(0, 2, fo::CHAINS_W)), "great_chain"); assert_eq!(name(rel(-12, -3, 21)), "lava", "la coulée");
                for (u, ww) in [(-14, 0), (14, 26), (0, 14), (0, 34)] { let g = w_ground(&w, pl.x + u, pl.z + ww); assert!(g >= f + 14, "la voûte reste sous terre en {u} {ww} : sol {g}"); }
            }
            Lieu::Catacombes => {
                // le mausolée et son portique, l'escalier, le vestibule et sa porte des morts, la crypte et ses rois ; la voûte reste sous terre
                use alw_core::catacombes::{self as ca, DEPTH}; let f = pl.y - DEPTH; let rel = |u: i32, v: i32, w: i32| at(pl.x + u, f + v, pl.z + w);
                assert_eq!(name(rel(-2, DEPTH + 1, -8)), "column_base", "le portique"); assert!(name(rel(0, DEPTH + 11, 0)).starts_with("statue_top"), "la statue du toit"); assert!(name(rel(4, DEPTH + 3, -24)).starts_with("statue_top"), "un roi de l'allée");
                for k in 0..DEPTH { assert!(name(rel(0, DEPTH - 1 - k, -3 + k)).starts_with("stone_bricks_stairs"), "marche {k}"); }
                assert_eq!(rel(0, 3, 15), 0, "le vestibule"); assert_eq!(name(rel(0, 2, ca::GATE_W)), "great_chain", "la porte des morts"); assert_eq!(rel(0, 5, -27), 0, "la crypte");
                for k in 0..4 { assert!(name(rel(ca::lever(k)[0], 1, -29)).starts_with("lever"), "le levier du roi {k}"); }
                for (u, ww) in [(-20, 0), (20, 0), (0, -28), (0, 32)] { let g = w_ground(&w, pl.x + u, pl.z + ww); assert!(g >= f + 10, "la voûte reste sous terre en {u} {ww} : sol {g}"); }
            }
            Lieu::Grotte => {
                // l'esplanade et sa statue, l'escalier du puits, le temple fermé sur son île ; la voûte reste sous terre
                use alw_core::grotte::{self as gr, DEPTH}; let f = pl.y - DEPTH; let rel = |u: i32, v: i32, w: i32| at(pl.x + u, f + v, pl.z + w);
                assert!(name(rel(0, DEPTH + 4, 10)).starts_with("statue_top"), "la statue du roi"); assert_eq!(name(rel(0, DEPTH + 1, -10)), "column_base", "la colonnade");
                for (i, (u, w, top, ..)) in gr::spiral().iter().enumerate() { assert!(name(rel(*u, *top, *w)).starts_with("stone_bricks"), "marche {i}"); }
                assert!(name(rel(0, 3, gr::DOOR_W)).starts_with("iron_bars"), "la grille du temple"); assert_eq!(name(rel(0, 3, gr::CHAINS_W)), "great_chain");
                assert_eq!(rel(0, 5, 18), 0, "la cella"); assert_eq!(name(rel(0, -1, 9)), "water", "le lac");
                for (u, ww) in [(-20, 12), (0, 12), (20, 12), (0, 30)] { let g = w_ground(&w, pl.x + u, pl.z + ww); assert!(g >= f + 24, "la voûte reste sous terre en {u} {ww} : sol {g}"); }
            }
            Lieu::Ruines => {
                // le puits descend jusqu'à la salle d'entrée ; chaque salle est creusée ; le trésor est au fond ; la voûte reste sous terre
                use alw_core::sanctuaire::DEPTH; let f = pl.y - DEPTH; let l = pl.ruins(); let rel = |q: [i32; 3]| at(pl.x + q[0], f + q[1], pl.z + q[2]);
                for z in -5..=5 { let st = rel(l.rooms[0].at([-5, 6 + z, z])); assert!(name(st).starts_with("stone_bricks_stairs"), "la marche {z} du grand escalier : {}", name(st)); }
                let e = l.rooms[0]; let zt = (6..=13).find(|z| name(rel(e.at([-6, 6 + z + 1, z + 1]))) == "column_base").expect("les colonnes du portail");
                assert!(name(rel(e.at([-5, 6 + zt + 6, zt + 1]))).starts_with("lintel"), "le linteau du portail"); assert!(name(rel(e.at([-8, 6 + zt + 2, zt + 1]))).starts_with("statue_top"), "un gardien du portail en {} {} {} : {}", pl.x, pl.y, pl.z, name(rel(e.at([-8, 6 + zt + 2, zt + 1]))));
                for r in &l.rooms { assert_eq!(rel(r.at([0, 3, -3])), 0, "{:?} creusée", r.kind); let c = r.c; let g = w_ground(&w, pl.x + c.0, pl.z + c.1); assert!(g >= f + r.kind.h() + 2, "la voûte de {:?} reste sous terre", r.kind); }
            }
            Lieu::Camp => {
                assert_eq!(count(&|n| n == "campfire", 1, 2), 1, "le feu"); assert!(count(&|n| n == "oak_log", 11, 5) > 150, "la palissade"); assert!(count(&|n| n.starts_with("thatch_slope"), 9, 4) >= 60, "les tentes"); assert_eq!(count(&|n| n == "bedroll", 9, 2), 4);
                assert!(count(&|n| n.starts_with("iron_bars"), 8, 3) >= 14, "la cage"); assert!(count(&|n| n.starts_with("ladder"), 8, 6) >= 5, "l'échelle de la tour de guet"); assert!(count(&|n| n.starts_with("throne"), 8, 2) == 1 && count(&|n| n.starts_with("cart"), 6, 2) == 1, "le siège du chef, la charrette");
                let y = pl.y; for h in 1..=2 { for u in -1..=1 { assert!(!is_solid({ let (lx, lz) = lo(u, -10); at(lx, y + h, lz) }), "le portail est ouvert"); } assert!(!is_solid({ let (lx, lz) = lo(0, 5); at(lx, y + h, lz) }), "on entre dans la tente du chef"); }
            }
        }
        // les gardiens ont les pieds sur du plein et la tête à l'air
        for (id, g) in pl.guards() { assert!(alw_core::hostile::species(id).is_some(), "{id}"); let (x, y, z) = (g[0].floor() as i32, g[1].floor() as i32, g[2].floor() as i32); assert!(is_solid(at(x, y - 1, z)), "{id} de {:?} n'a rien sous les pieds en {x} {y} {z}", pl.kind); assert!(!is_solid(at(x, y + 1, z)), "{id} de {:?} emmuré", pl.kind); }
    }
    println!("lieux : {} tours, {} cimetières, {} camps, {} sanctuaires, {} ruines, {} grottes, {} temples du soleil, {} catacombes, {} forges", seen[0], seen[1], seen[2], seen[3], seen[4], seen[5], seen[6], seen[7], seen[8]); assert!(seen.iter().all(|n| *n >= 6), "chaque sorte de lieu existe : {seen:?}"); assert!(seen[0] * 10 < seen.iter().sum::<usize>() * 6, "les tours ne prennent pas toute la place");
    assert!(WorldGen::new(alw_core::gen::FLAT).places_near(0.0, 0.0, 800.0).is_empty(), "rien dans le monde plat");
}

#[test]
fn on_y_va_les_gardiens_sortent_le_coffre_est_garni() {
    let w = WorldGen::new(SEED); let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [-38.5, 51.4, 0.5]); ready(&mut s); s.hour = 12.0; let sp = |id: &str| alw_core::hostile::species(id).unwrap();
    assert!(s.command("/lieu chateau").contains("inconnu"));
    for (key, who, n, loot) in [("tour", "archer", 4, "arrow"), ("brigands", "brigand", 6, "worn_coin"), ("sanctuaire", "noye", 3, "idole_engloutie"), ("ruines", "rodeur", 2, "worn_coin"), ("grotte", "noye", 3, "worn_coin"), ("soleil", "rodeur", 2, "worn_coin"), ("catacombes", "archer", 2, "worn_coin"), ("forge", "brigand", 2, "worn_coin"), ("cimetiere", "archer", 3, "worn_coin")] {
        s.horde.foes.clear(); let msg = s.command(&format!("/lieu {key}")); ready(&mut s); let pl = w.places_near(s.player.p[0], s.player.p[2], 40.0).into_iter().next().unwrap_or_else(|| panic!("{key} : {msg}")); assert_eq!(pl.kind.key(), key, "{msg}");
        s.notes.clear(); for _ in 0..3 { s.tick(0.05, Input::default(), Actions::default()); }
        assert!(s.horde.foes.iter().filter(|f| f.sp == sp(who)).count() >= n, "{key} : {n} {who} attendus, on a {:?}", s.horde.foes.iter().map(|f| (alw_core::hostile::bestiary().species[f.sp].id, f.p.map(|v| v.round()), f.hp)).collect::<Vec<_>>()); assert!(s.notes.iter().any(|t| t.contains("en vue")), "{key} : annoncé");
        // le chef du lieu est un boss : le capitaine au sommet de la tour, le gardien au fond de la crypte
        if let Some(boss) = match key { "tour" => Some("capitaine"), "cimetiere" => Some("gardien"), _ => None } { assert_eq!(s.horde.foes.iter().filter(|f| f.sp == sp(boss)).count(), 1, "{key} : son {boss}"); }
        assert_eq!(s.horde.foes.iter().filter(|f| f.sp == sp("fantome")).count(), 0, "pas de fantôme en plein jour");
        let before = s.horde.foes.len(); for _ in 0..40 { s.tick(0.05, Input::default(), Actions::default()); } assert!(s.horde.foes.len() <= before + 2, "{key} : on ne relâche pas des gardiens à chaque image");
        s.open_chest(pl.chest()); let got = &s.chests[&pl.chest()]; assert!(got.iter().flatten().any(|st| st.item == loot), "{key} : {loot} dans le coffre, on a {:?}", got.iter().flatten().map(|st| st.item).collect::<Vec<_>>()); s.close_chest(); s.inv_open = false;
        if key == "ruines" { assert!(s.mech.wires.iter().any(|w| w.name.starts_with(&format!("d{}:{}:", pl.x, pl.z))), "les fils des ruines"); assert_eq!(s.mech.guards.len(), 2, "le colosse du sanctuaire et le gardien des ruines"); }
        if key == "soleil" { assert!(s.mech.beams.len() == 1 && s.mech.wires.iter().any(|w| w.name.starts_with(&format!("t{}:{}:", pl.x, pl.z))), "le rayon et les fils du temple"); }
        if key == "forge" { assert!(s.mech.lifts.len() == 2 && s.mech.wires.iter().any(|w| w.name.starts_with(&format!("f{}:{}:", pl.x, pl.z))), "le pont, le monte-charge et les fils de la forge"); }
        if key == "catacombes" { assert!(s.mech.wires.iter().any(|w| w.name.starts_with(&format!("c{}:{}:", pl.x, pl.z))), "les fils des catacombes"); }
        if key == "grotte" { assert!(s.mech.wires.iter().any(|w| w.name.starts_with(&format!("g{}:{}:", pl.x, pl.z))), "les fils de la grotte"); assert_eq!(s.mech.pools.len(), 1, "la citerne"); }
        if key == "sanctuaire" {
            // ses mécanismes se sont branchés à l'approche, et son plan, posé par la génération du monde, tient : de la salle
            // de l'eau noire on va aux trois leviers, pas au-delà ; résolus, la grille se lève
            use alw_core::sanctuaire::{self as sa, DEPTH}; let f = pl.y - DEPTH; let rel = |q: [i32; 3]| [pl.x + q[0], f + q[1], pl.z + q[2]];
            assert_eq!(s.mech.wires.iter().filter(|w| w.name.starts_with(&format!("s{}:{}:", pl.x, pl.z))).count(), 7, "les fils du sanctuaire");
            let into = |r: &std::collections::BTreeSet<[i32; 3]>, room: (i32, i32, i32, i32, i32)| r.iter().any(|c| { let (u, v, w) = (c[0] - pl.x, c[1] - f, c[2] - pl.z); u >= room.0 && u <= room.1 && w >= room.2 && w <= room.3 && v >= 1 && v <= room.4 });
            let look = |s: &Session| sa::reach(&s.world, rel([0, 1, 4]), rel([-26, -3, -18]), rel([26, DEPTH + 4, 30]));
            let r = look(&s); assert!(into(&r, sa::R1) && !into(&r, sa::R2), "salle 1 ouverte, salle 2 fermée");
            for l in sa::R1_LEVERS { assert!(s.lever_at(rel(l))); for _ in 0..4 { s.tick(0.05, Input::default(), Actions::default()); } } assert!(into(&look(&s), sa::R2), "la grille de la salle 1 se lève");
        }
        s.vit = [100.0; 3]; assert!(s.dead.is_none(), "{key} : mort en arrivant ({:?}, {:?}, {:?})", s.dead, s.player.p, s.notes);
    }
    // la nuit, au cimetière, les fantômes se lèvent
    s.hour = 0.5; for _ in 0..3 { s.tick(0.05, Input::default(), Actions::default()); } assert_eq!(s.horde.foes.iter().filter(|f| f.sp == sp("fantome")).count(), 5, "cinq fantômes la nuit");
}
