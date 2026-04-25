use std::any::TypeId;
use atomic_refcell::{AtomicRef, AtomicRefMut};
use cgmath::{Matrix4, Vector2, Vector3};
use gilrs::{Axis, Button, GamepadId};
use super::{
    dispatcher::{EventDispatcher, EventType, SubEventType},
    super::{
        ecs::{resource::ResourceRefMut, component::{ComponentRefMut, Component}, world:: World, query::Query},
        physics::{transform::{Transform, Position}, collision::Collision, velocity::Velocity, gravity::Gravity, rigid_body::{RigidBody, BodyType}},
        managers::render::{manager::RenderState, cache},
        text::{text::TextHolder, font::Font},
        color::color::Color,
        camera::camera2d::Camera2d,
        animation::Animation,
        texture::sprite_sheet::{AnimationState, LoopingState},
        input::gamepad_input::{GamepadInput, GamepadInstance}
    }
};

/// Synchronizes events that were dispatched in another process.
pub(crate) fn events(world: &mut World, render_state: &RenderState) {
    let mut event_dispatcher: ResourceRefMut<'_, EventDispatcher> = world.get_resource_mut::<EventDispatcher>().unwrap();
    let events: Vec<_> = event_dispatcher.drain().into();

    for event in events {
        match &event.event_type {
            EventType::Transform(sub_event_type) => {
                let mut transform: ComponentRefMut<'_, Transform> = world.get_entity_component_mut::<Transform>(&event.entity).unwrap();

                if let Some(value) = event.get::<Vector2<f32>>() {
                    if sub_event_type == &SubEventType::UpdatePixelatedPosition {
                        transform.position.x = value.x;
                        transform.position.y = value.y;
                        transform.dirty_position = false;
                    } else {
                        transform.scale.x = value.x;
                        transform.scale.y = value.y;
                        transform.dirty_scale = false;
                    }
                }
            },
            EventType::Text(sub_event_type) => {
                let mut text_holder: ResourceRefMut<'_, TextHolder> = world.get_resource_mut::<TextHolder>().unwrap();

                if let Some(text_renderer) = text_holder.text_renderers.get_mut(&event.entity.0) {
                    match sub_event_type {
                        SubEventType::UpdateTextFont => {
                            if let Some(font) = event.get::<Font>() {
                                text_renderer.font(
                                    font.clone(),
                                    render_state.queue.clone(),
                                    render_state.physical_size.clone()
                                );
                            }
                        },
                        SubEventType::UpdateTextPosition => {
                            if let Some(position) = event.get::<Position>() {
                                text_renderer.position(
                                    position.clone(),
                                    render_state.queue.clone(),
                                    render_state.physical_size.clone()
                                );
                            }
                        },
                        SubEventType::UpdateTextContent => {
                            if let Some(content) = event.get::<String>() {
                                text_renderer.content(
                                    content.clone(),
                                    render_state.queue.clone(),
                                    render_state.physical_size.clone()
                                );
                            }
                        },
                        SubEventType::UpdateTextColor => {
                            if let Some(color) = event.get::<Color>() {
                                text_renderer.color(
                                    color.clone(),
                                    render_state.queue.clone(),
                                    render_state.physical_size.clone()
                                );
                            }
                        },
                        _ => {}
                    }
                }                    
            },
            EventType::Gamepad(sub_event_type) => {
                let mut gamepad_input: ResourceRefMut<'_, GamepadInput> = world.get_resource_mut::<GamepadInput>().unwrap();

                match sub_event_type {
                    SubEventType::GamepadConnected => {
                        let id: &GamepadId = event.get::<GamepadId>().unwrap();

                        if gamepad_input.instances.get_mut(id).is_some() {
                            gamepad_input.instances.get_mut(id).unwrap().connect();
                        } else {
                            gamepad_input.instances.insert(id.clone(), GamepadInstance::default());
                        }
                    },
                    SubEventType::GamepadDisconnected => {
                        gamepad_input.instances.get_mut(event.get::<GamepadId>().unwrap()).map(|instance| instance.disconnect());
                    },
                    SubEventType::GamepadButtonPressed => {
                        let (id, button): &(GamepadId, Button) = event.get::<(GamepadId, Button)>().unwrap();
                        gamepad_input.instances.get_mut(id).map(|instance| instance.pressed.insert(button.clone()));
                    },
                    SubEventType::GamepadButtonReleased => {
                        let (id, button): &(GamepadId, Button) = event.get::<(GamepadId, Button)>().unwrap();
                        gamepad_input.instances.get_mut(id).map(|instance| instance.pressed.remove(button));
                    },
                    SubEventType::GamepadAxisChanged => {
                        let (id, axis, direction): &(GamepadId, Axis, f32) = event.get::<(GamepadId, Axis, f32)>().unwrap();
                        gamepad_input.instances.get_mut(id).map(|instance| instance.joystick_actions.entry(*axis).or_insert(*direction));
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Synchronizes the camera with its target.
pub(crate) fn camera(world: &mut World, render_state: &mut RenderState) {
    let (target_entity, target_position) = {
        let camera2d: ResourceRefMut<'_, Camera2d> = world.get_resource_mut::<Camera2d>().unwrap();

        if let Some(entity) = camera2d.target {
            if let Some(transform) = world.get_entity_component::<Transform>(&entity) {
                (Some(entity), Some(transform.position.clone()))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        }
    };

    if let (Some(entity), Some(position)) = (target_entity, target_position) {
        let mut camera2d: ResourceRefMut<'_, Camera2d> = world.get_resource_mut::<Camera2d>().unwrap();
        camera2d.transform.position = position.clone();
        camera2d.view_matrix = Matrix4::from_translation(Vector3::new(
            -position.x.clone(),
            -position.y,
            0.0
        ));
        let _ = cache::buffer::get_projection_or_view_buffer(
            render_state,
            true,
            Some(&entity),
            &camera2d
        );
        let _ = cache::buffer::get_projection_or_view_buffer(
            render_state,
            false,
            Some(&entity),
            &camera2d
        );
    }
}

/// Synchronizes the animation of entities sprite sheets.
pub(crate) fn animations(world: &mut World, delta: f32) {
    let mut query: Query<'_> = Query::new(&world).with::<Animation>();

    for entity in query.entities_with_components().unwrap() {
        if let Some(mut animation) = world.get_entity_component_mut::<Animation>(&entity) {
            let mut to_remove_from_stack: Vec<String> = Vec::new();

            for (title, sprite_sheet) in animation.sprite_sheets.iter_mut() {
                if sprite_sheet.animation_state != AnimationState::Playing {
                    continue;
                }
                sprite_sheet.timer.tick(delta);

                if sprite_sheet.timer.is_finished() {
                    sprite_sheet.current_index = (sprite_sheet.current_index + 1) % sprite_sheet.indices.len() as u32;

                    if &sprite_sheet.current_index == sprite_sheet.indices.last().unwrap() && sprite_sheet.looping_state != LoopingState::Repeat {
                        sprite_sheet.animation_state = AnimationState::Finished;
                        to_remove_from_stack.push(title.clone());
                    }
                }
            }

            for title in to_remove_from_stack {
                animation.playing_stack.retain(|t| *t != title);
            }
        }
    }
}

/// Synchronizes the gravity with entities that are considered dynamic bodies.
pub(crate) fn gravity(world: &mut World, render_state: &mut RenderState, delta: f32) {
    let mut query: Query<'_> = Query::new(world).with::<Gravity>()
        .with::<Transform>()
        .with::<Velocity>()
        .with::<RigidBody>();

    for entity in query.entities_with_components().unwrap() {
        if let (Some(gravity), Some(mut transform), Some(mut velocity), Some(rigid_body)) = (
            world.get_entity_component::<Gravity>(&entity),
            world.get_entity_component_mut::<Transform>(&entity),
            world.get_entity_component_mut::<Velocity>(&entity),
            world.get_entity_component::<RigidBody>(&entity)
        ) {
            if rigid_body.body_type == BodyType::Dynamic && rigid_body.rest == false {
                velocity.y -= gravity.value * rigid_body.friction * delta;
                let new_y: f32 = transform.position.y + velocity.y * delta;
                transform.set_position_y(render_state, new_y);
            }
        }
    }
}

/// Synchronizes the transformation matrices with the collision objects.
pub(crate) fn collisions(world: &mut World) {
    for archetype in world.archetypes.values_mut() {
        if let (Some(transforms), Some(collisions)) = (
            archetype.components.get(&TypeId::of::<Transform>()),
            archetype.components.get(&TypeId::of::<Collision>())
        ) {
            for (transform, collision) in transforms.iter().zip(collisions) {
                let transform_ref: AtomicRef<'_, Box<dyn Component>> = transform.borrow();
                let mut collision_ref: AtomicRefMut<'_, Box<dyn Component>> = collision.borrow_mut();

                if let (Some(transform), Some(collision)) = (
                    transform_ref.as_any().downcast_ref::<Transform>(),
                    collision_ref.as_any_mut().downcast_mut::<Collision>()
                ) {
                    collision.collider.position = transform.position.to_vec();
                    collision.collider.scale = transform.scale;
                }
            }
        }
    }
}
