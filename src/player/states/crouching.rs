use super::{Player, PlayerState::*};
use crate::math::Math;

pub fn on_enter(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    println!("crouched!");

    if player.collider.size.y != Player::COLLISION_SIZE.x {
        // change hitbox height
        player.collider.size.y = Player::COLLISION_SIZE.x;
        // move hitbox by offset of sizes
        player.collider.position.y += Player::COLLISION_SIZE.y - Player::COLLISION_SIZE.x;
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
        reset_hitbox_from_crouch(player);
        player.transition(Idle, raylib);
    } else if raylib.is_key_down(player.controls.up) {
        reset_hitbox_from_crouch(player);
        player.transition(Jumping, raylib);
    } else if player.collider.on_wall() && !player.collider.on_floor() {
        reset_hitbox_from_crouch(player);
        player.transition(WallSliding, raylib);
    }
}

pub fn reset_hitbox_from_crouch(player: &mut Player) {
    // move hitbox by offset of sizes
    player.collider.position.y -= Player::COLLISION_SIZE.y - Player::COLLISION_SIZE.x;
    // reset hitbox size
    player.collider.size = Player::COLLISION_SIZE;
}
