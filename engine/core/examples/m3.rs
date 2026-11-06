//! Mesures du monde pour M3 : rivières, grottes, relief, temps de calcul.
use alw_core::{gen::WorldGen, blocks::*, mesher::far_mesh};
use std::collections::VecDeque; use std::time::Instant;
fn main() {
    for seed in [20261104, 7, -1234] {
        let g = WorldGen::new(seed); const N: usize = 1600; let step = 1.0; let (mut lo, mut hi) = (1e9f64, -1e9f64);
        let t = Instant::now(); let mut w = vec![false; N * N]; let mut lv = vec![0f64; N * N];
        for j in 0..N { for i in 0..N { let c = g.column((i as f64 - 800.0) * step, (j as f64 - 800.0) * step); lo = lo.min(c.h); hi = hi.max(c.h); if c.water != 0.0 && !c.still { w[i + j * N] = true; lv[i + j * N] = c.water; } } }
        let per = t.elapsed().as_secs_f64() / (N * N) as f64;
        let (mut seen, mut sizes, mut touch) = (vec![false; N * N], Vec::new(), 0);
        for s in 0..N * N { if !w[s] || seen[s] { continue; } let (mut q, mut n, mut edge) = (VecDeque::from([s]), 0, false); seen[s] = true;
            while let Some(p) = q.pop_front() { n += 1; let (i, j) = (p % N, p / N); if i == 0 || j == 0 || i == N - 1 || j == N - 1 { edge = true; }
                for dj in -3i32..=3 { for di in -3i32..=3 { let (a, b) = (i as i32 + di, j as i32 + dj); if a < 0 || b < 0 || a >= N as i32 || b >= N as i32 { continue; } let k = a as usize + b as usize * N; if w[k] && !seen[k] { seen[k] = true; q.push_back(k); } } } }
            sizes.push(n); if edge { touch += 1; } }
        sizes.sort(); let tot: usize = sizes.iter().sum(); let small = sizes.iter().filter(|s| **s < 40).count();
        println!("seed {seed}: relief {lo:.0}..{hi:.0}, colonne {:.1} µs, eau courante {:.2} %, {} morceaux (dont {small} petits), {touch} sortent de la zone, plus grand {}", per * 1e6, tot as f64 / (N * N) as f64 * 100.0, sizes.len(), sizes.last().unwrap_or(&0));
        let (mut air, mut open, mut solid, mut chunks) = (0usize, 0usize, 0usize, 0); let t = Instant::now();
        for cz in -8..8 { for cx in -8..8 { let v = g.gen_chunk(cx, cz); chunks += 1; let cn = CX as i32; for x in 0..CX { for z in 0..CX {
            let c = g.column((cx * cn + x as i32) as f64, (cz * cn + z as i32) as f64); let h = c.h as i32; let mut last_air = false; let mut gap = 0; let mut break_top = false; let mut counted = false;
            for y in (2..h.min(CH as i32 - 1)).rev() { let b = v[idx(x, y as usize, z)]; if b == 0 { air += 1; gap += 1; last_air = true; } else { solid += 1; if last_air && gap >= 0 { } last_air = false; if gap == 0 { break_top = true; } if !break_top && false { } } if !break_top && gap >= 6 && !counted { open += 1; counted = true; } }
        } } } }
        println!("  grottes: {:.2} % de vide sous la surface, {open} colonnes ouvertes en surface sur {} ; chunk {:.1} ms", air as f64 / (air + solid) as f64 * 100.0, chunks * CX * CX, t.elapsed().as_secs_f64() * 1000.0 / chunks as f64);
        let t = Instant::now(); let (_, n) = far_mesh(&g, 0, 0); println!("  horizon {n} sommets, {:.0} ms", t.elapsed().as_secs_f64() * 1000.0);
    }
    // Murs d'eau : une face d'eau exposée à l'air, sans rien pour la porter en dessous, est un aquarium suspendu.
    for seed in [20261104, 7, -1234] { let g = WorldGen::new(seed); let (mut walls, mut tall, mut mountain, mut columns) = (0, 0, 0, 0); let mut chunks = std::collections::HashMap::new();
        for cz in -16..16 { for cx in -16..16 { chunks.insert((cx, cz), g.gen_chunk(cx, cz)); } }
        let at = |x: i32, y: usize, z: i32| chunks.get(&(x.div_euclid(32), z.div_euclid(32))).map_or(1u8, |c| c[idx(x.rem_euclid(32) as usize, y, z.rem_euclid(32) as usize)]);
        for x in -500..500 { for z in -500..500 { let c = g.column(x as f64, z as f64); if c.water == 0.0 { continue; } columns += 1; if c.h > 76.0 { mountain += 1; } let wy = c.water as usize;
            for (a, b) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { let mut run = 0; let mut y = wy + 8; while y > 3 { if is_water(at(x, y, z)) && at(x + a, y, z + b) == 0 { run += 1; } y -= 1; } if run >= 1 && at(x + a, wy, z + b) == 0 { walls += 1; if run >= 3 { tall += 1; } } } } }
        println!("seed {seed}: {columns} colonnes d'eau, {mountain} en altitude, {walls} faces d'eau à l'air, {tall} de 3 blocs ou plus"); }
    // Une grotte : on part d'une entrée et on suit le vide sous terre.
    let g = WorldGen::new(20261104); let mut cache = std::collections::HashMap::new(); let mut get = |x: i32, y: i32, z: i32, g: &WorldGen| -> u8 { if !(0..CH as i32).contains(&y) { return 1; } let k = (x.div_euclid(32), z.div_euclid(32)); let c = cache.entry(k).or_insert_with(|| g.gen_chunk(k.0, k.1)); c[idx(x.rem_euclid(32) as usize, y as usize, z.rem_euclid(32) as usize)] };
    let mut reports = 0; 'e: for z in (-600..600).step_by(9) { for x in (-600..600).step_by(9) { let h = g.column(x as f64, z as f64).h as i32; if !(40..100).contains(&h) || !(1..7).all(|k| get(x, h - k, z, &g) == 0) { continue; }
        let (mut seen, mut q, mut deep, mut low) = (std::collections::HashSet::from([(x, h - 6, z)]), VecDeque::from([(x, h - 6, z)]), 0, h); 
        while let Some(p) = q.pop_front() { if seen.len() > 40000 { break; } let s = g.column(p.0 as f64, p.2 as f64).h as i32; if s - p.1 > 5 { deep += 1; low = low.min(p.1); } for d in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)] { let n = (p.0 + d.0, p.1 + d.1, p.2 + d.2); if seen.contains(&n) || get(n.0, n.1, n.2, &g) != 0 || n.1 >= g.column(n.0 as f64, n.2 as f64).h as i32 { continue; } seen.insert(n); q.push_back(n); } }
        println!("entrée ({x}, {z}) surface {h} : {} cases reliées, {deep} sous 5 blocs de roche, fond à {low}", seen.len()); reports += 1; if reports == 4 { break 'e; } } }
    // Sprint : le joueur avance tout droit, combien de chunks proches manquent ?
    for (name, speed) in [("course 7,2 m/s", 7.2), ("vol 22 m/s", 22.0)] {
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).saturating_sub(2).clamp(2, 12); let mut st = alw_core::stream::Streamer::new(20261104, &alw_core::stream::ULTRA, threads); let t0 = Instant::now(); let (mut x, mut wait, mut missing, mut frames, mut gaps) = (0.0f64, 0usize, 0usize, 0, 0);
        loop { let (b, _) = st.update(x, 0.5); drop(b); if st.waiting() == 0 && st.loaded() > 0 { break; } std::thread::sleep(std::time::Duration::from_millis(5)); if t0.elapsed().as_secs() > 60 { break; } } let load = t0.elapsed().as_secs_f64(); let t1 = Instant::now();
        while t1.elapsed().as_secs_f64() < 12.0 { x = t1.elapsed().as_secs_f64() * speed; let (b, _) = st.update(x, 0.5); drop(b); wait = wait.max(st.waiting()); frames += 1; let near = (0..4).filter(|k| { let px = x + *k as f64 * 32.0; st.voxel_ready(px, 0.5) }).count(); if near < 4 { missing += 1; gaps = gaps.max(4 - near); } std::thread::sleep(std::time::Duration::from_millis(16)); }
        println!("{name} : {threads} fils, chargement {load:.1} s, file max {wait}, {missing}/{frames} images avec un chunk manquant devant, jusqu'à {gaps}");
    }
}
