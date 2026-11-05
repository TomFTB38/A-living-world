// G-buffer : matériaux du monde. Comme le prototype, couche par couche.
@group(1) @binding(0) var t_alb: texture_2d_array<f32>;
@group(1) @binding(1) var t_nrm: texture_2d_array<f32>;
const STONE0: i32 = 16; const MOSSY0: i32 = 20; const GRAVEL0: i32 = 32; const BARK0: i32 = 40; const BEECH0: i32 = 48; const LEAF0: i32 = 52;

struct VIn { @location(0) pos: vec4i, @location(1) nrm: vec4f, @location(2) lit: vec4f, @location(3) tint: vec4f, @location(4) org: vec4f };
struct VOut { @builtin(position) clip: vec4f, @location(0) w: vec3f, @location(1) n: vec3f, @location(2) lit: vec3f, @location(3) tint: vec3f, @location(4) @interpolate(flat) data: vec3i, @location(5) local: vec3f, @location(6) nl: vec3f, @location(7) @interpolate(flat) flags: vec3f };
fn wind(p: vec3f, w: f32) -> vec3f {
  if (w <= 0.0) { return vec3f(0.0); }
  let time = u.cam.w; let g = sin(time * 0.7 + p.x * 0.05 + p.z * 0.07) * 0.5 + 0.5; let t = time * 1.8;
  let d = vec3f(sin(t + p.x * 0.45 + p.z * 0.2 + p.y * 0.3), sin(t * 1.3 + p.x * 0.3) * 0.3, cos(t * 0.9 + p.z * 0.4 + p.y * 0.2));
  return d * w * (0.035 + 0.085 * g);
}
@vertex fn vs_geom(v: VIn) -> VOut {
  var o: VOut;
  var world = vec3f(v.pos.xyz) * v.org.w + v.org.xyz;
  world += wind(world, v.nrm.w * 2.0);
  if (v.org.w > 0.1 && length(world.xz - u.cam.xz) < u.misc.z) { world.y -= 90.0; } // horizon : trou sous les chunks chargés
  o.w = world; o.n = v.nrm.xyz; o.lit = v.lit.xyz; o.tint = v.tint.rgb * 2.0; o.local = world; o.nl = o.n; o.flags = vec3f(0.0);
  o.data = vec3i(v.pos.w, i32(v.lit.w * 255.0 + 0.5), i32(v.tint.a * 255.0 + 0.5));
  o.clip = u.vp * vec4f(world, 1.0); let w = o.clip.w; o.clip = vec4f(o.clip.xy + u.jit.xy * w, o.clip.z, w);
  return o;
}
// Personnage et objets : une matrice par morceau. flags : première personne, texture liée à l'objet, héros.
struct AIn { @location(0) pos: vec4i, @location(1) nrm: vec4f, @location(2) lit: vec4f, @location(3) tint: vec4f, @location(5) m0: vec4f, @location(6) m1: vec4f, @location(7) m2: vec4f, @location(8) m3: vec4f, @location(9) flags: vec4f };
@vertex fn vs_actor(v: AIn) -> VOut {
  var o: VOut; let lp = vec3f(v.pos.xyz) / 128.0; let m = mat4x4f(v.m0, v.m1, v.m2, v.m3); let world = (m * vec4f(lp, 1.0)).xyz;
  o.w = world; o.n = (mat3x3f(m[0].xyz, m[1].xyz, m[2].xyz) * v.nrm.xyz); o.lit = v.lit.xyz; o.tint = v.tint.rgb * 2.0; o.local = lp; o.nl = v.nrm.xyz; o.flags = v.flags.xyz;
  o.data = vec3i(v.pos.w, i32(v.lit.w * 255.0 + 0.5), i32(v.tint.a * 255.0 + 0.5));
  var c = u.vp * vec4f(world, 1.0); let w = c.w;
  if (v.flags.x > 0.5) { c.z = (1.0 - (1.0 - c.z / w) * 0.05) * w; } // en première personne, la main passe devant le monde
  o.clip = vec4f(c.xy + u.jit.xy * w, c.z, w); return o;
}
struct GOut { @location(0) a: vec4f, @location(1) n: vec4f, @location(2) l: vec4f };
@fragment fn fs_geom(i: VOut, @builtin(front_facing) ff: bool) -> GOut {
  var n = normalize(i.n); let kind = i.data.y; let layer0 = i.data.x; let rot = i.data.z;
  if (!ff && kind >= 2) { n = vec3f(n.x, abs(n.y), n.z); }
  let held = i.flags.y > 0.5; let pp = select(i.w, i.local, held); let ln = select(n, normalize(i.nl), held); let ax = abs(ln);
  var uv0: vec2f; var tt: vec3f; var bb: vec3f;
  if (ax.y > 0.5) { uv0 = pp.xz; tt = vec3f(1.0, 0.0, 0.0); bb = vec3f(0.0, 0.0, 1.0); }
  else if (ax.x > 0.5) { uv0 = vec2f(pp.z, -pp.y); tt = vec3f(0.0, 0.0, 1.0); bb = vec3f(0.0, -1.0, 0.0); }
  else { uv0 = vec2f(pp.x, -pp.y); tt = vec3f(1.0, 0.0, 0.0); bb = vec3f(0.0, -1.0, 0.0); }
  var dx = dpdx(uv0); var dy = dpdy(uv0); let fine = length(dpdx(i.w));
  var alb = i.tint; var rough = 0.9; var trans = 0.0; var emis = 0.0; var nn = n;
  if (kind <= 1) {
    let cell = floor(pp - ln * 0.02); let h = hash13(cell + vec3f(0.5)); let h2 = hash13(cell.zxy + vec3f(7.5));
    var f = fract(uv0);
    if (rot == 1 || (rot == 0 && ax.y > 0.5 && h2 > 0.5)) { f = f.yx; dx = dx.yx; dy = dy.yx; let q = tt; tt = bb; bb = q; }
    let l = layer0 + (i32(h * 4.0) & 3);
    let a = textureSampleGrad(t_alb, s_rep, f, l, dx, dy); let m = textureSampleGrad(t_nrm, s_rep, f, l, dx, dy);
    alb = a.rgb * a.rgb * i.tint;
    alb = mix(vec3f(dot(alb, vec3f(0.3, 0.6, 0.1))), alb, 0.86);
    let bark = layer0 >= BARK0 && layer0 < LEAF0;
    alb *= select(0.9 + 0.2 * h2, 0.84 + 0.32 * h2, bark);
    if (bark || (layer0 >= STONE0 && layer0 < GRAVEL0)) { let e = min(f, vec2f(1.0) - f); alb *= 0.8 + 0.2 * smoothstep(0.0, 0.04, min(e.x, e.y)); }
    let d = m.xy * 2.0 - 1.0; if (!held) { nn = normalize(n + (tt * d.x + bb * d.y) * 0.9); } rough = m.b;
    if (kind == 1) { trans = 1.0; rough = 0.5; alb *= 1.05; }
    // mousse en petits pixels sur les pierres et l'écorce
    let mossy_rock = layer0 >= MOSSY0 && layer0 < MOSSY0 + 4;
    let mossable = mossy_rock || (layer0 >= BARK0 && layer0 < BARK0 + 4) || (layer0 >= BEECH0 && layer0 < BEECH0 + 4);
    if (mossable && !held && fine < 0.07) {
      let g = floor(i.w * 9.0) / 9.0; let mn = noise(g * 0.031) * 0.78 + hash13(g) * 0.22 + select(0.0, 0.14, n.y > 0.5);
      let mm = step(select(0.5, 0.7, bark), mn);
      alb = mix(alb, vec3f(0.13, 0.22, 0.045) * (0.75 + 0.7 * hash13(g + vec3f(3.0))), mm); rough = mix(rough, 0.95, mm);
    }
  } else {
    let k = 0.92 + 0.16 * hash13(floor(i.w * 7.0)); alb *= alb * k * 1.25;
    if (kind == 3) { trans = 0.45; alb = mix(vec3f(dot(alb, vec3f(0.3, 0.6, 0.1))), alb, 0.72); }
    if (kind == 4) { emis = 1.0; }
  }
  var o: GOut; o.a = vec4f(alb, i.lit.x); o.n = vec4f(nn * 0.5 + vec3f(0.5), rough); o.l = vec4f(i.lit.y, i.lit.z, trans, max(emis, i.flags.z * 0.4)); return o;
}
