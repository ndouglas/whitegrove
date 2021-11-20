use rltk::{to_cp437, RGB};
use serde::*;

use super::TileTrait;

use crate::render::Renderable;

#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Wall {}

impl TileTrait for Wall {
    fn get_renderable(&self) -> Renderable {
        Renderable {
            glyph: to_cp437('#'),
            fg: RGB::from_f32(0., 1., 0.),
            bg: RGB::from_f32(0., 0., 0.),
        }
    }

    fn is_walkable(&self) -> bool {
        false
    }
}
