// UI/ui.js — HUD et menus en DOM. Rendu des icônes à partir des vraies textures du jeu.
import { ITEMS, CATEGORIES, STACK } from './items.js';

export const $ = (id) => document.getElementById(id);

export class HUD {
  constructor(renderer, items) {
    this.slots = []; this.icons = {};
    for (const n in items) this.icons[n] = renderer.icon(items[n]);
    const bar = $('hotbar');
    for (let i = 0; i < 9; i++) { const d = document.createElement('div'); d.className = 'slot'; d.innerHTML = `<img alt=""><b></b><i>${i + 1}</i>`; bar.appendChild(d); this.slots.push(d); }
  }
  paint(el, it) {
    const img = el.querySelector('img'), n = el.querySelector('b');
    if (it) { if (img.dataset.k !== it.item) { img.src = this.icons[it.item] || ''; img.dataset.k = it.item; } img.style.visibility = ''; img.style.opacity = it.n === 0 ? 0.3 : 1; n.textContent = it.n === undefined ? '' : it.n; }
    else { img.style.visibility = 'hidden'; img.dataset.k = ''; n.textContent = ''; }
  }
  inventory(inv, sel, creative) {
    for (let i = 0; i < 9; i++) { const s = this.slots[i]; s.classList.toggle('on', i === sel); this.paint(s, creative && inv[i] && inv[i].n !== undefined ? { item: inv[i].item } : inv[i]); }
    const cur = inv[sel]; $('item').textContent = cur ? ITEMS[cur.item].name : '';
  }
  vitals(p) { $('barHp').style.width = p.hp + '%'; $('barFood').style.width = p.food + '%'; $('barWater').style.width = p.water + '%'; $('tHp').textContent = Math.round(p.hp) + ' / 100'; $('tFood').textContent = Math.round(p.food) + ' / 100'; $('tWater').textContent = Math.round(p.water) + ' / 100'; }
  compass(yaw) {
    const deg = ((-yaw * 180) / Math.PI + 180 + 360) % 360; const marks = [['N', 0], ['E', 90], ['S', 180], ['O', 270], ['', 45], ['', 135], ['', 225], ['', 315]];
    let h = ''; for (const [l, a] of marks) { let d = a - deg; d = ((d + 540) % 360) - 180; if (Math.abs(d) < 75) h += `<span style="left:${50 + d / 1.5}%" class="${l ? 'c' : 't'}">${l || '·'}</span>`; }
    $('compass').innerHTML = h;
  }
  toast(t) { const e = $('toast'); e.textContent = t; e.classList.add('on'); clearTimeout(this.tt); this.tt = setTimeout(() => e.classList.remove('on'), 2200); }
}

// Inventaire au glisser-déposer : mêmes événements pour la souris et le doigt.
export class InventoryUI {
  constructor(game) {
    this.g = game; this.cat = 0; this.cells = []; this.drag = null;
    const mk = (root, i) => { const d = document.createElement('div'); d.className = 'slot cell'; d.dataset.i = i; d.innerHTML = '<img alt="" draggable="false"><b></b>'; d.addEventListener('pointerdown', (e) => this.start(e, { slot: i })); root.appendChild(d); this.cells[i] = d; };
    for (let i = 9; i < 36; i++) mk($('invBag'), i);
    for (let i = 0; i < 9; i++) mk($('invBar'), i);
    CATEGORIES.forEach(([n], i) => { const b = document.createElement('button'); b.textContent = n; b.className = 'tab'; b.onclick = () => { this.cat = i; this.render(); }; $('catTabs').appendChild(b); });
    window.addEventListener('pointermove', (e) => this.move(e)); window.addEventListener('pointerup', (e) => this.end(e)); window.addEventListener('pointercancel', () => this.cancel());
  }
  open(on) { $('inv').classList.toggle('on', on); this.cancel(); if (on) this.render(); }
  start(e, src) {
    const it = src.slot !== undefined ? this.g.inv[src.slot] : src; if (!it || e.button > 0) return;
    e.preventDefault(); this.drag = { src, id: e.pointerId, x: e.clientX, y: e.clientY, moved: false, item: it.item || src.catalog };
  }
  move(e) {
    const d = this.drag; if (!d || e.pointerId !== d.id) return;
    if (!d.moved && Math.hypot(e.clientX - d.x, e.clientY - d.y) > 6) {
      d.moved = true; const g = $('dragGhost'); g.src = this.g.hud.icons[d.item]; g.hidden = false;
      if (d.src.slot !== undefined) this.cells[d.src.slot].classList.add('lifted');
    }
    if (!d.moved) return;
    const g = $('dragGhost'); g.style.left = e.clientX + 'px'; g.style.top = e.clientY + 'px';
    this.cells.forEach((c) => c.classList.remove('over')); const t = this.cellAt(e); if (t) t.classList.add('over');
  }
  cellAt(e) { const el = document.elementFromPoint(e.clientX, e.clientY); return el && el.closest ? el.closest('.cell[data-i]') : null; }
  end(e) {
    const d = this.drag; if (!d || e.pointerId !== d.id) return;
    const t = this.cellAt(e), to = t ? +t.dataset.i : null;
    if (!d.moved) { // simple toucher : choisit la case active, ou remplit celle-ci depuis le catalogue
      if (d.src.catalog) this.put(d.src.catalog, this.g.sel); else if (d.src.slot < 9) this.g.select(d.src.slot);
    } else if (to !== null) {
      if (d.src.catalog) this.put(d.src.catalog, to); else this.moveStack(d.src.slot, to);
    } else if (this.g.creative && d.src.slot !== undefined && !(document.elementFromPoint(e.clientX, e.clientY) || document.body).closest('#invBag,#invBar')) {
      this.g.inv[d.src.slot] = null; $('invTip').textContent = 'Objet retiré de la case.'; this.g.refreshInv(); // en créatif, le stock est illimité : jeter vide simplement la case
    }
    this.cancel(); this.render();
  }
  cancel() { this.drag = null; const g = $('dragGhost'); if (g) g.hidden = true; this.cells.forEach((c) => c.classList.remove('over', 'lifted')); }
  moveStack(a, i) {
    const g = this.g, inv = g.inv, A = inv[a], Z = inv[i];
    if (a === i || !A) return;
    if (Z && A.item === Z.item && A.n !== undefined && !g.creative) { const m = Math.min(STACK - Z.n, A.n); Z.n += m; A.n -= m; if (A.n <= 0) inv[a] = null; } else { inv[a] = Z; inv[i] = A; }
    g.refreshInv();
  }
  put(name, t) { const g = this.g; g.inv[t] = ITEMS[name].block !== undefined ? { item: name, n: STACK } : { item: name }; g.refreshInv(); $('invTip').textContent = ITEMS[name].name + (t < 9 ? ' dans la case ' + (t + 1) : ' dans le sac'); }
  render() {
    const g = this.g;
    this.cells.forEach((c, i) => { g.hud.paint(c, g.creative && g.inv[i] && g.inv[i].n !== undefined ? { item: g.inv[i].item } : g.inv[i]); c.classList.toggle('sel', i === g.sel); c.title = g.inv[i] ? ITEMS[g.inv[i].item].name : ''; });
    $('invCreative').hidden = !g.creative;
    $('invTitle').textContent = g.creative ? 'Inventaire, mode créatif' : 'Inventaire';
    if (!g.creative) return;
    [...$('catTabs').children].forEach((b, i) => b.classList.toggle('on', i === this.cat));
    const list = $('catItems'); list.textContent = '';
    for (const name of CATEGORIES[this.cat][1]) { const b = document.createElement('div'); b.className = 'slot cell cat'; b.title = ITEMS[name].name; b.dataset.item = name; b.innerHTML = `<img alt="${ITEMS[name].name}" draggable="false" src="${g.hud.icons[name]}"><span>${ITEMS[name].name}</span>`; b.addEventListener('pointerdown', (e) => this.start(e, { catalog: name })); list.appendChild(b); }
  }
}
