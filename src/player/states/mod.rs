use crate::{player::Player, state_machine::StateMachine};
use raylib::prelude::{RaylibHandle, RaylibThread};

mod crouch_walking;
mod crouching;
mod diving;
mod falling;
mod idle;
mod jumping;
mod running;
mod wall_sliding;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
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

/// Configuration for player states
impl StateMachine<PlayerState, Player> {
    pub fn player(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // add states to state machine
        let mut machine = StateMachine::default();
        machine.enter.insert(PlayerState::Idle, idle::on_enter);
        machine.add_state(
            PlayerState::Idle,
            idle::update,
            idle::on_enter,
            idle::on_exit,
        );
        machine.add_state(
            PlayerState::Running,
            running::update,
            running::on_enter,
            running::on_exit,
        );
        machine.add_state(
            PlayerState::Crouching,
            crouching::update,
            crouching::on_enter,
            crouching::on_exit,
        );
        machine.add_state(
            PlayerState::CrouchWalking,
            crouch_walking::update,
            crouch_walking::on_enter,
            crouch_walking::on_exit,
        );
        machine.add_state(
            PlayerState::Jumping,
            jumping::update,
            jumping::on_enter,
            jumping::on_exit,
        );
        machine.add_state(
            PlayerState::Falling,
            falling::update,
            falling::on_enter,
            falling::on_exit,
        );
        machine.add_state(
            PlayerState::WallSliding,
            wall_sliding::update,
            wall_sliding::on_enter,
            wall_sliding::on_exit,
        );
        machine.add_state(
            PlayerState::Diving,
            diving::update,
            diving::on_enter,
            diving::on_exit,
        );
        machine
    }
}
