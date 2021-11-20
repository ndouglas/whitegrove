use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::model::CompassDirection;

#[derive(Component, Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WantsToMove {
    pub compass_direction: CompassDirection,
}
