//! Planche des nouveaux objets de port, dessinés comme en jeu, rendus sur le processeur.
use alw_core::{blocks::*, mesher::{VBuf, VSIZE}, noise::Rng};
fn main() {
    let names = ["fishing_net", "fish_rack", "creel", "mooring_post", "buoy", "net_pile", "anchor", "oars", "rope_rail", "sail", "pennant", "fish_crate"];
    let (w, h, sc) = (1440usize, 300usize, 100.0); let mut img = vec![[52u8, 58, 66]; w * h]; let mut zb = vec![f64::MAX; w * h];
    for (i, n) in names.iter().enumerate() { let mut o = VBuf::new(4000); let mut r = Rng::new(3); alw_core::meubles::draw(&mut o, n, n, 0.0, 0.0, 0.0, 255.0, 0.0, &mut r);
        let (yaw, pit) = (0.6f64, 0.35f64); let ox = 60.0 + i as f64 * 115.0;
        for q in (0..o.n).step_by(4) { let mut pts = [[0f64; 3]; 4]; let mut col = [0f64; 3]; let mut nn = [0f64; 3];
            for k in 0..4 { let v = &o.data[(q + k) * VSIZE..(q + k + 1) * VSIZE]; let p = [0, 2, 4].map(|j| i16::from_le_bytes([v[j], v[j + 1]]) as f64 / o.scale - 0.5); let p = [p[0], p[1] + 0.5, p[2]];
                let (x1, z1) = (p[0] * yaw.cos() - p[2] * yaw.sin(), p[0] * yaw.sin() + p[2] * yaw.cos()); let (y2, z2) = (p[1] * pit.cos() - z1 * pit.sin(), p[1] * pit.sin() + z1 * pit.cos());
                pts[k] = [ox + x1 * sc, h as f64 - 60.0 - y2 * sc, z2]; col = [v[16] as f64 / 127.5, v[17] as f64 / 127.5, v[18] as f64 / 127.5]; nn = [v[8] as i8 as f64 / 127.0, v[9] as i8 as f64 / 127.0, v[10] as i8 as f64 / 127.0]; }
            let lit = 0.55 + 0.45 * (nn[1] * 0.7 + nn[0] * 0.4 - nn[2] * 0.3).max(0.0);
            for t in [[0, 1, 2], [0, 2, 3]] { let (a, b, c) = (pts[t[0]], pts[t[1]], pts[t[2]]); let ar = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]); if ar.abs() < 1e-9 { continue; }
                for yy in a[1].min(b[1]).min(c[1]).max(0.0) as usize..=(a[1].max(b[1]).max(c[1]) as usize).min(h - 1) { for xx in a[0].min(b[0]).min(c[0]).max(0.0) as usize..=(a[0].max(b[0]).max(c[0]) as usize).min(w - 1) {
                    let (px, py) = (xx as f64 + 0.5, yy as f64 + 0.5); let w0 = ((b[0] - px) * (c[1] - py) - (b[1] - py) * (c[0] - px)) / ar; let w1 = ((c[0] - px) * (a[1] - py) - (c[1] - py) * (a[0] - px)) / ar; let w2 = 1.0 - w0 - w1; if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 { continue; }
                    let z = w0 * a[2] + w1 * b[2] + w2 * c[2]; let i2 = yy * w + xx; if z < zb[i2] { zb[i2] = z; img[i2] = col.map(|c| ((c * c * 1.1 * lit).sqrt().min(1.0) * 255.0) as u8); } } } } } }
    let _ = BLOCKS.len(); let mut out = format!("P6 {w} {h} 255\n").into_bytes(); for p in &img { out.extend_from_slice(p); } std::fs::write(std::env::args().nth(1).unwrap(), out).unwrap();
}
