//! Les caravanes entre villages : une fois la semaine, des marchands vont d'un village à ses voisins (ceux qui ont une économie,
//! à moins de quinze cents pas). Chacun vend son surplus à qui en manque — les vivres d'abord, puis les matériaux du chantier —,
//! la caisse de l'acheteur paie celle du vendeur. Les deux journaux le racontent. Quand le joueur est au village, le marchand
//! se voit : il arrive par le chemin du côté de son village, reste à la réserve l'après-midi, et repart avant la nuit.
use crate::economie::{Census, Econ, COIN, LOTS};
use crate::session::Session;

/// Ce qui passe de `a` à `b` : jusqu'à trois lots de chaque bien que `a` a en trop et dont `b` manque, tant que `b` peut payer.
pub fn deal(a: &mut Econ, ca: &Census, b: &mut Econ, cb: &Census) -> (Vec<(&'static str, u32)>, u32) {
    let wants = b.wants(cb); let mut moved = Vec::new(); let mut paid = 0;
    for (id, n, p) in LOTS { if !wants.contains(&id) { continue; } let lots = (a.surplus(id, ca) / n as f64).floor().min(3.0) as u32;
        for _ in 0..lots { if b.have(COIN) < p as f64 { break; } a.take(id, n as f64); b.add(id, n as f64); b.take(COIN, p as f64); a.add(COIN, p as f64); paid += p;
            match moved.iter_mut().find(|m: &&mut (&'static str, u32)| m.0 == id) { Some(m) => m.1 += n, None => moved.push((id, n)) } } }
    (moved, paid)
}
fn list(m: &[(&'static str, u32)]) -> String { m.iter().map(|(k, n)| format!("{} ×{n}", crate::session::item(k).map_or(k.to_string(), |i| i.label.to_lowercase()))).collect::<Vec<_>>().join(", ") }

impl Session {
    /// Le jour des caravanes (le lendemain du marché) : le village où l'on est commerce avec ses voisins.
    pub(crate) fn caravans(&mut self, e: &mut Econ, day: u32) {
        let Some(v) = self.gen.site(e.id.0, e.id.1).map(|s| s.v.clone()) else { return }; let ca = e.census(&v);
        let near: Vec<(i32, i32)> = self.econs.keys().copied().filter(|id| self.gen.site(id.0, id.1).is_some_and(|s| ((s.v.c[0] - v.c[0]) as f64).hypot((s.v.c[2] - v.c[2]) as f64) < 1500.0)).take(3).collect();
        for id in near {
            let Some(w) = self.gen.site(id.0, id.1).map(|s| s.v.clone()) else { continue };
            if self.politics.bond(crate::factions::Faction::of_kind(v.kind), crate::factions::Faction::of_kind(w.kind)) == crate::politique::Bond::Guerre { e.note(day, format!("pas de caravane de {} : la guerre a coupé la route", w.name)); continue; }
            let Some(mut b) = self.econs.remove(&id) else { continue }; let cb = b.census(&w);
            let (sold, got) = deal(e, &ca, &mut b, &cb); let (bought, paid) = deal(&mut b, &cb, e, &ca);
            if !sold.is_empty() || !bought.is_empty() {
                let mut s = format!("une caravane est venue de {}", w.name); if !sold.is_empty() { s += &format!(" ; vendu {} (+{got} pièces)", list(&sold)); } if !bought.is_empty() { s += &format!(" ; acheté {} (−{paid} pièces)", list(&bought)); } e.note(day, s);
                let mut t = format!("caravane vers {}", v.name); if !bought.is_empty() { t += &format!(" ; vendu {}", list(&bought)); } if !sold.is_empty() { t += &format!(" ; acheté {}", list(&sold)); } b.note(day, t);
                if self.town.here.iter().any(|h| h.id == v.id) { self.caravan_arrives(&v, &w); }
            }
            self.econs.insert(id, b);
        }
    }
    /// Le marchand d'un village voisin entre au village par le côté d'où il vient, une caisse dans les bras, et va à la réserve.
    fn caravan_arrives(&mut self, v: &crate::village::Village, from: &crate::village::Village) {
        let Some(k) = self.town.folk.iter().position(|f| f.v == v.id) else { return }; let mut m = self.town.folk[k].clone();
        let (dx, dz) = ((from.c[0] - v.c[0]) as f64, (from.c[2] - v.c[2]) as f64); let l = dx.hypot(dz).max(1.0); let r = v.r as f64 + 6.0;
        let (x, z) = (v.c[0] as f64 + dx / l * r, v.c[2] as f64 + dz / l * r); let y = self.gen.column(x, z).h.floor() + 1.0;
        let to = self.econ.as_ref().and_then(|e| e.store.first()).map(|p| [p[0] as f64 + 0.5, p[1] as f64, p[2] as f64 + 1.8]).unwrap_or([v.c[0] as f64 + 0.5, v.c[1] as f64 + 1.0, v.c[2] as f64 + 0.5]);
        m.merchant(format!("{} · marchand de {}", m.name.split(' ').next().unwrap_or("Le"), from.name), [x, y, z], to); self.town.folk.push(m);
    }
}
