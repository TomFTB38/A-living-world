// Constantes de l'image, partagées par toutes les passes. Profondeur inversée : proche = 1, ciel = 0.
struct F {
  vp: mat4x4f, ivp: mat4x4f, prev_vp: mat4x4f, view: mat4x4f, proj: mat4x4f, iproj: mat4x4f, shadow_vp: array<mat4x4f, 3>,
  cascade: vec4f, cam: vec4f, sun: vec4f, sun_col: vec4f, top: vec4f, hor: vec4f, ground: vec4f, jit: vec4f, misc: vec4f, aim: vec4f, ghost: vec4f, weather: vec4f, extra: vec4f, extra2: vec4f, lights: array<vec4f, 96>, gbox: array<vec4f, 6>,
};
// cam.w temps · sun.w exposition · sun_col.w couverture nuageuse · top.w texel d'ombre · hor.w nuit · ground.w brume
// jit.xy gigue · jit.zw taille · misc : image, voile, trou de l'horizon lointain, remise à zéro du TAA
// weather : densification de la brume, pluie, luminosité choisie par le joueur, sous l'eau
// extra.x : lucioles · extra.y : nombre de lumières colorées (position, couleur) · extra.z : neige · extra.w : phase de la lune
// extra2.x : brume de vallée · extra2.y : aurores boréales · extra2.z : sous terre (0 à l'air libre, 1 au fond d'une grotte) · extra2.w : arc-en-ciel
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
  // L'heure dorée : quand le soleil est bas, l'horizon s'embrase de son côté (orange au ras, rose au-dessus), et le côté
  // opposé se teinte d'une bande mauve (la ceinture de Vénus).
  let el = u.sun.y; let gold = (1.0 - smoothstep(0.02, 0.42, el)) * smoothstep(-0.14, 0.0, el) * (1.0 - u.hor.w);
  if (gold > 0.001) {
    let az = dot(normalize(vec2f(d.x, d.z) + vec2f(1e-5)), normalize(vec2f(u.sun.x, u.sun.z) + vec2f(1e-5)));
    let toward = pow(max(az, 0.0), 2.0); let away = pow(max(-az, 0.0), 1.5); let low = pow(1.0 - y, 7.0); let mid = pow(1.0 - y, 3.0) - low * 0.6;
    c += (vec3f(1.25, 0.46, 0.14) * low * 1.4 * (0.35 + 0.65 * toward) + vec3f(0.85, 0.38, 0.5) * max(mid, 0.0) * 0.35 * toward + vec3f(0.5, 0.36, 0.62) * mid * 0.25 * away) * gold;
  }
  if (d.y < 0.0) { c = mix(c, u.ground.rgb, sat(-d.y * 7.0)); }
  return c;
}
/// La nuit : des étoiles qui scintillent (quelques-unes bleutées ou orangées), la Voie lactée, et la lune avec sa phase et
/// son halo. Rien en plein jour ; l'ensemble pâlit sous la pleine lune.
fn night_sky(v: vec3f) -> vec3f {
  let night = sat((u.hor.w - 0.35) / 0.5); if (night <= 0.0 || v.y <= -0.02) { return vec3f(0.0); }
  let lit = 0.5 - 0.5 * cos(u.extra.w * 6.2832); let t = u.cam.w; var c = vec3f(0.0); let fade = smoothstep(-0.02, 0.18, v.y);
  // les étoiles : une grille sur la sphère, une étoile par case de loin en loin, au centre décalé
  for (var l = 0; l < 2; l++) { let sc = select(150.0, 290.0, l == 1); let q = v * sc; let cell = floor(q); let h = hash13(cell + f32(l) * 17.0);
    if (h > select(0.988, 0.975, l == 1)) { let ofs = vec3f(hash13(cell + 3.1), hash13(cell + 7.7), hash13(cell + 1.3)) * 0.6 + 0.2; let dd = length(q - cell - ofs);
      let b = (h - 0.97) * 33.0 * select(1.0, 0.45, l == 1); let tw = 0.65 + 0.35 * sin(t * (2.0 + h * 5.0) + h * 91.0);
      let tint = mix(vec3f(0.75, 0.85, 1.25), vec3f(1.25, 0.9, 0.6), hash13(cell + 5.5)); c += tint * b * tw * (1.0 - smoothstep(0.0, 0.32, dd)); } }
  // la Voie lactée : une bande pâle et marbrée sur un grand cercle
  let gal = normalize(vec3f(0.35, 0.52, 0.78)); let bd = dot(v, gal); let band = exp(-bd * bd / 0.018) * (0.4 + 0.6 * noise(v * 3.0 + vec3f(0.0, 0.0, 9.0))) * (0.5 + 0.5 * noise(v * 9.0));
  c += vec3f(0.55, 0.6, 0.85) * band * 0.06;
  c *= (1.0 - 0.6 * lit) * fade;
  // la lune : un disque à la phase du jour, mers sombres, halo
  let md = normalize(u.sun.xyz); let mu = dot(v, md); let r = 0.024;
  if (mu > 0.95) { let right = normalize(cross(md, vec3f(0.0, 1.0, 0.0))); let up = cross(right, md); let p = vec2f(dot(v - md * mu, right), dot(v - md * mu, up)) / r; let rr = dot(p, p);
    if (rr < 1.0) { let edge = sqrt(1.0 - p.y * p.y); let k = cos(u.extra.w * 6.2832); let lx = select(-p.x, p.x, u.extra.w < 0.5); let on = smoothstep(k * edge - 0.08, k * edge + 0.08, lx);
      let maria = 0.78 + 0.22 * noise(vec3f(p * 1.7, 4.0)); c = mix(c * 0.2, vec3f(1.15, 1.12, 1.0) * maria * 1.6, max(on, 0.04)); }
    c += vec3f(0.5, 0.56, 0.7) * 0.05 * lit * exp(-(1.0 - mu) * 220.0) * 6.0; }
  return c * night;
}
/// L'arc-en-ciel : à 42° du point opposé au soleil, rouge dehors, violet dedans, et un second arc pâle, inversé, plus loin.
fn rainbow(v: vec3f) -> vec3f {
  let k = u.extra2.w; if (k <= 0.01) { return vec3f(0.0); }
  let th = acos(clamp(dot(v, -normalize(u.sun.xyz)), -1.0, 1.0));
  let f1 = (th - 0.7) / 0.035; let f2 = (0.905 - th) / 0.04;
  var c = vec3f(0.0);
  if (f1 > 0.0 && f1 < 1.0) { c += (0.5 + 0.5 * cos(6.2832 * (vec3f(0.0, 0.33, 0.67) + f1 * 0.8 - 0.05))) * sin(f1 * 3.1416) * 0.5; }
  if (f2 > 0.0 && f2 < 1.0) { c += (0.5 + 0.5 * cos(6.2832 * (vec3f(0.0, 0.33, 0.67) + f2 * 0.8 - 0.05))) * sin(f2 * 3.1416) * 0.14; }
  return c * k * smoothstep(-0.03, 0.06, v.y) * min(length(u.sun_col.rgb), 6.0) * 0.4;
}
/// Aurores boréales : des rideaux verts qui ondulent, frangés de violet en hauteur.
fn aurora(v: vec3f) -> vec3f {
  if (u.extra2.y <= 0.01 || v.y <= 0.01) { return vec3f(0.0); }
  let t = u.cam.w * 0.02; var acc = vec3f(0.0);
  for (var k = 0; k < 12; k++) {
    let f = f32(k) / 12.0; let q = v.xz / v.y * (1.0 + f * 0.55) * 0.35;
    let wv = noise(vec3f(q.x * 0.09 + t * 0.4, f * 0.2, q.y * 0.05 + t * 0.15));
    let dist = abs(fract(q.y * 0.11 + 1.3 * wv + 0.15 * sin(q.x * 0.35 + t * 3.0)) - 0.5);
    let band = smoothstep(0.09, 0.0, dist) * (0.55 + 0.45 * noise(vec3f(q.x * 0.7, q.y * 0.3, t * 2.0)));
    acc += mix(vec3f(0.08, 1.0, 0.42), vec3f(0.55, 0.16, 0.95), smoothstep(0.35, 0.95, f)) * band * (1.0 - f * 0.6);
  }
  return acc * 0.4 * u.extra2.y * smoothstep(0.02, 0.2, v.y);
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

// Lucioles : des points vivants qui flottent autour du joueur, cachés par ce qui est devant eux.
// Ajoutées après l'anticrénelage, qui effacerait des points aussi fins.
fn fireflies(px2: vec2f, d: f32) -> vec3f {
  if (u.extra.x <= 0.01) { return vec3f(0.0); }
  let uv = px2 / u.jit.zw; let len = length(world_from(uv, max(d, 1e-5)) - u.cam.xyz);
  let t = u.cam.w; let b = vec3f(26.0, 7.0, 26.0); var glow = vec3f(0.0);
  for (var k = 0; k < 56; k++) {
    let f = f32(k); let h = vec3f(hash13(vec3f(f, 1.7, 3.1)), hash13(vec3f(f, 8.3, 0.4)), hash13(vec3f(2.9, f, 5.5)));
    var p = h * b + vec3f(sin(t * 0.31 + f * 1.7) * 1.6, sin(t * 0.47 + f * 2.3) * 0.7, cos(t * 0.27 + f * 0.9) * 1.6);
    p = p - b * floor((p - u.cam.xyz + b * 0.5) / b) - vec3f(0.0, 1.5, 0.0);
    let cc = u.vp * vec4f(p, 1.0); if (cc.w <= 0.1) { continue; }
    let dist = length(p - u.cam.xyz); if (d > 0.0 && dist > len + 0.3) { continue; }
    let s = to_uv(cc.xy / cc.w) * u.jit.zw; let r = length(px2 - s); let size = 2.2 + 36.0 / max(dist, 1.0);
    let blink = smoothstep(0.1, 0.8, 0.5 + 0.5 * sin(t * (1.3 + h.x) + f * 4.1));
    glow += vec3f(0.62, 1.0, 0.32) * blink * (exp(-r * r / (size * size * 0.3)) * 3.5 + exp(-r / size) * 0.6) / (1.0 + dist * 0.04);
  }
  return glow * u.extra.x;
}
