#![doc(html_favicon_url = "https://raw.githubusercontent.com/zenialexandre/lotus/main/assets/textures/icons/lotus_doc_icon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/zenialexandre/lotus/main/assets/textures/lotus_pink_256x256.png")]

pub mod core;
pub mod utils;
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
