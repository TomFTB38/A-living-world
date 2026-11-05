struct S { vp: mat4x4f, time: vec4f };
@group(0) @binding(0) var<uniform> s: S;
struct VIn { @location(0) pos: vec4i, @location(1) nrm: vec4f, @location(4) org: vec4f };
@vertex fn vs_shadow(v: VIn) -> @builtin(position) vec4f {
  var w = vec3f(v.pos.xyz) * v.org.w + v.org.xyz; let wnd = v.nrm.w * 2.0;
  if (wnd > 0.0) { let t = s.time.x * 1.8; w += vec3f(sin(t + w.x * 0.45 + w.z * 0.2 + w.y * 0.3), 0.0, cos(t * 0.9 + w.z * 0.4 + w.y * 0.2)) * wnd * 0.08; }
  return s.vp * vec4f(w, 1.0);
}
struct AIn { @location(0) pos: vec4i, @location(5) m0: vec4f, @location(6) m1: vec4f, @location(7) m2: vec4f, @location(8) m3: vec4f };
@vertex fn vs_shadow_actor(v: AIn) -> @builtin(position) vec4f { return s.vp * mat4x4f(v.m0, v.m1, v.m2, v.m3) * vec4f(vec3f(v.pos.xyz) / 128.0, 1.0); }
