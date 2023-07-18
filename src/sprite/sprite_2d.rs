use crate::raylib_plugins::Texture2DPlugin;
use raylib::prelude::{
    Color, RaylibDraw, RaylibHandle, RaylibThread, Rectangle, Texture2D, Vector2,
};

/*
   * Split Rect into Position & Size

*/

pub struct Sprite2D {
    pub rect: Rectangle,
    pub rotation: f32,
    pub tint: Color,
    pub offset: Vector2,
    texture: Texture2D,
    source_rect: Rectangle,
}

impl Sprite2D {
    pub fn from_texture(texture: Texture2D) -> Sprite2D {
        let rect = Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32);
        Sprite2D {
            rect,
            rotation: 0.0,
            tint: Color::WHITE,
            offset: Vector2::new(0.0, 0.0),
            texture,
            source_rect: rect,
        }
    }

    pub fn from_path(raylib: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Sprite2D {
        Sprite2D::from_texture(Texture2D::from_path(raylib, thread, path))
    }

    pub fn draw(&self, raylib: &mut impl RaylibDraw) {
        raylib.draw_texture_pro(
            &self.texture,
            self.source_rect,
            self.rect,
            self.offset,
            self.rotation,
            self.tint,
        );
    }
}

impl Sprite2D {
    /// Set sprite position
    pub fn set_position(&mut self, position: Vector2) {
        self.rect.x = position.x;
        self.rect.y = position.y;
    }

    /// Resize sprite
    pub fn set_size(&mut self, size: Vector2) {
        self.rect.width = size.x;
        self.rect.height = size.y;
    }

    /// Set sprite position and return itself
    pub fn offset(mut self, offset: Vector2) -> Self {
        self.offset = offset;
        self
    }

    /// resize sprite and return itself
    pub fn size(mut self, size: Vector2) -> Self {
        self.rect.width = size.x;
        self.rect.height = size.y;
        self
    }

    /// Scale sprite by multiplier and return itself
    pub fn scale(mut self, scale: f32) -> Self {
        self.rect.width = self.source_rect.width * scale;
        self.rect.height = self.source_rect.height * scale;
        self
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
}
