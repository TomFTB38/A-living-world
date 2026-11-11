//! Interface dessinée en quads : réticule, barre d'objets, indicateurs de survie, inventaire avec équipement et statut,
//! coffre, livre de recettes, menu. Le texte vient d'un atlas Inter ; les icônes sont les vrais objets, rendus en 3D une fois.
use crate::font_data as fd;
use crate::gpu::ICON_COLS;
use crate::needs;
use alw_core::data::{def, Station, SLOTS};
use alw_core::map::{unpack, Atlas, STEP, ZOOMS};
use alw_core::play::{book, unknown, BOOK_CATS};
use alw_core::session::{item, order, tabs, Session, CHEST0, CHEST_SIZE, EQUIP0, INV_SIZE};
use alw_core::survival::{group_label, recipes, PROFILES, SKILLS};
pub const VERT: usize = 36; // octets par sommet : position, uv, couleur, couche
#[derive(Default)]
pub struct Ui { pub data: Vec<f32>, w: f32, h: f32 }
type C = [f32; 4];
const LAYER_FLAT: f32 = -1.0; const LAYER_TEXT: f32 = -2.0; const LAYER_ICON: f32 = -3.0;
const INK: C = [0.96, 0.95, 0.9, 1.0]; const DIM: C = [0.74, 0.76, 0.69, 0.95]; const PANEL: C = [0.075, 0.09, 0.08, 0.93]; const WELL: C = [0.0, 0.0, 0.0, 0.32]; const ACCENT: C = [0.55, 0.78, 0.42, 1.0]; const BAD: C = [1.0, 0.45, 0.38, 1.0]; const GOLD: C = [0.95, 0.78, 0.36, 1.0];
/// Ce que la souris peut viser dans l'inventaire. `Slot` couvre le sac, le coffre ouvert et l'équipement.
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Hit { Slot(usize), Catalog(&'static str), Tab(usize), Book(usize), BookCat(usize), Page(i32), Search, Craft(bool), Scroll(i32) }
fn scale(h: f32) -> f32 { (h / 900.0).clamp(0.8, 2.0) }
/// Échelle de l'inventaire de survie : assez petite pour que les trois panneaux tiennent en largeur.
fn inv_scale(w: f32, h: f32) -> f32 { scale(h).min(w / 1300.0).max(0.6) }
const BOOK_COLS: usize = 5; const BOOK_ROWS: usize = 4;
/// Rangées visibles du catalogue créatif, et rangées de l'onglet en cours.
pub const CAT_ROWS: usize = 4;
pub fn cat_rows(s: &Session) -> usize { tabs()[s.tab.min(tabs().len() - 1)].1.len().div_ceil(9) }
/// Disposition de l'écran d'inventaire, partagée par le dessin et la souris : [x, y, largeur, hauteur].
pub fn layout(s: &Session, w: f32, h: f32) -> Vec<(Hit, [f32; 4])> {
    let creative = s.creative; let k = if creative { scale(h) } else { inv_scale(w, h) }; let c = 58.0 * k; let g = 5.0 * k; let cell = c + g; let tw = 9.0 * c + 8.0 * g; let mut o = Vec::new();
    if creative {
        // Les onglets sur deux rangées : avec Mobilier et Décor, une seule rangée serait illisible.
        let x0 = (w - tw) * 0.5; let top = h * 0.5 - 300.0 * k; let per = tabs().len().div_ceil(2); let tb = (tw - (per as f32 - 1.0) * g) / per as f32; let th = 28.0 * k; let half = (th + g) * 0.5;
        for i in 0..tabs().len() { let (r, cl) = (i / per, i % per); o.push((Hit::Tab(i), [x0 + cl as f32 * (tb + g), top - half + r as f32 * (th + g), tb, th])); }
        let y1 = top + th + half + 10.0 * k;
        // Le catalogue défile par rangées (molette, ou les flèches à droite) : tout l'onglet reste accessible.
        let list = &tabs()[s.tab.min(tabs().len() - 1)].1; let row0 = s.cat_row.min(cat_rows(s).saturating_sub(CAT_ROWS));
        for (i, id) in list.iter().skip(row0 * 9).take(CAT_ROWS * 9).enumerate() { let (r, cl) = (i / 9, i % 9); o.push((Hit::Catalog(id), [x0 + cl as f32 * cell, y1 + r as f32 * cell, c, c])); }
        if cat_rows(s) > CAT_ROWS { let sx = x0 + tw + 10.0 * k; o.push((Hit::Scroll(-1), [sx, y1, 22.0 * k, 26.0 * k])); o.push((Hit::Scroll(1), [sx, y1 + 4.0 * cell - g - 26.0 * k, 22.0 * k, 26.0 * k])); }
        let y = y1 + 4.0 * cell + 16.0 * k; inv_grid(&mut o, x0, y, c, cell, k); return o;
    }
    let (lw, bw, gap) = (250.0 * k, BOOK_COLS as f32 * cell - g, 46.0 * k); let total = lw + gap + tw + gap + bw; let xl = (w - total) * 0.5; let x0 = xl + lw + gap; let xb = x0 + tw + gap;
    let chest = s.chest.is_some(); let y = h * 0.5 - 110.0 * k + if chest { 115.0 * k } else { 0.0 };
    if chest { let yc = y - 3.0 * cell - 50.0 * k; for i in 0..CHEST_SIZE { o.push((Hit::Slot(CHEST0 + i), [x0 + (i % 9) as f32 * cell, yc + (i / 9) as f32 * cell, c, c])); } }
    inv_grid(&mut o, x0, y, c, cell, k);
    for i in 0..SLOTS.len() { o.push((Hit::Slot(EQUIP0 + i), [xl, y + i as f32 * cell, c, c])); }
    // Livre de recettes : deux rangées d'onglets, la recherche, la grille, les pages, le détail et ses boutons.
    let yb = h * 0.5 - 300.0 * k; let th = 26.0 * k; let names = 1 + BOOK_CATS.len(); let per = names.div_ceil(2);
    for i in 0..names { let (r, cl) = (i / per, i % per); let n = if r == 0 { per } else { names - per }; let tbw = (bw - (n as f32 - 1.0) * g) / n as f32; o.push((Hit::BookCat(i), [xb + cl as f32 * (tbw + g), yb + r as f32 * (th + g), tbw, th])); }
    let ys = yb + 2.0 * (th + g) + 4.0 * k; o.push((Hit::Search, [xb, ys, bw, 28.0 * k]));
    let yg = ys + 36.0 * k; let list = book(s, cat_of(s.book.cat), &s.book.search); let page = s.book.page.min(list.len().saturating_sub(1) / (BOOK_COLS * BOOK_ROWS));
    for (j, r) in list.iter().skip(page * BOOK_COLS * BOOK_ROWS).take(BOOK_COLS * BOOK_ROWS).enumerate() { o.push((Hit::Book(*r), [xb + (j % BOOK_COLS) as f32 * cell, yg + (j / BOOK_COLS) as f32 * cell, c, c])); }
    let yp = yg + BOOK_ROWS as f32 * cell + 2.0 * k; o.push((Hit::Page(-1), [xb, yp, 40.0 * k, 26.0 * k])); o.push((Hit::Page(1), [xb + bw - 40.0 * k, yp, 40.0 * k, 26.0 * k]));
    let yd = yp + 36.0 * k; let bh = 32.0 * k; let yc = yd + 222.0 * k; o.push((Hit::Craft(false), [xb, yc, bw * 0.62, bh])); o.push((Hit::Craft(true), [xb + bw * 0.62 + g, yc, bw * 0.38 - g, bh]));
    o
}
fn inv_grid(o: &mut Vec<(Hit, [f32; 4])>, x0: f32, y: f32, c: f32, cell: f32, k: f32) {
    for r in 0..3 { for cl in 0..9 { o.push((Hit::Slot(9 + r * 9 + cl), [x0 + cl as f32 * cell, y + r as f32 * cell, c, c])); } }
    for cl in 0..9 { o.push((Hit::Slot(cl), [x0 + cl as f32 * cell, y + 3.0 * cell + 12.0 * k, c, c])); }
}
/// Catégorie du livre : 0 montre tout.
pub fn cat_of(i: usize) -> Option<&'static str> { if i == 0 { None } else { BOOK_CATS.get(i - 1).map(|c| c.0) } }
fn inside(m: [f32; 2], r: [f32; 4]) -> bool { m[0] >= r[0] && m[1] >= r[1] && m[0] < r[0] + r[2] && m[1] < r[1] + r[3] }
pub fn hit_at(s: &Session, w: f32, h: f32, m: [f32; 2]) -> Option<Hit> { layout(s, w, h).into_iter().find(|x| inside(m, x.1)).map(|x| x.0) }
/// La recette montrée en détail : celle sous la souris, sinon la dernière choisie.
pub fn shown(s: &Session, w: f32, h: f32, m: [f32; 2]) -> Option<usize> { match hit_at(s, w, h, m) { Some(Hit::Book(r)) => Some(r), _ => s.book.sel } }
pub const WEATHERS: [&str; 5] = ["Auto", "Clair", "Nuageux", "Pluie", "Brouillard"];
pub const QUALITIES: [&str; 2] = ["Haut", "Ultra"];
pub const MOODS: [&str; 6] = ["Auto", "Calme", "Nuit", "Construction", "Suspense", "Danger"];
const VOLS: [&str; 6] = ["Volume général", "Effets", "Musique", "Luminosité", "Sensibilité de la souris", "Champ de vision"];
/// Curseur de luminosité vers exposition : au milieu, l'image validée, plus sobre que la première version.
pub fn bright(v: f32) -> f32 { 0.6 + 0.5 * v }
const CREDITS: [&str; 13] = ["Musique générée par le jeu. Sons retravaillés pour le jeu.", "", "Licence CC BY 3.0", "crack : Benboncan, freesound.org/people/Benboncan/sounds/71823", "splash : AGFX, freesound.org/people/AGFX/packs/1253", "hurt : sonictechtonic, freesound.org/people/sonictechtonic/sounds/241872", "", "Licence CC0", "Kenney (Impact Sounds), worthahep88, lolamadeus, Sheyvan, blukotek, Yuval", "", "Police de l'interface", "Inter, SIL Open Font License 1.1", ""];
/// Réglages du menu. `profile` reflète le profil de survie de la partie ; `hud` : mode des indicateurs ; `alt` : touche Alt tenue.
pub struct Menu { pub open: bool, pub credits: bool, pub weather: usize, pub mood: usize, pub vol: [f32; 6], pub quality: usize, pub confirm: bool, pub profile: usize, pub hud: usize, pub reduce: bool, pub marker: bool, pub alt: bool, pub autostep: bool,
    /// Page des commandes, conseils de départ, touches (deux par action, par leur nom), touche attendue pour une action, écran titre.
    pub controls: bool, pub tips: bool, pub binds: Vec<[String; 2]>, pub listen: Option<(usize, usize)>, pub title: Option<Title> }
/// Les actions qu'on peut attribuer à une touche, dans l'ordre de `Menu::binds`.
pub const ACTIONS: [&str; 13] = ["Avancer", "Reculer", "Aller à gauche", "Aller à droite", "Sauter", "Courir", "S'accroupir", "Roulade", "Inventaire et recettes", "Carte", "Journal des villages", "Vue à la troisième personne", "Tourner le bloc"];
pub const A_FWD: usize = 0; pub const A_BACK: usize = 1; pub const A_LEFT: usize = 2; pub const A_RIGHT: usize = 3; pub const A_JUMP: usize = 4; pub const A_SPRINT: usize = 5; pub const A_CROUCH: usize = 6; pub const A_ROLL: usize = 7;
pub const A_INV: usize = 8; pub const A_MAP: usize = 9; pub const A_JOURNAL: usize = 10; pub const A_VIEW: usize = 11; pub const A_TURN: usize = 12;
/// Touches par défaut, par leur nom affiché : ZQSD et WASD marchent toutes deux.
pub fn default_binds() -> Vec<[String; 2]> {
    [["Z", "W"], ["S", ""], ["Q", "A"], ["D", ""], ["Espace", ""], ["Maj", ""], ["Ctrl", ""], ["V", ""], ["E", ""], ["M", ""], ["J", ""], ["C", ""], ["R", ""]].iter().map(|b| [b[0].to_string(), b[1].to_string()]).collect()
}
impl Menu {
    pub fn new() -> Menu { Menu { open: false, credits: false, weather: 0, mood: 0, vol: [0.7, 0.7, 0.5, 0.5, 0.5, 0.5], quality: 0, confirm: false, profile: 1, hud: 1, reduce: false, marker: true, alt: false, autostep: true, controls: false, tips: true, binds: default_binds(), listen: None, title: None } }
    /// Une touche (par son nom) déclenche-t-elle cette action ?
    pub fn is(&self, a: usize, key: &str) -> bool { !key.is_empty() && self.binds[a].iter().any(|b| b == key) }
    /// Sensibilité de la souris en radians par point, champ de vision vertical en radians.
    pub fn sens(&self) -> f64 { 0.0008 + 0.003 * self.vol[4] as f64 }
    pub fn fov(&self) -> f64 { (55.0 + 50.0 * self.vol[5] as f64).to_radians() }
}
impl Default for Menu { fn default() -> Self { Menu::new() } }

/// Un monde enregistré : nom (celui du fichier), graine, profil, heures jouées, ancienneté de la dernière partie.
#[derive(Clone, Debug)] pub struct WorldInfo { pub name: String, pub seed: i32, pub profile: usize, pub hours: f64, pub age: String }
#[derive(Clone, Copy, PartialEq, Debug)] pub enum TPage { Main, Worlds, New }
/// L'écran titre : page, mondes, monde choisi, champs du nouveau monde (nom, graine), profil, champ actif, suppression à confirmer.
#[derive(Clone, Debug)] pub struct Title { pub page: TPage, pub worlds: Vec<WorldInfo>, pub sel: usize, pub name: String, pub seed: String, pub profile: usize, pub field: usize, pub confirm: bool, pub current: String, pub scroll: usize }
impl Title { pub fn new(worlds: Vec<WorldInfo>, current: &str) -> Title { Title { page: TPage::Main, sel: 0, name: String::new(), seed: String::new(), profile: 1, field: 0, confirm: false, current: current.to_string(), scroll: 0, worlds } } }
#[derive(Clone, Copy, PartialEq, Debug)] pub enum T { Continue, New, Worlds, Settings, Quit, Row(usize), Play, Delete, Back, Field(usize), Profile(usize), Create }
const ROWS: usize = 7;
pub fn title_layout(t: &Title, w: f32, h: f32) -> Vec<(T, [f32; 4])> {
    let k = scale(h); let mut o = Vec::new();
    match t.page {
        TPage::Main => {
            let (x, bw, bh) = (90.0 * k, 400.0 * k, 52.0 * k); let mut y = h * 0.42;
            if !t.current.is_empty() { o.push((T::Continue, [x, y, bw, bh + 14.0 * k])); y += bh + 26.0 * k; }
            for a in [T::New, T::Worlds, T::Settings, T::Quit] { o.push((a, [x, y, bw, bh])); y += bh + 12.0 * k; }
        }
        TPage::Worlds => {
            let (pw, rh) = ((820.0 * k).min(w - 40.0), 60.0 * k); let x = (w - pw) * 0.5; let y = h * 0.16;
            for (i, _) in t.worlds.iter().enumerate().skip(t.scroll).take(ROWS) { o.push((T::Row(i), [x + 20.0 * k, y + 80.0 * k + (i - t.scroll) as f32 * (rh + 6.0 * k), pw - 40.0 * k, rh])); }
            let by = y + 80.0 * k + ROWS as f32 * (rh + 6.0 * k) + 16.0 * k; let b = (pw - 40.0 * k - 2.0 * 12.0 * k) / 3.0;
            for (j, a) in [T::Play, T::Delete, T::Back].into_iter().enumerate() { o.push((a, [x + 20.0 * k + j as f32 * (b + 12.0 * k), by, b, 46.0 * k])); }
        }
        TPage::New => {
            let pw = (820.0 * k).min(w - 40.0); let x = (w - pw) * 0.5 + 20.0 * k; let iw = pw - 40.0 * k; let y = h * 0.16;
            o.push((T::Field(0), [x, y + 104.0 * k, iw, 44.0 * k])); o.push((T::Field(1), [x, y + 190.0 * k, iw, 44.0 * k]));
            let b = (iw - 2.0 * 12.0 * k) / 3.0; for i in 0..PROFILES.len() { o.push((T::Profile(i), [x + i as f32 * (b + 12.0 * k), y + 276.0 * k, b, 96.0 * k])); }
            let b = (iw - 12.0 * k) / 2.0; o.push((T::Create, [x, y + 400.0 * k, b, 50.0 * k])); o.push((T::Back, [x + b + 12.0 * k, y + 400.0 * k, b, 50.0 * k]));
        }
    }
    o
}
pub fn title_at(t: &Title, w: f32, h: f32, p: [f32; 2]) -> Option<T> { title_layout(t, w, h).into_iter().find(|x| inside(p, x.1)).map(|x| x.0) }
/// Ce que change chaque profil, en une ligne.
pub fn profile_line(i: usize) -> String { let p = &PROFILES[i]; match i { 0 => "Besoins lents, blessures légères, dépouille gardée longtemps".into(), 1 => "Le jeu tel qu'il est pensé".into(), _ => format!("Besoins x{:.1}, blessures x{:.1}, dépouille {:.0} jours", p.needs, p.harm, p.stash_days) } }
#[derive(Clone, Copy, PartialEq, Debug)] pub enum M { Resume, Weather(usize), Mood(usize), Quality(usize), Vol(usize), Credits, Back, Quit, NewWorld, Profile(usize), Hud(usize), Reduce, Marker, Tips, Controls, Bind(usize, usize), ResetKeys, AutoStep }
fn menu_panel(w: f32, h: f32) -> ([f32; 4], f32) { let k = scale(h).min(h / 800.0); let (pw, ph) = (680.0 * k, 702.0 * k); ([(w - pw) * 0.5, (h - ph) * 0.5, pw, ph], k) }
pub fn menu_layout(w: f32, h: f32, m: &Menu) -> Vec<(M, [f32; 4])> {
    let (p, k) = menu_panel(w, h); let (x0, tw) = (p[0] + 34.0 * k, p[2] - 68.0 * k); let bh = 40.0 * k; let g = 8.0 * k; let mut o = Vec::new();
    if m.credits { o.push((M::Back, [x0, p[1] + p[3] - 66.0 * k, tw, bh])); return o; }
    if m.controls {
        // Commandes : deux touches par action, puis sensibilité et champ de vision.
        let rh = 30.0 * k; let mut y = p[1] + 88.0 * k; let kw = 120.0 * k;
        for a in 0..ACTIONS.len() { for sl in 0..2 { o.push((M::Bind(a, sl), [x0 + tw - 2.0 * kw - g + sl as f32 * (kw + g), y, kw, rh - 4.0 * k])); } y += rh; }
        y += 14.0 * k; for i in 4..6 { o.push((M::Vol(i), [x0 + 230.0 * k, y + (i - 4) as f32 * 30.0 * k, tw - 230.0 * k, 22.0 * k])); }
        y += 72.0 * k; let b = (tw - 2.0 * g) / 3.0; o.push((M::AutoStep, [x0, y, b, bh])); o.push((M::ResetKeys, [x0 + b + g, y, b, bh])); o.push((M::Back, [x0 + 2.0 * (b + g), y, b, bh])); return o;
    }
    let mut y = p[1] + 70.0 * k; o.push((M::Resume, [x0, y, tw, bh])); y += bh + 32.0 * k;
    let row = |n: usize, y: f32, f: &dyn Fn(usize) -> M, o: &mut Vec<(M, [f32; 4])>| { let b = (tw - (n as f32 - 1.0) * g) / n as f32; for i in 0..n { o.push((f(i), [x0 + i as f32 * (b + g), y, b, 32.0 * k])); } };
    row(WEATHERS.len(), y, &M::Weather, &mut o); y += 64.0 * k;
    row(MOODS.len(), y, &M::Mood, &mut o); y += 64.0 * k;
    row(QUALITIES.len(), y, &M::Quality, &mut o); y += 64.0 * k;
    row(PROFILES.len(), y, &M::Profile, &mut o); y += 64.0 * k;
    row(needs::MODES.len(), y, &M::Hud, &mut o); y += 40.0 * k;
    let b = (tw - 2.0 * g) / 3.0; o.push((M::Reduce, [x0, y, b, 32.0 * k])); o.push((M::Marker, [x0 + b + g, y, b, 32.0 * k])); o.push((M::Tips, [x0 + 2.0 * (b + g), y, b, 32.0 * k])); y += 50.0 * k;
    for i in 0..4 { o.push((M::Vol(i), [x0 + 190.0 * k, y + i as f32 * 30.0 * k, tw - 190.0 * k, 22.0 * k])); }
    y += 4.0 * 30.0 * k + 12.0 * k;
    let b = (tw - 3.0 * g) / 4.0; for (j, a) in [M::Controls, M::Credits, M::NewWorld, M::Quit].into_iter().enumerate() { o.push((a, [x0 + j as f32 * (b + g), y, b, bh])); } o
}
pub fn menu_at(w: f32, h: f32, m: &Menu, p: [f32; 2]) -> Option<M> { menu_layout(w, h, m).into_iter().find(|x| inside(p, x.1)).map(|x| x.0) }
/// Bouton « Réapparaître » de l'écran de mort.
/// Fenêtre de dialogue d'un villageois : le cadre, puis une ligne par réponse. Partagé par le dessin et la souris.
pub fn dialog_layout(n: usize, w: f32, h: f32) -> ([f32; 4], Vec<[f32; 4]>) {
    let k = scale(h); let pw = (720.0 * k).min(w - 40.0); let rh = 34.0 * k; let ph = 176.0 * k + n as f32 * (rh + 4.0 * k); let (x, y) = ((w - pw) * 0.5, (h - ph).max(20.0) * 0.62);
    ([x, y, pw, ph], (0..n).map(|i| [x + 18.0 * k, y + 136.0 * k + i as f32 * (rh + 4.0 * k), pw - 36.0 * k, rh]).collect())
}
pub fn dialog_at(s: &Session, w: f32, h: f32, m: [f32; 2]) -> Option<usize> { let d = s.dialog()?; dialog_layout(d.options.len(), w, h).1.iter().position(|r| inside(m, *r)) }
pub fn death_button(w: f32, h: f32) -> [f32; 4] { let k = scale(h); let (bw, bh) = (300.0 * k, 48.0 * k); [(w - bw) * 0.5, h * 0.58, bw, bh] }
pub fn death_at(w: f32, h: f32, m: [f32; 2]) -> bool { inside(m, death_button(w, h)) }
/// Coupe un texte en lignes qui tiennent dans une largeur.
fn wrap(t: &str, z: f32, width: f32) -> Vec<String> { let mut out = Vec::new(); let mut line = String::new(); for w in t.split_whitespace() { let next = if line.is_empty() { w.to_string() } else { format!("{line} {w}") }; if Ui::text_width(&next, z) > width && !line.is_empty() { out.push(std::mem::replace(&mut line, w.to_string())); } else { line = next; } } if !line.is_empty() { out.push(line); } out }
impl Ui {
    /// Un cadre aux coins adoucis. Les grands panneaux ont une ombre portée, un liseré et un filet de couleur en tête ;
    /// les petits (cases, boutons) un relief discret : clair en haut, sombre en bas.
    fn panel(&mut self, r: [f32; 4], k: f32, c: C) {
        let e = 2.0 * k; let big = r[2] > 200.0 * k && r[3] > 140.0 * k;
        if big { let o = 6.0 * k; self.rect(r[0] + o, r[1] + o + e, r[2] - e, r[3] - e, [0.0, 0.0, 0.0, 0.22 * c[3]]); }
        self.rect(r[0] + e, r[1], r[2] - 2.0 * e, r[3], c); self.rect(r[0], r[1] + e, e, r[3] - 2.0 * e, c); self.rect(r[0] + r[2] - e, r[1] + e, e, r[3] - 2.0 * e, c);
        let t = k.max(1.0);
        if big {
            let line: C = [0.5, 0.58, 0.44, 0.35 * c[3]];
            self.rect(r[0] + e, r[1] + r[3] - t, r[2] - 2.0 * e, t, line); self.rect(r[0], r[1] + e, t, r[3] - 2.0 * e, line); self.rect(r[0] + r[2] - t, r[1] + e, t, r[3] - 2.0 * e, line);
            self.rect(r[0] + e, r[1], r[2] - 2.0 * e, 2.0 * t, [ACCENT[0], ACCENT[1], ACCENT[2], 0.8 * c[3]]);
            // Bandeau de titre, un peu plus clair, séparé du contenu par un filet.
            self.rect(r[0] + e, r[1] + 2.0 * t, r[2] - 2.0 * e, 48.0 * k, [1.0, 1.0, 1.0, 0.035 * c[3]]); self.rect(r[0] + 12.0 * k, r[1] + 50.0 * k, r[2] - 24.0 * k, t, [1.0, 1.0, 1.0, 0.07 * c[3]]);
        } else if c[3] > 0.05 {
            // Une case sombre est creusée (ombre en haut), un bouton est en relief (lumière en haut).
            let (hi, lo): (C, C) = ([1.0, 1.0, 1.0, 0.07 + 0.05 * c[3]], [0.0, 0.0, 0.0, 0.3 * c[3].max(0.4)]); let dark = c[0] + c[1] + c[2] < 0.15; let (a, b) = if dark { (lo, [1.0, 1.0, 1.0, 0.06]) } else { (hi, lo) };
            self.rect(r[0] + e, r[1], r[2] - 2.0 * e, t, a); self.rect(r[0] + e, r[1] + r[3] - t, r[2] - 2.0 * e, t, b);
        }
    }
    fn centered(&mut self, t: &str, r: [f32; 4], z: f32, c: C) { self.text(t, r[0] + (r[2] - Ui::text_width(t, z)) * 0.5, r[1] + (r[3] - z * 1.35) * 0.5, z, c); }
    /// Une case avec son objet : icône, nombre, usure d'un outil.
    fn slot_view(&mut self, s: &Session, st: Option<alw_core::session::Stack>, r: [f32; 4], k: f32, on: bool) {
        self.panel(r, k, if on { [1.0, 1.0, 1.0, 0.2] } else { WELL });
        let Some(st) = st else { return }; self.icon(st.item, r[0] + r[2] * 0.5, r[1] + r[3] * 0.5, r[2] * 0.43);
        if let Some(n) = st.n { if n > 1 || def(st.item).is_some_and(|d| d.stack > 1) { let t = n.to_string(); let z = 16.0 * k; self.text(&t, r[0] + r[2] - Ui::text_width(&t, z) - 4.0 * k, r[1] + r[3] - z - 4.0 * k, z, [1.0; 4]); } }
        if let Some(wv) = s.wear_of(&st) { if st.wear > 0 { let left = (1.0 - wv as f32).clamp(0.0, 1.0); let (bx, by, bw) = (r[0] + 6.0 * k, r[1] + r[3] - 7.0 * k, r[2] - 12.0 * k); self.rect(bx, by, bw, 3.0 * k, [0.0, 0.0, 0.0, 0.6]); self.rect(bx, by, bw * left, 3.0 * k, if left > 0.5 { ACCENT } else if left > 0.2 { GOLD } else { BAD }); } }
    }
    /// Bulle d'aide : plusieurs lignes, la première en titre.
    fn bubble(&mut self, lines: &[(String, C)], mouse: [f32; 2], k: f32) {
        if lines.is_empty() { return; } let z = 15.5 * k; let lh = 21.0 * k; let tw = lines.iter().enumerate().fold(0.0f32, |a, (i, l)| a.max(Ui::text_width(&l.0, if i == 0 { 18.0 * k } else { z })));
        let (bx, by) = ((mouse[0] + 16.0 * k).min(self.w - tw - 30.0 * k), (mouse[1] + 10.0 * k).min(self.h - lines.len() as f32 * lh - 20.0 * k));
        self.panel([bx, by, tw + 18.0 * k, lines.len() as f32 * lh + 12.0 * k], k, [0.03, 0.04, 0.03, 0.96]); for (i, (t, c)) in lines.iter().enumerate() { self.text(t, bx + 9.0 * k, by + 6.0 * k + i as f32 * lh, if i == 0 { 18.0 * k } else { z }, *c); }
    }
    fn item_lines(s: &Session, st: &alw_core::session::Stack, k: f32) -> Vec<(String, C)> {
        let Some(d) = def(st.item) else { return Vec::new() }; let mut l = vec![(d.label.to_string(), INK)];
        for line in wrap(d.desc, 15.5 * k, 320.0 * k) { l.push((line, DIM)); }
        if let Some(t) = d.tool { let (dmg, _) = t.strike(); let max = (t.mat.dur as f64 / PROFILES[s.profile].wear).round(); l.push((format!("Usure {} / {}  ·  coup {:.0}", st.wear, max, dmg), DIM)); }
        if let Some(f) = d.food { l.push((format!("Faim +{:.0}{}{}", f.hunger, if f.thirst != 0.0 { format!("  ·  soif {:+.0}", f.thirst) } else { String::new() }, if f.health != 0.0 { format!("  ·  vie {:+.0}", f.health) } else { String::new() }), if f.health < 0.0 { BAD } else { ACCENT })); }
        if let Some(w) = d.wear { l.push((format!("Se porte : {}{}{}{}", SLOTS.iter().find(|x| x.0 == w.slot).map_or("", |x| x.1), if w.warm != 0.0 { format!("  ·  chaleur {:+.0}", w.warm) } else { String::new() }, if w.armor > 0.0 { format!("  ·  armure {:.0}", w.armor) } else { String::new() }, if w.bag > 0.0 { format!("  ·  +{:.0} kg", w.bag) } else { String::new() }), ACCENT)); }
        if d.fuel > 0.0 { l.push((format!("Combustible : {:.0} s de feu", d.fuel * alw_core::survival::FUEL_SCALE), DIM)); }
        l.push((format!("{:.1} kg", d.mass * st.n.unwrap_or(1) as f64), DIM)); l
    }
    fn inventory(&mut self, s: &Session, w: f32, h: f32, mouse: [f32; 2]) {
        // Le monde s'assombrit, plus fort sur les bords.
        self.rect(0.0, 0.0, w, h, [0.02, 0.03, 0.03, 0.42]); for i in 0..10 { let f = i as f32 / 10.0; let b = w.min(h) * 0.05 * (1.0 - f); let a = [0.0, 0.0, 0.0, 0.05]; self.rect(0.0, 0.0, w, b, a); self.rect(0.0, h - b, w, b, a); self.rect(0.0, b, b, h - 2.0 * b, a); self.rect(w - b, b, b, h - 2.0 * b, a); }
        let k = if s.creative { scale(h) } else { inv_scale(w, h) }; let l = layout(s, w, h); let m = 22.0 * k; let cell = 63.0 * k; let tw = 9.0 * 58.0 * k + 8.0 * 5.0 * k;
        let first = l.iter().find(|r| matches!(r.0, Hit::Slot(i) if (9..INV_SIZE).contains(&i))).map(|r| r.1).unwrap(); let (x0, y0) = (first[0], first[1]); let bottom = y0 + 4.0 * cell + 12.0 * k;
        let top = if s.creative { l[0].1[1] } else if s.chest.is_some() { y0 - 3.0 * cell - 50.0 * k } else { y0 };
        self.panel([x0 - m, top - m - 30.0 * k, tw + 2.0 * m, bottom - top + 2.0 * m + 30.0 * k], k, PANEL);
        self.text(if s.creative { "Catalogue" } else if s.chest.is_some() { "Coffre" } else { "Inventaire" }, x0, top - 38.0 * k, 22.0 * k, INK);
        if s.chest.is_some() { self.text("Inventaire", x0, y0 - 30.0 * k, 17.0 * k, DIM); }
        if s.creative { let t = "Maj + clic : vers la barre  ·  clic droit : moitié du tas"; let z = 15.0 * k; self.text(t, x0 + tw - Ui::text_width(t, z), top - 33.0 * k, z, DIM); }
        else { let (c, cap) = (s.carried(), s.capacity()); let t = format!("{c:.1} / {cap:.0} kg"); let z = 15.0 * k; self.text(&t, x0 + tw - Ui::text_width(&t, z), top - 33.0 * k, z, if c > cap { BAD } else { DIM }); }
        let hover = hit_at(s, w, h, mouse); let mut tip: Vec<(String, C)> = Vec::new();
        let st6 = if s.creative { [true; 6] } else { s.stations() };
        if !s.creative {
            // Équipement et état du personnage, à gauche.
            let e0 = l.iter().find(|r| r.0 == Hit::Slot(EQUIP0)).map(|r| r.1).unwrap(); let (xl, lw) = (e0[0], 250.0 * k);
            self.panel([xl - m, y0 - m - 30.0 * k, lw + 2.0 * m, SLOTS.len() as f32 * cell + 100.0 * k + if s.skills.iter().any(|v| *v >= 1.0) { 44.0 * k } else { 0.0 } + 2.0 * m + 30.0 * k], k, PANEL); self.text("Personnage", xl, y0 - 38.0 * k, 22.0 * k, INK);
            // Emplacements vides : la silhouette de ce qui s'y porte, en sourdine.
            for (i, ghost) in ["leather_cap", "leather_jacket", "leather_pants", "leather_boots", "backpack", "wooden_shield"].iter().enumerate() { if s.equip[i].is_none() { let c = 58.0 * k; let (gx, gy) = (xl + c * 0.5, y0 + i as f32 * cell + c * 0.5); self.icon(ghost, gx, gy, c * 0.36); self.rect(xl, y0 + i as f32 * cell, c, c, [0.06, 0.07, 0.06, 0.72]); } }
            for (i, (_, name)) in SLOTS.iter().enumerate() { self.text(name, xl + cell + 4.0 * k, y0 + i as f32 * cell + 8.0 * k, 15.5 * k, DIM); if let Some(st) = s.equip[i] { if let Some(d) = def(st.item) { self.text(d.label, xl + cell + 4.0 * k, y0 + i as f32 * cell + 29.0 * k, 13.5 * k, INK); } } }
            let (warm, cool, armor, _, _) = s.worn(); let z = 14.5 * k; let mut y = y0 + SLOTS.len() as f32 * cell + 8.0 * k;
            let lines = [format!("Ressenti {:.0} °C  ·  corps {:.1} °C", s.air_here(), s.body), format!("Fatigue {:.0} %  ·  armure {:.0}", s.fatigue, armor), format!("Chaleur des habits {:+.0}{}", warm, if cool > 0.0 { format!(", fraîcheur {cool:.0}") } else { String::new() }), format!("Survie : {}", PROFILES[s.profile].name)];
            for t in lines { self.text(&t, xl, y, z, DIM); y += 20.0 * k; }
            let sk: Vec<String> = SKILLS.iter().zip(s.skills.iter()).filter(|(_, v)| **v >= 1.0).map(|(n, v)| format!("{n} {:+.0} %", (alw_core::survival::skill_bonus(*v) - 1.0) * 100.0)).collect();
            if !sk.is_empty() { for t in wrap(&sk.join("  ·  "), z, lw) { self.text(&t, xl, y, z, ACCENT); y += 20.0 * k; } }
            // Livre de recettes, à droite.
            let c0 = l.iter().find(|r| r.0 == Hit::BookCat(0)).map(|r| r.1).unwrap(); let bw = BOOK_COLS as f32 * cell - 5.0 * k; let xb = c0[0];
            let craft = l.iter().find(|r| r.0 == Hit::Craft(false)).map(|r| r.1).unwrap();
            self.panel([xb - m, c0[1] - m - 30.0 * k, bw + 2.0 * m, craft[1] + craft[3] - c0[1] + 2.0 * m + 30.0 * k], k, PANEL); self.text("Recettes", xb, c0[1] - 38.0 * k, 22.0 * k, INK);
            let left = unknown(s); if left > 0 { let t = format!("{left} à découvrir"); let z = 14.0 * k; self.text(&t, xb + bw - Ui::text_width(&t, z), c0[1] - 32.0 * k, z, DIM); }
            let list = book(s, cat_of(s.book.cat), &s.book.search); let pages = list.len().div_ceil(BOOK_COLS * BOOK_ROWS).max(1);
            for (hit, r) in &l { let on = hover == Some(*hit); match hit {
                Hit::BookCat(i) => { let sel = *i == s.book.cat; self.panel(*r, k, if sel { [0.28, 0.4, 0.22, 1.0] } else if on { [0.2, 0.24, 0.2, 1.0] } else { [0.0, 0.0, 0.0, 0.34] }); self.centered(if *i == 0 { "Tout" } else { BOOK_CATS[i - 1].1 }, *r, 14.0 * k, if sel { INK } else { DIM }); }
                Hit::Search => { self.panel(*r, k, if s.book.typing { [0.16, 0.2, 0.16, 1.0] } else { [0.0, 0.0, 0.0, 0.4] }); let (t, c) = if s.book.search.is_empty() && !s.book.typing { ("Rechercher...".to_string(), DIM) } else { (format!("{}{}", s.book.search, if s.book.typing && (s.elapsed * 2.0) as i64 % 2 == 0 { "|" } else { "" }), INK) }; self.text(&t, r[0] + 8.0 * k, r[1] + 5.0 * k, 15.0 * k, c); }
                Hit::Book(ri) => { let rc = &recipes()[*ri]; let ok = s.can_craft(rc, &st6); let sel = s.book.sel == Some(*ri); self.panel(*r, k, if sel { [0.28, 0.4, 0.22, 0.9] } else if on { [1.0, 1.0, 1.0, 0.16] } else { WELL });
                    self.icon(rc.out, r[0] + r[2] * 0.5, r[1] + r[3] * 0.5, r[2] * 0.43); if rc.n > 1 { let t = rc.n.to_string(); let z = 15.0 * k; self.text(&t, r[0] + r[2] - Ui::text_width(&t, z) - 4.0 * k, r[1] + r[3] - z - 4.0 * k, z, [1.0; 4]); }
                    if s.fav.contains(rc.out) { self.rect(r[0] + 4.0 * k, r[1] + 4.0 * k, 7.0 * k, 7.0 * k, GOLD); }
                    if !ok { self.rect(r[0] + 2.0 * k, r[1] + 2.0 * k, r[2] - 4.0 * k, r[3] - 4.0 * k, [0.05, 0.06, 0.05, 0.6]); } }
                Hit::Page(d) => { if pages > 1 { self.panel(*r, k, if on { [0.2, 0.24, 0.2, 1.0] } else { [0.0, 0.0, 0.0, 0.34] }); self.centered(if *d < 0 { "<" } else { ">" }, *r, 17.0 * k, INK); } }
                _ => {}
            } }
            let pr = l.iter().find(|r| r.0 == Hit::Page(-1)).map(|r| r.1).unwrap();
            if pages > 1 { let t = format!("{} / {}", s.book.page.min(pages - 1) + 1, pages); self.centered(&t, [xb, pr[1], bw, pr[3]], 15.0 * k, DIM); }
            // Détail de la recette : ce qu'elle fait, ce qu'il faut, où, et les boutons.
            let mut y = pr[1] + 36.0 * k;
            if let Some(ri) = shown(s, w, h, mouse) { let rc = &recipes()[ri]; let ok = s.can_craft(rc, &st6);
                if let Some(d) = def(rc.out) { self.icon(rc.out, xb + 22.0 * k, y + 20.0 * k, 20.0 * k); let name = if rc.n > 1 { format!("{} (x{})", d.label, rc.n) } else { d.label.to_string() }; self.text(&name, xb + 50.0 * k, y + 4.0 * k, 18.0 * k, INK); y += 44.0 * k;
                    for line in wrap(d.desc, 14.0 * k, bw).into_iter().take(3) { self.text(&line, xb, y, 14.0 * k, DIM); y += 18.0 * k; } }
                y += 4.0 * k;
                for n in &rc.need { let have = s.count(n.id); let label = group_label(n.id).or(item(n.id).map(|i| i.label)).unwrap_or(n.id); let t = if n.keep { format!("{label} (outil, conservé)") } else { format!("{} {label}  ·  {have}", n.n) }; self.text(&t, xb, y, 14.5 * k, if have >= n.n { DIM } else { BAD }); y += 19.0 * k; }
                let here = Session::station_ok(&st6, rc.at); self.text(rc.at.label(), xb, y, 14.5 * k, if here || rc.at == Station::Hand { DIM } else { BAD });
                // Au-delà du fer : le métier qui sait le faire, et où le trouver.
                if let Some(a) = rc.artisan { if let Some(c) = alw_core::data::db().craftsmen.iter().find(|c| c.id == a) { y += 19.0 * k; let got = s.creative || s.taught.contains(a); let t = if got { format!("Technique apprise auprès d'un {}", c.name.to_lowercase()) } else { format!("Demande un {} : villages de {}", c.name.to_lowercase(), c.villages.iter().map(|v| v.replace('_', " ")).collect::<Vec<_>>().join(", ")) }; for line in wrap(&t, 14.0 * k, bw) { self.text(&line, xb, y, 14.0 * k, if got { DIM } else { BAD }); y += 18.0 * k; } } }
                for (hit, r) in l.iter().filter(|x| matches!(x.0, Hit::Craft(_))) { let on = hover == Some(*hit); self.panel(*r, k, if ok { if on { [0.4, 0.58, 0.3, 1.0] } else { [0.28, 0.4, 0.22, 1.0] } } else { [0.12, 0.13, 0.12, 1.0] }); self.centered(if *hit == Hit::Craft(false) { "Fabriquer" } else { "Tout" }, *r, 16.0 * k, if ok { INK } else { DIM }); }
            } else { self.text("Choisissez une recette.", xb, y, 14.5 * k, DIM); y += 20.0 * k; for t in wrap("Clic droit : favori. Les recettes apparaissent dès qu'on a eu en main l'un de leurs ingrédients.", 14.0 * k, bw) { self.text(&t, xb, y, 14.0 * k, DIM); y += 18.0 * k; } }
            if let Some(Hit::Book(ri)) = hover { let rc = &recipes()[ri]; tip = vec![(item(rc.out).map_or(rc.out, |i| i.label).to_string(), INK), (if s.can_craft(rc, &st6) { "Clic : choisir  ·  clic droit : favori" } else { "Il manque quelque chose : voir le détail" }.to_string(), DIM)]; }
        }
        for (hit, r) in &l {
            let on = hover == Some(*hit);
            match hit {
                Hit::Tab(i) => { let sel = *i == s.tab; self.panel(*r, k, if sel { [0.28, 0.4, 0.22, 1.0] } else if on { [0.2, 0.24, 0.2, 1.0] } else { [0.0, 0.0, 0.0, 0.34] }); self.centered(tabs()[*i].0, *r, 13.5 * k, if sel { INK } else { DIM }); }
                Hit::Slot(i) => { let st = s.slot(*i); self.slot_view(s, st, *r, k, on); if on { if let Some(st) = st { tip = Ui::item_lines(s, &st, k); } } }
                Hit::Scroll(d) => {
                    // Flèches et ascenseur du catalogue : la barre montre où l'on est dans l'onglet.
                    self.panel(*r, k, if on { [0.25, 0.32, 0.24, 1.0] } else { [0.0, 0.0, 0.0, 0.4] }); let (cx, cy) = (r[0] + r[2] * 0.5, r[1] + r[3] * 0.5); let a = 6.0 * k * *d as f32;
                    self.quad([[cx - 6.0 * k, cy - a * 0.5], [cx + 6.0 * k, cy - a * 0.5], [cx, cy + a * 0.5], [cx, cy + a * 0.5]], [[0.0; 2]; 4], INK, LAYER_FLAT);
                    if *d < 0 { let total = cat_rows(s).max(1); let top = r[1] + r[3] + 6.0 * k; let len = 4.0 * 63.0 * k - 2.0 * r[3] - 22.0 * k; let row0 = s.cat_row.min(total.saturating_sub(CAT_ROWS));
                        self.rect(cx - 2.0 * k, top, 4.0 * k, len, [0.0, 0.0, 0.0, 0.45]); let th = len * (CAT_ROWS as f32 / total as f32).min(1.0); let ty = top + (len - th) * (row0 as f32 / (total - CAT_ROWS).max(1) as f32); self.rect(cx - 3.0 * k, ty, 6.0 * k, th, ACCENT); }
                }
                Hit::Catalog(id) => { self.panel(*r, k, if on { [1.0, 1.0, 1.0, 0.2] } else { WELL }); self.icon(id, r[0] + r[2] * 0.5, r[1] + r[2] * 0.5, r[2] * 0.43); if on { tip = vec![(item(id).map_or(*id, |i| i.label).to_string(), INK)]; } }
                _ => {}
            }
        }
        if let Some(c) = s.cursor { self.icon(c.item, mouse[0], mouse[1], 30.0 * k); if let Some(n) = c.n { self.text(&n.to_string(), mouse[0] + 12.0 * k, mouse[1] + 10.0 * k, 16.0 * k, [1.0; 4]); } }
        else { self.bubble(&tip, mouse, k); }
    }
    fn menu(&mut self, m: &Menu, w: f32, h: f32, mouse: [f32; 2]) {
        self.rect(0.0, 0.0, w, h, [0.02, 0.03, 0.03, 0.6]);
        let (p, k) = menu_panel(w, h); let (x0, z) = (p[0] + 34.0 * k, 18.0 * k); self.panel(p, k, PANEL); let hover = menu_at(w, h, m, mouse);
        self.text(if m.credits { "Crédits" } else if m.controls { "Commandes" } else { "A Living World" }, x0, p[1] + 10.0 * k, 28.0 * k, INK);
        if m.controls { let t = if m.listen.is_some() { "Appuyez sur une touche · Échap : annuler · Retour arrière : effacer" } else { "Cliquez une case puis appuyez sur la touche voulue" }; self.text(t, x0, p[1] + 58.0 * k, 14.0 * k, if m.listen.is_some() { GOLD } else { DIM }); }
        if m.credits { for (i, l) in CREDITS.iter().enumerate() { let t = i == 2 || i == 7 || i == 10; self.text(l, x0, p[1] + 78.0 * k + i as f32 * 28.0 * k, if t { z } else { 16.0 * k }, if t { ACCENT } else { INK }); } }
        for (a, r) in menu_layout(w, h, m) {
            let on = hover == Some(a); let plain: C = if on { [0.25, 0.3, 0.24, 1.0] } else { [0.0, 0.0, 0.0, 0.34] }; let sel: C = [0.28, 0.4, 0.22, 1.0];
            let label = |t: &str, u: &mut Ui| u.text(t, x0, r[1] - 24.0 * k, 16.0 * k, DIM);
            match a {
                M::Resume | M::Credits | M::Back | M::Quit | M::NewWorld | M::Controls | M::ResetKeys => { self.panel(r, k, plain); self.centered(match a { M::Resume => "Reprendre", M::Credits => "Crédits", M::Back => "Retour", M::NewWorld => "Mondes", M::Controls => "Commandes", M::ResetKeys => "Touches par défaut", _ => "Quitter" }, r, 16.0 * k, INK); }
                M::Tips => { self.panel(r, k, if m.tips { sel } else { plain }); self.centered(if m.tips { "Conseils : oui" } else { "Conseils : non" }, r, 14.5 * k, INK); }
                M::Bind(i, sl) => {
                    if sl == 0 { self.text(ACTIONS[i], x0, r[1] + 3.0 * k, 16.0 * k, INK); }
                    let wait = m.listen == Some((i, sl)); let name = &m.binds[i][sl];
                    self.panel(r, k, if wait { [0.55, 0.42, 0.15, 1.0] } else if on { [0.25, 0.3, 0.24, 1.0] } else { [0.0, 0.0, 0.0, 0.34] });
                    self.centered(if wait { "..." } else if name.is_empty() { "-" } else { name }, r, 14.5 * k, if name.is_empty() && !wait { DIM } else { INK });
                }
                M::Weather(i) => { if i == 0 { label("Météo", self); } self.panel(r, k, if m.weather == i { sel } else { plain }); self.centered(WEATHERS[i], r, 15.0 * k, INK); }
                M::Mood(i) => { if i == 0 { label("Ambiance musicale", self); } self.panel(r, k, if m.mood == i { sel } else { plain }); self.centered(MOODS[i], r, 14.0 * k, INK); }
                M::Quality(i) => { if i == 0 { label("Qualité graphique (au prochain lancement)", self); } self.panel(r, k, if m.quality == i { sel } else { plain }); self.centered(QUALITIES[i], r, 15.0 * k, INK); }
                M::Profile(i) => { if i == 0 { label("Survie (pour ce monde)", self); } self.panel(r, k, if m.profile == i { sel } else { plain }); self.centered(PROFILES[i].name, r, 15.0 * k, INK); }
                M::Hud(i) => { if i == 0 { label("Indicateurs de survie (Alt : tout afficher)", self); } self.panel(r, k, if m.hud == i { sel } else { plain }); self.centered(needs::MODES[i], r, 15.0 * k, INK); }
                M::Reduce => { self.panel(r, k, if m.reduce { sel } else { plain }); self.centered(if m.reduce { "Animations réduites : oui" } else { "Animations réduites : non" }, r, 14.5 * k, INK); }
                M::AutoStep => { self.panel(r, k, if m.autostep { sel } else { plain }); self.centered(if m.autostep { "Saut automatique : oui" } else { "Saut automatique : non" }, r, 14.5 * k, INK); }
                M::Marker => { self.panel(r, k, if m.marker { sel } else { plain }); self.centered(if m.marker { "Marqueur de dépouille : oui" } else { "Marqueur de dépouille : non" }, r, 14.5 * k, INK); }
                M::Vol(i) => { self.text(VOLS[i], x0, r[1], 16.0 * k, DIM); let t = 6.0 * k; let y = r[1] + (r[3] - t) * 0.5; self.rect(r[0], y, r[2], t, [0.0, 0.0, 0.0, 0.5]); self.rect(r[0], y, r[2] * m.vol[i], t, ACCENT); let hx = r[0] + r[2] * m.vol[i]; self.panel([hx - 7.0 * k, r[1] + 1.0 * k, 14.0 * k, r[3] - 2.0 * k], k, INK); }
            }
        }
    }
    /// L'écran titre, sur le monde qui tourne lentement derrière.
    fn title(&mut self, t: &Title, w: f32, h: f32, mouse: [f32; 2], menu_open: bool) {
        let k = scale(h); let hover = if menu_open { None } else { title_at(t, w, h, mouse) };
        let btn = |u: &mut Ui, r: [f32; 4], on: bool, main: bool| u.panel(r, k, if on { [0.3, 0.42, 0.24, 0.98] } else if main { [0.2, 0.3, 0.17, 0.92] } else { [0.05, 0.07, 0.06, 0.78] });
        match t.page {
            TPage::Main => {
                // Un voile sombre à gauche, qui s'efface vers le monde.
                for i in 0..24 { self.rect(0.0, 0.0, w * 0.75 * (1.0 - i as f32 / 24.0), h, [0.02, 0.03, 0.02, 0.055]); }
                self.text("A Living World", 86.0 * k, h * 0.2, 64.0 * k, INK); self.text("Survivre, bâtir, explorer un monde qui vit", 92.0 * k, h * 0.2 + 84.0 * k, 20.0 * k, [0.85, 0.88, 0.8, 0.9]);
                for (a, r) in title_layout(t, w, h) {
                    let on = hover == Some(a); btn(self, r, on, a == T::Continue);
                    match a {
                        T::Continue => { self.text("Continuer", r[0] + 20.0 * k, r[1] + 9.0 * k, 22.0 * k, INK); self.text(&t.current, r[0] + 20.0 * k, r[1] + 39.0 * k, 15.0 * k, [0.85, 0.9, 0.78, 0.9]); }
                        _ => { let l = match a { T::New => "Nouveau monde", T::Worlds => "Mondes", T::Settings => "Réglages", _ => "Quitter" }; self.text(l, r[0] + 20.0 * k, r[1] + (r[3] - 20.0 * k * 1.35) * 0.5, 20.0 * k, INK); }
                    }
                }
                let v = &format!("Version {} · V1 en préparation", env!("CARGO_PKG_VERSION")); self.text(v, w - Ui::text_width(v, 14.0 * k) - 18.0 * k, h - 30.0 * k, 14.0 * k, [0.85, 0.88, 0.8, 0.7]);
            }
            TPage::Worlds => {
                self.rect(0.0, 0.0, w, h, [0.02, 0.03, 0.03, 0.55]); let pw = (820.0 * k).min(w - 40.0); let x = (w - pw) * 0.5; let y = h * 0.16; let rh = 60.0 * k;
                self.panel([x, y, pw, 80.0 * k + ROWS as f32 * (rh + 6.0 * k) + 80.0 * k], k, PANEL); self.text("Mondes", x + 20.0 * k, y + 18.0 * k, 28.0 * k, INK);
                let n = format!("{} monde{}", t.worlds.len(), if t.worlds.len() > 1 { "s" } else { "" }); self.text(&n, x + pw - Ui::text_width(&n, 15.0 * k) - 20.0 * k, y + 30.0 * k, 15.0 * k, DIM);
                if t.worlds.is_empty() { self.text("Aucun monde pour l'instant : créez-en un.", x + 20.0 * k, y + 90.0 * k, 17.0 * k, DIM); }
                for (a, r) in title_layout(t, w, h) {
                    let on = hover == Some(a);
                    match a {
                        T::Row(i) => {
                            let wi = &t.worlds[i]; let sel = t.sel == i; self.panel(r, k, if sel { [0.24, 0.34, 0.2, 1.0] } else if on { [0.16, 0.2, 0.15, 1.0] } else { WELL });
                            let cur = if wi.name == t.current { "  (en cours)" } else { "" }; self.text(&format!("{}{cur}", wi.name), r[0] + 14.0 * k, r[1] + 8.0 * k, 19.0 * k, INK);
                            let days = (wi.hours / 24.0).floor(); let d = format!("{} · {} jour{} de jeu · graine {} · {}", PROFILES[wi.profile.min(PROFILES.len() - 1)].name, days, if days > 1.0 { "s" } else { "" }, wi.seed, wi.age);
                            self.text(&d, r[0] + 14.0 * k, r[1] + 34.0 * k, 14.0 * k, DIM);
                        }
                        T::Delete => { let warn = t.confirm; self.panel(r, k, if warn { [0.5, 0.2, 0.14, 1.0] } else if on { [0.3, 0.16, 0.12, 1.0] } else { [0.0, 0.0, 0.0, 0.34] }); self.centered(if warn { "Confirmer la suppression" } else { "Supprimer" }, r, 17.0 * k, INK); }
                        _ => { btn(self, r, on, a == T::Play); self.centered(if a == T::Play { "Jouer" } else { "Retour" }, r, 18.0 * k, INK); }
                    }
                }
                if t.worlds.len() > ROWS { let h2 = "Molette : faire défiler"; self.text(h2, x + 20.0 * k, y + 56.0 * k, 13.0 * k, DIM); }
            }
            TPage::New => {
                self.rect(0.0, 0.0, w, h, [0.02, 0.03, 0.03, 0.55]); let pw = (820.0 * k).min(w - 40.0); let x = (w - pw) * 0.5; let y = h * 0.16;
                self.panel([x, y, pw, 480.0 * k], k, PANEL); self.text("Nouveau monde", x + 20.0 * k, y + 18.0 * k, 28.0 * k, INK);
                for (a, r) in title_layout(t, w, h) {
                    let on = hover == Some(a);
                    match a {
                        T::Field(f) => {
                            self.text(if f == 0 { "Nom du monde" } else { "Graine (facultative : un nombre ou un mot)" }, r[0], r[1] - 26.0 * k, 16.0 * k, DIM);
                            let act = t.field == f; self.panel(r, k, if act { [0.12, 0.16, 0.12, 1.0] } else { WELL }); let v = if f == 0 { &t.name } else { &t.seed };
                            let shown = if v.is_empty() && !act { if f == 0 { "Mon monde".to_string() } else { "au hasard".to_string() } } else { format!("{v}{}", if act { "_" } else { "" }) };
                            self.text(&shown, r[0] + 14.0 * k, r[1] + 11.0 * k, 18.0 * k, if v.is_empty() && !act { DIM } else { INK });
                        }
                        T::Profile(i) => {
                            if i == 0 { self.text("Survie", r[0], r[1] - 26.0 * k, 16.0 * k, DIM); }
                            let sel = t.profile == i; self.panel(r, k, if sel { [0.24, 0.36, 0.2, 1.0] } else if on { [0.16, 0.2, 0.15, 1.0] } else { WELL });
                            self.text(PROFILES[i].name, r[0] + 12.0 * k, r[1] + 10.0 * k, 18.0 * k, INK);
                            for (j, l) in wrap(&profile_line(i), 13.5 * k, r[2] - 24.0 * k).iter().take(3).enumerate() { self.text(l, r[0] + 12.0 * k, r[1] + 38.0 * k + j as f32 * 18.0 * k, 13.5 * k, DIM); }
                        }
                        _ => { btn(self, r, on, a == T::Create); self.centered(if a == T::Create { "Créer et jouer" } else { "Retour" }, r, 18.0 * k, INK); }
                    }
                }
                self.text("Tab : champ suivant · Entrée : créer", x + 20.0 * k, y + 456.0 * k, 13.0 * k, DIM);
            }
        }
    }
    /// Le conseil de départ en cours, en haut à gauche.
    fn hint(&mut self, s: &Session, k: f32) {
        let Some((n, total, t)) = s.hint() else { return }; let z = 16.0 * k; let lines = wrap(&t, z, 400.0 * k); let bw = lines.iter().fold(0.0f32, |a, l| a.max(Ui::text_width(l, z))).max(160.0 * k) + 28.0 * k;
        let r = [16.0 * k, 16.0 * k, bw, 42.0 * k + lines.len() as f32 * 22.0 * k]; self.panel(r, k, [0.06, 0.08, 0.06, 0.78]); self.rect(r[0], r[1], 3.0 * k, r[3], ACCENT);
        self.text(&format!("Premiers pas · {n}/{total}"), r[0] + 14.0 * k, r[1] + 9.0 * k, 13.5 * k, ACCENT);
        for (i, l) in lines.iter().enumerate() { self.text(l, r[0] + 14.0 * k, r[1] + 32.0 * k + i as f32 * 22.0 * k, z, INK); }
    }
    fn death(&mut self, s: &Session, d: &alw_core::survival::Death, w: f32, h: f32, k: f32, mouse: [f32; 2]) {
        self.rect(0.0, 0.0, w, h, [0.16, 0.02, 0.02, 0.72]);
        let line = |u: &mut Ui, t: &str, y: f32, z: f32, c: C| u.text(t, (w - Ui::text_width(t, z)) * 0.5, y, z, c);
        line(self, "Vous êtes mort", h * 0.33, 46.0 * k, INK);
        if !d.cause.is_empty() { line(self, &format!("Cause : {}", d.cause), h * 0.33 + 66.0 * k, 20.0 * k, [0.95, 0.75, 0.7, 1.0]); }
        if !s.stashes.is_empty() { line(self, &format!("Vos affaires sont restées dans un sac, là où vous êtes tombé. Il y restera {:.0} jours.", PROFILES[s.profile].stash_days), h * 0.33 + 98.0 * k, 17.0 * k, DIM); }
        let r = death_button(w, h); let on = inside(mouse, r); self.panel(r, k, if on { [0.5, 0.2, 0.14, 1.0] } else { [0.3, 0.1, 0.08, 1.0] }); self.centered(if s.bed.is_some() { "Réapparaître au lit" } else { "Réapparaître" }, r, 19.0 * k, INK);
        line(self, "ou Entrée", r[1] + r[3] + 12.0 * k, 15.0 * k, DIM);
    }
    fn quad(&mut self, p: [[f32; 2]; 4], uv: [[f32; 2]; 4], c: C, l: f32) { for i in [0, 1, 2, 0, 2, 3] { self.data.extend_from_slice(&[p[i][0], p[i][1], uv[i][0], uv[i][1], c[0], c[1], c[2], c[3], l]); } }
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, c: C) { self.quad([[x, y], [x + w, y], [x + w, y + h], [x, y + h]], [[0.0; 2]; 4], c, LAYER_FLAT); }
    fn diamond(&mut self, x: f32, y: f32, r: f32, c: C) { self.quad([[x, y - r], [x + r, y], [x, y + r], [x - r, y]], [[0.0; 2]; 4], c, LAYER_FLAT); }
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
        let Some(i) = order().iter().position(|o| *o == id) else { return }; let n = ICON_COLS as usize; let c = 1.0 / n as f32; let (u, v) = ((i % n) as f32 * c, (i / n) as f32 * c);
        self.quad([[cx - r, cy - r], [cx + r, cy - r], [cx + r, cy + r], [cx - r, cy + r]], [[u, v], [u + c, v], [u + c, v + c], [u, v + c]], [1.0; 4], LAYER_ICON);
    }
    fn dialog(&mut self, s: &Session, w: f32, h: f32, mouse: [f32; 2]) {
        let Some(d) = s.dialog() else { return }; let k = scale(h); let (r, rows) = dialog_layout(d.options.len(), w, h);
        self.panel(r, k, PANEL); self.text(&d.title, r[0] + 18.0 * k, r[1] + 14.0 * k, 23.0 * k, INK); self.text(&d.sub, r[0] + 18.0 * k, r[1] + 58.0 * k, 15.0 * k, DIM);
        for (i, l) in wrap(&format!("« {} »", d.said), 17.0 * k, r[2] - 36.0 * k).iter().take(2).enumerate() { self.text(l, r[0] + 18.0 * k, r[1] + 84.0 * k + i as f32 * 22.0 * k, 17.0 * k, GOLD); }
        for (i, (row, (t, ok))) in rows.iter().zip(d.options.iter()).enumerate() {
            let on = inside(mouse, *row); self.panel(*row, k, if on && *ok { [1.0, 1.0, 1.0, 0.16] } else { WELL });
            let z = 16.0 * k; let mut label = format!("{}. {t}", i + 1); while Ui::text_width(&label, z) > row[2] - 20.0 * k && label.chars().count() > 8 { label.pop(); label.pop(); label.push_str("..."); }
            self.text(&label, row[0] + 10.0 * k, row[1] + (row[3] - z * 1.35) * 0.5, z, if *ok { INK } else { [0.6, 0.62, 0.58, 0.8] });
        }
        let t = "Chiffres ou clic pour répondre · Échap pour partir"; let z = 14.0 * k; self.text(t, r[0] + r[2] - Ui::text_width(t, z) - 16.0 * k, r[1] + r[3] - 24.0 * k, z, DIM);
    }
    /// Journal des villages : connus, du plus proche au plus lointain, avec leurs artisans (rencontrés en couleur).
    fn journal(&mut self, s: &Session, w: f32, h: f32) {
        let k = scale(h); let l = s.journal(); let lh = 46.0 * k; let pw = (760.0 * k).min(w - 40.0); let ph = 104.0 * k + l.len().clamp(1, 9) as f32 * lh; let (x, y) = ((w - pw) * 0.5, h * 0.12);
        self.panel([x, y, pw, ph], k, PANEL); self.text("Journal des villages", x + 18.0 * k, y + 14.0 * k, 23.0 * k, INK);
        self.text("Chiffre : marquer le village sur votre route · J pour fermer", x + 18.0 * k, y + 58.0 * k, 14.0 * k, DIM);
        if l.is_empty() { self.text("Aucun village connu. Suivez les chemins, ou demandez aux voyageurs.", x + 18.0 * k, y + 80.0 * k, 16.0 * k, DIM); }
        for (i, (v, at, crafts)) in l.iter().take(9).enumerate() {
            let yy = y + 90.0 * k + i as f32 * lh; let marked = s.town.marked == Some(v.id);
            self.text(&format!("{}. {}{} · {} · {}", i + 1, v.name, if marked { " (marqué)" } else { "" }, alw_core::village::kind_label(v.kind), at), x + 18.0 * k, yy, 17.0 * k, if marked { GOLD } else { INK });
            let mut px = x + 40.0 * k;
            for (c, met) in crafts { let n = alw_core::village::craftsman(c).map_or(*c, |x| x.name); let z = 14.0 * k; self.text(n, px, yy + 22.0 * k, z, if *met { ACCENT } else { DIM }); px += Ui::text_width(n, z) + 18.0 * k; }
            let r = format!("réputation {}", s.rep(v.id)); let z = 14.0 * k; self.text(&r, x + pw - Ui::text_width(&r, z) - 18.0 * k, yy + 22.0 * k, z, DIM);
        }
    }
    /// Repère en haut de l'écran : un losange dans la direction d'un point, avec un nom et la distance.
    fn pin(&mut self, s: &Session, at: [f64; 2], label: &str, c: C, w: f32, k: f32) {
        let p = s.player.p; let (dx, dz) = (at[0] - p[0], at[1] - p[2]); let dist = dx.hypot(dz); if dist < 12.0 { return; }
        let mut rel = dx.atan2(dz) - s.player.yaw; while rel > std::f64::consts::PI { rel -= std::f64::consts::TAU; } while rel < -std::f64::consts::PI { rel += std::f64::consts::TAU; }
        let x = w * 0.5 - (rel / std::f64::consts::FRAC_PI_2).clamp(-1.0, 1.0) as f32 * w * 0.3; let y = 26.0 * k;
        self.diamond(x, y, 7.0 * k, c); let t = format!("{label} · {:.0} m", dist); self.text(&t, x - Ui::text_width(&t, 14.0 * k) * 0.5, y + 10.0 * k, 14.0 * k, [0.92, 0.92, 0.88, 0.85]);
    }
    /// Le terrain vu, en cases de `cell` blocs de `px` pixels, centré sur `at` (x, z du monde) et découpé au cadre `r`.
    /// Le relief s'ombre comme éclairé du nord-ouest ; l'inexploré reste vide (le fond du cadre).
    fn terrain(&mut self, a: &Atlas, r: [f32; 4], at: [f64; 2], cell: i32, px: f32, alpha: f32) {
        let g = (cell / STEP).max(1); let (cw, ch) = (r[2] / px, r[3] / px);
        let (fx, fz) = (at[0] / cell as f64 - cw as f64 * 0.5, at[1] / cell as f64 - ch as f64 * 0.5); let (i0, j0) = (fx.floor() as i32, fz.floor() as i32);
        let (ox, oz) = (r[0] - (fx - i0 as f64) as f32 * px, r[1] - (fz - j0 as f64) as f32 * px);
        let shade = |i: i32, j: i32| -> Option<[f32; 3]> {
            let (mx, mz) = (i * g, j * g); let (c, h) = unpack(a.get(mx, mz))?; let hn = unpack(a.get(mx - g, mz - g)).map_or(h, |v| v.1);
            let f = (1.0 + (h as f32 - hn as f32) / g as f32 * 0.09).clamp(0.7, 1.3); Some([0, 1, 2].map(|k| ((c[k] as f32 / 255.0 * f).min(1.0) * 24.0).round() / 24.0))
        };
        for j in 0..=ch.ceil() as i32 + 1 {
            let y = oz + j as f32 * px; let (y0, y1) = (y.max(r[1]), (y + px).min(r[1] + r[3])); if y1 <= y0 { continue; }
            let mut i = 0; let n = cw.ceil() as i32 + 2;
            while i < n {
                let Some(c) = shade(i0 + i, j0 + j) else { i += 1; continue };
                let mut e = i + 1; while e < n && shade(i0 + e, j0 + j) == Some(c) { e += 1; }
                let (x0, x1) = ((ox + i as f32 * px).max(r[0]), (ox + e as f32 * px).min(r[0] + r[2])); if x1 > x0 { self.rect(x0, y0, x1 - x0, y1 - y0, [c[0], c[1], c[2], alpha]); }
                i = e;
            }
        }
    }
    /// Position à l'écran d'un point du monde dans un cadre de carte, s'il y tombe.
    fn on_map(r: [f32; 4], at: [f64; 2], cell: i32, px: f32, q: [f64; 2]) -> Option<[f32; 2]> {
        let x = r[0] + r[2] * 0.5 + ((q[0] - at[0]) / cell as f64) as f32 * px; let y = r[1] + r[3] * 0.5 + ((q[1] - at[1]) / cell as f64) as f32 * px;
        (x >= r[0] && x <= r[0] + r[2] && y >= r[1] && y <= r[1] + r[3]).then_some([x, y])
    }
    /// La flèche du joueur : pointe dans le sens du regard (le nord en haut).
    fn arrow(&mut self, p: [f32; 2], yaw: f64, z: f32, c: C) {
        let (f, q) = ([yaw.sin() as f32, yaw.cos() as f32], [-(yaw.cos() as f32), yaw.sin() as f32]);
        let at = |a: f32, b: f32| [p[0] + f[0] * a + q[0] * b, p[1] + f[1] * a + q[1] * b];
        self.quad([at(z, 0.0), at(-z * 0.7, z * 0.65), at(-z * 0.35, 0.0), at(-z * 0.7, -z * 0.65)], [[0.0; 2]; 4], c, LAYER_FLAT);
    }
    fn places(s: &Session) -> Vec<(alw_core::village::Village, bool)> {
        s.town.known.iter().filter_map(|id| s.town.here.iter().find(|v| v.id == *id).or(s.town.atlas.get(id)).cloned()).map(|v| { let m = s.town.marked == Some(v.id); (v, m) }).collect()
    }
    /// Mini-carte en boussole, en haut à droite : la carte tourne avec le regard (devant soi en haut), couronne graduée,
    /// points cardinaux qui tournent autour ; couleurs lissées et relief ombré pour se lire d'un coup d'œil.
    fn minimap(&mut self, s: &Session, w: f32, k: f32) {
        const R_BLOCKS: f64 = 60.0; let rad = 92.0 * k; let ring = 17.0 * k; let c = [w - rad - ring - 16.0 * k, rad + ring + 14.0 * k];
        let sc = rad / R_BLOCKS as f32; let yaw = s.player.yaw; let (f, r) = ([yaw.sin(), yaw.cos()], [-yaw.cos(), yaw.sin()]); let me = [s.player.p[0], s.player.p[2]];
        let to = |dx: f64, dz: f64| [c[0] + ((dx * r[0] + dz * r[1]) as f32) * sc, c[1] - ((dx * f[0] + dz * f[1]) as f32) * sc];
        let circle = |u: &mut Ui, r0: f32, r1: f32, col: C, n: usize| for i in 0..n {
            let (a, b) = (i as f32 / n as f32 * std::f32::consts::TAU, (i + 1) as f32 / n as f32 * std::f32::consts::TAU);
            let p = |rr: f32, t: f32| [c[0] + rr * t.cos(), c[1] + rr * t.sin()]; u.quad([p(r0, a), p(r1, a), p(r1, b), p(r0, b)], [[0.0; 2]; 4], col, LAYER_FLAT);
        };
        // Le fond : l'inexploré, sombre.
        circle(self, 0.0, rad, [0.1, 0.12, 0.1, 0.88], 48);
        // Le terrain, par rangées de cases de 4 blocs, coupé au cercle. Couleur lissée sur les voisines, relief ombré.
        let a = &s.map; let g = STEP; let (pi, pj) = ((me[0] / g as f64).floor() as i32, (me[1] / g as f64).floor() as i32); let n = (R_BLOCKS / g as f64).ceil() as i32 + 1;
        let col = |i: i32, j: i32| -> Option<[f32; 3]> {
            let (c0, h) = unpack(a.get(i, j))?; let mut acc = [c0[0] as f32 * 4.0, c0[1] as f32 * 4.0, c0[2] as f32 * 4.0]; let mut wsum = 4.0;
            for (di, dj, wt) in [(1, 0, 2.0), (-1, 0, 2.0), (0, 1, 2.0), (0, -1, 2.0), (1, 1, 1.0), (-1, 1, 1.0), (1, -1, 1.0), (-1, -1, 1.0)] { if let Some((cn, _)) = unpack(a.get(i + di, j + dj)) { for q in 0..3 { acc[q] += cn[q] as f32 * wt; } wsum += wt; } }
            let hn = unpack(a.get(i - 1, j - 1)).map_or(h, |v| v.1); let fshade = (1.0 + (h as f32 - hn as f32) * 0.12).clamp(0.65, 1.35);
            Some([0, 1, 2].map(|q| ((acc[q] / wsum / 255.0 * fshade * 1.08).min(1.0) * 16.0).round() / 16.0))
        };
        for j in pj - n..=pj + n {
            let (z0, z1) = ((j * g) as f64 - me[1], ((j + 1) * g) as f64 - me[1]); let zc = (z0 + z1) * 0.5; if zc.abs() >= R_BLOCKS { continue; }
            let hw = (R_BLOCKS * R_BLOCKS - zc * zc).sqrt(); let mut i = pi - n;
            while i <= pi + n {
                let Some(cc) = col(i, j) else { i += 1; continue }; let mut e = i + 1; while e <= pi + n && col(e, j) == Some(cc) { e += 1; }
                let (x0, x1) = (((i * g) as f64 - me[0]).max(-hw), ((e * g) as f64 - me[0]).min(hw));
                if x1 > x0 { self.quad([to(x0, z0), to(x1, z0), to(x1, z1), to(x0, z1)], [[0.0; 2]; 4], [cc[0], cc[1], cc[2], 1.0], LAYER_FLAT); }
                i = e;
            }
        }
        // Villages et dépouilles ; le village marqué hors de portée reste indiqué au bord.
        for (v, m) in Ui::places(s) {
            let (dx, dz) = (v.c[0] as f64 + 0.5 - me[0], v.c[2] as f64 + 0.5 - me[1]); let d = dx.hypot(dz);
            if d < R_BLOCKS - 4.0 { let p = to(dx, dz); self.diamond(p[0], p[1], 7.5 * k, [0.1, 0.08, 0.05, 0.95]); self.diamond(p[0], p[1], 5.5 * k, if m { GOLD } else { [0.97, 0.94, 0.85, 1.0] }); }
            else if m { let p = to(dx / d * (R_BLOCKS - 5.0), dz / d * (R_BLOCKS - 5.0)); self.diamond(p[0], p[1], 6.5 * k, [0.1, 0.08, 0.05, 0.95]); self.diamond(p[0], p[1], 4.5 * k, GOLD); }
        }
        for st in &s.stashes { let (dx, dz) = (st.p[0] - me[0], st.p[2] - me[1]); if dx.hypot(dz) < R_BLOCKS - 3.0 { let p = to(dx, dz); self.diamond(p[0], p[1], 5.0 * k, BAD); } }
        // La couronne : bronze, graduée tous les 15°, avec N (rouge), E, S, O qui tournent avec le regard.
        circle(self, rad, rad + ring, [0.2, 0.15, 0.1, 0.96], 64); circle(self, rad - 1.5 * k, rad + 1.0 * k, [0.78, 0.63, 0.38, 1.0], 64); circle(self, rad + ring - 1.5 * k, rad + ring, [0.55, 0.42, 0.25, 1.0], 64);
        let north = { let p = to(0.0, -1.0); (p[1] - c[1]).atan2(p[0] - c[0]) };
        for t in 0..24 { if t % 6 == 0 { continue; } let a = north + t as f32 * std::f32::consts::TAU / 24.0; let (l0, l1) = (rad + 3.0 * k, rad + if t % 2 == 0 { 8.0 } else { 5.0 } * k); let (ca, sa) = (a.cos(), a.sin()); let (px, py) = (-sa * 0.9 * k, ca * 0.9 * k);
            self.quad([[c[0] + ca * l0 + px, c[1] + sa * l0 + py], [c[0] + ca * l1 + px, c[1] + sa * l1 + py], [c[0] + ca * l1 - px, c[1] + sa * l1 - py], [c[0] + ca * l0 - px, c[1] + sa * l0 - py]], [[0.0; 2]; 4], [0.86, 0.74, 0.52, 0.9], LAYER_FLAT); }
        let z = 14.0 * k;
        for (q, l) in ["N", "E", "S", "O"].iter().enumerate() {
            let a = north + q as f32 * std::f32::consts::FRAC_PI_2; let p = [c[0] + (rad + ring * 0.5) * a.cos(), c[1] + (rad + ring * 0.5) * a.sin()];
            if q == 0 { self.diamond(p[0], p[1], 9.5 * k, [0.72, 0.16, 0.12, 1.0]); }
            self.text(l, p[0] - Ui::text_width(l, z) * 0.5, p[1] - z * 0.62, z, if q == 0 { [1.0, 1.0, 1.0, 1.0] } else { [0.95, 0.88, 0.7, 1.0] });
        }
        // Soi-même au centre, tourné vers le haut.
        self.arrow(c, std::f64::consts::PI, 13.0 * k, [0.08, 0.06, 0.04, 1.0]); self.arrow(c, std::f64::consts::PI, 9.5 * k, [1.0, 1.0, 1.0, 1.0]);
        let t = format!("{:.0}, {:.0}", s.player.p[0].floor(), s.player.p[2].floor()); let z = 13.0 * k; let tw = Ui::text_width(&t, z);
        self.rect(c[0] - tw * 0.5 - 6.0 * k, c[1] + rad + ring + 4.0 * k, tw + 12.0 * k, z * 1.5, [0.0, 0.0, 0.0, 0.45]); self.text(&t, c[0] - tw * 0.5, c[1] + rad + ring + 5.0 * k, z, [0.95, 0.93, 0.86, 0.95]);
    }
    /// La grande carte : un parchemin qui se déroule depuis le milieu, puis le terrain qui s'y dessine ; les plis s'estompent.
    fn big_map(&mut self, s: &Session, w: f32, h: f32, k: f32) {
        let a = s.map.anim as f32; if a <= 0.0 { return; } let e = 1.0 - (1.0 - a).powi(3);
        let t1 = (e / 0.55).min(1.0); let t2 = ((e - 0.45) / 0.55).clamp(0.0, 1.0);
        self.rect(0.0, 0.0, w, h, [0.02, 0.03, 0.02, 0.5 * e]);
        let full = [44.0 * k, 44.0 * k, w - 88.0 * k, h - 88.0 * k]; let pw = full[2] * t1; let ph = full[3] * (0.9 + 0.1 * t1);
        let r = [full[0] + (full[2] - pw) * 0.5, full[1] + (full[3] - ph) * 0.5, pw, ph];
        let paper = [0.79, 0.71, 0.54, 0.97 * t1.max(0.3)]; let edge = 6.0 * k;
        self.rect(r[0] - edge, r[1] - edge, r[2] + 2.0 * edge, r[3] + 2.0 * edge, [0.36, 0.27, 0.16, 0.95 * t1.max(0.3)]); self.rect(r[0], r[1], r[2], r[3], paper);
        // Les rouleaux de chaque côté pendant le déroulé.
        if t1 < 1.0 { for x in [r[0] - edge * 1.8, r[0] + r[2] - edge * 0.2] { self.rect(x, r[1] - edge * 2.0, edge * 2.0, r[3] + edge * 4.0, [0.45, 0.33, 0.2, 1.0]); } }
        if t2 <= 0.0 { return; }
        let cell = ZOOMS[s.map.zoom]; let px = 6.0 * k; let me = [s.player.p[0], s.player.p[2]]; let inner = [r[0] + 10.0 * k, r[1] + 54.0 * k, r[2] - 20.0 * k, r[3] - 84.0 * k];
        self.terrain(&s.map, inner, me, cell, px, 0.9 * t2);
        for i in 1..4 { let x = r[0] + r[2] * i as f32 / 4.0; self.rect(x - k, r[1], 2.0 * k, r[3], [0.3, 0.22, 0.12, 0.12 + 0.45 * (1.0 - t2)]); }
        self.rect(r[0], r[1] + r[3] * 0.5 - k, r[2], 2.0 * k, [0.3, 0.22, 0.12, 0.12 + 0.45 * (1.0 - t2)]);
        let ink = [0.22, 0.15, 0.08, t2]; let z = 15.0 * k;
        for (v, m) in Ui::places(s) { if let Some(p) = Ui::on_map(inner, me, cell, px, [v.c[0] as f64 + 0.5, v.c[2] as f64 + 0.5]) {
            self.diamond(p[0], p[1], 8.0 * k, [0.22, 0.15, 0.08, t2]); self.diamond(p[0], p[1], 5.5 * k, if m { [0.95, 0.75, 0.2, t2] } else { [0.95, 0.9, 0.78, t2] });
            self.text(&v.name, p[0] - Ui::text_width(&v.name, z) * 0.5, p[1] + 9.0 * k, z, ink);
        } }
        for st in &s.stashes { if let Some(p) = Ui::on_map(inner, me, cell, px, [st.p[0], st.p[2]]) { self.diamond(p[0], p[1], 6.0 * k, [0.75, 0.15, 0.1, t2]); self.text("Dépouille", p[0] + 8.0 * k, p[1] - 8.0 * k, z, ink); } }
        let c = [inner[0] + inner[2] * 0.5, inner[1] + inner[3] * 0.5]; self.arrow(c, s.player.yaw, 13.0 * k, [0.22, 0.15, 0.08, t2]); self.arrow(c, s.player.yaw, 9.0 * k, [1.0, 1.0, 1.0, t2]);
        self.text("Carte", r[0] + 18.0 * k, r[1] + 14.0 * k, 24.0 * k, ink);
        let ha = s.map.explored() as f64 * (STEP * STEP) as f64 / 1e4; let info = format!("1 case = {cell} blocs · exploré : {ha:.0} hectares · nord en haut"); self.text(&info, r[0] + 110.0 * k, r[1] + 22.0 * k, z, [0.3, 0.22, 0.12, t2]);
        let help = "Molette : échelle · M ou Échap : fermer"; self.text(help, r[0] + r[2] - Ui::text_width(help, z) - 18.0 * k, r[1] + r[3] - 24.0 * k, z, [0.3, 0.22, 0.12, t2]);
    }
    /// Marqueur discret de la dépouille : un losange sur un arc en haut de l'écran, et la distance.
    fn marker(&mut self, s: &Session, w: f32, k: f32) {
        let p = s.player.p; let Some(st) = s.stashes.iter().min_by(|a, b| ((a.p[0] - p[0]).hypot(a.p[2] - p[2])).total_cmp(&(b.p[0] - p[0]).hypot(b.p[2] - p[2]))) else { return };
        let (dx, dz) = (st.p[0] - p[0], st.p[2] - p[2]); let dist = dx.hypot(dz); if dist < 3.0 { return; }
        let mut rel = dx.atan2(dz) - s.player.yaw; while rel > std::f64::consts::PI { rel -= std::f64::consts::TAU; } while rel < -std::f64::consts::PI { rel += std::f64::consts::TAU; }
        let x = w * 0.5 - (rel / std::f64::consts::FRAC_PI_2).clamp(-1.0, 1.0) as f32 * w * 0.3; let y = 26.0 * k; // regarder vers +z place +x à gauche
        self.diamond(x, y, 7.0 * k, [0.95, 0.78, 0.36, 0.9]); let t = format!("{:.0} m", dist); self.text(&t, x - Ui::text_width(&t, 14.0 * k) * 0.5, y + 10.0 * k, 14.0 * k, [0.95, 0.9, 0.8, 0.85]);
    }
    pub fn build(s: &Session, w: f32, h: f32, msg: Option<&str>, mouse: [f32; 2], menu: &Menu, info: &[String]) -> Ui {
        let mut u = Ui { data: Vec::with_capacity(12000), w, h }; let k = scale(h); let w1 = [1.0, 1.0, 1.0, 0.9];
        if let Some(t) = &menu.title { u.title(t, w, h, mouse, menu.open); if menu.open { u.menu(menu, w, h, mouse); } if let Some(m) = msg { let z = 20.0 * k; u.text(m, (w - Ui::text_width(m, z)) * 0.5, h * 0.08, z, w1); } return u; }
        // Blessure : l'écran rougit un instant ; à bout de forces, les bords restent rouges et battent.
        let fl = s.flash() as f32; if fl > 0.0 { u.rect(0.0, 0.0, w, h, [0.62, 0.04, 0.03, 0.3 * fl]); }
        if !s.creative && s.dead.is_none() && s.vit[0] < 25.0 { let a = (0.16 + 0.1 * if menu.reduce { 0.5 } else { (s.elapsed as f32 * 4.0).sin() }) * (1.0 - s.vit[0] as f32 / 25.0).max(0.35); let e = 60.0 * k; let c = [0.55, 0.03, 0.02, a];
            u.rect(0.0, 0.0, w, e, c); u.rect(0.0, h - e, w, e, c); u.rect(0.0, e, e, h - 2.0 * e, c); u.rect(w - e, e, e, h - 2.0 * e, c); }
        // Coup chargé : un anneau qui se remplit autour du réticule ; garde levée : un écu sous le réticule, doré tant qu'elle pare.
        if s.charge > 0.04 { let f = (s.charge / alw_core::survival::HEAVY_T).min(1.0) as f32; let n = 28; let (r0, r1) = (13.0 * k, 17.0 * k); let (cx, cy) = (w * 0.5, h * 0.5);
            for i in 0..n { let (a, b) = (-std::f32::consts::FRAC_PI_2 + i as f32 / n as f32 * std::f32::consts::TAU, -std::f32::consts::FRAC_PI_2 + (i + 1) as f32 / n as f32 * std::f32::consts::TAU); let on = (i as f32 + 0.5) / n as f32 <= f;
                let p = |r: f32, t: f32| [cx + r * t.cos(), cy + r * t.sin()]; u.quad([p(r0, a), p(r1, a), p(r1, b), p(r0, b)], [[0.0; 2]; 4], if on { if f >= 0.98 { [1.0, 0.55, 0.3, 0.95] } else { [1.0, 1.0, 1.0, 0.85] } } else { [0.0, 0.0, 0.0, 0.3] }, LAYER_FLAT); } }
        if s.guard_t >= 0.0 { let parry = s.guard_t <= alw_core::survival::PARRY_T; let c = if parry { GOLD } else { [0.85, 0.88, 0.9, 0.85] }; let (x, y, r) = (w * 0.5, h * 0.5 + 30.0 * k, 9.0 * k);
            u.quad([[x - r, y - r], [x + r, y - r], [x + r * 0.7, y + r * 0.4], [x - r * 0.7, y + r * 0.4]], [[0.0; 2]; 4], c, LAYER_FLAT); u.quad([[x - r * 0.7, y + r * 0.4], [x + r * 0.7, y + r * 0.4], [x, y + r * 1.3], [x, y + r * 1.3]], [[0.0; 2]; 4], c, LAYER_FLAT); }
        // Le réticule rougit quand il est sur un animal à portée.
        let ch = if s.prey.is_some() { [1.0, 0.5, 0.4, 1.0] } else { w1 }; let t = if s.prey.is_some() { 3.0 } else { 2.0 };
        u.rect(w * 0.5 - t * 0.5, h * 0.5 - 8.0, t, 16.0, ch); u.rect(w * 0.5 - 8.0, h * 0.5 - t * 0.5, 16.0, t, ch);
        let cell = 62.0 * k; let gap = 5.0 * k; let total = 9.0 * cell + 8.0 * gap; let (x0, y0) = ((w - total) * 0.5, h - cell - 16.0 * k);
        for i in 0..9.min(INV_SIZE) {
            let x = x0 + i as f32 * (cell + gap); let on = i == s.sel;
            u.panel([x, y0, cell, cell], k, if on { [1.0, 1.0, 1.0, 0.3] } else { [0.08, 0.1, 0.09, 0.55] });
            if on { let b = 2.0 * k; let g: C = [0.96, 0.84, 0.5, 1.0]; u.rect(x - b, y0 - b, cell + 2.0 * b, b, g); u.rect(x - b, y0 + cell, cell + 2.0 * b, b, g); u.rect(x - b, y0, b, cell, g); u.rect(x + cell, y0, b, cell, g); u.rect(x, y0 + cell - 3.0 * k, cell, 3.0 * k, [0.96, 0.84, 0.5, 0.5]); }
            if let Some(st) = s.inv[i] {
                u.icon(st.item, x + cell * 0.5, y0 + cell * 0.5, cell * 0.43);
                if let (Some(n), false) = (st.n, s.creative) { if n > 1 || def(st.item).is_some_and(|d| d.stack > 1) { let t = n.to_string(); let z = 17.0 * k; u.text(&t, x + cell - Ui::text_width(&t, z) - 5.0 * k, y0 + cell - z - 5.0 * k, z, w1); } }
                if let (Some(wv), false) = (s.wear_of(&st), s.creative) { if st.wear > 0 { let left = (1.0 - wv as f32).clamp(0.0, 1.0); let (bx, by, bw) = (x + 7.0 * k, y0 + cell - 7.0 * k, cell - 14.0 * k); u.rect(bx, by, bw, 3.0 * k, [0.0, 0.0, 0.0, 0.6]); u.rect(bx, by, bw * left, 3.0 * k, if left > 0.5 { ACCENT } else if left > 0.2 { GOLD } else { BAD }); } }
            }
        }
        if let Some(it) = s.held() { let z = 22.0 * k; u.text(it.label, (w - Ui::text_width(it.label, z)) * 0.5, y0 - z - 12.0 * k, z, w1); }
        if !s.creative {
            let o = needs::Opts { mode: menu.hud, reduce: menu.reduce, alt: menu.alt }; needs::row(&mut u, s, h, k, &o);
            if s.carried() > s.capacity() { u.text("Surchargé", 18.0 * k, h - 126.0 * k, 15.0 * k, BAD); }
            if menu.marker && s.dead.is_none() && !s.stashes.is_empty() { u.marker(s, w, k); }
        } else { let z = 18.0 * k; u.text("Mode créatif", 18.0 * k, h - 34.0 * k, z, w1); }
        if !info.is_empty() { let z = 16.0 * k; let bw = info.iter().fold(0.0f32, |a, l| a.max(Ui::text_width(l, z))); u.rect(10.0 * k, 10.0 * k, bw + 24.0 * k, info.len() as f32 * 22.0 * k + 14.0 * k, [0.0, 0.0, 0.0, 0.5]); for (i, l) in info.iter().enumerate() { u.text(l, 22.0 * k, 16.0 * k + i as f32 * 22.0 * k, z, INK); } }
        if s.dead.is_none() && !menu.open && !s.inv_open && s.town.talk.is_none() && s.map.anim <= 0.0 { u.minimap(s, w, k); }
        if menu.tips && !menu.open && info.is_empty() && s.dead.is_none() && !s.inv_open && s.town.talk.is_none() && s.map.anim <= 0.0 { u.hint(s, k); }
        if let (Some(v), None) = (s.marked_village(), &s.dead) { let c = [v.c[0] as f64 + 0.5, v.c[2] as f64 + 0.5]; u.pin(s, c, &v.name, [0.55, 0.75, 0.95, 0.9], w, k); }
        // Un villageois sous le réticule : on peut lui parler.
        if s.town.talk.is_none() && !s.inv_open { if let Some(f) = s.aim_folk().and_then(|i| s.town.folk.get(i)) { let t = format!("Clic droit : parler à {}{}", f.name, f.craft.and_then(alw_core::village::craftsman).map_or(String::new(), |c| format!(" ({})", c.name.to_lowercase()))); let z = 17.0 * k; u.text(&t, (w - Ui::text_width(&t, z)) * 0.5, h * 0.5 + 26.0 * k, z, w1); } }
        if s.town.journal && s.town.talk.is_none() && !s.inv_open { u.journal(s, w, h); }
        if s.town.talk.is_some() { u.dialog(s, w, h, mouse); }
        if s.inv_open { u.inventory(s, w, h, mouse); }
        u.big_map(s, w, h, k);
        if let Some(d) = &s.dead { u.death(s, d, w, h, k, mouse); }
        if menu.open { u.menu(menu, w, h, mouse); }
        if let Some(m) = msg { let z = 20.0 * k; u.text(m, (w - Ui::text_width(m, z)) * 0.5, h * 0.16, z, w1); }
        let _ = (u.w, u.h); u
    }
}
