use lotus::{
    *,
    core::{
        windowing_manager::*,
        rendering_manager::*,
        shape::*,
        sprite::*,
        color::*,
        transform::*
    }
};
use cgmath::{Deg, Matrix4, Vector2};

your_game!(
    WindowConfiguration {
        icon_path: "assets/textures/lotus_pink_256x256.png".to_string(),
        title: "Pong Game :)".to_string(),
        background_color: lotus::core::color::Color::WHITE,
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

fn setup(render_state: &mut RenderState) {
    //render_my_shape(render_state);
    render_my_sprite(render_state);
}

fn update(render_state: &mut RenderState, delta_time: f32) {
    let transform_matrix: Matrix4<f32> = Transform::new(Vector2::new(0.10, 0.25), Deg(0.), Vector2::new(1., 1.)).to_matrix();
    let transform_matrix_as_ref: &[[f32; 4]; 4] = transform_matrix.as_ref();

    render_state.queue.write_buffer(
        &render_state.transform_buffer.as_mut().unwrap(),
        0,
        bytemuck::cast_slice(&[*transform_matrix_as_ref])
    );
}

fn render_my_shape(render_state: &mut RenderState) {
    let triangle: Shape = Shape {
        geometry_type: GeometryType::Circle(Circle { radius: 0.5, number_of_segments: 32 }),
        color: Color::BLACK,
        orientation: Orientation::Horizontal,
        transform: Transform::new(Vector2::new(0.0, 0.0), Deg(0.), Vector2::new(0.0, 0.0))
    };
    render_shape(render_state, triangle);
}

fn render_my_sprite(render_state: &mut RenderState) {
    let sprite: Sprite = Sprite::new(
        "assets/textures/lotus_pink_256x256.png".to_string(),
        Transform::new(Vector2::new(0.10, 0.25), Deg(0.), Vector2::new(1., 1.))
    );
    render_sprite(render_state, sprite);
}
