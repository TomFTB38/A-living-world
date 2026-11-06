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
    let far = max(len - 70.0, 0.0); let a = 1.0 - exp(-far * 0.00052 * (1.0 + exp(-max(wp.y - 50.0, 0.0) * 0.02)));
    let air = sky_color(normalize(vec3f(v.x, max(v.y, 0.03), v.z))); c = mix(c, air, a); c = mix(c, mist, 1.0 - exp(-len * m));
  }
  var vv = vec4f(0.0); var w = 0.0; let d0 = inv_d(d);
  for (var j = 0; j < 4; j++) {
    let off = (vec2f(f32(j & 1), f32(j >> 1u)) - vec2f(0.5)) * texel * 2.0; let p = i.uv + off;
    let e = abs(inv_d(depth_at(t_d, p)) - d0) / d0; let g = exp(-e * 16.0) + 0.01;
    vv += textureSampleLevel(t_vol, s_clamp, p, 0.0) * g; w += g;
  }
  vv /= w; c = c * vv.a + vv.rgb;
  // Lucioles : des points vivants qui flottent autour du joueur, cachés par ce qui est devant eux.
  if (u.extra.x > 0.01) {
    let t = u.cam.w; let b = vec3f(26.0, 7.0, 26.0); let px2 = i.pos.xy; var glow = vec3f(0.0);
    for (var k = 0; k < 56; k++) {
      let f = f32(k); let h = vec3f(hash13(vec3f(f, 1.7, 3.1)), hash13(vec3f(f, 8.3, 0.4)), hash13(vec3f(2.9, f, 5.5)));
      var p = h * b + vec3f(sin(t * 0.31 + f * 1.7) * 1.6, sin(t * 0.47 + f * 2.3) * 0.7, cos(t * 0.27 + f * 0.9) * 1.6);
      p = p - b * floor((p - u.cam.xyz + b * 0.5) / b) - vec3f(0.0, 1.5, 0.0);
      let cc = u.vp * vec4f(p, 1.0); if (cc.w <= 0.1) { continue; }
      let dist = length(p - u.cam.xyz); if (dist > len + 0.3 && d > 0.0) { continue; }
      let s = to_uv(cc.xy / cc.w) * u.jit.zw; let r = length(px2 - s); let size = 3.0 + 26.0 / max(dist, 1.0);
      let blink = smoothstep(0.15, 0.85, 0.5 + 0.5 * sin(t * (1.3 + h.x) + f * 4.1));
      glow += vec3f(0.62, 1.0, 0.32) * blink * (exp(-r * r / (size * 0.35)) * 3.0 + exp(-r / size) * 0.18) / (1.0 + dist * 0.05);
    }
    c += glow * u.extra.x;
  }
  if (u.weather.w > 0.5) { // sous l'eau : la distance se perd vite dans un vert sombre, éclairé par la surface
    let glow = (mix(u.hor.rgb, u.top.rgb, 0.5) * 0.5 + u.sun_col.rgb * 0.06) * vec3f(0.14, 0.34, 0.36);
    c = mix(c * vec3f(0.55, 0.82, 0.8), glow, 1.0 - exp(-len * 0.16));
  }
  return vec4f(c, 1.0);
}
