use raylib::prelude::{Color, RaylibDraw, Rectangle, Vector2};

pub struct Rect {
    pub position: Vector2,
    pub size: Vector2,
    pub velocity: Vector2,
    pub colliding: Vector2,
}

impl Rect {
    pub const fn new(width: f32, height: f32) -> Self {
        Rect {
            position: Vector2::new(0.0, 0.0),
            size: Vector2::new(width, height),
            velocity: Vector2::new(0.0, 0.0),
            colliding: Vector2::new(0.0, 0.0),
        }
    }

    pub const fn newv(size: Vector2) -> Self {
        Rect {
            position: Vector2::new(0.0, 0.0),
            size,
            velocity: Vector2::new(0.0, 0.0),
            colliding: Vector2::new(0.0, 0.0),
        }
    }

    /// Position rect to (x, y)
    pub fn set_position(mut self, x: f32, y: f32) -> Self {
        self.position.x = x;
        self.position.y = y;
        self
    }

    /// Position center of rect to (x, y)
    pub fn set_position_center(mut self, x: f32, y: f32) -> Self {
        self.position.x = x - (self.size.x / 2.0);
        self.position.y = y - (self.size.y / 2.0);
        self
    }

    /// Check if bottom is colliding
    pub fn on_floor(&self) -> bool {
        self.colliding.y == -1.0
    }

    /// Check if top is colliding
    pub fn on_roof(&self) -> bool {
        self.colliding.y == 1.0
    }

    /// Check if sides are colliding
    pub fn on_wall(&self) -> bool {
        self.colliding.x != 0.0
    }

    /// Check if left side is colliding
    pub fn on_wall_left(&self) -> bool {
        self.colliding.x == -1.0
    }

    /// Check if right side is colliding
    pub fn on_wall_right(&self) -> bool {
        self.colliding.x == 1.0
    }

    /// Reset colliding vector
    pub fn reset_colliding(&mut self) {
        self.colliding = Vector2::new(0.0, 0.0);
    }
}

impl Rect {
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
        raylib.draw_circle_v(self.position + self.size / 2.0, 10.0, color.fade(0.5));
    }
}

impl Rect {
    /// Rect vs Point collision check
    fn check_vec2(&self, point: &Vector2) -> bool {
        return point.x >= self.position.x
            && point.y >= self.position.y
            && point.x <= self.position.x + self.size.x
            && point.y <= self.position.y + self.size.y;
    }

    /// Rect vs Rect collision check (aabb)
    pub fn check_rect(&self, rect2: &Rect) -> bool {
        self.position.x >= (rect2.position.x - self.size.x)
            && self.position.x <= (rect2.position.x + rect2.size.x)
            && self.position.y >= (rect2.position.y - self.size.y)
            && self.position.y <= (rect2.position.y + rect2.size.y)
    }

    /// Rect vs Rect collision resolution (aabb dynamic)
    pub fn collide_rect(&mut self, rect2: &mut Rect) -> bool {
        if self.check_rect(rect2) {
            // get center of rectangles
            let center1 = self.position + (self.size / 2.0);
            let center2 = rect2.position + (rect2.size / 2.0);

            // calc displacement of centers
            let displacement = Vector2::new(center1.x - center2.x, center1.y - center2.y);

            // check which side to resolve on, both axis
            let horizontal = displacement.x < 0.0;
            let vertical = displacement.y < 0.0;

            // get x axis offset
            let x_offset = if horizontal {
                (self.position.x + self.size.x) - rect2.position.x // left side
            } else {
                (rect2.position.x + rect2.size.x) - self.position.x // right side
            };

            // get y axis offset
            let y_offset = if vertical {
                (self.position.y + self.size.y) - rect2.position.y // top side
            } else {
                (rect2.position.y + rect2.size.y) - self.position.y // bottom side
            };

            // resolve rect1 position
            if x_offset < y_offset {
                if horizontal {
                    self.position.x = rect2.position.x - self.size.x; // move left
                    self.colliding.x = 1.0;
                    rect2.colliding.x = -1.0;
                } else {
                    self.position.x = rect2.position.x + rect2.size.x; // move right
                    self.colliding.x = -1.0;
                    rect2.colliding.x = 1.0;
                }
            } else {
                if vertical {
                    self.position.y = rect2.position.y - self.size.y; // move up
                    self.colliding.y = -1.0;
                    rect2.colliding.y = 1.0;
                } else {
                    self.position.y = rect2.position.y + rect2.size.y; // move down
                    self.colliding.y = 1.0;
                    rect2.colliding.y = -1.0;
                }
            }

            true
        } else {
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

/*
impl StaticCollisonDetection<Rect> for Rect {
    /// Rect vs Rect collision check (aabb)
    fn check(&self, rect2: &Rect) -> bool {
        self.position.x >= (rect2.position.x - self.size.x)
            && self.position.x <= (rect2.position.x + rect2.size.x)
            && self.position.y >= (rect2.position.y - self.size.y)
            && self.position.y <= (rect2.position.y + rect2.size.y)
    }
}

impl StaticCollisonResolution<Rect> for Rect {
    /// Rect vs Rect collision resolution (aabb dynamic)
    fn collide(&mut self, rect2: &mut Rect) -> bool {
        if StaticCollisonDetection::<Rect>::check(self, rect2) {
            // get center of rectangles
            let center1 = self.position + (self.size / 2.0);
            let center2 = rect2.position + (rect2.size / 2.0);

            // calc displacement of centers
            let displacement = Vector2::new(center1.x - center2.x, center1.y - center2.y);

            // check which side to resolve on, both axis
            let horizontal = displacement.x < 0.0;
            let vertical = displacement.y < 0.0;

            // get x axis offset
            let x_offset = if horizontal {
                (self.position.x + self.size.x) - rect2.position.x // left side
            } else {
                (rect2.position.x + rect2.size.x) - self.position.x // right side
            };

            // get y axis offset
            let y_offset = if vertical {
                (self.position.y + self.size.y) - rect2.position.y // top side
            } else {
                (rect2.position.y + rect2.size.y) - self.position.y // bottom side
            };

            // resolve rect1 position
            if x_offset < y_offset {
                if horizontal {
                    self.position.x = rect2.position.x - self.size.x; // move left
                    self.colliding.x = 1.0;
                    rect2.colliding.x = -1.0;
                } else {
                    self.position.x = rect2.position.x + rect2.size.x; // move right
                    self.colliding.x = -1.0;
                    rect2.colliding.x = 1.0;
                }
            } else {
                if vertical {
                    self.position.y = rect2.position.y - self.size.y; // move up
                    self.colliding.y = -1.0;
                    rect2.colliding.y = 1.0;
                } else {
                    self.position.y = rect2.position.y + rect2.size.y; // move down
                    self.colliding.y = 1.0;
                    rect2.colliding.y = -1.0;
                }
            }

            true
        } else {
            false
        }
    }
}

impl DynamicCollisonResulution<Rect> for Rect {
    fn collide(&mut self, other: &Rect, frame_time: f32) -> bool {
        // Check if dynamic rectangle is actually moving
        // we assume rectangles are NOT in collision to start
        if self.velocity == (Vector2 { x: 0.0, y: 0.0 }) {
            return false;
        }

        // Expand target rectangle by source dimensions
        let mut expanded_target = Rect::new(0.0, 0.0);
        expanded_target.position = other.position - self.size / 2.0;
        expanded_target.size = other.size + self.size;

        // Create ray at rect1
        // let mut ray = Ray2D::new();
        self.ray.position = self.position + self.size / 2.0;
        self.ray.direction = self.velocity * frame_time;

        // Shoot ray at expanded target rect and test for collision
        self.ray.collide(&mut expanded_target)
    }
}

impl StaticCollisonDetection<Vector2> for Rect {
    fn check(&self, point: &Vector2) -> bool {
        return point.x >= self.position.x
            && point.y >= self.position.y
            && point.x <= self.position.x + self.size.x
            && point.y <= self.position.y + self.size.y;
    }
}

impl Rect {
    pub fn swept_check(&self, other: &Rect) -> f32 {
        // dxEntry, dyEntry: the distance need to move to begin contact
        // dxExit, dyExit: the distance need to move to exit contact
        let (dx_entry, dx_exit): (f32, f32);
        let (dy_entry, dy_exit): (f32, f32);

        // entry/exit distance for x-axis
        if self.velocity.x > 0.0 {
            dx_entry = other.position.x - (self.position.x + self.size.x);
            dx_exit = (other.position.x + other.size.x) - self.position.x;
        } else {
            dx_entry = (other.position.x + other.size.x) - self.position.x;
            dx_exit = other.position.x - (self.position.x + self.size.x);
        }

        // entry/exit distance for y-axis
        if self.velocity.y > 0.0 {
            dy_entry = other.position.y - (self.position.y + self.size.y);
            dy_exit = (other.position.y + other.size.y) - self.position.y;
        } else {
            dy_entry = (other.position.y + other.size.y) - self.position.y;
            dy_exit = other.position.y - (self.position.y + self.size.y);
        }

        // from distance and velocity, we can calculate the time
        // time = distance / velocity
        let (tx_entry, tx_exit): (f32, f32);
        let (ty_entry, ty_exit): (f32, f32);

        // entry/exit time for x-axis
        if self.velocity.x == 0.0 {
            tx_entry = std::f32::NEG_INFINITY;
            tx_exit = std::f32::INFINITY;
        } else {
            tx_entry = dx_entry / self.velocity.x;
            tx_exit = dx_exit / self.velocity.x;
        }

        // entry/exit time for y-axis
        if self.velocity.y == 0.0 {
            ty_entry = std::f32::NEG_INFINITY;
            ty_exit = std::f32::INFINITY;
        } else {
            ty_entry = dy_entry / self.velocity.y;
            ty_exit = dy_exit / self.velocity.y;
        }

        // take the longest time to begin collision
        let entry_time = tx_entry.max(ty_entry);

        // take the quickest time to exit collision
        let exit_time = tx_exit.min(ty_exit);

        // check collision by:
        // time to begin collision from 0 to 1
        // time to end collision must be bigger than time to begin collision
        if entry_time > exit_time
            || (tx_entry < 0.0 && ty_entry < 0.0)
            || tx_entry > 1.0
            || ty_entry > 1.0
        {
            return 1.0;
        }

        // return time to begin a collision
        return entry_time;
    }

    pub fn swept_collide(&mut self, other: &Rect) {
        let collision_time = self.swept_check(other);

        self.position.x += self.velocity.x * collision_time;
        self.position.y += self.velocity.y * collision_time;
    }
}
*/
