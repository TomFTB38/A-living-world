@group(1) @binding(0) var t_t: texture_2d<f32>;
fn four(uv: vec2f, e: vec2f) -> vec3f {
  return (textureSampleLevel(t_t, s_clamp, uv + e * vec2f(-1.0, -1.0), 0.0).rgb + textureSampleLevel(t_t, s_clamp, uv + e * vec2f(1.0, -1.0), 0.0).rgb + textureSampleLevel(t_t, s_clamp, uv + e * vec2f(-1.0, 1.0), 0.0).rgb + textureSampleLevel(t_t, s_clamp, uv + e * vec2f(1.0, 1.0), 0.0).rgb) * 0.25;
}
@fragment fn fs_first(i: FOut) -> @location(0) vec4f { var c = four(i.uv, 1.0 / vec2f(textureDimensions(t_t))); let l = dot(c, vec3f(0.3, 0.6, 0.1)); c *= smoothstep(0.95, 2.2, l); return vec4f(min(c, vec3f(9.0)), 1.0); }
@fragment fn fs_down(i: FOut) -> @location(0) vec4f { return vec4f(four(i.uv, 1.0 / vec2f(textureDimensions(t_t))), 1.0); }
@fragment fn fs_up(i: FOut) -> @location(0) vec4f { return vec4f(four(i.uv, 0.25 / vec2f(textureDimensions(t_t))), 1.0); }
