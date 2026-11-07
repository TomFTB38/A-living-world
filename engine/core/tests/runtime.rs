use alw_core::{player::*, stream::*, textures::*, view::*};
#[test]
fn le_monde_se_charge_et_le_joueur_pose_le_pied() {
    let mut s = Streamer::new(20261104, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 4); let (x, z) = (-38.5, 0.5);
    let t = std::time::Instant::now(); let (mut n, mut ms, mut tris) = (0, 0.0, 0);
    loop { let (b, _) = s.update(x, z); for c in b { if c.lod == FAR { continue; } n += 1; ms += c.ms; tris += (c.mesh.n_o + c.mesh.n_d) / 2; } if s.waiting() == 0 { break; } std::thread::sleep(std::time::Duration::from_millis(2)); assert!(t.elapsed().as_secs() < 60); }
    println!("{n} chunks en {:.2} s, {:.1} ms par chunk sur un fil, {tris} triangles", t.elapsed().as_secs_f64(), ms / n as f64);
    assert_eq!(n, s.loaded()); assert!(n >= 29);
    let mut p = Player::new(x, 90.0, z);
    for _ in 0..600 { p.update(&s, 1.0 / 60.0, Input::default()); }
    assert!(p.ground, "au sol"); println!("sol à {}", p.p[1]); assert!(p.p[1] > 30.0 && p.p[1] < 80.0);
    let y0 = p.p[1]; p.update(&s, 1.0 / 60.0, Input { jump: true, ..Default::default() }); for _ in 0..8 { p.update(&s, 1.0 / 60.0, Input::default()); } assert!(p.p[1] > y0, "saut");
    p.yaw = 0.0; for _ in 0..240 { p.update(&s, 1.0 / 60.0, Input { z: 1.0, ..Default::default() }); } assert!(p.p[2] > 0.5, "avance vers +z : {:?}", p.p);
    let (b, gone) = { for _ in 0..3 { s.update(x, z); } s.update(x + 32.0 * 9.0, z) }; drop(b); assert!(!gone.is_empty(), "les chunks lointains sont oubliés");
}
#[test]
fn matrices_et_champ() {
    let v = look_dir([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]); let p = perspective(1.2, 16.0 / 9.0, 0.1); let vp = mul(&p, &v);
    let t = |q: [f32; 4]| { let mut o = [0.0; 4]; for r in 0..4 { for c in 0..4 { o[r] += vp[r + c * 4] * q[c]; } } o };
    let a = t([0.0, 0.0, 10.0, 1.0]); assert!(a[3] > 0.0 && a[0].abs() < 1e-5, "{a:?}"); assert!((a[2] / a[3] - 0.01).abs() < 1e-5, "profondeur inversée : {a:?}");
    let b = t([1.0, 0.0, 10.0, 1.0]); assert!(b[0] < 0.0, "+x est à gauche quand on regarde vers +z : {b:?}");
    let i = invert(&vp); let e = mul(&i, &vp); for k in 0..16 { assert!((e[k] - if k % 5 == 0 { 1.0 } else { 0.0 }).abs() < 1e-3, "{e:?}"); }
    assert!(visible(&vp, [-1.0, -1.0, 5.0], [1.0, 1.0, 6.0])); assert!(!visible(&vp, [-1.0, -1.0, -6.0], [1.0, 1.0, -5.0])); assert!(!visible(&vp, [200.0, -1.0, 5.0], [201.0, 1.0, 6.0]));
    let s = sky_at(10.0); assert!(s.dir[1] > 0.2 && s.col[0] > 1.0); assert!(sky_at(1.0).night > 0.9);
}
#[test]
fn niveaux_de_detail() { let t = build_textures(128); let m = mip_chain(&t.albedo, 128, 1.max(2)); assert_eq!(m.len(), 8); assert_eq!(m[7].len(), 4 * 2); }

#[test]
fn le_monde_ne_depend_que_de_la_seed() {
    use alw_core::{gen::WorldGen, biome::*};
    let (a, b) = (WorldGen::new(20261104), WorldGen::new(20261104)); let keys = [(0, 0), (-40, 12), (70, -55), (-120, -90), (150, 140)];
    let first: Vec<_> = keys.iter().map(|k| a.gen_chunk(k.0, k.1)).collect(); for (i, k) in keys.iter().enumerate().rev() { assert_eq!(first[i], b.gen_chunk(k.0, k.1), "chunk {k:?} différent selon l'ordre"); }
    assert_ne!(first[1], WorldGen::new(7).gen_chunk(-40, 12), "une autre seed donne un autre monde");
    let c = Climate::new(20261104); for (x, z) in [(0.0, 0.0), (120.0, -200.0), (-300.0, 150.0)] { assert!(c.at(x, z).of(Biome::Temperate) > 0.6, "le départ reste tempéré en {x},{z}"); }
    for x in (-6000..6000).step_by(97) { for z in (-6000..6000).step_by(89) { let m = c.at(x as f64, z as f64); assert!((m.w.iter().sum::<f64>() - 1.0).abs() < 1e-9); let n = c.at(x as f64 + 8.0, z as f64); for i in 0..COUNT { assert!((m.w[i] - n.w[i]).abs() < 0.16, "transition brutale en {x},{z}"); } } }
}

#[test]
fn chaque_biome_a_un_vrai_coeur() {
    use alw_core::biome::*; let c = Climate::new(20261104); let mut top = [0.0f64; COUNT];
    for x in (-9000..9000).step_by(40) { for z in (-9000..9000).step_by(40) { let m = c.at(x as f64, z as f64); for i in 0..COUNT { top[i] = top[i].max(m.w[i]); } } }
    println!("{top:?}"); for b in [Biome::Temperate, Biome::Meadow, Biome::DeepForest, Biome::Taiga, Biome::Steppe, Biome::Desert] { assert!(c_strong(&top, b) > 0.95, "{} jamais franc : {:?}", NAMES[b as usize], top); }
}
fn c_strong(t: &[f64; alw_core::biome::COUNT], b: alw_core::biome::Biome) -> f64 { alw_core::noise::smooth(0.3, 0.55, t[b as usize]) }

#[test]
fn la_foret_profonde_se_distingue() {
    use alw_core::{gen::WorldGen, biome::*, blocks::*}; let g = WorldGen::new(20261104); let (mut spot, mut r) = (None, 8); 'f: while r < 500 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); if g.column(x, z).mix.strong(Biome::DeepForest) > 0.97 { spot = Some(((x / 32.0).floor() as i32, (z / 32.0).floor() as i32)); break 'f; } } r += 1; }
    let (cx, cz) = spot.expect("une forêt profonde"); let mut n = std::collections::HashMap::new(); for a in -2..2 { for b in -2..2 { for id in g.gen_chunk(cx + a, cz + b) { *n.entry(id).or_insert(0usize) += 1; } } } let c = |b: u8| *n.get(&b).unwrap_or(&0);
    println!("feuilles sombres {}, claires {}, lianes {}, champignons lumineux {}", c(B::OAK_LEAVES_DEEP) + c(B::BEECH_LEAVES_DEEP), c(B::OAK_LEAVES) + c(B::BEECH_LEAVES), c(B::VINE), c(B::GLOWCAP));
    assert!(c(B::OAK_LEAVES_DEEP) + c(B::BEECH_LEAVES_DEEP) > 4 * (c(B::OAK_LEAVES) + c(B::BEECH_LEAVES) + 1)); assert!(c(B::GLOWCAP) > 3 && c(B::VINE) > 50);
}

#[test]
fn chaque_champignon_lumineux_a_sa_couleur() {
    use alw_core::mesher::*; let mut seen = std::collections::HashSet::new(); for x in 0..60 { let c = glow_color(x, 50, -x); seen.insert(format!("{c:?}")); assert_eq!(c, glow_color(x, 50, -x)); } assert_eq!(seen.len(), 5);
}

#[test]
fn la_taiga_est_enneigee() {
    use alw_core::{gen::WorldGen, biome::*, blocks::*}; let g = WorldGen::new(20261104); let (mut spot, mut r) = (None, 8); 'f: while r < 500 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); if g.column(x, z).mix.strong(Biome::Taiga) > 0.97 { spot = Some(((x / 32.0).floor() as i32, (z / 32.0).floor() as i32)); break 'f; } } r += 1; }
    let (cx, cz) = spot.expect("une taïga"); let mut n = std::collections::HashMap::new(); for a in -2..2 { for b in -2..2 { for id in g.gen_chunk(cx + a, cz + b) { *n.entry(id).or_insert(0usize) += 1; } } } let c = |b: u8| *n.get(&b).unwrap_or(&0);
    let (mut sn, mut gr) = (0, 0); for x in 0..64 { for z in 0..64 { let (wx, wz) = (cx * 32 - 16 + x, cz * 32 - 16 + z); if g.column(wx as f64, wz as f64).mix.strong(Biome::Taiga) < 0.97 { continue; } let v = g.gen_chunk(wx.div_euclid(32), wz.div_euclid(32)); let mut y = 126; while y > 1 && v[idx(wx.rem_euclid(32) as usize, y, wz.rem_euclid(32) as usize)] == 0 { y -= 1; } let b = v[idx(wx.rem_euclid(32) as usize, y, wz.rem_euclid(32) as usize)]; if b == B::SNOW_LAYER || is_leaf(b) || is_water(b) || b == B::ICE || is_deco(b) { sn += 1; } else if b == B::GRASS || is_opaque(b) { gr += 1; } } } println!("cœur : {sn} colonnes enneigées, {gr} herbe");
    println!("pellicule {}, roche {}, terre {}, pommes de pin {}, givre {}, neige {}, herbe {}, épicéa {}, aiguilles enneigées {}, glaçons {}, glace {}", c(B::SNOW_LAYER), c(B::STONE) + c(B::GRANITE) + c(B::LIMESTONE), c(B::DIRT), c(B::PINECONE), c(B::FROST_GRASS), c(B::SNOW), c(B::GRASS), c(B::SPRUCE_LOG), c(B::SPRUCE_LEAVES_SNOW), c(B::ICICLE), c(B::ICE));
    assert!(sn > 20 * (gr + 1), "cœur de taïga enneigé"); assert!(c(B::SPRUCE_LOG) > 20 && c(B::SPRUCE_LEAVES_SNOW) > 50 && c(B::PINECONE) > 10);
}
