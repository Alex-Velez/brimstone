use rayexlib::prelude::SceneManager;
use raylib::prelude::{RaylibHandle, RaylibThread};
use std::{fmt::Debug, hash::Hash};

mod global;
mod loading;
mod main_menu;
mod pause_menu;
pub mod world;

pub use global::GlobalEnvironment;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub enum SceneID {
    Loading,
    MainMenu,
    #[default]
    World,
    EndMenu,
    SplashScreen,
    PauseMenu,
}

/// Default configuration for scenes
pub trait SceneInitializer {
    fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self;
}

impl SceneInitializer for SceneManager<SceneID, GlobalEnvironment> {
    fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // init scenes
        let main_menu = main_menu::Environment::init(raylib, thread);
        let world = world::Environment::init(raylib, thread);
        let loading = loading::Environment::init(raylib, thread);
        let pause_menu = pause_menu::Environment::init(raylib, thread);

        // add scenes to scene machine
        let mut machine = SceneManager::default();
        machine.add_scene(SceneID::MainMenu, main_menu);
        machine.add_scene(SceneID::World, world);
        machine.add_scene(SceneID::Loading, loading);
        machine.add_scene(SceneID::PauseMenu, pause_menu);
        machine
    }
}
