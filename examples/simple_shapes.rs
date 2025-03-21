use lotus_engine::*;
use std::cell::RefCell;
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

fn setup(engine_context: &mut EngineContext) {
    let my_square: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BLUE);
    let my_rectangle: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::GREEN);
    let my_triangle: Shape = Shape::new(Orientation::Horizontal, GeometryType::Triangle, Color::RED);
    let my_circle: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(Circle::new(64, 0.5)), Color::BLACK);

    engine_context.world.spawn(&mut engine_context.render_state, &mut vec![
        RefCell::new(Box::new(my_square)),
        RefCell::new(Box::new(Transform::new(Vector2::new(-0.60, -0.25), 0., Vector2::new(0.10, 0.10)))),
        RefCell::new(Box::new(MySquare()))
    ]);
    engine_context.world.spawn(&mut engine_context.render_state, &mut vec![
        RefCell::new(Box::new(my_rectangle)),
        RefCell::new(Box::new(Transform::new(Vector2::new(-0.35, 0.20), 0., Vector2::new(0.50, 0.50)))),
        RefCell::new(Box::new(MyRectangle()))
    ]);
    engine_context.world.spawn(&mut engine_context.render_state, &mut vec![
        RefCell::new(Box::new(my_triangle)),
        RefCell::new(Box::new(Transform::new(Vector2::new(0.50, 0.50), 0., Vector2::new(0.25, 0.25)))),
        RefCell::new(Box::new(MyTriangle()))
    ]);
    engine_context.world.spawn(&mut engine_context.render_state, &mut vec![
        RefCell::new(Box::new(my_circle)),
        RefCell::new(Box::new(Transform::new(Vector2::new(0.80, 0.50), 0., Vector2::new(0.25, 0.25)))),
        RefCell::new(Box::new(MyCircle()))
    ]);
}

fn update(engine_context: &mut EngineContext) {
    let mut query: Query = Query::new(&engine_context.world).with_components::<Shape>();
    let results: Vec<(Entity, Vec<std::cell::RefMut<'_, Box<dyn Component>>>)> = query.get_all_entities_by_componenets_mut_flex().unwrap();

    for result in results {
        let (_, mut components) = result;

        for component in &mut components {
            if let Some(transform) = component.as_any_mut().downcast_mut::<Transform>() {
                let rotation: f32 = transform.get_rotation() + 100. * engine_context.delta;
                transform.set_rotation(&engine_context, rotation);
            }
        }
    }
}
