use super::{Player, PlayerState::*};
use crate::math::Math;

pub fn on_enter(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    println!("fall!");
}

pub fn on_exit(player: &mut Player, raylib: &mut raylib::RaylibHandle) {}

pub fn update(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
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

fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    let a = player.collider.on_floor();
    let b = player.collider.on_wall();
    let c = player.move_dir.x == 0.0;
    let d = raylib.is_key_down(player.controls.down);

    match (a, b, c, d) {
        (true, _, true, false) => player.transition(Idle, raylib),
        (true, _, false, false) => player.transition(Running, raylib),
        (true, _, true, true) => player.transition(Crouching, raylib),
        (true, _, false, true) => player.transition(CrouchWalking, raylib),
        (false, true, _, false) => player.transition(WallSliding, raylib),
        (false, _, _, true) => player.transition(Diving, raylib),
        _ => {}
    };
}
