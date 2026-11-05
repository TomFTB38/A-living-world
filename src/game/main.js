// Gameplay/main.js — boucle de jeu, entrées, sauvegarde, profilage.
import { M, V } from '../core/math.js';
import { clamp } from '../core/noise.js';
import { Renderer, PRESETS } from '../render/renderer.js';
import { ClientWorld } from './client.js';
import { Player, raycast } from './player.js';
import { buildCharacter, CharacterAnim } from './character.js';
import { buildFauna, Fauna } from './animals.js';
import { Weather } from './weather.js';
import { SoundScape, SOUNDS } from './audio.js';
import { pickMood } from './music.js';
import { HUD, InventoryUI, $ } from './ui.js';
import { ITEMS, STACK, buildItems } from './items.js';
import { BLOCKS, B, IS_SOLID, IS_DECO, orientedBlock, gazeAxis, axisOf, MODES } from '../world/blocks.js';
import { WorldGen, GEN_VERSION } from '../world/gen.js';
import { serialize, safeRead, atomicWrite, deserialize } from './save.js';

const params = new URLSearchParams(location.hash.slice(1));
const CAPTURE = params.has('capture');
const SLOT_KEY = 'alw.world.';
const storage = (() => { try { const s = window.localStorage; s.setItem('alw.t', '1'); s.removeItem('alw.t'); return s; } catch (e) { return null; } })();

const INV_SIZE = 36;
const DEFAULT_INV = [{ item: 'pickaxe' }, { item: 'axe' }, { item: 'sword' }, { item: 'lantern', n: 32 }, { item: 'dirt', n: 64 }, { item: 'stone', n: 32 }, { item: 'planks', n: 16 }];
const fullInv = (list) => { const a = list.map((i) => (i && i.n === 0 ? null : i ? { ...i } : null)); while (a.length < INV_SIZE) a.push(null); return a; };

class Game {
  constructor() {
    this.canvas = $('gl');
    this.settings = { preset: params.get('q') || 'moyen', scale: 0, fov: 60, sens: 1 };
    try { Object.assign(this.settings, JSON.parse(storage?.getItem('alw.settings') || '{}')); } catch (e) { /* réglages par défaut */ }
    if (params.get('q')) this.settings.preset = params.get('q');
    this.radiusParam = +params.get('r') || 0;
    this.r = new Renderer(this.canvas, { texSize: this.settings.preset === 'bas' ? 64 : 128 });
    this.r.texSource = null; // les piles sont sur le GPU : la copie JS peut être libérée
    this.items = buildItems(this.r);
    this.hud = new HUD(this.r, this.items);
    this.invUI = new InventoryUI(this);
    this.input = { keys: new Set(), x: 0, z: 0, jump: false, sprint: false, crouch: false, lmb: false, rmb: false, touch: { x: 0, z: 0 } };
    this.view = { third: true, dist: 4.2, camDist: 4.2 };
    this.charModel = buildCharacter(this.r); this.anim = new CharacterAnim(this.charModel); this.faunaModel = buildFauna(this.r);
    this.prof = { fps: 60, cpu: 0, show: CAPTURE ? false : false, frames: [] };
    this.hour = 16.4; this.dayLen = 1500; this.timeRun = !CAPTURE; this.elapsed = 0; this.mine = { key: '', t: 0 };
    this.vit = { hp: 100, food: 100, water: 100 };
    this.built = [];
    this.sound = new SoundScape(); this.sound.setVolume(this.settings.volume ?? 0.7); this.sound.setMusicVolume(this.settings.music ?? 0.5); this.sound.setSfxVolume(this.settings.sfx ?? 0.7);
    for (const n of ['pointerdown', 'keydown', 'touchend']) addEventListener(n, () => this.sound.start(), { passive: true });
    this.bindInput(); this.bindMenu();
    this.applySettings();
    const seed = Number(params.get('seed') || 20261104);
    const saved = !CAPTURE && storage ? safeRead(storage, SLOT_KEY + 'last') : null;
    this.startWorld(saved ? saved.seed : seed, saved);
    this.last = performance.now();
    requestAnimationFrame((t) => this.loop(t));
  }

  radius() { return this.radiusParam || (PRESETS[this.settings.preset] || PRESETS.moyen).radius; }

  startWorld(seed, save) {
    if (this.world) { for (const w of this.world.workers) w.terminate(); for (const c of this.world.chunks.values()) { this.r.freeMesh(c.opaque); this.r.freeMesh(c.deco); this.r.freeMesh(c.water); } if (this.world.far) this.r.freeMesh(this.world.far.mesh); }
    const edits = new Map(save ? save.edits.map(([k, e]) => [k, new Map(e)]) : []);
    this.seed = seed; this.worldId = save?.id || 'w' + seed; this.gen = new WorldGen(seed);
    const workers = Math.max(1, Math.min(4, (navigator.hardwareConcurrency || 4) - 1));
    this.world = new ClientWorld(this.r, seed, edits, { radius: this.radius(), workers });
    this.inv = fullInv(save?.inventory || DEFAULT_INV); this.sel = 0; this.creative = save?.creative || params.get('mode') === 'creatif'; document.getElementById('touch').classList.toggle('crea', this.creative);
    if (save) { this.hour = save.time; this.player = new Player(...save.player.p); this.player.yaw = save.player.yaw; this.player.pitch = save.player.pitch; this.player.moveYaw = save.player.yaw; this.player.fly = !!save.player.fly; this.vit = save.player.vit || this.vit; }
    else { const s = this.findSpawn(); this.player = new Player(s[0], s[1], s[2]); this.player.yaw = s[3]; this.player.moveYaw = s[3]; this.player.pitch = -0.13; this.hour = 16.4; }
    this.fauna = new Fauna(this.faunaModel, seed); this.fauna.emit = (k, pos, o) => this.sound[k](pos, o); this.enemyNear = false;
    this.weather = new Weather(seed); this.weather.forced = this.settings.weather || 'auto';
    this.loaded = false; $('loading').classList.add('on'); this.r.reset = true;
    this.refreshInv();
  }

  // Cherche un chemin en forêt : c'est le point de vue le plus proche de la référence.
  findSpawn() {
    const g = this.gen; let best = null, score = -1;
    for (let r = 0; r < 1600; r += 13) for (let a = 0; a < 6.283; a += 0.45) {
      const x = Math.round(Math.cos(a) * r), z = Math.round(Math.sin(a) * r), c = g.column(x, z);
      if (c.path < 0.7 || c.water) continue;
      const y = c.h, yy = Math.atan2(-0.75, 0.42), bh = g.column(x - Math.sin(yy) * 4.5, z - Math.cos(yy) * 4.5).h;
      if (bh > y + 0.6) continue; let f = c.forest * 2, flat = 0;
      for (const [dx, dz] of [[24, 0], [-24, 0], [0, 24], [0, -24], [17, 17], [-17, -17], [17, -17], [-17, 17]]) { const n = g.column(x + dx, z + dz); f += n.forest; flat += Math.abs(n.h - y); }
      let wet = 0; for (const [dx, dz] of [[14, 0], [-14, 0], [0, 14], [0, -14], [10, 10], [-10, -10], [10, -10], [-10, 10]]) if (g.column(x + dx, z + dz).water) wet = 1;
      const g2 = g.nForest.n2(x / 46 + 70, z / 46 - 40);
      const s = -Math.abs(g2 - 0.1) * 0.5 + Math.min(f / 10, 0.8) - flat * 0.006 - r / 9000 + wet * 0.35;
      if (s > score) { score = s; best = [x + 0.5, Math.ceil(c.h) + 2, z + 0.5, Math.atan2(-0.75, 0.42)]; }
    }
    if (best) { // Cadrage de départ : le sentier devant soi, l'eau sur la droite et le soleil en face si possible.
      let top = -1; const [bx, , bz] = best;
      for (let yaw = 0; yaw < 6.283; yaw += 0.26) {
        const fx = Math.sin(yaw), fz = Math.cos(yaw), rx = -fz, rz = fx; let sc = 0.5 * (-0.87 * fx + 0.49 * fz);
        for (const d of [7, 12, 18]) { if (g.column(bx + fx * d, bz + fz * d).path > 0.3) sc += 0.5; for (const r of [4, 8, 12]) if (g.column(bx + fx * d + rx * r, bz + fz * d + rz * r).water) sc += 0.3; }
        if (sc > top) { top = sc; best[3] = yaw; }
      }
      return best;
    }
    return [0.5, Math.ceil(g.column(0, 0).h) + 3, 0.5, 0];
  }

  applySettings() {
    this.r.setPreset(PRESETS[this.settings.preset] || PRESETS.moyen);
    this.resize();
    try { storage?.setItem('alw.settings', JSON.stringify(this.settings)); } catch (e) { /* sans effet */ }
  }
  resize() {
    const s = this.settings.scale || this.r.preset.scale, dpr = CAPTURE ? 1 : Math.min(window.devicePixelRatio || 1, 1.5);
    this.dynScale = this.dynScale || 1;
    this.r.resize(innerWidth, innerHeight, s * dpr * this.dynScale);
  }

  bindInput() {
    const c = this.canvas, K = this.input.keys;
    addEventListener('resize', () => this.resize());
    addEventListener('keydown', (e) => {
      if (e.repeat) return;
      if (e.code === 'KeyE') { this.toggleInv(); return; }
      if (e.code === 'Escape' && !document.pointerLockElement) { if (this.invOpen) this.toggleInv(false); else this.toggleMenu(); return; }
      if (this.paused) return;
      K.add(e.code);
      if (e.code === 'KeyG') this.setMode(!this.creative);
      if (e.code === 'KeyF' && this.creative) this.setFly(!this.player.fly);
      if (e.code === 'Space' && this.creative) { const t = performance.now(); if (t - (this.lastSpace || 0) < 320) this.setFly(!this.player.fly); this.lastSpace = t; }
      if (e.code.startsWith('Digit')) { const n = +e.code.slice(5); if (n >= 1) this.select(n - 1); }
      if (e.code === 'KeyR') this.cycleOrient();
      if (e.code === 'KeyC') this.toggleView();
      if (e.code === 'F3') { e.preventDefault(); this.prof.show = !this.prof.show; $('prof').classList.toggle('on', this.prof.show); }
      if (e.code === 'KeyT') this.hour = (this.hour + 1) % 24;
      if (e.code === 'KeyY') this.timeRun = !this.timeRun;
      if (e.code === 'F5') { e.preventDefault(); this.save(true); }
      if (['Space', 'ArrowUp', 'ArrowDown'].includes(e.code)) e.preventDefault();
    });
    addEventListener('keyup', (e) => K.delete(e.code));
    addEventListener('blur', () => { K.clear(); this.input.lmb = this.input.rmb = false; });
    const look = (dx, dy) => { const s = 0.0023 * this.settings.sens, p = this.player; p.yaw -= dx * s; p.pitch = clamp(p.pitch - dy * s, -1.5, 1.5); };
    document.addEventListener('pointerlockchange', () => { if (!document.pointerLockElement && !this.paused && !this.invOpen && this.lockedOnce && !this.dragMode) this.toggleMenu(true); });
    document.addEventListener('pointerlockerror', () => { this.dragMode = true; });
    let drag = null;
    c.addEventListener('mousedown', (e) => {
      if (this.paused) return;
      if (!document.pointerLockElement && !this.dragMode) {
        try { const q = c.requestPointerLock(); if (q && q.catch) q.catch(() => { this.dragMode = true; }); this.lockedOnce = true; } catch (x) { this.dragMode = true; }
      }
      if (e.button === 0) { if (this.dragMode) drag = { x: e.clientX, y: e.clientY, moved: 0 }; else this.input.lmb = true; }
      if (e.button === 2) { this.place(); this.input.rmb = true; this.placeT = 0.28; }
      e.preventDefault();
    });
    addEventListener('mouseup', (e) => {
      if (e.button === 0) { this.input.lmb = false; if (drag && drag.moved < 6) this.autoMine = 1.4; drag = null; }
      if (e.button === 2) this.input.rmb = false;
    });
    addEventListener('mousemove', (e) => {
      if (document.pointerLockElement === c) look(e.movementX, e.movementY);
      else if (drag) { look(e.clientX - drag.x, e.clientY - drag.y); drag.moved += Math.abs(e.clientX - drag.x) + Math.abs(e.clientY - drag.y); drag.x = e.clientX; drag.y = e.clientY; }
    });
    c.addEventListener('contextmenu', (e) => e.preventDefault());
    addEventListener('wheel', (e) => { if (this.paused) return; this.select((this.sel + Math.sign(e.deltaY) + 9) % 9); }, { passive: true });
    // Tactile : stick à gauche, regard à droite.
    const T = this.input.touch; let stick = null, lookT = null;
    c.addEventListener('touchstart', (e) => { $('touch').classList.add('on'); for (const t of e.changedTouches) { if (t.clientX < innerWidth * 0.4 && !stick) stick = { id: t.identifier, x: t.clientX, y: t.clientY }; else if (!lookT) lookT = { id: t.identifier, x: t.clientX, y: t.clientY }; } e.preventDefault(); }, { passive: false });
    c.addEventListener('touchmove', (e) => { for (const t of e.changedTouches) { if (stick && t.identifier === stick.id) { T.x = clamp((t.clientX - stick.x) / 50, -1, 1); T.z = clamp((t.clientY - stick.y) / 50, -1, 1); } else if (lookT && t.identifier === lookT.id) { look((t.clientX - lookT.x) * 2, (t.clientY - lookT.y) * 2); lookT.x = t.clientX; lookT.y = t.clientY; } } e.preventDefault(); }, { passive: false });
    c.addEventListener('touchend', (e) => { for (const t of e.changedTouches) { if (stick && t.identifier === stick.id) { stick = null; T.x = T.z = 0; } if (lookT && t.identifier === lookT.id) lookT = null; } });
    const hold = (id, on, off) => { const b = $(id); b.addEventListener('touchstart', (e) => { on(); e.preventDefault(); }, { passive: false }); b.addEventListener('touchend', (e) => { off(); e.preventDefault(); }, { passive: false }); };
    hold('tJump', () => K.add('Space'), () => K.delete('Space'));
    hold('tBreak', () => { this.input.lmb = true; }, () => { this.input.lmb = false; });
    hold('tPlace', () => this.place(), () => {});
    hold('tMenu', () => this.toggleMenu(true), () => {});
    hold('tAxis', () => this.cycleOrient(), () => {});
    hold('tInv', () => this.toggleInv(), () => {});
    hold('tView', () => this.toggleView(), () => {});
    const latch = (id, code) => hold(id, () => { const on = !K.has(code); if (on) K.add(code); else K.delete(code); $(id).classList.toggle('on', on); }, () => {});
    latch('tRun', 'ShiftLeft'); latch('tLow', 'ControlLeft');
    hold('tFly', () => this.setFly(!this.player.fly), () => {});
    hold('tDown', () => K.add('ControlLeft'), () => K.delete('ControlLeft'));
    hold('tFps', () => { this.prof.show = !this.prof.show; $('prof').classList.toggle('on', this.prof.show); $('tFps').classList.toggle('on', this.prof.show); this.report(); }, () => {});
  }

  bindMenu() {
    $('btnResume').onclick = () => this.toggleMenu(false);
    $('invClose').onclick = () => this.toggleInv(false);
    $('selMode').onchange = (e) => this.setMode(e.target.value === 'creatif');
    $('btnSave').onclick = () => this.save(true);
    $('btnExport').onclick = () => this.exportSave();
    $('btnImport').onclick = () => $('fileImport').click();
    $('fileImport').onchange = async (e) => { const f = e.target.files[0]; if (!f) return; try { const s = deserialize(await f.text()); this.startWorld(s.seed, s); this.toggleMenu(false); this.hud.toast('Monde importé'); } catch (x) { this.hud.toast(x.message); } };
    $('btnNew').onclick = () => { const v = Number($('inSeed').value) || Math.floor(Math.random() * 1e9); this.startWorld(v, null); $('inSeed').value = v; this.toggleMenu(false); };
    const met = $('selWeather'), vol = $('inVol');
    met.value = this.settings.weather || 'auto'; vol.value = this.settings.volume ?? 0.7;
    met.onchange = () => { this.settings.weather = met.value; this.weather.forced = met.value; this.applySettings(); };
    vol.oninput = () => { this.settings.volume = +vol.value; this.sound.setVolume(+vol.value); this.applySettings(); };
    const fx = $('inSfx'); fx.value = this.settings.sfx ?? 0.7; fx.oninput = () => { this.settings.sfx = +fx.value; this.sound.setSfxVolume(+fx.value); this.applySettings(); };
    const mv = $('inMusic'), mf = $('selMusic');
    mv.value = this.settings.music ?? 0.5; mf.value = this.settings.mood || 'auto';
    mv.oninput = () => { this.settings.music = +mv.value; this.sound.setMusicVolume(+mv.value); this.applySettings(); };
    mf.onchange = () => { this.settings.mood = mf.value; this.applySettings(); };
    const sel = $('selPreset'), fov = $('inFov'), sens = $('inSens');
    sel.value = this.settings.preset; fov.value = this.settings.fov; sens.value = this.settings.sens;
    sel.onchange = () => { this.settings.preset = sel.value; this.dynScale = 1; this.applySettings(); this.world.radius = this.radius(); };
    fov.oninput = () => { this.settings.fov = +fov.value; this.applySettings(); };
    sens.oninput = () => { this.settings.sens = +sens.value; this.applySettings(); };
  }
  toggleMenu(v) {
    this.paused = v === undefined ? !this.paused : v; $('menu').classList.toggle('on', this.paused);
    if (this.paused) { if (document.pointerLockElement) document.exitPointerLock(); $('inSeed').value = this.seed; }
  }
  toggleView() { this.view.third = !this.view.third; this.hud.toast(this.view.third ? 'Troisième personne' : 'Première personne'); }
  select(i) { this.sel = i; this.refreshInv(); }
  refreshInv() { this.hud.inventory(this.inv, this.sel, this.creative); this.dirtySave = true; }
  toggleInv(v) {
    this.invOpen = v === undefined ? !this.invOpen : v; this.paused = this.invOpen; this.invUI.open(this.invOpen);
    if (this.invOpen && document.pointerLockElement) document.exitPointerLock();
  }
  setMode(on) {
    this.creative = on; if (!on) this.setFly(false);
    document.getElementById('touch').classList.toggle('crea', on); $('selMode').value = on ? 'creatif' : 'survie';
    this.refreshInv(); this.hud.toast(on ? 'Mode créatif : blocs illimités, casse instantanée, vol avec F' : 'Mode survie');
  }
  setFly(on) { this.player.fly = on; this.player.v[1] = 0; document.getElementById('touch').classList.toggle('flying', on); this.hud.toast(on ? 'Vol activé' : 'Vol désactivé'); }
  addItem(name, n = 1) {
    if (!ITEMS[name]) return;
    let left = n;
    for (const s of this.inv) if (left && s && s.item === name && s.n !== undefined && s.n < STACK) { const m = Math.min(left, STACK - s.n); s.n += m; left -= m; }
    for (let i = 0; i < INV_SIZE && left; i++) if (!this.inv[i]) { this.inv[i] = { item: name, n: Math.min(left, STACK) }; left -= this.inv[i].n; }
    this.refreshInv();
  }

  saveState() {
    return { id: this.worldId, seed: this.seed, genVersion: GEN_VERSION, time: this.hour, day: 1, player: { p: this.player.p, yaw: this.player.yaw, pitch: this.player.pitch, vit: this.vit, fly: !!this.player.fly }, inventory: this.inv, creative: !!this.creative, edits: [...this.world.edits].map(([k, m]) => [k, [...m]]) };
  }
  async save(toast) {
    const text = serialize(this.saveState());
    let where = '';
    try { if (storage) { atomicWrite(storage, SLOT_KEY + 'last', text); where = 'navigateur'; } } catch (e) { where = ''; }
    try {
      if (window.claude) {
        const db = await window.claude.use('db'), user = await window.claude.use('user');
        if (db && user) { const id = await user.id(); await db.collection('data/users/' + id).doc('world').set({ text, savedAt: Date.now() }); where = 'compte'; }
      }
    } catch (e) { /* secours : navigateur */ }
    if (toast) this.hud.toast(where ? 'Monde sauvegardé (' + where + ')' : 'Sauvegarde impossible ici : utilisez Exporter');
  }
  async restoreRemote() {
    try {
      if (!window.claude || CAPTURE) return;
      const db = await window.claude.use('db'), user = await window.claude.use('user'); if (!db || !user) return;
      const id = await user.id(), snap = await db.collection('data/users/' + id).doc('world').get(), d = snap && (typeof snap.data === 'function' ? snap.data() : snap);
      if (d && d.text && !(storage && storage.getItem(SLOT_KEY + 'last'))) { const s = deserialize(d.text); this.startWorld(s.seed, s); }
    } catch (e) { /* jeu utilisable sans compte */ }
  }
  async exportSave() {
    const text = serialize(this.saveState()), name = 'a-living-world-' + this.seed + '.alw';
    try { if (window.claude) { const d = await window.claude.use('downloads'); if (d) { await d.save({ filename: name, data: text }); return; } } } catch (e) { return; }
    const a = document.createElement('a'); a.href = URL.createObjectURL(new Blob([text])); a.download = name; a.click();
  }

  camera() {
    if (this.camOverride) return this.camOverride;
    const p = this.player, cp = Math.cos(p.pitch), sp = Math.sin(p.pitch), f = [-Math.sin(p.yaw) * -cp, sp, Math.cos(p.yaw) * cp].map((v, i) => (i === 0 ? Math.sin(p.yaw) * cp : v));
    const head = [p.p[0], p.p[1] + (p.crouch ? 1.32 : 1.62), p.p[2]], right = V.norm([Math.cos(p.yaw), 0, -Math.sin(p.yaw)]);
    if (!this.view.third) return { eye: head, f };
    // Caméra d'épaule, retenue par le terrain : le héros reste au tiers gauche, comme dans les références.
    const anchor = V.add(V.add(head, V.mul(right, -0.8)), [0, 0.42, 0]), back = V.mul(f, -1); let d = this.view.dist;
    const hit = raycast(this.world, anchor, V.norm(back), d + 0.3, (x, y, z) => { const b = this.world.get(x, y, z); return IS_DECO[b] || BLOCKS[b]?.shape === 'leaf'; });
    if (hit) d = Math.max(0.5, hit.t - 0.3);
    this.view.camDist += (d - this.view.camDist) * (d < this.view.camDist ? 0.5 : 0.08);
    return { eye: V.add(anchor, V.mul(back, this.view.camDist)), f };
  }

  // 0 suit le regard, 1 force la verticale, 2 force l'axe horizontal du regard.
  orientFor(block) {
    const pl = this.player, cp = Math.cos(pl.pitch);
    return orientedBlock(block, this.lastAxis = gazeAxis(Math.sin(pl.yaw) * cp, Math.sin(pl.pitch), Math.cos(pl.yaw) * cp, this.orientMode || 0, this.lastAxis));
  }
  cycleOrient() { this.orientMode = ((this.orientMode || 0) + 1) % MODES.length; this.hud.toast('Orientation : ' + MODES[this.orientMode]); $('tAxis').textContent = ['Auto', 'Vert.', 'Couché'][this.orientMode]; }
  // Où irait le bloc sélectionné, et serait-il accepté ?
  ghost() {
    const it = this.inv[this.sel], def = it && ITEMS[it.item], t = this.target;
    if (def?.block === undefined || !t || this.camOverride) return null;
    const pos = IS_DECO[t.id] && t.id !== B.lantern && !IS_SOLID[t.id] ? [t.x, t.y, t.z] : [t.px, t.py, t.pz], cur = this.world.get(pos[0], pos[1], pos[2]), id = this.orientFor(def.block), p = this.player.p;
    const hit = IS_SOLID[id] && pos[0] + 1 > p[0] - 0.3 && pos[0] < p[0] + 0.3 && pos[2] + 1 > p[2] - 0.3 && pos[2] < p[2] + 0.3 && pos[1] + 1 > p[1] && pos[1] < p[1] + 1.8;
    const axis = 'xyz'.indexOf(axisOf(id)) + 1;
    return { pos, ok: (this.creative || it.n > 0) && cur !== -1 && !IS_SOLID[cur] && !hit, axis };
  }

  place() {
    if (this.paused || !this.target) return;
    const it = this.inv[this.sel], def = it && ITEMS[it.item];
    if (def?.block === undefined || (!this.creative && !(it.n > 0))) return;
    const t = this.target, id = IS_DECO[t.id] && t.id !== B.lantern && !IS_SOLID[t.id] ? [t.x, t.y, t.z] : [t.px, t.py, t.pz];
    const cur = this.world.get(...id); if (cur === -1 || IS_SOLID[cur]) return;
    const pl = this.player, p = pl.p, bid = this.orientFor(def.block);
    if (IS_SOLID[bid] && id[0] + 1 > p[0] - 0.3 && id[0] < p[0] + 0.3 && id[2] + 1 > p[2] - 0.3 && id[2] < p[2] + 0.3 && id[1] + 1 > p[1] && id[1] < p[1] + 1.8) return;
    if (this.world.set(id[0], id[1], id[2], bid)) { if (!this.creative) { it.n--; if (it.n <= 0) this.inv[this.sel] = null; } this.refreshInv(); this.anim.trigger(); this.sound.place(BLOCKS[bid].name); this.built.push(this.elapsed); this.dirtySave = true; }
  }
  // Ce qui tenait sur le bloc retiré tombe avec lui : plantes au-dessus, lianes en dessous, champignons sur les côtés.
  dropAttached(x, y, z) {
    const w = this.world, up = w.get(x, y + 1, z);
    if (up > 0 && IS_DECO[up] && up !== B.vine && up !== B.shelf_fungus && up !== B.fence) w.set(x, y + 1, z, 0);
    for (let k = y - 1, n = 0; w.get(x, k, z) === B.vine && n < 12; k--, n++) w.set(x, k, z, 0);
    for (const [dx, dz] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) if (w.get(x + dx, y, z + dz) === B.shelf_fungus) {
      const kept = [[1, 0], [-1, 0], [0, 1], [0, -1]].some(([a, b]) => { const c = w.get(x + dx + a, y, z + dz + b); return c > 0 && BLOCKS[c].opaque; });
      if (!kept) w.set(x + dx, y, z + dz, 0);
    }
  }
  mineStep(dt) {
    const go = this.input.lmb || this.autoMine > 0; if (this.autoMine > 0) this.autoMine -= dt;
    this.mining = false;
    const held = this.inv[this.sel]?.item, tool = ITEMS[held] && ITEMS[held].block === undefined ? held : null;
    this.swingT = (this.swingT || 0) - dt;
    // Dans le vide, on frappe quand même, avec n'importe quoi.
    if (go && !this.paused && !this.target) {
      this.mining = true; if (this.autoMine > 0.3) this.autoMine = 0.3;
      if (this.swingT <= 0) { this.swingT = 0.3; this.sound.swing(tool); }
    }
    if (!go || !this.target || this.paused) { this.mine.key = ''; this.mine.t = 0; return; }
    const t = this.target, key = t.x + ',' + t.y + ',' + t.z, b = BLOCKS[t.id], it = this.inv[this.sel], def = it && ITEMS[it.item];
    if (key !== this.mine.key) { this.mine.key = key; this.mine.t = 0; }
    this.mining = true;
    const rate = def?.tool && def.tool === b.cat ? 3 : b.cat === 'terrain' ? 1.6 : 1;
    if (this.creative) { this.mineCool = (this.mineCool || 0) - dt; if (this.mineCool > 0) return; this.mineCool = 0.2; this.mine.t = 1e9; }
    this.mine.t += dt * rate;
    this.digT = (this.digT || 0) - dt; if (this.digT <= 0 && !this.creative) { this.digT = 0.3; this.sound.dig(b.name, tool); }
    if (this.mine.t >= b.hardness) {
      if (this.world.set(t.x, t.y, t.z, 0)) {
        this.sound.breakBlock(b.name, tool);
        this.dropAttached(t.x, t.y, t.z);
        if (b.drop && !this.creative) this.addItem(b.drop);
        this.dirtySave = true; this.autoMine = 0;
      }
      this.mine.key = ''; this.mine.t = 0;
    }
  }

  loop(now) {
    if (this.stopAt && this.r.frame >= this.stopAt && (this.world.queued || 0) + this.world.pending === 0) { this.stopped = true; return; }
    requestAnimationFrame((t) => this.loop(t));
    const t0 = performance.now(); let dt = (now - this.last) / 1000; this.last = now; if (!(dt > 0)) dt = 0.016; const rdt = dt; dt = Math.min(dt, 0.05);
    this.step(dt, rdt);
    if (CAPTURE && (this.world.queued || this.world.pending)) return; // captures : pas de rendu logiciel pendant le chargement
    this.draw(dt);
    const cpu = performance.now() - t0, pr = this.prof;
    pr.cpu = pr.cpu * 0.92 + cpu * 0.08; pr.fps = pr.fps * 0.92 + (1 / Math.max(rdt, 0.001)) * 0.08;
    if ((this.tick = (this.tick || 0) + 1) % 15 === 0) this.report();
    if (!CAPTURE && this.settings.scale === 0 && this.loaded && this.tick % 120 === 0) { const f = pr.fps; const was = this.dynScale; if (f < 42) this.dynScale = Math.max(0.55, this.dynScale - 0.08); else if (f > 57 && this.dynScale < 1) this.dynScale = Math.min(1, this.dynScale + 0.04); if (was !== this.dynScale) this.resize(); }
  }

  step(dt) {
    const I = this.input, K = I.keys, p = this.player;
    if (!this.paused && !this.freeze) {
      I.x = (K.has('KeyD') || K.has('ArrowRight') ? 1 : 0) - (K.has('KeyA') || K.has('KeyQ') || K.has('ArrowLeft') ? 1 : 0) + I.touch.x;
      I.z = (K.has('KeyS') || K.has('ArrowDown') ? 1 : 0) - (K.has('KeyW') || K.has('KeyZ') || K.has('ArrowUp') ? 1 : 0) + I.touch.z;
      I.z = -I.z; I.x = -I.x;
      I.jump = K.has('Space'); I.sprint = K.has('ShiftLeft') || K.has('ShiftRight'); I.crouch = K.has('ControlLeft') || K.has('KeyX');
      if (this.world.nearReady(p.p[0], p.p[2])) p.update(this.world, dt, I);
      this.anim.update(dt, { speed: p.speed, ground: p.ground, vy: p.v[1], crouch: p.crouch, fly: p.fly, mining: this.mining });
      if (this.loaded) this.fauna.update(dt, this.world, p, Math.max(0, this.r.sun.dir[1] * 3));
      this.weather.update(dt); this.audioStep(dt);
      if (this.timeRun) this.hour = (this.hour + (dt / this.dayLen) * 24) % 24;
      this.elapsed += dt;
      const v = this.vit; if (!this.creative) v.food = Math.max(0, v.food - dt * 0.012 * (I.sprint ? 2 : 1)); if (!this.creative) v.water = Math.max(0, v.water - dt * 0.018 * (I.sprint ? 2 : 1));
      if (p.inWater) v.water = Math.min(100, v.water + dt * 8);
      if ((this.tick || 0) % 30 === 0) this.hud.vitals(v);
    }
    this.world.update(p.p[0], p.p[2], this.world.local ? 1 : Math.max(2, this.world.workers.length * 2));
    if (!this.loaded) {
      const near = this.world.ready(); $('loadbar').style.width = Math.round(near * 100) + '%';
      if (this.world.nearReady(p.p[0], p.p[2]) && near > 0.3) { this.loaded = true; $('loading').classList.remove('on'); if (!CAPTURE && !this.paused) this.hud.toast('Cliquez pour jouer'); }
    }
    if (this.dirtySave) { this.saveT = (this.saveT || 0) + dt; if (this.saveT > 20) { this.save(false); this.saveT = 0; this.dirtySave = false; } }
  }

  // Tout ce qui blesse passe par ici : la musique de danger en dépend. Aucun danger n'existe encore dans le monde.
  hurt(n) { this.vit.hp = Math.max(0, this.vit.hp - n); this.hurtAt = this.elapsed; this.sound.hurt(); }
  soundNames() { return SOUNDS; } // pour tools/audio_test.py
  // Eau qui coule autour du joueur : les étangs sont silencieux, seuls ruisseaux et chutes s'entendent.
  scanWater() {
    const p = this.player.p, g = this.gen, X = Math.floor(p[0]), Z = Math.floor(p[2]); let bd = 1e9, fd = 1e9; this.brook = null; this.fall = null;
    for (let dz = -24; dz <= 24; dz += 2) for (let dx = -24; dx <= 24; dx += 2) {
      const x = X + dx, z = Z + dz, c = g.column(x, z), lvl = c.water, still = c.still; if (!lvl || still) continue;
      const d = dx * dx + dz * dz + (lvl - p[1]) ** 2;
      if (d < bd) { bd = d; this.brook = [x, lvl, z]; }
      if (d < fd) for (const [a, b] of [[2, 0], [-2, 0], [0, 2], [0, -2]]) { const w = g.column(x + a, z + b).water; if (w && lvl - w >= 3) { fd = d; this.fall = [x, lvl - 2, z]; break; } }
    }
  }
  // Sons : ambiance, pas selon le bloc, musique selon la situation.
  audioStep(dt) {
    const pl = this.player, p = pl.p, s = this.sound, w = this.weather.state, eye = [p[0], p[1] + 1.6, p[2]], sunY = this.r.sun.dir[1];
    s.listener(eye, pl.yaw);
    this.waterT = (this.waterT || 0) - dt;
    if (this.waterT <= 0) { this.waterT = 0.5; this.scanWater(); }
    const covered = this.underRoof(eye);
    s.env({ cloud: w.cloud, rain: w.rain, covered, brook: this.brook, fall: this.fall });
    const below = this.world.get(Math.floor(p[0]), Math.floor(p[1] - 0.1), Math.floor(p[2])), feet = this.world.get(Math.floor(p[0]), Math.floor(p[1] + 0.1), Math.floor(p[2]));
    const nameOf = (b) => (b > 0 ? BLOCKS[b].name : null);
    if (pl.ground && pl.speed > 1) {
      this.stepT = (this.stepT || 0) - dt * pl.speed;
      if (this.stepT <= 0) { this.stepT = 2.1; s.step(nameOf(below), feet !== below ? nameOf(feet) : null); }
    } else if (pl.inWater && pl.speed > 1) { this.stepT = (this.stepT || 0) - dt * pl.speed; if (this.stepT <= 0) { this.stepT = 2.6; s.step('water'); } }
    if (pl.ground && !this.wasGround && this.fallV < -7 && !pl.fly) s.land(nameOf(below));
    this.wasGround = pl.ground; this.fallV = pl.v[1] < -1 ? pl.v[1] : (pl.ground ? this.fallV * 0.5 : this.fallV);
    this.built = this.built.filter((t) => this.elapsed - t < 25);
    this.mood = pickMood({ ...this.vit, hurt: this.elapsed - (this.hurtAt ?? -99) < 12, enemy: this.enemyNear, submerged: this.world.get(Math.floor(eye[0]), Math.floor(eye[1]), Math.floor(eye[2])) === B.water, underground: p[1] < this.gen.column(p[0], p[2]).h - 4 && covered, building: this.built.length >= 3, night: sunY < 0, rain: w.rain > 0.5 });
    if (s.music) { s.music.forced = this.settings.mood || 'auto'; s.music.update(dt, this.mood); }
  }

  // Sous un toit ou dans une grotte, il ne pleut pas (le feuillage laisse passer).
  underRoof(e) {
    const x = Math.floor(e[0]), z = Math.floor(e[2]), y0 = Math.floor(e[1]) + 1;
    for (let y = y0; y < y0 + 24; y++) { const b = this.world.get(x, y, z); if (b > 0 && BLOCKS[b].cat !== 'foliage' && BLOCKS[b].cat !== 'fluid') return true; }
    return false;
  }

  draw(dt) {
    const r = this.r, p = this.player, { eye, f } = this.camera(), fov = (this.settings.fov * Math.PI) / 180;
    this.dtSmooth = dt;
    r.setSun(this.hour); r.applyWeather(this.weather.state);
    const proj = M.persp(fov, r.W / r.H, 0.08, 4200), viewM = M.look(eye, V.add(eye, f), [0, 1, 0]), vp = M.mul(proj, viewM);
    const planes = M.frustum(vp), all = [], visible = [];
    for (const c of this.world.chunks.values()) {
      if (!c.ready || !c.opaque) continue;
      all.push(c);
      let inside = true;
      for (const pl of planes) { const px = c.org[0] + (pl[0] > 0 ? 32 : 0), py = pl[1] > 0 ? c.ymax : c.ymin, pz = c.org[2] + (pl[2] > 0 ? 32 : 0); if (pl[0] * px + pl[1] * py + pl[2] * pz + pl[3] < 0) { inside = false; break; } }
      if (inside) { c.dist = Math.hypot(c.wx - eye[0], c.wz - eye[2]); visible.push(c); }
    }
    visible.sort((a, b) => a.dist - b.dist);
    for (const c of all) { c.cx_ = c.cx; }
    // rayon de visée : passe au-dessus de l'épaule, ignore ce qui est derrière le héros
    const head = [p.p[0], p.p[1] + 1.5, p.p[2]], rt = raycast(this.world, eye, f, this.creative ? 13 : 9.5, this.view.third ? (x, y, z) => V.dot(V.sub([x + 0.5, y + 0.5, z + 0.5], head), f) < -0.5 : null);
    const reach = rt && Math.hypot(rt.x + 0.5 - head[0], rt.y + 0.5 - head[1], rt.z + 0.5 - head[2]) < (this.creative ? 10 : 6.2) ? rt : null;
    this.target = reach; this.mineStep(dt);
    const heldIt = this.inv[this.sel], held = heldIt ? this.items[heldIt.item] : null;
    const actors = (this.view.third ? this.anim.actors(p.p[0], p.p[1], p.p[2], p.speed > 0.2 ? p.moveYaw : p.yaw, p.pitch, held) : this.anim.viewActors(M.inv(viewM), held)).concat(this.fauna.actors());
    const mc = reach ? Math.min(1, this.mine.t / Math.max(0.05, BLOCKS[reach.id].hardness)) : 0;
    r.render({ eye, vp, viewM, proj, chunks: all.map(remap), visible: visible.map(remap), far: this.world.far, actors, time: this.elapsed + (this.fixTime || 0), target: reach && !this.camOverride ? { pos: [reach.x, reach.y, reach.z], color: [mc, 1 - mc * 0.6, 1 - mc, 1] } : null, farHole: (this.world.radius - 0.6) * 32, covered: this.underRoof(eye), ghost: this.ghost(), cloud: this.weather.state.cloud, lightDirty: r.lightDirty, taa: this.taaOff ? false : undefined });
    r.lightDirty = false;
    $('reticle').classList.toggle('hit', !!reach);
    this.hud.compass(p.yaw);
  }

  report() {
    const b = this.gen.column(Math.floor(this.player.p[0]), Math.floor(this.player.p[2]));
    const lab = b.forest > 0.4 ? 'Forêt tempérée' : 'Clairière tempérée'; if ($('biomeName').textContent !== lab) $('biomeName').textContent = lab;
    if (!this.prof.show) return;
    const s = this.r.stats, w = this.world, mem = performance.memory ? (performance.memory.usedJSHeapSize / 1048576).toFixed(0) + ' Mo JS' : 'n/d';
    let vram = 0, ncu = 0; for (const c of w.chunks.values()) { ncu++; for (const m of [c.opaque, c.deco, c.water]) if (m) vram += m.bytes; }
    $('prof').textContent = `${this.prof.fps.toFixed(0)} FPS   CPU ${this.prof.cpu.toFixed(1)} ms   GPU ${s.gpu ? s.gpu.toFixed(1) + ' ms' : 'n/d'}\nTriangles ${(s.tris / 1e6).toFixed(2)} M   Draw calls ${s.draws}\nChunks ${ncu}, file ${w.queued || 0}, méthode ${w.local ? 'fil principal' : w.workers.length + ' workers'}\nMeshing+génération ${w.stats.genMs.toFixed(1)} ms par chunk\nMémoire ${mem}, géométrie ${(vram / 1048576).toFixed(0)} Mo\nRésolution ${this.r.W}×${this.r.H}, échelle auto ${Math.round((this.dynScale || 1) * 100)} %\nPosition ${this.player.p.map((v) => v.toFixed(1)).join(' ')}   ${this.hour.toFixed(1)} h   seed ${this.seed}`;
  }
}
const remap = (c) => ({ org: c.org, opaque: c.opaque, deco: c.deco, water: c.water, cx: c.wx, cz: c.wz });

try {
  const g = new Game();
  window.ALW = g; // point d'accès pour les tests automatisés et les captures
  g.restoreRemote();
} catch (e) {
  document.getElementById('fatal').textContent = 'Impossible de démarrer : ' + e.message;
  document.getElementById('fatal').classList.add('on');
  console.error(e);
}
