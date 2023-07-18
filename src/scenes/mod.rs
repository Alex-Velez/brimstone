use crate::scene_machine::SceneMachine;
use raylib::prelude::{RaylibHandle, RaylibThread};
use std::{fmt::Debug, hash::Hash};

mod loading;
mod main_menu;
mod pause_menu;
mod world;

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
impl SceneMachine<SceneID> {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // init scenes
        let main_menu = main_menu::Environment::init(raylib, thread);
        let world = world::Environment::init(raylib, thread);
        let loading = loading::Environment::init(raylib, thread);
        let pause_menu = pause_menu::Environment::init(raylib, thread);

        // add scenes to scene machine
        let mut machine = SceneMachine::default();
        machine.add_scene(SceneID::MainMenu, main_menu);
        machine.add_scene(SceneID::World, world);
        machine.add_scene(SceneID::Loading, loading);
        machine.add_scene(SceneID::PauseMenu, pause_menu);
        machine
    }
}

/// Scene container
pub struct Stage {
    pause_menu: pause_menu::Environment,
    main_menu: main_menu::Environment,
    loading: loading::Environment,
    world: world::Environment,
}

impl Stage {
    pub fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            pause_menu: pause_menu::Environment::init(raylib, thread),
            main_menu: main_menu::Environment::init(raylib, thread),
            loading: loading::Environment::init(raylib, thread),
            world: world::Environment::init(raylib, thread),
        }
    }
}
