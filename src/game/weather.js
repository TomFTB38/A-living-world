// Weather/weather.js — météo déterministe par tranches de temps, transitions lentes et progressives.
import { hashFloat } from '../core/noise.js';

export const SLOT = 210; // secondes de jeu par épisode
export function kindAt(seed, t) {
  if (t < SLOT) return 'clair'; // on découvre le monde sous le soleil
  const r = hashFloat(seed, Math.floor(t / SLOT), 17, 3);
  return r < 0.5 ? 'clair' : r < 0.74 ? 'nuageux' : r < 0.88 ? 'pluie' : 'brouillard';
}
const TARGET = { clair: { cloud: 0.42, rain: 0, fog: 0 }, nuageux: { cloud: 0.8, rain: 0, fog: 0.1 }, pluie: { cloud: 1, rain: 1, fog: 0.35 }, brouillard: { cloud: 0.7, rain: 0, fog: 1 } };

export class Weather {
  constructor(seed) { this.seed = seed; this.t = 0; this.forced = 'auto'; this.state = { ...TARGET.clair }; this.kind = 'clair'; }
  update(dt) {
    this.t += dt;
    this.kind = this.forced === 'auto' ? kindAt(this.seed, this.t) : this.forced;
    const g = TARGET[this.kind], k = Math.min(1, dt / 14); // environ 40 s pour changer de temps
    for (const n in g) this.state[n] += (g[n] - this.state[n]) * k;
  }
}
