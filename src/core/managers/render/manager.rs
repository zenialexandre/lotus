use wgpu::*;
use uuid::Uuid;
use cgmath::{ortho, Matrix4, SquareMatrix};
use wgpu_text::glyph_brush::Section;
use winit::event_loop::ActiveEventLoop;
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};
use std::{collections::HashMap, sync::Arc};
use super::cache::{self, buffer::BufferCache, bind_group::BindGroupCache};
use super::rendering_type::RenderingType;
use super::super::super::{
    super::{ColorOption},
    event::dispatcher::{EventDispatcher, Event, EventType, SubEventType},
    shape::{shape::Shape, geometry_type::GeometryType, orientation::Orientation},
    physics::transform::{Transform, Strategy},
    texture,
    texture::{cache::TextureCache, sprite::Sprite, sprite_sheet::SpriteSheet},
    animation::animation::Animation,
    text::text::{TextHolder, TextRenderer},
    camera::camera2d::Camera2d,
    ecs::{entity::Entity, world::World}
};
use crate::utils::constants::{shader::SHADER_2D, cache::{RENDERING_TYPE_BUFFER, DUMMY_TEXTURE}};

/// Struct to represent the vertices that will be sent to the shader.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv_coordinates: [f32; 2],
    pub color: [f32; 4]
}

impl Vertex {
    const VERTEX_ATTRIBUTES: [VertexAttribute; 3] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
        2 => Float32x4
    ];

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
    pub color: Option<crate::core::color::color::Color>,
    pub background_image_path: Option<String>,
    pub window: Option<Arc<Window>>,
    pub render_pipeline_2d: Option<RenderPipeline>,
    pub number_of_indices: Option<u32>,
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
    pub transform_buffer: Option<Buffer>,
    pub projection_buffer: Option<Buffer>,
    pub view_buffer: Option<Buffer>,
    pub rendering_type_bind_group_layout: Option<BindGroupLayout>,
    pub rendering_type_bind_group: Option<BindGroup>,
    pub texture_bind_group_layout: Option<BindGroupLayout>,
    pub texture_bind_group: Option<BindGroup>,
    pub transform_bind_group_layout: Option<BindGroupLayout>,
    pub transform_bind_group: Option<BindGroup>,
    pub entities_to_render: Vec<Entity>,
    pub texture_cache: TextureCache,
    pub buffer_cache: BufferCache,
    pub bind_group_cache: BindGroupCache
}

impl RenderState {
    /// Create a dummy rendering state.
    ///
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
            render_pipeline_2d: None,
            number_of_indices: None,
            vertex_buffer: None,
            index_buffer: None,
            transform_buffer: None,
            projection_buffer: None,
            view_buffer: None,
            rendering_type_bind_group_layout: None,
            rendering_type_bind_group: None,
            texture_bind_group_layout: None,
            texture_bind_group: None,
            transform_bind_group_layout: None,
            transform_bind_group: None,
            entities_to_render: Vec::new(),
            texture_cache: TextureCache::new(),
            buffer_cache: BufferCache::new(),
            bind_group_cache: BindGroupCache::new()
        };
    }

    /// Create a new asynchronous rendering state for the window.
    pub async fn new(window: Arc<Window>, present_mode: PresentMode, event_loop: &ActiveEventLoop) -> Self {
        let physical_size: PhysicalSize<u32> = window.inner_size();
        let instance: Instance = Instance::new(InstanceDescriptor{
            backends: Backends::PRIMARY,
            backend_options: BackendOptions::from_env_or_default(),
            flags: InstanceFlags::default(),
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            display: Some(Box::new(event_loop.owned_display_handle()))
        });

        let surface: Surface = instance.create_surface(window.clone()).expect("Failed to create WGPU Surface.");
        let adapter: Adapter = instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            },
        ).await.unwrap();

        let (device, queue): (Device, Queue) = adapter.request_device(
            &DeviceDescriptor {
                required_features: Features::default(),
                required_limits: Limits::default(),
                label: None,
                memory_hints: Default::default(),
                trace: Trace::Off,
                experimental_features: ExperimentalFeatures::disabled()
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
            present_mode,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &surface_configuration);

        let mut render_state: RenderState = Self {
            surface: Some(surface),
            device: Some(device),
            queue: Some(queue),
            surface_configuration: Some(surface_configuration),
            physical_size: Some(physical_size),
            color: None,
            background_image_path: None,
            window: Some(window),
            render_pipeline_2d: None,
            number_of_indices: None,
            vertex_buffer: None,
            index_buffer: None,
            transform_buffer: None,
            projection_buffer: None,
            view_buffer: None,
            rendering_type_bind_group_layout: None,
            rendering_type_bind_group: None,
            texture_bind_group_layout: None,
            texture_bind_group: None,
            transform_bind_group_layout: None,
            transform_bind_group: None,
            entities_to_render: Vec::new(),
            texture_cache: TextureCache::new(),
            buffer_cache: BufferCache::new(),
            bind_group_cache: BindGroupCache::new()
        };

        let rendering_type_bind_group_layout: BindGroupLayout = render_state.device.as_ref().unwrap().create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Rendering Type Bind Group Layout"),
            entries: &[
                BindGroupLayoutEntry{
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
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
                }
            ]
        });
        let transform_bind_group_layout: BindGroupLayout = render_state.device.as_ref().unwrap().create_bind_group_layout(&BindGroupLayoutDescriptor {
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
                },
                BindGroupLayoutEntry {
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

        let render_pipeline_2d: RenderPipeline = render_state.get_render_pipeline(
            vec![Some(&rendering_type_bind_group_layout), Some(&texture_bind_group_layout), Some(&transform_bind_group_layout)],
            SHADER_2D
        );

        render_state.render_pipeline_2d = Some(render_pipeline_2d);
        render_state.rendering_type_bind_group_layout = Some(rendering_type_bind_group_layout);
        render_state.texture_bind_group_layout = Some(texture_bind_group_layout);
        render_state.transform_bind_group_layout = Some(transform_bind_group_layout);
        return render_state;
    }

    /// Execute the rendering process.
    pub(crate) fn prepare(&mut self, world: &mut World, event_loop: &ActiveEventLoop) {
        match self.surface.as_ref().unwrap().get_current_texture() {
            CurrentSurfaceTexture::Success(surface_texture) => {
                super::executor::on_success(self, world, surface_texture);
            },
            CurrentSurfaceTexture::Suboptimal(_) => {
                super::executor::on_suboptimal(self, world);
            },
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded | CurrentSurfaceTexture::Outdated => {
                log::warn!("Surface timeout, occluded or outdated");
            },
            CurrentSurfaceTexture::Lost => {
                log::error!("Surface lost error!");
                event_loop.exit();
            },
            CurrentSurfaceTexture::Validation => {
                log::error!("Surface validation error!");
                event_loop.exit();
            }
        }
    }

    /// Apply render pass with values and render.
    pub(crate) fn render(&mut self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_bind_group(0, &self.rendering_type_bind_group, &[]);
        render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
        render_pass.set_bind_group(2, &self.transform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_mut().unwrap().slice(..));
        render_pass.set_index_buffer(self.index_buffer.as_mut().unwrap().slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.number_of_indices.unwrap(), 0, 0..1);
    }

    /// Returns the window reference.
    pub fn window(&self) -> &Window {
        return &self.window.as_ref().unwrap();
    }

    /// Resize the rendering projection.
    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>, camera2d: &Camera2d, text_renderers: &HashMap<Uuid, TextRenderer>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.physical_size = Some(new_size);
            self.surface_configuration.as_mut().unwrap().width = new_size.width;
            self.surface_configuration.as_mut().unwrap().height = new_size.height;
            self.surface.as_ref().unwrap().configure(&self.device.as_ref().unwrap(), &self.surface_configuration.as_ref().unwrap());

            let _ = &cache::buffer::get_projection_or_view_buffer(
                self,
                true,
                None,
                camera2d
            );

            if !text_renderers.is_empty() {
                for text_renderer in text_renderers {
                    text_renderer.1.text_brush.update_matrix(
                        wgpu_text::ortho(new_size.width as f32, new_size.height as f32),
                        self.queue.as_ref().unwrap()
                    );
                }
            }
        }
    }

    /// Prepare for rendering text by applying necessary values.
    pub(crate) fn text(&mut self, world: &mut World) {
        for entity in self.entities_to_render.clone() {
            if world.is_entity_alive(entity) && world.is_entity_visible(entity) {
                if let Some(text_renderer) = world.get_resource_mut::<TextHolder>().unwrap().text_renderers.get_mut(&entity.0) {
                    let (x, y): (f32, f32) = text_renderer.text.get_position_by_strategy(&self.physical_size.as_ref().unwrap());
                    let width: f32 = self.physical_size.as_ref().unwrap().width as f32;
                    let height: f32 = self.physical_size.as_ref().unwrap().height as f32;

                    text_renderer.text_brush.queue(
                        self.device.as_ref().unwrap(),
                        self.queue.as_ref().unwrap(),
                        vec![Section {
                            screen_position: (x, y),
                            bounds: (width, height),
                            text: vec![
                                wgpu_text::glyph_brush::Text::new(&text_renderer.text.content)
                                    .with_color(text_renderer.text.color.to_array())
                                    .with_scale(text_renderer.text.font.size)
                            ],
                            ..Default::default()
                        }]
                    ).ok();
                }
            }
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

    /// Helper function to clean the Buffer cache related to the entity.
    pub(crate) fn clean_entity_buffer_cache(&mut self, entity: &Entity) {
        self.buffer_cache.clean(entity.0.to_string());
    }

    /// Helper function to clean the Bind Group cache related to the entity.
    pub(crate) fn clean_entity_bind_group_cache(&mut self, entity: &Entity) {
        self.bind_group_cache.clean(entity.0.to_string());
    }

    /// Placeholder function for the input functionality.
    pub(crate) fn input(&mut self, window_event: &WindowEvent) -> bool {
        match window_event {
            _ =>  { return false; }
        }
    }

    /// Send the entity to its rendering process.
    pub(crate) fn setup(
        &mut self,
        event_dispatcher: &mut EventDispatcher,
        entity: Option<&Entity>,
        sprite: Option<&Sprite>,
        shape: Option<&Shape>,
        transform: Option<&Transform>,
        animation: Option<&Animation>,
        camera2d: &Camera2d,
        is_background: bool
    ) {
        if let Some(sprite) = sprite {
            self.sprite(
                event_dispatcher,
                entity,
                sprite,
                transform,
                camera2d,
                is_background
            );
        } else if let Some(animation) = animation {
            self.animation(
                event_dispatcher,
                entity,
                animation,
                transform,
                camera2d,
                is_background
            );
        } else if let Some(shape) = shape {
            self.shape(
                event_dispatcher,
                entity,
                shape,
                transform,
                camera2d
            );
        }
    }

    /// Prepare for sprite rendering.
    pub(crate) fn sprite(
        &mut self,
        event_dispatcher: &mut EventDispatcher,
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
                self.texture_cache.load_texture(
                    sprite.path.clone(),
                    &self.device.as_ref().unwrap(),
                    &self.queue.as_ref().unwrap()
                ).unwrap()
            }
        };
        let rendering_type_buffer: Buffer = cache::buffer::get_conditional_buffer(
            self,
            RENDERING_TYPE_BUFFER,
            entity,
            if is_background { RenderingType::Background.to_shader_index() } else { RenderingType::Texture.to_shader_index() }
        );
        let rendering_type_bind_group: BindGroup = cache::bind_group::get_rendering_type_bind_group(
            self,
            entity,
            rendering_type_buffer
        );
        let texture_bind_group: BindGroup = cache::bind_group::get_texture_bind_group(
            self,
            entity,
            texture.as_ref(),
            None
        );

        let (transform_bind_group, projection_buffer, view_buffer): (BindGroup, Buffer, Buffer) = self.get_transform_bindings(
            event_dispatcher,
            entity,
            transform,
            None,
            None,
            Some(texture.as_ref()),
            camera2d
        );

        let (vertex_buffer, index_buffer): (Buffer, Buffer) = cache::buffer::get_vertex_and_index_buffers(
            self,
            entity,
            &sprite.vertices,
            &sprite.indices
        );

        self.rendering_type_bind_group = Some(rendering_type_bind_group);
        self.texture_bind_group = Some(texture_bind_group);
        self.transform_bind_group = Some(transform_bind_group);
        self.projection_buffer = Some(projection_buffer);
        self.view_buffer = Some(view_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.number_of_indices = Some(sprite.indices.len() as u32);
    }

    /// Prepare for animation rendering.
    pub(crate) fn animation(
        &mut self,
        event_dispatcher: &mut EventDispatcher,
        entity: Option<&Entity>,
        animation: &Animation,
        transform: Option<&Transform>,
        camera2d: &Camera2d,
        is_background: bool
    ) {
        let sprite_sheet: Option<&SpriteSheet> = animation.get_playing_animation_now();

        if let Some(sprite_sheet) = sprite_sheet {
            let texture: Arc<texture::texture::Texture> = {
                if let Some(texture_from_cache) = self.texture_cache.get_texture(sprite_sheet.path.clone()) {
                    texture_from_cache
                } else {
                    self.texture_cache.load_texture(
                        sprite_sheet.path.clone(),
                        &self.device.as_ref().unwrap(),
                        &self.queue.as_ref().unwrap()
                    ).unwrap()
                }
            };
            let rendering_type_buffer: Buffer = cache::buffer::get_conditional_buffer(
                self,
                RENDERING_TYPE_BUFFER,
                entity,
                if is_background { RenderingType::Background.to_shader_index() } else { RenderingType::Texture.to_shader_index() }
            );
            let rendering_type_bind_group: BindGroup = cache::bind_group::get_rendering_type_bind_group(
                self,
                entity,
                rendering_type_buffer
            );
            let texture_bind_group: BindGroup = cache::bind_group::get_texture_bind_group(
                self,
                entity,
                texture.as_ref(),
                Some(sprite_sheet)
            );

            let mut vertices: Vec<Vertex> = GeometryType::Square.to_vertex_array(Orientation::Horizontal, ColorOption::White.to_rgba());
            let indices: Vec<u16> = GeometryType::Square.to_index_array();
            let uv_coordinates : [f32; 8] = sprite_sheet.current_tile_uv_coordinates();

            vertices[0].uv_coordinates  = [uv_coordinates [0], uv_coordinates [1]];
            vertices[1].uv_coordinates  = [uv_coordinates [2], uv_coordinates [3]];
            vertices[2].uv_coordinates  = [uv_coordinates [4], uv_coordinates [5]];
            vertices[3].uv_coordinates  = [uv_coordinates [6], uv_coordinates [7]];

            let (transform_bind_group, projection_buffer, view_buffer): (BindGroup, Buffer, Buffer) = self.get_transform_bindings(
                event_dispatcher,
                entity,
                transform,
                Some(sprite_sheet.tile_width),
                Some(sprite_sheet.tile_height),
                Some(texture.as_ref()),
                camera2d
            );

            let (vertex_buffer, index_buffer): (Buffer, Buffer) = cache::buffer::get_vertex_and_index_buffers(
                self,
                entity,
                &vertices,
                &indices
            );

            self.rendering_type_bind_group = Some(rendering_type_bind_group);
            self.texture_bind_group = Some(texture_bind_group);
            self.transform_bind_group = Some(transform_bind_group);
            self.projection_buffer = Some(projection_buffer);
            self.view_buffer = Some(view_buffer);
            self.vertex_buffer = Some(vertex_buffer);
            self.index_buffer = Some(index_buffer);
            self.number_of_indices = Some(indices.len() as u32);
        }
    }

    /// Prepare for shape rendering.
    pub(crate) fn shape(
        &mut self,
        event_dispatcher: &mut EventDispatcher,
        entity: Option<&Entity>,
        shape: &Shape,
        transform: Option<&Transform>,
        camera2d: &Camera2d
    ) {
        let texture: Arc<texture::texture::Texture> = {
            if let Some(texture_from_cache) = self.texture_cache.get_texture(DUMMY_TEXTURE.to_string()) {
                texture_from_cache
            } else {
                self.texture_cache.load_texture(
                    DUMMY_TEXTURE.to_string(),
                    &self.device.as_ref().unwrap(),
                    &self.queue.as_ref().unwrap()
                ).unwrap()
            }
        };
        let rendering_type_buffer: Buffer = cache::buffer::get_conditional_buffer(
            self,
            RENDERING_TYPE_BUFFER,
            entity,
            RenderingType::Text.to_shader_index()
        );
        let rendering_type_bind_group: BindGroup = cache::bind_group::get_rendering_type_bind_group(
            self,
            entity,
            rendering_type_buffer
        );
        let texture_bind_group: BindGroup = cache::bind_group::get_texture_bind_group(
            self,
            entity,
            &texture,
            None
        );

        let (transform_bind_group, projection_buffer, view_buffer): (BindGroup, Buffer, Buffer) = self.get_transform_bindings(
            event_dispatcher,
            entity,
            transform,
            None,
            None,
            None,
            camera2d
        );
        let (vertex_buffer, index_buffer): (Buffer, Buffer) = cache::buffer::get_vertex_and_index_buffers(
            self,
            entity,
            &shape.geometry_type.to_vertex_array(Orientation::Horizontal, shape.color.to_array()),
            &shape.geometry_type.to_index_array()
        );

        self.rendering_type_bind_group = Some(rendering_type_bind_group);
        self.texture_bind_group = Some(texture_bind_group);
        self.transform_bind_group = Some(transform_bind_group);
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

    pub(crate) fn get_transform_bindings(
        &mut self,
        event_dispatcher: &mut EventDispatcher,
        entity: Option<&Entity>,
        transform: Option<&Transform>,
        tile_width: Option<f32>,
        tile_height: Option<f32>,
        texture: Option<&texture::texture::Texture>,
        camera2d: &Camera2d
    ) -> (BindGroup, Buffer, Buffer) {
        let projection_buffer: Buffer = cache::buffer::get_projection_or_view_buffer(
            self,
            true,
            entity,
            camera2d
        );
        let view_buffer: Buffer = cache::buffer::get_projection_or_view_buffer(
            self,
            false,
            entity,
            camera2d
        );
        let (width, height): (f32, f32) = (
            (self.physical_size.as_ref().unwrap().width as f32),
            (self.physical_size.as_ref().unwrap().height as f32)
        );
        let aspect_ratio: f32 = width / height;

        if let Some(transform_unwrapped) = transform {
            let mut transform_cloned: Transform = transform_unwrapped.clone();

            if transform_cloned.position.strategy == Strategy::Pixelated && transform_cloned.dirty_position {
                let normalized_x: f32 = transform_cloned.position.x / width * 2.0 * aspect_ratio - aspect_ratio;
                let normalized_y: f32 = -(transform_cloned.position.y / height * 2.0 - 1.0);

                transform_cloned.position.x = normalized_x;
                transform_cloned.position.y = normalized_y;

                event_dispatcher.send(Event::new(*entity.unwrap(), EventType::Transform(SubEventType::UpdatePixelatedPosition), transform_cloned.position.to_vec()));
            }

            if let Some(texture) = texture {
                if transform_cloned.dirty_scale {
                    let width_in_pixels: f32 = texture.wgpu_texture.size().width as f32;
                    let height_in_pixels: f32 = texture.wgpu_texture.size().height as f32;
                    let world_width: f32;
                    let world_height: f32;

                    if let (Some(tile_width), Some(tile_height)) = (tile_width, tile_height) {
                        world_width = (tile_width / width) * 1.0 * aspect_ratio;
                        world_height = (tile_height / height) * 1.0;
                    } else {
                        world_width = (width_in_pixels / width) * 1.0 * aspect_ratio;
                        world_height = (height_in_pixels / height) * 1.0;
                    }
                    transform_cloned.scale.x *= world_width;
                    transform_cloned.scale.y *= world_height;

                    event_dispatcher.send(Event::new(*entity.unwrap(), EventType::Transform(SubEventType::UpdatePixelatedScale), transform_cloned.scale));
                }
            }

            let transform_unwrapped: [[f32; 4]; 4] = *transform_cloned.to_matrix().as_ref();
            let transform_buffer: Buffer = cache::buffer::get_transform_buffer(self, entity, transform_unwrapped);
            self.transform_buffer = Some(transform_buffer);
        } else {
            let identity_matrix: Matrix4<f32> = Matrix4::identity();
            let identity_matrix_unwrapped: [[f32; 4]; 4] = *identity_matrix.as_ref();
            let transform_buffer: Buffer = cache::buffer::get_transform_buffer(self, entity, identity_matrix_unwrapped);
            self.transform_buffer = Some(transform_buffer);
        }

        let transform_bind_group: BindGroup = cache::bind_group::get_transform_bind_group(
            self,
            entity,
            self.transform_buffer.as_ref().unwrap().clone(),
            projection_buffer.clone(),
            view_buffer.clone()
        );
        return (transform_bind_group, projection_buffer, view_buffer);
    }

    pub(crate) fn get_render_pipeline(&self, bind_group_layouts: Vec<Option<&BindGroupLayout>>, shader_source: &str) -> RenderPipeline {
        let shader_module: ShaderModule = self.device.as_ref().unwrap().create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: ShaderSource::Wgsl(shader_source.into())
        });
        let render_pipeline_layout: PipelineLayout = self.device.as_ref().unwrap().create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &bind_group_layouts[..],
            immediate_size: 0
        });
        let render_pipeline: RenderPipeline = self.device.as_ref().unwrap().create_render_pipeline(&RenderPipelineDescriptor {
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
                    format: self.surface_configuration.as_ref().unwrap().format,
                    blend: Some(BlendState {
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
                    }),
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
            multiview_mask: None,
            cache: None
        });
        return render_pipeline;
    }
}
