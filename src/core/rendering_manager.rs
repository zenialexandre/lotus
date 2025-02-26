use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    window::Window
};
use wgpu::{
    include_wgsl,
    vertex_attr_array,
    Adapter,
    Backends,
    Color,
    ColorTargetState,
    ColorWrites,
    BlendState,
    CommandEncoder,
    CommandEncoderDescriptor,
    Device,
    DeviceDescriptor,
    Features,
    Instance,
    InstanceDescriptor,
    Limits,
    LoadOp,
    Operations,
    PowerPreference,
    Queue,
    RenderPassColorAttachment,
    RenderPassDescriptor,
    RequestAdapterOptions,
    StoreOp,
    Surface,
    SurfaceCapabilities,
    SurfaceConfiguration,
    SurfaceError,
    SurfaceTexture,
    TextureFormat,
    TextureUsages,
    TextureView,
    TextureViewDescriptor,
    RenderPipeline,
    RenderPipelineDescriptor,
    ShaderModule,
    PipelineLayoutDescriptor,
    PipelineLayout,
    PipelineCompilationOptions,
    VertexState,
    VertexStepMode,
    VertexBufferLayout,
    VertexAttribute,
    BufferAddress,
    FragmentState,
    PrimitiveState,
    PrimitiveTopology,
    FrontFace,
    Face,
    PolygonMode,
    MultisampleState,
    Buffer,
    BufferUsages,
    IndexFormat,
    TextureSampleType,
    SamplerBindingType,
    BindGroup,
    BindGroupDescriptor,
    BindGroupEntry,
    BindingType,
    BindGroupLayoutDescriptor,
    BindGroupLayout,
    BindGroupLayoutEntry,
    BindingResource,
    ShaderStages,
    util::{
        BufferInitDescriptor,
        DeviceExt
    }
};
use std::sync::Arc;

use super::texture;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    texture_coordinates: [f32; 2]
}

pub(crate) struct State {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    surface_configuration: SurfaceConfiguration,
    pub(crate) physical_size: PhysicalSize<u32>,
    color: Color,
    window: Arc<Window>,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    diffuse_bind_group: BindGroup,
    diffuse_texture: texture::Texture
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], texture_coordinates: [0.4131759, 0.00759614], }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], texture_coordinates: [0.0048659444, 0.43041354], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], texture_coordinates: [0.28081453, 0.949397], }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], texture_coordinates: [0.85967, 0.84732914], }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], texture_coordinates: [0.9414737, 0.2652641], }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4
];

impl Vertex {
    const VERTEX_ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    fn descriptor() -> VertexBufferLayout<'static> {
        return VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRIBUTES
        };
    }
}

impl State {
    pub(crate) async fn new(window: Arc<Window>) -> State {
        let color: Color = Color::WHITE;
        let physical_size: PhysicalSize<u32> = window.inner_size();
        let instance: Instance = Instance::new(&InstanceDescriptor{
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let surface: Surface = instance.create_surface(window.clone()).expect("Failed to create WGPU Surface.");
        let adapter: Adapter = instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                required_features: Features::default(),
                required_limits: Limits::default(),
                label: None,
                memory_hints: Default::default()
            },
            None
        ).await.unwrap();

        let surface_capabilities: SurfaceCapabilities = surface.get_capabilities(&adapter);
        let surface_format: TextureFormat = surface_capabilities.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);
        let surface_configuration: SurfaceConfiguration = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };

        let diffuse_bytes = include_bytes!("../../assets/textures/happy_tree_test.png");
        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy_tree_test.png").unwrap();

        let texture_bind_group_layout: BindGroupLayout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float {
                            filterable: true
                        }
                    },
                    count: None
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None
                },
            ]
        });
        let diffuse_bind_group: BindGroup = device.create_bind_group(&BindGroupDescriptor {
            label: Some("diffuse_bind_group"),
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&diffuse_texture.texture_view)
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&diffuse_texture.sampler)
                }
            ]
        });

        let shader_module: ShaderModule = device.create_shader_module(include_wgsl!("../../assets/shaders/shader.wgsl"));
        let render_pipeline_layout: PipelineLayout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[]
        });
        let render_pipeline: RenderPipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::descriptor()],
                compilation_options: PipelineCompilationOptions::default()
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: surface_configuration.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL
                })],
                compilation_options: PipelineCompilationOptions::default()
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
            cache: None
        });

        let vertex_buffer: Buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX
        });
        let index_buffer: Buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: BufferUsages::INDEX
        });

        let num_indices: u32 = INDICES.len() as u32;

        return Self {
            surface,
            device,
            queue,
            surface_configuration,
            physical_size,
            color,
            window,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture
        };
    }

    pub(crate) fn window(&self) -> &Window {
        return &self.window;
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.physical_size = new_size;
            self.surface_configuration.width = new_size.width;
            self.surface_configuration.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_configuration);
        }
    }

    pub(crate) fn input(&mut self, window_event: &WindowEvent) -> bool {
        match window_event {
            WindowEvent::CursorMoved { device_id: _, position } => {
                let color: Color = Color {
                    r: position.x / self.physical_size.width as f64,
                    g: position.y / self.physical_size.height as f64,
                    b: 0.1,
                    a: 1.0
                };
                self.color = color;
                return true;
            },
            _ =>  { return false; }
        }
    }

    pub(crate) fn update(&mut self) {
        //..
    }

    pub(crate) fn render(&mut self) -> Result<(), SurfaceError> {
        let surface_texture: SurfaceTexture = self.surface.get_current_texture()?;
        let texture_view: TextureView = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        let mut command_encoder: CommandEncoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: Operations {
                        /*
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0
                        })*/
                        load: LoadOp::Clear(self.color),
                        store: StoreOp::Store
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
        return Ok(());
    }
}
