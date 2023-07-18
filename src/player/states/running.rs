use super::{Player, PlayerState::*};
use crate::math::Math;

pub fn on_enter(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    println!("ran!");
}

pub fn on_exit(player: &mut Player, raylib: &mut raylib::RaylibHandle) {}

pub fn update(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    // accelerate velocity to max speed
    player.collider.velocity.x.lerp(
        player.move_dir.x * player.max_speed,
        player.acceleration * player.frame_time,
    );

    // next state
    check_next_state(player, raylib);
}

fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if player.collider.on_floor() {
        if player.move_dir.x == 0.0 {
            player.transition(Idle, raylib);
        } else if raylib.is_key_down(player.controls.down) {
            player.transition(Crouching, raylib);
        } else if raylib.is_key_down(player.controls.up) {
            player.transition(Jumping, raylib);
        }
    } else {
        player.transition(Falling, raylib);
    }
}
