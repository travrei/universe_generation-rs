use raylib::prelude::*;

use crate::{
    colors::{
        BASE, BURLYWOOD, CADETGREEN, CORNSILK, GREENYELL, LIGHTBLUE, LIGHTSTEEL, MIDBLUE, MIDBLUE2,
        MIDBLUE3, NAVY, ORANGE, PURP, PURPGREY, RED, ROSYPINK, SALMON, SKYBLUE, SLATEBLUE,
        SLAYTEGRASS, TAN, TEAL, TEAL_2, VIOLETRED,
    },
    fast_rand::FastRand,
};

const STAR_COLOR: [Color; 24] = [
    BASE,
    MIDBLUE,
    MIDBLUE2,
    MIDBLUE3,
    CORNSILK,
    SLAYTEGRASS,
    NAVY,
    TEAL,
    PURPGREY,
    PURP,
    LIGHTBLUE,
    TEAL_2,
    ROSYPINK,
    RED,
    SKYBLUE,
    GREENYELL,
    VIOLETRED,
    ORANGE,
    CADETGREEN,
    BURLYWOOD,
    TAN,
    SALMON,
    LIGHTSTEEL,
    SLATEBLUE,
];

#[derive(Clone, Copy)]
pub struct StarSystem {
    pub radius: f32,
    pub star_color: Color,
    pub luminosity: f32,
    pub surface_temp: f32,
    pub mass: f32,
}

impl StarSystem {
    pub fn generate_star(x: i32, y: i32, mut rnd: FastRand) -> StarSystem {
        rnd.seed = FastRand::perfectly_hash_them(x, y);

        StarSystem {
            radius: rnd.rand_float(1., 2000.),
            star_color: STAR_COLOR[rnd.rand_usize(0, STAR_COLOR.len())],
            luminosity: rnd.rand_float(1., 25.),
            surface_temp: rnd.rand_float(1., 10000.),
            mass: rnd.rand_float(1000., 210000.),
        }
    }
}
