//! La vue à la première personne avec l'arc : au repos, à moitié bandé, bandé à fond. fp_arc <sortie.ppm>
use alw_core::{actors::Models, mesher::VSIZE};
fn main() {
    let out = std::env::args().nth(1).unwrap_or("arc.ppm".into());
    let md = Models::build(); let (pw, h) = (640usize, 400usize); let n = 3; let w = pw * n; let mut img = vec![[120u8, 150, 185]; w * h]; let mut zb = vec![f32::MAX; w * h];
    let mut a = alw_core::actors::Anim::default(); let id: alw_core::view::M4 = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    let foc = 1.0 / (0.55f64).tan(); let sun = { let s = [0.4f64, 0.8, 0.45]; let l = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt(); s.map(|x| x / l) };
    for (k, b) in [0.0, 0.5, 1.0].iter().enumerate() {
        a.bow = *b; let held = md.held("bow"); let d = a.first(&md, &id, held, None); let ox = (k * pw) as f64;
        for dr in d { let m = dr.m.map(|x| x as f64); let (s0, s1) = (dr.r.first as usize / 6 * 4, (dr.r.first + dr.r.count) as usize / 6 * 4);
            for q in (s0..s1).step_by(4) { let mut pts = Vec::new(); let mut col = [0f64; 3]; let mut wp = Vec::new();
                for j in 0..4 { let v = &md.data[(q + j) * VSIZE..(q + j + 1) * VSIZE]; let p = [0, 2, 4].map(|i| i16::from_le_bytes([v[i], v[i + 1]]) as f64 / 128.0);
                    let x = m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12]; let y = m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13]; let z = m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14]; wp.push([x, y, z]);
                    let f = foc / -z.min(-0.01); pts.push([(ox + pw as f64 * 0.5 + x * f * h as f64 * 0.5) as f32, (h as f64 * 0.5 - y * f * h as f64 * 0.5) as f32, -z as f32]); col = [v[16] as f64, v[17] as f64, v[18] as f64]; }
                let (e1, e2) = ([wp[1][0] - wp[0][0], wp[1][1] - wp[0][1], wp[1][2] - wp[0][2]], [wp[3][0] - wp[0][0], wp[3][1] - wp[0][1], wp[3][2] - wp[0][2]]);
                let nn = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]]; let l = (nn[0] * nn[0] + nn[1] * nn[1] + nn[2] * nn[2]).sqrt().max(1e-12);
                let sh = 0.5 + 0.5 * ((nn[0] * sun[0] + nn[1] * sun[1] + nn[2] * sun[2]) / l).abs();
                for tri in [[0, 1, 2], [0, 2, 3]] { let (p0, p1, p2) = (pts[tri[0]], pts[tri[1]], pts[tri[2]]); let area = (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p1[1] - p0[1]) * (p2[0] - p0[0]); if area.abs() < 1e-6 { continue; }
                    let x0 = (p0[0].min(p1[0]).min(p2[0]).max(ox as f32)) as usize; let x1 = (p0[0].max(p1[0]).max(p2[0]) as usize).min(ox as usize + pw - 1); let y0 = p0[1].min(p1[1]).min(p2[1]).max(0.0) as usize; let y1 = (p0[1].max(p1[1]).max(p2[1]).max(0.0) as usize).min(h - 1);
                    for yy in y0..=y1 { for xx in x0..=x1 { let (px, py) = (xx as f32 + 0.5, yy as f32 + 0.5); let w0 = ((p1[0] - px) * (p2[1] - py) - (p1[1] - py) * (p2[0] - px)) / area; let w1 = ((p2[0] - px) * (p0[1] - py) - (p2[1] - py) * (p0[0] - px)) / area; let w2 = 1.0 - w0 - w1;
                        if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 { continue; } let z = w0 * p0[2] + w1 * p1[2] + w2 * p2[2]; let i = yy * w + xx; if z < zb[i] { zb[i] = z; img[i] = col.map(|c| (c * 2.0 * sh).min(255.0) as u8); } } } } } }
        for d in -6i32..=6 { for (x, y) in [(ox as i32 + pw as i32 / 2 + d, h as i32 / 2), (ox as i32 + pw as i32 / 2, h as i32 / 2 + d)] { img[y as usize * w + x as usize] = [255, 255, 255]; } }
    }
    let mut s = format!("P6 {w} {h} 255\n").into_bytes(); for p in &img { s.extend_from_slice(p); } std::fs::write(out, s).unwrap();
}
