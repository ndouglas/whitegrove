use serde::*;
use std::fmt;

use crate::error::Error;

use super::{get_dim_distance, idx_to_xy, xy_to_idx, CompassDirection, UnsafePosition};

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

    pub fn get_unsafe_to_delta_xy(&self, (dx, dy): (i32, i32)) -> UnsafePosition {
        self.get_unsafe().get_to_delta_xy((dx, dy))
    }

    pub fn is_safe_to_delta_xy(
        &self,
        (width, height): (usize, usize),
        (dx, dy): (i32, i32),
    ) -> bool {
        self.get_unsafe_to_delta_xy((dx, dy))
            .is_safe((width, height))
    }

    pub fn get_safe_to_delta_xy(
        &self,
        (width, height): (usize, usize),
        (dx, dy): (i32, i32),
    ) -> Result<Position, Error> {
        self.get_unsafe_to_delta_xy((dx, dy))
            .get_safe((width, height))
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

    pub fn get_unsafe_to_compass_direction(&self, dir: CompassDirection) -> UnsafePosition {
        self.get_unsafe().get_to_compass_direction(dir)
    }

    pub fn get_delta_xy_to_position(&self, position: &Position) -> (i32, i32) {
        self.get_unsafe()
            .get_delta_xy_to_position(&position.get_unsafe())
    }

    pub fn get_compass_direction_to_position(&self, position: &Position) -> CompassDirection {
        CompassDirection::get_from_delta_xy(self.get_delta_xy_to_position(position))
    }

    pub fn get_unsafe_toward_position(&self, position: &Position) -> UnsafePosition {
        self.get_unsafe_to_compass_direction(self.get_compass_direction_to_position(position))
    }

    pub fn get_safe_toward_position(
        &self,
        (width, height): (usize, usize),
        position: &Position,
    ) -> Result<Position, Error> {
        self.get_safe_to_compass_direction(
            (width, height),
            self.get_compass_direction_to_position(position),
        )
    }

    pub fn is_neighboring_position(&self, position: &Position) -> bool {
        get_dim_distance(self.x, position.x) <= 1 && get_dim_distance(self.y, position.y) <= 1
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
