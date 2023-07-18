use super::*;

mod enter;
mod exit;
mod update;

impl Default for StateMachine<PlayerState, Player> {
    fn default() -> Self {
        let mut state_machine = StateMachine::new();
        state_machine.add_state(
            PlayerState::Idle,
            Player::idle_update,
            Player::on_idle,
            Player::empty,
        );
        state_machine.add_state(
            PlayerState::Running,
            Player::running_update,
            Player::on_run,
            Player::empty,
        );
        state_machine.add_state(
            PlayerState::Crouching,
            Player::crouching_update,
            Player::on_crouch,
            Player::empty,
        );
        state_machine.add_state(
            PlayerState::CrouchWalking,
            Player::crouch_walking_update,
            Player::on_crouch_walk,
            Player::empty,
        );
        state_machine.add_state(
            PlayerState::Jumping,
            Player::jumping_update,
            Player::on_jump,
            Player::empty,
        );
        state_machine.add_state(
            PlayerState::Falling,
            Player::falling_update,
            Player::on_fall,
            Player::empty,
        );
        state_machine.add_state(
            PlayerState::WallSliding,
            Player::wall_sliding_update,
            Player::on_wall_slide,
            Player::exit_wall_sliding,
        );
        state_machine.add_state(
            PlayerState::Diving,
            Player::diving_update,
            Player::on_dive,
            Player::empty,
        );
        state_machine
    }
}
