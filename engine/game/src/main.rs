//! A Living World, moteur final. La partie vit dans alw-core ; ici, la fenêtre, les entrées et l'affichage.
//! Si quelque chose échoue, le détail est écrit dans alw-erreur.txt à côté du programme.
mod font_data;
mod gpu;
mod hud;
mod needs;
mod sound;
use alw_core::{actors::Models, audio::{Mood, SoundScape}, gen::WorldGen, player::Input, session::{Actions, Session}, stream::{Preset, HIGH, ULTRA}, view, weather::{self, Kind, KINDS}};
use std::collections::HashSet;
use winit::keyboard::{Key, NamedKey};
use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
use std::sync::Arc;
use std::time::Instant;
use winit::{application::ApplicationHandler, event::{DeviceEvent, DeviceId, ElementState, MouseButton, MouseScrollDelta, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{CursorGrabMode, Window, WindowId}};

const SEED: i32 = 20261104;
fn beside_exe(name: &str) -> std::path::PathBuf { std::env::current_exe().ok().and_then(|e| e.parent().map(|d| d.join(name))).unwrap_or_else(|| name.into()) }
pub fn report(msg: &str) { eprintln!("{msg}"); let _ = std::fs::write(beside_exe("alw-erreur.txt"), msg); }
/// Un point de départ au sec : sur un chemin si possible, en spirale autour de l'origine.
fn spawn_point(seed: i32) -> [f64; 3] {
    let g = WorldGen::new(seed); let mut best = [0.5, 80.0, 0.5];
    if g.flat { return [6.5, alw_core::gen::FLAT_Y as f64 + 1.0, 2.5]; } // monde plat : devant la galerie des bâtiments
    for r in 0..40 { for k in 0..16 {
        let a = k as f64 / 16.0 * std::f64::consts::TAU; let (x, z) = ((a.cos() * r as f64 * 6.0).floor() + 0.5, (a.sin() * r as f64 * 6.0).floor() + 0.5);
        let c = g.column(x, z);
        if c.water == 0.0 { best = [x, c.h + 3.0, z]; if c.path > 0.5 { return best; } }
    } }
    best
}
fn draw(gpu: &mut gpu::Gpu, md: &Models, s: &Session, time: f64, w: f32, h: f32, msg: Option<&str>, mouse: [f32; 2], menu: &hud::Menu, info: &[String]) -> usize {
    let (proj, vm) = (view::perspective(menu.fov() * (1.0 - 0.12 * s.anim.bow), w as f64 / h as f64, 0.08), view::look_dir(s.eye, s.look)); let vp = view::mul(&proj, &vm);
    let ui = hud::Ui::build(s, w, h, msg, mouse, menu, info); let mut draws = s.draws(md, &vm); if menu.title.is_some() { draws.retain(|d| !d.view); } /* pas de main tendue derrière l'écran titre */ let mut sky = view::sky_at(s.hour); if let Some(f) = &s.falling { gpu.set_fall(s.fall_serial, &f.data); }
    gpu.lights = s.lights(s.eye, gpu::LIGHTS);
    let mut env = weather::apply(&mut sky, s.weather.state, s.covered(s.eye)); env.bright = hud::bright(menu.vol[3]); env.under = s.under(); let night = sky.night.clamp(0.0, 1.0); env.flies = s.deep as f32 * (0.55 + 0.9 * night); // la nuit, la direction est celle de la lune : elle ne dit pas l'heure
    // Taïga : ciel froid, soleil pâle, neige qui tombe à la place de la pluie.
    let tg = s.taiga as f32; if tg > 0.01 { for k in 0..3 { sky.top[k] += ([0.5, 0.64, 0.82][k] * (0.3 + 0.7 * (1.0 - night)) - sky.top[k]) * 0.35 * tg; sky.hor[k] += ([0.82, 0.88, 0.95][k] * (0.3 + 0.7 * (1.0 - night)) - sky.hor[k]) * 0.4 * tg; sky.col[k] *= 1.0 + ([0.12, 0.02, -0.16][k]) * tg * (1.0 - night); }
        // Soleil d'hiver : bas sur l'horizon, doré, aux ombres longues ; brume au fond des vallées le matin ; aurores la nuit.
        if sky.dir[1] > 0.05 { sky.dir[1] = 0.05 + (sky.dir[1] - 0.05) * (1.0 - 0.5 * tg); let l = (sky.dir[0] * sky.dir[0] + sky.dir[1] * sky.dir[1] + sky.dir[2] * sky.dir[2]).sqrt(); for v in &mut sky.dir { *v /= l; } }
        let h = s.hour as f32; env.valley = tg * ((h - 4.5) / 2.0).clamp(0.0, 1.0) * (1.0 - ((h - 9.0) / 2.5).clamp(0.0, 1.0)) * 0.6; env.aurora = tg * night * (1.0 - env.cloud).clamp(0.3, 1.0); env.snow = tg * (0.3 * env.cloud + 0.7 * s.weather.state.rain as f32); /* ciel dégagé : pas de neige qui tombe */ env.rain *= 1.0 - tg; } // Steppe : lumière chaude et dorée, horizon voilé de poussière.
    let st = s.steppe as f32 * (1.0 - night); if st > 0.01 { for k in 0..3 { sky.hor[k] += ([0.96, 0.86, 0.68][k] - sky.hor[k]) * 0.3 * st; sky.col[k] *= 1.0 + ([0.07, 0.02, -0.08][k]) * st; } env.mist += 0.0012 * st; }
    // Désert : ciel pâle et brûlant, horizon blanchi par la chaleur, lumière crue.
    let de = s.desert as f32 * (1.0 - night); if de > 0.01 { for k in 0..3 { sky.hor[k] += ([1.0, 0.92, 0.78][k] * 1.15 - sky.hor[k]) * 0.35 * de; sky.top[k] += ([0.42, 0.56, 0.78][k] - sky.top[k]) * 0.25 * de; sky.col[k] *= 1.0 + ([0.1, 0.05, -0.04][k]) * de; } env.mist += 0.0016 * de; env.rain *= 1.0 - s.desert as f32; }
    // Hautes terres : ciel bas et gris-bleu, lumière froide, brume qui traîne sur la lande.
    let hl = s.highlands as f32; if hl > 0.01 { for k in 0..3 { sky.top[k] += ([0.4, 0.5, 0.62][k] * (0.3 + 0.7 * (1.0 - night)) - sky.top[k]) * 0.3 * hl; sky.hor[k] += ([0.72, 0.76, 0.8][k] * (0.3 + 0.7 * (1.0 - night)) - sky.hor[k]) * 0.35 * hl; sky.col[k] *= 1.0 + ([-0.05, -0.02, 0.03][k]) * hl; } env.mist += 0.003 * hl; env.fog_k += 1.2 * hl; }
    // Hautes montagnes : ciel d'altitude d'un bleu profond, air limpide, neige à la place de la pluie, brume au fond des vallons le matin.
    let mtn = s.mountains as f32; if mtn > 0.01 { for k in 0..3 { sky.top[k] += ([0.24, 0.4, 0.74][k] * (0.3 + 0.7 * (1.0 - night)) - sky.top[k]) * 0.4 * mtn; sky.hor[k] += ([0.76, 0.84, 0.94][k] * (0.3 + 0.7 * (1.0 - night)) - sky.hor[k]) * 0.3 * mtn; sky.col[k] *= 1.0 + ([0.04, 0.03, 0.06][k]) * mtn * (1.0 - night); }
        let h = s.hour as f32; env.valley = env.valley.max(mtn * ((h - 4.5) / 2.0).clamp(0.0, 1.0) * (1.0 - ((h - 9.5) / 2.5).clamp(0.0, 1.0)) * 0.5); env.snow = env.snow.max(mtn * (0.25 * env.cloud + 0.75 * s.weather.state.rain as f32)); env.rain *= 1.0 - mtn; env.fog_k *= 1.0 - 0.3 * mtn; }
    env.bright *= 1.0 - 0.42 * s.deep as f32; env.fog_k += 2.6 * s.deep as f32; env.mist += 0.0035 * s.deep as f32; for v in &mut sky.top { *v *= 1.0 - 0.3 * s.deep as f32; }
    gpu.render(&vp, &vm, &proj, s.eye, time, &sky, env, s.target.map(|t| ([t.x, t.y, t.z], if alw_core::blocks::is_deco(t.id) { 0.0 } else { s.progress() as f32 })) /* fissures sur les blocs pleins seulement */, s.ghost(), &s.ghost_boxes(), &draws, &ui)
}
/// Le nom d'une touche telle qu'elle est écrite dessus (disposition du clavier respectée, sans Maj) : c'est ce qu'on attribue aux actions.
fn key_name(e: &winit::event::KeyEvent) -> String {
    match e.key_without_modifiers() {
        Key::Character(c) => c.to_uppercase(),
        Key::Named(n) => match n { NamedKey::Space => "Espace", NamedKey::Shift => "Maj", NamedKey::Control => "Ctrl", NamedKey::Tab => "Tab", NamedKey::Alt => "Alt", NamedKey::CapsLock => "Verr.Maj",
            NamedKey::ArrowUp => "Haut", NamedKey::ArrowDown => "Bas", NamedKey::ArrowLeft => "Gauche", NamedKey::ArrowRight => "Droite", NamedKey::Enter => "Entrée", _ => "" }.to_string(),
        _ => String::new(),
    }
}
/// Les mondes vivent dans le dossier « mondes », un fichier par monde, nommé comme lui.
fn worlds_dir() -> std::path::PathBuf { beside_exe("mondes") }
fn world_path(name: &str) -> std::path::PathBuf { worlds_dir().join(format!("{name}.alw")) }
/// L'ancien monde unique (monde.alw) devient « Premier monde » au premier lancement de cette version.
fn migrate_worlds() {
    let _ = std::fs::create_dir_all(worlds_dir()); let old = beside_exe("monde.alw");
    if old.exists() && !world_path("Premier monde").exists() { let _ = std::fs::rename(&old, world_path("Premier monde")); let _ = std::fs::write(worlds_dir().join("dernier.txt"), "Premier monde"); }
}
fn ago(t: std::time::SystemTime) -> String {
    let s = std::time::SystemTime::now().duration_since(t).map_or(0, |d| d.as_secs());
    if s < 3600 { format!("il y a {} min", (s / 60).max(1)) } else if s < 86400 { format!("il y a {} h", s / 3600) } else { let d = s / 86400; format!("il y a {d} jour{}", if d > 1 { "s" } else { "" }) }
}
/// Les mondes enregistrés, le plus récemment joué en tête.
fn list_worlds() -> Vec<hud::WorldInfo> {
    let mut o: Vec<(std::time::SystemTime, hud::WorldInfo)> = Vec::new();
    for e in std::fs::read_dir(worlds_dir()).into_iter().flatten().flatten() {
        let p = e.path(); if p.extension().and_then(|x| x.to_str()) != Some("alw") { continue; }
        let Some(name) = p.file_stem().and_then(|x| x.to_str()).map(str::to_string) else { continue }; let Ok(t) = std::fs::read_to_string(&p) else { continue };
        let field = |k: &str| t.lines().find_map(|l| l.strip_prefix(k).map(|v| v.trim().to_string()));
        let when = e.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        o.push((when, hud::WorldInfo { name, seed: field("seed ").and_then(|v| v.parse().ok()).unwrap_or(SEED), profile: field("profile ").and_then(|v| v.parse().ok()).unwrap_or(1), hours: field("hours ").and_then(|v| v.parse().ok()).unwrap_or(0.0), age: ago(when) }));
    }
    o.sort_by(|a, b| b.0.cmp(&a.0)); o.into_iter().map(|x| x.1).collect()
}
/// Un nom de fichier sûr, et libre : « Mon monde », « Mon monde 2 »…
fn fresh_name(want: &str) -> String {
    let base: String = want.chars().filter(|c| !"/\\:*?\"<>|".contains(*c) && !c.is_control()).collect::<String>().trim().chars().take(32).collect();
    let base = if base.is_empty() || base == "dernier" { "Mon monde".to_string() } else { base };
    let mut n = base.clone(); let mut i = 2; while world_path(&n).exists() { n = format!("{base} {i}"); i += 1; } n
}
/// La graine d'un nouveau monde : le nombre tapé, sinon le mot tapé haché, sinon le hasard.
fn seed_of(t: &str) -> i32 {
    let t = t.trim(); if let Ok(v) = t.parse::<i32>() { return v; }
    if !t.is_empty() { let mut h: u32 = 2166136261; for b in t.bytes() { h = (h ^ b as u32).wrapping_mul(16777619); } return h as i32; }
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(1, |d| d.subsec_nanos() as i32 ^ d.as_secs() as i32)
}
/// Réglages enregistrés ; un fichier plus ancien garde ses valeurs et prend les nouvelles par défaut.
fn read_settings() -> hud::Menu {
    let mut m = hud::Menu::new();
    let Ok(t) = std::fs::read_to_string(beside_exe("reglages.alw")) else { return m }; let first = t.lines().next().unwrap_or(""); let v: Vec<f32> = first.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    // Seconde ligne : les touches, deux par action, « - » pour une case vide.
    if let Some(l) = t.lines().nth(1) { let k: Vec<&str> = l.split_whitespace().collect(); if k.len() % 2 == 0 && k.len() <= hud::ACTIONS.len() * 2 { for (i, b) in m.binds.iter_mut().enumerate().take(k.len() / 2) { for sl in 0..2 { b[sl] = if k[i * 2 + sl] == "-" { String::new() } else { k[i * 2 + sl].to_string() }; } } } }
    if v.len() >= 5 { m.vol[0] = v[0].clamp(0.0, 1.0); m.vol[1] = v[1].clamp(0.0, 1.0); m.vol[2] = v[2].clamp(0.0, 1.0); m.vol[3] = v.get(5).copied().unwrap_or(0.5).clamp(0.0, 1.0); m.weather = (v[3] as usize).min(hud::WEATHERS.len() - 1); m.mood = (v[4] as usize).min(hud::MOODS.len() - 1); m.quality = v.get(6).map_or(0, |q| (*q as usize).min(1));
        // Indicateurs de survie : mode d'affichage, animations réduites, marqueur de la dépouille.
        if let Some(x) = v.get(7) { m.hud = (*x as usize).min(needs::MODES.len() - 1); } if let Some(x) = v.get(8) { m.reduce = *x > 0.5; } if let Some(x) = v.get(9) { m.marker = *x > 0.5; } if let Some(x) = v.get(10) { m.tips = *x > 0.5; } if let Some(x) = v.get(11) { m.vol[4] = x.clamp(0.0, 1.0); } if let Some(x) = v.get(12) { m.vol[5] = x.clamp(0.0, 1.0); } if let Some(x) = v.get(13) { m.autostep = *x > 0.5; } }
    m
}
/// Panneau F3 : ce qu'il faut pour comprendre où l'on est et comment tourne le jeu.
fn debug_lines(s: &Session, fps: f64, tris: usize, quality: &str) -> Vec<String> {
    let (p, m) = (s.player.p, s.mix()); let mut order: Vec<usize> = (0..alw_core::biome::COUNT).collect(); order.sort_by(|a, b| m.w[*b].total_cmp(&m.w[*a]));
    let t = s.target.map_or("rien".to_string(), |t| format!("{} ({}, {}, {})", alw_core::blocks::BLOCKS[t.id as usize].name, t.x, t.y, t.z));
    vec![format!("{fps:.0} FPS  ·  {:.1} M triangles  ·  qualité {quality}", tris as f64 / 1e6), format!("Graine {}", s.seed), format!("Position {:.1}  {:.1}  {:.1}", p[0], p[1], p[2]), format!("Chunk {} {}  ·  {} chargés  ·  {} en attente", (p[0] / 32.0).floor(), (p[2] / 32.0).floor(), s.world.loaded(), s.world.waiting()),
        format!("Biome {}", alw_core::biome::NAMES[order[0]]), format!("Mélange {} {:.0} %, {} {:.0} %", alw_core::biome::NAMES[order[0]], m.w[order[0]] * 100.0, alw_core::biome::NAMES[order[1]], m.w[order[1]] * 100.0), format!("Climat : température {:.2}, humidité {:.2}", m.temp, m.wet),
        format!("Heure {:02}:{:02}  ·  météo {}", s.hour as u32, (s.hour.fract() * 60.0) as u32, s.weather.kind.name()), format!("Visé : {t}"), format!("Animaux : {} cerfs, {} oiseaux", s.fauna.deer.len(), s.fauna.birds.len()),
        format!("Vie {:.0}  ·  faim {:.0}  ·  soif {:.0}  ·  endurance {:.0}  ·  souffle {:.0}", s.vit[0], s.vit[1], s.vit[2], s.stamina, s.air)]
}
struct Game { /** la grande carte qu'on fait glisser : dernière position de la souris */ map_drag: Option<[f32; 2]>, held: HashSet<String>, preset: Preset, threads: usize, world: String, debug: bool, fps: f64, menu: hud::Menu, drag: Option<usize>, half: Option<usize>, window: Arc<Window>, gpu: gpu::Gpu, md: Models, mouse: [f32; 2], s: Session, keys: HashSet<KeyCode>, grabbed: bool, time: f64, run_time: bool, lmb: bool, rmb: bool, place_t: f64, msg: Option<(String, f64)>, save_t: f64, last: Instant, frames: u32, fps_t: f64, tris: usize }
#[derive(Default)]
struct App { game: Option<Game> }
impl Game {
    fn grab(&mut self, on: bool) {
        self.grabbed = on; if !on { self.lmb = false; self.rmb = false; }
        if on { if self.window.set_cursor_grab(CursorGrabMode::Confined).is_err() { let _ = self.window.set_cursor_grab(CursorGrabMode::Locked); } } else { let _ = self.window.set_cursor_grab(CursorGrabMode::None); }
        self.window.set_cursor_visible(!on);
    }
    fn say(&mut self, m: String) { self.msg = Some((m, 2.5)); }
    /// Réglages du menu : ils s'appliquent tout de suite et survivent à la fermeture.
    fn apply(&mut self) {
        let m = &self.menu; self.s.sound.volumes(m.vol[0], m.vol[1], m.vol[2]);
        self.s.weather.forced = if m.weather == 0 { None } else { Some(KINDS[m.weather - 1]) };
        self.s.sound.force(if m.mood == 0 { None } else { Some([Mood::Calme, Mood::Nuit, Mood::Construction, Mood::Suspense, Mood::Danger][m.mood - 1]) });
    }
    fn write_settings(&self) {
        let m = &self.menu; let keys: Vec<&str> = m.binds.iter().flat_map(|b| b.iter().map(|x| if x.is_empty() { "-" } else { x.as_str() })).collect();
        let _ = std::fs::write(beside_exe("reglages.alw"), format!("{} {} {} {} {} {} {} {} {} {} {} {} {} {}\n{}\n", m.vol[0], m.vol[1], m.vol[2], m.weather, m.mood, m.vol[3], m.quality, m.hud, m.reduce as u8, m.marker as u8, m.tips as u8, m.vol[4], m.vol[5], m.autostep as u8, keys.join(" ")));
    }
    fn slide(&mut self, i: usize) {
        let sz = self.window.inner_size(); let r = hud::menu_layout(sz.width as f32, sz.height as f32, &self.menu).into_iter().find(|x| x.0 == hud::M::Vol(i)).map(|x| x.1);
        if let Some(r) = r { self.menu.vol[i] = ((self.mouse[0] - r[0]) / r[2]).clamp(0.0, 1.0); self.apply(); }
    }
    fn close_menu(&mut self) { self.menu.open = false; self.menu.credits = false; self.menu.controls = false; self.menu.listen = None; self.drag = None; self.write_settings(); if self.menu.title.is_none() { self.grab(true); } }
    fn save(&mut self) { if self.world.is_empty() { return; } let _ = std::fs::create_dir_all(worlds_dir()); if std::fs::write(world_path(&self.world), self.s.save_text()).is_err() { self.say("Sauvegarde impossible".into()); } self.s.dirty = false; }
    /// L'écran titre, avec la liste des mondes à jour.
    fn open_title(&mut self) { self.save(); self.menu.open = false; self.menu.title = Some(hud::Title::new(list_worlds(), &self.world)); self.grab(false); }
    /// Quitter le monde en cours pour un autre (existant, ou neuf avec sa graine et son profil).
    fn switch_world(&mut self, name: &str, seed: Option<i32>, profile: Option<usize>) {
        self.save(); let text = std::fs::read_to_string(world_path(name)).ok();
        let seed = text.as_ref().and_then(|t| t.lines().find_map(|l| l.strip_prefix("seed ").and_then(|v| v.trim().parse().ok()))).or(seed).unwrap_or(SEED);
        let mut s = Session::new(seed, &self.preset, self.threads, spawn_point(seed));
        match &text { Some(t) => if let Err(e) = s.load_text(t) { report(&format!("Sauvegarde illisible : {e}")); }, None => { if let Some(p) = profile { s.profile = p; } if alw_core::gen::is_flat(s.seed) { s.set_creative(true); s.hour = 10.0; } } } // un monde plat commence en créatif, en plein jour
        std::mem::swap(&mut s.sound, &mut self.s.sound); self.gpu.forget_all(); self.s = s; self.world = name.to_string(); self.menu.profile = self.s.profile; self.apply();
        if text.is_none() { self.save(); } let _ = std::fs::write(worlds_dir().join("dernier.txt"), name);
        self.menu.title = None; self.held.clear(); self.keys.clear(); self.grab(true);
    }
    fn title_click(&mut self, a: hud::T) {
        let Some(t) = self.menu.title.as_mut() else { return }; if a != hud::T::Delete { t.confirm = false; }
        match a {
            hud::T::Continue => { if self.world.is_empty() { return; } self.menu.title = None; self.s.tour_end(); self.grab(true); }
            hud::T::New => { t.page = hud::TPage::New; t.name.clear(); t.seed.clear(); t.field = 0; t.profile = 1; t.flat = false; }
            hud::T::Worlds => { t.page = hud::TPage::Worlds; t.sel = t.worlds.iter().position(|w| w.name == t.current).unwrap_or(0); }
            hud::T::Settings => { self.menu.open = true; }
            hud::T::Quit => { self.save(); self.write_settings(); std::process::exit(0); }
            hud::T::Row(i) => { if t.sel == i { let n = t.worlds[i].name.clone(); if n == self.world { self.menu.title = None; self.s.tour_end(); self.grab(true); } else { self.switch_world(&n, None, None); } } else { t.sel = i; } }
            hud::T::Play => { if let Some(w) = t.worlds.get(t.sel) { let n = w.name.clone(); if n == self.world { self.menu.title = None; self.s.tour_end(); self.grab(true); } else { self.switch_world(&n, None, None); } } }
            hud::T::Delete => {
                if t.worlds.is_empty() { return; }
                if !t.confirm { t.confirm = true; return; }
                // Rien ne s'efface vraiment : le fichier part dans « mondes/corbeille ».
                let n = t.worlds[t.sel].name.clone(); let bin = worlds_dir().join("corbeille"); let _ = std::fs::create_dir_all(&bin);
                let _ = std::fs::rename(world_path(&n), bin.join(format!("{n}.alw"))); if n == self.world { self.world.clear(); }
                let cur = self.world.clone(); let page = t.page; *t = hud::Title::new(list_worlds(), &cur); t.page = page;
            }
            hud::T::Back => { t.page = hud::TPage::Main; }
            hud::T::Field(f) => { t.field = f; }
            hud::T::Profile(i) => { t.profile = i; }
            hud::T::Flat => { t.flat = !t.flat; }
            hud::T::Create => { let (name, mut seed, p) = (fresh_name(&t.name), seed_of(&t.seed), t.profile);
                // un monde plat porte sa marque dans la graine ; un monde naturel ne doit jamais la porter par hasard
                if t.flat { seed = (seed & 0xFFF) | alw_core::gen::FLAT; } else if alw_core::gen::is_flat(seed) { seed ^= 0x1000; }
                self.switch_world(&name, Some(seed), Some(p)); }
        }
    }
    fn frame(&mut self) {
        let now = Instant::now(); let real = (now - self.last).as_secs_f64(); let dt = if self.menu.open { 0.0 } else { real.min(0.05) }; self.last = now; self.time += dt;
        if self.run_time && !self.s.time_frozen { self.s.pass_hours(dt * 24.0 / 1500.0 * self.s.time_speed); }
        let k = |c: KeyCode| self.keys.contains(&c);
        // Regarder vers +z place +x à gauche de l'écran : la droite vaut -x.
        let ax = |a: bool, b: bool| (a as i32 - b as i32) as f64;
        let _ = k; let b = |a: usize| self.menu.binds[a].iter().any(|n| !n.is_empty() && self.held.contains(n));
        let titled = self.menu.title.is_some(); if titled { self.s.tour_step(real); } // derrière l'écran titre, un survol du monde : villages, biomes
        let talking = self.s.town.talk.is_some() || self.s.map.open || titled; let open = self.s.inv_open || self.menu.open || talking; let input = if open { Input::default() } else { Input { x: ax(b(hud::A_LEFT), b(hud::A_RIGHT)), z: ax(b(hud::A_FWD), b(hud::A_BACK)), jump: b(hud::A_JUMP), sprint: b(hud::A_SPRINT), crouch: b(hud::A_CROUCH), roll: b(hud::A_ROLL) } };
        self.place_t -= dt; let place = self.rmb && self.place_t <= 0.0; if place { self.place_t = 0.22; }
        self.s.player.autostep = self.menu.autostep; self.s.tick(dt, input, if talking { Actions::default() } else { Actions { mine: self.lmb, place, guard: self.rmb } });
        for n in std::mem::take(&mut self.s.notes) { self.say(n); }
        // les structures relevées par /exporter : un fichier par structure, dans « structures » à côté du jeu
        for (name, text) in std::mem::take(&mut self.s.exports) { let dir = beside_exe("structures"); let _ = std::fs::create_dir_all(&dir); let p = dir.join(&name);
            if let Err(e) = std::fs::write(&p, text) { self.say(format!("Impossible d'écrire {} : {e}", p.display())); } }
        if (self.s.dead.is_some() || self.s.inv_open || self.s.town.talk.is_some() || self.s.map.open) && self.grabbed && !self.menu.open { self.grab(false); }
        if talking && !titled && self.s.town.talk.is_none() && !self.s.map.open && !self.s.inv_open && !self.menu.open && self.s.dead.is_none() { self.grab(true); } // le villageois s'est éloigné // écran de mort, coffre ouvert d'un clic droit : il faut le pointeur
        let (built, gone) = self.s.sync();
        for b in built { self.gpu.upload(b); }
        for g in gone { self.gpu.forget(g); }
        self.save_t += real; if self.save_t > 30.0 { self.save_t = 0.0; self.save(); }
        if let Some(m) = &mut self.msg { m.1 -= real; if m.1 <= 0.0 { self.msg = None; } }
        let size = self.window.inner_size(); let info = if self.debug { let mut v = debug_lines(&self.s, self.fps, self.tris, if self.menu.quality == 1 { "Ultra" } else { "Haut" }); v.extend(self.s.hostile_report()); v.push("F6 : faire naître une créature · F7 : passer à la nuit".into()); v } else { Vec::new() };
        self.tris = draw(&mut self.gpu, &self.md, &self.s, self.time, size.width.max(1) as f32, size.height.max(1) as f32, self.msg.as_ref().map(|m| m.0.as_str()), self.mouse, &self.menu, &info);
        self.frames += 1; self.fps_t += real;
        if self.fps_t >= 0.5 {
            let fps = self.frames as f64 / self.fps_t; self.fps = fps; self.frames = 0; self.fps_t = 0.0;
            self.window.set_title(&format!("A Living World  |  {fps:.0} FPS  |  {:.1} M triangles  |  {} chunks  |  {} en attente", self.tris as f64 / 1e6, self.s.world.loaded(), self.s.world.waiting()));
        }
    }
}
impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.game.is_some() { return; }
        let window = Arc::new(el.create_window(Window::default_attributes().with_title("A Living World").with_inner_size(winit::dpi::LogicalSize::new(1600.0, 900.0)).with_window_icon(winit::window::Icon::from_rgba(include_bytes!("../assets/icone64.rgba").to_vec(), 64, 64).ok())).expect("création de la fenêtre"));
        let size = window.inner_size(); let md = Models::build(); let menu = read_settings(); let ultra = menu.quality == 1; let preset = if ultra { ULTRA } else { HIGH };
        let gpu = pollster::block_on(gpu::Gpu::new(Some(window.clone()), size.width, size.height, gpu::Options { shadow: if ultra { 6144 } else { 4096 }, fallback: false, hole: (preset.radius as f32 - 0.6) * 32.0, cascades: if ultra { gpu::CASCADES_ULTRA } else { gpu::CASCADES } }, &md));
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).saturating_sub(2).clamp(2, 12);
        // Le dernier monde joué tourne derrière l'écran titre ; sans monde, celui de départ, qui ne s'enregistre pas.
        migrate_worlds(); let worlds = list_worlds();
        let last = std::fs::read_to_string(worlds_dir().join("dernier.txt")).ok().map(|t| t.trim().to_string()).filter(|n| worlds.iter().any(|w| &w.name == n)).or(worlds.first().map(|w| w.name.clone())).unwrap_or_default();
        let text = if last.is_empty() { None } else { std::fs::read_to_string(world_path(&last)).ok() };
        let seed = text.as_ref().and_then(|t| t.lines().find_map(|l| l.strip_prefix("seed ").and_then(|v| v.trim().parse().ok()))).unwrap_or(SEED);
        let mut s = Session::new(seed, &preset, threads, spawn_point(seed));
        if let Some(t) = &text { if let Err(e) = s.load_text(t) { report(&format!("Sauvegarde illisible : {e}")); } }
        s.sound = SoundScape::new(sound::start());
        let mut menu = menu; menu.title = Some(hud::Title::new(worlds, &last));
        let mut g = Game { map_drag: None, held: HashSet::new(), preset, threads, world: last, debug: false, fps: 0.0, menu, drag: None, half: None, window, gpu, md, mouse: [0.0; 2], s, keys: HashSet::new(), grabbed: false, time: 0.0, run_time: true, lmb: false, rmb: false, place_t: 0.0, msg: None, save_t: 0.0, last: Instant::now(), frames: 0, fps_t: 0.0, tris: 0 };
        g.menu.profile = g.s.profile; g.apply(); g.grab(false); self.game = Some(g);
    }
    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, e: WindowEvent) {
        let Some(g) = self.game.as_mut() else { return };
        match e {
            WindowEvent::CloseRequested => { g.save(); g.write_settings(); std::process::exit(0); }
            WindowEvent::Resized(s) => g.gpu.resize(s.width, s.height),
            WindowEvent::Focused(false) => { g.keys.clear(); g.held.clear(); g.grab(false); }
            WindowEvent::MouseInput { state, button, .. } => {
                let down = state == ElementState::Pressed;
                let sz = g.window.inner_size(); let (w, h) = (sz.width as f32, sz.height as f32);
                if g.menu.open {
                    if button != MouseButton::Left { return; }
                    if !down { g.drag = None; g.write_settings(); return; }
                    let hit = hud::menu_at(w, h, &g.menu, g.mouse);
                    match hit {
                        Some(hud::M::NewWorld) => { g.write_settings(); g.open_title(); if let Some(t) = g.menu.title.as_mut() { t.page = hud::TPage::Worlds; t.sel = t.worlds.iter().position(|x| x.name == t.current).unwrap_or(0); } }
                        Some(hud::M::Controls) => { g.menu.controls = true; g.menu.listen = None; }
                        Some(hud::M::Bind(a, sl)) => g.menu.listen = Some((a, sl)),
                        Some(hud::M::AutoStep) => { g.menu.autostep = !g.menu.autostep; g.write_settings(); }
                        Some(hud::M::ResetKeys) => { g.menu.binds = hud::default_binds(); g.menu.listen = None; g.write_settings(); }
                        Some(hud::M::Tips) => g.menu.tips = !g.menu.tips,
                        Some(hud::M::Resume) => g.close_menu(),
                        Some(hud::M::Weather(i)) => { g.menu.weather = i; g.apply(); }
                        Some(hud::M::Mood(i)) => { g.menu.mood = i; g.apply(); }
                        Some(hud::M::Quality(i)) => { g.menu.quality = i; g.say("Qualité appliquée au prochain lancement".into()); }
                        Some(hud::M::Vol(i)) => { g.drag = Some(i); g.slide(i); }
                        Some(hud::M::Profile(i)) => { g.s.profile = i; g.menu.profile = i; g.say(format!("Profil de survie : {}", alw_core::survival::PROFILES[i].name)); }
                        Some(hud::M::Hud(i)) => g.menu.hud = i,
                        Some(hud::M::Reduce) => g.menu.reduce = !g.menu.reduce,
                        Some(hud::M::Marker) => g.menu.marker = !g.menu.marker,
                        Some(hud::M::Credits) => g.menu.credits = true,
                        Some(hud::M::Back) => { g.menu.credits = false; g.menu.controls = false; g.menu.listen = None; }
                        Some(hud::M::Quit) => { g.save(); g.write_settings(); std::process::exit(0); }
                        None => {}
                    }
                    return;
                }
                if g.menu.title.is_some() { if button == MouseButton::Left && down { if let Some(a) = g.menu.title.as_ref().and_then(|t| hud::title_at(t, w, h, g.mouse)) { g.title_click(a); } } return; }
                if g.s.dead.is_some() { if button == MouseButton::Left && down && hud::death_at(w, h, g.mouse) { g.s.respawn(); g.grab(true); } return; }
                // La grande carte : on la fait glisser au clic gauche, le clic droit la recentre ; elle se ferme au clavier.
                if g.s.map.open { if button == MouseButton::Left { g.map_drag = if down { Some(g.mouse) } else { None }; } else if button == MouseButton::Right && down { g.s.center_map(); } return; }
                // Conversation : un clic sur une réponse.
                if g.s.town.talk.is_some() { if button == MouseButton::Left && down { if let Some(i) = hud::dialog_at(&g.s, w, h, g.mouse) { g.s.choose(i); if g.s.town.talk.is_none() { g.grab(true); } } } return; }
                if g.s.inv_open {
                    let at = hud::hit_at(&g.s, w, h, g.mouse); let shift = g.keys.contains(&KeyCode::ShiftLeft) || g.keys.contains(&KeyCode::ShiftRight); let slot = if let Some(hud::Hit::Slot(i)) = at { Some(i) } else { None };
                    if down && at != Some(hud::Hit::Search) { g.s.book.typing = false; }
                    match (button, down) {
                        (MouseButton::Left, true) => match at {
                            Some(hud::Hit::Tab(i)) => { g.s.tab = i; g.s.cat_row = 0; }
                            Some(hud::Hit::Scroll(d)) => { let max = hud::cat_rows(&g.s).saturating_sub(hud::CAT_ROWS); g.s.cat_row = (g.s.cat_row as i32 + d).clamp(0, max as i32) as usize; }
                            Some(hud::Hit::Slot(i)) => if shift { g.s.quick_slot(i) } else { g.s.grab_slot(i) },
                            Some(hud::Hit::Catalog(id)) => if shift { g.s.quick_catalog(id) } else if g.s.cursor.is_none() { g.s.grab_catalog(id) },
                            Some(hud::Hit::BookCat(i)) => { g.s.book.cat = i; g.s.book.page = 0; }
                            Some(hud::Hit::Book(r)) => g.s.book.sel = Some(r),
                            Some(hud::Hit::Page(d)) => g.s.book.page = (g.s.book.page as i32 + d).max(0) as usize,
                            Some(hud::Hit::Search) => g.s.book.typing = true,
                            // « Fabriquer » une fois, ou « Tout » : autant que les ingrédients le permettent.
                            // À la main, tout de suite ; à un atelier, l'ouvrage prend le temps de son geste.
                            Some(hud::Hit::Craft(all)) => if let Some(r) = g.s.book.sel { if let Err(e) = g.s.order_craft(r, all || shift) { g.say(e.into()); } },
                            None => {}
                        },
                        (MouseButton::Left, false) => if slot.is_none() && g.s.cursor.is_some() && hud::outside_panel(&g.s, w, h, g.mouse) { g.s.throw_cursor() } else { g.s.drop_slot(slot) },
                        // Clic droit sur une recette : favorite ou non (les favorites passent en tête du livre).
                        (MouseButton::Right, true) if matches!(at, Some(hud::Hit::Book(_))) => if let Some(hud::Hit::Book(r)) = at { let out = alw_core::survival::recipes()[r].out; if !g.s.fav.remove(out) { g.s.fav.insert(out); } },
                        (MouseButton::Right, true) => if let Some(i) = slot { if g.s.cursor.is_some() { g.s.put_one(i) } else { g.s.take_half(i); g.half = Some(i); } },
                        // Un clic droit laisse la moitié en main ; un glisser la dépose là où on relâche.
                        (MouseButton::Right, false) => if let (Some(f), Some(i)) = (g.half.take(), slot) { if f != i { g.s.drop_slot(Some(i)); } },
                        _ => {}
                    }
                    return;
                }
                if !g.grabbed { if down { g.grab(true); } return; }
                match button { MouseButton::Left => g.lmb = down, MouseButton::Right => { g.rmb = down; if down { g.place_t = 0.0; } } _ => {} }
            }
            WindowEvent::CursorMoved { position, .. } => { let m0 = g.mouse; g.mouse = [position.x as f32, position.y as f32]; if let Some(i) = g.drag { g.slide(i); }
                if g.map_drag.is_some() { if g.s.map.open { let hh = g.window.inner_size().height.max(1) as f32; g.s.pan_map([(g.mouse[0] - m0[0]) as f64, (g.mouse[1] - m0[1]) as f64], hud::map_px(hh) as f64); } else { g.map_drag = None; } } }
            WindowEvent::MouseWheel { delta, .. } => {
                let d = match delta { MouseScrollDelta::LineDelta(_, y) => y, MouseScrollDelta::PixelDelta(p) => p.y as f32 };
                if let Some(t) = g.menu.title.as_mut() { if d != 0.0 && t.page == hud::TPage::Worlds { let max = t.worlds.len().saturating_sub(7); t.scroll = (t.scroll as i32 + if d > 0.0 { -1 } else { 1 }).clamp(0, max as i32) as usize; } } else if d != 0.0 && g.s.inv_open && g.s.creative { let max = hud::cat_rows(&g.s).saturating_sub(hud::CAT_ROWS); g.s.cat_row = (g.s.cat_row as i32 + if d > 0.0 { -1 } else { 1 }).clamp(0, max as i32) as usize; } else if d != 0.0 && g.s.map.open { g.s.zoom_map(if d > 0.0 { -1 } else { 1 }); } else if d != 0.0 { g.s.sel = (g.s.sel as i32 + if d > 0.0 { -1 } else { 1 }).rem_euclid(9) as usize; }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(c) = event.physical_key else { return };
                // Alt tenu : tous les besoins s'affichent.
                if matches!(c, KeyCode::AltLeft | KeyCode::AltRight) { g.menu.alt = event.state == ElementState::Pressed; }
                let name = key_name(&event);
                if event.state != ElementState::Pressed { g.keys.remove(&c); g.held.remove(&name); return; }
                // Une touche attendue pour une action : on l'attribue (Échap annule, Retour arrière vide la case).
                if let Some((a, sl)) = g.menu.listen {
                    if !event.repeat { match c { KeyCode::Escape => {}, KeyCode::Backspace => g.menu.binds[a][sl].clear(), _ if !name.is_empty() => { for b in g.menu.binds.iter_mut() { for x in b.iter_mut() { if *x == name { x.clear(); } } } g.menu.binds[a][sl] = name.clone(); } _ => { g.say("Cette touche ne peut pas servir".into()); return; } } g.menu.listen = None; g.write_settings(); }
                    return;
                }
                // Écran titre : les champs du nouveau monde, Échap pour revenir.
                if g.menu.title.is_some() && !g.menu.open {
                    let t = g.menu.title.as_mut().unwrap();
                    match c {
                        KeyCode::Escape => { t.page = hud::TPage::Main; t.confirm = false; }
                        KeyCode::Tab if t.page == hud::TPage::New => t.field = 1 - t.field,
                        KeyCode::Enter | KeyCode::NumpadEnter => { let a = match t.page { hud::TPage::New => hud::T::Create, hud::TPage::Worlds => hud::T::Play, hud::TPage::Main => hud::T::Continue }; g.title_click(a); }
                        KeyCode::Backspace if t.page == hud::TPage::New => { if t.field == 0 { t.name.pop(); } else { t.seed.pop(); } }
                        _ if t.page == hud::TPage::New => if let Some(txt) = &event.text { for ch in txt.chars().filter(|ch| !ch.is_control()) { let f = if t.field == 0 { &mut t.name } else { &mut t.seed }; if f.chars().count() < 32 { f.push(ch); } } },
                        _ => {}
                    }
                    return;
                }
                // Le chat : les touches écrivent la commande ; Entrée l'exécute, Échap la ferme.
                if g.s.chat.is_some() {
                    match c {
                        KeyCode::Escape => g.s.chat = None,
                        KeyCode::Enter | KeyCode::NumpadEnter => { let line = g.s.chat.take().unwrap_or_default(); let r = g.s.command(&line); if !r.is_empty() { g.say(r); } }
                        KeyCode::Backspace => { if let Some(l) = &mut g.s.chat { l.pop(); } }
                        _ => if let Some(t) = &event.text { for ch in t.chars().filter(|ch| !ch.is_control()) { if let Some(l) = &mut g.s.chat { if l.chars().count() < 60 { l.push(ch); } } } }
                    }
                    return;
                }
                if !name.is_empty() { g.held.insert(name.clone()); }
                // Recherche dans le livre de recettes : les touches écrivent au lieu de commander.
                if g.s.inv_open && g.s.book.typing {
                    match c { KeyCode::Backspace => { g.s.book.search.pop(); } KeyCode::Enter | KeyCode::NumpadEnter | KeyCode::Escape => g.s.book.typing = false,
                        _ => if let Some(t) = &event.text { for ch in t.chars().filter(|ch| !ch.is_control()) { if g.s.book.search.chars().count() < 24 { g.s.book.search.push(ch); } } } }
                    g.s.book.page = 0; return;
                }
                if !event.repeat { match c {
                    KeyCode::Enter | KeyCode::NumpadEnter if g.s.dead.is_some() && !g.menu.open => { g.s.respawn(); g.grab(true); }
                    KeyCode::Escape => { if g.s.map.open { g.s.toggle_map(); g.grab(true); } else if g.s.town.talk.is_some() { g.s.close_talk(); g.grab(true); } else if g.s.town.journal { g.s.town.journal = false; } else if g.s.inv_open { g.s.inv_open = false; g.s.drop_slot(None); g.s.close_chest(); g.grab(true); } else if g.menu.open { if g.menu.controls || g.menu.credits { g.menu.controls = false; g.menu.credits = false; } else { g.close_menu(); } } else { g.menu.open = true; g.grab(false); } }
                    _ if g.menu.open || g.s.dead.is_some() => {}
                    KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::Digit4 | KeyCode::Digit5 | KeyCode::Digit6 | KeyCode::Digit7 | KeyCode::Digit8 | KeyCode::Digit9 if g.s.town.talk.is_some() => { g.s.choose(c as usize - KeyCode::Digit1 as usize); if g.s.town.talk.is_none() { g.grab(true); } }
                    _ if g.s.town.talk.is_some() => {}
                    _ if g.menu.is(hud::A_MAP, &name) => { g.s.toggle_map(); g.grab(!g.s.map.open); }
                    _ if g.s.map.open => {}
                    _ if g.menu.is(hud::A_JOURNAL, &name) => { g.s.town.journal = !g.s.town.journal; }
                    _ if g.menu.is(hud::A_DROP, &name) && !g.s.inv_open => { let all = g.keys.contains(&KeyCode::ShiftLeft) || g.keys.contains(&KeyCode::ShiftRight); g.s.throw_held(all); }
                    _ if g.menu.is(hud::A_INV, &name) => { let on = !g.s.inv_open; g.s.inv_open = on; if !on { g.s.drop_slot(None); g.s.close_chest(); } g.grab(!on); }
                    _ if g.menu.is(hud::A_VIEW, &name) => { g.s.third = !g.s.third; let m = if g.s.third { "Troisième personne" } else { "Première personne" }; g.say(m.into()); }
                    _ if g.menu.is(hud::A_TURN, &name) => { let m = g.s.rotate(); g.say(m); }
                    // T (ou /) ouvre le chat des commandes : /jour, /nuit, /vitesse, /survie, /creatif, /vol… (/aide)
                    KeyCode::KeyT | KeyCode::Slash | KeyCode::NumpadDivide if !g.s.inv_open => { g.s.chat = Some("/".into()); g.keys.clear(); g.held.clear(); }
                    KeyCode::F3 => g.debug = !g.debug,
                    // Outils de développement, seulement avec le panneau F3 ouvert : une créature devant soi, la nuit tout de suite.
                    KeyCode::F6 if g.debug => { let k = g.s.horde.spawned as usize; let n = g.s.debug_spawn(k); g.say(format!("Apparition : {n}")); }
                    KeyCode::F7 if g.debug => { g.s.hour = 22.0; g.say("Nuit".into()); }
                    KeyCode::F5 => { g.save(); g.say("Monde sauvegardé".into()); }
                    KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::Digit4 | KeyCode::Digit5 | KeyCode::Digit6 | KeyCode::Digit7 | KeyCode::Digit8 | KeyCode::Digit9 => { let n = c as usize - KeyCode::Digit1 as usize; if g.s.town.journal { g.s.mark_nth(n); } else { g.s.sel = n; } }
                    _ => {}
                } }
                g.keys.insert(c);
            }
            WindowEvent::RedrawRequested => g.frame(),
            _ => {}
        }
    }
    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, e: DeviceEvent) {
        let Some(g) = self.game.as_mut() else { return };
        if let (true, DeviceEvent::MouseMotion { delta }) = (g.grabbed && !g.s.inv_open && !g.menu.open && g.s.town.talk.is_none() && !g.s.map.open && g.menu.title.is_none(), e) { let k = g.menu.sens(); g.s.player.yaw -= delta.0 * k; g.s.player.pitch = (g.s.player.pitch - delta.1 * k).clamp(-1.5, 1.5); }
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) { if let Some(g) = &self.game { g.window.request_redraw(); } }
}
/// Banc d'essai sans fenêtre : charge une scène, joue un scénario, rend quelques images, écrit un BMP.
/// a-living-world --capture sortie.bmp x y z vx vy vz [heure] [rayon] [largeur] [hauteur] [ombre] [scenario]
fn capture(a: &[String]) {
    let n = |i: usize, d: f64| a.get(i).and_then(|v| v.parse().ok()).unwrap_or(d);
    let (eye, look) = ([n(1, -38.5), n(2, 51.4), n(3, 0.5)], [n(4, -20.0), n(5, 52.0), n(6, -30.0)]);
    let (radius, w, h, shadow) = (n(8, 6.0) as i32, n(9, 1280.0) as u32, n(10, 720.0) as u32, n(11, 2048.0) as u32);
    let md = Models::build(); let scene = a.get(12).cloned().unwrap_or_default();
    let mut g = pollster::block_on(gpu::Gpu::new(None, w, h, gpu::Options { shadow, fallback: true, hole: (radius as f32 - 0.6) * 32.0, cascades: if scene == "ultra" { gpu::CASCADES_ULTRA } else { gpu::CASCADES } }, &md));
    let seed = if scene.starts_with("plat") { alw_core::gen::FLAT | 1 } else { SEED }; let mut s = Session::new(seed, &(if scene == "ultra" { Preset { radius, ..ULTRA } } else { Preset { radius, lod1: 2.3, lod2: 4.6 } }), 4, [eye[0], eye[1] - 1.62, eye[2]]); s.hour = n(7, 16.4); s.player.fly = true; s.give_kit(); // les captures gardent l'ancien équipement
    let pump = |s: &mut Session, g: &mut gpu::Gpu| loop { let (b, f) = s.sync(); for c in b { g.upload(c); } for k in f { g.forget(k); } if s.world.waiting() == 0 { break; } std::thread::sleep(std::time::Duration::from_millis(5)); };
    let t0 = Instant::now(); pump(&mut s, &mut g); println!("{} chunks chargés en {:.1} s", s.world.loaded(), t0.elapsed().as_secs_f64());
    let f = view::norm([look[0] - eye[0], look[1] - eye[1], look[2] - eye[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin();
    let none = Input::default(); s.third = scene == "tiers"; s.tick(0.0, none, Actions::default());
    if scene == "tiers" { s.sel = 1; s.tick(0.0, none, Actions::default()); }
    if scene == "bras" { s.sel = 0; for _ in 0..6 { s.tick(1.0 / 60.0, none, Actions { mine: true, place: false, ..Default::default() }); } }
    if scene == "inv" { s.inv_open = true; s.grab_slot(5); s.tick(0.0, none, Actions::default()); }
    // Un onglet du catalogue créatif, par son numéro : « onglet8 » pour les outils.
    if let Some(k) = scene.strip_prefix("onglet").and_then(|k| k.parse::<usize>().ok()) { s.set_creative(true); s.inv_open = true; s.tab = k; s.tick(0.0, none, Actions::default()); }
    // L'arc en main, une flèche encochée : vu par l'archer, ou de l'extérieur.
    if scene == "arc" || scene == "arc_tiers" { s.inv[0] = Some(alw_core::session::Stack::new("bow", None)); s.sel = 0; s.add_item("arrow", 12); s.third = scene == "arc_tiers"; s.tick(0.0, none, Actions::default()); }
    // La ferme : une bête de chaque, un cheval sellé, un loup apprivoisé assis ; puis le joueur en selle (3e personne).
    if scene == "plat_ferme" || scene == "plat_monte" {
        use alw_core::wild::Kind as K; let y = alw_core::gen::FLAT_Y as f64 + 1.0; s.wild.all.clear();
        let herd: [(K, f64, f64, f64); 7] = [(K::Cow, -43.5, -22.0, 0.6), (K::Sheep, -41.0, -21.0, -0.4), (K::Sheep, -39.4, -22.6, 2.2), (K::Pig, -37.0, -23.5, -0.9), (K::Chicken, -38.6, -25.0, 1.4), (K::Horse, -45.5, -24.5, 1.2), (K::Wolf, -36.0, -26.0, -1.6)];
        for (k, (kind, x, z, yaw)) in herd.into_iter().enumerate() { s.wild.put(kind, [x, y, z], yaw, false); let c = s.wild.all.last_mut().unwrap(); c.color = k as u8 % 3;
            if k == 2 { c.farm.shorn = 400.0; } if kind == K::Horse { c.farm.tame = true; c.farm.saddle = true; } if kind == K::Wolf { c.farm.tame = true; c.farm.sit = true; } }
        if scene == "plat_monte" { let h = s.wild.all.iter().position(|c| c.kind == K::Horse).unwrap(); s.wild.all[h].p = [s.player.p[0], y, s.player.p[2]]; s.wild.all[h].yaw = s.player.yaw; s.ride = Some(s.wild.all[h].farm.id); s.third = true; }
        s.tick(0.0, none, Actions::default());
    }
    if scene == "creatif" || scene == "roches" { s.set_creative(true); s.inv_open = true; s.tab = if scene == "roches" { 2 } else { 5 } /* Mobilier */; s.tick(0.0, none, Actions::default()); }
    if scene == "pluie" { s.weather.snap(Kind::Pluie); s.sel = 8; println!("pluie : couvert {}", s.covered(s.eye)); } if scene == "brume" { s.weather.snap(Kind::Brouillard); s.sel = 8; }
    if scene == "abattage" {
        // Un vrai arbre près de la caméra : la hache pendant dix secondes, puis on regarde tomber le tronc.
        let (bx, by, bz) = (eye[0].floor() as i32, eye[1].floor() as i32, eye[2].floor() as i32); let mut spot = None; let n = 34i32;
        'c: for r in 4..n { for dz in -r..=r { for dx in -r..=r { if i32::abs(dx).max(i32::abs(dz)) != r { continue; } for y in (by - 12)..(by + 6) { let (x, z) = (bx + dx, bz + dz); if alw_core::player::Voxels::get(&s.world, x, y, z).is_some_and(alw_core::blocks::is_log) && alw_core::felling::find_tree(&s.world, x, y, z).is_some_and(|t| t.logs.len() > 30) { spot = Some((x, y + 1, z)); break 'c; } } } } }
        let (x, y, z) = spot.expect("un arbre"); let e = [x as f64 + 6.5, y as f64 + 3.0, z as f64 + 0.5]; let mut clear = Vec::new(); for k in 1..8 { for h in 0..5 { for w in -1..=1 { if alw_core::player::Voxels::get(&s.world, x + k, y + h, z + w).is_some_and(|b| !alw_core::blocks::is_log(b)) { clear.push((x + k, y + h, z + w, 0)); } } } } s.world.edit_many(&clear); pump(&mut s, &mut g);
        s.player.p = [e[0], e[1] - 1.62, e[2]]; let d = view::norm([x as f64 + 0.5 - e[0], y as f64 + 0.5 - e[1], z as f64 + 0.5 - e[2]]); s.player.yaw = d[0].atan2(d[2]); s.player.pitch = d[1].asin(); s.sel = 1; s.third = false;
        for _ in 0..640 { s.tick(1.0 / 60.0, none, Actions { mine: true, place: false, ..Default::default() }); if s.falling.is_some() { break; } }
        println!("abattage : chute {}, cible {:?}", s.falling.is_some(), s.target.map(|t| alw_core::blocks::BLOCKS[t.id as usize].name)); for _ in 0..55 { s.tick(1.0 / 60.0, none, Actions::default()); } pump(&mut s, &mut g); s.player.pitch += 0.35; s.tick(0.0, none, Actions::default());
    }
    if scene == "coule" {
        // De l'eau posée sur le relief : elle s'étale, descend et se met à niveau.
        let (x, z) = (eye[0].floor() as i32 + 5, eye[2].floor() as i32 - 4); let c = WorldGen::new(SEED).column(x as f64, z as f64); let y = c.h as i32 + 2; s.world.edit(x, y, z, alw_core::blocks::B::WATER); s.flow.touch(x, y, z); pump(&mut s, &mut g);
        for _ in 0..240 { s.tick(1.0 / 60.0, none, Actions::default()); } pump(&mut s, &mut g); g.settle(); s.sel = 8;
        let e = [x as f64 - 5.0, y as f64 + 3.5, z as f64 - 5.0]; s.player.p = [e[0], e[1] - 1.62, e[2]]; let d = view::norm([x as f64 - e[0], y as f64 - 1.0 - e[1], z as f64 - e[2]]); s.player.yaw = d[0].atan2(d[2]); s.player.pitch = d[1].asin(); pump(&mut s, &mut g); s.tick(0.0, none, Actions::default());
    }
    if ["visage", "course", "accroupi", "saut", "roulade"].contains(&scene.as_str()) {
        // Le personnage vu de près : les vrais modèles et la vraie animation, caméra posée à côté de lui.
        s.player.fly = false; s.third = true; s.sel = 8; for _ in 0..240 { s.tick(1.0 / 60.0, none, Actions::default()); }
        let an = |s: &mut Session, i: alw_core::actors::AnimIn, n: usize| for _ in 0..n { s.anim.update(1.0 / 60.0, i); };
        match scene.as_str() { "course" => an(&mut s, alw_core::actors::AnimIn { speed: 7.2, ground: true, ..Default::default() }, 26), "accroupi" => { s.player.crouch = true; an(&mut s, alw_core::actors::AnimIn { speed: 0.0, ground: true, crouch: true, ..Default::default() }, 40); } "saut" => an(&mut s, alw_core::actors::AnimIn { speed: 4.0, ground: false, vy: 4.0, ..Default::default() }, 30),
            "roulade" => { s.tick(1.0 / 60.0, Input { roll: true, ..none }, Actions::default()); for _ in 0..15 { s.tick(1.0 / 60.0, none, Actions::default()); } } _ => {} }
        let (p, y) = (s.player.p, s.player.yaw); let (a, r, h) = if scene == "visage" { (0.22, 1.25, 1.55) } else { (1.57, 3.3, 1.0) }; let e = [p[0] + (y + a).sin() * r, p[1] + h, p[2] + (y + a).cos() * r]; s.eye = e; s.look = view::norm([p[0] - e[0], p[1] + h - 0.08 - e[1], p[2] - e[2]]);
    }
    if scene == "horizon" || scene == "montagne" {
        // La montagne la plus proche, cherchée en spirale ; on la regarde depuis une crête ou d'assez loin pour lire le relief.
        let w = WorldGen::new(SEED); let mut best = None; 'm: for r in 1..120 { for k in 0..64 { let a = k as f64 / 64.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 40.0, a.sin() * r as f64 * 40.0); if w.column(x, z).h > 100.0 { best = Some((x, z, r as f64 * 40.0)); break 'm; } } }
        println!("{scene} : montagne {:?}", best); let (x, z, _) = best.unwrap_or((2000.0, 0.0, 2000.0)); s.sel = 8; s.third = false;
        if scene == "horizon" { s.player.p[1] = 96.0; let f = view::norm([x - s.player.p[0], -8.0, z - s.player.p[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); pump(&mut s, &mut g); s.tick(0.0, none, Actions::default()); }
        else { let l = (x * x + z * z).sqrt(); let (ex, ez) = (x - x / l * 170.0, z - z / l * 170.0); let h = w.column(ex, ez).h.max(w.column(ex, ez).water); s.player.p = [ex, h + 12.0, ez]; let f = view::norm([x - ex, 20.0, z - ez]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); pump(&mut s, &mut g); s.tick(0.0, none, Actions::default()); }
    }
    if scene == "taiga" {
        if s.hour > 20.0 { s.weather.snap(Kind::Clair); } // les aurores ne se voient que par ciel dégagé
        let w = WorldGen::new(SEED); let mut found = None; 'p: for r in 8..500 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z); if c.mix.strong(alw_core::biome::Biome::Taiga) > 0.97 && c.water == 0.0 && c.path == 0.0 { found = Some((x, z, a, c.h)); break 'p; } } }
        println!("taïga : {found:?}"); if let Some((x, z, a, h)) = found { s.sel = 8; s.third = false; s.player.fly = true; s.player.p = [x, h + 4.0, z]; let f = view::norm([a.cos(), -0.12, a.sin()]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); g.settle(); pump(&mut s, &mut g); g.settle(); s.taiga = 1.0; s.tick(0.0, none, Actions::default()); }
    }
    if scene == "foret" {
        let w = WorldGen::new(SEED); let mut found = None; 'p: for r in 8..500 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z); if c.mix.strong(alw_core::biome::Biome::DeepForest) > 0.97 && c.water == 0.0 && c.path == 0.0 { found = Some((x, z, a, c.h)); break 'p; } } }
        println!("forêt : {found:?}"); if let Some((x, z, a, h)) = found { s.sel = 8; s.third = false; s.player.fly = true; s.player.p = [x, h + 1.0, z]; let f = view::norm([a.cos(), 0.1, a.sin()]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); g.settle(); pump(&mut s, &mut g); g.settle(); s.tick(0.0, none, Actions::default()); s.deep = 1.0; s.tick(0.0, none, Actions::default()); }
    }
    if scene == "taiga_faune" || scene == "virevoltant" {
        // Le cœur du biome, au sol ; on y pose la faune et une piste de pas dans la neige devant la caméra.
        let b = if scene == "virevoltant" { alw_core::biome::Biome::Steppe } else { alw_core::biome::Biome::Taiga };
        let w = WorldGen::new(SEED); let mut found = None; 'p: for r in 8..500 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z); if c.mix.strong(b) > 0.97 && c.water == 0.0 && c.path == 0.0 && c.forest < 0.5 { found = Some((x, z, a)); break 'p; } } }
        println!("{scene} : {found:?}");
        if let Some((x, z, a)) = found {
            s.sel = 8; s.third = false; s.player.fly = true; s.player.p = [x, 100.0, z]; g.settle(); pump(&mut s, &mut g); g.settle();
            let gy = |s: &Session, x: f64, z: f64| { let mut y = 120; while y > 2 && !s.world.get(x.floor() as i32, y, z.floor() as i32).is_some_and(alw_core::blocks::is_solid) { y -= 1; } y as f64 + 1.0 };
            let (dx, dz) = (a.cos(), a.sin()); let (sx, sz) = (-dz, dx); let at = |k: f64, l: f64| (x + dx * k + sx * l, z + dz * k + sz * l);
            s.player.p = [x, gy(&s, x, z) + 1.4, z]; let f = view::norm([dx, -0.22, dz]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin();
            use alw_core::wild::{snow_at, Kind, Print}; use alw_core::player::Voxels;
            if scene == "taiga_faune" {
                let (fx, fz) = at(9.0, 2.0); s.wild.put(Kind::Fox, [fx, gy(&s, fx, fz), fz], s.player.yaw + 1.2, false);
                let (hx, hz) = at(6.5, -2.2); s.wild.put(Kind::Hare, [hx, gy(&s, hx, hz), hz], s.player.yaw - 0.9, false);
                let (cx, cz) = at(11.0, -1.0); s.wild.put(Kind::Crow, [cx, gy(&s, cx, cz) + 3.5, cz], s.player.yaw + 1.6, true);
                let (cx, cz) = at(8.0, 0.5); s.wild.put(Kind::Crow, [cx, gy(&s, cx, cz), cz], s.player.yaw + 2.6, false);
                for i in 0..14 { let k = 1.5 + i as f64 * 0.62; let side = if i % 2 == 0 { 0.13 } else { -0.13 }; let (px, pz) = at(k, side + 0.6 * (k * 0.3).sin()); let y = gy(&s, px, pz); if let Some(top) = snow_at(&s.world, px, y, pz) { s.wild.prints.push(Print { p: [px, top + 0.004, pz], yaw: s.player.yaw, age: 0.0, small: false }); } }
            } else { for (k, l) in [(4.5, -0.8), (7.5, 1.4), (11.0, -0.4)] { let (tx, tz) = at(k, l); s.wild.put(Kind::Tumble, [tx, gy(&s, tx, tz), tz], 0.0, false); } s.steppe = 1.0; }
            s.taiga = if scene == "taiga_faune" { 1.0 } else { 0.0 }; s.tick(0.0, none, Actions::default());
        }
    }
    if scene == "tronc_creux" {
        // Le bout du tronc creux couché le plus proche, vu dans l'axe : on doit voir à travers.
        use alw_core::blocks::{idx, BLOCKS, CH, CX}; let w = WorldGen::new(SEED); let mut found = None;
        'g: for r in 1..60i32 { for dz in -r..=r { for dx in -r..=r { if dx.abs().max(dz.abs()) != r { continue; } let v = w.gen_chunk(dx, dz);
            for y in 2..CH - 2 { for z in 2..CX - 2 { for x in 2..CX - 2 { let n = BLOCKS[v[idx(x, y, z)] as usize].name; if !n.contains("_hollow_") { continue; }
                let ax = if n.ends_with("_x") { (1i32, 0i32) } else { (0, 1) }; let back = v[idx((x as i32 - ax.0) as usize, y, (z as i32 - ax.1) as usize)];
                if back == 0 { found = Some((dx * 32 + x as i32, y as i32, dz * 32 + z as i32, ax)); break 'g; } } } } } } }
        println!("tronc creux : {found:?}");
        if let Some((x, y, z, ax)) = found { let (cx, cy, cz) = (x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5); let e = [cx - ax.0 as f64 * 3.2 + ax.1 as f64 * 0.9, cy + 0.9, cz - ax.1 as f64 * 3.2 + ax.0 as f64 * 0.9]; s.sel = 8; s.third = false; s.player.fly = true; s.player.p = [e[0], e[1] - 1.62, e[2]]; pump(&mut s, &mut g); let f = view::norm([cx - e[0], cy - e[1], cz - e[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); s.tick(0.0, none, Actions::default()); }
    }
    if scene == "materiaux" {
        // Une rangée posée devant la caméra : roches et gravier retravaillés, buissons, champignons de chaque couleur.
        use alw_core::blocks::{is_solid, B}; use alw_core::player::Voxels; pump(&mut s, &mut g);
        let (fx, fz) = (s.player.yaw.sin(), s.player.yaw.cos()); let (sx, sz) = (-fz, fx); let gy = |s: &Session, x: f64, z: f64| { let mut y = 120; while y > 2 && !s.world.get(x.floor() as i32, y, z.floor() as i32).is_some_and(is_solid) { y -= 1; } y };
        let mut edits = vec![]; let p = s.player.p;
        for (i, b) in [B::GRANITE, B::LIMESTONE, B::GRAVEL, B::STONE].iter().enumerate() { let (x, z) = (p[0] + fx * 4.0 + sx * (i as f64 * 1.0 - 1.5), p[2] + fz * 4.0 + sz * (i as f64 * 1.0 - 1.5)); let y = gy(&s, x, z); edits.push((x.floor() as i32, y + 1, z.floor() as i32, *b)); }
        for (i, b) in [B::SAGEBRUSH, B::DEAD_BUSH, B::ROWAN, B::BLUEBERRY, B::GLOWCAP, B::GLOWCAP_BLUE, B::GLOWCAP_PINK, B::GLOWCAP_VIOLET, B::GLOWCAP_CYAN].iter().enumerate() { let (x, z) = (p[0] + fx * 2.6 + sx * (i as f64 * 0.9 - 3.6), p[2] + fz * 2.6 + sz * (i as f64 * 0.9 - 3.6)); let y = gy(&s, x, z); edits.push((x.floor() as i32, y + 1, z.floor() as i32, *b)); }
        s.world.edit_many(&edits); pump(&mut s, &mut g); s.player.pitch = -0.32; s.tick(0.0, none, Actions::default());
    }
    if scene == "desert" || scene == "oasis" {
        // Le cœur du désert, vu d'un peu haut ; pour l'oasis, le premier point d'eau du désert.
        let w = WorldGen::new(SEED); let mut found = None; 'p: for r in 8..700 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z); if c.mix.strong(alw_core::biome::Biome::Desert) > 0.97 && (if scene == "oasis" { c.water != 0.0 } else { c.water == 0.0 && c.path == 0.0 }) { found = Some((x, z, a, c.h.max(c.water))); break 'p; } } }
        println!("{scene} : {found:?}"); if let Some((x, z, a, h)) = found { s.sel = 8; s.third = false; s.player.fly = true; let back = if scene == "oasis" { 12.0 } else { 6.0 }; s.player.p = [x - a.cos() * back, h + 9.0, z - a.sin() * back]; let f = view::norm([a.cos(), -0.2, a.sin()]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); g.settle(); pump(&mut s, &mut g); g.settle(); s.desert = 1.0; s.tick(0.0, none, Actions::default()); }
    }
    if scene == "hautes_terres" {
        let w = WorldGen::new(SEED); let mut found = None; 'p: for r in 8..700 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z); if c.mix.strong(alw_core::biome::Biome::Highlands) > 0.97 && c.water == 0.0 && c.path == 0.0 { found = Some((x, z, a, c.h)); break 'p; } } }
        println!("hautes terres : {found:?}"); if let Some((x, z, a, h)) = found { s.sel = 8; s.third = false; s.player.fly = true; s.player.p = [x, h + 4.0, z]; let f = view::norm([a.cos(), -0.12, a.sin()]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); g.settle(); pump(&mut s, &mut g); g.settle(); s.highlands = 1.0; s.tick(0.0, none, Actions::default()); }
    }
    if scene == "hautes_montagnes" {
        // Un lac d'altitude des hautes montagnes, vu depuis sa rive, avec les crêtes enneigées derrière.
        let w = WorldGen::new(SEED); let mut found = None; 'p: for r in 8..900 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z); if c.mix.strong(alw_core::biome::Biome::Mountains) > 0.9 && c.still && c.water > 0.0 { found = Some((x - a.cos() * 34.0, z - a.sin() * 34.0, a, c.water)); break 'p; } } }
        println!("hautes montagnes : {found:?}"); if let Some((x, z, a, h)) = found { s.sel = 8; s.third = false; s.player.fly = true; s.player.p = [x, h + 16.0, z]; let f = view::norm([a.cos(), -0.3, a.sin()]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); g.settle(); pump(&mut s, &mut g); g.settle(); s.mountains = 1.0; s.tick(0.0, none, Actions::default()); }
    }
    if scene == "recif" || scene == "plage" {
        // Mer chaude : sous l'eau, un récif de corail et ses bancs de poissons ; ou la plage, ses crabes et ses goélands, des dauphins au large.
        let w = WorldGen::new(SEED); let reef = scene == "recif"; let mut found = None;
        'p: for r in 20..800 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z);
            let ok = if reef { c.mix.sea > 0.6 && c.mix.temp > 0.45 && c.water == alw_core::gen::SEA && c.water - c.h >= 4.0 && c.water - c.h <= 8.0 } else { c.mix.sea > 0.03 && c.mix.sea < 0.3 && c.water == 0.0 && c.h < alw_core::gen::SEA + 2.0 && c.mix.temp > 0.3 };
            if ok { found = Some((x, z, c.h)); break 'p; } } }
        println!("{scene} : {found:?}");
        if let Some((x, z, h)) = found {
            // Le regard vers le large : la direction où la mer s'étend le plus.
            let a = (0..16).map(|k| k as f64 / 16.0 * std::f64::consts::TAU).max_by(|p, q| { let f = |t: f64| w.column(x + t.cos() * 30.0, z + t.sin() * 30.0).mix.sea; f(*p).total_cmp(&f(*q)) }).unwrap_or(0.0);
            let (dx, dz) = (a.cos(), a.sin()); let (sx, sz) = (-dz, dx); let at = |k: f64, l: f64| (x + dx * k + sx * l, z + dz * k + sz * l);
            s.sel = 8; s.third = false; s.player.fly = true; s.player.p = if reef { [x, h + 1.4, z] } else { [x - dx * 4.0, h + 2.2, z - dz * 4.0] };
            let f = view::norm([dx, if reef { -0.12 } else { -0.08 }, dz]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); s.hour = 11.0;
            g.settle(); pump(&mut s, &mut g); g.settle(); s.sea = 1.0; s.sea_warm = 1.0;
            use alw_core::wild::Kind as K;
            if reef { for (i, (k, l, y)) in [(4.0, -1.2, 1.6), (6.5, 1.5, 2.2), (9.0, -0.5, 1.2)].iter().enumerate() { for n in 0..6 { let (fx, fz) = at(*k + (n % 3) as f64 * 0.45, *l + (n / 3) as f64 * 0.5); s.wild.put_swim(K::Reef, [fx, h + y + (n % 2) as f64 * 0.3, fz], s.player.yaw + 1.2, 10 + i as u32, i as u8); } }
                let (fx, fz) = at(11.0, 2.5); s.wild.put_swim(K::Dolphin, [fx, h + 2.6, fz], s.player.yaw + 1.4, 30, 0); }
            else { for (k, l) in [(2.0, -1.5), (3.2, 1.2)] { let (cx, cz) = at(k, l); s.wild.put(K::Crab, [cx, h + 1.0, cz], s.player.yaw + 1.0, false); }
                for (k, l, y) in [(8.0, -3.0, 6.0), (12.0, 2.0, 8.0)] { let (gx, gz) = at(k, l); s.wild.put(K::Gull, [gx, h + y, gz], s.player.yaw + 1.3, true); }
                for (k, l) in [(26.0, -2.0), (28.0, 0.5)] { let (dx2, dz2) = at(k, l); let c = w.column(dx2, dz2); if c.water > 0.0 { s.wild.put_swim(K::Dolphin, [dx2, c.water - 0.6, dz2], s.player.yaw + 1.5, 31, 0); } } }
            s.tick(0.0, none, Actions::default());
        }
    }
    if scene == "port" || scene == "epave" || scene == "large" {
        // Le port de pêche le plus proche, son ponton et ses barques ; une épave au fond de l'océan ; le grand large, ses baleines et ses requins.
        let w = WorldGen::new(SEED); use alw_core::wild::Kind as K; let sea = alw_core::gen::SEA;
        let look = |s: &mut Session, from: [f64; 3], to: [f64; 3]| { s.player.p = from; let f = view::norm([to[0] - from[0], to[1] - from[1] - 1.6, to[2] - from[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); };
        s.sel = 8; s.third = false; s.player.fly = true; s.hour = 11.0;
        if scene == "port" { if let Some(v) = w.villages_near(0.0, 0.0, 6000.0).into_iter().find(|v| v.kind == "port") {
            let (x, z) = (v.c[0] as f64, v.c[2] as f64); let a = (0..24).map(|k| k as f64 * std::f64::consts::TAU / 24.0).find(|a| (20..=50).step_by(3).any(|r| w.column(x + a.cos() * r as f64, z + a.sin() * r as f64).water == sea)).unwrap_or(0.0);
            println!("port : {} {:?}", v.name, v.c); let (sx, sz) = (-a.sin(), a.cos()); look(&mut s, [x + a.cos() * 18.0 + sx * 22.0, v.c[1] as f64 + 9.0, z + a.sin() * 18.0 + sz * 22.0], [x + a.cos() * 38.0 - sx * 2.0, sea, z + a.sin() * 38.0 - sz * 2.0]); } }
        if scene == "epave" { if let Some(p) = w.wrecks_near(0.0, 0.0, 9000.0).into_iter().find(|p| w.climate.at(p[0] as f64, p[2] as f64).temp > 0.0) {
            println!("épave : {p:?}"); let c = [p[0] as f64 + 0.5, p[1] as f64, p[2] as f64 + 0.5]; look(&mut s, [c[0] + 9.0, c[1] + 7.5, c[2] + 7.0], [c[0] - 2.0, c[1] + 1.0, c[2] - 1.0]); } }
        let mut deep = None; if scene == "large" { 'p: for r in 20..800 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z); if c.mix.sea > 0.8 && c.mix.deep > 0.6 && c.mix.temp > 0.3 && c.water == sea && sea - c.h > 14.0 { deep = Some((x, z, c.h)); break 'p; } } }
            println!("large : {deep:?}"); if let Some((x, z, h)) = deep { look(&mut s, [x, (h + sea) * 0.5, z], [x + 20.0, (h + sea) * 0.5 + 1.0, z]); } }
        g.settle(); pump(&mut s, &mut g); g.settle(); s.sea = 1.0;
        let (fx, fz) = (s.player.yaw.sin(), s.player.yaw.cos()); let (sx, sz) = (-fz, fx); let e = s.player.p; let at = |k: f64, l: f64, y: f64| [e[0] + fx * k + sx * l, e[1] + y, e[2] + fz * k + sz * l];
        if scene == "large" && deep.is_some() { s.wild.put_swim(K::Whale, at(24.0, -4.0, 1.0), s.player.yaw + 1.2, 40, 0); s.wild.put_swim(K::Shark, at(9.0, 3.0, -0.5), s.player.yaw - 1.4, 41, 0); for (k, l) in [(13.0, -2.0), (14.0, -0.5), (12.5, 1.0)] { s.wild.put_swim(K::Dolphin, at(k, l, 1.5), s.player.yaw + 1.5, 42, 0); } s.ocean = 1.0; }
        if scene == "epave" { s.wild.put_swim(K::Shark, at(6.0, 2.5, 1.0), s.player.yaw + 1.3, 43, 0); }
        s.tick(0.0, none, Actions::default());
    }
    if scene == "cactus" {
        // Le grand cactus le plus proche du cœur du désert, vu de près.
        use alw_core::blocks::{idx, B, CH, CX}; let w = WorldGen::new(SEED); let mut core = None; 'p: for r in 8..700 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); if w.column(x, z).mix.strong(alw_core::biome::Biome::Desert) > 0.97 { core = Some(((x / 32.0).floor() as i32, (z / 32.0).floor() as i32)); break 'p; } } }
        let mut found = None; if let Some((cx, cz)) = core { 'g: for r in 0..6i32 { for dz in -r..=r { for dx in -r..=r { if dx.abs().max(dz.abs()) != r { continue; } let v = w.gen_chunk(cx + dx, cz + dz); for y in 2..CH - 4 { for z in 0..CX { for x in 0..CX { if v[idx(x, y, z)] == B::CACTUS && v[idx(x, y + 3, z)] == B::CACTUS && v[idx(x, y - 1, z)] != B::CACTUS { found = Some(((cx + dx) * 32 + x as i32, y as i32, (cz + dz) * 32 + z as i32)); break 'g; } } } } } } } }
        println!("cactus : {found:?}"); if let Some((x, y, z)) = found { let (cx, cy, cz) = (x as f64 + 0.5, y as f64 + 2.0, z as f64 + 0.5); let e = [cx - 3.6, cy + 0.8, cz - 2.2]; s.sel = 8; s.third = false; s.player.fly = true; s.player.p = [e[0], e[1] - 1.62, e[2]]; pump(&mut s, &mut g); let f = view::norm([cx - e[0], cy - e[1], cz - e[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); s.desert = 1.0; s.tick(0.0, none, Actions::default()); }
    }
    if scene == "steppe" {
        let w = WorldGen::new(SEED); let mut found = None; 'p: for r in 8..500 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z); if c.mix.strong(alw_core::biome::Biome::Steppe) > 0.97 && c.water == 0.0 && c.path == 0.0 { found = Some((x, z, a, c.h)); break 'p; } } }
        println!("steppe : {found:?}"); if let Some((x, z, a, h)) = found { s.sel = 8; s.third = false; s.player.fly = true; s.player.p = [x - a.cos() * 14.0, h + 16.0, z - a.sin() * 14.0]; let f = view::norm([a.cos(), -0.22, a.sin()]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); g.settle(); pump(&mut s, &mut g); g.settle(); s.steppe = 1.0; s.tick(0.0, none, Actions::default()); }
    }
    if scene == "prairie" {
        // Le point le plus proche franchement en prairie, vu à hauteur d'homme vers le centre du biome.
        let w = WorldGen::new(SEED); let mut found = None; 'p: for r in 8..500 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z); if c.mix.strong(alw_core::biome::Biome::Meadow) > 0.95 && c.water == 0.0 { found = Some((x, z, a, c.h)); break 'p; } } }
        println!("prairie : {found:?}"); if let Some((x, z, a, h)) = found { s.sel = 8; s.third = false; s.player.p = [x, h + 3.0, z]; let f = view::norm([a.cos(), -0.08, a.sin()]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); pump(&mut s, &mut g); s.tick(0.0, none, Actions::default()); }
    }
    if scene == "chute" {
        // La plus proche chute de rivière : un palier où le niveau change entre deux colonnes voisines.
        let w = WorldGen::new(SEED); let mut found = None; 'f: for r in 2..500i32 { for k in 0..(r * 6) { let a = k as f64 / (r * 6) as f64 * std::f64::consts::TAU; let (x, z) = ((a.cos() * r as f64 * 3.0).floor(), (a.sin() * r as f64 * 3.0).floor()); let c = w.column(x, z); if c.water == 0.0 || c.still { continue; } for (dx, dz) in [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)] { let n = w.column(x + dx, z + dz); if n.water - c.water >= 4.0 { found = Some((x, z, dx, dz, c.water)); break 'f; } } } }
        println!("chute : {found:?}"); if let Some((x, z, dx, dz, l)) = found { s.sel = 8; s.third = false; let e = [x - dx * 8.0 + dz * 5.0, l + 5.0, z - dz * 8.0 + dx * 5.0]; s.player.p = [e[0], e[1] - 1.62, e[2]]; pump(&mut s, &mut g); let f = view::norm([x - e[0], l + 1.5 - e[1], z - e[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); pump(&mut s, &mut g); s.tick(0.0, none, Actions::default()); }
    }
    if scene == "grotte" {
        // Une vraie entrée : la colonne creusée la plus proche, on descend dedans.
        let w = WorldGen::new(SEED); let mut found = None; 'g: for r in 2..100i32 { for dz in -r..=r { for dx in -r..=r { if i32::abs(dx).max(i32::abs(dz)) != r { continue; } let (cx, cz) = (dx, dz); let v = w.gen_chunk(cx, cz); for x in (0..32).step_by(4) { for z in (0..32).step_by(4) { let (wx, wz) = (cx * 32 + x as i32, cz * 32 + z as i32); let h = w.column(wx as f64, wz as f64).h as i32; if h > 40 && h < 100 && (0..7).all(|k| v[alw_core::blocks::idx(x, (h - 1 - k) as usize, z)] == 0) { found = Some((wx, h, wz)); break 'g; } } } } } }
        println!("grotte : entrée {found:?}"); if let Some((x, h, z)) = found { s.sel = 3; s.third = false; s.player.p = [x as f64 + 0.5, h as f64 - 3.0, z as f64 + 0.5]; pump(&mut s, &mut g); s.tick(0.0, none, Actions::default()); s.player.pitch = 0.9; s.tick(0.0, none, Actions::default()); }
    }
    if scene == "sous_eau" { let (x, z) = (look[0].floor() + 0.5, look[2].floor() + 0.5); let w = WorldGen::new(SEED).column(x, z).water; println!("niveau d'eau {w}"); s.third = false; s.sel = 8; s.player.p = [x, w - 2.6, z]; s.tick(0.0, none, Actions::default()); println!("sous l'eau : {}", s.under()); }
    if scene == "faune" {
        // La faune vit quarante secondes, puis la caméra va voir le premier cerf (ou le premier oiseau posé).
        for _ in 0..2400 { s.tick(1.0 / 60.0, none, Actions::default()); }
        println!("faune : {} cerfs, {} oiseaux", s.fauna.deer.len(), s.fauna.birds.len());
        let spot = s.fauna.deer.first().map(|d| d.p).or(s.fauna.birds.first().map(|b| b.p));
        if let Some(q) = spot { let (a, r) = ((q[2] - s.player.p[2]).atan2(q[0] - s.player.p[0]), 8.2); let e = [q[0] - a.cos() * r, q[1] + 4.5, q[2] - a.sin() * r]; s.sel = 8; s.player.p = [e[0], e[1] - 1.62, e[2]]; let f = view::norm([q[0] - e[0], q[1] + 0.8 - e[1], q[2] - e[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); pump(&mut s, &mut g); s.tick(0.0, none, Actions::default()); }
    }
    if scene == "carte" {
        // La grande carte dépliée sur ce qu'on a vu du départ.
        s.player.fly = true; for _ in 0..60 { s.tick(1.0 / 60.0, none, Actions::default()); }
        s.toggle_map(); for _ in 0..60 { s.tick(1.0 / 60.0, none, Actions::default()); }
        println!("{scene} : {} pixels explorés, anim {:.2}", s.map.explored(), s.map.anim);
    }
    if scene == "village_desert" || scene == "village_nord" {
        // Le village du désert (grès, terrasses, coupoles) ou de la montagne (rondins, têtes de dragon), et ses habitants.
        let want = if scene == "village_desert" { "desert" } else { "montagne" };
        if let Some(v) = WorldGen::new(SEED).villages_near(0.0, 0.0, 9000.0).into_iter().find(|v| v.kind == want) {
            let c = [v.c[0] as f64 + 0.5, v.c[1] as f64 + 1.0, v.c[2] as f64 + 0.5]; s.hour = 10.5; s.sel = 8; s.third = false; s.player.fly = true;
            let e = [c[0] - 13.0, c[1] + 6.0, c[2] - 13.0]; s.player.p = [e[0], e[1] - 1.62, e[2]]; let f = view::norm([c[0] - e[0], c[1] - e[1], c[2] - e[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin();
            g.settle(); pump(&mut s, &mut g); for _ in 0..900 { s.tick(1.0 / 60.0, none, Actions::default()); } pump(&mut s, &mut g); g.settle();
            println!("{scene} : {} ({}) {:?}, {} habitants", v.name, v.kind, v.c, s.town.folk.len());
        }
    }
    if scene == "village" || scene == "artisan" || scene == "bourg" || scene == "interieur" {
        // Le village le plus proche du départ, en milieu de matinée : la place vue d'en haut, ou le forgeron à son poste et la conversation.
        if let Some(v) = s.nearest_village(0.0, 0.0) {
            let c = [v.c[0] as f64 + 0.5, v.c[1] as f64 + 1.0, v.c[2] as f64 + 0.5]; s.hour = 10.5; s.sel = 8; s.third = false; s.player.fly = true;
            let look = |s: &mut Session, e: [f64; 3], q: [f64; 3]| { s.player.p = [e[0], e[1] - 1.62, e[2]]; let f = view::norm([q[0] - e[0], q[1] - e[1], q[2] - e[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); };
            look(&mut s, [c[0] - 22.0, c[1] + 14.0, c[2] - 22.0], c); pump(&mut s, &mut g);
            for _ in 0..1500 { s.tick(1.0 / 60.0, none, Actions::default()); } pump(&mut s, &mut g);
            println!("village : {} ({}), {} habitants, ateliers {:?}", v.name, v.kind, s.town.folk.len(), v.crafts);
            if scene == "artisan" { if let Some(k) = s.town.folk.iter().position(|f| f.craft == Some("forgeron")) {
                let f = s.town.folk[k].clone(); let h = &v.houses[f.house]; let e = [h.inside[0] + (h.inside[0] - f.p[0]) * 0.2, c[1] + 1.62, h.inside[2] + (h.inside[2] - f.p[2]) * 0.2];
                look(&mut s, e, [f.p[0], f.p[1] + 1.2, f.p[2]]); s.player.fly = false; s.tick(0.0, none, Actions::default()); s.talk_to(k); s.add_item("iron_ingot", 2); s.add_item("coal", 8); s.tick(0.0, none, Actions::default());
            } } else if scene == "interieur" { if let Some(h) = v.houses.iter().find(|h| h.craft.is_none()) {
                // Dans l'auberge, du seuil vers le fond : tables rondes, lustre, cheminée.
                let (dx, dz) = (h.spot[0] - h.inside[0], h.spot[2] - h.inside[2]); let e = [h.inside[0] - dx * 0.15, h.inside[1] + 1.75, h.inside[2] - dz * 0.15];
                look(&mut s, e, [h.spot[0], h.spot[1] + 0.7, h.spot[2]]); s.player.fly = true; s.tick(0.0, none, Actions::default());
            } } else if scene == "bourg" { if let Some(h) = v.houses.iter().find(|h| h.craft.is_none()) {
                // À hauteur d'homme sur la place, face à l'auberge.
                let (dx, dz) = (h.ring[0] - c[0], h.ring[2] - c[2]); let l = dx.hypot(dz).max(0.01); let e = [c[0] + dx / l * 2.5 - dz / l * 2.0, c[1] + 1.7, c[2] + dz / l * 2.5 + dx / l * 2.0];
                look(&mut s, e, [h.door[0], h.door[1] + 3.2, h.door[2]]); s.player.fly = true; s.tick(0.0, none, Actions::default());
            } } else { s.tick(0.0, none, Actions::default()); }
        }
    }
    if scene == "jeu" {
        // Scénario : casser le bloc visé, puis poser de la pierre, avec les vrais chemins du jeu.
        s.sel = 0; if s.target.is_none() { s.player.pitch = -0.9; s.tick(0.0, none, Actions::default()); } let t = s.target.expect("un bloc visé"); let before = s.inv.iter().flatten().count();
        for _ in 0..240 { s.tick(1.0 / 60.0, none, Actions { mine: true, place: false, ..Default::default() }); if alw_core::player::Voxels::get(&s.world, t.x, t.y, t.z) == Some(0) { break; } }
        println!("casse : {} -> {:?}, objets {} -> {}", alw_core::blocks::BLOCKS[t.id as usize].name, alw_core::player::Voxels::get(&s.world, t.x, t.y, t.z), before, s.inv.iter().flatten().count());
        s.sel = 5; s.tick(0.0, none, Actions::default()); s.tick(0.0, none, Actions { mine: false, place: true, ..Default::default() }); pump(&mut s, &mut g); s.tick(0.0, none, Actions::default()); println!("pose : cible {:?}", s.target.map(|t| alw_core::blocks::BLOCKS[t.id as usize].name));
    }
    if ["survie", "craft", "campement", "mort", "ramassage", "fissure", "construction", "faune", "combat", "tenue", "creatures", "titre", "menagerie", "chat", "oeufs"].contains(&scene.as_str()) {
        // Survie : on retombe au sol, un établi et un feu de camp devant soi, des jauges entamées.
        use alw_core::blocks::{is_solid, B}; use alw_core::player::Voxels;
        s.player.fly = false; s.third = false; s.sel = 2; for _ in 0..240 { s.tick(1.0 / 60.0, none, Actions::default()); } if s.dead.is_some() { s.respawn(); } s.vit = [100.0; 3];
        let gy = |s: &Session, x: f64, z: f64| { let mut y = s.player.p[1] as i32 + 4; while y > 2 && !s.world.get(x.floor() as i32, y, z.floor() as i32).is_some_and(is_solid) { y -= 1; } y + 1 };
        let (p, fx, fz) = (s.player.p, s.player.yaw.sin(), s.player.yaw.cos()); let (sx, sz) = (-fz, fx); let at = |k: f64, l: f64| (p[0] + fx * k + sx * l, p[2] + fz * k + sz * l);
        let (wx, wz) = at(3.6, 1.4); let y2 = gy(&s, wx, wz);
        // Le feu sur la terre ferme la plus proche de la vue : jamais dans l'eau ni sous l'établi.
        let dry = |s: &Session, x: f64, z: f64| { let y = gy(s, x, z); (y as f64 - s.player.p[1]).abs() <= 1.5 && s.world.get(x.floor() as i32, y, z.floor() as i32).is_some_and(|b| !alw_core::blocks::is_water(b)) && !s.world.get(x.floor() as i32, y - 1, z.floor() as i32).is_some_and(alw_core::blocks::is_water) };
        let (cx, cz) = (0..40).map(|i| (2.4 + (i / 8) as f64 * 0.5, (i % 8) as f64 * 0.3 * if i % 2 == 0 { 1.0 } else { -1.0 } - 0.6)).map(|(k, l)| at(k, l)).find(|q| dry(&s, q.0, q.1) && (q.0.floor() != wx.floor() || q.1.floor() != wz.floor())).unwrap_or(at(2.6, 0.0)); let y1 = gy(&s, cx, cz);
        s.world.edit_many(&[(cx.floor() as i32, y1, cz.floor() as i32, B::CAMPFIRE), (wx.floor() as i32, y2, wz.floor() as i32, B::WORKBENCH)]); pump(&mut s, &mut g);
        s.player.pitch = -0.32; s.vit = [62.0, 34.0, 17.0]; s.stamina = 41.0;
        if scene == "survie" { let (dx, dz) = at(5.5, -2.6); s.fauna.put([dx, gy(&s, dx, dz) as f64, dz], s.player.yaw + 2.0); s.air = 55.0; s.tick(0.0, none, Actions::default()); s.hurt_at = s.elapsed - 0.25; }
        if scene == "craft" { s.inv = [None; alw_core::session::INV_SIZE]; for (id, n) in [("oak_log", 3), ("planks", 6), ("stick", 5), ("stone", 4), ("raw_meat", 2), ("berries", 7), ("leather", 1), ("glowcap", 2), ("pickaxe", 1)] { s.add_item(id, n); } s.inv_open = true; }
        if scene == "mort" { s.harm(500.0, "Une chute", false); }
        if scene == "ramassage" {
            // Ce qui se ramasse : branches, cailloux, un myrtillier chargé et un autre déjà cueilli, posés sur le sol sec devant soi.
            let mut used = vec![(cx.floor(), cz.floor()), (wx.floor(), wz.floor())]; let mut edits = Vec::new();
            for (b, k, l) in [(B::TWIGS, 2.0, -0.8), (B::PEBBLES, 2.2, 0.3), (B::BLUEBERRY, 2.6, -1.6), (B::BLUEBERRY_PICKED, 2.8, 1.2), (B::TWIGS, 3.0, -0.2)] {
                let spot = (0..30).map(|i| (k + (i / 6) as f64 * 0.4, l + ((i % 6) as f64 - 2.5) * 0.35)).map(|(a, c)| at(a, c)).find(|q| dry(&s, q.0, q.1) && !used.contains(&(q.0.floor(), q.1.floor())));
                if let Some((x, z)) = spot { used.push((x.floor(), z.floor())); edits.push((x.floor() as i32, gy(&s, x, z), z.floor() as i32, b)); }
            }
            s.world.edit_many(&edits); pump(&mut s, &mut g); s.player.pitch = -0.55; s.inv = [None; alw_core::session::INV_SIZE]; for (id, n) in [("stick", 4), ("pebble", 6), ("fiber", 3), ("seeds", 2), ("stone_knife", 1), ("stone_axe", 1), ("stone_pickaxe", 1)] { s.add_item(id, n); } s.sel = 4;
        }
        if scene == "construction" {
            // Une cabane devant la caméra : plancher de dalles, murs de planches, porte entrouverte, deux fenêtres, poteaux d'angle,
            // toit de chaume en pente, escalier de pierre au seuil, échelle contre le pignon. Tout en blocs du jeu.
            use alw_core::blocks::by_name; let b = |n: &str| by_name(n).expect(n); let p = s.player.p; let yaw = s.player.yaw;
            let f = alw_core::shapes::facing_of(yaw.sin(), yaw.cos()); let (fx, fz) = [(0, 1), (1, 0), (0, -1), (-1, 0)][f as usize]; let (sx, sz) = (-fz, fx);
            let (bx, bz) = (p[0].floor() as i32 + fx * 6, p[2].floor() as i32 + fz * 6); let y0 = gy(&s, bx as f64 + 0.5, bz as f64 + 0.5); let mut e = Vec::new();
            let at = |k: i32, l: i32| (bx + fx * k + sx * l, bz + fz * k + sz * l); let turn = |n: &str, t: u8| alw_core::shapes::variant(b(n), (f + t) % 4, false, false, false);
            for k in -1..6 { for l in -4..5 { let (x, z) = at(k, l); e.push((x, y0 - 1, z, B::STONE)); for h in 0..7 { e.push((x, y0 + h, z, 0)); } } }
            for k in 0..5 { for l in -3..4 { let (x, z) = at(k, l); e.push((x, y0, z, b("planks_slab_top"))); } }
            for k in 0..5 { for l in -3..4 { let edge = k == 0 || k == 4 || l == -3 || l == 3; if !edge { continue; } let (x, z) = at(k, l); let corner = (k == 0 || k == 4) && (l == -3 || l == 3);
                for h in 1..4 { let id = if corner { b("wood_pillar") } else if k == 0 && l == 0 && h < 3 { if h == 1 { alw_core::shapes::variant(b("door"), f, false, true, false) } else { alw_core::shapes::variant(b("door"), f, false, true, true) } }
                    else if h == 2 && k == 0 && (l == -2 || l == 2) { alw_core::shapes::variant(b("window"), f, false, false, false) } else if h == 2 && (l == -3 || l == 3) && k == 2 { alw_core::shapes::variant(b("window"), f + 1, false, false, false) } else { B::PLANKS }; e.push((x, y0 + h, z, id)); } } }
            // toit : deux pans de chaume qui montent vers le faîte, une rangée de chaume plein au sommet
            for k in -1..6 { for l in -4..5 { let (x, z) = at(k, l); let side = if l < 0 { 3 } else { 1 }; let h = if l.abs() >= 3 { 4 } else { 5 }; e.push((x, y0 + h, z, if l == 0 { B::THATCH } else { turn("thatch_stairs", side) })); } }
            for k in [0, 4] { for l in -2..3 { let (x, z) = at(k, l); e.push((x, y0 + 4, z, B::PLANKS)); } } // pignons
            let (x, z) = at(-1, 0); e.push((x, y0, z, turn("stone_bricks_stairs", 0)));
            // volets ouverts à côté des fenêtres de façade, avant-toit en escaliers renversés sous le bord du toit
            for l in [-2, 2] { let (x, z) = at(-1, l); let _ = (x, z); } for l in [-3, 3] { let (x, z) = at(-1, l); e.push((x, y0 + 3, z, turn("planks_stairs_top", 2))); }
            for h in 1..4 { let (x, z) = at(2, 4); e.push((x, y0 + h, z, turn("ladder", 3))); }
            s.world.edit_many(&e); pump(&mut s, &mut g); s.player.pitch = -0.12; s.sel = 4;
        }
        if scene == "tenue" {
            // Le personnage vu de dos, en troisième personne : casque, plastron, jambières, bottes et bouclier de fer, le sac au dos.
            for (i, id) in [(0, "iron_helmet"), (1, "iron_chestplate"), (2, "leather_pants"), (3, "leather_boots"), (4, "backpack"), (5, "iron_shield")] { if alw_core::data::def(id).is_some() { s.equip[i] = Some(alw_core::session::Stack::new(id, None)); } }
            s.third = true; s.player.yaw += 2.6; s.player.pitch = -0.2; s.tick(0.0, none, Actions::default());
        }
        if scene == "combat" {
            // Un loup qui arrive de face, bouclier de fer au bras et garde levée, épée en main et coup lourd en charge.
            s.equip[5] = Some(alw_core::session::Stack::new("iron_shield", None)); s.inv[0] = Some(alw_core::session::Stack::new("sword", None)); s.sel = 0; s.player.pitch = -0.08;
            s.tick(0.0, none, Actions::default());
        }
        if scene == "faune" {
            // Une meute de loups qui approche, un sanglier qui fouille le sol, l'arc en main et une flèche plantée devant soi.
            use alw_core::wild::Kind as K; s.inv[0] = Some(alw_core::session::Stack::new("bow", None)); s.sel = 0; s.add_item("arrow", 12); s.player.pitch = -0.05;
            for (k, l, kind) in [(7.0, -1.0, K::Wolf), (8.5, 0.6, K::Wolf), (9.5, -2.2, K::Wolf), (6.0, 3.2, K::Boar)] { let (x, z) = at(k, l); s.wild.put(kind, [x, gy(&s, x, z) as f64, z], s.player.yaw + 3.0 + l * 0.1, false); }
            let (ax, az) = at(3.0, 0.4); s.arrows.push(alw_core::outdoors::Arrow { p: [ax, gy(&s, ax, az) as f64 + 0.25, az], v: [s.player.yaw.sin() * 20.0, -14.0, s.player.yaw.cos() * 20.0], stuck: true, life: 0.0, pow: 1.0 });
        }
        if scene == "chat" { s.chat = Some("/vitesse 20".into()); }
        if scene == "oeufs" { s.set_creative(true); s.inv_open = true; s.tab = alw_core::session::tabs().iter().position(|t| t.0 == "Œufs").unwrap_or(0); }
        if scene == "menagerie" {
            // Toutes les bêtes sculptées en rang devant soi, en plein jour : ours, sanglier, loup, renard, bouquetin, lièvre, lézard, un cerf derrière.
            use alw_core::wild::Kind as K; s.hour = 14.5; s.player.pitch = -0.2; s.sel = 8; s.player.p[1] += 3.0;
            for (kind, l, k) in [(K::Bear, -5.5, 12.0), (K::Boar, -2.8, 11.0), (K::Wolf, -0.6, 11.0), (K::Fox, 1.4, 10.5), (K::Ibex, 3.4, 11.0), (K::Hare, 5.0, 10.0), (K::Lizard, 1.0, 9.0), (K::Crow, -1.8, 9.5)] { let (x, z) = at(k, l); s.wild.put(kind, [x, gy(&s, x, z) as f64, z], s.player.yaw + 1.5, false); }
            let (x, z) = at(14.0, -1.0); s.fauna.put([x, gy(&s, x, z) as f64, z], s.player.yaw + 1.9);
        }
        if scene == "fissure" {
            // Un bloc à moitié cassé à la pioche de pierre : les fissures doivent se voir.
            s.inv[0] = Some(alw_core::session::Stack::new("stone_pickaxe", None)); s.sel = 0; s.player.pitch = -0.75; s.tick(0.0, none, Actions::default());
            for _ in 0..8 { match s.target { Some(t) if alw_core::blocks::is_deco(t.id) => { s.world.edit(t.x, t.y, t.z, 0); pump(&mut s, &mut g); s.tick(0.0, none, Actions::default()); } _ => break } } // les herbes d'abord : c'est le bloc dessous qu'on veut voir se fendre
            for _ in 0..600 { s.stamina = 100.0; s.tick(1.0 / 60.0, none, Actions { mine: true, place: false, ..Default::default() }); if s.progress() > 0.62 { break; } }
            println!("fissure : progression {:.2}, cible {:?}", s.progress(), s.target.map(|t| alw_core::blocks::BLOCKS[t.id as usize].name));
        }
        // L'écran titre : dix secondes de survol, le temps de quitter le départ et d'approcher le premier village.
        if scene == "titre" { for i in 0..300 { s.tour_step(1.0 / 30.0); s.tick(1.0 / 30.0, none, Actions::default()); if i % 30 == 29 { pump(&mut s, &mut g); } } pump(&mut s, &mut g); println!("titre : survol en {:?}", s.player.p); }
        if scene != "fissure" { s.tick(0.0, none, Actions::default()); }
        // Les quatre créatures communes, au crépuscule, à quelques pas : pour juger des modèles en jeu.
        if scene == "creatures" { s.hour = 19.4; for (k, id) in ["rodeur", "brigand", "rodeuse", "rampant"].iter().enumerate() { let (fy, p0) = (s.player.yaw, s.player.p); let (fx, fz) = (fy.sin(), fy.cos()); let side = -2.4 + k as f64 * 1.6;
            // Sur la terre ferme (pas au fond de l'eau) : la première colonne sèche en avançant.
            let (x, z) = (3..12).map(|d| (p0[0] + fx * d as f64 + fz * side, p0[2] + fz * d as f64 - fx * side)).find(|(x, z)| { let y = gy(&s, *x, *z) as i32; !s.world.get(x.floor() as i32, y, z.floor() as i32).is_some_and(alw_core::blocks::is_water) }).unwrap_or((p0[0] + fx * 4.5, p0[2] + fz * 4.5)); let sp = alw_core::hostile::species(id).unwrap(); let i = s.horde.put(sp, [x, gy(&s, x, z) as f64, z], s.player.yaw + 3.14, false); s.horde.foes[i].state = alw_core::hostile::Ai::Detection; } s.tick(0.0, none, Actions::default()); println!("créatures : {} {:?}", s.horde.foes.len(), s.horde.foes.iter().map(|f| f.p).collect::<Vec<_>>()); }
        if scene == "combat" { let (x, z) = at(3.2, 0.3); s.wild.put(alw_core::wild::Kind::Wolf, [x, gy(&s, x, z) as f64, z], s.player.yaw + 3.14, false); s.guard_t = 0.12; s.anim.guard = 1.0; s.charge = 0.5; s.guide = 99; println!("combat : {} bêtes", s.wild.all.len()); } println!("{scene} : établi et feu {:?}, prey {:?}, mort {:?}", s.stations(), s.prey, s.dead);
    }
    let mut menu = hud::Menu { open: scene == "menu" || scene == "commandes", controls: scene == "commandes", weather: 3, quality: 1, hud: 2, ..hud::Menu::new() };
    if scene == "titre" { menu.title = Some(hud::Title::new(vec![hud::WorldInfo { name: "Premier monde".into(), seed: SEED, profile: 1, hours: 130.0, age: "il y a 2 h".into() }], "Premier monde")); }
    let info = if scene == "f3" || scene == "foret" { debug_lines(&s, 240.0, 5_200_000, "Ultra") } else { Vec::new() };
    let t1 = Instant::now(); let mut tris = 0;
    if scene == "arc" || scene == "arc_tiers" { s.anim.bow = 1.0; }
    for k in 0..10 { tris = draw(&mut g, &md, &s, 3.0 + k as f64 / 60.0, w as f32, h as f32, None, [w as f32 * 0.62, h as f32 * 0.4], &menu, &info); g.settle(); }
    println!("10 images, {:.1} M triangles, {:.0} ms par image sur cette carte", tris as f64 / 1e6, t1.elapsed().as_secs_f64() * 100.0);
    let (w, h, px) = g.read_bgr();
    if px.iter().all(|v| *v == 0) { eprintln!("image noire"); std::process::exit(3); }
    let mut b = Vec::with_capacity(px.len() + 54); let size = (px.len() + 54) as u32;
    b.extend_from_slice(b"BM"); b.extend_from_slice(&size.to_le_bytes()); b.extend_from_slice(&[0; 4]); b.extend_from_slice(&54u32.to_le_bytes()); b.extend_from_slice(&40u32.to_le_bytes());
    b.extend_from_slice(&w.to_le_bytes()); b.extend_from_slice(&h.to_le_bytes()); b.extend_from_slice(&1u16.to_le_bytes()); b.extend_from_slice(&24u16.to_le_bytes()); b.extend_from_slice(&[0; 24]); b.extend_from_slice(&px);
    std::fs::write(&a[0], b).expect("écriture de la capture");
    std::process::exit(0); // la carte logicielle peut caler en se fermant : rien à attendre, l'image est écrite
}
/// Les structures exportées des parties précédentes (dossier « structures » à côté du jeu) remplacent leurs patrons.
fn load_structures() {
    let mut bad = Vec::new();
    for e in std::fs::read_dir(beside_exe("structures")).into_iter().flatten().flatten() {
        let p = e.path(); if p.extension().and_then(|x| x.to_str()) != Some("txt") { continue; }
        if let Err(err) = std::fs::read_to_string(&p).map_err(|e| e.to_string()).and_then(|t| alw_core::structures::register(&t)) { bad.push(format!("{} : {err}", p.display())); }
    }
    if !bad.is_empty() { report(&format!("Structures illisibles (ignorées) :\n{}", bad.join("\n"))); }
}
fn main() {
    std::panic::set_hook(Box::new(|i| report(&format!("Erreur : {i}"))));
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--capture") { capture(&args[2..]); return; }
    load_structures();
    let el = EventLoop::new().expect("boucle d'évènements"); el.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    el.run_app(&mut app).expect("boucle principale");
}
