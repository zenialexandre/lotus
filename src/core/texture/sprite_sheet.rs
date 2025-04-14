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

    pub fn animate(&mut self, context: &mut Context) {
        self.timer.tick(context.delta);

        if self.timer.is_finished() {
            let current_index = self.indices[self.current_index as usize];
            let (tile_width, tile_height): (u32, u32) = self.tile_size;

            let column = current_index % self.columns;
            let row = current_index / self.columns;

            let normalized_column = column as f32 / self.columns as f32;
            let normalized_row = row as f32 / self.rows as f32;

            let relative_width = 1.0 / self.columns as f32;
            let relative_height = 1.0 / self.rows as f32;

            /*
            context.render_state.queue.as_ref().unwrap().write_buffer(
                context.render_state.animation_frame_buffer.as_ref().unwrap(),
                0,
                bytemuck::cast_slice(&[normalized_column, normalized_row, relative_width, relative_height])
            ); */

            self.current_index = (self.current_index + 1) % self.indices.len() as u32;
        }
    }

    pub fn animate_sprite() {
        // Do animation but first set the visibility of the sprite to false.
        // And after to visible again.
    }
}
