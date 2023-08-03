use crate::{
    raylib_plugins::FrameLimiter, raylib_plugins::Texture2DPlugin, sprite::SpriteTransform,
};
use raylib::prelude::{RaylibDraw, RaylibHandle, RaylibThread, Texture2D, Vector2};

pub struct ParallaxLayer2D {
    pub speed: Vector2,
    texture: Texture2D,
    transform: SpriteTransform,
}

impl ParallaxLayer2D {
    pub fn new(texture: Texture2D, speed: Vector2) -> ParallaxLayer2D {
        let width = texture.width as f32;
        let height = texture.height as f32;
        ParallaxLayer2D {
            speed,
            texture,
            transform: SpriteTransform::new(width, height),
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
        self.transform.set_size(
            raylib.get_screen_width() as f32,
            raylib.get_screen_height() as f32,
        );
    }

    pub fn update(&mut self, raylib: &mut RaylibHandle) {
        let frame_time = raylib.get_frame_time_limited();
        self.transform.source_rect.x += self.speed.x * frame_time;
        self.transform.source_rect.y += self.speed.y * frame_time;
    }

    pub fn draw(&self, raylib: &mut impl RaylibDraw) {
        raylib.draw_texture_pro(
            &self.texture,
            self.transform.source_rect,
            self.transform.rect,
            self.transform.offset,
            self.transform.rotation,
            self.transform.tint,
        );
    }
}
