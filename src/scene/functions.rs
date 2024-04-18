use json;
use crate::{player, camera};
use std::fs;
use std::io::Read;
use crate::entity::Enemy;
use crate::level::Level;
use crate::scene::Scene;

impl Scene {
    pub fn load_scene(path: String) -> Scene {

        let mut file = fs::File::open(path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let ld: Level = serde_json::from_str(&buffer).unwrap();
        dbg!(&ld);

        Scene {
            player: player::Player::default(),
            camera: camera::Cam::default(),
            entities: Enemy::default(),
            curr_level: ld
        }

    }
}