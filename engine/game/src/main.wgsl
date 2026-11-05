// Rendu du moteur final, premier palier : matériaux, lumière du ciel et du soleil, voxel light, brume, eau simple.
// Les ombres et l'atmosphère du prototype viendront aux paliers suivants.
struct U { vp: mat4x4f, ivp: mat4x4f, cam: vec4f, sun: vec4f, sun_col: vec4f, top: vec4f, hor: vec4f, ground: vec4f };
@group(0) @binding(0) var<uniform> u: U;
@group(0) @binding(1) var t_alb: texture_2d_array<f32>;
@group(0) @binding(2) var t_nrm: texture_2d_array<f32>;
@group(0) @binding(3) var s_tex: sampler;

// Couches de matériaux : vérifiées par un test du cœur.
const STONE0: i32 = 16; const MOSSY0: i32 = 20; const GRAVEL0: i32 = 32; const BARK0: i32 = 40; const BEECH0: i32 = 48; const LEAF0: i32 = 52;

struct VIn { @location(0) pos: vec4i, @location(1) nrm: vec4f, @location(2) lit: vec4f, @location(3) tint: vec4f, @location(4) org: vec4f };
struct VOut {
  @builtin(position) clip: vec4f, @location(0) w: vec3f, @location(1) n: vec3f, @location(2) lit: vec3f, @location(3) tint: vec3f,
  @location(4) @interpolate(flat) data: vec3i,
};

fn hash13(q: vec3f) -> f32 { var p = fract(q * 0.1031); p += dot(p, p.zyx + 31.32); return fract((p.x + p.y) * p.z); }
fn wind(p: vec3f, w: f32) -> vec3f {
  if (w <= 0.0) { return vec3f(0.0); }
  let time = u.cam.w; let g = sin(time * 0.7 + p.x * 0.05 + p.z * 0.07) * 0.5 + 0.5; let t = time * 1.8;
  let d = vec3f(sin(t + p.x * 0.45 + p.z * 0.2 + p.y * 0.3), sin(t * 1.3 + p.x * 0.3) * 0.3, cos(t * 0.9 + p.z * 0.4 + p.y * 0.2));
  return d * w * (0.035 + 0.085 * g);
}
@vertex fn vs_main(v: VIn) -> VOut {
  var o: VOut;
  var world = vec3f(v.pos.xyz) * v.org.w + v.org.xyz;
  world += wind(world, v.nrm.w * 2.0);
  o.w = world; o.n = v.nrm.xyz; o.lit = v.lit.xyz; o.tint = v.tint.rgb * 2.0;
  o.data = vec3i(v.pos.w, i32(v.lit.w * 255.0 + 0.5), i32(v.tint.a * 255.0 + 0.5));
  o.clip = u.vp * vec4f(world, 1.0);
  return o;
}
fn sky_color(d: vec3f) -> vec3f {
  let y = max(d.y, 0.0); let t = pow(1.0 - y, 3.6); let mu = clamp(dot(d, u.sun.xyz), 0.0, 1.0);
  var c = mix(u.top.rgb, u.hor.rgb, t);
  c += u.sun_col.rgb * (0.0035 / (1.02 - mu * 0.985) + 0.05 * pow(mu, 8.0)) * (0.25 + 0.75 * t);
  if (d.y < 0.0) { c = mix(c, u.ground.rgb, clamp(-d.y * 7.0, 0.0, 1.0)); }
  return c;
}
fn aces(x: vec3f) -> vec3f { return clamp(x * (2.51 * x + 0.03) / (x * (2.43 * x + 0.59) + 0.14), vec3f(0.0), vec3f(1.0)); }
// Sortie : même étalonnage que le prototype.
fn finish(c0: vec3f, fc: vec2f) -> vec4f {
  var c = pow(aces(c0 * u.sun.w), vec3f(1.0 / 2.2));
  let l = dot(c, vec3f(0.3, 0.59, 0.11));
  c = mix(vec3f(l), c, 1.2);
  c *= mix(vec3f(0.97, 1.0, 1.03), vec3f(1.03, 1.0, 0.95), smoothstep(0.1, 0.8, l));
  c = clamp(c, vec3f(0.0), vec3f(1.0));
  c = mix(c, c * c * (3.0 - 2.0 * c), 0.22);
  return vec4f(c, 1.0);
}
fn fogged(c: vec3f, w: vec3f) -> vec3f {
  let r = w - u.cam.xyz; let len = length(r); let v = r / len;
  let air = sky_color(normalize(vec3f(v.x, max(v.y, 0.03), v.z)));
  let a = 1.0 - exp(-max(len - 40.0, 0.0) * u.ground.w);
  let edge = smoothstep(205.0, 310.0, len); // bord du monde chargé : il se dissout dans le ciel
  return mix(mix(c, air, a), air, edge);
}
@fragment fn fs_main(i: VOut, @builtin(front_facing) ff: bool) -> @location(0) vec4f {
  var n = normalize(i.n);
  let kind = i.data.y; let layer0 = i.data.x; let rot = i.data.z;
  if (!ff && kind >= 2) { n = vec3f(n.x, abs(n.y), n.z); }
  let ax = abs(n);
  var uv0: vec2f; var tt: vec3f; var bb: vec3f;
  if (ax.y > 0.5) { uv0 = i.w.xz; tt = vec3f(1.0, 0.0, 0.0); bb = vec3f(0.0, 0.0, 1.0); }
  else if (ax.x > 0.5) { uv0 = vec2f(i.w.z, -i.w.y); tt = vec3f(0.0, 0.0, 1.0); bb = vec3f(0.0, -1.0, 0.0); }
  else { uv0 = vec2f(i.w.x, -i.w.y); tt = vec3f(1.0, 0.0, 0.0); bb = vec3f(0.0, -1.0, 0.0); }
  var dx = dpdx(uv0); var dy = dpdy(uv0);
  var alb = i.tint; var rough = 0.9; var trans = 0.0; var emis = 0.0; var nn = n;
  if (kind <= 1) {
    let cell = floor(i.w - n * 0.02); let h = hash13(cell + vec3f(0.5)); let h2 = hash13(cell.zxy + vec3f(7.5));
    var f = fract(uv0);
    if (rot == 1 || (rot == 0 && ax.y > 0.5 && h2 > 0.5)) { f = f.yx; dx = dx.yx; dy = dy.yx; let q = tt; tt = bb; bb = q; }
    let l = layer0 + (i32(h * 4.0) & 3);
    let a = textureSampleGrad(t_alb, s_tex, f, l, dx, dy); let m = textureSampleGrad(t_nrm, s_tex, f, l, dx, dy);
    alb = a.rgb * a.rgb * i.tint;
    alb = mix(vec3f(dot(alb, vec3f(0.3, 0.6, 0.1))), alb, 0.86);
    let bark = layer0 >= BARK0 && layer0 < LEAF0;
    alb *= select(0.9 + 0.2 * h2, 0.84 + 0.32 * h2, bark);
    if (bark || (layer0 >= STONE0 && layer0 < GRAVEL0)) { let e = min(f, vec2f(1.0) - f); alb *= 0.8 + 0.2 * smoothstep(0.0, 0.04, min(e.x, e.y)); }
    let d = m.xy * 2.0 - 1.0; nn = normalize(n + (tt * d.x + bb * d.y) * 0.9); rough = m.b;
    if (kind == 1) { trans = 1.0; rough = 0.5; alb *= 1.05; }
  } else {
    let k = 0.92 + 0.16 * hash13(floor(i.w * 7.0)); alb *= alb * k * 1.25;
    if (kind == 3) { trans = 0.45; alb = mix(vec3f(dot(alb, vec3f(0.3, 0.6, 0.1))), alb, 0.72); }
    if (kind == 4) { emis = 1.0; }
  }
  if (emis > 0.5) { return finish(fogged(alb * 5.0, i.w), i.clip.xy); }
  let vao = i.lit.x; let sky_l = i.lit.y; let blk = i.lit.z;
  let v = normalize(i.w - u.cam.xyz);
  let nol = dot(nn, u.sun.xyz);
  let wrp = select(clamp(nol, 0.0, 1.0), clamp((nol + 0.45) / 1.45, 0.0, 1.0), trans > 0.0);
  let sh = smoothstep(0.35, 0.9, sky_l); // en attendant les vraies ombres : la lumière du ciel tient lieu de masque
  let direct = u.sun_col.rgb * wrp * sh * 0.5; // moitié : sans ombres portées, tout le relief reçoit le soleil
  let back = pow(clamp(dot(v, u.sun.xyz), 0.0, 1.0), 5.0) * trans;
  let bl = u.sun_col.rgb * alb * vec3f(1.2, 1.4, 0.6) * back * sh * 0.9;
  let sky2 = sky_l * sky_l * (3.0 - 2.0 * sky_l);
  let sky_amb = mix(u.ground.rgb * 0.9, mix(u.hor.rgb, u.top.rgb, 0.4) * vec3f(1.0, 0.95, 0.82) * 0.78, nn.y * 0.5 + 0.5);
  let canopy = vec3f(0.27, 0.36, 0.13) * mix(1.0, 0.5, nn.y * 0.5 + 0.5);
  let amb = mix(canopy * 1.75, sky_amb, sky2 * 0.8 + 0.2) * (0.6 + 0.4 * sky_l) * vao;
  let blkc = vec3f(1.0, 0.62, 0.28) * pow(blk, 2.1) * 2.2 * vao;
  var c = alb * (direct + amb + blkc) + bl;
  let hv = normalize(u.sun.xyz - v);
  c += u.sun_col.rgb * pow(clamp(dot(nn, hv), 0.0, 1.0), mix(90.0, 10.0, rough)) * (1.0 - rough) * 0.9 * sh * clamp(nol, 0.0, 1.0) * 0.5;
  return finish(fogged(c, i.w), i.clip.xy);
}
@fragment fn fs_water(i: VOut) -> @location(0) vec4f {
  let t = u.cam.w; let fall = 1.0 - abs(normalize(i.n).y);
  var n = normalize(i.n);
  if (fall < 0.5) {
    let p = i.w.xz; let a = sin(p.x * 1.1 + t * 1.3) + sin(p.y * 1.4 - t * 1.1) + 0.5 * sin((p.x + p.y) * 2.7 + t * 2.1); let b = cos(p.x * 1.1 + t * 1.3) - cos(p.y * 1.4 - t * 1.1);
    n = normalize(vec3f(0.035 * b, 1.0, 0.035 * a));
  }
  let v = normalize(i.w - u.cam.xyz); let r = reflect(v, n); let rr = vec3f(r.x, abs(r.y), r.z);
  let fr = 0.03 + 0.97 * pow(1.0 - clamp(dot(-v, n), 0.0, 1.0), 5.0);
  let lit = u.sun_col.rgb * 0.35 * clamp(u.sun.y * 2.0, 0.0, 1.0) + mix(u.hor.rgb, u.top.rgb, 0.5) * 0.7 * i.lit.y;
  var c = mix(vec3f(0.05, 0.16, 0.16) * lit, sky_color(rr) * mix(0.22, 1.0, i.lit.y), fr);
  var alpha = 0.72 + 0.28 * fr;
  if (fall > 0.5) { let s = fract(sin(floor((i.w.x + i.w.z) * 5.0) * 12.9898) * 43758.5453); let st = smoothstep(0.55, 0.8, fract(i.w.y * 0.35 + t * 0.9 + s)); c = mix(c, vec3f(0.8, 0.88, 0.9) * lit, 0.25 + 0.4 * st); alpha = 0.85; }
  let foam = smoothstep(0.2, 1.5, i.tint.r) * smoothstep(0.35, 0.7, hash13(floor(vec3f(i.w.x, t * 3.0, i.w.z) * 3.0)));
  c = mix(c, vec3f(1.0) * lit, foam * step(fall, 0.5) * 0.7);
  let o = finish(fogged(c, i.w), i.clip.xy);
  return vec4f(o.rgb, alpha);
}
struct SOut { @builtin(position) clip: vec4f, @location(0) ndc: vec2f };
@vertex fn vs_sky(@builtin(vertex_index) id: u32) -> SOut {
  let p = vec2f(f32((id << 1u) & 2u), f32(id & 2u)) * 2.0 - 1.0;
  var o: SOut; o.clip = vec4f(p, 0.0, 1.0); o.ndc = p; return o;
}
@fragment fn fs_sky(i: SOut) -> @location(0) vec4f {
  let w = u.ivp * vec4f(i.ndc, 1.0, 1.0); let v = normalize(w.xyz / w.w - u.cam.xyz);
  var s = sky_color(v);
  s += u.sun_col.rgb * 22.0 * smoothstep(0.99955, 0.99985, dot(v, u.sun.xyz));
  return finish(s, i.clip.xy);
}
