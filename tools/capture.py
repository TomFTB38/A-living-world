#!/usr/bin/env python3
"""Lance le vrai jeu dans Chrome (rendu logiciel ici), place la caméra, capture et mesure."""
import sys, json, time, base64
from playwright.sync_api import sync_playwright
def run(out, w=1280, h=720, q='moyen', seed=20261104, radius=6, js=None, frames=6, hud=True, hour=None, timeout=100):
    with sync_playwright() as p:
        b = p.chromium.launch(args=['--use-angle=swiftshader','--enable-unsafe-swiftshader','--ignore-gpu-blocklist','--enable-webgl'])
        pg = b.new_page(viewport={'width':w,'height':h})
        logs=[]
        pg.on('console', lambda m: logs.append(m.text)); pg.on('pageerror', lambda e: logs.append('PAGEERROR '+str(e)))
        pg.goto(f'file:///home/claude/a-living-world/dist/index.html#capture&q={q}&seed={seed}&r={radius}')
        t0=time.time()
        try:
            pg.wait_for_timeout(1500); print(logs[:6]); pg.wait_for_function('window.ALW && ALW.loaded', timeout=timeout*1000)
            pg.wait_for_function('ALW.world.queued===0 && ALW.world.pending===0', timeout=timeout*1000)
        except Exception as e: logs.append('TIMEOUT '+str(e)[:100])
        print('chargé en %.1fs'%(time.time()-t0))
        if logs: print('\n'.join(logs[:12]))
        if js: pg.evaluate(js); time.sleep(.2)
        if hour is not None: pg.evaluate(f'ALW.hour={hour}')
        if not hud: pg.add_style_tag(content='#brand,#compassWrap,#biome,#vitals,#bottom,#hints,#reticle,#toast{display:none!important}')
        pg.evaluate(f'ALW.stopAt=ALW.r.frame+{frames}')
        pg.wait_for_function('ALW.stopped', timeout=timeout*1000, polling=500)
        pg.screenshot(path=out, timeout=120000)
        st=pg.evaluate('({s:ALW.r.stats,W:ALW.r.W,H:ALW.r.H,p:ALW.player.p,chunks:ALW.world.chunks.size,gen:ALW.world.stats.genMs,workers:ALW.world.workers.length,local:!!ALW.world.local,err:ALW.r.gl.getError()})')
        print(json.dumps(st)); b.close()
if __name__=='__main__':
    import argparse
    A=argparse.ArgumentParser(); A.add_argument('out'); A.add_argument('--js'); A.add_argument('-w',type=int,default=960); A.add_argument('-H',type=int,default=540); A.add_argument('-q',default='moyen'); A.add_argument('-r',type=int,default=6); A.add_argument('-n',type=int,default=6); A.add_argument('--nohud',action='store_true'); A.add_argument('--seed',type=int,default=20261104); A.add_argument('--hour',type=float)
    a=A.parse_args(); t=time.time(); run(a.out,a.w,a.H,a.q,a.seed,a.r,a.js,a.n,not a.nohud,a.hour); print('total %.0fs'%(time.time()-t))
