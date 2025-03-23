use std::time::{Duration, Instant};
use lotus_proc_macros::Resource;
use wgpu::SurfaceError;
use winit::event_loop::ActiveEventLoop;

use super::{
    engine::Context,
    managers::rendering_manager::RenderState,
    ecs::world::World
};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum GameLoopState {
    #[default]
    Running,
    Paused
}

pub struct GameLoop {
    delta: Duration,
    previous_time_of_last_run: Instant,
    pub setup: fn(context: &mut Context),
    pub update: fn(context: &mut Context),
}

#[derive(Clone, Debug, Resource)]
pub struct GameLoopListener {
    pub state: GameLoopState
}

impl GameLoop {
    pub fn new(setup: fn(context: &mut Context), update: fn(context: &mut Context)) -> Self {
        return Self {
            delta: Duration::from_secs_f32(1.0 / 60.0),
            previous_time_of_last_run: Instant::now(),
            setup,
            update
        };
    }

    pub(crate) fn run(&mut self, context: &mut Context, event_loop: &ActiveEventLoop) {
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

        if GameLoopState::Running == context.game_loop_listener.state {
            self.render(&mut context.render_state, &context.world, event_loop);
        }
    }

    fn render(&self, render_state: &mut RenderState, world: &World, event_loop: &ActiveEventLoop) {
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

    pub fn get_delta(&self) -> Duration {
        return self.delta;
    }

    pub fn get_delta_as_seconds(&self) -> f32 {
        return self.delta.as_secs_f32();
    }
}

impl GameLoopListener {
    pub fn new() -> Self {
        return Self { state: GameLoopState::Running };
    }

    pub fn pause(&mut self) {
        self.state = GameLoopState::Paused;
    }

    pub fn resume(&mut self) {
        self.state = GameLoopState::Running;
    }
}
