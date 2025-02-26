use lotus::core::window_manager::open_default_window;

fn main() {
    pollster::block_on(open_default_window());
}
