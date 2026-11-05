#!/usr/bin/env python3
"""Planche de vérification : vraies captures du jeu pour chaque pose et chaque vue."""
import sys, time
from PIL import Image
from playwright.sync_api import sync_playwright
W,H=560,340
side="(()=>{const p=ALW.player.p,y=ALW.player.yaw;const r=[-Math.cos(y),Math.sin(y)],f=[Math.sin(y),Math.cos(y)];const e=[p[0]+r[0]*3.1+f[0]*.9,p[1]+1.5,p[2]+r[1]*3.1+f[1]*.9],t=[p[0],p[1]+1,p[2]],d=[t[0]-e[0],t[1]-e[1],t[2]-e[2]],L=Math.hypot(d[0],d[1],d[2]);ALW.camOverride={eye:e,f:d.map(v=>v/L)};ALW.view.third=true;})()"
reset="ALW.camOverride=null;ALW.freeze=true;Object.assign(ALW.anim,{walk:0,run:0,low:0,air:0,rise:0,swing:0,punch:0,phase:0,landed:0});ALW.anim.update=()=>{};ALW.fauna.deer=[];ALW.fauna.birds=[];ALW.fauna.update=()=>{};ALW.inv[0]={item:'pickaxe'};ALW.inv[1]={item:'axe'};ALW.inv[2]={item:'sword'};ALW.inv[3]={item:'oak_log',n:9};ALW.inv[4]={item:'lantern',n:9};ALW.inv[5]={item:'stone',n:9};ALW.inv[6]={item:'fern',n:9};"
CASES=[('pioche, côté',side+";ALW.select(0)"),('hache, dos',"ALW.view.third=true;ALW.select(1)"),('épée, côté',side+";ALW.select(2)"),
('bûche, côté',side+";ALW.select(3)"),('lanterne, dos',"ALW.view.third=true;ALW.select(4)"),('bûche, 1re',"ALW.view.third=false;ALW.select(5)"),('marche',side+";ALW.select(0);Object.assign(ALW.anim,{walk:1,phase:1.3})"),
('sprint',side+";ALW.select(0);Object.assign(ALW.anim,{walk:1,run:1,phase:1.4})"),('accroupi',side+";ALW.select(0);ALW.anim.low=1"),('saut',side+";ALW.select(0);Object.assign(ALW.anim,{air:1,rise:1})"),
('minage',side+";ALW.select(0);ALW.anim.swing=.5"),('1re pers. pioche',"ALW.view.third=false;ALW.select(0);ALW.player.pitch=-.1"),('1re pers. bûche',"ALW.view.third=false;ALW.select(3);ALW.player.pitch=-.1"),
('1re pers. épée',"ALW.view.third=false;ALW.select(2)"),('1re pers. minage',"ALW.view.third=false;ALW.select(1);ALW.anim.swing=.35")]
if len(sys.argv)>2: CASES=[c for c in CASES if '1re' in c[0]]
with sync_playwright() as p:
    b=p.chromium.launch(args=['--use-angle=swiftshader','--enable-unsafe-swiftshader','--ignore-gpu-blocklist'])
    pg=b.new_page(viewport={'width':W,'height':H}); errs=[]; pg.on('pageerror',lambda e:errs.append(str(e)[:200]))
    pg.goto('file:///home/claude/a-living-world/dist/index.html#capture&q=moyen&r=2&seed=20261104')
    pg.wait_for_function('window.ALW&&ALW.loaded',timeout=120000); pg.wait_for_function('ALW.world.queued===0&&ALW.world.pending===0&&ALW.player.ground',timeout=120000)
    pg.add_style_tag(content='#brand,#compassWrap,#biome,#vitals,#bottom,#hints,#toast{display:none!important}')
    pg.evaluate(reset); ims=[]
    for n,js in CASES:
        pg.evaluate("ALW.camOverride=null;Object.assign(ALW.anim,{walk:0,run:0,low:0,air:0,rise:0,swing:0,punch:0})")
        pg.evaluate(js); pg.evaluate("ALW.stopped=false;ALW.stopAt=ALW.r.frame+3"); pg.wait_for_function('ALW.stopped',timeout=60000,polling=200)
        pg.screenshot(path='/tmp/claude-0/p.png',timeout=60000); ims.append(Image.open('/tmp/claude-0/p.png').convert('RGB').copy()); pg.evaluate("ALW.stopAt=0;ALW.stopped=false;requestAnimationFrame(t=>ALW.loop(t))")
    print('erreurs',errs); b.close()
cols=4; rows=(len(ims)+cols-1)//cols; sheet=Image.new('RGB',(W*cols//2*1,H*rows//2),(0,0,0))
for i,m in enumerate(ims): sheet.paste(m.resize((W//2,H//2)),((i%cols)*W//2,(i//cols)*H//2))
sheet=Image.new('RGB',(W*cols,H*rows)); 
for i,m in enumerate(ims): sheet.paste(m,((i%cols)*W,(i//cols)*H))
sheet.resize((W*cols*3//5,H*rows*3//5)).save(sys.argv[1] if len(sys.argv)>1 else 'captures/poses.png')
