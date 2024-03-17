use super::*;

pub fn on_enter(player: &mut Player, _raylib: &mut RaylibHandle) {
    if player.collider.size.y != Player::CROUCH_SIZE {
        // change hitbox height
        player.collider.size.y = Player::CROUCH_SIZE;
        // move hitbox by offset of sizes
        player.collider.position.y += Player::COLLISION_SIZE.y - Player::CROUCH_SIZE;
    }

    // set sprite to crouch offset
    player.animation_player.set_offset(Player::SPRITE_CR_OFFSET);
}

pub fn on_exit(player: &mut Player, _raylib: &mut raylib::prelude::RaylibHandle) {
    // reset sprite offset
    player.animation_player.set_offset(Player::SPRITE_OFFSET);
}

pub fn update(player: &mut Player, raylib: &mut raylib::prelude::RaylibHandle) {
    // calculate friction damping
    let g_friction = player.ground_friction * player.deceleration * player.frame_time;
    // stop velocity
    player.collider.velocity.x.lerp(0.0, g_friction);
    // round small values to 0
    player.collider.velocity.x.round_zero();

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
    if player.move_dir.x != 0.0 {
        StateManager::next_state(player, PlayerState::CrouchWalking, raylib);
    } else if !raylib.is_key_down(player.controls.down) {
        player.reset_hitbox_from_crouch();
        StateManager::next_state(player, PlayerState::Idle, raylib);
    } else if raylib.is_key_down(player.controls.up) {
        player.reset_hitbox_from_crouch();
        StateManager::next_state(player, PlayerState::Jumping, raylib);
    } else if player.collider.on_wall() && !player.collider.on_floor() {
        player.reset_hitbox_from_crouch();
        StateManager::next_state(player, PlayerState::WallSliding, raylib);
    }
}
