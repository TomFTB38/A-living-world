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
pub const GOODS: [(&str, &str); 11] = [("wheat", "blé"), ("flour", "farine"), ("bread", "pain"), ("cooked_meat", "viande"), ("oak_log", "bois"), ("planks", "planches"), ("cobblestone", "pierre"), ("iron_ore", "minerai"), ("coal", "charbon"), ("iron_ingot", "fer"), ("leather", "cuir")];
/// Au-delà, on ne produit plus : les greniers sont pleins, le village vit sur son acquis.
const FULL: f64 = 320.0;
/// Ce que coûte une maison.
pub const HOUSE_COST: [(&str, f64); 3] = [("planks", 48.0), ("cobblestone", 36.0), ("oak_log", 12.0)];
/// Ceux qui emménagent dans une maison neuve.
const NEWCOMERS: u32 = 2;

/// Une maison que le village a bâtie depuis la création du monde : son terrain et son niveau.
#[derive(Clone, Copy, Debug)] pub struct Extra { pub lot: Lot, pub y: i32 }
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
    /// Une maison payée, qui attend d'être posée (le joueur est loin).
    pub due: u32,
    /// Le journal du village : (jour, ce qui s'est passé), les plus récents à la fin.
    pub log: Vec<(u32, String)>,
    /// Les coffres de la réserve (quand le village est chargé), et si le joueur y a la main.
    pub store: Vec<[i32; 3]>, open: bool, held: f64,
}

/// Habitants d'un village tel qu'il a été tracé (la même règle que pour les faire apparaître).
pub fn people(v: &Village) -> u32 { v.houses.iter().enumerate().map(|(i, h)| if h.craft.is_some() { 1 } else { 1 + (crate::noise::hash_float(v.id.0, v.id.1, i as i32, 91) < 0.6) as u32 }).sum() }

impl Econ {
    /// Un village à sa première heure : il a déjà quelques jours de réserves, selon sa richesse.
    pub fn found(v: &Village, t: f64) -> Econ {
        let pop = people(v) as f64; let wealth = v.ident.map_or(0.5, |i| i.wealth); let mut stock = BTreeMap::new();
        for (k, n) in [("wheat", pop * 3.0), ("flour", pop), ("bread", pop * 2.5), ("cooked_meat", pop * 0.5), ("oak_log", 16.0 + 20.0 * wealth), ("planks", 10.0 + 24.0 * wealth), ("cobblestone", 8.0 + 20.0 * wealth), ("iron_ore", 4.0), ("coal", 6.0), ("iron_ingot", 2.0 + 4.0 * wealth), ("leather", 2.0)] { stock.insert(k, n.floor()); }
        Econ { id: v.id, stock, t, fed: 1.0, lean: 0, extra: Vec::new(), due: 0, log: Vec::new(), store: Vec::new(), open: false, held: 0.0 }
    }
    pub fn have(&self, k: &str) -> f64 { self.stock.get(k).copied().unwrap_or(0.0) }
    fn add(&mut self, k: &'static str, n: f64) { let e = self.stock.entry(k).or_insert(0.0); *e = (*e + n).max(0.0); }
    /// Prend jusqu'à `n` ; rend ce qui a été pris.
    fn take(&mut self, k: &'static str, n: f64) -> f64 { let h = self.have(k).min(n).max(0.0); self.add(k, -h); h }
    fn note(&mut self, day: u32, s: String) { self.log.push((day, s)); if self.log.len() > 12 { self.log.remove(0); } }
    /// Le recensement du village : ce qu'il a été tracé, plus les maisons bâties depuis.
    pub fn census(&self, v: &Village) -> Census {
        let n = |f: &dyn Fn(Arch) -> bool| v.works.iter().filter(|w| f(w.0)).count() as u32;
        let mut c = Census { pop: people(v) + self.extra.len() as u32 * NEWCOMERS, fields: v.fields, farms: n(&|a| matches!(a, Arch::Farm | Arch::Barn)), folds: n(&|a| matches!(a, Arch::Fold | Arch::Stable)), mills: n(&|a| a == Arch::Mill), inns: n(&|a| a == Arch::Inn),
            forges: n(&|a| a == Arch::Forge), woodshops: n(&|a| matches!(a, Arch::Woodshop | Arch::Workshop)), tanneries: n(&|a| a == Arch::Tannery), ..Default::default() };
        // ceux qui ne tiennent ni champ ni atelier vont au bois et à la carrière
        let busy = c.fields.max(c.farms) + c.folds + c.mills + c.inns + c.forges + c.woodshops + c.tanneries; let free = c.pop.saturating_sub(busy).max(2);
        c.cutters = (free as f64 * 0.45).ceil() as u32; c.miners = (free as f64 * 0.35).ceil() as u32;
        // un village tient à son tempérament : celui qui veut rester ce qu'il est ne s'agrandit pas. Le pilote, lui, bâtit au moins deux maisons, pour qu'on le voie faire.
        let want = match v.ident.map(|i| i.outlook) { Some(Outlook::Expansion) => 4, Some(Outlook::Prosperity) => 3, Some(Outlook::Trade | Outlook::SelfSufficiency) => 2, Some(Outlook::Defense) => 1, _ => 0u32 }.max(2);
        c.room = want.saturating_sub(self.extra.len() as u32 + self.due); c
    }
    /// Les jours de vivres devant soi.
    pub fn reserve(&self, pop: u32) -> f64 { (self.have("bread") + self.have("cooked_meat") + self.have("flour") * 0.8 + self.have("wheat") * 0.5) / pop.max(1) as f64 }
    /// Une heure de la vie du village. `hod` : l'heure du jour (0 à 23) ; `day` : le numéro du jour.
    pub fn step(&mut self, c: &Census, hod: u32, day: u32) {
        let work = (7..12).contains(&hod) || (13..18).contains(&hod);
        if work {
            let room = |e: &Econ, k: &str| e.have(k) < FULL; let pop = c.pop as f64;
            // la disette presse les champs : on y met tous les bras
            let press = if self.reserve(c.pop) < 1.0 { 1.6 } else { 1.0 };
            if room(self, "wheat") { self.add("wheat", (c.fields.max(1) as f64 * 0.2 + c.farms as f64 * 0.25) * press); }
            if room(self, "cooked_meat") { self.add("cooked_meat", c.folds as f64 * 0.12); } if room(self, "leather") { self.add("leather", c.tanneries.min(c.folds.max(1)) as f64 * 0.03); }
            // le moulin, ou à défaut la meule à bras de chaque foyer ; le four de l'auberge, ou à défaut celui des maisons
            if room(self, "flour") { let x = self.take("wheat", c.mills as f64 * 3.0 + pop * 0.12); self.add("flour", x); }
            if room(self, "bread") { let x = self.take("flour", c.inns as f64 * 2.0 + pop * 0.12); self.add("bread", x); }
            if room(self, "oak_log") { self.add("oak_log", c.cutters as f64 * 0.5); }
            if room(self, "planks") && self.have("oak_log") > 14.0 { let x = self.take("oak_log", (c.woodshops as f64 * 0.8 + 0.15).min(self.have("oak_log") - 14.0)); self.add("planks", x * 2.0); }
            if room(self, "cobblestone") { self.add("cobblestone", c.miners as f64 * 0.4); } if room(self, "iron_ore") { self.add("iron_ore", c.miners as f64 * 0.04); } if room(self, "coal") { self.add("coal", c.miners as f64 * 0.05); }
            if room(self, "iron_ingot") { let n = (c.forges as f64 * 0.15).min(self.have("iron_ore")).min(self.have("coal")); self.take("iron_ore", n); self.take("coal", n); self.add("iron_ingot", n); }
        }
        // le chantier se décide le matin : les greniers sont pleins pour quatre jours, personne n'a eu faim, et les matériaux y sont
        if hod == 8 && c.room > 0 && self.fed >= 0.95 && self.reserve(c.pop) >= 4.0 && HOUSE_COST.iter().all(|(k, n)| self.have(k) >= *n) {
            for (k, n) in HOUSE_COST { self.take(k, n); } self.due += 1; self.note(day, "les matériaux d'une maison sont réunis : on bâtit".into());
        }
        // le repas du soir : pain d'abord, puis viande, farine en bouillie, blé en grains ; le feu de la veillée
        if hod == 19 {
            let need = c.pop as f64; let mut left = need;
            for (k, worth) in [("bread", 1.0), ("cooked_meat", 1.0), ("flour", 0.8), ("wheat", 0.5)] { if left <= 0.0 { break; } let got = self.take(k, left / worth); left -= got * worth; }
            let was = self.fed; self.fed = ((need - left.max(0.0)) / need.max(1.0)).clamp(0.0, 1.0); self.take("oak_log", need * 0.05);
            if self.fed < 0.6 { self.lean += 1; if was >= 0.6 { self.note(day, "le grenier est vide : on se couche le ventre creux".into()); } } else { if self.lean > 0 { self.note(day, "on mange de nouveau à sa faim".into()); } self.lean = 0; }
        }
    }
    /// Une ligne par sujet, pour la commande `/village` et le journal.
    pub fn report(&self, v: &Village, day: u32) -> Vec<String> {
        let c = self.census(v); let mut o = vec![format!("{} — {} habitants, {} maison{} bâtie{} depuis votre arrivée", v.name, c.pop, self.extra.len(), if self.extra.len() > 1 { "s" } else { "" }, if self.extra.len() > 1 { "s" } else { "" })];
        o.push(format!("Stocks : {}", GOODS.iter().filter(|g| self.have(g.0) >= 1.0).map(|g| format!("{} {}", g.1, self.have(g.0).floor())).collect::<Vec<_>>().join(", ")));
        o.push(format!("Vivres pour {:.1} jours ; hier soir {} % de la population a mangé{}", self.reserve(c.pop), (self.fed * 100.0).round(), if self.lean > 0 { format!(" (disette depuis {} jour{})", self.lean, if self.lean > 1 { "s" } else { "" }) } else { String::new() }));
        o.push(format!("Au travail : {} champ{}, {} au bois, {} à la carrière, moulin {}, four {}, forge {}", c.fields, if c.fields > 1 { "s" } else { "" }, c.cutters, c.miners, c.mills, c.inns, c.forges));
        o.push(if self.due > 0 { "Chantier : une maison est payée, elle sera bâtie dès que le village vous verra revenir".into() } else if c.room == 0 { "Chantier : le village ne souhaite pas s'agrandir davantage".into() } else { format!("Prochaine maison : {}", HOUSE_COST.iter().map(|(k, n)| format!("{} {}/{}", GOODS.iter().find(|g| g.0 == *k).map_or(*k, |g| g.1), self.have(k).floor().min(*n), n)).collect::<Vec<_>>().join(", ")) + if self.reserve(c.pop) < 4.0 { " — et quatre jours de vivres d'avance" } else { "" } });
        for (d, s) in self.log.iter().rev().take(3) { o.push(format!("Il y a {} jour{} : {s}", day.saturating_sub(*d), if day.saturating_sub(*d) > 1 { "s" } else { "" })); }
        o
    }
    pub fn save(&self) -> String {
        let mut s = format!("econ {} {} {} {} {} {}", self.id.0, self.id.1, self.t, self.fed, self.lean, self.due); for (k, n) in &self.stock { s += &format!(" {k}:{n}"); } s.push('\n');
        for e in &self.extra { s += &format!("econ_house {} {} {} {} {} {} {}\n", e.lot.o.0, e.lot.o.1, e.lot.u.0, e.lot.u.1, e.lot.w.0, e.lot.w.1, e.y); }
        for (d, t) in &self.log { s += &format!("econ_log {d} {t}\n"); } s
    }
}

/// La maison posée sur un terrain : ses blocs, et où l'on y vit.
fn raise(lot: Lot, y: i32) -> (Vec<(i32, i32, i32, Id)>, House) {
    let mut k = Work::default(); let pl = patrons::plan(Kind::Maison, 0); let made = patrons::build(&mut k, lot, y, Kind::Maison, 0, Look { roof: "roof_tiles", infill: "plaster" }, 0.4);
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
    pub(crate) fn econ_extend(&self, v: &mut Village) { if let Some(e) = self.econ.as_ref().filter(|e| e.id == v.id) { for x in &e.extra { v.houses.push(raise(x.lot, x.y).1); } } }
    /// Le temps passe au village pilote : une heure de jeu à la fois, en rattrapant ce qui s'est écoulé loin de lui.
    pub(crate) fn econ_tick(&mut self) {
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
        // la maison payée se pose dès que le joueur est là
        if near && e.due > 0 { if let Some((lot, y)) = self.econ_lot(&v, &e) { let (blocks, house) = raise(lot, y);
                let mut list = blocks; for dx in house.area[0]..=house.area[2] { for dz in house.area[1]..=house.area[3] { for yy in (y - 6..=y).rev() { if self.world.get(dx, yy, dz).is_some_and(is_solid) { break; } list.push((dx, yy, dz, B::COBBLESTONE)); } } }
                self.world.edit_many(&list); e.due -= 1; e.extra.push(Extra { lot, y }); let day = (e.t / 24.0) as u32; e.note(day, "une maison neuve est debout, deux habitants s'y installent".into());
                said.push(format!("{} a bâti une nouvelle maison", v.name));
                // le village chargé apprend sa nouvelle maison, et ses habitants paraissent
                if let Some(i) = self.town.here.iter().position(|h| h.id == v.id) { let mut only = self.town.here[i].clone(); let n = only.houses.len(); self.town.here[i].houses.push(house.clone()); only.houses = vec![house]; let base = self.town.folk.len(); self.town.spawn(&only, self.hour); for f in &mut self.town.folk[base..] { f.house = n; } }
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
    fn econ_lot(&self, v: &Village, e: &Econ) -> Option<(Lot, i32)> {
        let pl = patrons::plan(Kind::Maison, 0); let taken: Vec<[i32; 4]> = v.works.iter().map(|w| w.1).chain(e.extra.iter().map(|x| raise(x.lot, x.y).1.area)).collect();
        let hits = |a: [i32; 4]| taken.iter().any(|t| a[0] <= t[2] + 3 && a[2] >= t[0] - 3 && a[1] <= t[3] + 3 && a[3] >= t[1] - 3);
        for ring in 0..7 { let r = v.r as f64 * 0.55 + ring as f64 * 4.0; for k in 0..20 {
            let a = (k as f64 + crate::noise::hash_float(v.id.0, v.id.1, ring * 31 + e.extra.len() as i32, 301)) / 20.0 * std::f64::consts::TAU; let (cx, cz) = (v.c[0] + (a.cos() * r) as i32, v.c[2] + (a.sin() * r) as i32);
            // la façade regarde le centre du village
            let (dx, dz) = (v.c[0] - cx, v.c[2] - cz); let w = if dx.abs() > dz.abs() { (-dx.signum(), 0) } else { (0, -dz.signum()) }; let u = (-w.1, w.0);
            let lot = Lot { o: (cx - u.0 * pl.w / 2 - w.0 * pl.d / 2, cz - u.1 * pl.w / 2 - w.1 * pl.d / 2), u, w };
            let (c0, c1) = (lot.cell(pl.ext[0] - 1, 0, pl.ext[1] - 2), lot.cell(pl.ext[2] + 1, 0, pl.ext[3] + 1)); let area = [c0[0].min(c1[0]), c0[2].min(c1[2]), c0[0].max(c1[0]), c0[2].max(c1[2])];
            if hits(area) { continue; }
            // à plat, au sec, hors des chemins et des cultures ; et le monde est bien chargé là
            let (mut lo, mut hi, mut ok) = (i32::MAX, i32::MIN, true);
            'scan: for x in area[0]..=area[2] { for z in area[1]..=area[3] { let col = self.gen.column(x as f64, z as f64); let h = col.h.floor() as i32; lo = lo.min(h); hi = hi.max(h);
                if col.wet || hi - lo > 2 { ok = false; break 'scan; } let top = self.world.get(x, h, z); let above = self.world.get(x, h + 1, z);
                if top.is_none() || top.is_some_and(|b| matches!(crate::blocks::BLOCKS[b as usize].name, "path" | "farmland" | "gravel" | "cobblestone")) || above.is_some_and(|b| is_solid(b) || crate::blocks::BLOCKS[b as usize].name.starts_with("wheat")) { ok = false; break 'scan; } } }
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
                self.econ = Some(Econ { id, stock, t, fed: f(3).unwrap_or(1.0), lean: n(4).unwrap_or(0) as u32, extra: Vec::new(), due: n(5).unwrap_or(0) as u32, log: Vec::new(), store: Vec::new(), open: false, held: 0.0 }); }
            "econ_house" => { if let (Some(e), Some(v)) = (self.econ.as_mut(), (0..7).map(n).collect::<Option<Vec<i32>>>()) { e.extra.push(Extra { lot: Lot { o: (v[0], v[1]), u: (v[2], v[3]), w: (v[4], v[5]) }, y: v[6] }); } }
            "econ_log" => { if let (Some(e), Some(d)) = (self.econ.as_mut(), n(0)) { e.log.push((d as u32, a[1..].join(" "))); } }
            _ => {}
        }
    }
}
