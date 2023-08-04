#![allow(unused_variables, dead_code)]
//#![windows_subsystem = "windows"]

mod engine;
mod gamestate;
mod paths;
mod player;
mod scenes;

use engine::prelude::Window;

fn main() {
    // initialize raylib audio
    raylib::audio::RaylibAudio::init_audio_device();

    // initialize raylib
    let (mut raylib, thread) = raylib::init()
        .title(Window::DEFAULT_TITLE)
        .size(Window::DEFAULT_WIDTH, Window::DEFAULT_HEIGHT)
        .resizable()
        // .undecorated()
        .build();

    // raylib setup
    {
        raylib.set_exit_key(None);
        // raylib.set_target_fps(30);

        // switch controller mappings
        // unsafe {
        //     let path = std::fs::read_to_string(paths::MAPPING).unwrap();
        //     let cpath = std::ffi::CString::new(path).unwrap();
        //     raylib::ffi::SetGamepadMappings(cpath.as_ptr());
        // }
    }

    // initialize game state
    let mut state = gamestate::GameState::new(&mut raylib, &thread);

    // run the game
    state.run(&mut raylib, &thread);
}
