use std::{
    any::TypeId,
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
    mem::take,
    sync::Arc
};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use cgmath::{Matrix4, Vector2, Vector3};
use lotus_proc_macros::Component;
use uuid::Uuid;
use super::{
    super::{
        event_dispatcher::{EventDispatcher, EventType, SubEventType},
        animation::Animation,
        texture::sprite_sheet::{AnimationState, LoopingState},
        color::Color,
        camera::camera2d::Camera2d,
        input::Input,
        draw_order::DrawOrder,
        visibility::Visibility,
        text::{text::{Text, TextHolder, TextRenderer}, font::{Font, Fonts}},
        managers::rendering::{cache, manager::RenderState},
        physics::{collision::Collision, velocity::Velocity, transform::{Transform, Position, Strategy}, gravity::Gravity, rigid_body::{RigidBody, BodyType}}
    },
    query::Query,
    entity::Entity,
    component::{Component, ComponentRefMut, ComponentRef, ComponentBorrowState},
    resource::{Resource, ResourceRefMut, ResourceRef, ResourceBorrowState}
};

/// Struct to represent the FPS entity on the world.
#[derive(Clone, Component)]
struct Fps();

/// Struct to represent the different archetypes and/or clusters of data.
pub struct Archetype {
    pub entities: Vec<Entity>,
    pub components: HashMap<TypeId, Vec<AtomicRefCell<Box<dyn Component>>>>
}

impl Archetype {
    /// Create a new archetype.
    pub fn new() -> Self {
        return Self {
            entities: Vec::new(),
            components: HashMap::new()
        };
    }

    /// Add a new entity to a archetype by its components.
    pub fn add_entity(&mut self, entity: Entity, components: Vec<AtomicRefCell<Box<dyn Component>>>) {
        self.entities.push(entity);

        for component in components {
            let type_id: TypeId = component.borrow().type_id();
            self.components.entry(type_id)
                .or_insert_with(Vec::new)
                .push(component);
        }
    }
}

/// Struct to represent the World of the Entity-Component-System architecture.
///
/// The World uses normalized coordinates at its core.
pub struct World {
    pub archetypes: HashMap<u64, Archetype>,
    pub resources: HashMap<TypeId, Arc<AtomicRefCell<Box<dyn Resource>>>>,
    pub resource_borrow_state: AtomicRefCell<ResourceBorrowState>,
    pub component_borrow_state: AtomicRefCell<ComponentBorrowState>
}

impl World {
    /// Create a new world.
    pub fn new() -> Self {
        let mut resources: HashMap<TypeId, Arc<AtomicRefCell<Box<dyn Resource>>>> = HashMap::new();
        resources.insert(TypeId::of::<EventDispatcher>(), Arc::new(AtomicRefCell::new(Box::new(EventDispatcher::new()))));
        resources.insert(TypeId::of::<Input>(), Arc::new(AtomicRefCell::new(Box::new(Input::default()))));
        resources.insert(TypeId::of::<Camera2d>(), Arc::new(AtomicRefCell::new(Box::new(Camera2d::default()))));
        resources.insert(TypeId::of::<TextHolder>(), Arc::new(AtomicRefCell::new(Box::new(TextHolder::default()))));

        return Self {
            archetypes: HashMap::new(),
            resources,
            resource_borrow_state: ResourceBorrowState::new().into(),
            component_borrow_state: ComponentBorrowState::new().into()
        };
    }

    /// Add a new resource to the world.
    pub fn add_resource(&mut self, resource: Box<dyn Resource>) {
        self.resources.insert(
            resource.type_id(),
            Arc::new(AtomicRefCell::new(resource))
        );
    }

    /// Add a list of resources to the world.
    pub fn add_resources(&mut self, resources: Vec<Box<dyn Resource>>) {
        for resource in resources {
            self.resources.insert(
                resource.type_id(),
                Arc::new(AtomicRefCell::new(resource))
            );
        }
    }

    /// Create a text showing the current application FPS.
    pub fn show_fps(&mut self, render_state: &mut RenderState, current_fps: u32, color: Color) {
        let mut query: Query = Query::new(&self).with::<Fps>();

        if let Some(fps_entity) = query.entities_with_components().unwrap().first() {
            if let Some(mut text_holder) = self.get_resource_mut::<TextHolder>() {
                if let Some(text_renderer) = text_holder.text_renderers.get_mut(&fps_entity.0) {
                    text_renderer.content(current_fps.to_string(), render_state.queue.clone(), render_state.physical_size);
                }
            }
        } else {
            let fps_text: Text = Text::new(
                render_state,
                Font::new(Fonts::RobotoMono.get_path(), 21.0),
                Position::new(Vector2::new(0.0, 0.0), Strategy::Pixelated),
                color,
                current_fps.to_string()
            );
            self.spawn(render_state, vec![Box::new(fps_text), Box::new(DrawOrder(99999)), Box::new(Fps())]);
        }
    }

    /// Spawn a new entity on the world.
    ///
    /// The entity can be rendered on the fly, if its a shape or a sprite.
    pub(crate) fn spawn(&mut self, render_state: &mut RenderState, components: Vec<Box<dyn Component>>) -> Entity {
        let entity: Entity = Entity(Uuid::new_v4());

        if components.iter().any(|component| component.as_any().is::<Text>()) {
            let text: &Text = components.iter()
                .find_map(|component| component.as_any().downcast_ref::<Text>()
            ).unwrap();
            let mut text_holder: ResourceRefMut<'_, TextHolder> = self.get_resource_mut::<TextHolder>().unwrap();

            let text_renderer: TextRenderer = TextRenderer::new(render_state, &text);
            text_holder.text_renderers.insert(entity.0, text_renderer);
        }

        let mut components_refs: Vec<AtomicRefCell<Box<dyn Component>>> = Vec::with_capacity(components.len());
        for component in components {
            components_refs.push(AtomicRefCell::new(component));
        }

        if !components_refs.iter().any(|component| component.borrow().as_any().is::<Transform>()) {
            components_refs.push(AtomicRefCell::new(Box::new(Transform::default())));
        }

        if !components_refs.iter().any(|component| component.borrow().as_any().is::<Visibility>()) {
            components_refs.push(AtomicRefCell::new(Box::new(Visibility::default())));
        }

        if !components_refs.iter().any(|component| component.borrow().as_any().is::<DrawOrder>()) {
            components_refs.push(AtomicRefCell::new(Box::new(DrawOrder::default())));
        }

        let mut components_types_ids: Vec<TypeId> = components_refs.iter().map(|c| c.borrow().type_id()).collect();
        let archetype_unique_key: u64 = self.get_archetype_unique_key(&mut components_types_ids);
        let archetype: &mut Archetype = self.archetypes.entry(archetype_unique_key).or_insert_with(Archetype::new);

        let moved_components: Vec<AtomicRefCell<Box<dyn Component>>> = take(&mut components_refs);
        archetype.add_entity(entity, moved_components);
        render_state.add_entity_to_render(entity);

        return entity;
    }

    /// Despawn a specific entity from the world.
    ///
    /// The entity is removed from the rendering flow and its related cached data is cleaned.
    pub(crate) fn despawn(&mut self, render_state: &mut RenderState, entity: &Entity) {
        render_state.remove_entity_to_render(entity);
        render_state.clean_entity_buffer_cache(entity);
        render_state.clean_entity_bind_group_cache(entity);

        for archetype in self.archetypes.values_mut() {
            if let Some(index) = archetype.entities.iter().position(|e| e.0 == entity.0) {
                for components in archetype.components.values_mut() {
                    if index < components.len() {
                        components.remove(index);
                    }
                }
                archetype.entities.remove(index);
                break;
            }
        }
    }

    /// Synchronizes events that were dispatched in another process.
    pub(crate) fn synchronize_events(&mut self, render_state: &RenderState) {
        let mut event_dispatcher: ResourceRefMut<'_, EventDispatcher> = self.get_resource_mut::<EventDispatcher>().unwrap();
        let events: Vec<_> = event_dispatcher.drain().into();

        for event in events {
            match &event.event_type {
                EventType::Transform(sub_event_type) => {
                    let mut transform: ComponentRefMut<'_, Transform> = self.get_entity_component_mut::<Transform>(&event.entity).unwrap();

                    if let Some(value) = event.get::<Vector2<f32>>() {
                        if sub_event_type == &SubEventType::UpdatePixelatedPosition {
                            transform.position.x = value.x;
                            transform.position.y = value.y;
                        } else {
                            transform.scale.x = value.x;
                            transform.scale.y = value.y;
                        }
                        transform.dirty_position = false;
                    }
                },
                EventType::Text(sub_event_type) => {
                    let mut text_holder: ResourceRefMut<'_, TextHolder> = self.get_resource_mut::<TextHolder>().unwrap();

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
                }
            }
        }
    }

    /// Synchronizes the camera with its target.
    pub fn synchronize_camera_with_target(&mut self, render_state: &mut RenderState) {
        let (target_entity, target_position) = {
            let camera2d: ResourceRefMut<'_, Camera2d> = self.get_resource_mut::<Camera2d>().unwrap();

            if let Some(entity) = camera2d.target {
                if let Some(transform) = self.get_entity_component::<Transform>(&entity) {
                    (Some(entity), Some(transform.position.clone()))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            }
        };

        if let (Some(entity), Some(position)) = (target_entity, target_position) {
            let mut camera2d: ResourceRefMut<'_, Camera2d> = self.get_resource_mut::<Camera2d>().unwrap();
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
    pub fn synchronize_animations_of_entities(&mut self, delta: f32) {
        let mut query: Query<'_> = Query::new(&self).with::<Animation>();

        for entity in query.entities_with_components().unwrap() {
            if let Some(mut animation) = self.get_entity_component_mut::<Animation>(&entity) {
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
    pub fn synchronize_gravity_with_dynamic_bodies(&mut self, render_state: &mut RenderState, delta: f32) {
        let mut query: Query<'_> = Query::new(self).with::<Gravity>()
            .with::<Transform>()
            .with::<Velocity>()
            .with::<RigidBody>();

        for entity in query.entities_with_components().unwrap() {
            if let (Some(gravity), Some(mut transform), Some(mut velocity), Some(rigid_body)) = (
                self.get_entity_component::<Gravity>(&entity),
                self.get_entity_component_mut::<Transform>(&entity),
                self.get_entity_component_mut::<Velocity>(&entity),
                self.get_entity_component::<RigidBody>(&entity)
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
    pub fn synchronize_transformations_with_collisions(&mut self) {
        for archetype in self.archetypes.values_mut() {
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

    /// Returns the unique key of a archetype.
    pub fn get_archetype_unique_key(&self, components_types_ids: &mut Vec<TypeId>) -> u64 {
        components_types_ids.sort();
        return self.get_hash_from_ids(&components_types_ids);
    }

    /// Returns a hash based on the components ids that make up the archetype.
    pub fn get_hash_from_ids(&self, type_ids: &[TypeId]) -> u64 {
        let mut default_hasher: DefaultHasher = DefaultHasher::new();

        for type_id in type_ids {
            type_id.hash(&mut default_hasher);
        }
        return default_hasher.finish();
    }

    /// Return an immutable reference to the specified resource.
    pub fn get_resource<T: Resource + 'static>(&self) -> Option<ResourceRef<'_, T>> {
        let type_id: TypeId = TypeId::of::<T>();
        let mut resource_borrow_state: AtomicRefMut<'_, ResourceBorrowState> = self.resource_borrow_state.borrow_mut();

        if resource_borrow_state.try_borrow_immutable(type_id) {
            if let Some(resource) = self.resources.get(&type_id) {
                return Some(ResourceRef {
                    inner: resource.borrow(),
                    type_id,
                    resource_borrow_state: &self.resource_borrow_state,
                    phantom_data: PhantomData
                });
            } else {
                resource_borrow_state.release_immutable(type_id);
                return None;
            }
        } else {
            return None;
        }
    }

    /// Return a mutable reference to the specified resource.
    pub fn get_resource_mut<T: Resource + 'static>(&self) -> Option<ResourceRefMut<'_, T>> {
        let type_id: TypeId = TypeId::of::<T>();
        let mut resource_borrow_state: AtomicRefMut<'_, ResourceBorrowState> = self.resource_borrow_state.borrow_mut();

        if resource_borrow_state.try_borrow_mutable(type_id) {
            if let Some(resource) = self.resources.get(&type_id) {
                return Some(
                    ResourceRefMut {
                        inner: resource.borrow_mut(),
                        type_id,
                        resource_borrow_state: &self.resource_borrow_state,
                        phantom_data: PhantomData
                    }  
                );
            } else {
                resource_borrow_state.release_mutable(type_id);
                return None;
            }
        } else {
            return None;
        }
    }

    /// Return a specific component from an entity.
    pub fn get_entity_component<T: Component + 'static>(&self, entity: &Entity) -> Option<ComponentRef<'_, T>> {
        for archetype in self.archetypes.values() {
            if archetype.entities.contains(entity) {
                let type_id: TypeId = TypeId::of::<T>();
                let mut component_borrow_state: AtomicRefMut<'_, ComponentBorrowState> = self.component_borrow_state.borrow_mut();

                if component_borrow_state.try_borrow_immutable(type_id, entity.0) {
                    if let Some(components) = archetype.components.get(&type_id) {
                        if let Some(index) = archetype.entities.iter().position(|e| e == entity) {
                            return Some(ComponentRef {
                                inner: components[index].borrow(),
                                type_id,
                                entity_id: entity.0,
                                component_borrow_state: &self.component_borrow_state,
                                phantom_data: std::marker::PhantomData
                            });
                        } else {
                            component_borrow_state.release_immutable(type_id, entity.0);
                            return None;
                        }
                    }
                } else {
                    return None;
                }
            }
        }
        return None;
    }

    /// Return a specific component from an entity as mutable.
    pub fn get_entity_component_mut<T: Component + 'static>(&self, entity: &Entity) -> Option<ComponentRefMut<'_, T>> {
        for archetype in self.archetypes.values() {
            if archetype.entities.contains(entity) {
                let type_id: TypeId = TypeId::of::<T>();
                let mut component_borrow_state: AtomicRefMut<'_, ComponentBorrowState> = self.component_borrow_state.borrow_mut();

                if component_borrow_state.try_borrow_mutable(type_id, entity.0) {
                    if let Some(components) = archetype.components.get(&type_id) {
                        if let Some(index) = archetype.entities.iter().position(|e| e == entity) {
                            return Some(ComponentRefMut {
                                inner: components[index].borrow_mut(),
                                type_id,
                                entity_id: entity.0,
                                component_borrow_state: &self.component_borrow_state,
                                phantom_data: std::marker::PhantomData
                            });
                        } else {
                            component_borrow_state.release_mutable(type_id, entity.0);
                            return None;
                        }
                    }
                } else {
                    return None;
                }
            }
        }
        return None;
    }

    /// Return all the components from a specific entity.
    pub fn get_entity_components<'a>(&'a self, entity: &'a Entity) -> Option<Vec<AtomicRef<'a, Box<dyn Component>>>> {
        if let Some((_, archetype)) = self.archetypes.iter().find(|(_, arch)| arch.entities.contains(entity)) {
            if let Some(index) = archetype.entities.iter().position(|e| e == entity) {
                let mut entity_components: Vec<AtomicRef<'a, Box<dyn Component>>> = Vec::new();
    
                for component_vec in archetype.components.values() {
                    if let Some(component) = component_vec.get(index) {
                        let borrowed: AtomicRef<'_, Box<dyn Component>> = component.borrow();
                        entity_components.push(borrowed);
                    }
                }
                return Some(entity_components);
            }
        }
        return None;
    }

    /// Return all the components from a specific entity as mutables.
    pub fn get_entity_components_mut<'a>(&'a self, entity: &'a Entity) -> Option<Vec<AtomicRefMut<'a, Box<dyn Component>>>> {
        if let Some((_, archetype)) = self.archetypes.iter().find(|(_, arch)| arch.entities.contains(entity)) {
            if let Some(index) = archetype.entities.iter().position(|e| e == entity) {
                let mut entity_components: Vec<AtomicRefMut<'a, Box<dyn Component>>> = Vec::new();
    
                for component_vec in archetype.components.values() {
                    if let Some(component) = component_vec.get(index) {
                        let borrowed: AtomicRefMut<'_, Box<dyn Component>> = component.borrow_mut();
                        entity_components.push(borrowed);
                    }
                }
                return Some(entity_components);
            }
        }
        return None;
    }

    /// Returns if an entity still is in the world.
    pub fn is_entity_alive(&self, entity: Entity) -> bool {
        return self.archetypes.values().any(|archetype| archetype.entities.iter().any(|e| e.0 == entity.0));    
    }

    /// Returns if an entity is visible.
    pub fn is_entity_visible(&self, entity: Entity) -> bool {
        let visibility: ComponentRef<'_, Visibility> = self.get_entity_component::<Visibility>(&entity).unwrap();
        return visibility.0;
    }
}

/// Struct to represent the mutable commands to be made on the world.
pub struct Commands {
    pub commands: Vec<Command>
}

impl Commands {
    /// Create a new command struct.
    pub fn new() -> Self {
        return Self {
            commands: Vec::new()
        };
    }

    /// Spawn a new entity on the world.
    ///
    /// The entity will be rendered as its type demands.
    pub fn spawn(&mut self, components: Vec<Box<dyn Component>>) {
        self.commands.push(Command::Spawn(components));
    }

    /// Despawn a specific entity from the world.
    ///
    /// The entity is removed from the rendering flow and its related cached data is cleaned.
    pub fn despawn(&mut self, entity: Entity) {
        self.commands.push(Command::Despawn(entity));
    }

    /// Add a new resource to the world.
    pub fn add_resource(&mut self, resource: Box<dyn Resource>) {
        self.commands.push(Command::AddResource(resource));
    }

    /// Add a list of resources to the world.
    pub fn add_resources(&mut self, resources: Vec<Box<dyn Resource>>) {
        self.commands.push(Command::AddResources(resources));
    }

    /// Show the current FPS value.
    pub fn show_fps(&mut self, current_fps: u32, color: Color) {
        self.commands.push(Command::ShowFps(current_fps, color));
    }

    /// Take the commands memory reference.
    pub(crate) fn _take_commands(&mut self) -> Vec<Command> {
        return std::mem::take(&mut self.commands);
    }

    /// Flush the commands inside the buffer.
    pub fn flush_commands(&mut self, world: &mut World, render_state: &mut RenderState) {
        for command in self.commands.drain(..) {
            match command {
                Command::Spawn(components) => {
                    world.spawn(render_state, components);
                },
                Command::Despawn(entity) => {
                    if world.is_entity_alive(entity) {
                        world.despawn(render_state, &entity);
                    }
                },
                Command::AddResource(resource) => {
                    world.add_resource(resource);   
                },
                Command::AddResources(resources) => {
                    world.add_resources(resources);
                },
                Command::ShowFps(current_fps, color) => {
                    world.show_fps(render_state, current_fps, color);
                }
            }
        }
    }
}

/// Enumerator that store the mutable commands allowed in the world.
pub enum Command {
    Spawn(Vec<Box<dyn Component>>),
    Despawn(Entity),
    AddResource(Box<dyn Resource>),
    AddResources(Vec<Box<dyn Resource>>),
    ShowFps(u32, Color)
}
