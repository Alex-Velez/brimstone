use raylib::prelude::{RaylibDrawHandle, RaylibHandle};

pub struct Debug {
    pub active: bool,
    pub step_frames: bool,
    pub paused: bool,
    pub step_fps: u32,
}

impl Default for Debug {
    fn default() -> Self {
        Self {
            active: false,
            step_frames: false,
            paused: false,
            step_fps: 30,
        }
    }
}

pub trait DebugTools {
    fn debug_update(&mut self, raylib: &mut RaylibHandle);
    fn debug_draw(&self, raylib: &mut RaylibDrawHandle);
}
