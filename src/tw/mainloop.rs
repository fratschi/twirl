use crate::tw::twirl::Tw;
use raylib::{color::Color, ffi::KeyboardKey, prelude::RaylibDraw, RaylibHandle};

pub fn render_loop(rl: &mut RaylibHandle, thread: &raylib::RaylibThread, tw: Vec<Vec<Tw>>) {
    let mut index = 0;
    let width = rl.get_screen_width();
    let height = rl.get_screen_height();

    while !rl.window_should_close() {

        // Handle key presses before mutable borrow
        let right_pressed = rl.is_key_pressed(KeyboardKey::KEY_RIGHT);
        let left_pressed = rl.is_key_pressed(KeyboardKey::KEY_LEFT);

        if right_pressed {
            index = (index + 1) % tw.len();
        }
        if left_pressed {
            if index == 0 {
                index = tw.len() - 1;
            } else {
                index -= 1;
            }
        }

        let mut d: raylib::prelude::RaylibDrawHandle<'_> = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for t in tw[index].iter() {
            match t {
                Tw::Color(c) => {
                    let r = ((*c >> 16) & 0xFF) as u8;
                    let g = ((*c >> 8) & 0xFF) as u8;
                    let b = (*c & 0xFF) as u8;
                    d.draw_rectangle(10, 10, 100, 100, Color::new(r, g, b, 255));
                }
                Tw::Text(s) => {
                    d.draw_text(s, 120, 20, 20, Color::WHITE);
                }
            }
        }
    }
}
