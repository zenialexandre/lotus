use std::{
    any::TypeId,
    cell::{
        Ref,
        RefCell,
        RefMut
    },
    collections::HashMap,
    hash::{
        DefaultHasher,
        Hash,
        Hasher
    },
    mem::take
};
use uuid::Uuid;

use super::{
    super::{
        physics::collision::Collision,
        input::Input,
        managers::rendering_manager::RenderState,
        physics::transform::Transform
    },
    component::Component,
    entitiy::Entity,
    resource::Resource
};

/// Struct to represent the different archetypes and/or clusters of data.
pub struct Archetype {
    pub entities: Vec<Entity>,
    pub components: HashMap<TypeId, Vec<RefCell<Box<dyn Component>>>>
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
    pub fn add_entity(&mut self, entity: Entity, components: Vec<RefCell<Box<dyn Component>>>) {
        self.entities.push(entity);

        for component in components {
            let type_id: TypeId = component.borrow().type_id();
            self.components.entry(type_id)
                .or_insert_with(Vec::new)
                .push(component);
        }
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
    pub(crate) fn flush_commands(&mut self, world: &mut World, render_state: &mut RenderState) {
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

/// Struct to represent the World of the Entity-Component-System architecture.
pub struct World {
    pub archetypes: HashMap<u64, Archetype>,
    pub resources: Vec<RefCell<Box<dyn Resource>>>
}

impl World {
    /// Create a new world.
    pub fn new() -> Self {
        return Self {
            archetypes: HashMap::new(),
            resources: vec![RefCell::new(Box::new(Input::default()))]
        };
    }

    /// Add a new resource to the world.
    pub fn add_resource(&mut self, resource: Box<dyn Resource>) {
        self.resources.push(RefCell::new(resource));
    }

    /// Add a list of resources to the world.
    pub fn add_resources(&mut self, resources: Vec<Box<dyn Resource>>) {
        for resource in resources {
            self.resources.push(RefCell::new(resource));
        }
    }

    /// # Spawn a new entity on the world.
    /// The entity can be rendered on the fly, if its a shape or a sprite.
    pub(crate) fn spawn(&mut self, render_state: &mut RenderState, components: Vec<Box<dyn Component>>) -> Entity {
        let entity: Entity = Entity(Uuid::new_v4());

        let mut components_refs: Vec<RefCell<Box<dyn Component>>> = Vec::with_capacity(components.len());
        for component in components {
            components_refs.push(RefCell::new(component));
        }

        let has_transform: bool = components_refs.iter().any(|c| c.borrow().as_any().is::<Transform>());
        if !has_transform {
            components_refs.push(RefCell::new(Box::new(Transform::default())));
        }
        let mut components_types_ids: Vec<TypeId> = components_refs.iter().map(|c| c.borrow().type_id()).collect();
        let archetype_unique_key: u64 = self.get_archetype_unique_key(&mut components_types_ids);
        let archetype: &mut Archetype = self.archetypes.entry(archetype_unique_key).or_insert_with(Archetype::new);

        let moved_components: Vec<RefCell<Box<dyn Component>>> = take(&mut components_refs);
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

    /// Synchronize the transformation matrices with the collision objects.
    pub fn synchronize_transformations_with_collisions(&mut self) {
        for archetype in self.archetypes.values_mut() {
            if let (Some(transforms), Some(collisions)) = (
                archetype.components.get(&TypeId::of::<Transform>()),
                archetype.components.get(&TypeId::of::<Collision>())
            ) {
                for (transform, collision) in transforms.iter().zip(collisions) {
                    let transform: Ref<'_, Box<dyn Component>> = transform.borrow();
                    let mut collision: RefMut<'_, Box<dyn Component>> = collision.borrow_mut();

                    if let Some(collision) = collision.as_any_mut().downcast_mut::<Collision>() {
                        let transform: &Transform = transform.as_any().downcast_ref::<Transform>().unwrap();
                        collision.collider.position = transform.get_position();
                        collision.collider.scale = transform.get_scale();
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

    /// Return the specified resource.
    pub fn get_resource<T: Resource + 'static>(&self) -> Option<Ref<'_, T>> {
        for resource in &self.resources {
            if resource.borrow().as_any().is::<T>() {
                return Some(Ref::map(
                    resource.borrow(),
                    |resource| resource.as_any().downcast_ref::<T>().unwrap()
                ));
            }
        }
        return None;
    }

    /// Return the specified resource as mutable.
    pub fn get_resource_mut<T: Resource + 'static>(&self) -> Option<RefMut<'_, T>> {
        for resource in &self.resources {
            if resource.borrow().as_any().is::<T>() {
                return Some(RefMut::map(
                    resource.borrow_mut(),
                    |resource| resource.as_any_mut().downcast_mut::<T>().unwrap()
                ));
            }
        }
        return None;
    }

    /// Return a specific component from a entity.
    pub fn get_entity_component<T: Component + 'static>(&self, entity: &Entity) -> Option<Ref<'_, T>> {
        for archetype in self.archetypes.values() {
            if archetype.entities.contains(entity) {
                let type_id: TypeId = TypeId::of::<T>();

                if let Some(components) = archetype.components.get(&type_id) {
                    if let Some(index) = archetype.entities.iter().position(|e| e == entity) {
                        return Some(Ref::map(
                            components[index].borrow(),
                            |component| component.as_any().downcast_ref::<T>().unwrap()
                        ));
                    }
                }
            }
        }
        return None;
    }

    /// Return a specific component from a entity as mutable.
    pub fn get_entity_component_mut<T: Component + 'static>(&self, entity: &Entity) -> Option<RefMut<'_, T>> {
        for archetype in self.archetypes.values() {
            if archetype.entities.contains(entity) {
                let type_id: TypeId = TypeId::of::<T>();

                if let Some(components) = archetype.components.get(&type_id) {
                    if let Some(index) = archetype.entities.iter().position(|e| e == entity) {
                        return Some(RefMut::map(
                            components[index].borrow_mut(),
                            |component| component.as_any_mut().downcast_mut::<T>().unwrap()
                        ));
                    }
                }
            }
        }
        return None;
    }

    /// Return all the components from a specific entity.
    pub fn get_entity_components<'a>(&'a self, entity: &'a Entity) -> Option<Vec<Ref<'a, Box<dyn Component>>>> {
        if let Some((_, archetype)) = self.archetypes.iter().find(|(_, arch)| arch.entities.contains(entity)) {
            if let Some(index) = archetype.entities.iter().position(|e| e == entity) {
                let mut entity_components: Vec<Ref<'a, Box<dyn Component>>> = Vec::new();
    
                for component_vec in archetype.components.values() {
                    if let Some(component) = component_vec.get(index) {
                        let borrowed: Ref<'_, Box<dyn Component>> = component.borrow();
                        entity_components.push(borrowed);
                    }
                }
                return Some(entity_components);
            }
        }
        return None;
    }

    /// Return all the components from a specific entity as mutables.
    pub fn get_entity_components_mut<'a>(&'a self, entity: &'a Entity) -> Option<Vec<RefMut<'a, Box<dyn Component>>>> {
        if let Some((_, archetype)) = self.archetypes.iter().find(|(_, arch)| arch.entities.contains(entity)) {
            if let Some(index) = archetype.entities.iter().position(|e| e == entity) {
                let mut entity_components: Vec<RefMut<'a, Box<dyn Component>>> = Vec::new();
    
                for component_vec in archetype.components.values() {
                    if let Some(component) = component_vec.get(index) {
                        let borrowed: RefMut<'_, Box<dyn Component>> = component.borrow_mut();
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
