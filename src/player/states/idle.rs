use rayexlib::state_manager::StateManager;

use super::*;

pub fn update(player: &mut Player, raylib: &mut RaylibHandle) {
    // calculate friction damping
    let g_friction = player.ground_friction * player.deceleration * player.frame_time;

    // stop velocity
    player.collider.velocity.x.lerp(0.0, g_friction);

    // round small values to 0
    player.collider.velocity.x.round_zero();

    // update idle animation speed
    let fps_rate = math::lerp(
        Player::FPS_IDLE_TIRED,
        Player::FPS_IDLE,
        player.stamina / player.max_stamina,
    );

    // dynamic anim speed
    player.animation_player.set_fps(PlayerState::Idle, fps_rate);

    // next state
    check_next_state(player, raylib);
}

#[inline]
fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if player.collider.on_floor() {
        if player.move_dir.x != 0.0 {
            StateManager::next_state(player, PlayerState::Running, raylib);
        } else if raylib.is_key_down(player.controls.up) {
            StateManager::next_state(player, PlayerState::Jumping, raylib);
        } else if raylib.is_key_down(player.controls.down) {
            StateManager::next_state(player, PlayerState::Crouching, raylib);
        }
    } else {
        StateManager::next_state(player, PlayerState::Falling, raylib);
    }
}
