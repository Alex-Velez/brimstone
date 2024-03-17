use raylib::RaylibHandle;

pub trait StateManager<State> {
    fn update(entity: &mut Self, raylib: &mut RaylibHandle);
    fn on_enter(entity: &mut Self, raylib: &mut RaylibHandle);
    fn on_exit(entity: &mut Self, raylib: &mut RaylibHandle);
    fn next_state(entity: &mut Self, next_state: State, raylib: &mut RaylibHandle);
}
