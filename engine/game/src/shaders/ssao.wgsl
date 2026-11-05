@group(1) @binding(0) var t_d: texture_depth_2d;
@group(1) @binding(1) var t_n: texture_2d<f32>;
fn view_at(p: vec2f) -> vec3f { let d = depth_at(t_d, p); let c = u.iproj * vec4f(p.x * 2.0 - 1.0, 1.0 - p.y * 2.0, max(d, 1e-7), 1.0); return c.xyz / c.w; }
@fragment fn fs_ssao(i: FOut) -> @location(0) vec4f {
  if (depth_at(t_d, i.uv) <= 0.0) { return vec4f(1.0); }
  let pp = view_at(i.uv); let dist = -pp.z; if (dist > 180.0) { return vec4f(1.0); }
  let v3 = mat3x3f(u.view[0].xyz, u.view[1].xyz, u.view[2].xyz);
  let px = vec2i(i.uv * vec2f(textureDimensions(t_n)));
  let nn = normalize(v3 * (textureLoad(t_n, px, 0).xyz * 2.0 - vec3f(1.0)));
  let a = ign(i.pos.xy + u.misc.x * 5.588) * 6.283; var occ = 0.0; let rr = clamp(0.55 + dist * 0.03, 0.5, 3.0);
  for (var k = 0; k < 10; k++) {
    let f = (f32(k) + 0.5) / 10.0; let t = a + f * 38.8; let r = sqrt(f) * rr;
    var dir = normalize(vec3f(cos(t), sin(t), 0.4)); if (dot(dir, nn) < 0.0) { dir = -dir; }
    let q = pp + dir * r + nn * 0.04; let c = u.proj * vec4f(q, 1.0); let s = to_uv(c.xy / c.w);
    let qq = view_at(s); let dz = qq.z - q.z;
    occ += step(0.03, dz) * (1.0 - smoothstep(rr * 0.6, rr * 2.0, abs(pp.z - qq.z)));
  }
  let ao = 1.0 - occ / 10.0 * 1.25; return vec4f(pow(sat(ao), 1.4));
}
