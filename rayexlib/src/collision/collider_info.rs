use raylib::prelude::Vector2;

pub trait ColliderInfo {
    fn center(&self) -> Vector2;
    fn velocity(&self) -> Vector2;
}
