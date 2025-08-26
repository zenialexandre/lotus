//! This example is a show off about applied gravity.
//! The Gravity component will be affect all entities with the Velocity and RigidBody components.
//! Making use of the velocity and collision components, one sprite is coliding with a shape.
//! After the collision, the sprite bounce until its stops based on its restitution value.

use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let table: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK);
    let object: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(Circle::default()), Color::BLUE);

    context.commands.spawn(vec![
        Box::new(table),
        Box::new(Transform::new(
            Position::new(Vector2::new(0.0, -0.70), Strategy::Normalized),
            0.0,
            Vector2::new(0.90, 0.10)
        )),
        Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
    ]);

    context.commands.spawn(vec![
        Box::new(object),
        Box::new(Transform::new(
            Position::new(Vector2::new(400.0, 100.0), Strategy::Pixelated),
            0.0,
            Vector2::new(0.50, 0.50)
        )),
        Box::new(Collision::new(Collider::new_simple(GeometryType::Square))),
        Box::new(Gravity::new(0.0)),
        Box::new(Velocity::new(Vector2::new(0.2, 0.2))),
        Box::new(RigidBody::new(BodyType::Dynamic, 1.0, 0.9, 1.0))
    ]);
}

fn update(context: &mut Context) {
    let input: Input = {
        let input_ref: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();
        input_ref.clone()
    };

    let mut query: Query = Query::new(&context.world).with::<Gravity>();
    let entity: Entity = query.entities_with_components().unwrap().first().unwrap().clone();

    if input.is_key_released(KeyCode::Enter) {
        let mut gravity: ComponentRefMut<'_, Gravity> = context.world.get_entity_component_mut::<Gravity>(&entity).unwrap();
        gravity.value = 9.8;
    }
    check_table_object_collision(context);
}

fn check_table_object_collision(context: &mut Context) {
    let mut table_query: Query = Query::new(&context.world).with::<RigidBody>();
    let mut object_query: Query = Query::new(&context.world).with::<RigidBody>();

    let table: Entity = table_query.entities_without_components().unwrap().first().unwrap().clone();
    let object: Entity = object_query.entities_with_components().unwrap().first().unwrap().clone();

    let table_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&table).unwrap();

    let (object_collision, mut object_transform, mut object_velocity, mut object_rigid_body) = (
        context.world.get_entity_component::<Collision>(&object).unwrap(),
        context.world.get_entity_component_mut::<Transform>(&object).unwrap(),
        context.world.get_entity_component_mut::<Velocity>(&object).unwrap(),
        context.world.get_entity_component_mut::<RigidBody>(&object).unwrap()
    );

    if Collision::check(CollisionAlgorithm::Aabb, &table_collision, &object_collision) {
        if object_velocity.y < -0.001 {
            object_velocity.y = -object_velocity.y * object_rigid_body.restitution;
            let y: f32 = object_transform.position.y + object_velocity.y * context.delta;
            object_transform.set_position_y(&context.render_state, y);
        } else {
            object_velocity.y = 0.0;
            object_rigid_body.rest = true;
        }
    }
}
