![lotus_pink_128x128](https://github.com/user-attachments/assets/a07bb1df-5a89-456d-9f73-02d4261d0c96)
----------------
Lotus is a game engine with the main focus of being easy-to-use and straight forward on developing 2D games.  
It's based on the Entity-Component-System paradigm, providing windowing, rendering, physics, input handling, and more.<br>
Heavily inspired by awesome open-source projects like [`Bevy`](https://github.com/bevyengine/bevy), [`Comfy`](https://github.com/darthdeus/comfy) and [`LÖVE`](https://github.com/love2d/love).

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

## Examples

The classic hello world:

```rust
use lotus_engine::*;

your_game!(WindowConfiguration::default(), setup, update);

fn setup(_context: &mut Context) {}

fn update(_context: &mut Context) {
    eprintln!("Hello World!");
}
```

And here are some more complex initial examples to demonstrate the engine's potential:

- **Pong:** [`examples/pong.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/pong.rs)
- **Breakout:** [`examples/breakout.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/breakout.rs)
- **Rendering geometric forms:** [`examples/simple_shapes.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/simple_shapes.rs)
- **Rendering sprites:** [`examples/simple_sprite.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/simple_sprite.rs)
- **Simple physics simulation:** [`examples/physics_simulation.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/physics_simulation.rs)

----------------

## Build Instructions
### Setting Up a Lotus Project
Lotus is a normal rust dependency, therefore an empty lotus project is very easy to set up.
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
----------------

## The Entity-Component-System paradigm

Lotus uses a custom Entity-Component-System (ECS) archictecture.<br>
You can see the documentation about it [`here`](https://docs.rs/lotus_engine/0.1.2/lotus_engine/core/ecs/index.html).<br>

As a brief overview:

- Structs defined with the #[derive] macro *Component* are Components, that can be spawned in our World within an Entity.
- Structs defined with the #[derive] macro *Resource* are Resources, that can be added to in our World.
- *Entities* are defined by it's components and every entity has a unique ID.
- Entities are stored in what is called as *Archetypes* in our World.
- Archetypes are defined by the Components that our Entities have, so a Archetype will only have Entities with the same Components.
- The World can store multiple Archetypes, Entities, Components and Resources!
- And all of them can be queried using the *Query* struct.
<br><br>
![lotus_ecs_diagramv2](https://github.com/user-attachments/assets/e92130c7-26fb-4747-a1da-fdafe3a7fc70)

----------------

## Engine architecture overview
<br></br>
![lotus_diagram_v2](https://github.com/user-attachments/assets/64f94220-1c37-422d-b699-54ba6c648ccc)
