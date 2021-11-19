use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::model::Position;

#[derive(Component, Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct HasPosition {
    pub position: Position,
}
