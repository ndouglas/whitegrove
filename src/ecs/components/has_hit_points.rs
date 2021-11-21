use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::combat::HitPoints;

#[derive(Component, Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct HasHitPoints {
    pub hit_points: HitPoints,
}
