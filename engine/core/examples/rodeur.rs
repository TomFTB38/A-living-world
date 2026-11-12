//! Planche du Rôdeur décharné, rendue sur le processeur : vues (face, profil droit, dos, profil gauche, trois-quarts)
//! puis animations (repos, marche, course, détection, attaque, blessure, mort), et le joueur à l'échelle.
use alw_core::{actors::{Models, Range}, hostile::{species, Ai, Horde}, mesher::VSIZE, view::M4};
fn draw(img: &mut [[u8; 3]], zb: &mut [f32], w: usize, h: usize, md: &Models, list: &[(M4, Range)], yaw: f64, ox: f64) {
    let (cy, sy) = (yaw.cos(), yaw.sin()); let sc = 105.0;
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
            let lit = (0.55 + 0.45 * ((n[0] * 0.4 + n[1] * 0.7 + n[2] * 0.6) / l).abs()) as f32;
            for tri in [[0, 1, 2], [0, 2, 3]] { let (p0, p1, p2) = (pts[tri[0]], pts[tri[1]], pts[tri[2]]);
                let area = (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p1[1] - p0[1]) * (p2[0] - p0[0]); if area.abs() < 1e-6 { continue; }
                let (x0, x1) = (p0[0].min(p1[0]).min(p2[0]).max(0.0) as usize, (p0[0].max(p1[0]).max(p2[0]) as usize).min(w - 1)); let (y0, y1) = (p0[1].min(p1[1]).min(p2[1]).max(0.0) as usize, (p0[1].max(p1[1]).max(p2[1]) as usize).min(h - 1));
                for yy in y0..=y1 { for xx in x0..=x1 { let (px, py) = (xx as f32 + 0.5, yy as f32 + 0.5); let w0 = ((p1[0] - px) * (p2[1] - py) - (p1[1] - py) * (p2[0] - px)) / area; let w1 = ((p2[0] - px) * (p0[1] - py) - (p2[1] - py) * (p0[0] - px)) / area; let w2 = 1.0 - w0 - w1;
                    if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 { continue; } let z = w0 * p0[2] + w1 * p1[2] + w2 * p2[2]; let i = yy * w + xx; if z < zb[i] { zb[i] = z; img[i] = col.map(|c| (c * 2.0 * lit).min(255.0) as u8); } } } }
        }
    }
}
fn main() {
    let md = Models::build(); let (w, h) = (1900usize, 520usize); let mut img = vec![[34u8, 36, 44]; w * h]; let mut zb = vec![f32::MAX; w * h];
    let sp = species("rodeur").unwrap();
    // Rangée du haut : les cinq vues (le dessin regarde vers -z : lacet 0 = de dos, π = de face).
    let mut top = Horde::new(3); let mut x = -6.4;
    for yaw in [std::f64::consts::PI, -1.5708, 0.0, 1.5708, 2.5] { let i = top.put(sp, [x, 2.4, 0.0], yaw, false); top.foes[i].state = Ai::Repos; x += 1.45; }
    let mut bot = Horde::new(4); let mut x = -6.4;
    for (k, pose) in ["repos", "marche", "course", "detection", "attaque", "blessure", "mort"].iter().enumerate() {
        let i = bot.put(sp, [x, 0.0, 0.0], -0.75, false); let f = &mut bot.foes[i]; f.phase = 1.3 + k as f64;
        match *pose { "marche" => { f.state = Ai::Patrouille; f.speed = 0.95; } "course" => { f.state = Ai::Poursuite; f.speed = 4.6; } "detection" => { f.state = Ai::Detection; }
            "attaque" => { f.state = Ai::Combat; f.wind = 0.08; } "blessure" => { f.hit = 0.15; } "mort" => { f.dead = 1.2; } _ => {} }
        x += if *pose == "mort" { 0.0 } else { 1.35 }; }
    let mut l: Vec<(M4, Range)> = top.draws(&md).iter().map(|x| (x.m, x.r)).collect(); l.extend(bot.draws(&md).iter().map(|x| (x.m, x.r)));
    draw(&mut img, &mut zb, w, h, &md, &l, 0.0, 0.0);
    // Le joueur, pour l'échelle.
    let anim = alw_core::actors::Anim::default(); let d = anim.third(&md, [3.8, 0.0, 0.0], 3.14, 0.0, None, None, [None; 6]);
    let l: Vec<(M4, Range)> = d.iter().map(|x| (x.m, x.r)).collect(); draw(&mut img, &mut zb, w, h, &md, &l, 0.0, 300.0);
    let mut out = format!("P6 {w} {h} 255\n").into_bytes(); for p in &img { out.extend_from_slice(p); } std::fs::write(std::env::args().nth(1).unwrap(), out).unwrap();
}
