use std::{any::TypeId, cell::RefMut};
use super::{world::{Archetype, World}, component::Component, entitiy::Entity};

pub struct Query<'a> {
    pub parameters: Vec<TypeId>,
    world: &'a World
}

impl<'a> Query<'a> {
    pub fn new(world: &'a World) -> Self {
        return Self { parameters: Vec::new(), world };
    }

    pub fn with_components<T: Component + 'static>(mut self) -> Self {
        self.parameters.push(TypeId::of::<T>());
        return self;
    }

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
