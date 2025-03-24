#![doc(html_favicon_url = "https://raw.githubusercontent.com/zenialexandre/lotus/main/assets/textures/icons/lotus_doc_icon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/zenialexandre/lotus/main/assets/textures/lotus_pink_256x256.png")]

#![doc = r#"
# Lotus

Lotus is a game engine with the main focus of being easy-to-use and straight forward on developing 2D games.  
It's based on the Entity-Component-System paradigm, providing windowing, rendering, physics, input handling, and more.

## Examples

Here are some initial examples to demonstrate the engine's potential:

- **Rendering basic geometric forms:** [`examples/simple_shapes.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/simple_shapes.rs)
- **Rendering sprites:** [`examples/simple_sprite.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/simple_sprite.rs)
- **A simple physics simulation:** [`examples/physics_simulation.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/physics_simulation.rs)
- **A example of a simple arcade game, the Pong:** [`examples/pong.rs`](https://github.com/zenialexandre/lotus/blob/main/examples/pong.rs)
"#]

/// Módulo principal do motor de jogo.
pub mod core;

/// Utilitários do motor.
pub mod utils;

/// Módulo para testes do motor.
pub mod test;

pub use core::managers::rendering_manager::*;
pub use core::managers::windowing_manager::*;
pub use core::game_loop::*;
pub use core::engine::*;
pub use core::color::*;
pub use core::shape::*;
pub use core::sprite::*;
pub use core::input::*;
pub use core::physics::transform::*;
pub use core::physics::acceleration::*;
pub use core::physics::collision::*;
pub use core::physics::velocity::*;
pub use core::ecs::world::*;
pub use core::ecs::entitiy::*;
pub use core::ecs::component::*;
pub use core::ecs::resource::*;
pub use core::ecs::query::*;
pub use lotus_proc_macros::Component;
pub use lotus_proc_macros::Resource;
