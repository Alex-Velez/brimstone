use crate::{
    paths::player::advn,
    player::{states::*, Player, PlayerState},
    sprite::{AnimationMachineBuilder, AnimationPlayer2D},
    state_machine::StateMachine,
};
use raylib::prelude::{RaylibHandle, RaylibThread};

pub trait PlayerDefault {
    fn player_default(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self;
}

/// Default configuration for player AnimationPlayer
impl PlayerDefault for AnimationPlayer2D<PlayerState> {
    fn player_default(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // add animations
        let mut b = AnimationMachineBuilder::new(Player::SPRITE_SIZE);
        b.add_animation(PlayerState::Idle, advn::IDLE, 4, Player::FPS_IDLE);
        b.add_animation(PlayerState::Running, advn::RUN, 6, Player::FPS_RUN);
        b.add_animation(PlayerState::Jumping, advn::JUMP, 4, Player::FPS_JUMP);
        b.add_animation(PlayerState::Falling, advn::FALL, 2, Player::FPS_FALL);
        b.add_animation(PlayerState::Crouching, advn::CRID, 4, Player::FPS_CRID);
        b.add_animation(PlayerState::CrouchWalking, advn::CRWK, 6, Player::FPS_CRWK);
        b.add_animation(PlayerState::Diving, advn::FALL, 2, Player::FPS_DIVE);
        b.add_animation(PlayerState::WallSliding, advn::WSLD, 2, Player::FPS_WSLD);

        // build animation player
        let mut m = b.build(raylib, thread);

        // resize all animations
        m.set_scale(Player::SPRITE_SCALE);
        m.set_offset(Player::SPRITE_OFFSET);

        m
    }
}

/// Default configuration for player states
impl PlayerDefault for StateMachine<PlayerState, Player> {
    fn player_default(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // add states to state machine
        let mut machine = StateMachine::default();
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
