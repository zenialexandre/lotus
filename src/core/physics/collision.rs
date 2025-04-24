use cgmath::Vector2;
use lotus_proc_macros::Component;
use super::super::shape::GeometryType;

/// Struct to represent the real collider of the object.
#[derive(Clone, Debug)]
pub struct Collider {
    pub geometry_type: GeometryType,
    pub position: Vector2<f32>,
    pub scale: Vector2<f32>
}

impl Collider {
    /// Create a new collider with parameters.
    pub fn new(geometry_type: GeometryType, position: Vector2<f32>, scale: Vector2<f32>) -> Self {
        return Self {
            geometry_type,
            position,
            scale
        };
    }

    /// Create a new collider only with the geometry type as a parameter.
    pub fn new_simple(geometry_type: GeometryType) -> Self {
        return Self {
            geometry_type,
            position: Vector2::new(0.0, 0.0),
            scale: Vector2::new(0.0, 0.0)
        };
    }
}

/// Enumerator to store the possible collision algorithms to be used. 
#[derive(Default, Clone, Debug)]
pub enum CollisionAlgorithm {
    #[default]
    Aabb,
    Sat
}

impl CollisionAlgorithm {
    /// Returns if a collision is made by a specific algorithm.
    pub fn check(&self, a: &Collider, b: &Collider) -> bool {
        match self {
            CollisionAlgorithm::Aabb => { return Self::check_aabb(a, b) },
            CollisionAlgorithm::Sat => { return Self::check_sat(a, b) }
        }
    }

    /// Returns if a collision is made by the AABB algorithm.
    pub fn check_aabb(a: &Collider, b: &Collider) -> bool {
        let a_min: Vector2<f32> = a.position - a.scale / 2.0;
        let a_max: Vector2<f32> = a.position + a.scale / 2.0;
        let b_min: Vector2<f32> = b.position - b.scale / 2.0;
        let b_max: Vector2<f32> = b.position + b.scale / 2.0;

        return a_min.x < b_max.x &&
            a_max.x > b_min.x &&
            a_min.y < b_max.y &&
            a_max.y > b_min.y;
    }

    /// Returns if a collision is made by the SAT algorithm.
    fn check_sat(_a: &Collider, _b: &Collider) -> bool {
        return false;
    }
}

/// Struct to represent the collision characteristic that an object can have.
#[derive(Clone, Debug, Component)]
pub struct Collision {
    pub collider: Collider
}

impl Collision {
    /// Create a new collision with its collider.
    pub fn new(collider: Collider) -> Self {
        return Self {
            collider
        };
    }

    /// Returns if a collision is made based on the algorithm passed.
    pub fn check(algorithm: CollisionAlgorithm, a: &Collision, b: &Collision) -> bool {
        return algorithm.check(&a.collider, &b.collider);
    }
}
