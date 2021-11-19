use rltk::{FontCharType, RGB};
use serde::*;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
