#[derive(Clone, Copy)]
pub struct FastRand {
    pub seed: u64,
}

impl FastRand {
    pub fn new() -> Self {
        FastRand { seed: 0 }
    }

    pub fn rnd(&mut self) -> u32 {
        self.seed = self.seed.wrapping_add(0xe120fc15);
        let mut tmp: u64 = self.seed.wrapping_mul(0x4a39b70d);
        let m1: u32 = ((tmp >> 32) as u32) ^ (tmp as u32);
        tmp = (m1 as u64).wrapping_mul(0x12fad5c9);
        let m2: u32 = ((tmp >> 32) as u32) ^ (tmp as u32);
        m2
    }

    pub fn rand_int(&mut self, min: i32, max: i32) -> i32 {
        (self.rnd() as i32 % (max - min)) + min
    }

    pub fn rand_usize(&mut self, min: usize, max: usize) -> usize {
        self.rnd() as usize % (max - min) + min
    }

    pub fn rand_float(&mut self, min: f32, max: f32) -> f32 {
        (self.rnd() as f32 / u32::MAX as f32) * (max - min) + min
    }

    pub fn perfectly_hash_them(a: i32, b: i32) -> u64 {
        let capital_a: i128 = if a >= 0 {
            2 * (a as i128)
        } else {
            -2 * (a as i128) - 1
        };

        let capital_b: i128 = if b >= 0 {
            2 * (b as i128)
        } else {
            -2 * (b as i128) - 1
        };

        let c: i128 = if capital_a >= capital_b {
            (capital_a * capital_a + capital_a + capital_b) / 2
        } else {
            (capital_a + capital_b * capital_b) / 2
        };

        if a.signum() == b.signum() || b == 0 {
            c as u64
        } else {
            (-c - 1) as u64
        }
    }
}
