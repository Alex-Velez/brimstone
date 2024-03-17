use super::*;

pub fn on_enter(player: &mut Player, _raylib: &mut RaylibHandle) {
    // set sprite to crouch offset
    player.animation_player.set_offset(Player::SPRITE_CR_OFFSET);
}

pub fn on_exit(player: &mut Player, _raylib: &mut RaylibHandle) {
    // reset sprite offset
    player.animation_player.set_offset(Player::SPRITE_OFFSET);
}

pub fn update(player: &mut Player, raylib: &mut RaylibHandle) {
    // accelerate velocity to crouch walk speed
    player.collider.velocity.x.lerp(
        player.move_dir.x * (player.max_speed / 4.0),
        player.acceleration * player.frame_time,
    );

    // player ledge to wall slide transition
    if player.collider.on_floor() && player.move_dir.y == 1.0 && !player.ground_ray.is_colliding() {
        // move collider, force collision resolution to wall
        player.collider.position.y += Player::CROUCH_SIZE / 2.0;
        player.collider.reset_colliding();
        player.reset_hitbox_from_crouch();

        // self.transition(PlayerState::WallSliding, raylib);
        StateManager::next_state(player, PlayerState::WallSliding, raylib);
    } else {
        // next state
        check_next_state(player, raylib);
    }
}

#[inline]
fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if player.move_dir.x == 0.0 {
        StateManager::next_state(player, PlayerState::Crouching, raylib);
    } else if !raylib.is_key_down(player.controls.down) {
        player.reset_hitbox_from_crouch();
        StateManager::next_state(player, PlayerState::Running, raylib);
    } else if raylib.is_key_down(player.controls.up) {
        player.reset_hitbox_from_crouch();
        StateManager::next_state(player, PlayerState::Jumping, raylib);
    } else if player.collider.on_wall() && !player.collider.on_floor() {
        player.reset_hitbox_from_crouch();
        StateManager::next_state(player, PlayerState::WallSliding, raylib);
    }
}
