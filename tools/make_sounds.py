#!/usr/bin/env python3
"""Prépare les sons du jeu : prises réelles libres (voir assets/sounds/CREDITS.md) et boucles générées.
Usage : make_sounds.py <dossier kenney-impact-sounds> <dossier minetest_game>
Chaque clip est calé, équilibré et adouci aux extrémités, pour éviter tout claquement."""
import sys, subprocess, glob, os
import numpy as np
from scipy.signal import lfilter
KEN, MTG = sys.argv[1], sys.argv[2]; OUT = os.path.join(os.path.dirname(__file__), '..', 'assets', 'sounds'); SR = 44100
np.random.seed(20261105)
def read(path):
    return np.frombuffer(subprocess.run(['ffmpeg', '-v', 'error', '-i', path, '-ac', '1', '-ar', str(SR), '-f', 'f32le', '-'], capture_output=True, check=True).stdout, dtype=np.float32).astype(np.float64)
def write(name, x, br='96k'):
    x = np.clip(x, -1, 1); ch = 2 if x.ndim == 2 else 1
    subprocess.run(['ffmpeg', '-v', 'error', '-y', '-f', 'f32le', '-ar', str(SR), '-ac', str(ch), '-i', '-', '-c:a', 'libmp3lame', '-b:a', br, os.path.join(OUT, name + '.mp3')], input=x.astype(np.float32).tobytes(), check=True)
def clean(x, peak=0.7, maxd=0.85, keep=0.02):
    th = np.abs(x).max() * keep; on = np.nonzero(np.abs(x) > th)[0]; a, b = max(on[0] - 40, 0), on[-1] + 1
    x = x[a:min(b, a + int(maxd * SR))].copy(); n = len(x); f = min(int(.012 * SR), n // 3)
    x[:32] *= np.linspace(0, 1, 32); x[-f:] *= np.cos(np.linspace(0, np.pi / 2, f)) ** 2
    return x / np.abs(x).max() * peak
ken = lambda s, i: sorted(glob.glob(f'{KEN}/**/{s}_00{i}.ogg', recursive=True))[0]
mt = lambda n: f'{MTG}/mods/default/sounds/{n}.ogg'
SETS = {
 'wood': [ken('footstep_wood', i) for i in range(4)], 
 'sand': [mt(f'default_sand_footstep.{i}') for i in (1, 2, 3)], 'gravel': [mt(f'default_gravel_dig.{i}') for i in (1, 2)] + [mt('default_gravel_dug.1')],
 'crumble': [mt(f'default_gravel_dug.{i}') for i in (1, 2, 3)], 'crack': [mt(f'default_dig_cracky.{i}') for i in (1, 2, 3)],
 'soft': [ken('impact_soft_medium', i) for i in range(3)], 'woodl': [ken('impact_wood_light', i) for i in range(3)],
  'plank': [ken('impact_plank_medium', i) for i in range(3)], 
 'leaf': [mt('default_dig_snappy')], 'metal': [ken('impact_metal_light', i) for i in range(3)], 'glass': [ken('impact_glass_light', i) for i in range(2)],
  'splash': [mt(f'default_water_footstep.{i}') for i in (1, 2)], 'hurt': [mt('player_damage')],
}
def band(x, lo, hi=None, boost=None):
    """Filtre en fréquence, sans déphasage : retire le grave qui fait tambour. boost = (f0, largeur, gain)."""
    X = np.fft.rfft(x); f = np.fft.rfftfreq(len(x), 1 / SR); g = (1 - np.exp(-(f / lo) ** 2.5))
    if hi: g *= np.exp(-(f / hi) ** 4)
    if boost: g *= 1 + boost[2] * np.exp(-((f - boost[0]) / boost[1]) ** 2)
    return np.fft.irfft(X * g, len(x))
for n, files in SETS.items():
    for i, f in enumerate(files): write(f'{n}_{i}', clean(read(f), keep=0.004 if n == 'crack' else 0.02))
# Matières retravaillées : on garde l'attaque sèche, le corps grave est retiré.
for i in range(4): write(f'rockhi_{i}', clean(band(read(ken('impact_mining', i)), 420), .7, .4))
for i in range(3):
    x = band(read(mt(f'default_dig_choppy.{i + 1}')), 190, 5000, (1100, 650, 2.2)); write(f'chopdry_{i}', clean(x, .7))
    y = band(read(ken('impact_wood_light', i)), 230, 4000, (900, 500, 2.5)); write(f'knock_{i}', clean(y, .7))
for i in range(4): write(f'stepstone_{i}', clean(band(read(ken('footstep_concrete', i)), 240, 7000, (1800, 900, 3)), .7))
for i in range(4): write(f'stepgrass_{i}', clean(band(read(ken('footstep_grass', i)), 260), .7, .35))

def sweep(dur, f0, f1, f2, q, seed):
    """Souffle filtré dont la fréquence suit le geste : monte puis retombe."""
    n = int(dur * SR); rng = np.random.default_rng(seed); w = rng.standard_normal(n); t = np.linspace(0, 1, n); e = np.sin(np.pi * t ** 0.8) ** 2
    f = np.where(t < .45, f0 + (f1 - f0) * (t / .45) ** 1.4, f1 + (f2 - f1) * ((t - .45) / .55)); y = np.zeros(n); a = b = 0.0
    for i in range(n):
        g = 2 * np.sin(np.pi * f[i] / SR); h = w[i] - a - b / q; b += g * h; a += g * b; y[i] = b  # filtre d'état
    return y * e / np.abs(y * e).max()
for i in range(3):  # sifflement de lame : souffle étroit, plus la note fine de l'acier qui fend l'air
    n = int(.34 * SR); t = np.arange(n) / SR; u = t / t[-1]; f = 2100 + 1500 * np.sin(np.pi * u) + 90 * i; ph = 2 * np.pi * np.cumsum(f) / SR
    x = sweep(.34, 900 + 120 * i, 3200 + 200 * i, 1300, 3.2, 10 + i) + .22 * np.sin(ph) * np.sin(np.pi * u) ** 4
    write(f'blade_{i}', clean(x, .6))
for i in range(2):
    write(f'whoosh_{i}', clean(sweep(.38, 240, 820 + 90 * i, 300, 1.4, 20 + i), .55)); write(f'puff_{i}', clean(sweep(.2, 300, 700 + 80 * i, 320, 1.1, 30 + i), .45))
for i in range(2):  # souffle de cerf : deux formants, attaque brève
    x = sweep(.34, 950, 620 + 60 * i, 380, 2.2, 40 + i) + .6 * sweep(.34, 2100, 1500, 900, 2.5, 50 + i); e = np.exp(-np.linspace(0, 5, len(x))); write(f'snort_{i}', clean(x * e, .6))

# Boucles : tout est circulaire, et le fichier porte une marge de PAD de chaque côté pour une reprise sans couture.
PAD = int(.25 * SR)
def wrap(x): return np.concatenate([x[-PAD:], x, x[:PAD]])
def shaped(n, fn, rng):
    X = np.fft.rfft(rng.standard_normal(n)); f = np.fft.rfftfreq(n, 1 / SR); y = np.fft.irfft(X * fn(np.maximum(f, 1)), n); return y / np.abs(y).max()
rng = np.random.default_rng(7); n = 12 * SR
def slowmod(n, rng, parts):
    t = np.arange(n) / n; return sum(a * np.sin(2 * np.pi * (k * t + rng.random())) for k, a in parts)
# Vent : grave et mouvant, sans aucun aigu.
w = np.stack([shaped(n, lambda f: 1 / f * np.exp(-(f / 520) ** 2) * (1 - np.exp(-(f / 55) ** 2)), rng) for _ in range(2)], 1); m = .6 + slowmod(n, rng, [(1, .25), (3, .14), (5, .08)]); w = (w.T * m).T
write('loop_wind', wrap(w) / np.abs(w).max() * .7, '112k')
# Pluie : des milliers de gouttes distinctes sur un fond doux, pas un souffle continu.
def rain(rng):
    y = shaped(n, lambda f: (f / 1800) ** .3 * np.exp(-(f / 6500) ** 2) * (1 - np.exp(-(f / 420) ** 2)), rng) * .3; d = np.zeros(n); k = rng.integers(0, n, 26000); np.add.at(d, k, rng.lognormal(-1.2, .9, len(k)) * rng.choice([-1, 1], len(k)))
    D = np.fft.rfft(d); f = np.fft.rfftfreq(n, 1 / SR); drops = np.fft.irfft(D * (np.exp(-((f - 3100) / 2300) ** 2) + .5 * np.exp(-((f - 900) / 500) ** 2)), n); return y + drops / np.abs(drops).max() * .75
r = np.stack([rain(rng), rain(rng)], 1); write('loop_rain', wrap(r) / np.abs(r).max() * .7, '112k')
def circular(x, fade):
    x = x.copy(); f = np.sin(np.linspace(0, np.pi / 2, fade)) ** 2; x[:fade] = x[:fade] * f + x[-fade:] * f[::-1]; return x[:-fade]
# Ruisseau : vraie prise de son, bouclée en fondu.
wt = [read(f'{MTG}/mods/env_sounds/sounds/env_sounds_water.{i}.ogg') for i in (1, 2, 3, 4)]
b = circular(np.concatenate(wt[:2]), SR); write('loop_brook', wrap(b) / np.abs(b).max() * .7, '80k')
# Chute : la même eau ralentie, épaissie d'un grondement doux.
c = np.concatenate(wt[2:]); s = np.interp(np.arange(0, len(c) - 1, .68), np.arange(len(c)), c); s = circular(s, SR); nf = len(s)
rum = shaped(nf, lambda f: np.minimum(1, f / 160) / np.sqrt(np.maximum(f, 160) / 160) * np.exp(-(f / 3800) ** 2), rng); fall = s / np.abs(s).max() + .7 * rum
write('loop_fall', wrap(fall) / np.abs(fall).max() * .7, '80k')
print(f'PAD {PAD/SR} brook {len(b)/SR:.3f} fall {nf/SR:.3f} autres 12')
