#![allow(unused_variables, dead_code)]
//#![windows_subsystem = "windows"]

mod gamestate;
mod paths;
mod player;
mod scenes;

fn main() {
    // raylib setup
    let (mut raylib, thread) = gamestate::GameState::raylib_setup();

    // initialize game state
    let mut game = gamestate::GameState::new(&mut raylib, &thread);

    // run the game
    game.run(&mut raylib, &thread);
}
