use crate::scenes::GlobalEnvironment;
use rayexlib::prelude::{ColliderInfo, Rect2D, Renderable, Scene};
use raylib::prelude::{
    Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibMode2DExt, RaylibThread,
};

const BACKGROUND_COLOR: Color = Color::new(25, 25, 25, 255);

pub struct Environment {
    // camera: CameraEx2D,
    // player: Player,
    floors: Vec<Rect2D>,
}

impl Environment {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            // camera: CameraEx2D::default()
            //     .with_offset(
            //         raylib.get_screen_width() as f32 / 2.0,
            //         raylib.get_screen_height() as f32 / 2.0,
            //     )
            //     .with_move_speed(7.0),
            // player: Player::init(raylib, thread),
            floors: vec![
                Rect2D::new(2000.0, 100.0).with_position_center(0.0, 200.0),
                Rect2D::new(100.0, 100.0).with_position_center(0.0, 100.0),
                Rect2D::new(100.0, 500.0).with_position_center(500.0, 100.0),
            ],
        }
    }
}

impl Scene<GlobalEnvironment> for Environment {
    fn update(&mut self, global: &mut GlobalEnvironment, raylib: &mut RaylibHandle) {
        let player = &mut global.player;

        player.update(raylib);

        global.camera.follow_bound(player.velocity(), 100.0, 7.0);
        global.camera.follow(player.center(), raylib);

        // collision
        {
            player.reset_colliding();
            player.collide_rects(raylib, &mut self.floors);
        }
    }

    fn draw(&self, global: &GlobalEnvironment, raylib: &mut RaylibDrawHandle) {
        let player = &global.player;

        raylib.clear_background(BACKGROUND_COLOR);

        // enter camera
        let mut rl = raylib.begin_mode2D(&global.camera);
        {
            player.draw(&mut rl);

            self.floors.iter().for_each(|floor| {
                floor.draw(Color::BEIGE, &mut rl);
            });

            // exit camera
            drop(rl);
        }
    }

    fn debug(&self, global: &GlobalEnvironment, raylib: &mut RaylibDrawHandle) {
        let player = &global.player;

        // start drawing within camera
        let mut rl = raylib.begin_mode2D(&global.camera);
        {
            // world origin reference point
            rl.draw_circle(0, 0, 10.0, Color::PINK);

            player.collider.draw(Color::WHITE, &mut rl);
            player.ground_ray.draw(Color::RED, &mut rl);

            let (x, y) = (player.center().x as i32, player.center().y as i32);
            rl.draw_circle_lines(x, y, 100.0, Color::GOLD);

            // exit camera
            drop(rl);
        }

        // information
        let pos = player.collider.position;
        let vel = player.collider.velocity;

        // debug info / text color
        let debug_info = [
            (
                Color::ORANGE,
                format!(
                    "position: Vector2 {{ x: {:?}, y: {:?} }}",
                    pos.x as i32, pos.y as i32
                ),
            ),
            (
                Color::DARKGREEN,
                format!(
                    "velocity: Vector2 {{ x: {:?}, y: {:?} }}",
                    vel.x as i32, vel.y as i32
                ),
            ),
            (Color::PINK, format!("player state: {:#?}", player.state)),
            (
                Color::SKYBLUE,
                format!(
                    "player energy: {} : {} ({}%)",
                    player.max_stamina.round(),
                    player.stamina.round(),
                    ((player.stamina / player.max_stamina) * 100.0).round()
                ),
            ),
            (
                Color::BEIGE,
                format!("camera zoom: {} ", global.camera.zoom()),
            ),
        ];

        // draw all debug info
        debug_info.iter().enumerate().for_each(|(i, val)| {
            let y_pos = 70 + (i as i32 * 20);
            raylib.draw_text(&val.1, 10, y_pos, 20, val.0);
        });
    }
}
