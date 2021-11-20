use serde::*;

use super::TileTrait;

use crate::render::{Factory as RenderableFactory, Renderable};

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Wall {}

impl TileTrait for Wall {
    fn get_renderable(&self) -> Renderable {
        RenderableFactory::Wall.create()
    }

    fn is_walkable(&self) -> bool {
        false
    }
}
