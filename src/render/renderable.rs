use serde::*;
use rltk::{ FontCharType, RGB };

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}
