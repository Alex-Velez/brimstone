use super::Rect;
use raylib::prelude::{Color, RaylibDraw, Vector2};

pub struct Ray2D {
    pub start: Vector2,
    pub end: Vector2,
}

impl Ray2D {
    pub const fn new() -> Self {
        Ray2D {
            start: Vector2::new(0.0, 0.0),
            end: Vector2::new(0.0, 0.0),
        }
    }

    pub const fn from(start: Vector2, end: Vector2) -> Self {
        Self { start, end }
    }

    pub fn with_start(&mut self, start: Vector2) {
        self.start = start;
    }

    pub fn with_end(&mut self, end: Vector2) {
        self.end = end;
    }

    /// Draw ray line
    pub fn draw(&self, color: Color, raylib: &mut impl RaylibDraw) {
        // line
        raylib.draw_line_v(self.start, self.end, color);

        // position
        raylib.draw_circle_v(self.start, 10.0, color.fade(0.5));
    }
}

impl Ray2D {
    fn collide_rect(&mut self, rect: &Rect) -> bool {
        false
    }
}

/*
pub struct Ray2D {
    pub position: Vector2,
    pub direction: Vector2,
    pub contact_point: Vector2,
    pub contact_normal: Vector2,
    pub contact_time: f32,
}

impl Ray2D {
    pub const fn new() -> Ray2D {
        Ray2D {
            position: Vector2::new(0.0, 0.0),
            direction: Vector2::new(0.0, 0.0),
            contact_point: Vector2::new(0.0, 0.0),
            contact_normal: Vector2::new(0.0, 0.0),
            contact_time: 0.0,
        }
    }

    pub fn with_direction() {}

    /// Draw ray line
    pub fn draw(&self, color: Color, raylib: &mut impl RaylibDraw) {
        // line
        raylib.draw_line_v(self.position, self.position + self.direction, color);

        // position
        raylib.draw_circle_v(self.position, 10.0, color.fade(0.5));
    }
}

impl Ray2D {
    fn collide_rect(&mut self, rect: &mut Rect) -> bool {
        // Calculate intersections with rectangle bounding axes
        let mut t_near = (rect.position - self.position) / self.direction;
        let mut t_far = (rect.position + rect.size - self.position) / self.direction;

        // Sort distances
        if t_near.x > t_far.x {
            std::mem::swap(&mut t_near.x, &mut t_far.x);
        }
        if t_near.y > t_far.y {
            std::mem::swap(&mut t_near.y, &mut t_far.y);
        }

        // Early rejection
        if t_near.x > t_far.y || t_near.y > t_far.x {
            return false;
        }

        // Closest 'time' will be the first contact (t_hit_near)
        self.contact_time = t_near.x.max(t_near.y);

        // Furthest 'time' is contact on opposite side of target
        let t_hit_far = t_far.x.min(t_far.y);

        // Reject if ray direction is pointing away from object
        if t_hit_far < 0.0 {
            return false;
        }

        // Contact point of collision from parametric line equation
        self.contact_point = self.position + self.direction * self.contact_time;

        // Normal vecter of contact point
        if t_near.x > t_near.y {
            if self.direction.x < 0.0 {
                self.contact_normal = Vector2::new(1.0, 0.0);
            } else {
                self.contact_normal = Vector2::new(-1.0, 0.0);
            }
        } else if t_near.x < t_near.y {
            if self.direction.y < 0.0 {
                self.contact_normal = Vector2::new(0.0, 1.0);
            } else {
                self.contact_normal = Vector2::new(0.0, -1.0);
            }
        }

        // Note if t_near == t_far, collision is principly in a diagonal
        // so pointless to resolve. By returning a CN={0,0} even though its
        // considered a hit, the resolver wont change anything.
        return self.contact_time >= 0.0 && self.contact_time <= 1.0;
    }
}
*/
