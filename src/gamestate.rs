use crate::{
    raylib_plugins::FrameLimiter,
    scene::{SceneID, SceneMachine, Stage},
    window::{self, Window},
};
use raylib::prelude::{
    Color, KeyboardKey, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread,
};

pub struct GameState {
    window: Window,
    stage: Stage,
    scene_machine: SceneMachine,
    fullscreen: bool,
    debug: bool,
    paused: bool,
    exit: bool,
}

impl GameState {
    pub fn new(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        GameState {
            window: Window::new(raylib),
            stage: Stage::new(raylib, thread),
            scene_machine: SceneMachine::init(raylib, thread),
            fullscreen: false,
            debug: false,
            paused: false,
            exit: false,
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

    pub fn update(&mut self, raylib: &mut RaylibHandle) {
        // global update process
        self.global_update(raylib);

        if !self.paused && raylib.is_window_focused() {
            // current scene update
            self.scene_machine.update(raylib);
        }
    }

    pub fn draw(&self, raylib: &mut RaylibDrawHandle) {
        // current scene draw
        self.scene_machine.draw(raylib);

        // global draw process
        self.global_draw(raylib);
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    fn toggle_pause(&mut self, raylib: &mut RaylibHandle) {
        self.paused = !self.paused;
    }

    fn toggle_debug(&mut self, raylib: &mut RaylibHandle) {
        self.debug = !self.debug;
    }

    fn toggle_fullscreen(&mut self, raylib: &mut RaylibHandle) {
        // toggle fullscreen
        self.fullscreen = !self.fullscreen;
        if self.fullscreen {
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
        } else {
            // toggle fullscreen mode
            raylib.toggle_fullscreen();

            // set window size to prev size
            raylib.set_window_size(self.window.prev_width(), self.window.prev_height());
        }
    }
}

impl GameState {
    /// Global update process
    pub fn global_update(&mut self, raylib: &mut RaylibHandle) {
        // fps checker
        if raylib.get_fps() < 15 && raylib.get_time() > 2.0 {
            println!("{}: FPS too low for engine!", window::DEFAULT_TITLE);
            self.exit();
        }

        // scene switcher / hotkeys
        if let Some(key) = raylib.get_key_pressed() {
            match key {
                KeyboardKey::KEY_ESCAPE => self.toggle_pause(raylib),
                KeyboardKey::KEY_F3 => self.toggle_debug(raylib),
                KeyboardKey::KEY_F11 => self.toggle_fullscreen(raylib),
                KeyboardKey::KEY_ONE => self.scene_machine.switch_scene(SceneID::MainMenu),
                KeyboardKey::KEY_TWO => self.scene_machine.switch_scene(SceneID::World),
                KeyboardKey::KEY_THREE => self.scene_machine.switch_scene(SceneID::Loading),
                KeyboardKey::KEY_FOUR => self.scene_machine.switch_scene(SceneID::PauseMenu),
                _ => {}
            }
        }
    }

    /// Global draw process
    pub fn global_draw(&self, raylib: &mut RaylibDrawHandle) {
        // window decorations
        if !self.fullscreen {
            self.window.draw(raylib);
        }

        // pause screen
        if self.paused {
            raylib.draw_text(
                "Paused",
                raylib.get_screen_width() / 2,
                raylib.get_screen_height() / 2,
                50,
                Color::WHITE,
            );
        }

        // debug overlay
        if self.debug {
            // scene debug overlay
            self.scene_machine.debug(raylib);

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
                    &format!("current scene: {:?}", self.scene_machine.id()),
                ),
            ];

            // draw all debug info
            for (i, val) in debug_info.iter().enumerate() {
                let y_pos = 10 + (i as i32 * 20);
                raylib.draw_text(val.1, 10, y_pos, 20, val.0);
            }
        }
    }
}
