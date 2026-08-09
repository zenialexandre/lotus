use lotus_proc_macros::Component;
use crate::{Color, ComponentRefMut};

/// Struct to represent an entity fading effect.
#[derive(Clone, Component)]
pub struct Fade {
    pub state: bool
}

impl Fade {
    /// Create a new Fade struct.
    pub fn new(value: bool) -> Self {
        return Self {
            state: value
        };
    }

    /// Apply a fade-in effect in a group of Color components.
    pub fn fade_out(&self, colors: Vec<ComponentRefMut<'_, Color>>, delta: f32, fade_speed: f32) -> bool {
        for mut color in colors {
            color.a = color.a - delta * fade_speed;

            if color.a <= 0.0 {
                return true;
            }
        }
        return false;
    }

    /// Apply a fade-out effect in a group of Color components.
    pub fn fade_in(&self, colors: Vec<ComponentRefMut<'_, Color>>, delta: f32, fade_speed: f32) -> bool {
        for mut color in colors {
            color.a = color.a + delta * fade_speed;

            if color.a >= 1.0 {
                return true;
            }
        }
        return false;
    }

    /// Verifies if the fading is happening.
    pub fn is_fading(&self) -> bool {
        return self.state;
    }
}

impl Default for Fade {
    fn default() -> Self {
        return Self {
            state: false
        };
    }
}

/// Verifies if there is any entity fading.
pub fn is_any_fading(fade_components: Vec<&Fade>) -> bool {
    for fade in fade_components {
        if fade.is_fading() {
            return true;
        }
    }
    return false;
}

/// Update the fading state for a set of entities.
pub fn set_fading_state(fade_components: Vec<ComponentRefMut<'_, Fade>>, fading_state: bool) {
    for mut fade in fade_components {
        fade.state = fading_state;
    }
}
