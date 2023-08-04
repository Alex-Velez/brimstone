mod camera;
mod dotted_line;
mod frame_limiter;
mod image;
mod texture;

pub use camera::{CameraEx2D, CameraMovement};
pub use dotted_line::DottedLine;
pub use frame_limiter::FrameLimiter;
pub use image::ImagePlugin;
pub use texture::Texture2DPlugin;

use super::{math::Math, window::Window};

/// Default texture when image path not found
const MISSING_TEXTURE: &[u8] = include_bytes!("missing_texture.png");
