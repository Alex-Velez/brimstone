use super::ColliderInfo;
use raylib::prelude::{Color, RaylibDraw, Rectangle, Vector2};

#[derive(PartialEq)]
pub enum CollisionDirection {
    Right,
    Left,
    Up,
    Down,
}

pub struct Rect2D {
    pub position: Vector2,
    pub size: Vector2,
    pub velocity: Vector2,
    pub colliding: Vec<CollisionDirection>,
}

impl Rect2D {
    pub const fn new(width: f32, height: f32) -> Self {
        Rect2D {
            position: Vector2::new(0.0, 0.0),
            size: Vector2::new(width, height),
            velocity: Vector2::new(0.0, 0.0),
            colliding: Vec::new(),
        }
    }

    pub const fn newv(size: Vector2) -> Self {
        Rect2D {
            position: Vector2::new(0.0, 0.0),
            size,
            velocity: Vector2::new(0.0, 0.0),
            colliding: Vec::new(),
        }
    }

    pub const fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position.x = x;
        self.position.y = y;
        self
    }

    pub fn with_position_center(mut self, x: f32, y: f32) -> Self {
        self.position.x = x - (self.size.x / 2.0);
        self.position.y = y - (self.size.y / 2.0);
        self
    }

    pub fn on_floor(&self) -> bool {
        self.colliding.contains(&CollisionDirection::Down)
    }

    pub fn on_roof(&self) -> bool {
        self.colliding.contains(&CollisionDirection::Up)
    }

    pub fn on_wall(&self) -> bool {
        self.colliding
            .iter()
            .any(|x| x == &CollisionDirection::Left || x == &CollisionDirection::Right)
    }

    pub fn on_wall_left(&self) -> bool {
        self.colliding.contains(&CollisionDirection::Left)
    }

    pub fn on_wall_right(&self) -> bool {
        self.colliding.contains(&CollisionDirection::Right)
    }

    pub fn direction(&self) -> Vector2 {
        let mut v = Vector2::new(0.0, 0.0);
        for x in &self.colliding {
            match x {
                CollisionDirection::Up => v.y -= 1.0,
                CollisionDirection::Down => v.y += 1.0,
                CollisionDirection::Left => v.x -= 1.0,
                CollisionDirection::Right => v.x += 1.0,
            }
        }
        v.normalized()
    }

    pub fn reset_colliding(&mut self) {
        self.colliding.clear()
    }
}

impl Rect2D {
    pub fn draw(&self, color: Color, raylib: &mut impl RaylibDraw) {
        // outline
        raylib.draw_rectangle_lines_ex(
            Rectangle {
                x: self.position.x,
                y: self.position.y,
                width: self.size.x,
                height: self.size.y,
            },
            2,
            color,
        );

        // center
        raylib.draw_circle_v(
            self.position + self.size / 2.0,
            (self.size.x * self.size.y).sqrt() / 10.0,
            color.fade(0.5),
        );
    }
}

impl Rect2D {
    /// Rect vs Point collision check
    fn check_vec2(&self, point: &Vector2) -> bool {
        return point.x >= self.position.x
            && point.y >= self.position.y
            && point.x <= self.position.x + self.size.x
            && point.y <= self.position.y + self.size.y;
    }

    /// Rect vs Rect collision check (aabb)
    pub fn check_rect(&self, rect2: &Rect2D) -> bool {
        self.position.x >= (rect2.position.x - self.size.x)
            && self.position.x <= (rect2.position.x + rect2.size.x)
            && self.position.y >= (rect2.position.y - self.size.y)
            && self.position.y <= (rect2.position.y + rect2.size.y)
    }

    /// Rect vs Rect collision resolution (aabb dynamic)
    pub fn collide_rect(&mut self, rect2: &mut Rect2D) -> bool {
        if self.check_rect(rect2) {
            // get center of rectangles
            let self_center = self.position + (self.size / 2.0);
            let other_center = rect2.position + (rect2.size / 2.0);

            // get rect sides
            let self_right = self.position.x + self.size.x;
            let self_bottom = self.position.y + self.size.y;
            let other_right = rect2.position.x + rect2.size.x;
            let other_bottom = rect2.position.y + rect2.size.y;

            // calc displacement of centers
            let displacement = Vector2::new(
                self_center.x - other_center.x,
                self_center.y - other_center.y,
            );

            // check which side to resolve on, both axis
            let horizontal = displacement.x < 0.0;
            let vertical = displacement.y < 0.0;

            // get x axis offset
            let x_offset = if horizontal {
                self_right - rect2.position.x // left side
            } else {
                other_right - self.position.x // right side
            };

            // get y axis offset
            let y_offset = if vertical {
                self_bottom - rect2.position.y // top side
            } else {
                other_bottom - self.position.y // bottom side
            };

            // resolve rect1 position
            if x_offset < y_offset {
                if horizontal {
                    self.position.x = rect2.position.x - self.size.x; // move left
                    self.colliding.push(CollisionDirection::Right);
                    rect2.colliding.push(CollisionDirection::Left);
                } else {
                    self.position.x = other_right; // move right
                    self.colliding.push(CollisionDirection::Left);
                    rect2.colliding.push(CollisionDirection::Right);
                }
            } else {
                if vertical {
                    self.position.y = rect2.position.y - self.size.y; // move up
                    self.colliding.push(CollisionDirection::Down);
                    rect2.colliding.push(CollisionDirection::Up);
                } else {
                    self.position.y = other_bottom; // move down
                    self.colliding.push(CollisionDirection::Up);
                    rect2.colliding.push(CollisionDirection::Down);
                }
            }

            // return colliding
            true
        } else {
            // return colliding
            false
        }
    }

    /// Rect vs Vector2
    pub fn check_v2(&self, point: &Vector2) -> bool {
        return point.x >= self.position.x
            && point.y >= self.position.y
            && point.x <= self.position.x + self.size.x
            && point.y <= self.position.y + self.size.y;
    }
}

impl ColliderInfo for Rect2D {
    fn center(&self) -> Vector2 {
        self.position + (self.size / 2.0)
    }

    fn velocity(&self) -> Vector2 {
        self.velocity
    }
}
