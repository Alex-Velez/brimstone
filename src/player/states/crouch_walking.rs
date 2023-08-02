use super::{Player, PlayerState::*};
use crate::{math::Math, sprite::SpriteTransform};

pub fn on_enter(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    // set sprite to crouch offset
    player.animation_player.set_offset(Player::SPRITE_CR_OFFSET);
}

pub fn on_exit(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    // reset sprite offset
    player.animation_player.set_offset(Player::SPRITE_OFFSET);
}

pub fn update(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    // accelerate velocity to crouch walk speed
    player.collider.velocity.x.lerp(
        player.move_dir.x * (player.max_speed / 4.0),
        player.acceleration * player.frame_time,
    );

    // next state
    check_next_state(player, raylib);
}

fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if player.move_dir.x == 0.0 {
        player.transition(Crouching, raylib);
    } else if !raylib.is_key_down(player.controls.down) {
        player.reset_hitbox_from_crouch();
        player.transition(Running, raylib);
    } else if raylib.is_key_down(player.controls.up) {
        player.reset_hitbox_from_crouch();
        player.transition(Jumping, raylib);
    } else if player.collider.on_wall() && !player.collider.on_floor() {
        player.reset_hitbox_from_crouch();
        player.transition(WallSliding, raylib);
    }
}
