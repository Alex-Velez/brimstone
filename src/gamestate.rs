use crate::{
    raylib_plugins::FrameLimiter,
    scene_machine::SceneMachine,
    scenes::{SceneID, Stage},
    window::{self, Window},
};
use raylib::prelude::{
    Color, KeyboardKey, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread, Rectangle,
};

pub struct GameState {
    window: Window,
    stage: Stage,
    scene_machine: SceneMachine<SceneID>,
    paused: bool,
    exit: bool,
    debug: DebugTools,
}

impl GameState {
    pub fn new(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        GameState {
            window: Window::new(raylib),
            stage: Stage::init(raylib, thread),
            scene_machine: SceneMachine::init(raylib, thread),
            paused: false,
            exit: false,
            debug: DebugTools::default(),
        }
    }

    pub fn run(&mut self, raylib: &mut RaylibHandle, thread: &RaylibThread) {
        // call current scene on_start fn
        self.scene_machine.on_enter(raylib);

        // main loop
        while !raylib.window_should_close() && !self.exit {
            // current scene update function
            self.update(raylib);

            // init draw handle
            let mut rl = raylib.begin_drawing(&thread);

            // current scene draw function
            self.draw(&mut rl);
        }
    }

    fn update(&mut self, raylib: &mut RaylibHandle) {
        // global update
        self.global_update(raylib);

        if !self.paused && !self.debug.paused && raylib.is_window_focused() {
            // current scene update
            self.scene_machine.update(raylib);
        }
    }

    fn draw(&self, raylib: &mut RaylibDrawHandle) {
        // current scene draw
        self.scene_machine.draw(raylib);

        // pause screen
        if self.paused {
            self.pause_overlay(raylib);
        }

        // window decorations
        if !raylib.is_window_fullscreen() {
            self.window.draw(raylib);
        }

        // debug overlay
        if self.debug.active {
            self.debug_draw(raylib);
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn next_scene(&mut self, raylib: &mut RaylibHandle, next_scene: SceneID) {
        self.scene_machine.next_scene(raylib, next_scene);
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn toggle_debug(&mut self) {
        self.debug.active = !self.debug.active;
    }

    pub fn toggle_fullscreen(&mut self, raylib: &mut RaylibHandle) {
        if raylib.is_window_fullscreen() {
            // toggle fullscreen mode
            raylib.toggle_fullscreen();

            // set window size to prev size
            raylib.set_window_size(self.window.prev_width(), self.window.prev_height());
        } else {
            // save prev window size
            self.window.save_size(raylib);

            // set window size to monitor size
            unsafe {
                raylib.set_window_size(
                    raylib::ffi::GetMonitorWidth(raylib::ffi::GetCurrentMonitor()),
                    raylib::ffi::GetMonitorHeight(raylib::ffi::GetCurrentMonitor()),
                );
            }

            // toggle fullscreen mode
            raylib.toggle_fullscreen();
        }
    }
}

impl GameState {
    fn global_update(&mut self, raylib: &mut RaylibHandle) {
        // debug utilitizes
        if self.debug.active {
            self.debug_update(raylib);
        } else {
            // fps checker
            if raylib.get_fps() < 15 && raylib.get_time() > 2.0 {
                println!("{}: FPS too low for engine!", window::DEFAULT_TITLE);
                self.exit();
            }
        }

        // hot keys
        if let Some(key) = raylib.get_key_pressed() {
            // global hot keys
            match key {
                // toggle pause
                KeyboardKey::KEY_ESCAPE => self.toggle_pause(),
                // toggle fullscreen
                KeyboardKey::KEY_F11 => self.toggle_fullscreen(raylib),
                // toggle debug
                KeyboardKey::KEY_F3 => self.toggle_debug(),
                // scene switchers
                KeyboardKey::KEY_ONE => self.next_scene(raylib, SceneID::MainMenu),
                KeyboardKey::KEY_TWO => self.next_scene(raylib, SceneID::World),
                KeyboardKey::KEY_THREE => self.next_scene(raylib, SceneID::Loading),
                KeyboardKey::KEY_FOUR => self.next_scene(raylib, SceneID::PauseMenu),
                _ => {}
            }

            if self.debug.active {
                // debug hot keys
                match key {
                    // step forward one frame
                    KeyboardKey::KEY_KP_6 => self.debug.paused = !self.debug.paused,
                    // toggle step frames
                    KeyboardKey::KEY_KP_0 => {
                        self.debug.step_frames = !self.debug.step_frames;
                        self.debug.paused = false;
                    }
                    // increase fps
                    KeyboardKey::KEY_KP_ADD => {
                        self.debug.step_fps += 1;
                        raylib.set_target_fps(self.debug.step_fps);
                    }
                    // decrease fps
                    KeyboardKey::KEY_KP_SUBTRACT => {
                        self.debug.step_fps -= 1;
                        raylib.set_target_fps(self.debug.step_fps);
                    }
                    _ => {}
                }
            }
        }
    }

    fn pause_overlay(&self, raylib: &mut RaylibDrawHandle) {
        raylib.draw_text(
            "Paused",
            raylib.get_screen_width() / 2,
            raylib.get_screen_height() / 2,
            50,
            Color::WHITE,
        );
    }
}

pub struct DebugTools {
    pub active: bool,
    pub step_frames: bool,
    pub paused: bool,
    pub step_fps: u32,
}

impl Default for DebugTools {
    fn default() -> Self {
        Self {
            active: false,
            step_frames: false,
            paused: false,
            step_fps: 30,
        }
    }
}

impl GameState {
    fn debug_update(&mut self, raylib: &mut RaylibHandle) {
        // pause game on each frame
        if self.debug.step_frames {
            self.debug.paused = true;
        }

        // move forward at normal fps
        if raylib.is_key_down(KeyboardKey::KEY_KP_8) {
            if self.debug.paused {
                self.debug.paused = false;
            }
        }
    }

    fn debug_draw(&self, raylib: &mut RaylibDrawHandle) {
        // scene debug overlay
        self.scene_machine.debug(raylib);

        let win_width = raylib.get_screen_width();
        let win_height = raylib.get_screen_height();

        // debug info / text color
        let debug_info = [
            (
                match raylib.get_fps() {
                    x if x < 15 => Color::RED,
                    x if x < 30 => Color::ORANGE,
                    _ => Color::LIME,
                },
                &format!("{} FPS", raylib.get_fps()),
            ),
            (
                Color::YELLOW,
                &format!("frame time: {}", raylib.get_frame_time_limited()),
            ),
            (
                Color::BEIGE,
                &format!("current scene: {:?}", self.scene_machine.current_scene()),
            ),
        ];

        // draw all debug info
        for (i, val) in debug_info.iter().enumerate() {
            let y_pos = 10 + (i as i32 * 20);
            raylib.draw_text(val.1, 10, y_pos, 20, val.0);
        }

        // debug window outline
        raylib.draw_rectangle_lines_ex(
            Rectangle::new(0.0, 0.0, win_width as f32, win_height as f32),
            5,
            Color::RED,
        );

        // step frame outline
        if self.debug.step_frames {
            raylib.draw_rectangle_lines_ex(
                Rectangle::new(0.0, 0.0, win_width as f32, win_height as f32),
                5,
                Color::YELLOW,
            );
        }
    }
}
