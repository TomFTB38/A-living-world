#!/usr/bin/env python3
"""Compile tous les modules WGSL avec le compilateur de WebGPU dans Chrome. Rapide, sans carte graphique."""
import http.server, threading, json, glob, os
from playwright.sync_api import sync_playwright
D = '/home/claude/a-living-world/engine/game/src/shaders'; common = open(D + '/common.wgsl').read()
mods = {os.path.basename(f)[:-5]: (open(f).read() if 'shadow' in f or 'ui.wgsl' in f else common + open(f).read()) for f in glob.glob(D + '/*.wgsl') if 'common' not in f}
class Q(http.server.SimpleHTTPRequestHandler):
    def log_message(self, *a): pass
    def do_GET(self): self.send_response(200); self.send_header('content-type', 'text/html'); self.end_headers(); self.wfile.write(b'ok')
s = http.server.ThreadingHTTPServer(('127.0.0.1', 8768), Q); threading.Thread(target=s.serve_forever, daemon=True).start()
with sync_playwright() as p:
    b = p.chromium.launch(args=['--enable-unsafe-webgpu', '--use-webgpu-adapter=swiftshader']); g = b.new_page(); g.goto('http://127.0.0.1:8768/')
    r = g.evaluate('''async (mods) => { const a = await navigator.gpu.requestAdapter({ forceFallbackAdapter: true }), d = await a.requestDevice(), o = {}; for (const [n, code] of Object.entries(mods)) { const m = d.createShaderModule({ code }), i = await m.getCompilationInfo(); o[n] = i.messages.filter((x) => x.type != 'info').map((x) => x.type + ' l.' + x.lineNum + ' ' + x.message + ' :: ' + code.split('\\n')[x.lineNum - 1].slice(0, 90)); } return o; }''', mods)
    bad = 0
    for k, v in sorted(r.items()):
        print(k, 'ok' if not v else ''); [print('  ', x[:260]) for x in v[:6]]; bad += len(v)
    print('messages', bad); b.close()
