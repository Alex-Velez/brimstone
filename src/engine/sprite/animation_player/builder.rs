use super::{AnimationPlayer2D, Texture2DPlugin};
use raylib::prelude::{RaylibHandle, RaylibThread, Texture2D, Vector2};
use std::hash::Hash;

pub struct AnimationMachineBuilder<T> {
    animation_machine: AnimationPlayer2D<T>,
    states: Vec<T>,
    texture_strip_paths: Vec<String>,
    frame_amounts: Vec<u32>,
    fps_values: Vec<f32>,
}

impl<T: Hash + Eq> AnimationMachineBuilder<T> {
    pub fn new(sprite_size: Vector2) -> Self {
        Self {
            animation_machine: AnimationPlayer2D::new(sprite_size),
            states: Vec::new(),
            texture_strip_paths: Vec::new(),
            frame_amounts: Vec::new(),
            fps_values: Vec::new(),
        }
    }

    /// Insert new animations
    pub fn add_animation(&mut self, state: T, texture_strip_path: &str, frames: u32, fps: f32) {
        self.states.push(state);
        self.texture_strip_paths.push(texture_strip_path.into());
        self.frame_amounts.push(frames);
        self.fps_values.push(fps);
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
            self.animation_machine.add_animation(
                self.states.remove(0),
                texture_strip,
                self.frame_amounts.remove(0),
                self.fps_values.remove(0),
            );
        }
        self.animation_machine
    }
}
