use raylib::prelude::{Color, RaylibDraw, Vector2};

#[derive(Clone)]
pub struct Polygon2D {
    pub position: Vector2,
    pub rotation: f32,
    pub velocity: Vector2,
    points: Vec<Vector2>,
    model_points: Vec<Vector2>,
    broad_collider_radius: f32,
    sides: usize,
    centroid: Vector2,
    colliding: bool,
}

impl Polygon2D {
    /// Construct a regular polygon
    pub fn new(size: f32, sides: usize) -> Polygon2D {
        // confirm valid shape
        assert!(sides > 1);

        let mut points: Vec<Vector2> = Vec::new();
        let theta = std::f32::consts::TAU / sides as f32;
        for i in 0..sides {
            points.push(Vector2::new(
                (size / std::f32::consts::SQRT_2) * (theta * i as f32).cos(),
                (size / std::f32::consts::SQRT_2) * (theta * i as f32).sin(),
            ));
        }

        // find centroid of polygon
        let centroid = {
            let mut centroid = Vector2::new(0.0, 0.0);
            for point in points.clone() {
                centroid += point;
            }
            centroid /= points.len() as f32;
            centroid
        };

        Polygon2D {
            position: Vector2::zero(),
            rotation: 0.0,
            velocity: Vector2::zero(),
            points: points.clone(),
            model_points: points,
            broad_collider_radius: size * std::f32::consts::SQRT_2 / 2.0,
            sides,
            centroid,
            colliding: false,
        }
    }

    /// Construct a polygon from a set of points [center at (0,0)]
    pub fn from(model_points: Vec<Vector2>) -> Polygon2D {
        // confirm valid shape
        assert!(model_points.len() > 1);

        // find centroid of polygon
        let centroid = {
            let mut centroid = Vector2::new(0.0, 0.0);
            for point in &model_points {
                centroid += *point;
            }
            centroid /= model_points.len() as f32;
            centroid
        };

        // find farthest point
        let mut farthest_point = centroid;
        for point in &model_points {
            if centroid.distance_to(*point) > centroid.distance_to(farthest_point) {
                farthest_point = *point;
            }
        }

        Polygon2D {
            position: Vector2::zero(),
            rotation: 0.0,
            velocity: Vector2::zero(),
            points: model_points.clone(),
            model_points: model_points.clone(),
            broad_collider_radius: centroid.distance_to(farthest_point),
            sides: model_points.len(),
            centroid,
            colliding: false,
        }
    }

    /// return colliding
    pub fn is_colliding(&self) -> bool {
        self.colliding
    }

    /// Reset colliding
    pub fn reset_colliding(&mut self) {
        self.colliding = false;
    }

    /// Update points with current position and rotation
    pub fn update_transform(&mut self) {
        // assume rotation is in radians
        let theta_sin = self.rotation.sin();
        let theta_cos = self.rotation.cos();
        for i in 0..self.sides {
            self.points[i].x = (self.model_points[i].x * theta_cos)
                - (self.model_points[i].y * theta_sin)
                + (self.position.x);

            self.points[i].y = (self.model_points[i].x * theta_sin)
                + (self.model_points[i].y * theta_cos)
                + (self.position.y);
        }
    }
}

impl Polygon2D {
    /// Only works for convex polygons (SAT)
    /// Collision detection begins to break down at ~50 sides
    pub fn check(&self, polygon2: &Polygon2D) -> bool {
        // if polygon not in broad range, return false
        if self.position.distance_to(polygon2.position)
            > self.broad_collider_radius + polygon2.broad_collider_radius
        {
            return false;
        }

        let mut poly1 = &self;
        let mut poly2 = &polygon2;

        for shape in 0..2 {
            // switch poly namespaces
            if shape == 1 {
                poly1 = &polygon2;
                poly2 = &self;
            }

            for i in 0..poly1.sides {
                // index of next point
                let next_i = (i + 1) % poly1.sides;
                // get axis projection for side
                let axis_proj = Vector2 {
                    x: -(poly1.points[next_i].y - poly1.points[i].y),
                    y: poly1.points[next_i].x - poly1.points[i].x,
                };
                // find min and max 1D points for poly1
                let mut min_poly1 = f32::INFINITY;
                let mut max_poly1 = f32::NEG_INFINITY;
                for p in 0..poly1.sides {
                    let dot_product = poly1.points[p].dot(axis_proj);
                    min_poly1 = min_poly1.min(dot_product);
                    max_poly1 = max_poly1.max(dot_product);
                }
                // find min and max 1D points for poly2
                let mut min_poly2 = f32::INFINITY;
                let mut max_poly2 = f32::NEG_INFINITY;
                for p in 0..poly2.sides {
                    let dot_product = poly2.points[p].dot(axis_proj);
                    min_poly2 = min_poly2.min(dot_product);
                    max_poly2 = max_poly2.max(dot_product);
                }
                // check if poly min/max values dont overlap
                if !(max_poly2 >= min_poly1 && max_poly1 >= min_poly2) {
                    return false;
                }
            }
        }

        return true;
    }

    /// Only works for convex polygons
    /// Collision resolution begins to break down at ~50 sides
    pub fn collide(&mut self, polygon2: &mut Polygon2D) -> bool {
        // if polygon not in broad range, return false
        if self.position.distance_to(polygon2.position)
            > self.broad_collider_radius + polygon2.broad_collider_radius
        {
            self.colliding = false;
            return false;
        }

        let mut poly1 = &self;
        let mut poly2 = &polygon2;
        let mut overlap = f32::INFINITY;

        for shape in 0..2 {
            // switch poly namespaces
            if shape == 1 {
                poly1 = &polygon2;
                poly2 = &self;
            }

            for i in 0..poly1.sides {
                // index of next point
                let next_i = (i + 1) % poly1.sides;
                // get axis projection for side
                let axis_proj = Vector2 {
                    x: -(poly1.points[next_i].y - poly1.points[i].y),
                    y: poly1.points[next_i].x - poly1.points[i].x,
                };
                // find min and max 1D points for poly1
                let mut min_poly1 = f32::INFINITY;
                let mut max_poly1 = f32::NEG_INFINITY;
                for p in 0..poly1.sides {
                    let dot_product = poly1.points[p].dot(axis_proj);
                    min_poly1 = min_poly1.min(dot_product);
                    max_poly1 = max_poly1.max(dot_product);
                }
                // find min and max 1D points for poly2
                let mut min_poly2 = f32::INFINITY;
                let mut max_poly2 = f32::NEG_INFINITY;
                for p in 0..poly2.sides {
                    let dot_product = poly2.points[p].dot(axis_proj);
                    min_poly2 = min_poly2.min(dot_product);
                    max_poly2 = max_poly2.max(dot_product);
                }
                // check if poly min/max values dont overlap
                if !(max_poly2 >= min_poly1 && max_poly1 >= min_poly2) {
                    self.colliding = false;
                    return false;
                }
                // calculate actual overlap along projected axis, and store the minimum
                overlap = (max_poly1.min(max_poly2) - min_poly1.max(min_poly2)).min(overlap);
            }
        }

        // if we got here, the objects have collided, we will displace poly1
        // by `overlap` along the vector between the two object centers
        let displacement = Vector2 {
            x: polygon2.position.x - self.position.x,
            y: polygon2.position.y - self.position.y,
        };

        // displace poly1 position
        if displacement.x.abs() > displacement.y.abs() {
            self.position.x -= overlap / displacement.x;
        } else {
            self.position.y -= overlap / displacement.y;
        }

        self.colliding = true;
        return true;
    }
}

impl Polygon2D {
    pub fn draw(&self, color: Color, raylib: &mut impl RaylibDraw) {
        // edges
        for i in 0..self.sides {
            let i2 = (i + 1) % self.sides;
            raylib.draw_line_v(self.points[i], self.points[i2], color);
        }

        // center
        raylib.draw_circle_v(self.position, 10.0, color.fade(0.5));

        // broad collider
        raylib.draw_circle_lines(
            self.position.x as i32,
            self.position.y as i32,
            self.broad_collider_radius,
            color.fade(0.1),
        );
    }
}
