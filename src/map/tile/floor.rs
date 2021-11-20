use serde::*;

use super::TileTrait;

use crate::render::{Factory as RenderableFactory, Renderable};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Floor {
    renderable: Renderable,
}

impl Floor {
    pub fn new() -> Self {
        Floor {
            renderable: RenderableFactory::Floor.create(),
        }
    }
}

impl TileTrait for Floor {
    fn get_renderable(&self) -> Renderable {
        self.renderable
    }

    fn is_walkable(&self) -> bool {
        true
    }

    fn is_opaque(&self) -> bool {
        false
    }
}
