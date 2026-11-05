#!/usr/bin/env python3
"""Vérifie les boutons tactiles avec de vrais événements tactiles, dans une fenêtre de la taille d'un iPad."""
import sys
from playwright.sync_api import sync_playwright
ok=True
def check(n,c,i=''):
    global ok; ok=ok and bool(c); print(('OK  ' if c else 'ÉCHEC ')+n,i)
with sync_playwright() as p:
    b=p.chromium.launch(args=['--use-angle=swiftshader','--enable-unsafe-swiftshader','--ignore-gpu-blocklist'])
    c=b.new_context(viewport={'width':1024,'height':768},has_touch=True); pg=c.new_page()
    errs=[]; pg.on('pageerror',lambda e:errs.append(str(e)))
    pg.goto('file:///home/claude/a-living-world/dist/index.html#q=bas&r=2&seed=20261104')
    pg.wait_for_function('window.ALW&&ALW.loaded',timeout=150000)
    pg.evaluate('ALW.r.resize(320,240,1)')
    pg.touchscreen.tap(700,300); pg.wait_for_function("document.getElementById('touch').classList.contains('on')",timeout=30000)
    check('boutons tactiles affichés',True)
    pg.tap('#tFps',timeout=60000,force=True)
    pg.wait_for_function("document.getElementById('prof').textContent.includes('FPS')",timeout=30000)
    check('FPS affiché',pg.evaluate("document.getElementById('prof').classList.contains('on')"),pg.evaluate("document.getElementById('prof').textContent.split('\\n')[0]"))
    pg.tap('#tFps',timeout=60000,force=True)
    pg.wait_for_function("!document.getElementById('prof').classList.contains('on')",timeout=30000); check('FPS masqué',True)
    pg.tap('#tMenu',timeout=60000,force=True)
    pg.wait_for_function('ALW.paused===true',timeout=30000); check('menu ouvert',True)
    pg.tap('#btnResume',timeout=60000)
    pg.wait_for_function('!ALW.paused',timeout=30000); check('reprise',True)
    check('aucune exception JS',not errs,str(errs[:2])); b.close()
sys.exit(0 if ok else 1)
