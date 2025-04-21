//! This is a more complex example of a more complete application.
//! The Pong arcade game, but with sprites.
//! This example shows off the usage of timer, audio, input and physics.
//! The timer works as a respawn for the pong ball after it goes outbounds.
//! The audio is used for the game music (streaming) and for the rackets hits (static).
//! The input is used for mapping the users keyboard actions.

use lotus_engine::*;
use rand::{rngs::ThreadRng, Rng};
use std::time::Duration;

#[derive(Component)]
struct Border();

#[derive(Component)]
struct Racket();

#[derive(Component)]
struct GrayRacket();

#[derive(Component)]
struct PinkRacket();

#[derive(Component)]
struct PongBall();

#[derive(Clone, Resource)]
pub struct PongBallRespawnTimer(pub Timer);

impl Default for PongBallRespawnTimer {
    fn default() -> Self {
        return Self(Timer::new(TimerType::Repeat, Duration::new(2, 0)))
    }
}

#[derive(Resource)]
pub struct GameAudio(pub AudioSource);

impl Default for GameAudio {
    fn default() -> Self {
        return Self(AudioSource::new().expect("Should create a audio source."));
    }
}

your_game!(
    WindowConfiguration {
        icon_path: "textures/pong/pink_racket_256x256.png".to_string(),
        title: "Pong Game :)".to_string(),
        background_color: None,
        background_image_path: Some("textures/pong/pong_background_960x600.png".to_string()),
        width: 960.0,
        height: 600.0,
        position_x: 200.0,
        position_y: 150.0,
        resizable: false,
        decorations: true,
        transparent: false,
        active: true,
        enabled_buttons: WindowButtons::CLOSE | WindowButtons::MINIMIZE
    },
    setup,
    update
);

fn setup(context: &mut Context) {
    let gray_racket_sprite: Sprite = Sprite::new("textures/pong/gray_racket_256x256.png".to_string());
    let pink_racket_sprite: Sprite = Sprite::new("textures/pong/pink_racket_256x256.png".to_string());
    let pong_ball_sprite: Sprite = Sprite::new("textures/pong/pong_ball_left_256x256.png".to_string());

    let mut game_audio: GameAudio = GameAudio::default();
    game_audio.0.load_streaming_sound(
        "game_music",
        "audio/pong/soundtrack/arcade_music.ogg",
        AudioSettings::default().loop_region(..).volume(Value::Fixed(Decibels(-10.0)))
    ).ok();
    game_audio.0.play_streaming_sound("game_music".to_string()).ok();

    game_audio.0.load_static_sound(
        "racket_hit",
        "audio/pong/effect/pong_hit.wav",
        AudioSettings::default()
    ).ok();

    context.commands.add_resources(vec![
        Box::new(PongBallRespawnTimer::default()),
        Box::new(game_audio)
    ]);

    spawn_border(context, Vector2::new(0.0, -1.0));
    spawn_border(context, Vector2::new(0.0, 1.0));

    context.commands.spawn(
        vec![
            Box::new(gray_racket_sprite),
            Box::new(Transform::new(
                Position::new(Vector2::new(-1.0, 0.23), Strategy::Normalized),
                0.0,
                Vector2::new(0.25, 0.25)
            )),
            Box::new(Racket()),
            Box::new(GrayRacket()),
            Box::new(Velocity::new(Vector2::new(1.5, 1.5))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Square)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(pink_racket_sprite),
            Box::new(Transform::new(
                Position::new(Vector2::new(1.0, 0.25), Strategy::Normalized),
                0.0,
                Vector2::new(0.25, 0.25)
            )),
            Box::new(Racket()),
            Box::new(PinkRacket()),
            Box::new(Velocity::new(Vector2::new(1.5, 1.5))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Square)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(pong_ball_sprite),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
                0.0,
                Vector2::new(0.25, 0.25)
            )),
            Box::new(PongBall()),
            Box::new(Velocity::new(Vector2::new(1.0, 1.0))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Square)))
        ]
    );
}

fn update(context: &mut Context) {
    let input: Input = {
        let input_ref: ResourceRefMut<'_, Input> = context.world.get_resource_mut::<Input>().unwrap();
        input_ref.clone()
    };

    let mut pong_ball_query: Query = Query::new(&context.world).with::<PongBall>();
    let pong_ball_entities: Vec<Entity> = pong_ball_query.entities_with_components().unwrap();
    let pong_ball: &Entity = pong_ball_entities.first().unwrap();
    let mut thread_rng: ThreadRng = rand::rng();
    let random_factor: f32 = thread_rng.random_range(-0.5..0.5);

    move_gray_racket(context, input.clone());
    move_pink_racket(context, input.clone());
    move_pong_ball(context, pong_ball);
    check_rackets_ball_collision(context, pong_ball, random_factor);
    check_borders_ball_collision(context, pong_ball, random_factor);
    respawn_pong_ball_after_outbounds(context, pong_ball);
}

fn spawn_border(context: &mut Context, position: Vector2<f32>) {
    let border: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK);

    context.commands.spawn(
        vec![
            Box::new(border),
            Box::new(Border()),
            Box::new(Transform::new(
                Position::new(position, Strategy::Normalized),
                0.0,
                Vector2::new(context.window_configuration.width as f32, 0.01)
            )),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
}

fn move_gray_racket(context: &mut Context, input: Input) {
    let mut query: Query = Query::new(&context.world).with::<GrayRacket>();
    let entities: Vec<Entity> = query.entities_with_components().unwrap();
    let gray_racket_entity: &Entity = entities.first().unwrap();

    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(gray_racket_entity).unwrap();
    let velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(gray_racket_entity).unwrap();

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyW)) {
        transform.position.y += velocity.y * context.delta;
        let new_position: Vector2<f32> = Vector2::new(transform.position.x, transform.position.y);
        transform.set_position(&context.render_state, new_position);
    } else if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyS)) {
        transform.position.y -= velocity.y * context.delta;
        let new_position: Vector2<f32> = Vector2::new(transform.position.x, transform.position.y);
        transform.set_position(&context.render_state, new_position);
    }
}

fn move_pink_racket(context: &mut Context, input: Input) {
    let mut query: Query = Query::new(&context.world).with::<PinkRacket>();
    let entities: Vec<Entity> = query.entities_with_components().unwrap();
    let pink_racket_entity: &Entity = entities.first().unwrap();

    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(pink_racket_entity).unwrap();
    let velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(pink_racket_entity).unwrap();
    
    if input.is_key_pressed(PhysicalKey::Code(KeyCode::ArrowUp)) {
        transform.position.y += velocity.y * context.delta;
        let new_position: Vector2<f32> = Vector2::new(transform.position.x, transform.position.y);
        transform.set_position(&context.render_state, new_position);
    } else if input.is_key_pressed(PhysicalKey::Code(KeyCode::ArrowDown)) {
        transform.position.y -= velocity.y * context.delta;
        let new_position: Vector2<f32> = Vector2::new(transform.position.x, transform.position.y);
        transform.set_position(&context.render_state, new_position);
    }
}

fn move_pong_ball(context: &mut Context, pong_ball: &Entity) {
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&pong_ball).unwrap();
    let velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(&pong_ball).unwrap();

    let new_position: Vector2<f32> = transform.position.to_vec() + velocity.to_vec() * context.delta;
    transform.set_position(&context.render_state, new_position);
}

fn check_rackets_ball_collision(context: &mut Context, pong_ball: &Entity, random_factor: f32) {
    let mut racket_query: Query = Query::new(&context.world).with::<Racket>();
    let rackets: Vec<Entity> = racket_query.entities_with_components().unwrap();
    let mut game_audio: ResourceRefMut<'_, GameAudio> = context.world.get_resource_mut::<GameAudio>().unwrap();

    for racket in &rackets {
        let racket_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(racket).unwrap();
        let racket_transform: ComponentRef<'_, Transform> = context.world.get_entity_component::<Transform>(racket).unwrap();

        let pong_ball_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&pong_ball).unwrap();
        let mut pong_ball_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&pong_ball).unwrap();
        let mut pong_ball_velocity: ComponentRefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(&pong_ball).unwrap();

        if Collision::check(CollisionAlgorithm::Aabb, &racket_collision, &pong_ball_collision) {
            game_audio.0.play_static_sound("racket_hit".to_string()).ok();

            let relative_collision_point: f32 = pong_ball_transform.position.y - racket_transform.position.y;
            let rebound_angle: f32 = relative_collision_point * 1.0 + random_factor;

            let pong_ball_new_velocity: Vector2<f32>;

            if racket_transform.position.x > 0.0 {
                pong_ball_new_velocity = Vector2::new(-1.0, rebound_angle).normalize() * pong_ball_velocity.to_vec().magnitude();
                pong_ball_velocity.x = pong_ball_new_velocity.x; pong_ball_velocity.y = pong_ball_new_velocity.y;
                pong_ball_transform.position.x -= 0.1;
            } else if racket_transform.position.x < 0.0 {
                pong_ball_new_velocity = Vector2::new(1.0, rebound_angle).normalize() * pong_ball_velocity.to_vec().magnitude();
                pong_ball_velocity.x = pong_ball_new_velocity.x; pong_ball_velocity.y = pong_ball_new_velocity.y;
                pong_ball_transform.position.x += 0.1;
            }
            let new_position: Vector2<f32> = Vector2::new(pong_ball_transform.position.x, pong_ball_transform.position.y);
            pong_ball_transform.set_position(&context.render_state, new_position);
        }
    }
}

fn check_borders_ball_collision(context: &mut Context, pong_ball: &Entity, random_factor: f32) {
    let mut border_query: Query = Query::new(&context.world).with::<Border>();
    let borders: Vec<Entity> = border_query.entities_with_components().unwrap();

    for border in &borders {
        let border_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(border).unwrap();
        let border_transform: ComponentRef<'_, Transform> = context.world.get_entity_component::<Transform>(border).unwrap();

        let pong_ball_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&pong_ball).unwrap();
        let mut pong_ball_velocity: ComponentRefMut<'_, Velocity> = context.world.get_entity_component_mut::<Velocity>(&pong_ball).unwrap();
        let mut pong_ball_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&pong_ball).unwrap();

        let pong_ball_new_velocity: Vector2<f32>;

        if Collision::check(CollisionAlgorithm::Aabb, &border_collision, &pong_ball_collision) {
            if border_transform.position.y > 0.0 {
                pong_ball_new_velocity = Vector2::new(pong_ball_velocity.x.signum(), -1.0 + random_factor).normalize() * pong_ball_velocity.to_vec().magnitude();
                pong_ball_velocity.x = pong_ball_new_velocity.x; pong_ball_velocity.y = pong_ball_new_velocity.y;
                pong_ball_transform.position.y -= 0.1;
            } else if border_transform.position.y < 0.0 {
                pong_ball_new_velocity = Vector2::new(pong_ball_velocity.x.signum(), 1.0 + random_factor).normalize() * pong_ball_velocity.to_vec().magnitude();
                pong_ball_velocity.x = pong_ball_new_velocity.x; pong_ball_velocity.y = pong_ball_new_velocity.y;
                pong_ball_transform.position.y += 0.1;
            }
            let new_position: Vector2<f32> = Vector2::new(pong_ball_transform.position.x, pong_ball_transform.position.y);
            pong_ball_transform.set_position(&context.render_state, new_position);
        }
    }
}

fn respawn_pong_ball_after_outbounds(context: &mut Context, pong_ball: &Entity) {
    let mut pong_ball_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(pong_ball).unwrap();
    let position_default: Vector2<f32> = Vector2::new(0.0, 0.0);

    if pong_ball_transform.position.x > 2.0 || pong_ball_transform.position.x < -2.0 {
        let mut pong_ball_respawn_timer: ResourceRefMut<'_, PongBallRespawnTimer> = context.world.get_resource_mut::<PongBallRespawnTimer>().unwrap();
        pong_ball_respawn_timer.0.tick(context.delta);

        if pong_ball_respawn_timer.0.is_finished() {
            pong_ball_transform.set_position(&context.render_state, position_default);
        }
    }
}
