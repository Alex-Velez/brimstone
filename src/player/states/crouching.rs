use super::{Player, PlayerState::*};
use crate::math::Math;

pub fn on_enter(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if player.collider.size.y != Player::CROUCH_SIZE {
        // change hitbox height
        player.collider.size.y = Player::CROUCH_SIZE;
        // move hitbox by offset of sizes
        player.collider.position.y += Player::COLLISION_SIZE.y - Player::CROUCH_SIZE;
    }
}

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
    if player.move_dir.x != 0.0 {
        player.transition(CrouchWalking, raylib);
    } else if !raylib.is_key_down(player.controls.down) {
        player.reset_hitbox_from_crouch();
        player.transition(Idle, raylib);
    } else if raylib.is_key_down(player.controls.up) {
        player.reset_hitbox_from_crouch();
        player.transition(Jumping, raylib);
    } else if player.collider.on_wall() && !player.collider.on_floor() {
        player.reset_hitbox_from_crouch();
        player.transition(WallSliding, raylib);
    }
}
