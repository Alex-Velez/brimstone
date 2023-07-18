use crate::scene::Scene;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread};

const BACKGROUND: Color = Color::new(219, 95, 83, 255);

pub struct Environment {}

impl Environment {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {}
    }
}

impl Scene for Environment {
    fn on_enter(&mut self, raylib: &mut RaylibHandle) {}

    fn on_exit(&mut self, raylib: &mut RaylibHandle) {}

    fn update(&mut self, raylib: &mut RaylibHandle) {}

    fn draw(&self, raylib: &mut RaylibDrawHandle) {
        // draw background color
        raylib.clear_background(BACKGROUND);
    }

    fn debug(&self, raylib: &mut RaylibDrawHandle) {}

    fn id(&self) -> String {
        String::from("PauseMenu")
    }
}
