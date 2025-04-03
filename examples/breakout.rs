use lotus_engine::*;
use std::{cell::{Ref, RefMut}, time::Duration};
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Component)]
pub struct Player();

#[derive(Clone, Component)]
pub struct LittleBall();

#[derive(Clone, Component)]
pub struct Target();

#[derive(Clone, Component)]
pub struct Border();

#[derive(Clone, Resource)]
pub struct LittleBallRespawnTimer(Timer);

impl LittleBallRespawnTimer {
    pub fn new() -> Self {
        return Self(Timer::new(TimerType::Repeat, Duration::new(2, 0)));
    }
}

your_game!(
    WindowConfiguration {
        icon_path: "assets/textures/lotus_pink_256x256.png".to_string(),
        title: "Breakout Game :)".to_string(),
        background_color: Some(Color::CYAN),
        background_image_path: None,
        width: 725.0,
        height: 695.0,
        position_x: 200.0,
        position_y: 150.0,
        resizable: false,
        decorations: true,
        transparent: false,
        visible: true,
        active: true,
        enabled_buttons: winit::window::WindowButtons::CLOSE | winit::window::WindowButtons::MINIMIZE
    },
    setup,
    update
);

fn setup(context: &mut Context) {
    let player: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::PURPLE);
    let little_ball: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(Circle::new(64, 0.2)), Color::BLACK);

    let mut thread_rng: ThreadRng = rand::rng();
    let random_x_direction: f32 = thread_rng.random_range(-0.8..0.8);

    context.world.add_resource(LittleBallRespawnTimer::new());

    context.commands.spawn(
        vec![
            Box::new(player),
            Box::new(Player()),
            Box::new(Transform::new(Vector2::new(0.0, -0.85), 0.0, Vector2::new(0.15, 0.10))),
            Box::new(Velocity::new(Vector2::new(2.0, 2.0))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(little_ball),
            Box::new(LittleBall()),
            Box::new(Transform::new(Vector2::new(0.0, 0.0), 0.0, Vector2::new(0.10, 0.10))),
            Box::new(Velocity::new(Vector2::new(random_x_direction, -0.5))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Square)))
        ]
    );

    spawn_border(context, Vector2::new(1.1, 0.));
    spawn_border(context, Vector2::new(-1.1, 0.));
    spawn_targets(context);
}

fn update(context: &mut Context) {
    let input: Input = {
        let input_ref: ResourceRefMut<'_, Input> = context.world.get_resource_mut::<Input>().unwrap();
        input_ref.clone()
    };

    let mut player_query: Query = Query::new(&context.world).with_components::<Player>();
    let player_entity: Entity = player_query.get_entities_flex().unwrap().first().unwrap().clone();

    let mut little_ball_query: Query = Query::new(&context.world).with_components::<LittleBall>();
    let little_ball_entity: Entity = little_ball_query.get_entities_flex().unwrap().first().unwrap().clone();

    let mut thread_rng: ThreadRng = rand::rng();
    let random_factor: f32 = thread_rng.random_range(-0.5..0.5);

    move_player(context, input, player_entity);
    move_little_ball(context, little_ball_entity);
    check_player_little_ball_collision(context, player_entity, little_ball_entity, random_factor);
    check_little_ball_borders_collision(context, little_ball_entity, random_factor);
    check_litte_ball_targets_collision(context, little_ball_entity, random_factor);
    respawn_little_ball_after_outbounds(context, little_ball_entity);
}

fn spawn_border(context: &mut Context, position: Vector2<f32>) {
    let border: Shape = Shape::new(Orientation::Vertical, GeometryType::Rectangle, Color::CYAN);

    context.commands.spawn(
        vec![
            Box::new(border),
            Box::new(Border()),
            Box::new(Transform::new(position, 0.0, Vector2::new(0.01, context.window_configuration.height as f32))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
}

fn spawn_targets(context: &mut Context) {
    let width: f32 = 0.15;
    let height: f32 = 0.10;

    let rows: i32 = 8;
    let columns: i32 = 10;
    let spacing_x: f32 = 0.09;
    let spacing_y: f32 = 0.02;

    let start_x: f32 = -(columns as f32 * (width + spacing_x)) / 2.0;
    let start_y: f32 = 1.0 - 0.1;

    for row in 0..rows {
        for column in 0..columns {
            let x: f32 = start_x + column as f32 * (width + spacing_x);
            let y: f32 = start_y - row as f32 * (height + spacing_y);

            let mut color: Color = Color::RED;

            if row == 2 || row == 3 {
                color = Color::ORANGE;
            } else if row == 4 || row == 5 {
                color = Color::GREEN;
            } else if row == 6 || row == 7 {
                color = Color::YELLOW;
            }

            context.commands.spawn(
                vec![
                    Box::new(Shape::new(Orientation::Horizontal, GeometryType::Rectangle, color)), 
                    Box::new(Target()),
                    Box::new(Transform::new(Vector2::new(x, y), 0.0, Vector2::new(width, height))),
                    Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle))),
                ]
            );
        }
    }
}

fn move_player(context: &mut Context, input: Input, player_entity: Entity) {
    let mut player_transform: RefMut<'_, Transform> = context.world.get_entity_component_mut(&player_entity).unwrap();
    let player_velocity: Ref<'_, Velocity> = context.world.get_entity_component(&player_entity).unwrap();

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::ArrowRight)) {
        let x: f32 = player_transform.position.x + player_velocity.value.x * context.delta;
        player_transform.set_position_x(&context.render_state, x);
    } else if input.is_key_pressed(PhysicalKey::Code(KeyCode::ArrowLeft)) {
        let x: f32 = player_transform.position.x - player_velocity.value.x * context.delta;
        player_transform.set_position_x(&context.render_state, x);
    }
}

fn move_little_ball(context: &mut Context, little_ball_entity: Entity) {
    let mut little_ball_transform: RefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&little_ball_entity).unwrap();
    let little_ball_velocity: Ref<'_, Velocity> = context.world.get_entity_component::<Velocity>(&little_ball_entity).unwrap();

    let new_position: Vector2<f32> = little_ball_transform.position + little_ball_velocity.value * context.delta;
    little_ball_transform.set_position(&context.render_state, new_position);
}

fn check_player_little_ball_collision(context: &mut Context, player_entity: Entity, little_ball_entity: Entity, random_factor: f32) {
    let mut little_ball_transform: RefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&little_ball_entity).unwrap();
    let mut little_ball_velocity: RefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(&little_ball_entity).unwrap();
    let little_ball_collision: Ref<'_, Collision> = context.world.get_entity_component::<Collision>(&little_ball_entity).unwrap();

    let player_collision: Ref<'_, Collision> = context.world.get_entity_component::<Collision>(&player_entity).unwrap();

    if Collision::check(CollisionAlgorithm::Aabb, &player_collision, &little_ball_collision) {
        let relative_collision_position: Vector2<f32> = little_ball_collision.collider.position - player_collision.collider.position;
        
        let rebound_direction: Vector2<f32> = if relative_collision_position.y > 0.0 {
            Vector2::new(relative_collision_position.x, 1.0)
        } else {
            Vector2::new(relative_collision_position.x, -1.0)
        };
        let rebound_vector: Vector2<f32> = (rebound_direction + Vector2::new(random_factor, 0.0)).normalize();
        little_ball_velocity.value = rebound_vector * little_ball_velocity.value.magnitude();
        little_ball_transform.position.y += rebound_direction.y * 0.02;
    }
}

fn check_little_ball_borders_collision(context: &mut Context, little_ball_entity: Entity, random_factor: f32) {
    let mut border_query: Query = Query::new(&context.world).with_components::<Border>();
    let borders_entities: Vec<Entity> = border_query.get_entities_flex().unwrap();

    let mut little_ball_transform: RefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&little_ball_entity).unwrap();
    let mut little_ball_velocity: RefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(&little_ball_entity).unwrap();
    let little_ball_collision: Ref<'_, Collision> = context.world.get_entity_component::<Collision>(&little_ball_entity).unwrap();

    for border in &borders_entities {
        let border_collision: Ref<'_, Collision> = context.world.get_entity_component::<Collision>(border).unwrap();

        if Collision::check(CollisionAlgorithm::Aabb, &little_ball_collision, &border_collision) {
            if border_collision.collider.position.x > 0.0 {
                little_ball_velocity.value = Vector2::new(-1.0 + random_factor, little_ball_velocity.value.y.signum()).normalize() * little_ball_velocity.value.magnitude();
                little_ball_transform.position.x -= 0.1;
            } else if border_collision.collider.position.x < 0.0 {
                little_ball_velocity.value = Vector2::new(1.0 + random_factor, little_ball_velocity.value.y.signum()).normalize() * little_ball_velocity.value.magnitude();
                little_ball_transform.position.x += 0.1;
            }
        }
    }
}

fn check_litte_ball_targets_collision(context: &mut Context, little_ball_entity: Entity, random_factor: f32) {
    let mut targets_query: Query = Query::new(&context.world).with_components::<Target>();
    let targets_entities: Vec<Entity> = targets_query.get_entities_flex().unwrap();

    let mut little_ball_transform: RefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&little_ball_entity).unwrap();
    let mut little_ball_velocity: RefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(&little_ball_entity).unwrap();
    let little_ball_collision: Ref<'_, Collision> = context.world.get_entity_component::<Collision>(&little_ball_entity).unwrap();

    for target in &targets_entities {
        let target_collision: Ref<'_, Collision> = context.world.get_entity_component::<Collision>(target).unwrap();

        if Collision::check(CollisionAlgorithm::Aabb, &little_ball_collision, &target_collision) {
            little_ball_velocity.value = Vector2::new(little_ball_velocity.value.x.signum(), -1.0 + random_factor).normalize() * little_ball_velocity.value.magnitude();
            little_ball_transform.position.y -= 0.1;
            context.commands.despawn(target.clone());
        }
    }
}

fn respawn_little_ball_after_outbounds(context: &mut Context, little_ball_entity: Entity) {
    let mut litte_ball_transform: RefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&little_ball_entity).unwrap();
    let position_default: Vector2<f32> = Vector2::new(0.0, -0.25);

    if litte_ball_transform.position.y < -1.0 {
        let mut little_ball_respawn_timer: ResourceRefMut<'_, LittleBallRespawnTimer> = context.world.get_resource_mut::<LittleBallRespawnTimer>().unwrap();
        little_ball_respawn_timer.0.tick(context.delta);

        if little_ball_respawn_timer.0.is_finished() {
            litte_ball_transform.set_position(&context.render_state, position_default);
        }
    }
}
