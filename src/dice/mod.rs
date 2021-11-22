use serde::*;

use crate::random;

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Dice {
    pub number: i32,
    pub sides: i32,
    pub bonus: i32,
}

impl Dice {
    pub fn roll(&self) -> i32 {
        random::roll_dice(self.number, self.sides) + self.bonus
    }
}
