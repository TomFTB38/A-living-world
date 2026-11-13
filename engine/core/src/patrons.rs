//! Les patrons de construction médiévaux, reproduits couche par couche d'après les planches fournies (A Living World,
//! « Civilisation médiévale ») : maison de villageois (M-MED-01), forge (M-FOR-01), auberge (M-AUB-01), moulin (M-005),
//! tanneur (M-TAN-01), église (M-EGL-01), grande mairie (M-MAI-01), petite mairie (M-MAI-02), atelier du bûcheron et
//! charpentier (M-BC-01).
//!
//! Repère local d'un patron : x de gauche à droite le long de la façade, z de la façade (z = 0, côté rue, « Nord » des
//! planches) vers le fond, y depuis le sol (y = 0 : fondations et plancher). Les toits sont faits d'escaliers de tuile ou
//! d'ardoise posés en gradins (45°), avec un faîtage plein, des pignons de torchis et de colombages, un débord d'un bloc.
//! Matières des planches : pierre taillée (pierres de taille), pierre brute (pavé), bûche et poutre de chêne, planches,
//! torchis, vitre (fenêtres), tuile, ardoise.
use crate::batisse::{id, Lot, Work};
use crate::blocks::Id;
use crate::shapes::{facing_of, variant};

/// Le chantier d'un patron : la commande de pose dans le repère local.
pub(crate) struct P<'a> { pub k: &'a mut Work, pub lot: Lot, pub y0: i32, pub mirror: bool, pub w: i32, pub d: i32, pub h: f64, pub rec: Rec }
/// Ce que le patron a posé et que l'habillage en relief reprendra : fenêtres, portes, pans de bois, toits, sablières, cheminées.
#[derive(Default)]
pub(crate) struct Rec { wins: Vec<(i32, i32, i32, bool)>, doors: Vec<(i32, i32, i32, i32, i32)>, frames: Vec<((i32, i32), (i32, i32), (i32, i32), i32)>, roofs: Vec<((i32, i32), (i32, i32), i32, i32, bool, String, Option<String>)>, belts: Vec<((i32, i32), (i32, i32), i32)>, lamps: Vec<(i32, i32, i32)>, pyrs: Vec<(i32, i32, i32, i32, String)> }
impl P<'_> {
    fn lx(&self, x: i32) -> i32 { if self.mirror { self.w - 1 - x } else { x } }
    pub fn put(&mut self, x: i32, y: i32, z: i32, b: Id) { let c = self.lot.cell(self.lx(x), self.y0 + y, z); self.k.put(c, b); }
    pub fn n(&mut self, x: i32, y: i32, z: i32, name: &str) { if name == "lantern" || name == "chandelier" { self.rec.lamps.push((x, y, z)); } self.put(x, y, z, id(name)); }
    /// La case porte-t-elle déjà un bloc ?
    pub fn filled(&self, x: i32, y: i32, z: i32) -> bool { let c = self.lot.cell(self.lx(x), self.y0 + y, z); self.k.blocks.iter().rev().find(|b| (b.0, b.1, b.2) == (c[0], c[1], c[2])).is_some_and(|b| b.3 != 0) }
    pub fn fill(&mut self, x: (i32, i32), y: (i32, i32), z: (i32, i32), name: &str) { let b = id(name); for xx in x.0..=x.1 { for yy in y.0..=y.1 { for zz in z.0..=z.1 { self.put(xx, yy, zz, b); } } } }
    /// Le sens monde d'une direction locale.
    pub fn face(&self, dx: i32, dz: i32) -> u8 { let dx = if self.mirror { -dx } else { dx }; facing_of((dx * self.lot.u.0 + dz * self.lot.w.0) as f64, (dx * self.lot.u.1 + dz * self.lot.w.1) as f64) }
    /// Un escalier dont la marche haute regarde la direction locale (dx, dz) ; `top` : renversé.
    pub fn stair(&mut self, x: i32, y: i32, z: i32, name: &str, dx: i32, dz: i32, top: bool) { let f = self.face(dx, dz); self.put(x, y, z, variant(id(name), f, top, false, false)); }
    /// Un objet orienté, le dos vers la direction locale (dx, dz).
    pub fn turn(&mut self, x: i32, y: i32, z: i32, name: &str, dx: i32, dz: i32) { let f = self.face(dx, dz); self.put(x, y, z, variant(id(name), f, false, false, false)); }
    /// Une bûche de chêne couchée le long de l'axe local x (ou z), ou debout.
    pub fn log(&mut self, x: i32, y: i32, z: i32, axis: char) {
        let world_x = match axis { 'x' => self.lot.u.0 != 0, 'z' => self.lot.w.0 != 0, _ => { self.n(x, y, z, "oak_log_c"); return; } };
        self.n(x, y, z, if world_x { "oak_log_x_c" } else { "oak_log_z_c" });
    }
    /// Porte de deux blocs dans un mur dont la normale locale (vers dehors) est (dx, dz).
    pub fn door(&mut self, x: i32, y: i32, z: i32, dx: i32, dz: i32) { self.rec.doors.push((x, y, z, dx, dz)); let f = self.face(-dx, -dz); let d = id("door"); self.put(x, y, z, variant(d, f, false, false, false)); self.put(x, y + 1, z, variant(d, f, false, false, true)); }
    /// Fenêtre dans un mur parallèle à x (`along_x`) ou à z.
    pub fn win(&mut self, x: i32, y: i32, z: i32, along_x: bool) { self.rec.wins.push((x, y, z, along_x)); let f = if along_x { self.face(0, 1) } else { self.face(1, 0) }; self.put(x, y, z, variant(id("window"), f, false, false, false)); }
    /// Un toit à deux pans en gradins d'escaliers : faîte le long de z (`along_z`) ou de x, sur l'emprise des murs
    /// [a0, a1] × [b0, b1], à partir de la hauteur y, débord `ov`. Le pignon (aux deux bouts du faîte) est rempli de `gable`.
    #[allow(clippy::too_many_arguments)]
    pub fn roof(&mut self, x: (i32, i32), z: (i32, i32), y: i32, ov: i32, along_z: bool, mat: &str, gable: Option<&str>) {
        self.rec.roofs.push((x, z, y, ov, along_z, mat.to_string(), gable.map(|g| g.to_string())));
        let st = format!("{mat}_stairs"); let ((a0, a1), (b0, b1)) = if along_z { (x, z) } else { (z, x) };
        let xy = |a: i32, b: i32| if along_z { (a, b) } else { (b, a) }; let up = |s: i32| if along_z { (s, 0) } else { (0, s) };
        let mut k = 0;
        loop { let (al, ar, yy) = (a0 - ov + k, a1 + ov - k, y + k); if al > ar { break; }
            for b in b0 - ov..=b1 + ov {
                if al == ar { let (px, pz) = xy(al, b); self.n(px, yy, pz, mat); continue; }
                let (lx, lz) = xy(al, b); let (rx, rz) = xy(ar, b); let (ux, uz) = up(1); let (vx, vz) = up(-1);
                self.stair(lx, yy, lz, &st, ux, uz, false); self.stair(rx, yy, rz, &st, vx, vz, false);
                if k > 1 { self.stair(lx, yy - 1, lz, &st, vx, vz, true); self.stair(rx, yy - 1, rz, &st, ux, uz, true); }
            }
            if let Some(g) = gable { for b in [b0, b1] { for a in (al + 1).max(a0)..=(ar - 1).min(a1) { let (px, pz) = xy(a, b); if k > 0 || !self.filled(px, yy, pz) { self.n(px, yy, pz, g); } } } }
            if al + 1 >= ar { if al + 1 == ar { for b in b0 - ov..=b1 + ov { let (px, pz) = xy(al, b); let (qx, qz) = xy(ar, b); self.n(px, yy + 1, pz, &format!("{mat}_slab")); self.n(qx, yy + 1, qz, &format!("{mat}_slab")); } } break; }
            k += 1;
        }
    }
    /// Un toit en pyramide (tour, clocher, moulin) de demi-côté r au-dessus de (cx, cz), à partir de y.
    pub fn pyramid(&mut self, cx: i32, cz: i32, r: i32, y: i32, mat: &str) {
        self.rec.pyrs.push((cx, cz, r, y, mat.to_string()));
        let st = format!("{mat}_stairs");
        for k in 0..=r { let rr = r - k; if rr == 0 { self.n(cx, y + k, cz, mat); break; }
            for dx in -rr..=rr { for dz in -rr..=rr { if dx.abs().max(dz.abs()) != rr { continue; } let (sx, sz) = if dx.abs() >= dz.abs() { (-dx.signum(), 0) } else { (0, -dz.signum()) }; self.stair(cx + dx, y + k, cz + dz, &st, sx, sz, false); } }
            for dx in -rr + 1..rr { for dz in -rr + 1..rr { self.n(cx + dx, y + k, cz + dz, mat); } } }
    }
    /// Un mur à pans de bois sur un rectangle : poteaux de chêne aux angles (et tous les `step`), torchis entre, de y0 à y1.
    pub fn frame(&mut self, x: (i32, i32), z: (i32, i32), y: (i32, i32), step: i32, infill: &str) {
        self.rec.frames.push((x, z, y, step));
        for xx in x.0..=x.1 { for zz in z.0..=z.1 { let edge = xx == x.0 || xx == x.1 || zz == z.0 || zz == z.1; if !edge { continue; }
            let corner = (xx == x.0 || xx == x.1) && (zz == z.0 || zz == z.1); let post = corner || (zz == z.0 || zz == z.1) && (xx - x.0) % step == 0 || (xx == x.0 || xx == x.1) && (zz - z.0) % step == 0;
            for yy in y.0..=y.1 { if post { self.log(xx, yy, zz, 'y'); } else { self.n(xx, yy, zz, infill); } } } }
    }
    /// Une sablière (ceinture de poutres couchées) autour d'un rectangle, à la hauteur y.
    pub fn belt(&mut self, x: (i32, i32), z: (i32, i32), y: i32) {
        self.rec.belts.push((x, z, y));
        for xx in x.0..=x.1 { for zz in [z.0, z.1] { self.log(xx, y, zz, 'x'); } } for zz in z.0 + 1..z.1 { for xx in [x.0, x.1] { self.log(xx, y, zz, 'z'); } }
        for (xx, zz) in [(x.0, z.0), (x.1, z.0), (x.0, z.1), (x.1, z.1)] { self.log(xx, y, zz, 'y'); }
    }
    /// Dégage le volume et pose les fondations (trois rangs sous le sol) et le plancher.
    pub fn ground(&mut self, ext: [i32; 4], h: i32, body: [i32; 4], floor: &str, found: &str) {
        for x in ext[0]..=ext[2] { for z in ext[1]..=ext[3] { for y in 1..=h { self.put(x, y, z, 0); } } }
        for x in body[0]..=body[2] { for z in body[1]..=body[3] { for y in -3..0 { self.n(x, y, z, "cobblestone"); } let edge = x == body[0] || x == body[2] || z == body[1] || z == body[3]; self.n(x, 0, z, if edge { found } else { floor }); } }
    }
    /// Le chapeau d'une cheminée : une dalle de pierre posée sur le conduit.
    pub fn cap(&mut self, x: i32, y: i32, z: i32) { self.n(x, y + 1, z, "stone_bricks_slab"); }
    /// Le bloc posé en dernier sur une case (0 : rien).
    fn at(&self, x: i32, y: i32, z: i32) -> Id { let c = self.lot.cell(self.lx(x), self.y0 + y, z); self.k.blocks.iter().rev().find(|b| (b.0, b.1, b.2) == (c[0], c[1], c[2])).map_or(0, |b| b.3) }
    fn name(&self, x: i32, y: i32, z: i32) -> &'static str { crate::blocks::BLOCKS[self.at(x, y, z) as usize].name }
    /// Un point du repère local, en continu (le miroir retourne x).
    fn fp(&self, x: f64, y: f64, z: f64) -> [f64; 3] { self.lot.p(if self.mirror { self.w as f64 - x } else { x }, self.y0 as f64 + y, z) }
    fn fv(&self, d: (f64, f64, f64)) -> [f64; 3] { self.lot.v(if self.mirror { -d.0 } else { d.0 }, d.1, d.2) }
    fn cw(&self, c: (i32, i32, i32)) -> [i32; 3] { self.lot.cell(self.lx(c.0), self.y0 + c.1, c.2) }
    /// Une pièce en relief de a à b, tenue par le bloc `at` : demi-largeur hw, demi-épaisseur ht selon `side`.
    #[allow(clippy::too_many_arguments)]
    fn piece(&mut self, at: (i32, i32, i32), a: (f64, f64, f64), b: (f64, f64, f64), side: (f64, f64, f64), hw: f64, ht: f64, col: [f64; 3]) {
        let (c, p0, p1, n) = (self.cw(at), self.fp(a.0, a.1, a.2), self.fp(b.0, b.1, b.2), self.fv(side)); self.k.bar(c, p0, p1, n, hw, ht, col, -1.0);
    }
    fn block(&mut self, at: (i32, i32, i32), lo: (f64, f64, f64), hi: (f64, f64, f64), col: [f64; 3]) {
        let (a, b) = (self.fp(lo.0, lo.1, lo.2), self.fp(hi.0, hi.1, hi.2)); let c = self.cw(at);
        self.k.cube(c, [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])], [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])], col, -1.0);
    }
    /// L'habillage en relief, une fois le gros œuvre posé : appuis, linteaux et volets peints des fenêtres, jardinières,
    /// chambranles des portes, écharpes et lisses des pans de bois, abouts de poutres, rives et planches d'égout des toits,
    /// faîtage, chapeaux de cheminée.
    pub fn dress(&mut self) {
        const OAK: [f64; 3] = [0.34, 0.22, 0.12]; const DARK: [f64; 3] = [0.24, 0.15, 0.08]; const END: [f64; 3] = [0.52, 0.38, 0.22]; const STONE: [f64; 3] = [0.52, 0.5, 0.47];
        const PAINT: [[f64; 3]; 5] = [[0.24, 0.36, 0.26], [0.22, 0.3, 0.42], [0.5, 0.18, 0.14], [0.38, 0.3, 0.2], [0.3, 0.4, 0.42]];
        let paint = PAINT[(self.h * 5.0) as usize % 5]; let rec = std::mem::take(&mut self.rec); let (cx, cz) = (self.w as f64 * 0.5, self.d as f64 * 0.5);
        let wins: std::collections::HashSet<(i32, i32, i32)> = rec.wins.iter().map(|w| (w.0, w.1, w.2)).collect();
        // Un point sur la face d'un mur : le mur suit x (`ax`) à la profondeur `z` ou suit z à l'abscisse `x` ; `s` : vers dehors.
        let on = |ax: bool, cell: (i32, i32), s: f64, t: f64, y: f64, o: f64| -> (f64, f64, f64) { if ax { (t, y, if s < 0.0 { cell.1 as f64 - o } else { cell.1 as f64 + 1.0 + o }) } else { (if s < 0.0 { cell.0 as f64 - o } else { cell.0 as f64 + 1.0 + o }, y, t) } };
        // ---- fenêtres
        for &(x, y, z, ax) in &rec.wins { if !self.name(x, y, z).starts_with("window") { continue; }
            let s = if ax { if (z as f64 + 0.5) < cz { -1.0 } else { 1.0 } } else if (x as f64 + 0.5) < cx { -1.0 } else { 1.0 };
            let n = if ax { (0.0, 0.0, s) } else { (s, 0.0, 0.0) }; let t = if ax { x } else { z } as f64; let (yf, c) = (y as f64, (x, z)); let at = (x, y, z);
            self.piece(at, on(ax, c, s, t - 0.14, yf - 0.03, 0.09), on(ax, c, s, t + 1.14, yf - 0.03, 0.09), n, 0.055, 0.09, OAK); // l'appui
            self.piece(at, on(ax, c, s, t - 0.1, yf + 1.04, 0.05), on(ax, c, s, t + 1.1, yf + 1.04, 0.05), n, 0.07, 0.05, OAK); // le linteau
            for (k, (t0, t1)) in [(t - 0.46, t - 0.03), (t + 1.03, t + 1.46)].into_iter().enumerate() { // les volets, s'il y a du mur à côté
                let nb = if ax { (x + if k == 0 { -1 } else { 1 }, y, z) } else { (x, y, z + if k == 0 { -1 } else { 1 }) }; let nm = self.name(nb.0, nb.1, nb.2);
                if wins.contains(&nb) || nm == "air" || nm.starts_with("door") || nm.contains("glass") { continue; }
                self.piece(at, on(ax, c, s, (t0 + t1) * 0.5, yf + 0.05, 0.03), on(ax, c, s, (t0 + t1) * 0.5, yf + 0.95, 0.03), n, (t1 - t0) * 0.5, 0.025, paint);
                for yy in [yf + 0.22, yf + 0.78] { self.piece(at, on(ax, c, s, t0 + 0.04, yy, 0.06), on(ax, c, s, t1 - 0.04, yy, 0.06), n, 0.035, 0.012, paint.map(|v| v * 0.65)); }
            }
            if y <= 2 && s < 0.0 && ax && (x + z + (self.h * 10.0) as i32) % 2 == 0 { // une jardinière fleurie sous les fenêtres de façade
                self.piece(at, on(ax, c, s, t - 0.05, yf - 0.17, 0.2), on(ax, c, s, t + 1.05, yf - 0.17, 0.2), n, 0.12, 0.14, [0.42, 0.27, 0.15]);
                let cols = [[0.75, 0.16, 0.2], [0.95, 0.85, 0.3], [0.85, 0.45, 0.65], [0.95, 0.95, 0.9], [0.3, 0.55, 0.22]];
                for k in 0..7 { let tt = t + 0.08 + k as f64 * 0.14; let q = on(ax, c, s, tt, yf + 0.02, 0.21); let col = cols[(k + x as usize) % 5];
                    self.block(at, (q.0 - 0.06, q.1 - 0.06, q.2 - 0.06), (q.0 + 0.06, q.1 + 0.06, q.2 + 0.06), [0.22, 0.4, 0.16]);
                    let q = on(ax, c, s, tt + 0.04, yf + 0.1, 0.23); self.block(at, (q.0 - 0.05, q.1 - 0.04, q.2 - 0.05), (q.0 + 0.05, q.1 + 0.05, q.2 + 0.05), col); }
            }
        }
        // ---- portes : chambranle et linteau de chêne
        for &(x, y, z, dx, dz) in &rec.doors { if !self.name(x, y, z).starts_with("door") { continue; }
            let ax = dz != 0; let s = (dx + dz) as f64; let n = (dx as f64, 0.0, dz as f64); let t = if ax { x } else { z } as f64; let (yf, c, at) = (y as f64, (x, z), (x, y, z));
            for tt in [t - 0.06, t + 1.06] { self.piece(at, on(ax, c, s, tt, yf, 0.05), on(ax, c, s, tt, yf + 2.1, 0.05), n, 0.08, 0.06, OAK); }
            self.piece(at, on(ax, c, s, t - 0.22, yf + 2.08, 0.06), on(ax, c, s, t + 1.22, yf + 2.08, 0.06), n, 0.1, 0.09, OAK);
        }
        // ---- pans de bois : écharpes en chevron et lisses sur le torchis
        for &(xr, zr, yr, step) in &rec.frames {
            let sides = [(true, zr.0, -1.0, xr), (true, zr.1, 1.0, xr), (false, xr.0, -1.0, zr), (false, xr.1, 1.0, zr)];
            for (si, &(ax, fixed, s, span)) in sides.iter().enumerate() { let n = if ax { (0.0, 0.0, s) } else { (s, 0.0, 0.0) };
                let mut posts: Vec<i32> = (span.0..=span.1).filter(|v| (v - span.0) % step == 0).collect(); if *posts.last().unwrap() != span.1 { posts.push(span.1); }
                for (pi, w2) in posts.windows(2).enumerate() { let (p0, p1) = (w2[0], w2[1]); if p1 - p0 < 2 { continue; }
                    let cell = |t: i32, y: i32| if ax { (t, y, fixed) } else { (fixed, y, t) };
                    let infill = |me: &Self, t: i32, y: i32| { let c = cell(t, y); let nm = me.name(c.0, c.1, c.2); nm.starts_with("plaster") || nm == "barn_boards" };
                    let rows: Vec<i32> = (yr.0..=yr.1).filter(|&y| (p0 + 1..p1).all(|t| infill(self, t, y))).collect(); if rows.len() < 2 { continue; }
                    let (ya, yb) = (rows[0] as f64, *rows.last().unwrap() as f64 + 1.0); if rows.len() as f64 != yb - ya { continue; }
                    let c2 = |t: i32| if ax { (t, fixed) } else { (fixed, t) }; let at = cell(p0 + 1, rows[0]); let (t0, t1) = (p0 as f64 + 1.06, p1 as f64 - 0.06);
                    let flip = (pi + si) % 2 == 1; let (ya2, yb2) = (ya + 0.08, yb - 0.08);
                    let (a, b) = if flip { ((t0, yb2), (t1, ya2)) } else { ((t0, ya2), (t1, yb2)) };
                    self.piece(at, on(ax, c2(p0 + 1), s, a.0, a.1, 0.04), on(ax, c2(p0 + 1), s, b.0, b.1, 0.04), n, 0.075, 0.045, DARK);
                    if rows.len() >= 3 { let ym = ya + 1.0; self.piece(at, on(ax, c2(p0 + 1), s, t0 - 0.06, ym, 0.04), on(ax, c2(p0 + 1), s, t1 + 0.06, ym, 0.04), n, 0.06, 0.045, DARK); }
                }
            }
        }
        // ---- abouts des poutres de plancher, qui dépassent des sablières sur les longs pans
        for &(xr, zr, y) in &rec.belts { for x in (xr.0 + 1..xr.1).step_by(2) { for (z, s) in [(zr.0, -1.0), (zr.1, 1.0)] { if !self.name(x, y, z).starts_with("oak_log") { continue; }
            let zf = if s < 0.0 { z as f64 } else { z as f64 + 1.0 }; self.block((x, y, z), (x as f64 + 0.32, y as f64 + 0.3, zf.min(zf + s * 0.22)), (x as f64 + 0.68, y as f64 + 0.7, zf.max(zf + s * 0.22)), END); } } }
        // ---- les lanternes et les lustres pendent à une chaîne jusqu'au plafond ou à la poutre au-dessus ; sans rien
        // au-dessus d'eux, ils se posent sur un poteau jusqu'au sol
        for &(x, y, z) in &rec.lamps { if !self.name(x, y, z).starts_with("lantern") && !self.name(x, y, z).starts_with("chandelier") { continue; }
            if let Some(top) = (y + 1..y + 9).find(|&yy| self.at(x, yy, z) != 0) { for yy in y + 1..top { self.n(x, yy, z, "chain"); } continue; }
            if let Some(bot) = (y - 8..y).rev().find(|&yy| self.at(x, yy, z) != 0) { for yy in bot + 1..y { self.n(x, yy, z, "fence"); } } }
    }
    /// Une corniche de pierre en relief autour d'un rectangle de murs, au sommet de la rangée y.
    pub fn cornice(&mut self, x: (i32, i32), z: (i32, i32), y: i32) {
        let top = variant(id("stone_bricks_slab"), 0, true, false, false);
        for xx in x.0 - 1..=x.1 + 1 { for zz in z.0 - 1..=z.1 + 1 { let ring = xx == x.0 - 1 || xx == x.1 + 1 || zz == z.0 - 1 || zz == z.1 + 1; if ring && !self.filled(xx, y, zz) { self.put(xx, y, zz, top); } } }
    }
    /// La charpente apparente d'un pignon : entrait, poinçon et deux jambes de force, sur la face z (dehors vers s).
    pub fn truss(&mut self, x: (i32, i32), z: i32, _s: f64, y: i32, _top: f64) {
        for xx in x.0..=x.1 { if self.name(xx, y, z) == "planks" { self.log(xx, y, z, 'x'); } } // l'entrait
        let mid = (x.0 + x.1) / 2; for yy in y + 1..y + 12 { if self.name(mid, yy, z) != "planks" { break; } self.log(mid, yy, z, 'y'); } // le poinçon
    }
    pub fn sconce(&mut self, x: i32, y: i32, z: i32, dx: i32, dz: i32) { self.turn(x, y, z, "wall_sconce", dx, dz); }
}

/// Ce que le village doit savoir d'un patron bâti, en repère local : porte (x, sur la façade z = 0), poste (case, poste), lit.
#[derive(Clone, Copy, Debug, Default)]
pub struct Built { pub door: Option<i32>, pub spot: Option<((i32, i32), (i32, i32))>, pub bed: Option<(i32, i32)> }

// =========================================================================================== maison de villageois
/// M-MED-01 : 7 × 6, un niveau et combles. Soubassement de pierre de taille, pans de bois et torchis, porte au milieu
/// encadrée de deux lanternes, deux fenêtres hautes sur chaque flanc, une au dos ; toit de tuiles à pignon sur rue,
/// cheminée de pierre au fond à droite. Variantes : `d` plus long, toit d'ardoise, bardage de planches au lieu du torchis,
/// appentis à bois.
pub(crate) fn maison(p: &mut P, d: i32, roof: &str, infill: &str, woodshed: bool) -> Built {
    let w = 7; p.ground([-1, -2, w, d], 9, [0, 0, w - 1, d - 1], "planks", "stone_bricks");
    p.stair(3, 0, -1, "stone_bricks_stairs", 0, 1, false); // la marche devant la porte
    p.frame((0, w - 1), (0, d - 1), (1, 2), 3, infill); p.belt((0, w - 1), (0, d - 1), 3);
    p.door(3, 1, 0, 0, -1); p.sconce(1, 2, -1, 0, 1); p.sconce(5, 2, -1, 0, 1); p.n(1, 1, 0, "stone_bricks"); p.n(5, 1, 0, "stone_bricks");
    for z in [1, d - 2] { for x in [0, w - 1] { p.win(x, 1, z, false); p.win(x, 2, z, false); } } p.win(3, 2, d - 1, true); p.win(3, 1, d - 1, true);
    for x in 1..w - 1 { for z in 1..d - 1 { p.n(x, 3, z, "planks"); } } p.n(1, 3, 1, "trapdoor");
    for y in 1..=2 { p.put(1, y, 1, variant(id("ladder"), p.face(0, -1), false, false, false)); }
    p.roof((0, w - 1), (0, d - 1), 3, 1, true, roof, Some(infill));
    for z in [0, d - 1] { p.log(3, 4, z, 'y'); p.log(3, 5, z, 'y'); } p.win(2, 4, 0, true); p.win(4, 4, 0, true); // les deux fenêtres du pignon
    // la cheminée : foyer au fond à droite, conduit de pierre qui perce le toit
    p.turn(5, 1, d - 2, "fireplace", 0, 1); for y in 2..=8 { p.n(5, y, d - 2, "stone_bricks"); } p.cap(5, 8, d - 2);
    // l'intérieur : lit, coffre, table et chaise, étagère, lanterne
    p.turn(1, 1, d - 2, "bed", 0, 1); p.turn(1, 1, d - 3, "bed_foot", 0, 1); p.n(5, 1, 1, "chest"); p.n(4, 1, d - 3, "table"); p.n(4, 2, d - 3, "candle"); p.turn(3, 1, d - 3, "chair", 1, 0);
    p.turn(5, 2, 1, "wall_shelf", 1, 0); p.n(3, 2, 2, "lantern"); p.n(2, 1, 2, "carpet_red");
    if woodshed { for z in 1..=3 { p.n(w, 0, z, "cobblestone"); p.n(w, 1, z, "log_pile"); p.n(w, 2, z, "log_pile"); p.stair(w, 3, z, &format!("{roof}_stairs"), -1, 0, false); } }
    Built { door: Some(3), spot: None, bed: Some((2, d - 2)) }
}

// =========================================================================================== forge
/// M-FOR-01 : 7 × 9. Halle ouverte sur la rue entre ses poteaux de chêne, sablière ; soubassement et fond de pierre de
/// taille, torchis en haut des flancs ; le foyer de pierre au fond à droite et sa grande cheminée ; l'enclume au milieu,
/// cuve de trempe, meule, râtelier ; un auvent d'ardoise sur le flanc gauche avec établi et tonneaux ; enseigne au pignon.
pub(crate) fn forge(p: &mut P, master: bool) -> Built {
    let (w, d) = (7, 9); p.ground([-3, -1, w, d], 11, [0, 0, w - 1, d - 1], "cobblestone", "stone_bricks");
    p.frame((0, w - 1), (0, d - 1), (1, 3), 4, "plaster"); for x in 0..w { for z in 0..d { if x == 0 || x == w - 1 || z == d - 1 { p.n(x, 1, z, "stone_bricks"); } } }
    for y in 1..=3 { for x in 1..w - 1 { p.n(x, y, d - 1, "stone_bricks"); } }
    for y in 1..=3 { for x in 1..=3 { p.put(x, y, 0, 0); } p.n(4, y, 0, "oak_log_c"); } p.n(5, 3, 0, "plaster"); p.door(5, 1, 0, 0, -1);
    p.belt((0, w - 1), (0, d - 1), 4); p.win(0, 2, 2, false); p.win(0, 2, 6, false); p.win(w - 1, 2, 4, false);
    p.roof((0, w - 1), (0, d - 1), 4, 1, true, "slate", Some("plaster")); for z in [0, d - 1] { p.log(3, 5, z, 'y'); p.log(3, 6, z, 'y'); }
    p.turn(3, 6, -1, "banner_blue", 0, 1); p.sconce(-1, 2, 0, 1, 0);
    // le foyer : un massif de pierre, la gueule rougeoyante, la hotte, le conduit jusqu'au-dessus du faîte
    p.fill((4, 5), (1, 2), (6, 7), "stone_bricks"); p.n(4, 1, 6, "furnace"); p.n(5, 1, 6, "coal_heap"); p.fill((4, 5), (3, 10), (7, 7), "stone_bricks"); p.cap(4, 10, 7); p.cap(5, 10, 7); p.fill((4, 5), (3, 3), (6, 6), "stone_bricks_slab");
    p.n(3, 1, 3, "anvil"); if master { p.n(2, 1, 5, "anvil"); } p.n(5, 1, 3, "cauldron"); p.turn(1, 1, 3, "grindstone", -1, 0); p.turn(1, 2, 5, "weapon_rack", -1, 0); p.n(3, 1, 7, "bucket"); p.n(3, 3, 2, "lantern"); p.n(3, 3, 6, "lantern");
    p.turn(1, 1, 7, "bed", 0, 1); p.turn(1, 1, 6, "bed_foot", 0, 1); p.n(2, 1, 1, "chest");
    // l'auvent du flanc gauche
    for z in 1..d - 1 { p.n(-1, 0, z, "cobblestone"); p.n(-2, 0, z, "cobblestone"); p.stair(-1, 4, z, "slate_stairs", 1, 0, false); p.stair(-2, 3, z, "slate_stairs", 1, 0, false); }
    for z in [1, 4, d - 2] { for y in 1..=2 { p.n(-2, y, z, "oak_log_c"); } } for z in [2, 3, 5, 6] { p.n(-2, 1, z, "fence"); }
    p.n(-1, 1, 2, "workbench"); p.n(-1, 1, 5, "barrel"); p.n(-1, 1, 6, "barrel"); p.n(-1, 2, 5, "crate");
    Built { door: Some(5), spot: Some(((3, 2), (3, 3))), bed: Some((2, 7)) }
}

// =========================================================================================== auberge
/// M-AUB-01 : 13 × 10, rez-de-chaussée de pierre, étage à pans de bois, combles ; toit d'ardoise au faîte parallèle à la
/// rue, un pignon central en façade (fenêtre, bannière) ; enseigne à la chope ; deux auvents bas sur les flancs avec
/// tables et bancs. Dedans : salle commune (tables, tabourets, cheminée, lustre), comptoir et tonneaux, cuisine ; à l'étage
/// quatre chambres.
pub(crate) fn auberge(p: &mut P) -> Built {
    let (w, d) = (13, 10); p.ground([-3, -2, w + 2, d], 14, [0, 0, w - 1, d - 1], "planks", "stone_bricks");
    for x in 4..=8 { p.stair(x, 0, -1, "stone_bricks_stairs", 0, 1, false); }
    // rez-de-chaussée : pierre de taille, poteaux de chêne tous les trois blocs
    for x in 0..w { for z in 0..d { if x > 0 && x < w - 1 && z > 0 && z < d - 1 { continue; } for y in 1..=3 { let post = (x % 3 == 0 && (z == 0 || z == d - 1)) || (z % 3 == 0 && (x == 0 || x == w - 1)); p.n(x, y, z, if post { "oak_log_c" } else if y == 1 { "stone_bricks" } else { "stone_bricks" }); } } }
    p.door(6, 1, 0, 0, -1); for x in [1, 2, 4, 8, 10, 11] { p.win(x, 2, 0, true); p.win(x, 2, d - 1, true); } for z in [1, 2, 4, 5, 7, 8] { p.win(0, 2, z, false); p.win(w - 1, 2, z, false); }
    p.belt((0, w - 1), (0, d - 1), 4); for x in 1..w - 1 { for z in 1..d - 1 { p.n(x, 4, z, "planks"); } }
    // l'étage : pans de bois et torchis, fenêtres fleuries
    p.frame((0, w - 1), (0, d - 1), (5, 6), 3, "plaster"); p.belt((0, w - 1), (0, d - 1), 7);
    for x in [1, 2, 4, 5, 7, 8, 10, 11] { p.win(x, 5, 0, true); p.win(x, 5, d - 1, true); } for z in [1, 2, 4, 5, 7, 8] { p.win(0, 5, z, false); p.win(w - 1, 5, z, false); }
    p.roof((0, w - 1), (0, d - 1), 7, 1, false, "slate", Some("plaster"));
    p.roof((4, 8), (-1, 4), 7, 1, true, "slate", Some("plaster")); p.win(6, 8, -1, true); p.win(6, 9, -1, true); p.log(4, 7, -1, 'y'); p.log(8, 7, -1, 'y');
    for x in 4..=8 { p.n(x, 7, -1, if x == 4 || x == 8 { "oak_log_c" } else { "plaster" }); } for x in 5..=7 { p.n(x, 4, -1, "oak_log_c"); p.n(x, 5, -1, "plaster"); p.n(x, 6, -1, "plaster"); } p.win(6, 5, -1, true); p.win(5, 5, -1, true); p.win(7, 5, -1, true);
    p.turn(6, 3, -1, "banner_red", 0, 1); p.turn(8, 3, -1, "banner_gold", 0, 1); p.sconce(4, 2, -1, 0, 1); p.sconce(8, 2, -1, 0, 1);
    // les cheminées
    for y in 1..=13 { p.n(w - 2, y, d - 2, "stone_bricks"); } p.cap(w - 2, 13, d - 2); p.turn(w - 2, 1, d - 3, "fireplace", 0, 1);
    // les auvents des flancs, bancs et tables dessous
    for (side, x0) in [(-1, -1), (1, w)] { for z in 1..d - 1 { for k in 0..2 { let x = x0 + side * k; p.n(x, 0, z, "cobblestone"); p.stair(x, 4 - k, z, "slate_stairs", -side, 0, false); } }
        for z in [1, 4, d - 2] { for y in 1..=2 { p.n(x0 + side, y, z, "oak_log_c"); } } p.n(x0, 3, 2, "lantern"); p.n(x0, 1, 3, "round_table"); p.n(x0, 1, 2, "stool"); p.n(x0, 1, 6, "round_table"); p.n(x0, 1, 7, "stool"); p.n(x0 + side, 1, 5, "barrel"); }
    // la salle : escalier à gauche, comptoir et tonneaux au fond à droite, tables rondes, lustre ; le lit de l'aubergiste
    for k in 0..4 { p.stair(1, 1 + k, 4 + k, "planks_stairs", 0, 1, false); if k < 3 { p.put(1, 4, 4 + k, 0); } }
    for x in 7..=10 { p.n(x, 1, 5, "table"); p.n(x, 1, 7, "barrel"); } p.n(8, 2, 5, "bottles"); p.n(10, 2, 5, "cheese"); p.n(9, 2, 7, "barrel");
    for (x, z) in [(3, 2), (6, 3), (3, 7)] { p.n(x, 1, z, "round_table"); p.n(x, 2, z, "candle"); p.n(x + 1, 1, z, "stool"); p.n(x, 1, z + 1, "stool"); p.n(x - 1, 1, z, "stool"); }
    p.n(6, 3, 5, "chandelier"); p.n(5, 1, 8, "cauldron"); p.n(6, 1, 8, "furnace");
    p.turn(11, 1, 2, "bed", 0, 1); p.turn(11, 1, 1, "bed_foot", 0, 1);
    // l'étage : quatre chambres de part et d'autre d'un couloir
    for z in 1..d - 1 { if z != 4 && z != 5 { p.n(4, 5, z, "planks"); p.n(8, 5, z, "planks"); p.n(4, 6, z, "planks"); p.n(8, 6, z, "planks"); } }
    for (x, z) in [(2, 1), (2, 7), (10, 1), (10, 7)] { p.turn(x, 5, z + 1, "bed", 0, if z == 1 { -1 } else { 1 }); p.turn(x, 5, if z == 1 { z + 2 } else { z }, "bed_foot", 0, if z == 1 { -1 } else { 1 }); p.n(x + 1, 5, z + 1, "chest"); }
    p.n(6, 6, 4, "lantern");
    Built { door: Some(6), spot: Some(((8, 4), (8, 5))), bed: Some((10, 2)) }
}

// =========================================================================================== moulin
/// M-005 : 9 × 9, tour octogonale : soubassement de pierre (rez-de-chaussée), étage à pans de bois et torchis plus
/// étroit, balcon de bois à garde-corps tout autour, toit pyramidal de tuiles, ailes entoilées sur la façade.
pub(crate) fn moulin(p: &mut P, h: f64) -> Built {
    let (c, r) = (4, 4); let oct = |dx: i32, dz: i32, r: i32| dx.abs().max(dz.abs()) <= r && dx.abs() + dz.abs() <= r + r / 2;
    let rim = |dx: i32, dz: i32, r: i32| oct(dx, dz, r) && (!oct(dx + 1, dz, r) || !oct(dx - 1, dz, r) || !oct(dx, dz + 1, r) || !oct(dx, dz - 1, r));
    p.ground([-2, -2, 10, 9], 20, [0, 0, 8, 8], "planks", "cobblestone");
    for dx in -r..=r { for dz in -r..=r { if !oct(dx, dz, r) { continue; } let (x, z) = (c + dx, c + dz);
        p.n(x, 0, z, if rim(dx, dz, r) { "cobblestone" } else { "planks" }); if rim(dx, dz, r) { for y in 1..=3 { p.n(x, y, z, if (x + z + y) % 3 == 0 { "cobblestone" } else { "stone_bricks" }); } } p.n(x, 4, z, "planks"); } }
    p.door(c, 1, 0, 0, -1); p.win(0, 2, c, false); p.win(8, 2, c, false); p.win(c, 2, 8, true);
    for dx in -r..=r { for dz in -r..=r { if oct(dx, dz, r) && !oct(dx, dz, r - 1) { p.n(c + dx, 5, c + dz, "fence"); } } } // le garde-corps du balcon
    // deux étages à pans de bois au-dessus du balcon : sablière à mi-hauteur, plancher entre les deux
    let r2 = 3; for dx in -r2..=r2 { for dz in -r2..=r2 { if !oct(dx, dz, r2) { continue; }
        if !rim(dx, dz, r2) { p.n(c + dx, 9, c + dz, "planks"); continue; }
        let corner = dx.abs() == dz.abs() || dx == 0 || dz == 0; for y in 5..=12 { if corner || y == 9 || y == 12 { p.log(c + dx, y, c + dz, 'y'); } else { p.n(c + dx, y, c + dz, "plaster"); } } } }
    for (x, z, ax) in [(c, c + 3, true), (c - 3, c, false), (c + 3, c, false)] { p.win(x, 6, z, ax); p.win(x, 7, z, ax); } p.door(c, 5, c - 3, 0, -1);
    for (x, z, ax) in [(c, c - 3, true), (c, c + 3, true), (c - 3, c, false), (c + 3, c, false)] { p.win(x, 10, z, ax); p.win(x, 11, z, ax); }
    p.pyramid(c, c, 4, 13, "roof_tiles"); p.n(c, 18, c, "oak_log_c");
    // les escaliers : du rez-de-chaussée au balcon, du premier au second étage ; les meules, les sacs, la trémie
    for k in 0..4 { p.stair(c - 2, 1 + k, 2 + k, "planks_stairs", 0, 1, false); if k < 3 { p.put(c - 2, 4, 2 + k, 0); } }
    for k in 0..4 { p.stair(c - 2 + k, 5 + k, c - 1, "planks_stairs", 1, 0, false); if k < 3 { p.put(c - 2 + k, 9, c - 1, 0); } }
    p.n(c + 1, 1, c + 1, "grindstone"); p.n(c + 2, 1, c - 1, "sack"); p.n(c + 2, 1, c, "sack"); p.n(c + 1, 1, c + 2, "barrel"); p.n(c, 3, c, "lantern"); p.n(c + 1, 5, c + 1, "sack"); p.n(c - 1, 5, c + 1, "crate");
    p.n(c, 10, c, "grindstone"); p.n(c + 1, 10, c + 1, "sack"); p.n(c - 1, 10, c + 1, "sack"); p.n(c, 12, c, "lantern"); p.n(c, 8, c, "lantern");
    // l'arbre des ailes, qui sort du toit en façade
    for z in -1..=2 { p.log(c, 15, z, 'z'); }
    let at = p.lot.cell(p.lx(c), p.y0 + 15, -1); let hub = p.lot.p(p.lx(c) as f64 + 0.5, (p.y0 + 15) as f64 + 0.5, -0.6); crate::batisse::sails(p.k, at, p.lot, hub, h);
    Built { door: Some(c), spot: None, bed: None }
}

// =========================================================================================== tanneur
/// M-TAN-01 : 12 × 9. Une maison de pierre et de colombages (8 × 9) avec ses cuves de tan et de chaux, l'établi, les
/// peaux aux murs, un comble ; à droite un séchoir ouvert sous le même toit de tuiles, perches de peaux et barrières ;
/// devant, deux cuves au soleil derrière une clôture.
pub(crate) fn tanneur(p: &mut P) -> Built {
    let (w, d) = (8, 9); p.ground([-1, -3, 12, d], 11, [0, 0, 11, d - 1], "planks", "stone_bricks");
    for x in w..12 { for z in 0..d { p.n(x, 0, z, "cobblestone"); } }
    p.frame((0, w - 1), (0, d - 1), (1, 3), 3, "plaster"); for x in 0..w { for z in [0, d - 1] { p.n(x, 1, z, "stone_bricks"); } } for z in 0..d { p.n(0, 1, z, "stone_bricks"); }
    p.belt((0, w - 1), (0, d - 1), 4); p.door(3, 1, 0, 0, -1); p.n(3, 3, 0, "plaster"); p.win(1, 2, 0, true); p.win(5, 2, 0, true); p.win(0, 2, 3, false); p.win(0, 2, 6, false); p.win(3, 2, d - 1, true);
    for x in 1..w - 1 { for z in 1..d - 1 { p.n(x, 4, z, "planks"); } }
    p.roof((0, 11), (0, d - 1), 4, 1, false, "roof_tiles", Some("plaster")); // un seul toit, faîte parallèle à la rue, sur la maison et le séchoir
    p.door(w - 1, 1, 2, 1, 0); // une porte donne sur le séchoir
    for x in w..12 { for z in 0..d { p.n(x, 4, z, "planks"); } } // le grenier au-dessus du séchoir // la maison s'ouvre sur le séchoir
    for (x, z) in [(11, 0), (11, 4), (11, d - 1), (w + 1, 0), (w + 1, d - 1)] { for y in 1..=4 { p.n(x, y, z, "oak_log_c"); } }
    for z in 1..d - 1 { if z != 4 { p.n(11, 1, z, "fence"); } } for x in w..11 { p.turn(x, 3, d - 1, "hides", 0, 1); } for z in [2, 6] { p.turn(10, 3, z, "hides", 1, 0); p.turn(10, 2, z, "hides", 1, 0); }
    p.n(9, 1, 2, "tanning_vat"); p.n(9, 1, 6, "lime_vat"); p.n(10, 1, 4, "barrel");
    for y in 1..=8 { p.n(5, y, d - 2, "stone_bricks"); } p.cap(5, 8, d - 2); p.turn(5, 1, d - 3, "fireplace", 0, 1);
    // dedans : cuves, établi (le tanneur s'y tient), peaux aux murs, coffre, lit
    p.n(2, 1, d - 2, "tanning_vat"); p.n(3, 1, d - 2, "lime_vat"); p.n(4, 1, 3, "workbench"); p.n(5, 1, 3, "table"); p.turn(1, 2, 5, "hides", -1, 0); p.turn(3, 2, d - 1 - 1, "chain", 0, 1);
    p.turn(1, 1, 2, "bed", 0, 1); p.turn(1, 1, 1, "bed_foot", 0, 1); p.n(6, 1, 1, "chest"); p.n(4, 3, 4, "lantern");
    for k in 0..3 { p.stair(6, 1 + k, 4 + k, "planks_stairs", 0, 1, false); p.put(6, 4, 4 + k, 0); }
    // devant : deux cuves au soleil, la clôture
    for x in 8..=11 { p.n(x, 0, -2, "cobblestone"); p.n(x, 0, -1, "cobblestone"); } p.n(9, 1, -2, "tanning_vat"); p.n(10, 1, -2, "lime_vat"); for x in 8..=11 { p.n(x, 1, -3, "fence"); } p.sconce(2, 2, -1, 0, 1); p.turn(4, 3, -1, "banner_red", 0, 1);
    Built { door: Some(3), spot: Some(((4, 2), (4, 3))), bed: Some((2, 2)) }
}

// =========================================================================================== église
/// M-EGL-01 : nef de pierre (11 × 17, la largeur sur la rue), contreforts, hauts vitraux colorés sur les flancs, rosace
/// au pignon, toit de tuiles ; clocher carré en façade sur l'entrée (porte, abat-sons, cloche, flèche de tuiles et croix
/// d'or) ; chœur plus étroit au fond. Dedans : bancs de part et d'autre de l'allée, tapis rouge, autel, lanternes, bannières.
pub(crate) fn eglise(p: &mut P) -> Built {
    let (w, d) = (11, 17); p.ground([-1, -2, w, d], 23, [0, 2, w - 1, d - 3], "planks", "stone_bricks");
    let (x0, x1, z0, z1) = (1, 9, 2, 14);
    for x in x0..=x1 { for z in z0..=z1 { let edge = x == x0 || x == x1 || z == z0 || z == z1; if !edge { continue; } for y in 1..=7 { p.n(x, y, z, if y == 1 { "cobblestone" } else { "stone_bricks" }); } } }
    for z in (z0..=z1).step_by(3) { for x in [x0 - 1, x1 + 1] { for y in 1..=5 { p.n(x, y, z, "stone_bricks"); } p.stair(x, 6, z, "stone_bricks_stairs", if x < x0 { 1 } else { -1 }, 0, false); } } // contreforts
    for z in [4, 7, 10, 12] { for x in [x0, x1] { for y in 3..=5 { p.n(x, y, z, "stained_glass"); } } }
    p.roof((x0, x1), (z0, z1), 8, 1, true, "roof_tiles", Some("stone_bricks"));
    // le chœur, plus étroit, au chevet
    for x in 3..=7 { for z in z1 + 1..=z1 + 2 { let edge = x == 3 || x == 7 || z == z1 + 2; p.n(x, 0, z, "planks"); for y in 1..=5 { if edge { p.n(x, y, z, "stone_bricks"); } else { p.put(x, y, z, 0); } } } }
    for x in 4..=6 { for y in 1..=4 { p.put(x, y, z1, 0); } } p.n(5, 3, z1 + 2, "stained_glass"); p.n(5, 4, z1 + 2, "stained_glass");
    p.roof((3, 7), (z1 + 1, z1 + 2), 6, 1, true, "roof_tiles", Some("stone_bricks"));
    for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] { p.n(5 + dx, 11 + dy, z1, "stained_glass"); } // la rosace du chevet
    // le clocher carré en façade, sur l'entrée
    let (tx0, tx1, tz0, tz1) = (3, 7, 0, 4);
    for x in tx0..=tx1 { for z in tz0..=tz1 { let edge = x == tx0 || x == tx1 || z == tz0 || z == tz1; p.n(x, 0, z, if edge { "stone_bricks" } else { "planks" }); for y in 1..=17 { if edge { p.n(x, y, z, if y == 1 { "cobblestone" } else { "stone_bricks" }); } else { p.put(x, y, z, 0); } } } }
    for x in tx0 + 1..tx1 { for y in 1..=5 { p.put(x, y, tz1, 0); } for y in 1..=5 { p.put(x, y, z0, 0); } } // la tour s'ouvre sur la nef
    p.door(5, 1, tz0, 0, -1); p.n(5, 3, tz0, "stained_glass");
    for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] { p.n(5 + dx, 8 + dy, tz0, "stained_glass"); } // la rosace de la façade
    for y in 14..=15 { p.put(5, y, tz0, 0); p.put(5, y, tz1, 0); p.put(tx0, y, 2, 0); p.put(tx1, y, 2, 0); } // les abat-sons
    for x in tx0 + 1..tx1 { for z in tz0 + 1..tz1 { p.n(x, 12, z, "planks"); p.n(x, 17, z, "stone_bricks"); } }
    p.n(5, 16, 2, "chain"); p.n(5, 15, 2, "bell");
    p.pyramid(5, 2, 2, 18, "roof_tiles"); p.n(5, 21, 2, "cross");
    for y in [7, 11, 16] { p.cornice((tx0, tx1), (tz0, tz1), y); } p.cornice((x0, x1), (z0, z1), 7);
    // dedans : l'allée au tapis rouge, les bancs, l'autel et ses cierges, lustres et bannières
    for z in 1..z1 { p.n(5, 1, z, "carpet_red"); } for z in (6..=12).step_by(2) { for x in [2, 3, 4, 6, 7, 8] { p.turn(x, 1, z, "bench", 0, -1); } }
    p.n(5, 1, z1 + 1, "table"); p.n(5, 2, z1 + 1, "candle"); p.n(4, 1, z1 + 1, "candle"); p.n(6, 1, z1 + 1, "candle");
    for z in [6, 9, 12] { p.turn(x0 + 1, 3, z, "banner_red", -1, 0); p.turn(x1 - 1, 3, z, "banner_red", 1, 0); } for z in [7, 10] { p.n(5, 6, z, "chandelier"); }
    for x in [2, 8] { p.turn(x, 4, 1, "banner_red", 0, 1); } p.sconce(4, 3, -1, 0, 1); p.sconce(6, 3, -1, 0, 1);
    Built { door: Some(5), spot: None, bed: None }
}

// =========================================================================================== mairies
/// M-MAI-02 : la petite mairie des villages, 9 × 7 : soubassement de pierre, pans de bois, toit de tuiles à pignon sur la
/// rue, un clocheton au milieu du faîte avec sa cloche, la bannière au lion sur le pignon, deux lanternes, un perron.
/// Dedans : deux tables sur tréteaux de part et d'autre d'un tapis rouge.
pub(crate) fn petite_mairie(p: &mut P) -> Built {
    let (w, d) = (9, 7); p.ground([-1, -2, w, d], 12, [0, 0, w - 1, d - 1], "planks", "stone_bricks");
    for x in 3..=5 { p.stair(x, 0, -1, "stone_bricks_stairs", 0, 1, false); }
    for x in 0..w { for z in 0..d { if x > 0 && x < w - 1 && z > 0 && z < d - 1 { continue; } p.n(x, 1, z, "stone_bricks"); } }
    p.frame((0, w - 1), (0, d - 1), (2, 3), 2, "plaster"); p.belt((0, w - 1), (0, d - 1), 4); p.door(4, 1, 0, 0, -1); p.n(4, 3, 0, "plaster");
    p.win(0, 2, 3, false); p.win(w - 1, 2, 3, false); p.win(4, 2, d - 1, true); p.win(4, 3, d - 1, true); p.win(2, 3, 0, true); p.win(6, 3, 0, true);
    p.roof((0, w - 1), (0, d - 1), 4, 1, true, "roof_tiles", Some("plaster")); for z in [0, d - 1] { p.log(4, 5, z, 'y'); p.log(4, 6, z, 'y'); }
    p.turn(4, 6, -1, "banner_red", 0, 1); p.sconce(3, 2, -1, 0, 1); p.sconce(5, 2, -1, 0, 1);
    // le clocheton : quatre poteaux sur le faîte, la cloche, un petit toit pyramidal
    let (cx, cz) = (4, 3); for (dx, dz) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] { for y in 9..=10 { p.n(cx + dx, y, cz + dz, "oak_log_c"); } } p.n(cx, 10, cz, "bell"); p.pyramid(cx, cz, 1, 11, "roof_tiles");
    for x in 1..w - 1 { p.n(x, 0, 3, "planks"); } for z in 1..d - 1 { p.n(4, 1, z, "carpet_red"); } for z in 2..=4 { p.n(2, 1, z, "table"); p.n(6, 1, z, "table"); } p.n(2, 2, 3, "books_pile"); p.n(6, 2, 3, "candle"); p.n(4, 3, 3, "chandelier");
    Built { door: Some(4), spot: None, bed: None }
}
/// M-MAI-01 : la grande mairie des bourgs, 19 × 15 : grand corps central à deux niveaux (pierre de taille au
/// rez-de-chaussée, pans de bois à l'étage), deux ailes en avancée aux pignons sur la rue, une tour d'horloge au-dessus
/// de l'entrée (horloge, cloche, flèche d'ardoise, drapeau), perron de pierre, bannières bleues ; toits d'ardoise.
/// Dedans : salle du conseil (grande table, tapis rouge, bannières, lustres), escalier, salle haute.
pub(crate) fn grande_mairie(p: &mut P) -> Built {
    let (w, d) = (19, 15); p.ground([-1, -3, w, d], 25, [0, 0, w - 1, d - 1], "planks", "stone_bricks");
    // les volumes : corps central (x 4..14, z 2..12), deux ailes en avancée (x 0..4 et 14..18, z 0..12), l'avant-corps
    // de l'entrée (x 7..11, z 0..2) ; rez-de-chaussée de pierre de taille, étage à pans de bois
    let parts = [(4, 14, 2, 12), (0, 4, 0, 12), (14, 18, 0, 12), (7, 11, 0, 2)];
    for &(x0, x1, z0, z1) in &parts { for x in x0..=x1 { for z in z0..=z1 { p.n(x, 0, z, "planks"); } }
        for x in x0..=x1 { for z in z0..=z1 { let edge = x == x0 || x == x1 || z == z0 || z == z1; if !edge { continue; } for y in 1..=4 { p.n(x, y, z, if y == 1 { "cobblestone" } else { "stone_bricks" }); } } }
        p.belt((x0, x1), (z0, z1), 5); p.frame((x0, x1), (z0, z1), (6, 8), 2, "plaster"); p.belt((x0, x1), (z0, z1), 9); }
    for &(x0, x1, z0, z1) in &parts { for x in x0 + 1..x1 { for z in z0 + 1..z1 { for y in 1..=8 { if y != 5 { p.put(x, y, z, 0); } else { p.n(x, 5, z, "planks"); } } } } } // les volumes communiquent
    for z in 3..12 { for y in 1..=8 { if y != 5 { p.put(4, y, z, 0); p.put(14, y, z, 0); } } } for x in 8..=10 { for y in 1..=8 { if y != 5 { p.put(x, y, 2, 0); } else { p.n(x, 5, 2, "planks"); } } }
    // fenêtres : hautes au rez-de-chaussée, à l'étage entre les colombages
    for x in [1, 3, 15, 17] { for y in [2, 3, 7] { p.win(x, y, 0, true); } } for x in [5, 6, 12, 13] { for y in [2, 3, 7] { p.win(x, y, 2, true); } }
    for x in [8, 10] { p.win(x, 2, 0, true); p.win(x, 3, 0, true); } for x in 8..=10 { p.win(x, 7, 0, true); }
    for z in [2, 4, 6, 8, 10] { for y in [2, 7] { p.win(0, y, z, false); p.win(w - 1, y, z, false); } } for x in [2, 6, 9, 12, 16] { for y in [2, 7] { p.win(x, y, 12, true); } }
    // toits d'ardoise : le corps central (faîte parallèle à la rue), les ailes et l'avant-corps (pignons sur la rue)
    p.roof((4, 14), (2, 12), 9, 1, false, "slate", Some("plaster")); p.roof((0, 4), (0, 12), 9, 1, true, "slate", Some("plaster")); p.roof((14, 18), (0, 12), 9, 1, true, "slate", Some("plaster"));
    p.roof((7, 11), (0, 3), 9, 1, true, "slate", Some("plaster")); p.win(9, 10, 0, true); p.log(9, 11, 0, 'y');
    // la tour de l'horloge, carrée, au milieu du toit : pans de bois, cadrans, chambre des cloches ajourée, corniche, flèche d'ardoise
    let (tx0, tx1, tz0, tz1) = (7, 11, 5, 9);
    for x in tx0..=tx1 { for z in tz0..=tz1 { let edge = x == tx0 || x == tx1 || z == tz0 || z == tz1; for y in 10..=19 { if !edge { p.put(x, y, z, 0); continue; }
        let corner = (x == tx0 || x == tx1) && (z == tz0 || z == tz1); p.n(x, y, z, if corner || y == 14 || y == 19 { "oak_log_c" } else if y < 14 { "plaster" } else { "stone_bricks" }); } } }
    for x in tx0 + 1..tx1 { for z in tz0 + 1..tz1 { p.n(x, 15, z, "planks"); } }
    for y in 16..=18 { p.put(9, y, tz0, 0); p.put(9, y, tz1, 0); p.put(tx0, y, 7, 0); p.put(tx1, y, 7, 0); } p.n(9, 18, 7, "chain"); p.n(9, 17, 7, "bell"); // les abat-sons et la cloche
    p.turn(9, 13, tz0 - 1, "wall_clock", 0, 1); p.turn(9, 13, tz1 + 1, "wall_clock", 0, -1); // les cadrans, au-dessus des toits
    p.cornice((tx0, tx1), (tz0, tz1), 19); p.pyramid(9, 7, 3, 20, "slate"); p.n(9, 24, 7, "flag");
    // perron de pierre, bannières bleues, lanternes
    for x in 7..=11 { p.stair(x, 0, -1, "stone_bricks_stairs", 0, 1, false); } p.door(9, 1, 0, 0, -1);
    for x in [6, 12] { p.turn(x, 3, 1, "banner_blue", 0, 1); } for x in [2, 16] { p.turn(x, 7, -1, "banner_blue", 0, 1); } p.turn(9, 4, -1, "banner_blue", 0, 1); for x in [7, 11] { p.n(x, 1, -2, "fence"); p.n(x, 2, -2, "lantern"); }
    for &(x, z) in &[(2, 6), (16, 6)] { for y in 1..=14 { p.n(x, y, z, "stone_bricks"); } p.cap(x, 14, z); p.turn(x, 1, z - 1, "fireplace", 0, 1); }
    // la salle du conseil : tapis rouge depuis la porte, grande table et chaises, lustres ; l'escalier vers la salle haute
    for z in 1..=10 { p.n(9, 1, z, "carpet_red"); } for z in 5..=9 { p.n(8, 1, z, "table"); p.n(10, 1, z, "table"); p.turn(7, 1, z, "chair", -1, 0); p.turn(11, 1, z, "chair", 1, 0); } p.n(8, 2, 6, "books_pile"); p.n(10, 2, 8, "candle");
    for x in [6, 12] { p.n(x, 4, 7, "chandelier"); } for k in 0..4 { p.stair(16, 1 + k, 7 + k, "planks_stairs", 0, 1, false); if k < 3 { p.put(16, 5, 7 + k, 0); } }
    for z in 3..=11 { p.n(9, 6, z, "carpet_red"); } for x in 7..=11 { p.n(x, 6, 7, "table"); } p.n(9, 7, 7, "candle"); p.n(9, 8, 9, "chandelier");
    Built { door: Some(9), spot: None, bed: None }
}

// =========================================================================================== atelier du bûcheron
/// M-BC-01 : 14 × 9. L'atelier fermé (8 × 9) de pierre et de colombages, porte et bannière aux haches, cheminée ; à
/// droite un grand appentis ouvert sur poteaux, sous le même toit de tuiles : bûches empilées, chevalets de sciage,
/// billot, planches. Dedans : l'établi (le charpentier s'y tient), des planches, des outils, un lit.
pub(crate) fn atelier_bois(p: &mut P) -> Built {
    let (w, d) = (8, 9); p.ground([-1, -2, 14, d], 11, [0, 0, 13, d - 1], "planks", "stone_bricks");
    for x in w..14 { for z in 0..d { p.n(x, 0, z, "planks"); } }
    p.frame((0, w - 1), (0, d - 1), (1, 3), 3, "plaster"); for x in 0..w { for z in [0, d - 1] { p.n(x, 1, z, "stone_bricks"); } } for z in 0..d { p.n(0, 1, z, "stone_bricks"); p.n(w - 1, 1, z, "stone_bricks"); }
    p.belt((0, w - 1), (0, d - 1), 4); p.door(3, 1, 0, 0, -1); p.n(3, 3, 0, "plaster"); p.win(1, 2, 0, true); p.win(5, 2, 0, true); p.win(0, 2, 4, false); p.win(3, 2, d - 1, true); p.win(5, 2, d - 1, true);
    p.roof((0, 13), (0, d - 1), 4, 1, false, "roof_tiles", Some("plaster"));
    for (x, z) in [(13, 0), (13, 4), (13, d - 1), (10, 0), (10, d - 1)] { for y in 1..=4 { p.n(x, y, z, "oak_log_c"); } }
    p.turn(3, 3, -1, "banner_red", 0, 1); p.sconce(1, 3, -1, 0, 1); p.sconce(6, 3, -1, 0, 1); p.sconce(13, 3, -1, 0, 1);
    for x in 1..w - 1 { for z in 1..d - 1 { p.n(x, 4, z, "planks"); } } // le plafond de l'atelier
    for y in 1..=8 { p.n(6, y, d - 2, "stone_bricks"); } p.cap(6, 8, d - 2); p.turn(6, 1, d - 3, "fireplace", 0, 1);
    // l'appentis : piles de bûches, chevalets, billot, planches
    for z in 5..d - 1 { for x in [11, 12] { p.n(x, 1, z, "log_pile"); if z % 2 == 1 { p.n(x, 2, z, "log_pile"); } } } p.turn(9, 1, 2, "sawhorse", 1, 0); p.turn(11, 1, 2, "sawhorse", 1, 0); p.n(9, 1, 6, "chopping_block"); p.n(12, 1, 1, "planks"); p.n(12, 2, 1, "planks_slab");
    p.n(10, 3, 4, "lantern");
    // l'atelier : établi, étagère d'outils, planches, coffre, lit
    p.n(3, 1, 4, "workbench"); p.n(4, 1, 4, "table"); p.turn(1, 2, 4, "tool_rack", -1, 0); p.n(1, 1, 6, "log_pile"); p.n(2, 1, 7, "crate"); p.n(5, 1, 1, "chest");
    p.turn(1, 1, 2, "bed", 0, 1); p.turn(1, 1, 1, "bed_foot", 0, 1); p.n(4, 3, 3, "lantern");
    Built { door: Some(3), spot: Some(((3, 3), (3, 4))), bed: Some((2, 2)) }
}

// =========================================================================================== ferme
/// M-FER-01 : 15 × 11, de plain-pied. Le logis à pans de bois (x 2..8) au pignon sur la rue, porte encadrée de deux
/// lanternes, cheminée de pierre ; à gauche un appentis ouvert où l'on serre le foin ; à droite, un peu en retrait, la
/// grange basse en planches, ouverte sur la cour derrière sa barrière, ses ballots et son abreuvoir. Toits de tuiles.
pub(crate) fn ferme(p: &mut P, roof: &str) -> Built {
    let (w, d) = (15, 11); p.ground([-1, -2, w, d], 11, [0, 0, w - 1, d - 1], "planks", "stone_bricks");
    for x in 0..w { for z in 0..d { if !(2..=8).contains(&x) { p.n(x, 0, z, "cobblestone"); } } }
    // le logis
    let (x0, x1) = (2, 8); p.frame((x0, x1), (0, d - 1), (1, 3), 3, "plaster"); for x in x0..=x1 { for z in 0..d { if x == x0 || x == x1 || z == 0 || z == d - 1 { p.n(x, 1, z, "stone_bricks"); } } }
    p.belt((x0, x1), (0, d - 1), 4); p.door(5, 1, 0, 0, -1); p.n(5, 3, 0, "plaster"); p.sconce(4, 2, -1, 0, 1); p.sconce(6, 2, -1, 0, 1); p.stair(5, 0, -1, "stone_bricks_stairs", 0, 1, false);
    for x in [3, 7] { p.win(x, 2, 0, true); p.win(x, 2, d - 1, true); } for z in [2, 5, 8] { p.win(x0, 2, z, false); } p.win(5, 2, d - 1, true);
    p.roof((x0, x1), (0, d - 1), 4, 1, true, roof, Some("plaster")); for z in [0, d - 1] { p.log(5, 5, z, 'y'); p.log(5, 6, z, 'y'); } p.win(4, 5, 0, true); p.win(6, 5, 0, true);
    for x in x0 + 1..x1 { for z in 5..d - 1 { p.n(x, 4, z, "planks"); } } for y in 1..=3 { p.put(3, y, 5, variant(id("ladder"), p.face(0, 1), false, false, false)); } p.n(3, 4, 5, "trapdoor");
    for (x, z) in [(4, 7), (4, 8), (6, 8)] { p.n(x, 5, z, "hay_bale"); } // le foin au grenier
    for y in 1..=10 { p.n(7, y, 7, "stone_bricks"); } p.cap(7, 10, 7); p.turn(7, 1, 6, "fireplace", 0, 1);
    p.turn(3, 1, 9, "bed", 0, 1); p.turn(3, 1, 8, "bed_foot", 0, 1); p.n(5, 1, 4, "table"); p.turn(4, 1, 4, "chair", 1, 0); p.n(5, 2, 4, "candle"); p.n(7, 1, 1, "chest"); p.n(7, 1, 2, "barrel"); p.n(3, 1, 1, "sack"); p.n(5, 3, 3, "lantern"); p.n(5, 1, 2, "carpet_green");
    // l'appentis du foin, à gauche
    for z in 1..d - 1 { p.stair(1, 4, z, &format!("{roof}_stairs"), 1, 0, false); p.stair(0, 3, z, &format!("{roof}_stairs"), 1, 0, false); }
    for z in [1, 5, d - 2] { for y in 1..=2 { p.n(0, y, z, "oak_log_c"); } } for z in [2, 3, 4, 6] { p.n(0, 1, z, "fence"); }
    for (x, z, y) in [(1, 6, 1), (1, 7, 1), (1, 8, 1), (1, 7, 2), (1, 8, 2), (0, 8, 1), (0, 7, 1)] { p.n(x, y, z, "hay_bale"); } p.n(1, 1, 2, "cart");
    // la grange basse, à droite, ouverte sur la cour
    let (g0, g1, gz0, gz1) = (9, 14, 2, 8);
    for x in g0..=g1 { for z in gz0..=gz1 { let back = z == gz1 || x == g1; if back { p.n(x, 1, z, "stone_bricks"); p.n(x, 2, z, "barn_boards"); } } }
    for (x, z) in [(g0, gz0), (11, gz0), (g1, gz0), (g1, 5), (g0, gz1), (g1, gz1)] { p.log(x, 1, z, 'y'); p.log(x, 2, z, 'y'); }
    p.roof((g0, g1), (gz0, gz1), 3, 1, false, roof, Some("barn_boards")); p.win(g1, 2, 4, false); p.win(g1, 2, 6, false);
    for x in g0 + 1..g1 { if x != 11 { p.n(x, 1, gz0, "fence"); } } p.n(12, 1, 3, "trough"); p.n(10, 1, 7, "hay_bale"); p.n(11, 1, 7, "hay_bale"); p.n(10, 2, 7, "hay_bale"); p.n(13, 1, 7, "crate"); p.n(13, 1, 6, "barrel"); p.n(12, 2, 5, "lantern");
    // la cour : ballots au soleil, barrière
    for x in 10..=14 { p.n(x, 1, -1, "fence"); } for (x, z, y) in [(13, 0, 1), (14, 0, 1), (14, 1, 1), (14, 0, 2)] { p.n(x, y, z, "hay_bale"); }
    Built { door: Some(5), spot: None, bed: Some((4, 9)) }
}

// =========================================================================================== étable et écurie
/// Une halle d'élevage au pignon sur la rue : soubassement de pierre, poteaux de chêne et sablière, murs de planches,
/// toit de tuiles, façade ouverte derrière sa barrière, l'enseigne au pignon. Rend l'emprise des stalles.
fn halle(p: &mut P, w: i32, d: i32, roof: &str, sign: &str, gate: i32) {
    p.ground([-1, -1, w, d], 12, [0, 0, w - 1, d - 1], "planks", "stone_bricks");
    for x in 1..w - 1 { for z in 1..d - 1 { p.n(x, 0, z, if (x + z) % 3 == 0 { "dirt" } else { "planks" }); } }
    for x in 0..w { for z in 0..d { let edge = x == 0 || x == w - 1 || z == d - 1; if !edge { continue; } p.n(x, 1, z, "stone_bricks"); for y in 2..=3 { p.n(x, y, z, "barn_boards"); } } }
    for x in (0..w).step_by(4).chain([w - 1]) { for z in [0, d - 1] { for y in 1..=3 { p.log(x, y, z, 'y'); } } } for z in [3, d - 4] { for x in [0, w - 1] { for y in 1..=3 { p.log(x, y, z, 'y'); } } }
    for x in 1..w - 1 { if x % 4 != 0 && x != gate { p.n(x, 1, 0, "fence"); } }
    p.belt((0, w - 1), (0, d - 1), 3); for z in 2..d - 2 { if z % 2 == 0 { p.win(0, 2, z, false); p.win(w - 1, 2, z, false); } }
    p.roof((0, w - 1), (0, d - 1), 3, 1, true, roof, Some("planks"));
    for (z, sg) in [(0, -1.0), (d - 1, 1.0)] { p.truss((0, w - 1), z, sg, 4, 3.0 + (w + 2) as f64 * 0.5 - 0.6); }
    let mid = (w - 1) / 2; p.turn(mid, 5, -1, sign, 0, 1); p.win(mid, 5, d - 1, true); p.sconce(0, 2, -1, 0, 1); p.sconce(w - 1, 2, -1, 0, 1); p.n(mid, 2, 2, "lantern");
}
/// M-ETA-01 : l'étable et bergerie, 13 × 9 : deux cloisons de barrières font trois parcs, des auges au fond, le foin
/// en ballots dans le coin gauche, l'enseigne au mouton.
pub(crate) fn etable(p: &mut P, roof: &str) -> Built {
    let (w, d) = (13, 9); halle(p, w, d, roof, "banner_gold", 6);
    for x in [4, 8] { p.log(x, 1, 3, 'y'); p.log(x, 2, 3, 'y'); for z in 4..d - 1 { p.n(x, 1, z, "fence"); } }
    for x in [2, 6, 10] { p.turn(x, 1, d - 2, "trough", 0, 1); } for (x, z, y) in [(1, 5, 1), (1, 6, 1), (2, 6, 1), (1, 6, 2), (1, 5, 2)] { p.n(x, y, z, "hay_bale"); } p.n(11, 1, 1, "bucket"); p.n(11, 1, 6, "hay_bale");
    Built { door: Some(6), spot: None, bed: None }
}
/// M-ECU-01 : l'écurie, 14 × 9 : trois boxes séparés par des cloisons de poteaux et de planches, une mangeoire dans
/// chacun, le foin dans le coin gauche, une allée devant, l'enseigne au fer à cheval.
pub(crate) fn ecurie(p: &mut P, roof: &str) -> Built {
    let (w, d) = (14, 9); halle(p, w, d, roof, "banner_blue", 7);
    for x in [5, 9] { for z in 4..d - 1 { for y in 1..=2 { if z == 4 { p.log(x, y, z, 'y'); } else { p.n(x, y, z, "barn_boards"); } } } }
    for x in [1, 2, 3, 4, 6, 8, 10, 11, 12] { p.n(x, 1, 4, "fence"); }
    for x in [3, 7, 11] { p.turn(x, 1, d - 2, "trough", 0, 1); p.n(x - 1, 1, d - 2, "hay_bale"); } for (x, z, y) in [(1, 1, 1), (1, 2, 1), (1, 1, 2), (2, 1, 1)] { p.n(x, y, z, "hay_bale"); } p.n(12, 1, 1, "barrel"); p.turn(12, 2, 2, "tool_rack", 1, 0);
    Built { door: Some(7), spot: None, bed: None }
}

// =========================================================================================== répartition
/// Les patrons.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind { Maison, Forge, Auberge, Moulin, Tanneur, Eglise, GrandeMairie, PetiteMairie, Atelier, Ferme, Etable, Ecurie }
/// Corps (largeur, profondeur), porte sur la façade et emprise [x0, z0, x1, z1] d'un patron, symétrie comprise.
#[derive(Clone, Copy, Debug)]
pub struct Plan { pub w: i32, pub d: i32, pub door: i32, pub ext: [i32; 4], pub mirror: bool }
/// Les variantes d'une maison : bit 0, une travée de plus ; bit 1, l'appentis à bois ; bit 2, en miroir.
pub fn plan(k: Kind, var: u8) -> Plan {
    let (w, d, door, ext) = match k {
        Kind::Maison => { let d = 6 + (var & 1) as i32; (7, d, 3, [-1, -2, 7, d]) }
        Kind::Forge => (7, 9, 5, [-3, -1, 7, 9]), Kind::Auberge => (13, 10, 6, [-3, -2, 15, 10]), Kind::Moulin => (9, 9, 4, [-2, -2, 10, 9]),
        Kind::Tanneur => (12, 9, 3, [-1, -3, 12, 9]), Kind::Eglise => (11, 17, 5, [-1, -2, 11, 17]), Kind::GrandeMairie => (19, 15, 9, [-1, -3, 19, 15]),
        Kind::PetiteMairie => (9, 7, 4, [-1, -2, 9, 7]), Kind::Atelier => (14, 9, 3, [-1, -2, 14, 9]), Kind::Ferme => (15, 11, 5, [-1, -2, 15, 11]),
        Kind::Etable => (13, 9, 6, [-1, -1, 13, 9]), Kind::Ecurie => (14, 9, 7, [-1, -1, 14, 9]),
    };
    let mirror = if k == Kind::Maison { var & 4 != 0 } else { var & 1 != 0 };
    if !mirror { return Plan { w, d, door, ext, mirror }; }
    Plan { w, d, door: w - 1 - door, ext: [w - 1 - ext[2], ext[1], w - 1 - ext[0], ext[3]], mirror }
}
/// Les matières d'un bâtiment : le toit (« roof_tiles », « slate », « mossy_roof_tiles ») et le remplissage des pans de bois.
#[derive(Clone, Copy, Debug)]
pub struct Look { pub roof: &'static str, pub infill: &'static str }
/// Bâtit le patron sur son terrain, le sol à y0 ; rend porte, poste et lit en coordonnées du terrain.
pub(crate) fn build(k: &mut Work, lot: Lot, y0: i32, kind: Kind, var: u8, look: Look, h: f64) -> Built {
    let pl = plan(kind, var); let mut p = P { k, lot, y0, mirror: pl.mirror, w: pl.w, d: pl.d, h, rec: Rec::default() };
    let b = match kind {
        Kind::Maison => maison(&mut p, pl.d, look.roof, look.infill, var & 2 != 0), Kind::Forge => forge(&mut p, h > 0.5), Kind::Auberge => auberge(&mut p),
        Kind::Moulin => moulin(&mut p, h), Kind::Tanneur => tanneur(&mut p), Kind::Eglise => eglise(&mut p), Kind::GrandeMairie => grande_mairie(&mut p),
        Kind::PetiteMairie => petite_mairie(&mut p), Kind::Atelier => atelier_bois(&mut p), Kind::Ferme => ferme(&mut p, look.roof),
        Kind::Etable => etable(&mut p, look.roof), Kind::Ecurie => ecurie(&mut p, look.roof),
    };
    p.dress();
    let fx = |x: i32| p.lx(x);
    Built { door: b.door.map(fx), spot: b.spot.map(|(a, s)| ((fx(a.0), a.1), (fx(s.0), s.1))), bed: b.bed.map(|(x, z)| (fx(x), z)) }
}
