use lotus::*;
use cgmath::Vector2;
use winit::keyboard::{KeyCode, PhysicalKey};
use std::cell::RefCell;

#[derive(Component)]
struct LeftRacket();

impl LeftRacket {
    fn new() -> Self {
        return Self();
    }
}

#[derive(Component)]
struct RightRacket();

impl RightRacket {
    fn new() -> Self {
        return Self();
    }
}

your_game!(
    WindowConfiguration {
        icon_path: "assets/textures/lotus_pink_256x256.png".to_string(),
        title: "Pong Game :)".to_string(),
        background_color: None,
        background_image_path: Some("assets/textures/pong/pong_background_2560x1600.png".to_string()),
        width: 800.,
        height: 600.,
        position_x: 200.,
        position_y: 150.,
        resizable: false,
        decorations: true,
        transparent: false,
        visible: true,
        enabled_buttons: winit::window::WindowButtons::CLOSE | winit::window::WindowButtons::MINIMIZE
    },
    setup,
    update
);

fn setup(engine_context: &mut EngineContext) {
    let left_racket_sprite: Sprite = Sprite::new("assets/textures/pong/gray_racket_256x256.png".to_string());
    let right_racket_sprite: Sprite = Sprite::new("assets/textures/pong/pink_racket_256x256.png".to_string());

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(left_racket_sprite)),
            RefCell::new(Box::new(Transform::new(Vector2::new(-0.80, 0.23), 0., Vector2::new(0.25, 0.25)))),
            RefCell::new(Box::new(LeftRacket::new()))
        ]
    );

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(right_racket_sprite)),
            RefCell::new(Box::new(Transform::new(Vector2::new(0.80, 0.25), 0., Vector2::new(0.25, 0.25)))),
            RefCell::new(Box::new(RightRacket::new()))
        ]
    );
}

fn update(engine_context: &mut EngineContext) {
    let mut first_sprite_query: Query = Query::new(&engine_context.world)
        .with_components::<LeftRacket>()
        .with_components::<Sprite>()
        .with_components::<Transform>();
    let first_sprite_query_results = first_sprite_query.get_entities_by_components_mut().unwrap();

    let mut second_sprite_query: Query = Query::new(&engine_context.world)
        .with_components::<RightRacket>()
        .with_components::<Sprite>()
        .with_components::<Transform>();
    let second_sprite_query_results = second_sprite_query.get_entities_by_components_mut().unwrap();

    for first_sprite_query_result in first_sprite_query_results {
        let (_entity, mut components) = first_sprite_query_result;

        for component in &mut components {
            if let Some(transform) = component.as_any_mut().downcast_mut::<Transform>() {
                let input: &Input = engine_context.world.resources.iter().filter_map(
                    |resource| resource.as_any().downcast_ref::<Input>()
                ).next().unwrap();
    
                if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyW)) {
                    let y: f32 = transform.position.y + 0.35 * engine_context.delta;
                    transform.set_position(&engine_context, Vector2::new(transform.position.x, y));
                } else if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyS)) {
                    let y: f32 = transform.position.y - 0.35 * engine_context.delta;
                    transform.set_position(&engine_context, Vector2::new(transform.position.x, y));
                }
            }
        }
    }

    for second_sprite_query_result in second_sprite_query_results {
        let (_entity, mut components) = second_sprite_query_result;

        for component in &mut components {
            if let Some(transform) = component.as_any_mut().downcast_mut::<Transform>() {
                let input: &Input = engine_context.world.resources.iter().filter_map(
                    |resource| resource.as_any().downcast_ref::<Input>()
                ).next().unwrap();
    
                if input.is_key_pressed(PhysicalKey::Code(KeyCode::ArrowUp)) {
                    let y: f32 = transform.position.y + 0.35 * engine_context.delta;
                    transform.set_position(&engine_context, Vector2::new(transform.position.x, y));
                } else if input.is_key_pressed(PhysicalKey::Code(KeyCode::ArrowDown)) {
                    let y: f32 = transform.position.y - 0.35 * engine_context.delta;
                    transform.set_position(&engine_context, Vector2::new(transform.position.x, y));
                }
            }
        }
    }
}
