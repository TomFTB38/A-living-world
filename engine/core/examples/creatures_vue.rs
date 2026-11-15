//! Planche de créatures assemblées (ennemis par identifiant d'espèce, ou « glowbug »/« bat » pour les bêtes), rendues sur le processeur.
//! Usage : creatures_vue sortie.ppm araignee golem golem:affut magma glowbug bat
use alw_core::{actors::Models, hostile as h, mesher::VSIZE, wild::{Kind, Wild}};
fn main() {
    let args: Vec<String> = std::env::args().skip(2).collect(); let md = Models::build();
    let z: f64 = std::env::var("ZOOM").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0); let (cw, w, h, sc) = (300usize, args.len() * 300, 460usize, 120.0 * z); let mut img = vec![[52u8, 58, 66]; w * h]; let mut zb = vec![f64::MAX; w * h];
    for (i, a) in args.iter().enumerate() {
        let (id, st) = a.split_once(':').unwrap_or((a, "")); let mut scale = sc;
        let draws = match id { "glowbug" | "bat" => { let mut wd = Wild::new(1); wd.put(if id == "bat" { Kind::Bat } else { Kind::Glowbug }, [0.0, 0.0, 0.0], 0.0, id == "bat"); scale = sc * 3.0; wd.draws(&md) }
            _ => { let mut hd = h::Horde::new(1); let k = hd.put(h::species(id).unwrap_or_else(|| panic!("{id} ?")), [0.0, 0.0, 0.0], 0.0, false); hd.foes[k].state = if st == "affut" { h::Ai::Embuscade } else { h::Ai::Repos };
                if st == "coup" { hd.foes[k].state = h::Ai::Combat; hd.foes[k].wind = 0.05; } if st == "vise" { hd.foes[k].state = h::Ai::Combat; hd.foes[k].wind = 0.3; } if st == "sorti" { hd.foes[k].up = 1.0; } if st == "pique" { hd.foes[k].air = true; } if st == "marche" { hd.foes[k].speed = 2.0; hd.foes[k].phase = 1.2; } if id == "magma" { hd.embers.push(([0.9, 0.0, 0.6], 4.0)); } hd.draws(&md) } };
        let (yaw, pit) = (0.75f64, 0.35f64); let ox = cw as f64 * (i as f64 + 0.5);
        for d in draws { let (st0, n) = (d.r.first as usize / 6 * 4, d.r.count as usize / 6 * 4); let m = d.m;
            for q in (st0..st0 + n).step_by(4) { let mut pts = [[0f64; 3]; 4]; let mut col = [0f64; 3]; let mut nn = [0f64; 3];
                for k in 0..4 { let v = &md.data[(q + k) * VSIZE..(q + k + 1) * VSIZE]; let l = [0, 2, 4].map(|j| i16::from_le_bytes([v[j], v[j + 1]]) as f64 / alw_core::mesher::POSK);
                    let tr = |x: f64, y: f64, z: f64, t: f64| [m[0] as f64 * x + m[4] as f64 * y + m[8] as f64 * z + m[12] as f64 * t, m[1] as f64 * x + m[5] as f64 * y + m[9] as f64 * z + m[13] as f64 * t, m[2] as f64 * x + m[6] as f64 * y + m[10] as f64 * z + m[14] as f64 * t];
                    let p = tr(l[0], l[1], l[2], 1.0); let n0 = tr(v[8] as i8 as f64, v[9] as i8 as f64, v[10] as i8 as f64, 0.0); let nl = (n0[0] * n0[0] + n0[1] * n0[1] + n0[2] * n0[2]).sqrt().max(1e-9); nn = [n0[0] / nl, n0[1] / nl, n0[2] / nl];
                    let (x1, z1) = (p[0] * yaw.cos() - p[2] * yaw.sin(), p[0] * yaw.sin() + p[2] * yaw.cos()); let (y2, z2) = (p[1] * pit.cos() - z1 * pit.sin(), p[1] * pit.sin() + z1 * pit.cos());
                    pts[k] = [ox + x1 * scale, h as f64 - 70.0 - y2 * scale, -z2]; col = [v[16] as f64 / 127.5, v[17] as f64 / 127.5, v[18] as f64 / 127.5]; }
                let lit = 0.55 + 0.45 * (nn[1] * 0.7 + nn[0] * 0.3 + nn[2] * 0.4).max(0.0);
                for t in [[0, 1, 2], [0, 2, 3]] { let (a, b, c) = (pts[t[0]], pts[t[1]], pts[t[2]]); let ar = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]); if ar.abs() < 1e-9 { continue; }
                    for yy in a[1].min(b[1]).min(c[1]).max(0.0) as usize..=(a[1].max(b[1]).max(c[1]).max(0.0) as usize).min(h - 1) { for xx in a[0].min(b[0]).min(c[0]).max(0.0) as usize..=(a[0].max(b[0]).max(c[0]).max(0.0) as usize).min(w - 1) {
                        let (px, py) = (xx as f64 + 0.5, yy as f64 + 0.5); let w0 = ((b[0] - px) * (c[1] - py) - (b[1] - py) * (c[0] - px)) / ar; let w1 = ((c[0] - px) * (a[1] - py) - (c[1] - py) * (a[0] - px)) / ar; let w2 = 1.0 - w0 - w1; if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 { continue; }
                        let z = w0 * a[2] + w1 * b[2] + w2 * c[2]; let i2 = yy * w + xx; if z < zb[i2] { zb[i2] = z; img[i2] = col.map(|c| ((c * c * 1.1 * lit).sqrt().min(1.0) * 255.0) as u8); } } } } } }
    }
    let mut out = format!("P6 {w} {h} 255\n").into_bytes(); for p in &img { out.extend_from_slice(p); } std::fs::write(std::env::args().nth(1).unwrap(), out).unwrap();
}
