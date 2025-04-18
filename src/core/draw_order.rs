use std::cmp::Ordering;
use lotus_proc_macros::Component;
use super::ecs::{world::World, entity::Entity};

/// Struct to order the drawing process of entities.
/// The smaller number will be rendered first.    
#[derive(Clone, Component)]
pub struct DrawOrder(pub u32);

impl DrawOrder {
    /// Returns the ordering related to the comparison.
    pub fn compare(world: &World, a: &Entity, b: &Entity) -> Ordering {
        let a_order: u32 = world.get_entity_component::<DrawOrder>(a).map(|d| d.0).unwrap_or(0);
        let b_order: u32 = world.get_entity_component::<DrawOrder>(b).map(|d| d.0).unwrap_or(0);

        match a_order.cmp(&b_order) {
            Ordering::Equal => {
                a.0.cmp(&b.0)
            },
            other => other
        }
    }
}

impl Default for DrawOrder {
    fn default() -> Self {
        return Self(0);
    }
}
