use raylib::camera::Camera2D;
use crate::camera::Cam;

impl Default for Cam {
    fn default() -> Self {
        Self {
            cam_obj: Camera2D::default(),
        }
    }
}