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

// Inventaire : toucher une case puis une autre échange leur contenu, à la souris comme au doigt.
export class InventoryUI {
  constructor(game) {
    this.g = game; this.pick = null; this.cat = 0; this.cells = [];
    const mk = (root, i) => { const d = document.createElement('button'); d.className = 'slot cell'; d.innerHTML = '<img alt=""><b></b>'; d.onclick = () => this.tap(i); root.appendChild(d); this.cells[i] = d; };
    for (let i = 9; i < 36; i++) mk($('invBag'), i);
    for (let i = 0; i < 9; i++) mk($('invBar'), i);
    CATEGORIES.forEach(([n], i) => { const b = document.createElement('button'); b.textContent = n; b.className = 'tab'; b.onclick = () => { this.cat = i; this.render(); }; $('catTabs').appendChild(b); });
  }
  open(on) { $('inv').classList.toggle('on', on); this.pick = null; if (on) this.render(); }
  tap(i) {
    const g = this.g, inv = g.inv;
    if (this.pick === null) { if (inv[i]) this.pick = i; this.render(); return; }
    const a = this.pick, A = inv[a], Z = inv[i]; this.pick = null;
    if (a !== i) {
      if (A && Z && A.item === Z.item && A.n !== undefined && !g.creative) { const m = Math.min(STACK - Z.n, A.n); Z.n += m; A.n -= m; if (A.n <= 0) inv[a] = null; }
      else { inv[a] = Z; inv[i] = A; }
    }
    g.refreshInv(); this.render();
  }
  grant(name) { const g = this.g, t = this.pick !== null ? this.pick : g.sel; g.inv[t] = ITEMS[name].block !== undefined ? { item: name, n: STACK } : { item: name }; this.pick = null; g.refreshInv(); this.render(); $('invTip').textContent = ITEMS[name].name + ' ajouté à la case ' + (t < 9 ? t + 1 : 'du sac'); }
  render() {
    const g = this.g;
    this.cells.forEach((c, i) => { g.hud.paint(c, g.creative && g.inv[i] && g.inv[i].n !== undefined ? { item: g.inv[i].item } : g.inv[i]); c.classList.toggle('on', i === this.pick); c.classList.toggle('sel', i === g.sel); });
    $('invCreative').hidden = !g.creative;
    $('invTitle').textContent = g.creative ? 'Inventaire, mode créatif' : 'Inventaire';
    if (!g.creative) return;
    [...$('catTabs').children].forEach((b, i) => b.classList.toggle('on', i === this.cat));
    const list = $('catItems'); list.textContent = '';
    for (const name of CATEGORIES[this.cat][1]) { const b = document.createElement('button'); b.className = 'slot cell'; b.title = ITEMS[name].name; b.innerHTML = `<img alt="${ITEMS[name].name}" src="${g.hud.icons[name]}"><span>${ITEMS[name].name}</span>`; b.onclick = () => this.grant(name); list.appendChild(b); }
  }
}
