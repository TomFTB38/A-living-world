//! Interface dessinée en quads : réticule, barre d'objets, vitalité, messages. Le texte vient d'un atlas Inter.
use crate::font_data as fd;
use alw_core::{blocks::BLOCKS, session::{item, Session, INV_SIZE, ORDER}};
pub const VERT: usize = 36; // octets par sommet : position, uv, couleur, couche
#[derive(Default)]
pub struct Ui { pub data: Vec<f32>, w: f32, h: f32 }
type C = [f32; 4];
const LAYER_FLAT: f32 = -1.0; const LAYER_TEXT: f32 = -2.0;
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Hit { Slot(usize), Catalog(&'static str) }
/// Disposition de l'écran d'inventaire, partagée par le dessin et la souris.
pub fn layout(creative: bool, w: f32, h: f32) -> Vec<(Hit, [f32; 3])> {
    let k = (h / 900.0).clamp(0.8, 2.0); let c = 58.0 * k; let g = 5.0 * k; let tw = 9.0 * c + 8.0 * g; let x0 = (w - tw) * 0.5; let mut o = Vec::new();
    let top = h * 0.5 - if creative { 300.0 } else { 130.0 } * k; let mut y = top;
    if creative { for (i, id) in ORDER.iter().enumerate() { let (r, cl) = (i / 9, i % 9); o.push((Hit::Catalog(id), [x0 + cl as f32 * (c + g), top + r as f32 * (c + g), c])); } y = top + 4.0 * (c + g) + 34.0 * k; }
    for r in 0..3 { for cl in 0..9 { o.push((Hit::Slot(9 + r * 9 + cl), [x0 + cl as f32 * (c + g), y + r as f32 * (c + g), c])); } }
    for cl in 0..9 { o.push((Hit::Slot(cl), [x0 + cl as f32 * (c + g), y + 3.0 * (c + g) + 14.0 * k, c])); }
    o
}
pub fn hit_at(creative: bool, w: f32, h: f32, m: [f32; 2]) -> Option<Hit> { layout(creative, w, h).into_iter().find(|(_, r)| m[0] >= r[0] && m[1] >= r[1] && m[0] < r[0] + r[2] && m[1] < r[1] + r[2]).map(|x| x.0) }
impl Ui {
    fn inventory(&mut self, s: &Session, w: f32, h: f32, k: f32, mouse: [f32; 2]) {
        self.rect(0.0, 0.0, w, h, [0.02, 0.03, 0.03, 0.55]);
        let l = layout(s.creative, w, h); let (x0, y0) = (l[0].1[0], l[0].1[1]); let (x1, y1) = l.iter().fold((0.0f32, 0.0f32), |a, r| (a.0.max(r.1[0] + r.1[2]), a.1.max(r.1[1] + r.1[2]))); let m = 22.0 * k;
        self.rect(x0 - m, y0 - m - 30.0 * k, x1 - x0 + 2.0 * m, y1 - y0 + 2.0 * m + 30.0 * k, [0.1, 0.12, 0.1, 0.92]);
        self.text(if s.creative { "Catalogue et inventaire" } else { "Inventaire" }, x0, y0 - 36.0 * k, 22.0 * k, [1.0, 1.0, 1.0, 0.95]);
        let hover = hit_at(s.creative, w, h, mouse); let mut tip = None;
        for (hit, r) in &l {
            let on = hover == Some(*hit); self.rect(r[0], r[1], r[2], r[2], if on { [1.0, 1.0, 1.0, 0.24] } else { [0.0, 0.0, 0.0, 0.34] });
            let st = match hit { Hit::Slot(i) => s.inv[*i].map(|v| (v.item, v.n)), Hit::Catalog(id) => Some((*id, None)) };
            if let Some((id, n)) = st {
                self.icon(id, r[0] + r[2] * 0.5, r[1] + r[2] * 0.47, r[2] * 0.36);
                if let Some(n) = n { let t = n.to_string(); let z = 16.0 * k; self.text(&t, r[0] + r[2] - Ui::text_width(&t, z) - 4.0 * k, r[1] + r[2] - z - 4.0 * k, z, [1.0; 4]); }
                if on { tip = item(id).map(|i| i.label); }
            }
        }
        if let Some(c) = s.cursor { self.icon(c.item, mouse[0], mouse[1], 26.0 * k); if let Some(n) = c.n { self.text(&n.to_string(), mouse[0] + 10.0 * k, mouse[1] + 8.0 * k, 16.0 * k, [1.0; 4]); } }
        else if let Some(t) = tip { self.text(t, mouse[0] + 16.0 * k, mouse[1] + 8.0 * k, 18.0 * k, [1.0; 4]); }
    }
    fn quad(&mut self, p: [[f32; 2]; 4], uv: [[f32; 2]; 4], c: C, l: f32) { for i in [0, 1, 2, 0, 2, 3] { self.data.extend_from_slice(&[p[i][0], p[i][1], uv[i][0], uv[i][1], c[0], c[1], c[2], c[3], l]); } }
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, c: C) { self.quad([[x, y], [x + w, y], [x + w, y + h], [x, y + h]], [[0.0; 2]; 4], c, LAYER_FLAT); }
    pub fn text_width(s: &str, size: f32) -> f32 { s.chars().map(|c| fd::CHARS.chars().position(|k| k == c).map_or(10, |i| fd::ADV[i] as u32) as f32).sum::<f32>() * size / 30.0 }
    pub fn text(&mut self, s: &str, x: f32, y: f32, size: f32, c: C) {
        let (k, mut pen) = (size / 30.0, x); let (cw, ch) = (fd::CELL as f32 / fd::W as f32, fd::CELL as f32 / fd::H as f32); let cell = fd::CELL as f32 * k;
        for g in s.chars() {
            let Some(i) = fd::CHARS.chars().position(|q| q == g) else { pen += 10.0 * k; continue };
            let (u, v) = ((i as u32 % fd::COLS) as f32 * cw, (i as u32 / fd::COLS) as f32 * ch);
            for (o, col) in [(1.0, [0.0, 0.0, 0.0, 0.7 * c[3]]), (0.0, c)] { let (a, b) = (pen + o, y + o); self.quad([[a, b], [a + cell, b], [a + cell, b + cell], [a, b + cell]], [[u, v], [u + cw, v], [u + cw, v + ch], [u, v + ch]], col, LAYER_TEXT); }
            pen += fd::ADV[i] as f32 * k;
        }
    }
    /// Cube isométrique avec les vraies textures du bloc.
    fn cube(&mut self, cx: f32, cy: f32, r: f32, top: u16, side: u16, fr: u16) {
        let (w, h) = (r, r * 0.55); let l = |v: u16| v as f32; let uv = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        self.quad([[cx, cy - h - h * 0.9], [cx + w, cy - h * 0.9], [cx, cy + h - h * 0.9], [cx - w, cy - h * 0.9]], uv, [1.0, 1.0, 1.0, 1.0], l(top));
        self.quad([[cx - w, cy - h * 0.9], [cx, cy + h - h * 0.9], [cx, cy + h + r * 0.95 - h * 0.9], [cx - w, cy + r * 0.95 - h * 0.9]], uv, [0.78, 0.78, 0.78, 1.0], l(side));
        self.quad([[cx, cy + h - h * 0.9], [cx + w, cy - h * 0.9], [cx + w, cy + r * 0.95 - h * 0.9], [cx, cy + h + r * 0.95 - h * 0.9]], uv, [0.58, 0.58, 0.58, 1.0], l(fr));
    }
    fn stroke(&mut self, a: [f32; 2], b: [f32; 2], t: f32, c: C) { let (dx, dy) = (b[0] - a[0], b[1] - a[1]); let l = (dx * dx + dy * dy).sqrt().max(0.01); let (nx, ny) = (-dy / l * t, dx / l * t); self.quad([[a[0] + nx, a[1] + ny], [b[0] + nx, b[1] + ny], [b[0] - nx, b[1] - ny], [a[0] - nx, a[1] - ny]], [[0.0; 2]; 4], c, LAYER_FLAT); }
    fn icon(&mut self, id: &str, cx: f32, cy: f32, r: f32) {
        const WOOD: C = [0.55, 0.36, 0.2, 1.0]; const STEEL: C = [0.8, 0.8, 0.82, 1.0];
        match id {
            "pickaxe" => { self.stroke([cx - r * 0.7, cy + r * 0.8], [cx + r * 0.4, cy - r * 0.4], r * 0.09, WOOD); self.stroke([cx - r * 0.35, cy - r * 0.75], [cx + r * 0.85, cy + r * 0.3], r * 0.13, STEEL); }
            "axe" => { self.stroke([cx - r * 0.7, cy + r * 0.8], [cx + r * 0.45, cy - r * 0.55], r * 0.09, WOOD); self.stroke([cx - r * 0.05, cy - r * 0.7], [cx + r * 0.7, cy + r * 0.05], r * 0.3, STEEL); }
            "sword" => { self.stroke([cx - r * 0.75, cy + r * 0.75], [cx - r * 0.35, cy + r * 0.35], r * 0.09, WOOD); self.stroke([cx - r * 0.65, cy + r * 0.1], [cx - r * 0.1, cy + r * 0.65], r * 0.09, [0.85, 0.65, 0.2, 1.0]); self.stroke([cx - r * 0.35, cy + r * 0.35], [cx + r * 0.8, cy - r * 0.8], r * 0.11, STEEL); }
            _ => if let Some(b) = item(id).and_then(|i| i.block) {
                let k = BLOCKS[b as usize];
                if k.opaque || k.shape == alw_core::blocks::Shape::Leaf { self.cube(cx, cy, r * 0.8, k.dir_faces[1], k.dir_faces[0], k.dir_faces[3]); }
                else { let c = match k.name { "lantern" => [1.0, 0.75, 0.3, 1.0], "water" => [0.25, 0.5, 0.75, 1.0], "lupine" => [0.5, 0.3, 0.85, 1.0], "pink_flower" => [0.95, 0.45, 0.65, 1.0], "yellow_flower" => [0.95, 0.8, 0.2, 1.0], "daisy" => [0.95, 0.95, 0.9, 1.0], "mushroom" | "shelf_fungus" => [0.75, 0.6, 0.45, 1.0], "fence" => WOOD, "flagstone" => [0.55, 0.55, 0.52, 1.0], _ => [0.35, 0.55, 0.2, 1.0] }; self.rect(cx - r * 0.45, cy - r * 0.45, r * 0.9, r * 0.9, c); }
            }
        }
    }
    pub fn build(s: &Session, w: f32, h: f32, msg: Option<&str>, mouse: [f32; 2]) -> Ui {
        let mut u = Ui { data: Vec::with_capacity(6000), w, h }; let k = (h / 900.0).clamp(0.8, 2.0); let w1 = [1.0, 1.0, 1.0, 0.9];
        u.rect(w * 0.5 - 1.0, h * 0.5 - 8.0, 2.0, 16.0, w1); u.rect(w * 0.5 - 8.0, h * 0.5 - 1.0, 16.0, 2.0, w1);
        let cell = 62.0 * k; let gap = 5.0 * k; let total = 9.0 * cell + 8.0 * gap; let (x0, y0) = ((w - total) * 0.5, h - cell - 16.0 * k);
        for i in 0..9.min(INV_SIZE) {
            let x = x0 + i as f32 * (cell + gap); let on = i == s.sel;
            u.rect(x, y0, cell, cell, if on { [1.0, 1.0, 1.0, 0.32] } else { [0.08, 0.1, 0.09, 0.55] });
            if on { let b = 2.0 * k; u.rect(x - b, y0 - b, cell + 2.0 * b, b, w1); u.rect(x - b, y0 + cell, cell + 2.0 * b, b, w1); u.rect(x - b, y0, b, cell, w1); u.rect(x + cell, y0, b, cell, w1); }
            if let Some(st) = s.inv[i] {
                u.icon(st.item, x + cell * 0.5, y0 + cell * 0.47, cell * 0.36);
                if let (Some(n), false) = (st.n, s.creative) { let t = n.to_string(); let z = 17.0 * k; u.text(&t, x + cell - Ui::text_width(&t, z) - 5.0 * k, y0 + cell - z - 5.0 * k, z, w1); }
            }
        }
        if let Some(it) = s.held() { let z = 22.0 * k; u.text(it.label, (w - Ui::text_width(it.label, z)) * 0.5, y0 - z - 12.0 * k, z, w1); }
        if !s.creative {
            for (i, (v, c)) in [(s.vit[0], [0.88, 0.25, 0.25, 0.95]), (s.vit[1], [0.9, 0.62, 0.2, 0.95]), (s.vit[2], [0.25, 0.6, 0.9, 0.95])].into_iter().enumerate() {
                let (bw, bh, y) = (190.0 * k, 12.0 * k, h - 26.0 * k - (2 - i) as f32 * 18.0 * k); u.rect(18.0 * k, y, bw, bh, [0.0, 0.0, 0.0, 0.45]); u.rect(18.0 * k, y, bw * (v as f32 / 100.0), bh, c);
            }
        } else { let z = 18.0 * k; u.text("Mode créatif", 18.0 * k, h - 34.0 * k, z, w1); }
        if s.inv_open { u.inventory(s, w, h, k, mouse); }
        if let Some(m) = msg { let z = 20.0 * k; u.text(m, (w - Ui::text_width(m, z)) * 0.5, h * 0.16, z, w1); }
        let _ = (u.w, u.h); u
    }
}
