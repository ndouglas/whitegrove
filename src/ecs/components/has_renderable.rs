use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::render::Renderable;

#[derive(Component, Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HasRenderable {
  renderable: Renderable,
}
