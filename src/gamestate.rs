use crate::{
    paths,
    scenes::{GlobalEnvironment, SceneID, SceneInitializer},
};
use rayexlib::prelude::{
    Debug, DebugTools, FrameLimiter, ImagePlugin, SceneManager, Toggle, Window,
};
use raylib::prelude::{
    Color, Image, KeyboardKey, MouseCursor, RaylibDraw, RaylibDrawHandle, RaylibHandle,
    RaylibThread, Rectangle,
};

pub struct GameState {
    window: Window,
    scene_machine: SceneManager<SceneID, GlobalEnvironment>,
    global_env: GlobalEnvironment,
    paused: bool,
    exit: bool,
    debug: Debug,
}

impl GameState {
    pub fn new(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            window: Window::new(),
            scene_machine: SceneManager::init(raylib, thread),
            global_env: GlobalEnvironment::init(raylib, thread),
            paused: false,
            exit: false,
            debug: Debug::default(),
        }
    }

    pub fn raylib_setup() -> (RaylibHandle, RaylibThread) {
        // initialize raylib audio
        let rl_audio = raylib::audio::RaylibAudio::init_audio_device();

        // initialize raylib (1920:1080 -> 864:486) (20/9)
        let (mut raylib, thread) = raylib::init()
            .title("Brimstone")
            .size(100, 100)
            .resizable()
            // .undecorated()
            .build();

        // Default window size
        let (monitor_width, monitor_height) = unsafe {
            let current_monitor = raylib::ffi::GetCurrentMonitor();
            let monitor_width = raylib::ffi::GetMonitorWidth(current_monitor);
            let monitor_height = raylib::ffi::GetMonitorHeight(current_monitor);
            (monitor_width, monitor_height)
        };
        let window_width = (monitor_width as f32 * Window::DEFAULT_SIZE_SCALAR) as i32;
        let window_height = (monitor_height as f32 * Window::DEFAULT_SIZE_SCALAR) as i32;
        let window_x = (monitor_width - window_width) / 2;
        let window_y = (monitor_height - window_height) / 2;
        raylib.set_window_size(window_width, window_height);
        raylib.set_window_position(window_x, window_y);

        // settings
        raylib.set_window_icon(Image::from_path(paths::ICON));
        raylib.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_CROSSHAIR);
        raylib.set_exit_key(None);
        // raylib.set_target_fps(60);

        // switch controller mappings
        // unsafe {
        //     let path = std::fs::read_to_string(paths::MAPPING).unwrap();
        //     let cpath = std::ffi::CString::new(path).unwrap();
        //     raylib::ffi::SetGamepadMappings(cpath.as_ptr());
        // }

        (raylib, thread)
    }

    pub fn run(&mut self, raylib: &mut RaylibHandle, thread: &RaylibThread) {
        // call current scene on_start fn
        self.scene_machine.on_enter(&mut self.global_env, raylib);

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
            self.scene_machine.update(&mut self.global_env, raylib);
        }
    }

    fn draw(&self, raylib: &mut RaylibDrawHandle) {
        // current scene draw
        self.scene_machine.draw(&self.global_env, raylib);

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
        // debug utilities
        if self.debug.active {
            self.debug_update(raylib);
        } else {
            // fps checker
            if raylib.get_fps() < 15 && raylib.get_time() > 5.0 {
                println!("Engine: FPS too low for engine!");
                self.exit();
            }
        }

        // hot keys
        if let Some(key) = raylib.get_key_pressed() {
            // global hot keys
            match key {
                // toggle pause
                KeyboardKey::KEY_ESCAPE => self.paused.toggle(),
                // toggle fullscreen
                KeyboardKey::KEY_F11 => self.toggle_fullscreen(raylib),
                // toggle debug
                KeyboardKey::KEY_F3 => self.debug.active.toggle(),
                // scene switchers
                KeyboardKey::KEY_ONE => {
                    self.scene_machine
                        .next_scene(&mut self.global_env, raylib, SceneID::MainMenu)
                }
                KeyboardKey::KEY_TWO => {
                    self.scene_machine
                        .next_scene(&mut self.global_env, raylib, SceneID::World)
                }
                KeyboardKey::KEY_THREE => {
                    self.scene_machine
                        .next_scene(&mut self.global_env, raylib, SceneID::Loading)
                }
                KeyboardKey::KEY_FOUR => {
                    self.scene_machine
                        .next_scene(&mut self.global_env, raylib, SceneID::PauseMenu)
                }
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

impl DebugTools for GameState {
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
        self.scene_machine.debug(&self.global_env, raylib);

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
                &format!("current scene: {:?}", self.scene_machine.id),
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
