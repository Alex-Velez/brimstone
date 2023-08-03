use raylib::prelude::{Color, Rectangle, Vector2};

pub struct SpriteTransform {
    pub tint: Color,
    pub rotation: f32,
    pub offset: Vector2,
    pub rect: Rectangle,
    pub source_rect: Rectangle,
}

impl SpriteTransform {
    pub const fn new(width: f32, height: f32) -> Self {
        Self {
            tint: Color::WHITE,
            rotation: 0.0,
            offset: Vector2::new(0.0, 0.0),
            rect: Rectangle::new(0.0, 0.0, width, height),
            source_rect: Rectangle::new(0.0, 0.0, width, height),
        }
    }
}

/// Getters
impl SpriteTransform {
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
}

/// Setters
impl SpriteTransform {
    /// Set sprite rotation
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    /// Set sprite tint
    pub fn set_tint(&mut self, tint: Color) {
        self.tint = tint;
    }

    /// Set sprite offset with x and y
    pub fn set_offset_xy(&mut self, x: f32, y: f32) {
        self.offset.x = x;
        self.offset.y = y;
    }

    /// Set sprite offset
    pub fn set_offset(&mut self, offset: Vector2) {
        self.offset = offset;
    }

    /// Set sprite x position
    pub fn set_x(&mut self, x: f32) {
        self.rect.x = x;
    }

    /// Set sprite y position
    pub fn set_y(&mut self, y: f32) {
        self.rect.y = y;
    }

    /// Set sprite position with x and y
    pub fn set_position_xy(&mut self, x: f32, y: f32) {
        self.rect.x = x;
        self.rect.y = y;
    }

    /// Set sprite position
    pub fn set_position(&mut self, position: Vector2) {
        self.rect.x = position.x;
        self.rect.y = position.y;
    }

    /// Resize sprite
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.rect.width = width;
        self.rect.height = height;
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
