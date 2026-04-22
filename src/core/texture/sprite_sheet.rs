use std::time::Duration;
use super::super::{time::timer::{Timer, TimerType}};

/// Enumerator for animation state mapping.
#[derive(Clone, PartialEq)]
pub enum AnimationState {
    Stopped,
    Paused,
    Playing,
    Finished
}

/// Enumerator for animation loop mapping.
#[derive(Clone, PartialEq)]
pub enum LoopingState {
    Repeat,
    Once
}

/// Struct to represent a sprite sheet.
#[derive(Clone)]
pub struct SpriteSheet {
    pub path: String,
    pub timer: Timer,
    pub tile_width: f32,
    pub tile_height: f32,
    pub rows: u32,
    pub columns: u32,
    pub indices: Vec<u32>,
    pub current_index: u32,
    pub animation_state: AnimationState,
    pub looping_state: LoopingState
}

impl SpriteSheet {
    /// Creates a new sprite sheet struct.
    pub fn new(
        path: String,
        looping_state: LoopingState,
        tile_size: (f32, f32),
        time_between_tiles: f32,
        rows: u32,
        columns: u32,
        indices: Vec<u32>
    ) -> Self {
        return Self {
            path,
            timer: Timer::new(TimerType::Repeat, Duration::from_secs_f32(time_between_tiles)),
            tile_width: tile_size.0,
            tile_height: tile_size.1,
            rows,
            columns,
            indices,
            current_index: 0,
            animation_state: AnimationState::Finished,
            looping_state
        };
    }

    /// Returns the current tile UV (texture coordinates).
    pub(crate) fn current_tile_uv_coordinates(&self) -> [f32; 8] {
        let columns: f32 = self.columns as f32;
        let rows: f32 = self.rows as f32;

        let tile_index: f32 = self.indices[self.current_index as usize] as f32;
        let column: f32 = tile_index % columns;
        let row: f32 = (tile_index / columns).floor();

        let tile_width: f32 = 1.0 / columns;
        let tile_height: f32 = 1.0 / rows;

        let left: f32 = column * tile_width;
        let right: f32 = left + tile_width;
        let top: f32 = row * tile_height;
        let bottom: f32 = top + tile_height;

        return [
            left, bottom,
            right, bottom,
            right, top,
            left, top
        ];
    }

    /// Play the current animation while reseting the timer.
    pub(crate) fn play(&mut self) {
        self.animation_state = AnimationState::Playing;
        self.timer.reset();
    }

    /// Stops the current animation and set its current index to 0.
    pub(crate) fn stop(&mut self) {
        self.animation_state = AnimationState::Stopped;
        self.current_index = 0;
    }

    /// Pauses the current animation if its already playing.
    pub(crate) fn pause(&mut self) {
        if self.animation_state == AnimationState::Playing {
            self.animation_state = AnimationState::Paused;
        }
    }

    /// Resumes the current animation if its already paused.
    pub(crate) fn resume(&mut self) {
        if self.animation_state == AnimationState::Paused {
            self.animation_state = AnimationState::Playing;
        }
    }
}
