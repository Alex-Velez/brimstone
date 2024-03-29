use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, Vector2};

pub struct Window {
    pub title: &'static str,
    pub color: Color,
    prev_size: (i32, i32),
}

impl Window {
    pub const DEFAULT_TITLE: &'static str = "8-Bit Brawl!";
    pub const DEFAULT_WIDTH: i32 = 856;
    pub const DEFAULT_HEIGHT: i32 = 482;
    pub const DEFAULT_SIZE: Vector2 = Vector2::new(856.0, 482.0);
    pub const BORDER_TOP: Color = Color::WHITE;
    pub const BORDER_BOTTOM: Color = Color::MAROON;

    pub const fn new() -> Self {
        Self {
            title: Self::DEFAULT_TITLE,
            color: Color::MAROON,
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
        raylib.draw_rectangle_gradient_v(0, 0, 1, height, Self::BORDER_TOP, Self::BORDER_BOTTOM);
        raylib.draw_rectangle_gradient_v(
            width - 1,
            0,
            1,
            height,
            Self::BORDER_TOP,
            Self::BORDER_BOTTOM,
        );
        raylib.draw_line(0, height - 1, width, height - 1, Self::BORDER_BOTTOM);
    }
}
