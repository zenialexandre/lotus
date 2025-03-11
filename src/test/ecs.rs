#[cfg(test)]
mod ecs_test {
    use std::cell::RefCell;

    use cgmath::Vector2;

    use crate::{core::ecs::{component::Component, world::World}, Transform};

    #[test]
    fn entity_creation_test() {
        let mut world: World = World::new();
        let mut components: Vec<RefCell<Box<dyn Component>>> = Vec::new();
        let transform: Transform = Transform::new(Vector2::new(0.10, 0.25), 0., Vector2::new(1., 1.));

        components.push(RefCell::new(Box::new(transform)));

        world.spawn(&mut components);

        assert!(world.archetypes.len() == 1);
    }
}
