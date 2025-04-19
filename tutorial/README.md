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
        Scale::new(Vector2::new(0.25, 0.25), Strategy::Normalized)
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
        Scale::new(Vector2::new(0.25, 0.25), Strategy::Normalized)
    );

    // Add our component.
    context.commands.spawn(vec![Box::new(my_shape), Box::new(transform), Box::new(JustAComponent())]);

    // Add our resource with its initial value.
    context.commands.add_resource(Box::new(JustAResource(1)));
}

fn update(context: &mut Context) {
    // Using our component on the query.
    // With a filter like this, it can be much easier to find an entity.
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
    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyX)) {
        let my_rotation: f32 = transform.rotation + 100.0 * context.delta;
        transform.set_rotation(&context.render_state, my_rotation);
    }

    // Every time the X key is released, the resource value will be updated and printed.
    if input.is_key_released(PhysicalKey::Code(KeyCode::KeyX)) {
        just_a_resource.0 += 1;
        eprintln!("Resource Value: {:?}", just_a_resource.0);
    }
}
```

### Now Its Up To You

Thank you for reading until here! <br>
To learn more about the engine, please look into our examples [`here`](https://github.com/zenialexandre/lotus/tree/main/examples). <br>
Happy coding!! 👾
