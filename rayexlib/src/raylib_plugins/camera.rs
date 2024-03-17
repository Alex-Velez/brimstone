use super::{FrameLimiter, Math, Window};
use raylib::prelude::{Camera2D, RaylibHandle, Vector2};

const ZOOM_MINIMUM: f32 = 0.5; // zoom out multiplier
const ZOOM_MAXIMUM: f32 = 1.00; // zoom in multiplier
const ZOOM_STEP_AMOUNT: f32 = 64.00;

pub struct CameraEx2D {
    camera: Camera2D,
    proj_zoom: f32,
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub zoom_step: f32,
    pub zoom_minimum: f32,
    pub zoom_maximum: f32,
    pub zoom_scale: f32,
    pub rotation: f32,
    pub shaking: bool,
    pub shake_speed: f32,
    pub shake_mult: f32,
}

impl Default for CameraEx2D {
    fn default() -> Self {
        Self {
            camera: Camera2D {
                zoom: 1.0,
                ..Default::default()
            },
            proj_zoom: 1.0,
            zoom_speed: 1.0,
            zoom_step: 10.0,
            move_speed: 1.0,
            zoom_minimum: ZOOM_MINIMUM,
            zoom_maximum: ZOOM_MAXIMUM,
            zoom_scale: 1.0,
            rotation: 0.0,
            shaking: false,
            shake_speed: 1.0,
            shake_mult: 1.0,
        }
    }
}

impl Into<raylib::ffi::Camera2D> for &CameraEx2D {
    fn into(self) -> raylib::ffi::Camera2D {
        self.camera.into()
    }
}

impl CameraEx2D {
    pub fn zoom(&self) -> f32 {
        self.camera.zoom
    }

    pub fn set_offset(&mut self, offset_x: f32, offset_y: f32) {
        self.camera.offset.x = offset_x;
        self.camera.offset.y = offset_y;
    }

    pub fn with_offset(mut self, offset_x: f32, offset_y: f32) -> Self {
        self.camera.offset.x = offset_x;
        self.camera.offset.y = offset_y;
        self
    }

    pub fn with_target(mut self, target: Vector2) -> Self {
        self.camera.target = target;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.camera.rotation = rotation;
        self
    }

    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.camera.zoom = zoom;
        self
    }

    pub fn with_zoom_speed(mut self, zoom_speed: f32) -> Self {
        self.zoom_speed = zoom_speed;
        self
    }

    pub fn with_zoom_step(mut self, zoom_step: f32) -> Self {
        self.zoom_step = zoom_step;
        self
    }

    pub fn with_move_speed(mut self, move_speed: f32) -> Self {
        self.move_speed = move_speed;
        self
    }

    pub fn with_zoom_min(mut self, zoom_min: f32) -> Self {
        self.zoom_minimum = zoom_min;
        self
    }

    pub fn with_zoom_max(mut self, zoom_max: f32) -> Self {
        self.zoom_maximum = zoom_max;
        self
    }

    pub fn with_shaking(mut self, shaking: bool) -> Self {
        self.shaking = shaking;
        self
    }

    pub fn with_shake_speed(mut self, shake_speed: f32) -> Self {
        self.shake_speed = shake_speed;
        self
    }

    pub fn with_shake_mult(mut self, shake_mult: f32) -> Self {
        self.shake_mult = shake_mult;
        self
    }

    pub fn follow(&mut self, target: Vector2, raylib: &mut RaylibHandle) {
        let win_width = raylib.get_screen_width() as f32;
        let win_height = raylib.get_screen_height() as f32;

        // keep target in center of screen
        self.camera.offset = Vector2::new(win_width / 2.0, win_height / 2.0);

        // adjust zoom scale
        self.zoom_scale =
            ((win_width * win_height) / (Window::DEFAULT_SIZE.x * Window::DEFAULT_SIZE.y)).sqrt();

        // frame time
        let frame_time = raylib.get_frame_time_limited();

        // control camera zoom
        self.proj_zoom += raylib.get_mouse_wheel_move() / self.zoom_step;

        // limit zoom
        self.proj_zoom = self.proj_zoom.clamp(self.zoom_minimum, self.zoom_maximum);

        // smooth camera zoom
        self.camera.zoom.lerp(
            self.proj_zoom * self.zoom_scale,
            self.zoom_speed * frame_time,
        );

        // camera shake
        if self.shaking {
            self.camera.rotation = self.rotation
                + ((raylib.get_time() as f32 * self.shake_speed).sin() * self.shake_mult);
        } else {
            self.camera.rotation = self.rotation;
        }

        // make camera follow target
        self.camera.target = self
            .camera
            .target
            .lerp(target, self.move_speed * frame_time);
    }

    /// Speed up camera move speed for fast velocities
    pub fn follow_bound(&mut self, velocity: Vector2, damping: f32, max: f32) {
        self.move_speed = (velocity.length() / damping).max(max);
    }
}

pub trait CameraMovement {
    fn follow(&mut self, target: Vector2, raylib: &mut RaylibHandle);
}

impl CameraMovement for Camera2D {
    fn follow(&mut self, target: Vector2, raylib: &mut RaylibHandle) {
        // keep camera/player in center
        if raylib.is_window_resized() {
            self.offset = Vector2::new(
                raylib.get_screen_width() as f32 / 2.0,
                raylib.get_screen_height() as f32 / 2.0,
            );
        }

        // control camera zoom
        self.zoom += raylib.get_mouse_wheel_move() * (ZOOM_MAXIMUM / ZOOM_STEP_AMOUNT);

        // limit zoom scale
        self.zoom = self.zoom.clamp(ZOOM_MINIMUM, ZOOM_MAXIMUM);

        // make camera follow target
        self.target = target;
    }
}
