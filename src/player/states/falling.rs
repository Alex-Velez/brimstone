use rayexlib::state_manager::StateManager;

use super::*;

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
    if player.collider.on_floor() {
        match (
            player.move_dir.x == 0.0,
            raylib.is_key_down(player.controls.down),
        ) {
            (true, true) => StateManager::next_state(player, PlayerState::Crouching, raylib),
            (true, false) => StateManager::next_state(player, PlayerState::Idle, raylib),
            (false, true) => StateManager::next_state(player, PlayerState::CrouchWalking, raylib),
            (false, false) => StateManager::next_state(player, PlayerState::Running, raylib),
        }
    } else {
        if player.collider.on_wall() {
            StateManager::next_state(player, PlayerState::WallSliding, raylib);
        } else if raylib.is_key_down(player.controls.down) {
            StateManager::next_state(player, PlayerState::Diving, raylib);
        }
    }
}
