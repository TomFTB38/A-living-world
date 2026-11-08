// Contour du bloc visé, rouge à mesure qu'il cède ; contour de l'emplacement de pose.
const E = array<vec3f, 24>(vec3f(0,0,0),vec3f(1,0,0),vec3f(1,0,0),vec3f(1,0,1),vec3f(1,0,1),vec3f(0,0,1),vec3f(0,0,1),vec3f(0,0,0),vec3f(0,1,0),vec3f(1,1,0),vec3f(1,1,0),vec3f(1,1,1),vec3f(1,1,1),vec3f(0,1,1),vec3f(0,1,1),vec3f(0,1,0),vec3f(0,0,0),vec3f(0,1,0),vec3f(1,0,0),vec3f(1,1,0),vec3f(1,0,1),vec3f(1,1,1),vec3f(0,0,1),vec3f(0,1,1));
struct L { @builtin(position) p: vec4f, @location(0) c: vec4f };
fn h1(x: f32) -> f32 { return fract(sin(x * 12.9898 + 4.1414) * 43758.5453); }
// Fissures : sur chaque face du bloc visé, trois lézardes partent du centre et s'allongent à mesure qu'il cède.
// 6 faces × 12 segments ; un segment n'apparaît qu'une fois la casse assez avancée.
fn crack(i: u32, a: vec4f) -> L {
  var o: L; let f = i / 24u; let sg = (i % 24u) / 2u; let br = sg % 3u; let j = sg / 3u + (i % 2u);
  let seed = a.x * 7.13 + a.y * 3.71 + a.z * 1.93 + f32(f) * 5.3; let ang = f32(br) * 2.094 + h1(seed) * 6.283; var uv = vec2f(0.5);
  for (var q = 1u; q <= j; q++) { let an = ang + (h1(seed + f32(br) * 9.1 + f32(q) * 3.3) - 0.5) * 1.2; uv += vec2f(cos(an), sin(an)) * (0.1 + 0.04 * h1(seed + f32(q) * 1.7)); }
  uv = clamp(uv, vec2f(0.03), vec2f(0.97)); let ax = f / 2u; let sd = f32(f % 2u) * 1.008 - 0.004; var p = vec3f(sd, uv.x, uv.y);
  if (ax == 1u) { p = vec3f(uv.x, sd, uv.y); } else if (ax == 2u) { p = vec3f(uv.x, uv.y, sd); }
  let c = u.vp * vec4f(a.xyz + p, 1.0); o.p = vec4f(c.xy + u.jit.xy * c.w, c.z, c.w); o.c = vec4f(0.02, 0.018, 0.015, 1.0);
  if (f32(sg) >= floor(a.w * 13.0)) { o.p = vec4f(2.0, 2.0, 2.0, 1.0); } // pas encore fissuré : hors de l'écran
  return o;
}
@vertex fn vs_line(@builtin(vertex_index) i: u32, @builtin(instance_index) k: u32) -> L {
  if (k == 2u) { return crack(i, u.aim); }
  var e = E; var o: L; let a = select(u.ghost, u.aim, k == 0u); let s = select(0.98, 1.008, k == 0u); let m = a.w;
  let c = u.vp * vec4f(a.xyz + vec3f((1.0 - s) * 0.5) + e[i] * s, 1.0); o.p = vec4f(c.xy + u.jit.xy * c.w, c.z, c.w);
  if (k == 0u) { o.c = vec4f(1.0 + m * 0.6, 1.0 - m * 0.6, 1.0 - m, 1.0); } else if (m > 0.5) { o.c = vec4f(0.6, 1.0, 0.7, 1.0); } else { o.c = vec4f(1.3, 0.2, 0.15, 1.0); }
  return o;
}
@fragment fn fs_line(i: L) -> @location(0) vec4f { return i.c; }
