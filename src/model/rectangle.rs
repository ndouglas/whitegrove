use serde::*;
use std::collections::HashSet;
use std::fmt;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rectangle {
    pub x1: usize,
    pub x2: usize,
    pub y1: usize,
    pub y2: usize,
}

impl Rectangle {
    pub fn with_corner_xy((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Self {
        Rectangle { x1, x2, y1, y2 }
    }

    pub fn with_size((x1, y1): (usize, usize), (width, height): (usize, usize)) -> Self {
        Rectangle::with_corner_xy((x1, y1), (x1 + width, y1 + height))
    }

    pub fn from_horizontal_line((x1, y1): (usize, usize), length: usize, width: usize) -> Self {
        Rectangle::with_corner_xy((x1, y1), (x1 + length, y1 + width))
    }

    pub fn from_vertical_line((x1, y1): (usize, usize), length: usize, width: usize) -> Self {
        Rectangle::with_corner_xy((x1, y1), (x1 + width, y1 + length))
    }

    pub fn intersects_rectangle(&self, other: &Rectangle) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn contains_xy(&self, (x, y): (usize, usize)) -> bool {
        self.x1 <= x && self.x2 >= x && self.y1 <= y && self.y2 >= y
    }

    pub fn get_center_xy(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn get_all_xy(&self) -> HashSet<(usize, usize)> {
        let mut result = HashSet::new();
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                result.insert((x, y));
            }
        }
        result
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(({}, {}), ({}, {}))",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}
