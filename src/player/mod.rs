use rayexlib::{
    prelude::{
        AnimationMachineBuilder, AnimationPlayer2D, ColliderInfo, FrameLimiter, Init, Ray2D, Rect2D,
    },
    state_manager::StateManager,
    traits::Renderable,
};
use raylib::prelude::{RaylibDraw, RaylibHandle, RaylibThread, Vector2};

mod controls;
mod states;

use controls::Controls;
pub use states::PlayerState;

impl Player {
    // animation fps
    pub const FPS_IDLE: f32 = 5.0;
    pub const FPS_IDLE_TIRED: f32 = 10.0;
    pub const FPS_WALK: f32 = 5.0;
    pub const FPS_RUN: f32 = 10.0;
    pub const FPS_JUMP: f32 = 5.0;
    pub const FPS_FALL: f32 = 10.0;
    pub const FPS_CRID: f32 = 2.5;
    pub const FPS_CRWK: f32 = 3.0;
    pub const FPS_DIVE: f32 = 20.0;
    pub const FPS_WSLD: f32 = 10.0;

    // sprite & collision sizes
    pub const COLLISION_SIZE: Vector2 = Vector2::new(75.0, 106.0);
    pub const CROUCH_SIZE: f32 = 75.0;
    pub const SPRITE_OFFSET: Vector2 = Vector2::new(50.0, 20.0);
    pub const SPRITE_CR_OFFSET: Vector2 = Vector2::new(50.0, 51.0); // (50, 82)
    pub const SPRITE_SL_SHIFT: f32 = 12.0;
    pub const SPRITE_SCALE: f32 = 3.5;
    pub const SPRITE_SIZE: Vector2 = Vector2::new(50.0, 37.0);
}

pub struct Player {
    // health
    pub max_health: f32,
    pub health: f32,
    pub regeneration: f32,

    // stamina
    pub max_stamina: f32,
    pub stamina: f32,
    pub endurance: f32,
    pub recovery: f32,

    // movement
    pub move_dir: Vector2,
    pub acceleration: f32,
    pub deceleration: f32,
    pub max_speed: f32,
    pub slide_speed: f32,
    pub jump: f32,
    pub dive: f32,

    // physics
    pub collider: Rect2D,
    pub ground_ray: Ray2D,
    pub frame_time: f32,
    pub gravity: f32,
    pub ground_friction: f32,
    pub air_friction: f32,

    // drawing
    pub animation_player: AnimationPlayer2D<PlayerState>,

    // states
    pub controls: Controls,
    pub state: PlayerState,
}

impl Init for Player {
    fn init(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            // health
            max_health: 100.0,
            health: 100.0,
            regeneration: 0.25,

            // stamina
            max_stamina: 100.0,
            stamina: 100.0,
            recovery: 4.0,
            endurance: 10.0,

            // movement
            move_dir: Vector2::zero(),
            acceleration: 25.0,
            deceleration: 8.0,
            max_speed: 500.0,
            slide_speed: 500.0,
            jump: 600.0,
            dive: 1.5,

            // physics
            collider: Rect2D::newv(Player::COLLISION_SIZE).with_position(100.0, -100.0),
            ground_ray: Ray2D::new()
                .with_position(200.0, 100.0)
                .with_direction(Ray2D::DOWN * 50.0),
            frame_time: 0.0,
            gravity: 1500.0,
            ground_friction: 1.0,
            air_friction: 0.25,

            // drawing
            animation_player: {
                use crate::paths::player::advn;

                // add animations
                let mut b = AnimationMachineBuilder::new(Player::SPRITE_SIZE);
                b.add_animation(PlayerState::Idle, advn::IDLE, 4, Player::FPS_IDLE);
                b.add_animation(PlayerState::Running, advn::RUN, 6, Player::FPS_RUN);
                b.add_animation(PlayerState::Jumping, advn::JUMP, 4, Player::FPS_JUMP);
                b.add_animation(PlayerState::Falling, advn::FALL, 2, Player::FPS_FALL);
                b.add_animation(PlayerState::Crouching, advn::CRID, 4, Player::FPS_CRID);
                b.add_animation(PlayerState::CrouchWalking, advn::CRWK, 6, Player::FPS_CRWK);
                b.add_animation(PlayerState::Diving, advn::FALL, 2, Player::FPS_DIVE);
                b.add_animation(PlayerState::WallSliding, advn::WSLD, 2, Player::FPS_WSLD);

                // build animation player
                let mut ap = b.build(raylib, thread);

                // resize all animations
                ap.set_scale(Player::SPRITE_SCALE);
                ap.set_offset(Player::SPRITE_OFFSET);
                ap
            },

            // states
            controls: Controls::default(),
            state: PlayerState::default(),
        }
    }
}

impl Renderable for Player {
    fn update(&mut self, raylib: &mut RaylibHandle) {
        // calculate move direction
        self.move_dir = Vector2 {
            x: (raylib.is_key_down(self.controls.right) as i8
                - raylib.is_key_down(self.controls.left) as i8) as f32,
            y: (raylib.is_key_down(self.controls.down) as i8
                - raylib.is_key_down(self.controls.up) as i8) as f32,
        };

        // get frame time
        self.frame_time = raylib.get_frame_time_limited();

        // face direction
        self.animation_player.face_x(self.move_dir.x);

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

        // current state update
        StateManager::update(self, raylib);

        // terminal velocity
        let terminal_vel = self.gravity * 2.0;
        self.collider.velocity = self.collider.velocity.clamp(-terminal_vel, terminal_vel);

        // move player with velocity
        self.collider.position += self.collider.velocity * self.frame_time;

        // update animation
        self.animation_player.set_position(self.collider.position);
        self.animation_player.next_frame(&self.state);
    }

    fn draw(&self, raylib: &mut impl RaylibDraw) {
        // sprite
        self.animation_player.draw(&self.state, raylib);
    }
}

impl ColliderInfo for Player {
    fn center(&self) -> Vector2 {
        self.collider.center()
    }

    fn velocity(&self) -> Vector2 {
        self.collider.velocity()
    }
}

/// Player physics functions
impl Player {
    pub fn reset_hitbox_from_crouch(&mut self) {
        // move hitbox by offset of sizes
        self.collider.position.y -= Player::COLLISION_SIZE.y - Player::CROUCH_SIZE;
        // reset hitbox size
        self.collider.size = Player::COLLISION_SIZE;
    }

    pub fn reset_colliding(&mut self) {
        // reset player collisions
        self.collider.reset_colliding();
        self.ground_ray.reset_colliding();
    }

    pub fn collide_rects(&mut self, raylib: &mut RaylibHandle, floors: &mut Vec<Rect2D>) {
        // collide player & floors
        for floor in floors {
            floor.reset_colliding();

            if self.collider.collide_rect(floor)
                && self.collider.on_floor()
                && self.move_dir.y == 1.0
            {
                // update ray position
                self.ground_ray.set_position(
                    self.collider.position.x + (Player::CROUCH_SIZE / 2.0),
                    self.collider.position.y + self.collider.size.y,
                );

                // check ray collision
                self.ground_ray.check_rect(floor);
            }
        }
    }
}
