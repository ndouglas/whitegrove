use specs::prelude::*;
use specs_derive::Component;

use crate::model::*;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub enum WantsToMove {
    CompassDirection { compass_direction: CompassDirection },
    Randomly { duration: usize },
    TowardTarget { target: Entity },
}
