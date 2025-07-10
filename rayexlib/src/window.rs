use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibHandle};

pub struct Window {
    pub color: [Color; 2],
    prev_size: (i32, i32),
}

impl Window {
    pub const DEFAULT_SIZE_SCALAR: f32 = 9.0 / 20.0;

    pub const fn new() -> Self {
        Self {
            color: [Color::WHITE, Color::MAROON],
            prev_size: (0, 0),
        }
    }

    pub fn save_size(&mut self, raylib: &mut RaylibHandle) {
        self.prev_size = (raylib.get_screen_width(), raylib.get_screen_height());
    }

    pub const fn prev_width(&self) -> i32 {
        self.prev_size.0
    }

    pub const fn prev_height(&self) -> i32 {
        self.prev_size.1
    }

    // draw window decorations
    pub fn draw(&self, raylib: &mut RaylibDrawHandle) {
        let width = raylib.get_screen_width();
        let height = raylib.get_screen_height();
        raylib.draw_rectangle_gradient_v(0, 0, 1, height, self.color[0], self.color[1]);
        raylib.draw_rectangle_gradient_v(width - 1, 0, 1, height, self.color[0], self.color[1]);
        raylib.draw_line(0, height - 1, width, height - 1, self.color[1]);
    }
}
