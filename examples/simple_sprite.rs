use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let sprite: Sprite = Sprite::new("assets/textures/lotus_pink_256x256.png".to_string());

    context.commands.spawn(
        vec![
            Box::new(sprite),
            Box::new(Transform::new(Vector2::new(-0.50, -0.50), 0.0, Vector2::new(0.25, 0.25))),
            Box::new(Velocity::new(Vector2::new(0.50, 0.50)))
        ]
    );
}

fn update(context: &mut Context) {
    let mut query: Query = Query::new(&context.world).with_components::<Sprite>();
    let results: Vec<Entity> = query.get_entities_flex().unwrap();
    let entity: &Entity = results.first().unwrap();

    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(entity).unwrap();
    let velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(entity).unwrap();

    transform.position.y += velocity.value.y * context.delta;
}
