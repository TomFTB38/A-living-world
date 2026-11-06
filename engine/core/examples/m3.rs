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
}
