mod traits;
mod functions;

use raylib::prelude::*;

#[derive(Debug)]
pub struct Cam {
    cam_obj: Camera2D,
}