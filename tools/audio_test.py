#!/usr/bin/env python3
"""Rend chaque son hors temps réel dans Chrome et mesure le signal. Cela vérifie niveaux et différences, pas la beauté."""
import json
from playwright.sync_api import sync_playwright
JS = r'''async()=>{
  const S=ALW.sound; S.start(); const Sc=S.constructor, Mu=S.music.constructor;
  const off=(d)=>new OfflineAudioContext(2,44100*d,44100);
  const stat=(b)=>{const d=b.getChannelData(0),e=b.getChannelData(1);let s=0,p=0,z=0,l=0,lp=0,saut=0;for(let i=0;i<d.length;i++){const v=d[i];if(i)saut=Math.max(saut,Math.abs(v-d[i-1]));s+=v*v;p=Math.max(p,Math.abs(v),Math.abs(e[i]));if(i&&(v>=0)!==(d[i-1]>=0))z++;lp+=(v-lp)*.028;l+=lp*lp;}
    return {rms:+Math.sqrt(s/d.length).toFixed(4),crete:+p.toFixed(3),hz:Math.round(z/2/(d.length/44100)),grave:+(l/Math.max(s,1e-9)).toFixed(2),saut:+saut.toFixed(3)}};
  const one=async(d,f)=>{const c=off(d),s=new Sc();s.start(c);await s.ready;s.listener([0,0,0],0);f(s);return stat(await c.startRendering())};
  const R={blocs:{},outils:{},faune:{},eau:{},temps:{},musique:{}};
  for(const n of Object.keys(ALW.soundNames()))R.blocs[n]=await one(.5,s=>s.breakBlock(n,null));
  for(const t of ['pickaxe','axe','sword',null])for(const b of ['stone','dirt','grass','oak_log','sand'])R.outils[(t||'main')+' sur '+b]=await one(.5,s=>s.dig(b,t));
  for(const t of ['pickaxe','axe','sword',null])R.outils['vide '+(t||'main')]=await one(.5,s=>s.swing(t));
  for(const n of ['grass','dirt','stone','sand','gravel','planks','oak_log'])R.outils['pas '+n]=await one(.5,s=>s.step(n));
  for(const b of ['stone','granite','oak_log','birch_log','planks'])R.outils['pose '+b]=await one(.5,s=>s.place(b));
  R.faune.oiseau_pres=await one(1,s=>s.bird([3,0,0],0));R.faune.oiseau_loin=await one(1,s=>s.bird([30,0,0],0));R.faune.alarme_cerf=await one(.8,s=>s.deerAlarm([6,0,0]));R.faune.appel_cerf=await one(.6,s=>s.deerCall([6,0,0]));R.faune.sabot=await one(.3,s=>s.hoof([6,0,0],true));
  const env=(a)=>async()=>one(6,s=>s.env({cloud:.42,rain:0,...a}));
  R.temps.clair=await env({})();R.temps.nuageux=await env({cloud:.8})();R.temps.pluie=await env({cloud:1,rain:1})();
  R.eau.ruisseau_4m=await env({brook:[4,0,0]})();R.eau.chute_8m=await env({fall:[8,0,0]})();R.eau.chute_25m=await env({fall:[25,0,0]})();
  for(const m of ['calme','nuit','construction','suspense','danger']){const c=off(22),g=c.createGain();g.connect(c.destination);const M=new Mu(c,g);M.setVolume(.5);M.begin(m);M.next=.2;
    while(M.next<21)M.schedule();R.musique[m]=stat(await c.startRendering())}
  return R}'''
with sync_playwright() as p:
    b=p.chromium.launch(args=['--use-angle=swiftshader','--enable-unsafe-swiftshader','--ignore-gpu-blocklist','--enable-webgl','--autoplay-policy=no-user-gesture-required'])
    g=b.new_page(viewport={'width':480,'height':270}); e=[]; g.on('pageerror',lambda x:e.append(str(x)))
    g.goto('file:///home/claude/a-living-world/dist/index.html#capture&q=bas&r=2'); g.wait_for_function('window.ALW&&ALW.loaded',timeout=90000)
    r=g.evaluate(JS); json.dump(r,open('captures/audio.json','w'),indent=0,ensure_ascii=False)
    for k,v in r.items():
        print(k)
        for n,s in v.items(): print('  %-16s'%n,s)
    print('erreurs',e); b.close()
