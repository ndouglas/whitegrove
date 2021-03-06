use rltk::{to_cp437, FontCharType, RGB};
use serde::*;

use crate::random;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Factory {
    Player,
    Idiot,
    Orc,
    Goblin,
    Moss,
}

impl Factory {
    pub fn create(&self) -> Renderable {
        use Factory::*;
        match self {
            Player => Renderable {
                glyph: to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
                render_order: 0,
            },
            Idiot => Renderable {
                glyph: to_cp437('☺'),
                fg: RGB::from_u8(
                    random::range(0, 255),
                    random::range(0, 255),
                    random::range(0, 255),
                ),
                bg: RGB::named(rltk::BLACK),
                render_order: 0,
            },
            Orc => Renderable {
                glyph: to_cp437('o'),
                fg: RGB::from_u8(0, random::range(128, 192), 0),
                bg: RGB::named(rltk::BLACK),
                render_order: 0,
            },
            Goblin => Renderable {
                glyph: to_cp437('g'),
                fg: RGB::from_u8(0, random::range(192, 255), 0),
                bg: RGB::named(rltk::BLACK),
                render_order: 0,
            },
            Moss => Renderable {
                glyph: to_cp437('#'),
                fg: RGB::from_u8(23, random::range(32, 92), 23),
                bg: RGB::named(rltk::BLACK),
                render_order: i32::MAX,
            },
        }
    }
}
