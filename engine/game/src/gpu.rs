//! Tout ce qui touche à wgpu. Le reste du jeu n'en dépend pas.
use alw_core::{mesher::VSIZE, stream::Built, textures, view::{self, Sky, M4}};
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;

const MAX_QUADS: u32 = 1_000_000; const MAX_CHUNKS: u64 = 1024; const UNIFORMS: usize = 56;
struct Part { buf: wgpu::Buffer, indices: u32 }
struct ChunkGpu { opaque: Option<Part>, deco: Option<Part>, water: Option<Part>, org: [f32; 4], min: [f32; 3], max: [f32; 3] }
pub struct Gpu {
    surface: wgpu::Surface<'static>, device: wgpu::Device, queue: wgpu::Queue, config: wgpu::SurfaceConfiguration, depth: wgpu::TextureView,
    sky: wgpu::RenderPipeline, opaque: wgpu::RenderPipeline, deco: wgpu::RenderPipeline, water: wgpu::RenderPipeline,
    bind: wgpu::BindGroup, ubuf: wgpu::Buffer, ibuf: wgpu::Buffer, inst: wgpu::Buffer, chunks: HashMap<(i32, i32), ChunkGpu>,
}
fn bytes_f32(v: &[f32]) -> Vec<u8> { v.iter().flat_map(|f| f.to_le_bytes()).collect() }
fn part(d: &wgpu::Device, data: &[u8], n: usize) -> Option<Part> {
    if n == 0 { return None; }
    let buf = d.create_buffer_init(&BufferInitDescriptor { label: None, contents: data, usage: wgpu::BufferUsages::VERTEX });
    Some(Part { buf, indices: (n as u32 / 4).min(MAX_QUADS) * 6 })
}
fn depth_view(d: &wgpu::Device, w: u32, h: u32) -> wgpu::TextureView {
    d.create_texture(&wgpu::TextureDescriptor { label: Some("profondeur"), size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Depth32Float, usage: wgpu::TextureUsages::RENDER_ATTACHMENT, view_formats: &[] }).create_view(&Default::default())
}
fn texture_array(d: &wgpu::Device, q: &wgpu::Queue, base: &[u8], size: u32, layers: u32) -> wgpu::TextureView {
    let mips = textures::mip_chain(base, size as usize, layers as usize);
    let t = d.create_texture(&wgpu::TextureDescriptor { label: Some("matériaux"), size: wgpu::Extent3d { width: size, height: size, depth_or_array_layers: layers }, mip_level_count: mips.len() as u32, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[] });
    for (level, data) in mips.iter().enumerate() {
        let s = size >> level;
        q.write_texture(wgpu::TexelCopyTextureInfo { texture: &t, mip_level: level as u32, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All }, data, wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4 * s), rows_per_image: Some(s) }, wgpu::Extent3d { width: s, height: s, depth_or_array_layers: layers });
    }
    t.create_view(&wgpu::TextureViewDescriptor { dimension: Some(wgpu::TextureViewDimension::D2Array), ..Default::default() })
}
impl Gpu {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: wgpu::Backends::PRIMARY, ..Default::default() });
        let surface = instance.create_surface(window).expect("surface de rendu");
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false }).await.expect("aucune carte graphique compatible");
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor { label: None, required_features: wgpu::Features::empty(), required_limits: wgpu::Limits::default(), memory_hints: wgpu::MemoryHints::Performance }, None).await.expect("ouverture de la carte graphique");
        device.on_uncaptured_error(Box::new(|e: wgpu::Error| crate::report(&format!("Erreur graphique : {e}"))));
        let caps = surface.get_capabilities(&adapter);
        // Le shader fait lui-même sa correction gamma, comme le prototype.
        let format = caps.formats.iter().copied().find(|f| !f.is_srgb()).unwrap_or(caps.formats[0]);
        let config = wgpu::SurfaceConfiguration { usage: wgpu::TextureUsages::RENDER_ATTACHMENT, format, width: size.width.max(1), height: size.height.max(1), present_mode: wgpu::PresentMode::AutoNoVsync, desired_maximum_frame_latency: 2, alpha_mode: caps.alpha_modes[0], view_formats: vec![] };
        surface.configure(&device, &config);
        let tex = textures::build_textures(128);
        let alb = texture_array(&device, &queue, &tex.albedo, tex.size, tex.layers); let nrm = texture_array(&device, &queue, &tex.nrm, tex.size, tex.layers);
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { address_mode_u: wgpu::AddressMode::Repeat, address_mode_v: wgpu::AddressMode::Repeat, mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear, anisotropy_clamp: 8, ..Default::default() });
        let ubuf = device.create_buffer(&wgpu::BufferDescriptor { label: Some("uniformes"), size: (UNIFORMS * 4) as u64, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let idx: Vec<u8> = (0..MAX_QUADS).flat_map(|i| { let v = i * 4; [v, v + 1, v + 2, v, v + 2, v + 3] }).flat_map(|v| v.to_le_bytes()).collect();
        let ibuf = device.create_buffer_init(&BufferInitDescriptor { label: Some("indices"), contents: &idx, usage: wgpu::BufferUsages::INDEX });
        let inst = device.create_buffer(&wgpu::BufferDescriptor { label: Some("origines"), size: MAX_CHUNKS * 16, usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let vis = wgpu::ShaderStages::VERTEX_FRAGMENT;
        let tex_entry = |binding| wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2Array, multisampled: false }, count: None };
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: vis, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
            tex_entry(1), tex_entry(2),
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
        ] });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &bgl, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: ubuf.as_entire_binding() }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&alb) },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&nrm) }, wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&sampler) },
        ] });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&bgl], push_constant_ranges: &[] });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("main"), source: wgpu::ShaderSource::Wgsl(include_str!("main.wgsl").into()) });
        let vertex_attrs = [
            wgpu::VertexAttribute { format: wgpu::VertexFormat::Sint16x4, offset: 0, shader_location: 0 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Snorm8x4, offset: 8, shader_location: 1 },
            wgpu::VertexAttribute { format: wgpu::VertexFormat::Unorm8x4, offset: 12, shader_location: 2 }, wgpu::VertexAttribute { format: wgpu::VertexFormat::Unorm8x4, offset: 16, shader_location: 3 },
        ];
        let inst_attrs = [wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x4, offset: 0, shader_location: 4 }];
        let buffers = [
            wgpu::VertexBufferLayout { array_stride: VSIZE as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &vertex_attrs },
            wgpu::VertexBufferLayout { array_stride: 16, step_mode: wgpu::VertexStepMode::Instance, attributes: &inst_attrs },
        ];
        let pipeline = |vs: &str, fs: &str, geo: bool, cull: Option<wgpu::Face>, blend: Option<wgpu::BlendState>, write: bool, cmp: wgpu::CompareFunction| {
            let vb: &[wgpu::VertexBufferLayout] = if geo { &buffers } else { &[] };
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(fs), layout: Some(&layout),
                vertex: wgpu::VertexState { module: &module, entry_point: Some(vs), compilation_options: Default::default(), buffers: vb },
                fragment: Some(wgpu::FragmentState { module: &module, entry_point: Some(fs), compilation_options: Default::default(), targets: &[Some(wgpu::ColorTargetState { format, blend, write_mask: wgpu::ColorWrites::ALL })] }),
                primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, cull_mode: cull, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: write, depth_compare: cmp, stencil: Default::default(), bias: Default::default() }),
                multisample: Default::default(), multiview: None, cache: None,
            })
        };
        let g = wgpu::CompareFunction::Greater;
        let sky = pipeline("vs_sky", "fs_sky", false, None, None, false, wgpu::CompareFunction::Always);
        let opaque = pipeline("vs_main", "fs_main", true, Some(wgpu::Face::Back), None, true, g);
        let deco = pipeline("vs_main", "fs_main", true, None, None, true, g);
        let water = pipeline("vs_main", "fs_water", true, None, Some(wgpu::BlendState::ALPHA_BLENDING), false, g);
        let depth = depth_view(&device, config.width, config.height);
        Gpu { surface, device, queue, config, depth, sky, opaque, deco, water, bind, ubuf, ibuf, inst, chunks: HashMap::new() }
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        if w == 0 || h == 0 { return; }
        self.config.width = w; self.config.height = h; self.surface.configure(&self.device, &self.config); self.depth = depth_view(&self.device, w, h);
    }
    pub fn upload(&mut self, b: Built) {
        let d = &self.device; let m = &b.mesh; let (x, z) = (b.key.0 as f32 * 32.0, b.key.1 as f32 * 32.0);
        self.chunks.insert(b.key, ChunkGpu { opaque: part(d, &m.opaque, m.n_o), deco: part(d, &m.deco, m.n_d), water: part(d, &m.water, m.n_w), org: [x, 0.0, z, 1.0 / 128.0], min: [x - 2.0, m.ymin as f32 - 2.0, z - 2.0], max: [x + 34.0, m.ymax as f32 + 2.0, z + 34.0] });
    }
    pub fn forget(&mut self, k: (i32, i32)) { self.chunks.remove(&k); }
    /// Dessine une image et rend le nombre de triangles envoyés.
    pub fn render(&mut self, vp: &M4, eye: [f64; 3], time: f64, s: &Sky) -> usize {
        let frame = match self.surface.get_current_texture() { Ok(f) => f, Err(_) => { self.surface.configure(&self.device, &self.config); return 0; } };
        let color = frame.texture.create_view(&Default::default());
        let ivp = view::invert(vp); let mut u = [0f32; UNIFORMS]; u[..16].copy_from_slice(vp); u[16..32].copy_from_slice(&ivp);
        u[32..48].copy_from_slice(&[eye[0] as f32, eye[1] as f32, eye[2] as f32, time as f32, s.dir[0], s.dir[1], s.dir[2], s.exposure, s.col[0], s.col[1], s.col[2], 0.0, s.top[0], s.top[1], s.top[2], 0.0]);
        u[48..56].copy_from_slice(&[s.hor[0], s.hor[1], s.hor[2], 0.0, s.ground[0], s.ground[1], s.ground[2], 0.0022]);
        self.queue.write_buffer(&self.ubuf, 0, &bytes_f32(&u));
        let list: Vec<&ChunkGpu> = self.chunks.values().filter(|c| view::visible(vp, c.min, c.max)).take(MAX_CHUNKS as usize).collect();
        let mut orgs = Vec::with_capacity(list.len() * 4); for c in &list { orgs.extend_from_slice(&c.org); }
        if !orgs.is_empty() { self.queue.write_buffer(&self.inst, 0, &bytes_f32(&orgs)); }
        let mut enc = self.device.create_command_encoder(&Default::default()); let mut tris = 0usize;
        {
            let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("monde"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &color, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &self.depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }), stencil_ops: None }),
                timestamp_writes: None, occlusion_query_set: None });
            p.set_bind_group(0, &self.bind, &[]);
            p.set_pipeline(&self.sky); p.draw(0..3, 0..1);
            p.set_index_buffer(self.ibuf.slice(..), wgpu::IndexFormat::Uint32);
            for (pipe, pick) in [(&self.opaque, 0), (&self.deco, 1), (&self.water, 2)] {
                p.set_pipeline(pipe);
                for (i, c) in list.iter().enumerate() {
                    let part = match pick { 0 => &c.opaque, 1 => &c.deco, _ => &c.water };
                    if let Some(q) = part {
                        let o = i as u64 * 16;
                        p.set_vertex_buffer(0, q.buf.slice(..)); p.set_vertex_buffer(1, self.inst.slice(o..o + 16));
                        p.draw_indexed(0..q.indices, 0, 0..1); tris += q.indices as usize / 3;
                    }
                }
            }
        }
        self.queue.submit([enc.finish()]); frame.present();
        tris
    }
}
