//! Rend les bêtes sculptées sur le processeur, pour juger des modèles : betes <sortie.ppm> <grands|petits|oiseaux|mer> <échelle> <angle>.
use alw_core::{actors::{Models, Range}, mesher::VSIZE, view::M4};
fn draw(img: &mut [[u8; 3]], zb: &mut [f32], w: usize, h: usize, md: &Models, list: &[(M4, Range)], yaw: f64, ox: f64) {
    let (cy, sy) = (yaw.cos(), yaw.sin()); let sc = std::env::args().nth(3).and_then(|s| s.parse().ok()).unwrap_or(75.0f64);
    for (m, r) in list {
        let (a, b) = (r.first as usize / 6 * 4, (r.first + r.count) as usize / 6 * 4);
        for q in (a..b).step_by(4) {
            let mut pts = Vec::new(); let mut col = [0f32; 3]; let mut wp = Vec::new();
            for k in 0..4 { let v = &md.data[(q + k) * VSIZE..(q + k + 1) * VSIZE]; let p = [0, 2, 4].map(|j| i16::from_le_bytes([v[j], v[j + 1]]) as f64 / 128.0); let m = m.map(|x| x as f64);
                let x = m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12]; let y = m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13]; let z = m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14];
                let (rx, rz) = (x * cy - z * sy, x * sy + z * cy); wp.push([x, y, z]);
                pts.push([(ox + rx * sc) as f32 + w as f32 * 0.5, (h as f64 - 20.0 - y * sc) as f32, -rz as f32]); col = [v[16] as f32, v[17] as f32, v[18] as f32]; }
            let e1 = [wp[1][0] - wp[0][0], wp[1][1] - wp[0][1], wp[1][2] - wp[0][2]]; let e2 = [wp[2][0] - wp[0][0], wp[2][1] - wp[0][1], wp[2][2] - wp[0][2]];
            let n = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]]; let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(1e-9);
            let d = (n[0] * 0.45 + n[1] * 0.75 + n[2] * 0.5) / l; let lit = (0.5 + 0.5 * d.abs()) as f32;
            for tri in [[0, 1, 2], [0, 2, 3]] { let (p0, p1, p2) = (pts[tri[0]], pts[tri[1]], pts[tri[2]]);
                let area = (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p1[1] - p0[1]) * (p2[0] - p0[0]); if area.abs() < 1e-6 { continue; }
                let (x0, x1) = (p0[0].min(p1[0]).min(p2[0]).max(0.0) as usize, (p0[0].max(p1[0]).max(p2[0]) as usize).min(w - 1)); let (y0, y1) = (p0[1].min(p1[1]).min(p2[1]).max(0.0) as usize, (p0[1].max(p1[1]).max(p2[1]) as usize).min(h - 1));
                for yy in y0..=y1 { for xx in x0..=x1 { let (px, py) = (xx as f32 + 0.5, yy as f32 + 0.5); let w0 = ((p1[0] - px) * (p2[1] - py) - (p1[1] - py) * (p2[0] - px)) / area; let w1 = ((p2[0] - px) * (p0[1] - py) - (p2[1] - py) * (p0[0] - px)) / area; let w2 = 1.0 - w0 - w1;
                    if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 { continue; } let z = w0 * p0[2] + w1 * p1[2] + w2 * p2[2]; let i = yy * w + xx; if z < zb[i] { zb[i] = z; img[i] = col.map(|c| (c * 2.0 * lit).min(255.0) as u8); } } } }
        }
    }
}

use alw_core::view::{mul, trs, roll_z};
fn main() {
    let a: Vec<String> = std::env::args().collect(); let which = a.get(2).cloned().unwrap_or("all".into()); let yaw: f64 = a.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.8);
    let md = Models::build(); let (w, h) = (1600usize, 800usize); let mut img = vec![[70u8, 78, 88]; w * h]; let mut zb = vec![f32::MAX; w * h];
    let mut l: Vec<(M4, Range)> = Vec::new(); let mut put = |n: &str, m: M4| l.push((m, md.part(n)));
    let quad = |put: &mut dyn FnMut(&str, M4), x: f64, z: f64, sc: f64, hip: f64, nb: &str, nh: &str, nl: &str, hp: [f64; 3], legs: [(f64, f64); 4], hpitch: f64| {
        let base = trs(x, 0.0, z, 0.0, 0.0, sc); put(nb, mul(&base, &trs(0.0, hip, 0.0, 0.0, 0.0, 1.0))); put(nh, mul(&base, &trs(hp[0], hip + hp[1], hp[2], 0.0, hpitch, 1.0)));
        for (k, (lx, lz)) in legs.iter().enumerate() { put(nl, mul(&base, &trs(*lx, hip, *lz, 0.0, if k % 2 == 0 { 0.25 } else { -0.25 }, 1.0))); } };
    let bird = |put: &mut dyn FnMut(&str, M4), x: f64, z: f64, nb: &str, nw: &str, k: f64, sx: f64, f: f64| { let base = trs(x, 0.0, z, 0.0, 0.0, 1.0); put(nb, base);
        put(nw, mul(&base, &mul(&trs(sx * k, 0.08 * k, 0.0, 0.0, 0.0, 1.0), &roll_z(f)))); put(nw, mul(&base, &mul(&trs(-sx * k, 0.08 * k, 0.0, 0.0, 0.0, 1.0), &roll_z(std::f64::consts::PI - f)))); };
    match which.as_str() {
        "grands" => {
            quad(&mut put, -3.2, 0.0, 1.0, 0.72, "deerBody", "deerHead", "deerLeg", [0.0, 0.28, 0.45], [(-0.13, 0.44), (0.13, 0.44), (-0.13, -0.44), (0.13, -0.44)], 0.42);
            { let base = trs(-0.6, 0.0, 0.0, 0.0, 0.0, 2.3); let body = mul(&base, &trs(0.0, 0.26, 0.0, 0.0, 0.0, 1.0)); put("bear", body); put("bearHead", mul(&body, &trs(0.0, 0.14, 0.32, 0.0, 0.1, 1.0)));
              for (k, (x, z)) in [(-0.11, 0.2), (0.11, 0.2), (-0.11, -0.22), (0.11, -0.22)].iter().enumerate() { put("bearLeg", mul(&body, &trs(*x, 0.0, *z, 0.0, if k % 2 == 0 { 0.2 } else { -0.2 }, 1.0))); } }
            quad(&mut put, 1.9, 0.0, 1.25, 0.2, "boar", "boarHead", "boarLeg", [0.0, 0.08, 0.32], [(-0.112, 0.22), (0.112, 0.22), (-0.112, -0.24), (0.112, -0.24)], 0.0);
            quad(&mut put, 3.6, 0.0, 1.5, 0.34, "ibex", "ibexHead", "ibexLeg", [0.0, 0.16, 0.26], [(-0.06, 0.18), (0.06, 0.18), (-0.06, -0.2), (0.06, -0.2)], -0.1);
        }
        "petits" => {
            quad(&mut put, -2.4, 0.0, 1.75, 0.32, "wolf", "wolfHead", "wolfLeg", [0.0, 0.14, 0.32], [(-0.08, 0.22), (0.08, 0.22), (-0.08, -0.24), (0.08, -0.24)], 0.0);
            quad(&mut put, -0.6, 0.0, 1.45, 0.28, "fox", "foxHead", "foxLeg", [0.0, 0.12, 0.3], [(-0.07, 0.2), (0.07, 0.2), (-0.07, -0.22), (0.07, -0.22)], 0.0);
            for (i, n) in ["hare0", "hare1", "hare2", "hare3"].iter().enumerate() { put(n, trs(0.6 + i as f64 * 0.55, 0.0, 0.0, 0.0, 0.0, 1.35)); }
            put("lizard", trs(3.2, 0.0, 0.0, 0.0, 0.0, 1.6));
        }
        "oiseaux" => {
            bird(&mut put, -2.0, 0.0, "bird", "wing", 1.7, 0.055, -1.25); bird(&mut put, -1.0, 0.0, "bird", "wing", 1.7, 0.055, 0.3);
            bird(&mut put, 0.0, 0.0, "crow", "crowWing", 1.9, 0.06, -1.25); bird(&mut put, 1.0, 0.0, "crow", "crowWing", 1.9, 0.06, 0.3);
            bird(&mut put, 2.0, 0.0, "gull", "gullWing", 1.5, 0.05, -1.3); bird(&mut put, 3.0, 0.0, "gull", "gullWing", 1.5, 0.05, 0.2);
        }
        _ => {
            for (i, n) in ["mackerel", "reef0", "reef1", "reef2"].iter().enumerate() { put(n, trs(-2.4 + i as f64 * 0.9, 0.3, 0.0, 0.0, 0.0, 1.0)); }
            put("crab", trs(1.6, 0.0, 0.0, std::f64::consts::FRAC_PI_2, 0.0, 1.3));
        }
    }
    draw(&mut img, &mut zb, w, h, &md, &l, yaw, 0.0);
    let mut out = format!("P6 {w} {h} 255\n").into_bytes(); for p in &img { out.extend_from_slice(p); } std::fs::write(&a[1], out).unwrap();
}
