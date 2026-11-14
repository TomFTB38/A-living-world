//! La vue à la première personne (main, outil, bouclier) rendue sur le processeur, au repos et en garde.
use alw_core::{actors::Models, mesher::VSIZE};
fn main() {
    let md = Models::build(); let (w, h) = (960usize, 270usize); let mut img = vec![[90u8, 120, 150]; w * h]; let mut zb = vec![f32::MAX; w * h];
    let mut a = alw_core::actors::Anim::default(); let id: alw_core::view::M4 = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    for (k, g) in [0.0, 1.0].iter().enumerate() {
        a.guard = *g; let held = md.held("sword"); let d = a.first(&md, &id, held, Some("iron"), Some("steel"));
        for dr in d { let m = dr.m.map(|x| x as f64); let (s0, s1) = (dr.r.first as usize / 6 * 4, (dr.r.first + dr.r.count) as usize / 6 * 4);
            for q in (s0..s1).step_by(4) { let mut pts = Vec::new(); let mut col = [0f32; 3];
                for j in 0..4 { let v = &md.data[(q + j) * VSIZE..(q + j + 1) * VSIZE]; let p = [0, 2, 4].map(|i| i16::from_le_bytes([v[i], v[i + 1]]) as f64 / 128.0);
                    let x = m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12]; let y = m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13]; let z = m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14];
                    let f = 1.0 / (0.5f64).tan() / -z.min(-0.01); pts.push([(k as f64 * 480.0 + 240.0 + x * f * 135.0) as f32, (135.0 - y * f * 135.0) as f32, -z as f32]); col = [v[16] as f32, v[17] as f32, v[18] as f32]; }
                for tri in [[0, 1, 2], [0, 2, 3]] { let (p0, p1, p2) = (pts[tri[0]], pts[tri[1]], pts[tri[2]]); let area = (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p1[1] - p0[1]) * (p2[0] - p0[0]); if area.abs() < 1e-6 { continue; }
                    let x0 = (p0[0].min(p1[0]).min(p2[0]).max(k as f32 * 480.0)) as usize; let x1 = (p0[0].max(p1[0]).max(p2[0]) as usize).min(k * 480 + 479); let y0 = p0[1].min(p1[1]).min(p2[1]).max(0.0) as usize; let y1 = (p0[1].max(p1[1]).max(p2[1]) as usize).min(h - 1);
                    for yy in y0..=y1 { for xx in x0..=x1 { let (px, py) = (xx as f32 + 0.5, yy as f32 + 0.5); let w0 = ((p1[0] - px) * (p2[1] - py) - (p1[1] - py) * (p2[0] - px)) / area; let w1 = ((p2[0] - px) * (p0[1] - py) - (p2[1] - py) * (p0[0] - px)) / area; let w2 = 1.0 - w0 - w1;
                        if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 { continue; } let z = w0 * p0[2] + w1 * p1[2] + w2 * p2[2]; let i = yy * w + xx; if z < zb[i] { zb[i] = z; img[i] = col.map(|c| (c * 2.0).min(255.0) as u8); } } } } } }
    }
    let mut out = format!("P6 {w} {h} 255\n").into_bytes(); for p in &img { out.extend_from_slice(p); } std::fs::write(std::env::args().nth(1).unwrap(), out).unwrap();
}
