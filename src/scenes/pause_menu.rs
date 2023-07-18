use crate::scene_machine::Scene;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread};

const BACKGROUND: Color = Color::new(219, 95, 83, 255);

pub struct Environment {}

impl Environment {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {}
    }
}

impl Scene for Environment {
    fn update(&mut self, raylib: &mut RaylibHandle) {}

    fn draw(&self, raylib: &mut RaylibDrawHandle) {
        // draw background color
        raylib.clear_background(BACKGROUND);
    }
}
