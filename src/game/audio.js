// Audio/audio.js — ambiance synthétisée à la volée, sans aucun fichier. Démarre au premier geste du joueur.
export class SoundScape {
  constructor() { this.ctx = null; this.volume = 0.7; }
  start() {
    if (this.ctx) { if (this.ctx.state === 'suspended') this.ctx.resume(); return; }
    const AC = window.AudioContext || window.webkitAudioContext; if (!AC) return;
    const c = this.ctx = new AC(); this.master = c.createGain(); this.master.gain.value = this.volume; this.master.connect(c.destination);
    const n = c.createBuffer(1, c.sampleRate * 3, c.sampleRate), d = n.getChannelData(0);
    for (let i = 0; i < d.length; i++) d[i] = Math.random() * 2 - 1;
    this.noise = n;
    const loop = (type, freq, q) => { const s = c.createBufferSource(); s.buffer = n; s.loop = true; const f = c.createBiquadFilter(); f.type = type; f.frequency.value = freq; f.Q.value = q; const g = c.createGain(); g.gain.value = 0; s.connect(f); f.connect(g); g.connect(this.master); s.start(); return { g, f }; };
    this.wind = loop('lowpass', 420, 0.6); this.water = loop('bandpass', 1300, 0.5); this.rain = loop('highpass', 2400, 0.3); this.nextBird = 2;
  }
  setVolume(v) { this.volume = v; if (this.master) this.master.gain.value = v; }
  env(dt, s) {
    const c = this.ctx; if (!c || c.state !== 'running') return;
    const t = c.currentTime + 0.1;
    this.wind.g.gain.setTargetAtTime(0.07 + 0.05 * s.cloud + 0.1 * s.rain, t, 0.8);
    this.wind.f.frequency.setTargetAtTime(340 + 180 * Math.sin(t * 0.13), t, 1.5);
    this.water.g.gain.setTargetAtTime(0.16 * s.water, t, 0.5);
    this.rain.g.gain.setTargetAtTime(0.13 * s.rain, t, 1);
    this.nextBird -= dt;
    if (this.nextBird <= 0 && s.day > 0.3 && s.rain < 0.3) { this.nextBird = 2.5 + Math.random() * 7; this.bird(); }
  }
  bird() {
    const c = this.ctx, base = 2300 + Math.random() * 1800, notes = 2 + Math.floor(Math.random() * 4), pan = c.createStereoPanner ? c.createStereoPanner() : null;
    if (pan) { pan.pan.value = Math.random() * 1.6 - 0.8; pan.connect(this.master); }
    for (let i = 0; i < notes; i++) {
      const o = c.createOscillator(), g = c.createGain(), t = c.currentTime + i * 0.11;
      o.frequency.setValueAtTime(base * (1 + 0.14 * (i % 2)), t); o.frequency.exponentialRampToValueAtTime(base * 1.25, t + 0.07);
      g.gain.setValueAtTime(0, t); g.gain.linearRampToValueAtTime(0.018, t + 0.015); g.gain.exponentialRampToValueAtTime(0.0005, t + 0.09);
      o.connect(g); g.connect(pan || this.master); o.start(t); o.stop(t + 0.1);
    }
  }
  burst(freq, dur, vol, type = 'lowpass') {
    const c = this.ctx; if (!c || c.state !== 'running') return;
    const s = c.createBufferSource(); s.buffer = this.noise; s.playbackRate.value = 0.6 + Math.random() * 0.8;
    const f = c.createBiquadFilter(); f.type = type; f.frequency.value = freq * (0.85 + Math.random() * 0.3);
    const g = c.createGain(), t = c.currentTime; g.gain.setValueAtTime(vol, t); g.gain.exponentialRampToValueAtTime(0.0008, t + dur);
    s.connect(f); f.connect(g); g.connect(this.master); s.start(t, Math.random() * 2, dur + 0.02);
  }
  step(cat) { const p = { rock: [1500, 0.09, 0.12], wood: [800, 0.1, 0.13], terrain: [520, 0.11, 0.1], fluid: [1800, 0.16, 0.08] }[cat] || [620, 0.1, 0.09]; this.burst(p[0], p[1], p[2]); }
  dig(cat) { this.burst(cat === 'rock' ? 2200 : cat === 'wood' ? 1000 : 700, 0.08, 0.16, 'bandpass'); }
  breakBlock(cat) { this.burst(cat === 'rock' ? 1700 : 850, 0.18, 0.24); }
  place() { this.burst(620, 0.09, 0.2); }
}
