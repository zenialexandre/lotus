use std::{
    any::TypeId,
    cell::{
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

use crate::{Transform, RenderState};
use super::{component::Component, entitiy::Entity};

pub struct Archetype {
    pub entities: Vec<Entity>,
    pub components: HashMap<TypeId, Vec<RefCell<Box<dyn Component>>>>
}

impl Archetype {
    pub fn new() -> Self {
        return Self {
            entities: Vec::new(),
            components: HashMap::new()
        };
    }

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

pub struct World {
    pub archetypes: HashMap<u64, Archetype>
}

impl World {
    pub fn new() -> Self {
        return Self {
            archetypes: HashMap::new()
        };
    }

    // When spawning, the render should draw this entity.
    pub fn spawn(&mut self, render_state: &mut RenderState, components: &mut Vec<RefCell<Box<dyn Component>>>) -> Entity {
        let entity: Entity = Entity(Uuid::new_v4());
        let has_transform: bool = components.iter().any(|c| c.borrow().as_any().is::<Transform>());

        if !has_transform {
            components.push(RefCell::new(Box::new(Transform::default())));
        }
        let mut components_types_ids: Vec<TypeId> = components.iter().map(|c| c.borrow().type_id()).collect();
        let archetype_unique_key: u64 = self.get_archetype_unique_key(&mut components_types_ids);
        let archetype: &mut Archetype = self.archetypes.entry(archetype_unique_key).or_insert_with(Archetype::new);
        
        // Moving the ownership after the operations are done.
        let moved_components: Vec<RefCell<Box<dyn Component>>> = take(components);
        archetype.add_entity(entity, moved_components);

        render_state.add_entity_to_render(entity);

        return entity;
    }

    pub fn despawn(&mut self, render_state: &mut RenderState, entity: &Entity) {
        if let Some((_, archetype)) = self.archetypes.iter_mut().find(|(_, arch)| arch.entities.contains(&entity)) {
            if let Some(index) = archetype.entities.iter().position(|e| e.0 == entity.0) {
                archetype.entities.remove(index);
                render_state.remove_entity_to_render(entity);
            }
        }
    }

    pub fn get_archetype_unique_key(&self, components_types_ids: &mut Vec<TypeId>) -> u64 {
        components_types_ids.sort();
        return self.get_hash_from_ids(&components_types_ids);
    }

    fn get_hash_from_ids(&self, type_ids: &[TypeId]) -> u64 {
        let mut default_hasher: DefaultHasher = DefaultHasher::new();

        for type_id in type_ids {
            type_id.hash(&mut default_hasher);
        }
        return default_hasher.finish();
    }

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

    pub fn is_entity_alive(&self, entity: Entity) -> bool {
        return self.archetypes.values().any(|archetype| archetype.entities.iter().any(|e| e.0 == entity.0));    
    }
}
