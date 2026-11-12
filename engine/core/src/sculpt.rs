//! Sculpture des êtres vivants : des volumes galbés, faits de tranches de boîtes, chaque face peinte en petits pixels
//! d'une matière (poil, plume, écaille, peau, étoffe…). La même méthode que les dauphins et les requins, en outils :
//! un corps allongé selon sa section, un ovoïde, un membre effilé entre deux points, une colonne verticale à section
//! arrondie (torse, crâne), une boîte aux arêtes abattues.
use crate::hostile::{paint, Mat};
use crate::mesher::VBuf;

pub(crate) type C = [f64; 3];
pub(crate) fn mix(a: C, b: C, t: f64) -> C { [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t] }
pub(crate) fn dk(c: C, k: f64) -> C { [c[0] * k, c[1] * k, c[2] * k] }
fn lerp(a: f64, b: f64, t: f64) -> f64 { a + (b - a) * t }

/// Le ciseau : un maillage, une matière, une taille de pixel.
pub(crate) struct Pen<'a> { pub q: &'a mut VBuf, pub m: Mat, pub pix: f64 }
impl<'a> Pen<'a> {
    pub fn new(q: &'a mut VBuf, m: Mat, pix: f64) -> Self { Pen { q, m, pix } }
    /// Une boîte de la matière courante.
    pub fn bx(&mut self, a: [f64; 6], c: C) { paint(self.q, a, c, self.m, self.pix) }
    /// Une boîte d'une autre matière (yeux unis, corne, cuir…).
    pub fn bm(&mut self, a: [f64; 6], c: C, m: Mat) { paint(self.q, a, c, m, self.pix) }
    /// Deux boîtes croisées : une section aux coins abattus. `round` : 0.6 bien rond, 0.85 un rectangle adouci.
    fn cross(&mut self, c: [f64; 3], h: [f64; 3], axis: usize, col: C, m: Mat, round: f64) {
        let (u, v) = match axis { 0 => (1, 2), 1 => (0, 2), _ => (0, 1) };
        let mut a = [c[0] - h[0], c[1] - h[1], c[2] - h[2], c[0] + h[0], c[1] + h[1], c[2] + h[2]];
        let mut b2 = a; a[v] = c[v] - h[v] * round; a[v + 3] = c[v] + h[v] * round; b2[u] = c[u] - h[u] * round; b2[u + 3] = c[u] + h[u] * round;
        let e = (h[axis] * 0.02).min(0.002); b2[axis] += e; b2[axis + 3] -= e; // l'autre boîte, à peine en retrait : pas de faces confondues
        paint(self.q, a, col, m, self.pix); paint(self.q, b2, col, m, self.pix);
    }
    /// Un ovoïde de centre `c` et de demi-axes `r`, du dessus `top` au dessous `bot` (le dos sombre, le ventre clair).
    pub fn blob(&mut self, c: [f64; 3], r: [f64; 3], top: C, bot: C) { let m = self.m; self.blob_m(c, r, top, bot, m) }
    pub fn blob_m(&mut self, c: [f64; 3], r: [f64; 3], top: C, bot: C, m: Mat) {
        let n = ((2.0 * r[1] / (self.pix * 1.1)).round() as usize).clamp(3, 9);
        for k in 0..n { let (t0, t1) = (k as f64 / n as f64, (k + 1) as f64 / n as f64); let tm = (t0 + t1) - 1.0; let s = (1.0 - tm * tm * 0.92).max(0.05).sqrt();
            let y0 = c[1] - r[1] + 2.0 * r[1] * t0; let y1 = c[1] - r[1] + 2.0 * r[1] * t1; let col = mix(bot, top, (tm + 1.0) * 0.5);
            self.cross([c[0], (y0 + y1) * 0.5, c[2]], [r[0] * s, (y1 - y0) * 0.5, r[2] * s], 1, col, m, 0.64); }
    }
    /// Un membre effilé de `a` à `b`, de rayons `ra` à `rb` (les deux axes perpendiculaires, dans l'ordre x, y, z), d'une couleur à l'autre.
    pub fn tube(&mut self, a: [f64; 3], b: [f64; 3], ra: [f64; 2], rb: [f64; 2], ca: C, cb: C) { let m = self.m; self.tube_m(a, b, ra, rb, ca, cb, m) }
    #[allow(clippy::too_many_arguments)]
    pub fn tube_m(&mut self, a: [f64; 3], b: [f64; 3], ra: [f64; 2], rb: [f64; 2], ca: C, cb: C, m: Mat) {
        let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]]; let ax = if d[0].abs() >= d[1].abs() && d[0].abs() >= d[2].abs() { 0 } else if d[1].abs() >= d[2].abs() { 1 } else { 2 };
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt(); let n = ((len / (self.pix * 1.2)).round() as usize).clamp(1, 14);
        let (u, v) = match ax { 0 => (1, 2), 1 => (0, 2), _ => (0, 1) };
        for k in 0..n { let (t0, t1) = (k as f64 / n as f64, (k + 1) as f64 / n as f64); let tm = (t0 + t1) * 0.5;
            let c = [lerp(a[0], b[0], tm), lerp(a[1], b[1], tm), lerp(a[2], b[2], tm)]; let mut h = [0.0; 3];
            h[ax] = (d[ax] * (t1 - t0)).abs() * 0.5 + self.pix * 0.08; h[u] = lerp(ra[0], rb[0], tm); h[v] = lerp(ra[1], rb[1], tm);
            self.cross(c, h, ax, mix(ca, cb, tm), m, 0.64); }
    }
    /// Un corps allongé selon z : des tranches (z, demi-largeur, dessus, dessous), adoucies entre elles, chaque tranche
    /// faite de couches dont la largeur suit une ellipse et la couleur passe du dos au flanc puis au ventre.
    pub fn body(&mut self, prof: &[(f64, f64, f64, f64)], back: C, side: C, belly: C) {
        let m = self.m;
        for w in prof.windows(2) {
            let (p0, p1) = (w[0], w[1]); let steps = (((p1.0 - p0.0).abs() / (self.pix * 1.25)).round() as usize).max(1);
            for s in 0..steps { let (t0, t1) = (s as f64 / steps as f64, (s + 1) as f64 / steps as f64); let tm = (t0 + t1) * 0.5;
                let (za, zb) = (lerp(p0.0, p1.0, t0), lerp(p0.0, p1.0, t1)); let (z0, z1) = (za.min(zb), za.max(zb));
                let hw = lerp(p0.1, p1.1, tm); let top = lerp(p0.2, p1.2, tm); let bot = lerp(p0.3, p1.3, tm);
                let layers = (((top - bot) / (self.pix * 0.9)).round() as usize).clamp(3, 7);
                for l in 0..layers { let (u0, u1) = (l as f64 / layers as f64, (l + 1) as f64 / layers as f64); let um = (u0 + u1) - 1.0;
                    let wk = (1.0 - um * um * 0.86).max(0.1).sqrt(); let y0 = lerp(bot, top, u0); let y1 = lerp(bot, top, u1);
                    let k = (um + 1.0) * 0.5; let col = if k > 0.6 { mix(side, back, ((k - 0.6) / 0.4).min(1.0)) } else if k < 0.3 { mix(belly, side, (k / 0.3).min(1.0)) } else { side };
                    paint(self.q, [-hw * wk, y0, z0, hw * wk, y1, z1], col, m, self.pix); }
            }
        }
    }
    /// Une colonne verticale (torse, crâne, jupe) : des tranches (y, demi-largeur, demi-profondeur), section adoucie.
    pub fn column(&mut self, prof: &[(f64, f64, f64)], dz: f64, c: C, round: f64) { let m = self.m; self.column_m(prof, dz, c, c, round, m) }
    #[allow(clippy::too_many_arguments)]
    pub fn column_m(&mut self, prof: &[(f64, f64, f64)], dz: f64, bot_c: C, top_c: C, round: f64, m: Mat) {
        let (ylo, yhi) = (prof[0].0, prof[prof.len() - 1].0);
        for w in prof.windows(2) {
            let (p0, p1) = (w[0], w[1]); let steps = (((p1.0 - p0.0).abs() / (self.pix * 1.3)).round() as usize).max(1);
            for s in 0..steps { let (t0, t1) = (s as f64 / steps as f64, (s + 1) as f64 / steps as f64); let tm = (t0 + t1) * 0.5;
                let (y0, y1) = (lerp(p0.0, p1.0, t0), lerp(p0.0, p1.0, t1)); let ym = (y0 + y1) * 0.5;
                let col = mix(bot_c, top_c, ((ym - ylo) / (yhi - ylo).max(1e-6)).clamp(0.0, 1.0));
                self.cross([0.0, ym, dz], [lerp(p0.1, p1.1, tm), (y1 - y0).abs() * 0.5, lerp(p0.2, p1.2, tm)], 1, col, m, round); }
        }
    }
    /// Une boîte aux arêtes abattues : trois boîtes croisées.
    pub fn rbox(&mut self, a: [f64; 6], c: C) { let m = self.m; rbox(self.q, a, c, m, self.pix) }
    pub fn rbox_m(&mut self, a: [f64; 6], c: C, m: Mat) { rbox(self.q, a, c, m, self.pix) }
}
/// Une boîte adoucie : ses douze arêtes sont abattues d'un cran, si elle est assez épaisse pour cela.
pub(crate) fn rbox(q: &mut VBuf, a: [f64; 6], c: C, m: Mat, pix: f64) {
    let (dx, dy, dz) = (a[3] - a[0], a[4] - a[1], a[5] - a[2]); let mn = dx.min(dy).min(dz);
    if mn < pix * 1.5 || m == Mat::Plain { paint(q, a, c, m, pix); return; }
    let e = (mn * 0.2).min(pix * 0.75); let f = 0.0015;
    paint(q, [a[0] + e, a[1], a[2] + e, a[3] - e, a[4], a[5] - e], c, m, pix);
    paint(q, [a[0], a[1] + e, a[2] + e + f, a[3], a[4] - e, a[5] - e - f], c, m, pix);
    paint(q, [a[0] + e + f, a[1] + e + f, a[2], a[3] - e - f, a[4] - e - f, a[5]], c, m, pix);
}
