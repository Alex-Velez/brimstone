#![allow(unused_variables, dead_code)]
//#![windows_subsystem = "windows"]

mod button;
mod collision;
mod gamestate;
mod math;
mod paths;
mod player;
mod raylib_plugins;
mod scene_machine;
mod scenes;
mod sprite;
mod timer;
mod window;

fn main() {
    // initialize raylib audio
    raylib::audio::RaylibAudio::init_audio_device();

    // initialize raylib
    let (mut raylib, thread) = raylib::init()
        .title(window::DEFAULT_TITLE)
        .size(window::DEFAULT_WIDTH, window::DEFAULT_HEIGHT)
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
