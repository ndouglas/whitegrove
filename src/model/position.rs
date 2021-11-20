use serde::*;

use super::{ idx_to_xy, xy_to_idx };

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {

    pub fn from_idx(width: usize, idx: usize) -> Self {
        let (x, y) = idx_to_xy(width, idx);
        Position {
            x: x,
            y: y,
        }
    }

    pub fn get_idx(&self, width: usize) -> usize {
        xy_to_idx(width, self.x, self.y)
    }

    pub fn to_west(&self) -> Self {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

}
