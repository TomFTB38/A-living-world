// Constantes de l'image, partagées par toutes les passes. Profondeur inversée : proche = 1, ciel = 0.
struct F {
  vp: mat4x4f, ivp: mat4x4f, prev_vp: mat4x4f, view: mat4x4f, proj: mat4x4f, iproj: mat4x4f, shadow_vp: array<mat4x4f, 3>,
  cascade: vec4f, cam: vec4f, sun: vec4f, sun_col: vec4f, top: vec4f, hor: vec4f, ground: vec4f, jit: vec4f, misc: vec4f, aim: vec4f, ghost: vec4f, weather: vec4f, extra: vec4f,
};
// cam.w temps · sun.w exposition · sun_col.w couverture nuageuse · top.w texel d'ombre · hor.w nuit · ground.w brume
// jit.xy gigue · jit.zw taille · misc : image, voile, trou de l'horizon lointain, remise à zéro du TAA
// weather : densification de la brume, pluie, luminosité choisie par le joueur, sous l'eau
// extra.x : lucioles
@group(0) @binding(0) var<uniform> u: F;
@group(0) @binding(1) var t_noise: texture_3d<f32>;
@group(0) @binding(2) var s_rep: sampler;
@group(0) @binding(3) var t_shadow: texture_depth_2d_array;
@group(0) @binding(4) var s_cmp: sampler_comparison;
@group(0) @binding(5) var s_clamp: sampler;

fn sat(x: f32) -> f32 { return clamp(x, 0.0, 1.0); }
fn ign(p: vec2f) -> f32 { return fract(52.9829189 * fract(dot(p, vec2f(0.06711056, 0.00583715)))); }
fn hash13(q: vec3f) -> f32 { var p = fract(q * 0.1031); p += dot(p, p.zyx + 31.32); return fract((p.x + p.y) * p.z); }
fn noise(p: vec3f) -> f32 { return textureSampleLevel(t_noise, s_rep, p, 0.0).r; }
fn world_from(uv: vec2f, d: f32) -> vec3f { let c = u.ivp * vec4f(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, d, 1.0); return c.xyz / c.w; }
fn to_uv(n: vec2f) -> vec2f { return vec2f(n.x * 0.5 + 0.5, 0.5 - n.y * 0.5); }
fn shadow_coord(lp: vec4f) -> vec3f { return vec3f(lp.x * 0.5 + 0.5, 0.5 - lp.y * 0.5, lp.z); }
fn cascade_of(dist: f32) -> i32 { if (dist < u.cascade.x) { return 0; } if (dist < u.cascade.y) { return 1; } if (dist < u.cascade.z) { return 2; } return 3; }

fn shadow_at(wp: vec3f, n: vec3f, dist: f32, fc: vec2f, samples: i32) -> f32 {
  let c = cascade_of(dist); if (c > 2) { return 1.0; }
  var bias = 0.26; if (c == 0) { bias = 0.035; } else if (c == 1) { bias = 0.09; }
  let p = wp + n * bias * 1.5 + u.sun.xyz * bias;
  let s = shadow_coord(u.shadow_vp[c] * vec4f(p, 1.0));
  if (s.x < 0.002 || s.y < 0.002 || s.x > 0.998 || s.y > 0.998) { return 1.0; }
  let a = ign(fc) * 6.283; let ca = cos(a); let sa = sin(a); var sum = 0.0;
  var rad = u.top.w * 1.2; if (c == 0) { rad = u.top.w * 2.6; } else if (c == 1) { rad = u.top.w * 1.8; }
  var vogel = array<vec2f, 8>(vec2f(0.177, 0.0), vec2f(-0.25, 0.228), vec2f(0.04, -0.446), vec2f(0.336, 0.391), vec2f(-0.578, -0.137), vec2f(0.569, -0.285), vec2f(-0.232, 0.649), vec2f(-0.301, -0.667));
  for (var i = 0; i < samples; i++) {
    let v = vogel[i]; let o = vec2f(v.x * ca - v.y * sa, v.x * sa + v.y * ca) * rad;
    sum += textureSampleCompareLevel(t_shadow, s_cmp, s.xy + o, c, s.z);
  }
  let sh = sum / f32(samples);
  if (c == 2) { return mix(1.0, sh, sat((u.cascade.z - dist) / (u.cascade.z * 0.15))); }
  return sh;
}
fn cloud_shade(wp: vec3f) -> f32 {
  let k = u.sun_col.w; if (k <= 0.0) { return 1.0; }
  let t = u.cam.w; let q = vec3f(wp.xz * 0.0011 + vec2f(t * 0.004, t * 0.0017), 0.31);
  return mix(1.0, (1.0 - smoothstep(0.3, 0.56, noise(q.xzy))) * 0.7 + 0.3, k);
}
fn sky_color(d: vec3f) -> vec3f {
  let y = max(d.y, 0.0); let t = pow(1.0 - y, 3.6); let mu = sat(dot(d, u.sun.xyz));
  var c = mix(u.top.rgb, u.hor.rgb, t);
  c += u.sun_col.rgb * (0.0035 / (1.02 - mu * 0.985) + 0.05 * pow(mu, 8.0)) * (0.25 + 0.75 * t);
  if (d.y < 0.0) { c = mix(c, u.ground.rgb, sat(-d.y * 7.0)); }
  return c;
}
struct FOut { @builtin(position) pos: vec4f, @location(0) uv: vec2f };
@vertex fn vs_full(@builtin(vertex_index) i: u32) -> FOut {
  let p = vec2f(f32((i << 1u) & 2u), f32(i & 2u)); var o: FOut;
  o.uv = vec2f(p.x, 1.0 - p.y); o.pos = vec4f(p * 2.0 - 1.0, 0.0, 1.0); return o;
}
fn depth_at(t: texture_depth_2d, uv: vec2f) -> f32 {
  let s = vec2f(textureDimensions(t)); let p = clamp(vec2i(uv * s), vec2i(0), vec2i(s) - vec2i(1));
  return textureLoad(t, p, 0);
}
