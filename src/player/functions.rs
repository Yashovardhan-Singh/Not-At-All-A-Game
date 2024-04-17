use raylib::math::Vector2;
use crate::player::Player;

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            position: Vector2::new(x, y)
        }
    }
}