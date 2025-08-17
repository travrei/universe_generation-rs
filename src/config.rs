use macroquad::prelude::*;

pub const CRUST: Color = Color::new(22.0 / 255.0, 22.0 / 255.0, 29.0 / 255.0, 1.0);
pub const BASE: Color = Color::new(30.0 / 255.0, 31.0 / 255.0, 40.0 / 255.0, 1.0);
pub const MIDBLUE: Color = Color::new(43.0 / 255.0, 42.0 / 255.0, 56.0 / 255.0, 1.0);
pub const MIDBLUE2: Color = Color::new(53.0 / 255.0, 54.0 / 255.0, 71.0 / 255.0, 1.0);
pub const MIDBLUE3: Color = Color::new(84.0 / 255.0, 84.0 / 255.0, 109.0 / 255.0, 1.0);
pub const CORNSILK: Color = Color::new(221.0 / 255.0, 215.0 / 255.0, 186.0 / 255.0, 1.0);
pub const SLAYTEGRASS: Color = Color::new(114.0 / 255.0, 113.0 / 255.0, 104.0 / 255.0, 1.0);
pub const NAVY: Color = Color::new(35.0 / 255.0, 50.0 / 255.0, 73.0 / 255.0, 1.0);
pub const TEAL: Color = Color::new(44.0 / 255.0, 79.0 / 255.0, 103.0 / 255.0, 1.0);
pub const PURPGREY: Color = Color::new(148.0 / 255.0, 138.0 / 255.0, 169.0 / 255.0, 1.0);
pub const PURP: Color = Color::new(149.0 / 255.0, 127.0 / 255.0, 184.0 / 255.0, 1.0);
pub const LIGHTBLUE: Color = Color::new(126.0 / 255.0, 156.0 / 255.0, 215.0 / 255.0, 1.0);
pub const TEAL_2: Color = Color::new(122.0 / 255.0, 168.0 / 255.0, 160.0 / 255.0, 1.0);
pub const ROSYPINK: Color = Color::new(210.0 / 255.0, 126.0 / 255.0, 153.0 / 255.0, 1.0);
pub const RED: Color = Color::new(232.0 / 255.0, 35.0 / 255.0, 35.0 / 255.0, 1.0);
pub const SKYBLUE: Color = Color::new(127.0 / 255.0, 180.0 / 255.0, 203.0 / 255.0, 1.0);
pub const GREENYELL: Color = Color::new(151.0 / 255.0, 187.0 / 255.0, 108.0 / 255.0, 1.0);
pub const VIOLETRED: Color = Color::new(227.0 / 255.0, 104.0 / 255.0, 118.0 / 255.0, 1.0);
pub const ORANGE: Color = Color::new(255.0 / 255.0, 160.0 / 255.0, 102.0 / 255.0, 1.0);
pub const CADETGREEN: Color = Color::new(107.0 / 255.0, 149.0 / 255.0, 137.0 / 255.0, 1.0);
pub const BURLYWOOD: Color = Color::new(230.0 / 255.0, 195.0 / 255.0, 132.0 / 255.0, 1.0);
pub const TAN: Color = Color::new(192.0 / 255.0, 163.0 / 255.0, 110.0 / 255.0, 1.0);
pub const SALMON: Color = Color::new(255.0 / 255.0, 93.0 / 255.0, 98.0 / 255.0, 1.0);
pub const LIGHTSTEEL: Color = Color::new(156.0 / 255.0, 170.0 / 255.0, 201.0 / 255.0, 1.0);
pub const SLATEBLUE: Color = Color::new(101.0 / 255.0, 132.0 / 255.0, 147.0 / 255.0, 1.0);

pub fn window_config() -> Conf {
    Conf {
        window_title: "Universe Generator Demo - Macroquad".to_string(),
        window_width: 900,
        window_height: 800,
        window_resizable: true,
        ..Default::default()
    }
}
