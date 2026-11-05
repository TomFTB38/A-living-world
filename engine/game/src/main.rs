//! A Living World, moteur final. Premier palier : fenêtre, monde réel, déplacement.
//! Si quelque chose échoue, le détail est écrit dans alw-erreur.txt à côté du programme.
mod gpu;
use alw_core::{gen::WorldGen, player::{Input, Player}, stream::{Streamer, HIGH}, view};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use winit::{application::ApplicationHandler, event::{DeviceEvent, DeviceId, ElementState, MouseButton, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{CursorGrabMode, Window, WindowId}};

const SEED: i32 = 20261104;
pub fn report(msg: &str) {
    eprintln!("{msg}");
    let p = std::env::current_exe().ok().and_then(|e| e.parent().map(|d| d.join("alw-erreur.txt"))).unwrap_or_else(|| "alw-erreur.txt".into());
    let _ = std::fs::write(p, msg);
}
/// Un point de départ au sec : sur un chemin si possible, en spirale autour de l'origine.
fn spawn_point(seed: i32) -> (f64, f64, f64) {
    let g = WorldGen::new(seed); let mut best = (0.5, 0.5, 80.0);
    for r in 0..40 { for k in 0..16 {
        let a = k as f64 / 16.0 * std::f64::consts::TAU; let (x, z) = ((a.cos() * r as f64 * 6.0).floor() + 0.5, (a.sin() * r as f64 * 6.0).floor() + 0.5);
        let c = g.column(x, z);
        if c.water == 0.0 { best = (x, z, c.h + 3.0); if c.path > 0.5 { return best; } }
    } }
    best
}
struct Game { window: Arc<Window>, gpu: gpu::Gpu, world: Streamer, player: Player, keys: HashSet<KeyCode>, grabbed: bool, hour: f64, time: f64, last: Instant, frames: u32, fps_t: f64, tris: usize }
#[derive(Default)]
struct App { game: Option<Game> }
impl Game {
    fn grab(&mut self, on: bool) {
        self.grabbed = on;
        if on { if self.window.set_cursor_grab(CursorGrabMode::Confined).is_err() { let _ = self.window.set_cursor_grab(CursorGrabMode::Locked); } } else { let _ = self.window.set_cursor_grab(CursorGrabMode::None); }
        self.window.set_cursor_visible(!on);
    }
    fn frame(&mut self) {
        let now = Instant::now(); let dt = (now - self.last).as_secs_f64().min(0.05); let real = (now - self.last).as_secs_f64(); self.last = now; self.time += dt;
        let k = |c: KeyCode| self.keys.contains(&c);
        // Regarder vers +z place +x à gauche de l'écran : la droite vaut -x.
        let ax = |a: bool, b: bool| (a as i32 - b as i32) as f64;
        let input = Input { x: ax(k(KeyCode::KeyA) || k(KeyCode::KeyQ), k(KeyCode::KeyD)), z: ax(k(KeyCode::KeyW) || k(KeyCode::KeyZ), k(KeyCode::KeyS)), jump: k(KeyCode::Space), sprint: k(KeyCode::ShiftLeft), crouch: k(KeyCode::ControlLeft) };
        self.player.update(&self.world, dt, input);
        let (built, gone) = self.world.update(self.player.p[0], self.player.p[2]);
        for b in built { self.gpu.upload(b); }
        for g in gone { self.gpu.forget(g); }
        let size = self.window.inner_size(); let aspect = size.width.max(1) as f64 / size.height.max(1) as f64;
        let eye = self.player.eye(); let vp = view::mul(&view::perspective(1.05, aspect, 0.08), &view::look_dir(eye, self.player.forward()));
        self.tris = self.gpu.render(&vp, eye, self.time, &view::sky_at(self.hour));
        self.frames += 1; self.fps_t += real;
        if self.fps_t >= 0.5 {
            let fps = self.frames as f64 / self.fps_t; self.frames = 0; self.fps_t = 0.0;
            self.window.set_title(&format!("A Living World  |  {fps:.0} FPS  |  {:.1} M triangles  |  {} chunks  |  {} en attente", self.tris as f64 / 1e6, self.world.loaded(), self.world.waiting()));
        }
    }
}
impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.game.is_some() { return; }
        let window = Arc::new(el.create_window(Window::default_attributes().with_title("A Living World").with_inner_size(winit::dpi::LogicalSize::new(1600.0, 900.0))).expect("création de la fenêtre"));
        let gpu = pollster::block_on(gpu::Gpu::new(window.clone()));
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).saturating_sub(2).clamp(2, 12);
        let (x, z, y) = spawn_point(SEED);
        let mut g = Game { window, gpu, world: Streamer::new(SEED, &HIGH, threads), player: Player::new(x, y, z), keys: HashSet::new(), grabbed: false, hour: 10.0, time: 0.0, last: Instant::now(), frames: 0, fps_t: 0.0, tris: 0 };
        g.grab(true); self.game = Some(g);
    }
    fn window_event(&mut self, el: &ActiveEventLoop, _: WindowId, e: WindowEvent) {
        let Some(g) = self.game.as_mut() else { return };
        match e {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(s) => g.gpu.resize(s.width, s.height),
            WindowEvent::Focused(false) => { g.keys.clear(); g.grab(false); }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => if !g.grabbed { g.grab(true); },
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(c) = event.physical_key {
                    if event.state == ElementState::Pressed {
                        if !event.repeat { match c {
                            KeyCode::Escape => g.grab(false),
                            KeyCode::KeyF => g.player.fly = !g.player.fly,
                            KeyCode::KeyT => g.hour = (g.hour + 1.0) % 24.0,
                            _ => {}
                        } }
                        g.keys.insert(c);
                    } else { g.keys.remove(&c); }
                }
            }
            WindowEvent::RedrawRequested => g.frame(),
            _ => {}
        }
    }
    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, e: DeviceEvent) {
        let Some(g) = self.game.as_mut() else { return };
        if let (true, DeviceEvent::MouseMotion { delta }) = (g.grabbed, e) {
            g.player.yaw -= delta.0 * 0.0023; g.player.pitch = (g.player.pitch - delta.1 * 0.0023).clamp(-1.5, 1.5);
        }
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) { if let Some(g) = &self.game { g.window.request_redraw(); } }
}
fn main() {
    std::panic::set_hook(Box::new(|i| report(&format!("Erreur : {i}"))));
    let el = EventLoop::new().expect("boucle d'évènements"); el.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    el.run_app(&mut app).expect("boucle principale");
}
