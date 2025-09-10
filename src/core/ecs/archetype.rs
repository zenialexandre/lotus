use std::{any::TypeId, collections::HashMap};
use atomic_refcell::AtomicRefCell;
use super::{entity::Entity, component::Component};

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
