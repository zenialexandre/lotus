//! This example is a show off on rendering geometric forms with solid colors.
//! Basic geometric forms are rendered inside the 'setup' function.
//! Inside the 'update' function, each entity has its rotation factor mutated by a constant value of 100.0.
//! In this example the FPS cap is setted to 120 and in the 'update' function the FPS is shown on the screen.

use lotus_engine::*;

#[derive(Component)]
struct MySquare();

#[derive(Component)]
struct MyRectangle();

#[derive(Component)]
struct MyTriangle();

#[derive(Component)]
struct MyCircle();

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    context.game_loop_listener.fps_cap(120);

    let my_square: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BLUE);
    let my_rectangle: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::GREEN);
    let my_triangle: Shape = Shape::new(Orientation::Horizontal, GeometryType::Triangle, Color::RED);
    let my_circle: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(Circle::new(64, 0.5)), Color::BLACK);

    context.commands.spawn(
        vec![
            Box::new(my_square),
            Box::new(Transform::new(
                Position::new(Vector2::new(-0.60, -0.25), Strategy::Normalized),
                0.0,
                Vector2::new(0.10, 0.10)
            )),
            Box::new(MySquare())
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(my_rectangle),
            Box::new(Transform::new(
                Position::new(Vector2::new(-0.35, 0.20), Strategy::Normalized),
                0.0,
                Vector2::new(0.50, 0.50)
            )),
            Box::new(MyRectangle())
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(my_triangle),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.50, 0.50), Strategy::Normalized),
                0.0,
                Vector2::new(0.25, 0.25)
            )),
            Box::new(MyTriangle())
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(my_circle),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.80, 0.50), Strategy::Normalized),
                0.0,
                Vector2::new(0.25, 0.25)
            )),
            Box::new(MyCircle())
        ]
    );
}

fn update(context: &mut Context) {
    context.commands.show_fps(context.game_loop_listener.current_fps);

    let mut query: Query = Query::new(&context.world).with::<Shape>();
    let entities: Vec<Entity> = query.entities_with_components().unwrap();

    for entity in &entities {
        let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(entity).unwrap();
        let rotation: f32 = transform.get_rotation() + 100.0 * context.delta;
        transform.set_rotation(&context.render_state, rotation);
    }
}
