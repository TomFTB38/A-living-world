//! Rend la planche d'icônes sur le processeur (couleurs plates, sans textures) pour vérifier leur sens.
use alw_core::{actors::Models, mesher::VSIZE, session::order};
fn main() {
    let md = Models::build(); let ids = order(); let cell = 48usize; let cols = 16usize; let rows = ids.len().div_ceil(cols);
    let (w, h) = (cols * cell, rows * (cell + 12)); let mut img = vec![[40u8, 44, 40]; w * h];
    for (n, id) in ids.iter().enumerate() {
        let Some(hd) = md.held(id) else { continue }; let m = md.icon_matrix(id); let (ox, oy) = ((n % cols) * cell, (n / cols) * (cell + 12));
        let mut zb = vec![f32::MAX; cell * cell];
        let (a, b) = (hd.r.first as usize / 6 * 4, (hd.r.first + hd.r.count) as usize / 6 * 4);
        for q in (a..b).step_by(4) {
            let mut pts = Vec::new(); let mut col = [0f32; 3];
            for k in 0..4 { let v = &md.data[(q + k) * VSIZE..(q + k + 1) * VSIZE]; let p = [0, 2, 4].map(|j| i16::from_le_bytes([v[j], v[j + 1]]) as f32 / 128.0);
                let x = m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12]; let y = m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13]; let z = m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14];
                pts.push([(x * 0.5 + 0.5) * cell as f32, (0.5 - y * 0.5) * cell as f32, z]); col = [v[16] as f32, v[17] as f32, v[18] as f32]; let _ = v[15]; }
            let shade = { let e1 = [pts[1][0] - pts[0][0], pts[1][1] - pts[0][1]]; let e2 = [pts[2][0] - pts[0][0], pts[2][1] - pts[0][1]]; let _ = e1[0] * e2[1] - e1[1] * e2[0]; 1.0 };
            for tri in [[0, 1, 2], [0, 2, 3]] {
                let (p0, p1, p2) = (pts[tri[0]], pts[tri[1]], pts[tri[2]]);
                let (x0, x1) = (p0[0].min(p1[0]).min(p2[0]).floor().max(0.0) as usize, (p0[0].max(p1[0]).max(p2[0]).ceil() as usize).min(cell - 1));
                let (y0, y1) = (p0[1].min(p1[1]).min(p2[1]).floor().max(0.0) as usize, (p0[1].max(p1[1]).max(p2[1]).ceil() as usize).min(cell - 1));
                let area = (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p1[1] - p0[1]) * (p2[0] - p0[0]); if area.abs() < 1e-6 { continue; }
                for yy in y0..=y1 { for xx in x0..=x1 { let (px, py) = (xx as f32 + 0.5, yy as f32 + 0.5);
                    let w0 = ((p1[0] - px) * (p2[1] - py) - (p1[1] - py) * (p2[0] - px)) / area; let w1 = ((p2[0] - px) * (p0[1] - py) - (p2[1] - py) * (p0[0] - px)) / area; let w2 = 1.0 - w0 - w1;
                    if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 { continue; } let z = w0 * p0[2] + w1 * p1[2] + w2 * p2[2]; let i = yy * cell + xx; if z < zb[i] { zb[i] = z; let c = col.map(|c| (c * 2.0 * shade).min(255.0) as u8); img[(oy + yy) * w + ox + xx] = c; } } }
            }
        }
    }
    let mut out = format!("P6 {w} {h} 255\n").into_bytes(); for p in &img { out.extend_from_slice(p); } std::fs::write(std::env::args().nth(1).unwrap_or("icones.ppm".into()), out).unwrap();
    for (n, id) in ids.iter().enumerate() { if n % cols == 0 { print!("\n{:3}: ", n / cols); } print!("{id} "); } println!();
}
