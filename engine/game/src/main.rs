//! A Living World, moteur final. La partie vit dans alw-core ; ici, la fenêtre, les entrées et l'affichage.
//! Si quelque chose échoue, le détail est écrit dans alw-erreur.txt à côté du programme.
mod font_data;
mod gpu;
mod hud;
mod sound;
use alw_core::{actors::Models, audio::{Mood, SoundScape}, blocks::MODES, gen::WorldGen, player::Input, session::{Actions, Session}, stream::{Preset, HIGH, ULTRA}, view, weather::{self, Kind, KINDS}};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use winit::{application::ApplicationHandler, event::{DeviceEvent, DeviceId, ElementState, MouseButton, MouseScrollDelta, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{CursorGrabMode, Window, WindowId}};

const SEED: i32 = 20261104;
fn beside_exe(name: &str) -> std::path::PathBuf { std::env::current_exe().ok().and_then(|e| e.parent().map(|d| d.join(name))).unwrap_or_else(|| name.into()) }
pub fn report(msg: &str) { eprintln!("{msg}"); let _ = std::fs::write(beside_exe("alw-erreur.txt"), msg); }
/// Un point de départ au sec : sur un chemin si possible, en spirale autour de l'origine.
fn spawn_point(seed: i32) -> [f64; 3] {
    let g = WorldGen::new(seed); let mut best = [0.5, 80.0, 0.5];
    for r in 0..40 { for k in 0..16 {
        let a = k as f64 / 16.0 * std::f64::consts::TAU; let (x, z) = ((a.cos() * r as f64 * 6.0).floor() + 0.5, (a.sin() * r as f64 * 6.0).floor() + 0.5);
        let c = g.column(x, z);
        if c.water == 0.0 { best = [x, c.h + 3.0, z]; if c.path > 0.5 { return best; } }
    } }
    best
}
fn draw(gpu: &mut gpu::Gpu, md: &Models, s: &Session, time: f64, w: f32, h: f32, msg: Option<&str>, mouse: [f32; 2], menu: &hud::Menu, info: &[String]) -> usize {
    let (proj, vm) = (view::perspective(1.05, w as f64 / h as f64, 0.08), view::look_dir(s.eye, s.look)); let vp = view::mul(&proj, &vm);
    let ui = hud::Ui::build(s, w, h, msg, mouse, menu, info); let draws = s.draws(md, &vm); let mut sky = view::sky_at(s.hour); if let Some(f) = &s.falling { gpu.set_fall(s.fall_serial, &f.data); }
    gpu.lights = s.lights(s.eye, gpu::LIGHTS);
    let mut env = weather::apply(&mut sky, s.weather.state, s.covered(s.eye)); env.bright = hud::bright(menu.vol[3]); env.under = s.under(); let night = (1.0 - (sky.dir[1] * 4.0).clamp(0.0, 1.0)) as f32; env.flies = s.deep as f32 * (0.55 + 0.9 * night); env.bright *= 1.0 - 0.42 * s.deep as f32; env.fog_k += 2.6 * s.deep as f32; env.mist += 0.0035 * s.deep as f32; for v in &mut sky.top { *v *= 1.0 - 0.3 * s.deep as f32; }
    gpu.render(&vp, &vm, &proj, s.eye, time, &sky, env, s.target.map(|t| ([t.x, t.y, t.z], s.progress() as f32)), s.ghost(), &draws, &ui)
}
/// Réglages enregistrés ; un fichier plus ancien garde ses valeurs et prend les nouvelles par défaut.
fn read_settings() -> hud::Menu {
    let mut m = hud::Menu { open: false, credits: false, weather: 0, mood: 0, vol: [0.7, 0.7, 0.5, 0.5], quality: 0, confirm: false };
    let Ok(t) = std::fs::read_to_string(beside_exe("reglages.alw")) else { return m }; let v: Vec<f32> = t.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    if v.len() >= 5 { m.vol = [v[0].clamp(0.0, 1.0), v[1].clamp(0.0, 1.0), v[2].clamp(0.0, 1.0), v.get(5).copied().unwrap_or(0.5).clamp(0.0, 1.0)]; m.weather = (v[3] as usize).min(hud::WEATHERS.len() - 1); m.mood = (v[4] as usize).min(hud::MOODS.len() - 1); m.quality = v.get(6).map_or(0, |q| (*q as usize).min(1)); }
    m
}
/// Panneau F3 : ce qu'il faut pour comprendre où l'on est et comment tourne le jeu.
fn debug_lines(s: &Session, fps: f64, tris: usize, quality: &str) -> Vec<String> {
    let (p, m) = (s.player.p, s.mix()); let mut order: Vec<usize> = (0..alw_core::biome::COUNT).collect(); order.sort_by(|a, b| m.w[*b].total_cmp(&m.w[*a]));
    let t = s.target.map_or("rien".to_string(), |t| format!("{} ({}, {}, {})", alw_core::blocks::BLOCKS[t.id as usize].name, t.x, t.y, t.z));
    vec![format!("{fps:.0} FPS  ·  {:.1} M triangles  ·  qualité {quality}", tris as f64 / 1e6), format!("Graine {}", s.seed), format!("Position {:.1}  {:.1}  {:.1}", p[0], p[1], p[2]), format!("Chunk {} {}  ·  {} chargés  ·  {} en attente", (p[0] / 32.0).floor(), (p[2] / 32.0).floor(), s.world.loaded(), s.world.waiting()),
        format!("Biome {}", alw_core::biome::NAMES[order[0]]), format!("Mélange {} {:.0} %, {} {:.0} %", alw_core::biome::NAMES[order[0]], m.w[order[0]] * 100.0, alw_core::biome::NAMES[order[1]], m.w[order[1]] * 100.0), format!("Climat : température {:.2}, humidité {:.2}", m.temp, m.wet),
        format!("Heure {:02}:{:02}  ·  météo {}", s.hour as u32, (s.hour.fract() * 60.0) as u32, s.weather.kind.name()), format!("Visé : {t}"), format!("Animaux : {} cerfs, {} oiseaux", s.fauna.deer.len(), s.fauna.birds.len())]
}
/// Graine de la partie : celle de la sauvegarde, sinon celle choisie par « Nouveau monde », sinon celle du monde de départ.
fn world_seed() -> i32 {
    let save = std::fs::read_to_string(beside_exe("monde.alw")).ok().and_then(|t| t.lines().find_map(|l| l.strip_prefix("seed ").and_then(|v| v.trim().parse().ok())));
    save.or_else(|| std::fs::read_to_string(beside_exe("graine.alw")).ok().and_then(|t| t.trim().parse().ok())).unwrap_or(SEED)
}
/// Nouveau monde : l'ancien est mis de côté, jamais effacé, puis le jeu repart sur une graine neuve.
fn new_world() {
    let _ = std::fs::rename(beside_exe("monde.alw"), beside_exe("monde-ancien.alw"));
    let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(1, |d| d.subsec_nanos() as i32 ^ d.as_secs() as i32);
    if std::fs::write(beside_exe("graine.alw"), seed.to_string()).is_ok() { if let Ok(e) = std::env::current_exe() { let _ = std::process::Command::new(e).spawn(); } }
    std::process::exit(0);
}
struct Game { debug: bool, fps: f64, menu: hud::Menu, drag: Option<usize>, half: Option<usize>, window: Arc<Window>, gpu: gpu::Gpu, md: Models, mouse: [f32; 2], s: Session, keys: HashSet<KeyCode>, grabbed: bool, time: f64, run_time: bool, lmb: bool, rmb: bool, place_t: f64, msg: Option<(String, f64)>, save_t: f64, last: Instant, frames: u32, fps_t: f64, tris: usize }
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
    fn write_settings(&self) { let m = &self.menu; let _ = std::fs::write(beside_exe("reglages.alw"), format!("{} {} {} {} {} {} {}\n", m.vol[0], m.vol[1], m.vol[2], m.weather, m.mood, m.vol[3], m.quality)); }
    fn slide(&mut self, i: usize) {
        let sz = self.window.inner_size(); let r = hud::menu_layout(sz.width as f32, sz.height as f32, &self.menu).into_iter().find(|x| x.0 == hud::M::Vol(i)).map(|x| x.1);
        if let Some(r) = r { self.menu.vol[i] = ((self.mouse[0] - r[0]) / r[2]).clamp(0.0, 1.0); self.apply(); }
    }
    fn close_menu(&mut self) { self.menu.open = false; self.menu.credits = false; self.drag = None; self.write_settings(); self.grab(true); }
    fn save(&mut self) { if std::fs::write(beside_exe("monde.alw"), self.s.save_text()).is_err() { self.say("Sauvegarde impossible".into()); } self.s.dirty = false; }
    fn frame(&mut self) {
        let now = Instant::now(); let real = (now - self.last).as_secs_f64(); let dt = if self.menu.open { 0.0 } else { real.min(0.05) }; self.last = now; self.time += dt;
        if self.run_time { self.s.hour = (self.s.hour + dt * 24.0 / 1500.0) % 24.0; }
        let k = |c: KeyCode| self.keys.contains(&c);
        // Regarder vers +z place +x à gauche de l'écran : la droite vaut -x.
        let ax = |a: bool, b: bool| (a as i32 - b as i32) as f64;
        let open = self.s.inv_open || self.menu.open; let input = if open { Input::default() } else { Input { x: ax(k(KeyCode::KeyA) || k(KeyCode::KeyQ), k(KeyCode::KeyD)), z: ax(k(KeyCode::KeyW) || k(KeyCode::KeyZ), k(KeyCode::KeyS)), jump: k(KeyCode::Space), sprint: k(KeyCode::ShiftLeft), crouch: k(KeyCode::ControlLeft), roll: k(KeyCode::KeyV) } };
        self.place_t -= dt; let place = self.rmb && self.place_t <= 0.0; if place { self.place_t = 0.22; }
        self.s.tick(dt, input, Actions { mine: self.lmb, place });
        let (built, gone) = self.s.sync();
        for b in built { self.gpu.upload(b); }
        for g in gone { self.gpu.forget(g); }
        self.save_t += real; if self.save_t > 30.0 { self.save_t = 0.0; self.save(); }
        if let Some(m) = &mut self.msg { m.1 -= real; if m.1 <= 0.0 { self.msg = None; } }
        let size = self.window.inner_size(); let info = if self.debug { debug_lines(&self.s, self.fps, self.tris, if self.menu.quality == 1 { "Ultra" } else { "Haut" }) } else { Vec::new() };
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
        let window = Arc::new(el.create_window(Window::default_attributes().with_title("A Living World").with_inner_size(winit::dpi::LogicalSize::new(1600.0, 900.0))).expect("création de la fenêtre"));
        let size = window.inner_size(); let md = Models::build(); let menu = read_settings(); let ultra = menu.quality == 1; let preset = if ultra { ULTRA } else { HIGH };
        let gpu = pollster::block_on(gpu::Gpu::new(Some(window.clone()), size.width, size.height, gpu::Options { shadow: if ultra { 6144 } else { 4096 }, fallback: false, hole: (preset.radius as f32 - 0.6) * 32.0, cascades: if ultra { gpu::CASCADES_ULTRA } else { gpu::CASCADES } }, &md));
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).saturating_sub(2).clamp(2, 12);
        let seed = world_seed(); let mut s = Session::new(seed, &preset, threads, spawn_point(seed));
        if let Ok(t) = std::fs::read_to_string(beside_exe("monde.alw")) { if let Err(e) = s.load_text(&t) { report(&format!("Sauvegarde illisible : {e}")); } }
        s.sound = SoundScape::new(sound::start());
        let mut g = Game { debug: false, fps: 0.0, menu, drag: None, half: None, window, gpu, md, mouse: [0.0; 2], s, keys: HashSet::new(), grabbed: false, time: 0.0, run_time: true, lmb: false, rmb: false, place_t: 0.0, msg: None, save_t: 0.0, last: Instant::now(), frames: 0, fps_t: 0.0, tris: 0 };
        g.apply(); g.grab(true); self.game = Some(g);
    }
    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, e: WindowEvent) {
        let Some(g) = self.game.as_mut() else { return };
        match e {
            WindowEvent::CloseRequested => { g.save(); g.write_settings(); std::process::exit(0); }
            WindowEvent::Resized(s) => g.gpu.resize(s.width, s.height),
            WindowEvent::Focused(false) => { g.keys.clear(); g.grab(false); }
            WindowEvent::MouseInput { state, button, .. } => {
                let down = state == ElementState::Pressed;
                let sz = g.window.inner_size(); let (w, h) = (sz.width as f32, sz.height as f32);
                if g.menu.open {
                    if button != MouseButton::Left { return; }
                    if !down { g.drag = None; g.write_settings(); return; }
                    let hit = hud::menu_at(w, h, &g.menu, g.mouse); if hit != Some(hud::M::NewWorld) { g.menu.confirm = false; }
                    match hit {
                        Some(hud::M::NewWorld) => { if g.menu.confirm { g.save(); g.write_settings(); new_world(); } g.menu.confirm = true; g.say("Votre monde actuel sera gardé dans monde-ancien.alw : cliquez à nouveau pour confirmer".into()); }
                        Some(hud::M::Resume) => g.close_menu(),
                        Some(hud::M::Weather(i)) => { g.menu.weather = i; g.apply(); }
                        Some(hud::M::Mood(i)) => { g.menu.mood = i; g.apply(); }
                        Some(hud::M::Quality(i)) => { g.menu.quality = i; g.say("Qualité appliquée au prochain lancement".into()); }
                        Some(hud::M::Vol(i)) => { g.drag = Some(i); g.slide(i); }
                        Some(hud::M::Credits) => g.menu.credits = true,
                        Some(hud::M::Back) => g.menu.credits = false,
                        Some(hud::M::Quit) => { g.save(); g.write_settings(); std::process::exit(0); }
                        None => {}
                    }
                    return;
                }
                if g.s.inv_open {
                    let at = hud::hit_at(g.s.creative, g.s.tab, w, h, g.mouse); let shift = g.keys.contains(&KeyCode::ShiftLeft) || g.keys.contains(&KeyCode::ShiftRight); let slot = if let Some(hud::Hit::Slot(i)) = at { Some(i) } else { None };
                    match (button, down) {
                        (MouseButton::Left, true) => match at {
                            Some(hud::Hit::Tab(i)) => g.s.tab = i,
                            Some(hud::Hit::Slot(i)) => if shift { g.s.quick_slot(i) } else { g.s.grab_slot(i) },
                            Some(hud::Hit::Catalog(id)) => if shift { g.s.quick_catalog(id) } else if g.s.cursor.is_none() { g.s.grab_catalog(id) },
                            None => {}
                        },
                        (MouseButton::Left, false) => g.s.drop_slot(slot),
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
            WindowEvent::CursorMoved { position, .. } => { g.mouse = [position.x as f32, position.y as f32]; if let Some(i) = g.drag { g.slide(i); } }
            WindowEvent::MouseWheel { delta, .. } => {
                let d = match delta { MouseScrollDelta::LineDelta(_, y) => y, MouseScrollDelta::PixelDelta(p) => p.y as f32 };
                if d != 0.0 { g.s.sel = (g.s.sel as i32 + if d > 0.0 { -1 } else { 1 }).rem_euclid(9) as usize; }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(c) = event.physical_key else { return };
                if event.state != ElementState::Pressed { g.keys.remove(&c); return; }
                if !event.repeat { match c {
                    KeyCode::Escape => { if g.s.inv_open { g.s.inv_open = false; g.s.drop_slot(None); g.grab(true); } else if g.menu.open { g.close_menu(); } else { g.menu.open = true; g.grab(false); } }
                    _ if g.menu.open => {}
                    KeyCode::KeyE => { let on = !g.s.inv_open; g.s.inv_open = on; if !on { g.s.drop_slot(None); } g.grab(!on); }
                    KeyCode::KeyC => { g.s.third = !g.s.third; let m = if g.s.third { "Troisième personne" } else { "Première personne" }; g.say(m.into()); }
                    KeyCode::KeyF => { if g.s.creative { g.s.player.fly = !g.s.player.fly; let m = if g.s.player.fly { "Vol activé" } else { "Vol désactivé" }; g.say(m.into()); } else { g.say("Le vol demande le mode créatif (G)".into()); } }
                    KeyCode::KeyG => { let on = !g.s.creative; g.s.set_creative(on); g.say(if on { "Mode créatif : casse instantanée, blocs illimités, vol avec F" } else { "Mode survie" }.into()); }
                    KeyCode::KeyR => { g.s.orient = (g.s.orient + 1) % MODES.len() as u8; g.say(format!("Orientation : {}", MODES[g.s.orient as usize])); }
                    KeyCode::KeyT => g.s.hour = (g.s.hour + 1.0) % 24.0,
                    KeyCode::KeyY => g.run_time = !g.run_time,
                    KeyCode::F3 => g.debug = !g.debug,
                    KeyCode::F5 => { g.save(); g.say("Monde sauvegardé".into()); }
                    KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::Digit4 | KeyCode::Digit5 | KeyCode::Digit6 | KeyCode::Digit7 | KeyCode::Digit8 | KeyCode::Digit9 => g.s.sel = c as usize - KeyCode::Digit1 as usize,
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
        if let (true, DeviceEvent::MouseMotion { delta }) = (g.grabbed && !g.s.inv_open && !g.menu.open, e) { g.s.player.yaw -= delta.0 * 0.0023; g.s.player.pitch = (g.s.player.pitch - delta.1 * 0.0023).clamp(-1.5, 1.5); }
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
    let mut s = Session::new(SEED, &(if scene == "ultra" { Preset { radius, ..ULTRA } } else { Preset { radius, lod1: 2.3, lod2: 4.6 } }), 4, [eye[0], eye[1] - 1.62, eye[2]]); s.hour = n(7, 16.4); s.player.fly = true;
    let pump = |s: &mut Session, g: &mut gpu::Gpu| loop { let p = s.player.p; let (b, f) = s.world.update(p[0], p[2]); for c in b { g.upload(c); } for k in f { g.forget(k); } if s.world.waiting() == 0 { break; } std::thread::sleep(std::time::Duration::from_millis(5)); };
    let t0 = Instant::now(); pump(&mut s, &mut g); println!("{} chunks chargés en {:.1} s", s.world.loaded(), t0.elapsed().as_secs_f64());
    let f = view::norm([look[0] - eye[0], look[1] - eye[1], look[2] - eye[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin();
    let none = Input::default(); s.third = scene == "tiers"; s.tick(0.0, none, Actions::default());
    if scene == "tiers" { s.sel = 1; s.tick(0.0, none, Actions::default()); }
    if scene == "bras" { s.sel = 0; for _ in 0..6 { s.tick(1.0 / 60.0, none, Actions { mine: true, place: false }); } }
    if scene == "inv" { s.inv_open = true; s.grab_slot(5); s.tick(0.0, none, Actions::default()); }
    if scene == "creatif" || scene == "roches" { s.set_creative(true); s.inv_open = true; s.tab = if scene == "roches" { 2 } else { 0 }; s.tick(0.0, none, Actions::default()); }
    if scene == "pluie" { s.weather.snap(Kind::Pluie); s.sel = 8; println!("pluie : couvert {}", s.covered(s.eye)); } if scene == "brume" { s.weather.snap(Kind::Brouillard); s.sel = 8; }
    if scene == "abattage" {
        // Un vrai arbre près de la caméra : la hache pendant dix secondes, puis on regarde tomber le tronc.
        let (bx, by, bz) = (eye[0].floor() as i32, eye[1].floor() as i32, eye[2].floor() as i32); let mut spot = None; let n = 34i32;
        'c: for r in 4..n { for dz in -r..=r { for dx in -r..=r { if i32::abs(dx).max(i32::abs(dz)) != r { continue; } for y in (by - 12)..(by + 6) { let (x, z) = (bx + dx, bz + dz); if alw_core::player::Voxels::get(&s.world, x, y, z).is_some_and(alw_core::blocks::is_log) && alw_core::felling::find_tree(&s.world, x, y, z).is_some_and(|t| t.logs.len() > 30) { spot = Some((x, y + 1, z)); break 'c; } } } } }
        let (x, y, z) = spot.expect("un arbre"); let e = [x as f64 + 6.5, y as f64 + 3.0, z as f64 + 0.5]; let mut clear = Vec::new(); for k in 1..8 { for h in 0..5 { for w in -1..=1 { if alw_core::player::Voxels::get(&s.world, x + k, y + h, z + w).is_some_and(|b| !alw_core::blocks::is_log(b)) { clear.push((x + k, y + h, z + w, 0)); } } } } s.world.edit_many(&clear); pump(&mut s, &mut g);
        s.player.p = [e[0], e[1] - 1.62, e[2]]; let d = view::norm([x as f64 + 0.5 - e[0], y as f64 + 0.5 - e[1], z as f64 + 0.5 - e[2]]); s.player.yaw = d[0].atan2(d[2]); s.player.pitch = d[1].asin(); s.sel = 1; s.third = false;
        for _ in 0..640 { s.tick(1.0 / 60.0, none, Actions { mine: true, place: false }); if s.falling.is_some() { break; } }
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
    if scene == "foret" {
        let w = WorldGen::new(SEED); let mut found = None; 'p: for r in 8..500 { for k in 0..96 { let a = k as f64 / 96.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 16.0, a.sin() * r as f64 * 16.0); let c = w.column(x, z); if c.mix.strong(alw_core::biome::Biome::DeepForest) > 0.97 && c.water == 0.0 && c.path == 0.0 { found = Some((x, z, a, c.h)); break 'p; } } }
        println!("forêt : {found:?}"); if let Some((x, z, a, h)) = found { s.sel = 8; s.third = false; s.player.fly = true; s.player.p = [x, h + 1.0, z]; let f = view::norm([a.cos(), 0.1, a.sin()]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin(); g.settle(); pump(&mut s, &mut g); g.settle(); s.tick(0.0, none, Actions::default()); s.deep = 1.0; s.tick(0.0, none, Actions::default()); }
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
    if scene == "jeu" {
        // Scénario : casser le bloc visé, puis poser de la pierre, avec les vrais chemins du jeu.
        s.sel = 0; if s.target.is_none() { s.player.pitch = -0.9; s.tick(0.0, none, Actions::default()); } let t = s.target.expect("un bloc visé"); let before = s.inv.iter().flatten().count();
        for _ in 0..240 { s.tick(1.0 / 60.0, none, Actions { mine: true, place: false }); if alw_core::player::Voxels::get(&s.world, t.x, t.y, t.z) == Some(0) { break; } }
        println!("casse : {} -> {:?}, objets {} -> {}", alw_core::blocks::BLOCKS[t.id as usize].name, alw_core::player::Voxels::get(&s.world, t.x, t.y, t.z), before, s.inv.iter().flatten().count());
        s.sel = 5; s.tick(0.0, none, Actions::default()); s.tick(0.0, none, Actions { mine: false, place: true }); pump(&mut s, &mut g); s.tick(0.0, none, Actions::default()); println!("pose : cible {:?}", s.target.map(|t| alw_core::blocks::BLOCKS[t.id as usize].name));
    }
    let menu = hud::Menu { open: scene == "menu", credits: false, weather: 3, mood: 0, vol: [0.7, 0.7, 0.5, 0.5], quality: 1, confirm: false };
    let info = if scene == "f3" || scene == "foret" { debug_lines(&s, 240.0, 5_200_000, "Ultra") } else { Vec::new() };
    let t1 = Instant::now(); let mut tris = 0;
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
fn main() {
    std::panic::set_hook(Box::new(|i| report(&format!("Erreur : {i}"))));
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--capture") { capture(&args[2..]); return; }
    let el = EventLoop::new().expect("boucle d'évènements"); el.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    el.run_app(&mut app).expect("boucle principale");
}
