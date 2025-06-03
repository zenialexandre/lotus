![lotus_pink_256x256_aligned](https://github.com/user-attachments/assets/362d4579-c524-40c8-b1b1-fe4ddac92d2f)

----------------

## Creating Your First Lotus Project

This tutorial aims to help users on their first moments using the Lotus engine.

### Rust Installation

First of all, you need to download the latest rustc version [`here`](https://www.rust-lang.org/tools/install).<br>
If everything went well, this command should give you the version of the rustc that was installed.

```shell
rustc --version
```

If you are on Windows, make sure you have the <strong>Microsoft C++ Build Tools</strong> installed or the <strong>Visual Studio Code</strong>.<br>
The following can be found [`here`](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup).<br>

### Project Initialization

Now you need to create a new folder that will hold your project. <br>
Then you should execute the following command on your prompt to initialize a new Rust project, using **cargo**. <br>
**Cargo** should be installed along side the rustc.

```rust
cargo init --bin
```

Now you need to execute the following command on your prompt to get the latest Lotus version.

```rust
cargo add lotus_engine
```

### Your First 'Hello World!'

After some small setup, now its time to execute some real code. <br>
On your 'main.rs' you should import the **lotus_engine** crate using:

```rust
use lotus_engine::*;
```

Then you will have everything you need to develop your first application! <br>
Start off by setting up the 'your_game!' macro, like this:

```rust
your_game!(
    WindowConfiguration::default(),
    setup,
    update
);
```

You will notice errors on the setup and update parameters. <br>
This two parameters are real functions, that we need to implement as well. <br>
The 'setup' function will only run once, at the start of your application! <br>
The 'update' function will run at each frame.

```rust
fn setup(_context: &mut Context) {}

fn update(_context: &mut Context) {
    eprintln!("Hello World!");
}
```

Then you will have the most simple template with Lotus, opening your first application.

```rust
use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(_context: &mut Context) {}

fn update(_context: &mut Context) {
    eprintln!("Hello World!");
}
```

Now just run the following command on your prompt targetting your root folder. <br>
You may notice that the assets directory will be automatically created if it wasn't before.

```rust
cargo run
```

### Rendering Your First Entity

Its time to render our first entity, it can be a lot of things, but in this case it will be a **shape**. <br>
Let's use our 'Hello World!' example and increase its complexity a little by adding some stuff. <br>
The explanation will be on the code!

```rust
use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    // First of all, you create your Shape variable.
    // Shape is a Component in our ECS World.
    let my_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BLUE);
    
    // Then we use the context to access commands.
    // Commands are used mostly to mutate our ECS World.
    // In this case, we spawn a new entity that is a Shape!
    context.commands.spawn(vec![Box::new(my_shape)]);
}

fn update(context: &mut Context) {
    // Then a new query is created.
    // On our ECS paradigm, querying is the core of the logic systems.
    // By querying we get our entities and its components for reading or writing.
    let mut query: Query = Query::new(&context.world).with::<Shape>();

    // Here we execute the query getting all the entities with the Shape component!
    // We know that only one entity exists, so I just get the first one.
    let my_entity: Entity = query.entities_with_components().unwrap().first().unwrap().clone();

    // Every entity has an unique ID on our World.
    eprintln!("My Entity Unique ID: {:?}", my_entity.0);
}
```

### Working With Components

Let's mutate our first component. <br>
If the Transform component isn't on our 'spawn' function, it will be created as default. <br>
So let's create the component with our own values.

```rust
use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let my_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BLUE);

    // We need to create a Transform variable.
    // Setting the initial position to x=0.0 and y=0.0 (middle of the screen).
    // Rotation to 0.0.
    // And scale to x=0.25 and y=0.25.
    // The strategy normalized means that it will not use pixelated coordinates.
    let transform: Transform = Transform::new(
        Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
        0.0,
        Vector2::new(0.25, 0.25)
    );

    // Now we send the transform component too.
    context.commands.spawn(vec![Box::new(my_shape), Box::new(transform)]);
}

fn update(context: &mut Context) {
    // Just to demonstrate, you can set more filters to your query.
    // In this case only passing Shape would do the trick.
    // But our entity have the Transform component too, so it will work as well.
    let mut query: Query = Query::new(&context.world).with::<Shape>().with::<Transform>();

    let my_entity: Entity = query.entities_with_components().unwrap().first().unwrap().clone();

    // Here we access our world through the context parameter.
    // Our world knows everything, so it only needs the entity to find its components!
    // In this case we want to mutate the transformation matrix of our entity, so we want the mutable reference.
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&my_entity).unwrap();

    // We declare our rotation variable.
    // It will be updated at each frame.
    // Remember always to make use of the delta value!
    let my_rotation: f32 = transform.rotation + 100.0 * context.delta;

    // Here we use the 'set_rotation' function of the Transform component.
    // At each frame, the rotation of our shape will be updated with our variable.
    // Note that we are accessing the 'render state' field.
    // Its the main struct related to the rendering, it can be useful and its needed in some functions.
    // Keep in mind some fields that can be accessed through this sctruct like 'queue' and 'device'.
    transform.set_rotation(&context.render_state, my_rotation);
}
```

### Working With Resources

Resources are available singletons in our world that store useful data and help us out. <br>
Always remember that you CAN create your own componentes and resources at easy. <br>
In this next example I will make use of a default resource of the world. <br>
And I will create my own component and resource.

```rust
use lotus_engine::*;

// You can create a component that can be very complex.
// In this case it will only work as a filter for our query.
#[derive(Clone, Component)]
pub struct JustAComponent();

// Here a resource is created.
// It hold a u32 value.
#[derive(Clone, Resource)]
pub struct JustAResource(u32);

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let my_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::BLUE);
    let transform: Transform = Transform::new(
        Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
        0.0,
        Vector2::new(0.25, 0.25)
    );

    // Add our component.
    context.commands.spawn(vec![Box::new(my_shape), Box::new(transform), Box::new(JustAComponent())]);

    // Add our resource with its initial value.
    context.commands.add_resource(Box::new(JustAResource(1)));
}

fn update(context: &mut Context) {
    // Using our component on the query.
    // With a filter like this, it can be easier to find an entity.
    let mut query: Query = Query::new(&context.world).with::<JustAComponent>();
    let my_entity: Entity = query.entities_with_components().unwrap().first().unwrap().clone();
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&my_entity).unwrap();

    // Here we use the world to get a specific resource.
    // We only want to read the information of input.
    // So the input will not be mutable.
    let input: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();

    // Here we get our resource as a mutable reference.
    // We want to change the value.
    let mut just_a_resource: ResourceRefMut<'_, JustAResource> = context.world.get_resource_mut::<JustAResource>().unwrap();

    // While we are pressing the X key, our shape will be rotated.
    if input.is_key_pressed(KeyCode::KeyX) {
        let my_rotation: f32 = transform.rotation + 100.0 * context.delta;
        transform.set_rotation(&context.render_state, my_rotation);
    }

    // Every time the X key is released, the resource value will be updated and printed out.
    if input.is_key_released(KeyCode::KeyX) {
        just_a_resource.0 += 1;
        eprintln!("Resource Value: {:?}", just_a_resource.0);
    }
}
```

### How About Some Physics?

Lotus supports collision detection using the AABB algorithm. <br>
And the concept of gravity can be enabled using our Gravity global resource! <br>
Let's create an small example of collision detection between two entities and enable the force of gravity.

```rust
use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    // Start off by creating two objects.
    // One will be static and will serve as a table.
    // The other will be our main entity.
    let table: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK);
    let object: Shape = Shape::new(Orientation::Horizontal, GeometryType::Circle(Circle::default()), Color::BLUE);

    context.commands.spawn(vec![
        Box::new(table),
        Box::new(Transform::new(
            Position::new(Vector2::new(0.0, -0.70), Strategy::Normalized),
            0.0,
            Vector2::new(0.90, 0.10)
        )),
        // Our table Collision.
        Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
    ]);

    // The spawn of our main entity will have two new components: 'Collision' and 'RigidBody'.
    // The Collision component will tell our world that this entity collides!
    // The posisiton and the scale of our collider is the same as its entity.
    //
    // The RigidBody component will tell our world that this entity CAN be affected by the forces of gravity.
    // Keep in mind that the gravity will only affect entities with 'Dynamic' rigid bodies and velocity. 
    context.commands.spawn(vec![
        Box::new(object),
        Box::new(Transform::new(
            Position::new(Vector2::new(400.0, 100.0), Strategy::Pixelated),
            0.0,
            Vector2::new(0.50, 0.50)
        )),
        Box::new(Collision::new(Collider::new_simple(GeometryType::Square))),
        Box::new(Velocity::new(Vector2::new(0.2, 0.2))),
        // The first parameter is the type of the body, in this case: Dynamic.
        // The next parameter is the mass of the body (it will affect movement after collisions with other objects with mass).
        // The third parameter is the restitution factor (it can affect movement after collisions).
        // The last parameter is the friction factor (it will affect gravity).
        Box::new(RigidBody::new(BodyType::Dynamic, 0.1, 0.9, 1.0))
    ]);
}

fn update(context: &mut Context) {
    let input: Input = {
        let input_ref: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();
        input_ref.clone()
    };

    // Gravity is a global resource in our world.
    // It starts disabled and with 9.8 as its value of force (default value of Earth).
    // So if you want it to act, enable it!
    if input.is_key_released(KeyCode::Enter) {
        let mut gravity: ResourceRefMut<'_, Gravity> = context.world.get_resource_mut::<Gravity>().unwrap();
        gravity.enable();
    }

    // As you can see, you can control the gravity acting state freely.
    if input.is_key_released(KeyCode::Escape) {
        let mut gravity: ResourceRefMut<'_, Gravity> = context.world.get_resource_mut::<Gravity>().unwrap();
        gravity.disable();
    }

    // This is our helper function to check collisions!
    check_table_object_collision(context);
}

fn check_table_object_collision(context: &mut Context) {
    // Start of by querying our entities.
    // Here I use the RigidBody as the main filter.
    let mut table_query: Query = Query::new(&context.world).with::<RigidBody>();
    let mut object_query: Query = Query::new(&context.world).with::<RigidBody>();

    let table: Entity = table_query.entities_without_components().unwrap().first().unwrap().clone();
    let object: Entity = object_query.entities_with_components().unwrap().first().unwrap().clone();

    // For collision checking purposes, you need to get the Collisions components.
    let table_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&table).unwrap();

    let (object_collision, mut object_transform, mut object_velocity, mut object_rigid_body) = (
        context.world.get_entity_component::<Collision>(&object).unwrap(),
        context.world.get_entity_component_mut::<Transform>(&object).unwrap(),
        context.world.get_entity_component_mut::<Velocity>(&object).unwrap(),
        context.world.get_entity_component_mut::<RigidBody>(&object).unwrap()
    );

    // Here we can use the 'check' function to search for a collision between two colliders.
    // In this case we are using the AABB algorithm.
    if Collision::check(CollisionAlgorithm::Aabb, &table_collision, &object_collision) {
        // This is our 'magic' number to tell when the object stopped boucing.
        if object_velocity.y < -0.001 {
            // Now this is the calculus for the 'bounce' effect in our object.
            // Note the use of the restitution value here!
            // It serves as a 'consequence' of the collisions, by decreasing our vertical velocity.
            object_velocity.y = -object_velocity.y * object_rigid_body.restitution;
            let y: f32 = object_transform.position.y + object_velocity.y * context.delta;
            object_transform.set_position_y(&context.render_state, y);
        } else {
            // When there is no need to bounce anymore, the object will have velocity equal to zero.
            // And we can use the rest parameter of our rigid body.
            object_velocity.y = 0.0;
            object_rigid_body.rest = true;
        }
    }
}
```

### Now Its Up To You

Thank you for reading until here! <br>
To learn more about the engine, please look into our examples [`here`](https://github.com/zenialexandre/lotus/tree/main/examples). <br>
Happy coding! 👾
