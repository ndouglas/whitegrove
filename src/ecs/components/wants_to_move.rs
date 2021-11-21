use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::model::CompassDirection;

#[derive(Component, Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum WantsToMove {
    CompassDirection { compass_direction: CompassDirection },
    Randomly { duration: usize },
}
