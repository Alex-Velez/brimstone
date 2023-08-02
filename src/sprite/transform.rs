use raylib::prelude::{Color, Vector2};

/// For structs with fields:
/// tint, rotation, offset, rect, & source_rect,
pub trait SpriteTransform {
    /// Set sprite rotation
    fn set_rotation(&mut self, rotation: f32);

    /// Set sprite tint
    fn set_tint(&mut self, tint: Color);

    /// Set sprite offset with x and y
    fn set_offset_xy(&mut self, x: f32, y: f32);

    /// Set sprite offset with
    fn set_offset(&mut self, offset: Vector2);

    // Set sprite x position
    fn set_x(&mut self, x: f32);

    // Set sprite y position
    fn set_y(&mut self, y: f32);

    /// Set sprite position with x and y
    fn set_position_xy(&mut self, x: f32, y: f32);

    /// Set sprite position
    fn set_position(&mut self, position: Vector2);

    /// Resize sprite
    fn set_size(&mut self, width: f32, height: f32);

    /// Scale sprite by multiplier
    fn set_scale(&mut self, scale: f32);

    /// Flip sprite horizontally
    fn flip_h(&mut self);

    /// Flip sprite vertically
    fn flip_v(&mut self);

    /// Face sprite right
    fn face_right(&mut self);

    /// Face sprite left
    fn face_left(&mut self);

    /// Face sprite up
    fn face_up(&mut self);

    /// Face sprite down
    fn face_down(&mut self);

    /// Set sprite horizontal direction
    /// meant for values of 1 & -1, other
    /// values will stretch the sprite
    fn face_x(&mut self, direction: f32);

    /// Set sprite vertical direction
    /// meant for values of 1 & -1, other
    /// values will stretch the sprite
    fn face_y(&mut self, direction: f32);
}
