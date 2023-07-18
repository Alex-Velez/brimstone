use super::{Player, PlayerState::*};

pub fn on_enter(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    println!("wall slide!");

    if raylib.is_key_down(player.controls.up) {
        player.collider.velocity.y = 0.0;
    }
}

pub fn on_exit(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if raylib.is_key_down(player.controls.up) {
        println!("wall jump!");

        // add jump force from wall
        player.collider.velocity.x -= player.collider.colliding.x * player.jump * 1.5;
    }
}

pub fn update(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    player.collider.velocity.y = player.collider.velocity.y.min(player.max_speed);

    // next state
    check_next_state(player, raylib);
}

fn check_next_state(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if player.collider.on_floor() {
        if player.move_dir.x == 0.0 {
            player.transition(Idle, raylib);
        } else {
            player.transition(Running, raylib);
        }
    } else {
        if raylib.is_key_down(player.controls.up) && player.move_dir.x == 0.0 {
            player.transition(Jumping, raylib);
        }
    }
}