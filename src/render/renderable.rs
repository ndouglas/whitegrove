use rltk::{to_cp437, FontCharType, RGB, RandomNumberGenerator};
use serde::*;

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
                let mut rng = RandomNumberGenerator::new();
                Renderable {
                    glyph: to_cp437('☺'),
                    fg: RGB::from_u8(rng.range(0, 255), rng.range(0, 255), rng.range(0, 255)),
                    bg: RGB::named(rltk::BLACK),
                }
            },
        }
    }
}
