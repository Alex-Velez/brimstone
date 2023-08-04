use raylib::prelude::{RaylibDrawHandle, RaylibHandle};
use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// Scene functions trait
pub trait Scene {
    fn on_enter(&mut self, raylib: &mut RaylibHandle) {}
    fn on_exit(&mut self, raylib: &mut RaylibHandle) {}
    fn update(&mut self, raylib: &mut RaylibHandle);
    fn draw(&self, raylib: &mut RaylibDrawHandle);
    fn debug(&self, raylib: &mut RaylibDrawHandle) {}
}

/// Scene manager
pub struct SceneMachine<ID: Debug + Default + PartialEq + Eq + Hash> {
    scenes: HashMap<ID, Box<dyn Scene>>,
    current_id: ID,
}

impl<ID: Debug + Default + PartialEq + Eq + Hash> Default for SceneMachine<ID> {
    fn default() -> Self {
        Self {
            scenes: HashMap::new(),
            current_id: ID::default(),
        }
    }
}

impl<ID: Debug + Default + PartialEq + Eq + Hash> SceneMachine<ID> {
    pub fn add_scene<T: Scene + 'static>(&mut self, id: ID, scene: T) {
        self.scenes.insert(id, Box::new(scene));
    }

    pub fn next_scene(&mut self, raylib: &mut RaylibHandle, scene_id: ID) {
        match self.scenes.contains_key(&scene_id) {
            true => {
                self.on_exit(raylib);
                self.current_id = scene_id;
                self.on_enter(raylib);
            }
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

    pub fn current_scene(&self) -> &ID {
        &self.current_id
    }
}
