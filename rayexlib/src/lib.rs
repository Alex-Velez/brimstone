#![allow(dead_code)]
pub mod collision;
pub mod debug;
pub mod math;
pub mod raylib_plugins;
pub mod scene_manager;
pub mod sprite;
pub mod state_manager;
pub mod timer;
pub mod toggle;
pub mod traits;
pub mod window;

pub mod prelude {
    pub use super::*;

    pub use collision::*;
    pub use debug::*;
    pub use math::*;
    pub use raylib_plugins::*;
    pub use scene_manager::*;
    pub use sprite::*;
    pub use state_manager::*;
    pub use timer::*;
    pub use toggle::*;
    pub use traits::*;
    pub use window::*;
}
