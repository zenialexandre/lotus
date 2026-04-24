use std::time::{Duration, Instant};
use winit::event_loop::ActiveEventLoop;
use super::{
    context::Context,
    input::{input::Input, keyboard_input::KeyboardInput, mouse_input::MouseInput, gamepad_input::GamepadInput},
    managers::render::manager::RenderState,
    ecs::world::World
};

/// Struct to store the engine loop data.
pub struct GameLoop {
    pub delta: Duration,
    pub previous_time_of_last_run: Instant,
    pub setup: fn(context: &mut Context),
    pub update: fn(context: &mut Context),
    pub frame_count: u32,
    pub last_fps_update: Instant,
    pub current_fps: u32
}

impl GameLoop {
    /// Create a new loop with parameters.
    pub fn new(setup: fn(context: &mut Context), update: fn(context: &mut Context)) -> Self {
        return Self {
            delta: Duration::from_secs_f32(1.0 / 60.0),
            previous_time_of_last_run: Instant::now(),
            setup,
            update,
            frame_count: 0,
            last_fps_update: Instant::now(),
            current_fps: 0
        };
    }

    /// Run the engine loop to start the logic and rendering processes.
    pub fn run(&mut self, context: &mut Context, event_loop: &ActiveEventLoop) {
        let now: Instant = Instant::now();
        self.delta = now - self.previous_time_of_last_run;
        self.previous_time_of_last_run = now;

        context.delta = self.get_delta_as_seconds();
        context.commands.flush_commands(&mut context.world, &mut context.render_state);
        context.world.synchronize(&mut context.render_state, context.delta);

        if context.gamepad_listener.enabled {
            context.gamepad_listener.manage(&mut context.world);
        }
        (self.update)(context);

        self.render(&mut context.render_state, &mut context.world, event_loop);
        self.calculate_current_fps(context);
        self.apply_fps_cap(context, now);

        context.world.get_resource_mut::<KeyboardInput>().unwrap().update_hashes();
        context.world.get_resource_mut::<MouseInput>().unwrap().update_hashes();
        context.world.get_resource_mut::<GamepadInput>().unwrap().instances
            .iter_mut()
            .for_each(|element| element.1.update_hashes());
    }

    /// Call the rendering process.
    pub fn render(&self, render_state: &mut RenderState, world: &mut World, event_loop: &ActiveEventLoop) {
        render_state.prepare(world, event_loop);
    }

    /// Calculates the current FPS and sets it.
    pub(crate) fn calculate_current_fps(&mut self, context: &mut Context) {
        self.frame_count += 1;
        let elapsed: Duration = self.last_fps_update.elapsed();
        if elapsed >= Duration::from_secs(1) {
            self.current_fps = self.frame_count;
            self.frame_count = 0;
            self.last_fps_update = Instant::now();
            context.game_loop_listener.current_fps = self.current_fps;
        }
    }

    /// Apply the FPS capping optionally.
    pub(crate) fn apply_fps_cap(&self, context: &mut Context, now: Instant) {
        if let Some(fps_cap) = context.game_loop_listener.fps_cap {
            let target_delta: Duration = Duration::from_secs_f32(1.0 / fps_cap as f32);
            let frame_time: Duration = Instant::now() - now;

            if frame_time < target_delta {
                std::thread::sleep(target_delta - frame_time);
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
