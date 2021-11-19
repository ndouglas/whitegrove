use serde::*;

#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Position {
    x: i32,
    y: i32,
}
