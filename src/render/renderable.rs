use rltk::{to_cp437, FontCharType, RGB};
use serde::*;

use crate::random;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Factory {
    Player,
    Monster,
}

impl Factory {
    pub fn create(&self) -> Renderable {
        use Factory::*;
        match self {
            Player => Renderable {
                glyph: to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            },
            Monster => {
                Renderable {
                    glyph: to_cp437('☺'),
                    fg: RGB::from_u8(random::range(0, 255), random::range(0, 255), random::range(0, 255)),
                    bg: RGB::named(rltk::BLACK),
                }
            },
        }
    }
}
