@group(1) @binding(0) var t_scene: texture_2d<f32>;
@group(1) @binding(1) var t_d: texture_depth_2d;
struct VIn { @location(0) pos: vec4i, @location(1) nrm: vec4f, @location(2) lit: vec4f, @location(3) tint: vec4f, @location(4) org: vec4f };
struct VOut { @builtin(position) clip: vec4f, @location(0) w: vec3f, @location(1) n: vec3f, @location(2) lit: vec3f, @location(3) foam: f32 };
@vertex fn vs_water(v: VIn) -> VOut {
  var o: VOut; o.w = vec3f(v.pos.xyz) * v.org.w + v.org.xyz; o.n = v.nrm.xyz; o.lit = v.lit.xyz; o.foam = v.tint.r * 2.0;
  o.clip = u.vp * vec4f(o.w, 1.0); let w = o.clip.w; o.clip = vec4f(o.clip.xy + u.jit.xy * w, o.clip.z, w); return o;
}
fn wave(p: vec2f, t: f32) -> f32 { return noise(vec3f(p * 0.09 + t * vec2f(0.022, 0.015), t * 0.016)) + 0.5 * noise(vec3f(p * 0.27 - t * vec2f(0.03, 0.05), 0.4 + t * 0.02)); }
fn pos(p: vec2f, d: f32) -> vec3f { return world_from(p, max(d, 1e-6)); }
@fragment fn fs_water(i: VOut) -> @location(0) vec4f {
  let time = u.cam.w; let suv = i.clip.xy / u.jit.zw; let n0 = normalize(i.n); let v = normalize(i.w - u.cam.xyz); let dist = length(i.w - u.cam.xyz); let fall = 1.0 - abs(n0.y);
  let e = 0.12; var n: vec3f;
  if (fall > 0.5) { let q = vec2f(i.w.x + i.w.z, i.w.y * 0.35 + time * 2.6); let t2 = time * 2.0; let a = wave(q, t2); let b = wave(q + vec2f(e, 0.0), t2); let c2 = wave(q + vec2f(0.0, e), t2); n = normalize(n0 + vec3f(a - b, c2 - a, a - b) * 1.2); }
  else { let q = i.w.xz; let a = wave(q, time); let b = wave(q + vec2f(e, 0.0), time); let c2 = wave(q + vec2f(0.0, e), time); n = normalize(vec3f((a - b) * 0.55, 1.0, (a - c2) * 0.55)); }
  let scene_d = depth_at(t_d, suv); let bg = pos(suv, scene_d); let depth = length(bg - i.w);
  let ruv = suv + n.xz * 0.03 * min(1.0, depth * 0.6) / (1.0 + dist * 0.03);
  var scene = textureSampleLevel(t_scene, s_clamp, ruv, 0.0).rgb; let rd = depth_at(t_d, ruv);
  if (length(pos(ruv, rd) - u.cam.xyz) < dist - 0.05) { scene = textureSampleLevel(t_scene, s_clamp, suv, 0.0).rgb; }
  let deep = vec3f(0.015, 0.075, 0.085); let shallow = vec3f(0.19, 0.42, 0.36);
  let absorb = exp(-depth * 0.42); let tint = mix(deep, shallow, exp(-depth * 0.28));
  var sh = shadow_at(i.w, vec3f(0.0, 1.0, 0.0), dist, i.clip.xy, 4); if (fall > 0.5) { sh = 0.5 + 0.2 * sh; }
  let water = mix(tint * (u.sun_col.rgb * 0.35 * sh + mix(u.hor.rgb, u.top.rgb, 0.5) * 0.7 * i.lit.y), scene, absorb * 0.82);
  var r = reflect(v, n); r.y = abs(r.y);
  let fr = 0.07 + 0.93 * pow(1.0 - sat(dot(-v, n)), 4.0); // assez de reflet, même vu d'au-dessus
  var refl = sky_color(r) * mix(0.22, 1.0, i.lit.y);
  if (fall < 0.5) {
    var q = i.w + r * 0.8; var st = 1.8; var hit = vec2f(-1.0);
    for (var k = 0; k < 30; k++) {
      let c = u.vp * vec4f(q, 1.0); if (c.w <= 0.0) { break; } let s = to_uv(c.xy / c.w);
      if (s.x < 0.0 || s.y < 0.0 || s.x > 1.0 || s.y > 1.0) { break; }
      let dq = depth_at(t_d, s); let pp = pos(s, dq);
      if (dq > 0.0 && length(pp - u.cam.xyz) < length(q - u.cam.xyz) && length(pp - q) < st * 2.5) { hit = s; break; }
      q += r * st; st *= 1.09;
    }
    if (hit.x >= 0.0) { let ed = smoothstep(0.0, 0.08, min(min(hit.x, hit.y), min(1.0 - hit.x, 1.0 - hit.y))); refl = mix(refl, textureSampleLevel(t_scene, s_clamp, hit, 0.0).rgb, ed * 0.95); }
  }
  var c = mix(water, refl, fr);
  let hv = normalize(u.sun.xyz - v); c += u.sun_col.rgb * pow(sat(dot(n, hv)), 900.0) * sh * 1.6;
  let glint = pow(sat(dot(reflect(v, normalize(n + vec3f(0.0, 0.2, 0.0))), u.sun.xyz)), 1400.0) * 40.0 * sh; c += u.sun_col.rgb * min(glint, 1.5);
  let fnz = noise(vec3f(i.w.xz * 0.42, time * 0.09)); let boil = 0.6 * noise(vec3f(i.w.xz * 0.37 + vec2f(0.13), time * 0.3)) + 0.4 * noise(vec3f(i.w.zx * 0.91, 0.4 - time * 0.45));
  var foam = max((1.0 - smoothstep(0.05, 0.7, depth)) * smoothstep(0.42, 0.7, fnz + 0.25), smoothstep(0.2, 1.5, i.foam) * smoothstep(0.42, 0.56, boil)) * select(0.0, 1.0, fall < 0.5);
  let lit_s = mix(u.hor.rgb, u.top.rgb, 0.5);
  if (fall > 0.5) { let streak = noise(vec3f((i.w.x + i.w.z) * 1.7, i.w.y * 0.09 + time * 0.9, 0.2)); foam = smoothstep(0.56, 0.76, streak) * 0.42; c = mix(c, vec3f(0.34, 0.5, 0.52) * (u.sun_col.rgb * 0.3 * sh + lit_s * 0.9 * max(i.lit.y, 0.4)), 0.3); }
  c = mix(c, vec3f(0.92, 0.97, 0.98) * (u.sun_col.rgb * 0.34 * sh + lit_s * 0.62 * i.lit.y), foam * 0.55);
  return vec4f(c, 1.0);
}
