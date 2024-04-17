mod traits;
mod functions;

use raylib::prelude::*;

#[derive(Debug)]
pub struct Player {
    position: Vector2,
}