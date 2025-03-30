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
The final goal of this project should be something similar to this architecture diagram:<br>
![lotus_diagram](https://github.com/user-attachments/assets/0643f3d0-84d2-4385-80fa-58ec8599f1ed)
