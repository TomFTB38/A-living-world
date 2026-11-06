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
    for x in (-6000..6000).step_by(97) { for z in (-6000..6000).step_by(89) { let m = c.at(x as f64, z as f64); assert!((m.w.iter().sum::<f64>() - 1.0).abs() < 1e-9); let n = c.at(x as f64 + 8.0, z as f64); for i in 0..COUNT { assert!((m.w[i] - n.w[i]).abs() < 0.15, "transition brutale en {x},{z}"); } } }
}
