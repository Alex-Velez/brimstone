use super::{Player, PlayerState::*};
use crate::math::Math;

pub fn on_enter(player: &mut Player, raylib: &mut raylib::RaylibHandle) {}

pub fn on_exit(player: &mut Player, raylib: &mut raylib::RaylibHandle) {}

pub fn update(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    // caclulate friction damping
    let g_friction = player.ground_friction * player.deceleration * player.frame_time;
    // stop velocity
    player.collider.velocity.x.lerp(0.0, g_friction);
    // round small values to 0
    player.collider.velocity.x.round_zero();

    // next state
    check_next_state(player, raylib);
}

fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if player.collider.on_floor() {
        if player.move_dir.x != 0.0 {
            player.transition(Running, raylib);
        } else if raylib.is_key_down(player.controls.up) {
            player.transition(Jumping, raylib);
        } else if raylib.is_key_down(player.controls.down) {
            player.transition(Crouching, raylib);
        }
    } else {
        player.transition(Falling, raylib);
    }
}
