use super::*;

/// State Exit Functions
impl Player {
    pub fn empty(&mut self, raylib: &mut RaylibHandle) {}

    pub fn exit_wall_sliding(&mut self, raylib: &mut RaylibHandle) {
        if raylib.is_key_down(self.controls.up) {
            println!("wall jump!");

            // add jump force from wall
            self.collider.velocity.x -= self.collider.colliding.x * self.jump * 1.5;
        }
    }
}
