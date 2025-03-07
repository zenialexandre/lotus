use std::time::{Duration, Instant};
use wgpu::SurfaceError;
use winit::event_loop::ActiveEventLoop;

use super::rendering_manager::RenderState;

#[derive(Default)]
pub(crate) enum GameLoopState {
    #[default]
    Running,
    Paused,
    Ended
}

pub struct GameLoop {
    game_loop_state: GameLoopState,
    delta_time: Duration,
    accumulated_time_from_last_update: Duration,
    previous_time_of_last_run: Instant,
    pub setup: fn(render_state: &mut RenderState),
    update: fn(render_state: &mut RenderState, delta_time: f32),
}

impl GameLoop {
    pub fn new(setup: fn(render_state: &mut RenderState), update: fn(render_state: &mut RenderState, delta_time: f32)) -> Self {
        return Self {
            game_loop_state: GameLoopState::Running,
            delta_time: Duration::from_secs_f32(1.0 / 60.0), // 60 FPS
            accumulated_time_from_last_update: Duration::ZERO,
            previous_time_of_last_run: Instant::now(),
            setup,
            update
        };
    }

    pub(crate) fn run(&mut self, render_state: &mut RenderState, event_loop: &ActiveEventLoop) {
        let now: Instant = Instant::now();
        let elapsed_time_from_last_run: Duration = now - self.previous_time_of_last_run;
        self.previous_time_of_last_run = now;
        self.accumulated_time_from_last_update += elapsed_time_from_last_run;

        while self.accumulated_time_from_last_update >= self.delta_time {
            (self.update)(render_state, self.get_delta_time_as_seconds());
            self.accumulated_time_from_last_update -= self.delta_time;
        }
        self.render(render_state, event_loop);
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

    pub fn get_delta_time(&self) -> Duration {
        return self.delta_time;
    }

    pub fn get_delta_time_as_seconds(&self) -> f32 {
        return self.delta_time.as_secs_f32();
    }
}
