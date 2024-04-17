use raylib::prelude::*;
use json;
use crate::{level, player, camera, entity};
use std::fs;
use std::io::Read;
use uuid::uuid;
use crate::entity::Enemy;
use crate::level::{Level, NeighbourLevel};
use crate::scene::Scene;

impl Scene {
    pub fn load_scene(path: String) -> Scene {

        let mut file = fs::File::open(path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let level_data = json::parse(&buffer).unwrap();

        let nbrs = level_data["neighbourLevels"].entries().map(
            |i| NeighbourLevel {
                uid: uuid!("73b65ae0-d7b0-11ee-88f9-51fa8febe0b5"),
                rel_dir: i.1["dir"].to_string(),
            }
        ).collect::<Vec<NeighbourLevel>>();

        let layers = level_data["layers"].entries().map(
            |i| i.0.to_string()
        ).collect::<Vec<String>>();

        Scene {
            player: player::Player::default(),
            camera: camera::Cam::default(),
            entities: Enemy::default(),
            curr_level: Level {
                id: level_data["identifier"].to_string(),
                uid: uuid!("73b65ae0-d7b0-11ee-88f9-51fa8febe0b5"),
                world_x: level_data["x"].as_i8().unwrap(),
                world_y: level_data["y"].as_i8().unwrap(),
                width: level_data["width"].as_i16().unwrap(),
                height: level_data["height"].as_i16().unwrap(),
                bg_color: Color::from_hex(&level_data["bgColor"].as_str().unwrap()[1..]).unwrap(),
                neighbours: nbrs,
                layers: vec![],
            }
        }

    }
}