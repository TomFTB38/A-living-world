@group(1) @binding(0) var t_cur: texture_2d<f32>;
@group(1) @binding(1) var t_hist: texture_2d<f32>;
@group(1) @binding(2) var t_d: texture_depth_2d;
@group(1) @binding(3) var t_l: texture_2d<f32>;
fn ycc(c: vec3f) -> vec3f { return vec3f(0.25 * c.r + 0.5 * c.g + 0.25 * c.b, 0.5 * c.r - 0.5 * c.b, -0.25 * c.r + 0.5 * c.g - 0.25 * c.b); }
fn rgb(c: vec3f) -> vec3f { return vec3f(c.x + c.y - c.z, c.x + c.z, c.x - c.y - c.z); }
fn ton(c: vec3f) -> vec3f { return c / (1.0 + dot(c, vec3f(0.299, 0.587, 0.114))); }
fn inv(c: vec3f) -> vec3f { return c / max(1.0 - dot(c, vec3f(0.299, 0.587, 0.114)), 0.002); }
@fragment fn fs_taa(i: FOut) -> @location(0) vec4f {
  let px = vec2i(i.pos.xy); let dim = vec2i(u.jit.zw) - vec2i(1); let texel = 1.0 / u.jit.zw;
  let c = ton(textureLoad(t_cur, px, 0).rgb); if (u.misc.w > 0.5) { return vec4f(inv(c), 1.0); }
  var m = vec3f(0.0); var s = vec3f(0.0); let y = ycc(c); var mn = y; var mx = y;
  for (var j = -1; j <= 1; j++) { for (var k = -1; k <= 1; k++) {
    let q = ycc(ton(textureLoad(t_cur, clamp(px + vec2i(k, j), vec2i(0), dim), 0).rgb)); m += q; s += q * q; mn = min(mn, q); mx = max(mx, q);
  } }
  m /= 9.0; s = sqrt(max(s / 9.0 - m * m, vec3f(0.0))); let lo = max(mn, m - s * 1.25); let hi = min(mx, m + s * 1.25);
  let d = depth_at(t_d, i.uv); let w = world_from(i.uv, max(d, 1e-6)); let p = u.prev_vp * vec4f(w, 1.0); var q = to_uv(p.xy / p.w);
  let hero = textureLoad(t_l, px, 0).a; if (hero > 0.2 && hero < 0.6) { q = i.uv; } // le héros suit la caméra : son historique est au même point de l'écran
  if (q.x < 0.0 || q.y < 0.0 || q.x > 1.0 || q.y > 1.0) { return vec4f(inv(c), 1.0); }
  let h = clamp(ycc(ton(textureSampleLevel(t_hist, s_clamp, q, 0.0).rgb)), lo, hi);
  let mv = length((q - i.uv) / texel); let a = mix(0.09, 0.28, sat(mv * 0.08));
  return vec4f(inv(rgb(mix(h, y, a))) + fireflies(i.pos.xy, depth_at(t_d, i.uv)), 1.0);
}
