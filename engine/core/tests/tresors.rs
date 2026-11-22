//! Cartes au trésor et trésors cachés : des cachettes (un cairn sur un coffre enfoui) semées sur la terre ferme, loin des
//! villages et des lieux ; une carte en révèle une, qu'on retrouve sur place, qu'on ouvre, qui rapporte de l'expérience.
use alw_core::{blocks::*, gen::*, mesher::{VBuf, VSIZE}, meubles, noise::Rng, player::*, session::*, stream::*, tresors};
const SEED: i32 = 20261104;
const DT: f64 = 1.0 / 60.0;

fn ready(s: &mut Session) { for _ in 0..20000 { let p = s.player.p; s.world.update(p[0], p[2]); let _ = s.sync(); if s.world.waiting() == 0 && s.world.loaded() > 0 { return; } std::thread::sleep(std::time::Duration::from_millis(3)); } panic!("trop long"); }
fn block_at(g: &WorldGen, p: [i32; 3]) -> Id { let v = g.gen_world(p[0].div_euclid(CX as i32), p[2].div_euclid(CX as i32)); v[idx(p[0].rem_euclid(CX as i32) as usize, p[1] as usize, p[2].rem_euclid(CX as i32) as usize)] }

#[test]
fn des_cachettes_sur_la_terre_ferme() {
    let g = WorldGen::new(SEED); let cairn = by_name("cairn").expect("le bloc cairn");
    let all = g.caches_near(0.0, 0.0, 3000.0); println!("{} cachettes dans un rayon de 3 km", all.len()); assert!(all.len() >= 40, "assez de trésors à chercher : {}", all.len());
    for p in all.iter().take(12) {
        assert!(g.cache_chest(*p)); assert!(!g.cache_chest([p[0], p[1] + 1, p[2]]));
        assert_eq!(block_at(&g, *p), B::CHEST, "le coffre est enfoui en {p:?}"); assert_eq!(block_at(&g, [p[0], p[1] + 3, p[2]]), cairn, "le cairn le marque en surface");
        for dy in 1..=2 { let b = block_at(&g, [p[0], p[1] + dy, p[2]]); assert!(is_solid(b) && !is_water(b), "de la terre au-dessus du coffre ({})", BLOCKS[b as usize].name); }
        assert!(!is_water(block_at(&g, [p[0], p[1] + 4, p[2]])), "pas sous l'eau");
        assert!(g.villages_near(p[0] as f64, p[2] as f64, 90.0).is_empty() && g.places_near(p[0] as f64, p[2] as f64, 48.0).is_empty(), "loin des villages et des lieux");
    }
    // le monde plat des essais n'en a pas
    assert!(WorldGen::new(FLAT | 0x123).caches_near(0.0, 0.0, 3000.0).is_empty());
}

#[test]
fn le_cairn_et_la_carte_se_dessinent() {
    let m = BLOCKS[by_name("cairn").unwrap() as usize].model.unwrap(); let mut o = VBuf::new(8000); assert!(meubles::draw(&mut o, m, "cairn", 0.0, 0.0, 0.0, 1.0, 1.0, &mut Rng::new(3)));
    assert!(o.n >= 100); for v in o.data.chunks_exact(VSIZE) { let g = |i: usize| i16::from_le_bytes([v[i], v[i + 1]]) as f64 / o.scale; for i in [0, 2, 4] { assert!((-0.02..=1.02).contains(&g(i)), "le cairn déborde : {:.3}", g(i)); } }
    let px = (0..56 * 56).filter(|k| alw_core::icons::look("treasure_map", (k % 56) as f64 / 56.0, (k / 56) as f64 / 56.0).is_some()).count(); assert!(px > 800, "l'icône de la carte : {px} pixels");
    let red = (0..56 * 56).filter_map(|k| alw_core::icons::look("treasure_map", (k % 56) as f64 / 56.0, (k / 56) as f64 / 56.0)).filter(|(c, _)| c[0] > 0.6 && c[1] < 0.2).count(); assert!(red > 4, "et sa croix rouge");
    assert!(alw_core::survival::recipes().iter().any(|r| r.out == "cairn"), "on dresse un cairn soi-même");
    assert_eq!(tresors::direction(0.0, -10.0), "au nord"); assert_eq!(tresors::direction(10.0, 0.0), "à l'est"); assert_eq!(tresors::direction(-7.0, 7.0), "au sud-ouest");
}

#[test]
fn une_carte_mene_a_un_tresor() {
    let g = WorldGen::new(SEED); let c0 = g.caches_near(0.0, 0.0, 3000.0)[0];
    // On lit la carte à 300 blocs d'une cachette : elle en marque une (entre 120 et 900 blocs).
    let start = [c0[0] as f64 + 300.5, 120.0, c0[2] as f64 + 0.5];
    let mut s = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, start); s.set_creative(false);
    s.inv = [None; INV_SIZE]; s.sel = 0; s.inv[0] = Some(Stack::new("treasure_map", None)); s.prog.xp = 0.0;
    assert!(s.read_treasure_map()); assert!(s.inv[0].is_none(), "la carte est lue"); assert_eq!(s.treasure_marks.len(), 1);
    let m = s.treasure_marks[0]; let d = (m[0] as f64 - start[0]).hypot(m[2] as f64 - start[2]); assert!(g.cache_chest(m) && (tresors::MAP_NEAR..=tresors::MAP_FAR).contains(&d), "une vraie cachette à {d:.0} m");
    let note = s.notes.last().unwrap().clone(); assert!(note.contains(" m ") && note.contains("cairn"), "{note}"); println!("{note}");
    assert_eq!(s.treasure_targets().len(), 1, "la boussole la montre");
    // une seconde carte mène ailleurs
    s.inv[0] = Some(Stack::new("treasure_map", None)); for _ in 0..40 { s.tick(DT, Input::default(), Actions::default()); } s.read_treasure_map(); assert_eq!(s.treasure_marks.len(), 2); assert_ne!(s.treasure_marks[0], s.treasure_marks[1]);
    // la marque se sauve avec la partie
    let t = s.save_text(); let mut b = Session::new(SEED, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, start); b.load_text(&t).unwrap(); assert_eq!(b.treasure_marks, s.treasure_marks);
    // Sur place : le cairn, et le coffre dessous. On l'ouvre : le trésor, de l'expérience, la marque s'efface.
    s.player.p = [m[0] as f64 + 0.5, m[1] as f64 + 4.0, m[2] as f64 + 2.5]; ready(&mut s);
    assert_eq!(s.world.get(m[0], m[1] + 3, m[2]), by_name("cairn")); assert_eq!(s.world.get(m[0], m[1], m[2]), Some(B::CHEST));
    let xp = s.prog.xp; s.open_chest(m); assert!(s.prog.xp >= xp + tresors::TREASURE_XP - 1e-9, "de l'expérience");
    let coins: u32 = s.chests[&m].iter().flatten().filter(|st| st.item == "worn_coin").map(|st| st.n.unwrap_or(1)).sum(); assert!(coins >= 25, "des pièces : {coins}");
    assert_eq!(s.treasure_marks.len(), 1, "la marque trouvée s'efface"); assert!(s.notes.iter().any(|n| n.contains("trésor de la carte")));
    s.close_chest(); let xp = s.prog.xp; s.open_chest(m); assert_eq!(s.prog.xp, xp, "une seule fois");
    // Là où il n'y a rien (monde plat), la carte n'est pas gâchée.
    let mut f = Session::new(FLAT | 0x123, &Preset { radius: 2, lod1: 1.5, lod2: 2.5 }, 4, [0.5, 60.0, 0.5]); f.inv[0] = Some(Stack::new("treasure_map", None)); f.sel = 0;
    assert!(f.read_treasure_map()); assert!(f.inv[0].is_some(), "la carte reste"); assert!(f.treasure_marks.is_empty());
    let _ = DT;
}

#[test]
fn des_cartes_dans_les_coffres() {
    // Une carte se trouve dans les coffres des lieux et des cachettes elles-mêmes (une piste qui en appelle une autre).
    let g = WorldGen::new(SEED); let s = Session::new(SEED, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 2, [0.5, 120.0, 0.5]);
    let caches = g.caches_near(0.0, 0.0, 4000.0); let with_map = caches.iter().filter(|p| s.chests.get(*p).is_none()).filter(|p| alw_core_loot(&s, **p)).count();
    println!("{with_map} cachettes sur {} gardent une autre carte", caches.len()); assert!(with_map >= 3 && with_map < caches.len());
}
fn alw_core_loot(s: &Session, p: [i32; 3]) -> bool { s.loot_of(p).iter().flatten().any(|st| st.item == "treasure_map") }
