// Contour du bloc visé, rouge à mesure qu'il cède ; contour de l'emplacement de pose.
const E = array<vec3f, 24>(vec3f(0,0,0),vec3f(1,0,0),vec3f(1,0,0),vec3f(1,0,1),vec3f(1,0,1),vec3f(0,0,1),vec3f(0,0,1),vec3f(0,0,0),vec3f(0,1,0),vec3f(1,1,0),vec3f(1,1,0),vec3f(1,1,1),vec3f(1,1,1),vec3f(0,1,1),vec3f(0,1,1),vec3f(0,1,0),vec3f(0,0,0),vec3f(0,1,0),vec3f(1,0,0),vec3f(1,1,0),vec3f(1,0,1),vec3f(1,1,1),vec3f(0,0,1),vec3f(0,1,1));
struct L { @builtin(position) p: vec4f, @location(0) c: vec4f };
@vertex fn vs_line(@builtin(vertex_index) i: u32, @builtin(instance_index) k: u32) -> L {
  var e = E; var o: L; let a = select(u.ghost, u.aim, k == 0u); let s = select(0.98, 1.008, k == 0u); let m = a.w;
  let c = u.vp * vec4f(a.xyz + vec3f((1.0 - s) * 0.5) + e[i] * s, 1.0); o.p = vec4f(c.xy + u.jit.xy * c.w, c.z, c.w);
  if (k == 0u) { o.c = vec4f(1.0 + m * 0.6, 1.0 - m * 0.6, 1.0 - m, 1.0); } else if (m > 0.5) { o.c = vec4f(0.6, 1.0, 0.7, 1.0); } else { o.c = vec4f(1.3, 0.2, 0.15, 1.0); }
  return o;
}
@fragment fn fs_line(i: L) -> @location(0) vec4f { return i.c; }
