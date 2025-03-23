use super::{
    managers::windowing_manager::WindowConfiguration,
    ecs::world::World,
    game_loop::GameLoopListener,
    managers::rendering_manager::RenderState
};

pub struct Context {
    pub render_state: RenderState,
    pub world: World,
    pub window_configuration: WindowConfiguration,
    pub game_loop_listener: GameLoopListener,
    pub delta: f32
}

impl Context {
    pub fn new(render_state: RenderState, world: World, window_configuration: WindowConfiguration, delta: f32) -> Self {
        return Self {
            render_state,
            world,
            window_configuration,
            game_loop_listener: GameLoopListener::new(),
            delta
        };
    }
}
