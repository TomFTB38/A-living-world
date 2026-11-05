// Audio/music.js — musique générative : des phrases composées à la volée, séparées de silences, selon la situation.
const hz = (m) => 440 * 2 ** ((m - 69) / 12);
const mrand = (a, b) => a + Math.random() * (b - a), pick = (a) => a[Math.floor(Math.random() * a.length)];

// La situation du joueur décide de l'humeur : le danger prime, puis ce qui inquiète, puis ce qu'il fait.
export function pickMood(s) {
  if (s.hp < 35 || s.food < 10 || s.water < 10 || s.falling) return 'danger';
  if (s.underground || s.submerged) return 'suspense';
  if (s.building) return 'construction';
  if (s.night || s.rain) return 'nuit';
  return 'calme';
}

export const MOODS = {
  calme: { bpm: 66, root: 62, scale: [0, 2, 4, 7, 9], chords: [[0, 4, 7], [9, 12, 16], [5, 9, 12], [7, 11, 14]], inst: 'piano', density: 0.38, bars: 8, rest: [16, 38], fade: 3.5 },
  nuit: { bpm: 52, root: 57, scale: [0, 3, 5, 7, 10], chords: [[0, 3, 7], [8, 12, 15], [5, 8, 12], [7, 10, 14]], inst: 'celesta', density: 0.26, bars: 6, rest: [12, 30], fade: 3.5 },
  construction: { bpm: 92, root: 67, scale: [0, 2, 4, 7, 9], chords: [[0, 4, 7], [5, 9, 12], [9, 12, 16], [7, 11, 14]], inst: 'pluck', density: 0.6, bars: 8, rest: [5, 12], arp: true, fade: 2.5 },
  suspense: { bpm: 46, root: 38, fade: 2.5 },
  danger: { bpm: 128, root: 38, fade: 0.8 },
};

export class Music {
  constructor(ctx, dest) {
    this.c = ctx; this.want = 'calme'; this.forced = 'auto'; this.mood = null; this.next = ctx.currentTime + 3; this.seen = 0; this.phrase = null;
    this.bus = ctx.createGain(); this.bus.connect(dest);
    // Salle : réponse impulsionnelle générée, qui donne du lien entre les notes.
    const n = Math.floor(ctx.sampleRate * 2.6), ir = ctx.createBuffer(2, n, ctx.sampleRate);
    for (let ch = 0; ch < 2; ch++) { const d = ir.getChannelData(ch); for (let i = 0; i < n; i++) d[i] = (Math.random() * 2 - 1) * (1 - i / n) ** 2.6; }
    this.rev = ctx.createConvolver(); this.rev.buffer = ir; const w = ctx.createGain(); w.gain.value = 0.34; this.rev.connect(w); w.connect(this.bus);
  }
  setVolume(v) { this.bus.gain.value = v; }
  // Une phrase a son propre gain : changer d'humeur la fait s'éteindre sans couper les notes.
  open() {
    const c = this.c, g = c.createGain(); g.gain.value = 1; g.connect(this.bus); g.connect(this.rev); this.out = g;
    return g;
  }
  close(fade) {
    const o = this.out; if (!o) return; const t = this.c.currentTime; o.gain.setTargetAtTime(0, t, fade / 3);
    setTimeout?.(() => { try { o.disconnect(); } catch (e) { /* déjà débranchée */ } }, (fade + 4) * 1000); this.out = null;
  }
  update(dt, mood) {
    this.want = this.forced === 'auto' ? mood : this.forced;
    const c = this.c; if (c.state !== 'running' && c.oncomplete === undefined) return;
    this.seen = this.want === this.mood ? 0 : this.seen + dt;
    // Le danger entre aussitôt ; le reste attend que la situation se confirme.
    if (this.want !== this.mood && (this.want === 'danger' || this.seen > 2.5 || this.mood === null)) this.begin(this.want);
    while (this.mood && this.next < c.currentTime + 0.5) this.schedule();
  }
  begin(m) {
    const was = this.mood; this.close(MOODS[m].fade); this.mood = m; this.seen = 0; this.open(); this.bar = 0; this.step = 0; this.last = 2 + Math.floor(Math.random() * 3); this.resting = false;
    this.next = Math.max(this.next > this.c.currentTime + 6 ? this.c.currentTime + 1 : this.next, this.c.currentTime + (m === 'danger' ? 0.05 : was ? 1 : 3));
  }
  schedule() { const m = this.mood; if (m === 'suspense') this.suspense(); else if (m === 'danger') this.danger(); else this.tonal(MOODS[m]); }
  // Une note par demi-temps, en marche par degrés, avec des respirations.
  tonal(P) {
    const t = this.next, e = 30 / P.bpm, s = this.step, o = this.out;
    if (s % 8 === 0) {
      if (this.bar >= P.bars) { this.bar = 0; this.step = 0; this.close(2); this.open(); this.next = t + mrand(P.rest[0], P.rest[1]); return; }
      const ch = P.chords[Math.floor(this.bar / 2) % P.chords.length];
      if (this.bar % 2 === 0) this.pad(ch.map((n) => hz(P.root - 12 + n)), t, e * 16 * 1.05, 0.03);
      this.note(hz(P.root - 24 + ch[0]), t, 0.1, P.inst, o); this.bar++;
    }
    const ch = P.chords[Math.floor((this.bar - 1) / 2) % P.chords.length];
    if (P.arp && s % 2 === 0) this.note(hz(P.root + ch[(s / 2) % 3]), t, 0.055, P.inst, o);
    if (Math.random() < P.density * (s % 4 === 0 ? 1.6 : s % 2 === 0 ? 1 : 0.45)) {
      this.last = Math.max(0, Math.min(9, this.last + pick([-2, -1, -1, 1, 1, 2]))); const sc = P.scale, deg = this.last % sc.length, oct = Math.floor(this.last / sc.length);
      this.note(hz(P.root + 12 * oct + sc[deg]), t + mrand(0, 0.012), mrand(0.07, 0.115), P.inst, o);
    }
    this.step = s + 1; this.next = t + e;
  }
  // Suspense : une basse tenue, deux notes voisines qui frottent, des coups isolés dans le noir.
  suspense() {
    const t = this.next, b = 60 / MOODS.suspense.bpm, o = this.out, r = MOODS.suspense.root;
    this.pad([hz(r), hz(r + 7)], t, b * 8, 0.05, 340);
    if (this.bar % 2 === 1) this.pad([hz(r + 37), hz(r + 38)], t + b, b * 5, 0.012, 1300);
    if (Math.random() < 0.55) this.heart(t + b * mrand(1, 5), 0.2);
    if (Math.random() < 0.7) this.note(hz(r + 24 + pick([0, 1, 6, 7, 13])), t + b * mrand(2, 7), 0.05, 'pluck', o);
    this.bar++; this.next = t + b * 8;
  }
  // Danger : ostinato grave, grand tambour, accents dissonants.
  danger() {
    const t = this.next, e = 15 / MOODS.danger.bpm, s = this.step % 32, o = this.out, r = MOODS.danger.root;
    const line = [0, 0, 12, 0, 0, 1, 0, 0, 0, 0, 12, 0, 0, 6, 0, 1];
    if (s % 2 === 0 || line[s % 16]) this.saw(hz(r + line[s % 16]), t, e * 1.5, 0.1);
    if ([0, 6, 8, 16, 22, 24, 28].includes(s)) this.drum(t, s % 8 === 0 ? 0.3 : 0.2);
    if (s === 12 || s === 28) this.pad([hz(r + 25), hz(r + 26), hz(r + 31)], t, e * 4, 0.035, 2200);
    this.step++; this.next = t + e;
  }
  env(g, t, a, peak, dur) { g.gain.setValueAtTime(0.0001, t); g.gain.exponentialRampToValueAtTime(peak, t + a); g.gain.exponentialRampToValueAtTime(0.0001, t + dur); }
  note(f, t, v, inst, dest) {
    const c = this.c, g = c.createGain(), out = c.createBiquadFilter(); out.type = 'lowpass'; out.frequency.value = inst === 'pluck' ? 2200 : 3400; g.connect(out); out.connect(dest);
    const parts = inst === 'celesta' ? [[1, 1], [4, 0.28]] : inst === 'pluck' ? [[1, 1], [2, 0.3]] : [[1, 1], [2, 0.42], [3, 0.12]], dur = inst === 'celesta' ? 1.5 : inst === 'pluck' ? 0.75 : 2.2;
    this.env(g, t, 0.006, v, dur);
    for (const [k, a] of parts) { const o = c.createOscillator(), h = c.createGain(); o.type = inst === 'pluck' ? 'triangle' : 'sine'; o.frequency.value = f * k; h.gain.value = a; o.connect(h); h.connect(g); o.start(t); o.stop(t + dur + 0.05); }
  }
  pad(freqs, t, dur, v, cut = 760) {
    const c = this.c, g = c.createGain(), f = c.createBiquadFilter(), a = Math.min(2, dur * 0.3); f.type = 'lowpass'; f.frequency.value = cut; f.connect(g); g.connect(this.out);
    g.gain.setValueAtTime(0, t); g.gain.linearRampToValueAtTime(v, t + a); g.gain.setValueAtTime(v, t + dur - a); g.gain.linearRampToValueAtTime(0, t + dur);
    for (const fr of freqs) for (const d of [-4, 5]) { const o = c.createOscillator(); o.type = 'sawtooth'; o.frequency.value = fr; o.detune.value = d; o.connect(f); o.start(t); o.stop(t + dur + 0.05); }
  }
  saw(f, t, dur, v) { const c = this.c, o = c.createOscillator(), g = c.createGain(), l = c.createBiquadFilter(); o.type = 'sawtooth'; o.frequency.value = f; l.type = 'lowpass'; l.frequency.value = 620; this.env(g, t, 0.004, v, dur); o.connect(l); l.connect(g); g.connect(this.out); o.start(t); o.stop(t + dur + 0.02); }
  drum(t, v) { const c = this.c, o = c.createOscillator(), g = c.createGain(); o.frequency.setValueAtTime(115, t); o.frequency.exponentialRampToValueAtTime(46, t + 0.22); this.env(g, t, 0.003, v, 0.42); o.connect(g); g.connect(this.out); o.start(t); o.stop(t + 0.45); }
  heart(t, v) { this.drum(t, v); this.drum(t + 0.26, v * 0.65); }
}
