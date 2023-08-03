use super::{SpriteTransform, Timer};
use raylib::prelude::{Color, RaylibDraw, Texture2D, Vector2};
use std::{collections::HashMap, hash::Hash};

mod builder;

pub use builder::AnimationMachineBuilder;

pub struct AnimationPlayer2D<T> {
    animations: HashMap<T, AnimationStrip>,
    transform: SpriteTransform,
}

impl<T: Hash + Eq> AnimationPlayer2D<T> {
    pub fn new(sprite_size: Vector2) -> Self {
        Self {
            animations: HashMap::new(),
            transform: SpriteTransform::new(sprite_size.x, sprite_size.y),
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
                self.transform.source_rect.x =
                    anim_strip.frame as f32 * self.transform.source_rect.width.abs();
                anim_strip.fps_timer.start();
            }
        }
    }

    pub fn draw(&self, state: &T, raylib: &mut impl RaylibDraw) {
        if let Some(anim_strip) = self.animations.get(state) {
            raylib.draw_texture_pro(
                &anim_strip.texture_strip,
                self.transform.source_rect,
                self.transform.rect,
                self.transform.offset,
                self.transform.rotation,
                self.transform.tint,
            );
        }
    }
}

/// Export transform getters
impl<T: Hash + Eq> AnimationPlayer2D<T> {
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
impl<T: Hash + Eq> AnimationPlayer2D<T> {
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

pub struct AnimationStrip {
    frame: u32,
    frames: u32,
    fps_timer: Timer,
    texture_strip: Texture2D,
}
