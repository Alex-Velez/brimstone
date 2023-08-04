use raylib::prelude::{
    Color, MouseButton::MOUSE_LEFT_BUTTON, RaylibDraw, RaylibHandle, Rectangle, Texture2D, Vector2,
};

pub struct Button {
    pub rect: Rectangle,
    pub texture: Option<Texture2D>,
    pub color: Color,
}

impl Button {
    pub fn new(width: f32, height: f32) -> Self {
        Button {
            rect: Rectangle::new(0.0, 0.0, width, height),
            texture: None,
            color: Color::WHITE,
        }
    }

    pub fn set_position(mut self, x: f32, y: f32) -> Self {
        self.rect.x = x;
        self.rect.y = y;
        self
    }

    pub fn set_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn on_hover(&self, mouse_position: Vector2) -> bool {
        self.rect.check_collision_point_rec(mouse_position)
    }

    pub fn on_down(&self, mouse_position: Vector2, raylib: &mut RaylibHandle) -> bool {
        self.on_hover(mouse_position) && raylib.is_mouse_button_down(MOUSE_LEFT_BUTTON)
    }

    pub fn on_press(&self, mouse_position: Vector2, raylib: &mut RaylibHandle) -> bool {
        self.on_hover(mouse_position) && raylib.is_mouse_button_pressed(MOUSE_LEFT_BUTTON)
    }

    pub fn on_release(&self, mouse_position: Vector2, raylib: &mut RaylibHandle) -> bool {
        self.on_hover(mouse_position) && raylib.is_mouse_button_released(MOUSE_LEFT_BUTTON)
    }

    pub fn draw(&self, raylib: &mut impl RaylibDraw) {
        if let Some(tex) = &self.texture {
        } else {
            raylib.draw_rectangle_rec(self.rect, self.color);
        }
    }
}
