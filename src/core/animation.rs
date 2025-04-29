use std::collections::{HashMap, VecDeque};
use lotus_proc_macros::Component;
use super::texture::sprite_sheet::{SpriteSheet, AnimationState};

/// Struct that represents the animations for a certain entity.
#[derive(Clone, Component)]
pub struct Animation {
    pub sprite_sheets: HashMap<String, SpriteSheet>,
    pub is_some_playing: bool,
    pub playing_stack: VecDeque<String>
}

impl Animation {
    /// Creates a new animation struct passing the mapping as an argument.
    pub fn new(sprite_sheets: HashMap<String, SpriteSheet>) -> Self {
        return Self {
            sprite_sheets,
            is_some_playing: false,
            playing_stack: VecDeque::new()
        };
    }

    /// Adds a new sprite sheet to the map.
    pub fn add_sprite_sheet(&mut self, title: String, sprite_sheet: SpriteSheet) {
        self.sprite_sheets.insert(title, sprite_sheet);
    }

    /// Merges a map of sprite sheets into the existing one.
    pub fn add_sprite_sheets(&mut self, sprite_sheets: HashMap<String, SpriteSheet>) {
        self.sprite_sheets.extend(sprite_sheets);
    }

    /// Returns a certain sprite sheet.
    pub fn get_sprite_sheet(&self, title: String) -> Option<&SpriteSheet> {
        return self.sprite_sheets.get(&title);
    }

    /// Returns a certain sprite sheet as mutable.
    pub fn get_sprite_sheet_as_mut(&mut self, title: String) -> Option<&mut SpriteSheet> {
        return self.sprite_sheets.get_mut(&title);
    }

    /// Returns the current animation playing on top of the stack.
    pub fn get_playing_animation_now(&self) -> Option<&SpriteSheet> {
        for title in &self.playing_stack {
            if let Some(sprite_sheet) = self.sprite_sheets.get(title) {
                if sprite_sheet.animation_state == AnimationState::Playing {
                    return Some(sprite_sheet);
                }
            }
        }
        return None;
    }

    /// Plays the animation of a certain sprite sheet.
    pub fn play(&mut self, title: String) {
        if  
            self.get_playing_animation_now().is_some() &&
            self.playing_stack.front().map(|t| t == &title).unwrap_or(false)
        {
            return;
        }

        if let Some(sprite_sheet) = self.sprite_sheets.get_mut(&title) {
            sprite_sheet.play();
            self.is_some_playing = true;
            self.playing_stack.retain(|t| t != &title);
            self.playing_stack.push_front(title);
        }
    }

    /// Stops the animation of a certain sprite sheet.
    pub fn stop(&mut self, title: String) {
        if let Some(sprite_sheet) = self.sprite_sheets.get_mut(&title) {
            sprite_sheet.stop();
            self.playing_stack.retain(|t| t != &title);
            self.is_some_playing = !self.playing_stack.is_empty();        
        }
    }

    /// Pauses the animation of a certain sprite sheet.
    pub fn pause(&mut self, title: String) {
        if let Some(sprite_sheet) = self.sprite_sheets.get_mut(&title) {
            sprite_sheet.pause();
        }
    }

    /// Resumes the animation of a certain sprite sheet.
    pub fn resume(&mut self, title: String) {
        if let Some(sprite_sheet) = self.sprite_sheets.get_mut(&title) {
            sprite_sheet.resume();
        }
    }
}

impl Default for Animation {
    fn default() -> Self {
        return Self {
            sprite_sheets: HashMap::new(),
            is_some_playing: false,
            playing_stack: VecDeque::new()
        };
    }
}
