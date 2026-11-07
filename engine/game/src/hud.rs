//! Interface dessinée en quads : réticule, barre d'objets, vitalité, inventaire, menu. Le texte vient d'un atlas Inter ;
//! les icônes sont les vrais objets, rendus en 3D une fois au démarrage.
use crate::font_data as fd;
use crate::gpu::ICON_COLS;
use alw_core::session::{item, Session, INV_SIZE, ORDER, TABS};
use alw_core::survival::{group_label, Station, RECIPES};
pub const VERT: usize = 36; // octets par sommet : position, uv, couleur, couche
#[derive(Default)]
pub struct Ui { pub data: Vec<f32>, w: f32, h: f32 }
type C = [f32; 4];
const LAYER_FLAT: f32 = -1.0; const LAYER_TEXT: f32 = -2.0; const LAYER_ICON: f32 = -3.0;
const INK: C = [0.95, 0.96, 0.92, 1.0]; const DIM: C = [0.72, 0.76, 0.7, 0.95]; const PANEL: C = [0.09, 0.11, 0.095, 0.95]; const WELL: C = [0.0, 0.0, 0.0, 0.32]; const ACCENT: C = [0.55, 0.78, 0.42, 1.0];
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Hit { Slot(usize), Catalog(&'static str), Tab(usize), Recipe(usize) }
const RECIPE_COLS: usize = 5; const BAD: C = [1.0, 0.45, 0.38, 1.0];
fn scale(h: f32) -> f32 { (h / 900.0).clamp(0.8, 2.0) }
/// Disposition de l'écran d'inventaire, partagée par le dessin et la souris : [x, y, largeur, hauteur].
pub fn layout(creative: bool, tab: usize, w: f32, h: f32) -> Vec<(Hit, [f32; 4])> {
    // En survie, la grille des recettes se tient à droite du sac : l'ensemble reste centré.
    let k = scale(h); let c = 58.0 * k; let g = 5.0 * k; let tw = 9.0 * c + 8.0 * g; let side = if creative { 0.0 } else { RECIPE_COLS as f32 * (c + g) - g + 46.0 * k }; let x0 = (w - tw - side) * 0.5; let mut o = Vec::new();
    let top = h * 0.5 - if creative { 300.0 } else { 130.0 } * k; let mut y = top;
    if creative {
        let n = TABS.len() as f32; let tb = (tw - (n - 1.0) * g) / n; let th = 32.0 * k;
        for i in 0..TABS.len() { o.push((Hit::Tab(i), [x0 + i as f32 * (tb + g), top, tb, th])); }
        let y1 = top + th + 10.0 * k;
        for (i, id) in TABS[tab].1.iter().enumerate() { let (r, cl) = (i / 9, i % 9); o.push((Hit::Catalog(id), [x0 + cl as f32 * (c + g), y1 + r as f32 * (c + g), c, c])); }
        y = y1 + 4.0 * (c + g) + 16.0 * k;
    }
    for r in 0..3 { for cl in 0..9 { o.push((Hit::Slot(9 + r * 9 + cl), [x0 + cl as f32 * (c + g), y + r as f32 * (c + g), c, c])); } }
    for cl in 0..9 { o.push((Hit::Slot(cl), [x0 + cl as f32 * (c + g), y + 3.0 * (c + g) + 12.0 * k, c, c])); }
    if !creative { let xr = x0 + tw + 46.0 * k; for i in 0..RECIPES.len() { let (r, cl) = (i / RECIPE_COLS, i % RECIPE_COLS); o.push((Hit::Recipe(i), [xr + cl as f32 * (c + g), y + r as f32 * (c + g), c, c])); } }
    o
}
fn inside(m: [f32; 2], r: [f32; 4]) -> bool { m[0] >= r[0] && m[1] >= r[1] && m[0] < r[0] + r[2] && m[1] < r[1] + r[3] }
pub fn hit_at(creative: bool, tab: usize, w: f32, h: f32, m: [f32; 2]) -> Option<Hit> { layout(creative, tab, w, h).into_iter().find(|x| inside(m, x.1)).map(|x| x.0) }
pub const WEATHERS: [&str; 5] = ["Auto", "Clair", "Nuageux", "Pluie", "Brouillard"];
pub const QUALITIES: [&str; 2] = ["Haut", "Ultra"];
pub const MOODS: [&str; 6] = ["Auto", "Calme", "Nuit", "Construction", "Suspense", "Danger"];
const VOLS: [&str; 4] = ["Volume général", "Effets", "Musique", "Luminosité"];
/// Curseur de luminosité vers exposition : au milieu, l'image validée, plus sobre que la première version.
pub fn bright(v: f32) -> f32 { 0.6 + 0.5 * v }
const CREDITS: [&str; 13] = ["Musique générée par le jeu. Sons retravaillés pour le jeu.", "", "Licence CC BY 3.0", "crack : Benboncan, freesound.org/people/Benboncan/sounds/71823", "splash : AGFX, freesound.org/people/AGFX/packs/1253", "hurt : sonictechtonic, freesound.org/people/sonictechtonic/sounds/241872", "", "Licence CC0", "Kenney (Impact Sounds), worthahep88, lolamadeus, Sheyvan, blukotek, Yuval", "", "Police de l'interface", "Inter, SIL Open Font License 1.1", ""];
pub struct Menu { pub open: bool, pub credits: bool, pub weather: usize, pub mood: usize, pub vol: [f32; 4], pub quality: usize, pub confirm: bool }
#[derive(Clone, Copy, PartialEq, Debug)] pub enum M { Resume, Weather(usize), Mood(usize), Quality(usize), Vol(usize), Credits, Back, Quit, NewWorld }
fn menu_panel(w: f32, h: f32) -> ([f32; 4], f32) { let k = scale(h); let (pw, ph) = (640.0 * k, 618.0 * k); ([(w - pw) * 0.5, (h - ph) * 0.5, pw, ph], k) }
pub fn menu_layout(w: f32, h: f32, m: &Menu) -> Vec<(M, [f32; 4])> {
    let (p, k) = menu_panel(w, h); let (x0, tw) = (p[0] + 34.0 * k, p[2] - 68.0 * k); let bh = 40.0 * k; let g = 8.0 * k; let mut o = Vec::new();
    if m.credits { o.push((M::Back, [x0, p[1] + p[3] - 66.0 * k, tw, bh])); return o; }
    let mut y = p[1] + 74.0 * k; o.push((M::Resume, [x0, y, tw, bh])); y += bh + 34.0 * k;
    let row = |n: usize, y: f32, f: &dyn Fn(usize) -> M, o: &mut Vec<(M, [f32; 4])>| { let b = (tw - (n as f32 - 1.0) * g) / n as f32; for i in 0..n { o.push((f(i), [x0 + i as f32 * (b + g), y, b, 34.0 * k])); } };
    row(WEATHERS.len(), y, &M::Weather, &mut o); y += 34.0 * k + 34.0 * k;
    row(MOODS.len(), y, &M::Mood, &mut o); y += 34.0 * k + 34.0 * k;
    row(QUALITIES.len(), y, &M::Quality, &mut o); y += 34.0 * k + 22.0 * k;
    for i in 0..4 { o.push((M::Vol(i), [x0 + 190.0 * k, y + i as f32 * 32.0 * k, tw - 190.0 * k, 24.0 * k])); }
    y += 4.0 * 32.0 * k + 14.0 * k;
    let b = (tw - 2.0 * g) / 3.0; o.push((M::Credits, [x0, y, b, bh])); o.push((M::NewWorld, [x0 + b + g, y, b, bh])); o.push((M::Quit, [x0 + 2.0 * (b + g), y, b, bh])); o
}
pub fn menu_at(w: f32, h: f32, m: &Menu, p: [f32; 2]) -> Option<M> { menu_layout(w, h, m).into_iter().find(|x| inside(p, x.1)).map(|x| x.0) }
/// Bouton « Réapparaître » de l'écran de mort.
pub fn death_button(w: f32, h: f32) -> [f32; 4] { let k = scale(h); let (bw, bh) = (300.0 * k, 48.0 * k); [(w - bw) * 0.5, h * 0.58, bw, bh] }
pub fn death_at(w: f32, h: f32, m: [f32; 2]) -> bool { inside(m, death_button(w, h)) }
impl Ui {
    fn death(&mut self, s: &Session, d: &alw_core::survival::Death, w: f32, h: f32, k: f32, mouse: [f32; 2]) {
        self.rect(0.0, 0.0, w, h, [0.16, 0.02, 0.02, 0.72]);
        let line = |u: &mut Ui, t: &str, y: f32, z: f32, c: C| u.text(t, (w - Ui::text_width(t, z)) * 0.5, y, z, c);
        line(self, "Vous êtes mort", h * 0.33, 46.0 * k, INK);
        if !d.cause.is_empty() { line(self, &format!("Cause : {}", d.cause), h * 0.33 + 66.0 * k, 20.0 * k, [0.95, 0.75, 0.7, 1.0]); }
        if !s.stashes.is_empty() { line(self, "Vos affaires sont restées dans un sac, là où vous êtes tombé.", h * 0.33 + 98.0 * k, 17.0 * k, DIM); }
        let r = death_button(w, h); let on = inside(mouse, r); self.panel(r, k, if on { [0.5, 0.2, 0.14, 1.0] } else { [0.3, 0.1, 0.08, 1.0] }); self.centered("Réapparaître", r, 19.0 * k, INK);
        line(self, "ou Entrée", r[1] + r[3] + 12.0 * k, 15.0 * k, DIM);
    }
    fn panel(&mut self, r: [f32; 4], k: f32, c: C) { let e = 2.0 * k; self.rect(r[0] + e, r[1], r[2] - 2.0 * e, r[3], c); self.rect(r[0], r[1] + e, e, r[3] - 2.0 * e, c); self.rect(r[0] + r[2] - e, r[1] + e, e, r[3] - 2.0 * e, c); }
    fn centered(&mut self, t: &str, r: [f32; 4], z: f32, c: C) { self.text(t, r[0] + (r[2] - Ui::text_width(t, z)) * 0.5, r[1] + (r[3] - z * 1.35) * 0.5, z, c); }
    fn inventory(&mut self, s: &Session, w: f32, h: f32, k: f32, mouse: [f32; 2]) {
        self.rect(0.0, 0.0, w, h, [0.02, 0.03, 0.03, 0.55]);
        let l = layout(s.creative, s.tab, w, h); let (x0, y0) = (l[0].1[0], l[0].1[1]); let bottom = l.iter().filter(|r| !matches!(r.0, Hit::Recipe(_))).fold(0.0f32, |a, r| a.max(r.1[1] + r.1[3])); let tw = 9.0 * 58.0 * k + 8.0 * 5.0 * k; let m = 22.0 * k;
        self.panel([x0 - m, y0 - m - 30.0 * k, tw + 2.0 * m, bottom - y0 + 2.0 * m + 30.0 * k + 18.0 * k], k, PANEL);
        self.text(if s.creative { "Catalogue" } else { "Inventaire" }, x0, y0 - 38.0 * k, 22.0 * k, INK);
        if s.creative { let t = "Maj + clic : vers la barre  ·  clic droit : moitié du tas"; let z = 15.0 * k; self.text(t, x0 + tw - Ui::text_width(t, z), y0 - 33.0 * k, z, DIM); }
        // Fabrication : ce qui est faisable ici et maintenant ressort, le reste est voilé ; la bulle dit ce qui manque.
        let st = if s.creative { (false, false) } else { s.stations() }; let mut lines: Vec<(String, C)> = Vec::new();
        if let Some((_, r0)) = l.iter().find(|r| r.0 == Hit::Recipe(0)) {
            let (xr, rw) = (r0[0], RECIPE_COLS as f32 * 63.0 * k - 5.0 * k); let rows = RECIPES.len().div_ceil(RECIPE_COLS) as f32; let gb = r0[1] + rows * 63.0 * k;
            self.panel([xr - m, y0 - m - 30.0 * k, rw + 2.0 * m, gb - y0 + 2.0 * m + 30.0 * k + 44.0 * k], k, PANEL); self.text("Fabrication", xr, y0 - 38.0 * k, 22.0 * k, INK);
            let z = 15.0 * k; let say = |on: bool, yes: &'static str, no: &'static str| if on { (yes, ACCENT) } else { (no, DIM) };
            for (i, (t, c)) in [say(st.0, "Établi à portée", "Aucun établi à portée"), say(st.1, "Feu de camp à portée", "Aucun feu à portée")].into_iter().enumerate() { self.text(t, xr, gb + 4.0 * k + i as f32 * 21.0 * k, z, c); }
        }
        let hover = hit_at(s.creative, s.tab, w, h, mouse); let mut tip = None;
        for (hit, r) in &l {
            let on = hover == Some(*hit);
            if let Hit::Recipe(i) = hit {
                let rc = &RECIPES[*i]; let ok = s.can_craft(rc, st); self.panel(*r, k, if on && ok { [0.55, 0.78, 0.42, 0.3] } else if on { [1.0, 1.0, 1.0, 0.14] } else { WELL });
                self.icon(rc.out, r[0] + r[2] * 0.5, r[1] + r[2] * 0.5, r[2] * 0.43); if rc.n > 1 { let t = rc.n.to_string(); let z = 16.0 * k; self.text(&t, r[0] + r[2] - Ui::text_width(&t, z) - 4.0 * k, r[1] + r[2] - z - 4.0 * k, z, [1.0; 4]); }
                if !ok { self.rect(r[0] + 2.0 * k, r[1] + 2.0 * k, r[2] - 4.0 * k, r[3] - 4.0 * k, [0.05, 0.06, 0.05, 0.6]); }
                if on {
                    let name = item(rc.out).map_or(rc.out, |i| i.label); lines.push((if rc.n > 1 { format!("{name} ({})", rc.n) } else { name.to_string() }, INK));
                    for (need, n) in rc.need { let have = s.count(need); let label = group_label(need).or(item(need).map(|i| i.label)).unwrap_or(need); lines.push((format!("{n} {label}  ·  {have} en stock"), if have >= *n { DIM } else { BAD })); }
                    let here = match rc.at { Station::Hand => true, Station::Bench => st.0, Station::Fire => st.1 }; lines.push((rc.at.label().to_string(), if here { DIM } else { BAD }));
                    lines.push((if ok { "Clic : fabriquer  ·  Maj + clic : tout" } else { "Impossible pour l'instant" }.to_string(), if ok { ACCENT } else { DIM }));
                }
                continue;
            }
            if let Hit::Tab(i) = hit { let sel = *i == s.tab; self.panel(*r, k, if sel { [0.28, 0.4, 0.22, 1.0] } else if on { [0.2, 0.24, 0.2, 1.0] } else { [0.0, 0.0, 0.0, 0.34] }); self.centered(TABS[*i].0, *r, 16.5 * k, if sel { INK } else { DIM }); continue; }
            self.panel(*r, k, if on { [1.0, 1.0, 1.0, 0.2] } else { WELL });
            let st = match hit { Hit::Slot(i) => s.inv[*i].map(|v| (v.item, v.n)), Hit::Catalog(id) => Some((*id, None)), _ => None };
            if let Some((id, n)) = st {
                self.icon(id, r[0] + r[2] * 0.5, r[1] + r[2] * 0.5, r[2] * 0.43);
                if let Some(n) = n { let t = n.to_string(); let z = 16.0 * k; self.text(&t, r[0] + r[2] - Ui::text_width(&t, z) - 4.0 * k, r[1] + r[2] - z - 4.0 * k, z, [1.0; 4]); }
                if on { tip = item(id).map(|i| i.label); }
            }
        }
        if let Some(c) = s.cursor { self.icon(c.item, mouse[0], mouse[1], 30.0 * k); if let Some(n) = c.n { self.text(&n.to_string(), mouse[0] + 12.0 * k, mouse[1] + 10.0 * k, 16.0 * k, [1.0; 4]); } }
        else if !lines.is_empty() {
            let z = 16.0 * k; let lh = 22.0 * k; let tw = lines.iter().fold(0.0f32, |a, l| a.max(Ui::text_width(&l.0, z))); let (bx, by) = ((mouse[0] + 14.0 * k).min(w - tw - 30.0 * k), mouse[1] + 8.0 * k);
            self.panel([bx, by, tw + 18.0 * k, lines.len() as f32 * lh + 12.0 * k], k, [0.03, 0.04, 0.03, 0.96]); for (i, (t, c)) in lines.iter().enumerate() { self.text(t, bx + 9.0 * k, by + 6.0 * k + i as f32 * lh, if i == 0 { 18.0 * k } else { z }, *c); }
        }
        else if let Some(t) = tip { let z = 17.0 * k; let tw = Ui::text_width(t, z); self.panel([mouse[0] + 14.0 * k, mouse[1] + 8.0 * k, tw + 18.0 * k, z + 14.0 * k], k, [0.03, 0.04, 0.03, 0.95]); self.text(t, mouse[0] + 23.0 * k, mouse[1] + 12.0 * k, z, INK); }
    }
    fn menu(&mut self, m: &Menu, w: f32, h: f32, mouse: [f32; 2]) {
        self.rect(0.0, 0.0, w, h, [0.02, 0.03, 0.03, 0.6]);
        let (p, k) = menu_panel(w, h); let (x0, z) = (p[0] + 34.0 * k, 18.0 * k); self.panel(p, k, PANEL); let hover = menu_at(w, h, m, mouse);
        self.text(if m.credits { "Crédits" } else { "A Living World" }, x0, p[1] + 22.0 * k, 28.0 * k, INK);
        if m.credits { for (i, l) in CREDITS.iter().enumerate() { let t = i == 2 || i == 7 || i == 10; self.text(l, x0, p[1] + 78.0 * k + i as f32 * 28.0 * k, if t { z } else { 16.0 * k }, if t { ACCENT } else { INK }); } }
        for (a, r) in menu_layout(w, h, m) {
            let on = hover == Some(a); let plain: C = if on { [0.25, 0.3, 0.24, 1.0] } else { [0.0, 0.0, 0.0, 0.34] }; let sel: C = [0.28, 0.4, 0.22, 1.0];
            match a {
                M::Resume | M::Credits | M::Back | M::Quit | M::NewWorld => { let warn = a == M::NewWorld && m.confirm; self.panel(r, k, if warn { [0.5, 0.2, 0.14, 1.0] } else { plain }); self.centered(match a { M::Resume => "Reprendre", M::Credits => "Crédits", M::Back => "Retour", M::NewWorld => if m.confirm { "Confirmer ?" } else { "Nouveau monde" }, _ => "Quitter" }, r, 16.0 * k, INK); }
                M::Weather(i) => { if i == 0 { self.text("Météo", x0, r[1] - 26.0 * k, 17.0 * k, DIM); } self.panel(r, k, if m.weather == i { sel } else { plain }); self.centered(WEATHERS[i], r, 15.0 * k, INK); }
                M::Mood(i) => { if i == 0 { self.text("Ambiance musicale", x0, r[1] - 26.0 * k, 17.0 * k, DIM); } self.panel(r, k, if m.mood == i { sel } else { plain }); self.centered(MOODS[i], r, 14.0 * k, INK); }
                M::Quality(i) => { if i == 0 { self.text("Qualité graphique (au prochain lancement)", x0, r[1] - 26.0 * k, 17.0 * k, DIM); } self.panel(r, k, if m.quality == i { sel } else { plain }); self.centered(QUALITIES[i], r, 15.0 * k, INK); }
                M::Vol(i) => { self.text(VOLS[i], x0, r[1], 17.0 * k, DIM); let t = 6.0 * k; let y = r[1] + (r[3] - t) * 0.5; self.rect(r[0], y, r[2], t, [0.0, 0.0, 0.0, 0.5]); self.rect(r[0], y, r[2] * m.vol[i], t, ACCENT); let hx = r[0] + r[2] * m.vol[i]; self.panel([hx - 7.0 * k, r[1] + 1.0 * k, 14.0 * k, r[3] - 2.0 * k], k, INK); }
            }
        }
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
    /// Icône : une case de l'atlas 3D, centrée, de demi-largeur r.
    fn icon(&mut self, id: &str, cx: f32, cy: f32, r: f32) {
        let Some(i) = ORDER.iter().position(|o| *o == id) else { return }; let n = ICON_COLS as usize; let c = 1.0 / n as f32; let (u, v) = ((i % n) as f32 * c, (i / n) as f32 * c);
        self.quad([[cx - r, cy - r], [cx + r, cy - r], [cx + r, cy + r], [cx - r, cy + r]], [[u, v], [u + c, v], [u + c, v + c], [u, v + c]], [1.0; 4], LAYER_ICON);
    }
    pub fn build(s: &Session, w: f32, h: f32, msg: Option<&str>, mouse: [f32; 2], menu: &Menu, info: &[String]) -> Ui {
        let mut u = Ui { data: Vec::with_capacity(9000), w, h }; let k = scale(h); let w1 = [1.0, 1.0, 1.0, 0.9];
        // Blessure : l'écran rougit un instant ; à bout de forces, les bords restent rouges et battent.
        let fl = s.flash() as f32; if fl > 0.0 { u.rect(0.0, 0.0, w, h, [0.62, 0.04, 0.03, 0.3 * fl]); }
        if !s.creative && s.dead.is_none() && s.vit[0] < 25.0 { let a = (0.16 + 0.1 * (s.elapsed as f32 * 4.0).sin()) * (1.0 - s.vit[0] as f32 / 25.0).max(0.35); let e = 60.0 * k; let c = [0.55, 0.03, 0.02, a];
            u.rect(0.0, 0.0, w, e, c); u.rect(0.0, h - e, w, e, c); u.rect(0.0, e, e, h - 2.0 * e, c); u.rect(w - e, e, e, h - 2.0 * e, c); }
        // Le réticule rougit quand il est sur un animal à portée.
        let ch = if s.prey.is_some() { [1.0, 0.5, 0.4, 1.0] } else { w1 }; let t = if s.prey.is_some() { 3.0 } else { 2.0 };
        u.rect(w * 0.5 - t * 0.5, h * 0.5 - 8.0, t, 16.0, ch); u.rect(w * 0.5 - 8.0, h * 0.5 - t * 0.5, 16.0, t, ch);
        let cell = 62.0 * k; let gap = 5.0 * k; let total = 9.0 * cell + 8.0 * gap; let (x0, y0) = ((w - total) * 0.5, h - cell - 16.0 * k);
        for i in 0..9.min(INV_SIZE) {
            let x = x0 + i as f32 * (cell + gap); let on = i == s.sel;
            u.panel([x, y0, cell, cell], k, if on { [1.0, 1.0, 1.0, 0.3] } else { [0.08, 0.1, 0.09, 0.55] });
            if on { let b = 2.0 * k; u.rect(x - b, y0 - b, cell + 2.0 * b, b, w1); u.rect(x - b, y0 + cell, cell + 2.0 * b, b, w1); u.rect(x - b, y0, b, cell, w1); u.rect(x + cell, y0, b, cell, w1); }
            if let Some(st) = s.inv[i] {
                u.icon(st.item, x + cell * 0.5, y0 + cell * 0.5, cell * 0.43);
                if let (Some(n), false) = (st.n, s.creative) { let t = n.to_string(); let z = 17.0 * k; u.text(&t, x + cell - Ui::text_width(&t, z) - 5.0 * k, y0 + cell - z - 5.0 * k, z, w1); }
            }
        }
        if let Some(it) = s.held() { let z = 22.0 * k; u.text(it.label, (w - Ui::text_width(it.label, z)) * 0.5, y0 - z - 12.0 * k, z, w1); }
        if !s.creative {
            // Vie, faim, soif, endurance ; le souffle seulement sous l'eau. Une jauge presque vide clignote.
            let mut bars = vec![("Vie", s.vit[0], [0.88, 0.25, 0.25, 0.95]), ("Faim", s.vit[1], [0.9, 0.62, 0.2, 0.95]), ("Soif", s.vit[2], [0.25, 0.6, 0.9, 0.95]), (if s.winded { "Essoufflé" } else { "Endurance" }, s.stamina, if s.winded { [0.6, 0.6, 0.45, 0.95] } else { [0.78, 0.86, 0.36, 0.95] })];
            if s.air < 99.9 { bars.push(("Souffle", s.air, [0.7, 0.92, 1.0, 0.95])); }
            let n = bars.len();
            for (i, (name, v, mut c)) in bars.into_iter().enumerate() {
                let (bw, bh, y) = (190.0 * k, 12.0 * k, h - 26.0 * k - (n - 1 - i) as f32 * 19.0 * k); if v < 20.0 { c[3] *= 0.55 + 0.45 * (s.elapsed as f32 * 6.0).sin().abs(); }
                u.rect(18.0 * k, y, bw, bh, [0.0, 0.0, 0.0, 0.45]); u.rect(18.0 * k, y, bw * (v as f32 / 100.0).clamp(0.0, 1.0), bh, c); u.text(name, 18.0 * k + bw + 8.0 * k, y - 3.0 * k, 14.0 * k, DIM);
            }
        } else { let z = 18.0 * k; u.text("Mode créatif", 18.0 * k, h - 34.0 * k, z, w1); }
        if !info.is_empty() { let z = 16.0 * k; let bw = info.iter().fold(0.0f32, |a, l| a.max(Ui::text_width(l, z))); u.rect(10.0 * k, 10.0 * k, bw + 24.0 * k, info.len() as f32 * 22.0 * k + 14.0 * k, [0.0, 0.0, 0.0, 0.5]); for (i, l) in info.iter().enumerate() { u.text(l, 22.0 * k, 16.0 * k + i as f32 * 22.0 * k, z, INK); } }
        if s.inv_open { u.inventory(s, w, h, k, mouse); }
        if let Some(d) = &s.dead { u.death(s, d, w, h, k, mouse); }
        if menu.open { u.menu(menu, w, h, mouse); }
        if let Some(m) = msg { let z = 20.0 * k; u.text(m, (w - Ui::text_width(m, z)) * 0.5, h * 0.16, z, w1); }
        let _ = (u.w, u.h); u
    }
}
