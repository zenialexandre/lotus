use wgpu::*;
use atomic_refcell::AtomicRefMut;
use crate::RenderState;
use super::super::super::{
    super::ColorOption,
    color::color,
    event::dispatcher::EventDispatcher,
    shape::shape::Shape,
    physics::transform::Transform,
    draw_order::DrawOrder,
    texture::sprite::Sprite,
    animation::animation::Animation,
    text::text::TextHolder,
    camera::camera2d::Camera2d,
    ecs::{entity::Entity, world::World, component::Component, resource::{ResourceRef, ResourceRefMut}}
};

/// Execute processes related to the sucess of getting the surface texture (frame).
pub(crate) fn on_success(render_state: &mut RenderState, world: &mut World, surface_texture: SurfaceTexture) {
    let texture_view: TextureView = surface_texture.texture.create_view(&TextureViewDescriptor::default());
    let mut command_encoder: CommandEncoder = render_state.device.as_ref().unwrap().create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Render Encoder")
    });
    render_state.text(world);

    {
        let camera2d: ResourceRef<'_, Camera2d> = world.get_resource::<Camera2d>().unwrap();
        let mut event_dispatcher: ResourceRefMut<'_, EventDispatcher> = world.get_resource_mut::<EventDispatcher>().unwrap();
        let text_holder: ResourceRef<'_, TextHolder> = world.get_resource::<TextHolder>().unwrap();
        let mut render_pass: RenderPass<'_> = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                depth_slice: None,
                view: &texture_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(color::Color::to_wgpu(render_state.color.unwrap_or_else(|| color::Color::by_option(ColorOption::White)))),
                    store: StoreOp::Store
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None
        });
        render_pass.set_viewport(
            0.0,
            0.0,
            render_state.physical_size.as_ref().unwrap().width as f32,
            render_state.physical_size.as_ref().unwrap().height as f32,
            0.0,
            1.0
        );

        if let Some(background_image_path) = &render_state.background_image_path {
            render_pass.set_pipeline(render_state.render_pipeline_2d.as_ref().unwrap());
            let background_sprite: Sprite = Sprite::new(background_image_path.to_string());
            render_state.setup(
                &mut event_dispatcher,
                None,
                Some(&background_sprite),
                None,
                None,
                None,
                &camera2d,
                true
            );
            render_state.render(&mut render_pass);
        }

        for entity in get_entities_to_render_sorted(render_state, world).clone() {
            if world.is_entity_alive(entity) {
                let is_entity_visible: bool = world.is_entity_visible(entity);
                let components: Vec<AtomicRefMut<'_, Box<dyn Component>>> = world.get_entity_components_mut(&entity).unwrap();
                let transform: Option<&Transform> = components.iter().find_map(
                    |component| component.as_any().downcast_ref::<Transform>()
                );
                let animation: Option<&Animation> = components.iter().find_map(
                    |component| component.as_any().downcast_ref::<Animation>()
                );

                if let Some(animation) = animation {
                    if !animation.playing_stack.is_empty() {
                        render_pass.set_pipeline(render_state.render_pipeline_2d.as_ref().unwrap());
                        render_state.setup(
                            &mut event_dispatcher,
                            Some(&entity),
                            None,
                            None,
                            transform,
                            Some(animation),
                            &camera2d,
                            false
                        );

                        if is_entity_visible {
                            render_state.render(&mut render_pass);
                        }
                        continue;
                    }
                }

                if let Some(sprite) = components.iter().find_map(|component| component.as_any().downcast_ref::<Sprite>()) {
                    render_pass.set_pipeline(render_state.render_pipeline_2d.as_ref().unwrap());
                    render_state.setup(
                        &mut event_dispatcher,
                        Some(&entity),
                        Some(sprite),
                        None,
                        transform,
                        animation,
                        &camera2d,
                        false
                    );

                    if is_entity_visible {
                        render_state.render(&mut render_pass);
                    }
                } else if let Some(shape) = components.iter().find_map(|component| component.as_any().downcast_ref::<Shape>()) {
                    render_pass.set_pipeline(render_state.render_pipeline_2d.as_ref().unwrap());
                    render_state.setup(
                        &mut event_dispatcher,
                        Some(&entity),
                        None,
                        Some(shape),
                        transform,
                        None,
                        &camera2d,
                        false
                    );

                    if is_entity_visible {
                        render_state.render(&mut render_pass);
                    }
                } else if let Some(text_renderer) = text_holder.text_renderers.get(&entity.0) {
                    if is_entity_visible {
                        text_renderer.text_brush.draw(&mut render_pass);
                    }
                }
            }
        }
    }
    render_state.queue.as_ref().unwrap().submit(std::iter::once(command_encoder.finish()));
    surface_texture.present();
}

/// Execute processes related to getting the surface texture (frame) being suboptimal.
pub(crate) fn on_suboptimal(render_state: &mut RenderState, world: &mut World) {
    log::warn!("Surface suboptimal -> Resizing the frame");
    let camera2d: ResourceRef<'_, Camera2d> = world.get_resource::<Camera2d>().unwrap();
    let text_holder: ResourceRef<'_, TextHolder> = world.get_resource::<TextHolder>().unwrap();

    render_state.resize(
        render_state.physical_size.as_ref().unwrap().clone(),
        &camera2d,
        &text_holder.text_renderers
    );
}

fn get_entities_to_render_sorted(render_state: &mut RenderState, world: &World) -> Vec<Entity> {
    let mut entities_to_render_sorted: Vec<Entity> = render_state.entities_to_render.clone();

    if entities_to_render_sorted.len() > 1 {
        entities_to_render_sorted.sort_by(|a, b| {
            DrawOrder::compare(world, a, b)
        });
    }
    return entities_to_render_sorted;
}
