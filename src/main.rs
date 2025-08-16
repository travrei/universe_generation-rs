use raylib::prelude::*;

use crate::{colors::CRUST, eui::Eui};

mod colors;
mod eui;
mod fast_rand;
mod model;

const SCREEN_WIDTH: i32 = 900;
const SCREEN_HEIGHT: i32 = 800;
const TITLE_WINDOW: &str = "Universe_Demo-rs";

fn main() {
    //----SETUP----
    let mut should_run: bool = true;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(TITLE_WINDOW)
        .resizable()
        .build();

    rl.set_target_fps(75);

    let mut ui = Eui::new();

    //----LOOP-----
    while should_run && !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            should_run = false;
        }

        ui.update(&rl);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(CRUST);

        ui.draw(&mut d);
        ui.show_ui(&mut d);
    }
}
