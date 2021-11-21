use rltk::{ to_cp437, RGB };
use serde::*;

use super::TileTrait;

use crate::render::Renderable;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Wall {
    renderable: Renderable,
}

impl Wall {
    pub fn new() -> Self {
        Wall {
            renderable: Renderable {
                glyph: to_cp437('#'),
                fg: RGB::from_f32(0., 1., 0.),
                bg: RGB::from_f32(0., 0., 0.),
            },
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
