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
    }

    #[test]
    fn commands_despawning_test() {
        let mut commands: Commands = Commands::new();
        let mut world: World = World::new();
        let mut render_state: RenderState = RenderState::dummy();
        let mut query: Query = Query::new(&world).with_components::<Velocity>();

        //world.spawn(vec![Box::new(Velocity::new(Vector2::new(0.0, 0.0)))]);

        let entities: Vec<Entity> = query.get_entities_ids_flex().unwrap();

        commands.despawn(entities.first().unwrap().clone());

        commands.flush_commands(&mut world, &mut render_state);

        let archetypes_as_vec: Vec<(&u64, &Archetype)> = world.archetypes.iter().collect::<Vec<_>>().clone();
        assert!(&archetypes_as_vec.first().unwrap().1.entities.len() == &0);
    }
}
