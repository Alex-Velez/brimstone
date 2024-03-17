use crate::player::Player;
use rayexlib::prelude::{CameraEx2D, Init};
use raylib::{RaylibHandle, RaylibThread};

pub struct GlobalEnvironment {
    pub camera: CameraEx2D,
    pub player: Player,
}

impl GlobalEnvironment {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            camera: CameraEx2D::default().with_move_speed(7.0),
            player: Player::init(raylib, thread),
        }
    }
}
