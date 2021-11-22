use serde::*;
use std::fmt;

use crate::error::Error;

use super::{idx_to_xy, xy_to_idx, CompassDirection, UnsafePosition};

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn from_idx(width: usize, idx: usize) -> Self {
        let (x, y) = idx_to_xy(width, idx);
        Position { x, y }
    }

    pub fn get_idx(&self, width: usize) -> usize {
        xy_to_idx(width, (self.x, self.y))
    }

    pub fn get_unsafe(&self) -> UnsafePosition {
        (*self).into()
    }

    pub fn get_unsafe_to_delta(&self, (dx, dy): (i32, i32)) -> UnsafePosition {
        self.get_unsafe().get_to_delta((dx, dy))
    }

    pub fn is_safe_to_delta(&self, (width, height): (usize, usize), (dx, dy): (i32, i32)) -> bool {
        self.get_unsafe_to_delta((dx, dy)).is_safe((width, height))
    }

    pub fn get_safe_to_delta(
        &self,
        (width, height): (usize, usize),
        (dx, dy): (i32, i32),
    ) -> Result<Position, Error> {
        self.get_unsafe_to_delta((dx, dy)).get_safe((width, height))
    }

    pub fn is_safe_to_compass_direction(
        &self,
        (width, height): (usize, usize),
        dir: CompassDirection,
    ) -> bool {
        self.get_unsafe()
            .get_to_compass_direction(dir)
            .is_safe((width, height))
    }

    pub fn get_safe_to_compass_direction(
        &self,
        (width, height): (usize, usize),
        dir: CompassDirection,
    ) -> Result<Position, Error> {
        self.get_unsafe()
            .get_to_compass_direction(dir)
            .get_safe((width, height))
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
