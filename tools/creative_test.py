#!/usr/bin/env python3
"""Inventaire et mode créatif, avec de vraies entrées."""
import sys, time
from playwright.sync_api import sync_playwright
ok=True
def check(n,c,i=''):
    global ok; ok=ok and bool(c); print(('OK  ' if c else 'ÉCHEC ')+n,i)
with sync_playwright() as p:
    b=p.chromium.launch(args=['--use-angle=swiftshader','--enable-unsafe-swiftshader','--ignore-gpu-blocklist'])
    pg=b.new_page(viewport={'width':900,'height':620}); errs=[]; pg.on('pageerror',lambda e:errs.append(str(e)[:200]))
    pg.goto('file:///home/claude/a-living-world/dist/index.html#q=bas&r=3&seed=20261104')
    pg.wait_for_function('window.ALW&&ALW.loaded',timeout=120000); pg.wait_for_function('ALW.world.queued===0&&ALW.world.pending===0&&ALW.player.ground',timeout=120000)
    pg.evaluate('ALW.r.resize(320,220,1);ALW.resize=()=>{}')
    # inventaire en survie : échange de deux cases
    pg.keyboard.press('KeyE'); pg.wait_for_function("document.getElementById('inv').classList.contains('on')",timeout=30000)
    check('inventaire ouvert, 36 cases', pg.evaluate("ALW.invUI.cells.length")==36)
    check('catalogue masqué en survie', pg.evaluate("document.getElementById('invCreative').hidden"))
    pg.evaluate("ALW.invUI.cells[4].click();ALW.invUI.cells[20].click()")
    check('échange de cases', pg.evaluate("ALW.inv[4]===null&&ALW.inv[20].item==='dirt'"))
    pg.evaluate("ALW.invUI.cells[20].click();ALW.invUI.cells[4].click()")
    pg.keyboard.press('KeyE'); pg.wait_for_function("!ALW.paused",timeout=30000)
    # créatif
    pg.keyboard.press('KeyG'); pg.wait_for_function('ALW.creative===true',timeout=30000)
    pg.keyboard.press('KeyE'); pg.wait_for_function("document.getElementById('inv').classList.contains('on')",timeout=30000)
    cats=pg.evaluate("[...document.querySelectorAll('#catTabs button')].map(b=>b.textContent)"); check('catégories classées',len(cats)==7,str(cats))
    pg.evaluate("[...document.querySelectorAll('#catTabs button')][1].click()")
    names=pg.evaluate("[...document.querySelectorAll('#catItems span')].map(b=>b.textContent)"); check('catégorie Roches',('Pierre moussue' in names),str(names))
    pg.screenshot(path='captures/inventaire.png',timeout=90000)
    pg.evaluate("ALW.select(4);[...document.querySelectorAll('#catItems button')][1].click()")
    check('bloc du catalogue dans la barre', pg.evaluate("ALW.inv[4].item")=='mossy_stone')
    pg.evaluate("[...document.querySelectorAll('#catTabs button')][4].click()"); nb=pg.evaluate("document.querySelectorAll('#catItems img').length"); check('icônes des plantes',nb==9,str(nb))
    pg.keyboard.press('KeyE'); pg.wait_for_function("!ALW.paused",timeout=30000)
    # vol
    y0=pg.evaluate('ALW.player.p[1]'); pg.keyboard.press('KeyF'); pg.wait_for_function('ALW.player.fly===true',timeout=30000)
    pg.keyboard.down('Space'); pg.wait_for_function(f'ALW.player.p[1]>{y0+2}',timeout=90000,polling=100); pg.keyboard.up('Space')
    y1=pg.evaluate('ALW.player.p[1]'); time.sleep(4); y2=pg.evaluate('ALW.player.p[1]'); check('vol : monte et reste en l\'air', y2>=y1-1 and y2>y0+1, f'{y0:.1f} -> {y2:.1f}')
    pg.keyboard.down('ControlLeft'); pg.wait_for_function(f'ALW.player.p[1]<{y2-1}',timeout=90000,polling=100); pg.keyboard.up('ControlLeft'); check('vol : descend',True)
    pg.keyboard.press('KeyF'); pg.wait_for_function('ALW.player.ground',timeout=90000)
    # casse instantanée sur la pierre, pose illimitée
    pg.evaluate("ALW.dragMode=true;ALW.view.third=false;ALW.player.pitch=-.95"); time.sleep(3)
    pg.wait_for_function('!!ALW.target',timeout=90000)
    before=pg.evaluate('ALW.world.edits.size'); pg.evaluate("window.__n=[...ALW.world.edits.values()].reduce((a,m)=>a+m.size,0)")
    pg.mouse.move(450,300); pg.mouse.click(450,300); pg.wait_for_function("[...ALW.world.edits.values()].reduce((a,m)=>a+m.size,0)>__n",timeout=90000); check('casse instantanée',True)
    n=pg.evaluate("ALW.inv[4]&&ALW.inv[4].n"); pg.evaluate("ALW.select(4)"); time.sleep(2)
    for _ in range(5):
        if pg.evaluate('!!ALW.target'): pg.mouse.click(450,300,button='right'); time.sleep(2)
        if pg.evaluate("[...ALW.world.edits.values()].some(m=>[...m.values()].includes(6))"): break
        pg.evaluate('ALW.player.yaw+=.9'); time.sleep(3)
    check('pose illimitée', pg.evaluate("[...ALW.world.edits.values()].some(m=>[...m.values()].includes(6))") and pg.evaluate("ALW.inv[4].n")==n, str(n))
    pg.evaluate('ALW.save(false)'); time.sleep(1); pg.reload(wait_until='commit',timeout=120000); time.sleep(4)
    pg.wait_for_function('window.ALW&&ALW.loaded',timeout=120000)
    check('mode et inventaire restaurés', pg.evaluate("ALW.creative===true&&ALW.inv.length===36&&ALW.inv[4].item==='mossy_stone'"))
    check('aucune exception JS',not errs,str(errs[:2])); b.close()
sys.exit(0 if ok else 1)
