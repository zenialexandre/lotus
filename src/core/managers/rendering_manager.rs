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
    *,
    util::{
        BufferInitDescriptor,
        DeviceExt
    }
};
use std::{
    cell::RefMut,
    collections::HashMap,
    sync::Arc
};

use super::super::{
    color,
    shape::{
        Orientation,
        Shape
    },
    physics::transform::Transform,
    sprite::Sprite,
    texture,
    texture::TextureCache,
    ecs::{
        entitiy::Entity,
        world::World,
        component::Component
    }
};
use crate::utils::constants::shader::{BACKGROUND_SHADER, COLOR_SHADER, TEXTURE_SHADER};

/// Struct for caching Vertices and Indices.
pub struct VertexIndexBufferCache {
    pub cache: HashMap<String, (Buffer, Buffer)>
}

impl VertexIndexBufferCache {
    /// Create a new caching for Vertices and Indices.
    pub fn new() -> Self {
        return Self {
            cache: HashMap::new()
        };
    }
}

struct _TextureBindGroupCache {}

struct _ColorBindGroupCache {}

/// Struct to represent the vertices that will be sent to the shader.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture_coordinates: [f32; 2]
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
    pub texture_render_pipeline: Option<RenderPipeline>,
    pub background_render_pipeline: Option<RenderPipeline>,
    pub color_render_pipeline: Option<RenderPipeline>,
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
    pub number_of_indices: Option<u32>,
    pub texture_bind_group: Option<BindGroup>,
    pub color_bind_group: Option<BindGroup>,
    pub projection_buffer: Option<Buffer>,
    pub transform_buffer: Option<Buffer>,
    pub transform_bind_group: Option<BindGroup>,
    pub transform_bind_group_layout: Option<BindGroupLayout>,
    pub texture_bind_group_layout: Option<BindGroupLayout>,
    pub color_bind_group_layout: Option<BindGroupLayout>,
    pub entities_to_render: Vec<Entity>,
    pub texture_cache: TextureCache,
    pub vertex_index_buffer_cache: VertexIndexBufferCache
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
        let mut render_state: RenderState = Self {
            surface,
            device,
            queue,
            surface_configuration,
            physical_size,
            color: None,
            background_image_path: None,
            window,
            background_render_pipeline: None,
            texture_render_pipeline: None,
            color_render_pipeline: None,
            vertex_buffer: None,
            index_buffer: None,
            transform_buffer: None,
            projection_buffer: None,
            number_of_indices: None,
            color_bind_group: None,
            texture_bind_group: None,
            transform_bind_group: None,
            transform_bind_group_layout: None,
            texture_bind_group_layout: None,
            color_bind_group_layout: None,
            entities_to_render: Vec::new(),
            texture_cache: TextureCache::new(),
            vertex_index_buffer_cache: VertexIndexBufferCache::new()
        };

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
        let texture_bind_group_layout: BindGroupLayout = render_state.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
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
        let texture_render_pipeline: RenderPipeline = get_render_pipeline(&render_state, vec![&texture_bind_group_layout, &transform_bind_group_layout], TEXTURE_SHADER);
        let background_render_pipeline: RenderPipeline = get_render_pipeline(&render_state, vec![&texture_bind_group_layout, &transform_bind_group_layout], BACKGROUND_SHADER);
        let color_render_pipeline: RenderPipeline = get_render_pipeline(&render_state, vec![&color_bind_group_layout, &transform_bind_group_layout], COLOR_SHADER);

        render_state.texture_render_pipeline = Some(texture_render_pipeline);
        render_state.background_render_pipeline = Some(background_render_pipeline);
        render_state.color_render_pipeline = Some(color_render_pipeline);
        render_state.transform_bind_group_layout = Some(transform_bind_group_layout);
        render_state.texture_bind_group_layout = Some(texture_bind_group_layout);
        render_state.color_bind_group_layout = Some(color_bind_group_layout);
        return render_state;
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

            let projection_matrix: Matrix4<f32> = get_projection_matrix(&self);
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

            if let Some(background_image_path) = &self.background_image_path {
                let background_sprite: Sprite = Sprite::new(background_image_path.to_string());
                render_pass.set_pipeline(self.background_render_pipeline.as_ref().unwrap());
                self.setup_sprite_rendering(None, &background_sprite, None);
                self.apply_render_pass_with_values(&mut render_pass, self.texture_bind_group.as_ref().unwrap().clone());
            }

            for entity in self.entities_to_render.clone() {
                if world.is_entity_alive(entity) {
                    let components: Vec<RefMut<'_, Box<dyn Component>>> = world.get_entity_components_mut(&entity).unwrap();

                    if let Some(sprite) = components.iter().find_map(|component| component.as_any().downcast_ref::<Sprite>()) {
                        let transform: Option<&Transform> = components.iter()
                            .find_map(|component| component.as_any().downcast_ref::<Transform>()
                        );
                        render_pass.set_pipeline(self.texture_render_pipeline.as_ref().unwrap());
                        self.setup_sprite_rendering(Some(&entity), sprite, transform);
                        self.apply_render_pass_with_values(&mut render_pass, self.texture_bind_group.as_ref().unwrap().clone());
                    } else if let Some(shape) = components.iter().find_map(|component| component.as_any().downcast_ref::<Shape>()) {
                        let transform: Option<&Transform> = components.iter()
                            .find_map(|component| component.as_any().downcast_ref::<Transform>()
                        );
                        render_pass.set_pipeline(self.color_render_pipeline.as_ref().unwrap());
                        self.setup_shape_rendering(&entity, shape, transform);
                        self.apply_render_pass_with_values(&mut render_pass, self.color_bind_group.as_ref().unwrap().clone());
                    }
                }
            }
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
        return Ok(());
    }

    pub(crate) fn apply_render_pass_with_values(&mut self, render_pass: &mut RenderPass<'_>, generic_bind_group: BindGroup) {
        render_pass.set_bind_group(0, &generic_bind_group, &[]);
        render_pass.set_bind_group(1, &self.transform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_mut().unwrap().slice(..));
        render_pass.set_index_buffer(self.index_buffer.as_mut().unwrap().slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.number_of_indices.unwrap(), 0, 0..1);
    }

    pub(crate) fn setup_sprite_rendering(&mut self, entity: Option<&Entity>, sprite: &Sprite, transform: Option<&Transform>) {
        let texture: Arc<texture::Texture> = {
            if let Some(texture_from_cache) = self.texture_cache.get_texture(sprite.path.clone()) {
                texture_from_cache
            } else {
                self.texture_cache.load_texture(sprite.path.clone(), &self.device, &self.queue).unwrap()
            }
        };
        create_layouts_on_sprite_rendering(self, entity, sprite, texture.as_ref(), transform);
    }

    pub(crate) fn setup_shape_rendering(&mut self, entity: &Entity, shape: &Shape, transform: Option<&Transform>) {
        let color_buffer: Buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Color Buffer"),
            contents:bytemuck::cast_slice(&color::Color::to_array(color::Color::to_wgpu(shape.color))),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        let color_bind_group: BindGroup = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Color Bind Group"),
            layout: &self.color_bind_group_layout.as_ref().unwrap(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: color_buffer.as_entire_binding()
                }
            ]
        });

        let (transform_bind_group, projection_buffer) = get_transform_bindings(self, transform);
        let (vertex_buffer, index_buffer) = get_vertex_and_index_buffers(
            self,
            Some(entity),
            &shape.geometry_type.to_vertex_array(Orientation::Horizontal),
            &shape.geometry_type.to_index_array()
        );

        self.color_bind_group = Some(color_bind_group);
        self.transform_bind_group = Some(transform_bind_group);
        self.projection_buffer = Some(projection_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.number_of_indices = Some(shape.geometry_type.to_index_array().len() as u32);
    }
}

fn create_layouts_on_sprite_rendering(
    render_state: &mut RenderState,
    entity: Option<&Entity>,
    sprite: &Sprite,
    texture: &texture::Texture,
    transform: Option<&Transform>
) {
    let texture_bind_group: BindGroup = render_state.device.create_bind_group(&BindGroupDescriptor {
        label: Some("Texture Bind Group"),
        layout: &render_state.texture_bind_group_layout.as_ref().unwrap(),
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&texture.texture_view)
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(&texture.sampler)
            }
        ]
    });

    let (transform_bind_group, projection_buffer) = get_transform_bindings(render_state, transform);
    let (vertex_buffer, index_buffer) = get_vertex_and_index_buffers(
        render_state,
        entity,
        &sprite.vertices,
        &sprite.indices
    );

    render_state.texture_bind_group = Some(texture_bind_group);
    render_state.transform_bind_group = Some(transform_bind_group);
    render_state.projection_buffer = Some(projection_buffer);
    render_state.vertex_buffer = Some(vertex_buffer);
    render_state.index_buffer = Some(index_buffer);
    render_state.number_of_indices = Some(sprite.indices.len() as u32);
}

fn get_transform_bindings(render_state: &mut RenderState, transform: Option<&Transform>) -> (BindGroup, Buffer) {
    let projection_buffer: Buffer = get_projection_buffer(render_state);

    if let Some(transform_unwrapped) = transform {
        let transform_matrix_unwrapped: [[f32; 4]; 4] = *transform_unwrapped.to_matrix().as_ref();
        let transform_buffer: Buffer = render_state.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_matrix_unwrapped]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        render_state.transform_buffer = Some(transform_buffer);
    } else {
        let identity_matrix: Matrix4<f32> = Matrix4::identity();
        let identity_matrix_unwrapped: [[f32; 4]; 4] = *identity_matrix.as_ref();
        let transform_buffer: Buffer = render_state.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[identity_matrix_unwrapped]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        render_state.transform_buffer = Some(transform_buffer);
    }

    let transform_bind_group: BindGroup = render_state.device.create_bind_group(&BindGroupDescriptor {
        label: Some("Transform Bind Group"),
        layout: &render_state.transform_bind_group_layout.as_ref().unwrap(),
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: render_state.transform_buffer.as_ref().unwrap().as_entire_binding()
            },
            BindGroupEntry {
                binding: 1,
                resource: projection_buffer.as_entire_binding()
            }
        ]
    });
    return (transform_bind_group, projection_buffer);
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
    return Some(BlendState::REPLACE);
}

fn get_vertex_and_index_buffers(
    render_state: &mut RenderState,
    entity: Option<&Entity>,
    vertex_array: &[Vertex],
    index_array: &[u16]
) -> (Buffer, Buffer) {
    if let Some(entity) = entity {
        if let Some((vertex_buffer_cache, index_buffer_cache)) = render_state.vertex_index_buffer_cache.cache.get(&entity.0.to_string()) {
            return (vertex_buffer_cache.clone(), index_buffer_cache.clone());
        } else {
            let vertex_buffer: Buffer = get_vertex_buffer(render_state, vertex_array).clone();
            let index_buffer: Buffer = get_index_buffer(render_state, index_array).clone();

            render_state.vertex_index_buffer_cache.cache.insert(entity.0.to_string(), (vertex_buffer.clone(), index_buffer.clone()));
            return (vertex_buffer.clone(), index_buffer.clone());
        }
    } else {
        let fixed_uuid: String = "fixed_uuid".to_string();

        if let Some((vertex_buffer_cache, index_buffer_cache)) = render_state.vertex_index_buffer_cache.cache.get(&fixed_uuid) {
            return (vertex_buffer_cache.clone(), index_buffer_cache.clone());
        } else {
            let vertex_buffer: Buffer = get_vertex_buffer(render_state, vertex_array).clone();
            let index_buffer: Buffer = get_index_buffer(render_state, index_array).clone();

            render_state.vertex_index_buffer_cache.cache.insert(fixed_uuid, (vertex_buffer.clone(), index_buffer.clone()));
            return (vertex_buffer, index_buffer);
        }
    }
}

fn get_vertex_buffer(render_state: &RenderState, vertex_array: &[Vertex]) -> Buffer {
    return render_state.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertex_array),
        usage: BufferUsages::VERTEX
    });
}

fn get_index_buffer(render_state: &RenderState, index_array: &[u16]) -> Buffer {
    return render_state.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(index_array),
        usage: BufferUsages::INDEX
    });
}
