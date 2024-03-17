use super::*;

pub fn on_enter(player: &mut Player, _raylib: &mut raylib::prelude::RaylibHandle) {
    // reset jump animation
    player.animation_player.reset_frame(&PlayerState::Jumping);

    // add jump force
    player.collider.velocity.y -= player.jump;
}

pub fn update(player: &mut Player, raylib: &mut raylib::prelude::RaylibHandle) {
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

#[inline]
fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if raylib.is_key_down(player.controls.down) {
        StateManager::next_state(player, PlayerState::Diving, raylib);
    } else if player.collider.velocity.y > 0.0 {
        StateManager::next_state(player, PlayerState::Falling, raylib);
    } else if player.collider.on_floor() {
        if player.move_dir.x == 0.0 {
            StateManager::next_state(player, PlayerState::Idle, raylib);
        } else {
            StateManager::next_state(player, PlayerState::Running, raylib);
        }
    }
}
