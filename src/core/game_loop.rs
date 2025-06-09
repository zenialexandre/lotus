use std::time::{Duration, Instant};
use lotus_proc_macros::Resource;
use wgpu::SurfaceError;
use winit::event_loop::ActiveEventLoop;
use super::{
    context::Context,
    input::Input,
    camera::camera2d::Camera2d,
    managers::rendering::manager::RenderState,
    ecs::{world::World, resource::{ResourceRef, ResourceRefMut}}
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
        context.world.synchronize_events();
        context.world.synchronize_animations_of_entities(context.delta);
        context.world.synchronize_gravity_with_dynamic_bodies(&mut context.render_state, context.delta);
        context.world.synchronize_transformations_with_collisions();
        context.world.synchronize_camera_with_target(&mut context.render_state);
        (self.update)(context);

        self.render(&mut context.render_state, &mut context.world, event_loop);

        let mut input: ResourceRefMut<'_, Input> = context.world.get_resource_mut::<Input>().unwrap();
        input.update_hashes();

        // FPS calculus.
        self.frame_count += 1;
        let elapsed: Duration = self.last_fps_update.elapsed();
        if elapsed >= Duration::from_secs(1) {
            self.current_fps = self.frame_count;
            self.frame_count = 0;
            self.last_fps_update = Instant::now();
            context.game_loop_listener.current_fps = self.current_fps;
        }

        // Optional FPS capping.
        if let Some(fps_cap) = context.game_loop_listener.fps_cap {
            let target_delta: Duration = Duration::from_secs_f32(1.0 / fps_cap as f32);
            let frame_time: Duration = Instant::now() - now;

            if frame_time < target_delta {
                std::thread::sleep(target_delta - frame_time);
            }
        }
    }

    /// Call the rendering process.
    pub fn render(&self, render_state: &mut RenderState, world: &mut World, event_loop: &ActiveEventLoop) {
        match render_state.render(world) {
            Ok(_) => {}

            Err(
                SurfaceError::Lost | SurfaceError::Outdated
            ) => {
                let camera2d: ResourceRef<'_, Camera2d> = world.get_resource::<Camera2d>().unwrap();
                render_state.resize(
                    render_state.physical_size.as_ref().unwrap().clone(),
                    &camera2d,
                    &world.text_renderers
                );
            },
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

/// Struct to update the current engine state as the end-user.
#[derive(Clone, Debug, Resource)]
pub struct GameLoopListener {
    pub state: GameLoopState,
    pub current_fps: u32,
    pub fps_cap: Option<u32>
}

impl GameLoopListener {
    /// Create a new loop listener.
    pub fn new() -> Self {
        return Self {
            state: GameLoopState::Running,
            current_fps: 60,
            fps_cap: None
        };
    }

    /// Update the current loop status to Paused.
    pub(crate) fn _pause(&mut self) {
        self.state = GameLoopState::Paused;
    }

    /// Update the current loop status to Running.
    pub(crate) fn _resume(&mut self) {
        self.state = GameLoopState::Running;
    }

    /// Enable FPS capping.
    pub fn fps_cap(&mut self, fps_cap: u32) {
        self.fps_cap = Some(fps_cap);
    }
}
