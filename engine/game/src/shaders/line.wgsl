// Contour du bloc visé. Rouge à mesure que le bloc cède.
const E = array<vec3f, 24>(vec3f(0,0,0),vec3f(1,0,0),vec3f(1,0,0),vec3f(1,0,1),vec3f(1,0,1),vec3f(0,0,1),vec3f(0,0,1),vec3f(0,0,0),vec3f(0,1,0),vec3f(1,1,0),vec3f(1,1,0),vec3f(1,1,1),vec3f(1,1,1),vec3f(0,1,1),vec3f(0,1,1),vec3f(0,1,0),vec3f(0,0,0),vec3f(0,1,0),vec3f(1,0,0),vec3f(1,1,0),vec3f(1,0,1),vec3f(1,1,1),vec3f(0,0,1),vec3f(0,1,1));
@vertex fn vs_line(@builtin(vertex_index) i: u32) -> @builtin(position) vec4f {
  var e = E; let c = u.vp * vec4f(u.aim.xyz - vec3f(0.004) + e[i] * 1.008, 1.0); return vec4f(c.xy + u.jit.xy * c.w, c.z, c.w);
}
@fragment fn fs_line() -> @location(0) vec4f { let m = u.aim.w; return vec4f(1.0 + m * 0.6, 1.0 - m * 0.6, 1.0 - m, 1.0); }
