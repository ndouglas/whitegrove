use rltk::RGB;

use crate::model::*;

#[derive(Clone, Debug, Default)]
pub struct TileLighting {
    pub width: usize,
    pub length: usize,
    pub vector: Vec<RGB>,
}

impl TileLighting {
    pub fn new(width: usize, length: usize) -> Self {
        TileLighting {
            width: width,
            length: length,
            vector: vec![RGB::named(rltk::BLACK); length],
        }
    }

    pub fn get_xy_as_idx(&self, (x, y): (usize, usize)) -> usize {
        xy_to_idx(self.width, (x, y))
    }

    pub fn get_position_as_idx(&self, position: &Position) -> usize {
        xy_to_idx(self.width, (position.x, position.y))
    }

    pub fn get_at_idx(&self, idx: usize) -> RGB {
        trace!("Getting lighting at {}", idx);
        self.vector[idx]
    }

    pub fn get_at_xy(&self, (x, y): (usize, usize)) -> RGB {
        self.get_at_idx(self.get_xy_as_idx((x, y)))
    }

    pub fn get_at_position(&self, position: &Position) -> RGB {
        self.get_at_idx(self.get_position_as_idx(position))
    }

    pub fn clear(&mut self) {
        trace!("Clearing vector.");
        self.vector = vec![RGB::named(rltk::BLACK); self.length];
    }

    pub fn set_at_idx(&mut self, idx: usize, rgb: RGB) {
        trace!("Setting lighting {:?} at {}.", rgb, idx);
        self.vector[idx] = rgb;
    }

    pub fn set_at_xy(&mut self, (x, y): (usize, usize), rgb: RGB) {
        self.set_at_idx(self.get_xy_as_idx((x, y)), rgb);
    }

    pub fn set_at_position(&mut self, position: &Position, rgb: RGB) {
        self.set_at_xy((position.x, position.y), rgb);
    }

    pub fn set_at_positions(&mut self, vector: &Vec<(RGB, &Position)>) {
        for (rgb, position) in vector.iter() {
            self.set_at_position(position, *rgb);
        }
    }

    pub fn remove_at_idx(&mut self, idx: usize) {
        trace!("Removing lighting at {}.", idx);
        self.vector[idx] = RGB::named(rltk::BLACK);
    }

    pub fn remove_at_xy(&mut self, (x, y): (usize, usize)) {
        self.remove_at_idx(self.get_xy_as_idx((x, y)));
    }

    pub fn remove_at_position(&mut self, position: &Position) {
        self.remove_at_xy((position.x, position.y));
    }

    pub fn set_dimensions(&mut self, width: usize, length: usize) {
        self.width = width;
        self.length = length;
        self.clear();
    }
}
