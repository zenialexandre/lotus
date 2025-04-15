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
You may notice that the assets directory will be automatically created, with it wasn't before.

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

To learn more about the engine, please look into our examples [`here`](https://github.com/zenialexandre/lotus/tree/main/examples).
