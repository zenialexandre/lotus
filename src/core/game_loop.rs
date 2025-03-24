use std::time::{Duration, Instant};
use lotus_proc_macros::Resource;
use wgpu::SurfaceError;
use winit::event_loop::ActiveEventLoop;

use super::{
    engine::Context,
    managers::rendering_manager::RenderState,
    ecs::world::World
};

/// Enumerator to store the engine current state.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum GameLoopState {
    #[default]
    Running,
    Paused
}

/// Struct to store the engine loop data.
pub struct GameLoop {
    pub delta: Duration,
    pub previous_time_of_last_run: Instant,
    pub setup: fn(context: &mut Context),
    pub update: fn(context: &mut Context),
}

/// Struct to update the current engine state as the end-user.
#[derive(Clone, Debug, Resource)]
pub struct GameLoopListener {
    pub state: GameLoopState
}

impl GameLoop {
    /// Create a new loop with parameters.
    pub fn new(setup: fn(context: &mut Context), update: fn(context: &mut Context)) -> Self {
        return Self {
            delta: Duration::from_secs_f32(1.0 / 60.0),
            previous_time_of_last_run: Instant::now(),
            setup,
            update
        };
    }

    /// Run the engine loop to start the logic and rendering processes.
    pub fn run(&mut self, context: &mut Context, event_loop: &ActiveEventLoop) {
        let now: Instant = Instant::now();
        let elapsed_time_from_last_run: Duration = now - self.previous_time_of_last_run;
        self.previous_time_of_last_run = now;
        let mut accumulator: f32 = elapsed_time_from_last_run.as_secs_f32();

        while accumulator >= self.get_delta_as_seconds() {
            context.delta = self.get_delta_as_seconds();
            (self.update)(context);
            context.world.sync_transformations_with_collisions();
            accumulator -= context.delta;
        }
        self.render(&mut context.render_state, &context.world, event_loop);
    }

    /// Call the rendering process.
    pub fn render(&self, render_state: &mut RenderState, world: &World, event_loop: &ActiveEventLoop) {
        match render_state.render(world) {
            Ok(_) => {}

            Err(
                SurfaceError::Lost | SurfaceError::Outdated
            ) => render_state.resize(render_state.physical_size),

            Err(
                SurfaceError::OutOfMemory | SurfaceError::Other
            ) => {
                log::error!("Application OOMKilled.");
                event_loop.exit();
            }

            Err(SurfaceError::Timeout) => {
                log::warn!("Surface Timeout.")
            }
        }
    }

    /// Returns delta as a Duration struct.
    pub fn get_delta(&self) -> Duration {
        return self.delta;
    }

    /// Returns delta as seconds on the f32 format. 
    pub fn get_delta_as_seconds(&self) -> f32 {
        return self.delta.as_secs_f32();
    }
}

impl GameLoopListener {
    /// Create a new loop listener.
    pub fn new() -> Self {
        return Self { state: GameLoopState::Running };
    }

    /// Update the current loop status to Paused.
    pub fn pause(&mut self) {
        self.state = GameLoopState::Paused;
    }

    /// Update the current loop status to Running.
    pub fn resume(&mut self) {
        self.state = GameLoopState::Running;
    }
}
