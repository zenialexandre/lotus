pub mod window_manager;
pub mod rendering_manager;

fn main() {
    pollster::block_on(window_manager::open_default_window());
}
