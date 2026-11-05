// Audio/audio.js — bruitages synthétisés à la volée, sans aucun fichier. Démarre au premier geste du joueur.
import { Music } from './music.js';

// Une signature par bloc : souffle filtré (n), corps grave (b), résonance (r), grains (g) et durée (d).
export const SOUNDS = {
  grass: { n: ['highpass', 1900, 0.5], d: 0.11, v: 0.34, b: [120, 0.05, 0.1], g: 2 },
  dirt: { n: ['lowpass', 620, 0.8], d: 0.09, v: 0.5, b: [105, 0.07, 0.3] },
  path: { n: ['bandpass', 1500, 0.8], d: 0.08, v: 0.42, b: [130, 0.05, 0.16], g: 3 },
  forest_soil: { n: ['lowpass', 480, 0.7], d: 0.12, v: 0.46, b: [90, 0.08, 0.28], g: 1 },
  stone: { n: ['bandpass', 2300, 1.3], d: 0.05, v: 0.42, b: [210, 0.04, 0.4], r: [1250, 0.03, 0.06] },
  mossy_stone: { n: ['bandpass', 1500, 1], d: 0.06, v: 0.34, b: [175, 0.05, 0.42] },
  limestone: { n: ['bandpass', 2900, 1.1], d: 0.05, v: 0.36, b: [245, 0.035, 0.32], g: 1 },
  granite: { n: ['bandpass', 3400, 1.6], d: 0.04, v: 0.46, b: [270, 0.04, 0.42], r: [1720, 0.035, 0.08] },
  gravel: { n: ['bandpass', 2700, 0.6], d: 0.1, v: 0.4, g: 6 },
  sand: { n: ['highpass', 2700, 0.4], d: 0.15, v: 0.24, g: 4 },
  oak_log: { n: ['bandpass', 760, 1.1], d: 0.07, v: 0.4, b: [165, 0.1, 0.42], r: [430, 0.07, 0.08] },
  birch_log: { n: ['bandpass', 1050, 1.1], d: 0.06, v: 0.4, b: [215, 0.08, 0.38], r: [590, 0.06, 0.08] },
  beech_log: { n: ['bandpass', 880, 1.3], d: 0.065, v: 0.4, b: [185, 0.09, 0.42], r: [510, 0.07, 0.08] },
  oak_leaves: { n: ['highpass', 2500, 0.5], d: 0.16, v: 0.26, g: 5 },
  birch_leaves: { n: ['highpass', 3300, 0.5], d: 0.14, v: 0.24, g: 5 },
  beech_leaves: { n: ['highpass', 2100, 0.5], d: 0.17, v: 0.26, g: 4 },
  planks: { n: ['bandpass', 1050, 1.5], d: 0.05, v: 0.34, b: [145, 0.12, 0.5], r: [335, 0.11, 0.12] },
  water: { n: ['bandpass', 1400, 0.5], d: 0.22, v: 0.34, b: [240, 0.08, 0.1], g: 3 },
  tall_grass: { n: ['highpass', 3000, 0.5], d: 0.09, v: 0.2, g: 2 },
  fern: { n: ['highpass', 2300, 0.6], d: 0.11, v: 0.22, g: 3 },
  daisy: { n: ['highpass', 4100, 0.6], d: 0.06, v: 0.14 },
  lupine: { n: ['highpass', 3600, 0.6], d: 0.07, v: 0.15, g: 1 },
  pink_flower: { n: ['highpass', 4500, 0.6], d: 0.05, v: 0.14 },
  yellow_flower: { n: ['highpass', 3900, 0.6], d: 0.055, v: 0.14 },
  mushroom: { n: ['lowpass', 900, 1.2], d: 0.05, v: 0.3, b: [310, 0.04, 0.18] },
  lily_pad: { n: ['bandpass', 1200, 0.8], d: 0.1, v: 0.22, b: [280, 0.05, 0.08] },
  flagstone: { n: ['bandpass', 1900, 1.2], d: 0.05, v: 0.4, b: [190, 0.05, 0.42] },
  lantern: { n: ['highpass', 3600, 1], d: 0.03, v: 0.2, b: [320, 0.03, 0.14], r: [2650, 0.2, 0.09] },
  vine: { n: ['bandpass', 2000, 0.9], d: 0.09, v: 0.18, g: 2 },
  shelf_fungus: { n: ['lowpass', 1100, 1], d: 0.05, v: 0.28, b: [360, 0.04, 0.14] },
  fence: { n: ['bandpass', 1250, 1.6], d: 0.045, v: 0.32, b: [230, 0.08, 0.36], r: [640, 0.07, 0.1] },
};
// Les variantes orientées sonnent comme le bloc d'origine.
export const soundOf = (name) => SOUNDS[(name || '').replace(/_(x|z)$/, '')] || SOUNDS.dirt;

const rnd = (a, b) => a + Math.random() * (b - a);

export class SoundScape {
  constructor() { this.ctx = null; this.volume = 0.7; this.musicVolume = 0.5; this.L = { p: [0, 0, 0], yaw: 0 }; }
  start(ctx) {
    if (this.ctx) { if (this.ctx.state === 'suspended') this.ctx.resume(); return; }
    const AC = window.AudioContext || window.webkitAudioContext; if (!AC && !ctx) return;
    const c = this.ctx = ctx || new AC();
    const lim = c.createDynamicsCompressor(); lim.threshold.value = -9; lim.ratio.value = 8; lim.attack.value = 0.004; lim.release.value = 0.2; lim.connect(c.destination);
    this.master = c.createGain(); this.master.gain.value = this.volume; this.master.connect(lim);
    this.sfx = c.createGain(); this.sfx.connect(this.master);
    const buf = c.createBuffer(1, c.sampleRate * 3, c.sampleRate), d = buf.getChannelData(0);
    for (let i = 0; i < d.length; i++) d[i] = Math.random() * 2 - 1;
    this.noise = buf;
    const loop = (nodes) => {
      const s = c.createBufferSource(); s.buffer = buf; s.loop = true; let last = s;
      for (const [type, freq, q] of nodes) { const f = c.createBiquadFilter(); f.type = type; f.frequency.value = freq; f.Q.value = q; last.connect(f); last = f; }
      const g = c.createGain(); g.gain.value = 0; const pan = this.panner(); last.connect(g); g.connect(pan); pan.connect(this.master); s.start(); return { g, pan, f: last };
    };
    this.wind = loop([['lowpass', 420, 0.6]]); this.rain = loop([['highpass', 2400, 0.3]]);
    this.brook = loop([['bandpass', 1750, 0.9], ['highpass', 500, 0.5]]); // léger écoulement
    this.fall = loop([['lowpass', 2300, 0.4], ['highpass', 170, 0.4]]); // chute d'eau : plus large et plus grave
    // Le ruisseau gargouille : son niveau et sa couleur ondulent lentement.
    const lfo = c.createOscillator(), lg = c.createGain(); lfo.frequency.value = 3.1; lg.gain.value = 420; lfo.connect(lg); lg.connect(this.brook.f.frequency); lfo.start();
    this.music = new Music(c, this.master); this.music.setVolume(this.musicVolume);
  }
  panner() { const c = this.ctx; if (c.createStereoPanner) return c.createStereoPanner(); const g = c.createGain(); g.pan = { value: 0, setTargetAtTime() {} }; return g; }
  setVolume(v) { this.volume = v; if (this.master) this.master.gain.value = v; }
  setMusicVolume(v) { this.musicVolume = v; this.music?.setVolume(v); }
  get live() { const c = this.ctx; return c && (c.state === 'running' || c.renderedBuffer !== undefined || c.oncomplete !== undefined); }
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
    const storm = Math.max(s.rain, Math.max(0, s.cloud - 0.72) / 0.28 * 0.45), q = s.covered ? 0.3 : 1;
    set(this.wind, storm > 0.02 ? 0.16 * storm * q : 0, 1.2);
    this.wind.f.frequency.setTargetAtTime(340 + 180 * Math.sin(t * 0.13), t, 1.5);
    set(this.rain, 0.12 * s.rain * q, 1);
    const flow = s.brook && this.place3d(s.brook, 15), drop = s.fall && this.place3d(s.fall, 30);
    set(this.brook, flow ? 0.09 * flow.v : 0, 0.4); this.brook.pan.pan.setTargetAtTime(flow ? flow.pan : 0, t, 0.3);
    set(this.fall, drop ? 0.16 * drop.v : 0, 0.4); this.fall.pan.pan.setTargetAtTime(drop ? drop.pan : 0, t, 0.3);
  }
  out(pan) { if (!pan) return this.sfx; const p = this.panner(); p.pan.value = pan; p.connect(this.sfx); return p; }
  burst(type, freq, q, dur, vol, at = 0, dest = this.sfx) {
    const c = this.ctx, s = c.createBufferSource(), f = c.createBiquadFilter(), g = c.createGain(), t = c.currentTime + at;
    s.buffer = this.noise; s.playbackRate.value = rnd(0.75, 1.25); f.type = type; f.frequency.value = freq * rnd(0.88, 1.12); f.Q.value = q;
    g.gain.setValueAtTime(vol, t); g.gain.exponentialRampToValueAtTime(0.0006, t + dur);
    s.connect(f); f.connect(g); g.connect(dest); s.start(t, Math.random() * 2, dur + 0.02); return f;
  }
  tone(freq, dur, vol, at = 0, dest = this.sfx, to = 0, type = 'sine') {
    const c = this.ctx, o = c.createOscillator(), g = c.createGain(), t = c.currentTime + at;
    o.type = type; o.frequency.setValueAtTime(freq, t); if (to) o.frequency.exponentialRampToValueAtTime(to, t + dur);
    g.gain.setValueAtTime(0.0006, t); g.gain.exponentialRampToValueAtTime(vol, t + 0.003); g.gain.exponentialRampToValueAtTime(0.0006, t + dur);
    o.connect(g); g.connect(dest); o.start(t); o.stop(t + dur + 0.02);
  }
  hit(p, k = 1, dk = 1, dest = this.sfx, grains = 0) {
    const pitch = rnd(0.93, 1.08);
    this.burst(p.n[0], p.n[1] * pitch, p.n[2], p.d * dk, p.v * k, 0, dest);
    if (p.b) this.tone(p.b[0] * 1.5 * pitch, p.b[1] * dk, p.b[2] * k, 0, dest, p.b[0] * pitch);
    if (p.r) this.tone(p.r[0] * pitch, p.r[1] * dk, p.r[2] * k, 0.002, dest);
    for (let i = 0, n = (p.g || 0) + grains; i < n; i++) this.burst(p.n[0], p.n[1] * rnd(0.8, 1.3), p.n[2], p.d * 0.45, p.v * k * rnd(0.3, 0.7), 0.012 + i * rnd(0.012, 0.024), dest);
  }
  // Pas : le bloc sous le pied, plus le froissement de ce qu'on traverse.
  step(name, through) { if (!this.live) return; this.hit(soundOf(name), 0.42, 0.8); if (through) this.hit(soundOf(through), 0.5, 0.9); }
  land(name) { if (this.live) this.hit(soundOf(name), 0.8, 1.2); }
  // Chaque outil a sa frappe, le bloc répond avec sa matière.
  toolHit(tool) {
    if (tool === 'pickaxe') { for (const [f, v] of [[1870, 0.1], [2950, 0.07], [4310, 0.045]]) this.tone(f * rnd(0.98, 1.02), 0.11, v); this.burst('highpass', 3200, 0.8, 0.025, 0.14); }
    else if (tool === 'axe') { this.tone(190, 0.1, 0.34, 0, this.sfx, 92); this.burst('bandpass', 1150, 1.2, 0.05, 0.28); this.tone(780, 0.05, 0.05); }
    else if (tool === 'sword') { this.burst('bandpass', 1700, 1.4, 0.12, 0.13).frequency.exponentialRampToValueAtTime(3400, this.ctx.currentTime + 0.1); for (const [f, v] of [[3120, 0.05], [5230, 0.03]]) this.tone(f, 0.16, v, 0.05); }
    else this.tone(140, 0.07, 0.24, 0, this.sfx, 85);
  }
  swing(tool) {
    if (!this.live) return;
    const p = { pickaxe: [760, 0.2, 0.09], axe: [560, 0.24, 0.1], sword: [1500, 0.17, 0.13] }[tool] || [420, 0.14, 0.05];
    this.burst('bandpass', p[0], 1.3, p[1], p[2]).frequency.exponentialRampToValueAtTime(p[0] * 2.3, this.ctx.currentTime + p[1] * 0.7);
  }
  dig(name, tool) { if (!this.live) return; this.toolHit(tool); this.hit(soundOf(name), 0.62, 0.8); }
  breakBlock(name, tool) { if (!this.live) return; this.toolHit(tool); this.hit(soundOf(name), 1, 1.5, this.sfx, 4); }
  place(name) { if (this.live) this.hit(soundOf(name), 0.8, 0.9); }
  // Faune : uniquement à proximité.
  bird(pos, voice) {
    const at = this.live && this.place3d(pos, 22); if (!at) return;
    const dest = this.out(at.pan), base = [3100, 2300, 3900][voice] * rnd(0.94, 1.08), n = [6, 3, 4][voice] + Math.floor(Math.random() * 3), gap = [0.07, 0.2, 0.12][voice];
    for (let i = 0; i < n; i++) {
      const f = voice === 1 ? base * (1 + 0.19 * (i % 3)) : base * (1 + 0.1 * (i % 2)), to = voice === 2 ? f * 0.74 : f * 1.2;
      this.tone(f, gap * 0.85, 0.09 * at.v, i * gap, dest, to);
    }
  }
  wings(pos) { const at = this.live && this.place3d(pos, 16); if (!at) return; const d = this.out(at.pan); for (let i = 0; i < 5; i++) this.burst('bandpass', 1100, 1, 0.05, 0.1 * at.v, i * 0.06, d); }
  hoof(pos, fast) { const at = this.live && this.place3d(pos, fast ? 30 : 20); if (!at) return; const d = this.out(at.pan); this.tone(150, 0.06, 0.2 * at.v, 0, d, 90); this.burst('lowpass', 760, 0.8, 0.05, 0.16 * at.v, 0, d); }
  // Cerf : aboiement d'alarme bref et rauque quand il s'enfuit, souffle quand il broute.
  deerAlarm(pos) {
    const at = this.live && this.place3d(pos, 40); if (!at) return;
    const d = this.out(at.pan), f = this.ctx.createBiquadFilter(); f.type = 'bandpass'; f.frequency.value = 820; f.Q.value = 1.2; f.connect(d);
    this.tone(430, 0.2, 0.34 * at.v, 0, f, 210, 'sawtooth'); this.burst('bandpass', 1400, 0.9, 0.15, 0.16 * at.v, 0, d); this.tone(380, 0.15, 0.2 * at.v, 0.27, f, 200, 'sawtooth');
  }
  deerCall(pos) { const at = this.live && this.place3d(pos, 26); if (!at) return; const d = this.out(at.pan); this.burst('bandpass', 620, 1.4, 0.28, 0.13 * at.v, 0, d); this.tone(135, 0.22, 0.09 * at.v, 0.02, d, 105, 'triangle'); }
}
