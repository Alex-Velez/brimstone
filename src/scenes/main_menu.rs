use crate::{
    paths::main_menu,
    scene_machine::Scene,
    sprite::{ParallaxLayer2D, Sprite2D},
};
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread, Vector2};

/// Main menu constant values
const BACKGROUND: Color = Color::new(219, 95, 83, 255);
const MENU_BACKGROUND: Color = Color::new(178, 34, 34, 100);
const PFAR_SPEED: Vector2 = Vector2::new(20.0, 0.0);
const PMID_SPEED: Vector2 = Vector2::new(30.0, 0.0);
const PCLO_SPEED: Vector2 = Vector2::new(40.0, 0.0);

pub struct Environment {
    logo: Sprite2D,
    parallax_far: ParallaxLayer2D,
    parallax_mid: ParallaxLayer2D,
    parallax_close: ParallaxLayer2D,
}

impl Environment {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // create parallax layers
        let (parallax_far, parallax_mid, parallax_close) = {
            // create new parallax layers
            let mut parallax_far =
                ParallaxLayer2D::from_path(raylib, &thread, main_menu::PRLX_TREES_FAR, PFAR_SPEED);
            let mut parallax_mid =
                ParallaxLayer2D::from_path(raylib, &thread, main_menu::PRLX_TREES_MID, PMID_SPEED);
            let mut parallax_clo =
                ParallaxLayer2D::from_path(raylib, &thread, main_menu::PRLX_TREES_CLO, PCLO_SPEED);

            // resize layers to screen
            parallax_far.maximize_to_screen(raylib);
            parallax_mid.maximize_to_screen(raylib);
            parallax_clo.maximize_to_screen(raylib);

            (parallax_far, parallax_mid, parallax_clo)
        };

        Self {
            logo: Sprite2D::from_path(raylib, thread, main_menu::LOGO),
            parallax_far,
            parallax_mid,
            parallax_close,
        }
    }
}

impl Scene for Environment {
    fn on_enter(&mut self, raylib: &mut RaylibHandle) {
        let win_width = raylib.get_screen_width() as f32;
        let win_height = raylib.get_screen_height() as f32;

        // scale main menu to screen size
        {
            // maximize texture to window size
            self.parallax_far.maximize_to_screen(raylib);
            self.parallax_mid.maximize_to_screen(raylib);
            self.parallax_close.maximize_to_screen(raylib);
            // resize logo
            self.logo.set_size(win_width / 2.0, win_height / 8.0);
            // center logo position to center of screen
            self.logo.set_x((win_width / 2.0) - self.logo.half_width());
        }
    }

    fn update(&mut self, raylib: &mut RaylibHandle) {
        let win_width = raylib.get_screen_width() as f32;
        let win_height = raylib.get_screen_height() as f32;

        // scale main menu to screen size
        if raylib.is_window_resized() {
            // set texture size to window size
            self.parallax_far.maximize_to_screen(raylib);
            self.parallax_mid.maximize_to_screen(raylib);
            self.parallax_close.maximize_to_screen(raylib);
            // resize logo
            self.logo.set_size(win_width / 2.0, win_height / 8.0);
            // center logo position to center of screen
            self.logo.set_x((win_width / 2.0) - self.logo.half_width());
        }

        // animate logo
        let rl_time = raylib.get_time() as f32;
        self.logo.set_rotation(rl_time.sin() * 2.0);
        self.logo.set_y((win_height / 12.0) + (rl_time.cos() * 5.0));

        // update parallax background
        self.parallax_far.update(raylib);
        self.parallax_mid.update(raylib);
        self.parallax_close.update(raylib);
    }

    fn draw(&self, raylib: &mut RaylibDrawHandle) {
        // draw background color
        raylib.clear_background(BACKGROUND);

        let win_width = raylib.get_screen_width() as f32;
        let win_height = raylib.get_screen_height() as f32;

        // draw parallax layers
        self.parallax_far.draw(raylib);
        self.parallax_mid.draw(raylib);
        self.parallax_close.draw(raylib);

        // background shadow gradient
        {
            // top shadow
            raylib.draw_rectangle_gradient_v(
                0,
                0,
                win_width as i32,
                (win_height / 2.0) as i32,
                Color::BLACK,
                Color::BLANK,
            );
            // bottom shadow
            raylib.draw_rectangle_gradient_v(
                0,
                (win_height / 2.0) as i32,
                win_width as i32,
                (win_height / 2.0) as i32,
                Color::BLANK,
                Color::BLACK,
            );
            // left shadow
            raylib.draw_rectangle_gradient_h(
                0,
                0,
                (win_width / 3.0) as i32,
                win_height as i32,
                Color::BLACK,
                Color::BLANK,
            );
            // right shadow
            raylib.draw_rectangle_gradient_h(
                ((win_width / 3.0) * 2.0) as i32,
                0,
                ((win_width / 3.0) + 1.0) as i32,
                win_height as i32,
                Color::BLANK,
                Color::BLACK,
            );
        }

        // draw logo
        self.logo.draw(raylib);
    }
}
