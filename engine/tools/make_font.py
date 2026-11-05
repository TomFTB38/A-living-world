#!/usr/bin/env python3
"""Fabrique l'atlas de texte de l'interface depuis Inter (licence OFL), une seule fois : le jeu n'embarque que l'image et les largeurs."""
from PIL import Image, ImageDraw, ImageFont
CH = ''.join(chr(c) for c in range(32, 127)) + 'àâäçéèêëîïôöùûüœÀÂÇÉÈÊËÎÏÔÖÙÛÜŒ«»’·'
F = ImageFont.truetype('/usr/share/fonts/opentype/inter/Inter-SemiBold.otf', 30); S = 40; COLS = 32; rows = len(CH) // COLS + 1
a = Image.new('L', (COLS * S, rows * S)); d = ImageDraw.Draw(a); w = []
for i, c in enumerate(CH):
    x, y = i % COLS * S, i // COLS * S; d.text((x + 2, y + 2), c, fill=255, font=F); w.append(round(F.getlength(c)))
a.save('/tmp/claude-0/s/font.png'); open('engine/game/assets/font.r8', 'wb').write(a.tobytes())
open('engine/game/src/font_data.rs', 'w').write(f'// Généré par engine/tools/make_font.py depuis Inter (SIL Open Font License).\npub const COLS: u32 = {COLS}; pub const CELL: u32 = {S}; pub const W: u32 = {a.width}; pub const H: u32 = {a.height};\npub const CHARS: &str = "{CH.replace(chr(92), chr(92)*2).replace(chr(34), chr(92)+chr(34))}";\npub const ADV: [u8; {len(w)}] = {w};\n')
print(a.size, len(CH))
