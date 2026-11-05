#!/usr/bin/env python3
"""Rassemble les sons en une banque PCM : le moteur n'a ainsi besoin d'aucun décodeur.
Sets : nom, fréquence, canaux, boucle ; échantillons 16 bits. Les boucles sont découpées exactement sur leur période."""
import subprocess, glob, os, struct, re
import numpy as np
SR = 32000; D = '../assets/sounds'; LOOPS = {'wind': 12, 'rain': 12, 'brook': 10.962, 'fall': 16.567}; PAD = 0.25
def dec(f, ch):
    return np.frombuffer(subprocess.run(['ffmpeg', '-v', 'error', '-i', f, '-ac', str(ch), '-ar', str(SR), '-f', 's16le', '-'], capture_output=True, check=True).stdout, dtype=np.int16)
out = [b'ALWB']; n = 0
for f in sorted(glob.glob(D + '/*.mp3')):
    name = os.path.basename(f)[:-4]; loop = name.startswith('loop_')
    ch = 2 if name in ('loop_wind', 'loop_rain') else 1; x = dec(f, ch)
    if loop:
        k = name[5:]; a = int(PAD * SR) * ch; b = a + int(LOOPS[k] * SR) * ch; x = x[a:b]
    else:
        on = np.nonzero(np.abs(x) > 320)[0]; x = x[max(on[0] - 16, 0):]
    b = name.encode(); out.append(struct.pack('<B', len(b)) + b + struct.pack('<BBI', ch, loop, len(x)) + x.tobytes()); n += 1
open('game/assets/bank.bin', 'wb').write(b''.join(out)); print(n, 'sons,', os.path.getsize('game/assets/bank.bin') // 1024, 'Ko')
