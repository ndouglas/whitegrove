use serde::*;

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
