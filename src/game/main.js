// Gameplay/main.js — boucle de jeu, entrées, sauvegarde, profilage.
import { M, V } from '../core/math.js';
import { clamp } from '../core/noise.js';
import { Renderer, PRESETS } from '../render/renderer.js';
import { ClientWorld } from './client.js';
import { Player, raycast } from './player.js';
import { buildCharacter, CharacterAnim } from './character.js';
import { HUD, ITEMS, $ } from './ui.js';
import { BLOCKS, B, IS_SOLID, IS_DECO } from '../world/blocks.js';
import { WorldGen, GEN_VERSION } from '../world/gen.js';
import { serialize, safeRead, atomicWrite, deserialize } from './save.js';

const params = new URLSearchParams(location.hash.slice(1));
const CAPTURE = params.has('capture');
const SLOT_KEY = 'alw.world.';
const storage = (() => { try { const s = window.localStorage; s.setItem('alw.t', '1'); s.removeItem('alw.t'); return s; } catch (e) { return null; } })();

const DEFAULT_INV = [{ item: 'pickaxe' }, { item: 'axe' }, { item: 'sword' }, { item: 'lantern', n: 32 }, { item: 'dirt', n: 64 }, { item: 'stone', n: 32 }, { item: 'planks', n: 16 }, { item: 'oak_log', n: 0 }, { item: 'birch_log', n: 0 }];

class Game {
  constructor() {
    this.canvas = $('gl');
    this.r = new Renderer(this.canvas);
    this.hud = new HUD(this.r.texSource);
    this.settings = { preset: params.get('q') || 'moyen', scale: 0, fov: 60, sens: 1, radius: 9 };
    try { Object.assign(this.settings, JSON.parse(storage?.getItem('alw.settings') || '{}')); } catch (e) { /* réglages par défaut */ }
    if (params.get('q')) this.settings.preset = params.get('q');
    if (params.get('r')) this.settings.radius = +params.get('r');
    this.input = { keys: new Set(), x: 0, z: 0, jump: false, sprint: false, crouch: false, lmb: false, rmb: false, touch: { x: 0, z: 0 } };
    this.view = { third: true, dist: 4.2, camDist: 4.2 };
    this.charModel = buildCharacter(this.r); this.anim = new CharacterAnim(this.charModel);
    this.prof = { fps: 60, cpu: 0, show: CAPTURE ? false : false, frames: [] };
    this.hour = 16.4; this.dayLen = 1500; this.timeRun = !CAPTURE; this.elapsed = 0; this.mine = { key: '', t: 0 };
    this.vit = { hp: 100, food: 100, water: 100 };
    this.bindInput(); this.bindMenu();
    this.applySettings();
    const seed = Number(params.get('seed') || 20261104);
    const saved = !CAPTURE && storage ? safeRead(storage, SLOT_KEY + 'last') : null;
    this.startWorld(saved ? saved.seed : seed, saved);
    this.last = performance.now();
    requestAnimationFrame((t) => this.loop(t));
  }

  startWorld(seed, save) {
    if (this.world) { for (const w of this.world.workers) w.terminate(); for (const c of this.world.chunks.values()) { this.r.freeMesh(c.opaque); this.r.freeMesh(c.deco); this.r.freeMesh(c.water); } if (this.world.far) this.r.freeMesh(this.world.far.mesh); }
    const edits = new Map(save ? save.edits.map(([k, e]) => [k, new Map(e)]) : []);
    this.seed = seed; this.worldId = save?.id || 'w' + seed; this.gen = new WorldGen(seed);
    const workers = Math.max(1, Math.min(4, (navigator.hardwareConcurrency || 4) - 1));
    this.world = new ClientWorld(this.r, seed, edits, { radius: this.settings.radius, workers });
    this.inv = save?.inventory || DEFAULT_INV.map((a) => ({ ...a })); this.sel = 0;
    if (save) { this.hour = save.time; this.player = new Player(...save.player.p); this.player.yaw = save.player.yaw; this.player.pitch = save.player.pitch; this.player.moveYaw = save.player.yaw; this.vit = save.player.vit || this.vit; }
    else { const s = this.findSpawn(); this.player = new Player(s[0], s[1], s[2]); this.player.yaw = s[3]; this.player.moveYaw = s[3]; this.player.pitch = -0.13; this.hour = 16.4; }
    this.loaded = false; $('loading').classList.add('on'); this.r.reset = true;
    this.hud.inventory(this.inv, this.sel);
  }

  // Cherche un chemin en forêt : c'est le point de vue le plus proche de la référence.
  findSpawn() {
    const g = this.gen; let best = null, score = -1;
    for (let r = 0; r < 1600; r += 13) for (let a = 0; a < 6.283; a += 0.45) {
      const x = Math.round(Math.cos(a) * r), z = Math.round(Math.sin(a) * r), c = g.column(x, z);
      if (c.path < 0.7 || c.water) continue;
      const y = c.h, yy = Math.atan2(0.75, 0.2), bh = g.column(x - Math.sin(yy) * 4.5, z - Math.cos(yy) * 4.5).h;
      if (bh > y + 0.6) continue; let f = c.forest * 2, flat = 0;
      for (const [dx, dz] of [[24, 0], [-24, 0], [0, 24], [0, -24], [17, 17], [-17, -17], [17, -17], [-17, 17]]) { const n = g.column(x + dx, z + dz); f += n.forest; flat += Math.abs(n.h - y); }
      let wet = 0; for (const [dx, dz] of [[14, 0], [-14, 0], [0, 14], [0, -14], [10, 10], [-10, -10], [10, -10], [-10, 10]]) if (g.column(x + dx, z + dz).water) wet = 1;
      const s = f / 10 - flat * 0.006 - r / 9000 + wet * 0.35;
      if (s > score) { score = s; best = [x + 0.5, Math.ceil(c.h) + 2, z + 0.5, Math.atan2(0.75, 0.2)]; }
    }
    return best || [0.5, Math.ceil(g.column(0, 0).h) + 3, 0.5, 0];
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
      if (e.code === 'Escape' && !document.pointerLockElement) { this.toggleMenu(); return; }
      if (this.paused) return;
      K.add(e.code);
      if (e.code.startsWith('Digit')) { const n = +e.code.slice(5); if (n >= 1) this.select(n - 1); }
      if (e.code === 'KeyC') { this.view.third = !this.view.third; this.hud.toast(this.view.third ? 'Troisième personne' : 'Première personne'); }
      if (e.code === 'F3') { e.preventDefault(); this.prof.show = !this.prof.show; $('prof').classList.toggle('on', this.prof.show); }
      if (e.code === 'KeyT') this.hour = (this.hour + 1) % 24;
      if (e.code === 'KeyY') this.timeRun = !this.timeRun;
      if (e.code === 'F5') { e.preventDefault(); this.save(true); }
      if (['Space', 'ArrowUp', 'ArrowDown'].includes(e.code)) e.preventDefault();
    });
    addEventListener('keyup', (e) => K.delete(e.code));
    addEventListener('blur', () => { K.clear(); this.input.lmb = this.input.rmb = false; });
    const look = (dx, dy) => { const s = 0.0023 * this.settings.sens, p = this.player; p.yaw -= dx * s; p.pitch = clamp(p.pitch - dy * s, -1.5, 1.5); };
    document.addEventListener('pointerlockchange', () => { if (!document.pointerLockElement && !this.paused && this.lockedOnce && !this.dragMode) this.toggleMenu(true); });
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
  }

  bindMenu() {
    $('btnResume').onclick = () => this.toggleMenu(false);
    $('btnSave').onclick = () => this.save(true);
    $('btnExport').onclick = () => this.exportSave();
    $('btnImport').onclick = () => $('fileImport').click();
    $('fileImport').onchange = async (e) => { const f = e.target.files[0]; if (!f) return; try { const s = deserialize(await f.text()); this.startWorld(s.seed, s); this.toggleMenu(false); this.hud.toast('Monde importé'); } catch (x) { this.hud.toast(x.message); } };
    $('btnNew').onclick = () => { const v = Number($('inSeed').value) || Math.floor(Math.random() * 1e9); this.startWorld(v, null); $('inSeed').value = v; this.toggleMenu(false); };
    const sel = $('selPreset'), fov = $('inFov'), sens = $('inSens');
    sel.value = this.settings.preset; fov.value = this.settings.fov; sens.value = this.settings.sens;
    sel.onchange = () => { this.settings.preset = sel.value; this.dynScale = 1; this.applySettings(); };
    fov.oninput = () => { this.settings.fov = +fov.value; this.applySettings(); };
    sens.oninput = () => { this.settings.sens = +sens.value; this.applySettings(); };
  }
  toggleMenu(v) {
    this.paused = v === undefined ? !this.paused : v; $('menu').classList.toggle('on', this.paused);
    if (this.paused) { if (document.pointerLockElement) document.exitPointerLock(); $('inSeed').value = this.seed; }
  }
  select(i) { this.sel = i; this.hud.inventory(this.inv, i); }

  saveState() {
    return { id: this.worldId, seed: this.seed, genVersion: GEN_VERSION, time: this.hour, day: 1, player: { p: this.player.p, yaw: this.player.yaw, pitch: this.player.pitch, vit: this.vit }, inventory: this.inv, edits: [...this.world.edits].map(([k, m]) => [k, [...m]]) };
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

  place() {
    if (this.paused || !this.target) return;
    const it = this.inv[this.sel], def = it && ITEMS[it.item];
    if (!def?.block || it.n <= 0) return;
    const t = this.target, id = IS_DECO[t.id] && t.id !== B.lantern ? [t.x, t.y, t.z] : [t.px, t.py, t.pz];
    const cur = this.world.get(...id); if (cur === -1 || IS_SOLID[cur]) return;
    const p = this.player.p, bid = def.block;
    if (IS_SOLID[bid] && id[0] + 1 > p[0] - 0.3 && id[0] < p[0] + 0.3 && id[2] + 1 > p[2] - 0.3 && id[2] < p[2] + 0.3 && id[1] + 1 > p[1] && id[1] < p[1] + 1.8) return;
    if (this.world.set(id[0], id[1], id[2], bid)) { it.n--; this.hud.inventory(this.inv, this.sel); this.dirtySave = true; }
  }
  mineStep(dt) {
    const go = this.input.lmb || this.autoMine > 0; if (this.autoMine > 0) this.autoMine -= dt;
    this.mining = false;
    if (!go || !this.target || this.paused) { this.mine.key = ''; this.mine.t = 0; return; }
    const t = this.target, key = t.x + ',' + t.y + ',' + t.z, b = BLOCKS[t.id], it = this.inv[this.sel], def = it && ITEMS[it.item];
    if (key !== this.mine.key) { this.mine.key = key; this.mine.t = 0; }
    this.mining = true;
    const rate = def?.tool && def.tool === b.cat ? 3 : b.cat === 'terrain' ? 1.6 : 1;
    this.mine.t += dt * rate;
    if (this.mine.t >= b.hardness) {
      if (this.world.set(t.x, t.y, t.z, 0)) {
        const up = this.world.get(t.x, t.y + 1, t.z); if (up > 0 && IS_DECO[up]) this.world.set(t.x, t.y + 1, t.z, 0);
        if (b.drop) { const s = this.inv.find((i) => i && i.item === b.drop); if (s) { s.n++; this.hud.inventory(this.inv, this.sel); } }
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
      if (this.timeRun) this.hour = (this.hour + (dt / this.dayLen) * 24) % 24;
      this.elapsed += dt;
      const v = this.vit; v.food = Math.max(0, v.food - dt * 0.012 * (I.sprint ? 2 : 1)); v.water = Math.max(0, v.water - dt * 0.018 * (I.sprint ? 2 : 1));
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

  draw(dt) {
    const r = this.r, p = this.player, { eye, f } = this.camera(), fov = (this.settings.fov * Math.PI) / 180;
    this.dtSmooth = dt;
    r.setSun(this.hour);
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
    const head = [p.p[0], p.p[1] + 1.5, p.p[2]], rt = raycast(this.world, eye, f, 9.5, this.view.third ? (x, y, z) => V.dot(V.sub([x + 0.5, y + 0.5, z + 0.5], head), f) < -0.5 : null);
    const reach = rt && Math.hypot(rt.x + 0.5 - head[0], rt.y + 0.5 - head[1], rt.z + 0.5 - head[2]) < 6.2 ? rt : null;
    this.target = reach; this.mineStep(dt);
    const actors = this.view.third ? this.anim.actors(p.p[0], p.p[1], p.p[2], this.player.speed > 0.2 ? p.moveYaw : p.yaw, p.speed, dt, this.mining, p.crouch) : [];
    const mc = reach ? Math.min(1, this.mine.t / Math.max(0.05, BLOCKS[reach.id].hardness)) : 0;
    r.render({ eye, vp, viewM, proj, chunks: all.map(remap), visible: visible.map(remap), far: this.world.far, actors, time: this.elapsed + (this.fixTime || 0), target: reach ? { pos: [reach.x, reach.y, reach.z], color: [mc, 1 - mc * 0.6, 1 - mc, 1] } : null, farHole: (this.settings.radius - 0.6) * 32, cloud: 0.6, lightDirty: r.lightDirty, taa: this.taaOff ? false : undefined });
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
    $('prof').textContent = `${this.prof.fps.toFixed(0)} FPS   CPU ${this.prof.cpu.toFixed(1)} ms   GPU ${s.gpu ? s.gpu.toFixed(1) + ' ms' : 'n/d'}\nTriangles ${(s.tris / 1e6).toFixed(2)} M   Draw calls ${s.draws}\nChunks ${ncu}, file ${w.queued || 0}, méthode ${w.local ? 'fil principal' : w.workers.length + ' workers'}\nMeshing+génération ${w.stats.genMs.toFixed(1)} ms par chunk\nMémoire ${mem}, géométrie ${(vram / 1048576).toFixed(0)} Mo\nRésolution ${this.r.W}×${this.r.H}\nPosition ${this.player.p.map((v) => v.toFixed(1)).join(' ')}   ${this.hour.toFixed(1)} h   seed ${this.seed}`;
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
