use crate::{collision, player::Player, raylib_plugins::CameraEx2D, scene::Scene};
use raylib::prelude::{
    Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibMode2DExt, RaylibThread,
};

const BACKGROUND_COLOR: Color = Color::new(25, 25, 25, 255);

pub struct Environment {
    camera: CameraEx2D,
    player: Player,
    floors: Vec<collision::Rect>,
}

impl Environment {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            camera: CameraEx2D::default()
                .with_offset(
                    raylib.get_screen_width() as f32 / 2.0,
                    raylib.get_screen_height() as f32 / 2.0,
                )
                .with_move_speed(7.0),
            player: Player::new(raylib, thread),
            floors: vec![
                collision::Rect::new(2000.0, 100.0).set_position_center(0.0, 200.0),
                collision::Rect::new(225.0, 5000.0).set_position_center(0.0, 100.0),
                collision::Rect::new(100.0, 5000.0).set_position_center(500.0, 100.0),
            ],
        }
    }
}

impl Scene for Environment {
    fn on_enter(&mut self, raylib: &mut RaylibHandle) {}

    fn on_exit(&mut self, raylib: &mut RaylibHandle) {}

    fn update(&mut self, raylib: &mut RaylibHandle) {
        // update player
        self.player.update(raylib);

        // speed up camera follow for fast velocity
        self.camera.move_speed = (self.player.collider.velocity.length() / 100.0).max(7.0);

        // make camera follow player
        self.camera.follow(self.player.get_center(), raylib);

        // collision
        {
            // reset player collisons
            self.player.collider.reset_colliding();

            // collide player & floors
            for floor in &mut self.floors {
                self.player.collider.collide_rect(floor);
            }
        }
    }

    fn draw(&self, raylib: &mut RaylibDrawHandle) {
        // draw background
        raylib.clear_background(BACKGROUND_COLOR);

        // enter camera
        let mut rl = raylib.begin_mode2D(&self.camera);
        {
            // draw player
            self.player.draw(&mut rl);

            // draw floors
            for floor in &self.floors {
                floor.draw(Color::BEIGE, &mut rl);
            }

            // exit camera
            drop(rl);
        }
    }

    fn debug(&self, raylib: &mut RaylibDrawHandle) {
        // start drawing within camera
        let mut rl = raylib.begin_mode2D(&self.camera);
        {
            // world origin refernce point
            rl.draw_circle(0, 0, 10.0, Color::PINK);

            // player collider
            self.player.collider.draw(Color::WHITE, &mut rl);

            // exit camera
            drop(rl);
        }

        // information
        let pos = self.player.collider.position;
        let vel = self.player.collider.velocity;

        // debug info / text color
        let debug_info = [
            (
                Color::ORANGE,
                &format!(
                    "postion: Vector2 {{ x: {:?}, y: {:?} }}",
                    pos.x as i32, pos.y as i32
                ),
            ),
            (
                Color::DARKGREEN,
                &format!(
                    "velocity: Vector2 {{ x: {:?}, y: {:?} }}",
                    vel.x as i32, vel.y as i32
                ),
            ),
            (
                Color::PINK,
                &format!("player state: {:#?}", self.player.state),
            ),
            (
                Color::SKYBLUE,
                &format!(
                    "player energy: {} : {} ({}%)",
                    self.player.max_stamina.round(),
                    self.player.stamina.round(),
                    ((self.player.stamina / self.player.max_stamina) * 100.0).round()
                ),
            ),
            (
                Color::BEIGE,
                &format!("camera zoom: {} ", self.camera.zoom()),
            ),
        ];

        // draw all debug info
        for (i, val) in debug_info.iter().enumerate() {
            let y_pos = 70 + (i as i32 * 20);
            raylib.draw_text(val.1, 10, y_pos, 20, val.0);
        }
    }

    fn id(&self) -> String {
        String::from("MainMenu")
    }
}
