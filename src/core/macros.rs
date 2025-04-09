/// # The your_game! macro.
/// Macro created to facilitate the game programming using the engine.<br>
/// If you do something like:
/// ```
/// use lotus_engine::*;
/// 
/// your_game!(WindowConfiguration::default(), setup, update);
/// 
/// fn setup(context: &mut Context) {}
/// 
/// fn update(context: &mut Context) {}
/// ```
/// You will already have a game running!
#[macro_export]
macro_rules! your_game {
    ($window_configuration:expr, $setup:ident, $update:ident) => {
        fn main() {
            ::lotus_engine::block_on(::lotus_engine::core::managers::windowing_manager::initialize_application(
                Some($window_configuration),
                $setup,
                $update
            ));
        }
    };
}
