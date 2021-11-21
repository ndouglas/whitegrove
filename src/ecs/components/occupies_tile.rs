use serde::*;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OccupiesTile {}
