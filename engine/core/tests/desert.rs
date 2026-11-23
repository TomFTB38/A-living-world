//! Les villages du désert : bâtis à leur manière (pisé, toits plats, coupoles, caravansérail), autour d'une oasis, avec
//! un souk et une palmeraie ; ni église ni moulin. Les bêtes autour : des dromadaires.
use alw_core::batisse::{Lot, Work};
use alw_core::blocks::{idx, Id, BLOCKS, CX};
use alw_core::bourg::Civ;
use alw_core::desert::{build, plan, DKind};
use alw_core::edifice::{raise, Arch, Cond, Spec};
use alw_core::gen::WorldGen;
use alw_core::{player::Input, session::{Actions, Session}, stream::Preset};
use std::collections::HashMap;
const SEED: i32 = 20261104;

fn names(k: &Work) -> HashMap<(i32, i32, i32), &'static str> { let mut m = HashMap::new(); for b in &k.blocks { m.insert((b.0, b.1, b.2), BLOCKS[b.3 as usize].name); } m }
fn count(m: &HashMap<(i32, i32, i32), &'static str>, f: impl Fn(&str) -> bool) -> usize { m.values().filter(|n| f(n)).count() }
const LOT: Lot = Lot { o: (0, 0), u: (1, 0), w: (0, 1) };

#[test]
fn les_batiments_du_desert() {
    for (kind, var, rich) in [(DKind::Maison, 0, 0.3), (DKind::Maison, 3, 0.8), (DKind::Atelier, 0, 0.4), (DKind::Caravanserail, 0, 0.5), (DKind::Conseil, 0, 0.5), (DKind::GrandConseil, 0, 0.6), (DKind::Ferme, 0, 0.4), (DKind::Enclos, 0, 0.4)] {
        let mut k = Work::default(); let craft = if kind == DKind::Atelier { Some("forgeron") } else { None }; let bt = build(&mut k, LOT, 50, kind, var, 0.37, rich, craft); let m = names(&k); let pl = plan(kind, var);
        println!("{:<26} {}×{} : {} blocs, porte {:?}, lit {:?}, poste {:?}", kind.label(), pl.w, pl.d, k.blocks.len(), bt.door, bt.bed, bt.spot);
        assert!(count(&m, |n| n.contains("roof_tiles") || n.contains("slate") || n.contains("plaster") && n.contains("frame")) == 0, "{kind:?} : pas de toit de tuiles ni d'ardoise");
        assert!(count(&m, |n| n.starts_with("sandstone")) > 10, "{kind:?} : du grès");
        assert!(bt.door.is_some_and(|d| d >= 0 && d < pl.w), "{kind:?} : une porte sur la façade");
        // la porte s'ouvre : rien devant elle, du vide dans le mur (ou un battant)
        let d = bt.door.unwrap(); let at = |x, y, z| m.get(&(x, y, z)).copied().unwrap_or("air");
        assert!(at(d, 51, 0) == "air" || at(d, 51, 0).starts_with("door"), "{kind:?} : on entre ({})", at(d, 51, 0));
        assert!(at(d, 51, -1) == "air" && at(d, 52, -1) == "air", "{kind:?} : le seuil est dégagé");
        match kind {
            DKind::Maison | DKind::Atelier | DKind::Conseil | DKind::GrandConseil => {
                // un toit plat : la rangée du toit pleine au-dessus de tout le corps, un muret autour
                let roof = if matches!(kind, DKind::Conseil | DKind::GrandConseil) { 55 } else { 54 };
                let solid = (0..pl.w).flat_map(|x| (0..pl.d).map(move |z| (x, z))).filter(|&(x, z)| { let n = at(x, roof, z); n.starts_with("sandstone") || n.starts_with("plaster") || n.starts_with("terracotta") || n.starts_with("trapdoor") }).count();
                assert!(solid as i32 >= pl.w * pl.d - 2, "{kind:?} : le toit plat couvre tout ({solid}/{})", pl.w * pl.d);
                assert!(count(&m, |n| n.starts_with("window_arched")) >= 2, "{kind:?} : des fenêtres en arc");
                assert!(count(&m, |n| n.starts_with("moroccan_lantern")) >= 1, "{kind:?} : une lanterne ajourée");
            }
            _ => {}
        }
        if matches!(kind, DKind::Maison | DKind::Atelier | DKind::Caravanserail | DKind::Ferme) { let b = bt.bed.expect("un lit"); assert!(count(&m, |n| n.starts_with("bed")) >= 2, "{kind:?} : un lit"); assert!(at(b.0, 51, b.1) == "air" || !BLOCKS.iter().find(|x| x.name == at(b.0, 51, b.1)).unwrap().solid, "{kind:?} : on peut se coucher près du lit"); }
        if kind == DKind::Maison && var & 2 != 0 { assert!(count(&m, |n| n.starts_with("plaster_stairs")) >= 4, "la chambre haute d'une maison aisée a sa coupole"); }
        if kind == DKind::Maison && var & 2 == 0 { assert!(count(&m, |n| n == "thatch") >= 9 && count(&m, |n| n == "fence") >= 8, "la terrasse a sa treille"); }
        if kind == DKind::Atelier { assert!(count(&m, |n| n.starts_with("awning")) >= 5, "la rangée d'auvents"); assert_eq!(at(2, 51, 3), "anvil", "l'enclume du forgeron"); let (sp, st) = bt.spot.unwrap(); assert_eq!((sp, st), ((2, 2), (2, 3))); }
        if kind == DKind::Caravanserail {
            for (x, z) in [(5, 5), (7, 7), (6, 4)] { assert!((52..60).all(|y| { let n = at(x, y, z); n == "air" || n.starts_with("palm") }), "la cour est à ciel ouvert ({x}, {z})"); }
            assert_eq!(at(6, 50, 6), "water", "le bassin au milieu de la cour"); assert!(count(&m, |n| n == "palm_log") >= 3, "le palmier de la cour");
            for x in 5..=7 { for y in 51..=52 { assert_eq!(at(x, y, 0), "air", "le portail de trois blocs"); } }
            assert!(count(&m, |n| n.starts_with("bed")) >= 6, "des chambres pour les voyageurs");
        }
        if kind == DKind::GrandConseil { assert!((60..64).all(|y| at(9, y, 0) == "plaster" || at(9, y, 0).starts_with("window")), "la tour de guet monte"); assert!(count(&m, |n| n.starts_with("plaster_stairs")) >= 8, "la coupole"); }
        if matches!(kind, DKind::Ferme | DKind::Enclos) { assert!(count(&m, |n| n.starts_with("trough")) >= 2 && count(&m, |n| n == "hay_bale") >= 3, "abreuvoirs et foin"); }
    }
    // Une maison abandonnée : ensablée, le toit effondré par endroits, sans porte ni meubles.
    let sp = Spec::desert(Arch::House, 0, 0.5, 0.4, None, Cond::Abandoned); let mut k = Work::default(); let made = raise(&mut k, LOT, &sp, 50); let m = names(&k);
    assert!(made.door.is_none() && made.bed.is_none(), "personne n'y vit"); assert!(count(&m, |n| n == "sand") > 5, "le sable entre"); assert_eq!(count(&m, |n| n.starts_with("bed") || n.starts_with("door")), 0, "vidée");
    let roof = (0..6).flat_map(|x| (0..6).map(move |z| (x, z))).filter(|&(x, z)| m.get(&(x, 54, z)).is_some_and(|n| n.starts_with("sandstone"))).count(); assert!(roof < 30, "le toit s'est effondré par endroits : {roof}/36");
}

fn block_at(g: &WorldGen, cache: &mut HashMap<(i32, i32), Vec<Id>>, x: i32, y: i32, z: i32) -> &'static str {
    let c = (x.div_euclid(CX as i32), z.div_euclid(CX as i32)); let v = cache.entry(c).or_insert_with(|| g.gen_world(c.0, c.1));
    BLOCKS[v[idx(x.rem_euclid(CX as i32) as usize, y as usize, z.rem_euclid(CX as i32) as usize)] as usize].name
}

#[test]
fn un_village_du_desert() {
    let g = WorldGen::new(SEED);
    let all: Vec<_> = g.villages_near(0.0, 0.0, 6000.0).into_iter().filter(|v| v.kind == "desert").collect(); assert!(!all.is_empty(), "des villages au désert");
    for v in &all {
        let i = v.ident.expect("un village refondu"); assert_eq!(i.civ, Civ::Desert);
        println!("{} {:?} : {} ; {} bâtiments ({:?}), {} jardins, {} foyers, rayon {}", v.name, v.c, i.describe(), v.works.len(), v.works.iter().map(|w| alw_core::desert::kind_of(w.0).label()).collect::<Vec<_>>(), v.fields, v.houses.len(), v.r);
        assert!(v.works.iter().all(|w| !matches!(w.0, Arch::Chapel | Arch::Mill)), "ni église ni moulin");
        assert!(v.works.iter().filter(|w| w.0 == Arch::House).count() >= 2 && !v.houses.is_empty(), "des maisons habitées");
        assert!(["Bir", "Aïn", "Ksar", "Oued", "Tin", "Dar"].iter().any(|p| v.name.starts_with(p)), "un nom du désert : {}", v.name);
        // dans le monde : l'oasis au milieu, des palmiers, des toits plats, pas une tuile
        let mut cache = HashMap::new(); let (x, y, z) = (v.c[0], v.c[1], v.c[2]);
        assert_eq!(block_at(&g, &mut cache, x, y, z), "water", "l'eau de l'oasis au milieu de la place"); assert_eq!(block_at(&g, &mut cache, x, y + 1, z), "air", "à ciel ouvert");
        let mut seen: HashMap<&str, usize> = HashMap::new(); let r = v.r;
        for dx in -r..=r { for dz in -r..=r { if dx * dx + dz * dz > r * r { continue; } for yy in y - 3..y + 16 { *seen.entry(block_at(&g, &mut cache, x + dx, yy, z + dz)).or_default() += 1; } } }
        let n = |k: &str| seen.get(k).copied().unwrap_or(0); let pre = |p: &str| seen.iter().filter(|(k, _)| k.starts_with(p)).map(|(_, c)| *c).sum::<usize>();
        println!("  palmiers {} ; grès {} ; fenêtres en arc {} ; lanternes ajourées {} ; tuiles {} ; ardoise {} ; pavés {} ; cultures {}", n("palm_log"), pre("sandstone"), pre("window_arched"), pre("moroccan_lantern"), pre("roof_tiles"), pre("slate"), n("cobblestone"), n("farmland"));
        assert!(n("palm_log") > 20, "des palmiers : à l'oasis et dans la palmeraie"); assert!(pre("window_arched") > 4 && pre("moroccan_lantern") > 2);
        assert_eq!(pre("roof_tiles") + pre("slate") + pre("mossy_roof_tiles"), 0, "pas un toit de tuiles ou d'ardoise"); assert_eq!(n("timber") + n("oak_log_c"), 0, "pas de colombages");
        if v.fields > 0 { assert!(n("farmland") > 10, "la palmeraie a ses jardins"); }
    }
}

#[test]
fn des_dromadaires_autour_du_village() {
    let g = WorldGen::new(SEED); let v = g.villages_near(0.0, 0.0, 6000.0).into_iter().find(|v| v.kind == "desert").unwrap();
    let mut s = Session::new(SEED, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4, [v.c[0] as f64 + 0.5, v.c[1] as f64 + 12.0, v.c[2] as f64 + 0.5]); s.player.fly = true; s.set_creative(true);
    for _ in 0..12000 { let p = s.player.p; s.world.update(p[0], p[2]); if s.world.waiting() == 0 && s.world.loaded() > 0 { break; } std::thread::sleep(std::time::Duration::from_millis(3)); }
    for _ in 0..900 { s.tick(0.05, Input::default(), Actions::default()); }
    use alw_core::wild::Kind; let n = |k: Kind| s.wild.all.iter().filter(|c| c.kind == k).count();
    println!("autour de {} : {} dromadaires, {} poules, {} vaches, {} cochons ; {} villageois", v.name, n(Kind::Camel), n(Kind::Chicken), n(Kind::Cow), n(Kind::Pig), s.town.folk.iter().filter(|f| f.v == v.id).count());
    assert!(n(Kind::Camel) >= 2, "des dromadaires"); assert_eq!(n(Kind::Cow) + n(Kind::Pig), 0, "ni vaches ni cochons au désert");
    assert!(s.town.folk.iter().filter(|f| f.v == v.id).count() >= 4, "le village est habité");
}
