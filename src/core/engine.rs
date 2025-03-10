use super::{ecs::world::World, managers::rendering_manager::RenderState};

pub struct EngineContext {
    pub render_state: RenderState,
    pub world: World,
    pub delta: f32
}

impl EngineContext {
    pub fn new(render_state: RenderState, world: World, delta: f32) -> Self {
        return Self {
            render_state,
            world,
            delta
        };
    }
}
