use crate::{
    paths::player::advn,
    player::{Player, PlayerState},
    sprite::AnimatedSprite2D,
};
use raylib::prelude::{RaylibDraw, RaylibHandle, RaylibThread, Rectangle, Texture2D, Vector2};
use std::{collections::HashMap, hash::Hash};

mod builder;

pub use builder::AnimationPlayerBuilder;

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

impl AnimationPlayer2D<PlayerState> {
    pub fn player(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // load and insert textures into animation player
        let mut animation_player = AnimationPlayerBuilder::<PlayerState>::new()
            .add_animation(PlayerState::Idle, advn::IDLE, 4, Player::FPS_IDLE)
            // .add_animation(PlayerState::Walking, advn::WALK, 6, Player::FPS_WALK)
            .add_animation(PlayerState::Running, advn::RUN, 6, Player::FPS_RUN)
            .add_animation(PlayerState::Jumping, advn::JUMP, 4, Player::FPS_JUMP)
            .add_animation(PlayerState::Falling, advn::FALL, 2, Player::FPS_FALL)
            .add_animation(PlayerState::Crouching, advn::CRID, 4, Player::FPS_CRID)
            .add_animation(PlayerState::CrouchWalking, advn::CRWK, 6, Player::FPS_CRWK)
            .add_animation(PlayerState::Diving, advn::FALL, 2, Player::FPS_DIVE)
            .add_animation(PlayerState::WallSliding, advn::WSLD, 2, Player::FPS_WSLD)
            .build(raylib, thread);

        // resize all animations
        animation_player.set_scale(Player::SPRITE_SCALE);
        animation_player.set_offset(Player::SPRITE_OFFSET);

        animation_player
    }
}
