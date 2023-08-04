use super::{SpriteTransform, Texture2DPlugin, Timer};
use raylib::prelude::{
    Color, RaylibDraw, RaylibHandle, RaylibThread, Rectangle, Texture2D, Vector2,
};

pub struct AnimatedSprite2D {
    pub frame: u32,
    pub fps_timer: Timer,
    pub looping: bool,
    frames: u32,
    texture_strip: Texture2D,
    transform: SpriteTransform,
}

impl AnimatedSprite2D {
    /// Create animated 2D sprite from a loaded texture strip
    pub fn from_texture_strip(texture_strip: Texture2D, frames: u32, fps: f32) -> Self {
        let frame_width = texture_strip.width as f32 / frames as f32;
        let frame_height = texture_strip.height as f32;
        Self {
            frame: 0,
            fps_timer: Timer::from_secs_f32(1.0 / fps),
            looping: true,
            frames,
            texture_strip,
            transform: SpriteTransform::new(frame_width, frame_height),
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
            self.transform.source_rect,
            self.transform.rect,
            self.transform.offset,
            self.transform.rotation,
            self.transform.tint,
        );
    }

    pub fn next_frame(&mut self) {
        if self.fps_timer.is_finished() {
            self.frame = (self.frame + 1) % self.frames;
            self.transform.source_rect.x =
                self.frame as f32 * self.transform.source_rect.width.abs();
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
        self.transform.source_rect = source_rect;
        self.frames = frames;
    }
}

/// Export transform getters
impl AnimatedSprite2D {
    pub fn width(&self) -> f32 {
        self.transform.width()
    }

    pub fn height(&self) -> f32 {
        self.transform.height()
    }

    pub fn half_width(&self) -> f32 {
        self.transform.half_width()
    }

    pub fn half_height(&self) -> f32 {
        self.transform.half_height()
    }
}

/// Export transform setters
impl AnimatedSprite2D {
    pub fn set_rotation(&mut self, rotation: f32) {
        self.transform.set_rotation(rotation);
    }

    pub fn set_tint(&mut self, tint: Color) {
        self.transform.set_tint(tint);
    }

    pub fn set_offset_xy(&mut self, x: f32, y: f32) {
        self.transform.set_offset_xy(x, y);
    }

    pub fn set_offset(&mut self, offset: Vector2) {
        self.transform.set_offset(offset);
    }

    pub fn set_x(&mut self, x: f32) {
        self.transform.set_x(x);
    }

    pub fn set_y(&mut self, y: f32) {
        self.transform.set_y(y);
    }

    pub fn set_position_xy(&mut self, x: f32, y: f32) {
        self.transform.set_position_xy(x, y);
    }

    pub fn set_position(&mut self, position: Vector2) {
        self.transform.set_position(position);
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.transform.set_size(width, height);
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.transform.set_scale(scale);
    }

    pub fn flip_h(&mut self) {
        self.transform.flip_h();
    }

    pub fn flip_v(&mut self) {
        self.transform.flip_v();
    }

    pub fn face_right(&mut self) {
        self.transform.face_right();
    }

    pub fn face_left(&mut self) {
        self.transform.face_left();
    }

    pub fn face_up(&mut self) {
        self.transform.face_up();
    }

    pub fn face_down(&mut self) {
        self.transform.face_down();
    }

    pub fn face_x(&mut self, direction: f32) {
        self.transform.face_x(direction);
    }

    pub fn face_y(&mut self, direction: f32) {
        self.transform.face_y(direction);
    }
}
