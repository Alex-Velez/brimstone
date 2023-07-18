use raylib::prelude::{RaylibDrawHandle, RaylibHandle, RaylibThread};
use std::collections::HashMap;

mod loading;
mod main_menu;
mod pause_menu;
mod world;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SceneID {
    Loading,
    MainMenu,
    #[default]
    World,
    EndMenu,
    SplashScreen,
    PauseMenu,
}

pub trait Scene {
    fn on_enter(&mut self, raylib: &mut RaylibHandle);
    fn on_exit(&mut self, raylib: &mut RaylibHandle);
    fn update(&mut self, raylib: &mut RaylibHandle);
    fn draw(&self, raylib: &mut RaylibDrawHandle);
    fn debug(&self, raylib: &mut RaylibDrawHandle);
    fn id(&self) -> String;
}

/// Scene manager
pub struct SceneMachine {
    scenes: HashMap<SceneID, Box<dyn Scene>>,
    current_id: SceneID,
}

impl Default for SceneMachine {
    fn default() -> Self {
        Self {
            scenes: HashMap::new(),
            current_id: SceneID::default(),
        }
    }
}

impl SceneMachine {
    pub fn add_scene<T: Scene + 'static>(&mut self, id: SceneID, scene: T) {
        self.scenes.insert(id, Box::new(scene));
    }

    pub fn switch_scene(&mut self, scene_id: SceneID) {
        match self.scenes.contains_key(&scene_id) {
            true => self.current_id = scene_id,
            false => panic!("Invalid scene: {:?}", scene_id),
        }
    }

    pub fn on_enter(&mut self, raylib: &mut RaylibHandle) {
        match self.scenes.get_mut(&self.current_id) {
            Some(scene) => scene.on_enter(raylib),
            None => panic!("Invalid scene: {:?}", self.current_id),
        }
    }

    pub fn on_exit(&mut self, raylib: &mut RaylibHandle) {
        match self.scenes.get_mut(&self.current_id) {
            Some(scene) => scene.on_exit(raylib),
            None => panic!("Invalid scene: {:?}", self.current_id),
        }
    }

    pub fn update(&mut self, raylib: &mut RaylibHandle) {
        match self.scenes.get_mut(&self.current_id) {
            Some(scene) => scene.update(raylib),
            None => panic!("Invalid scene: {:?}", self.current_id),
        }
    }

    pub fn draw(&self, raylib: &mut RaylibDrawHandle) {
        match self.scenes.get(&self.current_id) {
            Some(scene) => scene.draw(raylib),
            None => panic!("Invalid scene: {:?}", self.current_id),
        }
    }

    pub fn debug(&self, raylib: &mut RaylibDrawHandle) {
        match self.scenes.get(&self.current_id) {
            Some(scene) => scene.debug(raylib),
            None => panic!("Invalid scene: {:?}", self.current_id),
        }
    }

    pub fn id(&self) -> SceneID {
        self.current_id
    }
}

/// Default configuration for scenes
impl SceneMachine {
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

/// Scene object container
pub struct Stage {
    pause_menu: pause_menu::Environment,
    main_menu: main_menu::Environment,
    loading: loading::Environment,
    world: world::Environment,
}

impl Stage {
    pub fn new(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            pause_menu: pause_menu::Environment::init(raylib, thread),
            main_menu: main_menu::Environment::init(raylib, thread),
            loading: loading::Environment::init(raylib, thread),
            world: world::Environment::init(raylib, thread),
        }
    }
}
