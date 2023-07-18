use super::*;

/// State Enter Functions
impl Player {
    pub fn on_idle(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            println!("idled!");
        }

        // next state
        self.state = PlayerState::Idle;
    }

    pub fn on_run(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            println!("ran!");
        }

        // next state
        self.state = PlayerState::Running;
    }

    pub fn on_crouch(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            println!("crouched!");

            if self.collider.size.y != Player::COLLISION_SIZE.x {
                // change hitbox height
                self.collider.size.y = Player::COLLISION_SIZE.x;
                // move hitbox by offset of sizes
                self.collider.position.y += Player::COLLISION_SIZE.y - Player::COLLISION_SIZE.x;
            }
        }

        // next state
        self.state = PlayerState::Crouching;
    }

    pub fn on_crouch_walk(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            println!("crouch walk!");
        }

        // next state
        self.state = PlayerState::CrouchWalking;
    }

    pub fn on_jump(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            println!("jumped!");

            // reset jump animation
            self.animation_player.reset_frame(PlayerState::Jumping);

            // add jump force
            self.collider.velocity.y -= self.jump;
        }

        // next state
        self.state = PlayerState::Jumping;
    }

    pub fn on_fall(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            println!("fall!");
        }

        // next state
        self.state = PlayerState::Falling;
    }

    pub fn on_wall_slide(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            println!("wall slide!");
            if raylib.is_key_down(self.controls.up) {
                self.collider.velocity.y = 0.0;
            }
        }

        // next state
        self.state = PlayerState::WallSliding;
    }

    pub fn on_wall_jump(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            println!("wall jump!");

            // reset jump animation
            self.animation_player.reset_frame(PlayerState::Jumping);

            // add jump force up
            self.collider.velocity.y -= self.jump;

            // add jump force from wall
            self.collider.velocity.x -= self.collider.colliding.x * self.jump * 1.5;
        }

        // next state
        {
            self.state = PlayerState::Jumping;
        }
    }

    pub fn on_dive(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            println!("dive!");
        }

        // next state
        self.state = PlayerState::Diving;
    }
}
