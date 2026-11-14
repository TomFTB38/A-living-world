//! Les structures exportées : un bâtiment retouché à la main dans le monde plat, relevé bloc par bloc (`/coin1`,
//! `/coin2`, `/exporter nom`), devient un fichier texte qui remplace le patron du même nom dans les villages.
//!
//! Format (lisible, une couche par niveau, une ligne par rang de la façade vers le fond) :
//! ```text
//! nom maison
//! taille 11 16 10          largeur (x), hauteur (y), profondeur (z)
//! origine -2 -3 -2         position, dans le repère du patron, de la première case (y = 0 : le plancher)
//! legende 1                largeur des codes (1 ou 2 caractères)
//! . air
//! ~ (terrain laissé tel quel)
//! a stone_bricks
//! b oak_stairs_2
//! couche -3
//! aaaaaaaaaaa
//! ...
//! fin
//! ```
//! Le repère est celui du patron, sans miroir : x le long de la façade, z de la rue vers le fond. Les blocs orientés
//! (escaliers, pentes, portes…) sont notés dans ce repère et tournés à la pose, selon le terrain du village.
use crate::blocks::{by_name, Id, BLOCKS};
use crate::patrons::Kind;
use crate::shapes::{info, variant, Kind as Shape};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Les structures livrées avec le jeu (celles que vous m'avez envoyées).
pub const EMBEDDED: &str = include_str!("../data/structures.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell { /** on ne touche pas au terrain */ Keep, Block(Id) }

#[derive(Clone, Debug)]
pub struct Structure { pub name: String, pub size: [i32; 3], pub origin: [i32; 3], pub cells: Vec<Cell> }

impl Structure {
    fn idx(&self, x: i32, y: i32, z: i32) -> usize { (x + self.size[0] * (z + self.size[2] * y)) as usize }
    /// La case en coordonnées du fichier (0 ≤ x < largeur…).
    pub fn at(&self, x: i32, y: i32, z: i32) -> Cell { self.cells[self.idx(x, y, z)] }
    /// L'emprise au sol dans le repère du patron : [x0, z0, x1, z1].
    pub fn ext(&self) -> [i32; 4] { [self.origin[0], self.origin[2], self.origin[0] + self.size[0] - 1, self.origin[2] + self.size[2] - 1] }
    /// Les cases posées, dans le repère du patron.
    pub fn each(&self) -> impl Iterator<Item = ([i32; 3], Cell)> + '_ {
        let s = self.size; let o = self.origin;
        (0..s[1]).flat_map(move |y| (0..s[2]).flat_map(move |z| (0..s[0]).map(move |x| ([o[0] + x, o[1] + y, o[2] + z], self.at(x, y, z)))))
    }
    /// La porte d'entrée : la plus proche de la rue (x dans le repère du patron).
    pub fn door(&self) -> Option<i32> {
        self.each().filter_map(|(p, c)| match c { Cell::Block(b) if info(b).is_some_and(|i| i.kind == Shape::Door && !i.upper) => Some(p), _ => None }).min_by_key(|p| (p[2], p[1])).map(|p| p[0])
    }
    pub fn to_text(&self) -> String {
        let mut names: Vec<Id> = Vec::new();
        for c in &self.cells { if let Cell::Block(b) = c { if *b != 0 && !names.contains(b) { names.push(*b); } } }
        const POOL: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!$%&*+-=?@^_<>()[]{}|:;,'\"/\\`";
        let pool: Vec<char> = POOL.chars().collect(); let wide = names.len() > pool.len();
        let code = |k: usize| -> String { if wide { format!("{}{}", pool[k / pool.len()], pool[k % pool.len()]) } else { pool[k].to_string() } };
        let (air, keep) = if wide { ("..", "~~") } else { (".", "~") };
        let mut t = format!("# A Living World — structure exportée (/exporter)\nnom {}\ntaille {} {} {}\norigine {} {} {}\nlegende {}\n{air} air\n{keep} (terrain)\n",
            self.name, self.size[0], self.size[1], self.size[2], self.origin[0], self.origin[1], self.origin[2], if wide { 2 } else { 1 });
        for (k, b) in names.iter().enumerate() { t += &format!("{} {}\n", code(k), BLOCKS[*b as usize].name); }
        for y in 0..self.size[1] {
            t += &format!("couche {}\n", self.origin[1] + y);
            for z in 0..self.size[2] { for x in 0..self.size[0] { t += &match self.at(x, y, z) { Cell::Keep => keep.to_string(), Cell::Block(0) => air.to_string(), Cell::Block(b) => code(names.iter().position(|n| *n == b).unwrap()) }; } t.push('\n'); }
        }
        t + "fin\n"
    }
}

/// Lit un ou plusieurs blocs « nom … fin ». Les lignes `#` sont des commentaires.
pub fn parse(text: &str) -> Result<Vec<Structure>, String> {
    let mut out = Vec::new(); let mut lines = text.lines().map(str::trim_end).filter(|l| !l.trim_start().starts_with('#')).peekable();
    while let Some(l) = lines.next() {
        let Some(name) = l.trim().strip_prefix("nom ") else { if l.trim().is_empty() { continue } else { return Err(format!("« nom » attendu, lu « {l} »")) } };
        let name = name.trim().to_string();
        let nums = |l: Option<&str>, key: &str| -> Result<[i32; 3], String> {
            let l = l.ok_or(format!("{name} : « {key} » manque"))?; let v: Vec<i32> = l.trim().strip_prefix(key).ok_or(format!("{name} : « {key} » attendu, lu « {l} »"))?.split_whitespace().filter_map(|x| x.parse().ok()).collect();
            if v.len() < 3 { return Err(format!("{name} : « {key} » demande trois nombres")); } Ok([v[0], v[1], v[2]])
        };
        let size = nums(lines.next(), "taille")?; let origin = nums(lines.next(), "origine")?;
        if size.iter().any(|&s| !(1..=96).contains(&s)) { return Err(format!("{name} : taille hors limites {size:?}")); }
        let w: usize = lines.next().and_then(|l| l.trim().strip_prefix("legende").and_then(|x| x.trim().parse().ok())).ok_or(format!("{name} : « legende 1 » ou « legende 2 » attendu"))?;
        let mut key: HashMap<String, Cell> = HashMap::new();
        while let Some(l) = lines.peek() { if l.trim().starts_with("couche") { break; } let l = lines.next().unwrap(); if l.trim().is_empty() { continue; }
            let (c, n) = (l.chars().take(w).collect::<String>(), l.chars().skip(w).collect::<String>()); let n = n.trim();
            let cell = if n == "air" { Cell::Block(0) } else if n.starts_with("(terrain") { Cell::Keep } else { Cell::Block(by_name(n).ok_or(format!("{name} : bloc inconnu « {n} »"))?) };
            key.insert(c, cell); }
        let mut cells = vec![Cell::Keep; (size[0] * size[1] * size[2]) as usize];
        for y in 0..size[1] {
            let l = lines.next().ok_or(format!("{name} : couche {} manquante", origin[1] + y))?; if !l.trim().starts_with("couche") { return Err(format!("{name} : « couche » attendu, lu « {l} »")); }
            for z in 0..size[2] {
                let row: Vec<char> = lines.next().ok_or(format!("{name} : rang manquant"))?.chars().collect();
                for x in 0..size[0] { let c: String = row.iter().skip(x as usize * w).take(w).collect(); let cell = *key.get(&c).ok_or(format!("{name} : code « {c} » absent de la légende (couche {}, rang {z})", origin[1] + y))?;
                    cells[(x + size[0] * (z + size[2] * y)) as usize] = cell; }
            }
        }
        if lines.next().map(str::trim) != Some("fin") { return Err(format!("{name} : « fin » attendu")); }
        out.push(Structure { name, size, origin, cells });
    }
    Ok(out)
}

// ------------------------------------------------------------------ registre
fn reg() -> &'static RwLock<HashMap<String, Arc<Structure>>> {
    static R: std::sync::OnceLock<RwLock<HashMap<String, Arc<Structure>>>> = std::sync::OnceLock::new();
    R.get_or_init(|| { let mut m = HashMap::new(); if let Ok(v) = parse(EMBEDDED) { for s in v { m.insert(s.name.clone(), Arc::new(s)); } } RwLock::new(m) })
}
/// Ajoute (ou remplace) des structures lues dans un fichier ; rend leurs noms.
pub fn register(text: &str) -> Result<Vec<String>, String> {
    let v = parse(text)?; let mut r = reg().write().unwrap(); let names = v.iter().map(|s| s.name.clone()).collect();
    for s in v { r.insert(s.name.clone(), Arc::new(s)); } Ok(names)
}
pub fn add(s: Structure) { reg().write().unwrap().insert(s.name.clone(), Arc::new(s)); }
pub fn forget(name: &str) -> bool { reg().write().unwrap().remove(name).is_some() }
pub fn names() -> Vec<String> { let mut v: Vec<String> = reg().read().unwrap().keys().cloned().collect(); v.sort(); v }
pub fn get(name: &str) -> Option<Arc<Structure>> { reg().read().unwrap().get(name).cloned() }

/// Le nom de fichier d'un patron.
pub fn key(k: Kind) -> &'static str {
    match k { Kind::Maison => "maison", Kind::Forge => "forge", Kind::Auberge => "auberge", Kind::Moulin => "moulin", Kind::Tanneur => "tanneur", Kind::Eglise => "eglise",
        Kind::GrandeMairie => "grande_mairie", Kind::PetiteMairie => "petite_mairie", Kind::Atelier => "atelier", Kind::Ferme => "ferme", Kind::Etable => "etable", Kind::Ecurie => "ecurie" }
}
pub const KINDS: [Kind; 12] = [Kind::Maison, Kind::Forge, Kind::Auberge, Kind::Moulin, Kind::Tanneur, Kind::Eglise, Kind::GrandeMairie, Kind::PetiteMairie, Kind::Atelier, Kind::Ferme, Kind::Etable, Kind::Ecurie];
/// La structure qui remplace ce patron : « maison_3 » pour une variante précise, sinon « maison » pour toutes.
pub fn find(k: Kind, var: u8) -> Option<Arc<Structure>> { let r = reg().read().unwrap(); if r.is_empty() { return None; } r.get(&format!("{}_{var}", key(k))).or_else(|| r.get(key(k))).cloned() }
/// Le patron que désigne un nom de structure, s'il en désigne un.
pub fn kind_of(name: &str) -> Option<Kind> {
    KINDS.into_iter().find(|k| name == key(*k) || name.strip_prefix(key(*k)).and_then(|r| r.strip_prefix('_')).is_some_and(|v| v.parse::<u8>().is_ok()))
}

// ------------------------------------------------------------------ orientation
/// Une direction au sol, d'après un sens (0 : +z, 1 : +x, 2 : -z, 3 : -x).
fn dir(f: u8) -> (i32, i32) { match f % 4 { 0 => (0, 1), 1 => (1, 0), 2 => (0, -1), _ => (-1, 0) } }
fn fac(d: (i32, i32)) -> u8 { match d { (0, 1) => 0, (1, 0) => 1, (0, -1) => 2, _ => 3 } }
/// Le bloc tourné (et retourné) par la matrice `m` : colonnes = images de +x et de +z.
pub fn orient(b: Id, m: [[i32; 2]; 2]) -> Id {
    let map = |f: u8| { let (x, z) = dir(f); fac((m[0][0] * x + m[1][0] * z, m[0][1] * x + m[1][1] * z)) };
    // une poutre couchée suit son axe
    let n = BLOCKS[b as usize].name; let swap = m[0][0] == 0;
    if swap { if let Some(r) = n.strip_suffix("_x") { if let Some(o) = by_name(&format!("{r}_z")) { return o; } } if let Some(r) = n.strip_suffix("_z") { if let Some(o) = by_name(&format!("{r}_x")) { return o; } } }
    let Some(i) = info(b) else { return b };
    let f = match i.kind {
        // un angle couvre deux directions : f et f + 1
        Shape::Corner | Shape::Valley => { let (a, c) = (map(i.facing), map(i.facing + 1)); if (a + 1) % 4 == c { a } else { c } }
        Shape::Peak => return b,
        _ => map(i.facing),
    };
    variant(i.base, f, i.top, i.open, i.upper)
}
/// L'inverse d'une matrice de rotation et de symétrie (orthogonale : sa transposée).
pub fn inverse(m: [[i32; 2]; 2]) -> [[i32; 2]; 2] { [[m[0][0], m[1][0]], [m[0][1], m[1][1]]] }

// ------------------------------------------------------------------ relevé en jeu
use crate::player::Voxels;
/// Nom de fichier sûr : minuscules, chiffres, soulignés.
pub fn clean(n: &str) -> String { n.chars().filter_map(|c| if c.is_ascii_alphanumeric() { Some(c.to_ascii_lowercase()) } else if c == '_' || c == '-' || c == ' ' { Some('_') } else { None }).take(40).collect() }

impl crate::session::Session {
    /// `/coin1`, `/coin2` : le bloc visé, sinon celui sous les pieds.
    pub fn set_corner(&mut self, k: usize) -> String {
        let p = self.target.map(|t| [t.x, t.y, t.z]).unwrap_or([self.player.p[0].floor() as i32, self.player.p[1].floor() as i32 - 1, self.player.p[2].floor() as i32]);
        self.coins[k] = Some(p);
        let size = match self.coins { [Some(a), Some(b)] => format!(" — boîte de {} × {} × {}", (a[0] - b[0]).abs() + 1, (a[1] - b[1]).abs() + 1, (a[2] - b[2]).abs() + 1), _ => String::new() };
        format!("Coin {} : {} {} {}{size}", k + 1, p[0], p[1], p[2])
    }
    /// Le bâtiment de la galerie sous le joueur (ou sous la boîte), s'il y en a un.
    fn exhibit_here(&self, at: [i32; 2]) -> Option<crate::gen::Exhibit> {
        if !self.gen.flat { return None; }
        self.gen.exhibits().iter().copied().find(|e| { let (a, b) = (e.lot.cell(e.ext[0], 0, e.ext[1]), e.lot.cell(e.ext[2], 0, e.ext[3]));
            (a[0].min(b[0]) - 2..=a[0].max(b[0]) + 2).contains(&at[0]) && (a[2].min(b[2]) - 2..=a[2].max(b[2]) + 2).contains(&at[1]) })
    }
    /// `/exporter [nom]` : relève la boîte des deux coins, ou à défaut le bâtiment de la galerie où l'on se tient.
    pub fn export(&mut self, name: Option<&str>) -> String {
        let me = [self.player.p[0].floor() as i32, self.player.p[2].floor() as i32];
        let center = match self.coins { [Some(a), Some(b)] => [(a[0] + b[0]) / 2, (a[2] + b[2]) / 2], _ => me };
        let ex = self.exhibit_here(center);
        // le repère : celui du patron de la galerie, ou, pour une construction libre, la façade face au joueur
        let (lot, y0, mirror, w) = match ex {
            Some(e) => { let pl = crate::patrons::plan(e.kind, e.var); (e.lot, e.y0, pl.mirror, pl.w) }
            None => { let f = crate::shapes::facing_of(self.look[0], self.look[2]); let d = dir(f); (crate::batisse::Lot { o: (0, 0), u: (d.1, -d.0), w: d }, 0, false, 1) }
        };
        // la boîte, en coordonnées du monde
        let (lo, hi) = match (self.coins, ex) {
            ([Some(a), Some(b)], _) => ([a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])], [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]),
            (_, Some(e)) => { let (a, b) = (e.lot.cell(e.ext[0], 0, e.ext[1]), e.lot.cell(e.ext[2], 0, e.ext[3])); let (x0, x1, z0, z1) = (a[0].min(b[0]), a[0].max(b[0]), a[2].min(b[2]), a[2].max(b[2]));
                let mut top = e.y0; for y in e.y0..e.y0 + 64 { if (x0..=x1).any(|x| (z0..=z1).any(|z| self.world.get(x, y, z).is_some_and(|b| b != 0))) { top = y; } }
                ([x0, e.y0 - 3, z0], [x1, top, z1]) }
            _ => return "Rien à exporter : placez-vous dans un bâtiment de la galerie (monde plat), ou marquez la boîte avec /coin1 et /coin2".into(),
        };
        if (0..3).any(|k| hi[k] - lo[k] + 1 > 96) { return "Boîte trop grande : 96 blocs au plus de chaque côté".into(); }
        let name = match name.map(clean).filter(|n| !n.is_empty()) { Some(n) => n, None => match ex { Some(e) => if e.var == 0 { key(e.kind).to_string() } else { format!("{}_{}", key(e.kind), e.var) }, None => return "Donnez un nom : /exporter mon_batiment (ou maison, forge… pour remplacer un patron)".into() } };
        // du monde au repère du patron
        let local = |x: i32, z: i32| { let (dx, dz) = (x - lot.o.0, z - lot.o.1); let u = dx * lot.u.0 + dz * lot.u.1; let ww = dx * lot.w.0 + dz * lot.w.1; (if mirror { w - 1 - u } else { u }, ww) };
        let s = if mirror { -1 } else { 1 }; let back = inverse([[lot.u.0 * s, lot.u.1 * s], [lot.w.0, lot.w.1]]);
        let (mut lmin, mut lmax) = ([i32::MAX; 2], [i32::MIN; 2]);
        for x in [lo[0], hi[0]] { for z in [lo[2], hi[2]] { let (a, b) = local(x, z); lmin = [lmin[0].min(a), lmin[1].min(b)]; lmax = [lmax[0].max(a), lmax[1].max(b)]; } }
        let base = if ex.is_some() { y0 } else { lo[1] };
        let size = [lmax[0] - lmin[0] + 1, hi[1] - lo[1] + 1, lmax[1] - lmin[1] + 1];
        let mut st = Structure { name: name.clone(), size, origin: [lmin[0], lo[1] - base, lmin[1]], cells: vec![Cell::Keep; (size[0] * size[1] * size[2]) as usize] };
        let natural = |y: i32| { let t = crate::gen::FLAT_Y; if y == t { crate::blocks::B::GRASS } else if y < t && y + 3 >= t { crate::blocks::B::DIRT } else if y < t { crate::blocks::B::STONE } else { 0 } };
        let mut n = 0;
        for y in lo[1]..=hi[1] { for z in lo[2]..=hi[2] { for x in lo[0]..=hi[0] {
            let Some(b) = self.world.get(x, y, z) else { return "Une partie de la boîte n'est pas chargée : rapprochez-vous".into() };
            let (a, c) = local(x, z); let i = st.idx(a - lmin[0], y - lo[1], c - lmin[1]);
            st.cells[i] = if self.gen.flat && y <= crate::gen::FLAT_Y && b == natural(y) { Cell::Keep } else { if b != 0 { n += 1; } Cell::Block(orient(b, back)) };
        } } }
        let text = st.to_text(); let what = match kind_of(&name) { Some(_) => format!("il remplace le patron « {name} » dans les villages bâtis à partir de maintenant"), None => "structure libre : envoyez-moi le fichier pour l'intégrer".into() };
        add(st); self.exports.push((format!("{name}.txt"), text));
        format!("Exporté « {name} » : {} × {} × {}, {n} blocs → structures/{name}.txt ; {what}", size[0], size[1], size[2])
    }
}
impl crate::session::Session {
    /// Les bâtiments de la galerie (monde plat seulement).
    pub fn exhibits(&self) -> Vec<crate::gen::Exhibit> { if self.gen.flat { self.gen.exhibits().clone() } else { Vec::new() } }
}
