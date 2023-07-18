use crate::{raylib_plugins::FrameLimiter, raylib_plugins::Texture2DPlugin};
use raylib::prelude::{
    Color, RaylibDraw, RaylibHandle, RaylibThread, Rectangle, Texture2D, Vector2,
};

pub struct ParallaxLayer2D {
    pub rect: Rectangle,
    pub rotation: f32,
    pub speed: Vector2,
    pub tint: Color,
    texture: Texture2D,
    source_rect: Rectangle,
}

impl ParallaxLayer2D {
    pub fn new(texture: Texture2D, speed: Vector2) -> ParallaxLayer2D {
        let size = Vector2::new(texture.width as f32, texture.height as f32);
        ParallaxLayer2D {
            rect: Rectangle::new(0.0, 0.0, size.x, size.y),
            rotation: 0.0,
            speed,
            tint: Color::WHITE,
            texture,
            source_rect: Rectangle::new(0.0, 0.0, size.x, size.y),
        }
    }

    pub fn from_path(
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        speed: Vector2,
    ) -> ParallaxLayer2D {
        let texture = Texture2D::from_path(raylib, &thread, path);
        ParallaxLayer2D::new(texture, speed)
    }

    pub fn maximize_to_screen(&mut self, raylib: &mut RaylibHandle) {
        self.rect.width = raylib.get_screen_width() as f32;
        self.rect.height = raylib.get_screen_height() as f32;
    }

    pub fn update(&mut self, raylib: &mut RaylibHandle) {
        // move texture
        self.source_rect.x += self.speed.x * raylib.get_frame_time_limited();
        self.source_rect.y += self.speed.y * raylib.get_frame_time_limited();
    }

    pub fn draw(&self, raylib: &mut impl RaylibDraw) {
        // draw main texture
        raylib.draw_texture_pro(
            &self.texture,
            self.source_rect,
            self.rect,
            Vector2 { x: 0.0, y: 0.0 },
            self.rotation,
            self.tint,
        );
    }
}
