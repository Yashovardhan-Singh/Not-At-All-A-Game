use crate::player::Player;
use raylib::prelude::*;

impl Default for Player {
    fn default() -> Self {
        Self {
            bbox: Rectangle{
                x: 0.0,
                y: 0.0,
                width: 16.0 * 7.5,
                height: 16.0 * 7.5,
            }
        }
    }
}
