//! Pipeline d'image : ombres en cascades, G-buffer, SSAO, lumière, nuages, brume, eau, composition, TAA, bloom, sortie.
//! Tout ce qui touche à wgpu est ici ; le reste du jeu n'en dépend pas.
use alw_core::{actors::{Draw, Models}, mesher::VSIZE, session::order, stream::{Built, FAR}, textures, view::{self, Sky, M4}, weather::Env};
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;
use crate::{font_data as fd, hud};

const MAX_QUADS: u32 = 1_000_000; const MAX_SLOTS: u64 = 2048; pub const LIGHTS: usize = 48; const FLOATS: usize = 200 + LIGHTS * 8 + 16; // … + les boîtes de l'aperçu de pose const MAX_ACTORS: u64 = 512; const FALL_BYTES: u64 = 1500 * 24 * VSIZE as u64; const UI_VERTS: u64 = 60_000; pub const CASCADES: [f64; 3] = [26.0, 90.0, 300.0]; pub const CASCADES_ULTRA: [f64; 3] = [30.0, 120.0, 440.0]; const DECO_SHADOW: f32 = 52.0;
pub const ICON_CELL: u32 = 128; pub const ICON_COLS: u32 = 16; // 256 icônes au plus (atlas 2048 × 2048)
const HDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float; const LDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm; const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub struct Options { pub shadow: u32, pub fallback: bool, pub hole: f32, pub cascades: [f64; 3] }
struct Part { buf: wgpu::Buffer, indices: u32 }
struct ChunkGpu { opaque: Option<Part>, deco: Option<Part>, water: Option<Part>, slot: u64, min: [f32; 3], max: [f32; 3], c: [f32; 2] }
struct Far { part: Part, slot: u64 }
/// Passe plein écran : son pipeline et le groupe de textures à lier.
struct Full { pipe: wgpu::RenderPipeline, layout: wgpu::BindGroupLayout }
struct Pipes { rain: wgpu::RenderPipeline, actor: wgpu::RenderPipeline, actor_shadow: wgpu::RenderPipeline, line: wgpu::RenderPipeline, ui: wgpu::RenderPipeline, geom_o: wgpu::RenderPipeline, geom_d: wgpu::RenderPipeline, shadow_o: wgpu::RenderPipeline, shadow_d: wgpu::RenderPipeline, water: wgpu::RenderPipeline, water_layout: wgpu::BindGroupLayout, ssao: Full, blur_h: Full, blur_v: Full, light: Full, clouds: wgpu::RenderPipeline, volume: Full, compose: Full, taa: Full, first: Full, down: Full, up: Full, last: Full }
struct Rt { tex: wgpu::Texture, view: wgpu::TextureView }
struct Targets {
    a: Rt, n: Rt, l: Rt, d: Rt, d2: Rt, hdr: Rt, scene: Rt, ao: Rt, ao2: Rt, vol: Rt, cl: Rt, comp: Rt, hist: [Rt; 2], bloom: Vec<Rt>,
    b_ssao: wgpu::BindGroup, b_blur_h: wgpu::BindGroup, b_blur_v: wgpu::BindGroup, b_light: wgpu::BindGroup, b_vol: wgpu::BindGroup, b_water: wgpu::BindGroup, b_comp: wgpu::BindGroup, b_taa: [wgpu::BindGroup; 2], b_first: [wgpu::BindGroup; 2], b_down: Vec<wgpu::BindGroup>, b_up: Vec<wgpu::BindGroup>, b_last: [wgpu::BindGroup; 2],
}
pub struct Gpu {
    pub lights: Vec<([f32; 3], [f32; 3])>,
    surface: Option<wgpu::Surface<'static>>, device: wgpu::Device, queue: wgpu::Queue, config: wgpu::SurfaceConfiguration, format: wgpu::TextureFormat,
    p: Pipes, t: Targets, fall_vb: wgpu::Buffer, fall_serial: u64, actor_vb: wgpu::Buffer, actor_inst: wgpu::Buffer, ui_buf: wgpu::Buffer, ui_u: wgpu::Buffer, ui_bind: wgpu::BindGroup, frame0: wgpu::BindGroup, geom1: wgpu::BindGroup, ubuf: wgpu::Buffer, ibuf: wgpu::Buffer, inst: wgpu::Buffer,
    shadow_views: Vec<wgpu::TextureView>, shadow_bind: Vec<wgpu::BindGroup>, shadow_buf: Vec<wgpu::Buffer>, shadow_vp: [M4; 3], shadow_size: u32, hole: f32, cascades: [f64; 3],
    chunks: HashMap<(i32, i32), ChunkGpu>, far: Option<Far>, free: Vec<u64>, frame: u64, prev_vp: M4, reset: bool, capture: Option<Rt>,
}
fn bytes_f32(v: &[f32]) -> Vec<u8> { v.iter().flat_map(|f| f.to_le_bytes()).collect() }
fn part(d: &wgpu::Device, data: &[u8], n: usize) -> Option<Part> {
    if n == 0 { return None; }
    let buf = d.create_buffer_init(&BufferInitDescriptor { label: None, contents: data, usage: wgpu::BufferUsages::VERTEX });
    Some(Part { buf, indices: (n as u32 / 4).min(MAX_QUADS) * 6 })
}
fn rt(d: &wgpu::Device, w: u32, h: u32, f: wgpu::TextureFormat, copy: bool) -> Rt {
    let mut usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
    if copy { usage |= wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST; }
    let tex = d.create_texture(&wgpu::TextureDescriptor { label: None, size: wgpu::Extent3d { width: w.max(1), height: h.max(1), depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: f, usage, view_formats: &[] });
    let view = tex.create_view(&Default::default()); Rt { tex, view }
}
fn texture_array(d: &wgpu::Device, q: &wgpu::Queue, base: &[u8], size: u32, layers: u32) -> wgpu::TextureView {
    let mips = textures::mip_chain(base, size as usize, layers as usize);
    let t = d.create_texture(&wgpu::TextureDescriptor { label: Some("matériaux"), size: wgpu::Extent3d { width: size, height: size, depth_or_array_layers: layers }, mip_level_count: mips.len() as u32, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: LDR, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[] });
    for (level, data) in mips.iter().enumerate() {
        let s = size >> level;
        q.write_texture(wgpu::TexelCopyTextureInfo { texture: &t, mip_level: level as u32, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All }, data, wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4 * s), rows_per_image: Some(s) }, wgpu::Extent3d { width: s, height: s, depth_or_array_layers: layers });
    }
    t.create_view(&wgpu::TextureViewDescriptor { dimension: Some(wgpu::TextureViewDimension::D2Array), ..Default::default() })
}
fn tex_layout(d: &wgpu::Device, kinds: &str) -> wgpu::BindGroupLayout {
    let entries: Vec<_> = kinds.bytes().enumerate().map(|(i, k)| wgpu::BindGroupLayoutEntry { binding: i as u32, visibility: wgpu::ShaderStages::FRAGMENT, count: None, ty: wgpu::BindingType::Texture {
        sample_type: if k == b'D' { wgpu::TextureSampleType::Depth } else { wgpu::TextureSampleType::Float { filterable: true } },
        view_dimension: if k == b'A' { wgpu::TextureViewDimension::D2Array } else { wgpu::TextureViewDimension::D2 }, multisampled: false } }).collect();
    d.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &entries })
}
fn tex_bind(d: &wgpu::Device, l: &wgpu::BindGroupLayout, views: &[&wgpu::TextureView]) -> wgpu::BindGroup {
    let entries: Vec<_> = views.iter().enumerate().map(|(i, v)| wgpu::BindGroupEntry { binding: i as u32, resource: wgpu::BindingResource::TextureView(v) }).collect();
    d.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: l, entries: &entries })
}
fn targets(d: &wgpu::Device, p: &Pipes, w: u32, h: u32) -> Targets {
    let (hw, hh, qw, qh) = ((w / 2).max(1), (h / 2).max(1), (w / 4).max(1), (h / 4).max(1));
    let a = rt(d, w, h, LDR, false); let n = rt(d, w, h, LDR, false); let l = rt(d, w, h, LDR, false); let dp = rt(d, w, h, DEPTH, true); let d2 = rt(d, w, h, DEPTH, true);
    let hdr = rt(d, w, h, HDR, true); let scene = rt(d, w, h, HDR, true); let ao = rt(d, hw, hh, LDR, false); let ao2 = rt(d, hw, hh, LDR, false); let vol = rt(d, hw, hh, HDR, false); let cl = rt(d, qw, qh, HDR, false); let comp = rt(d, w, h, HDR, false);
    let hist = [rt(d, w, h, HDR, false), rt(d, w, h, HDR, false)];
    let (mut bw, mut bh, mut bloom) = (hw, hh, Vec::new()); for _ in 0..5 { bloom.push(rt(d, bw.max(2), bh.max(2), HDR, false)); bw = (bw / 2).max(2); bh = (bh / 2).max(2); }
    let b_ssao = tex_bind(d, &p.ssao.layout, &[&dp.view, &n.view]); let b_blur_h = tex_bind(d, &p.blur_h.layout, &[&ao.view, &dp.view]); let b_blur_v = tex_bind(d, &p.blur_v.layout, &[&ao2.view, &dp.view]);
    let b_light = tex_bind(d, &p.light.layout, &[&a.view, &n.view, &l.view, &dp.view, &ao.view]); let b_vol = tex_bind(d, &p.volume.layout, &[&dp.view]); let b_water = tex_bind(d, &p.water_layout, &[&scene.view, &d2.view]);
    let b_comp = tex_bind(d, &p.compose.layout, &[&hdr.view, &vol.view, &cl.view, &dp.view]);
    let b_taa = [tex_bind(d, &p.taa.layout, &[&comp.view, &hist[1].view, &dp.view, &l.view]), tex_bind(d, &p.taa.layout, &[&comp.view, &hist[0].view, &dp.view, &l.view])];
    let b_first = [tex_bind(d, &p.first.layout, &[&hist[0].view]), tex_bind(d, &p.first.layout, &[&hist[1].view])];
    let b_down = (1..5).map(|i| tex_bind(d, &p.down.layout, &[&bloom[i - 1].view])).collect(); let b_up = (1..5).map(|i| tex_bind(d, &p.up.layout, &[&bloom[i].view])).collect();
    let b_last = [tex_bind(d, &p.last.layout, &[&hist[0].view, &bloom[0].view]), tex_bind(d, &p.last.layout, &[&hist[1].view, &bloom[0].view])];
    Targets { a, n, l, d: dp, d2, hdr, scene, ao, ao2, vol, cl, comp, hist, bloom, b_ssao, b_blur_h, b_blur_v, b_light, b_vol, b_water, b_comp, b_taa, b_first, b_down, b_up, b_last }
}
fn clear_color(v: &wgpu::TextureView, load: wgpu::LoadOp<wgpu::Color>) -> Option<wgpu::RenderPassColorAttachment<'_>> { Some(wgpu::RenderPassColorAttachment { view: v, resolve_target: None, ops: wgpu::Operations { load, store: wgpu::StoreOp::Store } }) }
const BLACK: wgpu::LoadOp<wgpu::Color> = wgpu::LoadOp::Clear(wgpu::Color::BLACK);
fn fullscreen(enc: &mut wgpu::CommandEncoder, pipe: &wgpu::RenderPipeline, g0: &wgpu::BindGroup, g1: Option<&wgpu::BindGroup>, out: &wgpu::TextureView, load: wgpu::LoadOp<wgpu::Color>) {
    let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor { label: None, color_attachments: &[clear_color(out, load)], depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None });
    p.set_pipeline(pipe); p.set_bind_group(0, g0, &[]); if let Some(g) = g1 { p.set_bind_group(1, g, &[]); } p.draw(0..3, 0..1);
}
impl Gpu {
    pub async fn new(window: Option<Arc<Window>>, width: u32, height: u32, o: Options, models: &Models) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: wgpu::Backends::PRIMARY, ..Default::default() });
        let surface = window.map(|w| instance.create_surface(w).expect("surface de rendu"));
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: surface.as_ref(), force_fallback_adapter: o.fallback }).await.expect("aucune carte graphique compatible");
        println!("Carte : {:?}", adapter.get_info());
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor { label: None, required_features: wgpu::Features::empty(), required_limits: wgpu::Limits::default(), memory_hints: wgpu::MemoryHints::Performance }, None).await.expect("ouverture de la carte graphique");
        device.on_uncaptured_error(Box::new(|e: wgpu::Error| crate::report(&format!("Erreur graphique : {e}"))));
        // Le shader fait lui-même sa correction gamma, comme le prototype.
        let format = surface.as_ref().map(|s| { let f = s.get_capabilities(&adapter).formats; f.iter().copied().find(|x| !x.is_srgb()).unwrap_or(f[0]) }).unwrap_or(LDR);
        let mut config = wgpu::SurfaceConfiguration { usage: wgpu::TextureUsages::RENDER_ATTACHMENT, format, width: width.max(1), height: height.max(1), present_mode: wgpu::PresentMode::AutoNoVsync, desired_maximum_frame_latency: 2, alpha_mode: wgpu::CompositeAlphaMode::Auto, view_formats: vec![] };
        if let Some(s) = &surface { config.alpha_mode = s.get_capabilities(&adapter).alpha_modes[0]; s.configure(&device, &config); }
        let d = &device;
        let tex = textures::build_textures(256); /* 256 pixels par bloc : un grain de roche net de près */
        let alb = texture_array(d, &queue, &tex.albedo, tex.size, tex.layers); let nrm = texture_array(d, &queue, &tex.nrm, tex.size, tex.layers);
        const NZ: u32 = 40;
        let noise_t = d.create_texture(&wgpu::TextureDescriptor { label: Some("bruit"), size: wgpu::Extent3d { width: NZ, height: NZ, depth_or_array_layers: NZ }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D3, format: wgpu::TextureFormat::R8Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[] });
        queue.write_texture(wgpu::TexelCopyTextureInfo { texture: &noise_t, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All }, &textures::build_noise_3d(NZ as usize), wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(NZ), rows_per_image: Some(NZ) }, wgpu::Extent3d { width: NZ, height: NZ, depth_or_array_layers: NZ });
        let noise_v = noise_t.create_view(&Default::default());
        let shadow_t = d.create_texture(&wgpu::TextureDescriptor { label: Some("ombres"), size: wgpu::Extent3d { width: o.shadow, height: o.shadow, depth_or_array_layers: 3 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: DEPTH, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[] });
        let shadow_all = shadow_t.create_view(&wgpu::TextureViewDescriptor { dimension: Some(wgpu::TextureViewDimension::D2Array), ..Default::default() });
        let shadow_views: Vec<_> = (0..3).map(|i| shadow_t.create_view(&wgpu::TextureViewDescriptor { dimension: Some(wgpu::TextureViewDimension::D2), base_array_layer: i, array_layer_count: Some(1), ..Default::default() })).collect();
        let lin = |m: wgpu::AddressMode, an: u16| wgpu::SamplerDescriptor { address_mode_u: m, address_mode_v: m, address_mode_w: m, mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear, anisotropy_clamp: an, ..Default::default() };
        let s_rep = d.create_sampler(&lin(wgpu::AddressMode::Repeat, 8)); let s_clamp = d.create_sampler(&lin(wgpu::AddressMode::ClampToEdge, 1));
        let s_cmp = d.create_sampler(&wgpu::SamplerDescriptor { compare: Some(wgpu::CompareFunction::LessEqual), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
        let ubuf = d.create_buffer(&wgpu::BufferDescriptor { label: Some("image"), size: (FLOATS * 4) as u64, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let idx: Vec<u8> = (0..MAX_QUADS).flat_map(|i| { let v = i * 4; [v, v + 1, v + 2, v, v + 2, v + 3] }).flat_map(|v| v.to_le_bytes()).collect();
        let ibuf = d.create_buffer_init(&BufferInitDescriptor { label: Some("indices"), contents: &idx, usage: wgpu::BufferUsages::INDEX });
        let inst = d.create_buffer(&wgpu::BufferDescriptor { label: Some("origines"), size: MAX_SLOTS * 16, usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let vf = wgpu::ShaderStages::VERTEX_FRAGMENT; let e = |binding, ty| wgpu::BindGroupLayoutEntry { binding, visibility: vf, ty, count: None };
        let g0 = d.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("image"), entries: &[
            e(0, wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }),
            e(1, wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D3, multisampled: false }),
            e(2, wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)),
            e(3, wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Depth, view_dimension: wgpu::TextureViewDimension::D2Array, multisampled: false }),
            e(4, wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison)),
            e(5, wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)),
        ] });
        let frame0 = d.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &g0, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: ubuf.as_entire_binding() }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&noise_v) }, wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&s_rep) },
            wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&shadow_all) }, wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&s_cmp) }, wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::Sampler(&s_clamp) },
        ] });
        let common = include_str!("shaders/common.wgsl");
        let module = |src: &str| d.create_shader_module(wgpu::ShaderModuleDescriptor { label: None, source: wgpu::ShaderSource::Wgsl(format!("{common}\n{src}").into()) });
        let vattrs = [wgpu::VertexAttribute { format: wgpu::VertexFormat::Sint16x4, offset: 0, shader_location: 0 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Snorm8x4, offset: 8, shader_location: 1 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Unorm8x4, offset: 12, shader_location: 2 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Unorm8x4, offset: 16, shader_location: 3 }];
        let iattrs = [wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 0, shader_location: 4 }];
        let buffers = [wgpu::VertexBufferLayout { array_stride: VSIZE as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &vattrs }, wgpu::VertexBufferLayout { array_stride: 16, step_mode: wgpu::VertexStepMode::Instance, attributes: &iattrs }];
        let ds = |write: bool, cmp: wgpu::CompareFunction| Some(wgpu::DepthStencilState { format: DEPTH, depth_write_enabled: write, depth_compare: cmp, stencil: Default::default(), bias: Default::default() });
        let prim = |cull: Option<wgpu::Face>| wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, cull_mode: cull, ..Default::default() };
        // Géométrie
        let geom_l = tex_layout(d, "AA"); let geom1 = tex_bind(d, &geom_l, &[&alb, &nrm]);
        let geom_pl = d.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&g0, &geom_l], push_constant_ranges: &[] });
        let m_geom = module(include_str!("shaders/geom.wgsl"));
        let gtargets = [Some(LDR.into()), Some(LDR.into()), Some(LDR.into())];
        let geom = |cull| d.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("géométrie"), layout: Some(&geom_pl), vertex: wgpu::VertexState { module: &m_geom, entry_point: Some("vs_geom"), compilation_options: Default::default(), buffers: &buffers }, fragment: Some(wgpu::FragmentState { module: &m_geom, entry_point: Some("fs_geom"), compilation_options: Default::default(), targets: &gtargets }), primitive: prim(cull), depth_stencil: ds(true, wgpu::CompareFunction::Greater), multisample: Default::default(), multiview: None, cache: None });
        let (geom_o, geom_d) = (geom(Some(wgpu::Face::Back)), geom(None));
        let aattrs = [wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 0, shader_location: 5 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 16, shader_location: 6 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 32, shader_location: 7 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 48, shader_location: 8 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 64, shader_location: 9 }];
        let abuf = [wgpu::VertexBufferLayout { array_stride: VSIZE as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &vattrs }, wgpu::VertexBufferLayout { array_stride: 80, step_mode: wgpu::VertexStepMode::Instance, attributes: &aattrs }];
        let actor = d.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("personnage"), layout: Some(&geom_pl), vertex: wgpu::VertexState { module: &m_geom, entry_point: Some("vs_actor"), compilation_options: Default::default(), buffers: &abuf }, fragment: Some(wgpu::FragmentState { module: &m_geom, entry_point: Some("fs_geom"), compilation_options: Default::default(), targets: &gtargets }), primitive: prim(None), depth_stencil: ds(true, wgpu::CompareFunction::Greater), multisample: Default::default(), multiview: None, cache: None });
        // Ombres
        let s_l = d.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }] });
        let s_pl = d.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&s_l], push_constant_ranges: &[] });
        let m_sh = d.create_shader_module(wgpu::ShaderModuleDescriptor { label: None, source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shadow.wgsl").into()) });
        let shadow = |cull| d.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("ombres"), layout: Some(&s_pl), vertex: wgpu::VertexState { module: &m_sh, entry_point: Some("vs_shadow"), compilation_options: Default::default(), buffers: &buffers }, fragment: None, primitive: prim(cull), depth_stencil: ds(true, wgpu::CompareFunction::Less), multisample: Default::default(), multiview: None, cache: None });
        let (shadow_o, shadow_d) = (shadow(Some(wgpu::Face::Back)), shadow(None));
        let actor_shadow = d.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("ombre du personnage"), layout: Some(&s_pl), vertex: wgpu::VertexState { module: &m_sh, entry_point: Some("vs_shadow_actor"), compilation_options: Default::default(), buffers: &abuf }, fragment: None, primitive: prim(None), depth_stencil: ds(true, wgpu::CompareFunction::Less), multisample: Default::default(), multiview: None, cache: None });
        let shadow_buf: Vec<_> = (0..3).map(|_| d.create_buffer(&wgpu::BufferDescriptor { label: None, size: 80, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false })).collect();
        let shadow_bind: Vec<_> = shadow_buf.iter().map(|b| d.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &s_l, entries: &[wgpu::BindGroupEntry { binding: 0, resource: b.as_entire_binding() }] })).collect();
        // Passes plein écran
        let full = |src: &str, kinds: &str, fs: &str, fmt: wgpu::TextureFormat, blend: Option<wgpu::BlendState>| -> Full {
            let m = module(src); let layout = tex_layout(d, kinds);
            let groups: Vec<&wgpu::BindGroupLayout> = if kinds.is_empty() { vec![&g0] } else { vec![&g0, &layout] };
            let pl = d.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &groups, push_constant_ranges: &[] });
            let pipe = d.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some(fs), layout: Some(&pl), vertex: wgpu::VertexState { module: &m, entry_point: Some("vs_full"), compilation_options: Default::default(), buffers: &[] }, fragment: Some(wgpu::FragmentState { module: &m, entry_point: Some(fs), compilation_options: Default::default(), targets: &[Some(wgpu::ColorTargetState { format: fmt, blend, write_mask: wgpu::ColorWrites::ALL })] }), primitive: prim(None), depth_stencil: None, multisample: Default::default(), multiview: None, cache: None });
            Full { pipe, layout }
        };
        let add = Some(wgpu::BlendState { color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::One, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add }, alpha: wgpu::BlendComponent::OVER });
        let bloom = include_str!("shaders/bloom.wgsl"); let blur = include_str!("shaders/blur.wgsl");
        let water_layout = tex_layout(d, "TD");
        let w_pl = d.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&g0, &water_layout], push_constant_ranges: &[] });
        let m_w = module(include_str!("shaders/water.wgsl"));
        let water = d.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("eau"), layout: Some(&w_pl), vertex: wgpu::VertexState { module: &m_w, entry_point: Some("vs_water"), compilation_options: Default::default(), buffers: &buffers }, fragment: Some(wgpu::FragmentState { module: &m_w, entry_point: Some("fs_water"), compilation_options: Default::default(), targets: &[Some(HDR.into())] }), primitive: prim(None), depth_stencil: ds(false, wgpu::CompareFunction::Greater), multisample: Default::default(), multiview: None, cache: None });
        let m_l = module(include_str!("shaders/line.wgsl"));
        let line_pl = d.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&g0], push_constant_ranges: &[] });
        let line = d.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("contour"), layout: Some(&line_pl), vertex: wgpu::VertexState { module: &m_l, entry_point: Some("vs_line"), compilation_options: Default::default(), buffers: &[] }, fragment: Some(wgpu::FragmentState { module: &m_l, entry_point: Some("fs_line"), compilation_options: Default::default(), targets: &[Some(HDR.into())] }), primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::LineList, ..Default::default() }, depth_stencil: ds(false, wgpu::CompareFunction::Greater), multisample: Default::default(), multiview: None, cache: None });
        // Interface
        let font_tex = d.create_texture_with_data(&queue, &wgpu::TextureDescriptor { label: Some("police"), size: wgpu::Extent3d { width: fd::W, height: fd::H, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::R8Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[] }, wgpu::util::TextureDataOrder::LayerMajor, include_bytes!("../assets/font.r8"));
        let font_view = font_tex.create_view(&Default::default());
        // Icônes : chaque objet est rendu une fois en vraie 3D avec ses textures, sur fond transparent.
        let icon_size = wgpu::Extent3d { width: ICON_CELL * ICON_COLS, height: ICON_CELL * ICON_COLS, depth_or_array_layers: 1 };
        let icon_tex = d.create_texture(&wgpu::TextureDescriptor { label: Some("icônes"), size: icon_size, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: LDR, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[] });
        let icon_view = icon_tex.create_view(&Default::default());
        let ui_buf = d.create_buffer(&wgpu::BufferDescriptor { label: Some("interface"), size: UI_VERTS * hud::VERT as u64, usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let ui_u = d.create_buffer(&wgpu::BufferDescriptor { label: None, size: 16, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let ui_l = d.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
        ] });
        let ui_bind = d.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &ui_l, entries: &[wgpu::BindGroupEntry { binding: 0, resource: ui_u.as_entire_binding() }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&font_view) }, wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&icon_view) }, wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&s_clamp) }] });
        let ui_pl = d.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&ui_l], push_constant_ranges: &[] });
        let m_ui = d.create_shader_module(wgpu::ShaderModuleDescriptor { label: None, source: wgpu::ShaderSource::Wgsl(include_str!("shaders/ui.wgsl").into()) });
        let ui_attrs = [wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x2, offset: 0, shader_location: 0 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x2, offset: 8, shader_location: 1 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 16, shader_location: 2 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32, offset: 32, shader_location: 3 }];
        let ui = d.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("interface"), layout: Some(&ui_pl), vertex: wgpu::VertexState { module: &m_ui, entry_point: Some("vs_ui"), compilation_options: Default::default(), buffers: &[wgpu::VertexBufferLayout { array_stride: hud::VERT as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &ui_attrs }] }, fragment: Some(wgpu::FragmentState { module: &m_ui, entry_point: Some("fs_ui"), compilation_options: Default::default(), targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })] }), primitive: prim(None), depth_stencil: None, multisample: Default::default(), multiview: None, cache: None });
        let m_rain = module(include_str!("shaders/rain.wgsl"));
        let rain = d.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("pluie"), layout: Some(&line_pl), vertex: wgpu::VertexState { module: &m_rain, entry_point: Some("vs_rain"), compilation_options: Default::default(), buffers: &[] }, fragment: Some(wgpu::FragmentState { module: &m_rain, entry_point: Some("fs_rain"), compilation_options: Default::default(), targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState { color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::SrcAlpha, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add }, alpha: wgpu::BlendComponent::OVER }), write_mask: wgpu::ColorWrites::ALL })] }), primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::LineList, ..Default::default() }, depth_stencil: None, multisample: Default::default(), multiview: None, cache: None });
        let clouds = full(include_str!("shaders/clouds.wgsl"), "", "fs_clouds", HDR, None).pipe;
        let p = Pipes { rain, actor, actor_shadow, line, ui, geom_o, geom_d, shadow_o, shadow_d, water, water_layout,
            ssao: full(include_str!("shaders/ssao.wgsl"), "DT", "fs_ssao", LDR, None), blur_h: full(blur, "TD", "fs_h", LDR, None), blur_v: full(blur, "TD", "fs_v", LDR, None),
            light: full(include_str!("shaders/light.wgsl"), "TTTDT", "fs_light", HDR, None), clouds, volume: full(include_str!("shaders/volume.wgsl"), "D", "fs_volume", HDR, None),
            compose: full(include_str!("shaders/compose.wgsl"), "TTTD", "fs_compose", HDR, None), taa: full(include_str!("shaders/taa.wgsl"), "TTDT", "fs_taa", HDR, None),
            first: full(bloom, "T", "fs_first", HDR, None), down: full(bloom, "T", "fs_down", HDR, None), up: full(bloom, "T", "fs_up", HDR, add),
            last: full(include_str!("shaders/final.wgsl"), "TT", "fs_final", format, None) };
        let t = targets(d, &p, config.width, config.height);
        let actor_vb = d.create_buffer_init(&BufferInitDescriptor { label: Some("modèles"), contents: &models.data, usage: wgpu::BufferUsages::VERTEX });
        let actor_inst = d.create_buffer(&wgpu::BufferDescriptor { label: Some("morceaux"), size: MAX_ACTORS * 80, usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        {
            let tex_ms = |f: wgpu::TextureFormat| d.create_texture(&wgpu::TextureDescriptor { label: None, size: icon_size, mip_level_count: 1, sample_count: 4, dimension: wgpu::TextureDimension::D2, format: f, usage: wgpu::TextureUsages::RENDER_ATTACHMENT, view_formats: &[] }).create_view(&Default::default());
            let (msaa, depth) = (tex_ms(LDR), tex_ms(DEPTH));
            let l = d.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2Array, multisampled: false }, count: None }, wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None }] });
            let bg = d.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &l, entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&alb) }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&s_rep) }] });
            let pl = d.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&l], push_constant_ranges: &[] });
            let m = d.create_shader_module(wgpu::ShaderModuleDescriptor { label: None, source: wgpu::ShaderSource::Wgsl(include_str!("shaders/icon.wgsl").into()) });
            let ia = [wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 0, shader_location: 5 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 16, shader_location: 6 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 32, shader_location: 7 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 48, shader_location: 8 }];
            let vb = [wgpu::VertexBufferLayout { array_stride: VSIZE as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &vattrs }, wgpu::VertexBufferLayout { array_stride: 64, step_mode: wgpu::VertexStepMode::Instance, attributes: &ia }];
            let pipe = d.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("icônes"), layout: Some(&pl), vertex: wgpu::VertexState { module: &m, entry_point: Some("vs_icon"), compilation_options: Default::default(), buffers: &vb }, fragment: Some(wgpu::FragmentState { module: &m, entry_point: Some("fs_icon"), compilation_options: Default::default(), targets: &[Some(LDR.into())] }), primitive: prim(None), depth_stencil: ds(true, wgpu::CompareFunction::Less), multisample: wgpu::MultisampleState { count: 4, ..Default::default() }, multiview: None, cache: None });
            let mats: Vec<f32> = order().iter().flat_map(|id| models.icon_matrix(id)).collect();
            let ib = d.create_buffer_init(&BufferInitDescriptor { label: None, contents: &bytes_f32(&mats), usage: wgpu::BufferUsages::VERTEX });
            let mut enc = d.create_command_encoder(&Default::default());
            {
                let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("icônes"), color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &msaa, resolve_target: Some(&icon_view), ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } })], depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), timestamp_writes: None, occlusion_query_set: None });
                p.set_pipeline(&pipe); p.set_bind_group(0, &bg, &[]); p.set_index_buffer(ibuf.slice(..), wgpu::IndexFormat::Uint32); p.set_vertex_buffer(0, actor_vb.slice(..)); p.set_vertex_buffer(1, ib.slice(..));
                for (i, id) in order().iter().enumerate() {
                    let (cx, cy) = (i as u32 % ICON_COLS, i as u32 / ICON_COLS); let r = models.held(id).expect("objet").r; let c = ICON_CELL as f32;
                    p.set_viewport(cx as f32 * c, cy as f32 * c, c, c, 0.0, 1.0); p.draw_indexed(r.first..r.first + r.count, 0, i as u32..i as u32 + 1);
                }
            }
            queue.submit([enc.finish()]);
        }
        let fall_vb = d.create_buffer(&wgpu::BufferDescriptor { label: Some("arbre qui tombe"), size: FALL_BYTES, usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let capture = if surface.is_none() { Some(rt(d, config.width, config.height, LDR, true)) } else { None };
        let free = (1..MAX_SLOTS).rev().collect();
        Gpu { lights: Vec::new(), surface, p, t, fall_vb, fall_serial: 0, actor_vb, actor_inst, ui_buf, ui_u, ui_bind, frame0, geom1, ubuf, ibuf, inst, shadow_views, shadow_bind, shadow_buf, shadow_vp: [[0.0; 16]; 3], shadow_size: o.shadow, hole: o.hole, cascades: o.cascades, chunks: HashMap::new(), far: None, free, frame: 0, prev_vp: [0.0; 16], reset: true, capture, device, queue, config, format }
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        if w == 0 || h == 0 { return; }
        self.config.width = w; self.config.height = h;
        if let Some(s) = &self.surface { s.configure(&self.device, &self.config); }
        self.t = targets(&self.device, &self.p, w, h); self.reset = true;
    }
    /// Envoie le tronc en train de tomber, une seule fois par chute.
    pub fn set_fall(&mut self, serial: u64, data: &[u8]) { if serial != self.fall_serial { self.fall_serial = serial; self.queue.write_buffer(&self.fall_vb, 0, &data[..data.len().min(FALL_BYTES as usize)]); } }
    fn set_origin(&self, slot: u64, o: [f32; 4]) { self.queue.write_buffer(&self.inst, slot * 16, &bytes_f32(&o)); }
    pub fn upload(&mut self, b: Built) {
        let m = &b.mesh; let (x, z) = (b.key.0 as f32 * 32.0, b.key.1 as f32 * 32.0);
        if b.lod == FAR {
            if let Some(part) = part(&self.device, &m.opaque, m.n_o) { self.set_origin(0, [x, 0.0, z, 1.0 / 8.0]); self.far = Some(Far { part, slot: 0 }); self.reset = true; }
            return;
        }
        let old = self.chunks.remove(&b.key); let slot = match old { Some(c) => c.slot, None => match self.free.pop() { Some(s) => s, None => return } };
        self.set_origin(slot, [x, 0.0, z, 1.0 / 128.0]);
        self.chunks.insert(b.key, ChunkGpu { opaque: part(&self.device, &m.opaque, m.n_o), deco: part(&self.device, &m.deco, m.n_d), water: part(&self.device, &m.water, m.n_w), slot, min: [x - 2.0, m.ymin as f32 - 2.0, z - 2.0], max: [x + 34.0, m.ymax as f32 + 2.0, z + 34.0], c: [x + 16.0, z + 16.0] });
    }
    pub fn forget(&mut self, k: (i32, i32)) { if let Some(c) = self.chunks.remove(&k) { self.free.push(c.slot); } }
    /// Une image complète. Rend le nombre de triangles envoyés.
    pub fn render(&mut self, vp: &M4, view_m: &M4, proj: &M4, eye: [f64; 3], time: f64, s: &Sky, env: Env, aim: Option<([i32; 3], f32)>, ghost: Option<([i32; 3], bool)>, gbox: &[[f64; 6]], draws: &[Draw], ui: &hud::Ui) -> usize {
        let frame = match &self.surface { Some(sf) => match sf.get_current_texture() { Ok(f) => Some(f), Err(_) => { sf.configure(&self.device, &self.config); return 0; } }, None => None };
        let out_view = frame.as_ref().map(|f| f.texture.create_view(&Default::default()));
        let (w, h) = (self.config.width as f32, self.config.height as f32); self.frame += 1; let fr = self.frame; let mut tris = 0usize;
        let hal = |mut i: u64, b: u64| { let (mut f, mut x) = (1.0f32, 0.0f32); while i > 0 { f /= b as f32; x += f * (i % b) as f32; i /= b; } x };
        let jit = [(hal(fr % 8 + 1, 2) - 0.5) * 2.0 / w, (hal(fr % 8 + 1, 3) - 0.5) * 2.0 / h];
        let mut enc = self.device.create_command_encoder(&Default::default());
        let draws = &draws[..draws.len().min(MAX_ACTORS as usize)]; let mut inst: Vec<f32> = Vec::with_capacity(draws.len() * 20);
        for d in draws { inst.extend_from_slice(&d.m); inst.extend_from_slice(&[d.view as u8 as f32, d.local as u8 as f32, d.hero as u8 as f32, 0.0]); }
        if !inst.is_empty() { self.queue.write_buffer(&self.actor_inst, 0, &bytes_f32(&inst)); }
        // Ombres : la cascade proche à chaque image, les lointaines à tour de rôle.
        let list_all: Vec<&ChunkGpu> = self.chunks.values().collect();
        for i in 0..3usize {
            let step = [1u64, 2, 4][i];
            if !((fr + i as u64) % step == 0 || self.reset || fr < 6) { continue; }
            let sv = view::shadow_cascade(s.dir, eye, self.cascades[i], self.shadow_size as f64); self.shadow_vp[i] = sv;
            let mut data = sv.to_vec(); data.extend_from_slice(&[time as f32, 0.0, 0.0, 0.0]); self.queue.write_buffer(&self.shadow_buf[i], 0, &bytes_f32(&data));
            let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("ombres"), color_attachments: &[], depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &self.shadow_views[i], depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), timestamp_writes: None, occlusion_query_set: None });
            p.set_bind_group(0, &self.shadow_bind[i], &[]); p.set_index_buffer(self.ibuf.slice(..), wgpu::IndexFormat::Uint32);
            let r = self.cascades[i] as f32 + 40.0; let (ex, ez) = (eye[0] as f32, eye[2] as f32);
            p.set_pipeline(&self.p.shadow_o);
            for c in &list_all { let (dx, dz) = (c.c[0] - ex, c.c[1] - ez); if dx * dx + dz * dz > r * r { continue; } if let Some(q) = &c.opaque { p.set_vertex_buffer(0, q.buf.slice(..)); p.set_vertex_buffer(1, self.inst.slice(c.slot * 16..c.slot * 16 + 16)); p.draw_indexed(0..q.indices, 0, 0..1); tris += q.indices as usize / 3; } }
            if i == 0 {
                p.set_pipeline(&self.p.actor_shadow); p.set_vertex_buffer(0, self.actor_vb.slice(..)); p.set_vertex_buffer(1, self.actor_inst.slice(..));
                for (k, d) in draws.iter().enumerate() { if !d.view && !d.dynamic { p.draw_indexed(d.r.first..d.r.first + d.r.count, 0, k as u32..k as u32 + 1); } }
                p.set_vertex_buffer(0, self.fall_vb.slice(..)); for (k, d) in draws.iter().enumerate() { if d.dynamic { p.draw_indexed(d.r.first..d.r.first + d.r.count, 0, k as u32..k as u32 + 1); } }
                p.set_pipeline(&self.p.shadow_d);
                for c in &list_all { let (dx, dz) = (c.c[0] - ex, c.c[1] - ez); if dx * dx + dz * dz > DECO_SHADOW * DECO_SHADOW { continue; } if let Some(q) = &c.deco { p.set_vertex_buffer(0, q.buf.slice(..)); p.set_vertex_buffer(1, self.inst.slice(c.slot * 16..c.slot * 16 + 16)); p.draw_indexed(0..q.indices, 0, 0..1); tris += q.indices as usize / 3; } }
            }
        }
        // Constantes de l'image
        let ivp = view::invert(vp); let ip = view::invert(proj); let mut u: Vec<f32> = Vec::with_capacity(FLOATS);
        for m in [vp, &ivp, &self.prev_vp, view_m, proj, &ip, &self.shadow_vp[0], &self.shadow_vp[1], &self.shadow_vp[2]] { u.extend_from_slice(m); }
        u.extend_from_slice(&[self.cascades[0] as f32, self.cascades[1] as f32, self.cascades[2] as f32, 0.0, eye[0] as f32, eye[1] as f32, eye[2] as f32, time as f32, s.dir[0], s.dir[1], s.dir[2], s.exposure, s.col[0], s.col[1], s.col[2], env.cloud, s.top[0], s.top[1], s.top[2], 1.0 / self.shadow_size as f32, s.hor[0], s.hor[1], s.hor[2], s.night, s.ground[0], s.ground[1], s.ground[2], 0.0048, jit[0], jit[1], w, h, (fr % 64) as f32, env.mist, self.hole, if self.reset { 1.0 } else { 0.0 }, aim.map_or(0.0, |a| a.0[0] as f32), aim.map_or(0.0, |a| a.0[1] as f32), aim.map_or(0.0, |a| a.0[2] as f32), aim.map_or(0.0, |a| a.1), ghost.map_or(0.0, |g| g.0[0] as f32), ghost.map_or(0.0, |g| g.0[1] as f32), ghost.map_or(0.0, |g| g.0[2] as f32), ghost.map_or(0.0, |g| g.1 as u8 as f32), env.fog_k, env.rain, env.bright, env.under as u8 as f32, env.flies, self.lights.len().min(LIGHTS) as f32, env.snow, 0.0, env.valley, env.aurora, 0.0, 0.0]);
        for k in 0..LIGHTS { let (p, c) = self.lights.get(k).copied().unwrap_or(([0.0; 3], [0.0; 3])); u.extend_from_slice(&[p[0], p[1], p[2], 0.0, c[0], c[1], c[2], 0.0]); }
        // Forme de l'aperçu de pose : deux boîtes au plus, relatives à sa case (un cube plein par défaut).
        for k in 0..2 { let b = gbox.get(k).copied().unwrap_or(if k == 0 { [0.0, 0.0, 0.0, 1.0, 1.0, 1.0] } else { [0.0; 6] }); u.extend_from_slice(&[b[0] as f32, b[1] as f32, b[2] as f32, 0.0, b[3] as f32, b[4] as f32, b[5] as f32, 0.0]); }
        self.queue.write_buffer(&self.ubuf, 0, &bytes_f32(&u));
        let list: Vec<&ChunkGpu> = list_all.iter().copied().filter(|c| view::visible(vp, c.min, c.max)).collect();
        let t = &self.t; let g0 = &self.frame0;
        // G-buffer
        {
            let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("géométrie"), color_attachments: &[clear_color(&t.a.view, BLACK), clear_color(&t.n.view, BLACK), clear_color(&t.l.view, BLACK)],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &t.d.view, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), timestamp_writes: None, occlusion_query_set: None });
            p.set_bind_group(0, g0, &[]); p.set_bind_group(1, &self.geom1, &[]); p.set_index_buffer(self.ibuf.slice(..), wgpu::IndexFormat::Uint32);
            p.set_pipeline(&self.p.geom_o);
            for c in &list { if let Some(q) = &c.opaque { p.set_vertex_buffer(0, q.buf.slice(..)); p.set_vertex_buffer(1, self.inst.slice(c.slot * 16..c.slot * 16 + 16)); p.draw_indexed(0..q.indices, 0, 0..1); tris += q.indices as usize / 3; } }
            if let Some(f) = &self.far { p.set_vertex_buffer(0, f.part.buf.slice(..)); p.set_vertex_buffer(1, self.inst.slice(f.slot * 16..f.slot * 16 + 16)); p.draw_indexed(0..f.part.indices, 0, 0..1); tris += f.part.indices as usize / 3; }
            if !draws.is_empty() { p.set_pipeline(&self.p.actor); p.set_vertex_buffer(0, self.actor_vb.slice(..)); p.set_vertex_buffer(1, self.actor_inst.slice(..)); for (k, d) in draws.iter().enumerate() { if !d.dynamic { p.draw_indexed(d.r.first..d.r.first + d.r.count, 0, k as u32..k as u32 + 1); } } p.set_vertex_buffer(0, self.fall_vb.slice(..)); for (k, d) in draws.iter().enumerate() { if d.dynamic { p.draw_indexed(d.r.first..d.r.first + d.r.count, 0, k as u32..k as u32 + 1); } } }
            p.set_pipeline(&self.p.geom_d);
            for c in &list { if let Some(q) = &c.deco { p.set_vertex_buffer(0, q.buf.slice(..)); p.set_vertex_buffer(1, self.inst.slice(c.slot * 16..c.slot * 16 + 16)); p.draw_indexed(0..q.indices, 0, 0..1); tris += q.indices as usize / 3; } }
        }
        let wht = wgpu::LoadOp::Clear(wgpu::Color::WHITE);
        fullscreen(&mut enc, &self.p.ssao.pipe, g0, Some(&t.b_ssao), &t.ao.view, wht);
        fullscreen(&mut enc, &self.p.blur_h.pipe, g0, Some(&t.b_blur_h), &t.ao2.view, wht);
        fullscreen(&mut enc, &self.p.blur_v.pipe, g0, Some(&t.b_blur_v), &t.ao.view, wht);
        fullscreen(&mut enc, &self.p.light.pipe, g0, Some(&t.b_light), &t.hdr.view, BLACK);
        fullscreen(&mut enc, &self.p.clouds, g0, None, &t.cl.view, BLACK);
        fullscreen(&mut enc, &self.p.volume.pipe, g0, Some(&t.b_vol), &t.vol.view, BLACK);
        // Eau : elle lit une copie de la scène et de la profondeur.
        let sz = wgpu::Extent3d { width: self.config.width, height: self.config.height, depth_or_array_layers: 1 };
        enc.copy_texture_to_texture(t.hdr.tex.as_image_copy(), t.scene.tex.as_image_copy(), sz);
        enc.copy_texture_to_texture(t.d.tex.as_image_copy(), t.d2.tex.as_image_copy(), sz);
        {
            let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("eau"), color_attachments: &[clear_color(&t.hdr.view, wgpu::LoadOp::Load)],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &t.d.view, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }), stencil_ops: None }), timestamp_writes: None, occlusion_query_set: None });
            p.set_bind_group(0, g0, &[]); p.set_bind_group(1, &t.b_water, &[]); p.set_index_buffer(self.ibuf.slice(..), wgpu::IndexFormat::Uint32); p.set_pipeline(&self.p.water);
            for c in &list { if let Some(q) = &c.water { p.set_vertex_buffer(0, q.buf.slice(..)); p.set_vertex_buffer(1, self.inst.slice(c.slot * 16..c.slot * 16 + 16)); p.draw_indexed(0..q.indices, 0, 0..1); tris += q.indices as usize / 3; } }
        }
        if aim.is_some() {
            let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("contour"), color_attachments: &[clear_color(&t.hdr.view, wgpu::LoadOp::Load)], depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &t.d.view, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }), stencil_ops: None }), timestamp_writes: None, occlusion_query_set: None });
            p.set_pipeline(&self.p.line); p.set_bind_group(0, g0, &[]); p.draw(0..24, 0..if ghost.is_some() { 2 } else { 1 }); if ghost.is_some() && gbox.len() > 1 { p.draw(0..24, 3..4); } // seconde boîte de l'aperçu (marche, haut de porte)
            if aim.is_some_and(|a| a.1 > 0.0) { p.draw(0..288, 2..3); } // fissures du bloc qu'on casse
        }
        fullscreen(&mut enc, &self.p.compose.pipe, g0, Some(&t.b_comp), &t.comp.view, BLACK);
        let cur = (fr & 1) as usize;
        fullscreen(&mut enc, &self.p.taa.pipe, g0, Some(&t.b_taa[cur]), &t.hist[cur].view, BLACK);
        fullscreen(&mut enc, &self.p.first.pipe, g0, Some(&t.b_first[cur]), &t.bloom[0].view, BLACK);
        for i in 1..5 { fullscreen(&mut enc, &self.p.down.pipe, g0, Some(&t.b_down[i - 1]), &t.bloom[i].view, BLACK); }
        for i in (1..5).rev() { fullscreen(&mut enc, &self.p.up.pipe, g0, Some(&t.b_up[i - 1]), &t.bloom[i - 1].view, wgpu::LoadOp::Load); }
        let target = match (&out_view, &self.capture) { (Some(v), _) => v, (None, Some(c)) => &c.view, _ => unreachable!() };
        fullscreen(&mut enc, &self.p.last.pipe, g0, Some(&t.b_last[cur]), target, BLACK);
        let fall = env.rain.max(env.snow);
        if fall > 0.02 && !env.covered {
            let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("pluie"), color_attachments: &[clear_color(target, wgpu::LoadOp::Load)], depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None });
            p.set_pipeline(&self.p.rain); p.set_bind_group(0, g0, &[]); p.draw(0..(fall * if env.snow > 0.01 { 1600.0 } else { 1100.0 }) as u32 * 2, 0..1);
        }
        let verts = (ui.data.len() / 9).min(UI_VERTS as usize);
        if verts > 0 {
            self.queue.write_buffer(&self.ui_u, 0, &bytes_f32(&[w, h, 0.0, 0.0])); self.queue.write_buffer(&self.ui_buf, 0, &bytes_f32(&ui.data[..verts * 9]));
            let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("interface"), color_attachments: &[clear_color(target, wgpu::LoadOp::Load)], depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None });
            p.set_pipeline(&self.p.ui); p.set_bind_group(0, &self.ui_bind, &[]); p.set_vertex_buffer(0, self.ui_buf.slice(..)); p.draw(0..verts as u32, 0..1);
        }
        self.queue.submit([enc.finish()]); if let Some(f) = frame { f.present(); }
        self.prev_vp = *vp; self.reset = false;
        tris
    }
    /// Laisse la carte finir l'image en cours : la carte logicielle du banc d'essai décroche si on l'empile.
    pub fn settle(&self) { let _ = self.device.poll(wgpu::Maintain::Wait); }
    /// Lit l'image hors écran en BGR, ligne du bas d'abord, pour un fichier BMP.
    pub fn read_bgr(&self) -> (u32, u32, Vec<u8>) {
        let (w, h) = (self.config.width, self.config.height); let row = ((w * 4 + 255) / 256) * 256;
        let buf = self.device.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * h) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let mut enc = self.device.create_command_encoder(&Default::default());
        enc.copy_texture_to_buffer(self.capture.as_ref().expect("mode capture").tex.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(h) } }, wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
        self.queue.submit([enc.finish()]);
        let s = buf.slice(..); s.map_async(wgpu::MapMode::Read, |_| {}); let _ = self.device.poll(wgpu::Maintain::Wait);
        let d = s.get_mapped_range(); let pad = (4 - (w * 3) % 4) % 4; let mut o = Vec::with_capacity(((w * 3 + pad) * h) as usize);
        for y in (0..h).rev() { for x in 0..w { let p = (y * row + x * 4) as usize; o.extend_from_slice(&[d[p + 2], d[p + 1], d[p]]); } for _ in 0..pad { o.push(0); } }
        (w, h, o)
    }
    #[allow(dead_code)] pub fn format(&self) -> wgpu::TextureFormat { self.format }
}
