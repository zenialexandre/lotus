use lotus_proc_macros::Component;

/// Enumerator to represent the types of bodies in our physics system.
#[derive(Clone, Default, PartialEq)]
pub enum BodyType {
    /// The Static body never moves.
    Static,
    /// The Dynamic body is affected by forces.
    Dynamic,
    /// The Kinematic body is only moved manually and can create collisions.
    #[default]
    Kinematic
}

/// Struct to represent the rigid body of an entity.
#[derive(Clone, Component)]
pub struct RigidBody {
    /// The type of the rigid body.
    pub body_type: BodyType,
    /// The mass of the body.
    /// It will affect the effects of a collision between other objects with mass.
    pub mass: f32,
    /// The restitution factor.
    /// It will affect the effects of a collision. 
    pub restitution: f32,
    /// The friction factor.
    /// It can be used to affect movement.
    pub friction: f32
}

impl RigidBody {
    /// Create a new rigid body will all the arguments.
    pub fn new(body_type: BodyType, mass: f32, restitution: f32, friction: f32) -> Self {
        return Self {
            body_type,
            mass,
            restitution,
            friction
        };
    }

    /// Create a rigid body in a simplified way.
    pub fn new_simple(body_type: BodyType, mass: f32) -> Self {
        return Self {
            body_type,
            mass,
            restitution: 1.0,
            friction: 1.0
        };
    }
}
