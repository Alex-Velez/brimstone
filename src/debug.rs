use crate::{gamestate::GameState, raylib_plugins::FrameLimiter, scenes::SceneID};
use raylib::prelude::{Color, KeyboardKey, RaylibDraw, RaylibDrawHandle, RaylibHandle};

pub struct DebugTools {
    pub active: bool,
    pub step_frames: bool,
    pub paused: bool,
    pub step_fps: u32,
}

impl Default for DebugTools {
    fn default() -> Self {
        Self {
            active: true,
            step_frames: false,
            paused: false,
            step_fps: 30,
        }
    }
}

pub trait DebugUtil {
    fn debug_update(&mut self, raylib: &mut RaylibHandle);
    fn debug_draw(&self, raylib: &mut RaylibDrawHandle);
}

impl DebugUtil for GameState {
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

        // hotkeys
        if let Some(key) = raylib.get_key_pressed() {
            match key {
                // step forward one frame
                KeyboardKey::KEY_KP_6 => self.debug.paused = !self.debug.paused,
                // toggle step frames
                KeyboardKey::KEY_KP_0 => {
                    self.debug.step_frames = !self.debug.step_frames;
                    self.debug.paused = false;
                }
                // toggle pause
                KeyboardKey::KEY_ESCAPE => self.toggle_pause(),
                // toggle debug
                KeyboardKey::KEY_F3 => self.toggle_debug(),
                // toggle fullscreen
                KeyboardKey::KEY_F11 => self.toggle_fullscreen(raylib),
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
                // scene switchers
                KeyboardKey::KEY_ONE => self.next_scene(raylib, SceneID::MainMenu),
                KeyboardKey::KEY_TWO => self.next_scene(raylib, SceneID::World),
                KeyboardKey::KEY_THREE => self.next_scene(raylib, SceneID::Loading),
                KeyboardKey::KEY_FOUR => self.next_scene(raylib, SceneID::PauseMenu),
                _ => {}
            }
        }
    }

    fn debug_draw(&self, raylib: &mut RaylibDrawHandle) {
        // scene debug overlay
        self.current_scene_debug(raylib);

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
                &format!("current scene: {:?}", self.current_scene()),
            ),
        ];

        // draw all debug info
        for (i, val) in debug_info.iter().enumerate() {
            let y_pos = 10 + (i as i32 * 20);
            raylib.draw_text(val.1, 10, y_pos, 20, val.0);
        }
    }
}
