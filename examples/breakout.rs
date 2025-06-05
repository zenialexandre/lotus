//! This example aims to recreate the Breakout arcade game.
//! Is a show off of timer, multiple entities rendering, game state workflow, text rendering and physics.
//! The targets are spawned as a matrix of 8 rows and 10 columns.
//! Each target is a specific entity with its own physics.

use lotus_engine::*;
use std::time::Duration;
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

#[derive(Clone, Resource)]
pub struct NextState(pub GameState);

impl Default for NextState {
    fn default() -> Self {
        return Self(GameState::Stopped);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum GameState {
    Running,
    Stopped
}

your_game!(
    WindowConfiguration {
        icon_path: "textures/lotus_pink_256x256.png".to_string(),
        title: "Breakout Game :)".to_string(),
        background_color: Some(Color::LIGHTGRAY),
        background_image_path: None,
        width: 725.0,
        height: 695.0,
        position_x: 200.0,
        position_y: 150.0,
        resizable: false,
        decorations: true,
        transparent: false,
        active: true,
        enabled_buttons: WindowButtons::CLOSE | WindowButtons::MINIMIZE,
        ..Default::default()
    },
    setup,
    update
);

fn setup(context: &mut Context) {
    let player: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::PURPLE);
    let little_ball: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(Circle::new(64, 0.2)), Color::BLACK);
    let start_text: Text = Text::new(
        &mut context.render_state,
        Font::new(Fonts::RobotoMonoItalic.get_path(), 40.0),
        Position::new(Vector2::new(298.0, 380.0), Strategy::Pixelated),
        Color::BLACK,
        "> enter <".to_string()
    );

    let mut thread_rng: ThreadRng = rand::rng();
    let random_direction: bool = thread_rng.random_bool(1.0 / 3.0);
 
    let velocity_x: f32 = if random_direction {
        1.2
    } else {
        -1.2
    };

    context.commands.add_resources(vec![
        Box::new(LittleBallRespawnTimer::new()),
        Box::new(NextState::default())
    ]);
    context.commands.spawn(vec![Box::new(start_text)]);

    context.commands.spawn(
        vec![
            Box::new(player),
            Box::new(Player()),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.0, -0.85), Strategy::Normalized),
                0.0,
                Vector2::new(0.15, 0.10)
            )),
            Box::new(Velocity::new(Vector2::new(2.0, 2.0))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(little_ball),
            Box::new(LittleBall()),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.0, -0.5), Strategy::Normalized),
                0.0,
                Vector2::new(0.10, 0.10)
            )),
            Box::new(Velocity::new(Vector2::new(velocity_x, -0.5))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Square)))
        ]
    );

    spawn_border(context, Vector2::new(1.05, 0.0));
    spawn_border(context, Vector2::new(-1.05, 0.0));
    spawn_targets(context);
}

fn update(context: &mut Context) {
    let input: Input = {
        let input_ref: ResourceRefMut<'_, Input> = context.world.get_resource_mut::<Input>().unwrap();
        input_ref.clone()
    };
    let is_hover: bool = input.mouse_position.x >= 298.0 && (input.mouse_position.y > 380.0 && input.mouse_position.y < 416.0);

    if
        input.is_key_released(KeyCode::Enter) ||
        (input.is_mouse_button_released(MouseButton::Left) && is_hover)
    {
        let mut next_state: ResourceRefMut<'_, NextState> = context.world.get_resource_mut::<NextState>().unwrap();
        next_state.0 = GameState::Running;

        let mut query: Query = Query::new(&context.world).with::<Text>();
        if let Some(entity) = query.entities_with_components().unwrap().first() {
            context.commands.despawn(entity.clone());
        }
    }

    if context.world.get_resource::<NextState>().unwrap().0 == GameState::Running {
        let mut player_query: Query = Query::new(&context.world).with::<Player>();
        let player_entity: Entity = player_query.entities_with_components().unwrap().first().unwrap().clone();

        let mut little_ball_query: Query = Query::new(&context.world).with::<LittleBall>();
        let little_ball_entity: Entity = little_ball_query.entities_with_components().unwrap().first().unwrap().clone();

        let mut thread_rng: ThreadRng = rand::rng();
        let random_factor: f32 = thread_rng.random_range(-0.5..0.5);

        move_player(context, input, player_entity);
        move_little_ball(context, little_ball_entity);
        check_player_little_ball_collision(context, player_entity, little_ball_entity, random_factor);
        check_little_ball_borders_collision(context, little_ball_entity, random_factor);
        check_litte_ball_targets_collision(context, little_ball_entity, random_factor);
        respawn_little_ball_after_outbounds(context, little_ball_entity);
    }
}

fn spawn_border(context: &mut Context, position: Vector2<f32>) {
    let border: Shape = Shape::new(Orientation::Vertical, GeometryType::Rectangle, Color::CYAN);

    context.commands.spawn(
        vec![
            Box::new(border),
            Box::new(Border()),
            Box::new(Transform::new(
                Position::new(position, Strategy::Normalized),
                0.0,
                Vector2::new(0.01, context.window_configuration.height as f32)
            )),
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
                    Box::new(Transform::new(
                        Position::new(Vector2::new(x, y), Strategy::Normalized),
                        0.0,
                        Vector2::new(width, height)
                    )),
                    Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle))),
                ]
            );
        }
    }
}

fn move_player(context: &mut Context, input: Input, player_entity: Entity) {
    let mut player_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut(&player_entity).unwrap();
    let player_velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component(&player_entity).unwrap();

    if input.is_key_pressed(KeyCode::ArrowRight) {
        let x: f32 = player_transform.position.x + player_velocity.x * context.delta;
        player_transform.set_position_x(&context.render_state, x);
    } else if input.is_key_pressed(KeyCode::ArrowLeft) {
        let x: f32 = player_transform.position.x - player_velocity.x * context.delta;
        player_transform.set_position_x(&context.render_state, x);
    }
}

fn move_little_ball(context: &mut Context, little_ball_entity: Entity) {
    let mut little_ball_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&little_ball_entity).unwrap();
    let little_ball_velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(&little_ball_entity).unwrap();

    let new_position: Vector2<f32> = little_ball_transform.position.to_vec() + little_ball_velocity.to_vec() * context.delta;
    little_ball_transform.set_position(&context.render_state, new_position);
}

fn check_player_little_ball_collision(context: &mut Context, player_entity: Entity, little_ball_entity: Entity, random_factor: f32) {
    let mut little_ball_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&little_ball_entity).unwrap();
    let mut little_ball_velocity: ComponentRefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(&little_ball_entity).unwrap();
    let little_ball_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&little_ball_entity).unwrap();

    let player_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&player_entity).unwrap();

    if Collision::check(CollisionAlgorithm::Aabb, &player_collision, &little_ball_collision) {
        let velocity_magnitude: f32 = little_ball_velocity.to_vec().magnitude();
        let collision_point: f32 = (
            (little_ball_collision.collider.position.x - player_collision.collider.position.x) / 
            (player_collision.collider.scale.x * 0.5)
        ).clamp(-1.0, 1.0);
        
        let mut new_direction: Vector2<f32> = Vector2::new(
            collision_point * 1.5,
            1.0 - collision_point.abs() * 0.3
        ).normalize();

        new_direction.x += random_factor * 0.15;
        new_direction = new_direction.normalize();

        little_ball_velocity.x = new_direction.x * velocity_magnitude;
        little_ball_velocity.y = new_direction.y * velocity_magnitude;
        little_ball_transform.position.y += 0.03;
    }
}

fn check_little_ball_borders_collision(context: &mut Context, little_ball_entity: Entity, random_factor: f32) {
    let mut border_query: Query = Query::new(&context.world).with::<Border>();
    let borders_entities: Vec<Entity> = border_query.entities_with_components().unwrap();

    let mut little_ball_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&little_ball_entity).unwrap();
    let mut little_ball_velocity: ComponentRefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(&little_ball_entity).unwrap();
    let little_ball_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&little_ball_entity).unwrap();

    for border in &borders_entities {
        let border_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(border).unwrap();

        if Collision::check(CollisionAlgorithm::Aabb, &little_ball_collision, &border_collision) {
            let velocity_magnitude: f32 = little_ball_velocity.to_vec().magnitude();
            let collision_normal: Vector2<f32> = if border_collision.collider.position.x > 0.0 {
                Vector2::new(-1.0, 0.0)
            } else {
                Vector2::new(1.0, 0.0)
            };

            let new_direction: Vector2<f32> = (
                little_ball_velocity.to_vec().normalize() - 2.0 *
                little_ball_velocity.to_vec().normalize().dot(collision_normal) * collision_normal
            ).normalize();
            
            let randomized_direction: Vector2<f32> = Vector2::new(
                new_direction.x + random_factor * 0.3,
                new_direction.y
            ).normalize();

            little_ball_velocity.x = randomized_direction.x * velocity_magnitude;
            little_ball_velocity.y = randomized_direction.y * velocity_magnitude;

            let collision_offset: Vector2<f32> = collision_normal * 0.02;
            little_ball_transform.position.x += collision_offset.x;
        }
    }
}

fn check_litte_ball_targets_collision(context: &mut Context, little_ball_entity: Entity, random_factor: f32) {
    let mut targets_query: Query = Query::new(&context.world).with::<Target>();
    let targets_entities: Vec<Entity> = targets_query.entities_with_components().unwrap();

    let mut little_ball_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&little_ball_entity).unwrap();
    let mut little_ball_velocity: ComponentRefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(&little_ball_entity).unwrap();
    let little_ball_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&little_ball_entity).unwrap();

    for target in &targets_entities {
        let target_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(target).unwrap();

        if Collision::check(CollisionAlgorithm::Aabb, &little_ball_collision, &target_collision) {
            let velocity_magnitude: f32 = little_ball_velocity.to_vec().magnitude();
            let impact_vector: Vector2<f32> = (target_collision.collider.position - little_ball_collision.collider.position).normalize();

            let mut new_direction: Vector2<f32> = Vector2::new(
                -impact_vector.x * 0.8 + random_factor * 0.2,
                -impact_vector.y * 0.8 + random_factor * 0.2
            ).normalize();

            new_direction.y = new_direction.y.signum() * new_direction.y.abs().max(0.3);

            little_ball_velocity.x = new_direction.x * velocity_magnitude;
            little_ball_velocity.y = new_direction.y * velocity_magnitude;

            little_ball_transform.position.x -= impact_vector.x * 0.05;
            little_ball_transform.position.y -= impact_vector.y * 0.05;
            context.commands.despawn(target.clone());
        }
    }
}

fn respawn_little_ball_after_outbounds(context: &mut Context, little_ball_entity: Entity) {
    let mut litte_ball_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&little_ball_entity).unwrap();
    let position_default: Vector2<f32> = Vector2::new(0.0, -0.25);

    if litte_ball_transform.position.y < -1.0 {
        let mut little_ball_respawn_timer: ResourceRefMut<'_, LittleBallRespawnTimer> = context.world.get_resource_mut::<LittleBallRespawnTimer>().unwrap();
        little_ball_respawn_timer.0.tick(context.delta);

        if little_ball_respawn_timer.0.is_finished() {
            litte_ball_transform.set_position(&context.render_state, position_default);
        }
    }
}
