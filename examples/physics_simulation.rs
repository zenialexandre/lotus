use std::cell::{Ref, RefCell, RefMut};
use cgmath::Vector2;
use rand::{rng, rngs::ThreadRng, Rng};
use lotus_engine::*;

#[derive(Clone, Component)]
struct Object();

#[derive(Clone, Component)]
struct Border();

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(engine_context: &mut EngineContext) {
    let circle: Circle = Circle::new(64, 0.5);
    let red_object: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(circle.clone()), Color::RED);
    let blue_object: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(circle.clone()), Color::BLUE);

    let bottom_border: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::WHITE);
    let top_border: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::WHITE);
    let right_border: Shape = Shape::new(Orientation::Vertical, GeometryType::Rectangle, Color::WHITE);
    let left_border: Shape = Shape::new(Orientation::Vertical, GeometryType::Rectangle, Color::WHITE);

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(red_object)),
            RefCell::new(Box::new(Object())),
            RefCell::new(Box::new(Transform::new(Vector2::new(0., 0.), 0., Vector2::new(0.30, 0.30)))),
            RefCell::new(Box::new(Velocity::new(Vector2::new(0.45, 0.45)))),
            RefCell::new(Box::new(Collision::new(
                Collider::new(GeometryType::Square, Vector2::new(0., 0.), Vector2::new(0., 0.))
            )))
        ]
    );

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(blue_object)),
            RefCell::new(Box::new(Object())),
            RefCell::new(Box::new(Transform::new(Vector2::new(-0.45, 0.), 0., Vector2::new(0.30, 0.30)))),
            RefCell::new(Box::new(Velocity::new(Vector2::new(0.45, 0.45)))),
            RefCell::new(Box::new(Collision::new(
                Collider::new(GeometryType::Square, Vector2::new(0., 0.), Vector2::new(0., 0.))
            )))
        ]
    );

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(bottom_border)),
            RefCell::new(Box::new(Border())),
            RefCell::new(Box::new(Transform::new(Vector2::new(0., -1.), 0., Vector2::new(engine_context.window_configuration.width as f32, 0.01)))),
            RefCell::new(Box::new(Collision::new(
                Collider::new(GeometryType::Rectangle, Vector2::new(0., 0.), Vector2::new(0., 0.))
            )))
        ]
    );

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(top_border)),
            RefCell::new(Box::new(Border())),
            RefCell::new(Box::new(Transform::new(Vector2::new(0., 1.), 0., Vector2::new(engine_context.window_configuration.width as f32, 0.01)))),
            RefCell::new(Box::new(Collision::new(
                Collider::new(GeometryType::Rectangle, Vector2::new(0., 0.), Vector2::new(0., 0.))
            )))
        ]
    );

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(right_border)),
            RefCell::new(Box::new(Border())),
            RefCell::new(Box::new(Transform::new(Vector2::new(1.35, 0.), 0., Vector2::new(0.01, engine_context.window_configuration.height as f32)))),
            RefCell::new(Box::new(Collision::new(
                Collider::new(GeometryType::Rectangle, Vector2::new(0., 0.), Vector2::new(0., 0.))
            )))
        ]
    );

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(left_border)),
            RefCell::new(Box::new(Border())),
            RefCell::new(Box::new(Transform::new(Vector2::new(-1.35, 0.), 0., Vector2::new(0.01, engine_context.window_configuration.height as f32)))),
            RefCell::new(Box::new(Collision::new(
                Collider::new(GeometryType::Rectangle, Vector2::new(0., 0.), Vector2::new(0., 0.))
            )))
        ]
    );
}

fn update(engine_context: &mut EngineContext) {
    let mut query: Query = Query::new(&engine_context.world).with_components::<Object>();
    let entities: Vec<Entity> = query.get_entities_ids_flex().unwrap();

    let mut border_query: Query = Query::new(&engine_context.world).with_components::<Border>();
    let borders: Vec<Entity> = border_query.get_entities_ids_flex().unwrap();

    for entity in &entities {
        let mut velocity: RefMut<'_, Velocity> = engine_context.world.get_entity_component_mut::<Velocity>(entity).unwrap();
        let collision: Ref<'_, Collision> = engine_context.world.get_entity_component::<Collision>(entity).unwrap();

        for border in &borders {
            let border_collision: Ref<'_, Collision> = engine_context.world.get_entity_component::<Collision>(&border).unwrap();
    
            if Collision::check(CollisionAlgorithm::Aabb, &collision, &border_collision) {
                let border_transform: Ref<'_, Transform> = engine_context.world.get_entity_component::<Transform>(&border).unwrap();
    
                if border_transform.get_position().x.abs() < border_transform.get_position().y.abs() {
                    velocity.value.y *= -1.0;
                } else {
                    velocity.value.x *= -1.0;
                }
                break;
            }
        }
    }

    for (index, entity) in entities.iter().enumerate() {
        let mut velocity: RefMut<'_, Velocity> = engine_context.world.get_entity_component_mut::<Velocity>(entity).unwrap();
        let collision: Ref<'_, Collision> = engine_context.world.get_entity_component::<Collision>(entity).unwrap();

        if let Some(next_entity) = entities.get(index + 1) {
            let mut next_entity_velocity: RefMut<'_, Velocity> = engine_context.world.get_entity_component_mut::<Velocity>(next_entity).unwrap();
            let next_entity_collision: Ref<'_, Collision> = engine_context.world.get_entity_component::<Collision>(next_entity).unwrap();

            if Collision::check(CollisionAlgorithm::Aabb, &collision, &next_entity_collision) {
                let mut thread_rng: ThreadRng = rng();
                let random_angle: f32 = thread_rng.random_range(0.0..std::f32::consts::TAU);
                let new_direction: Vector2<f32> = Vector2::new(random_angle.cos(), random_angle.sin());

                velocity.value = new_direction;
                next_entity_velocity.value = -new_direction;
            }
        }
    }

    for entity in &entities {
        let mut transform: RefMut<'_, Transform> = engine_context.world.get_entity_component_mut::<Transform>(entity).unwrap();
        let velocity: Ref<'_, Velocity> = engine_context.world.get_entity_component::<Velocity>(entity).unwrap();

        let new_position: Vector2<f32> = transform.get_position() + velocity.value * engine_context.delta;
        transform.set_position(&engine_context, new_position);
    }
}
