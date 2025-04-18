use atomic_refcell::AtomicRefMut;
use cgmath::{ortho, Matrix4};
use uuid::Uuid;
use wgpu_text::glyph_brush::Section;
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};
use wgpu::{*, util::{BufferInitDescriptor, DeviceExt}};
use std::{collections::HashMap, sync::Arc};

use super::super::{
    color,
    shape::{Orientation, Shape},
    physics::transform::Transform,
    texture,
    texture::{texture::TextureCache, sprite::Sprite},
    text::{TextRenderer, Text},
    camera::camera2d::Camera2d,
    ecs::{entity::Entity, world::World, component::Component, resource::ResourceRef}
};
use crate::{core::shape, utils::constants::shader::{COLOR_SHADER, TEXTURE_SHADER}};

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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    transform: [[f32; 4]; 4],
    color: [f32; 4]
}

impl InstanceRaw {
    fn descriptor() -> VertexBufferLayout<'static> {
        return VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 3,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: 4,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 5,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 16]>() as BufferAddress,
                    shader_location: 6,
                    format: VertexFormat::Float32x4,
                }
            ]
        };
    }
}

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
    pub surface: Option<Surface<'static>>,
    pub device: Option<Device>,
    pub queue: Option<Queue>,
    pub surface_configuration: Option<SurfaceConfiguration>,
    pub physical_size: Option<PhysicalSize<u32>>,
    pub color: Option<color::Color>,
    pub background_image_path: Option<String>,
    pub window: Option<Arc<Window>>,
    pub texture_render_pipeline: Option<RenderPipeline>,
    pub color_render_pipeline: Option<RenderPipeline>,
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
    pub number_of_indices: Option<u32>,
    pub texture_bind_groups: HashMap<String, BindGroup>,
    pub color_bind_group: Option<BindGroup>,
    pub projection_buffer: Option<Buffer>,
    pub view_buffer: Option<Buffer>,
    pub transform_buffer: Option<Buffer>,
    pub window_bind_group: Option<BindGroup>,
    pub window_bind_group_layout: Option<BindGroupLayout>,
    pub texture_bind_group_layout: Option<BindGroupLayout>,
    pub entities_to_render: Vec<Entity>,
    pub shape_instances: Vec<InstanceRaw>,
    pub sprite_instances: HashMap<String, Vec<InstanceRaw>>,
    pub text_renderers: HashMap<Uuid, TextRenderer>,
    pub texture_cache: TextureCache,
    pub vertex_index_buffer_cache: VertexIndexBufferCache
}

impl RenderState {
    /// Create a dummy rendering state.
    /// Mainly for testing purposes.
    pub fn dummy() -> Self {
        return Self {
            surface: None,
            device: None,
            queue: None,
            surface_configuration: None,
            physical_size: None,
            color: None,
            background_image_path: None,
            window: None,
            texture_render_pipeline: None,
            color_render_pipeline: None,
            vertex_buffer: None,
            index_buffer: None,
            transform_buffer: None,
            projection_buffer: None,
            view_buffer: None,
            number_of_indices: None,
            color_bind_group: None,
            texture_bind_groups: HashMap::new(),
            window_bind_group: None,
            window_bind_group_layout: None,
            texture_bind_group_layout: None,
            entities_to_render: Vec::new(),
            shape_instances: Vec::new(),
            sprite_instances: HashMap::new(),
            text_renderers: HashMap::new(),
            texture_cache: TextureCache::new(),
            vertex_index_buffer_cache: VertexIndexBufferCache::new()
        };
    }

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
                memory_hints: Default::default(),
                trace: Trace::Off
            }
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
            surface: Some(surface),
            device: Some(device),
            queue: Some(queue),
            surface_configuration: Some(surface_configuration),
            physical_size: Some(physical_size),
            color: None,
            background_image_path: None,
            window: Some(window),
            texture_render_pipeline: None,
            color_render_pipeline: None,
            vertex_buffer: None,
            index_buffer: None,
            transform_buffer: None,
            projection_buffer: None,
            view_buffer: None,
            number_of_indices: None,
            color_bind_group: None,
            texture_bind_groups: HashMap::new(),
            window_bind_group: None,
            window_bind_group_layout: None,
            texture_bind_group_layout: None,
            entities_to_render: Vec::new(),
            shape_instances: Vec::new(),
            sprite_instances: HashMap::new(),
            text_renderers: HashMap::new(),
            texture_cache: TextureCache::new(),
            vertex_index_buffer_cache: VertexIndexBufferCache::new()
        };

        let window_bind_group_layout: BindGroupLayout = render_state.device.as_ref().unwrap().create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Window Bind Group Layout"),
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
        let texture_bind_group_layout: BindGroupLayout = render_state.device.as_ref().unwrap().create_bind_group_layout(&BindGroupLayoutDescriptor {
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
                BindGroupLayoutEntry{
                    binding: 2,
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

        let texture_render_pipeline: RenderPipeline = get_render_pipeline(
            &render_state,
            vec![&texture_bind_group_layout, &window_bind_group_layout],
            TEXTURE_SHADER
        );
        let color_render_pipeline: RenderPipeline = get_render_pipeline(
            &render_state,
            vec![&window_bind_group_layout],
            COLOR_SHADER
        );

        render_state.texture_render_pipeline = Some(texture_render_pipeline);
        render_state.color_render_pipeline = Some(color_render_pipeline);
        render_state.window_bind_group_layout = Some(window_bind_group_layout);
        render_state.texture_bind_group_layout = Some(texture_bind_group_layout);
        return render_state;
    }

    /// Returns the window reference.
    pub fn window(&self) -> &Window {
        return &self.window.as_ref().unwrap();
    }

    /// Resize the rendering projection.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>, camera2d: &Camera2d) {
        if new_size.width > 0 && new_size.height > 0 {
            self.physical_size = Some(new_size);
            self.surface_configuration.as_mut().unwrap().width = new_size.width;
            self.surface_configuration.as_mut().unwrap().height = new_size.height;
            self.surface.as_ref().unwrap().configure(&self.device.as_ref().unwrap(), &self.surface_configuration.as_ref().unwrap());

            let projection_matrix: Matrix4<f32> = self.get_projection_matrix(camera2d);
            let projection_matrix_unwrapped: [[f32; 4]; 4] = *projection_matrix.as_ref();

            if let Some(projection_buffer) = self.projection_buffer.as_ref() {
                self.queue.as_mut().unwrap().write_buffer(
                    projection_buffer,
                    0,
                    bytemuck::cast_slice(&[projection_matrix_unwrapped])
                );
            }

            if !self.text_renderers.is_empty() {
                for text_renderer in &self.text_renderers {
                    text_renderer.1.text_brush.update_matrix(
                        wgpu_text::ortho(new_size.width as f32, new_size.height as f32),
                        self.queue.as_ref().unwrap()
                    );
                }
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
        let mut entities_of_text: Vec<Entity> = Vec::new();

        for entity in self.entities_to_render.clone() {
            if world.is_entity_alive(entity) {
                if let Some(text_renderer) = self.text_renderers.get_mut(&entity.0) {
                    let (x, y): (f32, f32) = text_renderer.text.get_position_as_pixels(&self.physical_size.as_ref().unwrap());
                    entities_of_text.push(entity);

                    text_renderer.text_brush.queue(
                        self.device.as_ref().unwrap(),
                        self.queue.as_ref().unwrap(),
                        vec![Section {
                            screen_position: (x, y),
                            bounds: (self.physical_size.as_ref().unwrap().width as f32, self.physical_size.as_ref().unwrap().height as f32),
                            text: vec![
                                wgpu_text::glyph_brush::Text::new(&text_renderer.text.content)
                                    .with_color(text_renderer.text.color.to_rgba())
                                    .with_scale(text_renderer.text.font.size)
                            ],
                            ..Default::default()
                        }]
                    ).ok();
                }
            }
        }

        let surface_texture: SurfaceTexture = self.surface.as_ref().unwrap().get_current_texture()?;
        let texture_view: TextureView = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        let mut command_encoder: CommandEncoder = self.device.as_ref().unwrap().create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let camera2d: ResourceRef<'_, Camera2d> = world.get_resource::<Camera2d>().unwrap();
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
                self.physical_size.as_ref().unwrap().width as f32,
                self.physical_size.as_ref().unwrap().height as f32,
                0.0,
                1.0
            );

            if let Some(background_image_path) = &self.background_image_path {
                let background_sprite: Sprite = Sprite::new(background_image_path.to_string());
                render_pass.set_pipeline(self.texture_render_pipeline.as_ref().unwrap());
                self.setup_sprite_rendering(None, &background_sprite, None, &camera2d, true);
            }

            for entity in self.entities_to_render.clone() {
                if world.is_entity_alive(entity) && world.is_entity_visible(entity) {
                    let components: Vec<AtomicRefMut<'_, Box<dyn Component>>> = world.get_entity_components_mut(&entity).unwrap();

                    if let Some(sprite) = components.iter().find_map(|component| component.as_any().downcast_ref::<Sprite>()) {
                        let transform: Option<&Transform> = components.iter()
                            .find_map(|component| component.as_any().downcast_ref::<Transform>()
                        );
                        render_pass.set_pipeline(self.texture_render_pipeline.as_ref().unwrap());
                        self.setup_sprite_rendering(Some(&entity), sprite, transform, &camera2d, false);
                    } else if let Some(shape) = components.iter().find_map(|component| component.as_any().downcast_ref::<Shape>()) {
                        let transform: Option<&Transform> = components.iter()
                            .find_map(|component| component.as_any().downcast_ref::<Transform>()
                        );
                        render_pass.set_pipeline(self.color_render_pipeline.as_ref().unwrap());
                        self.setup_shape_rendering(&entity, shape, transform, &camera2d);
                    }
                }
            }

            let x = self.sprite_instances.clone();
            let z = self.shape_instances.clone();
            let y = self.texture_bind_groups.clone();

            for (label, instances) in x {
                if let Some(texture_bind_group) = y.get(&label) {
                    self.apply_render_pass_with_values(
                        &mut render_pass,
                        &instances,
                        Some(texture_bind_group)
                    );
                }
            }
            self.apply_render_pass_with_values(&mut render_pass, &z, None);

            for entity in entities_of_text {
                let components: Vec<AtomicRefMut<'_, Box<dyn Component>>> = world.get_entity_components_mut(&entity).unwrap();
                if components.iter().any(|component| component.as_any().is::<Text>()) {
                    if let Some(text_renderer) = self.text_renderers.get(&entity.0) {
                        text_renderer.text_brush.draw(&mut render_pass);
                    }
                }   
            }
        }
        self.queue.as_ref().unwrap().submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
        return Ok(());
    }

    pub(crate) fn apply_render_pass_with_values(
        &mut self,
        render_pass: &mut RenderPass<'_>, 
        instances: &[InstanceRaw], 
        texture_bind_group: Option<&BindGroup>
    ) {
        let vertex_buffer: Buffer = get_vertex_buffer(self, &shape::GeometryType::Circle(shape::Circle::default()).to_vertex_array(Orientation::Horizontal)).clone();
        let index_buffer: Buffer = get_index_buffer(self, &shape::GeometryType::Circle(shape::Circle::default()).to_index_array()).clone();

        let instance_buffer: Buffer = self.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(instances),
            usage: BufferUsages::VERTEX
        });

        if let Some(bind_group) = texture_bind_group {
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.set_bind_group(1, &self.window_bind_group.as_ref().unwrap().clone(), &[]);
        } else {
            render_pass.set_bind_group(0, &self.window_bind_group.as_ref().unwrap().clone(), &[]);
        }

        let size: u32 = shape::GeometryType::Circle(shape::Circle::default()).to_index_array().len() as u32;

        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..size, 0, 0..instances.len() as u32);
    }

    pub(crate) fn setup_sprite_rendering(
        &mut self,
        entity: Option<&Entity>,
        sprite: &Sprite,
        transform: Option<&Transform>,
        camera2d: &Camera2d,
        is_background: bool
    ) {
        let texture: Arc<texture::texture::Texture> = {
            if let Some(texture_from_cache) = self.texture_cache.get_texture(sprite.path.clone()) {
                texture_from_cache
            } else {
                self.texture_cache.load_texture(sprite.path.clone(), &self.device.as_ref().unwrap(), &self.queue.as_ref().unwrap()).unwrap()
            }
        };
        create_layouts_on_sprite_rendering(self, entity, sprite, texture.as_ref(), transform, camera2d, is_background);
    }

    pub(crate) fn setup_shape_rendering(&mut self, entity: &Entity, shape: &Shape, transform: Option<&Transform>, camera2d: &Camera2d) {
        let (window_bind_group, projection_buffer, view_buffer) = get_window_bindings(self, transform, camera2d);
        /*let (vertex_buffer, index_buffer) = get_vertex_and_index_buffers(
            self,
            Some(entity),
            &shape.geometry_type.to_vertex_array(Orientation::Horizontal),
            &shape.geometry_type.to_index_array()
        );*/

        let vertex_buffer: Buffer = get_vertex_buffer(self, &shape.geometry_type.to_vertex_array(Orientation::Horizontal)).clone();
        let index_buffer: Buffer = get_index_buffer(self, &shape.geometry_type.to_index_array()).clone();

        let instance_raw: InstanceRaw = InstanceRaw {
            transform: *transform.unwrap().to_matrix().as_ref(),
            color: shape.color.to_rgba()
        };
        self.shape_instances.push(instance_raw);

        self.window_bind_group = Some(window_bind_group);
        self.projection_buffer = Some(projection_buffer);
        self.view_buffer = Some(view_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.number_of_indices = Some(shape.geometry_type.to_index_array().len() as u32);
    }

    pub(crate) fn get_projection_matrix(&self, camera2d: &Camera2d) -> Matrix4<f32> {
        let aspect_ratio: f32 = self.physical_size.as_ref().unwrap().width as f32 / self.physical_size.as_ref().unwrap().height as f32;

        return ortho(
            -aspect_ratio * camera2d.zoom,
            aspect_ratio * camera2d.zoom,
            -1.0 * camera2d.zoom,
            1.0 * camera2d.zoom,
            -1.0,
            1.0
        );
    }
}

fn create_layouts_on_sprite_rendering(
    render_state: &mut RenderState,
    entity: Option<&Entity>,
    sprite: &Sprite,
    texture: &texture::texture::Texture,
    transform: Option<&Transform>,
    camera2d: &Camera2d,
    is_background: bool
) {
    let is_background_buffer: Buffer = render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
        label: Some("Is Background Buffer"),
        contents: bytemuck::cast_slice(&[if is_background { 1 } else { 0 }]),
        usage: BufferUsages::UNIFORM
    });

    let texture_bind_group: BindGroup = render_state.device.as_ref().unwrap().create_bind_group(&BindGroupDescriptor {
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
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::Buffer(is_background_buffer.as_entire_buffer_binding())
            }
        ]
    });

    let (window_bind_group, projection_buffer, view_buffer) = get_window_bindings(render_state, transform, camera2d);
    let (vertex_buffer, index_buffer) = get_vertex_and_index_buffers(
        render_state,
        entity,
        &sprite.vertices,
        &sprite.indices
    );

    let instance_raw: InstanceRaw = InstanceRaw {
        transform: *transform.unwrap().to_matrix().as_ref(),
        color: color::Color::WHITE.to_rgba()
    };
    render_state.sprite_instances.entry(texture.label.clone()).or_insert_with(Vec::new).push(instance_raw);
    render_state.texture_bind_groups.insert(texture.label.clone(), texture_bind_group);

    render_state.window_bind_group = Some(window_bind_group);
    render_state.projection_buffer = Some(projection_buffer);
    render_state.view_buffer = Some(view_buffer);
    render_state.vertex_buffer = Some(vertex_buffer);
    render_state.index_buffer = Some(index_buffer);
    render_state.number_of_indices = Some(sprite.indices.len() as u32);
}

fn get_window_bindings(render_state: &mut RenderState, transform: Option<&Transform>, camera2d: &Camera2d) -> (BindGroup, Buffer, Buffer) {
    let projection_buffer: Buffer = get_projection_buffer(render_state, camera2d);
    let view_buffer: Buffer = get_view_buffer(render_state, camera2d);

    /*
    if let Some(transform_unwrapped) = transform {
        let transform_matrix_unwrapped: [[f32; 4]; 4] = *transform_unwrapped.to_matrix().as_ref();
        let transform_buffer: Buffer = render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_matrix_unwrapped]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        render_state.transform_buffer = Some(transform_buffer);
    } else {
        let identity_matrix: Matrix4<f32> = Matrix4::identity();
        let identity_matrix_unwrapped: [[f32; 4]; 4] = *identity_matrix.as_ref();
        let transform_buffer: Buffer = render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[identity_matrix_unwrapped]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        render_state.transform_buffer = Some(transform_buffer);
    }*/

    let window_bind_group: BindGroup = render_state.device.as_ref().unwrap().create_bind_group(&BindGroupDescriptor {
        label: Some("Transform Bind Group"),
        layout: &render_state.window_bind_group_layout.as_ref().unwrap(),
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: projection_buffer.as_entire_binding()
            },
            BindGroupEntry {
                binding: 1,
                resource: view_buffer.as_entire_binding()
            }
        ]
    });
    return (window_bind_group, projection_buffer, view_buffer);
}

pub(crate) fn get_projection_buffer(render_state: &RenderState, camera2d: &Camera2d) -> Buffer {
    let projection_matrix: Matrix4<f32> = render_state.get_projection_matrix(camera2d);
    let projection_matrix_unwrapped: [[f32; 4]; 4] = *projection_matrix.as_ref();
    return render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
        label: Some("Projection Buffer"),
        contents: bytemuck::cast_slice(&[projection_matrix_unwrapped]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    });
}

pub(crate) fn get_view_buffer(render_state: &RenderState, camera2d: &Camera2d) -> Buffer {
    let view_matrix: Matrix4<f32> = camera2d.view_matrix;
    let view_matrix_unwrapped: [[f32; 4]; 4] = *view_matrix.as_ref();
    return render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
        label: Some("View Buffer"),
        contents: bytemuck::cast_slice(&[view_matrix_unwrapped]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    });
}

fn get_render_pipeline(render_state: &RenderState, bind_group_layouts: Vec<&BindGroupLayout>, shader_source: &str) -> RenderPipeline {
    let shader_module: ShaderModule = render_state.device.as_ref().unwrap().create_shader_module(ShaderModuleDescriptor {
        label: Some("Shader Module"),
        source: ShaderSource::Wgsl(shader_source.into())
    });
    let render_pipeline_layout: PipelineLayout = render_state.device.as_ref().unwrap().create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &bind_group_layouts[..],
        push_constant_ranges: &[]
    });
    let render_pipeline: RenderPipeline = render_state.device.as_ref().unwrap().create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::descriptor(), InstanceRaw::descriptor()],
            compilation_options: PipelineCompilationOptions::default()
        },
        fragment: Some(FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(ColorTargetState {
                format: render_state.surface_configuration.as_ref().unwrap().format,
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
    return render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertex_array),
        usage: BufferUsages::VERTEX
    });
}

fn get_index_buffer(render_state: &RenderState, index_array: &[u16]) -> Buffer {
    return render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(index_array),
        usage: BufferUsages::INDEX
    });
}
