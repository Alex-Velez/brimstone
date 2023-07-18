use super::{Player, PlayerState::*};
use crate::math::Math;

pub fn on_enter(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    // reset jump animation
    player.animation_player.reset_frame(Jumping);

    // add jump force
    player.collider.velocity.y -= player.jump;
}

pub fn on_exit(player: &mut Player, raylib: &mut raylib::RaylibHandle) {}

pub fn update(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if player.move_dir.x == 0.0 {
        // stop velocity
        player.collider.velocity.x.lerp(
            0.0,
            player.air_friction * player.deceleration * player.frame_time,
        );
        // round small values to 0
        player.collider.velocity.x.round_zero();
    } else {
        // accelerate velocity to max speed
        player.collider.velocity.x.lerp(
            player.move_dir.x * player.max_speed,
            player.acceleration * player.frame_time,
        );
    }

    // next state
    check_next_state(player, raylib);
}

fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if raylib.is_key_down(player.controls.down) {
        player.transition(Diving, raylib);
    } else if player.collider.velocity.y > 0.0 {
        player.transition(Falling, raylib);
    } else if player.collider.on_floor() {
        if player.move_dir.x == 0.0 {
            player.transition(Idle, raylib);
        } else {
            player.transition(Running, raylib);
        }
    }
}
