use lotus_engine::*;
use cgmath::Vector2;
use winit::keyboard::{KeyCode, PhysicalKey};
use std::cell::{RefCell, RefMut};

#[derive(Component)]
struct GrayRacket();

#[derive(Component)]
struct PinkRacket();

#[derive(Component)]
struct PongBall();

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
    let pong_ball_sprite: Sprite = Sprite::new("assets/textures/pong/pong_ball_left_256x256.png".to_string());

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(gray_racket_sprite)),
            RefCell::new(Box::new(Transform::new(Vector2::new(-0.90, 0.23), 0., Vector2::new(0.25, 0.25)))),
            RefCell::new(Box::new(GrayRacket())),
            RefCell::new(Box::new(Velocity::new(Vector2::new(0., 0.50)))),
            RefCell::new(Box::new(Collision::new(
                Collider::new(GeometryType::Rectangle, Vector2::new(-0.80, 0.23), Vector2::new(1., 1.))
            )))
        ]
    );

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(pink_racket_sprite)),
            RefCell::new(Box::new(Transform::new(Vector2::new(0.90, 0.25), 0., Vector2::new(0.25, 0.25)))),
            RefCell::new(Box::new(PinkRacket())),
            RefCell::new(Box::new(Velocity::new(Vector2::new(0., 0.50)))),
            RefCell::new(Box::new(Collision::new(
                Collider::new(GeometryType::Rectangle, Vector2::new(0.80, 0.25), Vector2::new(1., 1.))
            )))
        ]
    );

    engine_context.world.spawn(
        &mut engine_context.render_state,
        &mut vec![
            RefCell::new(Box::new(pong_ball_sprite)),
            RefCell::new(Box::new(Transform::new(Vector2::new(0., 0.), 0., Vector2::new(0.25, 0.25)))),
            RefCell::new(Box::new(PongBall())),
            RefCell::new(Box::new(Collision::new(
                Collider::new(GeometryType::Rectangle, Vector2::new(0., 0.), Vector2::new(1., 1.))
            ))),
            RefCell::new(Box::new(Velocity::new(Vector2::new(100., 50.)))),
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
    //apply_velocity(engine_context);
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
    let mut query: Query = Query::new(&engine_context.world).with_components::<GrayRacket>();
    let entities: Vec<Entity> = query.get_entities_ids_flex().unwrap();
    let gray_racket_entity: &Entity = entities.first().unwrap();

    let mut transform: RefMut<'_, Transform> = engine_context.world.get_entity_component_mut::<Transform>(gray_racket_entity).unwrap();
    let velocity: RefMut<'_, Velocity> = engine_context.world.get_entity_component_mut::<Velocity>(gray_racket_entity).unwrap();
    let position: Vector2<f32> = transform.get_position();

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyW)) {
        let new_y: f32 = position.y + velocity.value.y * engine_context.delta;
        transform.set_position(&engine_context, Vector2::new(position.x, new_y));
    } else if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyS)) {
        let new_y: f32 = position.y - velocity.value.y * engine_context.delta;
        transform.set_position(&engine_context, Vector2::new(position.x, new_y));
    }
}

fn move_pink_racket(engine_context: &mut EngineContext, input: &Input) {
    let mut query: Query = Query::new(&engine_context.world).with_components::<PinkRacket>();
    let results: Vec<(Entity, Vec<std::cell::RefMut<'_, Box<dyn Component>>>)> = query.get_entities_by_components_mut_flex().unwrap();

    for result in results {
        let (_, mut components) = result;

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

fn apply_velocity(engine_context: &mut EngineContext) {
    let mut query: Query = Query::new(&engine_context.world).with_components::<PongBall>();
    let results: Vec<(Entity, Vec<std::cell::RefMut<'_, Box<dyn Component>>>)> = query.get_entities_by_components_mut_flex().unwrap();

    for result in results {
        let (_, mut components) = result;

        let mut transform_index = None;
        let mut velocity_index = None;

        // Encontrar índices dos componentes na lista
        for (i, component) in components.iter().enumerate() {
            if component.as_any().is::<Transform>() {
                transform_index = Some(i);
            }
            if component.as_any().is::<Velocity>() {
                velocity_index = Some(i);
            }
        }

        if let (Some(t_idx), Some(v_idx)) = (transform_index, velocity_index) {
            let mut transform = components.swap_remove(t_idx);
            let mut velocity = components.swap_remove(v_idx - (t_idx < v_idx) as usize); // Ajustar índice se removermos antes

            let transform = transform.as_any_mut().downcast_mut::<Transform>().unwrap();
            let velocity = velocity.as_any_mut().downcast_mut::<Velocity>().unwrap();

            // Aplicar gravidade na velocidade Y
            velocity.value.y -= 100. * engine_context.delta.clamp(0.0, 0.016);

            // Atualizar posição da bola com base na velocidade
            let new_pos = transform.position + velocity.value * engine_context.delta.clamp(0.0, 0.016);
            transform.set_position(&engine_context, new_pos);
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

    if Collision::check(CollisionAlgorithm::Aabb, &collisions[0], &collisions[1]) {
        //if let Some(velocity) = engine_context.world.get_entity_component_mut::<Velocity>(collisions)
    }
}
