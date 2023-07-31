use super::{Player, PlayerState::*};
use raylib::prelude::Vector2;

pub fn on_enter(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if raylib.is_key_down(player.controls.up) {
        player.collider.velocity.y = 0.0;
    }
}

pub fn on_exit(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    if raylib.is_key_down(player.controls.up) {
        // add jump force from wall
        player.collider.velocity.x -= player.collider.colliding.x * player.jump * 1.5;
    }

    // update sprite
    player.animation_player.set_offset(Player::SPRITE_OFFSET);
    player.animation_player.flip_h();
}

pub fn update(player: &mut Player, raylib: &mut raylib::RaylibHandle) {
    player.collider.velocity.y = player.collider.velocity.y.min(player.max_speed);

    // face wall
    if player.collider.on_wall_right() {
        player.animation_player.face_right();
    } else if player.collider.on_wall_left() {
        player.animation_player.face_left();
    }

    // change sprite offset
    player.animation_player.set_offset(Vector2 {
        x: Player::SPRITE_OFFSET.x - (player.collider.colliding.x * Player::SPRITE_SL_SHIFT),
        y: Player::SPRITE_OFFSET.y,
    });

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
