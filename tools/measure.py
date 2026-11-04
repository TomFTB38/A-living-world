#!/usr/bin/env python3
"""Mesures de charge réelles du jeu. Les ms de rendu viennent ici du rendu logiciel : elles ne valent pas pour une carte graphique."""
import json, time
from playwright.sync_api import sync_playwright
out={}
with sync_playwright() as p:
    b=p.chromium.launch(args=['--use-angle=swiftshader','--enable-unsafe-swiftshader','--ignore-gpu-blocklist'])
    for q,r in [('bas',6),('moyen',8),('haut',10)]:
        pg=b.new_page(viewport={'width':640,'height':360})
        t0=time.time(); pg.goto(f'file:///home/claude/a-living-world/dist/index.html#capture&q={q}&r={r}&seed=20261104')
        pg.wait_for_function('window.ALW&&ALW.loaded',timeout=200000); t1=time.time()
        pg.wait_for_function('ALW.world.queued===0&&ALW.world.pending===0',timeout=280000,polling=500); t2=time.time()
        pg.evaluate('window.__m=[];const o=ALW.r.render.bind(ALW.r);ALW.r.render=(v)=>{const t=performance.now();o(v);__m.push({tris:ALW.r.stats.tris,draws:ALW.r.stats.draws,cpu:performance.now()-t,gpu:ALW.r.stats.gpu})};0')
        pg.wait_for_function('__m.length>=6',timeout=200000,polling=500)
        m=pg.evaluate('({m:__m,chunks:ALW.world.chunks.size,gen:ALW.world.stats.genMs,vram:[...ALW.world.chunks.values()].reduce((s,c)=>s+[c.opaque,c.deco,c.water].reduce((a,x)=>a+(x?x.bytes:0),0),0),heap:performance.memory?performance.memory.usedJSHeapSize:0})')
        ms=m['m']; out[q]=dict(radius=r,chunks=m['chunks'],tris_max=max(x['tris'] for x in ms),draws_max=max(x['draws'] for x in ms),geo_mo=round(m['vram']/1048576),heap_mo=round(m['heap']/1048576),premier_affichage_s=round(t1-t0,1),chargement_s=round(t2-t0,1),gen_mesh_ms=round(m['gen'],1),gpu_logiciel_ms=round(ms[-1]['gpu']))
        print(q,json.dumps(out[q])); pg.close()
    b.close()
json.dump(out,open('captures/mesures.json','w'),indent=1)
