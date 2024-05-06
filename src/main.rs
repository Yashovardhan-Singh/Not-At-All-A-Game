mod level;
mod scene;
mod player;
mod camera;
mod entity;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = init().size(1920, 1080).title("Heya").build();
    rl.set_target_fps(60);

    let mut def_sc = scene::Scene::load_scene("assets/levels/Level_0".to_string(), &mut rl, &thread);

    let mut p = player::Player::new(0.0, 0.0);

    dbg!(&def_sc);

    while !rl.window_should_close() {

        p.walk(&mut rl);

        let mut renderer = rl.begin_drawing(&thread);
        renderer.clear_background(Color::WHITE);

        def_sc.draw_bg(&mut renderer);
        p.draw(&mut renderer);
    }
}
