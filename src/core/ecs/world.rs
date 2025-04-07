use std::{
    any::TypeId,
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
    mem::take,
    sync::Arc
};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use cgmath::{Matrix4, Vector3};
use uuid::Uuid;

use super::{
    super::{
        camera::camera2d::Camera2d,
        input::Input,
        text::{Text, TextRenderer},
        managers::rendering_manager::RenderState,
        physics::{collision::Collision, transform::Transform}
    },
    entity::Entity,
    component::{Component, ComponentRefMut, ComponentRef, ComponentBorrowState},
    resource::{Resource, ResourceRefMut, ResourceRef, ResourceBorrowState}
};

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
        resources.insert(TypeId::of::<Input>(), Arc::new(AtomicRefCell::new(Box::new(Input::default()))));
        resources.insert(TypeId::of::<Camera2d>(), Arc::new(AtomicRefCell::new(Box::new(Camera2d::default()))));

        return Self {
            archetypes: HashMap::new(),
            resources,
            resource_borrow_state: ResourceBorrowState::new().into(),
            component_borrow_state: ComponentBorrowState::new().into()
        };
    }

    /// Add a new resource to the world.
    pub fn add_resource<T: Resource + 'static>(&mut self, resource: T) {
        self.resources.insert(
            TypeId::of::<T>(),
            Arc::new(AtomicRefCell::new(Box::new(resource)))
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

    /// # Spawn a new entity on the world.
    /// The entity can be rendered on the fly, if its a shape or a sprite.
    pub(crate) fn spawn(&mut self, render_state: &mut RenderState, components: Vec<Box<dyn Component>>) -> Entity {
        let entity: Entity = Entity(Uuid::new_v4());

        if components.iter().any(|component| component.as_any().is::<Text>()) {
            let text: &Text = components.iter()
                .find_map(|component| component.as_any().downcast_ref::<Text>()
            ).unwrap();

            let text_renderer: TextRenderer = TextRenderer::new(render_state, &text);
            render_state.text_renderers.insert(entity.0, text_renderer);
        }

        let mut components_refs: Vec<AtomicRefCell<Box<dyn Component>>> = Vec::with_capacity(components.len());
        for component in components {
            components_refs.push(AtomicRefCell::new(component));
        }

        if !components_refs.iter().any(|component| component.borrow().as_any().is::<Transform>()) {
            components_refs.push(AtomicRefCell::new(Box::new(Transform::default())));
        }

        let mut components_types_ids: Vec<TypeId> = components_refs.iter().map(|c| c.borrow().type_id()).collect();
        let archetype_unique_key: u64 = self.get_archetype_unique_key(&mut components_types_ids);
        let archetype: &mut Archetype = self.archetypes.entry(archetype_unique_key).or_insert_with(Archetype::new);

        let moved_components: Vec<AtomicRefCell<Box<dyn Component>>> = take(&mut components_refs);
        archetype.add_entity(entity, moved_components);
        render_state.add_entity_to_render(entity);

        return entity;
    }

    /// # Despawn a specific entity from the world.
    /// The entity can be removed from the rendering flow on the fly, if its necessary. 
    pub(crate) fn despawn(&mut self, render_state: &mut RenderState, entity: &Entity) {
        render_state.remove_entity_to_render(entity);

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

    /// Synchronizes the transformation matrices with the collision objects.
    pub fn synchronize_transformations_with_collisions(&mut self) {
        for archetype in self.archetypes.values_mut() {
            if let (Some(transforms), Some(collisions)) = (
                archetype.components.get(&TypeId::of::<Transform>()),
                archetype.components.get(&TypeId::of::<Collision>())
            ) {
                for (transform, collision) in transforms.iter().zip(collisions) {
                    let transform: AtomicRef<'_, Box<dyn Component>> = transform.borrow();
                    let mut collision: AtomicRefMut<'_, Box<dyn Component>> = collision.borrow_mut();

                    if let Some(collision) = collision.as_any_mut().downcast_mut::<Collision>() {
                        let transform: &Transform = transform.as_any().downcast_ref::<Transform>().unwrap();
                        collision.collider.position = transform.get_position();
                        collision.collider.scale = transform.get_scale();
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
                    (Some(entity), Some(transform.position))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            }
        };

        if let (Some(_), Some(position)) = (target_entity, target_position) {
            let mut camera2d: ResourceRefMut<'_, Camera2d> = self.get_resource_mut::<Camera2d>().unwrap();
            camera2d.transform.position = position;
            camera2d.view_matrix = Matrix4::from_translation(Vector3::new(
                -position.x,
                -position.y,
                0.0
            ));
            let view_matrix_unwrapped: [[f32; 4]; 4] = *camera2d.view_matrix.as_ref();
            let projection_matrix: Matrix4<f32> = render_state.get_projection_matrix(&camera2d);
            let projection_matrix_unwrapped: [[f32; 4]; 4] = *projection_matrix.as_ref();
    
            if let Some(projection_buffer) = render_state.projection_buffer.as_ref() {
                render_state.queue.as_mut().unwrap().write_buffer(
                    projection_buffer,
                    0,
                    bytemuck::cast_slice(&[projection_matrix_unwrapped])
                );
            }

            if let Some(view_buffer) = render_state.view_buffer.as_ref() {
                render_state.queue.as_mut().unwrap().write_buffer(
                    view_buffer,
                    0,
                    bytemuck::cast_slice(&[view_matrix_unwrapped])
                );
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

    /// Return an immutable reference to the specified resource
    pub fn get_resource<T: Resource + 'static>(&self) -> Option<ResourceRef<T>> {
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

    /// Return a mutable reference to the specified resource
    pub fn get_resource_mut<T: Resource + 'static>(&self) -> Option<ResourceRefMut<T>> {
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

    /// # Spawn a new entity on the world.
    /// The entity can be rendered on the fly, if its a shape or a sprite.
    pub fn spawn(&mut self, components: Vec<Box<dyn Component>>) {
        self.commands.push(Command::Spawn(components));
    }

    /// # Despawn a specific entity from the world.
    /// The entity can be removed from the rendering flow on the fly, if its necessary. 
    pub fn despawn(&mut self, entity: Entity) {
        self.commands.push(Command::Despawn(entity));
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
                }
            }
        }
    }
}

/// Enumerator that store the mutable commands allowed in the world.
pub enum Command {
    Spawn(Vec<Box<dyn Component>>),
    Despawn(Entity)
}
