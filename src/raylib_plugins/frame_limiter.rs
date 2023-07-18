use super::FRAME_LIMIT;
use raylib::prelude::RaylibHandle;

pub trait FrameLimiter {
    fn get_frame_time_limited(&self) -> f32;
}

impl FrameLimiter for RaylibHandle {
    fn get_frame_time_limited(&self) -> f32 {
        self.get_frame_time().clamp(0.0, FRAME_LIMIT)
    }
}
