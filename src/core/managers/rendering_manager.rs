use cgmath::{
    ortho,
    Matrix4,
    SquareMatrix
};
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    window::Window
};
use wgpu::{
    vertex_attr_array,
    Adapter,
    Backends,
    Color,
    ColorTargetState,
    ColorWrites,
    BlendState,
    BlendComponent,
    BlendFactor,
    BlendOperation,
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
    ShaderModuleDescriptor,
    ShaderSource,
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
    BufferBindingType,
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
use std::{path::Path, sync::Arc};

use super::super::{color, shape::{Orientation, Shape}, sprite::Sprite, texture};
use crate::utils::constants::shader::{TEXTURE_SHADER, COLOR_SHADER};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture_coordinates: [f32; 2]
}

pub struct RenderState {
    surface: Surface<'static>,
    device: Device,
    pub queue: Queue,
    surface_configuration: SurfaceConfiguration,
    pub(crate) physical_size: PhysicalSize<u32>,
    pub(crate) color: Option<color::Color>,
    window: Arc<Window>,
    render_pipeline: Option<RenderPipeline>,
    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
    number_of_indices: Option<u32>,
    diffuse_bind_group: Option<BindGroup>,
    color_bind_group: Option<BindGroup>,
    projection_buffer: Option<Buffer>,
    pub transform_buffer: Option<Buffer>,
    transform_bind_group: Option<BindGroup>
}

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

impl RenderState {
    pub(crate) async fn new(window: Arc<Window>) -> RenderState {
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

        return Self {
            surface,
            device,
            queue,
            surface_configuration,
            physical_size,
            color: None,
            window,
            render_pipeline: None,
            vertex_buffer: None,
            index_buffer: None,
            transform_buffer: None,
            projection_buffer: None,
            number_of_indices: None,
            color_bind_group: None,
            diffuse_bind_group: None,
            transform_bind_group: None
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

            let projection_matrix: Matrix4<f32> = get_projection_matrix(&self);
            let projection_matrix_unwrapped: [[f32; 4]; 4] = *projection_matrix.as_ref();

            self.queue.write_buffer(
                &self.projection_buffer.as_ref().unwrap(),
                0,
                bytemuck::cast_slice(&[projection_matrix_unwrapped])
            );
        }
    }

    pub(crate) fn input(&mut self, window_event: &WindowEvent) -> bool {
        match window_event {
            WindowEvent::CursorMoved { device_id: _, position: _ } => {
                /*let color: Color = Color {
                    r: position.x / self.physical_size.width as f64,
                    g: position.y / self.physical_size.height as f64,
                    b: 0.1,
                    a: 1.0
                };
                self.color = color;*/
                return true;
            },
            _ =>  { return false; }
        }
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
                        load: LoadOp::Clear(color::to_wgpu(self.color.unwrap())),
                        store: StoreOp::Store
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None
            });
            render_pass.set_pipeline(&self.render_pipeline.as_mut().unwrap());
            render_pass.set_viewport(
                0.0,
                0.0, 
                self.physical_size.width as f32, 
                self.physical_size.height as f32, 
                0.0,
                1.0
            );

            if let Some(_diffuse_bind_group) = &self.diffuse_bind_group {
                render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            } else {
                render_pass.set_bind_group(0, &self.color_bind_group, &[]);
            }
            render_pass.set_bind_group(1, &self.transform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.as_mut().unwrap().slice(..));
            render_pass.set_index_buffer(self.index_buffer.as_mut().unwrap().slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.number_of_indices.unwrap(), 0, 0..1);
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
        return Ok(());
    }
}

pub fn render_sprite(render_state: &mut RenderState, sprite: Sprite) {
    if let Ok(diffuse_dynamic_image) = image::open(Path::new(sprite.path.as_str())) {
        let diffuse_texture: texture::Texture = texture::Texture::from_image(
            &render_state.device,
            &render_state.queue,
            &diffuse_dynamic_image,
            Some("Sprite")
        ).unwrap();

        let diffuse_bind_group_layout: BindGroupLayout = render_state.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Diffuse Bind Group Layout"),
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
        let diffuse_bind_group: BindGroup = render_state.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Diffuse Bind Group"),
            layout: &diffuse_bind_group_layout,
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

        let (
            transform_bind_group,
            transform_bind_group_layout,
            transform_buffer,
            projection_buffer
        ) = get_transform_bindings(render_state);

        let render_pipeline: RenderPipeline = get_render_pipeline(
            render_state,
            vec![&diffuse_bind_group_layout, &transform_bind_group_layout],
            TEXTURE_SHADER
        );
        let vertex_buffer: Buffer = get_vertex_buffer(render_state, &sprite.vertices);
        let (index_buffer, number_of_indices) = get_index_attributes(render_state, &sprite.indices);

        render_state.render_pipeline = Some(render_pipeline);
        render_state.diffuse_bind_group = Some(diffuse_bind_group);
        render_state.transform_bind_group = Some(transform_bind_group);
        render_state.transform_buffer = Some(transform_buffer);
        render_state.projection_buffer = Some(projection_buffer);
        render_state.vertex_buffer = Some(vertex_buffer);
        render_state.index_buffer = Some(index_buffer);
        render_state.number_of_indices = Some(number_of_indices);
    } else {
        panic!("Image not found on the render_sprite process!");
    }
}

pub fn render_shape(render_state: &mut RenderState, shape: Shape) {
    let color_buffer: Buffer = render_state.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Color Buffer"),
        contents:bytemuck::cast_slice(&color::to_array(Color::BLACK)),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    });
    let color_bind_group_layout: BindGroupLayout = render_state.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Color Bind Group Layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT | ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        ]
    });
    let color_bind_group: BindGroup = render_state.device.create_bind_group(&BindGroupDescriptor {
        label: Some("Color Bind Group"),
        layout: &color_bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: color_buffer.as_entire_binding()
            }
        ]
    });

    let (
        transform_bind_group,
        transform_bind_group_layout,
        transform_buffer,
        projection_buffer
    ) = get_transform_bindings(render_state);

    let render_pipeline: RenderPipeline = get_render_pipeline(
        render_state,
        vec![&color_bind_group_layout, &transform_bind_group_layout],
        COLOR_SHADER
    );
    let vertex_buffer: Buffer = get_vertex_buffer(render_state, &shape.geometry_type.to_vertex_array(Orientation::Horizontal));
    let (index_buffer, number_of_indices) = get_index_attributes(render_state, &shape.geometry_type.to_index_array());

    render_state.render_pipeline = Some(render_pipeline);
    render_state.color_bind_group = Some(color_bind_group);
    render_state.transform_bind_group = Some(transform_bind_group);
    render_state.transform_buffer = Some(transform_buffer);
    render_state.projection_buffer = Some(projection_buffer);
    render_state.vertex_buffer = Some(vertex_buffer);
    render_state.index_buffer = Some(index_buffer);
    render_state.number_of_indices = Some(number_of_indices);
}

fn get_transform_bindings(render_state: &RenderState) -> (BindGroup, BindGroupLayout, Buffer, Buffer) {
    let identity_matrix: Matrix4<f32> = Matrix4::identity();
    let identity_matrix_unwrapped: [[f32; 4]; 4] = *identity_matrix.as_ref();
    let transform_buffer: Buffer = render_state.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Transform Buffer"),
        contents: bytemuck::cast_slice(&[identity_matrix_unwrapped]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    });

    let projection_buffer: Buffer = get_projection_buffer(render_state);
    let transform_bind_group_layout: BindGroupLayout = render_state.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Transform Bind Group Layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        ]
    });
    let transform_bind_group: BindGroup = render_state.device.create_bind_group(&BindGroupDescriptor {
        label: Some("Transform Bind Group"),
        layout: &transform_bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding()
            },
            BindGroupEntry {
                binding: 1,
                resource: projection_buffer.as_entire_binding()
            }
        ]
    });
    return (transform_bind_group, transform_bind_group_layout, transform_buffer, projection_buffer);
}

pub(crate) fn get_projection_buffer(render_state: &RenderState) -> Buffer {
    let projection_matrix: Matrix4<f32> = get_projection_matrix(render_state);
    let projection_matrix_unwrapped: [[f32; 4]; 4] = *projection_matrix.as_ref();
    return render_state.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Projection Buffer"),
        contents: bytemuck::cast_slice(&[projection_matrix_unwrapped]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    });
}

fn get_projection_matrix(render_state: &RenderState) -> Matrix4<f32> {
    let aspect_ratio: f32 = render_state.physical_size.width as f32 / render_state.physical_size.height as f32;
    return ortho(
        -aspect_ratio,
        aspect_ratio as f32,
        -1.0,
        1.0,
        -1.0,
        1.0
    );
}

fn get_render_pipeline(render_state: &RenderState, bind_group_layouts: Vec<&BindGroupLayout>, shader_source: &str) -> RenderPipeline {
    let shader_module: ShaderModule = render_state.device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Shader Module"),
        source: ShaderSource::Wgsl(shader_source.into())
    });
    let render_pipeline_layout: PipelineLayout = render_state.device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &bind_group_layouts[..],
        push_constant_ranges: &[]
    });
    let render_pipeline: RenderPipeline = render_state.device.create_render_pipeline(&RenderPipelineDescriptor {
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
                format: render_state.surface_configuration.format,
                blend: get_blend_state(shader_source),
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
    return render_pipeline;
}

fn get_blend_state(shader_source: &str) -> Option<BlendState> {
    if shader_source.contains("texture") {
        return Some(BlendState {
            color: BlendComponent {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add
            },
            alpha: BlendComponent {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add
            }
        });
    }
    return None;
}

fn get_vertex_buffer(render_state: &RenderState, vertex_array: &[Vertex]) -> Buffer {
    return render_state.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertex_array),
        usage: BufferUsages::VERTEX
    });
}

fn get_index_attributes(render_state: &RenderState, index_array: &[u16]) -> (Buffer, u32) {
    let number_of_indices: u32 = index_array.len() as u32;
    let index_buffer: Buffer = render_state.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(index_array),
        usage: BufferUsages::INDEX
    });
    return (index_buffer, number_of_indices);
}
