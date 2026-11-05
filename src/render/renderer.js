// Rendering/renderer.js — pipeline différé WebGL2 : ombres en cascades, G-buffer, SSAO, éclairage,
// nuages, brume volumétrique, eau, perspective aérienne, TAA, bloom, tonemapping.
import { M, V } from '../core/math.js';
import { buildTextures, buildNoise3D } from './textures.js';
import * as S from './shaders.js';
import { VSIZE } from '../world/mesher.js';

export const PRESETS = {
  bas: { shadow: 1024, samples: 4, vol: 6, clouds: false, ssao: true, scale: 0.7, radius: 6, lod: [1.3, 3.2], shadowStep: [3, 6, 12], shadowOff: [0, 1, 2], decoShadow: 0, cloudStep: 1 },
  moyen: { shadow: 2048, samples: 6, vol: 10, clouds: true, ssao: true, scale: 0.85, radius: 8, lod: [1.8, 4], shadowStep: [2, 4, 8], shadowOff: [0, 1, 7], decoShadow: 34, cloudStep: 2 },
  haut: { shadow: 3072, samples: 8, vol: 14, clouds: true, ssao: true, scale: 1, radius: 10, lod: [2.3, 4.6], shadowStep: [1, 2, 4], shadowOff: [0, 1, 2], decoShadow: 52, cloudStep: 1 },
};
const MAX_QUADS = 500000;
const CASC = [26, 90, 300];

export class Renderer {
  constructor(canvas, opts = {}) {
    this.texSize = opts.texSize || 128;
    const gl = canvas.getContext('webgl2', { antialias: false, depth: false, stencil: false, alpha: false, powerPreference: 'high-performance', preserveDrawingBuffer: false });
    if (!gl) throw new Error('WebGL2 indisponible sur ce navigateur.');
    if (!gl.getExtension('EXT_color_buffer_float') && !gl.getExtension('EXT_color_buffer_half_float')) throw new Error('Rendu HDR flottant indisponible sur cette carte graphique.');
    this.gl = gl; this.canvas = canvas; this.frame = 0; this.time = 0; this.stats = { draws: 0, tris: 0, gpu: 0 };
    this.aniso = gl.getExtension('EXT_texture_filter_anisotropic');
    this.timer = gl.getExtension('EXT_disjoint_timer_query_webgl2'); this.q = null;
    this.preset = PRESETS.moyen; this.fog = 0.0048; this.exposure = 0.52;
    this.sun = { dir: [0.4, 0.6, 0.3], col: [3, 3, 3], top: [0.2, 0.4, 0.8], hor: [0.6, 0.7, 0.9], ground: [0.2, 0.2, 0.15] };
    this.buildPrograms(); this.buildStatic(); this.prevVP = M.id();
  }

  prog(vs, fs) {
    const gl = this.gl, sh = (t, s) => { const o = gl.createShader(t); gl.shaderSource(o, s); gl.compileShader(o); if (!gl.getShaderParameter(o, gl.COMPILE_STATUS)) throw new Error(gl.getShaderInfoLog(o) + '\n' + s.split('\n').map((l, i) => i + 1 + ': ' + l).slice(0, 8).join('\n')); return o; };
    const p = gl.createProgram(); gl.attachShader(p, sh(gl.VERTEX_SHADER, vs)); gl.attachShader(p, sh(gl.FRAGMENT_SHADER, fs)); gl.linkProgram(p);
    if (!gl.getProgramParameter(p, gl.LINK_STATUS)) throw new Error(gl.getProgramInfoLog(p));
    const u = {}, n = gl.getProgramParameter(p, gl.ACTIVE_UNIFORMS);
    for (let i = 0; i < n; i++) { const a = gl.getActiveUniform(p, i), name = a.name.replace('[0]', ''); u[name] = { loc: gl.getUniformLocation(p, a.name), type: a.type }; }
    return { p, u };
  }
  use(pr, vals, texs) {
    const gl = this.gl; gl.useProgram(pr.p);
    if (vals) for (const k in vals) this.set(pr, k, vals[k]);
    if (texs) { let i = 0; for (const k in texs) { const t = texs[k]; if (!pr.u[k]) continue; gl.activeTexture(gl.TEXTURE0 + i); const y = pr.u[k].type; gl.bindTexture(y === gl.SAMPLER_2D_ARRAY || y === gl.SAMPLER_2D_ARRAY_SHADOW ? gl.TEXTURE_2D_ARRAY : y === gl.SAMPLER_3D ? gl.TEXTURE_3D : gl.TEXTURE_2D, t); gl.uniform1i(pr.u[k].loc, i); i++; } }
  }
  set(pr, k, v) {
    const u = pr.u[k]; if (!u) return; const gl = this.gl;
    switch (u.type) {
      case gl.FLOAT: gl.uniform1f(u.loc, v); break; case gl.INT: gl.uniform1i(u.loc, v); break;
      case gl.FLOAT_VEC2: gl.uniform2fv(u.loc, v); break; case gl.FLOAT_VEC3: gl.uniform3fv(u.loc, v); break; case gl.FLOAT_VEC4: gl.uniform4fv(u.loc, v); break;
      case gl.FLOAT_MAT4: gl.uniformMatrix4fv(u.loc, false, v); break;
    }
  }

  buildPrograms() {
    const F = S.VS_FULL;
    this.P = {
      geom: this.prog(S.VS_GEOM, S.FS_GEOM), shadow: this.prog(S.VS_SHADOW, S.FS_SHADOW), light: this.prog(F, S.FS_LIGHT), ssao: this.prog(F, S.FS_SSAO), blur: this.prog(F, S.FS_BLUR),
      clouds: this.prog(F, S.FS_SKYCLOUD), vol: this.prog(F, S.FS_VOLUME), water: this.prog(S.VS_WATER, S.FS_WATER), compose: this.prog(F, S.FS_COMPOSE), taa: this.prog(F, S.FS_TAA),
      bright: this.prog(F, S.FS_BRIGHT), final: this.prog(F, S.FS_FINAL), line: this.prog(S.VS_LINE, S.FS_LINE),
    };
  }

  buildStatic() {
    const gl = this.gl;
    this.emptyVAO = gl.createVertexArray();
    const idx = new Uint32Array(MAX_QUADS * 6);
    for (let i = 0; i < MAX_QUADS; i++) { const a = i * 6, v = i * 4; idx[a] = v; idx[a + 1] = v + 1; idx[a + 2] = v + 2; idx[a + 3] = v; idx[a + 4] = v + 2; idx[a + 5] = v + 3; }
    this.ebo = gl.createBuffer();
    gl.bindVertexArray(this.emptyVAO); gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.ebo); gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, idx, gl.STATIC_DRAW); gl.bindVertexArray(null);
    const T = buildTextures(this.texSize), S2 = T.size, mk = (data) => {
      const t = gl.createTexture(); gl.bindTexture(gl.TEXTURE_2D_ARRAY, t);
      gl.texImage3D(gl.TEXTURE_2D_ARRAY, 0, gl.RGBA8, S2, S2, T.layers, 0, gl.RGBA, gl.UNSIGNED_BYTE, data); gl.generateMipmap(gl.TEXTURE_2D_ARRAY);
      gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_MAG_FILTER, gl.NEAREST); gl.texParameteri(gl.TEXTURE_2D_ARRAY, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
      if (this.aniso) gl.texParameterf(gl.TEXTURE_2D_ARRAY, this.aniso.TEXTURE_MAX_ANISOTROPY_EXT, 8);
      return t;
    };
    this.tAlbedo = mk(T.albedo); this.tNormal = mk(T.nrm); this.texSource = T;
    const N = buildNoise3D(); this.tNoise = gl.createTexture(); gl.bindTexture(gl.TEXTURE_3D, this.tNoise);
    gl.pixelStorei(gl.UNPACK_ALIGNMENT, 1);
    gl.texImage3D(gl.TEXTURE_3D, 0, gl.R8, N.n, N.n, N.n, 0, gl.RED, gl.UNSIGNED_BYTE, N.data); gl.generateMipmap(gl.TEXTURE_3D);
    gl.pixelStorei(gl.UNPACK_ALIGNMENT, 4);
    for (const p of [gl.TEXTURE_WRAP_S, gl.TEXTURE_WRAP_T, gl.TEXTURE_WRAP_R]) gl.texParameteri(gl.TEXTURE_3D, p, gl.REPEAT);
    gl.texParameteri(gl.TEXTURE_3D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR); gl.texParameteri(gl.TEXTURE_3D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    // cube de sélection
    const L = [0,0,0,1,0,0,1,0,0,1,1,0,1,1,0,0,1,0,0,1,0,0,0,0,0,0,1,1,0,1,1,0,1,1,1,1,1,1,1,0,1,1,0,1,1,0,0,1,0,0,0,0,0,1,1,0,0,1,0,1,1,1,0,1,1,1,0,1,0,0,1,1];
    const Q = [[0,0,1,1,0,1,1,1,1,0,0,1,1,1,1,0,1,1],[1,0,0,0,0,0,0,1,0,1,0,0,0,1,0,1,1,0],[0,1,1,1,1,1,1,1,0,0,1,1,1,1,0,0,1,0],[0,0,0,1,0,0,1,0,1,0,0,0,1,0,1,0,0,1],[1,0,1,1,0,0,1,1,0,1,0,1,1,1,0,1,1,1],[0,0,0,0,0,1,0,1,1,0,0,0,0,1,1,0,1,0]].flat();
    this.cubeVAO = gl.createVertexArray(); gl.bindVertexArray(this.cubeVAO); const cb = gl.createBuffer(); gl.bindBuffer(gl.ARRAY_BUFFER, cb); gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(Q), gl.STATIC_DRAW); gl.enableVertexAttribArray(0); gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 0, 0); gl.bindVertexArray(null);
    this.lineVAO = gl.createVertexArray(); gl.bindVertexArray(this.lineVAO); const b = gl.createBuffer(); gl.bindBuffer(gl.ARRAY_BUFFER, b); gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(L), gl.STATIC_DRAW); gl.enableVertexAttribArray(0); gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 0, 0); gl.bindVertexArray(null);
  }

  colorTex(w, h, ifmt, fmt, type, filter = 'LINEAR') {
    const gl = this.gl, t = gl.createTexture(); gl.bindTexture(gl.TEXTURE_2D, t); gl.texImage2D(gl.TEXTURE_2D, 0, ifmt, w, h, 0, fmt, type, null);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl[filter]); gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl[filter]);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE); gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE); return t;
  }
  fbo(colors, depth) {
    const gl = this.gl, f = gl.createFramebuffer(); gl.bindFramebuffer(gl.FRAMEBUFFER, f);
    colors.forEach((t, i) => gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0 + i, gl.TEXTURE_2D, t, 0));
    if (depth) gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.TEXTURE_2D, depth, 0);
    gl.drawBuffers(colors.map((_, i) => gl.COLOR_ATTACHMENT0 + i));
    const st = gl.checkFramebufferStatus(gl.FRAMEBUFFER); if (st !== gl.FRAMEBUFFER_COMPLETE) throw new Error('FBO incomplet ' + st);
    return f;
  }

  resize(cssW, cssH, scale) {
    const gl = this.gl, W = Math.max(2, Math.floor(cssW * scale)), H = Math.max(2, Math.floor(cssH * scale));
    if (W === this.W && H === this.H) return;
    this.W = W; this.H = H; this.canvas.width = W; this.canvas.height = H;
    const RGBA8 = [gl.RGBA8, gl.RGBA, gl.UNSIGNED_BYTE], F16 = [gl.RGBA16F, gl.RGBA, gl.HALF_FLOAT];
    if (this.res) { for (const t of this.res.tex) gl.deleteTexture(t); for (const f of this.res.fbo) gl.deleteFramebuffer(f); }
    const tex = [], fbo = [], T = (w, h, f, filter) => { const t = this.colorTex(w, h, f[0], f[1], f[2], filter); tex.push(t); return t; }, B = (c, d) => { const f = this.fbo(c, d); fbo.push(f); return f; };
    const depthTex = () => { const t = gl.createTexture(); gl.bindTexture(gl.TEXTURE_2D, t); gl.texImage2D(gl.TEXTURE_2D, 0, gl.DEPTH_COMPONENT32F, W, H, 0, gl.DEPTH_COMPONENT, gl.FLOAT, null); for (const [p, v] of [[gl.TEXTURE_MIN_FILTER, gl.NEAREST], [gl.TEXTURE_MAG_FILTER, gl.NEAREST], [gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE], [gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE]]) gl.texParameteri(gl.TEXTURE_2D, p, v); tex.push(t); return t; };
    const r = this.r = {};
    r.A = T(W, H, RGBA8, 'NEAREST'); r.N = T(W, H, RGBA8, 'NEAREST'); r.L = T(W, H, RGBA8, 'NEAREST'); r.D = depthTex(); r.D2 = depthTex();
    r.gbuf = B([r.A, r.N, r.L], r.D);
    r.hdr = T(W, H, F16); r.hdrF = B([r.hdr], r.D); r.scene = T(W, H, F16); r.sceneF = B([r.scene], r.D2);
    const hw = W >> 1, hh = H >> 1, qw = W >> 2, qh = H >> 2;
    r.ao = T(hw, hh, RGBA8); r.aoF = B([r.ao]); r.ao2 = T(hw, hh, RGBA8); r.ao2F = B([r.ao2]);
    r.vol = T(hw, hh, F16); r.volF = B([r.vol]); r.cl = T(qw, qh, F16); r.clF = B([r.cl]);
    r.comp = T(W, H, F16); r.compF = B([r.comp]);
    r.hist = [T(W, H, F16), T(W, H, F16)]; r.histF = r.hist.map((t) => B([t]));
    r.bloom = []; r.bloomF = []; let bw = hw, bh = hh;
    for (let i = 0; i < 5; i++) { const t = T(Math.max(2, bw), Math.max(2, bh), F16); r.bloom.push(t); r.bloomF.push(B([t])); bw >>= 1; bh >>= 1; }
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    this.res = { tex, fbo }; this.reset = true;
  }

  setPreset(p) {
    const gl = this.gl; this.preset = p;
    if (this.shadowSize === p.shadow) return;
    this.shadowSize = p.shadow;
    if (this.tShadow) gl.deleteTexture(this.tShadow);
    this.tShadow = gl.createTexture(); gl.bindTexture(gl.TEXTURE_2D_ARRAY, this.tShadow);
    gl.texStorage3D(gl.TEXTURE_2D_ARRAY, 1, gl.DEPTH_COMPONENT24, p.shadow, p.shadow, 3);
    for (const [k, v] of [[gl.TEXTURE_MIN_FILTER, gl.LINEAR], [gl.TEXTURE_MAG_FILTER, gl.LINEAR], [gl.TEXTURE_COMPARE_MODE, gl.COMPARE_REF_TO_TEXTURE], [gl.TEXTURE_COMPARE_FUNC, gl.LEQUAL], [gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE], [gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE]]) gl.texParameteri(gl.TEXTURE_2D_ARRAY, k, v);
    if (!this.tActor) {
      this.tActor = gl.createTexture(); gl.bindTexture(gl.TEXTURE_2D, this.tActor);
      gl.texStorage2D(gl.TEXTURE_2D, 1, gl.DEPTH_COMPONENT24, 1024, 1024);
      for (const [k, v] of [[gl.TEXTURE_MIN_FILTER, gl.LINEAR], [gl.TEXTURE_MAG_FILTER, gl.LINEAR], [gl.TEXTURE_COMPARE_MODE, gl.COMPARE_REF_TO_TEXTURE], [gl.TEXTURE_COMPARE_FUNC, gl.LEQUAL], [gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE], [gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE]]) gl.texParameteri(gl.TEXTURE_2D, k, v);
      this.actorF = gl.createFramebuffer(); gl.bindFramebuffer(gl.FRAMEBUFFER, this.actorF); gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.TEXTURE_2D, this.tActor, 0); gl.drawBuffers([gl.NONE]); gl.readBuffer(gl.NONE); gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    }
    this.shadowF = this.shadowF || gl.createFramebuffer(); this.shadowVP = [M.id(), M.id(), M.id()]; this.shadowCenters = [null, null, null];
  }

  mesh(ab, count) {
    const gl = this.gl, vao = gl.createVertexArray(), vbo = gl.createBuffer();
    gl.bindVertexArray(vao); gl.bindBuffer(gl.ARRAY_BUFFER, vbo); gl.bufferData(gl.ARRAY_BUFFER, ab, gl.STATIC_DRAW);
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.ebo);
    gl.enableVertexAttribArray(0); gl.vertexAttribPointer(0, 4, gl.SHORT, false, VSIZE, 0);
    gl.enableVertexAttribArray(1); gl.vertexAttribPointer(1, 4, gl.BYTE, true, VSIZE, 8);
    gl.enableVertexAttribArray(2); gl.vertexAttribPointer(2, 4, gl.UNSIGNED_BYTE, true, VSIZE, 12);
    gl.enableVertexAttribArray(3); gl.vertexAttribPointer(3, 4, gl.UNSIGNED_BYTE, true, VSIZE, 16);
    gl.bindVertexArray(null);
    return { vao, vbo, count: Math.min(count / 4, MAX_QUADS) * 6, bytes: ab.byteLength };
  }
  freeMesh(m) { if (!m) return; this.gl.deleteVertexArray(m.vao); this.gl.deleteBuffer(m.vbo); }

  full(pr, fb, w, h, vals, texs) {
    const gl = this.gl; gl.bindFramebuffer(gl.FRAMEBUFFER, fb); gl.viewport(0, 0, w, h); this.use(pr, vals, texs);
    gl.bindVertexArray(this.emptyVAO); gl.drawArrays(gl.TRIANGLES, 0, 3); this.stats.draws++;
  }

  setSun(hour) {
    const a = ((hour - 6) / 12) * Math.PI, el = Math.sin(a), s = this.sun;
    const mix = (x, y, t) => [x[0] + (y[0] - x[0]) * t, x[1] + (y[1] - x[1]) * t, x[2] + (y[2] - x[2]) * t];
    const clampT = (v) => Math.min(1, Math.max(0, v));
    const d = V.norm([Math.cos(a) * 0.82, el * 0.9 + 0.02, 0.38 + 0.2 * (1 - el)]);
    const k = clampT((d[1] + 0.06) / 0.5), st = k * k * (3 - 2 * k), day = clampT((d[1] + 0.1) / 0.25), night = 1 - clampT((d[1] + 0.12) / 0.18);
    // La lune prend le relais du soleil : un seul astre porte les ombres, donc jamais de lumière par-dessous.
    const moonUp = clampT((-d[1] - 0.04) / 0.12);
    s.dir = d[1] < -0.04 ? V.norm([-d[0], -d[1] + 0.05, d[2]]) : d;
    s.col = d[1] < -0.04 ? [0.13 * moonUp, 0.18 * moonUp, 0.3 * moonUp] : mix([4.2, 1.9, 0.75], [6.2, 5.3, 3.7], st).map((c) => c * clampT((d[1] + 0.04) / 0.14));
    s.top = mix(mix([0.012, 0.02, 0.05], [0.19, 0.3, 0.62], day), [0.22, 0.46, 1.05], st);
    s.hor = mix(mix([0.03, 0.045, 0.09], [1.05, 0.62, 0.42], day), [0.82, 1.0, 1.25], st);
    s.ground = mix([0.02, 0.022, 0.03], [0.3, 0.3, 0.15], day);
    this.night = night; this.exposure = 0.52 + 0.4 * night;
  }

  // Icône d'inventaire rendue depuis le vrai modèle de l'objet.
  icon(item, S = 96) {
    const gl = this.gl;
    if (!this.iconF) {
      const mk = () => { const t = gl.createTexture(); gl.bindTexture(gl.TEXTURE_2D, t); gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, S, S, 0, gl.RGBA, gl.UNSIGNED_BYTE, null); return t; };
      this.iconT = [mk(), mk(), mk()]; const d = gl.createRenderbuffer(); gl.bindRenderbuffer(gl.RENDERBUFFER, d); gl.renderbufferStorage(gl.RENDERBUFFER, gl.DEPTH_COMPONENT24, S, S);
      this.iconF = gl.createFramebuffer(); gl.bindFramebuffer(gl.FRAMEBUFFER, this.iconF); this.iconT.forEach((t, i) => gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0 + i, gl.TEXTURE_2D, t, 0));
      gl.framebufferRenderbuffer(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.RENDERBUFFER, d); gl.drawBuffers([gl.COLOR_ATTACHMENT0, gl.COLOR_ATTACHMENT1, gl.COLOR_ATTACHMENT2]);
    }
    const tool = item.kind === 'tool', e = item.kind === 'block' ? [2.2, 1.9, 2.2] : tool ? [0, 0, 3] : [1.6, 1.5, 2.2], c = item.kind === 'block' ? [0, 0, 0] : tool ? [0, 0, 0] : [0, 0.05, 0];
    const k = item.kind === 'block' ? 0.95 : tool ? 0.62 : 0.74, off = item.kind === 'block' ? [-0.5, -0.5, -0.5] : item.kind === 'deco' ? [-0.5, -0.45, -0.5] : [-0.18, -0.18, 0];
    const vp = M.mul(M.ortho(-k, k, -k, k, -6, 6), M.look(e, c, [0, 1, 0])), G = this.P.geom;
    gl.bindFramebuffer(gl.FRAMEBUFFER, this.iconF); gl.viewport(0, 0, S, S); gl.clearColor(0, 0, 0, 0); gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT); gl.enable(gl.DEPTH_TEST); gl.disable(gl.CULL_FACE); gl.disable(gl.BLEND);
    this.use(G, { uVP: vp, uTime: 0, uModel: M.trs(off[0], off[1], off[2], 0), uPosScale: 1 / 128, uJitter: [0, 0], uDepthK: 1, uLocal: item.local ? 1 : 0, uHero: 0, uOrigin: [0, 0, 0], uFarHole: 0 }, { uAlbedo: this.tAlbedo, uNormal: this.tNormal, uNoise: this.tNoise });
    gl.bindVertexArray(item.mesh.vao); gl.drawElements(gl.TRIANGLES, item.count, gl.UNSIGNED_INT, item.offset);
    const a = new Uint8Array(S * S * 4), n = new Uint8Array(S * S * 4);
    gl.readBuffer(gl.COLOR_ATTACHMENT0); gl.readPixels(0, 0, S, S, gl.RGBA, gl.UNSIGNED_BYTE, a);
    gl.readBuffer(gl.COLOR_ATTACHMENT1); gl.readPixels(0, 0, S, S, gl.RGBA, gl.UNSIGNED_BYTE, n);
    const cv = document.createElement('canvas'); cv.width = cv.height = S; const ctx = cv.getContext('2d'), im = ctx.createImageData(S, S);
    for (let y = 0; y < S; y++) for (let x = 0; x < S; x++) {
      const i = (x + (S - 1 - y) * S) * 4, o = (x + y * S) * 4; if (!a[i + 3]) continue;
      const lit = 0.72 + 0.5 * Math.max(0, (n[i] / 127.5 - 1) * 0.3 + (n[i + 1] / 127.5 - 1) * 0.8 + (n[i + 2] / 127.5 - 1) * 0.5);
      for (let c2 = 0; c2 < 3; c2++) im.data[o + c2] = Math.min(255, 255 * Math.pow((a[i + c2] / 255) * lit * 1.2, 1 / 2.2));
      im.data[o + 3] = 255;
    }
    ctx.putImageData(im, 0, 0); gl.bindFramebuffer(gl.FRAMEBUFFER, null); gl.bindVertexArray(null); gl.enable(gl.CULL_FACE); gl.disable(gl.DEPTH_TEST);
    return cv.toDataURL();
  }

  render(view) {
    const gl = this.gl, r = this.r, st = this.stats, p = this.preset, W = this.W, H = this.H;
    st.draws = 0; st.tris = 0; this.frame++; const fr = this.frame;
    this.time = view.time;
    if (this.timer) { if (this.q && gl.getQueryParameter(this.q, gl.QUERY_RESULT_AVAILABLE)) { if (!gl.getParameter(this.timer.GPU_DISJOINT_EXT)) st.gpu = gl.getQueryParameter(this.q, gl.QUERY_RESULT) / 1e6; gl.deleteQuery(this.q); this.q = null; } if (!this.q) { this.q = gl.createQuery(); gl.beginQuery(this.timer.TIME_ELAPSED_EXT, this.q); this.qOn = true; } }
    const { eye, vp, viewM, proj, chunks, far, actors } = view;
    const invVP = M.inv(vp), invP = M.inv(proj), sun = this.sun;
    const hal = (i, b) => { let f = 1, x = 0; while (i > 0) { f /= b; x += f * (i % b); i = Math.floor(i / b); } return x; };
    const jit = view.taa === false ? [0, 0] : [(hal(fr % 8 + 1, 2) - 0.5) * 2 / W, (hal(fr % 8 + 1, 3) - 0.5) * 2 / H];
    gl.enable(gl.DEPTH_TEST); gl.depthFunc(gl.LEQUAL); gl.disable(gl.BLEND);

    /* --- ombres --- */
    gl.bindFramebuffer(gl.FRAMEBUFFER, this.shadowF); gl.viewport(0, 0, p.shadow, p.shadow); gl.colorMask(false, false, false, false); gl.drawBuffers([gl.NONE]);
    gl.enable(gl.CULL_FACE); gl.cullFace(gl.BACK);
    this.use(this.P.shadow, { uTime: view.time, uPosScale: 1 / 128, uModel: M.id() });
    const snap = (R, size) => {
      const up = Math.abs(sun.dir[1]) > 0.95 ? [1, 0, 0] : [0, 1, 0], texel = (2 * R) / size;
      const lv = M.look([0, 0, 0], V.mul(sun.dir, -1), up), c = eye;
      let lx = lv[0] * c[0] + lv[4] * c[1] + lv[8] * c[2], ly = lv[1] * c[0] + lv[5] * c[1] + lv[9] * c[2]; const lz = lv[2] * c[0] + lv[6] * c[1] + lv[10] * c[2];
      lx = Math.floor(lx / texel) * texel; ly = Math.floor(ly / texel) * texel;
      return M.mul(M.ortho(lx - R, lx + R, ly - R, ly + R, -lz - 420, -lz + 420), lv);
    };
    // Cascades échelonnées : en Moyen et Bas, jamais deux cascades sur la même image.
    const force = this.reset;
    for (let i = 0; i < 3; i++) {
      if (!((fr + p.shadowOff[i]) % p.shadowStep[i] === 0 || force || fr < 6)) continue;
      const sv = snap(CASC[i], p.shadow); this.shadowVP[i] = sv;
      gl.framebufferTextureLayer(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, this.tShadow, 0, i); gl.clear(gl.DEPTH_BUFFER_BIT);
      this.set(this.P.shadow, 'uVP', sv);
      const R = CASC[i] + 40;
      for (const c of chunks) {
        const dx = c.cx - eye[0], dz = c.cz - eye[2]; if (dx * dx + dz * dz > R * R) continue;
        this.set(this.P.shadow, 'uOrigin', c.org); this.set(this.P.shadow, 'uModel', M.id());
        gl.bindVertexArray(c.opaque.vao); gl.drawElements(gl.TRIANGLES, c.opaque.count, gl.UNSIGNED_INT, 0); st.draws++; st.tris += c.opaque.count / 3;
        if (i === 0 && c.deco && c.deco.count && dx * dx + dz * dz < p.decoShadow * p.decoShadow) { gl.disable(gl.CULL_FACE); gl.bindVertexArray(c.deco.vao); gl.drawElements(gl.TRIANGLES, c.deco.count, gl.UNSIGNED_INT, 0); gl.enable(gl.CULL_FACE); st.draws++; st.tris += c.deco.count / 3; }
      }
    }
    // Personnages : carte à part, à jour à chaque image. Sinon l'ombre de l'image précédente retombe sur le corps.
    this.actorVP = snap(18, 1024);
    gl.bindFramebuffer(gl.FRAMEBUFFER, this.actorF); gl.viewport(0, 0, 1024, 1024); gl.clear(gl.DEPTH_BUFFER_BIT);
    this.set(this.P.shadow, 'uVP', this.actorVP); this.set(this.P.shadow, 'uOrigin', [0, 0, 0]);
    for (const a of actors) { if (a.view) continue; this.set(this.P.shadow, 'uModel', a.model); gl.bindVertexArray(a.mesh.vao); gl.drawElements(gl.TRIANGLES, a.count, gl.UNSIGNED_INT, a.offset); st.draws++; }
    gl.colorMask(true, true, true, true);

    /* --- G-buffer --- */
    gl.bindFramebuffer(gl.FRAMEBUFFER, r.gbuf); gl.viewport(0, 0, W, H); gl.clearColor(0, 0, 0, 0); gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    const G = this.P.geom;
    this.use(G, { uVP: vp, uTime: view.time, uModel: M.id(), uPosScale: 1 / 128, uJitter: jit, uDepthK: 1, uLocal: 0, uHero: 0 }, { uAlbedo: this.tAlbedo, uNormal: this.tNormal, uNoise: this.tNoise });
    this.set(G, 'uOrigin', [0, 0, 0]);
    for (const c of view.visible) { this.set(G, 'uOrigin', c.org); gl.bindVertexArray(c.opaque.vao); gl.drawElements(gl.TRIANGLES, c.opaque.count, gl.UNSIGNED_INT, 0); st.draws++; st.tris += c.opaque.count / 3; }
    gl.disable(gl.CULL_FACE);
    for (const c of view.visible) if (c.deco && c.deco.count) { this.set(G, 'uOrigin', c.org); gl.bindVertexArray(c.deco.vao); gl.drawElements(gl.TRIANGLES, c.deco.count, gl.UNSIGNED_INT, 0); st.draws++; st.tris += c.deco.count / 3; }
    gl.enable(gl.CULL_FACE);
    this.set(G, 'uOrigin', [0, 0, 0]);
    gl.disable(gl.CULL_FACE);
    for (const a of actors) { this.set(G, 'uHero', a.hero ? 1 : 0); this.set(G, 'uLocal', a.local ? 1 : 0); this.set(G, 'uDepthK', a.view ? 0.05 : 1); this.set(G, 'uModel', a.model); gl.bindVertexArray(a.mesh.vao); gl.drawElements(gl.TRIANGLES, a.count, gl.UNSIGNED_INT, a.offset); st.draws++; st.tris += a.count / 3; }
    gl.enable(gl.CULL_FACE); this.set(G, 'uHero', 0); this.set(G, 'uLocal', 0); this.set(G, 'uDepthK', 1); this.set(G, 'uModel', M.id());
    if (far) { this.set(G, 'uPosScale', 1 / 8); this.set(G, 'uFarHole', view.farHole); this.set(G, 'uCamPos', eye); this.set(G, 'uOrigin', far.org); gl.bindVertexArray(far.mesh.vao); gl.drawElements(gl.TRIANGLES, far.mesh.count, gl.UNSIGNED_INT, 0); st.draws++; st.tris += far.mesh.count / 3; this.set(G, 'uFarHole', 0); }
    gl.disable(gl.DEPTH_TEST); gl.disable(gl.CULL_FACE);

    /* --- SSAO --- */
    const hw = W >> 1, hh = H >> 1;
    if (p.ssao) {
      this.full(this.P.ssao, r.aoF, hw, hh, { uP: proj, uInvP: invP, uV: viewM, uFrame: fr % 64 }, { uD: r.D, uN: r.N });
      this.full(this.P.blur, r.ao2F, hw, hh, { uDir: [1 / hw, 0] }, { uT: r.ao, uD: r.D });
      this.full(this.P.blur, r.aoF, hw, hh, { uDir: [0, 1 / hh] }, { uT: r.ao2, uD: r.D });
    } else { gl.bindFramebuffer(gl.FRAMEBUFFER, r.aoF); gl.clearColor(1, 1, 1, 1); gl.clear(gl.COLOR_BUFFER_BIT); }

    /* --- éclairage --- */
    const shared = { uActorVP: this.actorVP, uShadowVP: new Float32Array([...this.shadowVP[0], ...this.shadowVP[1], ...this.shadowVP[2]]), uCascade: CASC, uShadowTexel: 1 / p.shadow, uSunDir: sun.dir, uSunCol: sun.col, uSkyTop: sun.top, uSkyHor: sun.hor, uGround: sun.ground, uCam: eye, uTime: view.time, uCloudShadow: view.cloud ?? 0.65 };
    gl.bindFramebuffer(gl.FRAMEBUFFER, r.hdrF); gl.drawBuffers([gl.COLOR_ATTACHMENT0]);
    // attachement de profondeur détaché pendant la lecture, évite la boucle de rétroaction
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.TEXTURE_2D, null, 0);
    this.full(this.P.light, r.hdrF, W, H, { ...shared, uInvVP: invVP, uShadowSamples: p.samples, uNight: this.night || 0 }, { uA: r.A, uN: r.N, uL: r.L, uD: r.D, uSSAO: r.ao, uShadow: this.tShadow, uNoise: this.tNoise, uActorShadow: this.tActor });

    /* --- nuages et volume --- */
    if (p.clouds && fr % p.cloudStep !== 0 && p.cloudStep > 1) { /* image précédente conservée */ }
    else if (p.clouds) this.full(this.P.clouds, r.clF, W >> 2, H >> 2, { ...shared, uInvVP: invVP, uFrame: fr % 64 }, { uD: r.D, uNoise: this.tNoise });
    else { gl.bindFramebuffer(gl.FRAMEBUFFER, r.clF); gl.clearColor(0, 0, 0, 1); gl.clear(gl.COLOR_BUFFER_BIT); }
    this.full(this.P.vol, r.volF, hw, hh, { ...shared, uInvVP: invVP, uFrame: fr % 64, uFog: this.fog, uSteps: p.vol }, { uD: r.D, uShadow: this.tShadow, uNoise: this.tNoise });

    /* --- eau --- */
    gl.bindFramebuffer(gl.READ_FRAMEBUFFER, r.gbuf); gl.bindFramebuffer(gl.DRAW_FRAMEBUFFER, r.sceneF);
    gl.blitFramebuffer(0, 0, W, H, 0, 0, W, H, gl.DEPTH_BUFFER_BIT, gl.NEAREST);
    gl.bindFramebuffer(gl.READ_FRAMEBUFFER, r.hdrF); gl.readBuffer(gl.COLOR_ATTACHMENT0);
    gl.blitFramebuffer(0, 0, W, H, 0, 0, W, H, gl.COLOR_BUFFER_BIT, gl.NEAREST);
    gl.bindFramebuffer(gl.FRAMEBUFFER, r.hdrF); gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.TEXTURE_2D, r.D, 0);
    gl.viewport(0, 0, W, H); gl.enable(gl.DEPTH_TEST); gl.depthMask(false);
    const wat = view.visible.filter((c) => c.water && c.water.count);
    if (wat.length) {
      this.use(this.P.water, { ...shared, uVP: vp, uInvVP: invVP, uRes: [W, H], uPosScale: 1 / 128, uJitter: jit }, { uScene: r.scene, uD: r.D2, uShadow: this.tShadow, uNoise: this.tNoise, uActorShadow: this.tActor });
      gl.disable(gl.CULL_FACE);
      for (const c of wat) { this.set(this.P.water, 'uOrigin', c.org); gl.bindVertexArray(c.water.vao); gl.drawElements(gl.TRIANGLES, c.water.count, gl.UNSIGNED_INT, 0); st.draws++; st.tris += c.water.count / 3; }
    }
    if (view.target) {
      this.use(this.P.line, { uVP: vp, uOffset: view.target.pos.map((v) => v - 0.004), uScale: [1.008, 1.008, 1.008], uColor: view.target.color });
      gl.bindVertexArray(this.lineVAO); gl.drawArrays(gl.LINES, 0, 24); st.draws++;
    }
    if (view.ghost) {
      // Aperçu de pose : volume translucide, faces de cernes marquées pour lire l'orientation.
      const g = view.ghost, L = this.P.line; gl.enable(gl.BLEND); gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA); gl.bindVertexArray(this.cubeVAO); gl.disable(gl.CULL_FACE);
      this.use(L, { uVP: vp, uOffset: g.pos.map((v) => v + 0.02), uScale: [0.96, 0.96, 0.96], uColor: g.ok ? [0.75, 0.85, 0.65, 0.16] : [1.4, 0.2, 0.14, 0.26] });
      gl.drawArrays(gl.TRIANGLES, 0, 36); st.draws++;
      if (g.axis) for (const e of [0, 1]) {
        const o = g.pos.map((v, i) => v + 0.04 + (i === g.axis - 1 ? e * 0.97 - 0.015 : 0)), sc = [0.92, 0.92, 0.92]; sc[g.axis - 1] = 0.02;
        this.use(L, { uVP: vp, uOffset: o, uScale: sc, uColor: [1.6, 0.9, 0.25, 0.6] }); gl.drawArrays(gl.TRIANGLES, 0, 36); st.draws++;
      }
      gl.disable(gl.BLEND);
    }
    gl.depthMask(true); gl.disable(gl.DEPTH_TEST);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.TEXTURE_2D, null, 0);

    /* --- composition, TAA, bloom, sortie --- */
    this.full(this.P.compose, r.compF, W, H, { ...shared, uInvVP: invVP, uFog: this.fog, uTexel: [1 / W, 1 / H] }, { uHdr: r.hdr, uVol: r.vol, uClouds: r.cl, uD: r.D });
    const cur = fr & 1;
    this.full(this.P.taa, r.histF[cur], W, H, { uInvVP: invVP, uPrevVP: this.prevVP, uTexel: [1 / W, 1 / H], uReset: this.reset || view.taa === false ? 1 : 0 }, { uCur: r.comp, uHist: r.hist[1 - cur], uD: r.D, uL: r.L });
    this.prevVP = vp; this.reset = false;
    let bw = hw, bh = hh;
    this.full(this.P.bright, r.bloomF[0], bw, bh, { uTexel: [1 / W, 1 / H], uFirst: 1 }, { uT: r.hist[cur] });
    for (let i = 1; i < 5; i++) { const pw = bw, ph = bh; bw = Math.max(2, bw >> 1); bh = Math.max(2, bh >> 1); this.full(this.P.bright, r.bloomF[i], bw, bh, { uTexel: [1 / pw, 1 / ph], uFirst: 0 }, { uT: r.bloom[i - 1] }); }
    gl.enable(gl.BLEND); gl.blendFunc(gl.ONE, gl.ONE);
    for (let i = 4; i > 0; i--) { const w = Math.max(2, hw >> (i - 1)), h = Math.max(2, hh >> (i - 1)); this.full(this.P.bright, r.bloomF[i - 1], w, h, { uTexel: [0.5 / w, 0.5 / h], uFirst: 0 }, { uT: r.bloom[i] }); }
    gl.disable(gl.BLEND);
    this.full(this.P.final, null, W, H, { uTexel: [1 / W, 1 / H], uExposure: this.exposure, uBloom: 0.05, uSharp: 0.32 }, { uT: r.hist[cur], uB: r.bloom[0] });
    if (this.q && this.qOn) { gl.endQuery(this.timer.TIME_ELAPSED_EXT); this.qOn = false; }
    gl.bindVertexArray(null);
  }
}
