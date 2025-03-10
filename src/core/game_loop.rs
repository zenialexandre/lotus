use std::time::{Duration, Instant};
use wgpu::SurfaceError;
use winit::event_loop::ActiveEventLoop;

use super::{engine::EngineContext, managers::rendering_manager::RenderState};

#[derive(Default)]
pub(crate) enum GameLoopState {
    #[default]
    Running,
    Paused,
    Ended
}

pub struct GameLoop {
    game_loop_state: GameLoopState,
    delta: Duration,
    accumulated_time_from_last_update: Duration,
    previous_time_of_last_run: Instant,
    pub setup: fn(engine_context: &mut EngineContext),
    update: fn(engine_context: &mut EngineContext),
}

impl GameLoop {
    pub fn new(setup: fn(engine_context: &mut EngineContext), update: fn(engine_context: &mut EngineContext)) -> Self {
        return Self {
            game_loop_state: GameLoopState::Running,
            delta: Duration::from_secs_f32(1.0 / 60.0), // 60 FPS
            accumulated_time_from_last_update: Duration::ZERO,
            previous_time_of_last_run: Instant::now(),
            setup,
            update
        };
    }

    pub(crate) fn run(&mut self, engine_context: &mut EngineContext, event_loop: &ActiveEventLoop) {
        let now: Instant = Instant::now();
        let elapsed_time_from_last_run: Duration = now - self.previous_time_of_last_run;
        self.previous_time_of_last_run = now;
        self.accumulated_time_from_last_update += elapsed_time_from_last_run;

        engine_context.delta = self.get_delta_as_seconds();

        while self.accumulated_time_from_last_update >= self.delta {
            (self.update)(engine_context);
            self.accumulated_time_from_last_update -= self.delta;
        }
        self.render(&mut engine_context.render_state, event_loop);
    }

    fn render(&self, render_state: &mut RenderState, event_loop: &ActiveEventLoop) {
        match render_state.render() {
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
