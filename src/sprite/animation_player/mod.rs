use crate::{
    paths::player::advn,
    player::{Player, PlayerState},
    timer::Timer,
};
use raylib::prelude::{
    Color, RaylibDraw, RaylibHandle, RaylibThread, Rectangle, Texture2D, Vector2,
};
use std::{collections::HashMap, hash::Hash};

mod builder;

use builder::AnimationMachineBuilder;

pub struct AnimationPlayer2D<T> {
    pub rect: Rectangle,
    pub tint: Color,
    pub rotation: f32,
    pub offset: Vector2,
    animations: HashMap<T, AnimationStrip>,
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

/// Animated Sprite Transform
impl<T: Hash + Eq> AnimationPlayer2D<T> {
    /// Set sprite rect
    pub fn set_rect(&mut self, rect: Rectangle) {
        self.rect = rect;
    }

    /// Set sprite position
    pub fn set_position(&mut self, position: Vector2) {
        self.rect.x = position.x;
        self.rect.y = position.y;
    }

    /// Set sprite offset
    pub fn set_offset(&mut self, offset: Vector2) {
        self.offset = offset;
    }

    /// resize sprite
    pub fn set_size(&mut self, size: Vector2) {
        self.rect.width = size.x;
        self.rect.height = size.y;
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

impl AnimationPlayer2D<PlayerState> {
    pub fn player(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // add animations
        let mut b = AnimationMachineBuilder::new(Player::SPRITE_SIZE);
        b.add_animation(PlayerState::Idle, advn::IDLE, 4, 4.0);
        b.add_animation(PlayerState::Idle, advn::IDLE, 4, Player::FPS_IDLE);
        b.add_animation(PlayerState::Running, advn::RUN, 6, Player::FPS_RUN);
        b.add_animation(PlayerState::Jumping, advn::JUMP, 4, Player::FPS_JUMP);
        b.add_animation(PlayerState::Falling, advn::FALL, 2, Player::FPS_FALL);
        b.add_animation(PlayerState::Crouching, advn::CRID, 4, Player::FPS_CRID);
        b.add_animation(PlayerState::CrouchWalking, advn::CRWK, 6, Player::FPS_CRWK);
        b.add_animation(PlayerState::Diving, advn::FALL, 2, Player::FPS_DIVE);
        b.add_animation(PlayerState::WallSliding, advn::WSLD, 2, Player::FPS_WSLD);

        // build animation player
        let mut m = b.build(raylib, thread);

        // resize all animations
        m.set_scale(Player::SPRITE_SCALE);
        m.set_offset(Player::SPRITE_OFFSET);

        m
    }
}

pub struct AnimationStrip {
    frame: u32,
    frames: u32,
    fps_timer: Timer,
    texture_strip: Texture2D,
}
