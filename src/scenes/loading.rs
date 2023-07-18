use crate::{paths, scene_machine::Scene, sprite::Sprite2D};
use raylib::prelude::{
    Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread, Rectangle, Vector2,
};

/// Loading constant values
const BACKGROUND_TOP_COLOR: Color = Color::MAROON;
const BACKGROUND_BOTTOM_COLOR: Color = Color::new(10, 10, 10, 255);
const LOADING_ICON_ROTATION_SPEED: f64 = 360.0;

pub struct Environment {
    icon: Sprite2D,
}

impl Environment {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            icon: Sprite2D::from_path(raylib, &thread, paths::loading::ICON),
        }
    }
}

impl Scene for Environment {
    fn on_enter(&mut self, raylib: &mut RaylibHandle) {
        raylib.hide_cursor();
    }

    fn on_exit(&mut self, raylib: &mut RaylibHandle) {
        raylib.show_cursor();
    }

    fn update(&mut self, raylib: &mut RaylibHandle) {
        let win_width = raylib.get_screen_width() as f32;
        let win_height = raylib.get_screen_height() as f32;

        // rotate loading sprite
        self.icon.rotation = (raylib.get_time() * LOADING_ICON_ROTATION_SPEED) as f32;

        // update sprite size and position
        let icon_size = win_width / 16.0;
        self.icon.rect = Rectangle {
            x: self.icon.rect.width,
            y: win_height - self.icon.rect.height,
            width: icon_size,
            height: icon_size,
        };

        // set offset
        self.icon.offset = Vector2::new(self.icon.rect.width / 2.0, self.icon.rect.width / 2.0);
    }

    fn draw(&self, raylib: &mut RaylibDrawHandle) {
        let win_width = raylib.get_screen_width() as f32;
        let win_height = raylib.get_screen_height() as f32;

        // background gradient
        raylib.draw_rectangle_gradient_v(
            0,
            0,
            win_width as i32,
            win_height as i32,
            BACKGROUND_TOP_COLOR,
            BACKGROUND_BOTTOM_COLOR,
        );

        // loading text
        let time = raylib.get_time() as f32;
        let amount = (((1.5 * time.sin()) + 1.5).round()) as usize;
        let txt = &format!("Loading{}", ".".repeat(amount));
        raylib.draw_text(
            txt,
            (win_width / 16.0) as i32,
            (win_height / 16.0) as i32,
            (win_width / 10.0) as i32,
            Color::WHITE,
        );

        // loading sprite
        self.icon.draw(raylib);
    }
}
