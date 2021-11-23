use serde::*;

use crate::model::Position;
use crate::render::Renderable;

use super::Lifetime;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Request {
    pub position: Position,
    pub renderable: Renderable,
    pub lifetime: Lifetime,
}

impl Request {
    pub fn new(position: Position, renderable: Renderable, lifetime: Lifetime) -> Self {
        Request {
            position,
            renderable,
            lifetime,
        }
    }
}
