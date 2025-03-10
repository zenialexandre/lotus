use std::{any::{Any, TypeId}, collections::HashMap};

use super::{component::Component, entitiy::Entity};

struct Archetype {
    entities: Vec<Entity>,
    components: HashMap<TypeId, Vec<Box<dyn Any>>>
}

pub struct World {
    archetypes: Vec<Archetype>
}

impl World {
    pub fn new() -> Self {
        return Self { archetypes: Vec::new() };
    }
}
