use raylib::prelude::RaylibHandle;

/// Minimum frame limit
const FRAME_LIMIT: f32 = 1.0 / 20.0_f32;

pub trait FrameLimiter {
    fn get_frame_time_limited(&self) -> f32;
}

impl FrameLimiter for RaylibHandle {
    fn get_frame_time_limited(&self) -> f32 {
        self.get_frame_time().clamp(0.0, FRAME_LIMIT)
    }
}
