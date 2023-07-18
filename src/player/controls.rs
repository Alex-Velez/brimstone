use super::*;

pub struct Controls {
    pub up: KeyboardKey,
    pub down: KeyboardKey,
    pub left: KeyboardKey,
    pub right: KeyboardKey,
    pub attack: MouseButton,
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            up: KeyboardKey::KEY_W,
            down: KeyboardKey::KEY_S,
            left: KeyboardKey::KEY_A,
            right: KeyboardKey::KEY_D,
            attack: MouseButton::MOUSE_LEFT_BUTTON,
        }
    }
}
