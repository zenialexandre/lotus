use wgpu::*;
use uuid::Uuid;
use atomic_refcell::AtomicRefMut;
use cgmath::{ortho, Matrix4, SquareMatrix};
use glyph_brush::{ab_glyph::{point, Rect}, BrushAction, BrushError, GlyphBrush, GlyphVertex, Section};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};
use std::{cell::RefMut, collections::HashMap, sync::Arc};
use super::cache::{self, buffer_cache::BufferCache, bind_group_cache::BindGroupCache};
use super::rendering_type::RenderingType;
use super::super::super::{
    color,
    event_dispatcher::{EventDispatcher, Event, EventType},
    shape::{Orientation, Shape, GeometryType},
    physics::transform::{Transform, Strategy},
    draw_order::DrawOrder,
    texture,
    texture::{cache::TextureCache, sprite::Sprite, sprite_sheet::SpriteSheet},
    animation::Animation,
    text::text::TextRenderer,
    camera::camera2d::Camera2d,
    ecs::{entity::Entity, world::World, component::Component, resource::{ResourceRef, ResourceRefMut}}
};
use crate::utils::constants::{shader::SHADER_2D, cache::{RENDERING_TYPE_BUFFER, DUMMY_TEXTURE}};

/// Struct to represent the vertices that will be sent to the shader.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv_coordinates: [f32; 2],
    pub text_pixelated_position: [f32; 2],
    pub text_uv_coordinates: [f32; 2],
    pub color: [f32; 4]
}

impl Vertex {
    const VERTEX_ATTRIBUTES: [VertexAttribute; 5] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32x4
    ];

    fn descriptor() -> VertexBufferLayout<'static> {
        return VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRIBUTES
        };
    }

    pub(crate) fn build_default(position: [f32; 3], uv_coordinates: [f32; 2]) -> Self {
        return Self {
            position,
            uv_coordinates,
            text_pixelated_position: [0.0, 0.0],
            text_uv_coordinates: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0]
        };
    }

    fn to_vertex(mut glyph_vertex: GlyphVertex) -> [Vertex; 4] {
        let mut rect: Rect = Rect {
            min: point(glyph_vertex.pixel_coords.min.x, glyph_vertex.pixel_coords.min.y),
            max: point(glyph_vertex.pixel_coords.max.x, glyph_vertex.pixel_coords.max.y),
        };

        // Handle overlapping bounds -> modify the UV to preserve the texture aspect.
        // Extracted from the wgpu_text crate.
        if rect.max.x > glyph_vertex.bounds.max.x {
            let old_width: f32 = rect.width();
            rect.max.x = glyph_vertex.bounds.max.x;
            glyph_vertex.tex_coords.max.x = glyph_vertex.tex_coords.min.x + glyph_vertex.tex_coords.width() * rect.width() / old_width;
        }

        if rect.min.x < glyph_vertex.bounds.min.x {
            let old_width: f32 = rect.width();
            rect.min.x = glyph_vertex.bounds.min.x;
            glyph_vertex.tex_coords.min.x = glyph_vertex.tex_coords.max.x - glyph_vertex.tex_coords.width() * rect.width() / old_width;
        }

        if rect.max.y > glyph_vertex.bounds.max.y {
            let old_height: f32 = rect.height();
            rect.max.y = glyph_vertex.bounds.max.y;
            glyph_vertex.tex_coords.max.y = glyph_vertex.tex_coords.min.y + glyph_vertex.tex_coords.height() * rect.height() / old_height;
        }

        if rect.min.y < glyph_vertex.bounds.min.y {
            let old_height: f32 = rect.height();
            rect.min.y = glyph_vertex.bounds.min.y;
            glyph_vertex.tex_coords.min.y = glyph_vertex.tex_coords.max.y - glyph_vertex.tex_coords.height() * rect.height() / old_height;
        }

        // Generate a group of 4 vertices to form a Quad from the Glyph!
        let top_left: [f32; 3] = [rect.min.x, rect.min.y, glyph_vertex.extra.z];
        let bottom_right: [f32; 2] = [rect.max.x, rect.max.y];
        let uv_top_left: [f32; 2] = [glyph_vertex.tex_coords.min.x, glyph_vertex.tex_coords.min.y];
        let uv_bottom_right: [f32; 2] = [glyph_vertex.tex_coords.max.x, glyph_vertex.tex_coords.max.y];

        return [
            // Top-Left
            Vertex {
                position: [0.0, 0.0, 0.0],
                uv_coordinates: [0.0, 0.0],
                text_pixelated_position: [rect.min.x, rect.min.y],
                text_uv_coordinates: [glyph_vertex.tex_coords.min.x, glyph_vertex.tex_coords.min.y],
                color: glyph_vertex.extra.color
            },
            // Top-Right
            Vertex {
                position: [0.0, 0.0, 0.0],
                uv_coordinates: [0.0, 0.0],
                text_pixelated_position: [rect.max.x, rect.min.y],
                text_uv_coordinates: [glyph_vertex.tex_coords.max.x, glyph_vertex.tex_coords.min.y],
                color: glyph_vertex.extra.color
            },
            // Bottom-Left
            Vertex {
                position: [0.0, 0.0, 0.0],
                uv_coordinates: [0.0, 0.0],
                text_pixelated_position: [rect.min.x, rect.max.y],
                text_uv_coordinates: [glyph_vertex.tex_coords.min.x, glyph_vertex.tex_coords.max.y],
                color: glyph_vertex.extra.color
            },
            // Bottom-Right
            Vertex {
                position: [0.0, 0.0, 0.0],
                uv_coordinates: [0.0, 0.0],
                text_pixelated_position: [rect.max.x, rect.max.y],
                text_uv_coordinates: [glyph_vertex.tex_coords.max.x, glyph_vertex.tex_coords.max.y],
                color: glyph_vertex.extra.color
            }
        ]
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
    pub screen_size_buffer: Option<Buffer>,
    pub transform_buffer: Option<Buffer>,
    pub projection_buffer: Option<Buffer>,
    pub view_buffer: Option<Buffer>,
    pub rendering_type_bind_group: Option<BindGroup>,
    pub texture_bind_group: Option<BindGroup>,
    pub transform_bind_group: Option<BindGroup>,
    pub rendering_type_bind_group_layout: Option<BindGroupLayout>,
    pub texture_bind_group_layout: Option<BindGroupLayout>,
    pub transform_bind_group_layout: Option<BindGroupLayout>,
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
            vertex_buffer: None,
            index_buffer: None,
            screen_size_buffer: None,
            transform_buffer: None,
            projection_buffer: None,
            view_buffer: None,
            number_of_indices: None,
            rendering_type_bind_group: None,
            texture_bind_group: None,
            transform_bind_group: None,
            rendering_type_bind_group_layout: None,
            texture_bind_group_layout: None,
            transform_bind_group_layout: None,
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
            screen_size_buffer: None,
            transform_buffer: None,
            projection_buffer: None,
            view_buffer: None,
            number_of_indices: None,
            rendering_type_bind_group: None,
            texture_bind_group: None,
            transform_bind_group: None,
            rendering_type_bind_group_layout: None,
            texture_bind_group_layout: None,
            transform_bind_group_layout: None,
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
                    visibility: ShaderStages::VERTEX_FRAGMENT,
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
                },
                BindGroupLayoutEntry {
                    binding: 3,
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
            vec![
                &rendering_type_bind_group_layout,
                &texture_bind_group_layout,
                &transform_bind_group_layout,
            ],
            SHADER_2D
        );

        render_state.render_pipeline_2d = Some(render_pipeline_2d);
        render_state.rendering_type_bind_group_layout = Some(rendering_type_bind_group_layout);
        render_state.texture_bind_group_layout = Some(texture_bind_group_layout);
        render_state.transform_bind_group_layout = Some(transform_bind_group_layout);
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

            let _ = cache::outbound_functions::get_projection_or_view_buffer(self, true, None, camera2d);

            if !text_renderers.is_empty() {
                for text_renderer in text_renderers {
                    let mut glyph_brush: RefMut<'_, GlyphBrush<[Vertex; 4]>> = text_renderer.1.glyph_brush.borrow_mut();
                    
                    /*glyph_brush.resize_texture(
                        new_size.width,
                        new_size.height
                    );*/
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

    /// Helper function to clean the Buffer cache related to the entity.
    pub(crate) fn clean_entity_buffer_cache(&mut self, entity: &Entity) {
        self.buffer_cache.clean(entity.0.to_string());
    }

    /// Helper function to clean the Bind Group cache related to the entity.
    pub(crate) fn clean_entity_bind_group_cache(&mut self, entity: &Entity) {
        self.bind_group_cache.clean(entity.0.to_string());
    }

    /// Execute the rendering process.
    pub fn render(&mut self, world: &mut World) -> Result<(), SurfaceError> {
        let surface_texture: SurfaceTexture = self.surface.as_ref().unwrap().get_current_texture()?;
        let texture_view: TextureView = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        let mut command_encoder: CommandEncoder = self.device.as_ref().unwrap().create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let camera2d: ResourceRef<'_, Camera2d> = world.get_resource::<Camera2d>().unwrap();
            let mut event_dispatcher: ResourceRefMut<'_, EventDispatcher> = world.get_resource_mut::<EventDispatcher>().unwrap();
            let mut render_pass: RenderPass<'_> = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    depth_slice: None,
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
                    &mut event_dispatcher,
                    None,
                    Some(&background_sprite),
                    None,
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
                if world.is_entity_alive(entity) {
                    let is_entity_visible: bool = world.is_entity_visible(entity);
                    let components: Vec<AtomicRefMut<'_, Box<dyn Component>>> = world.get_entity_components_mut(&entity).unwrap();
                    let transform: Option<&Transform> = components.iter().find_map(
                        |component| component.as_any().downcast_ref::<Transform>()
                    );
                    let animation: Option<&Animation> = components.iter().find_map(
                        |component| component.as_any().downcast_ref::<Animation>()
                    );

                    render_pass.set_pipeline(self.render_pipeline_2d.as_ref().unwrap());

                    if let Some(animation) = animation {
                        if !animation.playing_stack.is_empty() {
                            self.setup_rendering_2d(
                                &mut event_dispatcher,
                                Some(&entity),
                                None,
                                None,
                                None,
                                transform,
                                Some(animation),
                                &camera2d,
                                false
                            );

                            if is_entity_visible {
                                self.apply_render_pass_with_values(&mut render_pass);
                            }
                            continue;
                        }
                    }

                    if let Some(sprite) = components.iter().find_map(|component| component.as_any().downcast_ref::<Sprite>()) {
                        self.setup_rendering_2d(
                            &mut event_dispatcher,
                            Some(&entity),
                            Some(sprite),
                            None,
                            None,
                            transform,
                            animation,
                            &camera2d,
                            false
                        );

                        if is_entity_visible {
                            self.apply_render_pass_with_values(&mut render_pass);
                        }
                    } else if let Some(shape) = components.iter().find_map(|component| component.as_any().downcast_ref::<Shape>()) {
                        self.setup_rendering_2d(
                            &mut event_dispatcher,
                            Some(&entity),
                            None,
                            Some(shape),
                            None,
                            transform,
                            None,
                            &camera2d,
                            false
                        );

                        if is_entity_visible {
                            self.apply_render_pass_with_values(&mut render_pass);
                        }
                    } else if let Some(text_renderer) = world.get_text_renderer(&entity) {
                        self.setup_rendering_2d(
                            &mut event_dispatcher,
                            Some(&entity),
                            None,
                            None,
                            Some(text_renderer),
                            transform,
                            None,
                            &camera2d,
                            false
                        );

                        if is_entity_visible {
                            self.apply_render_pass_with_values(&mut render_pass);
                        }
                    }
                }
            }
        }
        self.queue.as_ref().unwrap().submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
        return Ok(());
    }

    pub(crate) fn apply_render_pass_with_values(&mut self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_bind_group(0, &self.rendering_type_bind_group, &[]);
        render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
        render_pass.set_bind_group(2, &self.transform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_mut().unwrap().slice(..));
        render_pass.set_index_buffer(self.index_buffer.as_mut().unwrap().slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.number_of_indices.unwrap(), 0, 0..1);
    }

    pub(crate) fn setup_rendering_2d(
        &mut self,
        event_dispatcher: &mut EventDispatcher,
        entity: Option<&Entity>,
        sprite: Option<&Sprite>,
        shape: Option<&Shape>,
        text_renderer: Option<&TextRenderer>,
        transform: Option<&Transform>,
        animation: Option<&Animation>,
        camera2d: &Camera2d,
        is_background: bool
    ) {
        if let Some(sprite) = sprite {
            self.setup_sprite_rendering(
                event_dispatcher,
                entity,
                sprite,
                transform,
                camera2d,
                is_background
            );
        } else if let Some(animation) = animation {
            self.setup_animation_rendering(
                event_dispatcher,
                entity,
                animation,
                transform,
                camera2d,
                is_background
            );
        } else if let Some(shape) = shape {
            self.setup_shape_rendering(
                event_dispatcher,
                entity,
                shape,
                transform,
                camera2d
            );
        } else {
            self.setup_text_rendering(
                event_dispatcher,
                entity,
                text_renderer.unwrap(),
                transform,
                camera2d
            );
        }
    }

    pub(crate) fn setup_sprite_rendering(
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
                    &self.queue.as_ref().unwrap(),
                    false,
                    None,
                    None
                ).unwrap()
            }
        };
        let rendering_type_buffer: Buffer = cache::outbound_functions::get_conditional_buffer(
            self,
            RENDERING_TYPE_BUFFER,
            entity,
            if is_background { RenderingType::BACKGROUND.to_shader_index() } else { RenderingType::TEXTURE.to_shader_index() }
        );
        let rendering_type_bind_group: BindGroup = cache::outbound_functions::get_rendering_type_bind_group(
            self,
            entity,
            rendering_type_buffer
        );
        let texture_bind_group: BindGroup = cache::outbound_functions::get_texture_bind_group(
            self,
            entity,
            texture.as_ref(),
            None
        );

        let (transform_bind_group, screen_size_buffer, projection_buffer, view_buffer) = self.get_transform_bindings(
            RenderingType::TEXTURE,
            event_dispatcher,
            entity,
            transform,
            None,
            None,
            Some(texture.as_ref()),
            camera2d
        );

        let (vertex_buffer, index_buffer) = cache::outbound_functions::get_vertex_and_index_buffers(
            self,
            entity,
            &sprite.vertices,
            &sprite.indices
        );

        self.texture_bind_group = Some(texture_bind_group);
        self.transform_bind_group = Some(transform_bind_group);
        self.rendering_type_bind_group = Some(rendering_type_bind_group);
        self.screen_size_buffer = Some(screen_size_buffer);
        self.projection_buffer = Some(projection_buffer);
        self.view_buffer = Some(view_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.number_of_indices = Some(sprite.indices.len() as u32);
    }

    pub(crate) fn setup_animation_rendering(
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
                        &self.queue.as_ref().unwrap(),
                        false,
                        None,
                        None
                    ).unwrap()
                }
            };
            let rendering_type_buffer: Buffer = cache::outbound_functions::get_conditional_buffer(
                self,
                RENDERING_TYPE_BUFFER,
                entity,
                if is_background { RenderingType::BACKGROUND.to_shader_index() } else { RenderingType::TEXTURE.to_shader_index() }
            );
            let rendering_type_bind_group: BindGroup = cache::outbound_functions::get_rendering_type_bind_group(
                self,
                entity,
                rendering_type_buffer
            );
            let texture_bind_group: BindGroup = cache::outbound_functions::get_texture_bind_group(self, entity, texture.as_ref(), Some(sprite_sheet));

            let mut vertices: Vec<Vertex> = GeometryType::Square.to_vertex_array(Orientation::Horizontal);
            let indices: Vec<u16> = GeometryType::Square.to_index_array();
            let texure_coordinates: [f32; 8] = sprite_sheet.current_tile_uv_coordinates();

            vertices[0].uv_coordinates = [texure_coordinates[0], texure_coordinates[1]];
            vertices[1].uv_coordinates = [texure_coordinates[2], texure_coordinates[3]]; 
            vertices[2].uv_coordinates = [texure_coordinates[4], texure_coordinates[5]];
            vertices[3].uv_coordinates = [texure_coordinates[6], texure_coordinates[7]];

            let (transform_bind_group, screen_size_buffer, projection_buffer, view_buffer) = self.get_transform_bindings(
                RenderingType::TEXTURE,
                event_dispatcher,
                entity,
                transform,
                Some(sprite_sheet.tile_width),
                Some(sprite_sheet.tile_height),
                Some(texture.as_ref()),
                camera2d
            );

            let (vertex_buffer, index_buffer) = cache::outbound_functions::get_vertex_and_index_buffers(
                self,
                entity,
                &vertices,
                &indices
            );

            self.texture_bind_group = Some(texture_bind_group);
            self.transform_bind_group = Some(transform_bind_group);
            self.rendering_type_bind_group = Some(rendering_type_bind_group);
            self.screen_size_buffer = Some(screen_size_buffer);
            self.projection_buffer = Some(projection_buffer);
            self.view_buffer = Some(view_buffer);
            self.vertex_buffer = Some(vertex_buffer);
            self.index_buffer = Some(index_buffer);
            self.number_of_indices = Some(indices.len() as u32);
        }
    }

    pub(crate) fn setup_shape_rendering(
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
                    &self.queue.as_ref().unwrap(),
                    false,
                    None,
                    None
                ).unwrap()
            }
        };
        let rendering_type_buffer: Buffer = cache::outbound_functions::get_conditional_buffer(
            self, 
            RENDERING_TYPE_BUFFER,
            entity,
            RenderingType::TEXT.to_shader_index()
        );
        let rendering_type_bind_group: BindGroup = cache::outbound_functions::get_rendering_type_bind_group(
            self,
            entity,
            rendering_type_buffer
        );
        let texture_bind_group: BindGroup = cache::outbound_functions::get_texture_bind_group(
            self,
            entity,
            &texture,
            None
        );

        let (transform_bind_group, screen_size_buffer, projection_buffer, view_buffer) = self.get_transform_bindings(
            RenderingType::SHAPE,
            event_dispatcher,
            entity,
            transform,
            None,
            None,
            None,
            camera2d
        );

        let mut vertices: Vec<Vertex> = shape.geometry_type.to_vertex_array(Orientation::Horizontal);
        for vertex in &mut vertices {
            vertex.color = shape.color.to_rgba();
        }

        let (vertex_buffer, index_buffer) = cache::outbound_functions::get_vertex_and_index_buffers(
            self,
            entity,
            &vertices,
            &shape.geometry_type.to_index_array()
        );

        self.texture_bind_group = Some(texture_bind_group);
        self.transform_bind_group = Some(transform_bind_group);
        self.rendering_type_bind_group = Some(rendering_type_bind_group);
        self.screen_size_buffer = Some(screen_size_buffer);
        self.projection_buffer = Some(projection_buffer);
        self.view_buffer = Some(view_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.number_of_indices = Some(shape.geometry_type.to_index_array().len() as u32);
    }

    pub(crate) fn setup_text_rendering(
        &mut self,
        event_dispatcher: &mut EventDispatcher,
        entity: Option<&Entity>,
        text_renderer: &TextRenderer,
        transform: Option<&Transform>,
        camera2d: &Camera2d
    ) {
        let rendering_type_buffer: Buffer = cache::outbound_functions::get_conditional_buffer(
            self, 
            RENDERING_TYPE_BUFFER,
            entity,
            RenderingType::TEXT.to_shader_index()
        );
        let rendering_type_bind_group: BindGroup = cache::outbound_functions::get_rendering_type_bind_group(
            self,
            entity,
            rendering_type_buffer
        );
        let (transform_bind_group, screen_size_buffer, projection_buffer, view_buffer) = self.get_transform_bindings(
            RenderingType::TEXT,
            event_dispatcher,
            entity,
            transform,
            None,
            None,
            None,
            camera2d
        );

        let mut glyph_brush: RefMut<GlyphBrush<[Vertex; 4]>> = text_renderer.glyph_brush.borrow_mut();
        let width: f32 = self.physical_size.as_ref().unwrap().width as f32;
        let height: f32 = self.physical_size.as_ref().unwrap().height as f32;
        let section: Section = Section::default().add_text(
            glyph_brush::Text::new(&text_renderer.text.content)
                .with_color(text_renderer.text.color.to_rgba())
                .with_scale(text_renderer.text.font.size)
        )
        .with_screen_position(text_renderer.text.get_position_by_strategy(self.physical_size.as_ref().unwrap()))
        .with_bounds((width, height));

        glyph_brush.queue(section.clone());

        loop {
            let brush_action: Result<BrushAction<[Vertex; 4]>, BrushError> = glyph_brush.process_queued(
                |size, data| {
                    let texture: Arc<texture::texture::Texture> = {
                        if let Some(texture_from_cache) = self.texture_cache.get_texture(text_renderer.text.uuid.to_string()) {
                            texture_from_cache
                        } else {
                            self.texture_cache.load_texture(
                                text_renderer.text.uuid.to_string(),
                                &self.device.as_ref().unwrap(),
                                &self.queue.as_ref().unwrap(),
                                false,
                                Some(size),
                                Some(data),
                            ).unwrap()
                        }
                    };

                    self.queue.as_ref().unwrap().write_texture(
                        TexelCopyTextureInfo {
                            texture: &texture.wgpu_texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d {
                                x: size.min[0],
                                y: size.min[1],
                                z: 0,
                            },
                            aspect: TextureAspect::All,
                        },
                        data,
                        TexelCopyBufferLayout {
                            offset: 0,
                            bytes_per_row: Some(size.width()),
                            rows_per_image: Some(size.height()),
                        },
                        Extent3d {
                            width: size.width(),
                            height: size.height(),
                            depth_or_array_layers: 1,
                        },
                    );

                    let texture_bind_group: BindGroup = cache::outbound_functions::get_texture_bind_group(
                        self,
                        entity,
                        &texture,
                        None
                    );
                    self.texture_bind_group = Some(texture_bind_group);
                },
                |glyph_vertex: GlyphVertex<'_>| Vertex::to_vertex(glyph_vertex)
            );

            match brush_action {
                Ok(action) => {
                    break match action {
                        BrushAction::Draw(generated_vertices) => {
                            let mut vertices: Vec<Vertex> = Vec::new();
                            let mut indices: Vec<u16> = Vec::new();

                            for (i, vertex_group) in generated_vertices.into_iter().enumerate() {
                                // Add the group of vertices (Glyph) to the list
                                vertices.extend(vertex_group);

                                eprintln!("Primeiro glifo ({}):", i);
                                for (j, vertex) in vertex_group.iter().enumerate() {
                                    eprintln!("  Vértice {}: posição={:?}, UV={:?}", 
                                            j, vertex.text_pixelated_position, vertex.text_uv_coordinates);
                                }

                                // 6 indices per Glyph -> 3 triangles
                                let base_index: u16 = (i * 4) as u16;
                                indices.extend(&[
                                    base_index,
                                    base_index + 1,
                                    base_index + 2,
                                    base_index + 1,
                                    base_index + 3,
                                    base_index + 2
                                ]);
                            }
                            text_renderer.vertices.borrow_mut().extend(vertices);
                            text_renderer.indices.borrow_mut().extend(indices);

                            eprintln!("vertices {:?}",  text_renderer.vertices.borrow_mut().len());
                            eprintln!("indices {:?}",  text_renderer.indices.borrow_mut().len());
                        },
                        BrushAction::ReDraw => ()
                    }
                },
                Err(BrushError::TextureTooSmall { suggested }) => {
                    let limits: u32 = self.device.as_ref().unwrap().limits().max_texture_dimension_2d;
                    let (width, height): (u32, u32) = if suggested.0 > limits || suggested.1 > limits {
                        if glyph_brush.texture_dimensions().0 < limits || glyph_brush.texture_dimensions().1 < limits {
                            (limits, limits)
                        } else {
                            suggested
                        }
                    } else {
                        suggested
                    };
                    glyph_brush.resize_texture(width, height);
                }
            }
        }

        let (vertex_buffer, index_buffer) = cache::outbound_functions::get_vertex_and_index_buffers(
            self,
            entity,
            &text_renderer.vertices.borrow(),
            &text_renderer.indices.borrow()
        );

        self.transform_bind_group = Some(transform_bind_group);
        self.rendering_type_bind_group = Some(rendering_type_bind_group);
        self.screen_size_buffer = Some(screen_size_buffer);
        self.projection_buffer = Some(projection_buffer);
        self.view_buffer = Some(view_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.number_of_indices = Some(text_renderer.indices.borrow().len() as u32);
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
        rendering_type: RenderingType,
        event_dispatcher: &mut EventDispatcher,
        entity: Option<&Entity>,
        transform: Option<&Transform>,
        tile_width: Option<f32>,
        tile_height: Option<f32>,
        texture: Option<&texture::texture::Texture>,
        camera2d: &Camera2d
    ) -> (BindGroup, Buffer, Buffer, Buffer) {
        let screen_size_buffer: Buffer = cache::outbound_functions::get_screen_size_buffer(self, entity);
        let projection_buffer: Buffer = cache::outbound_functions::get_projection_or_view_buffer(self, true, entity, camera2d);
        let view_buffer: Buffer = cache::outbound_functions::get_projection_or_view_buffer(self, false, entity, camera2d);
        let (width, height): (f32, f32) = (
            (self.physical_size.as_ref().unwrap().width as f32),
            (self.physical_size.as_ref().unwrap().height as f32)
        );
        let aspect_ratio: f32 = width / height;

        if let Some(transform_unwrapped) = transform {
            let mut transform_cloned: Transform = transform_unwrapped.clone();

            if
                transform_cloned.position.strategy == Strategy::Pixelated &&
                transform_cloned.dirty_position &&
                RenderingType::TEXT != rendering_type
            {
                let normalized_x: f32 = transform_cloned.position.x / width * 2.0 * aspect_ratio - aspect_ratio;
                let normalized_y: f32 = -(transform_cloned.position.y / height * 2.0 - 1.0);

                transform_cloned.position.x = normalized_x;
                transform_cloned.position.y = normalized_y;

                event_dispatcher.send(Event::new(*entity.unwrap(), EventType::UpdatePixelatedPosition, transform_cloned.position.to_vec()));
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

                    event_dispatcher.send(Event::new(*entity.unwrap(), EventType::UpdatePixelatedScale, transform_cloned.scale));
                }
            }

            let transform_unwrapped: [[f32; 4]; 4] = *transform_cloned.to_matrix().as_ref();
            let transform_buffer: Buffer = cache::outbound_functions::get_transform_buffer(self, entity, transform_unwrapped);
            self.transform_buffer = Some(transform_buffer);
        } else {
            let identity_matrix: Matrix4<f32> = Matrix4::identity();
            let identity_matrix_unwrapped: [[f32; 4]; 4] = *identity_matrix.as_ref();
            let transform_buffer: Buffer = cache::outbound_functions::get_transform_buffer(self, entity, identity_matrix_unwrapped);
            self.transform_buffer = Some(transform_buffer);
        }

        let transform_bind_group: BindGroup = cache::outbound_functions::get_transform_bind_group(
            self,
            entity,
            screen_size_buffer.clone(),
            self.transform_buffer.as_ref().unwrap().clone(),
            projection_buffer.clone(),
            view_buffer.clone()
        );
        return (transform_bind_group, screen_size_buffer, projection_buffer, view_buffer);
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
                cull_mode: None,
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
