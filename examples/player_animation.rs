//! This example is a show off about the animation system of the engine in a more complex way.
//! The entity in this case doesn't have the sprite component.
//! It has three different sprite sheets to animate.
//! The player is in a constant animation called 'idle'.
//! The player can be animated by movement or by pressing a specific keyboard key for attacking.

use lotus_engine::*;
use std::{collections::HashMap, time::Duration};

your_game!(
    WindowConfiguration::default()
        .title("Player Animation".to_string())
        .background_color(Some(Color::LIGHTGRAY)),
    setup,
    update
);

fn setup(context: &mut Context) {
    context.game_loop_listener.fps_cap(120);

    let idle: SpriteSheet = SpriteSheet::new(
        "textures/animations/player/idle.png".to_string(),
        Transform::default(),
        Timer::new(TimerType::Repeat, Duration::from_secs_f32(0.1)),
        (512, 512),
        1,
        10,
        (0..=9).collect()
    );
    let attack: SpriteSheet = SpriteSheet::new(
        "textures/animations/player/attack.png".to_string(),
        Transform::default(),
        Timer::new(TimerType::Repeat, Duration::from_secs_f32(0.1)),
        (512, 512),
        1,
        7,
        (0..=6).collect()
    );
    let walk: SpriteSheet = SpriteSheet::new(
        "textures/animations/player/walk.png".to_string(),
        Transform::default(),
        Timer::new(TimerType::Repeat, Duration::from_secs_f32(0.1)),
        (512, 512),
        1,
        16,
        (0..=15).collect()
    );

    let mut my_animations: HashMap<String, SpriteSheet> = HashMap::new();
    my_animations.insert("idle".to_string(), idle);
    my_animations.insert("attack".to_string(), attack);
    my_animations.insert("walk".to_string(), walk);

    let mut animation: Animation = Animation::new(my_animations);
    animation.play("idle".to_string());

    context.commands.spawn(vec![
        Box::new(animation),
        Box::new(Velocity::new(Vector2::new(0.5, 0.5)))
    ]);
}

fn update(context: &mut Context) {
    context.commands.show_fps(context.game_loop_listener.current_fps, Color::BLACK);

    let input: Input = {
        let input_ref: ResourceRefMut<'_, Input> = context.world.get_resource_mut::<Input>().unwrap();
        input_ref.clone()
    };
    move_player(context, input.clone());
    attack(context, input);
}

fn move_player(context: &mut Context, input: Input) {
    let mut query: Query = Query::new(&context.world).with::<Animation>();
    let result: Entity = query.entities_with_components().unwrap().first().unwrap().clone();
    let mut animation: ComponentRefMut<'_, Animation> = context.world.get_entity_component_mut::<Animation>(&result).unwrap();
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&result).unwrap();
    let velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(&result).unwrap();

    if input.is_key_pressed(KeyCode::KeyW) {
        let y: f32 = transform.position.y + velocity.y * context.delta;
        transform.set_position_y(&context.render_state, y);
    } else if input.is_key_pressed(KeyCode::KeyS) {
        let y: f32 = transform.position.y - velocity.y * context.delta;
        transform.set_position_y(&context.render_state, y);
    } else if input.is_key_pressed(KeyCode::KeyD) {
        let x: f32 = transform.position.x + velocity.x * context.delta;
        transform.set_position_x(&context.render_state, x);
    } else if input.is_key_pressed(KeyCode::KeyA) {
        let x: f32 = transform.position.x - velocity.x * context.delta;
        transform.set_position_x(&context.render_state, x);
    }

    if input.is_some_of_keys_pressed(vec![KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD]) {
        animation.play("walk".to_string());
    }

    if input.is_some_of_keys_released(vec![KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD]) {
        animation.stop("walk".to_string());
    }
}

fn attack(context: &mut Context, input: Input) {
    let mut query: Query = Query::new(&context.world).with::<Animation>();
    let result: Entity = query.entities_with_components().unwrap().first().unwrap().clone();
    let mut animation: ComponentRefMut<'_, Animation> = context.world.get_entity_component_mut::<Animation>(&result).unwrap();

    if input.is_key_pressed(KeyCode::Space) {
        animation.play("attack".to_string());
    }

    if input.is_key_released(KeyCode::Space) {
        animation.stop("attack".to_string());
    }
}
