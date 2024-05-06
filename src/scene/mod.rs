mod functions;

use raylib::prelude::*;
use crate::{level, player, camera, entity};

#[derive(Debug)]
pub struct Scene {
    pub player: player::Player,
    pub camera: camera::Cam,
    pub curr_level: level::Level,
    pub entities: entity::Enemy,
    pub level_bg: Texture2D,
}
