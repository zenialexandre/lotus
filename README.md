![lotus_pink_256x256_aligned](https://github.com/user-attachments/assets/362d4579-c524-40c8-b1b1-fe4ddac92d2f)
--------------

Lotus is a game engine with the main focus of being easy-to-use and straight forward on developing 2D games.  
It's based on the Entity-Component-System paradigm, providing windowing, rendering, physics, input handling, and more.<br>
Heavily inspired by awesome open-source projects like [`Bevy`](https://github.com/bevyengine/bevy), [`Comfy`](https://github.com/darthdeus/comfy) and [`LÖVE`](https://github.com/love2d/love).<br>

First full game made with Lotus: [`CyberLancer: Neon Rush`](https://github.com/maumafra/cyberlancer).<br>

![cyberlancer](https://github.com/user-attachments/assets/ef3869d5-a7e5-4d89-bb94-c9dd56bfeb05)

----------------

## How it works?

With the power of macros, the engine basic template could be very abstracted and easy to look up to.<br>
The `your_game!` macro only needs three parameters to make a game real.

-> The window configuration
- This parameter will be used to personalize and create the game window.

-> The setup function
- This parameter is a real function that will be ran once at the start of the application.
- The function should contain a mutable reference to the context as the parameter.
- Should contain all the initial entity spawning code for the game.

-> The update function
- This parameter is a real function as well, that will be ran at each frame of the application.
- The function should contain a mutable reference to the context as the parameter.
- Should contain all the logic functions behind the game.

----------------

## About assets

Make sure your textures, fonts, sounds and all that nice stuff are inside of the **assets** folder located in the root of your project!<br>
The engine will use the **CARGO_MANIFEST_DIR** to search for your assets and make sure that all is loaded correctly.<br>
Your folder tree should look similar to this:<br>

```shell
my_awesome_2d_application/
├── assets/
│ ├── textures/
│ ├── fonts/
│ ├── sounds/
│ └── ...
├── src/
│ ├── main.rs
└── Cargo.toml
```

You should use your relative paths like this:

```rust
use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(_context: &mut Context) {
    // As you can see, you DON'T need to use 'assets/' in your relative path.
    let sprite: Sprite = Sprite::new("textures/lotus_pink_256x256.png".to_string());
}

fn update(_context: &mut Context) {}
```

----------------

## The Entity-Component-System paradigm

Lotus uses a custom Entity-Component-System (ECS) archictecture.<br>
You can see the documentation about it [`here`](https://docs.rs/lotus_engine/0.1.4/lotus_engine/core/ecs/index.html).<br>

As a brief overview:

- Structs defined with the #derive macro *Component* are Components that can be spawned in our World within an Entity.
- Structs defined with the #derive macro *Resource* are Resources that can be added to in our World.
- *Entities* are defined by it's components and every entity has a unique ID.
- Entities are stored in what is called as *Archetypes* in our World.
- Archetypes are defined by the Components that our Entities have, so a Archetype will only have Entities with the same Components.
- The World can store multiple Archetypes, Entities, Components and Resources!
- And all of them can be queried using the *Query* struct.
<br><br>
![lotus_ecs_diagramv2](https://github.com/user-attachments/assets/e92130c7-26fb-4747-a1da-fdafe3a7fc70)

----------------

## Examples

The classic hello world:

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

Refer to the [`tutorial`](https://github.com/zenialexandre/lotus/blob/main/tutorial/README.md).<br>
And here are some more complex initial examples to demonstrate the engine's potential:<br>

- **Pong:** [`examples/pong.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/pong.rs)
- **Breakout:** [`examples/breakout.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/breakout.rs)
- **Rendering geometric forms:** [`examples/simple_shapes.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/simple_shapes.rs)
- **Rendering sprites:** [`examples/simple_sprite.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/simple_sprite.rs)
- **Simple physics simulation:** [`examples/physics_simulation.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/physics_simulation.rs)
- **Gravity simulation:** [`examples/physics_simulation.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/gravity_simulation.rs)

----------------

## Build Instructions
### Setting Up a Lotus Project

Lotus is a normal Rust dependency, therefore an empty Lotus project is very easy to set up.
You should use the latest stable version of rustc or above.

- To check which version you have downloaded, use:
```shell
rustc --version
```

- Initialize a new rust project:
```rust
cargo init --bin
```

- Add the engine as a depedency on your Cargo.toml:
```rust
[dependencies]
lotus_engine = "0.1.x"
```

- You may use the following command to get the latest version of the crate:
```rust
cargo add lotus_engine
```

- And now to run it natively:
```rust
cargo run
```

### Exporting a Lotus Project

Its very simple to export your brand new Lotus project for distribution.

- You may first build your Rust project as a release.
```rust
cargo build --release
```

- Then you can go to **target/release/** to get your executable archive.<br>
-> It should be the executable that only has your project name.<br>
-> Something like **nice-project-name.exe**.

- As of a commmon step on releasing indie games, you should send your executable archive along side your assets folder.

- So your file tree should look like this:

```shell
nice-project-release/
├── assets
├── nice-project-name.exe
```

----------------

## Engine Architecture Overview
![lotus_diagram_v2](https://github.com/user-attachments/assets/64f94220-1c37-422d-b699-54ba6c648ccc)
