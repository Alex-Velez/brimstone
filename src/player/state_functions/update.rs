use super::*;

/// State Update Functions
impl Player {
    pub fn global_update(&mut self, raylib: &mut RaylibHandle) {
        // reset x velocity on wall collision
        if self.collider.on_wall() {
            self.collider.velocity.x = 0.0;
        }

        // gravity
        self.collider.velocity.y = if self.collider.on_floor() {
            0.0 // reset y velocity on floor
        } else {
            // add gravity force
            self.collider.velocity.y + self.gravity * self.frame_time
        };
    }

    pub fn idle_update(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            Player::ground_friction(self);
        }

        // next state
        {
            if self.collider.on_floor() {
                if self.move_dir.x != 0.0 {
                    self.transition(PlayerState::Running, raylib);
                    // self.state = PlayerState::OnRun;
                } else if raylib.is_key_down(self.controls.up) {
                    self.transition(PlayerState::Jumping, raylib);
                    // self.state = PlayerState::OnJump;
                } else if raylib.is_key_down(self.controls.down) {
                    self.transition(PlayerState::Crouching, raylib);
                    // self.state = PlayerState::OnCrouch;
                }
            } else {
                self.transition(PlayerState::Falling, raylib);
                // self.state = PlayerState::OnFall;
            }
        }
    }

    pub fn running_update(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            // accelerate velocity to max speed
            self.collider.velocity.x.lerp(
                self.move_dir.x * self.max_speed,
                self.acceleration * self.frame_time,
            );
        }

        // next state
        {
            if self.collider.on_floor() {
                if self.move_dir.x == 0.0 {
                    self.transition(PlayerState::Idle, raylib);
                } else if raylib.is_key_down(self.controls.down) {
                    self.transition(PlayerState::Crouching, raylib);
                } else if raylib.is_key_down(self.controls.up) {
                    self.transition(PlayerState::Jumping, raylib);
                }
            } else {
                self.transition(PlayerState::Falling, raylib);
            }
        }
    }
    pub fn crouching_update(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            Player::ground_friction(self);
        }

        // next state
        {
            if self.move_dir.x != 0.0 {
                self.transition(PlayerState::CrouchWalking, raylib);
            } else if !raylib.is_key_down(self.controls.down) {
                self.reset_hitbox_from_crouch();
                self.transition(PlayerState::Idle, raylib);
            } else if raylib.is_key_down(self.controls.up) {
                self.reset_hitbox_from_crouch();
                self.transition(PlayerState::Jumping, raylib);
            } else if self.collider.on_wall() && !self.collider.on_floor() {
                self.reset_hitbox_from_crouch();
                self.transition(PlayerState::WallSliding, raylib);
            }
        }
    }

    pub fn crouch_walking_update(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            // accelerate velocity to crouch walk speed
            self.collider.velocity.x.lerp(
                self.move_dir.x * (self.max_speed / 4.0),
                self.acceleration * self.frame_time,
            );
        }

        // next state
        {
            if self.move_dir.x == 0.0 {
                self.transition(PlayerState::Crouching, raylib);
            } else if !raylib.is_key_down(self.controls.down) {
                self.reset_hitbox_from_crouch();
                self.transition(PlayerState::Running, raylib);
            } else if raylib.is_key_down(self.controls.up) {
                self.reset_hitbox_from_crouch();
                self.transition(PlayerState::Jumping, raylib);
            } else if self.collider.on_wall() && !self.collider.on_floor() {
                self.reset_hitbox_from_crouch();
                self.transition(PlayerState::WallSliding, raylib);
            }
        }
    }

    pub fn jumping_update(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            Player::air_friction(self);
        }

        // next state
        {
            if raylib.is_key_down(self.controls.down) {
                self.transition(PlayerState::Diving, raylib);
            } else if self.collider.velocity.y > 0.0 {
                self.transition(PlayerState::Falling, raylib);
            } else if self.collider.on_floor() {
                if self.move_dir.x == 0.0 {
                    self.transition(PlayerState::Idle, raylib);
                } else {
                    self.transition(PlayerState::Running, raylib);
                }
            }
        }
    }

    pub fn falling_update(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            Player::air_friction(self);
        }

        // next state
        {
            let a = self.collider.on_floor();
            let b = self.collider.on_wall();
            let c = self.move_dir.x == 0.0;
            let d = raylib.is_key_down(self.controls.down);

            match (a, b, c, d) {
                (true, _, true, false) => self.transition(PlayerState::Idle, raylib),
                (true, _, false, false) => self.transition(PlayerState::Running, raylib),
                (true, _, true, true) => self.transition(PlayerState::Crouching, raylib),
                (true, _, false, true) => self.transition(PlayerState::CrouchWalking, raylib),
                (false, true, _, false) => self.transition(PlayerState::WallSliding, raylib),
                (false, _, _, true) => self.transition(PlayerState::Diving, raylib),
                _ => {}
            };
        }
    }

    pub fn wall_sliding_update(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            self.collider.velocity.y = self.collider.velocity.y.min(self.max_speed);
        }

        // next state
        {
            if self.collider.on_floor() {
                if self.move_dir.x == 0.0 {
                    self.transition(PlayerState::Idle, raylib);
                } else {
                    self.transition(PlayerState::Running, raylib);
                }
            } else {
                if raylib.is_key_down(self.controls.up) && self.move_dir.x == 0.0 {
                    self.transition(PlayerState::Jumping, raylib);
                }
            }
        }
    }

    pub fn diving_update(&mut self, raylib: &mut RaylibHandle) {
        // action
        {
            Player::air_friction(self);

            // add downward velocity
            self.collider.velocity.y += self.gravity * self.dive * self.frame_time;
        }

        // next state
        {
            let a = self.collider.on_floor();
            let b = self.move_dir.x == 0.0;
            let c = raylib.is_key_down(self.controls.down);

            match (a, b, c) {
                (true, true, _) => self.transition(PlayerState::Idle, raylib),
                (true, false, _) => self.transition(PlayerState::Running, raylib),
                (false, _, false) => self.transition(PlayerState::Falling, raylib),
                _ => {}
            };
        }
    }
}
