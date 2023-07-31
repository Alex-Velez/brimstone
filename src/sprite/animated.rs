use crate::{raylib_plugins::Texture2DPlugin, timer::Timer};
use raylib::prelude::{
    Color, RaylibDraw, RaylibHandle, RaylibThread, Rectangle, Texture2D, Vector2,
};

pub struct AnimatedSprite2D {
    pub rect: Rectangle,
    pub tint: Color,
    pub rotation: f32,
    pub offset: Vector2,
    pub fps_timer: Timer,
    pub frame: u32,
    frames: u32,
    pub looping: bool,
    texture_strip: Texture2D,
    source_rect: Rectangle,
}

impl AnimatedSprite2D {
    /// Create animated 2D sprite from a loaded texture strip
    pub fn from_texture_strip(texture_strip: Texture2D, frames: u32, fps: f32) -> Self {
        let frame_width = texture_strip.width as f32 / frames as f32;
        let source_rect = Rectangle::new(0.0, 0.0, frame_width, texture_strip.height as f32);

        Self {
            rect: source_rect,
            tint: Color::WHITE,
            rotation: 0.0,
            offset: Vector2::new(0.0, 0.0),
            fps_timer: Timer::from_secs_f32(1.0 / fps),
            frame: 0,
            frames,
            looping: true,
            texture_strip,
            source_rect,
        }
    }

    /// Create animated 2D sprite with a texture strip
    pub fn from_path(
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        frames: u32,
        fps: f32,
    ) -> Self {
        Self::from_texture_strip(Texture2D::from_path(raylib, thread, path), frames, fps)
    }

    pub fn draw(&self, raylib: &mut impl RaylibDraw) {
        raylib.draw_texture_pro(
            &self.texture_strip,
            self.source_rect,
            self.rect,
            self.offset,
            self.rotation,
            self.tint,
        );
    }

    pub fn next_frame(&mut self) {
        if self.fps_timer.is_finished() {
            self.frame = (self.frame + 1) % self.frames;
            self.source_rect.x = self.frame as f32 * self.source_rect.width.abs();
            self.fps_timer.start();
        }
    }
}

/// Animated Sprite Field Accessors & Mutators
impl AnimatedSprite2D {
    pub fn fps(&self) -> f32 {
        1.0 / self.fps_timer.wait_time.as_secs_f32()
    }

    pub fn frames(&self) -> u32 {
        self.frames
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.fps_timer.set_wait_time(1.0 / fps);
    }

    pub fn set_texture_strip(&mut self, texture_strip: Texture2D, frames: u32) {
        let frame_width = texture_strip.width as f32 / frames as f32;
        let source_rect = Rectangle::new(
            self.frame as f32 * frame_width,
            0.0,
            frame_width,
            texture_strip.height as f32,
        );
        self.texture_strip = texture_strip;
        self.source_rect = source_rect;
        self.frames = frames;
    }
}

/// Animated Sprite Transform
impl AnimatedSprite2D {
    /// Set sprite rect
    pub fn set_rect(&mut self, rect: Rectangle) {
        self.rect = rect;
    }

    /// Set sprite position
    pub fn set_position(&mut self, position: Vector2) {
        self.rect.x = position.x;
        self.rect.y = position.y;
    }

    /// Set sprite offset
    pub fn set_offset(&mut self, offset: Vector2) {
        self.offset = offset;
    }

    /// resize sprite
    pub fn set_size(&mut self, size: Vector2) {
        self.rect.width = size.x;
        self.rect.height = size.y;
    }

    /// Scale sprite by multiplier
    pub fn set_scale(&mut self, scale: f32) {
        self.rect.width = self.source_rect.width * scale;
        self.rect.height = self.source_rect.height * scale;
    }

    /// Flip sprite horizontally
    pub fn flip_h(&mut self) {
        self.source_rect.width *= -1.0;
    }

    /// Flip sprite vertically
    pub fn flip_v(&mut self) {
        self.source_rect.height *= -1.0;
    }

    /// Face sprite right
    pub fn face_right(&mut self) {
        self.source_rect.width = self.source_rect.width.abs();
    }

    /// Face sprite left
    pub fn face_left(&mut self) {
        self.source_rect.width = -self.source_rect.width.abs();
    }

    /// Face sprite up
    pub fn face_up(&mut self) {
        self.source_rect.height = self.source_rect.height.abs();
    }

    /// Face sprite down
    pub fn face_down(&mut self) {
        self.source_rect.height = -self.source_rect.height.abs();
    }

    /// Set sprite horizontal direction
    /// meant for values of 1 & -1, other
    /// values will stretch the sprite
    pub fn face_x(&mut self, direction: f32) {
        if direction == 0.0 {
            return;
        }
        assert!(direction == 1.0 || direction == -1.0);
        self.source_rect.width = direction * self.source_rect.width.abs();
    }

    /// Set sprite vertical direction
    /// meant for values of 1 & -1, other
    /// values will stretch the sprite
    pub fn face_y(&mut self, direction: f32) {
        if direction == 0.0 {
            return;
        }
        assert!(direction == 1.0 || direction == -1.0);
        self.source_rect.height = direction * self.source_rect.height.abs();
    }
}
