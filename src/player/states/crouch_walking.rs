use super::{Player, PlayerState::*};
use crate::math::Math;

pub fn on_enter(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    println!("crouch walk!");
}

pub fn on_exit(player: &mut Player, raylib: &mut raylib::RaylibHandle) {}

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
        reset_hitbox_from_crouch(player);
        player.transition(Running, raylib);
    } else if raylib.is_key_down(player.controls.up) {
        reset_hitbox_from_crouch(player);
        player.transition(Jumping, raylib);
    } else if player.collider.on_wall() && !player.collider.on_floor() {
        reset_hitbox_from_crouch(player);
        player.transition(WallSliding, raylib);
    }
}

pub fn reset_hitbox_from_crouch(player: &mut Player) {
    // move hitbox by offset of sizes
    player.collider.position.y -= Player::COLLISION_SIZE.y - Player::COLLISION_SIZE.x;
    // reset hitbox size
    player.collider.size = Player::COLLISION_SIZE;
}
