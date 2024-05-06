use json;
use crate::{player, camera};
use std::fs;
use std::io::Read;
use raylib::prelude::*;
use crate::entity::Enemy;
use crate::level::Level;
use crate::scene::Scene;

impl Scene {
    pub fn load_scene(path: String, rl: &mut RaylibHandle, thread: &RaylibThread) -> Scene {

        let mut file = fs::File::open(path.clone() + &String::from("/data.json")).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let ld: Level = serde_json::from_str(&buffer).unwrap();
        dbg!(&ld);

        let texture = rl.load_texture(thread, &(path + "/_composite.png")).unwrap();

        Scene {
            player: player::Player::default(),
            camera: camera::Cam::default(),
            entities: Enemy::default(),
            curr_level: ld,
            level_bg: texture,
        }

    }

    pub fn draw_bg(&mut self, renderer: &mut RaylibDrawHandle) {
        renderer.draw_texture_ex(&self.level_bg, Vector2::new(0.0, 0.0), 0.0, 7.5, Color::WHITE);
    }
}