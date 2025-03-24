use std::{any::TypeId, cell::RefMut};
use super::{
    world::{
        Archetype,
        World
    },
    component::Component,
    entitiy::Entity
};

/// Struct to represent the querys made on the world.
pub struct Query<'a> {
    pub parameters: Vec<TypeId>,
    world: &'a World
}

impl<'a> Query<'a> {
    /// Create a new query with the world as the parameter.
    pub fn new(world: &'a World) -> Self {
        return Self { parameters: Vec::new(), world };
    }

    /// Set the components that will make up the query.
    pub fn with_components<T: Component + 'static>(mut self) -> Self {
        self.parameters.push(TypeId::of::<T>());
        return self;
    }

    /// Returns entities by the exact components as mutables of a archetype.
    pub fn get_entities_by_components_mut_exact(&'a mut self) -> Option<Vec<(Entity, Vec<RefMut<'a, Box<dyn Component>>>)>> {
        let archetype_unique_key: u64 = self.world.get_archetype_unique_key(&mut self.parameters);
        let archetype: &Archetype = self.world.archetypes.get(&archetype_unique_key)?;
        let mut results: Vec<(Entity, Vec<RefMut<'_, Box<dyn Component>>>)> = Vec::new();

        for entity in &archetype.entities {
            if let Some(components) = self.world.get_entity_components_mut(entity) {
                results.push((*entity, components));
            }
        }
        return Some(results);
    }

    /// Returns entities ids by the components of a archetype in a flexible way.
    pub fn get_entities_ids_flex(&'a mut self) -> Option<Vec<Entity>> {
        let mut results: Vec<Entity> = Vec::new();

        for (_, archetype) in &self.world.archetypes {
            if self.parameters.iter().all(|param| archetype.components.contains_key(param)) {
                for entity in &archetype.entities {
                    results.push(*entity);
                }
            }
        }
        return Some(results);
    }

    /// Returns entities by the components as mutables of a archetype in a flexible way.
    pub fn get_entities_by_components_mut_flex(&'a mut self) -> Option<Vec<(Entity, Vec<RefMut<'a, Box<dyn Component>>>)>> {
        for (_, archetype) in &self.world.archetypes {
            if self.parameters.iter().all(|param| archetype.components.contains_key(param)) {
                let mut results: Vec<(Entity, Vec<RefMut<'_, Box<dyn Component>>>)> = Vec::new();
    
                for entity in &archetype.entities {
                    if let Some(components) = self.world.get_entity_components_mut(entity) {
                        results.push((*entity, components));
                    }
                }
                return Some(results);
            }
        }
        return None;
    }

    /// Returns all entites by components as mutables in a flexible way.
    pub fn get_all_entities_by_componenets_mut_flex(&'a mut self) -> Option<Vec<(Entity, Vec<RefMut<'a, Box<dyn Component>>>)>> {
        let mut results: Vec<(Entity, Vec<RefMut<'_, Box<dyn Component>>>)> = Vec::new();

        for (_, archetype) in &self.world.archetypes {
            if self.parameters.iter().all(|param| archetype.components.contains_key(param)) {    
                for entity in &archetype.entities {
                    if let Some(components) = self.world.get_entity_components_mut(entity) {
                        results.push((*entity, components));
                    }
                }
            }
        }
        return Some(results);
    }

    fn _matches(&self, entity_components: &[&mut dyn Component]) -> bool {
        return self.parameters.iter().all(|param| {
            entity_components.iter().any(|component| component.as_any().type_id() == *param)
        });
    }
}
