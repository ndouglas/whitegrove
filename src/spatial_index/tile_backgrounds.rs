use std::sync::Mutex;

use crate::map::TileBackgrounds;

lazy_static! {
    pub static ref TILE_BACKGROUNDS: Mutex<TileBackgrounds> =
        Mutex::new(TileBackgrounds::new(128, 128 * 128));
}

pub fn set_dimensions(width: usize, length: usize) {
    TILE_BACKGROUNDS
        .lock()
        .unwrap()
        .set_dimensions(width, length);
}
