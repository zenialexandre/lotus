//! This example is a simple show off about telling the camera resource that it need to follow a specific target.
//! The camera 2D is a resource of the world and can be accessed at easy.

use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let player: Sprite = Sprite::new("textures/lotus_pink_256x256.png".to_string());
    let secondary_sprite: Sprite = Sprite::new("textures/lotus_pink_256x256.png".to_string());
    let shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BURGUNDY);
    let text: Text = Text::new(
        &mut context.render_state,
        Font::new(Fonts::RobotoMono.get_path(), 30.0),
        Position::new(Vector2::new(0.0, 0.0), Strategy::Pixelated),
        Color::BLACK,
        "Boiler Plate".to_string()
    );

    context.commands.spawn(
        vec![
            Box::new(player),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
                0.0,
                Vector2::new(0.25, 0.25)
            )),
            Box::new(Velocity::new(Vector2::new(1.0, 1.0)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(secondary_sprite),
            Box::new(Transform::new(
                Position::new(Vector2::new(-0.25, 0.0), Strategy::Normalized),
                0.0,
                Vector2::new(0.25, 0.25)
            ))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(shape),
            Box::new(Transform::new(
                Position::new(Vector2::new(-0.75, 0.0), Strategy::Normalized),
                0.0,
                Vector2::new(0.25, 0.25)
            )),
            Box::new(Velocity::new(Vector2::new(1.0, 1.0)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(text)
        ]
    );
}

fn update(context: &mut Context) {
    let input: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();

    let mut query: Query = Query::new(&context.world).with::<Sprite>().with::<Velocity>();
    let player_entity: Entity = query.entities_with_components().unwrap().first().unwrap().clone();

    let mut camera2d: ResourceRefMut<'_, Camera2d> = context.world.get_resource_mut::<Camera2d>().unwrap();
    camera2d.set_target(player_entity);

    let velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(&player_entity).unwrap();
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&player_entity).unwrap();

    if input.is_key_pressed(KeyCode::ArrowRight) {
        let x: f32 = transform.position.x + velocity.x * context.delta;
        transform.set_position_x(&context.render_state, x);
    } else if input.is_key_pressed(KeyCode::ArrowLeft) {
        let x: f32 = transform.position.x - velocity.x * context.delta;
        transform.set_position_x(&context.render_state, x);
    }
}
