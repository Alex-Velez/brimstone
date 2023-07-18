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

use crate::window::DEFAULT_TITLE;

/// Default texture when image path not found
const MISSING_TEXTURE: &[u8] = include_bytes!("../../resources/missing_texture.png");

/// Minimum frame limit
const FRAME_LIMIT: f32 = 1.0 / 20.0_f32;
