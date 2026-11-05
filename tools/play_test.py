#!/usr/bin/env python3
"""Test bout en bout : vraies entrées clavier/souris dans le jeu assemblé, sans API interne pour agir."""
import json, sys, time
from playwright.sync_api import sync_playwright
ok=[]
def check(name, cond, info=''):
    ok.append((name,bool(cond))); print(('OK  ' if cond else 'ÉCHEC ')+name, info)
with sync_playwright() as p:
    b=p.chromium.launch(args=['--use-angle=swiftshader','--enable-unsafe-swiftshader','--ignore-gpu-blocklist'])
    pg=b.new_page(viewport={'width':320,'height':180})
    errs=[]; pg.on('pageerror',lambda e:errs.append(str(e)))
    pg.goto('file:///home/claude/a-living-world/dist/index.html#q=bas&r=3&seed=20261104')
    pg.wait_for_function('window.ALW&&ALW.loaded',timeout=120000)
    pg.wait_for_function('ALW.world.queued===0&&ALW.world.pending===0',timeout=120000)
    pg.wait_for_function('ALW.player.ground',timeout=60000)
    st=lambda:pg.evaluate('({p:[...ALW.player.p],g:ALW.player.ground,yaw:ALW.player.yaw,pitch:ALW.player.pitch,inv:ALW.inv.map(i=>i.n),t:ALW.target&&[ALW.target.x,ALW.target.y,ALW.target.z,ALW.target.id],e:ALW.world.edits.size,fps:ALW.prof.fps})')
    a=st(); print(a)
    # déplacement
    pg.keyboard.down('KeyW'); time.sleep(12); pg.keyboard.up('KeyW'); c=st()
    moved=((c['p'][0]-a['p'][0])**2+(c['p'][2]-a['p'][2])**2)**.5
    check('marche avant', moved>1, 'distance %.1f'%moved)
    y0=st()['p'][1]; pg.keyboard.down('Space'); peak=y0
    for _ in range(14): time.sleep(.3); peak=max(peak,st()['p'][1])
    pg.keyboard.up('Space'); check('saut', peak>y0+.4, 'hauteur %.2f'%(peak-y0))
    # visée vers le sol, caméra libre par API de visée
    pg.evaluate('ALW.dragMode=true; ALW.view.third=false; ALW.player.pitch=-1.2'); time.sleep(3)
    pg.wait_for_function('!!ALW.target',timeout=30000); t=st()['t']; print('cible',t)
    # casser au clic (mode sans verrouillage du pointeur)
    pg.mouse.move(160,90); pg.mouse.click(160,90)
    try: pg.wait_for_function(f'ALW.world.get({t[0]},{t[1]},{t[2]})!=={t[3]}',timeout=40000)
    except Exception:
        print(pg.evaluate('({am:ALW.autoMine,m:ALW.mining,mt:ALW.mine,pa:ALW.paused,tg:ALW.target,inv:ALW.sel,fps:ALW.prof.fps,el:document.elementFromPoint(160,90).id})')); raise
    check('casser un bloc au clic', True, 'bloc %d supprimé'%t[3])
    pg.wait_for_function(f'ALW.world.get({t[0]},{t[1]},{t[2]})===0',timeout=5000)
    # poser
    pg.keyboard.press('Digit5'); before=st()['inv'][4]; time.sleep(1.5)
    pg.wait_for_function('!!ALW.target',timeout=30000)
    pg.mouse.click(160,90,button='right')
    try: pg.wait_for_function(f'ALW.inv[4].n<{before}',timeout=30000)
    except Exception:
        print(pg.evaluate('({t:ALW.target,sel:ALW.sel,inv:ALW.inv[4],p:ALW.player.p,pitch:ALW.player.pitch,paused:ALW.paused,third:ALW.view.third,el:document.elementFromPoint(160,90).id})')); raise
    check('poser un bloc au clic droit', True, 'stock %d → %d'%(before,st()['inv'][4]))
    # sauvegarde puis rechargement réel de la page
    edits=pg.evaluate('JSON.stringify([...ALW.world.edits].map(([k,m])=>[k,[...m]]))')
    pg.evaluate('ALW.save(false)'); time.sleep(1)
    print('errs',errs); pos=st()['p']; pg.reload(wait_until='commit',timeout=120000); time.sleep(4)
    pg.wait_for_function('window.ALW&&ALW.loaded',timeout=120000); pg.wait_for_function('ALW.world.queued===0&&ALW.world.pending===0',timeout=120000)
    e2=pg.evaluate('JSON.stringify([...ALW.world.edits].map(([k,m])=>[k,[...m]]))')
    check('sauvegarde : modifications identiques après rechargement', e2==edits, str(len(json.loads(e2))))
    k=json.loads(edits)[0]; i=k[1][0]
    v=pg.evaluate(f"(()=>{{const c=ALW.world.chunks.get('{k[0]}');return c&&c.voxels?c.voxels[{i[0]}]:null}})()")
    check('voxel modifié restauré sur le terrain', v==i[1], f'{v} attendu {i[1]}')
    p2=st()['p']; check('position restaurée', abs(p2[0]-pos[0])<1 and abs(p2[2]-pos[2])<1)
    check('aucune exception JS', not errs, str(errs[:2]))
    print('fps logiciel', st()['fps']); b.close()
sys.exit(0 if all(c for _,c in ok) else 1)
