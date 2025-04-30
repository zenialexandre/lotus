use atomic_refcell::AtomicRefMut;
use cgmath::{ortho, Matrix4, SquareMatrix};
use uuid::Uuid;
use wgpu_text::glyph_brush::Section;
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};
use wgpu::{*, util::{BufferInitDescriptor, DeviceExt}};
use std::{collections::HashMap, sync::Arc};

use super::cache::{self, BufferCache, BindGroupCache};
use super::super::super::{
    color,
    shape::{Orientation, Shape, GeometryType},
    physics::transform::{Transform, Strategy},
    draw_order::DrawOrder,
    texture,
    texture::{texture::TextureCache, sprite::Sprite, sprite_sheet::SpriteSheet},
    animation::Animation,
    text::TextRenderer,
    camera::camera2d::Camera2d,
    ecs::{entity::Entity, world::World, component::Component, resource::ResourceRef}
};
use crate::utils::constants::shader::SHADER_2D;

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
    pub render_pipeline_2d: Option<RenderPipeline>,
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
    pub number_of_indices: Option<u32>,
    pub texture_bind_group: Option<BindGroup>,
    pub color_bind_group: Option<BindGroup>,
    pub projection_buffer: Option<Buffer>,
    pub view_buffer: Option<Buffer>,
    pub transform_buffer: Option<Buffer>,
    pub transform_bind_group: Option<BindGroup>,
    pub transform_bind_group_layout: Option<BindGroupLayout>,
    pub texture_bind_group_layout: Option<BindGroupLayout>,
    pub color_bind_group_layout: Option<BindGroupLayout>,
    pub entities_to_render: Vec<Entity>,
    pub texture_cache: TextureCache,
    pub buffer_cache: BufferCache,
    pub bind_group_cache: BindGroupCache
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
            render_pipeline_2d: None,
            vertex_buffer: None,
            index_buffer: None,
            transform_buffer: None,
            projection_buffer: None,
            view_buffer: None,
            number_of_indices: None,
            color_bind_group: None,
            texture_bind_group: None,
            transform_bind_group: None,
            transform_bind_group_layout: None,
            texture_bind_group_layout: None,
            color_bind_group_layout: None,
            entities_to_render: Vec::new(),
            texture_cache: TextureCache::new(),
            buffer_cache: BufferCache::new(),
            bind_group_cache: BindGroupCache::new()
        };
    }

    /// Create a new asynchronous rendering state for the window.
    pub async fn new(window: Arc<Window>, present_mode: PresentMode) -> Self {
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
            vertex_buffer: None,
            index_buffer: None,
            transform_buffer: None,
            projection_buffer: None,
            view_buffer: None,
            number_of_indices: None,
            color_bind_group: None,
            texture_bind_group: None,
            transform_bind_group: None,
            transform_bind_group_layout: None,
            texture_bind_group_layout: None,
            color_bind_group_layout: None,
            entities_to_render: Vec::new(),
            texture_cache: TextureCache::new(),
            buffer_cache: BufferCache::new(),
            bind_group_cache: BindGroupCache::new()
        };

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
                },
                BindGroupLayoutEntry{
                    binding: 3,
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
        let color_bind_group_layout: BindGroupLayout = render_state.device.as_ref().unwrap().create_bind_group_layout(&BindGroupLayoutDescriptor {
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
            vec![&texture_bind_group_layout, &color_bind_group_layout, &transform_bind_group_layout],
            SHADER_2D
        );

        render_state.render_pipeline_2d = Some(render_pipeline_2d);
        render_state.transform_bind_group_layout = Some(transform_bind_group_layout);
        render_state.texture_bind_group_layout = Some(texture_bind_group_layout);
        render_state.color_bind_group_layout = Some(color_bind_group_layout);
        return render_state;
    }

    /// Returns the window reference.
    pub fn window(&self) -> &Window {
        return &self.window.as_ref().unwrap();
    }

    /// Resize the rendering projection.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>, camera2d: &Camera2d, text_renderers: &HashMap<Uuid, TextRenderer>) {
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

    pub(crate) fn input(&mut self, window_event: &WindowEvent) -> bool {
        match window_event {
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
    pub fn render(&mut self, world: &mut World) -> Result<(), SurfaceError> {
        let surface_texture: SurfaceTexture = self.surface.as_ref().unwrap().get_current_texture()?;
        let texture_view: TextureView = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        let mut command_encoder: CommandEncoder = self.device.as_ref().unwrap().create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });
        self.prepare_text_rendering(world);

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
                render_pass.set_pipeline(self.render_pipeline_2d.as_ref().unwrap());
                self.setup_rendering_2d(
                    None,
                    Some(&background_sprite),
                    None,
                    None,
                    None,
                    &camera2d,
                    true
                );
                self.apply_render_pass_with_values(&mut render_pass);
            }

            let mut entities_to_render_sorted: Vec<Entity> = self.entities_to_render.clone();
            if entities_to_render_sorted.len() > 1 {
                entities_to_render_sorted.sort_by(|a, b| {
                    DrawOrder::compare(world, a, b)
                });
            }

            for entity in entities_to_render_sorted.clone() {
                if world.is_entity_alive(entity) && world.is_entity_visible(entity) {
                    let components: Vec<AtomicRefMut<'_, Box<dyn Component>>> = world.get_entity_components_mut(&entity).unwrap();
                    let transform: Option<&Transform> = components.iter().find_map(
                        |component| component.as_any().downcast_ref::<Transform>()
                    );
                    let animation: Option<&Animation> = components.iter().find_map(
                        |component| component.as_any().downcast_ref::<Animation>()
                    );

                    if let Some(animation) = animation {
                        if !animation.playing_stack.is_empty() {
                            render_pass.set_pipeline(self.render_pipeline_2d.as_ref().unwrap());
                            self.setup_rendering_2d(
                                Some(&entity),
                                None,
                                None,
                                transform,
                                Some(animation),
                                &camera2d,
                                false
                            );
                            self.apply_render_pass_with_values(&mut render_pass);
                            continue;
                        }
                    }

                    if let Some(sprite) = components.iter().find_map(|component| component.as_any().downcast_ref::<Sprite>()) {
                        render_pass.set_pipeline(self.render_pipeline_2d.as_ref().unwrap());
                        self.setup_rendering_2d(
                            Some(&entity),
                            Some(sprite),
                            None,
                            transform,
                            animation,
                            &camera2d,
                            false
                        );
                        self.apply_render_pass_with_values(&mut render_pass);
                    } else if let Some(shape) = components.iter().find_map(|component| component.as_any().downcast_ref::<Shape>()) {
                        render_pass.set_pipeline(self.render_pipeline_2d.as_ref().unwrap());
                        self.setup_rendering_2d(
                            Some(&entity),
                            None,
                            Some(shape),
                            transform,
                            None,
                            &camera2d,
                            false
                        );
                        self.apply_render_pass_with_values(&mut render_pass);
                    } else if let Some(text_renderer) = world.text_renderers.get(&entity.0) {
                        text_renderer.text_brush.draw(&mut render_pass);
                    }
                }
            }
        }
        self.queue.as_ref().unwrap().submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
        return Ok(());
    }

    pub(crate) fn prepare_text_rendering(&mut self, world: &mut World) {
        for entity in self.entities_to_render.clone() {
            if world.is_entity_alive(entity) && world.is_entity_visible(entity) {
                if let Some(text_renderer) = world.text_renderers.get_mut(&entity.0) {
                    let (x, y): (f32, f32) = text_renderer.text.get_position_by_strategy(&self.physical_size.as_ref().unwrap());

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
    }

    pub(crate) fn apply_render_pass_with_values(&mut self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        render_pass.set_bind_group(1, &self.color_bind_group, &[]);
        render_pass.set_bind_group(2, &self.transform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_mut().unwrap().slice(..));
        render_pass.set_index_buffer(self.index_buffer.as_mut().unwrap().slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.number_of_indices.unwrap(), 0, 0..1);
    }

    pub(crate) fn setup_rendering_2d(
        &mut self,
        entity: Option<&Entity>,
        sprite: Option<&Sprite>,
        shape: Option<&Shape>,
        transform: Option<&Transform>,
        animation: Option<&Animation>,
        camera2d: &Camera2d,
        is_background: bool
    ) {
        if let Some(sprite) = sprite {
            self.setup_sprite_rendering(
                entity,
                sprite,
                transform,
                camera2d,
                is_background
            );
        } else if let Some(animation) = animation {
            self.setup_animation_rendering(
                entity,
                animation,
                transform,
                camera2d,
                is_background
            );
        } else if let Some(shape) = shape {
            self.setup_shape_rendering(entity, shape, transform, camera2d);
        }
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
                self.texture_cache.load_texture(
                    sprite.path.clone(),
                    &self.device.as_ref().unwrap(),
                    &self.queue.as_ref().unwrap()
                ).unwrap()
            }
        };
        let is_background_buffer: Buffer = self.get_conditional_buffer(cache::IS_BACKGROUND, entity, if is_background { 1 } else { 0 });
        let is_texture_buffer: Buffer = self.get_conditional_buffer(cache::IS_TEXTURE, entity, 1);
        let texture_bind_group: BindGroup = self.get_texture_bind_group(entity, texture.as_ref(), is_background_buffer, is_texture_buffer);

        let color_buffer: Buffer = self.get_color_buffer(entity, None);
        let color_bind_group: BindGroup = self.get_color_bind_group(entity, color_buffer);

        let (transform_bind_group, projection_buffer, view_buffer) = self.get_transform_bindings(
            entity,
            transform,
            None,
            None,
            Some(texture.as_ref()),
            camera2d
        );

        let (vertex_buffer, index_buffer) = self.get_vertex_and_index_buffers(
            entity,
            &sprite.vertices,
            &sprite.indices
        );

        self.texture_bind_group = Some(texture_bind_group);
        self.color_bind_group = Some(color_bind_group);
        self.transform_bind_group = Some(transform_bind_group);
        self.projection_buffer = Some(projection_buffer);
        self.view_buffer = Some(view_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.number_of_indices = Some(sprite.indices.len() as u32);
    }

    pub(crate) fn setup_animation_rendering(
        &mut self,
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
            let is_background_buffer: Buffer = self.get_conditional_buffer(cache::IS_BACKGROUND, entity, if is_background { 1 } else { 0 });
            let is_texture_buffer: Buffer = self.get_conditional_buffer(cache::IS_TEXTURE, entity, 1);
            let texture_bind_group: BindGroup = self.get_texture_bind_group_sheet(
                entity.unwrap(),
                sprite_sheet,
                texture.as_ref(),
                is_background_buffer,
                is_texture_buffer
            );

            let color_buffer: Buffer = self.get_color_buffer(entity, None);
            let color_bind_group: BindGroup = self.get_color_bind_group(entity, color_buffer);

            let mut vertices: Vec<Vertex> = GeometryType::Square.to_vertex_array(Orientation::Horizontal);
            let indices: Vec<u16> = GeometryType::Square.to_index_array();
            let texure_coordinates: [f32; 8] = sprite_sheet.current_tile_texture_coordinates();

            vertices[0].texture_coordinates = [texure_coordinates[0], texure_coordinates[1]];
            vertices[1].texture_coordinates = [texure_coordinates[2], texure_coordinates[3]]; 
            vertices[2].texture_coordinates = [texure_coordinates[4], texure_coordinates[5]];
            vertices[3].texture_coordinates = [texure_coordinates[6], texure_coordinates[7]];

            let (transform_bind_group, projection_buffer, view_buffer) = self.get_transform_bindings(
                entity,
                transform,
                Some(sprite_sheet.tile_width),
                Some(sprite_sheet.tile_height),
                Some(texture.as_ref()),
                camera2d
            );

            let (vertex_buffer, index_buffer) = self.get_vertex_and_index_buffers(
                entity,
                &vertices,
                &indices
            );

            self.texture_bind_group = Some(texture_bind_group);
            self.color_bind_group = Some(color_bind_group);
            self.transform_bind_group = Some(transform_bind_group);
            self.projection_buffer = Some(projection_buffer);
            self.view_buffer = Some(view_buffer);
            self.vertex_buffer = Some(vertex_buffer);
            self.index_buffer = Some(index_buffer);
            self.number_of_indices = Some(indices.len() as u32);
        }
    }

    pub(crate) fn setup_shape_rendering(
        &mut self,
        entity: Option<&Entity>,
        shape: &Shape,
        transform: Option<&Transform>,
        camera2d: &Camera2d
    ) {
        let color_buffer: Buffer = self.get_color_buffer(entity, Some(shape));
        let color_bind_group: BindGroup = self.get_color_bind_group(entity, color_buffer);

        let texture: Arc<texture::texture::Texture> = {
            if let Some(texture_from_cache) = self.texture_cache.get_texture(texture::texture::TextureCache::DUMMY_TEXTURE.to_string()) {
                texture_from_cache
            } else {
                self.texture_cache.load_texture(
                    texture::texture::TextureCache::DUMMY_TEXTURE.to_string(),
                    &self.device.as_ref().unwrap(),
                    &self.queue.as_ref().unwrap()
                ).unwrap()
            }
        };
        let is_background_buffer: Buffer = self.get_conditional_buffer(cache::IS_BACKGROUND, entity,  0);
        let is_texture_buffer: Buffer = self.get_conditional_buffer(cache::IS_TEXTURE, entity, 0);
        let texture_bind_group: BindGroup = self.get_texture_bind_group(entity, &texture, is_background_buffer, is_texture_buffer);

        let (transform_bind_group, projection_buffer, view_buffer) = self.get_transform_bindings(
            entity,
            transform,
            None,
            None,
            None,
            camera2d
        );
        let (vertex_buffer, index_buffer) = self.get_vertex_and_index_buffers(
            entity,
            &shape.geometry_type.to_vertex_array(Orientation::Horizontal),
            &shape.geometry_type.to_index_array()
        );

        self.texture_bind_group = Some(texture_bind_group);
        self.color_bind_group = Some(color_bind_group);
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
        entity: Option<&Entity>,
        transform: Option<&Transform>,
        tile_width: Option<f32>,
        tile_height: Option<f32>,
        texture: Option<&texture::texture::Texture>,
        camera2d: &Camera2d
    ) -> (BindGroup, Buffer, Buffer) {
        let projection_buffer: Buffer = self.get_projection_or_view_buffer(true, entity, camera2d);
        let view_buffer: Buffer = self.get_projection_or_view_buffer(false, entity, camera2d);
        let (width, height): (f32, f32) = (
            (self.physical_size.as_ref().unwrap().width as f32),
            (self.physical_size.as_ref().unwrap().height as f32)
        );
        let aspect_ratio: f32 = width / height;

        if let Some(transform_unwrapped) = transform {
            let mut transform_cloned: Transform = transform_unwrapped.clone();

            if transform_cloned.position.strategy == Strategy::Pixelated {
                let pixelated_x: f32 = transform_cloned.position.x / width * 2.0 * aspect_ratio - aspect_ratio;
                let pixelated_y: f32 = -(transform_cloned.position.y / height * 2.0 - 1.0);

                transform_cloned.position.x = pixelated_x;
                transform_cloned.position.y = pixelated_y;
            }

            if let Some(texture) = texture {
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
            }
            let transform_unwrapped: [[f32; 4]; 4] = *transform_cloned.to_matrix().as_ref();
            let transform_buffer: Buffer = self.get_transform_buffer(entity, transform_unwrapped);
            self.transform_buffer = Some(transform_buffer);
        } else {
            let identity_matrix: Matrix4<f32> = Matrix4::identity();
            let identity_matrix_unwrapped: [[f32; 4]; 4] = *identity_matrix.as_ref();
            let transform_buffer: Buffer = self.get_transform_buffer(entity, identity_matrix_unwrapped);
            self.transform_buffer = Some(transform_buffer);
        }

        let transform_bind_group: BindGroup = self.get_transform_bind_group(
            entity,
            self.transform_buffer.as_ref().unwrap().clone(),
            projection_buffer.clone(),
            view_buffer.clone()
        );
        return (transform_bind_group, projection_buffer, view_buffer);
    }

    pub(crate) fn get_texture_bind_group(
        &mut self,
        entity: Option<&Entity>,
        texture: &texture::texture::Texture,
        is_background_buffer: Buffer,
        is_texture_buffer: Buffer
    ) -> BindGroup {
        let uuid: String = cache::extract_id_from_entity(entity);
        let key: (String, String) = (uuid, cache::TEXTURE_BIND_GROUP.to_string());

        if let Some(texture_bind_group) = self.bind_group_cache.find(key.clone()) {
            return texture_bind_group.clone();
        } else {
            let texture_bind_group: BindGroup = self.device.as_ref().unwrap().create_bind_group(&BindGroupDescriptor {
                label: Some("Texture Bind Group"),
                layout: &self.texture_bind_group_layout.as_ref().unwrap(),
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
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: BindingResource::Buffer(is_texture_buffer.as_entire_buffer_binding())
                    }
                ]
            });
            self.bind_group_cache.cache.insert(key, texture_bind_group.clone());
            return texture_bind_group.clone();
        }
    }

    pub(crate) fn get_texture_bind_group_sheet(
        &mut self,
        entity: &Entity,
        sprite_sheet: &SpriteSheet,
        texture: &texture::texture::Texture,
        is_background_buffer: Buffer,
        is_texture_buffer: Buffer
    ) -> BindGroup {
        let key: (String, String) = (entity.0.to_string(), sprite_sheet.path.to_string());

        if let Some(texture_bind_group) = self.bind_group_cache.find(key.clone()) {
            return texture_bind_group.clone();
        } else {
            let texture_bind_group: BindGroup = self.device.as_ref().unwrap().create_bind_group(&BindGroupDescriptor {
                label: Some("Texture Bind Group"),
                layout: &self.texture_bind_group_layout.as_ref().unwrap(),
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
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: BindingResource::Buffer(is_texture_buffer.as_entire_buffer_binding())
                    }
                ]
            });
            self.bind_group_cache.cache.insert(key, texture_bind_group.clone());
            return texture_bind_group.clone();
        }
    }

    pub(crate) fn get_color_bind_group(&mut self, entity: Option<&Entity>, color_buffer: Buffer) -> BindGroup {
        let uuid: String = cache::extract_id_from_entity(entity);
        let key: (String, String) = (uuid, cache::COLOR_BIND_GROUP.to_string());

        if let Some(color_bind_group) = self.bind_group_cache.find(key.clone()) {
            return color_bind_group.clone();
        } else {
            let color_bind_group: BindGroup = self.device.as_ref().unwrap().create_bind_group(&BindGroupDescriptor {
                label: Some("Color Bind Group"),
                layout: &self.color_bind_group_layout.as_ref().unwrap(),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: color_buffer.as_entire_binding()
                    }
                ]
            });
            self.bind_group_cache.cache.insert(key, color_bind_group.clone());
            return color_bind_group;
        }
    }

    pub(crate) fn get_transform_bind_group(
        &mut self,
        entity: Option<&Entity>,
        transform_buffer: Buffer,
        projection_buffer: Buffer,
        view_buffer: Buffer
    ) -> BindGroup {
        let uuid: String = cache::extract_id_from_entity(entity);
        let key: (String, String) = (uuid, cache::TRANSFORM_BIND_GROUP.to_string());

        if let Some(transform_bind_group) = self.bind_group_cache.find(key.clone()) {
            return transform_bind_group.clone();
        } else {
            let transform_bind_group: BindGroup = self.device.as_ref().unwrap().create_bind_group(&BindGroupDescriptor {
                label: Some("Transform Bind Group"),
                layout: &self.transform_bind_group_layout.as_ref().unwrap(),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: transform_buffer.as_entire_binding()
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: projection_buffer.as_entire_binding()
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: view_buffer.as_entire_binding()
                    }
                ]
            });
            self.bind_group_cache.cache.insert(key, transform_bind_group.clone());
            return transform_bind_group;
        }
    }

    pub(crate) fn get_conditional_buffer(&mut self, title: &str, entity: Option<&Entity>, value: u32) -> Buffer {
        let uuid: String = cache::extract_id_from_entity(entity);
        let key: (String, String) = (uuid, title.to_string().clone());

        if let Some(buffer) = self.buffer_cache.find(key.clone()) {
            self.queue.as_ref().unwrap().write_buffer(
                &buffer,
                0,
                bytemuck::cast_slice(&[value])
            );
            return buffer.clone();
        } else {
            let buffer: Buffer = self.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
                label: Some(title),
                contents: bytemuck::cast_slice(&[value]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
            });
            self.buffer_cache.cache.insert(key, buffer.clone());
            return buffer.clone();
        }
    }

    pub(crate) fn get_vertex_and_index_buffers(&mut self, entity: Option<&Entity>, vertex_array: &[Vertex], index_array: &[u16]) -> (Buffer, Buffer) {
        let uuid: String = cache::extract_id_from_entity(entity);
        let vertex_key: (String, String) = (uuid.clone(), cache::VERTEX.to_string());
        let index_key: (String, String) = (uuid.clone(), cache::INDEX.to_string());

        if let (Some(vertex_buffer), Some(index_buffer)) = (
            self.buffer_cache.find(vertex_key.clone()),
            self.buffer_cache.find(index_key.clone())
        ) {
            self.queue.as_ref().unwrap().write_buffer(
                &vertex_buffer,
                0,
                bytemuck::cast_slice(vertex_array)
            );
            return (vertex_buffer, index_buffer);
        } else {
            let vertex_buffer: Buffer = self.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertex_array),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST
            });
            let index_buffer: Buffer = self.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(index_array),
                usage: BufferUsages::INDEX | BufferUsages::COPY_DST
            });

            self.buffer_cache.cache.insert(vertex_key, vertex_buffer.clone());
            self.buffer_cache.cache.insert(index_key, index_buffer.clone());
            return (vertex_buffer.clone(), index_buffer.clone());
        }
    }

    pub(crate) fn get_color_buffer(&mut self, entity: Option<&Entity>, shape: Option<&Shape>) -> Buffer {
        let uuid: String = cache::extract_id_from_entity(entity);
        let key: (String, String) = (uuid, cache::COLOR_BUFFER.to_string());
        let color: color::Color = if let Some(shape) = shape { shape.color } else { color::Color::WHITE };

        if let Some(color_buffer) = self.buffer_cache.find(key.clone()) {
            self.queue.as_ref().unwrap().write_buffer(
                &color_buffer,
                0,
                bytemuck::cast_slice(&color::Color::to_array(color::Color::to_wgpu(color)))
            );
            return color_buffer.clone();
        } else {
            let color_buffer: Buffer = self.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
                label: Some("Color Buffer"),
                contents: bytemuck::cast_slice(&color::Color::to_array(color::Color::to_wgpu(color))),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
            });
            self.buffer_cache.cache.insert(key, color_buffer.clone());
            return color_buffer;
        }
    }

    pub(crate) fn get_transform_buffer(&mut self, entity: Option<&Entity>, transform_matrix_unwrapped: [[f32; 4]; 4]) -> Buffer {
        let uuid: String = cache::extract_id_from_entity(entity);
        let key: (String, String) = (uuid, cache::TRANSFORM_BUFFER.to_string());

        if let Some(transform_buffer) = self.buffer_cache.find(key.clone()) {
            self.queue.as_ref().unwrap().write_buffer(
                &transform_buffer,
                0,
                bytemuck::cast_slice(&[transform_matrix_unwrapped])
            );
            return transform_buffer.clone();
        } else {
            let transform_buffer: Buffer = self.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
                label: Some("Transform Buffer"),
                contents: bytemuck::cast_slice(&[transform_matrix_unwrapped]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
            });
            self.buffer_cache.cache.insert(key, transform_buffer.clone());
            return transform_buffer.clone();
        }
    }

    pub(crate) fn get_projection_or_view_buffer(&mut self, is_projection: bool, entity: Option<&Entity>, camera2d: &Camera2d) -> Buffer {
        let title: &str = if is_projection { cache::PROJECTION } else { cache::VIEW };
        let uuid: String = cache::extract_id_from_entity(entity);
        let key: (String, String) = (uuid, title.to_string().clone());
        let matrix: Matrix4<f32> = if is_projection { self.get_projection_matrix(camera2d) } else { camera2d.view_matrix };
        let matrix_unwrapped: [[f32; 4]; 4] = *matrix.as_ref();

        if let Some(buffer) = self.buffer_cache.find(key.clone()) {
            self.queue.as_ref().unwrap().write_buffer(
                &buffer,
                0,
                bytemuck::cast_slice(&[matrix_unwrapped])
            );
            return buffer.clone();
        } else {
            let buffer: Buffer = self.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
                label: Some(title),
                contents: bytemuck::cast_slice(&[matrix_unwrapped]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
            });
            self.buffer_cache.cache.insert(key, buffer.clone());
            return buffer;
        }
    }

    pub(crate) fn get_render_pipeline(&self, bind_group_layouts: Vec<&BindGroupLayout>, shader_source: &str) -> RenderPipeline {
        let shader_module: ShaderModule = self.device.as_ref().unwrap().create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: ShaderSource::Wgsl(shader_source.into())
        });
        let render_pipeline_layout: PipelineLayout = self.device.as_ref().unwrap().create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &bind_group_layouts[..],
            push_constant_ranges: &[]
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
            multiview: None,
            cache: None
        });
        return render_pipeline;
    }
}
