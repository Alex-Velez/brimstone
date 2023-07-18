#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum PlayerState {
    // normal states
    Idle,
    Running,
    Crouching,
    CrouchWalking,
    Jumping,
    Falling,
    WallSliding,
    Diving,
}
