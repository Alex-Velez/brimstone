use super::*;

pub fn update(player: &mut Player, raylib: &mut raylib::prelude::RaylibHandle) {
    // accelerate velocity to max speed
    player.collider.velocity.x.lerp(
        player.move_dir.x * player.max_speed,
        player.acceleration * player.frame_time,
    );

    // next state
    check_next_state(player, raylib);
}

#[inline]
fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if player.collider.on_floor() {
        if player.move_dir.x == 0.0 {
            StateManager::next_state(player, PlayerState::Idle, raylib);
        } else if raylib.is_key_down(player.controls.down) {
            StateManager::next_state(player, PlayerState::Crouching, raylib);
        } else if raylib.is_key_down(player.controls.up) {
            StateManager::next_state(player, PlayerState::Jumping, raylib);
        }
    } else {
        StateManager::next_state(player, PlayerState::Falling, raylib)
    }
}
