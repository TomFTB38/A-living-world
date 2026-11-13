// Interface : aplats, texte et icônes rendues au démarrage, en pixels d'écran.
struct S { size: vec4f };
@group(0) @binding(0) var<uniform> s: S;
@group(0) @binding(1) var t_font: texture_2d<f32>;
@group(0) @binding(2) var t_icons: texture_2d<f32>;
@group(0) @binding(3) var smp: sampler;
@group(0) @binding(4) var t_logo: texture_2d<f32>;
struct VI { @location(0) p: vec2f, @location(1) uv: vec2f, @location(2) col: vec4f, @location(3) layer: f32 };
struct VO { @builtin(position) pos: vec4f, @location(0) uv: vec2f, @location(1) col: vec4f, @location(2) @interpolate(flat) layer: i32 };
@vertex fn vs_ui(v: VI) -> VO { var o: VO; o.pos = vec4f(v.p.x / s.size.x * 2.0 - 1.0, 1.0 - v.p.y / s.size.y * 2.0, 0.0, 1.0); o.uv = v.uv; o.col = v.col; o.layer = i32(v.layer); return o; }
@fragment fn fs_ui(i: VO) -> @location(0) vec4f {
  let f = textureSampleLevel(t_font, smp, i.uv, 0.0).r; let dx = dpdx(i.uv) * 0.25; let dy = dpdy(i.uv) * 0.25;
  // quatre lectures : l'icône reste nette et sans scintillement quand elle est réduite
  let c = (textureSampleLevel(t_icons, smp, i.uv - dx - dy, 0.0) + textureSampleLevel(t_icons, smp, i.uv + dx - dy, 0.0) + textureSampleLevel(t_icons, smp, i.uv - dx + dy, 0.0) + textureSampleLevel(t_icons, smp, i.uv + dx + dy, 0.0)) * 0.25;
  if (i.layer == -1) { return i.col; }
  // Le logo du jeu (couche -4), en alpha prémultiplié comme les icônes.
  if (i.layer == -4) { let g = (textureSampleLevel(t_logo, smp, i.uv - dx - dy, 0.0) + textureSampleLevel(t_logo, smp, i.uv + dx - dy, 0.0) + textureSampleLevel(t_logo, smp, i.uv - dx + dy, 0.0) + textureSampleLevel(t_logo, smp, i.uv + dx + dy, 0.0)) * 0.25; return vec4f(g.rgb / max(g.a, 0.001), g.a * i.col.a); }
  if (i.layer == -2) { return vec4f(i.col.rgb, i.col.a * f); }
  // Icônes : un liseré sombre d'un pixel et une ombre portée en bas à droite, lus dans la case de l'atlas seulement.
  let px = length(dx) * 4.0; let py = length(dy) * 4.0; // dérivées lues plus haut, hors des branches let cell = floor(i.uv * 24.0) / 24.0; let lo = cell + vec2f(0.0004); let hi = cell + vec2f(1.0 / 24.0 - 0.0004);
  var ring = 0.0;
  for (var k = 0; k < 8; k++) { let a = f32(k) * 0.7853982; let q = clamp(i.uv + vec2f(cos(a) * px, sin(a) * py) * 1.1, lo, hi); ring = max(ring, textureSampleLevel(t_icons, smp, q, 0.0).a); }
  let sh = textureSampleLevel(t_icons, smp, clamp(i.uv - vec2f(px * 1.6, py * 2.2), lo, hi), 0.0).a;
  var o = c + vec4f(vec3f(0.06, 0.05, 0.04) * ring * 0.9, ring * 0.9) * (1.0 - c.a);
  o = o + vec4f(0.0, 0.0, 0.0, sh * 0.35) * (1.0 - o.a);
  return vec4f(o.rgb / max(o.a, 0.001), o.a * i.col.a);
}
