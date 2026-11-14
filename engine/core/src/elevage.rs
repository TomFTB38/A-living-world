//! L'élevage et les compagnons : traire la vache au seau, tondre le mouton aux cisailles (la laine repousse), ramasser
//! les œufs que pondent les poules ; apprivoiser un loup (os ou viande) ou un cheval (blé, carotte, pomme de terre),
//! qui vous suit et, pour le loup, vous défend ; seller l'un ou l'autre et le chevaucher.
use crate::session::{Prey, Session};
use crate::wild::Kind;

/// Les bêtes qu'on peut monter, une fois apprivoisées et sellées.
pub fn mount(k: Kind) -> bool { matches!(k, Kind::Horse | Kind::Wolf) }
/// La hauteur de la selle au-dessus du sol.
/// (On ne monte un loup que sellé : c'est alors un grand loup de monte, voir `wild::WOLF_MOUNT`.)
pub fn seat(k: Kind) -> f64 { if k == Kind::Horse { 1.05 } else { 0.72 * crate::wild::WOLF_MOUNT } }

impl Session {
    /// Près d'un village (0 à 1) : c'est là que paissent les bêtes de ferme.
    pub fn farmland(&self) -> f64 {
        let p = self.player.p; self.town.here.iter().map(|v| { let d = (v.c[0] as f64 - p[0]).hypot(v.c[2] as f64 - p[2]) - v.r as f64; crate::noise::smooth(70.0, 10.0, d) }).fold(0.0, f64::max)
    }
    /// Le clic droit sur un animal visé : traire, tondre, nourrir et apprivoiser, seller, monter, faire asseoir.
    pub(crate) fn tend(&mut self) -> bool {
        let Some(Prey::Wild(i)) = self.prey else { return false }; let Some(c) = self.wild.all.get(i) else { return false };
        let (kind, farm, at) = (c.kind, c.farm, c.p); let held = self.inv[self.sel].map(|s| s.item); if self.use_t > 0.0 { return true; }
        let name = match kind { Kind::Cow => "La vache", Kind::Sheep => "Le mouton", Kind::Pig => "Le cochon", Kind::Horse => "Le cheval", Kind::Chicken => "La poule", Kind::Wolf => "Le loup", _ => return false };
        let done = |s: &mut Session, note: String| { s.notes.push(note); s.use_t = 0.4; s.anim.trigger(); true };
        match (kind, held) {
            (Kind::Cow, Some("bucket")) => { if farm.milk > 0.0 { return done(self, "La vache n'a plus de lait pour l'instant".into()); }
                self.wild.all[i].farm.milk = 180.0; self.consume_held(); if self.add_item("milk_bucket", 1) > 0 { self.spill("milk_bucket", 1, at); } done(self, "Un seau de lait frais".into()) }
            (Kind::Sheep, Some("shears")) => { if farm.shorn > 0.0 { return done(self, "Le mouton est déjà tondu : sa laine repousse".into()); }
                self.wild.all[i].farm.shorn = 480.0; let n = 1 + (crate::noise::hash_float(i as i32, (self.elapsed * 13.0) as i32, self.seed, 41) * 3.0) as u32; self.spill("wool_white", n, [at[0], at[1] + 0.8, at[2]]); self.wear_slot(self.sel, 1); done(self, format!("{n} toison(s) de laine")) }
            // Apprivoiser : le loup aime l'os et la viande, le cheval le blé, la carotte et la pomme de terre.
            (Kind::Wolf, Some(f @ ("bone" | "raw_meat" | "cooked_meat"))) | (Kind::Horse, Some(f @ ("wheat" | "carrot" | "potato" | "seeds"))) if !farm.tame => {
                self.consume_held(); let odds = match f { "bone" | "cooked_meat" | "wheat" | "carrot" => 0.4, _ => 0.25 };
                if crate::noise::hash_float(i as i32 * 31, (self.elapsed * 97.0) as i32, self.seed, 43) < odds || self.creative { let c = &mut self.wild.all[i]; c.farm.tame = true; c.hp = c.hp.max(16.0); done(self, format!("{name} vous a adopté : il vous suit désormais")) }
                else { done(self, format!("{name} accepte, mais se méfie encore : recommencez")) } }
            (_, Some("saddle")) if mount(kind) && farm.tame && !farm.saddle => { self.wild.all[i].farm.saddle = true; self.consume_held(); done(self, format!("{name} est sellé : clic droit pour monter, accroupi pour descendre")) }
            (_, Some("saddle")) if mount(kind) && !farm.tame => done(self, format!("{name} n'est pas encore apprivoisé")),
            // Monter une bête sellée, ou faire asseoir (et relever) un loup sans selle.
            _ if mount(kind) && farm.tame && farm.saddle && held.is_none_or(|h| crate::blocks::by_name(h).is_none()) => { self.ride = Some(farm.id); self.wild.all[i].farm.sit = false; done(self, format!("En selle ! Accroupi pour descendre")) }
            (Kind::Wolf, None) if farm.tame => { let s = !farm.sit; self.wild.all[i].farm.sit = s; done(self, if s { "Le loup s'assoit et vous attend".into() } else { "Le loup se relève et vous suit".into() }) }
            (Kind::Horse, None) if farm.tame => { let s = !farm.sit; self.wild.all[i].farm.sit = s; done(self, if s { "Le cheval reste là".into() } else { "Le cheval vous suit de loin".into() }) }
            _ => false,
        }
    }
    /// En selle : la monture va où l'on pousse (devant, côtés, relatifs au regard), au galop en courant ; accroupi, on descend.
    /// Rend vrai tant que l'on chevauche (le joueur ne marche pas alors : il est porté).
    pub(crate) fn ride_step(&mut self, dt: f64, i: crate::player::Input) -> bool {
        let Some(id) = self.ride else { return false };
        let Some(k) = self.wild.by_id(id) else { self.ride = None; return false }; let kind = self.wild.all[k].kind;
        if i.crouch { self.ride = None; let p = self.wild.all[k].p; let side = self.player.yaw + std::f64::consts::FRAC_PI_2; self.player.p = [p[0] + side.sin() * 1.2, p[1] + 0.2, p[2] + side.cos() * 1.2]; self.player.v = [0.0; 3]; self.notes.push("Vous mettez pied à terre".into()); return false; }
        let (sy, cy) = (self.player.yaw.sin(), self.player.yaw.cos()); let dir = [sy * i.z + cy * i.x * -1.0, cy * i.z - sy * i.x * -1.0];
        let Some(p) = self.wild.ride(id, dir, i.sprint, dt, &self.world) else { self.ride = None; return false };
        self.player.p = [p[0], p[1] + seat(kind), p[2]]; self.player.v = [0.0; 3]; self.player.ground = true; self.player.speed = 0.0; true
    }
    /// Les compagnons gardés dans la sauvegarde : espèce, position, sellé, assis, robe.
    pub(crate) fn pets_save(&self) -> String {
        let mut s = String::new();
        for c in self.wild.all.iter().filter(|c| c.farm.tame) { s += &format!("pet {:?} {} {} {} {} {} {}\n", c.kind, c.p[0], c.p[1], c.p[2], c.farm.saddle as u8, c.farm.sit as u8, c.color); }
        s
    }
    pub(crate) fn pet_load(&mut self, v: &[&str]) {
        let kind = match v.first().copied() { Some("Wolf") => Kind::Wolf, Some("Horse") => Kind::Horse, Some("Cow") => Kind::Cow, Some("Sheep") => Kind::Sheep, Some("Pig") => Kind::Pig, Some("Chicken") => Kind::Chicken, _ => return };
        let n = |k: usize| v.get(k).and_then(|x| x.parse::<f64>().ok()).unwrap_or(0.0);
        self.wild.put(kind, [n(1), n(2), n(3)], 0.0, false); if let Some(c) = self.wild.all.last_mut() { c.farm.tame = true; c.farm.saddle = n(4) > 0.5; c.farm.sit = n(5) > 0.5; c.color = n(6) as u8; }
    }
}
