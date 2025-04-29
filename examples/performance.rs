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
    let mut thread_rng: ThreadRng = rng();

    let left_border_perimeter: f64 = -1.3;
    let right_border_perimiter: f64 = 1.3;
    let bottom_border_perimeter: f64 = -0.9;
    let top_border_perimeter: f64 = 0.9;

    // Update the number of spawns for performance benchmarking.
    for _ in 1..=10 {
        let randomic_position_x: f32 = thread_rng.random_range(left_border_perimeter..right_border_perimiter) as f32;
        let randomic_position_y: f32 = thread_rng.random_range(bottom_border_perimeter..top_border_perimeter) as f32;

        let randomic_velocity_x: f32 = thread_rng.random_range(-0.5..0.5) as f32;
        let randomic_velocity_y: f32 = thread_rng.random_range(-0.5..0.5) as f32;

        context.commands.spawn(
            vec![
                Box::new(Shape::new(Orientation::Horizontal, GeometryType::Circle(circle.clone()), Color::BLUE)),
                Box::new(Object()),
                Box::new(Transform::new(
                    Position::new(Vector2::new(randomic_position_x, randomic_position_y), Strategy::Normalized),
                    0.0,
                    Vector2::new(0.15, 0.15)
                )),
                Box::new(Velocity::new(Vector2::new(randomic_velocity_x, randomic_velocity_y))),
                Box::new(Collision::new(Collider::new_simple(GeometryType::Square)))
            ]
        );
    }

    spawn_border(context, Orientation::Horizontal, Vector2::new(0.0, -1.), Vector2::new(context.window_configuration.width as f32, 0.01));
    spawn_border(context, Orientation::Horizontal, Vector2::new(0.0, 1.), Vector2::new(context.window_configuration.width as f32, 0.01));
    spawn_border(context, Orientation::Vertical, Vector2::new(1.35, 0.), Vector2::new(0.01, context.window_configuration.height as f32));
    spawn_border(context, Orientation::Vertical, Vector2::new(-1.35, 0.), Vector2::new(0.01, context.window_configuration.height as f32));
}

fn update(context: &mut Context) {
    context.commands.show_fps(context.game_loop_listener.current_fps, Color::BLACK);

    let mut query: Query = Query::new(&context.world).with::<Object>();
    let entities: Vec<Entity> = query.entities_with_components().unwrap();

    check_border_collision(context, &entities);
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

fn move_objects(context: &mut Context, entities: &Vec<Entity>) {
    for entity in entities {
        let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(entity).unwrap();
        let velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(entity).unwrap();

        let new_position: Vector2<f32> = transform.position.to_vec() + velocity.to_vec() * context.delta;
        transform.set_position(&context.render_state, new_position);
    }
}
