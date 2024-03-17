use raylib::{drawing::RaylibDraw, RaylibHandle, RaylibThread};

/// This trait implies that the object
/// is unique and only initilized once.
pub trait Init {
    fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self;
}

/// This trait implies that the object
/// has an update & draw loop.
pub trait Renderable {
    fn update(&mut self, raylib: &mut RaylibHandle);
    fn draw(&self, raylib: &mut impl RaylibDraw);
}
