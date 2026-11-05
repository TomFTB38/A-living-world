@group(1) @binding(0) var t_hdr: texture_2d<f32>;
@group(1) @binding(1) var t_vol: texture_2d<f32>;
@group(1) @binding(2) var t_cl: texture_2d<f32>;
@group(1) @binding(3) var t_d: texture_depth_2d;
fn inv_d(d: f32) -> f32 { return 1.0 / max(d, 1e-6); }
@fragment fn fs_compose(i: FOut) -> @location(0) vec4f {
  let px = vec2i(i.pos.xy); var c = textureLoad(t_hdr, px, 0).rgb; let d = depth_at(t_d, i.uv); let texel = 1.0 / u.jit.zw;
  let wp = world_from(i.uv, max(d, 1e-5)); let ray = wp - u.cam.xyz; let len = length(ray); let v = ray / len;
  let mist = sky_color(normalize(vec3f(v.x, max(v.y, 0.03) * 0.25 + 0.03, v.z))); let m = u.misc.y;
  if (d <= 0.0) { let cl = textureSampleLevel(t_cl, s_clamp, i.uv, 0.0); c = c * cl.a + cl.rgb; c = mix(c, mist, min(1.0, m * 160.0)); }
  else {
    let far = max(len - 70.0, 0.0); let a = 1.0 - exp(-far * 0.00125 * (1.0 + exp(-max(wp.y - 50.0, 0.0) * 0.02)));
    let air = sky_color(normalize(vec3f(v.x, max(v.y, 0.03), v.z))); c = mix(c, air, a); c = mix(c, mist, 1.0 - exp(-len * m));
  }
  var vv = vec4f(0.0); var w = 0.0; let d0 = inv_d(d);
  for (var j = 0; j < 4; j++) {
    let off = (vec2f(f32(j & 1), f32(j >> 1u)) - vec2f(0.5)) * texel * 2.0; let p = i.uv + off;
    let e = abs(inv_d(depth_at(t_d, p)) - d0) / d0; let g = exp(-e * 16.0) + 0.01;
    vv += textureSampleLevel(t_vol, s_clamp, p, 0.0) * g; w += g;
  }
  vv /= w; c = c * vv.a + vv.rgb;
  return vec4f(c, 1.0);
}
