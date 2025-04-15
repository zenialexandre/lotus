use std::time::Duration;
use lotus_proc_macros::Component;
use super::super::{time::timer::Timer, physics::transform::Transform, engine::Context};

/// Struct to represent a sprite sheet.
/// An entity can have multiple sprite sheets to do multiple animations.
#[derive(Clone, Component)]
pub struct SpriteSheet {
    path: String,
    transform: Transform,
    timer: Timer,
    tile_size: (u32, u32),
    rows: u32,
    columns: u32,
    indices: Vec<u32>,
    current_index: u32
}

impl SpriteSheet {
    /// Creates a new sprite sheet struct.
    pub fn new(
        path: String,
        transform: Transform,
        time_between_tiles: f32,
        tile_size: (u32, u32),
        rows: u32,
        columns: u32,
        indices: Vec<u32>
    ) -> Self {
        return Self {
            path,
            transform,
            timer: Timer::new(crate::TimerType::Repeat, Duration::from_secs_f32(time_between_tiles)),
            tile_size,
            rows,
            columns,
            indices,
            current_index: 0
        };
    }

    pub fn next_frame(&mut self, delta: f32) {
        self.timer.tick(delta);

        if self.timer.is_finished() {
            self.current_index = if self.current_index == self.indices.last().unwrap().clone() {
                self.current_index + 1
            } else {
                self.current_index + 1
            };
        }
    }

    pub fn animate(&mut self, context: &mut Context) {
        
    }

    pub fn animate_sprite() {
        // Do animation but first set the visibility of the sprite to false.
        // And after to visible again.
    }
}
