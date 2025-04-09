#![doc(html_favicon_url = "https://raw.githubusercontent.com/zenialexandre/lotus/main/assets/textures/icons/lotus_doc_icon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/zenialexandre/lotus/main/assets/textures/lotus_pink_256x256.png")]

#![doc = r#"
![Lotus Logo](https://raw.githubusercontent.com/zenialexandre/lotus/main/assets/textures/lotus_pink_256x256.png)

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

## The Entity-Component-System paradigm

Lotus uses a custom Entity-Component-System (ECS) archictecture.<br>
You can see the documentation about it [`here`](https://docs.rs/lotus_engine/0.1.4/lotus_engine/core/ecs/index.html).<br>

As a brief overview:

- Structs defined with the derive macro *Component* are Components that can be spawned in our World within an Entity.
- Structs defined with the derive macro *Resource* are Resources that can be added to in our World.
- *Entities* are defined by it's components and every entity has a unique ID.
- Entities are stored in what is called as *Archetypes* in our World.
- Archetypes are defined by the Components that our Entities have, so a Archetype will only have Entities with the same Components.
- The World can store multiple Archetypes, Entities, Components and Resources!
- And all of them can be queried using the *Query* struct.
<br><br>
![Lotus ECS Diagram](https://raw.githubusercontent.com/zenialexandre/lotus/main/assets/textures/lotus_ecs_diagram_v2.png)

----------------

## Engine Architecture Overview
![Lotus Architecture Diagram](https://raw.githubusercontent.com/zenialexandre/lotus/main/assets/textures/lotus_diagram_v2.png)
"#]

/// Module with the main features of the engine.
pub mod core;

/// Module with utilities of the engine.
pub mod utils;

pub use core::managers::rendering_manager::*;
pub use core::managers::windowing_manager::*;
pub use core::game_loop::*;
pub use core::engine::*;
pub use core::color::*;
pub use core::shape::*;
pub use core::sprite::*;
pub use core::input::*;
pub use core::text::*;
pub use core::camera::camera2d::*;
pub use core::physics::transform::Transform;
pub use core::physics::acceleration::*;
pub use core::physics::collision::*;
pub use core::physics::velocity::*;
pub use core::time::timer::*;
pub use core::audio::audio_source::*;
pub use core::audio::audio_error::*;
pub use core::ecs::world::*;
pub use core::ecs::entity::*;
pub use core::ecs::component::*;
pub use core::ecs::resource::*;
pub use core::ecs::query::*;
pub use lotus_proc_macros::Component;
pub use lotus_proc_macros::Resource;
pub use cgmath::*;
pub use kira::*;
pub use pollster::block_on;
pub use winit::keyboard::{KeyCode, PhysicalKey};
