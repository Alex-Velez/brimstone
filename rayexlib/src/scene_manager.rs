use raylib::{core::RaylibHandle, drawing::RaylibDrawHandle};
use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub trait Scene<GlobalEnv> {
    fn on_enter(&mut self, _global_env: &mut GlobalEnv, _raylib: &mut RaylibHandle) {}
    fn on_exit(&mut self, _global_env: &mut GlobalEnv, _raylib: &mut RaylibHandle) {}
    fn update(&mut self, global_env: &mut GlobalEnv, raylib: &mut RaylibHandle);
    fn draw(&self, global_env: &GlobalEnv, raylib: &mut RaylibDrawHandle);
    fn debug(&self, _global_env: &GlobalEnv, _raylib: &mut RaylibDrawHandle) {}
}

pub struct SceneManager<ID, GlobalEnv>
where
    ID: Debug + Default + PartialEq + Eq + Hash,
{
    pub id: ID,
    scenes: HashMap<ID, Box<dyn Scene<GlobalEnv>>>,
}

impl<ID, GlobalEnv> Default for SceneManager<ID, GlobalEnv>
where
    ID: Debug + Default + PartialEq + Eq + Hash,
{
    fn default() -> Self {
        Self {
            id: ID::default(),
            scenes: HashMap::new(),
        }
    }
}

impl<ID, GlobalEnv> SceneManager<ID, GlobalEnv>
where
    ID: Debug + Default + PartialEq + Eq + Hash,
{
    pub fn add_scene(&mut self, id: ID, scene: impl Scene<GlobalEnv> + 'static) {
        self.scenes.insert(id, Box::new(scene));
    }

    pub fn next_scene(
        &mut self,
        global_env: &mut GlobalEnv,
        raylib: &mut RaylibHandle,
        next_scene_id: ID,
    ) {
        self.on_exit(global_env, raylib);
        self.id = next_scene_id;
        self.on_enter(global_env, raylib);
    }

    pub fn on_enter(&mut self, global_env: &mut GlobalEnv, raylib: &mut RaylibHandle) {
        if let Some(scene) = self.scenes.get_mut(&self.id) {
            scene.on_enter(global_env, raylib);
        } else {
            panic!("Invalid scene: {:?}", self.id);
        }
    }

    pub fn on_exit(&mut self, global_env: &mut GlobalEnv, raylib: &mut RaylibHandle) {
        if let Some(scene) = self.scenes.get_mut(&self.id) {
            scene.on_exit(global_env, raylib);
        } else {
            panic!("Invalid scene: {:?}", self.id);
        }
    }

    pub fn update(&mut self, global_env: &mut GlobalEnv, raylib: &mut RaylibHandle) {
        if let Some(scene) = self.scenes.get_mut(&self.id) {
            scene.update(global_env, raylib);
        } else {
            panic!("Invalid scene: {:?}", self.id);
        }
    }

    pub fn draw(&self, global_env: &GlobalEnv, raylib: &mut RaylibDrawHandle) {
        if let Some(scene) = self.scenes.get(&self.id) {
            scene.draw(global_env, raylib);
        } else {
            panic!("Invalid scene: {:?}", self.id);
        }
    }

    pub fn debug(&self, global_env: &GlobalEnv, raylib: &mut RaylibDrawHandle) {
        if let Some(scene) = self.scenes.get(&self.id) {
            scene.debug(global_env, raylib);
        } else {
            panic!("Invalid scene: {:?}", self.id);
        }
    }
}
