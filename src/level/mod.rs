mod functions;

use raylib::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    #[serde(rename = "identifier")]
    pub id: String,
    #[serde(rename = "uniqueIdentifer")] // I know it's "identifier" but the export is broken
    pub uid: String,
    #[serde(rename = "x")]
    pub world_x: i8,
    #[serde(rename = "y")]
    pub world_y: i8,
    pub width: i16,
    pub height: i16,
    #[serde(rename = "neighbourLevels")]
    pub neighbours: Vec<NeighbourLevel>,
    pub layers: Vec<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeighbourLevel {
    #[serde(rename = "levelIid")]
    pub uid: String, // universally unique identification
    #[serde(rename = "dir")]
    pub rel_dir: String, // direction relative to current level
}