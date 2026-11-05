fn dens(p: vec3f, cov: f32) -> f32 {
  let h = sat((p.y - 210.0) / 170.0); let t = u.cam.w;
  let q = p * 0.00105 + vec3f(t * 0.0024, 0.0, t * 0.001);
  let b = noise(q); let d = noise(q * 3.7 + vec3f(0.2));
  let prof = smoothstep(0.0, 0.18, h) * (1.0 - smoothstep(0.45, 1.0, h));
  return sat((b * 0.78 + d * 0.22 - (1.0 - cov) * 0.82 - 0.1) * 2.6) * prof;
}
@fragment fn fs_clouds(i: FOut) -> @location(0) vec4f {
  let w = world_from(i.uv, 0.5); let v = normalize(w - u.cam.xyz);
  if (v.y < 0.015) { return vec4f(0.0, 0.0, 0.0, 1.0); }
  let t0 = (210.0 - u.cam.y) / v.y; let t1 = (380.0 - u.cam.y) / v.y; let st = (t1 - t0) / 14.0; var t = t0 + st * ign(i.pos.xy + u.misc.x * 5.588);
  var col = vec3f(0.0); var tr = 1.0; let mu = dot(v, u.sun.xyz); let ph = 0.6 + 0.9 * pow(sat(mu), 6.0);
  let amb = mix(u.hor.rgb, u.top.rgb, 0.3) * 1.25; let cov = u.sun_col.w * 0.5 + 0.28;
  for (var k = 0; k < 14; k++) {
    if (tr <= 0.02) { break; }
    let p = u.cam.xyz + v * t; let d = dens(p, cov);
    if (d > 0.01) {
      var sh = 0.0; for (var j = 1; j <= 3; j++) { sh += dens(p + u.sun.xyz * f32(j) * 22.0, cov); }
      let l = exp(-sh * 1.5); let s = (u.sun_col.rgb * l * ph * 0.95 + amb * (0.45 + 0.55 * sat((p.y - 210.0) / 170.0))) * d;
      let a = 1.0 - exp(-d * st * 0.05); col += tr * s * a / max(d, 0.01); tr *= 1.0 - a;
    }
    t += st;
  }
  let fade = smoothstep(0.015, 0.14, v.y);
  return vec4f(col * fade, mix(1.0, tr, fade));
}
