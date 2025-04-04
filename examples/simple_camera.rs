use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let player: Sprite = Sprite::new("assets/textures/lotus_pink_256x256.png".to_string());
    let secondary_sprite: Sprite = Sprite::new("assets/textures/lotus_pink_256x256.png".to_string());

    context.commands.spawn(
        vec![
            Box::new(player),
            Box::new(Velocity::new(Vector2::new(1.0, 1.0)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(secondary_sprite),
            Box::new(Transform::new(Vector2::new(-0.25, 0.0), 0.0, Vector2::new(0.25, 0.25)))
        ]
    );
}

fn update(context: &mut Context) {
    let input: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();

    let mut query: Query = Query::new(&context.world).with_components::<Sprite>().with_components::<Velocity>();
    let player_entity: Entity = query.get_entities_flex().unwrap().first().unwrap().clone();

    let mut camera2d: ResourceRefMut<'_, Camera2d> = context.world.get_resource_mut::<Camera2d>().unwrap();
    camera2d.set_target(player_entity);

    let velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(&player_entity).unwrap();
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&player_entity).unwrap();

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::ArrowRight)) {
        let x: f32 = transform.position.x + velocity.value.x * context.delta;
        transform.set_position_x(&context.render_state, x);
    } else if input.is_key_pressed(PhysicalKey::Code(KeyCode::ArrowLeft)) {
        let x: f32 = transform.position.x - velocity.value.x * context.delta;
        transform.set_position_x(&context.render_state, x);
    }
}
