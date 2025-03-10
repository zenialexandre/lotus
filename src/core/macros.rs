#[macro_export]
macro_rules! your_game {
    ($window_configuration:expr, $setup:ident, $update:ident) => {
        fn main() {
            pollster::block_on(crate::core::managers::windowing_manager::initialize_application(
                Some($window_configuration),
                $setup,
                $update
            ));
        }
    };
}
