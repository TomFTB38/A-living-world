//! A Living World, moteur final. La partie vit dans alw-core ; ici, la fenêtre, les entrées et l'affichage.
//! Si quelque chose échoue, le détail est écrit dans alw-erreur.txt à côté du programme.
mod font_data;
mod gpu;
mod hud;
use alw_core::{blocks::MODES, gen::WorldGen, player::Input, session::{Actions, Session}, stream::{Preset, HIGH}, view};
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
fn draw(gpu: &mut gpu::Gpu, s: &Session, time: f64, w: f32, h: f32, msg: Option<&str>) -> usize {
    let eye = s.player.eye(); let (proj, vm) = (view::perspective(1.05, w as f64 / h as f64, 0.08), view::look_dir(eye, s.player.forward())); let vp = view::mul(&proj, &vm);
    let ui = hud::Ui::build(s, w, h, msg);
    gpu.render(&vp, &vm, &proj, eye, time, &view::sky_at(s.hour), 0.42, s.target.map(|t| ([t.x, t.y, t.z], s.progress() as f32)), &ui)
}
struct Game { window: Arc<Window>, gpu: gpu::Gpu, s: Session, keys: HashSet<KeyCode>, grabbed: bool, time: f64, run_time: bool, lmb: bool, rmb: bool, place_t: f64, msg: Option<(String, f64)>, save_t: f64, last: Instant, frames: u32, fps_t: f64, tris: usize }
#[derive(Default)]
struct App { game: Option<Game> }
impl Game {
    fn grab(&mut self, on: bool) {
        self.grabbed = on; if !on { self.lmb = false; self.rmb = false; }
        if on { if self.window.set_cursor_grab(CursorGrabMode::Confined).is_err() { let _ = self.window.set_cursor_grab(CursorGrabMode::Locked); } } else { let _ = self.window.set_cursor_grab(CursorGrabMode::None); }
        self.window.set_cursor_visible(!on);
    }
    fn say(&mut self, m: String) { self.msg = Some((m, 2.5)); }
    fn save(&mut self) { if std::fs::write(beside_exe("monde.alw"), self.s.save_text()).is_err() { self.say("Sauvegarde impossible".into()); } self.s.dirty = false; }
    fn frame(&mut self) {
        let now = Instant::now(); let real = (now - self.last).as_secs_f64(); let dt = real.min(0.05); self.last = now; self.time += dt;
        if self.run_time { self.s.hour = (self.s.hour + dt * 24.0 / 1500.0) % 24.0; }
        let k = |c: KeyCode| self.keys.contains(&c);
        // Regarder vers +z place +x à gauche de l'écran : la droite vaut -x.
        let ax = |a: bool, b: bool| (a as i32 - b as i32) as f64;
        let input = Input { x: ax(k(KeyCode::KeyA) || k(KeyCode::KeyQ), k(KeyCode::KeyD)), z: ax(k(KeyCode::KeyW) || k(KeyCode::KeyZ), k(KeyCode::KeyS)), jump: k(KeyCode::Space), sprint: k(KeyCode::ShiftLeft), crouch: k(KeyCode::ControlLeft) };
        self.place_t -= dt; let place = self.rmb && self.place_t <= 0.0; if place { self.place_t = 0.22; }
        self.s.tick(dt, input, Actions { mine: self.lmb, place });
        let p = self.s.player.p; let (built, gone) = self.s.world.update(p[0], p[2]);
        for b in built { self.gpu.upload(b); }
        for g in gone { self.gpu.forget(g); }
        self.save_t += real; if self.save_t > 30.0 { self.save_t = 0.0; self.save(); }
        if let Some(m) = &mut self.msg { m.1 -= real; if m.1 <= 0.0 { self.msg = None; } }
        let size = self.window.inner_size();
        self.tris = draw(&mut self.gpu, &self.s, self.time, size.width.max(1) as f32, size.height.max(1) as f32, self.msg.as_ref().map(|m| m.0.as_str()));
        self.frames += 1; self.fps_t += real;
        if self.fps_t >= 0.5 {
            let fps = self.frames as f64 / self.fps_t; self.frames = 0; self.fps_t = 0.0;
            self.window.set_title(&format!("A Living World  |  {fps:.0} FPS  |  {:.1} M triangles  |  {} chunks  |  {} en attente", self.tris as f64 / 1e6, self.s.world.loaded(), self.s.world.waiting()));
        }
    }
}
impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.game.is_some() { return; }
        let window = Arc::new(el.create_window(Window::default_attributes().with_title("A Living World").with_inner_size(winit::dpi::LogicalSize::new(1600.0, 900.0))).expect("création de la fenêtre"));
        let size = window.inner_size();
        let gpu = pollster::block_on(gpu::Gpu::new(Some(window.clone()), size.width, size.height, gpu::Options { shadow: 4096, fallback: false, hole: (HIGH.radius as f32 - 0.6) * 32.0 }));
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).saturating_sub(2).clamp(2, 12);
        let mut s = Session::new(SEED, &HIGH, threads, spawn_point(SEED));
        if let Ok(t) = std::fs::read_to_string(beside_exe("monde.alw")) { if let Err(e) = s.load_text(&t) { report(&format!("Sauvegarde illisible : {e}")); } }
        let mut g = Game { window, gpu, s, keys: HashSet::new(), grabbed: false, time: 0.0, run_time: true, lmb: false, rmb: false, place_t: 0.0, msg: None, save_t: 0.0, last: Instant::now(), frames: 0, fps_t: 0.0, tris: 0 };
        g.grab(true); self.game = Some(g);
    }
    fn window_event(&mut self, el: &ActiveEventLoop, _: WindowId, e: WindowEvent) {
        let Some(g) = self.game.as_mut() else { return };
        match e {
            WindowEvent::CloseRequested => { g.save(); el.exit(); }
            WindowEvent::Resized(s) => g.gpu.resize(s.width, s.height),
            WindowEvent::Focused(false) => { g.keys.clear(); g.grab(false); }
            WindowEvent::MouseInput { state, button, .. } => {
                let down = state == ElementState::Pressed;
                if !g.grabbed { if down { g.grab(true); } return; }
                match button { MouseButton::Left => g.lmb = down, MouseButton::Right => { g.rmb = down; if down { g.place_t = 0.0; } } _ => {} }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let d = match delta { MouseScrollDelta::LineDelta(_, y) => y, MouseScrollDelta::PixelDelta(p) => p.y as f32 };
                if d != 0.0 { g.s.sel = (g.s.sel as i32 + if d > 0.0 { -1 } else { 1 }).rem_euclid(9) as usize; }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(c) = event.physical_key else { return };
                if event.state != ElementState::Pressed { g.keys.remove(&c); return; }
                if !event.repeat { match c {
                    KeyCode::Escape => g.grab(false),
                    KeyCode::KeyF => { if g.s.creative { g.s.player.fly = !g.s.player.fly; let m = if g.s.player.fly { "Vol activé" } else { "Vol désactivé" }; g.say(m.into()); } else { g.say("Le vol demande le mode créatif (G)".into()); } }
                    KeyCode::KeyG => { let on = !g.s.creative; g.s.set_creative(on); g.say(if on { "Mode créatif : casse instantanée, blocs illimités, vol avec F" } else { "Mode survie" }.into()); }
                    KeyCode::KeyR => { g.s.orient = (g.s.orient + 1) % MODES.len() as u8; g.say(format!("Orientation : {}", MODES[g.s.orient as usize])); }
                    KeyCode::KeyT => g.s.hour = (g.s.hour + 1.0) % 24.0,
                    KeyCode::KeyY => g.run_time = !g.run_time,
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
        if let (true, DeviceEvent::MouseMotion { delta }) = (g.grabbed, e) { g.s.player.yaw -= delta.0 * 0.0023; g.s.player.pitch = (g.s.player.pitch - delta.1 * 0.0023).clamp(-1.5, 1.5); }
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) { if let Some(g) = &self.game { g.window.request_redraw(); } }
}
/// Banc d'essai sans fenêtre : charge une scène, joue un scénario, rend quelques images, écrit un BMP.
/// a-living-world --capture sortie.bmp x y z vx vy vz [heure] [rayon] [largeur] [hauteur] [ombre] [scenario]
fn capture(a: &[String]) {
    let n = |i: usize, d: f64| a.get(i).and_then(|v| v.parse().ok()).unwrap_or(d);
    let (eye, look) = ([n(1, -38.5), n(2, 51.4), n(3, 0.5)], [n(4, -20.0), n(5, 52.0), n(6, -30.0)]);
    let (radius, w, h, shadow) = (n(8, 6.0) as i32, n(9, 1280.0) as u32, n(10, 720.0) as u32, n(11, 2048.0) as u32);
    let mut g = pollster::block_on(gpu::Gpu::new(None, w, h, gpu::Options { shadow, fallback: true, hole: (radius as f32 - 0.6) * 32.0 }));
    let mut s = Session::new(SEED, &Preset { radius, lod1: 2.3, lod2: 4.6 }, 4, [eye[0], eye[1] - 1.62, eye[2]]); s.hour = n(7, 16.4); s.player.fly = true;
    let pump = |s: &mut Session, g: &mut gpu::Gpu| loop { let p = s.player.p; let (b, f) = s.world.update(p[0], p[2]); for c in b { g.upload(c); } for k in f { g.forget(k); } if s.world.waiting() == 0 { break; } std::thread::sleep(std::time::Duration::from_millis(5)); };
    let t0 = Instant::now(); pump(&mut s, &mut g); println!("{} chunks chargés en {:.1} s", s.world.loaded(), t0.elapsed().as_secs_f64());
    let f = view::norm([look[0] - eye[0], look[1] - eye[1], look[2] - eye[2]]); s.player.yaw = f[0].atan2(f[2]); s.player.pitch = f[1].asin();
    let none = Input::default(); s.tick(0.0, none, Actions::default());
    if a.get(12).map(String::as_str) == Some("jeu") {
        // Scénario : casser le bloc visé, puis poser de la pierre, avec les vrais chemins du jeu.
        s.sel = 0; let t = s.target.expect("un bloc visé"); let before = s.inv.iter().flatten().count();
        for _ in 0..240 { s.tick(1.0 / 60.0, none, Actions { mine: true, place: false }); if alw_core::player::Voxels::get(&s.world, t.x, t.y, t.z) == Some(0) { break; } }
        println!("casse : {} -> {:?}, objets {} -> {}", alw_core::blocks::BLOCKS[t.id as usize].name, alw_core::player::Voxels::get(&s.world, t.x, t.y, t.z), before, s.inv.iter().flatten().count());
        s.sel = 5; s.tick(0.0, none, Actions::default()); s.tick(0.0, none, Actions { mine: false, place: true }); pump(&mut s, &mut g); s.tick(0.0, none, Actions::default()); println!("pose : cible {:?}", s.target.map(|t| alw_core::blocks::BLOCKS[t.id as usize].name));
    }
    let t1 = Instant::now(); let mut tris = 0;
    for k in 0..10 { tris = draw(&mut g, &s, 3.0 + k as f64 / 60.0, w as f32, h as f32, if a.len() > 12 { Some("Orientation : selon le regard") } else { None }); }
    println!("10 images, {:.1} M triangles, {:.0} ms par image sur cette carte", tris as f64 / 1e6, t1.elapsed().as_secs_f64() * 100.0);
    let (w, h, px) = g.read_bgr();
    let mut b = Vec::with_capacity(px.len() + 54); let size = (px.len() + 54) as u32;
    b.extend_from_slice(b"BM"); b.extend_from_slice(&size.to_le_bytes()); b.extend_from_slice(&[0; 4]); b.extend_from_slice(&54u32.to_le_bytes()); b.extend_from_slice(&40u32.to_le_bytes());
    b.extend_from_slice(&w.to_le_bytes()); b.extend_from_slice(&h.to_le_bytes()); b.extend_from_slice(&1u16.to_le_bytes()); b.extend_from_slice(&24u16.to_le_bytes()); b.extend_from_slice(&[0; 24]); b.extend_from_slice(&px);
    std::fs::write(&a[0], b).expect("écriture de la capture");
}
fn main() {
    std::panic::set_hook(Box::new(|i| report(&format!("Erreur : {i}"))));
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--capture") { capture(&args[2..]); return; }
    let el = EventLoop::new().expect("boucle d'évènements"); el.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    el.run_app(&mut app).expect("boucle principale");
}
