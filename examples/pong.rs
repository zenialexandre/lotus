use lotus_engine::*;
use cgmath::Vector2;
use winit::keyboard::{KeyCode, PhysicalKey};
use std::cell::RefCell;

#[derive(Component)]
struct GrayRacket();

#[derive(Component)]
struct PinkRacket();

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
        active: true,
        enabled_buttons: winit::window::WindowButtons::CLOSE | winit::window::WindowButtons::MINIMIZE
    },
    setup,
    update
);

fn setup(engine_context: &mut EngineContext) {
    let gray_racket_sprite: Sprite = Sprite::new("assets/textures/pong/gray_racket_256x256.png".to_string());
    let pink_racket_sprite: Sprite = Sprite::new("assets/textures/pong/pink_racket_256x256.png".to_string());

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(gray_racket_sprite)),
            RefCell::new(Box::new(Transform::new(Vector2::new(-0.80, 0.23), 0., Vector2::new(0.25, 0.25)))),
            RefCell::new(Box::new(GrayRacket())),
            RefCell::new(Box::new(Collision::new(
                CollisionAlgorithm::Aabb,
                Collider::new(GeometryType::Rectangle, Vector2::new(-0.80, 0.23), Vector2::new(1., 1.))
            )))
        ]
    );

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(pink_racket_sprite)),
            RefCell::new(Box::new(Transform::new(Vector2::new(0.80, 0.25), 0., Vector2::new(0.25, 0.25)))),
            RefCell::new(Box::new(PinkRacket())),
            RefCell::new(Box::new(Collision::new(
                CollisionAlgorithm::Aabb,
                Collider::new(GeometryType::Rectangle, Vector2::new(0.80, 0.25), Vector2::new(1., 1.))
            )))
        ]
    );
}

fn update(engine_context: &mut EngineContext) {
    let input: Input = {
        let resources: &Vec<Box<dyn Resource>> = &engine_context.world.resources;
        resources.iter().filter_map(|resource| resource.as_any().downcast_ref::<Input>()).next().cloned().unwrap()
    };

    move_gray_racket(engine_context, &input);
    move_pink_racket(engine_context, &input);
    check_for_collision(engine_context);

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::Escape)) {
        if GameLoopState::Running == engine_context.game_loop_listener.state {
            engine_context.game_loop_listener.pause();
        } else {
            engine_context.game_loop_listener.resume();
        }
    }
}

fn move_gray_racket(engine_context: &mut EngineContext, input: &Input) {
    let mut first_sprite_query: Query = Query::new(&engine_context.world).with_components::<GrayRacket>();
    let first_sprite_query_results: Vec<(Entity, Vec<std::cell::RefMut<'_, Box<dyn Component>>>)> = first_sprite_query.get_entities_by_components_mut_flex().unwrap();

    for first_sprite_query_result in first_sprite_query_results {
        let (_, mut components) = first_sprite_query_result;

        for component in &mut components {
            if let Some(transform) = component.as_any_mut().downcast_mut::<Transform>() {
                if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyW)) {
                    let y: f32 = transform.position.y + 0.35 * engine_context.delta;
                    transform.set_position(&engine_context, Vector2::new(transform.position.x, y));
                } else if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyS)) {
                    let y: f32 = transform.position.y - 0.35 * engine_context.delta;
                    transform.set_position(&engine_context, Vector2::new(transform.position.x, y));
                } else if input.is_key_pressed(PhysicalKey::Code(KeyCode::ArrowRight)) {
                    let x: f32 = transform.position.x + 0.35 * engine_context.delta;
                    transform.set_position(&engine_context, Vector2::new(x, transform.position.y));
                }
            }
        }
    }
}

fn move_pink_racket(engine_context: &mut EngineContext, input: &Input) {
    let mut second_sprite_query: Query = Query::new(&engine_context.world).with_components::<PinkRacket>();
    let second_sprite_query_results: Vec<(Entity, Vec<std::cell::RefMut<'_, Box<dyn Component>>>)> = second_sprite_query.get_entities_by_components_mut_flex().unwrap();

    for second_sprite_query_result in second_sprite_query_results {
        let (_, mut components) = second_sprite_query_result;

        for component in &mut components {
            if let Some(transform) = component.as_any_mut().downcast_mut::<Transform>() {
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

fn check_for_collision(engine_context: &mut EngineContext) {
    let mut query: Query = Query::new(&engine_context.world).with_components::<Collision>();
    let results: Vec<(Entity, Vec<std::cell::RefMut<'_, Box<dyn Component>>>)> = query.get_all_entities_by_componenets_mut_flex().unwrap();

    let mut collisions: Vec<Collision> = vec![];

    for result in results {
        let (_, components) = result;

        for component in &components {
            if let Some(collision) = component.as_any().downcast_ref::<Collision>() {
                collisions.push(collision.clone());
            }
        }
    }

    let is_colliding: bool = collisions[0].algorithm.check(collisions[0].collider.clone(), collisions[1].collider.clone());

    if is_colliding {
        eprintln!("toma!");
    }
}
