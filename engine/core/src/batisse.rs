//! Bâtisse : le décor fin des villages, plus petit qu'un bloc. Toits en pente, colombages, enseignes, auvents, ailes de
//! moulin : des boîtes orientées (ou des faces) posées au dixième de bloc, maillées avec le chunk de leur bloc d'appui.
//! Si l'on casse ce bloc, le décor qu'il porte s'en va avec lui.
use crate::blocks::{Id, by_name, idx, BLOCKS, CH, CX};
use crate::gen::WorldGen;
use crate::mesher::{obox, Mesh, VBuf};

pub type C3 = [f64; 3];
/// Un élément de décor : boîte orientée (centre, trois demi-axes) ou face (quatre coins, `quad`), couleur unie
/// (`layer` < 0) ou texture d'un matériau teintée par `col`. `rot` : sens du motif (1 tourné, 2 fixe).
#[derive(Clone, Debug)]
pub struct Prop { pub at: [i32; 3], pub c: C3, pub a: [C3; 3], pub col: C3, pub layer: f64, pub rot: u8, pub quad: bool }

pub(crate) fn id(n: &str) -> Id { by_name(n).unwrap_or_else(|| panic!("bloc inconnu : {n}")) }
/// La couche de texture d'un bloc plein, pour habiller le décor de la même matière.
pub fn layer(b: Id) -> f64 { BLOCKS[b as usize].faces[0] as f64 }
fn add(a: C3, b: C3) -> C3 { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn sub(a: C3, b: C3) -> C3 { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn mulv(a: C3, k: f64) -> C3 { [a[0] * k, a[1] * k, a[2] * k] }
fn dot(a: C3, b: C3) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn cross(a: C3, b: C3) -> C3 { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn unit(a: C3) -> C3 { let m = dot(a, a).sqrt().max(1e-9); mulv(a, 1.0 / m) }

/// Le terrain d'un bâtiment : la case d'origine, et les directions de la largeur (u) et de la profondeur (w).
/// En coordonnées continues, la case (0, 0) couvre u ∈ [0, 1], w ∈ [0, 1].
#[derive(Clone, Copy, Debug)]
pub struct Lot { pub o: (i32, i32), pub u: (i32, i32), pub w: (i32, i32) }
impl Lot {
    pub fn cell(&self, u: i32, y: i32, ww: i32) -> [i32; 3] { [self.o.0 + u * self.u.0 + ww * self.w.0, y, self.o.1 + u * self.u.1 + ww * self.w.1] }
    pub fn p(&self, u: f64, y: f64, ww: f64) -> C3 {
        let (cx, cz) = (self.o.0 as f64 + 0.5, self.o.1 as f64 + 0.5);
        [cx + (u - 0.5) * self.u.0 as f64 + (ww - 0.5) * self.w.0 as f64, y, cz + (u - 0.5) * self.u.1 as f64 + (ww - 0.5) * self.w.1 as f64]
    }
    pub fn v(&self, du: f64, dy: f64, dw: f64) -> C3 { [du * self.u.0 as f64 + dw * self.w.0 as f64, dy, du * self.u.1 as f64 + dw * self.w.1 as f64] }
    /// Sens du motif d'une texture vue de dessus, pour que les rangs de tuiles suivent la pente le long de u.
    pub fn grain(&self) -> u8 { if self.u.0 != 0 { 1 } else { 2 } }
}

/// Ce que l'on pose : des blocs, et le décor fin.
#[derive(Default)]
pub struct Work { pub blocks: Vec<(i32, i32, i32, Id)>, pub props: Vec<Prop> }
impl Work {
    pub fn put(&mut self, p: [i32; 3], b: Id) { self.blocks.push((p[0], p[1], p[2], b)); }
    /// Boîte orientée (le repère est remis dans le sens direct s'il le faut : la boîte est symétrique).
    pub fn solid(&mut self, at: [i32; 3], c: C3, mut a: [C3; 3], col: C3, layer: f64, rot: u8) {
        if dot(cross(a[0], a[1]), a[2]) < 0.0 { a[2] = mulv(a[2], -1.0); }
        self.props.push(Prop { at, c, a, col, layer, rot, quad: false });
    }
    /// Une pièce de bois (ou de pierre) de `p0` à `p1` : demi-largeur `hw`, demi-épaisseur `ht` dans la direction `side`.
    #[allow(clippy::too_many_arguments)]
    pub fn bar(&mut self, at: [i32; 3], p0: C3, p1: C3, side: C3, hw: f64, ht: f64, col: C3, layer: f64) {
        let d = sub(p1, p0); let a0 = mulv(d, 0.5); let s = sub(side, mulv(d, dot(side, d) / dot(d, d).max(1e-9)));
        let a2 = mulv(unit(s), ht); let a1 = mulv(unit(cross(a2, a0)), hw);
        self.solid(at, mulv(add(p0, p1), 0.5), [a0, a1, a2], col, layer, 0);
    }
    /// Une boîte alignée sur le monde.
    pub fn cube(&mut self, at: [i32; 3], lo: C3, hi: C3, col: C3, layer: f64) {
        let h = mulv(sub(hi, lo), 0.5); self.solid(at, mulv(add(lo, hi), 0.5), [[h[0], 0.0, 0.0], [0.0, h[1], 0.0], [0.0, 0.0, h[2]]], col, layer, 0);
    }
    /// Une face à quatre coins (un triangle si les deux derniers se confondent), visible des deux côtés.
    pub fn quad(&mut self, at: [i32; 3], p: [C3; 4], col: C3, layer: f64, rot: u8) { self.props.push(Prop { at, c: p[0], a: [p[1], p[2], p[3]], col, layer, rot, quad: true }); }
}

/// Habille un chunk maillé du décor des villages proches dont le bloc d'appui est dans ce chunk et tient encore.
pub fn dress(gen: &WorldGen, cx: i32, cz: i32, vox: &[Id], mesh: &mut Mesh) {
    let (ox, oz) = (cx * CX as i32, cz * CX as i32); let span = crate::village::SPAN; let cell = crate::village::CELL;
    let mut o = VBuf { data: std::mem::take(&mut mesh.deco), n: mesh.n_d, scale: crate::mesher::POSK, rot: 0 }; let mut top = mesh.ymax;
    for gx in (ox - span).div_euclid(cell)..=(ox + CX as i32 + span).div_euclid(cell) { for gz in (oz - span).div_euclid(cell)..=(oz + CX as i32 + span).div_euclid(cell) {
        let Some(s) = gen.site(gx, gz) else { continue };
        for p in s.props() {
            let (lx, lz) = (p.at[0] - ox, p.at[2] - oz); if lx < 0 || lz < 0 || lx >= CX as i32 || lz >= CX as i32 || p.at[1] < 0 || p.at[1] >= CH as i32 { continue; }
            if vox.get(idx(lx as usize, p.at[1] as usize, lz as usize)).copied().unwrap_or(0) == 0 { continue; }
            let off = [ox as f64, 0.0, oz as f64]; o.rot = p.rot;
            if p.quad {
                let q = [sub(p.c, off), sub(p.a[0], off), sub(p.a[1], off), sub(p.a[2], off)];
                let n = unit(cross(sub(q[1], q[0]), sub(q[3], q[0]))); let (kd, ly) = if p.layer < 0.0 { (crate::mesher::kind::FLAT, 0.0) } else { (crate::mesher::kind::TEX, p.layer) };
                for (order, sg) in [([0, 1, 2, 3], 1.0), ([0, 3, 2, 1], -1.0)] { for k in order { let v = q[k]; o.v(v[0], v[1], v[2], ly, n[0] * sg, n[1] * sg, n[2] * sg, 0.0, 235.0, 255.0, 0.0, kd, p.col[0], p.col[1], p.col[2]); } }
                for v in q { top = top.max(v[1].ceil() as i32 + 1); }
            } else {
                obox(&mut o, sub(p.c, off), p.a, p.col, p.layer, 255.0, 0.0);
                top = top.max((p.c[1] + p.a[0][1].abs() + p.a[1][1].abs() + p.a[2][1].abs()).ceil() as i32 + 1);
            }
            o.rot = 0;
        }
    } }
    mesh.deco = o.data; mesh.n_d = o.n; mesh.ymax = top;
}

// ------------------------------------------------------------------------------------------------ styles et plans
/// Les matières d'un village : soubassement, rez-de-chaussée, étage, poteaux, sol, âme des toits, couvertures.
#[derive(Clone, Copy)]
pub struct Style { pub found: Id, pub ground: Id, pub upper: Id, pub frame: bool, pub post: Id, pub floor: Id, pub core: Id, pub tiles: Id, pub thatch: Id, pub noble: Id, pub chimney: Id, pub pave: Id, pub beam: C3 }
pub fn style(kind: &str) -> Style {
    let (found, cob, plaster, timber, planks, tiles, slate, thatch) = (id("cobblestone"), id("cobblestone"), id("plaster"), id("timber"), id("planks"), id("roof_tiles"), id("slate"), id("thatch"));
    let dark = [0.25, 0.16, 0.1];
    match kind {
        "taiga" => Style { found, ground: cob, upper: id("spruce_planks"), frame: false, post: id("spruce_log"), floor: id("spruce_planks"), core: id("spruce_planks"), tiles: slate, thatch, noble: slate, chimney: cob, pave: cob, beam: [0.2, 0.14, 0.09] },
        "desert" => Style { found: id("sandstone"), ground: id("sandstone"), upper: plaster, frame: false, post: id("red_sandstone"), floor: id("sandstone"), core: planks, tiles, thatch, noble: tiles, chimney: id("sandstone"), pave: id("sandstone"), beam: [0.36, 0.24, 0.13] },
        "aride" => Style { found, ground: cob, upper: plaster, frame: true, post: timber, floor: planks, core: planks, tiles, thatch, noble: tiles, chimney: cob, pave: cob, beam: dark },
        "montagne" | "hautes_terres" => Style { found, ground: id("stone_bricks"), upper: plaster, frame: true, post: timber, floor: planks, core: planks, tiles: slate, thatch: slate, noble: slate, chimney: id("stone_bricks"), pave: cob, beam: dark },
        _ => Style { found, ground: cob, upper: plaster, frame: true, post: timber, floor: planks, core: planks, tiles, thatch, noble: slate, chimney: id("clay_bricks"), pave: cob, beam: dark },
    }
}

/// Ce que l'on bâtit à un emplacement du village.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Plan { Cottage, Town, Tavern, Shop(&'static str), Forge(&'static str), Church, Mill }
impl Plan {
    /// Largeur et profondeur des murs (le toit déborde d'une case autour).
    pub fn size(self) -> (i32, i32) { match self { Plan::Cottage => (5, 6), Plan::Town => (5, 7), Plan::Tavern => (7, 8), Plan::Shop(_) | Plan::Forge(_) => (7, 7), Plan::Church => (7, 12), Plan::Mill => (3, 3) } }
    pub fn craft(self) -> Option<&'static str> { match self { Plan::Shop(c) | Plan::Forge(c) => Some(c), _ => None } }
    pub fn dwelling(self) -> bool { !matches!(self, Plan::Church | Plan::Mill) }
}
/// Les ouvertures d'un mur : côté (0 devant, 1 droite, 2 fond, 3 gauche), case le long du mur vu de dehors, rangées couvertes.
#[derive(Clone, Copy)] struct Gap { side: u8, t: i32, y0: i32, y1: i32, door: bool }

struct B<'a> { k: &'a mut Work, lot: Lot, w: i32, d: i32, s: Style, gaps: Vec<Gap>, paint: C3, h: f64 }
impl B<'_> {
    fn put(&mut self, u: i32, y: i32, ww: i32, b: Id) { let c = self.lot.cell(u, y, ww); self.k.put(c, b); }
    fn len(&self, side: u8) -> i32 { if side % 2 == 0 { self.w } else { self.d } }
    /// Le point (t, y) d'un mur vu de dehors, décollé de `out` de sa face.
    fn on(&self, side: u8, t: f64, y: f64, out: f64) -> C3 {
        let (w, d) = (self.w as f64, self.d as f64);
        match side { 0 => self.lot.p(t, y, -out), 1 => self.lot.p(w + out, y, t), 2 => self.lot.p(w - t, y, d + out), _ => self.lot.p(-out, y, d - t) }
    }
    fn normal(&self, side: u8) -> C3 { match side { 0 => self.lot.v(0.0, 0.0, -1.0), 1 => self.lot.v(1.0, 0.0, 0.0), 2 => self.lot.v(0.0, 0.0, 1.0), _ => self.lot.v(-1.0, 0.0, 0.0) } }
    /// Le bloc de mur qui porte ce point.
    fn anchor(&self, side: u8, t: f64, y: f64) -> [i32; 3] {
        let k = t.floor() as i32; let y = y.floor() as i32; let (w, d) = (self.w, self.d);
        match side { 0 => self.lot.cell(k.clamp(0, w - 1), y, 0), 1 => self.lot.cell(w - 1, y, k.clamp(0, d - 1)), 2 => self.lot.cell((w - 1 - k).clamp(0, w - 1), y, d - 1), _ => self.lot.cell(0, y, (d - 1 - k).clamp(0, d - 1)) }
    }
    /// La case (u, ww) d'un mur, en position le long de ce mur.
    fn t_of(&self, side: u8, u: i32, ww: i32) -> i32 { match side { 0 => u, 1 => ww, 2 => self.w - 1 - u, _ => self.d - 1 - ww } }
    fn cell_of(&self, side: u8, t: i32) -> (i32, i32) { match side { 0 => (t, 0), 1 => (self.w - 1, t), 2 => (self.w - 1 - t, self.d - 1), _ => (0, self.d - 1 - t) } }

    /// Murs pleins : une bande de `low` sur la première rangée, `mat` au-dessus, poteaux aux angles.
    fn walls(&mut self, y0: i32, y1: i32, low: Id, mat: Id, post: Id) {
        for y in y0..=y1 { for u in 0..self.w { for ww in 0..self.d {
            let edge = u == 0 || ww == 0 || u == self.w - 1 || ww == self.d - 1; if !edge { continue; }
            let corner = (u == 0 || u == self.w - 1) && (ww == 0 || ww == self.d - 1);
            let b = if corner { post } else if y == y0 { low } else { mat }; self.put(u, y, ww, b);
        } } }
    }
    fn window(&mut self, side: u8, t: i32, y: i32) {
        let (u, ww) = self.cell_of(side, t); let f = match side { 0 | 2 => 0u8, _ => 1u8 };
        let face = (self.lot_face() + f) % 4; let b = crate::shapes::variant(id("window"), face, false, false, false); self.put(u, y, ww, b);
        self.gaps.push(Gap { side, t, y0: y, y1: y, door: false });
    }
    /// Sens de la façade (celui des portes et fenêtres de devant).
    fn lot_face(&self) -> u8 { crate::shapes::facing_of(self.lot.w.0 as f64, self.lot.w.1 as f64) }
    fn door(&mut self, side: u8, t: i32, y: i32) {
        let (u, ww) = self.cell_of(side, t); let f = self.lot_face(); let b = id("door");
        self.put(u, y, ww, crate::shapes::variant(b, f, false, false, false)); self.put(u, y + 1, ww, crate::shapes::variant(b, f, false, false, true));
        self.gaps.push(Gap { side, t, y0: y, y1: y + 1, door: true });
        // Chambranle et linteau de chêne.
        let (n, beam) = (self.normal(side), self.s.beam); let at = self.anchor(side, t as f64 + 0.5, y as f64 + 2.0);
        for x in [t as f64 - 0.06, t as f64 + 1.06] { let (a, b) = (self.on(side, x, y as f64, 0.05), self.on(side, x, y as f64 + 2.1, 0.05)); self.k.bar(at, a, b, n, 0.07, 0.06, beam, -1.0); }
        let (a, b) = (self.on(side, t as f64 - 0.2, y as f64 + 2.08, 0.06), self.on(side, t as f64 + 1.2, y as f64 + 2.08, 0.06)); self.k.bar(at, a, b, n, 0.09, 0.08, beam, -1.0);
    }
    /// Colombages d'un mur entre les hauteurs `ya` et `yb` : sablière basse et haute, poteaux, écharpes en diagonale,
    /// appuis et linteaux des fenêtres.
    fn frame(&mut self, side: u8, ya: f64, yb: f64) {
        let len = self.len(side) as f64; let n = self.normal(side); let c = self.s.beam; let gaps: Vec<Gap> = self.gaps.iter().filter(|g| g.side == side).copied().collect();
        let mid = self.anchor(side, len * 0.5, ya + 0.5);
        for (y, hw) in [(ya + 0.09, 0.09), (yb - 0.09, 0.09)] { let (a, b) = (self.on(side, -0.02, y, 0.045), self.on(side, len + 0.02, y, 0.045)); self.k.bar(mid, a, b, n, hw, 0.05, c, -1.0); }
        let busy = |t0: f64, t1: f64| gaps.iter().any(|g| (g.t as f64) < t1 - 0.01 && (g.t + 1) as f64 > t0 + 0.01 && (g.y0 as f64) < yb - 0.01 && (g.y1 + 1) as f64 > ya + 0.01);
        let mut t = 0.0; while t <= len + 0.01 {
            let edge = t < 0.01 || t > len - 0.01; let at = self.anchor(side, t.min(len - 0.5), ya + 0.5);
            if edge || !busy(t - 0.05, t + 0.05) { let (a, b) = (self.on(side, t, ya, 0.045), self.on(side, t, yb, 0.045)); self.k.bar(at, a, b, n, if edge { 0.12 } else { 0.085 }, 0.05, c, -1.0); }
            t += 1.0;
        }
        // Écharpes : dans les panneaux de deux cases sans ouverture, en chevron.
        let mut p = 0; while (p + 2) as f64 <= len + 0.01 {
            let (t0, t1) = (p as f64, p as f64 + 2.0);
            if !busy(t0, t1) && yb - ya > 1.5 {
                let at = self.anchor(side, t0 + 1.0, ya + 0.5); let flip = (p / 2) % 2 == 1;
                let (a, b) = if flip { (self.on(side, t0 + 0.15, yb - 0.2, 0.04), self.on(side, t1 - 0.15, ya + 0.2, 0.04)) } else { (self.on(side, t0 + 0.15, ya + 0.2, 0.04), self.on(side, t1 - 0.15, yb - 0.2, 0.04)) };
                self.k.bar(at, a, b, n, 0.075, 0.045, c, -1.0);
            }
            p += 2;
        }
        // Lisse à mi-hauteur, sauf devant les portes.
        if yb - ya > 2.4 { let y = ya + 1.0; for k in 0..self.len(side) {
            let (t0, t1) = (k as f64, k as f64 + 1.0); if gaps.iter().any(|g| g.door && g.t == k) { continue; }
            let at = self.anchor(side, t0 + 0.5, ya + 0.5); let (a, b) = (self.on(side, t0, y, 0.045), self.on(side, t1, y, 0.045)); self.k.bar(at, a, b, n, 0.06, 0.045, c, -1.0);
        } }
        self.trims(side, ya, yb);
    }
    /// Appuis, linteaux et volets des fenêtres de ce mur entre `ya` et `yb`.
    fn trims(&mut self, side: u8, ya: f64, yb: f64) {
        let n = self.normal(side); let c = self.s.beam; let paint = self.paint;
        for g in self.gaps.clone().into_iter().filter(|g| g.side == side && !g.door && (g.y0 as f64) >= ya - 0.01 && (g.y0 as f64) < yb) {
            let (t, y) = (g.t as f64, g.y0 as f64); let at = self.anchor(side, t + 0.5, y);
            let (a, b) = (self.on(side, t - 0.12, y - 0.04, 0.08), self.on(side, t + 1.12, y - 0.04, 0.08)); self.k.bar(at, a, b, n, 0.05, 0.08, c, -1.0);
            let (a, b) = (self.on(side, t - 0.08, y + 1.05, 0.05), self.on(side, t + 1.08, y + 1.05, 0.05)); self.k.bar(at, a, b, n, 0.06, 0.05, c, -1.0);
            // Volets peints, planches verticales et barres en Z.
            for (t0, t1) in [(t - 0.47, t - 0.03), (t + 1.03, t + 1.47)] {
                let (a, b) = (self.on(side, (t0 + t1) * 0.5, y + 0.04, 0.03), self.on(side, (t0 + t1) * 0.5, y + 0.96, 0.03)); self.k.bar(at, a, b, n, (t1 - t0) * 0.5, 0.025, paint, -1.0);
                for yy in [y + 0.22, y + 0.78] { let (a, b) = (self.on(side, t0 + 0.04, yy, 0.06), self.on(side, t1 - 0.04, yy, 0.06)); self.k.bar(at, a, b, n, 0.035, 0.012, [paint[0] * 0.7, paint[1] * 0.7, paint[2] * 0.7], -1.0); }
            }
        }
    }
    /// Jardinière fleurie sous une fenêtre.
    fn flowers(&mut self, side: u8, t: i32, y: i32) {
        let n = self.normal(side); let at = self.anchor(side, t as f64 + 0.5, y as f64);
        let (a, b) = (self.on(side, t as f64 - 0.05, y as f64 - 0.16, 0.2), self.on(side, t as f64 + 1.05, y as f64 - 0.16, 0.2)); self.k.bar(at, a, b, n, 0.12, 0.14, [0.42, 0.27, 0.15], -1.0);
        let cols = [[0.75, 0.16, 0.2], [0.95, 0.85, 0.3], [0.85, 0.45, 0.65], [0.95, 0.95, 0.9], [0.3, 0.55, 0.22]];
        for k in 0..7 { let tt = t as f64 + 0.05 + k as f64 * 0.15; let c = cols[(k + (self.h * 7.0) as usize) % 5]; let leaf = [0.22, 0.4, 0.16];
            let p = self.on(side, tt, y as f64 + 0.02, 0.22); self.k.solid(at, p, [mulv(self.lot.v(1.0, 0.0, 0.0), 0.06), [0.0, 0.07, 0.0], mulv(self.lot.v(0.0, 0.0, 1.0), 0.06)], leaf, -1.0, 0);
            let q = self.on(side, tt + 0.05, y as f64 + 0.1, 0.24); self.k.solid(at, q, [mulv(self.lot.v(1.0, 0.0, 0.0), 0.05), [0.0, 0.045, 0.0], mulv(self.lot.v(0.0, 0.0, 1.0), 0.05)], c, -1.0, 0); }
    }

    /// Toit à deux pans au-dessus de `yr` : âme en gradins (les blocs, pour l'intérieur et les collisions), pans inclinés
    /// à 45° posés dessus, faîtage, rives, pignons fermés, rangs de tuiles en relief.
    fn roof(&mut self, yr: i32, cover: Id, gable: Id, thick: f64) {
        let (w, d) = (self.w, self.d); let core = if cover == self.s.thatch { cover } else { self.s.core };
        // L'âme en gradins reste à l'aplomb des murs : les débords du toit ne sont que le pan incliné, sans blocs dessous.
        // Au premier gradin, une sablière (poutre ou mur) ferme le haut des longs pans.
        let plate = if self.s.frame && gable == self.s.upper { self.s.post } else { gable };
        let (mut k, mut lu, mut ru) = (0, -1, w);
        while lu <= ru {
            let (l, r) = if k == 0 { (0, w - 1) } else { (lu, ru) }; let m = if k == 0 { plate } else { core };
            let mm = |ww: i32| if k > 0 && (ww == 0 || ww == d - 1) { gable } else { m }; // aux pignons, la matière du pignon
            if l == r { for ww in 0..d { self.put(l, yr + k, ww, mm(ww)); } break; }
            for ww in 0..d { self.put(l, yr + k, ww, mm(ww)); self.put(r, yr + k, ww, mm(ww)); }
            for u in l + 1..r { self.put(u, yr + k, 0, gable); self.put(u, yr + k, d - 1, gable); }
            k += 1; lu += 1; ru -= 1;
        }
        let (wf, df, y) = (w as f64, d as f64, yr as f64); let ly = layer(cover); let rot = self.lot.grain(); let lot = self.lot;
        let (e, over) = (1.45, 1.3); let peak = y + 2.0 + wf * 0.5; let hl = (df + 2.0 * over) * 0.5;
        let at = if w % 2 == 1 { lot.cell((w - 1) / 2, yr + (w + 1) / 2, d / 2) } else { lot.cell(w / 2 - 1, yr + w / 2, d / 2) }; // le bloc du faîte
        let along = lot.v(0.0, 0.0, hl);
        for side in [-1.0f64, 1.0] {
            // Bord bas (avant-toit) et faîte, sur le plan qui affleure les gradins.
            let (ue, ur) = if side < 0.0 { (-e, wf * 0.5) } else { (wf + e, wf * 0.5) }; let ye = y + 2.0 + if side < 0.0 { ue } else { wf - ue };
            let nrm = unit(lot.v(side, 1.0, 0.0)); let slope = lot.v(ur - ue, peak - ye, 0.0);
            let c = add(lot.p((ue + ur) * 0.5, (ye + peak) * 0.5, df * 0.5), mulv(nrm, thick * 0.5));
            self.k.solid(at, c, [along, mulv(slope, 0.5), mulv(nrm, thick * 0.5)], [1.0, 1.0, 1.0], ly, rot);
            // Rangs en relief : une lèvre tous les demi-blocs le long de la pente (les tuiles se recouvrent).
            if thick < 0.3 { let n = ((ur - ue).abs() * std::f64::consts::SQRT_2 / 0.5) as i32;
                for i in 1..n { let f = i as f64 / n as f64; let p = add(add(lot.p(ue + (ur - ue) * f, ye + (peak - ye) * f, df * 0.5), mulv(nrm, thick + 0.025)), [0.0; 3]);
                    self.k.solid(at, p, [along, mulv(unit(slope), 0.035), mulv(nrm, 0.03)], [0.85, 0.85, 0.85], ly, rot); } }
            else { let p = add(lot.p(ue, ye, df * 0.5), mulv(nrm, thick * 0.3)); self.k.solid(at, p, [along, mulv(unit(slope), 0.22), mulv(nrm, 0.22)], [0.95, 0.95, 0.95], ly, rot); }
            // Rives : planches de chêne qui cachent les gradins sous les bords du toit, devant et au fond.
            for wv in [-over + 0.06, df + over - 0.06] {
                let (a, b) = (add(lot.p(ue, ye, wv), mulv(nrm, -0.3)), add(lot.p(ur, peak, wv), mulv(nrm, -0.3)));
                self.k.bar(at, a, b, lot.v(0.0, 0.0, 1.0), 0.36, 0.05, self.s.beam, -1.0);
            }
            // Pignons : le triangle entre le dernier gradin et le pan, devant et au fond, de la matière du pignon.
            for (wv, sg) in [(-0.025, -1.0), (df + 0.025, 1.0)] {
                let _ = sg; let (u0, u1) = if side < 0.0 { (0.0, wf * 0.5) } else { (wf, wf * 0.5) }; let (y0p, y1p) = (y + 2.0, peak);
                self.k.quad(at, [lot.p(u0, y0p - 1.06, wv), lot.p(u1, y1p - 1.06, wv), lot.p(u1, y1p, wv), lot.p(u0, y0p, wv)], [1.0, 1.0, 1.0], layer(gable), 0);
            }
        }
        // Faîtage : une rangée de tuiles faîtières posée en losange.
        let r = 0.17 + thick * 0.4; self.k.solid(at, lot.p(wf * 0.5, peak + thick * 0.9, df * 0.5), [along, lot.v(r, r, 0.0), lot.v(-r, r, 0.0)], [0.8, 0.8, 0.8], ly, rot);
        // Colombages des pignons : poinçon, poteaux, entrait.
        if self.s.frame && gable == self.s.upper {
            for wv in [-0.07, df + 0.07] {
                let n = lot.v(0.0, 0.0, if wv < 0.0 { -1.0 } else { 1.0 });
                let (a, b) = (lot.p(-0.05, y + 0.08, wv), lot.p(wf + 0.05, y + 0.08, wv)); self.k.bar(at, a, b, n, 0.09, 0.05, self.s.beam, -1.0);
                let (a, b) = (lot.p(0.6, y + 1.3, wv), lot.p(wf - 0.6, y + 1.3, wv)); self.k.bar(at, a, b, n, 0.07, 0.05, self.s.beam, -1.0);
                let mut u = 1.0; while u < wf - 0.5 { let top = y + 2.0 + u.min(wf - u) - 0.12; let (a, b) = (lot.p(u, y, wv), lot.p(u, top, wv)); self.k.bar(at, a, b, n, 0.075, 0.05, self.s.beam, -1.0); u += if (wf - 1.0) as i32 % 2 == 0 { 1.0 } else { 1.0 }; }
                let (a, b) = (lot.p(wf * 0.5, y, wv), lot.p(wf * 0.5, peak - 0.1, wv)); self.k.bar(at, a, b, n, 0.08, 0.05, self.s.beam, -1.0);
            }
        }
    }
    /// Cheminée de pierre contre le mur du fond, qui dépasse le faîte ; chapeau de dalles.
    fn chimney(&mut self, u: i32, y0: i32, top: i32) {
        let d = self.d; let m = self.s.chimney; for y in y0..=top { self.put(u, y, d, m); }
        let at = self.lot.cell(u, top, d); let c = self.lot.p(u as f64 + 0.5, top as f64 + 1.05, d as f64 + 0.5);
        self.k.solid(at, c, [self.lot.v(0.62, 0.0, 0.0), [0.0, 0.06, 0.0], self.lot.v(0.0, 0.0, 0.62)], [0.55, 0.53, 0.5], -1.0, 0);
        let c = self.lot.p(u as f64 + 0.5, top as f64 + 1.32, d as f64 + 0.5); self.k.solid(at, c, [self.lot.v(0.42, 0.0, 0.0), [0.0, 0.2, 0.0], self.lot.v(0.0, 0.0, 0.42)], [1.0, 1.0, 1.0], layer(m), 0);
        let c = self.lot.p(u as f64 + 0.5, top as f64 + 1.55, d as f64 + 0.5); self.k.solid(at, c, [self.lot.v(0.5, 0.0, 0.0), [0.0, 0.04, 0.0], self.lot.v(0.0, 0.0, 0.5)], [0.48, 0.46, 0.44], -1.0, 0);
    }
    /// Enseigne pendue à une potence de fer devant la façade : panneau et emblème du métier.
    fn sign(&mut self, t: f64, y: f64, emblem: &str) {
        let n = self.normal(0); let at = self.anchor(0, t, y - 0.5); let iron = [0.16, 0.16, 0.17];
        let (a, b) = (self.on(0, t, y, 0.0), self.on(0, t, y, 1.05)); self.k.bar(at, a, b, [0.0, 1.0, 0.0], 0.03, 0.03, iron, -1.0);
        let (a, b) = (self.on(0, t, y - 0.5, 0.0), self.on(0, t, y, 0.7)); self.k.bar(at, a, b, self.lot.v(1.0, 0.0, 0.0), 0.02, 0.02, iron, -1.0);
        for o in [0.32, 0.92] { let (a, b) = (self.on(0, t, y, o), self.on(0, t, y - 0.16, o)); self.k.bar(at, a, b, self.lot.v(1.0, 0.0, 0.0), 0.012, 0.012, iron, -1.0); }
        let side = self.lot.v(1.0, 0.0, 0.0); let cb = self.on(0, t, y - 0.52, 0.62);
        self.k.solid(at, cb, [mulv(unit(n), 0.42), [0.0, 0.36, 0.0], mulv(side, 0.035)], [0.45, 0.3, 0.17], -1.0, 0);
        self.k.solid(at, cb, [mulv(unit(n), 0.46), [0.0, 0.4, 0.0], mulv(side, 0.025)], [0.24, 0.15, 0.09], -1.0, 0);
        let face = |k: &mut Work, du: f64, dy: f64, hu: f64, hy: f64, c: C3| { for sg in [-1.0, 1.0] { let p = add(add(cb, mulv(unit(n), du)), add([0.0, dy, 0.0], mulv(side, 0.045 * sg))); k.solid(at, p, [mulv(unit(n), hu), [0.0, hy, 0.0], mulv(side, 0.012)], c, -1.0, 0); } };
        match emblem {
            "chope" => { face(self.k, -0.04, -0.04, 0.17, 0.2, [0.85, 0.62, 0.2]); face(self.k, -0.04, 0.2, 0.19, 0.06, [0.95, 0.93, 0.86]); face(self.k, 0.2, -0.02, 0.05, 0.12, [0.85, 0.62, 0.2]); }
            "enclume" => { face(self.k, 0.0, 0.08, 0.28, 0.07, [0.22, 0.22, 0.24]); face(self.k, -0.24, 0.1, 0.08, 0.04, [0.22, 0.22, 0.24]); face(self.k, 0.0, -0.06, 0.09, 0.08, [0.22, 0.22, 0.24]); face(self.k, 0.0, -0.18, 0.2, 0.05, [0.22, 0.22, 0.24]); }
            "peau" => { face(self.k, 0.0, 0.0, 0.24, 0.22, [0.62, 0.42, 0.24]); face(self.k, 0.0, 0.0, 0.32, 0.08, [0.62, 0.42, 0.24]); }
            "pierre" => { face(self.k, 0.0, -0.05, 0.22, 0.16, [0.62, 0.6, 0.56]); face(self.k, 0.1, 0.15, 0.04, 0.14, [0.35, 0.22, 0.12]); }
            _ => { face(self.k, 0.0, 0.0, 0.2, 0.2, [0.75, 0.6, 0.25]); }
        }
    }
}

/// Bâtit `plan` : murs, ouvertures, toit, cheminée, décor. Rend la case de la porte (u) et le fond (ww du poste).
pub fn raise(k: &mut Work, lot: Lot, plan: Plan, y0: i32, s: Style, h: f64) -> (i32, i32) {
    let (w, d) = plan.size(); let du = w / 2;
    const PAINT: [C3; 5] = [[0.24, 0.36, 0.26], [0.22, 0.3, 0.42], [0.5, 0.18, 0.14], [0.38, 0.3, 0.2], [0.3, 0.4, 0.42]];
    let mut b = B { k, lot, w, d, s, gaps: Vec::new(), paint: PAINT[(h * 5.0) as usize % 5], h };
    let floors = match plan { Plan::Town | Plan::Tavern | Plan::Shop(_) => 2, _ => 1 };
    let top_wall = match plan { Plan::Church => y0 + 5, Plan::Mill => y0 + 6, _ => y0 + 3 + 4 * (floors - 1) };
    // Place nette : rien dans le volume du bâtiment et de son toit.
    let clear = match plan { Plan::Church => 18, Plan::Mill => 10, _ => top_wall - y0 + w / 2 + 4 };
    for u in -1..=w { for ww in -1..=d { for y in y0 + 1..y0 + clear { b.put(u, y, ww, 0); } } }
    for u in 0..w { for ww in 0..d { for y in y0 - 3..y0 { b.put(u, y, ww, s.found); } b.put(u, y0, ww, s.floor); } }
    match plan {
        Plan::Church => {
            let stone = s.ground; b.walls(y0 + 1, y0 + 5, s.found, stone, stone);
            for ww in [3, 6, 9] { for side in [1u8, 3] { let t = b.t_of(side, 0, ww); b.window(side, t, y0 + 2); b.window(side, t, y0 + 3); } }
            b.window(2, du, y0 + 3); b.window(2, du, y0 + 4);
            b.roof(y0 + 6, s.noble, stone, 0.16);
            // Contreforts entre les baies, coiffés d'un glacis ; arcs brisés au-dessus des fenêtres hautes.
            let sl = layer(stone);
            for side in [1u8, 3] { let n = b.normal(side);
                for ww in [1, 4, 7, 10] { let t = b.t_of(side, 0, ww) as f64 + if side == 1 { 1.0 } else { 0.0 }; let at = b.anchor(side, t - 0.5, (y0 + 2) as f64);
                    let base = b.on(side, t, (y0 + 1) as f64 + 2.0, 0.32); b.k.solid(at, base, [mulv(cross_dir(n), 0.28), [0.0, 2.0, 0.0], mulv(n, 0.32)], [0.92, 0.92, 0.92], sl, 0);
                    let cap = b.on(side, t, (y0 + 5) as f64 + 0.2, 0.18); b.k.solid(at, cap, [mulv(cross_dir(n), 0.26), add([0.0, 0.32, 0.0], mulv(n, -0.16)), mulv(add(mulv(n, 0.3), [0.0, 0.15, 0.0]), 0.5)], [0.85, 0.85, 0.85], sl, 0);
                }
                for ww in [3, 6, 9] { let t = b.t_of(side, 0, ww) as f64; let at = b.anchor(side, t + 0.5, (y0 + 3) as f64); let y = (y0 + 4) as f64 + 0.02;
                    let (l, m, r) = (b.on(side, t - 0.08, y, 0.07), b.on(side, t + 0.5, y + 0.55, 0.07), b.on(side, t + 1.08, y, 0.07)); b.k.bar(at, l, m, n, 0.07, 0.07, [0.7, 0.68, 0.64], -1.0); b.k.bar(at, m, r, n, 0.07, 0.07, [0.7, 0.68, 0.64], -1.0);
                    b.k.quad(at, [b.on(side, t + 0.04, y, 0.02), b.on(side, t + 0.96, y, 0.02), b.on(side, t + 0.5, y + 0.48, 0.02), b.on(side, t + 0.5, y + 0.48, 0.02)], [0.38, 0.5, 0.62], -1.0, 0); }
            }
            // Clocher carré sur l'entrée : creux, ouvert en arcade vers la nef, beffroi à deux baies par face, flèche d'ardoise.
            let tt = y0 + 14;
            for y in y0 + 1..=tt { for u in du - 1..=du + 1 { for ww in -2..=0 {
                let edge = u != du || ww != -1; let bay = y >= tt - 2 && y < tt && ((u == du && ww != -1) || (ww == -1 && u != du));
                let arch = ww == 0 && u == du && y <= y0 + 3; let doorway = ww == -2 && u == du && y <= y0 + 2;
                if arch || bay || !edge { b.put(u, y, ww, 0); } else { b.put(u, y, ww, if doorway { 0 } else { stone }); }
            } } }
            // La porte s'ouvre au pied de la tour ; une arcade mène de la tour à la nef.
            let f = b.lot_face(); let dr = id("door");
            b.put(du, y0 + 1, -2, crate::shapes::variant(dr, f, false, false, false)); b.put(du, y0 + 2, -2, crate::shapes::variant(dr, f, false, false, true));
            let at = lot.cell(du - 1, tt, -1); let (cu, cw) = (du as f64 + 0.5, -0.5);
            pyramid(b.k, at, lot.p(cu, (tt + 1) as f64, cw), lot, 1.75, 1.75, 6.5, s.noble);
            b.k.cube(at, add(lot.p(cu, (tt - 2) as f64 + 0.1, cw), [-0.3, 0.0, -0.3]), add(lot.p(cu, (tt - 1) as f64 + 0.5, cw), [0.3, 0.0, 0.3]), [0.62, 0.48, 0.2], -1.0); // la cloche
            let cross = lot.p(cu, (tt + 1) as f64 + 6.5, cw); b.k.cube(at, add(cross, [-0.04, 0.0, -0.04]), add(cross, [0.04, 0.9, 0.04]), [0.3, 0.28, 0.24], -1.0); b.k.bar(at, add(cross, [0.0, 0.62, 0.0]), add(add(cross, [0.0, 0.62, 0.0]), lot.v(0.3, 0.0, 0.0)), [0.0, 0.0, 1.0], 0.035, 0.035, [0.3, 0.28, 0.24], -1.0); b.k.bar(at, add(cross, [0.0, 0.62, 0.0]), add(add(cross, [0.0, 0.62, 0.0]), lot.v(-0.3, 0.0, 0.0)), [0.0, 0.0, 1.0], 0.035, 0.035, [0.3, 0.28, 0.24], -1.0);
            // La nef : bancs tournés vers l'autel, tapis rouge dans l'allée, chandeliers, bannières aux murs.
            let f = b.lot_face(); let (left, right) = (crate::shapes::facing_of(-lot.u.0 as f64, -lot.u.1 as f64), crate::shapes::facing_of(lot.u.0 as f64, lot.u.1 as f64));
            for ww in (3..d - 2).step_by(2) { for u in [1, 2, w - 3, w - 2] { b.put(u, y0 + 1, ww, turned("bench", f + 2)); } }
            for ww in 1..d - 1 { b.put(du, y0 + 1, ww, id("carpet_red")); }
            b.put(du, y0 + 1, d - 2, id("chiseled_stone")); b.put(du, y0 + 2, d - 2, id("candle")); for u in [du - 1, du + 1] { b.put(u, y0 + 1, d - 2, id("candelabra")); }
            for ww in [2, 5, 8] { b.put(1, y0 + 4, ww, turned("banner_red", left)); b.put(w - 2, y0 + 4, ww, turned("banner_red", right)); }
            return (du, d - 2);
        }
        Plan::Mill => {
            b.walls(y0 + 1, y0 + 6, s.found, s.upper, s.post); b.door(0, 1, y0 + 1); b.window(1, 1, y0 + 4); b.window(3, 1, y0 + 4); b.window(2, 1, y0 + 2);
            if s.frame { for side in 0..4u8 { b.frame(side, (y0 + 2) as f64, (y0 + 7) as f64); } }
            for u in 0..3 { for ww in 0..3 { b.put(u, y0 + 7, ww, s.core); } }
            let at = lot.cell(1, y0 + 7, 1); pyramid(b.k, at, lot.p(1.5, (y0 + 8) as f64, 1.5), lot, 2.3, 2.3, 3.6, s.thatch);
            sails(b.k, at, lot, lot.p(1.5, (y0 + 6) as f64 + 0.6, -0.35), h);
            return (1, 1);
        }
        _ => {}
    }
    let craft = plan.craft();
    let forge = matches!(plan, Plan::Forge(_));
    // Rez-de-chaussée : la pierre ; à l'étage (ou au-dessus du soubassement pour une chaumière), le torchis entre les poutres.
    match plan {
        Plan::Cottage => b.walls(y0 + 1, y0 + 3, s.found, s.upper, s.post),
        Plan::Forge(_) => b.walls(y0 + 1, y0 + 3, s.found, s.ground, s.ground),
        _ => { b.walls(y0 + 1, y0 + 3, s.found, s.ground, s.ground); b.walls(y0 + 4, y0 + 4, s.post, s.post, s.post); b.walls(y0 + 5, y0 + 7, s.upper, s.upper, s.post); }
    }
    if floors == 2 { for u in 1..w - 1 { for ww in 1..d - 1 { b.put(u, y0 + 4, ww, s.floor); } } }
    b.door(0, du, y0 + 1);
    let side_t = |b: &B, side: u8, ww: i32| b.t_of(side, 0, ww);
    for ww in [d / 2] { for side in [1u8, 3] { let t = side_t(&b, side, ww); b.window(side, t, y0 + 2); } }
    b.window(2, w - 1 - du, y0 + 2);
    if craft.is_some() || plan == Plan::Tavern { b.window(0, du - 2, y0 + 2); b.window(0, du + 2, y0 + 2); }
    if floors == 2 {
        let front: Vec<i32> = if w >= 7 { vec![1, du, w - 2] } else { vec![1, w - 2] };
        for t in front { b.window(0, t, y0 + 6); b.flowers(0, t, y0 + 6); }
        for side in [1u8, 3] { for ww in [1, d - 2] { let t = side_t(&b, side, ww); b.window(side, t, y0 + 6); } }
        b.window(2, w - 1 - du, y0 + 6);
    }
    let yr = top_wall + 1;
    if s.frame && !forge {
        let (ya, yb) = if floors == 2 { ((y0 + 5) as f64, (y0 + 8) as f64) } else { ((y0 + 2) as f64, (y0 + 4) as f64) };
        for side in 0..4u8 { b.frame(side, ya, yb); }
        if floors == 2 { for side in 0..4u8 { b.trims(side, (y0 + 1) as f64, (y0 + 4) as f64); }
            // Encorbellement : la poutre de plancher saille, portée par des consoles.
            for side in 0..4u8 { let len = b.len(side) as f64; let n = b.normal(side); let at = b.anchor(side, len * 0.5, (y0 + 4) as f64);
                let (a, c) = (b.on(side, -0.1, (y0 + 4) as f64 + 0.5, 0.1), b.on(side, len + 0.1, (y0 + 4) as f64 + 0.5, 0.1)); b.k.bar(at, a, c, n, 0.5, 0.1, s.beam, -1.0);
                let mut t = 0.5; while t < len { let at = b.anchor(side, t, (y0 + 4) as f64); let p = b.on(side, t, (y0 + 4) as f64 - 0.12, 0.08); b.k.solid(at, p, [mulv(cross_dir(n), 0.08), [0.0, 0.12, 0.0], mulv(n, 0.08)], s.beam, -1.0, 0); t += 2.0; } }
        }
    } else { for side in 0..4u8 { b.trims(side, (y0 + 1) as f64, (yr + 1) as f64); } }
    let cover = match plan { Plan::Cottage => s.thatch, Plan::Forge(_) => s.noble, _ => s.tiles };
    let gable = if forge { s.ground } else { s.upper };
    b.roof(yr, cover, gable, if cover == s.thatch && cover == id("thatch") { 0.42 } else { 0.16 });
    let cu = if forge { du + 1 } else { w - 2 }; b.chimney(cu, y0 + 1, if forge { yr + w / 2 + 2 } else { yr + w / 2 + 1 });
    // Abords : massifs fleuris de part et d'autre de la porte, un banc, des buissons aux angles.
    garden(b.k, lot, w, d, du, y0, h);
    // Enseignes.
    match plan {
        Plan::Tavern => { b.sign(du as f64 + 1.9, (y0 + 3) as f64 + 0.6, "chope"); for u in [du - 2, du + 2] { b.put(u, y0 + 1, -2, id("barrel")); } }
        Plan::Forge(_) => { b.sign(du as f64 + 1.9, (y0 + 3) as f64 + 0.6, "enclume"); b.put(-2, y0 + 1, 1, id("barrel")); woodpile(b.k, lot, w, y0); }
        Plan::Shop(c) => { b.sign(du as f64 + 1.9, (y0 + 3) as f64 + 0.6, match c { "tanneur" => "peau", "tailleur_pierre" => "pierre", _ => "autre" }); b.put(w + 1, y0 + 1, 1, id("barrel")); crates(b.k, lot, -2.0, y0, 1.0, h); }
        Plan::Cottage => { hay(b.k, lot, w, y0, d); }
        _ => { crates(b.k, lot, w as f64 + 1.4, y0, 2.0, h); }
    }
    furnish(&mut b, plan, y0);
    if floors == 2 { upstairs(&mut b, plan, y0); }
    let _ = craft; (du, d - 2)
}
fn cross_dir(n: C3) -> C3 { [-n[2], 0.0, n[0]] }
/// Pyramide à base rectangulaire (flèche, toit conique du moulin) : quatre faces texturées.
fn pyramid(k: &mut Work, at: [i32; 3], base: C3, lot: Lot, hu: f64, hw: f64, h: f64, cover: Id) {
    let c = [add(base, add(lot.v(-hu, 0.0, -hw), [0.0; 3])), add(base, lot.v(hu, 0.0, -hw)), add(base, lot.v(hu, 0.0, hw)), add(base, lot.v(-hu, 0.0, hw))];
    let apex = add(base, [0.0, h, 0.0]);
    for i in 0..4 { k.quad(at, [c[i], c[(i + 1) % 4], apex, apex], [1.0, 1.0, 1.0], layer(cover), 0); }
    k.quad(at, [c[0], c[1], c[2], c[3]], [0.7, 0.7, 0.7], layer(cover), 0);
}
/// Les quatre ailes du moulin, en croix de Saint-André, devant la tour : vergue, cadre en échelle, toile.
fn sails(k: &mut Work, at: [i32; 3], lot: Lot, hub: C3, h: f64) {
    let wood = [0.42, 0.29, 0.17]; let cloth = [0.86, 0.8, 0.68]; let fwd = lot.v(0.0, 0.0, -1.0);
    k.solid(at, hub, [mulv(lot.v(1.0, 0.0, 0.0), 0.22), [0.0, 0.22, 0.0], mulv(fwd, 0.3)], [0.3, 0.2, 0.12], -1.0, 0);
    let a0 = 0.785 + h * 0.6;
    for i in 0..4 {
        let a = a0 + i as f64 * std::f64::consts::FRAC_PI_2; let dir = add(mulv(lot.v(1.0, 0.0, 0.0), a.cos()), [0.0, a.sin(), 0.0]); let perp = add(mulv(lot.v(1.0, 0.0, 0.0), -a.sin()), [0.0, a.cos(), 0.0]);
        let front = add(hub, mulv(fwd, 0.25));
        k.bar(at, front, add(front, mulv(dir, 4.6)), fwd, 0.07, 0.07, wood, -1.0);
        for off in [0.25, 0.85] { let p0 = add(add(front, mulv(dir, 1.1)), mulv(perp, off)); let p1 = add(add(front, mulv(dir, 4.5)), mulv(perp, off)); k.bar(at, p0, p1, fwd, 0.035, 0.035, wood, -1.0); }
        let mut r = 1.1; while r < 4.6 { let p0 = add(add(front, mulv(dir, r)), mulv(perp, -0.05)); let p1 = add(add(front, mulv(dir, r)), mulv(perp, 0.88)); k.bar(at, p0, p1, fwd, 0.025, 0.03, wood, -1.0); r += 0.38; }
        let c = add(add(add(front, mulv(dir, 2.9)), mulv(perp, 0.55)), mulv(fwd, -0.03)); k.solid(at, c, [mulv(dir, 1.65), mulv(perp, 0.3), mulv(fwd, 0.01)], cloth, -1.0, 0);
    }
}
/// Bûcher contre le flanc gauche : des piles de bûches (de vrais blocs, qu'on peut prendre).
fn woodpile(k: &mut Work, lot: Lot, _w: i32, y0: i32) { let lp = id("log_pile"); for ww in 1..=3 { k.put(lot.cell(-1, y0 + 1, ww), lp); } k.put(lot.cell(-1, y0 + 2, 2), lp); }
/// Caisses de planches et sacs à côté de la porte (de vrais blocs).
fn crates(k: &mut Work, lot: Lot, u: f64, y0: i32, ww: f64, h: f64) {
    let (u, ww) = (u.floor() as i32, ww.floor() as i32); let cr = id("crate");
    k.put(lot.cell(u, y0 + 1, ww), cr); k.put(lot.cell(u, y0 + 1, ww + 1), if h > 0.5 { cr } else { id("sack") }); if h > 0.3 { k.put(lot.cell(u, y0 + 2, ww), id(if h > 0.65 { "sack" } else { "crate" })); }
}
/// Bottes de foin contre le pignon d'une chaumière (de vrais blocs).
fn hay(k: &mut Work, lot: Lot, w: i32, y0: i32, d: i32) { let hb = id("hay_bale"); for ww in [d - 3, d - 2] { k.put(lot.cell(w, y0 + 1, ww), hb); } k.put(lot.cell(w, y0 + 2, d - 2), hb); }

/// Le puits : margelle de pierre (blocs), deux montants, traverse, treuil, petit toit, seau pendu.
pub fn well(k: &mut Work, c: [i32; 3], tiles: Id) {
    let (x, y, z) = (c[0] as f64 + 0.5, c[1] as f64 + 2.0, c[2] as f64 + 0.5); let wood = [0.4, 0.27, 0.15];
    for sx in [-1.0, 1.0] { let at = [c[0] + sx as i32, c[1] + 1, c[2]]; k.cube(at, [x + sx * 1.1 - 0.09, y, z - 0.09], [x + sx * 1.1 + 0.09, y + 2.3, z + 0.09], wood, -1.0); }
    let at = [c[0] - 1, c[1] + 1, c[2]];
    k.cube(at, [x - 1.25, y + 2.2, z - 0.07], [x + 1.25, y + 2.36, z + 0.07], wood, -1.0);
    k.cube(at, [x - 0.9, y + 1.25, z - 0.1], [x + 0.9, y + 1.45, z + 0.1], [0.34, 0.22, 0.12], -1.0);
    k.cube(at, [x - 0.02, y + 0.35, z - 0.02], [x + 0.02, y + 1.27, z + 0.02], [0.62, 0.55, 0.4], -1.0);
    k.cube(at, [x - 0.16, y + 0.05, z - 0.16], [x + 0.16, y + 0.38, z + 0.16], [0.36, 0.25, 0.14], -1.0);
    k.cube(at, [x + 0.9, y + 1.3, z - 0.03], [x + 1.3, y + 1.38, z + 0.03], [0.2, 0.2, 0.22], -1.0);
    let ly = layer(tiles); let r2 = std::f64::consts::FRAC_1_SQRT_2;
    for sz in [-1.0, 1.0] { k.solid(at, [x, y + 2.75, z + sz * 0.5], [[1.45, 0.0, 0.0], [0.0, 0.62 * r2, -sz * 0.62 * r2], [0.0, 0.08 * r2, sz * 0.08 * r2]], [1.0, 1.0, 1.0], ly, 2); }
    k.solid(at, [x, y + 3.18, z], [[1.47, 0.0, 0.0], [0.0, 0.1, 0.1], [0.0, -0.1, 0.1]], [0.8, 0.8, 0.8], ly, 2);
}
/// Un étal de marché face au centre : poteaux, comptoir de planches, auvent rayé en pente, marchandises.
pub fn stall(k: &mut Work, at: [i32; 3], c: C3, facing: C3, stripes: [C3; 2], h: f64) {
    let f = unit(facing); let side = [-f[2], 0.0, f[0]]; let (wd, dp) = (1.25, 0.6); let wood = [0.4, 0.27, 0.15]; let pl = layer(id("planks"));
    let pt = |a: f64, b: f64, y: f64| add(add(c, mulv(side, a)), add(mulv(f, b), [0.0, y, 0.0]));
    for (a, b, hgt) in [(-wd, dp, 2.05), (wd, dp, 2.05), (-wd, -dp, 2.45), (wd, -dp, 2.45)] { k.bar(at, pt(a, b, 0.0), pt(a, b, hgt), f, 0.06, 0.06, wood, -1.0); }
    k.solid(at, pt(0.0, 0.15, 0.92), [mulv(side, wd + 0.05), [0.0, 0.05, 0.0], mulv(f, 0.45)], [0.95, 0.95, 0.95], pl, 0);
    k.solid(at, pt(0.0, 0.58, 0.48), [mulv(side, wd), [0.0, 0.42, 0.0], mulv(f, 0.03)], [0.8, 0.8, 0.8], pl, 0);
    // Auvent : bandes alternées, de l'arrière (haut) vers l'avant (bas), avec un lambrequin festonné.
    let n = 7; for i in 0..n { let a0 = -wd - 0.15 + (2.0 * wd + 0.3) * i as f64 / n as f64; let a1 = -wd - 0.15 + (2.0 * wd + 0.3) * (i + 1) as f64 / n as f64; let col = stripes[i % 2];
        let (p0, p1) = (pt((a0 + a1) * 0.5, -dp - 0.1, 2.5), pt((a0 + a1) * 0.5, dp + 0.45, 2.0)); k.bar(at, p0, p1, [0.0, 1.0, 0.0], (a1 - a0) * 0.5, 0.025, col, -1.0);
        let q = pt((a0 + a1) * 0.5, dp + 0.45, 1.88); k.solid(at, q, [mulv(side, (a1 - a0) * 0.5 - 0.01), [0.0, 0.12, 0.0], mulv(f, 0.015)], stripes[(i + 1) % 2], -1.0, 0); }
    let goods = [[0.66, 0.12, 0.1], [0.4, 0.58, 0.22], [0.82, 0.58, 0.2], [0.86, 0.76, 0.42], [0.58, 0.36, 0.18]];
    for i in 0..9 { let a = -wd + 0.25 + (i % 5) as f64 * 0.5; let b = 0.05 + (i / 5) as f64 * 0.3; let g = goods[(i + (h * 5.0) as usize) % 5]; let s = 0.07 + 0.03 * ((i * 7) % 3) as f64;
        k.solid(at, pt(a, b, 0.97 + s), [mulv(side, s * 1.2), [0.0, s, 0.0], mulv(f, s)], g, -1.0, 0); }
    // Paniers et caisses au pied.
    for (a, b) in [(-wd - 0.45, 0.2), (wd + 0.45, -0.1)] { k.solid(at, pt(a, b, 0.25), [mulv(side, 0.25), [0.0, 0.25, 0.0], mulv(f, 0.25)], [0.9, 0.9, 0.9], pl, 0); }
}

/// Massifs le long de la façade (feuillage en touffes, fleurs), un banc contre le mur, deux buissons aux angles.
fn garden(k: &mut Work, lot: Lot, w: i32, _d: i32, du: i32, y0: i32, h: f64) {
    let leaf = [[0.2, 0.36, 0.13], [0.26, 0.42, 0.16], [0.17, 0.3, 0.12]]; let flower = [[0.78, 0.2, 0.22], [0.95, 0.86, 0.35], [0.8, 0.5, 0.75], [0.95, 0.95, 0.92], [0.45, 0.4, 0.8]];
    let mut r = crate::noise::Rng::new((h * 1e6) as u32 + 7);
    for (i, (u0, u1)) in [(0.2, du as f64 - 0.25), (du as f64 + 1.25, w as f64 - 0.2)].into_iter().enumerate() { if u1 - u0 < 0.6 || (i == 0 && w >= 7) { continue; }
        let at = lot.cell(((u0 + u1) * 0.5) as i32, y0, 0);
        k.solid(at, lot.p((u0 + u1) * 0.5, y0 as f64 + 1.06, -0.32), [lot.v((u1 - u0) * 0.5, 0.0, 0.0), [0.0, 0.07, 0.0], lot.v(0.0, 0.0, 0.3)], [0.3, 0.2, 0.12], -1.0, 0);
        let n = ((u1 - u0) / 0.22) as i32; for i in 0..n { let u = u0 + 0.11 + i as f64 * 0.22 + r.range(-0.05, 0.05); let ww = -0.32 + r.range(-0.15, 0.15); let s = r.range(0.07, 0.13);
            k.solid(at, lot.p(u, y0 as f64 + 1.13 + s, ww), [lot.v(s, 0.0, 0.0), [0.0, s * 1.1, 0.0], lot.v(0.0, 0.0, s)], leaf[r.int(0, 2) as usize], -1.0, 0);
            if r.next() < 0.7 { let f = flower[((h * 5.0) as usize + r.int(0, 1) as usize * 2) % 5]; let q = s * 0.45; k.solid(at, lot.p(u + r.range(-0.04, 0.04), y0 as f64 + 1.15 + s * 2.1, ww + r.range(-0.04, 0.04)), [lot.v(q, 0.0, 0.0), [0.0, q * 0.7, 0.0], lot.v(0.0, 0.0, q)], f, -1.0, 0); } }
    }
    // Buissons ronds aux deux angles de façade.
    for u in [-0.55, w as f64 + 0.55] { let at = lot.cell(if u < 0.0 { 0 } else { w - 1 }, y0, 0);
        for j in 0..5 { let s = r.range(0.22, 0.34); k.solid(at, lot.p(u + r.range(-0.15, 0.15), y0 as f64 + 1.0 + s * 0.9 + j as f64 * 0.06, -0.3 + r.range(-0.2, 0.2)), [lot.v(s, 0.0, 0.0), [0.0, s, 0.0], lot.v(0.0, 0.0, s)], leaf[j % 3], -1.0, 0); } }
    // Banc de planches sous une fenêtre.
    if w >= 7 { let at = lot.cell(1, y0, 0); let wood = [0.42, 0.28, 0.15];
        k.solid(at, lot.p(du as f64 - 1.4, y0 as f64 + 1.45, -0.3), [lot.v(0.55, 0.0, 0.0), [0.0, 0.04, 0.0], lot.v(0.0, 0.0, 0.16)], wood, -1.0, 0);
        for du2 in [-0.45, 0.45] { k.solid(at, lot.p(du as f64 - 1.4 + du2, y0 as f64 + 1.21, -0.3), [lot.v(0.04, 0.0, 0.0), [0.0, 0.21, 0.0], lot.v(0.0, 0.0, 0.13)], wood, -1.0, 0); } }
}

/// Un meuble orientable dans le sens `f` (son dos vers cette direction).
fn turned(n: &str, f: u8) -> Id { crate::shapes::variant(id(n), f % 4, false, false, false) }
/// L'intérieur : table et bougie, chaise, armoire, tapis ; à l'auberge des tables rondes, des tabourets, un lustre ;
/// chez l'artisan ses outils. Le mobilier du village (lit, coffre, poste) se pose ensuite et a priorité.
/// L'escalier le long du mur de droite, du devant vers le fond (quatre marches, la dernière au niveau du plancher), la trémie ouverte
/// au-dessus ; puis la chambre de l'étage.
fn upstairs(b: &mut B, plan: Plan, y0: i32) {
    let (w, d) = (b.w, b.d); let du = w / 2; let f = b.lot_face(); let su = w - 2;
    let fname = BLOCKS[b.s.floor as usize].name; let stairs = by_name(&format!("{fname}_stairs")).or_else(|| by_name("planks_stairs")).unwrap_or(b.s.floor);
    for k in 0..4 { b.put(su, y0 + 1 + k, 2 + k, crate::shapes::variant(stairs, f, false, false, false)); if k < 3 { b.put(su, y0 + 4, 2 + k, 0); } }
    for k in 0..2 { b.put(su, y0 + 1 + k, 4 + k, 0); }
    let y = y0 + 5; let left = crate::shapes::facing_of(-b.lot.u.0 as f64, -b.lot.u.1 as f64);
    // La chambre : lit contre le mur du fond, coffre, une lanterne au bout de la trémie, une étagère.
    b.put(1, y, d - 2, turned("bed", f)); b.put(1, y, d - 3, turned("bed_foot", f)); b.put(su, y, 1, id("lantern"));
    b.put(1, y + 2, 2, turned("wall_shelf", left));
    match plan {
        Plan::Tavern => { b.put(3, y, d - 2, turned("bed", f)); b.put(3, y, d - 3, turned("bed_foot", f)); b.put(2, y, d - 2, id("chest")); b.put(1, y, 1, id("table")); b.put(1, y + 1, 1, id("candle")); b.put(2, y, 1, id("stool")); }
        Plan::Shop(_) => { b.put(2, y, d - 2, id("chest")); b.put(1, y, 1, id("table")); b.put(1, y + 1, 1, id("candle")); b.put(1, y, 2, turned("chair", f)); b.put(du, y, d - 2, turned("cupboard", f)); b.put(du, y, 3, id("carpet_blue")); }
        _ => { b.put(1, y, 1, id("chest")); b.put(2, y, 2, id(if b.h > 0.5 { "carpet_red" } else { "carpet_green" })); }
    }
}
fn furnish(b: &mut B, plan: Plan, y0: i32) {
    let (w, d) = (b.w, b.d); let du = w / 2; let f = b.lot_face(); let lot = b.lot;
    let right = crate::shapes::facing_of(lot.u.0 as f64, lot.u.1 as f64);
    match plan {
        Plan::Tavern => {
            for (u, ww) in [(1, 2), (w - 3, 4)] { b.put(u, y0 + 1, ww, id("round_table")); b.put(u, y0 + 2, ww, id(if ww == 2 { "candle" } else { "bottles" })); for dw in [-1, 1] { b.put(u, y0 + 1, ww + dw, id("stool")); } }
            b.put(du, y0 + 3, 3, id("chandelier")); for ww in 2..d - 2 { b.put(du, y0 + 1, ww, id("carpet_red")); }
            b.put(1, y0 + 1, d - 4, turned("fireplace", crate::shapes::facing_of(-lot.u.0 as f64, -lot.u.1 as f64))); b.put(1, y0 + 3, 2, turned("wall_shelf", crate::shapes::facing_of(-lot.u.0 as f64, -lot.u.1 as f64))); let _ = right;
        }
        Plan::Forge(_) => { b.put(1, y0 + 1, 2, turned("grindstone", f)); b.put(w - 2, y0 + 1, 3, id("cauldron")); b.put(1, y0 + 1, 3, id("bucket")); b.put(w - 2, y0 + 3, 2, turned("weapon_rack", right)); }
        Plan::Shop(_) => { b.put(1, y0 + 1, 2, id("crate")); b.put(1, y0 + 2, 2, id("sack")); b.put(w - 3, y0 + 1, 2, id("table")); b.put(w - 3, y0 + 2, 2, id("books_pile")); b.put(w - 3, y0 + 1, 3, turned("chair", f)); b.put(du - 1, y0 + 1, 2, id("carpet_blue")); }
        // La chaumière : la table à droite, le lit (deux blocs) à gauche. La maison de ville : la table à gauche, l'armoire au fond, l'escalier à droite.
        Plan::Cottage => {
            b.put(w - 2, y0 + 1, 2, id("table")); b.put(w - 2, y0 + 2, 2, id(if b.h > 0.5 { "candle" } else { "bread_basket" })); b.put(w - 2, y0 + 1, 3, turned("chair", f));
            b.put(du, y0 + 1, 2, id(if b.h > 0.5 { "carpet_red" } else { "carpet_green" })); b.put(w - 2, y0 + 3, 3, turned("wall_shelf", right)); b.put(1, y0 + 1, 1, id("flower_pot"));
        }
        Plan::Town => {
            b.put(1, y0 + 1, 2, id("table")); b.put(1, y0 + 2, 2, id(if b.h > 0.5 { "candle" } else { "bread_basket" })); b.put(1, y0 + 1, 3, turned("chair", f)); b.put(1, y0 + 1, 1, turned("chair", f + 2));
            b.put(du, y0 + 1, 2, id(if b.h > 0.5 { "carpet_red" } else { "carpet_green" }));
        }
        _ => {}
    }
}
