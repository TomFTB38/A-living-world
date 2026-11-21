//! Le mobilier dont on se sert : on s'assied sur une chaise, un banc, un fauteuil, un tabouret ; on range ses affaires dans une
//! armoire, un tonneau, une caisse ; on pose des objets sur une étagère ou une table, et ils y restent, visibles.
//! Accroupi, le clic droit pose un bloc contre le meuble comme avant. Tout est vérifié par `tests/mobilier.rs`.
use crate::blocks::{Id, BLOCKS};
use crate::player::Voxels;
use crate::session::{Session, Stack};

/// Un meuble dont on se sert (siège, rangement, plateau).
pub fn furniture_like(id: Id) -> bool { seat(id).is_some() || storage(id) || !slots(id).is_empty() }
/// Le nom d'un meuble sans son sens (« chair@2 » → « chair »).
pub fn base(id: Id) -> &'static str { let n = BLOCKS[id as usize].name; n.split('@').next().unwrap_or(n) }
/// La hauteur de l'assise d'un siège, si c'en est un.
pub fn seat(id: Id) -> Option<f64> {
    Some(match base(id) { "chair" => 0.52, "bench" => 0.5, "armchair" | "throne" => 0.56, "stool" => 0.535, "pouf" => 0.36, "sedari" => 0.42, "fur_bench" => 0.5, "sea_chest" => 0.55, _ => return None })
}
/// Un meuble où l'on range : il s'ouvre comme un coffre.
pub fn storage(id: Id) -> bool { matches!(base(id), "cupboard" | "barrel" | "crate" | "wardrobe" | "sea_chest" | "rune_chest" | "wine_rack" | "dresser") }
/// Les places où l'on pose un objet sur un meuble (dans le sens 0, dos vers +z) : (x, hauteur du plateau, z).
pub fn slots(id: Id) -> Vec<[f64; 3]> {
    match base(id) {
        "shelf" => vec![[0.2, 0.12, 0.78], [0.5, 0.12, 0.78], [0.8, 0.12, 0.78], [0.2, 0.58, 0.78], [0.5, 0.58, 0.78], [0.8, 0.58, 0.78]],
        "table" => vec![[0.28, 0.96, 0.28], [0.72, 0.96, 0.28], [0.28, 0.96, 0.72], [0.72, 0.96, 0.72]],
        "round_table" => vec![[0.5, 0.92, 0.3], [0.32, 0.92, 0.64], [0.68, 0.92, 0.64]],
        "tray_table" => vec![[0.5, 0.42, 0.5], [0.3, 0.42, 0.42], [0.7, 0.42, 0.58]],
        "writing_desk" => vec![[0.3, 0.82, 0.4], [0.7, 0.82, 0.4]],
        "dresser" => vec![[0.2, 1.0, 0.65], [0.5, 1.0, 0.65], [0.8, 1.0, 0.65]],
        _ => vec![],
    }
}
/// Le sens d'un meuble (0 à 3).
pub fn facing(id: Id) -> u8 { BLOCKS[id as usize].name.rsplit_once('@').and_then(|(_, d)| d.parse().ok()).unwrap_or(0) }
/// Une place du meuble dans le monde : le quart de tour du meuble, comme son dessin ((x, z) devient (z, 1 - x)).
pub fn place(p: [i32; 3], id: Id, s: [f64; 3]) -> [f64; 3] { let (mut x, mut z) = (s[0], s[2]); for _ in 0..facing(id) % 4 { (x, z) = (z, 1.0 - x); } [p[0] as f64 + x, p[1] as f64 + s[1], p[2] as f64 + z] }
/// Le devant d'un meuble (où l'on se lève d'un siège) : à l'opposé de son dos.
pub fn front(id: Id) -> (i32, i32) { let (dx, dz) = [(0, 1), (1, 0), (0, -1), (-1, 0)][crate::shapes::info(id).map_or(facing(id), |i| i.facing) as usize % 4]; (-dx, -dz) }

impl Session {
    /// Clic droit sur un meuble : s'asseoir, ouvrir ce qui range, poser ou reprendre un objet. Rend vrai si c'en était un.
    pub fn furniture_at(&mut self, p: [i32; 3], id: Id) -> bool {
        if seat(id).is_some() && !storage(id) { self.sit(p, id); return true; }
        if storage(id) { self.open_storage(p); return true; }
        let s = slots(id); if s.is_empty() { return false; }
        let row = self.shelves.entry(p).or_insert_with(|| vec![None; s.len()]); row.resize(s.len(), None);
        // la main pleine : on pose (un seul objet) sur la première place libre ; sinon, on reprend le dernier posé
        if let (Some(st), Some(free)) = (self.inv[self.sel], row.iter().position(|x| x.is_none())) {
            row[free] = Some(Stack { item: st.item, n: st.n.map(|_| 1), wear: st.wear }); self.consume_held(); self.sound.place(base(id)); self.dirty = true; return true; }
        if self.inv[self.sel].is_some() { self.notes.push("Il n'y a plus de place".into()); return true; }
        if let Some(k) = row.iter().rposition(|x| x.is_some()) { let st = row[k].take().unwrap(); if row.iter().all(|x| x.is_none()) { self.shelves.remove(&p); }
            if let Some(r) = self.add_stack(st) { self.spill_worn(r.item, r.n.unwrap_or(1), r.wear, [p[0] as f64 + 0.5, p[1] as f64 + 1.0, p[2] as f64 + 0.5]); } self.sound.place(base(id)); self.dirty = true; return true; }
        true
    }
    /// S'asseoir : le joueur se pose sur l'assise, face au devant du siège.
    pub fn sit(&mut self, p: [i32; 3], id: Id) {
        let Some(h) = seat(id) else { return }; let (fx, fz) = front(id); self.seated = Some(p); self.ride = None;
        self.player.p = [p[0] as f64 + 0.5, p[1] as f64 + h - 0.45, p[2] as f64 + 0.5]; self.player.v = [0.0; 3]; self.player.yaw = (fx as f64).atan2(fz as f64);
        self.notes.push("Vous vous asseyez (avancez ou sautez pour vous lever)".into());
    }
    /// Assis : on ne bouge pas, on reprend son souffle trois fois plus vite ; un pas, un saut, s'accroupir : on se lève, devant le
    /// siège. Rend vrai tant qu'on est assis.
    pub(crate) fn seat_step(&mut self, dt: f64, i: crate::player::Input) -> bool {
        let Some(p) = self.seated else { return false };
        let id = self.world.get(p[0], p[1], p[2]).unwrap_or(0); let Some(h) = seat(id) else { self.seated = None; return false };
        if i.x != 0.0 || i.z != 0.0 || i.jump || i.crouch || self.dead.is_some() { self.stand_up(); return false; }
        self.player.p = [p[0] as f64 + 0.5, p[1] as f64 + h - 0.45, p[2] as f64 + 0.5]; self.player.v = [0.0; 3]; self.player.ground = true; self.player.speed = 0.0;
        self.stamina = (self.stamina + 30.0 * dt).min(100.0); self.winded = false; true
    }
    /// Se lever : devant le siège si la place est libre, sinon dessus.
    pub fn stand_up(&mut self) {
        let Some(p) = self.seated.take() else { return }; let id = self.world.get(p[0], p[1], p[2]).unwrap_or(0); let (fx, fz) = front(id);
        let free = |s: &Session, x: i32, y: i32, z: i32| [y, y + 1].iter().all(|yy| !s.world.get(x, *yy, z).is_some_and(crate::blocks::is_solid));
        let (x, z) = (p[0] + fx, p[2] + fz); let y = if free(self, x, p[1], z) { p[1] } else { p[1] + 1 };
        let (x, z) = if y == p[1] { (x, z) } else { (p[0], p[2]) }; self.player.p = [x as f64 + 0.5, y as f64, z as f64 + 0.5]; self.player.v = [0.0; 3];
    }
    /// Ouvrir un meuble de rangement : son contenu, dans la fenêtre du coffre.
    pub fn open_storage(&mut self, p: [i32; 3]) {
        self.chests.entry(p).or_insert_with(|| vec![None; crate::session::CHEST_SIZE]); self.chest = Some(p); self.chest2 = None; self.at_station = None; self.inv_open = true; self.sound.place("chest"); self.sound.creak();
    }
    /// Ce qui est posé sur les étagères et les tables, à dessiner.
    pub fn shelf_draws(&self, md: &crate::actors::Models) -> Vec<crate::actors::Draw> {
        use crate::view::{mul, trs};
        let mut o = Vec::new();
        for (p, row) in &self.shelves { let Some(id) = self.world.get(p[0], p[1], p[2]) else { continue }; let s = slots(id); let yaw = facing(id) as f64 * std::f64::consts::FRAC_PI_2;
            for (k, st) in row.iter().enumerate() { let (Some(st), Some(sl)) = (st, s.get(k)) else { continue }; let Some(h) = md.held(st.item) else { continue };
                let q = place(*p, id, *sl); let base = trs(q[0], q[1], q[2], yaw + k as f64 * 0.4, 0.0, 1.0);
                let m = match h.hold { crate::actors::Hold::Tool => mul(&base, &trs(0.0, 0.03, 0.0, 0.0, -std::f64::consts::FRAC_PI_2, 0.62)), _ => mul(&base, &mul(&trs(0.0, 0.0, 0.0, 0.0, 0.0, 0.42), &trs(-0.5, 0.0, -0.5, 0.0, 0.0, 1.0))) };
                o.push(crate::actors::Draw { m, r: h.r, view: false, local: h.local, hero: false, dynamic: false, hurt: 0.0 }); } }
        o
    }
}
