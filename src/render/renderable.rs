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
    Idiot,
    Orc,
    Goblin,
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
            Idiot => Renderable {
                glyph: to_cp437('☺'),
                fg: RGB::from_u8(
                    random::range(0, 255),
                    random::range(0, 255),
                    random::range(0, 255),
                ),
                bg: RGB::named(rltk::BLACK),
            },
            Orc => Renderable {
                glyph: to_cp437('o'),
                fg: RGB::from_u8(0, random::range(128, 192), 0),
                bg: RGB::named(rltk::BLACK),
            },
            Goblin => Renderable {
                glyph: to_cp437('g'),
                fg: RGB::from_u8(0, random::range(192, 255), 0),
                bg: RGB::named(rltk::BLACK),
            },
        }
    }
}
