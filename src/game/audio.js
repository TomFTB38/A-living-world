// Audio/audio.js — bruitages : vraies prises de son libres (assets/sounds/CREDITS.md), lues sans claquement.
import { Music } from './music.js';

const RAW = typeof SOUND_DATA !== 'undefined' ? SOUND_DATA : {}; // injecté par tools/build.mjs
export const LOOPS = { wind: 12, rain: 12, brook: 10.962, fall: 16.567 }, LOOP_PAD = 0.25; // voir tools/make_sounds.py

const L = (set, rate = 1, vol = 1, lp = 0, at = 0) => ({ set, rate, vol, lp, at });
// Chaque bloc a sa matière : dure et sèche, sableuse, grasse, boisée... `m` choisit aussi comment l'outil y résonne.
// Aucune de ces prises ne garde de grave de tambour : la terre grène, la pierre claque, le tronc cogne sec.
const mat = (m, step, hit, brk, put) => ({ m, step, hit, brk, put: put || [{ ...hit[0], vol: hit[0].vol * 0.9 }] });
const plant = (rate) => mat('plant', [L('leaf', rate, 0.2)], [L('leaf', rate, 0.3)], [L('leaf', rate * 0.9, 0.45)]);
const log = (r) => mat('wood', [L('plankdry', 0.9 * r, 0.3), L('wood', 0.95 * r, 0.2)], [L('plankdry', 0.95 * r, 0.56), L('chopdry', r, 0.36)], [L('plankdry', 0.84 * r, 0.66), L('chopdry', 0.85 * r, 0.56), L('crumble', 0.5, 0.14, 2600, 0.025)], [L('chopdry', 0.95 * r, 0.56)]);
// La pierre se pose comme un tronc, en plus grave : un choc plein, pas un claquement.
const stonePut = (r) => [L('chopdry', r, 0.58)];
export const SOUNDS = {
  grass: mat('earth', [L('stepgrass', 1, 0.5), L('sand', 1.1, 0.12)], [L('sand', 0.95, 0.42), L('leaf', 0.85, 0.3), L('stepgrass', 1.1, 0.24)], [L('sand', 0.8, 0.6), L('crumble', 0.6, 0.3, 3500), L('leaf', 0.8, 0.4)]),
  dirt: mat('earth', [L('sand', 0.88, 0.38)], [L('sand', 0.84, 0.5), L('crumble', 0.6, 0.26, 3500)], [L('sand', 0.72, 0.66), L('crumble', 0.55, 0.42, 3500)]),
  path: mat('earth', [L('gravel', 0.8, 0.24, 3000), L('sand', 0.9, 0.2)], [L('sand', 1, 0.42), L('crumble', 0.8, 0.3)], [L('sand', 0.86, 0.56), L('crumble', 0.75, 0.46)]),
  forest_soil: mat('earth', [L('sand', 0.7, 0.4, 2200)], [L('sand', 0.66, 0.55, 2000)], [L('sand', 0.6, 0.72, 2200), L('crumble', 0.5, 0.2, 2200)]),
  stone: mat('hard', [L('stepstone', 1, 0.5), L('crack', 1.1, 0.08)], [L('crack', 1, 0.5), L('rockhi', 1, 0.42), L('crumble', 1, 0.1, 0, 0.02)], [L('rockhi', 0.9, 0.6), L('crack', 0.85, 0.55), L('crumble', 0.85, 0.5, 0, 0.03)], stonePut(0.78)),
  mossy_stone: mat('hard', [L('stepstone', 0.9, 0.42, 3000)], [L('rockhi', 0.85, 0.38, 3000), L('crack', 0.9, 0.3, 2800)], [L('rockhi', 0.8, 0.5, 3000), L('crack', 0.8, 0.36, 2800), L('sand', 0.9, 0.26, 0, 0.03)], stonePut(0.74)),
  limestone: mat('hard', [L('stepstone', 1.12, 0.46)], [L('crack', 1.2, 0.44), L('rockhi', 1.2, 0.3), L('crumble', 1.1, 0.24)], [L('crack', 1.15, 0.5), L('crumble', 1, 0.5), L('rockhi', 1.1, 0.36)], stonePut(0.84)),
  granite: mat('hard', [L('stepstone', 0.94, 0.54)], [L('rockhi', 0.82, 0.5), L('crack', 0.85, 0.5)], [L('rockhi', 0.74, 0.66), L('crack', 0.78, 0.55), L('crumble', 0.75, 0.4, 0, 0.03)], stonePut(0.7)),
  gravel: mat('earth', [L('gravel', 1, 0.42)], [L('crumble', 1, 0.5)], [L('crumble', 0.95, 0.66), L('gravel', 1.05, 0.36)]),
  sand: mat('sand', [L('sand', 1, 0.42)], [L('sand', 1.06, 0.5)], [L('sand', 0.94, 0.66), L('crumble', 1.3, 0.18)]),
  oak_log: log(1), birch_log: log(1.12), beech_log: log(1.05),
  oak_leaves: mat('plant', [L('stepgrass', 1.3, 0.36)], [L('leaf', 1, 0.42)], [L('leaf', 0.9, 0.56), L('stepgrass', 1.35, 0.3)]),
  birch_leaves: mat('plant', [L('stepgrass', 1.4, 0.34)], [L('leaf', 1.12, 0.42)], [L('leaf', 1.02, 0.56), L('stepgrass', 1.45, 0.3)]),
  beech_leaves: mat('plant', [L('stepgrass', 1.22, 0.36)], [L('leaf', 0.94, 0.42)], [L('leaf', 0.84, 0.56), L('stepgrass', 1.28, 0.3)]),
  planks: mat('wood', [L('wood', 1, 0.56)], [L('plank', 1, 0.5)], [L('plank', 0.9, 0.62), L('woodl', 1, 0.42)]),
  water: mat('plant', [L('splash', 1, 0.42)], [L('splash', 1.1, 0.3)], [L('splash', 1, 0.4)]),
  tall_grass: plant(1.25), fern: plant(1), daisy: plant(1.5), lupine: plant(1.38), pink_flower: plant(1.6), yellow_flower: plant(1.45), vine: plant(0.85),
  mushroom: mat('plant', [L('leaf', 0.6, 0.16)], [L('leaf', 0.55, 0.3)], [L('leaf', 0.5, 0.4)]),
  lily_pad: mat('plant', [L('splash', 1.5, 0.2)], [L('leaf', 0.7, 0.26)], [L('splash', 1.4, 0.28)]),
  flagstone: mat('hard', [L('stepstone', 1.05, 0.5)], [L('rockhi', 1.1, 0.42), L('crack', 1.1, 0.3)], [L('crack', 1.1, 0.52), L('crumble', 1, 0.34)], stonePut(0.8)),
  lantern: mat('metal', [L('metal', 1.2, 0.2)], [L('glass', 1, 0.3), L('metal', 1.2, 0.22)], [L('glass', 0.92, 0.46), L('metal', 1, 0.3)]),
  shelf_fungus: mat('plant', [L('leaf', 0.66, 0.16)], [L('leaf', 0.62, 0.3)], [L('leaf', 0.56, 0.4)]),
  fence: mat('wood', [L('wood', 1.1, 0.4)], [L('plank', 1.14, 0.44)], [L('plank', 1.04, 0.56), L('woodl', 1.2, 0.3)]),
};
// L'outil se devine sous la matière : le métal tinte sur la roche, s'étouffe dans la terre, mord le bois.
const TOOL = {
  pickaxe: { hard: L('metal', 1, 0.2), earth: L('sand', 1.3, 0.1), sand: L('sand', 1.4, 0.08), wood: L('plankdry', 1.2, 0.2), plant: L('leaf', 1.3, 0.1), metal: L('metal', 0.9, 0.3) },
  axe: { hard: L('metal', 0.8, 0.2), earth: L('sand', 1.15, 0.12), sand: L('sand', 1.25, 0.08), wood: L('chopdry', 1.05, 0.36), plant: L('leaf', 1, 0.18), metal: L('metal', 0.76, 0.3) },
  sword: { hard: L('metal', 1.36, 0.16), earth: L('sand', 1.5, 0.08), sand: L('sand', 1.6, 0.06), wood: L('plankdry', 1.5, 0.14), plant: L('leaf', 1.25, 0.2), metal: L('metal', 1.3, 0.26) },
  hand: { hard: L('knock', 1.4, 0.12), earth: L('sand', 1.1, 0.12), sand: L('sand', 1.2, 0.1), wood: L('plankdry', 1, 0.16), plant: null, metal: L('knock', 1.5, 0.14) },
};
// Les variantes orientées sonnent comme le bloc d'origine.
export const soundOf = (name) => SOUNDS[(name || '').replace(/_(x|y|z)$/, '')] || SOUNDS.dirt;

// Ces prises sont riches en aigus secs : on les arrondit pour éviter tout grésillement.
const ROUND = { leaf: 4200, crumble: 6000, gravel: 5200, sand: 7000 };
const rnd = (a, b) => a + Math.random() * (b - a);

export class SoundScape {
  constructor() { this.ctx = null; this.volume = 0.7; this.musicVolume = 0.5; this.sfxVolume = 0.7; this.L = { p: [0, 0, 0], yaw: 0 }; this.bank = {}; this.last = {}; }
  start(ctx) {
    if (this.ctx) { if (this.ctx.state === 'suspended') this.ctx.resume(); return; }
    const AC = window.AudioContext || window.webkitAudioContext; if (!AC && !ctx) return;
    const c = this.ctx = ctx || new AC();
    const lim = c.createDynamicsCompressor(); lim.threshold.value = -6; lim.ratio.value = 6; lim.attack.value = 0.012; lim.release.value = 0.3; lim.connect(c.destination);
    this.master = c.createGain(); this.master.gain.value = this.volume; this.master.connect(lim);
    this.sfx = c.createGain(); this.sfx.gain.value = this.sfxVolume; this.sfx.connect(this.master);
    this.wind = this.bed(); this.rain = this.bed(); this.brook = this.bed(); this.fall = this.bed();
    this.ready = this.load();
    this.music = new Music(c, this.master); this.music.setVolume(this.musicVolume);
  }
  bed() { const g = this.ctx.createGain(), pan = this.panner(); g.gain.value = 0; g.connect(pan); pan.connect(this.master); return { g, pan }; }
  // Décodage au premier geste. Les fichiers ne sont lus qu'une fois prêts ; d'ici là, le jeu reste simplement silencieux.
  async load() {
    const c = this.ctx, jobs = Object.entries(RAW).map(async ([name, b64]) => {
      const bin = atob(b64), u8 = new Uint8Array(bin.length); for (let i = 0; i < bin.length; i++) u8[i] = bin.charCodeAt(i);
      const buf = await c.decodeAudioData(u8.buffer), d = buf.getChannelData(0); let at = 0; const th = 0.01; while (at < d.length && Math.abs(d[at]) < th) at++;
      const m = /^(.*)_(\d)$/.exec(name);
      if (m) (this.bank[m[1]] = this.bank[m[1]] || []).push({ buf, at: Math.max(0, at - 24) / buf.sampleRate });
      else this.startLoop(name.replace('loop_', ''), buf);
    });
    await Promise.all(jobs); this.loaded = true;
  }
  startLoop(n, buf) { const s = this.ctx.createBufferSource(); s.buffer = buf; s.loop = true; s.loopStart = LOOP_PAD; s.loopEnd = LOOP_PAD + LOOPS[n]; s.connect(this[n].g); s.start(0, LOOP_PAD + Math.random() * 3); }
  panner() { const c = this.ctx; if (c.createStereoPanner) return c.createStereoPanner(); const g = c.createGain(); g.pan = { value: 0, setTargetAtTime() {} }; return g; }
  setVolume(v) { this.volume = v; if (this.master) this.master.gain.value = v; }
  setSfxVolume(v) { this.sfxVolume = v; if (this.sfx) this.sfx.gain.value = v; }
  setMusicVolume(v) { this.musicVolume = v; this.music?.setVolume(v); }
  get live() { const c = this.ctx; return !!c && (c.state === 'running' || c.oncomplete !== undefined); }
  listener(p, yaw) { this.L.p = p; this.L.yaw = yaw; }
  // Volume et position d'un son dans le monde, selon la distance et l'orientation du joueur.
  place3d(pos, radius) {
    const l = this.L.p, dx = pos[0] - l[0], dy = pos[1] - l[1], dz = pos[2] - l[2], d = Math.hypot(dx, dy, dz);
    if (d >= radius) return null;
    const a = 1 - d / radius, side = (dx * Math.cos(this.L.yaw) - dz * Math.sin(this.L.yaw)) / Math.max(d, 1);
    return { v: a * a, pan: 0.7 * Math.max(-1, Math.min(1, side)) };
  }
  // Ambiance : le vent n'existe que par mauvais temps, l'eau dépend de ce qui coule autour du joueur.
  env(s) {
    const c = this.ctx; if (!c || !this.live) return;
    const t = c.currentTime + 0.05, set = (n, v, k) => n.g.gain.setTargetAtTime(v, t, k);
    const storm = Math.max(s.rain, Math.max(0, s.cloud - 0.72) / 0.28 * 0.4), q = s.covered ? 0.3 : 1;
    set(this.wind, storm > 0.02 ? 0.3 * storm * q : 0, 1.2);
    set(this.rain, 0.24 * s.rain * q, 1);
    const flow = s.brook && this.place3d(s.brook, 15), drop = s.fall && this.place3d(s.fall, 30);
    set(this.brook, flow ? 0.3 * flow.v : 0, 0.5); this.brook.pan.pan.setTargetAtTime(flow ? flow.pan : 0, t, 0.3);
    set(this.fall, drop ? 0.42 * drop.v : 0, 0.5); this.fall.pan.pan.setTargetAtTime(drop ? drop.pan : 0, t, 0.3);
  }
  out(pan) { if (!pan) return this.sfx; const p = this.panner(); p.pan.value = pan; p.connect(this.sfx); return p; }
  // Lit une prise de son. Jamais deux fois de suite la même, et légère variation de hauteur pour rester naturel.
  play(l, k = 1, dest = this.sfx, at = 0) {
    const set = this.bank[l.set]; if (!set || !set.length) return;
    let i = Math.floor(Math.random() * set.length); if (set.length > 1 && i === this.last[l.set]) i = (i + 1) % set.length; this.last[l.set] = i;
    const c = this.ctx, s = c.createBufferSource(), g = c.createGain(), t = c.currentTime + at + (l.at || 0); s.buffer = set[i].buf; s.playbackRate.value = l.rate * rnd(0.96, 1.04); g.gain.value = l.vol * k * rnd(0.9, 1);
    const lp = l.lp || ROUND[l.set]; let tail = s; if (lp) { const f = c.createBiquadFilter(); f.type = 'lowpass'; f.frequency.value = lp; f.Q.value = 0.5; s.connect(f); tail = f; }
    tail.connect(g); g.connect(dest); s.start(t, set[i].at);
  }
  layers(ls, k, dest) { for (const l of ls) this.play(l, k, dest); }
  tone(freq, dur, vol, at = 0, dest = this.sfx, to = 0, type = 'sine') {
    const c = this.ctx, o = c.createOscillator(), g = c.createGain(), t = c.currentTime + at;
    o.type = type; o.frequency.setValueAtTime(freq, t); if (to) o.frequency.exponentialRampToValueAtTime(to, t + dur);
    g.gain.setValueAtTime(0.0001, t); g.gain.exponentialRampToValueAtTime(vol, t + 0.012); g.gain.exponentialRampToValueAtTime(0.0001, t + dur);
    o.connect(g); g.connect(dest); o.start(t); o.stop(t + dur + 0.02);
  }
  // Pas : le bloc sous le pied, plus le froissement de ce qu'on traverse.
  step(name, through) { if (!this.live) return; this.layers(soundOf(name).step, 0.6); if (through) this.layers(soundOf(through).step, 0.3); }
  land(name) { if (this.live) this.layers(soundOf(name).step, 0.8); }
  // Frappe : la matière d'abord, l'outil en léger, tel qu'il sonne sur cette matière.
  strike(name, tool) { const b = soundOf(name), t = (TOOL[tool] || TOOL.hand)[b.m]; if (t) this.play(t); if (tool === 'sword') this.swing(tool, 0.45); return b; }
  dig(name, tool) { if (this.live) this.layers(this.strike(name, tool).hit, 0.85); }
  breakBlock(name, tool) { if (this.live) this.layers(this.strike(name, tool).brk, 1); }
  place(name) { if (this.live) this.layers(soundOf(name).put, 0.9); }
  // Dans le vide : la lame siffle, les outils lourds brassent l'air, le poing ou le bloc font un souffle bref.
  swing(tool, k = 1) {
    if (!this.live) return;
    if (tool === 'sword') this.play(L('blade', 1, 0.5), k); else if (tool === 'pickaxe' || tool === 'axe') this.play(L('whoosh', tool === 'axe' ? 0.9 : 1.05, 0.36), k); else this.play(L('puff', 1, 0.3), k);
  }
  hurt() { if (this.live) this.play(L('hurt', 1, 0.6)); }
  // Faune : uniquement à proximité.
  bird(pos, voice) {
    const at = this.live && this.place3d(pos, 22); if (!at) return;
    const dest = this.out(at.pan), base = [3100, 2300, 3900][voice] * rnd(0.94, 1.08), n = [6, 3, 4][voice] + Math.floor(Math.random() * 3), gap = [0.07, 0.2, 0.12][voice];
    for (let i = 0; i < n; i++) {
      const f = voice === 1 ? base * (1 + 0.19 * (i % 3)) : base * (1 + 0.1 * (i % 2)), to = voice === 2 ? f * 0.74 : f * 1.2;
      this.tone(f, gap * 0.85, 0.125 * at.v, i * gap, dest, to);
    }
  }
  wings(pos) { const at = this.live && this.place3d(pos, 16); if (!at) return; const d = this.out(at.pan); for (let i = 0; i < 4; i++) this.play(L('puff', 1.7, 0.22 * at.v), 1, d, i * 0.07); }
  hoof(pos, fast) { const at = this.live && this.place3d(pos, fast ? 30 : 20); if (at) this.play(L('soft', 0.8, 0.3 * at.v, 1500), 1, this.out(at.pan)); }
  // Cerf : souffle d'alarme quand il s'enfuit, souffle bas quand il broute.
  deerAlarm(pos) { const at = this.live && this.place3d(pos, 40); if (!at) return; const d = this.out(at.pan); this.play(L('snort', 1, 0.6 * at.v), 1, d); this.play(L('snort', 0.92, 0.45 * at.v), 1, d, 0.42); }
  deerCall(pos) { const at = this.live && this.place3d(pos, 26); if (at) this.play(L('snort', 0.7, 0.24 * at.v, 900), 1, this.out(at.pan)); }
}
