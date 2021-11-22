use serde::*;

use crate::model::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TileFlags {
    pub width: usize,
    pub length: usize,
    pub vector: Vec<bool>,
}

impl TileFlags {
    pub fn new(width: usize, length: usize) -> Self {
        TileFlags {
            width: width,
            length: length,
            vector: vec![false; length],
        }
    }

    pub fn get_xy_as_idx(&self, (x, y): (usize, usize)) -> usize {
        xy_to_idx(self.width, (x, y))
    }

    pub fn get_position_as_idx(&self, position: &Position) -> usize {
        xy_to_idx(self.width, (position.x, position.y))
    }

    pub fn get_at_idx(&self, idx: usize) -> bool {
        self.vector[idx]
    }

    pub fn get_at_xy(&self, (x, y): (usize, usize)) -> bool {
        self.get_at_idx(self.get_xy_as_idx((x, y)))
    }

    pub fn get_at_position(&self, position: &Position) -> bool {
        self.get_at_idx(self.get_position_as_idx(position))
    }

    pub fn clear(&mut self) {
        self.vector = vec![false; self.length];
    }

    pub fn set_at_idx(&mut self, idx: usize, value: bool) {
        self.vector[idx] = value;
    }

    pub fn set_at_xy(&mut self, (x, y): (usize, usize), value: bool) {
        self.set_at_idx(self.get_xy_as_idx((x, y)), value);
    }

    pub fn set_at_position(&mut self, position: &Position, value: bool) {
        self.set_at_idx(self.get_position_as_idx(position), value);
    }

    pub fn set_at_positions(&mut self, positions: Vec<&Position>, value: bool) {
        for position in positions.iter() {
            self.set_at_position(position, value);
        }
    }
}
