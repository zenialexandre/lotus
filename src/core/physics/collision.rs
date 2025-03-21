use cgmath::Vector2;
use lotus_proc_macros::Component;

use crate::GeometryType;

#[derive(Clone, Debug)]
pub struct Collider {
    pub geometry_type: GeometryType,
    pub position: Vector2<f32>,
    pub scale: Vector2<f32>
}

impl Collider {
    pub fn new(geometry_type: GeometryType, position: Vector2<f32>, scale: Vector2<f32>) -> Self {
        return Self {
            geometry_type,
            position,
            scale
        };
    }
}

#[derive(Default, Clone, Debug)]
pub enum CollisionAlgorithm {
    #[default]
    Aabb,
    Sat
}

impl CollisionAlgorithm {
    pub fn check(&self, a: &Collider, b: &Collider) -> bool {
        match self {
            CollisionAlgorithm::Aabb => { return Self::check_aabb(a, b) },
            CollisionAlgorithm::Sat => { return Self::check_sat(a, b) }
        }
    }

    fn check_aabb(a: &Collider, b: &Collider) -> bool {
        let a_min: Vector2<f32> = a.position - a.scale / 2.0;
        let a_max: Vector2<f32> = a.position + a.scale / 2.0;
        let b_min: Vector2<f32> = b.position - b.scale / 2.0;
        let b_max: Vector2<f32> = b.position + b.scale / 2.0;

        return a_min.x < b_max.x &&
            a_max.x > b_min.x &&
            a_min.y < b_max.y &&
            a_max.y > b_min.y;
    }

    fn check_sat(_a: &Collider, _b: &Collider) -> bool {
        return false;
    }
}

#[derive(Clone, Debug, Component)]
pub struct Collision {
    pub collider: Collider
}

impl Collision {
    pub fn new(collider: Collider) -> Self {
        return Self {
            collider
        };
    }

    pub fn check(algorithm: CollisionAlgorithm, a: &Collision, b: &Collision) -> bool {
        return algorithm.check(&a.collider, &b.collider);
    }
}
