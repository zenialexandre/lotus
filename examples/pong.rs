use lotus::{
    *,
    core::{
        windowing_manager::*,
        rendering_manager::*,
        shape::*,
        color::*,
        transform::*
    }
};
use cgmath::{Matrix4, Deg, Vector2};

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
    let triangle: Shape = Shape {
        geometry_type: GeometryType::Triangle,
        color: Color::BLACK,
        orientation: Orientation::Horizontal
    };
    render_shape(render_state, triangle);
}

fn update(render_state: &mut RenderState, delta_time: f32) {
    /*let temp_position: Vector2<f32> = Vector2 { x: 0.10, y: 0.25};

    let transform_matrix: Matrix4<f32> = Transform::new(
        temp_position,
        Deg(0.0),
        Vector2::new(1., 1.)
    ).to_matrix();
    let transform_matrix_as_ref: &[[f32; 4]; 4] = transform_matrix.as_ref();

    render_state.queue.write_buffer(
        &render_state.transform_buffer.as_mut().unwrap(),
        0,
        bytemuck::cast_slice(&[*transform_matrix_as_ref])
    );*/
}
