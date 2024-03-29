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

    // add downward velocity
    player.collider.velocity.y += player.gravity * player.dive * player.frame_time;

    // next state
    check_next_state(player, raylib);
}

#[inline]
fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    let a = player.collider.on_floor();
    let b = player.move_dir.x == 0.0;
    let c = raylib.is_key_down(player.controls.down);

    match (a, b, c) {
        (true, true, _) => StateManager::next_state(player, PlayerState::Idle, raylib),
        (true, false, _) => StateManager::next_state(player, PlayerState::Running, raylib),
        (false, _, false) => StateManager::next_state(player, PlayerState::Falling, raylib),
        _ => {}
    };
}
