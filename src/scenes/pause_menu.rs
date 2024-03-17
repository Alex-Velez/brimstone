use crate::scenes::GlobalEnvironment;
use rayexlib::prelude::Scene;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread};

const BACKGROUND: Color = Color::new(219, 95, 83, 255);

pub struct Environment {}

impl Environment {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {}
    }
}

impl Scene<GlobalEnvironment> for Environment {
    fn update(&mut self, global: &mut GlobalEnvironment, raylib: &mut RaylibHandle) {}

    fn draw(&self, global: &GlobalEnvironment, raylib: &mut RaylibDrawHandle) {
        // draw background color
        raylib.clear_background(BACKGROUND);
    }
}
