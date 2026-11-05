//! Le monde Rust doit être identique, voxel par voxel, à celui du prototype validé.
use alw_core::{det, gen::WorldGen, mesher::{far_mesh, Mesher}, noise::*, store::ChunkStore};
fn fnv(b: &[u8]) -> u32 { let mut h = 0x811c9dc5u32; for v in b { h ^= *v as u32; h = h.wrapping_mul(0x01000193); } h }
fn bits(v: f64) -> String { format!("{:x}", v.to_bits()) }
#[test]
fn parite_avec_le_prototype() {
    let txt = include_str!("parity.txt"); let (mut ok, mut chunks, mut meshes) = (0, 0, 0);
    for l in txt.lines() {
        let a: Vec<&str> = l.split(' ').collect(); let f = |i: usize| a[i].parse::<f64>().unwrap();
        match a[0] {
            "trig" => { assert_eq!(bits(det::sin(f(1))), a[2], "sin {}", a[1]); assert_eq!(bits(det::cos(f(1))), a[3], "cos {}", a[1]); }
            "hyp" => { assert_eq!(bits(det::hypot2(f(1), f(2))), a[4]); assert_eq!(bits(det::hypot3(f(1), f(2), f(3))), a[5]); }
            "hash" => assert_eq!(hash_int(-5, 7, 20261104, 21).to_string(), a[1]),
            "rng" => { let mut r = Rng::new(12345); assert_eq!(bits(r.next()), a[1]); assert_eq!(bits(r.next()), a[2]); }
            "noise" => { let n = Noise::new(77); assert_eq!(bits(n.n2(12.5, -3.25)), a[1]); assert_eq!(bits(n.n3(1.2, 2.3, -3.4)), a[2]); assert_eq!(bits(n.fbm(0.7, 0.2, 3)), a[3]); }
            "chunk" => { let g = WorldGen::new(a[1].parse().unwrap()); let v = g.gen_chunk(a[2].parse().unwrap(), a[3].parse().unwrap()); assert_eq!(fnv(&v).to_string(), a[4], "chunk {} {} {}", a[1], a[2], a[3]); chunks += 1; }
            "mesh" => {
                let mut st = ChunkStore::new(a[1].parse().unwrap(), 200); let mut m = Mesher::new();
                let (cx, cz, lod): (i32, i32, u8) = (a[2].parse().unwrap(), a[3].parse().unwrap(), a[4].parse().unwrap());
                let r = m.mesh_chunk(&mut |x, z| st.get(x, z), cx, cz, lod);
                let got = format!("{} {} {} {} {} {} {} {}", r.n_o, r.n_d, r.n_w, fnv(&r.opaque), fnv(&r.deco), fnv(&r.water), r.ymin, r.ymax);
                assert_eq!(got, a[5..].join(" "), "{l}"); meshes += 1;
            }
            "far" => { let g = WorldGen::new(a[1].parse().unwrap()); let (d, n) = far_mesh(&g, 1, -2); assert_eq!(format!("{} {}", n, fnv(&d)), a[2..].join(" ")); }
            "noise3" => assert_eq!(fnv(&alw_core::textures::build_noise_3d(40)).to_string(), a[1]),
            "tex" => { let t = alw_core::textures::build_textures(128); assert_eq!(format!("{} {}", fnv(&t.albedo), fnv(&t.nrm)), a[1..].join(" ")); }
            _ => panic!("ligne inconnue"),
        }
        ok += 1;
    }
    println!("{ok} empreintes vérifiées, dont {chunks} chunks et {meshes} maillages");
}
