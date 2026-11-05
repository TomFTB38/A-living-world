// UI/ui.js — HUD et menus en DOM. Rendu des icônes à partir des vraies textures du jeu.
import { BLOCKS, B, MATERIALS, VARIANTS } from '../world/blocks.js';

export const $ = (id) => document.getElementById(id);

function faceCanvas(src, mat, shade) {
  const Z = src.size, c = document.createElement('canvas'); c.width = c.height = Z;
  const im = c.getContext('2d').createImageData(Z, Z), o = MATERIALS.indexOf(mat) * VARIANTS * Z * Z * 4;
  for (let i = 0; i < Z * Z * 4; i += 4) { im.data[i] = src.albedo[o + i] * shade; im.data[i + 1] = src.albedo[o + i + 1] * shade; im.data[i + 2] = src.albedo[o + i + 2] * shade; im.data[i + 3] = 255; }
  c.getContext('2d').putImageData(im, 0, 0); return c;
}
export function blockIcon(src, top, side) {
  const c = document.createElement('canvas'); c.width = c.height = 56; const g = c.getContext('2d'); g.imageSmoothingEnabled = false;
  const t = faceCanvas(src, top, 1), l = faceCanvas(src, side, 0.78), r = faceCanvas(src, side, 0.58), S = 22 / src.size;
  g.setTransform(S * 1.07, S * 0.55, -S * 1.07, S * 0.55, 28, 5); g.drawImage(t, 0, 0);
  g.setTransform(S * 1.07, S * 0.55, 0, S * 1.2, 4.5, 17); g.drawImage(l, 0, 0);
  g.setTransform(S * 1.07, -S * 0.55, 0, S * 1.2, 28, 29); g.drawImage(r, 0, 0);
  return c.toDataURL();
}
const TOOL = {
  pickaxe: '<path d="M9 45 34 16" stroke="#7a4a22" stroke-width="5"/><path d="M15 12c13-5 25 0 32 15-9-8-20-12-29-9z" fill="#a9b0b8"/>',
  axe: '<path d="M14 46 38 12" stroke="#7a4a22" stroke-width="5"/><path d="M30 6l16 14-9 10c-5-3-11-8-14-13z" fill="#a9b0b8"/>',
  sword: '<path d="M44 8 22 34l4 4L48 14z" fill="#c9ced4"/><path d="M15 31l14 14M18 42l-6 6" stroke="#7a4a22" stroke-width="5"/>',
};
export const toolIcon = (n) => 'data:image/svg+xml,' + encodeURIComponent(`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 56 56" fill="none">${TOOL[n]}</svg>`);

export const ITEMS = {
  pickaxe: { tool: 'rock', name: 'Pioche' }, axe: { tool: 'wood', name: 'Hache' }, sword: { tool: null, name: 'Épée' },
  lantern: { block: B.lantern, name: 'Lanterne' }, dirt: { block: B.dirt, name: 'Terre' }, stone: { block: B.stone, name: 'Pierre' },
  planks: { block: B.planks, name: 'Planches' }, oak_log: { block: B.oak_log, name: 'Bois de chêne' }, birch_log: { block: B.birch_log, name: 'Bois de bouleau' },
};

export class HUD {
  constructor(tex) {
    this.slots = [];
    const icon = { pickaxe: toolIcon('pickaxe'), axe: toolIcon('axe'), sword: toolIcon('sword'), dirt: blockIcon(tex, 'dirt', 'dirt'), stone: blockIcon(tex, 'stone', 'stone'), planks: blockIcon(tex, 'planks', 'planks'), oak_log: blockIcon(tex, 'end_oak', 'bark_oak'), birch_log: blockIcon(tex, 'end_birch', 'bark_birch'), beech_log: blockIcon(tex, 'end_beech', 'bark_beech'),
      lantern: 'data:image/svg+xml,' + encodeURIComponent('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 56 56"><rect x="19" y="16" width="18" height="26" rx="2" fill="#ffc35a"/><path d="M16 14h24v5H16zM16 40h24v5H16zM25 7h6v8h-6z" fill="#4a2c15"/></svg>') };
    this.icon = icon; const bar = $('hotbar');
    for (let i = 0; i < 9; i++) { const d = document.createElement('div'); d.className = 'slot'; d.innerHTML = `<img alt=""><b></b><i>${i + 1}</i>`; bar.appendChild(d); this.slots.push(d); }
  }
  inventory(inv, sel) {
    inv.forEach((it, i) => {
      const s = this.slots[i], img = s.querySelector('img'), n = s.querySelector('b');
      s.classList.toggle('on', i === sel);
      if (it) { if (img.dataset.k !== it.item) { img.src = this.icon[it.item] || ''; img.dataset.k = it.item; } img.style.visibility = ''; img.style.opacity = it.n === 0 ? 0.3 : 1; n.textContent = it.n === undefined ? '' : it.n; }
      else { img.style.visibility = 'hidden'; img.dataset.k = ''; n.textContent = ''; }
    });
    const cur = inv[sel]; const nm = $('item'); nm.textContent = cur ? ITEMS[cur.item].name : '';
  }
  vitals(p) { $('barHp').style.width = p.hp + '%'; $('barFood').style.width = p.food + '%'; $('barWater').style.width = p.water + '%'; $('tHp').textContent = Math.round(p.hp) + ' / 100'; $('tFood').textContent = Math.round(p.food) + ' / 100'; $('tWater').textContent = Math.round(p.water) + ' / 100'; }
  compass(yaw) {
    const deg = ((-yaw * 180) / Math.PI + 180 + 360) % 360; const marks = [['N', 0], ['E', 90], ['S', 180], ['O', 270], ['', 45], ['', 135], ['', 225], ['', 315]];
    let h = ''; for (const [l, a] of marks) { let d = a - deg; d = ((d + 540) % 360) - 180; if (Math.abs(d) < 75) h += `<span style="left:${50 + d / 1.5}%" class="${l ? 'c' : 't'}">${l || '·'}</span>`; }
    $('compass').innerHTML = h;
  }
  toast(t) { const e = $('toast'); e.textContent = t; e.classList.add('on'); clearTimeout(this.tt); this.tt = setTimeout(() => e.classList.remove('on'), 2200); }
}
void BLOCKS;
