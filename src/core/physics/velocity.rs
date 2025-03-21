use cgmath::Vector2;
use lotus_proc_macros::Component;

#[derive(Clone, Component)]
pub struct Velocity {
    pub value: Vector2<f32>
}

impl Velocity {
    pub fn new(value: Vector2<f32>) -> Self {
        return Self {
            value
        };
    }
}