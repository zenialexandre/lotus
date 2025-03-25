use cgmath::{
    ortho,
    Matrix4
};
use strum::IntoEnumIterator;
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    window::Window
};
use wgpu::{
    *,
    util::{
        BufferInitDescriptor,
        DeviceExt
    }
};
use std::{
    cell::Ref,
    collections::HashMap,
    sync::Arc
};

use super::super::{
    color,
    shape::{
        Orientation,
        GeometryType,
        Shape
    },
    physics::transform::Transform,
    sprite::Sprite,
    texture,
    ecs::{
        entitiy::Entity,
        world::World,
        component::Component
    }
};
use crate::utils::constants::shader::{SHARED_SHADER, BACKGROUND_SHADER};

/// Struct to represent the vertices that will be sent to the shader.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture_coordinates: [f32; 2]
}

/// Struct to apply the instancing pattern.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 4],
    pub texture_index: u32
}

impl InstanceData {
    pub fn from_transform(transform: Matrix4<f32>, color: [f32; 4], texture_index: u32) -> Self {
        let matrix: [[f32; 4]; 4] = transform.into();
        return Self {
            transform: matrix,
            color,
            texture_index
        }
    }
}

/// Enumerator to represent the type of a rendering batch.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderBatchType {
    Triangle,
    Square,
    Rectangle,
    Circle(u16),
    Textured(String),
    Background
}

/// Struct to represent a batch to be rendered.
pub struct RenderBatch {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    number_of_indices: u32,
    instances: Vec<InstanceData>
}

/// Struct to represent the current rendering state of the engine.
pub struct RenderState {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub surface_configuration: SurfaceConfiguration,
    pub physical_size: PhysicalSize<u32>,
    pub color: Option<color::Color>,
    pub background_image_path: Option<String>,
    pub window: Arc<Window>,
    pub batches: HashMap<RenderBatchType, RenderBatch>,
    pub instance_buffer: Buffer,
    pub projection_buffer: Option<Buffer>,
    pub texture_cache: HashMap<String, Arc<texture::Texture>>,
    pub pipeline_layout: Option<PipelineLayout>,
    pub texture_bind_group: Option<BindGroup>,
    pub next_texture_index: u32,
    pub entities_to_render: Vec<Entity>
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
    /// Create a new asynchronous rendering state for the window.
    pub async fn new(window: Arc<Window>) -> Self {
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
            present_mode: PresentMode::Fifo,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &surface_configuration);

        let instance_buffer: Buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Instance Buffer"),
            size: 1024 * 1024, // -> 1MB
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let texture_bind_group_layout: BindGroupLayout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
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
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::D2Array,
                        sample_type: TextureSampleType::Float { filterable: true }
                    },
                    count: None
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None
                }
            ]
        });

        let pipeline_layout: PipelineLayout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[]
        });

        let texture_array: wgpu::Texture = device.create_texture(&TextureDescriptor {
            label: Some("Texture Array"),
            size: Extent3d { width: 2048, height: 2048, depth_or_array_layers: 16 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[]
        });

        let texture_array_view: TextureView = texture_array.create_view(&TextureViewDescriptor {
            dimension: Some(TextureViewDimension::D2Array),
            ..Default::default()
        });

        let sampler: Sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("Texture Sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            ..Default::default()
        });

        let projection_matrix: Matrix4<f32> = get_projection_matrix(physical_size);
        let projection_matrix_unwrapped: [[f32; 4]; 4] = *projection_matrix.as_ref();
        let projection_buffer: Buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Projection Buffer"),
            contents: bytemuck::cast_slice(&[projection_matrix_unwrapped]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let texture_bind_group: BindGroup = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: projection_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&texture_array_view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        let mut render_state: RenderState = Self {
            surface,
            device,
            queue,
            surface_configuration,
            physical_size,
            color: None,
            background_image_path: None,
            window,
            batches: HashMap::new(),
            instance_buffer,
            projection_buffer: Some(projection_buffer),
            texture_cache: HashMap::new(),
            pipeline_layout: Some(pipeline_layout),
            texture_bind_group: Some(texture_bind_group),
            next_texture_index: 0,
            entities_to_render: Vec::new()
        };
        render_state.initialize_batches().await;
        return render_state;
    }

    async fn initialize_batches(&mut self) {
        let shared_pipeline: RenderPipeline = self.create_pipeline(SHARED_SHADER, true);
        let background_pipeline: RenderPipeline = self.create_pipeline(BACKGROUND_SHADER, false);
    
        for geometry_type in GeometryType::iter() {
            let vertices: Vec<Vertex> = geometry_type.to_vertex_array(Orientation::Horizontal);
            let indices: Vec<u16> = geometry_type.to_index_array();

            self.batches.insert(
                GeometryType::to_batch_type(&geometry_type.clone()),
                RenderBatch {
                    render_pipeline: shared_pipeline.clone(),
                    vertex_buffer: self.device.create_buffer_init(&BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: BufferUsages::VERTEX
                    }),
                    index_buffer: self.device.create_buffer_init(&BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(&indices),
                        usage: BufferUsages::INDEX
                    }),
                    number_of_indices: indices.len() as u32,
                    instances: Vec::new()
                }
            );
        }

        let background_vertices: Vec<Vertex> = GeometryType::Square.to_vertex_array(Orientation::Horizontal);
        let background_indices: Vec<u16> = GeometryType::Square.to_index_array();

        self.batches.insert(
            RenderBatchType::Background,
            RenderBatch {
                render_pipeline: background_pipeline,
                vertex_buffer: self.device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Background Vertex Buffer"),
                    contents: bytemuck::cast_slice(&background_vertices),
                    usage: BufferUsages::VERTEX
                }),
                index_buffer: self.device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Background Index Buffer"),
                    contents: bytemuck::cast_slice(&background_indices),
                    usage: BufferUsages::INDEX
                }),
                number_of_indices: background_indices.len() as u32,
                instances: Vec::new(),
            },
        );
    }

    fn create_pipeline(&self, shader_source: &str, instanced: bool) -> RenderPipeline {
        let shader_module: ShaderModule = self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: ShaderSource::Wgsl(shader_source.into())
        });
        let mut vertex_attributes: Vec<VertexAttribute> = vec![
            VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                shader_location: 1,
            },
        ];

        if instanced {
            vertex_attributes.extend([
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 3,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 2 * std::mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 4,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 3 * std::mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 5,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 6,
                },
                VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    offset: 0,
                    shader_location: 10,
                },
            ]);
        }

        let mut vertex_buffer_layouts: Vec<VertexBufferLayout<'_>> = vec![
            VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
                step_mode: VertexStepMode::Vertex,
                attributes: &vertex_attributes[0..2]
            }
        ];

        if instanced {
            vertex_buffer_layouts.push(
                VertexBufferLayout {
                    array_stride: std::mem::size_of::<InstanceData> as BufferAddress,
                    step_mode: VertexStepMode::Instance,
                    attributes: &vertex_attributes[2..]
                }
            );
        }

        return self.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&self.pipeline_layout.as_ref().unwrap()),
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
                    format: self.surface_configuration.format,
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
    }

    /// Returns the window reference.
    pub fn window(&self) -> &Window {
        return &self.window;
    }

    /// Resize the rendering projection.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.physical_size = new_size;
            self.surface_configuration.width = new_size.width;
            self.surface_configuration.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_configuration);

            let projection_matrix: Matrix4<f32> = get_projection_matrix(self.physical_size);
            let projection_matrix_unwrapped: [[f32; 4]; 4] = *projection_matrix.as_ref();

            if let Some(projection_buffer) = self.projection_buffer.as_ref() {
                self.queue.write_buffer(
                    projection_buffer,
                    0,
                    bytemuck::cast_slice(&[projection_matrix_unwrapped])
                );
            }
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

    /// Add an entity to be rendered.
    pub fn add_entity_to_render(&mut self, entity: Entity) {
        self.entities_to_render.push(entity);
    }

    /// Remove an entity from the rendering list.
    pub fn remove_entity_to_render(&mut self, entity: &Entity) {
        if let Some(index) = self.entities_to_render.iter().position(|e| e == entity) {
            self.entities_to_render.remove(index);
        }
    }

    /// Execute the rendering process.
    pub fn render(&mut self, world: &World) -> Result<(), SurfaceError> {
        // Limpa as instâncias de todos os batches
        for batch in self.batches.values_mut() {
            batch.instances.clear();
        }

        // Processa todas as entidades para renderização
        for entity in &self.entities_to_render {
            if !world.is_entity_alive(*entity) {
                continue;
            }

            let components = world.get_entity_components(entity).unwrap();
            let default_transform = Transform::default();
            let transform = components.iter()
                .find_map(|c| c.as_any().downcast_ref::<Transform>())
                .unwrap_or(&default_transform);

            // Processa formas geométricas
            if let Some(shape) = components.iter()
                .find_map(|c| c.as_any().downcast_ref::<Shape>()) 
            {
                let batch_type = GeometryType::to_batch_type(&shape.geometry_type);
                if let Some(batch) = self.batches.get_mut(&batch_type) {
                    batch.instances.push(InstanceData {
                        transform: *transform.to_matrix().as_ref(),
                        color: shape.color.to_rgba(),
                        texture_index: 0xFFFFFFFF, // Valor especial para indicar sem textura
                    });
                }
            } 
            // Processa sprites com textura
            else if let Some(sprite) = components.iter()
                .find_map(|c| c.as_any().downcast_ref::<Sprite>()) 
            {
                // 1. Primeiro carregue a textura (se necessário)
                self.texture_cache.entry(sprite.path.clone())
                    .or_insert_with(|| {
                        Arc::new(texture::Texture::from_image(
                            &self.device,
                            &self.queue,
                            &image::open(&sprite.path).unwrap(),
                            Some(&sprite.path),
                        ).unwrap())
                    });

                // 2. Determine o batch_type
                let batch_type = RenderBatchType::Textured(sprite.path.clone());
                
                // 3. Verifique se o batch já existe
                if !self.batches.contains_key(&batch_type) {
                    // 4. Crie os buffers ANTES de inserir no HashMap
                    let vertex_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
                        label: Some("Sprite Vertex Buffer"),
                        contents: bytemuck::cast_slice(&sprite.vertices),
                        usage: BufferUsages::VERTEX,
                    });
                    
                    let index_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
                        label: Some("Sprite Index Buffer"),
                        contents: bytemuck::cast_slice(&sprite.indices),
                        usage: BufferUsages::INDEX,
                    });

                    // 5. Use o pipeline compartilhado que já existe
                    let shared_pipeline = self.batches.get(&RenderBatchType::Square)
                        .unwrap()
                        .render_pipeline
                        .clone();

                    // 6. Agora sim, insira no HashMap
                    self.batches.insert(batch_type.clone(), RenderBatch {
                        render_pipeline: shared_pipeline,
                        vertex_buffer,
                        index_buffer,
                        number_of_indices: sprite.indices.len() as u32,
                        instances: Vec::new(),
                    });
                }

                // 7. Agora podemos pegar a referência mutável sem problemas
                let batch = self.batches.get_mut(&batch_type).unwrap();
                batch.instances.push(InstanceData {
                    transform: *transform.to_matrix().as_ref(),
                    color: [1.0, 1.0, 1.0, 1.0],
                    texture_index: self.next_texture_index,
                });
                
                self.next_texture_index += 1;
            }
        }

        let surface_texture: SurfaceTexture = self.surface.get_current_texture()?;
        let texture_view: TextureView = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        let mut command_encoder: CommandEncoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let mut render_pass: RenderPass<'_> = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(color::Color::to_wgpu(self.color.unwrap_or_else(|| color::Color::WHITE))),
                        store: StoreOp::Store
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None
            });
            render_pass.set_viewport(
                0.0,
                0.0, 
                self.physical_size.width as f32, 
                self.physical_size.height as f32, 
                0.0,
                1.0
            );
            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);

            if let Some(background_batch) = self.batches.get(&RenderBatchType::Background) {
                if !background_batch.instances.is_empty() {
                    self.render_batch(&mut render_pass, background_batch);
                }
            }

            for (key, batch) in &self.batches {
                if matches!(key, RenderBatchType::Background) || batch.instances.is_empty() {
                    continue;
                }
                self.render_batch(&mut render_pass, batch);
            }
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
        return Ok(());
    }

    fn render_batch<'a>(&'a self, render_pass: &mut RenderPass<'a>, batch: &'a RenderBatch) {
        self.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&batch.instances),
        );

        render_pass.set_pipeline(&batch.render_pipeline);
        render_pass.set_vertex_buffer(0, batch.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(batch.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..batch.number_of_indices, 0, 0..batch.instances.len() as u32);
    }
}

fn get_projection_buffer(render_state: &RenderState) -> Buffer {
    let projection_matrix: Matrix4<f32> = get_projection_matrix(render_state.physical_size);
    let projection_matrix_unwrapped: [[f32; 4]; 4] = *projection_matrix.as_ref();
    return render_state.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Projection Buffer"),
        contents: bytemuck::cast_slice(&[projection_matrix_unwrapped]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    });
}

fn get_projection_matrix(physical_size: PhysicalSize<u32>) -> Matrix4<f32> {
    let aspect_ratio: f32 = physical_size.width as f32 / physical_size.height as f32;
    return ortho(
        -aspect_ratio,
        aspect_ratio as f32,
        -1.0,
        1.0,
        -1.0,
        1.0
    );
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
    return Some(BlendState::REPLACE);
}
