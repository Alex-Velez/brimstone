const ROUNDING_THRESHOLD: f32 = 0.05;

/// Generic math functions
pub trait Math {
    fn lerp(&mut self, end: Self, t: f32);
    fn round_zero(&mut self);
}

impl Math for f32 {
    fn lerp(&mut self, end: Self, t: f32) {
        // Imprecise method
        *self += t * (end - *self);
        // Precise method
        // *self = (1.0 - t) * *self + t * end;
    }

    fn round_zero(&mut self) {
        if (*self).abs() < ROUNDING_THRESHOLD {
            *self = 0.0;
        }
    }
}

/// Linear interpolation from `start` to `end`
/// if input time `t` is not in [0,1],
/// respective bounds will be returned
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    match t {
        x if x > 1.0 => end,
        x if x < 0.0 => start,
        _ => start + t * (end - start),
    }
    // Imprecise method
    // start + t * (end - start)
    // Precise method
    // (1.0 - t) * v0 + t * v1
}
