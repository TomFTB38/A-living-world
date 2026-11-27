//! Les villages vikings de la montagne et de la taïga : maisons longues de rondins sous un toit de gazon aux têtes de
//! dragon, ateliers, halle à hydromel, halle du jarl, étables ; au milieu, le cercle de pierres et la pierre runique ; une
//! palissade de pieux autour ; ni église ni moulin.
use alw_core::batisse::{Lot, Work};
use alw_core::blocks::{idx, Id, BLOCKS, CX};
use alw_core::bourg::Civ;
use alw_core::edifice::{raise, Arch, Cond, Spec};
use alw_core::gen::WorldGen;
use alw_core::nord::{build, plan, NKind};
use std::collections::HashMap;
const SEED: i32 = 20261104;
const LOT: Lot = Lot { o: (0, 0), u: (1, 0), w: (0, 1) };
fn names(k: &Work) -> HashMap<(i32, i32, i32), &'static str> { let mut m = HashMap::new(); for b in &k.blocks { m.insert((b.0, b.1, b.2), BLOCKS[b.3 as usize].name); } m }
fn count(m: &HashMap<(i32, i32, i32), &'static str>, f: impl Fn(&str) -> bool) -> usize { m.values().filter(|n| f(n)).count() }

#[test]
fn les_batiments_du_nord() {
    for (kind, var) in [(NKind::Longue, 0), (NKind::Longue, 1), (NKind::Atelier, 0), (NKind::Hydromel, 0), (NKind::Halle, 0), (NKind::GrandeHalle, 0), (NKind::Etable, 0), (NKind::Ferme, 0)] {
        let mut k = Work::default(); let craft = if kind == NKind::Atelier { Some("forgeron") } else { None }; let bt = build(&mut k, LOT, 50, kind, var, 0.4, 0.6, craft); let m = names(&k); let pl = plan(kind, var);
        let at = |x: i32, y: i32, z: i32| m.get(&(x, y, z)).copied().unwrap_or("air");
        println!("{:<22} {}×{} : {} blocs, {} pièces de charpente, porte {:?}, lit {:?}, poste {:?}", kind.label(), pl.w, pl.d, k.blocks.len(), k.props.len(), bt.door, bt.bed, bt.spot);
        assert!(count(&m, |n| n.starts_with("turf")) > 25, "{kind:?} : un toit de gazon"); assert!(count(&m, |n| n.starts_with("spruce_log")) > 20, "{kind:?} : des rondins de sapin");
        assert_eq!(count(&m, |n| n.starts_with("roof_tiles") || n.starts_with("slate") || n.starts_with("thatch") || n.starts_with("plaster")), 0, "{kind:?} : ni tuiles, ni ardoise, ni chaume, ni enduit");
        assert!(k.props.len() >= 20, "{kind:?} : les rives croisées et les têtes de dragon aux deux pignons ({})", k.props.len());
        let d = bt.door.unwrap(); assert!(at(d, 51, 0).starts_with("door") || at(d, 51, 0) == "air", "{kind:?} : l'entrée ({})", at(d, 51, 0)); assert!(at(d, 51, -1) == "air" && at(d, 52, -1) == "air", "{kind:?} : le seuil dégagé");
        assert_eq!(at(0, 51, 0), "fieldstone", "{kind:?} : le soubassement de pierres sèches");
        if let Some(b) = bt.bed { assert!(at(b.0, 51, b.1) == "air" || !BLOCKS.iter().find(|x| x.name == at(b.0, 51, b.1)).unwrap().solid, "{kind:?} : place près du lit ({})", at(b.0, 51, b.1)); assert!(count(&m, |n| n.starts_with("bed")) >= 2); }
        match kind {
            NKind::Longue => { assert!(count(&m, |n| n.starts_with("round_shield")) >= 2, "les boucliers"); assert!(count(&m, |n| n == "campfire") >= 1, "le foyer"); assert!(count(&m, |n| n.starts_with("fur_bench")) >= 1 && count(&m, |n| n.starts_with("log_pile")) >= 4); }
            NKind::Atelier => { assert_eq!(at(2, 51, 3), "anvil"); assert_eq!(at(2, 51, 0), "air", "ouvert sur la rue"); }
            NKind::Hydromel => { assert!(count(&m, |n| n.starts_with("bed")) >= 6 && count(&m, |n| n == "barrel") >= 4 && count(&m, |n| n.starts_with("bench")) >= 6, "tables, tonneaux, lits"); }
            NKind::Halle | NKind::GrandeHalle => { assert!(count(&m, |n| n.starts_with("throne")) == 1, "le trône du jarl"); assert!(count(&m, |n| n.starts_with("round_shield")) >= 4, "la façade bordée de boucliers"); assert!(count(&m, |n| n == "campfire") >= 2, "le long foyer"); }
            NKind::Etable | NKind::Ferme => { assert!(count(&m, |n| n == "hay_bale") >= 3 && count(&m, |n| n.starts_with("trough")) == 2); assert!(at(d, 51, 0) == "air" && at(d + 1, 51, 0) == "air", "la grande porte"); }
        }
    }
    for a in [Arch::Chapel, Arch::Mill] { assert!(!Spec::north(a, 3, 0.5, 0.5, None, Cond::Kept).north, "{a:?} n'existe pas au Nord"); }
    let sp = Spec::north(Arch::House, 0, 0.5, 0.4, None, Cond::Abandoned); let mut k = Work::default(); let made = raise(&mut k, LOT, &sp, 50); let m = names(&k);
    assert!(made.door.is_none() && made.bed.is_none()); assert_eq!(count(&m, |n| n.starts_with("bed") || n.starts_with("door")), 0);
}

fn block_at(g: &WorldGen, cache: &mut HashMap<(i32, i32), Vec<Id>>, x: i32, y: i32, z: i32) -> &'static str {
    let c = (x.div_euclid(CX as i32), z.div_euclid(CX as i32)); let v = cache.entry(c).or_insert_with(|| g.gen_world(c.0, c.1));
    BLOCKS[v[idx(x.rem_euclid(CX as i32) as usize, y.clamp(0, 255) as usize, z.rem_euclid(CX as i32) as usize)] as usize].name
}

#[test]
fn un_village_viking() {
    let g = WorldGen::new(SEED);
    let all: Vec<_> = g.villages_near(0.0, 0.0, 3500.0).into_iter().filter(|v| matches!(v.kind, "taiga" | "montagne")).collect(); assert!(all.len() >= 2, "des villages du Nord");
    let mut palisades = 0; let mut fires = 0;
    for v in all.iter().take(4) {
        let i = v.ident.expect("refondu"); assert_eq!(i.civ, Civ::Nordic);
        println!("{} ({}) {:?} : {} ; {} bâtiments ({:?}), {} foyers, rayon {}", v.name, v.kind, v.c, i.describe(), v.works.len(), v.works.iter().map(|w| alw_core::nord::kind_of(w.0).map_or(w.0.label(), |k| k.label())).collect::<Vec<_>>(), v.houses.len(), v.r);
        assert!(v.works.iter().all(|w| !matches!(w.0, Arch::Chapel | Arch::Mill)), "ni église ni moulin"); assert!(v.houses.len() >= 2, "des maisons habitées");
        assert!(["Skog", "Fjell", "Ulf", "Hrafn", "Ask", "Stein", "Bjørn", "Is", "Varg", "Ørn"].iter().any(|p| v.name.starts_with(p)), "un nom du Nord : {}", v.name);
        let mut cache = HashMap::new(); let (x, y, z) = (v.c[0], v.c[1], v.c[2]); println!("  au milieu : {} sous {}", block_at(&g, &mut cache, x, y, z), block_at(&g, &mut cache, x, y + 1, z));
        if block_at(&g, &mut cache, x, y, z) != "water" { fires += 1; assert_eq!(block_at(&g, &mut cache, x, y + 1, z), "campfire", "le feu au milieu du cercle"); }
        let mut seen: HashMap<&str, usize> = HashMap::new(); let r = v.r;
        for dx in -r..=r { for dz in -r..=r { if dx * dx + dz * dz > r * r { continue; } for yy in y - 10..y + 24 { *seen.entry(block_at(&g, &mut cache, x + dx, yy, z + dz)).or_default() += 1; } } }
        let pre = |p: &str| seen.iter().filter(|(k, _)| k.starts_with(p)).map(|(_, c)| *c).sum::<usize>();
        println!("  gazon {} ; rondins {} ; boucliers {} ; pieux {} ; pierres moussues {} ; tuiles {} ; ardoise {}", pre("turf"), pre("spruce_log"), pre("round_shield"), pre("spruce_post"), pre("mossy_stone"), pre("roof_tiles"), pre("slate"));
        assert!(pre("turf") > 100, "des toits de gazon"); if v.works.iter().any(|w| w.0 == Arch::House) { assert!(pre("round_shield") >= 2, "les maisons longues et leurs boucliers"); } if block_at(&g, &mut cache, x, y, z) != "water" { assert!(pre("stone_pillar") >= 3, "la pierre runique"); }
        assert_eq!(pre("roof_tiles") + pre("thatch"), 0, "rien de médiéval");
        if pre("spruce_post") > 20 { palisades += 1; }
    }
    assert!(palisades >= 1, "au moins un village a sa palissade"); assert!(fires >= 1, "au moins un lieu du thing sur la terre ferme");
}
