use crate::{
    math, paths,
    scene::{ManageScene, Scene},
    sprite::Sprite2D,
    state::State,
    timer::Timer,
};
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread, Vector2};

/// Splash screen constants
const BACKGROUND: Color = Color::new(50, 50, 50, 255);
const ANIMATION_ELASTICITY: f32 = 10.0;
const ANIMATION_FREQUENCY: f32 = 0.25;

pub fn on_start(state: &mut State, raylib: &mut RaylibHandle) {
    state.resources.splash_screen.timer1.start();
    state.audio_handler.play_sound(&state.audio.bubbles);
}

pub fn update(state: &mut State, raylib: &mut RaylibHandle) {
    if !state.resources.splash_screen.timer1.is_stopped() {
        // calc animation time
        let animation_time = state.resources.splash_screen.timer1.time_left().as_millis() as f32
            / state.resources.splash_screen.timer1.wait_time().as_millis() as f32;
        // animate logo
        {
            // calc logo animation
            state.resources.splash_screen.logo.size = Vector2::new(
                math::ease_out_elastic_bounded(
                    ANIMATION_ELASTICITY * 2.0,
                    ANIMATION_FREQUENCY,
                    state.window.height / 2.0,
                    animation_time * 4.0,
                ),
                math::ease_out_elastic_bounded(
                    ANIMATION_ELASTICITY,
                    ANIMATION_FREQUENCY,
                    state.window.height / 2.0,
                    animation_time * 4.0,
                ),
            );
            // set origin to center
            state.resources.splash_screen.logo.center_origin();
            // center logo
            state.resources.splash_screen.logo.position =
                Vector2::new(state.window.width / 2.0, state.window.height / 2.0);
        }

        // animate water
        {
            // calc water animation
            state.resources.splash_screen.water.position.x =
                -(state.resources.splash_screen.water.size.x / 8.0)
                    + (animation_time * 50.0).sin() * 10.0;
            state.resources.splash_screen.water.position.y =
                math::lerp(state.window.height, 0.0, animation_time);
            //scale water to window size
            state.resources.splash_screen.water.size =
                Vector2::new(state.window.width * 1.25, state.window.height);
        }
    } else {
        // queue next scene
        state.queue_scene(raylib, Scene::MAIN_MENU);
    }
}

pub fn draw(state: &State, raylib: &mut RaylibDrawHandle) {
    // draw background color
    raylib.draw_rectangle_gradient_v(
        0,
        0,
        state.window.width as i32,
        state.window.height as i32,
        Color::BLUE,
        Color::DARKBLUE,
    );
    // draw logo
    state.resources.splash_screen.logo.draw(raylib);

    // draw water
    state.resources.splash_screen.water.draw(raylib);
}

pub struct LOAD {
    timer1: Timer,
    logo: Sprite2D,
    water: Sprite2D,
}

impl LOAD {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> LOAD {
        LOAD {
            timer1: Timer::new(std::time::Duration::from_secs(4)),
            logo: {
                // load image
                let mut sprite = Sprite2D::from_path(raylib, thread, paths::splash_screen::LOGO);
                // set initial size
                sprite.size = Vector2::zero();
                // set origin to center
                sprite.center_origin();
                // set initial position
                sprite.center_to_screen(raylib);
                sprite
            },
            water: {
                // get window dimensions
                let screen_size = Vector2::new(
                    raylib.get_screen_width() as f32,
                    raylib.get_screen_height() as f32,
                );
                // load image
                let mut sprite = Sprite2D::from_path(raylib, thread, paths::splash_screen::WATER);
                // set initial size
                sprite.size = screen_size;
                sprite.size.x *= 1.5;
                // set initial position
                sprite.position = Vector2::new(-(screen_size.x / 2.0), screen_size.y);
                sprite
            },
        }
    }
}
