mod level;
mod scene;
mod player;
mod camera;
mod entity;

use raylib::prelude::*;
use uuid;

fn main() {
    let (mut rl, thread) = init().size(1920, 1080).title("Heya").build();
    rl.set_target_fps(60);

    let def_sc = scene::Scene::load_scene("assets/levels/Level_0/data.json".to_string());

    dbg!(def_sc);

    while !rl.window_should_close() {


        let mut renderer = rl.begin_drawing(&thread);
        renderer.clear_background(Color::WHITE);
    }
}
