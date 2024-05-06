use raylib::prelude::*;
use crate::player::Player;

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            bbox: Rectangle::new(x, y, 16.0 * 7.5, 16.0 * 7.5),
        }
    }

    pub fn walk(&mut self, rl: &mut RaylibHandle) {
        self.bbox.x += rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X) * 2.0;
        self.bbox.y += rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y) * 2.0;
    }

    pub fn draw(&mut self, renderer: &mut RaylibDrawHandle) {
        renderer.draw_rectangle_rec(self.bbox, Color::GRAY);
    }
}