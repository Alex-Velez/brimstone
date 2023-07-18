use crate::{raylib_plugins::Texture2DPlugin, sprite::AnimatedSprite2D};
use raylib::prelude::{RaylibDraw, Rectangle, Texture2D, Vector2};
use std::{collections::HashMap, hash::Hash};

pub struct AnimationPlayer2D<T> {
    animations: HashMap<T, AnimatedSprite2D>,
    direction: Vector2,
}

impl<T: Hash + Eq> AnimationPlayer2D<T> {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            direction: Vector2::new(1.0, 1.0),
        }
    }

    /// Create and insert new AnimatedSprite2D
    pub fn add_animation(
        mut self,
        state: T,
        texture_strip: Texture2D,
        frames: u32,
        fps: f32,
    ) -> Self {
        let animation = AnimatedSprite2D::from_texture_strip(texture_strip, frames, fps);
        self.animations.insert(state, animation);
        self
    }

    /// Insert AnimatedSprite2D
    pub fn add_animated_sprite(&mut self, state: T, animation: AnimatedSprite2D) {
        self.animations.insert(state, animation);
    }

    pub fn get_animation(&self, state: &T) -> Option<&AnimatedSprite2D> {
        self.animations.get(state)
    }

    pub fn get_animation_mut(&mut self, state: &T) -> Option<&mut AnimatedSprite2D> {
        self.animations.get_mut(state)
    }

    pub fn reset_frame(&mut self, state: T) {
        if let Some(animation) = self.animations.get_mut(&state) {
            animation.frame = 0;
        }
    }

    pub fn next_frame(&mut self, state: T) {
        if let Some(animation) = self.animations.get_mut(&state) {
            animation.next_frame();
        }
    }

    pub fn draw(&self, state: &T, raylib: &mut impl RaylibDraw) {
        if let Some(animation) = self.animations.get(state) {
            animation.draw(raylib);
        }
    }
}

/// Add transforms for all animated sprites
impl<T: Hash + Eq> AnimationPlayer2D<T> {
    /// Set sprite rect
    pub fn set_rect(&mut self, rect: Rectangle) {
        for (_, asprite) in &mut self.animations {
            asprite.set_rect(rect);
        }
    }

    /// Set sprite position
    pub fn set_position(&mut self, position: Vector2) {
        for (_, asprite) in &mut self.animations {
            asprite.set_position(position);
        }
    }

    /// Set sprite position
    pub fn set_offset(&mut self, offset: Vector2) {
        for (_, asprite) in &mut self.animations {
            asprite.set_offset(offset);
        }
    }

    /// resize sprite
    pub fn set_size(&mut self, size: Vector2) {
        for (_, asprite) in &mut self.animations {
            asprite.set_size(size);
        }
    }

    /// Scale sprite by multiplier
    pub fn set_scale(&mut self, scale: f32) {
        for (_, asprite) in &mut self.animations {
            asprite.set_scale(scale);
        }
    }

    /// Flip sprite horizontally
    pub fn flip_h(&mut self) {
        for (_, asprite) in &mut self.animations {
            asprite.flip_h();
        }
    }

    /// Flip sprite vertically
    pub fn flip_v(&mut self) {
        for (_, asprite) in &mut self.animations {
            asprite.flip_v();
        }
    }

    /// Face sprite right
    pub fn face_right(&mut self) {
        for (_, asprite) in &mut self.animations {
            asprite.face_right();
        }
    }

    /// Face sprite left
    pub fn face_left(&mut self) {
        for (_, asprite) in &mut self.animations {
            asprite.face_left();
        }
    }

    /// Face sprite up
    pub fn face_up(&mut self) {
        for (_, asprite) in &mut self.animations {
            asprite.face_up();
        }
    }

    /// Face sprite down
    pub fn face_down(&mut self) {
        for (_, asprite) in &mut self.animations {
            asprite.face_down();
        }
    }
}

use raylib::prelude::{RaylibHandle, RaylibThread};

pub struct AnimationPlayerBuilder<T> {
    animation_player: AnimationPlayer2D<T>,
    states: Vec<T>,
    texture_strip_paths: Vec<String>,
    frame_amounts: Vec<u32>,
    fps_values: Vec<f32>,
}

impl<T: Hash + Eq> AnimationPlayerBuilder<T> {
    pub fn new() -> Self {
        Self {
            animation_player: AnimationPlayer2D::new(),
            states: Vec::new(),
            texture_strip_paths: Vec::new(),
            frame_amounts: Vec::new(),
            fps_values: Vec::new(),
        }
    }

    /// Insert new animations
    pub fn add_animation(
        mut self,
        state: T,
        texture_strip_path: &str,
        frames: u32,
        fps: f32,
    ) -> Self {
        self.states.push(state);
        self.texture_strip_paths.push(texture_strip_path.into());
        self.frame_amounts.push(frames);
        self.fps_values.push(fps);
        self
    }

    /// Build AnimationPlayer with all added animations
    pub fn build(
        mut self,
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> AnimationPlayer2D<T> {
        for i in 0..self.states.len() {
            let texture_strip =
                Texture2D::from_path(raylib, thread, &self.texture_strip_paths.remove(0));
            self.animation_player = self.animation_player.add_animation(
                self.states.remove(0),
                texture_strip,
                self.frame_amounts.remove(0),
                self.fps_values.remove(0),
            );
        }
        self.animation_player
    }
}
