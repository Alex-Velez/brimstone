use crate::{
    debug::{DebugTools, DebugUtil},
    scene_machine::SceneMachine,
    scenes::{SceneID, Stage},
    window::{self, Window},
};
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread};

pub struct GameState {
    window: Window,
    stage: Stage,
    scene_machine: SceneMachine<SceneID>,
    paused: bool,
    exit: bool,
    pub debug: DebugTools,
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
        // toggle debug
        if raylib.is_key_pressed(raylib::prelude::KeyboardKey::KEY_F3) {
            self.toggle_debug();
        }

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

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn toggle_debug(&mut self) {
        self.debug.active = !self.debug.active;
    }

    pub fn toggle_fullscreen(&mut self, raylib: &mut RaylibHandle) {
        if raylib.is_window_fullscreen() {
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

    pub fn current_scene(&self) -> &SceneID {
        self.scene_machine.current_scene()
    }

    pub fn current_scene_debug(&self, raylib: &mut RaylibDrawHandle) {
        self.scene_machine.debug(raylib);
    }

    pub fn next_scene(&mut self, raylib: &mut RaylibHandle, next_scene: SceneID) {
        self.scene_machine.next_scene(raylib, next_scene);
    }
}

impl GameState {
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
