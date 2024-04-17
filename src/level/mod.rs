mod functions;

use raylib::prelude::*;
use uuid;

#[derive(Debug)]
pub struct Level {
    pub id: String,
    pub uid: uuid::Uuid,
    pub world_x: i8,
    pub world_y: i8,
    pub width: i16,
    pub height: i16,
    pub bg_color: Color,
    pub neighbours: Vec<NeighbourLevel>,
    pub layers: Vec<String>,

}

#[derive(Debug)]
pub struct NeighbourLevel {
    pub uid: uuid::Uuid, // universally unique identification
    pub rel_dir: String, // direction relative to current level
}