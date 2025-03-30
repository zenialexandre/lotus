use super::{
    managers::windowing_manager::WindowConfiguration,
    ecs::world::{World, Commands},
    managers::rendering_manager::RenderState
};

/// Struct created to provide the main features of the engine for the end-user.
pub struct Context {
    /// The actual state of rendering within the engine.
    pub render_state: RenderState,
    /// The actual world at its current state.
    /// Use for reading the worlds entities, componenents and resources.
    /// Can be used for adding new resources too.
    pub world: World,
    /// Mutable commands for the world.
    /// Use for mutating the world entities, components and resources.
    pub commands: Commands,
    /// The window configuration data for reading purposes.
    pub window_configuration: WindowConfiguration,
    /// The delta time for reading purposes.
    pub delta: f32
}

impl Context {
    /// Create a new context with parameters.
    pub fn new(render_state: RenderState, world: World, window_configuration: WindowConfiguration, delta: f32) -> Self {
        return Self {
            render_state,
            world,
            window_configuration,
            commands: Commands::new(),
            delta
        };
    }
}
