//! Son : mélangeur logiciel, banque de prises, bruitages et musique générative. Aucune dépendance ;
//! la sortie vers la carte son vit dans le programme, ce module se teste en rendant des échantillons.
use std::collections::HashMap;
use std::f32::consts::TAU;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
pub use crate::sounds_data::{SOUNDS, TOOLS};

#[derive(Clone, Copy, Debug)] pub struct L { pub set: &'static str, pub rate: f32, pub vol: f32, pub lp: f32, pub at: f32 }
const fn l(set: &'static str, rate: f32, vol: f32) -> L { L { set, rate, vol, lp: 0.0, at: 0.0 } }
#[derive(Clone, Copy, Debug)] pub struct Mat { pub m: &'static str, pub step: &'static [L], pub hit: &'static [L], pub brk: &'static [L], pub put: &'static [L] }
pub fn sound_of(name: &str) -> &'static Mat {
    let name = name.strip_suffix("_c").unwrap_or(name); let owned; let name = if let Some(sp) = name.strip_suffix("_branch").or(name.strip_suffix("_hollow")).or(name.strip_suffix("_hollow_x")).or(name.strip_suffix("_hollow_z")) { owned = format!("{sp}_log"); owned.as_str() } else { name }; let n = name.strip_suffix("_x").or(name.strip_suffix("_y")).or(name.strip_suffix("_z")).unwrap_or(name);
    let n = if n.starts_with("water") { "water" } else { n.strip_suffix("_deep").unwrap_or(n) };
    if n == "snow" || n.starts_with("snow_layer") { return &SNOW; }
    let n = match n { "frost_grass" | "feather_grass" | "sagebrush" | "dead_bush" | "blueberry" | "rowan" | "fern_dry" | "blueberry_picked" | "rowan_picked" | "twigs" => "tall_grass", "pebbles" => "gravel", "dead_log" | "acacia_log" | "campfire" => "oak_log", "workbench" | "leather" => "planks", "acacia_leaves" => "oak_leaves", "birch_leaves_gold" => "birch_leaves", "lichen_stone" | "bone" | "sandstone" | "red_sandstone" => "stone", "cactus" | "palm_leaves" => "oak_leaves", "palm_log" => "oak_log", "steppe_grass" | "moor_grass" => "grass", "heather" | "cotton_grass" => "tall_grass", "ice" | "icicle" => "stone", "spruce_log" => "oak_log", "spruce_leaves" | "spruce_leaves_snow" => "oak_leaves", _ => n };
    SOUNDS.iter().find(|s| s.0 == n).map(|s| &s.1).unwrap_or_else(|| &SOUNDS.iter().find(|s| s.0 == "dirt").unwrap().1)
}
/// Neige : un crissement feutré, grave, avec le grain fin des cristaux qui s'écrasent.
static SNOW: Mat = Mat { m: "earth",
    step: &[L { set: "crumble", rate: 1.5, vol: 0.3, lp: 4500.0, at: 0.0 }, L { set: "sand", rate: 0.62, vol: 0.34, lp: 2200.0, at: 0.0 }, L { set: "gravel", rate: 1.45, vol: 0.07, lp: 3200.0, at: 0.02 }],
    hit: &[L { set: "sand", rate: 0.7, vol: 0.45, lp: 2600.0, at: 0.0 }, L { set: "crumble", rate: 1.3, vol: 0.2, lp: 4000.0, at: 0.0 }],
    brk: &[L { set: "sand", rate: 0.66, vol: 0.6, lp: 2400.0, at: 0.0 }, L { set: "crumble", rate: 1.2, vol: 0.3, lp: 4000.0, at: 0.0 }],
    put: &[L { set: "sand", rate: 0.7, vol: 0.5, lp: 2400.0, at: 0.0 }] };
fn round_lp(set: &str) -> f32 { match set { "leaf" => 4200.0, "crumble" => 6000.0, "gravel" => 5200.0, "sand" => 7000.0, _ => 0.0 } }

pub struct Clip { pub pcm: Vec<f32>, pub ch: usize }
#[derive(Default)] pub struct Bank { sets: HashMap<String, Vec<Arc<Clip>>>, loops: HashMap<String, Arc<Clip>> }
pub const BANK_RATE: f32 = 32000.0;
impl Bank {
    pub fn parse(b: &[u8]) -> Option<Bank> {
        if b.get(..4)? != b"ALWB" { return None; }
        let (mut p, mut o) = (4, Bank::default());
        while p < b.len() {
            let n = b[p] as usize; let name = std::str::from_utf8(b.get(p + 1..p + 1 + n)?).ok()?.to_string(); p += 1 + n;
            let (ch, lp) = (b[p] as usize, b[p + 1] != 0); let len = u32::from_le_bytes(b.get(p + 2..p + 6)?.try_into().ok()?) as usize; p += 6;
            let pcm: Vec<f32> = b.get(p..p + len * 2)?.chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0).collect(); p += len * 2;
            let clip = Arc::new(Clip { pcm, ch });
            if lp { o.loops.insert(name[5..].to_string(), clip); } else { let (s, _) = name.rsplit_once('_')?; o.sets.entry(s.to_string()).or_default().push(clip); }
        }
        Some(o)
    }
    pub fn set_count(&self) -> usize { self.sets.len() }
    pub fn has(&self, s: &str) -> bool { self.sets.contains_key(s) }
}
#[derive(Clone, Copy, Default)] pub struct Rnd(pub u32);
impl Rnd { fn next(&mut self) -> f32 { let mut x = self.0.max(1); x ^= x << 13; x ^= x >> 17; x ^= x << 5; self.0 = x; (x >> 8) as f32 / 16777216.0 } fn range(&mut self, a: f32, b: f32) -> f32 { a + (b - a) * self.next() } fn pick(&mut self, n: usize) -> usize { (self.next() * n as f32) as usize % n } }

/// Filtre passe-bas à un pôle : aucun grésillement ajouté, coût négligeable.
#[derive(Clone, Copy, Default)] struct Lp { a: f32, z: f32 }
impl Lp { fn new(f: f32, sr: f32) -> Self { Lp { a: if f <= 0.0 { 1.0 } else { 1.0 - (-TAU * f / sr).exp() }, z: 0.0 } } #[inline] fn run(&mut self, x: f32) -> f32 { self.z += self.a * (x - self.z); self.z } }
enum Voice {
    Clip { c: Arc<Clip>, pos: f32, inc: f32, g: [f32; 2], lp: Lp, wait: u32 },
    Tone { ph: f32, f0: f32, k: f32, t: f32, dur: f32, a: f32, vol: f32, g: [f32; 2], wait: u32, saw: bool, lp: Lp, music: bool, group: u32 },
    Note { f: f32, ph: [f32; 3], amp: [f32; 3], mult: [f32; 3], t: f32, dur: f32, vol: f32, lp: Lp, tri: bool, wait: u32, group: u32 },
    Pad { f: [f32; 6], ph: [f32; 6], n: usize, t: f32, dur: f32, a: f32, vol: f32, lp: [Lp; 2], wait: u32, group: u32 },
}
/// Bandes en boucle : vent, pluie, ruisseau, chute.
struct Bed { c: Option<Arc<Clip>>, pos: f32, gain: f32, want: f32, pan: f32, tau: f32, lp: Lp }
struct Comb { b: Vec<f32>, i: usize, fb: f32, z: f32, d: f32 }
impl Comb { fn new(n: usize, fb: f32, d: f32) -> Self { Comb { b: vec![0.0; n], i: 0, fb, z: 0.0, d } } #[inline] fn run(&mut self, x: f32) -> f32 { let o = self.b[self.i]; self.z = o * (1.0 - self.d) + self.z * self.d; self.b[self.i] = x + self.z * self.fb; self.i = (self.i + 1) % self.b.len(); o } }
struct All { b: Vec<f32>, i: usize } impl All { fn new(n: usize) -> Self { All { b: vec![0.0; n], i: 0 } } #[inline] fn run(&mut self, x: f32) -> f32 { let o = self.b[self.i]; let y = o - 0.5 * x; self.b[self.i] = x + 0.5 * o; self.i = (self.i + 1) % self.b.len(); y } }
/// Réverbération de Schroeder : donne de l'espace sans fichier de salle.
struct Reverb { c: Vec<Comb>, a: Vec<All> }
impl Reverb {
    fn new(sr: f32, k: f32, fb: f32, st: usize) -> Self { let s = |m: f32| ((m * k * sr / 1000.0) as usize + st).max(8); Reverb { c: [29.7, 37.1, 41.1, 43.7].iter().map(|m| Comb::new(s(*m), fb, 0.3)).collect(), a: [5.0, 1.7].iter().map(|m| All::new(s(*m))).collect() } }
    #[inline] fn run(&mut self, x: f32) -> f32 { let mut o = 0.0; for c in &mut self.c { o += c.run(x); } o *= 0.25; for a in &mut self.a { o = a.run(o); } o }
}
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Mood { Calme, Nuit, Construction, Suspense, Danger }
/// La situation du joueur décide de l'humeur. Le danger est réservé aux vraies menaces.
#[derive(Clone, Copy, Default)] pub struct Situation { pub hurt: bool, pub enemy: bool, pub food: f64, pub water: f64, pub underground: bool, pub submerged: bool, pub building: bool, pub night: bool, pub rain: bool }
pub fn pick_mood(s: Situation) -> Mood {
    if s.hurt || s.enemy || s.food < 15.0 || s.water < 15.0 { Mood::Danger } else if s.underground || s.submerged { Mood::Suspense } else if s.building { Mood::Construction } else if s.night || s.rain { Mood::Nuit } else { Mood::Calme }
}
struct Piece { bpm: f32, root: f32, scale: &'static [f32], chords: [[f32; 3]; 4], inst: u8, density: f32, bars: u32, rest: (f32, f32), arp: bool }
fn piece(m: Mood) -> Option<Piece> {
    const MAJ: &[f32] = &[0., 2., 4., 7., 9.]; const MIN: &[f32] = &[0., 3., 5., 7., 10.];
    match m {
        Mood::Calme => Some(Piece { bpm: 66., root: 62., scale: MAJ, chords: [[0., 4., 7.], [9., 12., 16.], [5., 9., 12.], [7., 11., 14.]], inst: 0, density: 0.38, bars: 8, rest: (16., 38.), arp: false }),
        Mood::Nuit => Some(Piece { bpm: 52., root: 57., scale: MIN, chords: [[0., 3., 7.], [8., 12., 15.], [5., 8., 12.], [7., 10., 14.]], inst: 1, density: 0.26, bars: 6, rest: (12., 30.), arp: false }),
        Mood::Construction => Some(Piece { bpm: 92., root: 67., scale: MAJ, chords: [[0., 4., 7.], [5., 9., 12.], [9., 12., 16.], [7., 11., 14.]], inst: 2, density: 0.6, bars: 8, rest: (5., 12.), arp: true }),
        _ => None,
    }
}
fn fade_of(m: Mood) -> f32 { match m { Mood::Danger => 0.8, Mood::Construction | Mood::Suspense => 2.5, _ => 3.5 } }
fn hz(m: f32) -> f32 { 440.0 * 2f32.powf((m - 69.0) / 12.0) }
pub enum Cmd {
    Play { l: L, k: f32, pan: f32, delay: f32 }, Tone { f: f32, to: f32, dur: f32, vol: f32, pan: f32, delay: f32, saw: bool, lp: f32 },
    Env { wind: f32, rain: f32, brook: (f32, f32), fall: (f32, f32), under: bool }, Volumes { master: f32, sfx: f32, music: f32 }, Mood(Mood), Force(Option<Mood>),
}
pub struct Mixer {
    sr: f32, bank: Bank, rx: Receiver<Cmd>, v: Vec<Voice>, beds: [Bed; 4], rnd: Rnd, last: HashMap<&'static str, usize>,
    master: f32, sfx: f32, music: f32, want: Mood, forced: Option<Mood>, mood: Option<Mood>, seen: f32, now: f64, next: f64, bar: u32, step: u32, last_deg: i32, group: u32, groups: Vec<(u32, f32, f32)>,
    rev_m: [Reverb; 2], rev_s: [Reverb; 2], mute: [Lp; 2], mute_f: f32, block: Vec<f32>,
}
impl Mixer {
    pub fn new(sr: f32, mut bank: Bank) -> (Self, Sender<Cmd>) {
        let (tx, rx) = channel();
        let bed = |n: &str, lp: f32, b: &mut Bank| Bed { c: b.loops.remove(n), pos: 0.0, gain: 0.0, want: 0.0, pan: 0.0, tau: 0.6, lp: Lp::new(lp, sr) };
        let beds = [bed("wind", 0.0, &mut bank), bed("rain", 0.0, &mut bank), bed("brook", 5200.0, &mut bank), bed("fall", 0.0, &mut bank)];
        (Mixer { sr, bank, rx, v: Vec::with_capacity(128), beds, rnd: Rnd(0x9e3779b1), last: HashMap::new(), master: 0.7, sfx: 0.7, music: 0.5, want: Mood::Calme, forced: None, mood: None, seen: 0.0, now: 0.0, next: 3.0, bar: 0, step: 0, last_deg: 3, group: 1, groups: vec![(1, 1.0, 0.0)],
            rev_m: [Reverb::new(sr, 1.6, 0.86, 0), Reverb::new(sr, 1.6, 0.86, 23)], rev_s: [Reverb::new(sr, 0.5, 0.62, 0), Reverb::new(sr, 0.5, 0.62, 17)], mute: [Lp::new(0.0, sr); 2], mute_f: 20000.0, block: vec![0.0; 512] }, tx)
    }
    fn secs(&self, s: f32) -> u32 { (s * self.sr) as u32 }
    fn pans(p: f32) -> [f32; 2] { let a = (p.clamp(-1.0, 1.0) + 1.0) * 0.25 * std::f32::consts::PI; [a.cos() * std::f32::consts::SQRT_2, a.sin() * std::f32::consts::SQRT_2] }
    fn play(&mut self, l: L, k: f32, pan: f32, delay: f32) {
        let Some(set) = self.bank.sets.get(l.set) else { return }; if self.v.len() > 96 { return; }
        let n = set.len(); let mut i = self.rnd.pick(n); if n > 1 && self.last.get(l.set) == Some(&i) { i = (i + 1) % n; } self.last.insert(l.set, i);
        let c = set[i].clone(); let inc = l.rate * self.rnd.range(0.96, 1.04) * BANK_RATE / self.sr; let g = l.vol * k * self.rnd.range(0.9, 1.0); let lp = if l.lp > 0.0 { l.lp } else { round_lp(l.set) };
        let p = Self::pans(pan); self.v.push(Voice::Clip { c, pos: 0.0, inc, g: [g * p[0], g * p[1]], lp: Lp::new(lp, self.sr), wait: self.secs(delay + l.at) });
    }
    /// Change d'humeur : la phrase en cours s'éteint sans couper les notes.
    fn begin(&mut self, m: Mood) {
        let was = self.mood; let f = fade_of(m); let g = self.group; if let Some(e) = self.groups.iter_mut().find(|e| e.0 == g) { e.2 = 1.0 / f; }
        self.group += 1; self.groups.push((self.group, 1.0, 0.0)); self.mood = Some(m); self.seen = 0.0; self.bar = 0; self.step = 0; self.last_deg = 2 + self.rnd.pick(3) as i32;
        self.next = (if self.next > self.now + 6.0 { self.now + 1.0 } else { self.next }).max(self.now + if m == Mood::Danger { 0.05 } else if was.is_some() { 1.0 } else { 3.0 });
    }
    fn note(&mut self, f: f32, t: f64, vol: f32, inst: u8) {
        let (mult, amp, dur, cut): ([f32; 3], [f32; 3], f32, f32) = match inst { 1 => ([1., 4., 0.], [1., 0.28, 0.], 1.5, 3400.), 2 => ([1., 2., 0.], [1., 0.3, 0.], 0.75, 2200.), _ => ([1., 2., 3.], [1., 0.42, 0.12], 2.2, 3400.) };
        self.v.push(Voice::Note { f, ph: [0.0; 3], amp, mult, t: 0.0, dur, vol, lp: Lp::new(cut, self.sr), tri: inst == 2, wait: self.secs((t - self.now).max(0.0) as f32), group: self.group });
    }
    fn pad(&mut self, fr: &[f32], t: f64, dur: f32, vol: f32, cut: f32) {
        let mut f = [0.0; 6]; let n = (fr.len() * 2).min(6); for (i, x) in fr.iter().take(3).enumerate() { f[i * 2] = x * 0.99769; f[i * 2 + 1] = x * 1.00289; }
        let mut ph = [0.0; 6]; for p in ph.iter_mut() { *p = self.rnd.next(); }
        self.v.push(Voice::Pad { f, ph, n, t: 0.0, dur, a: (dur * 0.3).min(2.0), vol, lp: [Lp::new(cut, self.sr); 2], wait: self.secs((t - self.now).max(0.0) as f32), group: self.group });
    }
    fn tone_m(&mut self, f: f32, to: f32, dur: f32, vol: f32, t: f64, saw: bool, cut: f32) { let wait = self.secs((t - self.now).max(0.0) as f32); self.v.push(Voice::Tone { ph: 0.0, f0: f, k: if to > 0.0 { (to / f).ln() / dur } else { 0.0 }, t: 0.0, dur, a: 0.004, vol, g: [1.0, 1.0], wait, saw, lp: Lp::new(cut, self.sr), music: true, group: self.group }); }
    fn schedule(&mut self) {
        let m = self.mood.unwrap(); let t = self.next;
        match m {
            Mood::Suspense => {
                let b = 60.0 / 46.0; let r = 38.0f32; self.pad(&[hz(r), hz(r + 7.0)], t, b * 8.0, 0.05, 340.0);
                if self.bar % 2 == 1 { self.pad(&[hz(r + 37.0), hz(r + 38.0)], t + b as f64, b * 5.0, 0.012, 1300.0); }
                if self.rnd.next() < 0.55 { let o = t + self.rnd.range(b, b * 5.0) as f64; self.tone_m(115.0, 46.0, 0.42, 0.2, o, false, 0.0); self.tone_m(115.0, 46.0, 0.42, 0.13, o + 0.26, false, 0.0); }
                if self.rnd.next() < 0.7 { let d = [0., 1., 6., 7., 13.][self.rnd.pick(5)]; let o = t + self.rnd.range(b * 2.0, b * 7.0) as f64; self.note(hz(r + 24.0 + d), o, 0.05, 2); }
                self.bar += 1; self.next = t + (b * 8.0) as f64;
            }
            Mood::Danger => {
                let e = 15.0 / 128.0f32; let s = self.step % 32; let r = 38.0f32; const LINE: [f32; 16] = [0., 0., 12., 0., 0., 1., 0., 0., 0., 0., 12., 0., 0., 6., 0., 1.]; let n = LINE[(s % 16) as usize];
                if s % 2 == 0 || n != 0.0 { self.tone_m(hz(r + n), 0.0, e * 1.5, 0.1, t, true, 620.0); }
                if [0, 6, 8, 16, 22, 24, 28].contains(&s) { self.tone_m(115.0, 46.0, 0.42, if s % 8 == 0 { 0.3 } else { 0.2 }, t, false, 0.0); }
                if s == 12 || s == 28 { self.pad(&[hz(r + 25.0), hz(r + 26.0), hz(r + 31.0)], t, e * 4.0, 0.035, 2200.0); }
                self.step += 1; self.next = t + e as f64;
            }
            _ => {
                let p = piece(m).unwrap(); let e = 30.0 / p.bpm; let s = self.step;
                if s % 8 == 0 {
                    if self.bar >= p.bars { self.bar = 0; self.step = 0; let g = self.group; if let Some(x) = self.groups.iter_mut().find(|x| x.0 == g) { x.2 = 0.5; } self.group += 1; self.groups.push((self.group, 1.0, 0.0)); let r = self.rnd.range(p.rest.0, p.rest.1); self.next = t + r as f64; return; }
                    let ch = p.chords[((self.bar / 2) % 4) as usize];
                    if self.bar % 2 == 0 { let f: Vec<f32> = ch.iter().map(|n| hz(p.root - 12.0 + n)).collect(); self.pad(&f, t, e * 16.0 * 1.05, 0.03, 760.0); }
                    self.note(hz(p.root - 24.0 + ch[0]), t, 0.1, p.inst); self.bar += 1;
                }
                let ch = p.chords[(((self.bar.max(1) - 1) / 2) % 4) as usize];
                if p.arp && s % 2 == 0 { self.note(hz(p.root + ch[((s / 2) % 3) as usize]), t, 0.055, p.inst); }
                let w = if s % 4 == 0 { 1.6 } else if s % 2 == 0 { 1.0 } else { 0.45 };
                if self.rnd.next() < p.density * w {
                    self.last_deg = (self.last_deg + [-2, -1, -1, 1, 1, 2][self.rnd.pick(6)]).clamp(0, 9); let n = p.scale.len() as i32; let (deg, oct) = (self.last_deg % n, self.last_deg / n);
                    let v = self.rnd.range(0.07, 0.115); let o = t + self.rnd.range(0.0, 0.012) as f64; self.note(hz(p.root + 12.0 * oct as f32 + p.scale[deg as usize]), o, v, p.inst);
                }
                self.step = s + 1; self.next = t + e as f64;
            }
        }
    }
    fn control(&mut self, dt: f32) {
        while let Ok(c) = self.rx.try_recv() { match c {
            Cmd::Play { l, k, pan, delay } => self.play(l, k, pan, delay),
            Cmd::Tone { f, to, dur, vol, pan, delay, saw, lp } => { let p = Self::pans(pan); let wait = self.secs(delay); let a = 0.012f32.min(dur * 0.3); let cut = Lp::new(lp, self.sr); if self.v.len() < 110 { self.v.push(Voice::Tone { ph: 0.0, f0: f, k: if to > 0.0 { (to / f).ln() / dur } else { 0.0 }, t: 0.0, dur, a, vol, g: p, wait, saw, lp: cut, music: false, group: 0 }); } }
            Cmd::Env { wind, rain, brook, fall, under } => { self.beds[0].want = wind; self.beds[0].tau = 1.2; self.beds[1].want = rain; self.beds[1].tau = 1.0; self.beds[2].want = brook.0; self.beds[2].pan = brook.1; self.beds[3].want = fall.0; self.beds[3].pan = fall.1; self.mute_f = if under { 620.0 } else { 20000.0 }; }
            Cmd::Volumes { master, sfx, music } => { self.master = master; self.sfx = sfx; self.music = music; }
            Cmd::Mood(m) => self.want = m, Cmd::Force(m) => self.forced = m,
        } }
        let want = self.forced.unwrap_or(self.want);
        self.seen = if Some(want) == self.mood { 0.0 } else { self.seen + dt };
        if Some(want) != self.mood && (want == Mood::Danger || self.seen > 2.5 || self.mood.is_none()) { self.begin(want); }
        while self.mood.is_some() && self.next < self.now + 0.3 { self.schedule(); }
        for g in &mut self.groups { g.1 = (g.1 - g.2 * dt).max(0.0); }
        let keep: Vec<u32> = self.groups.iter().filter(|g| g.1 > 0.0 || g.2 == 0.0).map(|g| g.0).collect(); self.v.retain(|v| match v { Voice::Note { group, .. } | Voice::Pad { group, .. } => keep.contains(group), Voice::Tone { group, music: true, .. } => keep.contains(group), _ => true }); self.groups.retain(|g| keep.contains(&g.0));
        let k = 1.0 - (-TAU * self.mute_f / self.sr).exp(); self.mute[0].a = k; self.mute[1].a = k;
    }
    /// Remplit un tampon stéréo entrelacé.
    pub fn render(&mut self, out: &mut [f32]) {
        for chunk in out.chunks_mut(512) {
            let frames = chunk.len() / 2; let dt = frames as f32 / self.sr; self.control(dt);
            let mut sfx = [0f32; 512]; let mut mus = [0f32; 512];
            let sr = self.sr; let gr = &self.groups; let glev = |g: u32| gr.iter().find(|x| x.0 == g).map_or(0.0, |x| x.1);
            self.v.retain_mut(|v| match v {
                Voice::Clip { c, pos, inc, g, lp, wait } => {
                    for f in 0..frames {
                        if *wait > 0 { *wait -= 1; continue; }
                        let i = *pos as usize; if i + 1 >= c.pcm.len() { return false; }
                        let (a, b) = (c.pcm[i], c.pcm[i + 1]); let s = lp.run(a + (b - a) * pos.fract()); sfx[f * 2] += s * g[0]; sfx[f * 2 + 1] += s * g[1]; *pos += *inc;
                    } true
                }
                Voice::Tone { ph, f0, k, t, dur, a, vol, g, wait, saw, lp, music, group } => {
                    let gl = if *music { glev(*group) } else { 1.0 };
                    for f in 0..frames {
                        if *wait > 0 { *wait -= 1; continue; }
                        if *t >= *dur { return false; }
                        let fq = *f0 * (*k * *t).exp(); *ph = (*ph + fq / sr).fract();
                        let raw = if *saw { let x = *ph * 2.0 - 1.0; let d = fq / sr; x - blep(*ph, d) } else { (*ph * TAU).sin() };
                        let e = if *t < *a { 0.0001 * (*vol / 0.0001).powf(*t / *a) } else { *vol * (0.0001 / *vol).powf((*t - *a) / (*dur - *a)) };
                        let s = lp.run(raw) * e * gl; if *music { mus[f * 2] += s; mus[f * 2 + 1] += s; } else { sfx[f * 2] += s * g[0]; sfx[f * 2 + 1] += s * g[1]; } *t += 1.0 / sr;
                    } true
                }
                Voice::Note { f: fq, ph, amp, mult, t, dur, vol, lp, tri, wait, group } => {
                    let gl = glev(*group);
                    for f in 0..frames {
                        if *wait > 0 { *wait -= 1; continue; }
                        if *t >= *dur { return false; }
                        let mut s = 0.0; for k in 0..3 { if amp[k] > 0.0 { ph[k] = (ph[k] + *fq * mult[k] / sr).fract(); s += amp[k] * if *tri { 1.0 - 4.0 * (ph[k] - 0.5).abs() } else { (ph[k] * TAU).sin() }; } }
                        let e = if *t < 0.006 { 0.0001 * (*vol / 0.0001).powf(*t / 0.006) } else { *vol * (0.0001 / *vol).powf((*t - 0.006) / (*dur - 0.006)) };
                        let x = lp.run(s) * e * gl; mus[f * 2] += x; mus[f * 2 + 1] += x; *t += 1.0 / sr;
                    } true
                }
                Voice::Pad { f: fr, ph, n, t, dur, a, vol, lp, wait, group } => {
                    let gl = glev(*group);
                    for f in 0..frames {
                        if *wait > 0 { *wait -= 1; continue; }
                        if *t >= *dur { return false; }
                        let (mut x, mut y) = (0.0, 0.0); for k in 0..*n { let d = fr[k] / sr; ph[k] = (ph[k] + d).fract(); let s = ph[k] * 2.0 - 1.0 - blep(ph[k], d); if k % 2 == 0 { x += s; } else { y += s; } }
                        let e = (if *t < *a { *t / *a } else if *t > *dur - *a { (*dur - *t) / *a } else { 1.0 }) * *vol;
                        mus[f * 2] += lp[0].run(x) * e * gl; mus[f * 2 + 1] += lp[1].run(y) * e * gl; *t += 1.0 / sr;
                    } true
                }
            });
            let block = &mut self.block; for b in block.iter_mut() { *b = 0.0; }
            for bd in &mut self.beds {
                let Some(c) = &bd.c else { continue }; let k = 1.0 - (-dt / bd.tau).exp(); let start = bd.gain; bd.gain += (bd.want - bd.gain) * k; let end = bd.gain; if start < 0.0002 && end < 0.0002 { continue; }
                let p = Self::pans(bd.pan); let n = c.pcm.len() / c.ch; let inc = BANK_RATE / sr;
                for f in 0..frames {
                    let g = start + (end - start) * f as f32 / frames as f32; let i = bd.pos as usize % n; let j = (i + 1) % n; let fr = bd.pos.fract();
                    let (a, b) = if c.ch == 2 { (c.pcm[i * 2] + (c.pcm[j * 2] - c.pcm[i * 2]) * fr, c.pcm[i * 2 + 1] + (c.pcm[j * 2 + 1] - c.pcm[i * 2 + 1]) * fr) } else { let x = c.pcm[i] + (c.pcm[j] - c.pcm[i]) * fr; let y = bd.lp.run(x); (y, y) };
                    block[f * 2] += a * g * p[0]; block[f * 2 + 1] += b * g * p[1]; bd.pos += inc; if bd.pos >= n as f32 { bd.pos -= n as f32; }
                }
            }
            for f in 0..frames { for c in 0..2 {
                let s = sfx[f * 2 + c] * self.sfx; let m = mus[f * 2 + c] * self.music;
                let wet_s = self.rev_s[c].run(s) * 0.42; let wet_m = self.rev_m[c].run(m) * 0.5;
                let x = (s + wet_s + m + wet_m + block[f * 2 + c]) * self.master; let x = self.mute[c].run(x);
                chunk[f * 2 + c] = if x.abs() < 0.5 { x } else { x.signum() * (0.5 + 0.5 * ((x.abs() - 0.5) / 0.5).tanh()) };
            } }
            self.now += dt as f64;
        }
    }
    pub fn voices(&self) -> usize { self.v.len() }
    pub fn mood(&self) -> Option<Mood> { self.mood }
}
#[inline] fn blep(t: f32, d: f32) -> f32 { if t < d { let x = t / d; x + x - x * x - 1.0 } else if t > 1.0 - d { let x = (t - 1.0) / d; x * x + x + x + 1.0 } else { 0.0 } }

/// Ce que le jeu demande : mêmes règles que le prototype, réglées à l'oreille.
pub struct SoundScape { tx: Option<Sender<Cmd>>, pub ear: [f64; 3], pub yaw: f64, rnd: Rnd, pub breeze: f32 }
impl SoundScape {
    pub fn new(tx: Option<Sender<Cmd>>) -> Self { SoundScape { tx, ear: [0.0; 3], yaw: 0.0, rnd: Rnd(0x51ed270b), breeze: 0.0 } }
    fn send(&self, c: Cmd) { if let Some(t) = &self.tx { let _ = t.send(c); } }
    fn at(&self, pos: [f64; 3], r: f64) -> Option<(f32, f32)> {
        let (dx, dy, dz) = (pos[0] - self.ear[0], pos[1] - self.ear[1], pos[2] - self.ear[2]); let d = (dx * dx + dy * dy + dz * dz).sqrt(); if d >= r { return None; }
        let a = 1.0 - d / r; let side = (dz * self.yaw.sin() - dx * self.yaw.cos()) / d.max(1.0); /* regarder +z place +x à gauche : la droite vaut -x */ Some(((a * a) as f32, (0.7 * side.clamp(-1.0, 1.0)) as f32))
    }
    fn lay(&self, ls: &[L], k: f32) { for l in ls { self.send(Cmd::Play { l: *l, k, pan: 0.0, delay: 0.0 }); } }
    pub fn step(&self, name: &str, through: Option<&str>) { self.lay(sound_of(name).step, 0.6); if let Some(t) = through { self.lay(sound_of(t).step, 0.3); } }
    pub fn land(&self, name: &str) { self.lay(sound_of(name).step, 0.8); }
    fn strike(&self, name: &str, tool: Option<&str>) -> &'static Mat {
        let b = sound_of(name); let t = tool.unwrap_or("hand");
        if let Some(l) = TOOLS.iter().find(|x| x.0 == t && x.1 == b.m).and_then(|x| x.2) { self.send(Cmd::Play { l, k: 1.0, pan: 0.0, delay: 0.0 }); }
        if tool == Some("sword") { self.swing(tool, 0.45); } b
    }
    pub fn dig(&self, name: &str, tool: Option<&str>) { self.lay(self.strike(name, tool).hit, 0.85); }
    pub fn break_block(&self, name: &str, tool: Option<&str>) { self.lay(self.strike(name, tool).brk, 1.0); }
    pub fn place(&self, name: &str) { self.lay(sound_of(name).put, 0.9); }
    pub fn swing(&self, tool: Option<&str>, k: f32) {
        let l = match tool { Some("sword") => l("blade", 1.0, 0.5), Some("pickaxe") => l("whoosh", 1.05, 0.36), Some("axe") => l("whoosh", 0.9, 0.36), _ => l("puff", 1.0, 0.3) };
        self.send(Cmd::Play { l, k, pan: 0.0, delay: 0.0 });
    }
    /// L'arbre cède : craquements espacés, plus graves à mesure qu'il part.
    pub fn tree_crack(&self) { for (i, r) in [1.0, 0.86, 0.74].iter().enumerate() { self.send(Cmd::Play { l: l("crack", *r, 0.6), k: 1.0, pan: 0.0, delay: i as f32 * 0.18 }); } self.send(Cmd::Play { l: l("chopdry", 0.8, 0.5), k: 1.0, pan: 0.0, delay: 0.0 }); }
    /// Il touche le sol : choc sourd, puis branches.
    pub fn tree_land(&self) { self.send(Cmd::Play { l: L { lp: 900.0, ..l("knock", 0.55, 0.8) }, k: 1.0, pan: 0.0, delay: 0.0 }); self.send(Cmd::Play { l: l("crack", 0.7, 0.35), k: 1.0, pan: 0.0, delay: 0.08 }); self.send(Cmd::Play { l: l("leaf", 0.8, 0.4), k: 1.0, pan: 0.0, delay: 0.05 }); }
    /// Roulade : frottement du corps et du sac au sol.
    pub fn roll(&self) { self.send(Cmd::Play { l: l("puff", 0.75, 0.35), k: 1.0, pan: 0.0, delay: 0.0 }); self.send(Cmd::Play { l: l("leaf", 0.8, 0.28), k: 1.0, pan: 0.0, delay: 0.14 }); self.send(Cmd::Play { l: l("sand", 0.9, 0.3), k: 1.0, pan: 0.0, delay: 0.3 }); }
    pub fn splash(&self, k: f32) { self.send(Cmd::Play { l: l("splash", 0.85, 0.5), k, pan: 0.0, delay: 0.0 }); }
    /// La corde de l'arc qu'on tend : bois qui grince.
    pub fn creak(&self) { self.send(Cmd::Play { l: L { lp: 1400.0, ..l("plankdry", 0.55, 0.22) }, k: 1.0, pan: 0.0, delay: 0.0 }); }
    /// Un objet ramassé : petit claquement clair.
    pub fn pickup(&self) { self.send(Cmd::Tone { f: 820.0, to: 1240.0, dur: 0.06, vol: 0.1, pan: 0.0, delay: 0.0, saw: false, lp: 0.0 }); self.send(Cmd::Play { l: l("leaf", 1.7, 0.18), k: 1.0, pan: 0.0, delay: 0.0 }); }
    /// Un temps de l'ouvrage, selon l'atelier (dans l'ordre de `Station`) : scie et marteau à l'établi, feu qui crépite,
    /// soufflet et ronflement du four, marteau sur l'enclume qui sonne, ciseau sur la pierre.
    pub fn work(&self, at: u8, beat: u32) {
        let p = |s: &Self, l: L, k: f32, d: f32| s.send(Cmd::Play { l, k, pan: 0.0, delay: d });
        match at {
            1 => if beat % 2 == 0 { p(self, L { lp: 2600.0, ..l("plankdry", 1.35, 0.45) }, 1.0, 0.0); p(self, L { lp: 2600.0, ..l("plankdry", 1.5, 0.35) }, 1.0, 0.12); } else { p(self, l("knock", 1.25, 0.55), 1.0, 0.0); },
            2 => { p(self, l("crack", 1.3, 0.25), 1.0, 0.0); p(self, l("soft", 0.8, 0.3), 1.0, 0.1); }
            3 => { p(self, L { lp: 700.0, ..l("puff", 0.55, 0.7) }, 1.0, 0.0); p(self, l("crack", 1.5, 0.18), 1.0, 0.2); }
            4 => { p(self, l("metal", 1.3, 0.75), 1.0, 0.0); self.send(Cmd::Tone { f: 1760.0, to: 1720.0, dur: 0.5, vol: 0.05, pan: 0.0, delay: 0.0, saw: false, lp: 0.0 }); self.send(Cmd::Tone { f: 2640.0, to: 2600.0, dur: 0.3, vol: 0.025, pan: 0.0, delay: 0.0, saw: false, lp: 0.0 }); }
            5 => { p(self, l("metal", 1.8, 0.3), 1.0, 0.0); p(self, l("crumble", 1.7, 0.35), 1.0, 0.02); }
            _ => p(self, l("puff", 1.0, 0.3), 1.0, 0.0),
        }
    }
    pub fn hurt(&self) { self.send(Cmd::Play { l: l("hurt", 1.0, 0.6), k: 1.0, pan: 0.0, delay: 0.0 }); }
    pub fn bird(&mut self, pos: [f64; 3], voice: usize) {
        let Some((v, pan)) = self.at(pos, 22.0) else { return }; let r = &mut self.rnd;
        let base = [3100.0, 2300.0, 3900.0][voice] * r.range(0.94, 1.08); let n = [6, 3, 4][voice] + r.pick(3); let gap = [0.07, 0.2, 0.12][voice]; let mut o = Vec::new();
        for i in 0..n { let f = if voice == 1 { base * (1.0 + 0.19 * (i % 3) as f32) } else { base * (1.0 + 0.1 * (i % 2) as f32) }; let to = if voice == 2 { f * 0.74 } else { f * 1.2 }; o.push(Cmd::Tone { f, to, dur: gap * 0.85, vol: 0.125 * v, pan, delay: i as f32 * gap, saw: false, lp: 0.0 }); }
        for c in o { self.send(c); }
    }
    pub fn wings(&self, pos: [f64; 3]) { if let Some((v, pan)) = self.at(pos, 16.0) { for i in 0..4 { self.send(Cmd::Play { l: l("puff", 1.7, 0.22 * v), k: 1.0, pan, delay: i as f32 * 0.07 }); } } }
    pub fn hoof(&self, pos: [f64; 3], fast: bool) { if let Some((v, pan)) = self.at(pos, if fast { 30.0 } else { 20.0 }) { self.send(Cmd::Play { l: L { lp: 1500.0, ..l("soft", 0.8, 0.3 * v) }, k: 1.0, pan, delay: 0.0 }); } }
    pub fn deer_alarm(&self, pos: [f64; 3]) { if let Some((v, pan)) = self.at(pos, 40.0) { self.send(Cmd::Play { l: l("snort", 1.0, 0.6 * v), k: 1.0, pan, delay: 0.0 }); self.send(Cmd::Play { l: l("snort", 0.92, 0.45 * v), k: 1.0, pan, delay: 0.42 }); } }
    pub fn deer_call(&self, pos: [f64; 3]) { if let Some((v, pan)) = self.at(pos, 26.0) { self.send(Cmd::Play { l: L { lp: 900.0, ..l("snort", 0.7, 0.24 * v) }, k: 1.0, pan, delay: 0.0 }); } }
    /// Ambiance : le vent n'existe que par mauvais temps, l'eau dépend de ce qui coule autour du joueur.
    pub fn env(&self, cloud: f64, rain: f64, covered: bool, submerged: bool, brook: Option<[f64; 3]>, fall: Option<[f64; 3]>) {
        let storm = rain.max((cloud - 0.72).max(0.0) / 0.28 * 0.4) as f32; let q = if covered { 0.3 } else { 1.0 }; let soft = |v: f32| v.sqrt() * v.sqrt().sqrt();
        let fl = brook.and_then(|b| self.at(b, 22.0)); let dr = fall.and_then(|b| self.at(b, 38.0));
        self.send(Cmd::Env { wind: (if storm > 0.02 { 0.3 * storm * q } else { 0.0 }).max(self.breeze * q), rain: 0.24 * rain as f32 * q, brook: (if submerged { 0.16 } else { fl.map_or(0.0, |a| 0.22 * soft(a.0)) }, fl.map_or(0.0, |a| a.1)), fall: (dr.map_or(0.0, |a| 0.36 * soft(a.0)), dr.map_or(0.0, |a| a.1)), under: submerged });
    }
    /// Corbeau : deux ou trois croassements rauques.
    pub fn caw(&mut self, pos: [f64; 3]) {
        let Some((v, pan)) = self.at(pos, 60.0) else { return }; let r = &mut self.rnd; let n = 2 + r.pick(2); let f = r.range(520.0, 640.0); let mut o = Vec::new();
        for i in 0..n { o.push(Cmd::Tone { f, to: f * 0.7, dur: 0.24, vol: 0.16 * v, pan, delay: i as f32 * 0.36, saw: true, lp: 1700.0 }); o.push(Cmd::Tone { f: f * 1.5, to: f * 1.02, dur: 0.2, vol: 0.05 * v, pan, delay: i as f32 * 0.36, saw: true, lp: 2400.0 }); }
        for c in o { self.send(c); }
    }
    /// Renard : des jappements rauques et brefs, en série — une voix éraillée (dents de scie et harmonique) mêlée d'un souffle.
    pub fn fox_bark(&mut self, pos: [f64; 3]) {
        let Some((v, pan)) = self.at(pos, 60.0) else { return }; let r = &mut self.rnd; let n = 3 + r.pick(3); let f0 = r.range(560.0, 700.0); let gap = r.range(0.5, 0.8); let mut o = Vec::new();
        for i in 0..n { let d = i as f32 * gap + r.range(-0.05, 0.05); let f = f0 * r.range(0.95, 1.05);
            o.push(Cmd::Tone { f: f * 0.8, to: f * 1.1, dur: 0.05, vol: 0.12 * v, pan, delay: d, saw: true, lp: 2800.0 });
            o.push(Cmd::Tone { f: f * 1.1, to: f * 0.7, dur: 0.14, vol: 0.17 * v, pan, delay: d + 0.05, saw: true, lp: 2400.0 });
            o.push(Cmd::Tone { f: f * 2.3, to: f * 1.5, dur: 0.13, vol: 0.05 * v, pan, delay: d + 0.05, saw: true, lp: 3800.0 });
            o.push(Cmd::Play { l: L { lp: 3000.0, ..l("snort", 2.1, 0.16 * v) }, k: 1.0, pan, delay: d + 0.02 }); }
        for c in o { self.send(c); }
    }
    /// Cri de renard : une plainte aiguë et rauque qui monte puis retombe.
    pub fn fox_scream(&mut self, pos: [f64; 3]) {
        let Some((v, pan)) = self.at(pos, 90.0) else { return }; let f = self.rnd.range(1050.0, 1300.0);
        for (a, b, dur, d, k) in [(f, f * 1.25, 0.22, 0.0, 1.0), (f * 1.25, f * 0.6, 0.55, 0.22, 1.0), (f * 2.1, f * 1.3, 0.6, 0.15, 0.3)] { self.send(Cmd::Tone { f: a, to: b, dur, vol: 0.13 * v * k, pan, delay: d, saw: true, lp: 3200.0 }); }
        self.send(Cmd::Play { l: L { lp: 3500.0, ..l("puff", 2.4, 0.18 * v) }, k: 1.0, pan, delay: 0.05 });
    }
    /// Lièvre qui détale : un coup sourd de patte arrière sur le sol.
    pub fn thump(&self, pos: [f64; 3]) { if let Some((v, pan)) = self.at(pos, 22.0) { self.send(Cmd::Play { l: L { lp: 500.0, ..l("knock", 0.5, 0.5 * v) }, k: 1.0, pan, delay: 0.0 }); self.send(Cmd::Play { l: L { lp: 500.0, ..l("knock", 0.45, 0.4 * v) }, k: 1.0, pan, delay: 0.16 }); } }
    /// Virevoltant qui retombe : un froissement de brindilles sèches.
    pub fn rustle(&self, pos: [f64; 3]) { if let Some((v, pan)) = self.at(pos, 24.0) { self.send(Cmd::Play { l: l("leaf", 1.3, 0.3 * v), k: 1.0, pan, delay: 0.0 }); self.send(Cmd::Play { l: l("crumble", 1.6, 0.12 * v), k: 1.0, pan, delay: 0.03 }); } }
    /// Branche qui casse au loin, puis la neige qui tombe des aiguilles.
    pub fn branch(&self, pos: [f64; 3]) { if let Some((v, pan)) = self.at(pos, 40.0) { self.send(Cmd::Play { l: l("crack", 1.25, 0.32 * v), k: 1.0, pan, delay: 0.0 }); self.send(Cmd::Play { l: L { lp: 2500.0, ..l("sand", 0.7, 0.22 * v) }, k: 1.0, pan, delay: 0.25 }); } }
    pub fn mood(&self, m: Mood) { self.send(Cmd::Mood(m)); }
    pub fn force(&self, m: Option<Mood>) { self.send(Cmd::Force(m)); }
    pub fn volumes(&self, master: f32, sfx: f32, music: f32) { self.send(Cmd::Volumes { master, sfx, music }); }
}
