use cgmath::{Matrix4, SquareMatrix};
use lotus_proc_macros::Resource;
use super::super::{ecs::entity::Entity, physics::transform::Transform};

/// Struct to represent the global 2D camera resource in the world.
#[derive(Clone, Resource)]
pub struct Camera2d {
    pub transform: Transform,
    pub zoom: f32,
    pub target: Option<Entity>,
    pub view_matrix: Matrix4<f32>
}

impl Default for Camera2d {
    fn default() -> Self {
        return Self {
            transform: Transform::default(),
            zoom: 1.0,
            target: None,
            view_matrix: Matrix4::identity()
        }
    }
}

impl Camera2d {
    /// Set a new target to the camera following strategy.
    pub fn set_target(&mut self, entity: Entity) {
        if let Some(old_target) = self.target {
            if old_target.0 != entity.0 {
                self.target = Some(entity);
            }
        } else {
            self.target = Some(entity);
        }
    }
}
