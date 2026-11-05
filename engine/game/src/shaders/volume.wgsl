@group(1) @binding(0) var t_d: texture_depth_2d;
fn hg(c: f32, g: f32) -> f32 { let g2 = g * g; return (1.0 - g2) / (12.57 * pow(1.0 + g2 - 2.0 * g * c, 1.5)); }
fn sh(p: vec3f, dist: f32) -> f32 {
  let c = cascade_of(dist); if (c > 2) { return 1.0; }
  let s = shadow_coord(u.shadow_vp[c] * vec4f(p, 1.0)); if (s.x < 0.0 || s.y < 0.0 || s.x > 1.0 || s.y > 1.0) { return 1.0; }
  return textureSampleCompareLevel(t_shadow, s_cmp, s.xy, c, s.z + 0.0003);
}
const STEPS = 14;
@fragment fn fs_volume(i: FOut) -> @location(0) vec4f {
  let d = depth_at(t_d, i.uv); let wp = world_from(i.uv, max(d, 1e-5)); let ray = wp - u.cam.xyz; let len = length(ray); let v = ray / len;
  let end = min(len, 150.0); let st = end / f32(STEPS); var t = st * ign(i.pos.xy + u.misc.x * 5.588); let mu = dot(v, u.sun.xyz);
  let ph = min(hg(mu, 0.8) * 9.0, 3.2) + hg(mu, -0.2) * 0.5 + 0.3;
  var acc = vec3f(0.0); var tr = 1.0; let time = u.cam.w;
  for (var k = 0; k < STEPS; k++) {
    let p = u.cam.xyz + v * t; let hf = exp(-max(p.y - 52.0, 0.0) * 0.055) + 0.25;
    let n = noise(p * 0.016 + vec3f(time * 0.012, 0.0, time * 0.007)); let rho = u.ground.w * u.weather.x * hf * (0.3 + 1.15 * n * n);
    let s = sh(p, t); let sky_s = mix(u.hor.rgb, u.top.rgb, 0.4) * 0.42;
    acc += tr * (u.sun_col.rgb * s * ph * 0.13 + sky_s * 0.14) * rho * st;
    tr *= exp(-rho * st); t += st;
  }
  return vec4f(acc, tr);
}
