// Interface : aplats, texte et icônes de blocs, en pixels d'écran.
struct S { size: vec4f };
@group(0) @binding(0) var<uniform> s: S;
@group(0) @binding(1) var t_blocks: texture_2d_array<f32>;
@group(0) @binding(2) var t_font: texture_2d<f32>;
@group(0) @binding(3) var smp: sampler;
struct VI { @location(0) p: vec2f, @location(1) uv: vec2f, @location(2) col: vec4f, @location(3) layer: f32 };
struct VO { @builtin(position) pos: vec4f, @location(0) uv: vec2f, @location(1) col: vec4f, @location(2) @interpolate(flat) layer: i32 };
@vertex fn vs_ui(v: VI) -> VO { var o: VO; o.pos = vec4f(v.p.x / s.size.x * 2.0 - 1.0, 1.0 - v.p.y / s.size.y * 2.0, 0.0, 1.0); o.uv = v.uv; o.col = v.col; o.layer = i32(v.layer); return o; }
@fragment fn fs_ui(i: VO) -> @location(0) vec4f {
  let f = textureSampleLevel(t_font, smp, i.uv, 0.0).r; let b = textureSampleLevel(t_blocks, smp, i.uv, max(i.layer, 0), 2.0);
  if (i.layer == -1) { return i.col; }
  if (i.layer == -2) { return vec4f(i.col.rgb, i.col.a * f); }
  return vec4f(b.rgb * i.col.rgb, i.col.a);
}
