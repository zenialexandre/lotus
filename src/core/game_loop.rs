use std::time::{Duration, Instant};
use wgpu::SurfaceError;
use winit::event_loop::ActiveEventLoop;

use crate::core::rendering_manager::State;

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
    accumulated_time: Duration,
    previous_time: Instant
}

impl GameLoop {
    pub fn new() -> Self {
        return Self {
            game_loop_state: GameLoopState::Running,
            delta_time: Duration::from_secs_f32(1.0 / 60.0), // 60 FPS
            accumulated_time: Duration::ZERO,
            previous_time: Instant::now()
        };
    }

    pub(crate) fn run(&mut self, state: &mut State, event_loop: &ActiveEventLoop) {
        let now: Instant = Instant::now();
        let elapsed_time: Duration = now - self.previous_time;
        self.previous_time = now;
        self.accumulated_time += elapsed_time;

        while self.accumulated_time >= self.delta_time {
            self.update(self.delta_time);
            self.accumulated_time -= self.delta_time;
        }
        self.render(state, event_loop);
    }

    fn update(&mut self, delta_time: Duration) {
        //..
    }

    fn render(&self, state: &mut State, event_loop: &ActiveEventLoop) {
        match state.render() {
            Ok(_) => {}

            Err(
                SurfaceError::Lost | SurfaceError::Outdated
            ) => state.resize(state.physical_size),

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
}
