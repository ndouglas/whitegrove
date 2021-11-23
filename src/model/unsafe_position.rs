use serde::*;
use std::convert::From;
use std::fmt;

use crate::error::Error;

use super::{idx_to_xy, xy_to_idx, CompassDirection, Position};

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct UnsafePosition {
    pub x: i32,
    pub y: i32,
}

impl UnsafePosition {
    pub fn new(x: i32, y: i32) -> Self {
        UnsafePosition { x, y }
    }

    pub fn from_idx(width: usize, idx: usize) -> Self {
        let (x, y) = idx_to_xy(width, idx);
        UnsafePosition {
            x: x as i32,
            y: y as i32,
        }
    }

    pub fn get_idx(&self, width: usize) -> usize {
        xy_to_idx(width, (self.x as usize, self.y as usize))
    }

    pub fn get_xy(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

    pub fn get_xy_with_offset(&self, (dx, dy): (i32, i32)) -> (i32, i32) {
        (self.x as i32 + dx, self.y as i32 + dy)
    }

    pub fn get_to_delta_xy(&self, (dx, dy): (i32, i32)) -> Self {
        let (new_x, new_y) = self.get_xy_with_offset((dx, dy));
        UnsafePosition::new(new_x, new_y)
    }

    pub fn get_delta_xy_to_position(&self, position: &UnsafePosition) -> (i32, i32) {
        (self.x - position.x, self.y - position.y)
    }

    pub fn get_to_compass_direction(&self, dir: CompassDirection) -> Self {
        self.get_to_delta_xy(dir.get_delta_xy())
    }

    pub fn is_safe(&self, (width, height): (usize, usize)) -> bool {
        self.x >= 0
            && self.y >= 0
            && self.x < width.try_into().unwrap()
            && self.y < height.try_into().unwrap()
    }

    pub fn get_safe(&self, (width, height): (usize, usize)) -> Result<Position, Error> {
        if self.is_safe((width, height)) {
            Ok(Position::new(self.x as usize, self.y as usize))
        } else {
            Err(Error::new(format!(
                "Coordinates {:?} exceed the limits {:?}",
                (self.x, self.y),
                (width, height),
            )))
        }
    }
}

impl From<Position> for UnsafePosition {
    fn from(item: Position) -> Self {
        UnsafePosition {
            x: item.x as i32,
            y: item.y as i32,
        }
    }
}

impl fmt::Display for UnsafePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
