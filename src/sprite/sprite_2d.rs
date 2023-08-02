use super::{SpriteTransform, Texture2DPlugin};
use raylib::prelude::{
    Color, RaylibDraw, RaylibHandle, RaylibThread, Rectangle, Texture2D, Vector2,
};

pub struct Sprite2D {
    texture: Texture2D,
    tint: Color,
    rotation: f32,
    offset: Vector2,
    rect: Rectangle,
    source_rect: Rectangle,
}

impl Sprite2D {
    pub fn from_texture(texture: Texture2D) -> Self {
        let rect = Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32);
        Sprite2D {
            texture,
            tint: Color::WHITE,
            rotation: 0.0,
            offset: Vector2::new(0.0, 0.0),
            rect,
            source_rect: rect,
        }
    }

    pub fn from_path(raylib: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Self {
        Sprite2D::from_texture(Texture2D::from_path(raylib, thread, path))
    }

    pub fn width(&self) -> f32 {
        self.rect.width
    }

    pub fn height(&self) -> f32 {
        self.rect.height
    }

    pub fn half_width(&self) -> f32 {
        self.rect.width / 2.0
    }

    pub fn half_height(&self) -> f32 {
        self.rect.height / 2.0
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

/// Sprite transforms
impl SpriteTransform for Sprite2D {
    /// Set sprite rotation
    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    /// Set sprite tint
    fn set_tint(&mut self, tint: Color) {
        self.tint = tint;
    }

    /// Set sprite offset with x and y
    fn set_offset_xy(&mut self, x: f32, y: f32) {
        self.offset.x = x;
        self.offset.y = y;
    }

    /// Set sprite offset
    fn set_offset(&mut self, offset: Vector2) {
        self.offset = offset;
    }

    /// Set sprite x position
    fn set_x(&mut self, x: f32) {
        self.rect.x = x;
    }

    /// Set sprite y position
    fn set_y(&mut self, y: f32) {
        self.rect.y = y;
    }

    /// Set sprite position with x and y
    fn set_position_xy(&mut self, x: f32, y: f32) {
        self.rect.x = x;
        self.rect.y = y;
    }

    /// Set sprite position
    fn set_position(&mut self, position: Vector2) {
        self.rect.x = position.x;
        self.rect.y = position.y;
    }

    /// Resize sprite
    fn set_size(&mut self, width: f32, height: f32) {
        self.rect.width = width;
        self.rect.height = height;
    }

    /// Scale sprite by multiplier
    fn set_scale(&mut self, scale: f32) {
        self.rect.width = self.source_rect.width * scale;
        self.rect.height = self.source_rect.height * scale;
    }

    /// Flip sprite horizontally
    fn flip_h(&mut self) {
        self.source_rect.width *= -1.0;
    }

    /// Flip sprite vertically
    fn flip_v(&mut self) {
        self.source_rect.height *= -1.0;
    }

    /// Face sprite right
    fn face_right(&mut self) {
        self.source_rect.width = self.source_rect.width.abs();
    }

    /// Face sprite left
    fn face_left(&mut self) {
        self.source_rect.width = -self.source_rect.width.abs();
    }

    /// Face sprite up
    fn face_up(&mut self) {
        self.source_rect.height = self.source_rect.height.abs();
    }

    /// Face sprite down
    fn face_down(&mut self) {
        self.source_rect.height = -self.source_rect.height.abs();
    }

    /// Set sprite horizontal direction
    /// meant for values of 1 & -1, other
    /// values will stretch the sprite
    fn face_x(&mut self, direction: f32) {
        if direction == 0.0 {
            return;
        }
        assert!(direction == 1.0 || direction == -1.0);
        self.source_rect.width = direction * self.source_rect.width.abs();
    }

    /// Set sprite vertical direction
    /// meant for values of 1 & -1, other
    /// values will stretch the sprite
    fn face_y(&mut self, direction: f32) {
        if direction == 0.0 {
            return;
        }
        assert!(direction == 1.0 || direction == -1.0);
        self.source_rect.height = direction * self.source_rect.height.abs();
    }
}
