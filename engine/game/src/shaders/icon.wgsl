// Icônes d'inventaire : les mêmes maillages et les mêmes textures que le monde, sous une lumière de studio.
@group(0) @binding(0) var t_alb: texture_2d_array<f32>;
@group(0) @binding(1) var smp: sampler;
const STONE0: i32 = 16; const MOSSY0: i32 = 20; const GRAVEL0: i32 = 32; const BARK0: i32 = 40; const LEAF0: i32 = 52;
struct VI { @location(0) pos: vec4i, @location(1) nrm: vec4f, @location(2) lit: vec4f, @location(3) tint: vec4f, @location(5) m0: vec4f, @location(6) m1: vec4f, @location(7) m2: vec4f, @location(8) m3: vec4f };
struct VO { @builtin(position) clip: vec4f, @location(0) local: vec3f, @location(1) nl: vec3f, @location(2) nv: vec3f, @location(3) ao: f32, @location(4) tint: vec3f, @location(5) @interpolate(flat) data: vec3i };
fn hash13(q: vec3f) -> f32 { var p = fract(q * 0.1031); p += dot(p, p.zyx + 31.32); return fract((p.x + p.y) * p.z); }
@vertex fn vs_icon(v: VI) -> VO {
  var o: VO; let lp = vec3f(v.pos.xyz) / 128.0; o.clip = mat4x4f(v.m0, v.m1, v.m2, v.m3) * vec4f(lp, 1.0);
  // La matrice étire x, y et la profondeur différemment : on retrouve la normale vue en divisant chaque ligne.
  let q = mat3x3f(v.m0.xyz, v.m1.xyz, v.m2.xyz) * v.nrm.xyz; let sx = length(vec3f(v.m0.x, v.m1.x, v.m2.x)); let sz = length(vec3f(v.m0.z, v.m1.z, v.m2.z));
  o.nv = vec3f(q.xy / sx, -q.z / sz); o.nl = v.nrm.xyz; o.local = lp; o.ao = v.lit.x; o.tint = v.tint.rgb * 2.0;
  o.data = vec3i(v.pos.w, i32(v.lit.w * 255.0 + 0.5), i32(v.tint.a * 255.0 + 0.5)); return o;
}
@fragment fn fs_icon(i: VO) -> @location(0) vec4f {
  let kind = i.data.y; let layer0 = i.data.x; let rot = i.data.z; let ln = normalize(i.nl); let ax = abs(ln); let pp = i.local;
  var alb = i.tint;
  if (kind <= 1) {
    var uv0: vec2f;
    if (ax.y > 0.5) { uv0 = pp.xz; } else if (ax.x > 0.5) { uv0 = vec2f(pp.z, -pp.y); } else { uv0 = vec2f(pp.x, -pp.y); }
    var f = fract(uv0 * 0.999 + vec2f(0.0005)); if (rot == 1) { f = f.yx; }
    let a = textureSampleLevel(t_alb, smp, f, layer0, 1.0);
    alb = a.rgb * a.rgb * i.tint; alb = mix(vec3f(dot(alb, vec3f(0.3, 0.6, 0.1))), alb, 0.86);
    if (layer0 >= 4 && layer0 < 16) { alb = mix(vec3f(dot(alb, vec3f(0.3, 0.6, 0.1))), alb, 0.74) * vec3f(0.84, 0.82, 0.86); } // terre : brune, pas orange
    if (layer0 >= STONE0 && layer0 < GRAVEL0) { let e = min(f, vec2f(1.0) - f); alb *= 0.8 + 0.2 * smoothstep(0.0, 0.04, min(e.x, e.y)); }
    // même mousse en petits pixels que dans le monde : c'est elle qui distingue la pierre moussue
    if (layer0 >= MOSSY0 && layer0 < MOSSY0 + 4) {
      let g = floor(pp * 9.0) / 9.0; let mn = hash13(g * 13.0 + vec3f(1.5)) * 0.8 + select(0.0, 0.12, ln.y > 0.5);
      if (mn > 0.72) { alb = vec3f(0.13, 0.22, 0.045) * (0.75 + 0.7 * hash13(g * 7.0 + vec3f(3.0))); }
    }
    if (kind == 1) { alb = mix(vec3f(dot(alb, vec3f(0.3, 0.6, 0.1))), alb, 0.8) * vec3f(0.74, 0.8, 0.74); }
  } else {
    alb *= alb * 1.25;
    if (kind == 3) { alb = mix(vec3f(dot(alb, vec3f(0.3, 0.6, 0.1))), alb, 0.72); }
    if (kind == 4) { alb *= 1.3; }
  }
  let l = normalize(vec3f(-0.3, 0.72, 0.62)); let n = normalize(i.nv); let shade = 0.38 + 0.62 * max(dot(n, l), abs(n.z) * 0.35);
  let c = sqrt(clamp(alb * i.ao * shade * 1.45, vec3f(0.0), vec3f(1.0))); return vec4f(c, 1.0);
}
