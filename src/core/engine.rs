use super::{ecs::world::World, game_loop::GameLoopListener, managers::rendering_manager::RenderState};

pub struct EngineContext {
    pub render_state: RenderState,
    pub world: World,
    pub game_loop_listener: GameLoopListener,
    pub delta: f32
}

impl EngineContext {
    pub fn new(render_state: RenderState, world: World, delta: f32) -> Self {
        return Self {
            render_state,
            world,
            game_loop_listener: GameLoopListener::new(),
            delta
        };
    }
}
