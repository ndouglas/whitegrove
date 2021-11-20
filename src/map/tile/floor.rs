use serde::*;

use super::TileTrait;

use crate::render::{Factory as RenderableFactory, Renderable};

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Floor {}

impl TileTrait for Floor {
    fn get_renderable(&self) -> Renderable {
        RenderableFactory::Floor.create()
    }

    fn is_walkable(&self) -> bool {
        true
    }

    fn is_opaque(&self) -> bool {
        false
    }
}
