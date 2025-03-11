use std::{any::TypeId, cell::RefMut};

use crate::{Archetype, World};

use super::{component::Component, entitiy::Entity};

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

    pub fn get_entity_by_components_mut(&mut self) -> Option<(Entity, Vec<RefMut<'a, Box<dyn Component>>>)> {
        let archetype_unique_key: u64 = self.world.get_archetype_unique_key(&mut self.parameters);
        let archetype: &Archetype = self.world.archetypes.get(&archetype_unique_key)?;
        let components: Vec<RefMut<'a, Box<dyn Component>>> = self.world.get_entity_components_mut(&*archetype.entities.first()?)?;

        return Some((
            *archetype.entities.first()?,
            components
        ));
    }

    fn _matches(&self, entity_components: &[&mut dyn Component]) -> bool {
        return self.parameters.iter().all(|param| {
            entity_components.iter().any(|component| component.as_any().type_id() == *param)
        });
    }
}
