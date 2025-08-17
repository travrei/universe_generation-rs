use macroquad::prelude::*;

use crate::config::{CRUST, window_config};

mod config;

#[macroquad::main(window_config())]
async fn main() {
    //--Setup--
    let mut should_run: bool = true;

    //--Main Loop--
    while should_run {
        if is_key_pressed(KeyCode::Tab){
            should_run = false;
        }
        
        clear_background(CRUST);

        draw_text("Initial Development!", 20., 20., 20., WHITE);

        next_frame().await
    }
}
