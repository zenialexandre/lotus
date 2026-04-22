use std::{
    any::TypeId,
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
    mem::take,
    sync::Arc
};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use cgmath::Vector2;
use lotus_proc_macros::Component;
use uuid::Uuid;
use crate::core::event::synchronizer;

use super::{
    super::{
        event::{
            dispatcher::EventDispatcher,
        },
        super::Color,
        camera::camera2d::Camera2d,
        input::keyboard_input::KeyboardInput,
        input::mouse_input::MouseInput,
        input::gamepad_input::GamepadInput,
        draw_order::DrawOrder,
        visibility::Visibility,
        text::{text::{Text, TextHolder, TextRenderer}, font::{Font, Fonts}},
        managers::render::manager::RenderState,
        physics::transform::{Transform, Position, Strategy}
    },
    archetype::Archetype,
    query::Query,
    entity::Entity,
    component::{Component, ComponentRefMut, ComponentRef, ComponentBorrowState},
    resource::{Resource, ResourceRefMut, ResourceRef, ResourceBorrowState}
};

/// Struct to represent the FPS entity on the world.
#[derive(Clone, Component)]
struct Fps();

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
        resources.insert(TypeId::of::<KeyboardInput>(), Arc::new(AtomicRefCell::new(Box::new(KeyboardInput::default()))));
        resources.insert(TypeId::of::<MouseInput>(), Arc::new(AtomicRefCell::new(Box::new(MouseInput::default()))));
        resources.insert(TypeId::of::<GamepadInput>(), Arc::new(AtomicRefCell::new(Box::new(GamepadInput::default()))));
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

    /// Synchronize all pending events.
    pub(crate) fn synchronize(&mut self, render_state: &mut RenderState, delta: f32) {
        synchronizer::events(self, render_state);
        synchronizer::camera(self, render_state);
        synchronizer::animations(self, delta);
        synchronizer::collisions(self);
        synchronizer::gravity(self, render_state, delta);
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

    /// Returns an immutable reference to the specified resource.
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

    /// Returns an immutable cloned reference to the specified resource.
    pub fn get_resource_cloned<T: Resource + Clone + 'static>(&self) -> Option<T> {
        return self.get_resource::<T>().map(|resource| resource.clone());
    }

    /// Returns a mutable reference to the specified resource.
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

    /// Returns a specific component from an entity.
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
