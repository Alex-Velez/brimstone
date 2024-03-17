use crate::player::Player;
use rayexlib::{
    prelude::{math, Math},
    state_manager::StateManager,
};
use raylib::prelude::RaylibHandle;

mod crouch_walking;
mod crouching;
mod diving;
mod falling;
mod idle;
mod jumping;
mod running;
mod wall_sliding;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub enum PlayerState {
    #[default]
    Idle,
    Running,
    Crouching,
    CrouchWalking,
    Jumping,
    Falling,
    WallSliding,
    Diving,
}

impl StateManager<PlayerState> for Player {
    fn update(player: &mut Player, raylib: &mut RaylibHandle) {
        match player.state {
            PlayerState::Idle => idle::update(player, raylib),
            PlayerState::Running => running::update(player, raylib),
            PlayerState::Crouching => crouching::update(player, raylib),
            PlayerState::CrouchWalking => crouch_walking::update(player, raylib),
            PlayerState::Jumping => jumping::update(player, raylib),
            PlayerState::Falling => falling::update(player, raylib),
            PlayerState::Diving => diving::update(player, raylib),
            PlayerState::WallSliding => wall_sliding::update(player, raylib),
        }
    }

    fn on_enter(player: &mut Player, raylib: &mut RaylibHandle) {
        match player.state {
            PlayerState::Idle => {}
            PlayerState::Running => {}
            PlayerState::Crouching => crouching::on_enter(player, raylib),
            PlayerState::CrouchWalking => crouch_walking::on_enter(player, raylib),
            PlayerState::Jumping => jumping::on_enter(player, raylib),
            PlayerState::Falling => {}
            PlayerState::Diving => {}
            PlayerState::WallSliding => wall_sliding::on_enter(player, raylib),
        }
    }

    fn on_exit(player: &mut Player, raylib: &mut RaylibHandle) {
        match player.state {
            PlayerState::Idle => {}
            PlayerState::Running => {}
            PlayerState::Crouching => crouching::on_exit(player, raylib),
            PlayerState::CrouchWalking => crouch_walking::on_exit(player, raylib),
            PlayerState::Jumping => {}
            PlayerState::Falling => {}
            PlayerState::Diving => {}
            PlayerState::WallSliding => wall_sliding::on_exit(player, raylib),
        }
    }

    fn next_state(player: &mut Player, next_state: PlayerState, raylib: &mut RaylibHandle) {
        StateManager::on_exit(player, raylib);
        player.state = next_state;
        StateManager::on_enter(player, raylib);
    }
}
