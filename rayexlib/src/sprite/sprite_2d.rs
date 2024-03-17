use super::{SpriteTransform, Texture2DPlugin};
use raylib::prelude::{Color, RaylibDraw, RaylibHandle, RaylibThread, Texture2D, Vector2};

pub struct Sprite2D {
    texture: Texture2D,
    transform: SpriteTransform,
}

impl Sprite2D {
    pub fn from_texture(texture: Texture2D) -> Self {
        let width = texture.width as f32;
        let height = texture.height as f32;
        Sprite2D {
            texture,
            transform: SpriteTransform::new(width, height),
        }
    }

    pub fn from_path(raylib: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Self {
        Sprite2D::from_texture(Texture2D::from_path(raylib, thread, path))
    }

    pub fn draw(&self, raylib: &mut impl RaylibDraw) {
        raylib.draw_texture_pro(
            &self.texture,
            self.transform.source_rect,
            self.transform.rect,
            self.transform.offset,
            self.transform.rotation,
            self.transform.tint,
        );
    }
}

/// Export transform getters
impl Sprite2D {
    pub const fn width(&self) -> f32 {
        self.transform.width()
    }

    pub const fn height(&self) -> f32 {
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
impl Sprite2D {
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
