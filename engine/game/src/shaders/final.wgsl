@group(1) @binding(0) var t_t: texture_2d<f32>;
@group(1) @binding(1) var t_b: texture_2d<f32>;
fn aces(x: vec3f) -> vec3f { return clamp(x * (2.51 * x + vec3f(0.03)) / (x * (2.43 * x + vec3f(0.59)) + vec3f(0.14)), vec3f(0.0), vec3f(1.0)); }
@fragment fn fs_final(i: FOut) -> @location(0) vec4f {
  let px = vec2i(i.pos.xy); let dim = vec2i(textureDimensions(t_t)) - vec2i(1);
  var c = textureLoad(t_t, px, 0).rgb;
  let nb = (textureLoad(t_t, min(px + vec2i(1, 0), dim), 0).rgb + textureLoad(t_t, max(px - vec2i(1, 0), vec2i(0)), 0).rgb + textureLoad(t_t, min(px + vec2i(0, 1), dim), 0).rgb + textureLoad(t_t, max(px - vec2i(0, 1), vec2i(0)), 0).rgb) * 0.25;
  c = max(c + (c - nb) * 0.32, vec3f(0.0));
  c += textureSampleLevel(t_b, s_clamp, i.uv, 0.0).rgb * 0.04;
  c = pow(aces(c * u.sun.w * u.weather.z), vec3f(1.0 / 2.2));
  let l = dot(c, vec3f(0.3, 0.59, 0.11));
  c = mix(vec3f(l), c, 1.06);
  c *= mix(vec3f(0.97, 1.0, 1.03), vec3f(1.03, 1.0, 0.95), smoothstep(0.1, 0.8, l));
  c = clamp(c, vec3f(0.0), vec3f(1.0));
  c = mix(c, c * c * (vec3f(3.0) - 2.0 * c), 0.14);
  c += vec3f((ign(i.pos.xy) - 0.5) / 255.0);
  return vec4f(c, 1.0);
}
