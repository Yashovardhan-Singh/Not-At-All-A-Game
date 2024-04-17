use crate::player::Player;
use raylib::prelude::*;

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Vector2{
                ..Default::default()
            }
        }
    }
}
