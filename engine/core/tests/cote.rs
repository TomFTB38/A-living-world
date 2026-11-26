//! Les villages côtiers : maisons de pêcheurs en granite et ardoise, hangars, auberge du port, capitainerie à tourelle ;
//! le village tourné vers la mer, le port (ponton, voilier, halle aux poissons) gardé, et un phare sur le rivage.
use alw_core::batisse::{Lot, Work};
use alw_core::blocks::{idx, Id, BLOCKS, CX};
use alw_core::bourg::Civ;
use alw_core::cote::{build, plan, CKind};
use alw_core::edifice::{raise, Arch, Cond, Spec};
use alw_core::gen::WorldGen;
use std::collections::HashMap;
const SEED: i32 = 20261104;
const LOT: Lot = Lot { o: (0, 0), u: (1, 0), w: (0, 1) };
fn names(k: &Work) -> HashMap<(i32, i32, i32), &'static str> { let mut m = HashMap::new(); for b in &k.blocks { m.insert((b.0, b.1, b.2), BLOCKS[b.3 as usize].name); } m }
fn count(m: &HashMap<(i32, i32, i32), &'static str>, f: impl Fn(&str) -> bool) -> usize { m.values().filter(|n| f(n)).count() }

#[test]
fn les_batiments_de_la_cote() {
    for (kind, var) in [(CKind::Maison, 0), (CKind::Maison, 2 | 8), (CKind::Maison, 1 | 4), (CKind::Atelier, 0), (CKind::Auberge, 0), (CKind::Capitainerie, 0), (CKind::GrandeCapitainerie, 0)] {
        let mut k = Work::default(); let craft = if kind == CKind::Atelier { Some("forgeron") } else { None }; let bt = build(&mut k, LOT, 50, kind, var, 0.4, 0.4, craft); let m = names(&k); let pl = plan(kind, var);
        let at = |x: i32, y: i32, z: i32| m.get(&(x, y, z)).copied().unwrap_or("air");
        println!("{:<20} {}×{} var {var:>2} : {} blocs, porte {:?}, lit {:?}, poste {:?}", kind.label(), pl.w, pl.d, k.blocks.len(), bt.door, bt.bed, bt.spot);
        assert!(count(&m, |n| n.starts_with("slate")) > 20, "{kind:?} : un toit d'ardoise"); assert!(count(&m, |n| n.starts_with("granite")) > 15, "{kind:?} : du granite");
        assert_eq!(count(&m, |n| n.contains("roof_tiles") || n.contains("thatch") || n.starts_with("sandstone")), 0, "{kind:?} : ni tuiles, ni chaume, ni grès");
        let d = bt.door.unwrap(); assert!(at(d, 51, 0).starts_with("door"), "{kind:?} : la porte ({})", at(d, 51, 0)); assert!(at(d, 51, -1) == "air" && at(d, 52, -1) == "air", "{kind:?} : le seuil dégagé");
        assert!(count(&m, |n| n.starts_with("shutter")) >= 2, "{kind:?} : des volets");
        if kind != CKind::Atelier { assert!(count(&m, |n| n.starts_with("window")) >= 3, "{kind:?} : des fenêtres"); }
        if let Some(b) = bt.bed { assert!(at(b.0, 51, b.1) == "air" || !BLOCKS.iter().find(|x| x.name == at(b.0, 51, b.1)).unwrap().solid, "{kind:?} : place près du lit ({})", at(b.0, 51, b.1)); assert!(count(&m, |n| n.starts_with("bed")) >= 2); }
        match kind {
            CKind::Maison => { assert_eq!(at(0, 57, 2), "granite", "la souche de cheminée au pignon"); assert!(count(&m, |n| n == "creel") >= 2 && count(&m, |n| n.starts_with("anchor")) == 1, "casiers et ancre devant");
                assert_eq!(var & 2 != 0, at(2, 52, 0 + 0) == "plaster" || count(&m, |n| n == "plaster") > 10, "blanchie à la chaux selon la variante");
                if var & 8 != 0 { assert!(count(&m, |n| n.starts_with("net_pile")) == 1 && count(&m, |n| n.starts_with("oars")) == 1, "l'appentis et son matériel"); } }
            CKind::Atelier => { assert_eq!(at(2, 51, 3), "anvil"); assert!(at(2, 51, 0) == "air", "ouvert sur la rue"); }
            CKind::Auberge => { assert!(count(&m, |n| n.starts_with("bed")) >= 6, "les chambres"); assert!(count(&m, |n| n.starts_with("hanging_sign")) == 1, "l'enseigne"); assert_eq!(at(5, 54, 3), "spruce_planks", "un étage"); }
            CKind::Capitainerie | CKind::GrandeCapitainerie => { let x0 = pl.w - 3; assert!(at(x0 + 1, 63, 1) == "lantern", "la lanterne de la tourelle ({})", at(x0 + 1, 63, 1)); assert!((55..62).all(|y| at(x0, y, 0) == "granite" || at(x0, y, 0).starts_with("window")), "la tourelle monte"); }
        }
    }
    // Les fermes, l'église, le moulin : ceux de la campagne.
    for a in [Arch::Farm, Arch::Chapel, Arch::Mill, Arch::Fold] { let sp = Spec::coast(a, 3, 0.5, 0.5, None, Cond::Kept); assert!(!sp.coast, "{a:?} reste médiéval"); }
    // Une maison abandonnée : le toit crevé, l'herbe qui gagne, vidée.
    let sp = Spec::coast(Arch::House, 0, 0.5, 0.4, None, Cond::Abandoned); let mut k = Work::default(); let made = raise(&mut k, LOT, &sp, 50); let m = names(&k);
    assert!(made.door.is_none() && made.bed.is_none()); assert_eq!(count(&m, |n| n.starts_with("bed") || n.starts_with("door")), 0); assert!(count(&m, |n| n == "tall_grass") > 3, "l'herbe gagne");
}

fn block_at(g: &WorldGen, cache: &mut HashMap<(i32, i32), Vec<Id>>, x: i32, y: i32, z: i32) -> &'static str {
    let c = (x.div_euclid(CX as i32), z.div_euclid(CX as i32)); let v = cache.entry(c).or_insert_with(|| g.gen_world(c.0, c.1));
    BLOCKS[v[idx(x.rem_euclid(CX as i32) as usize, y as usize, z.rem_euclid(CX as i32) as usize)] as usize].name
}

#[test]
fn un_port_de_peche() {
    let g = WorldGen::new(SEED);
    let ports: Vec<_> = g.villages_near(0.0, 0.0, 4000.0).into_iter().filter(|v| v.kind == "port").collect(); assert!(!ports.is_empty(), "des ports");
    for v in ports.iter().take(3) {
        let i = v.ident.expect("refondu"); assert_eq!(i.civ, Civ::Coastal);
        println!("{} {:?} : {} ; {} bâtiments ({:?}), {} champs, {} foyers, rayon {}", v.name, v.c, i.describe(), v.works.len(), v.works.iter().map(|w| alw_core::cote::kind_of(w.0).map_or(w.0.label(), |k| k.label())).collect::<Vec<_>>(), v.fields, v.houses.len(), v.r);
        assert!(v.works.iter().filter(|w| alw_core::cote::kind_of(w.0) == Some(CKind::Maison)).count() >= 2 && v.houses.len() >= 2, "des maisons de pêcheurs habitées");
        let mut cache = HashMap::new(); let (x, y, z) = (v.c[0], v.c[1], v.c[2]); let mut seen: HashMap<&str, usize> = HashMap::new(); let r = v.r;
        for dx in -r..=r { for dz in -r..=r { if dx * dx + dz * dz > r * r { continue; } for yy in (y - 12).max(1)..y + 26 { *seen.entry(block_at(&g, &mut cache, x + dx, yy, z + dz)).or_default() += 1; } } }
        let n = |k: &str| seen.get(k).copied().unwrap_or(0); let pre = |p: &str| seen.iter().filter(|(k, _)| k.starts_with(p)).map(|(_, c)| *c).sum::<usize>();
        println!("  ardoise {} ; granite {} ; amarres {} ; voile {} ; phare : rouge {}, lanternes {} ; halle {}", pre("slate"), pre("granite"), n("mooring_post"), pre("sail"), n("wool_red"), n("lantern"), pre("shingles"));
        assert!(pre("slate") > 100 && pre("granite") > 100, "les maisons de pêcheurs");
        assert!(n("mooring_post") >= 3 && pre("sail") >= 6, "le port gardé : ponton et voilier");
        assert!(n("wool_red") >= 8, "le phare cerclé de rouge");
    }
}
