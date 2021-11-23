use serde::*;
use std::fmt;

use crate::error::Error;

use super::{get_dim_distance, idx_to_xy, xy_to_idx, CompassDirection};

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub idx: usize,
    pub x_max: usize,
    pub y_max: usize,
}

impl Position {
    pub fn from_xy((x, y): (usize, usize), (width, height): (usize, usize)) -> Self {
        Position {
            x,
            y,
            idx: xy_to_idx(width, (x, y)),
            x_max: width,
            y_max: height,
        }
    }

    pub fn from_idx(idx: usize, (width, height): (usize, usize)) -> Self {
        let (x, y) = idx_to_xy(width, idx);
        Position {
            x,
            y,
            idx,
            x_max: width,
            y_max: height,
        }
    }

    pub fn set_from_position(&mut self, position: &Position) {
        self.x = position.x;
        self.y = position.y;
        self.idx = position.idx;
        self.x_max = position.x_max;
        self.y_max = position.y_max;
    }

    pub fn is_safe_to_delta_xy(&self, (dx, dy): (i32, i32)) -> bool {
        let ix = self.x as i32 + dx;
        let iy = self.y as i32 + dy;
        ix >= 0
            && ix < self.x_max.try_into().unwrap()
            && iy >= 0
            && iy < self.y_max.try_into().unwrap()
    }

    pub fn get_to_delta_xy(&self, (dx, dy): (i32, i32)) -> Result<Self, Error> {
        let ix = self.x as i32 + dx;
        let iy = self.y as i32 + dy;
        if ix < 0 {
            Err(Error::new(format!(
                "x is outside the bounds of the map (< 0)."
            )))
        } else if ix >= self.x_max.try_into().unwrap() {
            Err(Error::new(format!(
                "x is outside the bounds of the map (> {}).",
                self.x_max
            )))
        } else if iy < 0 {
            Err(Error::new(format!(
                "y is outside the bounds of the map (< 0)."
            )))
        } else if iy >= self.y_max.try_into().unwrap() {
            Err(Error::new(format!(
                "y is outside the bounds of the map (> {}).",
                self.y_max
            )))
        } else {
            Ok(Position::from_xy(
                (ix.try_into().unwrap(), iy.try_into().unwrap()),
                (self.x_max, self.y_max),
            ))
        }
    }

    pub fn is_safe_to_compass_direction(&self, dir: CompassDirection) -> bool {
        self.is_safe_to_delta_xy(dir.get_delta_xy())
    }

    pub fn get_to_compass_direction(&self, dir: CompassDirection) -> Result<Position, Error> {
        self.get_to_delta_xy(dir.get_delta_xy())
    }

    pub fn get_delta_xy_to_position(&self, position: &Position) -> (i32, i32) {
        (
            position.x as i32 - self.x as i32,
            position.y as i32 - self.y as i32,
        )
    }

    pub fn get_compass_direction_to_position(&self, position: &Position) -> CompassDirection {
        CompassDirection::get_from_delta_xy(self.get_delta_xy_to_position(position))
    }

    pub fn get_toward_position(&self, position: &Position) -> Result<Position, Error> {
        self.get_to_compass_direction(self.get_compass_direction_to_position(position))
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
