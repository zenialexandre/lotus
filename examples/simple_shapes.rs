use lotus_engine::*;
use std::cell::RefMut;
use cgmath::Vector2;

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
    let my_square: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BLUE);
    let my_rectangle: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::GREEN);
    let my_triangle: Shape = Shape::new(Orientation::Horizontal, GeometryType::Triangle, Color::RED);
    let my_circle: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(Circle::new(64, 0.5)), Color::BLACK);

    context.world.spawn(
        &mut context.render_state,
        vec![
            Box::new(my_square),
            Box::new(Transform::new(Vector2::new(-0.60, -0.25), 0.0, Vector2::new(0.10, 0.10))),
            Box::new(MySquare())
        ]
    );
    context.world.spawn(
        &mut context.render_state,
        vec![
            Box::new(my_rectangle),
            Box::new(Transform::new(Vector2::new(-0.35, 0.20), 0.0, Vector2::new(0.50, 0.50))),
            Box::new(MyRectangle())
        ]
    );
    context.world.spawn(
        &mut context.render_state,
        vec![
            Box::new(my_triangle),
            Box::new(Transform::new(Vector2::new(0.50, 0.50), 0.0, Vector2::new(0.25, 0.25))),
            Box::new(MyTriangle())
        ]
    );
    context.world.spawn(
        &mut context.render_state,
        vec![
            Box::new(my_circle),
            Box::new(Transform::new(Vector2::new(0.80, 0.50), 0.0, Vector2::new(0.25, 0.25))),
            Box::new(MyCircle())
        ]
    );
}

fn update(context: &mut Context) {
    let mut query: Query = Query::new(&context.world).with_components::<Shape>();
    let entities: Vec<Entity> = query.get_entities_ids_flex().unwrap();

    for entity in &entities {
        let mut transform: RefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(entity).unwrap();
        let rotation: f32 = transform.get_rotation() + 100.0 * context.delta;
        transform.set_rotation(&context, rotation);
    }
}
