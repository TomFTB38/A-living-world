@group(1) @binding(0) var t_hdr: texture_2d<f32>;
@group(1) @binding(1) var t_vol: texture_2d<f32>;
@group(1) @binding(2) var t_cl: texture_2d<f32>;
@group(1) @binding(3) var t_d: texture_depth_2d;
fn inv_d(d: f32) -> f32 { return 1.0 / max(d, 1e-6); }
@fragment fn fs_compose(i: FOut) -> @location(0) vec4f {
  let px = vec2i(i.pos.xy); var c = textureLoad(t_hdr, px, 0).rgb; let d = depth_at(t_d, i.uv); let texel = 1.0 / u.jit.zw;
  let wp = world_from(i.uv, max(d, 1e-5)); let ray = wp - u.cam.xyz; let len = length(ray); let v = ray / len;
  let mist = sky_color(normalize(vec3f(v.x, max(v.y, 0.03) * 0.25 + 0.03, v.z))); let m = u.misc.y;
  if (d <= 0.0) { let cl = textureSampleLevel(t_cl, s_clamp, i.uv, 0.0); c = c * cl.a + cl.rgb + aurora(v) * max(cl.a, 0.6) + night_sky(v) * cl.a + rainbow(v) * (0.75 + 0.25 * cl.a); /* les aurores percent un ciel voilé ; les étoiles et la lune, seulement entre les nuages ; l'arc, dans la pluie, devant les nuages */ c = mix(c, mist, min(1.0, m * 160.0)); }
  else {
    let far = max(len - 70.0, 0.0); let a = 1.0 - exp(-far * 0.00036 * (1.0 + exp(-max(wp.y - 50.0, 0.0) * 0.02))); // assez léger pour voir les montagnes à plusieurs kilomètres
    let air = sky_color(normalize(vec3f(v.x, max(v.y, 0.03), v.z))); let cave = u.extra2.z; c = mix(c, air, a * (1.0 - cave)); c = mix(c, mist, (1.0 - exp(-len * m)) * (1.0 - 0.85 * cave)); /* sous terre, le lointain ne se fond pas dans le ciel */
    // Brume de vallée : une nappe basse et pâle qui remplit les creux, plus dense au loin.
    if (u.extra2.x > 0.01 && cave < 0.5) { let lo = exp(-max(wp.y - 57.0 - 5.0 * noise(wp * 0.004), 0.0) * 0.16); let veil = mix(u.hor.rgb, vec3f(1.0, 0.92, 0.8) * length(u.sun_col.rgb) * 0.5, 0.25); c = mix(c, veil, u.extra2.x * lo * (1.0 - exp(-len * 0.012)) * 0.9); }
  }
  var vv = vec4f(0.0); var w = 0.0; let d0 = inv_d(d);
  for (var j = 0; j < 4; j++) {
    let off = (vec2f(f32(j & 1), f32(j >> 1u)) - vec2f(0.5)) * texel * 2.0; let p = i.uv + off;
    let e = abs(inv_d(depth_at(t_d, p)) - d0) / d0; let g = exp(-e * 16.0) + 0.01;
    vv += textureSampleLevel(t_vol, s_clamp, p, 0.0) * g; w += g;
  }
  vv /= w; c = c * vv.a + vv.rgb;
  // Rayons de soleil : de chaque point de l'image, on remonte vers le soleil (s'il est devant nous) en comptant le ciel dégagé
  // qu'on y croise — ni relief, ni feuillage, ni nuage. Là où il perce entre les arbres ou entre les nuages, des faisceaux dorés
  // s'étirent jusqu'à nous.
  let sp = u.vp * vec4f(u.cam.xyz + u.sun.xyz * 1000.0, 1.0);
  if (sp.w > 0.0 && u.sun.y > -0.02 && u.weather.w < 0.5 && u.extra2.z < 0.9) {
    let suv = vec2f(sp.x / sp.w * 0.5 + 0.5, 0.5 - sp.y / sp.w * 0.5);
    let edge = 1.0 - smoothstep(0.5, 1.1, max(abs(suv.x - 0.5), abs(suv.y - 0.5)));
    if (edge > 0.0) {
      let delta = (suv - i.uv) / 24.0 * 0.85; var p = i.uv + delta * ign(i.pos.xy + u.misc.x * 7.31); var acc = 0.0; var wsum = 0.0; var decay = 1.0;
      for (var k = 0; k < 24; k++) {
        if (p.x >= 0.0 && p.y >= 0.0 && p.x <= 1.0 && p.y <= 1.0) { let dd = depth_at(t_d, p); if (dd <= 0.0) { acc += textureSampleLevel(t_cl, s_clamp, p, 0.0).a * decay; } }
        wsum += decay; decay *= 0.94; p += delta;
      }
      let dist = length((i.uv - suv) * vec2f(u.jit.z / u.jit.w, 1.0));
      let k_ray = acc / wsum * exp(-dist * 2.2) * edge * sat(u.sun.y * 6.0 + 0.3) * (1.0 - u.extra2.z) * (1.0 - 0.6 * sat(u.weather.y));
      c += u.sun_col.rgb * vec3f(1.0, 0.84, 0.58) * k_ray * 0.12;
    }
  }
  if (u.weather.w > 0.5) { // sous l'eau : la distance se perd vite dans un vert sombre, éclairé par la surface
    let glow = (mix(u.hor.rgb, u.top.rgb, 0.5) * 0.5 + u.sun_col.rgb * 0.06) * vec3f(0.14, 0.34, 0.36);
    c = mix(c * vec3f(0.55, 0.82, 0.8), glow, 1.0 - exp(-len * 0.16));
  }
  return vec4f(c, 1.0);
}
