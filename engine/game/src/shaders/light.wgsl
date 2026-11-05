@group(1) @binding(0) var t_a: texture_2d<f32>;
@group(1) @binding(1) var t_n: texture_2d<f32>;
@group(1) @binding(2) var t_l: texture_2d<f32>;
@group(1) @binding(3) var t_d: texture_depth_2d;
@group(1) @binding(4) var t_ao: texture_2d<f32>;
@fragment fn fs_light(i: FOut) -> @location(0) vec4f {
  let px = vec2i(i.pos.xy); let d = textureLoad(t_d, px, 0);
  let wp = world_from(i.uv, max(d, 1e-7)); let vv = normalize(wp - u.cam.xyz);
  if (d <= 0.0) {
    var s = sky_color(vv); let mu = dot(vv, u.sun.xyz); s += u.sun_col.rgb * 22.0 * smoothstep(0.99955, 0.99985, mu);
    if (u.hor.w > 0.0 && vv.y > 0.0) { let g = floor(vv * 520.0); s += vec3f(0.8, 0.87, 1.0) * step(0.9985, hash13(g)) * (0.25 + 0.75 * hash13(g + vec3f(2.0))) * 0.8 * u.hor.w * smoothstep(0.0, 0.15, vv.y); }
    return vec4f(s, 1.0);
  }
  let a = textureLoad(t_a, px, 0); let nt = textureLoad(t_n, px, 0); let l = textureLoad(t_l, px, 0);
  let n = normalize(nt.xyz * 2.0 - vec3f(1.0)); let alb = a.rgb; let rough = nt.a; let sky_l = l.x; let blk = l.y; let trans = l.z; let emis = l.w; let vao = a.a;
  if (emis > 0.5) { return vec4f(alb * 5.0, 1.0); }
  let dist = length(wp - u.cam.xyz); let ss = textureSampleLevel(t_ao, s_clamp, i.uv, 0.0).r;
  let sh = shadow_at(wp, n, dist, i.pos.xy, 8) * cloud_shade(wp);
  let nol = dot(n, u.sun.xyz); let wrp = select(sat(nol), sat((nol + 0.45) / 1.45), trans > 0.0);
  let direct = u.sun_col.rgb * wrp * sh;
  let back = pow(sat(dot(vv, u.sun.xyz)), 5.0) * trans;
  let bl = u.sun_col.rgb * alb * vec3f(1.2, 1.4, 0.6) * back * sh * 0.9;
  let sky2 = sky_l * sky_l * (3.0 - 2.0 * sky_l); let vis = vao * mix(1.0, ss, 0.9);
  let sky_amb = mix(u.ground.rgb * 0.9, mix(u.hor.rgb, u.top.rgb, 0.4) * vec3f(1.0, 0.95, 0.82) * 0.78, n.y * 0.5 + 0.5);
  let canopy = vec3f(0.27, 0.36, 0.13) * mix(1.0, 0.5, n.y * 0.5 + 0.5);
  let amb = mix(canopy * 1.75, sky_amb, sky2 * 0.8 + 0.2) * (0.6 + 0.4 * sky_l) * vis;
  let blkc = vec3f(1.0, 0.62, 0.28) * pow(blk, 2.1) * 2.2 * vao;
  var c = alb * (direct + amb + blkc) + bl;
  let hv = normalize(u.sun.xyz - vv); let nh = sat(dot(n, hv));
  c += u.sun_col.rgb * pow(nh, mix(90.0, 10.0, rough)) * (1.0 - rough) * 0.9 * sh * sat(nol) * 0.5;
  return vec4f(c, 1.0);
}
