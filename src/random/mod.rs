use rand::distributions::uniform::SampleUniform;
use rltk::prelude::*;
use std::ops::{Add, Sub};
use std::sync::Mutex;

lazy_static! {
    static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

pub fn reseed(seed: u64) {
    *RNG.lock().unwrap() = RandomNumberGenerator::seeded(seed);
}

pub fn roll_dice(n: i32, die_type: i32) -> i32 {
    RNG.lock().unwrap().roll_dice(n, die_type)
}

pub fn range<T: Add<Output = T> + Sub<Output = T> + Copy + PartialEq + PartialOrd + SampleUniform>(min: T, max: T) -> T {
    RNG.lock().unwrap().range(min, max)
}
