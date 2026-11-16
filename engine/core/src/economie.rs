//! L'économie du village pilote : le premier village qui vit pour de bon. Il a des stocks réels, une population qui
//! mange chaque soir, des bâtiments qui produisent et transforment (champs → blé → farine → pain ; bois → planches ;
//! minerai et charbon → lingots), et il bâtit une maison de plus quand il en a les moyens — d'un coup, le jour où les
//! matériaux sont réunis. Rien n'y sort de nulle part : ce qui est dans ses coffres est ce qu'il a produit.
//!
//! La simulation avance d'heure en heure sur l'horloge de la partie, que le joueur soit là ou non : au retour, le
//! village a rattrapé le temps écoulé. Seule la pose d'une maison attend que le joueur soit dans les parages (le monde
//! doit être chargé pour la recevoir).
//!
//! Le pilote est le village médiéval le plus proche du point de départ ; les autres villages restent comme avant.
use crate::batisse::{Lot, Work};
use crate::blocks::{is_chest, is_solid, Id, B};
use crate::bourg::Outlook;
use crate::edifice::Arch;
use crate::patrons::{self, Kind, Look};
use crate::player::Voxels;
use crate::session::{Session, Stack, CHEST_SIZE};
use crate::village::{House, Village};
use std::collections::BTreeMap;

/// Ce que le village compte, dans l'ordre où on le lit.
pub const GOODS: [(&str, &str); 13] = [("wheat", "blé"), ("carrot", "carottes"), ("potato", "pommes de terre"), ("flour", "farine"), ("bread", "pain"), ("cooked_meat", "viande"), ("oak_log", "bois"), ("planks", "planches"), ("cobblestone", "pierre"), ("iron_ore", "minerai"), ("coal", "charbon"), ("iron_ingot", "fer"), ("leather", "cuir")];
/// Au-delà, on ne produit plus : les greniers sont pleins, le village vit sur son acquis.
const FULL: f64 = 320.0;
/// Ce que coûte une maison.
pub const HOUSE_COST: [(&str, f64); 3] = [("planks", 48.0), ("cobblestone", 36.0), ("oak_log", 12.0)];
/// Ceux qui emménagent dans une maison neuve.
const NEWCOMERS: u32 = 2;

/// Un bâtiment que le village peut entreprendre : son patron, son nom, ce qu'il coûte, ce qu'il apporte.
#[derive(Clone, Copy, Debug)] pub struct Project { pub kind: Kind, pub name: &'static str, pub cost: &'static [(&'static str, f64)], pub what: &'static str }
pub const PROJECTS: [Project; 8] = [
    Project { kind: Kind::Maison, name: "Maison", cost: &HOUSE_COST, what: "deux habitants de plus" },
    Project { kind: Kind::Moulin, name: "Moulin", cost: &[("planks", 40.0), ("cobblestone", 50.0), ("oak_log", 16.0), ("iron_ingot", 4.0)], what: "le blé moulu en farine, en grand" },
    Project { kind: Kind::Auberge, name: "Auberge", cost: &[("planks", 70.0), ("cobblestone", 50.0), ("oak_log", 20.0), ("iron_ingot", 6.0)], what: "un four : le pain pour tous" },
    Project { kind: Kind::Ferme, name: "Ferme", cost: &[("planks", 40.0), ("cobblestone", 24.0), ("oak_log", 16.0)], what: "plus de blé" },
    Project { kind: Kind::Etable, name: "Étable", cost: &[("planks", 50.0), ("cobblestone", 20.0), ("oak_log", 20.0)], what: "viande et cuir" },
    Project { kind: Kind::Atelier, name: "Menuiserie", cost: &[("planks", 30.0), ("cobblestone", 20.0), ("oak_log", 30.0), ("iron_ingot", 4.0)], what: "le bois scié en planches, vite" },
    Project { kind: Kind::Forge, name: "Forge", cost: &[("planks", 30.0), ("cobblestone", 60.0), ("oak_log", 10.0), ("iron_ingot", 10.0)], what: "le minerai fondu en fer" },
    Project { kind: Kind::Tanneur, name: "Tannerie", cost: &[("planks", 40.0), ("cobblestone", 30.0), ("oak_log", 12.0)], what: "les peaux tannées en cuir" },
];
pub fn project(k: Kind) -> &'static Project { PROJECTS.iter().find(|p| p.kind == k).unwrap_or(&PROJECTS[0]) }
fn kind_index(k: Kind) -> usize { PROJECTS.iter().position(|p| p.kind == k).unwrap_or(0) }
/// Un bâtiment que le village a bâti depuis la création du monde : son terrain, son niveau, son patron.
#[derive(Clone, Copy, Debug)] pub struct Extra { pub lot: Lot, pub y: i32, pub kind: Kind }
/// Le recensement : qui vit là, et ce qui tourne.
#[derive(Clone, Copy, Debug, Default, PartialEq)] pub struct Census { pub pop: u32, pub fields: u32, pub farms: u32, pub folds: u32, pub mills: u32, pub inns: u32, pub forges: u32, pub woodshops: u32, pub tanneries: u32, pub cutters: u32, pub miners: u32, /** combien de maisons le village veut encore bâtir, selon son tempérament */ pub room: u32 }

#[derive(Clone, Debug)]
pub struct Econ {
    pub id: (i32, i32), pub stock: BTreeMap<&'static str, f64>,
    /// Jusqu'où la simulation est allée, en heures de jeu depuis le début de la partie.
    pub t: f64,
    /// Part de la population nourrie au dernier repas ; jours de disette d'affilée.
    pub fed: f64, pub lean: u32,
    pub extra: Vec<Extra>,
    /// Les bâtiments payés, qui attendent d'être posés (le joueur est loin).
    pub due: Vec<Kind>,
    /// Le projet du moment : quel bâtiment, et s'il a été choisi par le joueur (chez le maire) plutôt que par le village.
    pub project: Option<(Kind, bool)>,
    /// Le journal du village : (jour, ce qui s'est passé), les plus récents à la fin.
    pub log: Vec<(u32, String)>,
    /// Les coffres de la réserve (quand le village est chargé), et si le joueur y a la main.
    pub store: Vec<[i32; 3]>, open: bool, held: f64,
    /// Village chargé : les champs, le bois et la pierre viennent des gestes de ses gens (moisson, abattage, taille), plus du calcul.
    pub live: [bool; 3],
    /// Les plants replantés par les bûcherons : où, et quand (heures de jeu).
    pub saplings: Vec<([i32; 3], f64)>,
    /// Ce que les gens de métier savent trouver autour du village : pieds d'arbres, cultures, roche à nu ; quand on a regardé.
    trees: Vec<[i32; 3]>, crops: Vec<[i32; 3]>, rocks: Vec<[i32; 3]>, scan_t: f64,
}

/// Habitants d'un village tel qu'il a été tracé (la même règle que pour les faire apparaître).
pub fn people(v: &Village) -> u32 { v.houses.iter().enumerate().map(|(i, h)| if h.craft.is_some() { 1 } else { 1 + (crate::noise::hash_float(v.id.0, v.id.1, i as i32, 91) < 0.6) as u32 }).sum() }

impl Econ {
    /// Un village à sa première heure : il a déjà quelques jours de réserves, selon sa richesse.
    pub fn found(v: &Village, t: f64) -> Econ {
        let pop = people(v) as f64; let wealth = v.ident.map_or(0.5, |i| i.wealth); let mut stock = BTreeMap::new();
        for (k, n) in [("wheat", pop * 3.0), ("flour", pop), ("bread", pop * 2.5), ("cooked_meat", pop * 0.5), ("oak_log", 16.0 + 20.0 * wealth), ("planks", 10.0 + 24.0 * wealth), ("cobblestone", 8.0 + 20.0 * wealth), ("iron_ore", 4.0), ("coal", 6.0), ("iron_ingot", 2.0 + 4.0 * wealth), ("leather", 2.0)] { stock.insert(k, n.floor()); }
        Econ { id: v.id, stock, t, fed: 1.0, lean: 0, extra: Vec::new(), due: Vec::new(), project: None, log: Vec::new(), store: Vec::new(), open: false, held: 0.0, live: [false; 3], saplings: Vec::new(), trees: Vec::new(), crops: Vec::new(), rocks: Vec::new(), scan_t: -1.0 }
    }
    /// Oublie ce qu'on sait des alentours : on regardera de nouveau à la prochaine heure de travail.
    pub fn scan_reset(&mut self) { self.scan_t = -1.0; }
    pub fn have(&self, k: &str) -> f64 { self.stock.get(k).copied().unwrap_or(0.0) }
    fn add(&mut self, k: &'static str, n: f64) { let e = self.stock.entry(k).or_insert(0.0); *e = (*e + n).max(0.0); }
    /// Prend jusqu'à `n` ; rend ce qui a été pris.
    fn take(&mut self, k: &'static str, n: f64) -> f64 { let h = self.have(k).min(n).max(0.0); self.add(k, -h); h }
    fn note(&mut self, day: u32, s: String) { self.log.push((day, s)); if self.log.len() > 12 { self.log.remove(0); } }
    /// Le recensement du village : ce qu'il a été tracé, plus les maisons bâties depuis.
    pub fn census(&self, v: &Village) -> Census {
        let n = |f: &dyn Fn(Arch) -> bool| v.works.iter().filter(|w| f(w.0)).count() as u32;
        let built = |k: Kind| self.extra.iter().filter(|x| x.kind == k).count() as u32; let mut c = Census { pop: people(v) + built(Kind::Maison) * NEWCOMERS, fields: v.fields, farms: n(&|a| matches!(a, Arch::Farm | Arch::Barn)), folds: n(&|a| matches!(a, Arch::Fold | Arch::Stable)), mills: n(&|a| a == Arch::Mill), inns: n(&|a| a == Arch::Inn),
            forges: n(&|a| a == Arch::Forge), woodshops: n(&|a| matches!(a, Arch::Woodshop | Arch::Workshop)), tanneries: n(&|a| a == Arch::Tannery), ..Default::default() };
        c.farms += built(Kind::Ferme); c.folds += built(Kind::Etable) + built(Kind::Ecurie); c.mills += built(Kind::Moulin); c.inns += built(Kind::Auberge); c.forges += built(Kind::Forge); c.woodshops += built(Kind::Atelier); c.tanneries += built(Kind::Tanneur);
        // ceux qui ne tiennent ni champ ni atelier vont au bois et à la carrière
        let busy = c.fields.max(c.farms) + c.folds + c.mills + c.inns + c.forges + c.woodshops + c.tanneries; let free = c.pop.saturating_sub(busy).max(2);
        c.cutters = (free as f64 * 0.45).ceil() as u32; c.miners = (free as f64 * 0.35).ceil() as u32;
        // un village tient à son tempérament : celui qui veut rester ce qu'il est ne s'agrandit pas. Le pilote, lui, bâtit au moins deux maisons, pour qu'on le voie faire.
        let want = match v.ident.map(|i| i.outlook) { Some(Outlook::Expansion) => 4, Some(Outlook::Prosperity) => 3, Some(Outlook::Trade | Outlook::SelfSufficiency) => 2, Some(Outlook::Defense) => 1, _ => 0u32 }.max(2);
        c.room = want.saturating_sub(built(Kind::Maison) + self.due.iter().filter(|k| **k == Kind::Maison).count() as u32); c
    }
    /// Les jours de vivres devant soi.
    pub fn reserve(&self, pop: u32) -> f64 { (self.have("bread") + self.have("cooked_meat") + self.have("flour") * 0.8 + self.have("wheat") * 0.5 + self.have("potato") * 0.5 + self.have("carrot") * 0.4) / pop.max(1) as f64 }
    /// Une heure de la vie du village. `hod` : l'heure du jour (0 à 23) ; `day` : le numéro du jour.
    pub fn step(&mut self, c: &Census, hod: u32, day: u32) {
        let work = (7..12).contains(&hod) || (13..18).contains(&hod);
        if work {
            let room = |e: &Econ, k: &str| e.have(k) < FULL; let pop = c.pop as f64;
            // la disette presse les champs : on y met tous les bras
            let press = if self.reserve(c.pop) < 1.0 { 1.6 } else { 1.0 };
            if room(self, "wheat") && !self.live[0] { self.add("wheat", (c.fields.max(1) as f64 * 0.2 + c.farms as f64 * 0.25) * press); }
            if room(self, "cooked_meat") { self.add("cooked_meat", c.folds as f64 * 0.12); } if room(self, "leather") { self.add("leather", c.tanneries.min(c.folds.max(1)) as f64 * 0.03); }
            // le moulin, ou à défaut la meule à bras de chaque foyer ; le four de l'auberge, ou à défaut celui des maisons
            if room(self, "flour") { let x = self.take("wheat", c.mills as f64 * 3.0 + pop * 0.12); self.add("flour", x); }
            if room(self, "bread") { let x = self.take("flour", c.inns as f64 * 2.0 + pop * 0.12); self.add("bread", x); }
            if room(self, "oak_log") && !self.live[1] { self.add("oak_log", c.cutters as f64 * 0.5); }
            if room(self, "planks") && self.have("oak_log") > 14.0 { let x = self.take("oak_log", (c.woodshops as f64 * 0.8 + 0.15).min(self.have("oak_log") - 14.0)); self.add("planks", x * 2.0); }
            if room(self, "cobblestone") && !self.live[2] { self.add("cobblestone", c.miners as f64 * 0.4); } if room(self, "iron_ore") { self.add("iron_ore", c.miners as f64 * 0.04); } if room(self, "coal") { self.add("coal", c.miners as f64 * 0.05); }
            if room(self, "iron_ingot") { let n = (c.forges as f64 * 0.15).min(self.have("iron_ore")).min(self.have("coal")); self.take("iron_ore", n); self.take("coal", n); self.add("iron_ingot", n); }
        }
        // le chantier se décide le matin : les greniers sont pleins pour quatre jours, personne n'a eu faim, et les matériaux y sont
        // (un projet choisi chez le maire n'attend ni le matin ni les vivres : dès que la réserve a de quoi, on bâtit)
        if self.project.is_none() && hod == 8 { self.project = self.auto_project(c).map(|k| (k, false)); }
        if let Some((k, mine)) = self.project { if (mine || (hod == 8 && self.fed >= 0.95 && self.reserve(c.pop) >= 4.0)) && self.affords(k) { self.start(k, day); } }
        // le repas du soir : pain d'abord, puis viande, farine en bouillie, blé en grains ; le feu de la veillée
        if hod == 19 {
            let need = c.pop as f64; let mut left = need;
            for (k, worth) in [("bread", 1.0), ("cooked_meat", 1.0), ("potato", 0.5), ("carrot", 0.4), ("flour", 0.8), ("wheat", 0.5)] { if left <= 0.0 { break; } let got = self.take(k, left / worth); left -= got * worth; }
            let was = self.fed; self.fed = ((need - left.max(0.0)) / need.max(1.0)).clamp(0.0, 1.0); self.take("oak_log", need * 0.05);
            if self.fed < 0.6 { self.lean += 1; if was >= 0.6 { self.note(day, "le grenier est vide : on se couche le ventre creux".into()); } } else { if self.lean > 0 { self.note(day, "on mange de nouveau à sa faim".into()); } self.lean = 0; }
        }
    }
    /// La réserve a-t-elle de quoi bâtir ce bâtiment ?
    pub fn affords(&self, k: Kind) -> bool { project(k).cost.iter().all(|(id, n)| self.have(id) >= *n) }
    /// On paie le chantier : il sera posé dès que le joueur sera là.
    fn start(&mut self, k: Kind, day: u32) { let p = project(k); for (id, n) in p.cost { self.take(id, *n); } self.due.push(k); self.project = None; self.note(day, format!("les matériaux sont réunis : on bâtit {}", article(p.name))); }
    /// Ce que le village bâtirait de lui-même : un moulin quand le blé s'entasse, une auberge quand la farine attend, sinon une maison s'il veut s'agrandir.
    pub fn auto_project(&self, c: &Census) -> Option<Kind> {
        if c.mills == 0 && self.have("wheat") >= 40.0 { Some(Kind::Moulin) } else if c.inns == 0 && self.have("flour") >= 20.0 { Some(Kind::Auberge) } else if c.room > 0 { Some(Kind::Maison) } else { None }
    }
    /// Ce que le maire peut proposer : une maison toujours (jusqu'à six), les bâtiments de métier qui manquent ou qu'on peut doubler.
    pub fn options(&self, c: &Census) -> Vec<Kind> {
        let houses = self.extra.iter().filter(|x| x.kind == Kind::Maison).count() + self.due.iter().filter(|k| **k == Kind::Maison).count(); let pending = |k: Kind| self.due.contains(&k) as u32;
        PROJECTS.iter().map(|p| p.kind).filter(|k| match k { Kind::Maison => houses < 6, Kind::Moulin => c.mills + pending(*k) == 0, Kind::Auberge => c.inns + pending(*k) == 0, Kind::Tanneur => c.tanneries + pending(*k) == 0,
            Kind::Forge => c.forges + pending(*k) < 2, Kind::Atelier => c.woodshops + pending(*k) < 2, Kind::Etable => c.folds + pending(*k) < 2, Kind::Ferme => c.farms + pending(*k) < 2, _ => false }).collect()
    }
    /// Ce qui manque encore à la réserve pour ce bâtiment (vide : tout y est).
    pub fn missing(&self, k: Kind) -> Vec<(&'static str, f64)> { project(k).cost.iter().filter(|(id, n)| self.have(id) < *n).map(|(id, n)| (*id, n - self.have(id).floor())).collect() }
    /// Une ligne par sujet, pour la commande `/village` et le journal.
    pub fn report(&self, v: &Village, day: u32) -> Vec<String> {
        let c = self.census(v); let mut o = vec![format!("{} — {} habitants, {} maison{} bâtie{} depuis votre arrivée", v.name, c.pop, self.extra.len(), if self.extra.len() > 1 { "s" } else { "" }, if self.extra.len() > 1 { "s" } else { "" })];
        o.push(format!("Stocks : {}", GOODS.iter().filter(|g| self.have(g.0) >= 1.0).map(|g| format!("{} {}", g.1, self.have(g.0).floor())).collect::<Vec<_>>().join(", ")));
        o.push(format!("Vivres pour {:.1} jours ; hier soir {} % de la population a mangé{}", self.reserve(c.pop), (self.fed * 100.0).round(), if self.lean > 0 { format!(" (disette depuis {} jour{})", self.lean, if self.lean > 1 { "s" } else { "" }) } else { String::new() }));
        o.push(format!("Au travail : {} champ{}, {} au bois, {} à la carrière, moulin {}, four {}, forge {}", c.fields, if c.fields > 1 { "s" } else { "" }, c.cutters, c.miners, c.mills, c.inns, c.forges));
        o.push(if let Some(k) = self.due.first() { format!("Chantier : {} est payé{}, il sera bâti dès que le village vous verra revenir", article(project(*k).name), if project(*k).name.ends_with('e') && *k != Kind::Moulin { "e" } else { "" }) } else if let Some((k, mine)) = self.project { format!("Projet{} : {} — {}", if mine { " (choisi par vous)" } else { "" }, project(k).name, project(k).cost.iter().map(|(id, n)| format!("{} {}/{}", GOODS.iter().find(|g| g.0 == *id).map_or(*id, |g| g.1), self.have(id).floor().min(*n), n)).collect::<Vec<_>>().join(", ")) } else if c.room == 0 { "Chantier : le village ne souhaite pas s'agrandir davantage".into() } else { format!("Prochaine maison : {}", HOUSE_COST.iter().map(|(k, n)| format!("{} {}/{}", GOODS.iter().find(|g| g.0 == *k).map_or(*k, |g| g.1), self.have(k).floor().min(*n), n)).collect::<Vec<_>>().join(", ")) + if self.reserve(c.pop) < 4.0 { " — et quatre jours de vivres d'avance" } else { "" } });
        for (d, s) in self.log.iter().rev().take(3) { o.push(format!("Il y a {} jour{} : {s}", day.saturating_sub(*d), if day.saturating_sub(*d) > 1 { "s" } else { "" })); }
        o
    }
    pub fn save(&self) -> String {
        let mut s = format!("econ {} {} {} {} {} 0", self.id.0, self.id.1, self.t, self.fed, self.lean); for (k, n) in &self.stock { s += &format!(" {k}:{n}"); } s.push('\n');
        for e in &self.extra { s += &format!("econ_house {} {} {} {} {} {} {} {}\n", e.lot.o.0, e.lot.o.1, e.lot.u.0, e.lot.u.1, e.lot.w.0, e.lot.w.1, e.y, kind_index(e.kind)); }
        for k in &self.due { s += &format!("econ_due {}\n", kind_index(*k)); } if let Some((k, m)) = self.project { s += &format!("econ_project {} {}\n", kind_index(k), m as u8); }
        for p in &self.saplings { s += &format!("econ_plant {} {} {} {}\n", p.0[0], p.0[1], p.0[2], p.1); }
        for (d, t) in &self.log { s += &format!("econ_log {d} {t}\n"); } s
    }
}

/// La maison posée sur un terrain : ses blocs, et où l'on y vit.
fn raise(lot: Lot, y: i32, kind: Kind) -> (Vec<(i32, i32, i32, Id)>, House) {
    let mut k = Work::default(); let pl = patrons::plan(kind, 0); let made = patrons::build(&mut k, lot, y, kind, 0, Look { roof: "roof_tiles", infill: "plaster" }, 0.4);
    let fy = (y + 1) as f64; let mid = |u: i32, w: i32| { let c = lot.cell(u, 0, w); [c[0] as f64 + 0.5, fy, c[2] as f64 + 0.5] };
    let du = made.door.unwrap_or(pl.door); let (door, inside) = (mid(du, -1), mid(du, 1)); let bed = made.bed.map_or(inside, |b| mid(b.0, b.1));
    let (mut x0, mut z0, mut x1, mut z1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN); for b in &k.blocks { x0 = x0.min(b.0); x1 = x1.max(b.0); z0 = z0.min(b.2); z1 = z1.max(b.2); }
    (k.blocks, House { craft: None, area: [x0, z0, x1, z1], door, inside, spot: inside, bed, ring: door, back: crate::shapes::facing_of(lot.w.0 as f64, lot.w.1 as f64), way: vec![door] })
}

impl Session {
    /// Le village pilote : le village médiéval le plus proche du point de départ.
    fn pilot(&self) -> Option<Village> { self.gen.villages_near(self.spawn[0], self.spawn[2], 1400.0).into_iter().find(|v| v.ident.is_some() && !v.works.is_empty()) }
    fn pilot_village(&self) -> Option<Village> { let id = self.econ.as_ref()?.id; self.gen.site(id.0, id.1).map(|s| s.v.clone()) }
    /// Les maisons bâties depuis la création du monde s'ajoutent au village tracé (leurs habitants avec).
    pub(crate) fn econ_extend(&self, v: &mut Village) { if let Some(e) = self.econ.as_ref().filter(|e| e.id == v.id) { for x in e.extra.iter().filter(|x| x.kind == Kind::Maison) { v.houses.push(raise(x.lot, x.y, x.kind).1); } } }
    /// Le temps passe au village pilote : une heure de jeu à la fois, en rattrapant ce qui s'est écoulé loin de lui.
    pub(crate) fn econ_tick(&mut self, dt: f64) {
        if self.econ.is_none() { let Some(v) = self.pilot() else { return }; self.econ = Some(Econ::found(&v, self.hours.floor())); }
        let Some(v) = self.pilot_village() else { return }; let mut e = self.econ.take().unwrap();
        if self.hours - e.t > 24.0 * 60.0 { e.t = self.hours - 24.0 * 60.0; } // au-delà de deux mois d'absence, on ne rejoue pas tout
        let mut said = Vec::new(); let before = e.log.len();
        while e.t + 1.0 <= self.hours { e.t += 1.0; let hod = (self.hour - (self.hours - e.t)).rem_euclid(24.0).floor() as u32; let c = e.census(&v); e.step(&c, hod, (e.t / 24.0) as u32); }
        if e.log.len() > before || (e.log.len() == 12 && before == 12) { if let Some(l) = e.log.last() { said.push(format!("{} : {}", v.name, l.1)); } }
        let near = self.town.here.iter().any(|h| h.id == v.id);
        // la réserve : on repère ses coffres une fois le village chargé ; quand le joueur referme, on recompte
        if near && e.store.is_empty() { e.store = self.econ_chests(&v); }
        let at_store = self.inv_open && [self.chest, self.chest2].into_iter().flatten().any(|c| e.store.contains(&c));
        if e.open && !at_store { self.econ_count(&mut e, &v); } e.open = at_store;
        if near && std::env::var_os("ALW_NO_CHORES").is_none() { self.econ_chores(&mut e, &v, dt); } else { e.live = [false; 3]; } /* (diagnostic provisoire) */
        // la maison payée se pose dès que le joueur est là
        // un projet choisi chez le maire se lance dès que la réserve a de quoi (le joueur vient peut-être de la remplir)
        if let Some((k, true)) = e.project { if e.affords(k) { let day = (e.t / 24.0) as u32; e.start(k, day); said.push(format!("{} : la réserve est pleine, le chantier {} commence", v.name, de(project(k).name))); } }
        if near && !e.due.is_empty() { let kind = e.due[0]; if let Some((lot, y)) = self.econ_lot(&v, &e, kind) { let (blocks, house) = raise(lot, y, kind);
                // la place dégagée : buissons, herbes et branches basses au-dessus du sol, puis le bâtiment, puis ses fondations
                let mut list: Vec<(i32, i32, i32, Id)> = Vec::new(); for dx in house.area[0]..=house.area[2] { for dz in house.area[1]..=house.area[3] { for yy in y + 1..y + 16 { if self.world.get(dx, yy, dz).is_some_and(|b| b != 0) { list.push((dx, yy, dz, 0)); } } } } list.extend(blocks); for dx in house.area[0]..=house.area[2] { for dz in house.area[1]..=house.area[3] { for yy in (y - 6..=y).rev() { if self.world.get(dx, yy, dz).is_some_and(is_solid) { break; } list.push((dx, yy, dz, B::COBBLESTONE)); } } }
                self.world.edit_many(&list); e.due.remove(0); e.extra.push(Extra { lot, y, kind }); let day = (e.t / 24.0) as u32; let p = project(kind);
                if kind == Kind::Maison { e.note(day, "une maison neuve est debout, deux habitants s'y installent".into()); said.push(format!("{} a bâti une nouvelle maison", v.name)); }
                else { e.note(day, format!("{} est debout : {}", article(p.name), p.what)); said.push(format!("{} a bâti {}", v.name, article(&p.name.to_lowercase()))); }
                // le village chargé apprend sa nouvelle maison, et ses habitants paraissent
                if let Some(i) = self.town.here.iter().position(|h| h.id == v.id).filter(|_| kind == Kind::Maison) { let mut only = self.town.here[i].clone(); let n = only.houses.len(); self.town.here[i].houses.push(house.clone()); only.houses = vec![house]; let base = self.town.folk.len(); self.town.spawn(&only, self.hour); for f in &mut self.town.folk[base..] { f.house = n; } }
                self.dirty = true; } }
        self.econ = Some(e); for s in said { self.notes.push(s); }
    }
    /// Les coffres de la réserve : ceux de la mairie, sinon de l'auberge, sinon d'une grange, sinon de la première maison qui en a.
    fn econ_chests(&self, v: &Village) -> Vec<[i32; 3]> {
        let scan = |a: [i32; 4]| { let mut o = Vec::new(); for x in a[0]..=a[2] { for z in a[1]..=a[3] { for y in v.c[1] - 3..=v.c[1] + 9 { if self.world.get(x, y, z).is_some_and(is_chest) { o.push([x, y, z]); } } } } o };
        for pick in [&[Arch::TownHall, Arch::Hall][..], &[Arch::Inn], &[Arch::Barn, Arch::Farm]] { for w in v.works.iter().filter(|w| pick.contains(&w.0)) { let c = scan(w.1); if !c.is_empty() { return c; } } }
        v.works.iter().map(|w| scan(w.1)).find(|c| !c.is_empty()).unwrap_or_default()
    }
    /// On ouvre un coffre de la réserve : il montre ce que le village possède, à l'unité près.
    pub(crate) fn econ_fill(&mut self, p: [i32; 3]) {
        let Some(e) = self.econ.as_mut().filter(|e| e.store.contains(&p)) else { return }; if e.open { return; } e.open = true;
        // les objets qu'on ne compte pas (une épée déposée là) restent où ils sont ; le reste est rangé à neuf
        let mut slots: Vec<([i32; 3], usize)> = Vec::new();
        for c in e.store.clone() { let ch = self.chests.entry(c).or_insert_with(|| vec![None; CHEST_SIZE]); for (i, s) in ch.iter_mut().enumerate() { if s.is_some_and(|s| s.n.is_some()) { *s = None; } if s.is_none() { slots.push((c, i)); } } }
        let mut k = 0; e.held = 0.0;
        for (id, _) in GOODS { let mut n = e.have(id).floor() as u32; e.held += n as f64; while n > 0 && k < slots.len() { let take = n.min(crate::session::STACK); let (c, i) = slots[k]; if let (Some(ch), Some(it)) = (self.chests.get_mut(&c), crate::session::item(id)) { ch[i] = Some(Stack { item: it.id, n: Some(take), wear: 0 }); } n -= take; k += 1; } }
    }
    /// Le joueur referme : ce qu'il a pris manque au village, ce qu'il a laissé lui profite.
    fn econ_count(&mut self, e: &mut Econ, v: &Village) {
        let mut sum: BTreeMap<&'static str, f64> = BTreeMap::new(); for c in &e.store { if let Some(ch) = self.chests.get(c) { for s in ch.iter().flatten() { if let Some(n) = s.n { *sum.entry(s.item).or_insert(0.0) += n as f64; } } } }
        let total: f64 = sum.values().sum(); for (id, _) in GOODS { let frac = e.have(id).fract(); e.stock.insert(id, sum.get(id).copied().unwrap_or(0.0) + frac); }
        for (k, n) in &sum { if !GOODS.iter().any(|g| g.0 == *k) { e.stock.insert(k, *n); } }
        // on se sert dans la réserve commune : le village le remarque ; on y dépose : il s'en souvient aussi
        let delta = total - e.held; if delta <= -6.0 { *self.town.rep.entry(v.id).or_insert(0) -= 1; self.notes.push(format!("{} a vu sa réserve baisser : on vous regarde de travers", v.name)); } else if delta >= 12.0 { *self.town.rep.entry(v.id).or_insert(0) += 1; self.notes.push(format!("{} vous remercie pour ce que vous laissez à la réserve", v.name)); }
        self.dirty = true;
    }
    /// Un terrain libre pour une maison : près des autres, à plat, hors des rues et des champs, la porte vers la place.
    fn econ_lot(&self, v: &Village, e: &Econ, kind: Kind) -> Option<(Lot, i32)> {
        let pl = patrons::plan(kind, 0); let taken: Vec<[i32; 4]> = v.works.iter().map(|w| w.1).chain(e.extra.iter().map(|x| raise(x.lot, x.y, x.kind).1.area)).collect();
        let hits = |a: [i32; 4]| taken.iter().any(|t| a[0] <= t[2] + 3 && a[2] >= t[0] - 3 && a[1] <= t[3] + 3 && a[3] >= t[1] - 3);
        let big = kind != Kind::Maison; // un grand bâtiment cherche plus loin, et le terrain sera aplani sous lui
        for ring in 0..if big { 14 } else { 7 } { let r = v.r as f64 * 0.55 + ring as f64 * 4.0 + pl.w.max(pl.d) as f64 * 0.3; for k in 0..20 {
            let a = (k as f64 + crate::noise::hash_float(v.id.0, v.id.1, ring * 31 + e.extra.len() as i32, 301)) / 20.0 * std::f64::consts::TAU; let (cx, cz) = (v.c[0] + (a.cos() * r) as i32, v.c[2] + (a.sin() * r) as i32);
            // la façade regarde le centre du village
            let (dx, dz) = (v.c[0] - cx, v.c[2] - cz); let w = if dx.abs() > dz.abs() { (-dx.signum(), 0) } else { (0, -dz.signum()) }; let u = (-w.1, w.0);
            let lot = Lot { o: (cx - u.0 * pl.w / 2 - w.0 * pl.d / 2, cz - u.1 * pl.w / 2 - w.1 * pl.d / 2), u, w };
            let (c0, c1) = (lot.cell(pl.ext[0] - 1, 0, pl.ext[1] - 2), lot.cell(pl.ext[2] + 1, 0, pl.ext[3] + 1)); let area = [c0[0].min(c1[0]), c0[2].min(c1[2]), c0[0].max(c1[0]), c0[2].max(c1[2])];
            if hits(area) { continue; }
            // à plat, au sec, hors des chemins et des cultures ; et le monde est bien chargé là
            let (mut lo, mut hi, mut ok) = (i32::MAX, i32::MIN, true);
            'scan: for x in area[0]..=area[2] { for z in area[1]..=area[3] { let col = self.gen.column(x as f64, z as f64); let g = col.h.floor() as i32;
                // le vrai sol : le plus haut bloc plein qui n'est ni feuillage, ni tronc, ni herbe
                let ground = |b: Id| is_solid(b) && !crate::blocks::is_leaf(b) && !crate::blocks::is_log(b) && crate::blocks::BLOCKS[b as usize].shape != crate::blocks::Shape::Deco;
                let h = (g - 3..=g + 4).rev().find(|&y| self.world.get(x, y, z).is_some_and(ground)).unwrap_or(g); lo = lo.min(h); hi = hi.max(h);
                if col.wet || hi - lo > if big { 4 } else { 2 } { ok = false; break 'scan; } let top = self.world.get(x, h, z); let above = self.world.get(x, h + 1, z);
                let tree = big && above.is_some_and(crate::blocks::is_log); let brush = |b: Id| big && (crate::blocks::is_leaf(b) || crate::blocks::BLOCKS[b as usize].shape == crate::blocks::Shape::Deco) && !crate::blocks::BLOCKS[b as usize].name.starts_with("wheat");
                if top.is_none() || tree || top.is_some_and(|b| matches!(crate::blocks::BLOCKS[b as usize].name, "path" | "farmland" | "gravel" | "cobblestone")) || above.is_some_and(|b| (is_solid(b) && !brush(b)) || crate::blocks::BLOCKS[b as usize].name.starts_with("wheat")) { ok = false; break 'scan; } } }
            if ok { return Some((lot, hi)); }
        } }
        None
    }
    /// Le centre du village pilote (pour s'y rendre).
    pub fn pilot_center(&self) -> Option<[i32; 3]> { self.pilot_village().map(|v| v.c) }
    /// `/village` : l'état du village pilote.
    pub fn village_report(&self) -> String { match (self.econ.as_ref(), self.pilot_village()) { (Some(e), Some(v)) => e.report(&v, (e.t / 24.0) as u32).join("\n"), _ => "Aucun village pilote dans les parages du point de départ".into() } }
    pub(crate) fn econ_save(&self) -> String { self.econ.as_ref().map_or(String::new(), |e| e.save()) }
    pub(crate) fn econ_load(&mut self, key: &str, a: &[&str]) {
        let n = |i: usize| a.get(i).and_then(|v| v.parse::<i32>().ok()); let f = |i: usize| a.get(i).and_then(|v| v.parse::<f64>().ok());
        match key {
            "econ" => { let (Some(id), Some(t)) = (n(0).zip(n(1)), f(2)) else { return }; let mut stock = BTreeMap::new();
                for kv in a.iter().skip(6) { if let Some((k, v)) = kv.split_once(':') { if let (Some(it), Ok(v)) = (crate::session::item(k), v.parse::<f64>()) { stock.insert(it.id, v); } } }
                self.econ = Some(Econ { id, stock, t, fed: f(3).unwrap_or(1.0), lean: n(4).unwrap_or(0) as u32, extra: Vec::new(), due: vec![Kind::Maison; n(5).unwrap_or(0).max(0) as usize], project: None, log: Vec::new(), store: Vec::new(), open: false, held: 0.0, live: [false; 3], saplings: Vec::new(), trees: Vec::new(), crops: Vec::new(), rocks: Vec::new(), scan_t: -1.0 }); }
            "econ_house" => { if let (Some(e), Some(v)) = (self.econ.as_mut(), (0..7).map(n).collect::<Option<Vec<i32>>>()) { e.extra.push(Extra { lot: Lot { o: (v[0], v[1]), u: (v[2], v[3]), w: (v[4], v[5]) }, y: v[6], kind: n(7).and_then(|i| PROJECTS.get(i as usize)).map_or(Kind::Maison, |p| p.kind) }); } }
            "econ_due" => { if let (Some(e), Some(i)) = (self.econ.as_mut(), n(0)) { e.due.push(PROJECTS.get(i as usize).map_or(Kind::Maison, |p| p.kind)); } }
            "econ_project" => { if let (Some(e), Some(i)) = (self.econ.as_mut(), n(0)) { e.project = Some((PROJECTS.get(i as usize).map_or(Kind::Maison, |p| p.kind), n(1) == Some(1))); } }
            "econ_plant" => { if let (Some(e), Some(x), Some(y), Some(z), Some(t)) = (self.econ.as_mut(), n(0), n(1), n(2), f(3)) { e.saplings.push(([x, y, z], t)); } }
            "econ_log" => { if let (Some(e), Some(d)) = (self.econ.as_mut(), n(0)) { e.log.push((d as u32, a[1..].join(" "))); } }
            _ => {}
        }
    }
}

/// Durées d'un geste de métier (secondes) : quelques coups de hache pour un arbre, un coup de faucille par plant mûr,
/// quelques coups de pioche par bloc, le temps de poser son fardeau.
const CHOP: f64 = 22.0; const REAP: f64 = 3.0; const DIG: f64 = 6.0; const DROP: f64 = 2.0;
/// Combien on rapporte à la fois : six gerbes, cinq pierres ; un arbre entier pour le bûcheron.
const LOAD_CROPS: u32 = 6; const LOAD_STONE: u32 = 5;
/// Un plant devient jeune chêne au bout de trois jours.
const SAPLING_HOURS: f64 = 72.0;

fn is_stone_face(n: &str) -> bool { matches!(n, "stone" | "cobblestone" | "granite" | "limestone" | "andesite" | "diorite" | "mossy_stone" | "coal_ore" | "iron_ore" | "copper_ore" | "tin_ore") }

impl Session {
    /// Les gens de métier du village pilote, quand il est chargé : le fermier moissonne les cultures mûres et ressème, le bûcheron
    /// abat un arbre et replante, le mineur taille la roche à nu ; chacun rapporte son ouvrage à la réserve, où il compte.
    pub(crate) fn econ_chores(&mut self, e: &mut Econ, v: &Village, dt: f64) {
        let c = e.census(v); let hod = self.hour; let work = (7.5..12.0).contains(&hod) || (13.0..18.0).contains(&hod);
        // ce qu'il y a à faire autour : rafraîchi toutes les deux minutes
        if e.scan_t < 0.0 || self.elapsed - e.scan_t > 120.0 { e.scan_t = self.elapsed; self.econ_scan(e, v); }
        e.live = [!e.crops.is_empty(), !e.trees.is_empty(), !e.rocks.is_empty()];
        // les plants qui ont eu le temps deviennent de jeunes chênes
        let now = self.hours; let grown: Vec<[i32; 3]> = e.saplings.iter().filter(|s| now - s.1 >= SAPLING_HOURS).map(|s| s.0).collect();
        for p in grown { if self.world.get(p[0], p[1], p[2]) == Some(B::SAPLING) { let tree: Vec<(i32, i32, i32, Id)> = crate::gen::young_oak(p[0], p[1], p[2], (p[0] * 31 + p[2] * 17) as u32).into_iter().filter(|d| d[3] > 0 && self.world.get(d[0], d[1], d[2]).is_some_and(|b| b == 0 || crate::blocks::BLOCKS[b as usize].shape == crate::blocks::Shape::Deco || b == B::SAPLING)).map(|d| (d[0], d[1], d[2], d[3] as Id)).collect(); self.world.edit_many(&tree); e.trees.push(p); } e.saplings.retain(|s| s.0 != p); }
        // qui fait quoi : les métiers suivent le recensement, dans l'ordre des habitants
        let mine: Vec<usize> = (0..self.town.folk.len()).filter(|&k| self.town.folk[k].v == v.id && self.town.folk[k].craft.is_none()).collect();
        let (farmers, cutters, miners) = (if c.fields > 0 { c.fields.div_ceil(2).max(1) } else { 0 }, c.cutters, c.miners);
        // le maire : le dernier venu des habitants sans métier, sinon le dernier de la liste ; il reçoit à la place, où l'on vient le voir
        let workers = (farmers + cutters + miners).min(mine.len().saturating_sub(1) as u32);
        for (i, &k) in mine.iter().enumerate() { let i = i as u32; self.town.folk[k].job = if i + 1 == mine.len() as u32 { 9 } else if i >= workers { 0 } else if i < farmers { 1 } else if i < farmers + cutters { 2 } else { 3 }; }
        let store = e.store.first().copied().map(|p| { let s = [p[0] as f64 + 0.5, p[1] as f64, p[2] as f64 + 0.5]; (s, self.econ_beside(s)) });
        let near_player = |p: [f64; 3], me: [f64; 3]| (p[0] - me[0]).hypot(p[2] - me[2]) < 40.0; let me = self.player.p;
        let mut taken: Vec<[i32; 3]> = mine.iter().filter_map(|&k| { let f = &self.town.folk[k]; f.task.map(|_| [f.aim[0].floor() as i32, f.aim[1].floor() as i32, f.aim[2].floor() as i32]) }).collect();
        for &k in &mine {
            let mut f = self.town.folk[k].clone(); if f.job == 0 || f.scared > 0.0 { continue; }
            // la journée finie, on dépose ce qu'on porte et on rentre
            if !work { for (id, n) in f.carry.drain(..) { e.add(id, n as f64); } if f.task.is_some() { f.task = None; f.task_k += 1; } self.town.folk[k] = f; continue; }
            if f.task.is_none() {
                let full = |id: &str| e.have(id) >= FULL; let load = f.carry.iter().map(|c| c.1).sum::<u32>();
                let back = match f.job { 1 => load >= LOAD_CROPS, 2 => load > 0, _ => load >= LOAD_STONE };
                let pick = |list: &[[i32; 3]], taken: &[[i32; 3]]| -> Option<[i32; 3]> { let o = [v.c[0] as f64, v.c[2] as f64]; let salt = k as i32 * 13 + f.task_k as i32;
                    list.iter().filter(|p| !taken.contains(p)).min_by(|a, b| { let d = |p: &[i32; 3]| (p[0] as f64 - o[0]).hypot(p[2] as f64 - o[1]) * (0.8 + 0.4 * crate::noise::hash_float(p[0], p[2], salt, 401)); d(a).total_cmp(&d(b)) }).copied() };
                let target = if back { None } else { match f.job { 1 => pick(&e.crops.iter().filter(|p| self.world.get(p[0], p[1], p[2]).is_some_and(|b| crate::play::crop_first(b).is_some_and(|f0| b == f0 + 3))).copied().collect::<Vec<_>>(), &taken), 2 if !full("oak_log") => { /* un vrai arbre, avec son feuillage : pas un tas de bûches ni une poutre */ let mut got = None; while let Some(t) = pick(&e.trees, &taken) { if crate::felling::find_tree(&self.world, t[0], t[1], t[2]).is_some() { got = Some(t); break; } e.trees.retain(|p| *p != t); } got } 3 if !full("cobblestone") => pick(&e.rocks, &taken), _ => None } };
                match (target, store) {
                    (Some(t), _) => { let a = [t[0] as f64 + 0.5, t[1] as f64 + 0.5, t[2] as f64 + 0.5]; let at = self.econ_beside(a); f.aim = a; f.task = Some(at); f.task_k += 1; f.work_t = 0.0; taken.push(t); }
                    (None, Some((sp, at))) if load > 0 => { f.aim = [sp[0], sp[1] + 0.5, sp[2]]; f.task = Some(at); f.task_k += 1; f.work_t = 0.0; }
                    _ => {}
                }
            }
            // au travail, sur place
            if f.working && f.task.is_some() {
                f.work_t += dt; let t = [f.aim[0].floor() as i32, f.aim[1].floor() as i32, f.aim[2].floor() as i32]; let at_store = store.is_some_and(|s| (s.0[0] - f.aim[0]).abs() < 0.6 && (s.0[2] - f.aim[2]).abs() < 0.6);
                let need = if at_store { DROP } else { match f.job { 1 => REAP, 2 => CHOP, _ => DIG } };
                if f.work_t >= need {
                    let b = self.world.get(t[0], t[1], t[2]).unwrap_or(0); let name = crate::blocks::BLOCKS[b as usize].name;
                    if at_store { for (id, n) in f.carry.drain(..) { e.add(id, n as f64); } }
                    else { match f.job {
                        1 => if let Some(f0) = crate::play::crop_first(b).filter(|f0| b == f0 + 3) {
                            // la moisson : le plant mûr donne sa récolte et l'on ressème aussitôt
                            let got = match name { n if n.starts_with("wheat") => "wheat", n if n.starts_with("carrot") => "carrot", _ => "potato" }; let n = if got == "wheat" { 1 } else { 2 };
                            self.world.edit(t[0], t[1], t[2], f0); self.crops.retain(|c| c.0 != t); self.crops.push((t, self.hours)); add_to(&mut f.carry, got, n); }
                        2 => if let Some(tree) = crate::felling::find_tree(&self.world, t[0], t[1], t[2]) {
                            // l'arbre tombe ; on replante au pied
                            let at = |s: &Self, p: (i32, i32, i32)| s.world.get(p.0, p.1, p.2).unwrap_or(0);
                            let logs: Vec<((i32, i32, i32), Id)> = tree.logs.iter().filter(|p| p.1 >= t[1]).map(|p| (*p, crate::blocks::log_of(at(self, *p)))).collect(); let leaf = tree.leaves.first().map_or(0, |p| at(self, *p));
                            let edits: Vec<_> = logs.iter().map(|(p, _)| (p.0, p.1, p.2, 0)).chain(tree.leaves.iter().map(|p| (p.0, p.1, p.2, 0))).collect(); self.world.edit_many(&edits);
                            if self.falling.is_none() && near_player(f.p, me) { self.falling = Some(crate::felling::Fall::new(&logs, &tree.leaves, leaf, (t[0], t[1], t[2]), f.p, (t[0] * 31 + t[2]) as u32)); self.fall_serial += 1; self.sound.tree_crack(); }
                            if self.world.get(t[0], t[1] - 1, t[2]).is_some_and(|g| matches!(crate::blocks::BLOCKS[g as usize].name, "grass" | "dirt" | "podzol" | "forest_floor")) { self.world.edit(t[0], t[1], t[2], B::SAPLING); e.saplings.push((t, self.hours)); }
                            add_to(&mut f.carry, "oak_log", logs.len().clamp(1, 24) as u32); e.trees.retain(|p| *p != t); } else { e.trees.retain(|p| *p != t); }
                        _ => if is_stone_face(name) {
                            // un bloc de roche tombe sous la pioche ; le minerai va au minerai
                            let got = match name { "coal_ore" => "coal", "iron_ore" => "iron_ore", _ => "cobblestone" }; self.world.edit(t[0], t[1], t[2], 0); add_to(&mut f.carry, got, 1);
                            if near_player(f.p, me) { self.sound.place("stone"); } e.rocks.retain(|p| *p != t); }
                    } }
                    f.task = None; f.work_t = 0.0; self.dirty = true;
                }
            }
            self.town.folk[k] = f;
        }
    }
    /// Où se tenir pour travailler un bloc : sur le sol libre à côté, du côté du village.
    fn econ_beside(&self, a: [f64; 3]) -> [f64; 3] {
        let (x, y, z) = (a[0].floor() as i32, a[1].floor() as i32, a[2].floor() as i32); let mut best: Option<(f64, [f64; 3])> = None;
        for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)] {
            for dy in [0, -1, 1, -2] { let (px, py, pz) = (x + dx, y + dy, z + dz);
                let free = |yy: i32| self.world.get(px, yy, pz).is_none_or(|b| !is_solid(b)); let floor = self.world.get(px, py - 1, pz).is_some_and(is_solid);
                if floor && free(py) && free(py + 1) { let p = [px as f64 + 0.5 - dx as f64 * 0.25, py as f64, pz as f64 + 0.5 - dz as f64 * 0.25]; let d = (dx * dx + dz * dz) as f64 + dy.abs() as f64 * 0.5; if best.is_none_or(|b| d < b.0) { best = Some((d, p)); } break; } } }
        best.map_or([a[0], a[1] - 0.5, a[2]], |b| b.1)
    }
    /// Ce qu'il y a à faire autour du village : les pieds d'arbre de la lisière, les cultures de ses champs, la roche à nu.
    fn econ_scan(&self, e: &mut Econ, v: &Village) {
        let (cx, cy, cz) = (v.c[0], v.c[1], v.c[2]); let r = (v.r + 26).min(64);
        let houses: Vec<[i32; 4]> = v.works.iter().map(|w| w.1).chain(v.houses.iter().map(|h| h.area)).collect(); let inside = |x: i32, z: i32| houses.iter().any(|a| x >= a[0] - 2 && x <= a[2] + 2 && z >= a[1] - 2 && z <= a[3] + 2);
        let (mut trees, mut crops, mut rocks) = (Vec::new(), Vec::new(), Vec::new());
        for x in cx - r..=cx + r { for z in cz - r..=cz + r {
            let d2 = (x - cx).pow(2) + (z - cz).pow(2); if d2 > r * r { continue; }
            // le haut de la colonne, de là où le monde est chargé
            let Some(top) = (cy - 10..cy + 14).rev().find(|&y| self.world.get(x, y, z).is_some_and(|b| b != 0 && !crate::blocks::is_leaf(b) && crate::blocks::BLOCKS[b as usize].shape != crate::blocks::Shape::Deco || self.world.get(x, y, z).is_some_and(crate::blocks::is_log))) else { continue };
            let b = self.world.get(x, top, z).unwrap_or(0); let n = crate::blocks::BLOCKS[b as usize].name;
            // un pied d'arbre : le tronc posé sur la terre, hors du village
            if !inside(x, z) { for y in (cy - 8..=top).rev() { let t = self.world.get(x, y, z).unwrap_or(0); if crate::blocks::is_log(t) && !crate::blocks::BLOCKS[t as usize].name.ends_with("_c") && self.world.get(x, y - 1, z).is_some_and(|g| matches!(crate::blocks::BLOCKS[g as usize].name, "grass" | "dirt" | "podzol" | "forest_floor")) && crate::blocks::BLOCKS[t as usize].name.starts_with("oak") && (1..=3).all(|d| self.world.get(x, y + d, z).is_some_and(crate::blocks::is_log)) { trees.push([x, y, z]); break; } } }
            // une culture, dans un champ du village
            if let Some(cb) = self.world.get(x, top + 1, z) { if crate::play::crop_first(cb).is_some() { crops.push([x, top + 1, z]); } }
            if self.world.get(x, top, z).is_some_and(|b| crate::play::crop_first(b).is_some()) { crops.push([x, top, z]); }
            // la roche à nu, pas trop bas, hors du village et des chemins
            if is_stone_face(n) && !inside(x, z) && top >= cy - 6 && self.world.get(x, top + 1, z).is_some_and(|a| !is_solid(a)) && (x - cx).abs().max((z - cz).abs()) > v.r / 2 { rocks.push([x, top, z]); }
        } }
        e.trees = trees; e.crops = crops; e.rocks = rocks;
    }
}
fn add_to(carry: &mut Vec<(&'static str, u32)>, id: &'static str, n: u32) { if let Some(c) = carry.iter_mut().find(|c| c.0 == id) { c.1 += n; } else { carry.push((id, n)); } }

/// « la maison », « l'auberge », « le moulin ».
fn article(name: &str) -> String { let l = name.to_lowercase(); let fem = matches!(l.as_str(), "maison" | "ferme" | "étable" | "menuiserie" | "forge" | "tannerie"); if l.starts_with(['a', 'e', 'é', 'i', 'o', 'u']) { format!("l'{l}") } else if fem || l == "auberge" { format!("la {l}") } else { format!("le {l}") } }
/// « de la maison », « du moulin », « de l'auberge ».
fn de(name: &str) -> String { let a = article(name); if let Some(r) = a.strip_prefix("le ") { format!("du {r}") } else { format!("de {a}") } }
