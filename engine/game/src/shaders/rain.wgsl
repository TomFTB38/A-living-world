// Pluie : traits dans une boîte qui suit la caméra, dessinés après l'image finale pour que le TAA ne les efface pas.
fn h1(n: f32) -> f32 { return fract(sin(n * 12.9898) * 43758.5453); }
fn wrap(p: vec3f, b: vec3f) -> vec3f { return p - b * floor(p / b); }
@vertex fn vs_rain(@builtin(vertex_index) v: u32) -> @builtin(position) vec4f {
  let i = f32(v / 2u); let e = f32(v & 1u); let b = vec3f(28.0, 18.0, 28.0); let t = u.cam.w;
  var p = vec3f(h1(i), h1(i + 7.3), h1(i + 13.1)) * b; p.y -= t * (20.0 + 6.0 * h1(i + 3.0)); p = p + vec3f(t * 1.8, 0.0, t * 0.9);
  p = wrap(p - u.cam.xyz, b) - b * 0.5 + u.cam.xyz; p += vec3f(0.1, -1.25, 0.05) * e;
  return u.vp * vec4f(p, 1.0);
}
@fragment fn fs_rain() -> @location(0) vec4f { return vec4f(0.72, 0.8, 0.92, 0.17 * u.weather.y); }
