use super::{SpriteTransform, Timer};
use raylib::prelude::{Color, RaylibDraw, Rectangle, Texture2D, Vector2};
use std::{collections::HashMap, hash::Hash};

mod builder;

pub use builder::AnimationMachineBuilder;

pub struct AnimationPlayer2D<T> {
    animations: HashMap<T, AnimationStrip>,
    tint: Color,
    rotation: f32,
    offset: Vector2,
    rect: Rectangle,
    source_rect: Rectangle,
}

impl<T: Hash + Eq> AnimationPlayer2D<T> {
    pub fn new(sprite_size: Vector2) -> Self {
        let source_rect = Rectangle::new(0.0, 0.0, sprite_size.x, sprite_size.y);
        Self {
            rect: source_rect,
            tint: Color::WHITE,
            rotation: 0.0,
            offset: Vector2::new(0.0, 0.0),
            animations: HashMap::new(),
            source_rect,
        }
    }

    pub fn add_animation(&mut self, state: T, texture_strip: Texture2D, frames: u32, fps: f32) {
        let anim_strip = AnimationStrip {
            frame: 0,
            frames,
            fps_timer: Timer::from_secs_f32(1.0 / fps),
            texture_strip,
        };
        self.animations.insert(state, anim_strip);
    }

    pub fn set_fps(&mut self, state: &T, fps: f32) {
        if let Some(anim_strip) = self.animations.get_mut(state) {
            anim_strip.fps_timer.set_wait_time(1.0 / fps);
        }
    }

    pub fn reset_frame(&mut self, state: &T) {
        if let Some(anim_strip) = self.animations.get_mut(state) {
            anim_strip.frame = 0;
        }
    }

    pub fn next_frame(&mut self, current_state: &T) {
        if let Some(anim_strip) = self.animations.get_mut(current_state) {
            if anim_strip.fps_timer.is_finished() {
                anim_strip.frame = (anim_strip.frame + 1) % anim_strip.frames;
                self.source_rect.x = anim_strip.frame as f32 * self.source_rect.width.abs();
                anim_strip.fps_timer.start();
            }
        }
    }

    pub fn draw(&self, state: &T, raylib: &mut impl RaylibDraw) {
        if let Some(anim_strip) = self.animations.get(state) {
            raylib.draw_texture_pro(
                &anim_strip.texture_strip,
                self.source_rect,
                self.rect,
                self.offset,
                self.rotation,
                self.tint,
            );
        }
    }
}

/// Animated Sprite Transforms
impl<T: Hash + Eq> SpriteTransform for AnimationPlayer2D<T> {
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

    // Set sprite x position
    fn set_x(&mut self, x: f32) {
        self.rect.x = x;
    }

    // Set sprite y position
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

pub struct AnimationStrip {
    frame: u32,
    frames: u32,
    fps_timer: Timer,
    texture_strip: Texture2D,
}
