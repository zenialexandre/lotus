use lotus::*;
use cgmath::{Matrix4, Vector2};
use std::cell::RefCell;

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
    let sprite: Sprite = Sprite::new(
        "assets/textures/lotus_pink_256x256.png".to_string(),
    );
    render_sprite(&mut engine_context.render_state, sprite.clone()); // This should be moved to the spawn

    engine_context.world.spawn(
        &mut vec![
            RefCell::new(Box::new(sprite)),
            RefCell::new(Box::new(Transform::new(Vector2::new(0.10, 0.25), 0., Vector2::new(1., 1.))))
        ]
    );
}

fn update(engine_context: &mut EngineContext) {
    let mut query: Query = Query::new(&engine_context.world)
        .with_components::<Sprite>()
        .with_components::<Transform>();

    let (_entity, mut components) = query.get_entity_by_components_mut().unwrap();

    for component in &mut components {
        if let Some(transform) = component.as_any_mut().downcast_mut::<Transform>() {
            transform.rotation += 100. * engine_context.delta;

            let transform_matrix: Matrix4<f32> = transform.to_matrix();
            let transform_matrix_as_ref: &[[f32; 4]; 4] = transform_matrix.as_ref();

            engine_context.render_state.queue.write_buffer(
                &engine_context.render_state.transform_buffer.as_mut().unwrap(),
                0,
                bytemuck::cast_slice(&[*transform_matrix_as_ref])
            );
        }
    }
}
