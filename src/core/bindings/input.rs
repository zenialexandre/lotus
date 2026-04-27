/// Trait with input related functions that should be implemented.
pub trait Input {
    /// Update input state.
    fn update_hashes(&mut self);

    /// Returns if some binding is pressed.
    fn is_some_pressed(&self) -> bool;

    /// Returns if some binding is released.
    fn is_some_released(&self) -> bool;
}
