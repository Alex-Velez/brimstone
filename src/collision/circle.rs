use raylib::prelude::{Color, RaylibDraw, Vector2};

pub struct Circle {
    pub position: Vector2,
    pub radius: f32,
    pub colliding: bool,
}

impl Circle {
    pub const fn new(radius: f32) -> Circle {
        Circle {
            position: Vector2::new(0.0, 0.0),
            radius,
            colliding: false,
        }
    }

    /// Position circle to (x, y)
    pub fn set_position(mut self, x: f32, y: f32) -> Self {
        self.position.x = x;
        self.position.y = y;
        self
    }

    /// Reset colliding
    pub fn reset_colliding(&mut self) {
        self.colliding = false;
    }
}

impl Circle {
    /// Circle vs Circle collision check
    pub fn check(&self, circle2: &Circle) -> bool {
        (circle2.position.x - self.position.x).powi(2)
            + (circle2.position.y - self.position.y).powi(2)
            <= (self.radius + circle2.radius).powi(2)
    }

    /// Circle vs Circle collision resolution
    pub fn collide(&mut self, circle2: &mut Circle) -> bool {
        if self.check(circle2) {
            // get offset
            let displacement = Vector2::new(
                circle2.position.x - self.position.x,
                circle2.position.y - self.position.y,
            );
            let radii_sum = self.radius + circle2.radius;
            let length = self.position.distance_to(circle2.position);
            let unit = displacement / length;

            // resolve circle1 position
            self.position = circle2.position - unit * radii_sum;

            // set colliding
            self.colliding = true;
            circle2.colliding = true;
            true
        } else {
            self.colliding = false;
            circle2.colliding = false;
            false
        }
    }
}

impl Circle {
    pub fn draw(&self, color: Color, raylib: &mut impl RaylibDraw) {
        // outline
        raylib.draw_circle_lines(
            self.position.x as i32,
            self.position.y as i32,
            self.radius,
            color,
        );

        // center
        raylib.draw_circle_v(self.position, 10.0, color.fade(0.5));
    }
}
