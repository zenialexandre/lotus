#[cfg(test)]
pub mod tests {
    use lotus_engine::*;

    #[test]
    fn commands_spawning_test() {
        let mut commands: Commands = Commands::new();
        let mut world: World = World::new();
        let mut render_state: RenderState = RenderState::dummy();

        commands.spawn(vec![Box::new(Velocity::new(Vector2::new(0.0, 0.0)))]);
        commands.flush_commands(&mut world, &mut render_state);

        assert!(world.archetypes.len() == 1);

        let archetypes_as_vec: Vec<(&u64, &Archetype)> = world.archetypes.iter().collect::<Vec<_>>().clone();
        assert!(&archetypes_as_vec.first().unwrap().1.entities.len() == &1);
        assert!(world.is_entity_alive(archetypes_as_vec.first().unwrap().1.entities[0]));
        assert!(world.is_entity_visible(archetypes_as_vec.first().unwrap().1.entities[0]));
    }

    #[test]
    fn commands_despawning_test() {
        let mut commands: Commands = Commands::new();
        let mut world: World = World::new();
        let mut render_state: RenderState = RenderState::dummy();

        commands.spawn(vec![Box::new(Velocity::new(Vector2::new(0.0, 0.0)))]);
        commands.flush_commands(&mut world, &mut render_state);

        let entity: Entity = {
            let mut query: Query = Query::new(&world).with::<Velocity>();
            query.entities_with_components().unwrap().first().unwrap().clone()
        };

        commands.despawn(entity);
        commands.flush_commands(&mut world, &mut render_state);

        let archetypes_as_vec: Vec<(&u64, &Archetype)> = world.archetypes.iter().collect::<Vec<_>>().clone();
        assert!(&archetypes_as_vec.first().unwrap().1.entities.len() == &0);
        assert!(!world.is_entity_alive(entity));
    }

    #[test]
    fn get_component_from_entity_as_immutable_test() {
        let mut commands: Commands = Commands::new();
        let mut world: World = World::new();
        let mut render_state: RenderState = RenderState::dummy();

        let dummy_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BLACK);
        commands.spawn(vec![Box::new(dummy_shape)]);
        commands.flush_commands(&mut world, &mut render_state);

        let entity: Entity = {
            let mut query: Query = Query::new(&world).with::<Shape>();
            query.entities_with_components().unwrap().first().unwrap().clone()
        };
        assert!(!world.get_entity_component::<Shape>(&entity).is_none());
    }

    #[test]
    fn get_component_from_entity_as_mutable_test() {
        let mut commands: Commands = Commands::new();
        let mut world: World = World::new();
        let mut render_state: RenderState = RenderState::dummy();

        let dummy_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BLACK);
        commands.spawn(vec![Box::new(dummy_shape)]);
        commands.flush_commands(&mut world, &mut render_state);

        let entity: Entity = {
            let mut query: Query = Query::new(&world).with::<Shape>();
            query.entities_with_components().unwrap().first().unwrap().clone()
        };
        assert!(!world.get_entity_component_mut::<Shape>(&entity).is_none());
    }

    #[test]
    fn get_all_components_as_immutable_from_entity_test() {
        let mut commands: Commands = Commands::new();
        let mut world: World = World::new();
        let mut render_state: RenderState = RenderState::dummy();

        let dummy_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BLACK);
        commands.spawn(vec![Box::new(dummy_shape)]);
        commands.flush_commands(&mut world, &mut render_state);

        let entity: Entity = {
            let mut query: Query = Query::new(&world).with::<Shape>();
            query.entities_with_components().unwrap().first().unwrap().clone()
        };
        assert!(!world.get_entity_components(&entity).is_none());
        assert!(!world.get_entity_components(&entity).unwrap().is_empty());
    }

    #[test]
    fn get_all_components_as_mutable_from_entity_test() {
        let mut commands: Commands = Commands::new();
        let mut world: World = World::new();
        let mut render_state: RenderState = RenderState::dummy();

        let dummy_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BLACK);
        commands.spawn(vec![Box::new(dummy_shape)]);
        commands.flush_commands(&mut world, &mut render_state);

        let entity: Entity = {
            let mut query: Query = Query::new(&world).with::<Shape>();
            query.entities_with_components().unwrap().first().unwrap().clone()
        };
        assert!(!world.get_entity_components_mut(&entity).is_none());
        assert!(!world.get_entity_components_mut(&entity).unwrap().is_empty());
    }

    #[test]
    fn get_resource_as_immutable_test() {
        let world: World = World::new();
        assert!(!world.get_resource::<Input>().is_none());
    }

    #[test]
    fn get_resource_as_mutable_test() {
        let world: World = World::new();
        assert!(!world.get_resource_mut::<Input>().is_none());
    }
}
