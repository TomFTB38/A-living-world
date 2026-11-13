//! Le village le plus proche de l'origine, maillé comme en jeu (blocs et décor fin), rendu sur le processeur :
//! couleur moyenne des matériaux, éclairage d'un soleil, tampon de profondeur. Pour vérifier les bâtiments sans carte graphique.
use alw_core::{batisse::dress, blocks::CX, gen::WorldGen, mesher::{Mesher, VSIZE}, textures::build_textures};
use std::collections::HashMap;
fn main() {
    let a: Vec<String> = std::env::args().collect(); let out = &a[1]; let seed: i32 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(20261104);
    let yaw: f64 = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.6); let dist: f64 = a.get(4).and_then(|s| s.parse().ok()).unwrap_or(34.0); let pitch: f64 = a.get(5).and_then(|s| s.parse().ok()).unwrap_or(0.42);
    let g = WorldGen::new(seed); let want = a.get(7).cloned(); let named = a.get(8).cloned(); let v = g.villages_near(0.0, 0.0, 4000.0).into_iter().find(|v| want.as_deref().is_none_or(|k| v.kind == k) && named.as_deref().is_none_or(|n| v.name == n)).expect("un village"); eprintln!("{} ({}) {:?} r {}", v.name, v.kind, v.c, v.r);
    let t = build_textures(16); let n = 16 * 16; let mut avg = vec![[0.5f64; 3]; t.layers as usize];
    for l in 0..t.layers as usize { let mut c = [0.0; 3]; for p in t.albedo[l * n * 4..(l + 1) * n * 4].chunks(4) { for k in 0..3 { c[k] += (p[k] as f64 / 255.0).powi(2); } } avg[l] = c.map(|x| x / n as f64); }
    let mut cache: HashMap<(i32, i32), std::sync::Arc<Vec<u16>>> = HashMap::new(); let mut m = Mesher::new();
    let (w, h) = (1280usize, 720usize); let mut img = vec![[0f64; 3]; w * h]; let mut zb = vec![f64::MAX; w * h];
    for p in img.iter_mut() { *p = [0.55, 0.7, 0.9]; }
    let c = match a.get(6).and_then(|s| s.parse::<usize>().ok()) { Some(i) => { let d = v.houses[i].door; eprintln!("maison {i} : {:?}", v.houses[i].craft); [d[0], d[1] + 3.0, d[2]] } None => [v.c[0] as f64 + 0.5, v.c[1] as f64 + 2.0, v.c[2] as f64 + 0.5] }; let eye = [c[0] + dist * yaw.cos() * pitch.cos(), c[1] + dist * pitch.sin(), c[2] + dist * yaw.sin() * pitch.cos()];
    let fwd = { let d = [c[0] - eye[0], c[1] - eye[1], c[2] - eye[2]]; let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt(); d.map(|x| x / l) };
    let right = { let r = [-fwd[2], 0.0, fwd[0]]; let l = (r[0] * r[0] + r[2] * r[2]).sqrt(); r.map(|x| x / l) }; let up = [right[1] * fwd[2] - right[2] * fwd[1], right[2] * fwd[0] - right[0] * fwd[2], right[0] * fwd[1] - right[1] * fwd[0]];
    let sun = { let s = [0.45f64, 0.75, 0.35]; let l = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt(); s.map(|x| x / l) }; let foc = 1.0 / (0.55f64).tan();
    let r = (v.r + 8) / CX as i32 + 1; let (kx, kz) = (v.c[0].div_euclid(CX as i32), v.c[2].div_euclid(CX as i32));
    for cz in kz - r..=kz + r { for cx in kx - r..=kx + r {
        let mut get = |x: i32, z: i32| cache.entry((x, z)).or_insert_with(|| std::sync::Arc::new(g.gen_world(x, z))).clone();
        let mut mesh = m.mesh_chunk(&mut get, cx, cz, 0); let vox = get(cx, cz); dress(&g, cx, cz, &vox, &mut mesh);
        let org = [(cx * CX as i32) as f64, 0.0, (cz * CX as i32) as f64];
        for (data, cnt) in [(&mesh.opaque, mesh.n_o), (&mesh.deco, mesh.n_d)] {
            for q in (0..cnt).step_by(4) { let mut pts = [[0f64; 3]; 4]; let mut col = [0.0; 3]; let mut nn = [0.0; 3];
                for j in 0..4 { let vv = &data[(q + j) * VSIZE..(q + j + 1) * VSIZE]; let p = [0, 2, 4].map(|i| i16::from_le_bytes([vv[i], vv[i + 1]]) as f64 / 128.0); let layer = i16::from_le_bytes([vv[6], vv[7]]) as usize;
                    let wp = [p[0] + org[0], p[1], p[2] + org[2]]; let d = [wp[0] - eye[0], wp[1] - eye[1], wp[2] - eye[2]];
                    let (xc, yc, zc) = (d[0] * right[0] + d[1] * right[1] + d[2] * right[2], d[0] * up[0] + d[1] * up[1] + d[2] * up[2], d[0] * fwd[0] + d[1] * fwd[1] + d[2] * fwd[2]);
                    pts[j] = [w as f64 * 0.5 + xc / zc.max(0.05) * foc * h as f64 * 0.5, h as f64 * 0.5 - yc / zc.max(0.05) * foc * h as f64 * 0.5, zc];
                    nn = [vv[8] as i8 as f64 / 127.0, vv[9] as i8 as f64 / 127.0, vv[10] as i8 as f64 / 127.0]; let kind = vv[15]; let tint = [vv[16], vv[17], vv[18]].map(|x| x as f64 / 127.5);
                    col = if kind == 0 || kind == 1 { let a = avg[layer.min(avg.len() - 1)]; [a[0] * tint[0], a[1] * tint[1], a[2] * tint[2]] } else { tint.map(|x| x * x * 1.1) }; }
                if pts.iter().any(|p| p[2] < 0.3) { continue; }
                let lit = 0.35 + 0.75 * (nn[0] * sun[0] + nn[1] * sun[1] + nn[2] * sun[2]).abs().max(0.0) * if nn[0] * sun[0] + nn[1] * sun[1] + nn[2] * sun[2] > 0.0 { 1.0 } else { 0.25 };
                for tri in [[0, 1, 2], [0, 2, 3]] { let (p0, p1, p2) = (pts[tri[0]], pts[tri[1]], pts[tri[2]]); let area = (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p1[1] - p0[1]) * (p2[0] - p0[0]); if area.abs() < 1e-9 { continue; }
                    let x0 = p0[0].min(p1[0]).min(p2[0]).max(0.0) as usize; let x1 = (p0[0].max(p1[0]).max(p2[0]).min(w as f64 - 1.0)).max(0.0) as usize; let y0 = p0[1].min(p1[1]).min(p2[1]).max(0.0) as usize; let y1 = (p0[1].max(p1[1]).max(p2[1]).min(h as f64 - 1.0)).max(0.0) as usize;
                    if x1 - x0 > w || y1.saturating_sub(y0) > h { continue; }
                    for yy in y0..=y1 { for xx in x0..=x1 { let (px, py) = (xx as f64 + 0.5, yy as f64 + 0.5); let w0 = ((p1[0] - px) * (p2[1] - py) - (p1[1] - py) * (p2[0] - px)) / area; let w1 = ((p2[0] - px) * (p0[1] - py) - (p2[1] - py) * (p0[0] - px)) / area; let w2 = 1.0 - w0 - w1;
                        if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 { continue; } let z = 1.0 / (w0 / p0[2] + w1 / p1[2] + w2 / p2[2]); let i = yy * w + xx; if z < zb[i] { zb[i] = z; img[i] = col.map(|c| c * lit); } } } } }
        }
    } }
    let mut o = format!("P6 {w} {h} 255\n").into_bytes(); for p in &img { for k in 0..3 { o.push((p[k].max(0.0).powf(1.0 / 2.2) * 255.0).min(255.0) as u8); } } std::fs::write(out, o).unwrap();
}
