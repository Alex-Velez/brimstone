use crate::prelude::Timer;
use raylib::prelude::{Color, RaylibDraw, Rectangle, Texture2D, Vector2};

pub struct AnimationStrip {
    frame: u32,
    frames: u32,
    fps_timer: Timer,
    texture_strip: Texture2D,
}

impl AnimationStrip {
    pub fn new(frames: u32, fps_timer: Timer, texture_strip: Texture2D) -> AnimationStrip {
        AnimationStrip {
            frame: 0,
            frames,
            fps_timer,
            texture_strip,
        }
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.fps_timer.set_wait_time(1.0 / fps);
    }

    pub fn reset_frame(&mut self) {
        self.frame = 0;
    }

    pub fn next_frame(&mut self, source_rect: &mut Rectangle) {
        if self.fps_timer.is_finished() {
            self.frame = (self.frame + 1) % self.frames;
            source_rect.x = self.frame as f32 * source_rect.width.abs();
            self.fps_timer.start();
        }
    }

    pub fn draw_with(
        &self,
        raylib: &mut impl RaylibDraw,
        source_rect: Rectangle,
        rect: Rectangle,
        offset: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        raylib.draw_texture_pro(
            &self.texture_strip,
            source_rect,
            rect,
            offset,
            rotation,
            tint,
        );
    }
}
