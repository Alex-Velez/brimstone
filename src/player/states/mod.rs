use self::PlayerState::*;
use super::Player;
use crate::engine::{math, prelude::Math};

pub(crate) mod crouch_walking;
pub(crate) mod crouching;
pub(crate) mod diving;
pub(crate) mod falling;
pub(crate) mod idle;
pub(crate) mod jumping;
pub(crate) mod running;
pub(crate) mod wall_sliding;

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
