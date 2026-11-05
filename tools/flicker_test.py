#!/usr/bin/env python3
"""Mesure le clignotement du personnage. Il avance à vitesse réelle et fixe (7,5 cm par image, 60 FPS) :
à l'écran il reste immobile, donc ses pixels devraient rester stables d'une image à l'autre.
Un défaut une image sur deux donne un écart à 1 image bien plus grand qu'à 2 images."""
import sys
from playwright.sync_api import sync_playwright
Q=sys.argv[1] if len(sys.argv)>1 else 'moyen'
JS="""()=>{window.__P=[];const r=ALW.r,g=r.gl,o=r.render.bind(r);const pl=ALW.player;ALW.freeze=true;ALW.view.camDist=ALW.view.dist;
const sy=Math.sin(pl.yaw),cy=Math.cos(pl.yaw);
r.render=(v)=>{pl.p[0]+=sy*.075;pl.p[2]+=cy*.075;o(v);const w=r.W,h=r.H,x0=Math.floor(w*.375),x1=Math.floor(w*.435),y0=Math.floor(h*.2),y1=Math.floor(h*.46),W=x1-x0,H=y1-y0,b=new Uint8Array(W*H*4);
g.bindFramebuffer(g.FRAMEBUFFER,null);g.readPixels(x0,y0,W,H,g.RGBA,g.UNSIGNED_BYTE,b);const a=new Float32Array(W*H);for(let i=0;i<W*H;i++)a[i]=b[i*4]*.3+b[i*4+1]*.6+b[i*4+2]*.1;__P.push(Array.from(a))};return 0}"""
with sync_playwright() as p:
    b=p.chromium.launch(args=['--use-angle=swiftshader','--enable-unsafe-swiftshader','--ignore-gpu-blocklist'])
    pg=b.new_page(viewport={'width':560,'height':316})
    pg.goto(f'file:///home/claude/a-living-world/dist/index.html#capture&q={Q}&r=3&seed=20261104')
    pg.wait_for_function('window.ALW&&ALW.loaded',timeout=120000); pg.wait_for_function('ALW.world.queued===0&&ALW.world.pending===0',timeout=120000)
    pg.evaluate('ALW.fauna.update=()=>{};ALW.fauna.deer=[];ALW.fauna.birds=[]')
    pg.wait_for_function('ALW.player.ground',timeout=60000)
    pg.evaluate(JS); pg.wait_for_function('__P.length>=24',timeout=250000,polling=500)
    P=pg.evaluate('__P.slice(8,24)'); b.close()
d=lambda a,b:sum(abs(x-y) for x,y in zip(a,b))/len(a)
d1=sum(d(P[i],P[i-1]) for i in range(2,len(P)))/(len(P)-2); d2=sum(d(P[i],P[i-2]) for i in range(2,len(P)))/(len(P)-2)
print(Q,'écart à 1 image %.2f, à 2 images %.2f, rapport %.2f'%(d1,d2,d1/max(d2,1e-6)))
