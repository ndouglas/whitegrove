use serde::*;

use super::TileTrait;

use crate::render::{Factory as RenderableFactory, Renderable};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Wall {
    renderable: Renderable,
}

impl Wall {
    pub fn new() -> Self {
        Wall {
            renderable: RenderableFactory::Wall.create(),
        }
    }
}

impl TileTrait for Wall {
    fn get_renderable(&self) -> Renderable {
        self.renderable
    }

    fn is_walkable(&self) -> bool {
        false
    }

    fn is_opaque(&self) -> bool {
        true
    }
}
