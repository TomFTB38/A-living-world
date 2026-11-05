#!/usr/bin/env python3
"""Banc d'essai du shader : rend la vraie scène du moteur avec WebGPU dans Chrome, sans wgpu ni carte graphique.
Valide le WGSL, les formats de sommets et les conventions de caméra."""
import http.server, threading, functools, sys, json, subprocess
from playwright.sync_api import sync_playwright
OUT = sys.argv[1]; EYE = json.loads(sys.argv[2]); LOOK = json.loads(sys.argv[3]); HOUR = float(sys.argv[4]) if len(sys.argv) > 4 else 10
D = '/tmp/claude-0/s/bench'; subprocess.run(['mkdir', '-p', D])
subprocess.run(['cargo', 'run', '--release', '--offline', '--quiet', '--example', 'dump', '--', D + '/scene.bin', str(EYE[0]), str(EYE[2])], cwd='/home/claude/a-living-world/engine/core', check=True)
open(D + '/main.wgsl', 'w').write(open('/home/claude/a-living-world/engine/game/src/main.wgsl').read())
open(D + '/i.html', 'w').write('<canvas id=c width=1280 height=720></canvas>')
JS = r'''async ([EYE, LOOK, HOUR]) => {
  const buf = await (await fetch('scene.bin')).arrayBuffer(), code = await (await fetch('main.wgsl')).text(); const h = new Int32Array(buf, 0, 2), layers = h[0], chunks = h[1], S = 128, N = S * S * 4 * layers;
  const a = await navigator.gpu.requestAdapter({ forceFallbackAdapter: true }), d = await a.requestDevice(); console.log('etape device');
  const errs = []; d.addEventListener('uncapturederror', (e) => errs.push(e.error.message)); d.pushErrorScope('validation');
  console.log('etape shader'); const m = d.createShaderModule({ code }), info = await m.getCompilationInfo(); const msgs = info.messages.map((x) => x.type + ' ' + x.lineNum + ': ' + x.message); if (info.messages.some((x) => x.type === 'error')) return { msgs };
  console.log('etape tex'); const tex = (off) => { const t = d.createTexture({ size: [S, S, layers], format: 'rgba8unorm', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST }); d.queue.writeTexture({ texture: t }, new Uint8Array(buf, off, N), { bytesPerRow: S * 4, rowsPerImage: S }, [S, S, layers]); return t; };
  const alb = tex(8), nrm = tex(8 + N), fmt = 'rgba8unorm', W = 1024, H = 576, target = d.createTexture({ size: [W, H], format: fmt, usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC });
  console.log('etape ub'); const ub = d.createBuffer({ size: 224, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST }), smp = d.createSampler({ magFilter: 'linear', minFilter: 'linear', addressModeU: 'repeat', addressModeV: 'repeat' });
  const bl = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: 3, buffer: {} }, { binding: 1, visibility: 2, texture: { viewDimension: '2d-array' } }, { binding: 2, visibility: 2, texture: { viewDimension: '2d-array' } }, { binding: 3, visibility: 2, sampler: {} }] });
  const bg = d.createBindGroup({ layout: bl, entries: [{ binding: 0, resource: { buffer: ub } }, { binding: 1, resource: alb.createView({ dimension: '2d-array' }) }, { binding: 2, resource: nrm.createView({ dimension: '2d-array' }) }, { binding: 3, resource: smp }] });
  const layout = d.createPipelineLayout({ bindGroupLayouts: [bl] });
  const vb = [{ arrayStride: 20, attributes: [{ shaderLocation: 0, offset: 0, format: 'sint16x4' }, { shaderLocation: 1, offset: 8, format: 'snorm8x4' }, { shaderLocation: 2, offset: 12, format: 'unorm8x4' }, { shaderLocation: 3, offset: 16, format: 'unorm8x4' }] }, { arrayStride: 16, stepMode: 'instance', attributes: [{ shaderLocation: 4, offset: 0, format: 'float32x4' }] }];
  const pipe = (fs, cull, blend, write) => d.createRenderPipeline({ layout, vertex: { module: m, entryPoint: 'vs_main', buffers: vb }, fragment: { module: m, entryPoint: fs, targets: [{ format: fmt, ...(blend ? { blend: { color: { srcFactor: 'src-alpha', dstFactor: 'one-minus-src-alpha' }, alpha: { srcFactor: 'one', dstFactor: 'one-minus-src-alpha' } } } : {}) }] }, primitive: { topology: 'triangle-list', cullMode: cull }, depthStencil: { format: 'depth32float', depthWriteEnabled: write, depthCompare: 'greater' } });
  console.log('etape pipe'); const pO = pipe('fs_main', 'back', false, true), pD = pipe('fs_main', 'none', false, true), pW = pipe('fs_water', 'none', true, false);
  const pS = d.createRenderPipeline({ layout, vertex: { module: m, entryPoint: 'vs_sky' }, fragment: { module: m, entryPoint: 'fs_sky', targets: [{ format: fmt }] }, primitive: { topology: 'triangle-list' }, depthStencil: { format: 'depth32float', depthWriteEnabled: false, depthCompare: 'always' } });
  console.log('etape idx'); const Q = 700000, idx = new Uint32Array(Q * 6); for (let i = 0; i < Q; i++) { const v = i * 4; idx.set([v, v + 1, v + 2, v, v + 2, v + 3], i * 6); }
  const ib = d.createBuffer({ size: idx.byteLength, usage: GPUBufferUsage.INDEX | GPUBufferUsage.COPY_DST }); d.queue.writeBuffer(ib, 0, idx);
  const inst = d.createBuffer({ size: 16 * chunks, usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST }), list = []; let p = 8 + N * 2;
  for (let c = 0; c < chunks; c++) {
    const e = new Int32Array(buf.slice(p, p + 20)); p += 20; const mk = (n) => { if (!n) return null; const b = d.createBuffer({ size: n * 20, usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST }); d.queue.writeBuffer(b, 0, new Uint8Array(buf, p, n * 20)); p += n * 20; return b; };
    const o = mk(e[2]), de = mk(e[3]), w = mk(e[4]); d.queue.writeBuffer(inst, c * 16, new Float32Array([e[0] * 32, 0, e[1] * 32, 1 / 128])); list.push({ o, de, w, n: [e[2], e[3], e[4]] });
  }
  const norm = (v) => { const l = Math.hypot(...v); return v.map((x) => x / l); }, cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]], dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  const fw = norm([LOOK[0] - EYE[0], LOOK[1] - EYE[1], LOOK[2] - EYE[2]]), s = norm(cross(fw, [0, 1, 0])), up = cross(s, fw);
  const V = [s[0], up[0], -fw[0], 0, s[1], up[1], -fw[1], 0, s[2], up[2], -fw[2], 0, -dot(s, EYE), -dot(up, EYE), dot(fw, EYE), 1], f = 1 / Math.tan(1.05 / 2), P = [f * H / W, 0, 0, 0, 0, f, 0, 0, 0, 0, 0, -1, 0, 0, 0.1, 0];
  const mul = (A, B) => { const o = new Array(16).fill(0); for (let c = 0; c < 4; c++) for (let r = 0; r < 4; r++) for (let k = 0; k < 4; k++) o[r + c * 4] += A[r + k * 4] * B[k + c * 4]; return o; };
  const vp = mul(P, V), a2 = ((HOUR - 6) / 12) * Math.PI, el = Math.sin(a2), sd = norm([Math.cos(a2) * 0.82, el * 0.9 + 0.02, 0.38 + 0.2 * (1 - el)]);
  // inverse de vp par élimination
  const inv = (m) => { const a = []; for (let r = 0; r < 4; r++) a.push([m[r], m[r + 4], m[r + 8], m[r + 12], r == 0 ? 1 : 0, r == 1 ? 1 : 0, r == 2 ? 1 : 0, r == 3 ? 1 : 0]); for (let i = 0; i < 4; i++) { let b = i; for (let r = i; r < 4; r++) if (Math.abs(a[r][i]) > Math.abs(a[b][i])) b = r; [a[i], a[b]] = [a[b], a[i]]; const q = a[i][i]; a[i] = a[i].map((x) => x / q); for (let r = 0; r < 4; r++) if (r != i) { const k = a[r][i]; a[r] = a[r].map((x, j) => x - k * a[i][j]); } } const o = new Array(16); for (let r = 0; r < 4; r++) for (let c = 0; c < 4; c++) o[r + c * 4] = a[r][c + 4]; return o; };
  d.queue.writeBuffer(ub, 0, new Float32Array([...vp, ...inv(vp), EYE[0], EYE[1], EYE[2], 3, sd[0], sd[1], sd[2], 0.6, 6.2, 5.3, 3.7, 0, 0.22, 0.46, 1.05, 0, 0.82, 1.0, 1.25, 0, 0.3, 0.3, 0.15, 0.0022]));
  console.log('etape draw'); const depth = d.createTexture({ size: [W, H], format: 'depth32float', usage: GPUTextureUsage.RENDER_ATTACHMENT }), enc = d.createCommandEncoder();
  const pass = enc.beginRenderPass({ colorAttachments: [{ view: target.createView(), loadOp: 'clear', storeOp: 'store', clearValue: [0, 0, 0, 1] }], depthStencilAttachment: { view: depth.createView(), depthClearValue: 0, depthLoadOp: 'clear', depthStoreOp: 'store' } });
  pass.setBindGroup(0, bg); pass.setPipeline(pS); pass.draw(3); pass.setIndexBuffer(ib, 'uint32');
  const draw = (pl, key, i) => { pass.setPipeline(pl); list.forEach((c, k) => { if (c[key]) { pass.setVertexBuffer(0, c[key]); pass.setVertexBuffer(1, inst, k * 16, 16); pass.drawIndexed(c.n[i] / 4 * 6); } }); };
  const STEP = window.STEP ?? 3; if (STEP > 0) draw(pO, 'o', 0); if (STEP > 1) draw(pD, 'de', 1); if (STEP > 2) draw(pW, 'w', 2); pass.end();
  const rb = d.createBuffer({ size: W * H * 4, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ }); enc.copyTextureToBuffer({ texture: target }, { buffer: rb, bytesPerRow: W * 4 }, [W, H]); d.queue.submit([enc.finish()]);
  console.log('etape fin'); await rb.mapAsync(GPUMapMode.READ); console.log('etape lue'); const px = new Uint8ClampedArray(rb.getMappedRange()); const cv = document.getElementById('c'); cv.width = W; cv.height = H; cv.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(px), W, H), 0, 0); const ve = await d.popErrorScope(); return { msgs, ve: ve && ve.message, errs, chunks };
}'''
h = functools.partial(http.server.SimpleHTTPRequestHandler, directory=D)
class Q(h.func):
    def log_message(self, *a): pass
srv = http.server.ThreadingHTTPServer(('127.0.0.1', 8766), functools.partial(Q, directory=D)); threading.Thread(target=srv.serve_forever, daemon=True).start()
with sync_playwright() as p:
    b = p.chromium.launch(args=['--enable-unsafe-webgpu', '--use-webgpu-adapter=swiftshader', '--disable-gpu-watchdog', '--disable-gpu-process-crash-limit'])
    g = b.new_page(viewport={'width': 1280, 'height': 720}); g.on('console', lambda m: print(m.text[:200])); g.on('crash', lambda *a: print('CRASH')); g.goto('http://127.0.0.1:8766/i.html'); g.add_style_tag(content='body{margin:0}'); g.evaluate('window.STEP=' + (sys.argv[5] if len(sys.argv) > 5 else '3'))
    print(g.evaluate(JS, [EYE, LOOK, HOUR])); g.wait_for_timeout(1500); g.screenshot(path=OUT, timeout=240000); b.close()
