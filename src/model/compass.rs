use serde::*;
use std::fmt;

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum CompassDirection {
    Northwest,
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
}

impl CompassDirection {
    pub fn get_delta_xy(&self) -> (i32, i32) {
        use CompassDirection::*;
        match self {
            Northwest => (-1, -1),
            North => (0, -1),
            Northeast => (1, -1),
            East => (1, 0),
            Southeast => (1, 1),
            South => (0, 1),
            Southwest => (-1, 1),
            West => (-1, 0),
        }
    }
}

impl fmt::Display for CompassDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
