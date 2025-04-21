//! This example is a show off about physics.
//! Making use of the velocity and collision components, two shapes (circles) are moving and colliding around the window.
//! The collision happens between the two shapes and with the borders of the window. 

use lotus_engine::*;
use rand::{rng, rngs::ThreadRng, Rng};

#[derive(Clone, Component)]
struct Object();

#[derive(Clone, Component)]
struct Border();

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let circle: Circle = Circle::new(64, 0.5);
    let red_object: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(circle.clone()), Color::RED);
    let blue_object: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(circle.clone()), Color::BLUE);

    context.commands.spawn(
        vec![
            Box::new(red_object),
            Box::new(Object()),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
                0.0,
                Vector2::new(0.30, 0.30)
            )),
            Box::new(Velocity::new(Vector2::new(0.45, 0.45))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Square)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(blue_object),
            Box::new(Object()),
            Box::new(Transform::new(
                Position::new(Vector2::new(-0.45, 0.0), Strategy::Normalized),
                0.0,
                Vector2::new(0.30, 0.30)
            )),
            Box::new(Velocity::new(Vector2::new(0.45, 0.45))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Square)))
        ]
    );

    spawn_border(context, Orientation::Horizontal, Vector2::new(0.0, -1.), Vector2::new(context.window_configuration.width as f32, 0.01));
    spawn_border(context, Orientation::Horizontal, Vector2::new(0.0, 1.), Vector2::new(context.window_configuration.width as f32, 0.01));
    spawn_border(context, Orientation::Vertical, Vector2::new(1.35, 0.), Vector2::new(0.01, context.window_configuration.height as f32));
    spawn_border(context, Orientation::Vertical, Vector2::new(-1.35, 0.), Vector2::new(0.01, context.window_configuration.height as f32));
}

fn update(context: &mut Context) {
    let mut query: Query = Query::new(&context.world).with::<Object>();
    let entities: Vec<Entity> = query.entities_with_components().unwrap();

    check_border_collision(context, &entities);
    check_object_collision(context, &entities);
    move_objects(context, &entities);
}

fn spawn_border(context: &mut Context, orientation: Orientation, position: Vector2<f32>, scale: Vector2<f32>) {
    let border: Shape = Shape::new(orientation, GeometryType::Rectangle, Color::WHITE);

    context.commands.spawn(
        vec![
            Box::new(border),
            Box::new(Border()),
            Box::new(Transform::new(
                Position::new(position, Strategy::Normalized),
                0.0,
                scale
            )),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
}

fn check_border_collision(context: &mut Context, entities: &Vec<Entity>) {
    let mut border_query: Query = Query::new(&context.world).with::<Border>();
    let borders: Vec<Entity> = border_query.entities_with_components().unwrap();

    for entity in entities {
        let mut velocity: ComponentRefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(entity).unwrap();
        let collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(entity).unwrap();

        for border in &borders {
            let border_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&border).unwrap();
    
            if Collision::check(CollisionAlgorithm::Aabb, &collision, &border_collision) {
                let border_transform: ComponentRef<'_, Transform> = context.world.get_entity_component::<Transform>(&border).unwrap();
    
                if border_transform.position.x.abs() < border_transform.position.y.abs() {
                    velocity.y *= -1.0;
                } else {
                    velocity.x *= -1.0;
                }
                break;
            }
        }
    }
}

fn check_object_collision(context: &mut Context, entities: &Vec<Entity>) {
    for (index, entity) in entities.iter().enumerate() {
        let mut velocity: ComponentRefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(entity).unwrap();
        let collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(entity).unwrap();

        if let Some(next_entity) = entities.get(index + 1) {
            let mut next_entity_velocity: ComponentRefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(next_entity).unwrap();
            let next_entity_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(next_entity).unwrap();

            if Collision::check(CollisionAlgorithm::Aabb, &collision, &next_entity_collision) {
                let mut thread_rng: ThreadRng = rng();
                let random_angle: f32 = thread_rng.random_range(0.0..std::f32::consts::TAU);
                let new_direction: Vector2<f32> = Vector2::new(random_angle.cos(), random_angle.sin());
                let collision_impulse: f32 = 1.5;

                velocity.update_values(new_direction * collision_impulse);
                next_entity_velocity.update_values(-new_direction * collision_impulse);
            }
        }
    }
}

fn move_objects(context: &mut Context, entities: &Vec<Entity>) {
    for entity in entities {
        let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(entity).unwrap();
        let velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(entity).unwrap();

        let new_position: Vector2<f32> = transform.position.to_vec() + velocity.to_vec() * context.delta;
        transform.set_position(&context.render_state, new_position);
    }
}
