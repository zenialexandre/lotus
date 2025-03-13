use lotus::*;
use cgmath::Vector2;
use std::cell::RefCell;

#[derive(Component)]
struct FirstSprite();

impl FirstSprite {
    fn new() -> Self {
        return Self();
    }
}

#[derive(Component)]
struct SecondSprite();

impl SecondSprite {
    fn new() -> Self {
        return Self();
    }
}

your_game!(
    WindowConfiguration {
        icon_path: "assets/textures/lotus_pink_256x256.png".to_string(),
        title: "Pong Game :)".to_string(),
        background_color: lotus::core::color::Color::WHITE,
        width: 800.,
        height: 600.,
        position_x: 200.,
        position_y: 150.,
        resizable: true,
        decorations: true,
        transparent: false,
        visible: true,
        enabled_buttons: winit::window::WindowButtons::all()
    },
    setup,
    update
);

fn setup(engine_context: &mut EngineContext) {
    let sprite: Sprite = Sprite::new("assets/textures/lotus_pink_256x256.png".to_string());
    let sprite2: Sprite = Sprite::new("assets/textures/lotus_pink_256x256.png".to_string());

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(sprite)),
            RefCell::new(Box::new(Transform::new(Vector2::new(0.10, 0.60), 0., Vector2::new(1., 1.)))),
            RefCell::new(Box::new(FirstSprite::new()))
        ]
    );

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(sprite2)),
            RefCell::new(Box::new(Transform::new(Vector2::new(0.10, 0.25), 0., Vector2::new(1., 1.)))),
            RefCell::new(Box::new(SecondSprite::new()))
        ]
    );
}

fn update(engine_context: &mut EngineContext) {
    let mut first_sprite_query: Query = Query::new(&engine_context.world)
        .with_components::<FirstSprite>()
        .with_components::<Sprite>()
        .with_components::<Transform>();
    let first_sprite_query_results = first_sprite_query.get_entities_by_components_mut().unwrap();

    let mut second_sprite_query: Query = Query::new(&engine_context.world)
        .with_components::<SecondSprite>()
        .with_components::<Sprite>()
        .with_components::<Transform>();
    let second_sprite_query_results = second_sprite_query.get_entities_by_components_mut().unwrap();

    for first_sprite_query_result in first_sprite_query_results {
        let (_entity, mut components) = first_sprite_query_result;

        for component in &mut components {
            if let Some(transform) = component.as_any_mut().downcast_mut::<Transform>() {
                let mut rotation: f32 = transform.get_rotation();
                rotation += 100. * engine_context.delta;
                transform.set_rotation(&engine_context, rotation);
                transform.set_scale(&engine_context, Vector2::new(10. * engine_context.delta, 10. * engine_context.delta));
            }
        }
    }

    for second_sprite_query_result in second_sprite_query_results {
        let (_entity, mut components) = second_sprite_query_result;

        for component in &mut components {
            if let Some(transform) = component.as_any_mut().downcast_mut::<Transform>() {
                let mut rotation: f32 = transform.get_rotation();
                rotation += -100. * engine_context.delta;
                transform.set_rotation(&engine_context, rotation);
                transform.set_scale(&engine_context, Vector2::new(10. * engine_context.delta, 10. * engine_context.delta));
            }
        }
    }
}
