//! La Forge sous la lave : un atelier de forgerons en surface, et, dix-huit blocs dessous, la grande forge que coupe un fleuve
//! de lave. Les énigmes y tiennent toutes à des contrepoids, à des charges, à la lave :
//! - **le pont** : un pont de basalte repose au fond du fleuve de lave ; une dalle le fait monter, tant que quelque chose pèse
//!   dessus (le joueur, ou mieux, un bloc de pierre qu'on y pousse). Sur l'autre rive, un levier le verrouille en haut ;
//! - **le monte-charge** : une plate-forme à contrepoids mène à la galerie haute (et son levier du soufflet). Elle monte quand son
//!   plateau de contrepoids est chargé — il faut donc y amener le seul bloc de pierre de la salle (celui du pont : d'où le
//!   verrou), et le pousser sur le plateau *depuis la plate-forme*, pour monter avec elle. On redescend par une échelle ;
//! - **la coulée** : une fosse pleine de lave, un levier tout au fond. La vanne arrête la coulée vingt secondes : la lave se
//!   retire, on descend à l'échelle, on abaisse le levier, on remonte… avant que la lave ne revienne.
//!
//! Les trois chaînes de la Porte du Foyer tombées, la grille se lève : dans la Fournaise, le Maître de forge prend son marteau.
//! Tout est vérifié par `tests/forge.rs` avec le solveur de chemins (`sanctuaire::reach`).
use crate::blocks::{by_name, Id, B};
use crate::mecanismes::{Guard, Hint, Home, Lift, Mech, Pool, Seal, Wire};
use crate::noise::hash_float;
use crate::session::Session;
use crate::sites::{Place, Plan};

/// Le sol de la forge est à `DEPTH` blocs sous le sol de la cour.
pub const DEPTH: i32 = 18;
/// La grande forge (intérieur u0, u1, w0, w1, voûte) ; le fleuve de lave (w0, w1 ; la lave de -2 à 0).
pub const HALL: (i32, i32, i32, i32, i32) = (-16, 16, 0, 28, 12); pub const RIVER: (i32, i32) = (10, 13);
/// Le pont : ses colonnes (u0, u1), son niveau au fond de la lave ; sa dalle, le bloc de pierre, le levier du verrou.
pub const BRIDGE: (i32, i32) = (-1, 1); pub const BRIDGE_LOW: i32 = -2; pub const BRIDGE_PLATE: [i32; 3] = [3, 0, 8]; pub const BLOCK: [i32; 3] = [6, 1, 6]; pub const LOCK: [i32; 3] = [2, 1, 15];
/// Le monte-charge : ses colonnes (u0, u1, w0, w1), sa hauteur ; son plateau de contrepoids ; la galerie (u0, u1, w0, w1, sol) ;
/// le levier du soufflet ; l'échelle de la descente (u, w, du bas, au haut).
pub const LIFT: (i32, i32, i32, i32) = (10, 11, 19, 20); pub const LIFT_UP: i32 = 8; pub const TRAY: [i32; 3] = [8, 0, 20];
pub const GALLERY: (i32, i32, i32, i32, i32) = (12, 16, 15, 26, 8); pub const BELLOWS: [i32; 3] = [16, 9, 25]; pub const LADDER: (i32, i32, i32, i32) = (16, 15, 4, 8);
/// La fosse de la coulée (u0, u1, w0, w1, fond) ; la vanne ; le levier du fond ; l'échelle (u, w) ; le temps que tient la vanne.
pub const PIT: (i32, i32, i32, i32, i32) = (-13, -10, 20, 23, -6); pub const VALVE: [i32; 3] = [-9, 1, 19]; pub const DEEP: [i32; 3] = [-13, -6, 23]; pub const PIT_LADDER: (i32, i32) = (-11, 20); pub const VALVE_T: f64 = 20.0;
/// La Porte du Foyer : ses chaînes (w), sa grille (w) ; la Fournaise (intérieur u0, u1, w0, w1, voûte) ; le Maître de forge ; le coffre.
pub const CHAINS_W: i32 = 28; pub const GATE_W: i32 = 29; pub const FURNACE: (i32, i32, i32, i32, i32) = (-9, 9, 30, 37, 8);
pub const BOSS: &str = "maitre_forge"; pub const BOSS_AT: [f64; 3] = [0.5, 1.0, 33.5]; pub const CHEST: [i32; 3] = [0, 2, 37];

struct F<'a> { p: &'a mut Plan, x: i32, f: i32, z: i32 }
impl F<'_> {
    fn at(&self, u: i32, v: i32, w: i32) -> (i32, i32, i32) { (self.x + u, self.f + v, self.z + w) }
    fn n(&mut self, u: i32, v: i32, w: i32, n: &'static str) { let (x, y, z) = self.at(u, v, w); self.p.n(x, y, z, n); }
    fn t(&mut self, u: i32, v: i32, w: i32, n: &'static str, dx: i32, dz: i32) { let (x, y, z) = self.at(u, v, w); self.p.turn(x, y, z, n, dx, dz); }
    fn air(&mut self, u: i32, v: i32, w: i32) { let (x, y, z) = self.at(u, v, w); self.p.air(x, y, z); }
    fn h(&self, u: i32, v: i32, w: i32, k: i32) -> f64 { hash_float(self.x + u + 401 * v, self.z + w, 9157, k) }
    /// Les murs : du basalte, un rang de brique sombre tous les quatre blocs, quelques veines de magma et d'obsidienne.
    fn rock(&self, u: i32, v: i32, w: i32) -> &'static str { let h = self.h(u, v, w, 1); if h < 0.025 { "magma" } else if v.rem_euclid(4) == 1 && v > 0 { "bricks_dark" } else if h < 0.12 { "obsidian" } else { "basalt" } }
    fn floor(&self, u: i32, w: i32) -> &'static str { let h = self.h(u, 0, w, 2); if h < 0.6 { "stone_bricks_dark" } else if h < 0.9 { "basalt" } else { "cobblestone" } }
    /// Une boîte pleine (murs, sol, voûte), puis l'air de son intérieur.
    fn room(&mut self, u0: i32, u1: i32, w0: i32, w1: i32, v1: i32) {
        for u in u0 - 1..=u1 + 1 { for w in w0 - 1..=w1 + 1 { for v in -1..=v1 + 1 { let inside = u >= u0 && u <= u1 && w >= w0 && w <= w1 && v >= 1 && v <= v1;
            if inside { self.air(u, v, w); } else if v <= 0 { let f = if v == 0 { self.floor(u, w) } else { "basalt" }; self.n(u, v, w, f); } else { let s = self.rock(u, v, w); self.n(u, v, w, s); } } } }
    }
}

/// Pose la forge dans le plan : `o` est le sol de la cour, au centre de l'atelier.
pub(crate) fn carve(p: &mut Plan, o: [i32; 3]) {
    let mut c = F { p, x: o[0], f: o[1] - DEPTH, z: o[2] }; let s = DEPTH;
    // la grande forge ; le fleuve de lave qui la coupe, son lit ; le pont au fond
    let (u0, u1, w0, w1, hv) = HALL; c.room(u0, u1, w0, w1, hv);
    for u in u0..=u1 { for w in RIVER.0..=RIVER.1 { c.n(u, -3, w, "basalt"); for v in -2..=0 { c.n(u, v, w, "lava"); } } for v in -3..=0 { for w in [RIVER.0 - 1, RIVER.1 + 1] { if v < 0 { c.n(u, v, w, "basalt"); } } } }
    for v in -3..=0 { for u in [u0 - 1, u1 + 1] { for w in RIVER.0..=RIVER.1 { c.n(u, v, w, "basalt"); } } }
    for u in BRIDGE.0..=BRIDGE.1 { for w in RIVER.0..=RIVER.1 { c.n(u, BRIDGE_LOW, w, "basalt"); } }
    c.n(BRIDGE_PLATE[0], BRIDGE_PLATE[1], BRIDGE_PLATE[2], "pressure_plate"); c.n(BLOCK[0], BLOCK[1], BLOCK[2], "push_block");
    for v in 1..=2 { c.n(LOCK[0] + 1, v, LOCK[2], "basalt"); } c.n(LOCK[0] + 1, 3, LOCK[2], "lantern"); c.t(LOCK[0], LOCK[1], LOCK[2], "lever", 1, 0);
    for (u, w) in [(-2, 9), (2, 9), (-2, 14), (2, 14)] { c.n(u, 1, w, "brazier_stand"); }
    // le monte-charge : sa plate-forme (au ras du sol), son plateau de contrepoids sous ses chaînes, la galerie haute, l'échelle
    for u in LIFT.0..=LIFT.1 { for w in LIFT.2..=LIFT.3 { c.n(u, 0, w, "bricks_dark"); for v in [-1, -2] { c.n(u, v, w, "basalt"); } } }
    c.n(TRAY[0], TRAY[1], TRAY[2], "pressure_plate"); for v in 9..=hv { c.n(TRAY[0], v, TRAY[2], "chain"); }
    let (g0, g1, gw0, gw1, gv) = GALLERY; for u in g0..=g1 { for w in gw0..=gw1 { c.n(u, gv, w, if (u + w) % 3 == 0 { "bricks_dark" } else { "stone_bricks_dark" }); } }
    for w in gw0..=gw1 { if w < LIFT.2 || w > LIFT.3 { c.n(g0, gv + 1, w, "stone_bricks_slab"); } } for w in [gw0, gw1] { for v in 1..gv { c.n(g0, v, w, "basalt"); } }
    c.t(BELLOWS[0], BELLOWS[1], BELLOWS[2], "lever", 1, 0); c.t(15, 9, 22, "anvil", 1, 0); c.n(14, 9, 24, "coal_heap"); c.n(16, 9, 18, "barrel");
    let (lu, lw, l0, l1) = LADDER; for v in l0..=l1 { c.t(lu, v, lw, "ladder", 1, 0); }
    // la coulée : une fosse pleine de lave, son échelle, le levier du fond, la vanne sur son poteau
    let (p0, p1, q0, q1, pb) = PIT; for u in p0 - 1..=p1 + 1 { for w in q0 - 1..=q1 + 1 { for v in pb - 1..=0 { let inside = u >= p0 && u <= p1 && w >= q0 && w <= q1 && v >= pb; c.n(u, v, w, if inside { "lava" } else if v == 0 { "stone_bricks_dark" } else { "basalt" }); } } }
    for v in pb..=0 { c.t(PIT_LADDER.0, v, PIT_LADDER.1, "ladder", 0, -1); } c.t(DEEP[0], DEEP[1], DEEP[2], "lever", 0, 1);
    c.n(VALVE[0], 1, VALVE[2] - 1, "basalt"); c.n(VALVE[0], 2, VALVE[2] - 1, "basalt"); c.t(VALVE[0], VALVE[1], VALVE[2], "lever", 0, -1); for w in q0..=q1 { c.n(p0 - 1, 1, w, "iron_bars"); }
    // la forge proprement dite : des fourneaux le long du mur de l'ouest, des enclumes, du charbon, des chaînes et des lanternes
    for w in [3, 5, 7] { c.t(u0, 1, w, "furnace_lit", -1, 0); c.t(u0, 2, w, "furnace_lit", -1, 0); } for (u, w) in [(-11, 4), (-9, 7), (-12, 17), (6, 25)] { c.t(u, 1, w, "anvil", 1, 0); }
    for (u, w) in [(-14, 1), (-14, 8), (14, 1), (-15, 27), (13, 27)] { c.n(u, 1, w, "coal_heap"); } for (u, w) in [(-6, 2), (12, 4), (-3, 26)] { c.n(u, 1, w, "barrel"); } c.n(-7, 1, 26, "cauldron"); c.t(14, 1, 7, "grindstone", 1, 0);
    for (u, w) in [(-8, 4), (8, 3), (-6, 17), (4, 22), (-12, 26), (0, 20)] { for v in hv - 2..=hv { c.n(u, v, w, "chain"); } c.n(u, hv - 3, w, "lantern"); }
    for (u, w) in [(-16, 12), (16, 12)] { c.n(u, 1, w, "brazier_stand"); }
    // la Porte du Foyer : trois chaînes devant une grille, deux brasiers
    for u in -1..=1 { for v in 1..=3 { c.n(u, v, CHAINS_W, "great_chain"); c.n(u, v, GATE_W, "iron_bars"); } } for u in [-3, 3] { c.n(u, 1, CHAINS_W, "brazier_stand"); }
    // la Fournaise : des rigoles de lave derrière des grilles, l'autel du maître, ses enclumes, ses fourneaux
    let (m0, m1, n0, n1, mv) = FURNACE; c.room(m0, m1, n0, n1, mv); for u in -1..=1 { for v in 1..=3 { c.n(u, v, GATE_W, "iron_bars"); } }
    for side in [-1, 1] { for w in n0 + 1..=n1 - 1 { c.n(9 * side, 0, w, "lava"); c.n(8 * side, 1, w, "iron_bars"); } }
    c.t(CHEST[0], 1, CHEST[2], "altar", 0, 1); c.n(CHEST[0], CHEST[1], CHEST[2], "chest"); for u in [-3, 3] { c.t(u, 1, 35, "anvil", 1, 0); } for u in [-6, 6] { c.t(u, 1, n1, "furnace_lit", 0, 1); c.t(u, 2, n1, "furnace_lit", 0, 1); }
    for (u, w) in [(-5, 31), (5, 31)] { for v in mv - 2..=mv { c.n(u, v, w, "chain"); } c.n(u, mv - 3, w, "lantern"); }
    // l'escalier, de l'atelier à la forge : dix-huit marches, trois de large
    for k in 0..DEPTH { let w = -19 + k; let top = DEPTH - 1 - k; let open_top = if w <= -15 { s } else { top + 4 };
        for u in -2..=2i32 { if u.abs() == 2 { for v in top - 1..=open_top.max(top + 5) { if v < s { c.n(u, v, w, if v == top - 1 { "basalt" } else { "bricks_dark" }); } } continue; }
            c.n(u, top - 1, w, "basalt"); c.t(u, top, w, "stone_bricks_stairs", 0, -1); for v in top + 1..=open_top { c.air(u, v, w); } if w > -15 { c.n(u, top + 5, w, "basalt"); } }
        if k % 4 == 2 && w > -15 { c.t(-1, top + 2, w, "wall_sconce", -1, 0); } }
    for u in -1..=1 { for v in 1..=4 { c.air(u, v, w0 - 1); } }

    // En surface : l'atelier (brique sombre, angles de basalte), sa grande cheminée, la cour et ses enclumes
    for u in -6..=6i32 { for w in -24..=-14i32 { c.n(u, s, w, if u.abs() <= 2 && (-19..=-15).contains(&w) { "basalt" } else { "stone_bricks" });
        let edge = u.abs() == 6 || w == -24 || w == -14; for v in s + 1..=s + 5 { if edge { c.n(u, v, w, if u.abs() == 6 && (w == -24 || w == -14) { "basalt" } else { "bricks_dark" }); } else { c.air(u, v, w); } } c.n(u, s + 6, w, "stone_bricks_dark"); } }
    for u in -1..=1 { for w in -19..=-15 { c.air(u, s, w); } for v in s + 1..=s + 3 { c.air(u, v, -24); } } for u in [-2, 2] { for w in -19..=-15 { c.n(u, s + 1, w, "stone_bricks_slab"); } }
    for u in 3..=4 { for w in -17..=-16 { for v in s + 1..=s + 13 { c.n(u, v, w, if v % 4 == 0 { "basalt" } else { "bricks_dark" }); } } } c.n(3, s + 14, -17, "campfire");
    c.t(3, s + 1, -18, "furnace_lit", 0, 1); c.t(-4, s + 1, -21, "anvil", 1, 0); c.n(-5, s + 1, -16, "coal_heap"); c.n(5, s + 1, -22, "barrel"); c.n(4, s + 1, -23, "crate"); c.t(-5, s + 1, -23, "grindstone", 1, 0);
    for (u, w, dx, dz) in [(-5, -19, -1, 0), (5, -20, 1, 0)] { c.t(u, s + 3, w, "wall_sconce", dx, dz); }
    for u in -9..=9 { for w in -32..=-25 { c.n(u, s, w, if c.h(u, s, w, 3) < 0.3 { "gravel" } else { "cobblestone" }); } }
    for (u, w) in [(-6, -28), (6, -27)] { c.t(u, s + 1, w, "anvil", 1, 0); } c.t(-7, s + 1, -31, "trough", 1, 0); c.t(7, s + 1, -30, "cart", 0, 1); c.n(8, s + 1, -26, "log_pile"); c.n(-8, s + 1, -26, "crate"); c.n(-8, s + 2, -26, "crate");
    for u in [-9, 9] { for v in s + 1..=s + 3 { c.n(u, v, -32, "basalt"); } c.n(u, s + 4, -32, "lantern"); }
}

/// Les mécanismes de la forge bâtie en `o`, préfixés de `tag`.
pub fn mechs(o: [i32; 3], tag: &str) -> Mech {
    let f = o[1] - DEPTH; let at = |q: [i32; 3]| [o[0] + q[0], f + q[1], o[2] + q[2]]; let name = |n: &str| format!("{tag}{n}");
    let chain = by_name("great_chain").unwrap_or(0); let iron = by_name("iron_bars").unwrap_or(0); let basalt = by_name("basalt").unwrap_or(0); let bricks = by_name("bricks_dark").unwrap_or(0);
    let chain_of = |u: i32| (1..=3).map(|v| (at([u, v, CHAINS_W]), chain)).collect::<Vec<_>>();
    let mut m = Mech::default();
    // le pont : la dalle le lève, le verrou le tient
    m.wires.push(Wire { name: name("pont"), inputs: vec![at(LOCK)], latch: true, doors: chain_of(-1), ..Default::default() });
    m.lifts.push(Lift { cols: (BRIDGE.0..=BRIDGE.1).flat_map(|u| (RIVER.0..=RIVER.1).map(move |w| [o[0] + u, o[2] + w])).collect(), base: f + BRIDGE_LOW, per: -BRIDGE_LOW, max: 1, plates: vec![at(BRIDGE_PLATE)], lock: name("pont"), block: basalt, fill: B::LAVA, level: f + BRIDGE_LOW, t: 0.0 });
    // le monte-charge, le soufflet
    m.lifts.push(Lift { cols: (LIFT.0..=LIFT.1).flat_map(|u| (LIFT.2..=LIFT.3).map(move |w| [o[0] + u, o[2] + w])).collect(), base: f, per: LIFT_UP, max: 1, plates: vec![at(TRAY)], lock: String::new(), block: bricks, fill: 0, level: f, t: 0.0 });
    m.wires.push(Wire { name: name("soufflet"), inputs: vec![at(BELLOWS)], latch: true, doors: chain_of(0), ..Default::default() });
    // la coulée : la vanne tient vingt secondes ; la lave revient ensuite
    m.wires.push(Wire { name: name("vanne"), inputs: vec![at(VALVE)], hold: VALVE_T, ..Default::default() });
    m.wires.push(Wire { name: name("coulee"), needs: vec![name("vanne")], not: true, ..Default::default() });
    let (p0, p1, q0, q1, pb) = PIT; let cols: Vec<[i32; 3]> = (p0..=p1).flat_map(|u| (q0..=q1).map(move |w| [o[0] + u, o[2] + w, f + pb])).collect();
    m.pools.push(Pool { cols, top: f, by: name("coulee"), level: f, t: 0.0, liquid: B::LAVA });
    m.wires.push(Wire { name: name("fond"), inputs: vec![at(DEEP)], latch: true, doors: chain_of(1), ..Default::default() });
    // la Porte du Foyer, le Maître de forge
    let gate: Vec<([i32; 3], Id)> = (-1..=1).flat_map(|u| (1..=3).map(move |v| [u, v, GATE_W])).map(|q| (at(q), iron)).collect();
    m.wires.push(Wire { name: name("foyer"), needs: vec![name("pont"), name("soufflet"), name("fond")], latch: true, doors: gate.clone(), ..Default::default() });
    m.guards.push(Guard { min: at([FURNACE.0, 1, FURNACE.2]), max: at([FURNACE.1, FURNACE.4, FURNACE.3]), species: BOSS.into(), at: [o[0] as f64 + BOSS_AT[0], f as f64 + BOSS_AT[1], o[2] as f64 + BOSS_AT[2]], doors: gate, state: 0 });
    // le bloc de pierre revient à sa place si l'on quitte la forge avant d'avoir monté le monte-charge
    m.homes.push(Home { min: at([HALL.0, PIT.4, HALL.2]), max: at([HALL.1, HALL.4, HALL.3]), homes: vec![at(BLOCK)], keep: name("soufflet") });
    m.vaults.push(at(CHEST)); m.seals.push(Seal { min: at([-18, -8, -20]), max: at([18, DEPTH - 1, 38]), until: String::new() });
    let hint = |m: &mut Mech, a: [i32; 3], b: [i32; 3], t: &str| m.hints.push(Hint { min: at(a), max: at(b), text: t.into(), shown: false, xp: 0.0 });
    hint(&mut m, [-3, 1, 0], [3, 4, 3], "La Forge sous la lave. Au sud, la Porte du Foyer est tenue par trois chaînes : celle du pont, celle du soufflet (dans la galerie haute), celle de la coulée. Un fleuve de lave coupe la salle.");
    hint(&mut m, [0, 1, 6], [6, 3, 9], "Le pont de basalte repose au fond de la lave. Cette dalle le fait monter… tant que quelque chose pèse dessus.");
    hint(&mut m, [-1, 1, 14], [4, 3, 16], "Un levier de verrou, sur cette rive : il bloque le pont en position haute.");
    hint(&mut m, [6, 1, 17], [11, 3, 22], "Un monte-charge à contrepoids : il monte quand son plateau est chargé. Il faudrait un bloc de pierre… et être sur la plate-forme quand elle s'élève.");
    hint(&mut m, [-14, 1, 17], [-8, 3, 24], "Une coulée de lave remplit la fosse ; au fond, un levier. La vanne arrête la coulée, vingt secondes seulement.");
    m
}

impl Session {
    /// Bâtit ici la forge (monde déjà chargé : essais, captures) et branche ses mécanismes. `o` : le sol de la cour.
    pub fn build_forge(&mut self, o: [i32; 3]) {
        let mut p = Plan::new(); carve(&mut p, o);
        let e: Vec<(i32, i32, i32, Id)> = p.m.iter().filter(|(_, b)| **b != -2).map(|(&(x, y, z), &b)| (x, y, z, if b < 0 { 0 } else { b as Id })).collect();
        self.world.edit_many(&e); let tag = format!("f{}:{}:", o[0], o[2]); if !self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { self.mech.absorb(mechs(o, &tag)); } self.dirty = true;
    }
    /// Une forge de la carte, qu'on approche : ses mécanismes se branchent, une fois.
    pub(crate) fn forge_wake(&mut self, pl: &Place) { let tag = format!("f{}:{}:", pl.x, pl.z); if self.mech.wires.iter().any(|w| w.name.starts_with(&tag)) { return; } self.mech.absorb(mechs([pl.x, pl.y, pl.z], &tag)); self.dirty = true; }
}
