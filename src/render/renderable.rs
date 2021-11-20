use rltk::{FontCharType, RGB, to_cp437};
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
    Floor,
    Wall,
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
            Monster => Renderable {
                glyph: to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            },
            Floor => Renderable {
                glyph: to_cp437('.'),
                fg: RGB::from_f32(0.5, 0.5, 0.5),
                bg: RGB::from_f32(0., 0., 0.),
            },
            Wall => Renderable {
                glyph: to_cp437('#'),
                fg: RGB::from_f32(0., 1., 0.),
                bg: RGB::from_f32(0., 0., 0.),
            },
        }
    }

}
