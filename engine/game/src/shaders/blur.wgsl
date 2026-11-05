@group(1) @binding(0) var t_t: texture_2d<f32>;
@group(1) @binding(1) var t_d: texture_depth_2d;
fn lin(d: f32) -> f32 { return 1.0 / max(d, 1e-6); }
fn blur(uv: vec2f, dir: vec2f) -> vec4f {
  let d0 = lin(depth_at(t_d, uv)); var s = textureSampleLevel(t_t, s_clamp, uv, 0.0); var w = 1.0;
  for (var k = -3; k <= 3; k++) {
    if (k == 0) { continue; }
    let p = uv + dir * f32(k); let e = abs(lin(depth_at(t_d, p)) - d0) / d0; let g = exp(-f32(k * k) * 0.18) * exp(-e * 24.0);
    s += textureSampleLevel(t_t, s_clamp, p, 0.0) * g; w += g;
  }
  return s / w;
}
@fragment fn fs_h(i: FOut) -> @location(0) vec4f { return blur(i.uv, vec2f(1.0 / f32(textureDimensions(t_t).x), 0.0)); }
@fragment fn fs_v(i: FOut) -> @location(0) vec4f { return blur(i.uv, vec2f(0.0, 1.0 / f32(textureDimensions(t_t).y))); }
