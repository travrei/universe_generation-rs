use raylib::prelude::*;

use crate::{fast_rand::FastRand, model::StarSystem};
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
pub struct Eui {
    sec_size: i32,
    cam: Point,
    mouse_pos: Point,
    hovered_star: Option<StarSystem>,
    frand: FastRand,
    rand_star: FastRand,
}

impl Eui {
    pub fn new() -> Self {
        Self {
            sec_size: 32,
            cam: Point { x: 0, y: 0 },
            mouse_pos: Point { x: 0, y: 0 },
            hovered_star: None,
            frand: FastRand::new(),
            rand_star: FastRand::new(),
        }
    }

    pub fn show_ui(&self, d: &mut RaylibDrawHandle) {
        match self.hovered_star {
            Some(star) => {
                let sector_txt = format!("Sector: {}, {}", self.mouse_pos.x, self.mouse_pos.y);
                let radius_txt = format!("Radius: {}", star.radius);
                let mass_txt = format!("Mass: {}", star.mass);
                let surface_txt = format!("Surface: {}ºC", star.surface_temp);
                let luminosity_txt = format!("Luminosity: {}", star.luminosity);
                let bg_rect = Rectangle::new(
                    d.get_mouse_x() as f32 + 20.,
                    d.get_mouse_y() as f32,
                    250.,
                    20. * 5.,
                );
                d.draw_rectangle_rounded(bg_rect, 0.20, 1, Color::BLACK);
                d.draw_rectangle_rounded_lines(bg_rect, 0.20, 1, Color::WHITE);
                d.draw_text(
                    &sector_txt,
                    bg_rect.x as i32 + 10,
                    bg_rect.y as i32,
                    20,
                    Color::WHITE,
                );
                d.draw_text(
                    &radius_txt,
                    bg_rect.x as i32 + 10,
                    bg_rect.y as i32 + 20,
                    20,
                    Color::WHITE,
                );
                d.draw_text(
                    &mass_txt,
                    bg_rect.x as i32 + 10,
                    bg_rect.y as i32 + 20 * 2,
                    20,
                    Color::WHITE,
                );
                d.draw_text(
                    &surface_txt,
                    bg_rect.x as i32 + 10,
                    bg_rect.y as i32 + 20 * 3,
                    20,
                    Color::WHITE,
                );
                d.draw_text(
                    &luminosity_txt,
                    bg_rect.x as i32 + 10,
                    bg_rect.y as i32 + 20 * 4,
                    20,
                    Color::WHITE,
                );
            }

            None => {}
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.mouse_pos = Point {
            x: rl.get_mouse_x() / self.sec_size + self.cam.x,
            y: rl.get_mouse_y() / self.sec_size + self.cam.y,
        };

        self.frand.seed = FastRand::perfectly_hash_them(self.mouse_pos.x, self.mouse_pos.y);
        if self.frand.rand_int(0, 20) == 1 {
            self.hovered_star = Some(StarSystem::generate_star(
                self.mouse_pos.x,
                self.mouse_pos.y,
                self.rand_star,
            ))
        } else {
            self.hovered_star = None
        }

        let keypad_sense = (self.sec_size as f32 * 4. * rl.get_frame_time()) as i32;

        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.cam.y += keypad_sense;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.cam.y -= keypad_sense;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.cam.x -= keypad_sense;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.cam.x += keypad_sense;
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let num_sec_x = d.get_screen_width() / self.sec_size;
        let num_sec_y = d.get_screen_height() / self.sec_size;
        let radius = self.sec_size as f32 / 2.;

        for x in 0..num_sec_x {
            /*d.draw_line(
                x * self.sec_size,
                0,
                x * self.sec_size,
                d.get_screen_height(),
                CORNSILK,
            );*/
            for y in 0..num_sec_y {
                /*d.draw_line(
                    0,
                    y * self.sec_size,
                    d.get_screen_width(),
                    y * self.sec_size,
                    CORNSILK,
                );*/

                let global_sector: Point = Point {
                    x: self.cam.x + x,
                    y: self.cam.y + y,
                };

                self.frand.seed = FastRand::perfectly_hash_them(global_sector.x, global_sector.y);

                if self.frand.rand_int(0, 20) == 1 {
                    let star =
                        StarSystem::generate_star(global_sector.x, global_sector.y, self.rand_star);
                    d.draw_circle(
                        (x * self.sec_size) + radius as i32,
                        y * self.sec_size + radius as i32,
                        (star.radius / 1000.) * radius,
                        star.star_color,
                    );
                }
            }
        }
    }
}
